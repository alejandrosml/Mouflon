//! Animation procédurale : traduit les signaux de pose du moteur
//! (démarche, écrasement, regard…) en transformations des parties du rig.
//!
//! Toutes les transformations sont exprimées dans le repère du document SVG
//! et pensées pour un personnage tourné vers la droite ; le retournement est
//! appliqué en dernier.

use std::f64::consts::PI;

use mouflon_core::{PoseSignals, RUN_STRIDE};
use vello::kurbo::{Affine, Point, Vec2};

use crate::rig::{Part, Rig};

/// Transformations d'une image.
#[derive(Clone, Debug)]
pub struct PoseTransforms {
    /// Transformation du personnage entier (écrasement, inclinaison, retournement).
    pub root: Affine,
    parts: [Affine; Part::ALL.len()],
}

impl PoseTransforms {
    #[must_use]
    pub fn part(&self, part: Part) -> Affine {
        self.parts[part.index()]
    }
}

fn about(p: Point, t: Affine) -> Affine {
    Affine::translate(p.to_vec2()) * t * Affine::translate(-p.to_vec2())
}

fn rotate_about(p: Point, angle: f64) -> Affine {
    about(p, Affine::rotate(angle))
}

fn scale_about(p: Point, sx: f64, sy: f64) -> Affine {
    about(p, Affine::scale_non_uniform(sx, sy))
}

/// Lissage de 0 à 1 entre `a` et `b`.
fn ease(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Décalage de chaque patte dans le cycle de marche : au pas, l'ordre d'un
/// mouton (arrière, avant du même côté, puis de l'autre côté, un quart de
/// cycle entre chaque pied) ; au galop, des bonds (les deux avant, puis les
/// deux arrière).
fn leg_offset(part: Part, run: f64) -> f64 {
    let (walk, gallop) = match part {
        Part::LegBackNear => (0.0, PI),
        Part::LegFrontNear => (0.5 * PI, 0.0),
        Part::LegBackFar => (PI, PI + 0.3),
        _ => (1.5 * PI, 0.3),
    };
    walk + (gallop - walk) * run
}

/// Une patte dans son cycle (`cycle` en radians) : appui pendant la part
/// `duty` du cycle, le sabot posé pendant que la patte balaie de l'avant
/// vers l'arrière à vitesse constante (il ne glisse pas), puis retour vers
/// l'avant, levée. Rend l'angle (positif : sabot vers l'arrière), la levée
/// (0 à 1) et l'appui (0 à 1, pour garder le sabot au sol).
fn stepping(cycle: f64, duty: f64, amplitude: f64) -> (f64, f64, f64) {
    let t = cycle.rem_euclid(2.0 * PI) / (2.0 * PI);
    if t < duty {
        let s = t / duty;
        // Appui : de -A à +A, régulièrement ; à ses bords, il se fond dans le vol.
        let support = (s * (1.0 - s) * 12.0).min(1.0);
        (amplitude * (2.0 * s - 1.0), 0.0, support)
    } else {
        let s = (t - duty) / (1.0 - duty);
        let e = s * s * (3.0 - 2.0 * s);
        (amplitude * (1.0 - 2.0 * e), (PI * s).sin(), 0.0)
    }
}

/// Calcule la pose d'une image. `stride` : distance parcourue pendant un
/// cycle de marche, en unités du dessin (0 : 0,9 fois sa hauteur).
#[must_use]
pub fn pose(rig: &Rig, s: &PoseSignals, facing: f32, stride: f64) -> PoseTransforms {
    let f = |v: f32| f64::from(v);
    let h = rig.bounds.height().max(1.0);
    let u = h / 100.0;
    let g = rig.ground;
    let phase = f(s.gait_phase);
    let (walk, run, sit, sleep, air, dangle) = (
        f(s.walk),
        f(s.run),
        f(s.sit),
        f(s.sleep),
        f(s.air),
        f(s.dangle),
    );
    let graze = ease(0.0, 1.0, f(s.graze));
    let rise = f(s.rise);
    let breath = f(s.breath);
    let look = Vec2::new(f(s.look.x), f(s.look.y));

    // --- Personnage entier. L'écrasement de l'atterrissage reste mesuré :
    // les pattes s'écartent pour amortir le reste.
    let landing = f(s.squash).clamp(-0.15, 0.16);
    let squash = landing + breath * (0.008 * (1.0 - sleep) + 0.022 * sleep);
    let squash_t = scale_about(g, 1.0 + 0.55 * squash, 1.0 - squash);
    let lean_pivot = Point::new(g.x, g.y - h * 0.45);
    let lean_t = rotate_about(lean_pivot, f(s.lean) * 0.6);
    let hold = Point::new(rig.pivot(Part::Body).x, rig.bounds.min_y() + h * 0.12);
    let swing_t = rotate_about(hold, f(s.local_swing(facing)));
    let dangle_t = scale_about(hold, 1.0 - 0.035 * dangle, 1.0 + 0.06 * dangle);
    let fx = {
        let v = f(facing).clamp(-1.0, 1.0);
        // On ne passe jamais par une largeur nulle : le demi-tour reste lisible.
        let sign = if v < 0.0 { -1.0 } else { 1.0 };
        sign * (0.34 + 0.66 * v.abs())
    };
    let facing_t = about(Point::new(g.x, 0.0), Affine::scale_non_uniform(fx, 1.0));
    // Culbute dans le repère de l'écran, autour du milieu du corps (après le retournement).
    let roll_t = rotate_about(Point::new(g.x, g.y - h * 0.5), f(s.roll));
    let root = roll_t * facing_t * swing_t * dangle_t * lean_t * squash_t;

    // --- Corps couché : l'avant se plie d'abord (il s'agenouille), puis
    // l'arrière ; il se relève dans l'ordre inverse, l'arrière d'abord.
    let body = rig.part_bounds(Part::Body);
    let has_body = rig.has_part(Part::Body);
    let clearance = if has_body {
        (g.y - body.max_y() - 0.6 * u).max(0.0)
    } else {
        h * 0.12
    };
    let front_down = ease(0.0, 0.65, sit);
    let back_down = ease(0.35, 1.0, sit);
    let drop_front = clearance * front_down;
    let drop_back = clearance * back_down;
    let half_length = if has_body {
        (body.width() * 0.5).max(u)
    } else {
        h * 0.4
    };
    let center = if has_body { body.center() } else { g };
    let pitch = ((drop_front - drop_back) / (2.0 * half_length)).atan();

    // Trot ou galop : le corps monte et descend à chaque appui ; au galop, il
    // se balance aussi d'avant en arrière.
    let bob = -(walk * 1.0 * (phase * 2.0).sin().abs() + run * 2.6 * phase.sin().abs()) * u;
    let rock = run * 0.07 * phase.cos();

    let mut parts = [Affine::IDENTITY; Part::ALL.len()];
    parts[Part::Body.index()] = Affine::translate((0.0, bob + 0.5 * (drop_front + drop_back)))
        * rotate_about(center, pitch + rock + 0.05 * graze)
        * scale_about(g, 1.0 + 0.04 * sleep, 1.0 - 0.05 * sleep);

    // --- Tête : regard, hochement au pas, sommeil, et broute la tête au sol.
    let head_pivot = rig.pivot(Part::Head);
    let head_bounds = rig.part_bounds(Part::Head);
    let mut head_angle = look.y * 0.16 + 0.42 * sleep - 0.06 * air
        + f(s.lean) * 0.3
        + (phase * 2.0).sin() * 0.035 * walk
        - 0.12 * dangle
        - 0.12 * air * ease(0.0, 1.0, rise)
        + 0.1 * air * ease(0.0, 1.0, -rise);
    let mut head_shift = Vec2::new(
        look.x * u * 1.4,
        look.y * u * 1.0 + bob * 1.15 + drop_front + sleep * h * 0.06,
    );
    if graze > 0.0 && rig.has_part(Part::Head) {
        // Bouche au ras du sol, juste devant les pattes de devant : la tête
        // pivote vers le bas, et le cou (dans la laine) l'abaisse du reste.
        let angle = 0.85 + 0.05 * f(s.nibble);
        // Le point le plus bas de la tête une fois penchée (coin de sa boîte,
        // côté museau) descend jusqu'au ras du sol.
        let turn = rotate_about(head_pivot, angle);
        let lowest = [
            Point::new(head_bounds.max_x(), head_bounds.max_y()),
            Point::new(head_bounds.max_x(), head_bounds.min_y()),
            Point::new(head_bounds.min_x(), head_bounds.max_y()),
        ]
        .into_iter()
        .map(|corner| (turn * corner).y)
        .fold(f64::NEG_INFINITY, f64::max);
        let lower = (g.y - 1.5 * u - lowest).max(0.0);
        head_angle += graze * angle;
        head_shift += Vec2::new(graze * 1.5 * u, graze * (lower + 0.5 * u * f(s.nibble)));
    }
    parts[Part::Head.index()] =
        Affine::translate(head_shift) * rotate_about(head_pivot, head_angle);

    let ear_droop = 0.3 * sleep + 0.15 * dangle + 0.18 * graze;
    parts[Part::EarNear.index()] = rotate_about(
        rig.pivot(Part::EarNear),
        f(s.ear) + 0.05 * breath + ear_droop,
    );
    parts[Part::EarFar.index()] = rotate_about(
        rig.pivot(Part::EarFar),
        f(s.ear) * 0.8 + 0.04 * breath + ear_droop * 0.8,
    );

    let open = (1.0 - 0.94 * f(s.blink)).max(0.06);
    parts[Part::Eyes.index()] = scale_about(rig.pivot(Part::Eyes), 1.0, open);
    parts[Part::Pupils.index()] = Affine::translate((look.x * u * 2.6, look.y * u * 2.0));

    // La queue suit l'arrière du corps.
    parts[Part::Tail.index()] = Affine::translate((0.0, bob + drop_back))
        * rotate_about(rig.pivot(Part::Tail), f(s.tail) - 0.2 * dangle);

    // --- Pattes. Le balayage de l'appui couvre exactement la distance que le
    // corps parcourt pendant ce temps : les sabots restent où ils sont posés.
    let stride = if stride > 0.0 { stride } else { h * 0.9 } * (1.0 + f64::from(RUN_STRIDE) * run);
    let duty = 0.65 + (0.38 - 0.65) * run;
    let leg_len = {
        let b = rig.part_bounds(Part::LegFrontNear);
        if rig.has_part(Part::LegFrontNear) {
            (g.y - rig.pivot(Part::LegFrontNear).y)
                .max(u * 8.0)
                .max(b.height() * 0.5)
        } else {
            h * 0.3
        }
    };
    let gait = walk.max(run);
    let amplitude = (stride * duty / (2.0 * leg_len)).clamp(0.1, 0.75);
    let lift_amount = (2.2 + 3.0 * run) * u * gait;
    let tuck = air * ease(0.0, 1.0, rise);
    let reach = air * ease(0.0, 1.0, -rise);
    let splay = landing.max(0.0) * 3.0;
    for part in [
        Part::LegFrontNear,
        Part::LegBackFar,
        Part::LegFrontFar,
        Part::LegBackNear,
    ] {
        let front = matches!(part, Part::LegFrontNear | Part::LegFrontFar);
        let hip = rig.pivot(part);
        let cycle = phase + leg_offset(part, run);
        let (step, raised, support) = stepping(cycle, duty, amplitude);
        let mut angle = gait * step;
        // En l'air : pattes repliées en montant, tendues vers le sol en descendant.
        angle += if front {
            -0.15 * air + 1.0 * tuck - 0.3 * reach
        } else {
            0.15 * air + 0.55 * tuck - 0.12 * reach
        };
        angle += dangle * (if front { -0.1 } else { 0.12 } + f(s.swing) * 0.4);
        // À l'atterrissage, elles s'écartent ; couché, elles se replient
        // sous le corps (l'avant vers l'arrière, l'arrière vers l'avant).
        angle += if front { -splay } else { splay };
        angle += if front {
            1.3 * front_down
        } else {
            -1.25 * back_down
        };
        let lift = raised * lift_amount;
        let drop = if front { drop_front } else { drop_back };
        // En appui, la patte penchée s'allonge juste assez pour rester au sol.
        let planted = gait * support * (1.0 / (gait * step).cos().max(0.5) - 1.0);
        let stretch = 1.0 + planted + 0.07 * dangle + 0.04 * air;
        parts[part.index()] = Affine::translate((0.0, drop - lift))
            * rotate_about(hip, angle)
            * scale_about(hip, 1.0, stretch);
    }

    PoseTransforms { root, parts }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <g id="leg-front-near"><rect x="70" y="60" width="8" height="36" fill="#333"/></g>
        <g id="body"><ellipse cx="50" cy="55" rx="35" ry="22" fill="#999"/></g>
        <g id="head"><circle cx="80" cy="35" r="15" fill="#888"/></g>
        <circle id="pivot-ground" cx="50" cy="96" r="1"/>
    </svg>"##;

    #[test]
    fn neutral_pose_keeps_ground_point_fixed() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let p = pose(&rig, &PoseSignals::default(), 1.0, 0.0);
        let moved = p.root * rig.ground;
        assert!((moved - rig.ground).hypot() < 1e-6);
    }

    #[test]
    fn facing_left_mirrors_around_ground() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let p = pose(&rig, &PoseSignals::default(), -1.0, 0.0);
        let head = Point::new(80.0, 35.0);
        let mirrored = p.root * head;
        assert!((mirrored.x - (2.0 * rig.ground.x - head.x)).abs() < 1e-6);
    }

    #[test]
    fn lying_down_kneels_in_front_first() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let signals = PoseSignals {
            sit: 0.4,
            ..PoseSignals::default()
        };
        let p = pose(&rig, &signals, 1.0, 0.0);
        let body = p.part(Part::Body);
        let front = body * Point::new(80.0, 70.0);
        let back = body * Point::new(20.0, 70.0);
        assert!(front.y - 70.0 > back.y - 70.0 + 1.0, "{front:?} {back:?}");
        // Couché tout à fait : le corps touche presque le sol, les pattes repliées dessous.
        let lying = PoseSignals {
            sit: 1.0,
            ..PoseSignals::default()
        };
        let p = pose(&rig, &lying, 1.0, 0.0);
        let bottom = p.part(Part::Body) * Point::new(50.0, 77.0);
        assert!(bottom.y > 90.0 && bottom.y <= 96.0, "{bottom:?}");
        let leg = rig.part_bounds(Part::LegFrontNear);
        let foot = p.part(Part::LegFrontNear) * Point::new(leg.center().x, leg.max_y());
        assert!(foot.y <= 96.0, "sabot sous le sol : {foot:?}");
    }

    #[test]
    fn grazing_brings_the_head_down_to_the_ground() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let signals = PoseSignals {
            graze: 1.0,
            ..PoseSignals::default()
        };
        let p = pose(&rig, &signals, 1.0, 0.0);
        let head = rig.part_bounds(Part::Head);
        let lowest = [
            Point::new(head.min_x(), head.max_y()),
            Point::new(head.max_x(), head.max_y()),
            Point::new(head.max_x(), head.min_y()),
        ]
        .map(|c| (p.part(Part::Head) * c).y)
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max);
        assert!(
            lowest > 90.0 && lowest < 96.0,
            "tête au plus bas en {lowest}"
        );
    }

    #[test]
    fn landing_squash_flattens() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let signals = PoseSignals {
            squash: 0.3,
            ..PoseSignals::default()
        };
        let p = pose(&rig, &signals, 1.0, 0.0);
        let top = p.root * Point::new(50.0, 33.0);
        assert!(top.y > 33.0, "le haut du corps doit descendre");
    }
}
