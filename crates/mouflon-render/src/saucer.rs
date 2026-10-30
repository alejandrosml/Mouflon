//! La soucoupe volante des surprises (voir [`mouflon_core::Saucer`]) :
//! dessinée par Mouflon, sans image. Un corps argenté et lavande, un dôme de
//! verre, des lumières qui clignotent autour, et, sous elle, le faisceau qui
//! aspire un animal.

use mouflon_core::{BEAM_MAX, SAUCER_HEIGHT, SAUCER_WIDTH, SaucerView, Vec2 as CoreVec2};
use vello::kurbo::{Affine, BezPath, Ellipse, Point, Shape};
use vello::peniko::{Color, Extend};

use crate::display::{DisplayList, Paint, Stop, rgba};
use crate::paint::{Canvas, Frame, Style};

/// Marge autour du dessin (points).
const MARGIN: f32 = 22.0;

/// Taille de la fenêtre de la soucoupe, et où est son centre dedans.
#[must_use]
pub fn saucer_canvas() -> Canvas {
    let width = SAUCER_WIDTH + 2.0 * MARGIN;
    let height = SAUCER_HEIGHT + BEAM_MAX + 2.0 * MARGIN;
    Canvas {
        width: width.ceil(),
        height: height.ceil(),
        anchor: CoreVec2::new(width * 0.5, MARGIN + SAUCER_HEIGHT * 0.5),
    }
}

fn stops(colors: &[(f32, Color)]) -> Vec<Stop> {
    colors
        .iter()
        .map(|&(offset, color)| Stop { offset, color })
        .collect()
}

/// Ajoute à `out` la soucoupe, son faisceau et ses lumières.
pub fn paint_saucer(view: &SaucerView, frame: &Frame, style: &Style, out: &mut DisplayList) {
    let center = Point::new(f64::from(view.pos.x), f64::from(view.pos.y));
    let world = frame.world_to_px();
    // Elle penche avec sa vitesse ; le faisceau, lui, tombe droit.
    let body = world * Affine::translate(center.to_vec2()) * Affine::rotate(f64::from(view.tilt));
    let at = world * Affine::translate(center.to_vec2());
    let beam = f64::from(view.beam.clamp(0.0, 1.0));
    let w = f64::from(SAUCER_WIDTH);
    let h = f64::from(SAUCER_HEIGHT);

    // Le faisceau : un cône de lumière pâle, avec des anneaux qui descendent.
    if beam > 0.01 {
        let len = f64::from(view.beam_len.clamp(20.0, BEAM_MAX));
        let top = h * 0.3;
        let mut cone = BezPath::new();
        cone.move_to((-w * 0.2, top));
        cone.line_to((w * 0.2, top));
        cone.line_to((w * 0.42, top + len));
        cone.quad_to((0.0, top + len + 10.0), (-w * 0.42, top + len));
        cone.close_path();
        let glow = if style.dark {
            rgba(170, 255, 225, 1.0)
        } else {
            rgba(120, 235, 200, 1.0)
        };
        out.fill(
            cone,
            at,
            Paint::Linear {
                start: Point::new(0.0, top),
                end: Point::new(0.0, top + len),
                stops: stops(&[
                    (0.0, glow.with_alpha((0.55 * beam) as f32)),
                    (0.7, glow.with_alpha((0.22 * beam) as f32)),
                    (1.0, glow.with_alpha(0.0)),
                ]),
                extend: Extend::Pad,
            },
        );
        for k in 0..3 {
            let phase = (f64::from(view.lights) * 0.8 + f64::from(k) / 3.0).fract();
            let y = top + phase * len;
            let half = w * (0.2 + 0.22 * phase);
            let ring = Ellipse::new((0.0, y), (half, 3.0 + 4.0 * phase), 0.0).to_path(0.1);
            let fade = (1.0 - phase) * phase * 4.0;
            out.fill(
                ring,
                at,
                Paint::Solid(Color::WHITE.with_alpha((0.22 * beam * fade) as f32)),
            );
        }
    }

    // Lueur sous elle quand le faisceau s'allume.
    if beam > 0.01 {
        let c = Point::new(0.0, h * 0.25);
        out.disc(
            c,
            w * 0.45,
            body,
            Paint::Radial {
                start: c,
                start_radius: 0.0,
                end: c,
                end_radius: (w * 0.45) as f32,
                stops: stops(&[
                    (0.0, rgba(190, 255, 235, (0.5 * beam) as f32)),
                    (1.0, rgba(190, 255, 235, 0.0)),
                ]),
                extend: Extend::Pad,
            },
        );
    }

    // Le dôme de verre, derrière le corps.
    let dome = Ellipse::new((0.0, -h * 0.12), (w * 0.24, h * 0.42), 0.0).to_path(0.1);
    out.fill(
        dome,
        body,
        Paint::Linear {
            start: Point::new(0.0, -h * 0.55),
            end: Point::new(0.0, h * 0.1),
            stops: stops(&[
                (0.0, rgba(214, 248, 255, 0.92)),
                (0.6, rgba(120, 205, 225, 0.88)),
                (1.0, rgba(60, 130, 160, 0.9)),
            ]),
            extend: Extend::Pad,
        },
    );
    // Reflet du dôme.
    let shine = Ellipse::new((-w * 0.08, -h * 0.34), (w * 0.07, h * 0.1), -0.5).to_path(0.1);
    out.fill(shine, body, Paint::Solid(Color::WHITE.with_alpha(0.7)));

    // Le corps : une ellipse argentée, lavande dessous.
    let hull = Ellipse::new((0.0, h * 0.08), (w * 0.5, h * 0.27), 0.0).to_path(0.1);
    out.fill(
        hull,
        body,
        Paint::Linear {
            start: Point::new(0.0, -h * 0.2),
            end: Point::new(0.0, h * 0.36),
            stops: stops(&[
                (0.0, rgba(238, 236, 250, 1.0)),
                (0.45, rgba(196, 190, 230, 1.0)),
                (1.0, rgba(108, 100, 150, 1.0)),
            ]),
            extend: Extend::Pad,
        },
    );
    // La ceinture, plus sombre.
    let mut belt = BezPath::new();
    belt.move_to((-w * 0.5, h * 0.08));
    belt.quad_to((0.0, h * 0.3), (w * 0.5, h * 0.08));
    belt.quad_to((0.0, h * 0.42), (-w * 0.5, h * 0.08));
    belt.close_path();
    out.fill(belt, body, Paint::Solid(rgba(92, 84, 140, 0.55)));
    // Un liseré clair sur le dessus.
    let rim = Ellipse::new((0.0, h * 0.0), (w * 0.44, h * 0.06), 0.0).to_path(0.1);
    out.fill(rim, body, Paint::Solid(Color::WHITE.with_alpha(0.45)));

    // Les lumières, qui clignotent l'une après l'autre.
    let colors = [
        rgba(255, 212, 90, 1.0),
        rgba(255, 122, 182, 1.0),
        rgba(111, 230, 255, 1.0),
    ];
    for (i, color) in colors.iter().cycle().take(7).enumerate() {
        let k = i as f64;
        let u = k / 6.0 - 0.5;
        let x = u * w * 0.8;
        let y = h * 0.2 + (1.0 - (2.0 * u).powi(2)) * h * 0.08;
        let blink = (f64::from(view.lights) * std::f64::consts::TAU - k * 0.9).sin();
        let on = 0.45 + 0.55 * blink.max(0.0);
        let color = *color;
        let c = Point::new(x, y);
        out.disc(
            c,
            7.0,
            body,
            Paint::Radial {
                start: c,
                start_radius: 0.0,
                end: c,
                end_radius: 7.0,
                stops: stops(&[
                    (0.0, color.with_alpha((0.55 * on) as f32)),
                    (1.0, color.with_alpha(0.0)),
                ]),
                extend: Extend::Pad,
            },
        );
        out.disc(
            c,
            2.6,
            body,
            Paint::Solid(color.with_alpha((0.6 + 0.4 * on) as f32)),
        );
    }
}
