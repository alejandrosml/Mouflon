//! Lainou, livré avec Mouflon : ses bandes d'images se chargent, se rognent,
//! se réduisent à la finesse de l'écran et se dessinent dans les deux sens.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mouflon_core::{Input, Pet, Rect, Screen, Support, TICK, Vec2, World, load_pet};
use mouflon_render::display::Command;
use mouflon_render::{DisplayList, Frame, Look, PetArt, Style, paint_pet};

fn world() -> World {
    World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1280.0, 800.0),
        usable: Rect::new(0.0, 25.0, 1280.0, 700.0),
        scale: 2.0,
    }])
}

#[test]
fn lainou_loads_and_paints_both_ways() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/lainou/pet.toml");
    let def = Arc::new(load_pet(&path).expect("pet.toml de Lainou"));
    let started = Instant::now();
    let art = PetArt::load(&def).expect("images de Lainou");
    eprintln!("Lainou décodé en {:?}", started.elapsed());
    let Look::Sprites(frames) = &art.look else {
        panic!("Lainou est fait d'images")
    };
    // 245 images par sens, 256 px réduits à 192 (96 points sur un écran Retina).
    assert_eq!(frames.frames.len(), 490);
    assert_eq!((art.width, art.height), (96.0, 96.0));
    let bytes: usize = frames
        .frames
        .iter()
        .map(|f| f.bitmap.rgba.data().len())
        .sum();
    eprintln!("{:.1} Mo d'images en mémoire", bytes as f64 / 1e6);
    assert!(bytes < 40_000_000, "{bytes} octets");
    assert!(frames.frames.iter().all(|f| f.bitmap.width <= 192));

    let world = world();
    let mut pet = Pet::spawn(0, Arc::clone(&def), 5, &world, Some(600.0));
    let input = Input::default();
    for _ in 0..(4.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }
    assert!(matches!(pet.support(), Support::Ground { .. }));
    for (facing, behavior) in [(-1.0, "marche"), (1.0, "marche"), (1.0, "dort")] {
        assert!(pet.force(behavior));
        let mut view = pet.view(1.0);
        view.facing = facing;
        let canvas = art.canvas();
        let origin = view.ground - canvas.anchor;
        let mut list = DisplayList::new((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
        paint_pet(
            &art,
            &view,
            &Frame { origin, scale: 2.0 },
            &Style::default(),
            &mut list,
        );
        let images: Vec<_> = list
            .commands
            .iter()
            .filter_map(|c| match c {
                Command::Image {
                    transform, smooth, ..
                } => Some((*transform, *smooth)),
                _ => None,
            })
            .collect();
        assert_eq!(images.len(), 1, "{behavior}");
        let (transform, smooth) = images[0];
        // Au repos, un pixel d'image par pixel d'écran Retina : net, jamais retourné.
        let [a, _, _, d, x, y] = transform.as_coeffs();
        assert!((a - 1.0).abs() < 1e-6 && (d - 1.0).abs() < 1e-6, "{a} {d}");
        assert!(!smooth);
        assert_eq!((x.fract(), y.fract()), (0.0, 0.0), "calé sur les pixels");
        // Pas d'ombre ajoutée au sol : elle est dans les images.
        assert!(
            !list
                .commands
                .iter()
                .any(|c| matches!(c, Command::BlurredRect { .. }))
        );
    }

    // Tenu par le curseur : pris par le dos et emporté plus haut, il reste
    // pris au même endroit, avec l'ombre de Mouflon au sol.
    let caught = pet.position() + Vec2::new(12.0, -40.0);
    pet.grab(caught, &world);
    let cursor = Vec2::new(640.0, 600.0);
    for _ in 0..(3.0 / TICK) as usize {
        pet.drag_to(cursor);
        pet.step(TICK, &world, &input);
    }
    let view = pet.view(1.0);
    let held = view.ground + Vec2::new(12.0, -40.0);
    assert!(
        (held - cursor).length() < 1e-3,
        "{:?} {:?} {:?} {}",
        view.ground,
        held,
        view.support,
        view.behavior
    );
    assert_eq!(view.grip, Some(cursor));
    let canvas = art.canvas();
    let origin = view.ground - canvas.anchor;
    let mut list = DisplayList::new((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
    paint_pet(
        &art,
        &view,
        &Frame { origin, scale: 2.0 },
        &Style::default(),
        &mut list,
    );
    assert!(
        list.commands
            .iter()
            .any(|c| matches!(c, Command::BlurredRect { .. })),
        "ombre portée en l'air"
    );
}
