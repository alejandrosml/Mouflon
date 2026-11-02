//! Animation procédurale : traduit les signaux de pose du moteur
//! (démarche, écrasement, regard…) en transformations des parties du rig.
//!
//! Toutes les transformations sont exprimées dans le repère du document SVG
//! et pensées pour un personnage tourné vers la droite ; le retournement est
//! appliqué en dernier.

use std::f64::consts::PI;

use mouflon_core::{PoseSignals, RUN_STRIDE};

use crate::gesture::{ears_turn, gesturing, leg_slot, with_posture};
use peniko::kurbo::{Affine, Point, Vec2};

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

pub(crate) fn about(p: Point, t: Affine) -> Affine {
    Affine::translate(p.to_vec2()) * t * Affine::translate(-p.to_vec2())
}

pub(crate) fn rotate_about(p: Point, angle: f64) -> Affine {
    about(p, Affine::rotate(angle))
}

pub(crate) fn scale_about(p: Point, sx: f64, sy: f64) -> Affine {
    about(p, Affine::scale_non_uniform(sx, sy))
}

/// Lissage de 0 à 1 entre `a` et `b`.
pub(crate) fn ease(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Un tour de cycle (radians) : les instants du cycle sont donnés en
/// fractions de la foulée, comptées depuis le poser du postérieur qu'on voit.
const TURN: f64 = 2.0 * PI;

// Les mouvements du corps à chaque allure, en centièmes de la hauteur de
// l'animal, crête à crête (voir `docs/animation-animale.md`).
/// Au pas, le garrot et la croupe montent à mi-appui de leurs pattes
/// (pendule inversé), deux fois par foulée ; la croupe plus que le garrot.
const WITHERS_VAULT: f64 = 1.0;
const CROUP_VAULT: f64 = 2.2;
/// Instants où ils sont au plus haut : mi-appui d'un antérieur, d'un postérieur.
const WITHERS_HIGH: f64 = 0.59;
const CROUP_HIGH: f64 = 0.32;
/// La tête au pas : au plus bas quand le garrot est au plus haut (elle
/// charge les antérieurs).
const HEAD_NOD: f64 = 2.4;
/// Au trot, le corps rebondit comme sur un ressort : au plus bas vers le
/// milieu de l'appui de chaque paire diagonale.
const TROT_BOUNCE: f64 = 2.4;
const TROT_LOW: f64 = 0.2;
/// Au galop, une montée par foulée, au plus haut pendant la suspension
/// groupée ; le corps bascule, le nez plonge quand les antérieurs
/// reçoivent le poids (radians).
const RUN_BOUNCE: f64 = 1.4;
const RUN_HIGH: f64 = 0.19;
const RUN_PITCH: f64 = 0.08;
const RUN_NOSE_DOWN: f64 = 0.82;

/// Les allures mêlées d'un quadrupède : le pas, le trot et le galop, selon
/// les poids que donne le moteur (voir `docs/animation-animale.md`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Gait {
    pub walk: f64,
    /// Part du trot qui n'est pas encore du galop.
    pub trot: f64,
    pub run: f64,
}

impl Gait {
    pub(crate) fn of(s: &PoseSignals) -> Self {
        let run = f64::from(s.run);
        Self {
            walk: f64::from(s.walk),
            trot: f64::from(s.trot) * (1.0 - run),
            run,
        }
    }

    /// Une allure se dessine dès que l'animal avance.
    pub(crate) fn moving(&self) -> f64 {
        self.walk.max(self.run)
    }

    /// Part de l'allure qui est du pas (ni trot ni galop).
    fn walking(&self) -> f64 {
        self.walk * (1.0 - self.trot) * (1.0 - self.run)
    }

    /// Part du cycle où une patte est à l'appui : au pas, près des deux
    /// tiers (0,58 à 0,69 chez le mouton) ; au trot, une paire diagonale,
    /// un peu moins de la moitié ; au galop, moins d'un tiers.
    pub(crate) fn duty(&self) -> f64 {
        let d = 0.64 + (0.48 - 0.64) * self.trot;
        d + (0.32 - d) * self.run
    }

    /// Hauteur où monte un sabot en avançant (`u` : centième de la hauteur) :
    /// chez le mouton, 6 à 7 % de la hauteur au garrot, un peu plus au trot ;
    /// bien plus au galop.
    pub(crate) fn lift(&self, u: f64) -> f64 {
        (3.4 + 0.6 * self.trot + 3.5 * self.run) * u * self.moving()
    }

    /// Au pas, de combien montent le garrot et la croupe (en `u`).
    fn vault(&self, phase: f64) -> (f64, f64) {
        let k = self.walking();
        let front = 0.5 * WITHERS_VAULT * (2.0 * (phase - TURN * WITHERS_HIGH)).cos();
        let back = 0.5 * CROUP_VAULT * (2.0 * (phase - TURN * CROUP_HIGH)).cos();
        (k * front, k * back)
    }

    /// Montée et descente du corps (négatif : vers le haut). Au pas, il
    /// passe par-dessus chaque patte d'appui comme un pendule renversé ; au
    /// trot, il descend à chaque appui diagonal et remonte pendant le bref
    /// envol ; au galop, une grande montée par foulée.
    pub(crate) fn bob(&self, phase: f64, u: f64) -> f64 {
        let (front, back) = self.vault(phase);
        let walk = 0.5 * (front + back);
        let trot = self.trot * TROT_BOUNCE * (0.5 - 0.5 * (2.0 * (phase - TURN * TROT_LOW)).cos());
        let run = self.run * RUN_BOUNCE * (0.5 + 0.5 * (phase - TURN * RUN_HIGH).cos());
        -(walk + trot + run) * u
    }

    /// Tangage du corps (positif : l'avant descend) : au pas, la croupe et
    /// le garrot montent tour à tour ; au galop, le corps bascule à chaque
    /// foulée. `length` : longueur du corps.
    pub(crate) fn pitch(&self, phase: f64, u: f64, length: f64) -> f64 {
        let (front, back) = self.vault(phase);
        let walk = ((back - front) * u / length.max(u)).atan();
        walk + self.run * RUN_PITCH * (phase - TURN * RUN_NOSE_DOWN).cos()
    }

    /// La tête au pas, en plus du corps (positif : vers le bas).
    pub(crate) fn head_bob(&self, phase: f64, u: f64) -> f64 {
        self.walking() * 0.5 * HEAD_NOD * (2.0 * (phase - TURN * WITHERS_HIGH)).cos() * u
    }

    /// Hochement de la tête au pas : elle penche quand elle descend.
    pub(crate) fn nod(&self, phase: f64) -> f64 {
        0.04 * self.walking() * (2.0 * (phase - TURN * WITHERS_HIGH)).cos()
    }
}

/// Longueur de la foulée en cours, en fois celle du pas.
pub(crate) fn stride_scale(s: &PoseSignals) -> f64 {
    if s.stride_scale > 0.0 {
        f64::from(s.stride_scale)
    } else {
        1.0 + f64::from(RUN_STRIDE) * f64::from(s.run)
    }
}

/// Décalage de chaque patte dans le cycle (voir [`leg_phase`]).
fn leg_offset(part: Part, gait: &Gait) -> f64 {
    leg_phase(
        matches!(part, Part::LegFrontNear | Part::LegFrontFar),
        matches!(part, Part::LegFrontNear | Part::LegBackNear),
        gait,
    )
}

/// Décalage d'une patte de devant ou de derrière, du côté qu'on voit ou de
/// l'autre, dans le cycle (radians). Au pas, l'ordre du mouton : arrière,
/// avant du même côté un peu plus d'un quart de cycle après (0,27), puis
/// l'autre côté. Au trot, les paires diagonales ensemble, l'arrière posé un
/// rien avant l'avant. Au galop, le galop transverse des moutons : arrière
/// de ce côté, arrière de l'autre, avant de ce côté, avant de l'autre, puis
/// une suspension, pattes groupées. Les valeurs sont choisies pour passer
/// de l'une à l'autre sans qu'une patte fasse un tour de trop.
pub(crate) fn leg_phase(front: bool, near: bool, gait: &Gait) -> f64 {
    let (walk, trot, gallop) = match (front, near) {
        (false, true) => (0.0, 0.0, 0.29),
        (true, true) => (0.27, 0.53, 0.55),
        (false, false) => (0.5, 0.5, 0.45),
        (true, false) => (0.77, 1.03, 0.77),
    };
    let mid = walk + (trot - walk) * gait.trot;
    TURN * (mid + (gallop - mid) * gait.run)
}

/// Une patte dans son cycle (`cycle` en radians) : appui pendant la part
/// `duty` du cycle, le sabot posé pendant que la patte balaie de l'avant
/// vers l'arrière à vitesse constante (il ne glisse pas), puis retour vers
/// l'avant, levée. Rend l'angle (positif : sabot vers l'arrière), la levée
/// (0 à 1) et l'appui (0 à 1, pour garder le sabot au sol).
pub(crate) fn stepping(cycle: f64, duty: f64, amplitude: f64, front: bool) -> (f64, f64, f64) {
    let t = cycle.rem_euclid(2.0 * PI) / (2.0 * PI);
    if t < duty {
        let s = t / duty;
        // Appui : de -A à +A, régulièrement ; à ses bords, il se fond dans le vol.
        let support = (s * (1.0 - s) * 12.0).min(1.0);
        (amplitude * (2.0 * s - 1.0), 0.0, support)
    } else {
        let s = (t - duty) / (1.0 - duty);
        let e = s * s * (3.0 - 2.0 * s);
        // Un antérieur monte au plus haut tôt dans son vol (vers 0,38), un
        // postérieur à mi-vol.
        let s_lift = if front { s.powf(FORE_LIFT_SKEW) } else { s };
        (amplitude * (1.0 - 2.0 * e), (PI * s_lift).sin(), 0.0)
    }
}

/// `ln 0,5 / ln 0,38` : l'antérieur au plus haut à 38 % de son vol.
const FORE_LIFT_SKEW: f64 = 0.716;

/// Le personnage entier, autour de son point d'appui `g` (repère du dessin,
/// `h` : sa hauteur) : écrasement, inclinaison, balancement et étirement
/// quand on le tient par `hold`, retournement, culbute. L'écrasement de
/// l'atterrissage reste mesuré : les pattes s'écartent pour amortir le reste.
pub(crate) fn root_transform(
    s: &PoseSignals,
    facing: f32,
    g: Point,
    h: f64,
    hold: Point,
) -> Affine {
    let f = |v: f32| f64::from(v);
    let (sleep, dangle, breath) = (f(s.sleep), f(s.dangle), f(s.breath));
    let landing = f(s.squash).clamp(-0.15, 0.16);
    let squash = landing + breath * (0.008 * (1.0 - sleep) + 0.022 * sleep);
    let squash_t = scale_about(g, 1.0 + 0.55 * squash, 1.0 - squash);
    let lean_pivot = Point::new(g.x, g.y - h * 0.45);
    let lean_t = rotate_about(lean_pivot, f(s.lean) * 0.6);
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
    roll_t * facing_t * swing_t * dangle_t * lean_t * squash_t
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
    let gait = Gait::of(s);
    let (sit, sleep, air, dangle) = (f(s.sit), f(s.sleep), f(s.air), f(s.dangle));
    let graze = ease(0.0, 1.0, f(s.graze));
    let rise = f(s.rise);
    let breath = f(s.breath);
    let look = Vec2::new(f(s.look.x), f(s.look.y));
    // Le geste en cours et la posture (humeur, élan, saut d'agneau).
    let act = with_posture(gesturing(s), s);

    // --- Personnage entier.
    let landing = f(s.squash).clamp(-0.15, 0.16);
    let hold = Point::new(rig.pivot(Part::Body).x, rig.bounds.min_y() + h * 0.12);
    let root = root_transform(s, facing, g, h, hold);

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

    // Le corps monte et descend à chaque appui ; au galop, il se balance
    // aussi d'avant en arrière.
    let bob = gait.bob(phase, u) + act.body_shift.y * u;
    let rock = gait.pitch(phase, u, 2.0 * half_length);

    let mut parts = [Affine::IDENTITY; Part::ALL.len()];
    parts[Part::Body.index()] =
        Affine::translate((act.body_shift.x * u, bob + 0.5 * (drop_front + drop_back)))
            * rotate_about(center, pitch + rock + 0.05 * graze + act.body_angle)
            * scale_about(g, 1.0 + 0.04 * sleep, 1.0 - 0.05 * sleep)
            * scale_about(center, 1.0 + act.body_squash, 1.0 - act.body_squash);

    // --- Tête : regard, hochement au pas, sommeil, et broute la tête au sol.
    let head_pivot = rig.pivot(Part::Head);
    let head_bounds = rig.part_bounds(Part::Head);
    let mut head_angle =
        look.y * 0.16 + 0.42 * sleep - 0.06 * air + f(s.lean) * 0.3 + gait.nod(phase)
            - 0.12 * dangle
            - 0.12 * air * ease(0.0, 1.0, rise)
            + 0.1 * air * ease(0.0, 1.0, -rise)
            + act.head_angle;
    let mut head_shift = Vec2::new(
        look.x * u * 1.4 + act.head_shift.x * u + act.body_shift.x * u,
        look.y * u * 1.0
            + bob * 1.15
            + gait.head_bob(phase, u)
            + drop_front
            + sleep * h * 0.06
            + act.head_shift.y * u,
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
    // Vue de profil, la tête qui tourne d'un côté à l'autre paraît plus étroite.
    parts[Part::Head.index()] = Affine::translate(head_shift)
        * rotate_about(head_pivot, head_angle)
        * scale_about(head_bounds.center(), 1.0 - act.head_squash, 1.0);
    // La mâchoire s'ouvre autour de sa charnière (elle suit la tête).
    parts[Part::Jaw.index()] = rotate_about(rig.pivot(Part::Jaw), 0.32 * f(s.jaw));

    // Les oreilles tombent vers le bas (et se dressent vers le haut) de
    // quelque côté qu'elles partent de leur articulation.
    let ear_side = |part: Part| {
        if rig.part_bounds(part).center().x < rig.pivot(part).x {
            -1.0
        } else {
            1.0
        }
    };
    // Pendantes au calme et en mangeant ; asymétriques sous la surprise.
    let ear_droop = 0.3 * sleep + 0.15 * dangle + 0.1 * graze + 0.55 * f(s.ears_droop);
    let ear_pose = act.ear + ears_turn(act.ears_back);
    let split = f(s.ear_split);
    parts[Part::EarNear.index()] = rotate_about(
        rig.pivot(Part::EarNear),
        ear_side(Part::EarNear) * (f(s.ear) + 0.05 * breath + ear_droop + ear_pose + split),
    );
    parts[Part::EarFar.index()] = rotate_about(
        rig.pivot(Part::EarFar),
        ear_side(Part::EarFar)
            * (f(s.ear) * 0.8 + 0.04 * breath + ear_droop * 0.8 + ear_pose * 0.9 - split),
    );

    let closed = (f(s.blink) + act.eyes * (1.0 - f(s.blink))).min(1.0);
    let open = (1.0 - 0.94 * closed).max(0.06);
    parts[Part::Eyes.index()] = scale_about(rig.pivot(Part::Eyes), 1.0, open);
    parts[Part::Pupils.index()] = Affine::translate((look.x * u * 2.6, look.y * u * 2.0));

    // La queue suit l'arrière du corps.
    parts[Part::Tail.index()] = Affine::translate((act.body_shift.x * u, bob + drop_back))
        * rotate_about(rig.pivot(Part::Tail), f(s.tail) - 0.2 * dangle + act.tail);

    // --- Pattes. Le balayage de l'appui couvre exactement la distance que le
    // corps parcourt pendant ce temps : les sabots restent où ils sont posés.
    let stride = if stride > 0.0 { stride } else { h * 0.9 } * stride_scale(s);
    let duty = gait.duty();
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
    let moving = gait.moving();
    let amplitude = (stride * duty / (2.0 * leg_len)).clamp(0.1, 0.75);
    let lift_amount = gait.lift(u);
    // Un agneau qui saute garde les pattes raides.
    let loose = 1.0 - f(s.stiff);
    let tuck = air * ease(0.0, 1.0, rise) * loose;
    let reach = air * ease(0.0, 1.0, -rise) * loose;
    let splay = landing.max(0.0) * 3.0;
    for part in [
        Part::LegFrontNear,
        Part::LegBackFar,
        Part::LegFrontFar,
        Part::LegBackNear,
    ] {
        let front = matches!(part, Part::LegFrontNear | Part::LegFrontFar);
        let near = matches!(part, Part::LegFrontNear | Part::LegBackNear);
        let hip = rig.pivot(part);
        let cycle = phase + leg_offset(part, &gait);
        let (step, raised, support) = stepping(cycle, duty, amplitude, front);
        let mut angle = moving * step;
        // En l'air : pattes repliées en montant, tendues vers le sol en descendant.
        angle += if front {
            -0.15 * air * loose + 1.0 * tuck - 0.3 * reach
        } else {
            0.15 * air * loose + 0.55 * tuck - 0.12 * reach
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
        // Le geste (gratter le sol, taper du pied, s'étirer…).
        let (gesture_angle, gesture_lift) = act.legs[leg_slot(front, near)];
        angle += gesture_angle;
        let lift = raised * lift_amount + gesture_lift * u;
        let drop = if front { drop_front } else { drop_back };
        // En appui, la patte penchée s'allonge juste assez pour rester au sol.
        let planted = moving * support * (1.0 / (moving * step).cos().max(0.5) - 1.0);
        let stretch = 1.0 + planted + 0.07 * dangle + 0.04 * air;
        parts[part.index()] = Affine::translate((act.body_shift.x * u, drop - lift))
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
    fn sheep_walk_in_lateral_sequence_trot_in_diagonals_and_gallop_transversely() {
        let at = |front: bool, near: bool, gait: &Gait| leg_phase(front, near, gait) / TURN;
        // Au pas : l'antérieur du même côté se pose 0,27 de foulée après le
        // postérieur ; l'autre côté, une demi-foulée plus tard.
        let walk = Gait {
            walk: 1.0,
            trot: 0.0,
            run: 0.0,
        };
        assert!((at(true, true, &walk) - at(false, true, &walk) - 0.27).abs() < 1e-9);
        assert!((at(false, false, &walk) - at(false, true, &walk) - 0.5).abs() < 1e-9);
        // Au trot : chaque postérieur avec l'antérieur de l'autre côté, posé
        // un rien avant lui.
        let trot = Gait {
            walk: 1.0,
            trot: 1.0,
            run: 0.0,
        };
        let lag = |a: f64, b: f64| (b - a).rem_euclid(1.0);
        assert!(lag(at(false, true, &trot), at(true, false, &trot)) < 0.05);
        assert!(lag(at(false, false, &trot), at(true, true, &trot)) < 0.05);
        // Au galop transverse : les deux postérieurs, puis les deux
        // antérieurs, dans le même ordre de côté, et une suspension.
        let run = Gait {
            walk: 1.0,
            trot: 0.0,
            run: 1.0,
        };
        let order = [
            at(false, true, &run),
            at(false, false, &run),
            at(true, true, &run),
            at(true, false, &run),
        ];
        assert!(order.windows(2).all(|w| w[0] < w[1]), "{order:?}");
        let last_lift = order[3] + run.duty();
        assert!(order[0] + 1.0 - last_lift > 0.1, "une suspension groupée");
    }

    #[test]
    fn the_walk_rocks_the_body_and_the_trot_bounces_it() {
        let u = 1.0;
        let walk = Gait {
            walk: 1.0,
            trot: 0.0,
            run: 0.0,
        };
        let samples: Vec<f64> = (0..100)
            .map(|i| walk.pitch(TURN * f64::from(i) / 100.0, u, 60.0))
            .collect();
        let swing = samples.iter().copied().fold(f64::MIN, f64::max)
            - samples.iter().copied().fold(f64::MAX, f64::min);
        assert!(swing > 0.01, "la croupe et le garrot montent tour à tour");
        // Au trot, le corps est au plus bas vers le milieu de l'appui d'une
        // paire diagonale, au plus haut entre deux.
        let trot = Gait {
            walk: 1.0,
            trot: 1.0,
            run: 0.0,
        };
        let low = trot.bob(TURN * TROT_LOW, u);
        let high = trot.bob(TURN * (TROT_LOW + 0.25), u);
        assert!(low > high + 2.0, "{low} {high}");
    }

    #[test]
    fn gestures_never_push_a_hoof_into_the_ground() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let leg = rig.part_bounds(Part::LegFrontNear);
        for gesture in mouflon_core::Gesture::ALL {
            for i in 0..30 {
                let signals = PoseSignals {
                    gesture: Some(gesture),
                    gesture_amount: 1.0,
                    gesture_time: i as f32 * 0.1,
                    ..PoseSignals::default()
                };
                let p = pose(&rig, &signals, 1.0, 0.0);
                let foot = p.part(Part::LegFrontNear) * Point::new(leg.center().x, leg.max_y());
                assert!(foot.y <= 96.0 + 0.5, "{gesture:?} {i} : {foot:?}");
            }
        }
    }

    #[test]
    fn the_jaw_opens_downward_with_the_head() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <g id="body"><ellipse cx="50" cy="55" rx="35" ry="22" fill="#999"/></g>
            <g id="head"><circle cx="80" cy="35" r="15" fill="#888"/>
              <g id="jaw"><rect x="80" y="40" width="14" height="6" fill="#777"/>
                <circle id="pivot-jaw" cx="80" cy="42" r="0.5"/></g></g>
            <circle id="pivot-ground" cx="50" cy="96" r="1"/>
        </svg>"##;
        let rig = Rig::parse(svg.as_bytes()).unwrap();
        let tip = Point::new(94.0, 42.0);
        let closed = pose(&rig, &PoseSignals::default(), 1.0, 0.0).part(Part::Jaw) * tip;
        let open = PoseSignals {
            jaw: 1.0,
            ..PoseSignals::default()
        };
        let opened = pose(&rig, &open, 1.0, 0.0).part(Part::Jaw) * tip;
        assert!(opened.y > closed.y + 3.0, "{closed:?} {opened:?}");
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
