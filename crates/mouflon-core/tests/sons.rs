//! Le pack du mouflon charge ses appels et les transmet au lecteur sans
//! ouvrir la mâchoire pour un souffle nasal.

use std::path::Path;
use std::sync::Arc;

use mouflon_core::{Input, Pet, PetEvent, Rect, Screen, TICK, World, load_pet};

fn world() -> World {
    World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
        usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
        scale: 2.0,
    }])
}

#[test]
fn the_mouflons_pack_loads_its_own_alarm_and_courtship_calls() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mouflon-corse");
    let def = load_pet(&root.join("pet.toml")).expect("le pack se charge");
    for (behavior, name, file) in [
        ("stamp", "alarme", "mouflon-alarme.wav"),
        ("flehmen", "grondement", "mouflon-grondement.wav"),
    ] {
        let index = def
            .sounds
            .iter()
            .position(|sound| sound.name == name)
            .unwrap();
        let sound = &def.sounds[index];
        assert!(!sound.mouth, "le souffle n'ouvre pas la bouche");
        assert_eq!(sound.clips.len(), 1);
        assert_eq!(
            sound.clips[0].as_ref(),
            std::fs::read(root.join("sons").join(file)).unwrap()
        );
        let cues = &def.behaviors[def.behavior_index(behavior).unwrap()].sounds;
        assert_eq!(cues.len(), 1);
        assert_eq!(cues[0].sound, index);
        assert_eq!(cues[0].frame, None);
        assert_eq!(cues[0].chance, 1.0);
    }
}

#[test]
fn stamping_emits_one_alarm_without_opening_the_mouflons_mouth() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mouflon-corse/pet.toml");
    let def = Arc::new(load_pet(&path).unwrap());
    let alarm = def
        .sounds
        .iter()
        .position(|sound| sound.name == "alarme")
        .unwrap();
    let world = world();
    let mut pet = Pet::spawn(0, def, 3, &world, Some(600.0));
    for _ in 0..240 {
        pet.step(TICK, &world, &Input::default());
    }
    // Un éventuel son d'atterrissage a fini : seul le chuintement ci-dessous
    // peut maintenant rouvrir la mâchoire.
    assert!(pet.force("stand"));
    for _ in 0..120 {
        pet.step(TICK, &world, &Input::default());
    }
    pet.take_events();
    assert!(pet.force("stamp"));
    let sounds: Vec<_> = pet
        .take_events()
        .into_iter()
        .filter_map(|event| match event {
            PetEvent::Sound { sound, clip } => Some((sound, clip)),
            _ => None,
        })
        .collect();
    assert_eq!(sounds, [(alarm, 0)]);
    for _ in 0..40 {
        pet.step(TICK, &world, &Input::default());
        assert!(pet.view(1.0).pose.jaw < 0.05);
        assert!(
            !pet.take_events()
                .iter()
                .any(|event| matches!(event, PetEvent::Sound { .. }))
        );
    }
}
