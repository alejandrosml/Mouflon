//! Test de bout en bout : le personnage de test se charge, se simule et se
//! dessine sans erreur, et la scène produite n'est pas vide.

use std::path::Path;
use std::sync::Arc;

use mouflon_core::{Input, Pet, Rect, Screen, TICK, Vec2, World, load_pet};
use mouflon_render::display::Command;
use mouflon_render::{DisplayList, Frame, PetArt, Style, paint_pet};

#[test]
fn mannequin_loads_simulates_and_paints() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mannequin/pet.toml");
    let def = Arc::new(load_pet(&path).expect("pet.toml du mannequin"));
    let art = PetArt::load(&def).expect("dessin du mannequin");
    assert!(art.width > 0.0 && art.height > 0.0);

    let world = World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1280.0, 800.0),
        usable: Rect::new(0.0, 25.0, 1280.0, 700.0),
        scale: 2.0,
    }]);
    let mut pet = Pet::spawn(0, Arc::clone(&def), 11, &world, Some(600.0));
    let input = Input {
        cursor: Some(Vec2::new(640.0, 600.0)),
        local_hour: 23.0,
    };
    for _ in 0..(20.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }

    let canvas = art.canvas();
    let view = pet.view(0.5);
    let origin = view.ground - canvas.anchor;
    let mut list = DisplayList::new((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
    paint_pet(
        &art,
        &view,
        &Frame { origin, scale: 2.0 },
        &Style::default(),
        &mut list,
    );
    let fills = list
        .commands
        .iter()
        .filter(|c| matches!(c, Command::Fill { .. }))
        .count();
    assert!(fills > 20, "seulement {fills} formes dessinées");

    let svg = mouflon_render::svg::to_svg(&list);
    assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"));
}
