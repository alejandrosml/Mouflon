//! Animation procédurale : traduit les signaux de pose du moteur
//! (démarche, écrasement, regard…) en transformations des parties du rig.
//!
//! Toutes les transformations sont exprimées dans le repère du document SVG
//! et pensées pour un personnage tourné vers la droite ; le retournement est
//! appliqué en dernier.

use std::f64::consts::PI;

use mouflon_core::PoseSignals;
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

/// Calcule la pose d'une image.
#[must_use]
pub fn pose(rig: &Rig, s: &PoseSignals, facing: f32) -> PoseTransforms {
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
    let breath = f(s.breath);
    let look = Vec2::new(f(s.look.x), f(s.look.y));

    // --- Personnage entier.
    let squash = f(s.squash) + breath * (0.008 * (1.0 - sleep) + 0.022 * sleep);
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

    // --- Parties.
    let mut parts = [Affine::IDENTITY; Part::ALL.len()];

    let leg_len = {
        let b = rig.part_bounds(Part::LegFrontNear);
        if rig.has_part(Part::LegFrontNear) {
            (g.y - b.min_y()).max(u * 8.0)
        } else {
            h * 0.3
        }
    };
    let fold = sit * 0.42;
    let drop = leg_len * fold;
    let bob = -(walk * 1.3 + run * 2.6) * u * phase.sin().abs();

    parts[Part::Body.index()] = Affine::translate((0.0, bob + drop))
        * scale_about(g, 1.0 + 0.04 * sleep, 1.0 - 0.05 * sleep);

    let head_angle = look.y * 0.16 + 0.34 * sleep - 0.06 * air
        + f(s.lean) * 0.3
        + (phase * 2.0).sin() * 0.035 * walk
        - 0.12 * dangle;
    parts[Part::Head.index()] = Affine::translate((
        look.x * u * 1.4,
        look.y * u * 1.0 + bob * 1.15 + drop * 1.1 + sleep * h * 0.05,
    )) * rotate_about(rig.pivot(Part::Head), head_angle);

    parts[Part::EarNear.index()] = rotate_about(
        rig.pivot(Part::EarNear),
        f(s.ear) + 0.05 * breath + 0.3 * sleep + 0.15 * dangle,
    );
    parts[Part::EarFar.index()] = rotate_about(
        rig.pivot(Part::EarFar),
        f(s.ear) * 0.8 + 0.04 * breath + 0.24 * sleep + 0.12 * dangle,
    );

    let open = (1.0 - 0.94 * f(s.blink)).max(0.06);
    parts[Part::Eyes.index()] = scale_about(rig.pivot(Part::Eyes), 1.0, open);
    parts[Part::Pupils.index()] = Affine::translate((look.x * u * 2.6, look.y * u * 2.0));

    parts[Part::Tail.index()] = rotate_about(rig.pivot(Part::Tail), f(s.tail) - 0.2 * dangle);

    // Trot : les pattes en diagonale vont ensemble.
    let legs = [
        (Part::LegFrontNear, 0.0, true),
        (Part::LegBackFar, 0.0, false),
        (Part::LegFrontFar, PI, true),
        (Part::LegBackNear, PI, false),
    ];
    let amplitude = walk * 0.42 + run * 0.38;
    let lift_amount = (walk * 3.2 + run * 6.0) * u;
    for (part, offset, front) in legs {
        let hip = rig.pivot(part);
        let b = rig.part_bounds(part);
        let foot = Point::new(b.center().x, b.max_y());
        let cycle = phase + offset;
        let mut angle = -amplitude * cycle.sin();
        angle += air * if front { -0.5 } else { 0.42 };
        angle += dangle * (if front { -0.1 } else { 0.12 } + f(s.swing) * 0.4);
        let lift = cycle.cos().max(0.0) * lift_amount;
        let stretch = 1.0 + 0.07 * dangle + 0.04 * air;
        parts[part.index()] = scale_about(foot, 1.0, 1.0 - fold)
            * Affine::translate((0.0, -lift))
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
        let p = pose(&rig, &PoseSignals::default(), 1.0);
        let moved = p.root * rig.ground;
        assert!((moved - rig.ground).hypot() < 1e-6);
    }

    #[test]
    fn facing_left_mirrors_around_ground() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let p = pose(&rig, &PoseSignals::default(), -1.0);
        let head = Point::new(80.0, 35.0);
        let mirrored = p.root * head;
        assert!((mirrored.x - (2.0 * rig.ground.x - head.x)).abs() < 1e-6);
    }

    #[test]
    fn landing_squash_flattens() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let signals = PoseSignals {
            squash: 0.3,
            ..PoseSignals::default()
        };
        let p = pose(&rig, &signals, 1.0);
        let top = p.root * Point::new(50.0, 33.0);
        assert!(top.y > 33.0, "le haut du corps doit descendre");
    }
}
