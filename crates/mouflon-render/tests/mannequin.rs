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
    let mut art = PetArt::load(&def).expect("dessin du mannequin");
    assert!(art.width > 0.0 && art.height > 0.0);
    let baked = art.baked.clone().expect("le mannequin se cuit en images");
    assert!(baked.len() >= 10, "{} morceaux", baked.len());

    let world = World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1280.0, 800.0),
        usable: Rect::new(0.0, 25.0, 1280.0, 700.0),
        scale: 2.0,
    }]);
    let mut pet = Pet::spawn(0, Arc::clone(&def), 11, &world, Some(600.0));
    let input = Input {
        cursor: Some(Vec2::new(640.0, 600.0)),
        local_hour: 23.0,
        ..Input::default()
    };
    for _ in 0..(20.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }

    let canvas = art.canvas();
    let view = pet.view(0.5);
    let origin = view.ground - canvas.anchor;
    let paint = |art: &PetArt| {
        let mut list = DisplayList::new((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
        paint_pet(
            art,
            &view,
            &Frame { origin, scale: 2.0 },
            &Style::default(),
            &mut list,
        );
        let count = |f: fn(&Command) -> bool| list.commands.iter().filter(|c| f(c)).count();
        let fills = count(|c| matches!(c, Command::Fill { .. }));
        let images = count(|c| matches!(c, Command::Image { .. }));
        (list, fills, images)
    };
    // Cuit : ses morceaux sont des images, sans un seul tracé.
    let (_, fills, images) = paint(&art);
    assert_eq!(fills, 0);
    assert!(images >= 10, "seulement {images} images posées");
    // Tel quel : ses tracés.
    art.baked = None;
    let (list, fills, _) = paint(&art);
    assert!(fills > 20, "seulement {fills} formes dessinées");

    let svg = mouflon_render::svg::to_svg(&list);
    assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"));
}

/// Au pas, les sabots posés ne glissent presque pas : la patte balaie juste
/// ce que le corps parcourt pendant l'appui.
#[test]
fn mannequin_hooves_stay_planted_while_walking() {
    use mouflon_render::animate::pose;
    use mouflon_render::rig::Part;
    use mouflon_render::vello::kurbo::{Affine, Point, Vec2 as KVec2};

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mannequin/pet.toml");
    let def = Arc::new(load_pet(&path).expect("pet.toml du mannequin"));
    let art = PetArt::load(&def).expect("dessin du mannequin");
    let mouflon_render::Look::Rig(rig) = &art.look else {
        panic!("le mannequin est un dessin vectoriel")
    };
    let world = World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 2000.0, 600.0),
        usable: Rect::new(0.0, 0.0, 2000.0, 560.0),
        scale: 2.0,
    }]);
    let mut pet = Pet::spawn(0, Arc::clone(&def), 3, &world, Some(1000.0));
    let input = Input::default();
    for _ in 0..(2.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }
    assert!(pet.force("walk"));
    for _ in 0..(1.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }
    let k = f64::from(art.height) / rig.bounds.height();
    let stride = f64::from(art.stride) / k;
    let mut previous: Option<Point> = None;
    let (mut planted, mut slide) = (0usize, 0.0f64);
    for _ in 0..(2.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
        if pet.behavior_name() != "walk" {
            pet.force("walk");
        }
        let view = pet.view(1.0);
        let p = pose(rig, &view.pose, view.facing, stride);
        let base = Affine::translate(KVec2::new(
            f64::from(view.ground.x),
            f64::from(view.ground.y),
        )) * Affine::scale(k)
            * Affine::translate(-rig.ground.to_vec2())
            * p.root;
        let b = rig.part_bounds(Part::LegFrontNear);
        let foot = base * p.part(Part::LegFrontNear) * Point::new(b.center().x, b.max_y());
        let ground = f64::from(view.ground.y);
        if let Some(before) = previous
            && foot.y > ground - 0.3
            && before.y > ground - 0.3
        {
            planted += 1;
            slide += (foot.x - before.x).abs() / f64::from(TICK);
        }
        previous = Some(foot);
    }
    let speed = f64::from(pet.velocity().x.abs());
    assert!(planted > 60, "sabot au sol pendant {planted} pas");
    let mean = slide / planted as f64;
    assert!(
        mean < 0.15 * speed,
        "glisse à {mean:.1} pt/s pour {speed:.0} pt/s"
    );
}

/// Le mannequin broute : la tête descend en une seconde ou deux et grignote.
#[test]
fn mannequin_grazes_with_its_head_down() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mannequin/pet.toml");
    let def = Arc::new(load_pet(&path).expect("pet.toml du mannequin"));
    let world = World::new(vec![Screen {
        frame: Rect::new(0.0, 0.0, 1280.0, 800.0),
        usable: Rect::new(0.0, 25.0, 1280.0, 700.0),
        scale: 2.0,
    }]);
    let mut pet = Pet::spawn(0, Arc::clone(&def), 5, &world, Some(600.0));
    let input = Input::default();
    for _ in 0..(3.0 / TICK) as usize {
        pet.step(TICK, &world, &input);
    }
    assert!(pet.force("graze"), "le mannequin sait brouter");
    let mut bites = Vec::new();
    for _ in 0..(2.5 / TICK) as usize {
        pet.step(TICK, &world, &input);
        bites.push(pet.view(1.0).pose.nibble);
    }
    let view = pet.view(1.0);
    assert!(view.pose.graze > 0.9, "tête basse : {}", view.pose.graze);
    let (lo, hi) = bites[bites.len() / 2..]
        .iter()
        .fold((f32::MAX, f32::MIN), |(lo, hi), &b| (lo.min(b), hi.max(b)));
    assert!(hi - lo > 1.0, "il grignote ({lo} à {hi})");
}
