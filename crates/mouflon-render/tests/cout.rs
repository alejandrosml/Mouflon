//! Le coût du dessin d'une image : un mouton articulé (encolure, pattes en
//! deux segments, lèvre, mèches) ne doit pas coûter sensiblement plus cher
//! que le même mouton d'une pièce. Mesuré le 3 novembre 2026 (Linux, un
//! cœur) : 3,2 µs par image pour le Mannequin d'une pièce avant le chantier
//! des articulations ; après, 1,8 µs d'une pièce (pivots et morceaux cuits
//! rangés dans des tableaux) et 3,8 µs articulé.
//!
//! `cargo test --release -p mouflon-render --test cout -- --ignored --nocapture`

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mouflon_core::{Appearance, PetView, PoseSignals, Shadow, Support, Vec2, load_pet};
use mouflon_render::{DisplayList, Frame, PetArt, Style, paint_pet};

/// Microsecondes par image (pose et liste des morceaux), sur des poses qui
/// changent à chaque image : au pas, au galop, en broutant.
fn per_frame(art: &PetArt) -> f64 {
    let canvas = art.canvas();
    let frames = 4000;
    let (w, h) = ((canvas.width * 2.0) as u32, (canvas.height * 2.0) as u32);
    let mut list = DisplayList::new(w, h);
    let started = Instant::now();
    for i in 0..frames {
        let t = i as f32 / 120.0;
        let pose = PoseSignals {
            walk: 1.0,
            run: (t * 0.3).sin().max(0.0),
            graze: (t * 0.2).cos().max(0.0),
            gait_phase: t * 7.0,
            look: Vec2::new(0.3, 0.1),
            fleece: 0.1 * t.sin(),
            ..PoseSignals::default()
        };
        let view = PetView {
            id: 0,
            ground: canvas.anchor,
            facing: 1.0,
            pose,
            shadow: Some(Shadow::on(canvas.anchor.y, 0.0)),
            sprite_frame: None,
            sprite_offset_y: 0.0,
            opacity: 1.0,
            particles: &[],
            support: Support::Air,
            behavior: "",
            grip: None,
        };
        list.clear(w, h);
        paint_pet(
            art,
            &view,
            &Frame {
                origin: Vec2::ZERO,
                scale: 2.0,
            },
            &Style::default(),
            &mut list,
        );
    }
    started.elapsed().as_secs_f64() * 1e6 / f64::from(frames)
}

#[test]
#[ignore = "mesure : à lancer à la main, en release"]
fn drawing_a_jointed_sheep_costs_about_as_much_as_before() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mannequin/pet.toml");
    let def = load_pet(&path).expect("pet.toml du mannequin");
    let jointed = PetArt::load(&def).expect("mannequin articulé");
    let mut before = def.clone();
    let svg = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/donnees/mannequin-avant.svg"),
    )
    .unwrap();
    let Appearance::Rig { height, .. } = def.appearance else {
        panic!("le mannequin est un dessin vectoriel");
    };
    before.appearance = Appearance::Rig {
        svg: Arc::from(svg),
        height,
    };
    let whole = PetArt::load(&before).expect("mannequin d'avant");
    // Chauffe, puis trois mesures de chaque : la meilleure.
    per_frame(&whole);
    per_frame(&jointed);
    let best = |art: &PetArt| (0..3).map(|_| per_frame(art)).fold(f64::MAX, f64::min);
    let (a, b) = (best(&whole), best(&jointed));
    println!("d'une pièce : {a:.1} µs par image ; articulé : {b:.1} µs par image");
    assert!(b < a * 2.5, "{a:.1} µs puis {b:.1} µs");
}
