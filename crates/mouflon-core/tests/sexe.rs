//! L'identité biologique est déterministe et ne décale pas les tirages des
//! mouvements indépendants du sexe.

use std::sync::Arc;

use mouflon_core::{
    Age, Input, Pet, PetDef, Rect, Screen, Sex, SexRatio, TICK, World, parse_pet_toml,
};

fn definition(identity: &str) -> Result<PetDef, mouflon_core::LoadError> {
    parse_pet_toml(
        &format!(
            r#"
[pet]
id = "sheep"
name = "Mouton"
category = "sheep"
{identity}
[appearance]
rig = "mouton.svg"
height = 50.0
[[behavior]]
name = "stand"
motion = "stand"
duration = [0.5, 1.5]
next = "walk"
[[behavior]]
name = "walk"
motion = "walk"
duration = [0.5, 1.5]
next = "stand"
"#
        ),
        |_: &str| Ok(b"<svg/>".to_vec()),
    )
}

fn world() -> World {
    World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
        usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
        scale: 2.0,
    }])
}

#[test]
fn an_author_can_fix_or_weight_the_sex_and_declare_a_lambs_age() {
    let adult = definition("").unwrap();
    assert_eq!(adult.sex_ratio.male_probability(), 0.5);
    assert_eq!(adult.age, Age::Adult);
    assert_eq!(
        definition("sex = \"male\"").unwrap().sex_ratio,
        SexRatio::MALE
    );
    assert_eq!(
        definition("sex = \"female\"").unwrap().sex_ratio,
        SexRatio::FEMALE
    );
    let lamb = definition("sex = { male = 0.3 }\nage = \"lamb\"\nage_weeks = 6.5").unwrap();
    assert_eq!(lamb.sex_ratio.male_probability(), 0.3);
    assert_eq!(lamb.age, Age::Lamb { weeks: 6.5 });
    assert_eq!(
        definition("age = \"lamb\"").unwrap().age,
        Age::Lamb { weeks: 2.0 }
    );
}

#[test]
fn invalid_sex_ratios_and_ages_are_rejected_instead_of_silently_clamped() {
    for identity in [
        "sex = \"ram\"",
        "sex = { male = -0.1 }",
        "sex = { male = 1.1 }",
        "sex = { male = nan }",
        "sex = { male = inf }",
        "sex = { male = 0.5, typo = 1 }",
        "age = \"old\"",
        "age_weeks = 2.0",
        "age = \"adult\"\nage_weeks = 2.0",
        "age = \"lamb\"\nage_weeks = 0.0",
        "age = \"lamb\"\nage_weeks = -1.0",
        "age = \"lamb\"\nage_weeks = nan",
        "age = \"lamb\"\nage_weeks = inf",
        "age = \"lamb\"\nage_weeks = 53.0",
    ] {
        assert!(definition(identity).is_err(), "{identity}");
    }
    for ratio in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.1, 1.1] {
        assert!(SexRatio::new(ratio).is_err());
    }
}

#[test]
fn sex_is_reproducible_and_follows_the_authors_distribution() {
    let world = world();
    let def = Arc::new(definition("sex = { male = 0.3 }").unwrap());
    let mut males = 0;
    for id in 0..4096 {
        let a = Pet::spawn(id, Arc::clone(&def), 0x1234, &world, Some(600.0));
        let b = Pet::spawn(id, Arc::clone(&def), 0x1234, &world, Some(600.0));
        assert_eq!(a.sex(), b.sex());
        males += usize::from(a.sex() == Sex::Male);
    }
    assert!((1100..1400).contains(&males), "{males} mâles sur 4096");
}

#[test]
fn choosing_a_sex_does_not_shift_unrelated_motion_or_animation_randomness() {
    let world = world();
    let mut male = Pet::spawn(
        7,
        Arc::new(definition("sex = \"male\"").unwrap()),
        5,
        &world,
        Some(600.0),
    );
    let mut female = Pet::spawn(
        7,
        Arc::new(definition("sex = \"female\"").unwrap()),
        5,
        &world,
        Some(600.0),
    );
    assert_eq!(male.sex(), Sex::Male);
    assert_eq!(female.sex(), Sex::Female);
    for _ in 0..2400 {
        male.step(TICK, &world, &Input::default());
        female.step(TICK, &world, &Input::default());
        assert_eq!(male.position(), female.position());
        assert_eq!(male.velocity(), female.velocity());
        assert_eq!(male.behavior_name(), female.behavior_name());
        assert_eq!(male.view(1.0).pose, female.view(1.0).pose);
    }
}

#[test]
fn resizing_preserves_sex_and_does_not_turn_a_lamb_into_an_adult() {
    let world = world();
    let def = Arc::new(definition("age = \"lamb\"\nage_weeks = 2.5").unwrap());
    let mut pet = Pet::spawn(8, Arc::clone(&def), 17, &world, Some(600.0));
    let sex = pet.sex();
    assert!(pet.rescale(Arc::new(def.scaled(2.0)), 2.0));
    assert_eq!(pet.sex(), sex);
    assert_eq!(pet.age(), Age::Lamb { weeks: 2.5 });
}
