//! La cuisson garde les traits complets, même lorsque leur transformation
//! SVG étire beaucoup un axe ou que leur jointure forme une pointe longue.

use mouflon_render::DisplayList;
use mouflon_render::bake::BakedRig;
use mouflon_render::display::Command;
use mouflon_render::raster::rasterize;
use mouflon_render::rig::{Item, Rig};

fn assert_baking_preserves_strokes(svg: &str) {
    let rig = Rig::parse(svg.as_bytes()).expect("dessin d'essai");
    // 0,4 point par unité donne exactement un pixel cuit par unité : aucun
    // redimensionnement ne brouille la comparaison des contours.
    let baked = BakedRig::bake(&rig, 0.4, 0.0).expect("cuisson");
    let mut direct = DisplayList::new(200, 160);
    let mut images = DisplayList::new(200, 160);
    for (index, item) in rig.items.iter().enumerate() {
        if let Item::Stroke {
            path,
            transform,
            paint,
            brush_transform,
            style,
        } = item
        {
            direct.push(Command::Stroke {
                path: path.clone(),
                transform: *transform,
                paint: paint.clone(),
                brush_transform: *brush_transform,
                style: style.clone(),
            });
        }
        if let Some(piece) = baked.piece(index) {
            images.push(Command::Image {
                bitmap: piece.bitmap.clone(),
                transform: piece.placement,
                alpha: 1.0,
                smooth: false,
            });
        }
    }
    let direct = rasterize(&direct).expect("traits");
    let images = rasterize(&images).expect("images cuites");
    let mut opaque = 0;
    for (index, (original, baked)) in direct.pixels().iter().zip(images.pixels()).enumerate() {
        if original.alpha() > 10 {
            opaque += 1;
            assert!(
                baked.alpha().abs_diff(original.alpha()) <= 2,
                "trait coupé en ({}, {}) : alpha {} au lieu de {}",
                index % 200,
                index / 200,
                baked.alpha(),
                original.alpha()
            );
        }
    }
    assert!(opaque > 100, "le tracé de référence doit être visible");
}

#[test]
fn baking_keeps_the_full_width_of_a_non_uniformly_scaled_stroke() {
    assert_baking_preserves_strokes(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="160">
        <path d="M10 20 V100" transform="translate(40 40) scale(10 .1)"
              fill="none" stroke="#222" stroke-width="4" stroke-linecap="round"/>
        </svg>"##,
    );
}

#[test]
fn baking_keeps_a_long_miter_join_inside_the_image() {
    assert_baking_preserves_strokes(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="160">
        <path d="M50 100 L60 20 L70 100" fill="none" stroke="#222"
              stroke-width="4" stroke-linejoin="miter" stroke-miterlimit="20"/>
        </svg>"##,
    );
}
