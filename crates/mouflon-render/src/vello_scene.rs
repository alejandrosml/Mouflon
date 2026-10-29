//! Traduction de la liste d'affichage en scène Vello (rendu GPU).

use vello::Scene;
use vello::kurbo::{Cap, Join, Stroke};
use vello::peniko::{
    BlendMode, Brush, Color, ColorStop, Compose, Gradient, ImageAlphaType, ImageBrush, ImageData,
    ImageFormat, ImageQuality,
};

use crate::display::{Command, DisplayList, LineCap, LineJoin, Paint, Stop};

fn stops(list: &[Stop]) -> Vec<ColorStop> {
    list.iter()
        .map(|s| ColorStop::from((s.offset, s.color)))
        .collect()
}

fn brush(paint: &Paint) -> Brush {
    match paint {
        Paint::Solid(c) => Brush::Solid(*c),
        Paint::Linear {
            start,
            end,
            stops: s,
            extend,
        } => Brush::Gradient(
            Gradient::new_linear(*start, *end)
                .with_stops(stops(s).as_slice())
                .with_extend(*extend),
        ),
        Paint::Radial {
            start,
            start_radius,
            end,
            end_radius,
            stops: s,
            extend,
        } => Brush::Gradient(
            Gradient::new_two_point_radial(*start, *start_radius, *end, *end_radius)
                .with_stops(stops(s).as_slice())
                .with_extend(*extend),
        ),
    }
}

/// Ajoute la liste d'affichage à une scène Vello.
pub fn encode(list: &DisplayList, scene: &mut Scene) {
    for command in &list.commands {
        match command {
            Command::Fill {
                path,
                transform,
                paint,
                brush_transform,
                rule,
            } => {
                scene.fill(*rule, *transform, &brush(paint), *brush_transform, path);
            }
            Command::Stroke {
                path,
                transform,
                paint,
                brush_transform,
                style,
            } => {
                let stroke = Stroke::new(style.width)
                    .with_caps(match style.cap {
                        LineCap::Butt => Cap::Butt,
                        LineCap::Round => Cap::Round,
                        LineCap::Square => Cap::Square,
                    })
                    .with_join(match style.join {
                        LineJoin::Miter => Join::Miter,
                        LineJoin::Round => Join::Round,
                        LineJoin::Bevel => Join::Bevel,
                    })
                    .with_miter_limit(style.miter_limit);
                scene.stroke(&stroke, *transform, &brush(paint), *brush_transform, path);
            }
            Command::BlurredRect {
                rect,
                radius,
                std_dev,
                color,
                transform,
            } => {
                scene.draw_blurred_rounded_rect(*transform, *rect, *color, *radius, *std_dev);
            }
            Command::Image {
                bitmap,
                transform,
                alpha,
                smooth,
            } => {
                let data = ImageData {
                    data: bitmap.rgba.clone(),
                    format: ImageFormat::Rgba8,
                    alpha_type: ImageAlphaType::Alpha,
                    width: bitmap.width,
                    height: bitmap.height,
                };
                let quality = if *smooth {
                    ImageQuality::Medium
                } else {
                    ImageQuality::Low
                };
                scene.draw_image(
                    &ImageBrush::new(data)
                        .with_quality(quality)
                        .with_alpha(*alpha),
                    *transform,
                );
            }
            Command::PushLayer {
                clip,
                transform,
                alpha,
                mix,
            } => {
                scene.push_layer(
                    vello::peniko::Fill::NonZero,
                    BlendMode::new(*mix, Compose::SrcOver),
                    *alpha,
                    *transform,
                    clip,
                );
            }
            Command::PopLayer => scene.pop_layer(),
        }
    }
}

/// Couleur de fond transparente pour les fenêtres des animaux.
pub const TRANSPARENT: Color = Color::TRANSPARENT;
