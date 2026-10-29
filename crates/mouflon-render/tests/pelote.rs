//! Pelote, livré avec Mouflon : un animal desktopPet redessiné, dont le
//! fichier XML a perdu son image au profit d'une planche de 240 pixels.

use std::path::Path;
use std::sync::Arc;

use mouflon_core::{Input, Pet, Rect, Screen, Support, TICK, World, load_pet};
use mouflon_render::display::Command;
use mouflon_render::{DisplayList, Frame, Look, PetArt, Style, paint_pet};

#[test]
fn pelote_loads_at_80_points_and_paints_both_ways() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/pelote/pet.toml");
    let def = Arc::new(load_pet(&path).expect("pet.toml de Pelote"));
    assert_eq!(def.meta.name, "Pelote");
    let art = PetArt::load(&def).expect("planche de Pelote");
    let Look::Sprites(frames) = &art.look else {
        panic!("Pelote est fait d'images")
    };
    // 12 × 10 cases de 240 pixels, affichées en 80 points (160 pixels Retina).
    assert_eq!((art.width, art.height), (80.0, 80.0));
    assert!(frames.frames.len() >= 119, "{}", frames.frames.len());
    assert!(frames.frames.iter().all(|f| f.bitmap.width <= 160));
    // Ses images intermédiaires suivent la planche : la marche (images 1 et
    // 0) en a trois dans chaque sens.
    let sheet = def.sprites().expect("planche");
    let (first, count) = sheet.inbetween(1, 0).expect("marche adoucie");
    assert_eq!(count, 3);
    assert!(first >= 120 && (first + count) as usize <= frames.frames.len());

    let world = World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1280.0, 800.0),
        usable: Rect::new(0.0, 25.0, 1280.0, 700.0),
        scale: 2.0,
    }]);
    let mut pet = Pet::spawn(0, Arc::clone(&def), 5, &world, Some(600.0));
    let input = Input::default();
    for _ in 0..(6.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }
    assert!(matches!(pet.support(), Support::Ground { .. }));
    for facing in [-1.0, 1.0] {
        assert!(pet.force("walk"));
        let mut view = pet.view(1.0);
        view.facing = facing;
        let canvas = art.canvas();
        let mut list = DisplayList::new((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
        paint_pet(
            &art,
            &view,
            &Frame {
                origin: view.ground - canvas.anchor,
                scale: 2.0,
            },
            &Style::default(),
            &mut list,
        );
        let images = list
            .commands
            .iter()
            .filter(|c| matches!(c, Command::Image { .. }))
            .count();
        assert_eq!(images, 1, "une image par animal");
    }
}
