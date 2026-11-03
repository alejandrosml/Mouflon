//! Les petites touffes d'encolure suivent leur racine, sans se coucher avec
//! le cou quand le Nez noir broute. Elles restent des morceaux d'image.

use mouflon_core::{Gesture, PoseSignals};
use mouflon_render::animate::{PoseTransforms, pose};
use mouflon_render::bake::BakedRig;
use mouflon_render::kurbo::{Affine, Point, Vec2};
use mouflon_render::rig::{Part, Rig};

fn blacknose() -> Rig {
    Rig::parse(include_bytes!("../../../pets/nez-noir/nez-noir.svg")).unwrap()
}

fn absolute(rig: &Rig, pose: &PoseTransforms, part: Part) -> Affine {
    let mut transform = pose.part(part);
    let mut parent = rig.parent(part);
    while let Some(part) = parent {
        transform = pose.part(part) * transform;
        parent = rig.parent(part);
    }
    pose.root * transform
}

#[test]
fn neck_locks_keep_their_roots_on_the_neck_and_hang_through_the_whole_grazing_pose() {
    let rig = blacknose();
    for facing in [-1.0, 1.0] {
        for step in 0..=100 {
            let signals = PoseSignals {
                graze: step as f32 / 100.0,
                ..PoseSignals::default()
            };
            let posed = pose(&rig, &signals, facing, 0.0);
            let neck = absolute(&rig, &posed, Part::Neck);
            for part in Part::NECK_SWAYS {
                assert!(rig.has_part(part));
                assert_eq!(rig.parent(part), Some(Part::Neck));
                let pivot = rig.pivot(part);
                let tuft = absolute(&rig, &posed, part);
                assert!((tuft * pivot - neck * pivot).hypot() < 1e-9);
                let down = tuft * (pivot + Vec2::new(0.0, 8.0)) - tuft * pivot;
                assert!(down.x.abs() < 1e-9, "{part:?}, phase {step}: {down:?}");
                assert!((down.y - 8.0).abs() < 1e-9);
                // À 90° de cou, la touffe ne s'aplatit pas, et au-delà
                // elle ne se retourne pas. Le miroir reste celui de l'animal.
                assert!((tuft.determinant().abs() - 1.0).abs() < 1e-9);
            }
        }
    }
}

#[test]
fn neck_locks_still_sway_in_the_wind_and_during_a_shake_without_moving_their_roots() {
    let rig = blacknose();
    let rest = PoseSignals {
        graze: 0.6,
        ..PoseSignals::default()
    };
    let calm = pose(&rig, &rest, 1.0, 0.0);
    for signals in [
        PoseSignals {
            fleece: 0.25,
            fleece_tip: 0.15,
            ..rest
        },
        PoseSignals {
            gesture: Some(Gesture::Shake),
            gesture_amount: 1.0,
            gesture_time: 0.2,
            ..rest
        },
    ] {
        let posed = pose(&rig, &signals, 1.0, 0.0);
        for part in Part::NECK_SWAYS {
            let pivot = rig.pivot(part);
            let tuft = absolute(&rig, &posed, part);
            let neck = absolute(&rig, &posed, Part::Neck);
            assert!((tuft * pivot - neck * pivot).hypot() < 1e-9);
            let tip = pivot + Vec2::new(0.0, 8.0);
            let moving = tuft * tip - tuft * pivot;
            let steady = absolute(&rig, &calm, part) * tip - absolute(&rig, &calm, part) * pivot;
            assert!((moving - steady).hypot() > 0.01, "{part:?}");
        }
    }
}

#[test]
fn neck_locks_bake_without_changing_the_existing_belly_locks() {
    let rig = blacknose();
    let baked = BakedRig::bake(&rig, 0.5, 0.6).expect("la toison se cuit");
    assert!(baked.len() <= 32, "{} morceaux", baked.len());
    for part in Part::SWAYS {
        assert!(rig.has_part(part));
        assert_eq!(part.contract_parent(true), Some(Part::Body));
    }
}

#[test]
fn a_neck_lock_placed_beside_the_neck_keeps_the_same_absolute_pose() {
    const LOCK: &str = r##"<g id="neck-sway-1"><path d="M77 43L82 43L80 58Z" fill="#ddd"/>
        <circle id="pivot-neck-sway-1" cx="79" cy="44" r="0.5"/></g>"##;
    let drawing = |lock_in_neck: bool| {
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 100">
            <g id="body"><ellipse cx="48" cy="55" rx="30" ry="23" fill="#eee"/></g>
            <g id="neck"><rect x="70" y="30" width="14" height="26" fill="#ccc"/>{}
                <circle id="pivot-neck" cx="74" cy="55" r="0.5"/></g>{}
            <g id="head"><ellipse cx="92" cy="35" rx="14" ry="9" fill="#888"/>
                <circle id="pivot-head" cx="80" cy="31" r="0.5"/>
                <circle id="pivot-muzzle" cx="105" cy="41" r="0.5"/></g>
            <circle id="pivot-ground" cx="50" cy="96" r="0.5"/></svg>"##,
            if lock_in_neck { LOCK } else { "" },
            if lock_in_neck { "" } else { LOCK },
        )
    };
    let nested = Rig::parse(drawing(true).as_bytes()).unwrap();
    let beside = Rig::parse(drawing(false).as_bytes()).unwrap();
    for step in 0..=10 {
        let signals = PoseSignals {
            graze: step as f32 / 10.0,
            fleece: 0.2,
            ..PoseSignals::default()
        };
        let one = pose(&nested, &signals, 1.0, 0.0);
        let two = pose(&beside, &signals, 1.0, 0.0);
        for point in [Point::new(79.0, 44.0), Point::new(80.0, 58.0)] {
            let a = absolute(&nested, &one, Part::NeckSway1) * point;
            let b = absolute(&beside, &two, Part::NeckSway1) * point;
            assert!((a - b).hypot() < 1e-8, "phase {step}: {a:?}, {b:?}");
        }
    }
}
