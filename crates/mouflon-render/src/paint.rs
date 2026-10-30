//! Dessin d'un animal dans sa fenêtre : ombre, personnage, lumière, particules.

use std::sync::Arc;

use mouflon_core::{
    Appearance, Drawn, Facings, Particle, ParticleKind, PetDef, PetView, Shadow, Support,
    Vec2 as CoreVec2,
};
use vello::kurbo::{Affine, BezPath, Point, Rect, Shape};
use vello::peniko::{Color, Extend, Fill, Mix};

use crate::animate::{PoseTransforms, pose};
use crate::bake::BakedRig;
use crate::display::{Command, DisplayList, Paint, Stop, rgba};
use crate::rig::{Item, Part, Rig, RigError};
use crate::sprites::{SpriteError, SpriteFrames};

#[derive(Debug, thiserror::Error)]
pub enum ArtError {
    #[error(transparent)]
    Rig(#[from] RigError),
    #[error(transparent)]
    Sprites(#[from] SpriteError),
}

/// Apparence prête à dessiner.
#[derive(Clone, Debug)]
pub enum Look {
    Rig(Arc<Rig>),
    Sprites(Arc<SpriteFrames>),
}

#[derive(Clone, Debug)]
pub struct PetArt {
    pub look: Look,
    /// Taille affichée du dessin (une case pour des sprites), en points.
    pub width: f32,
    pub height: f32,
    /// Intensité de l'éclairage dynamique sur le corps (0 = aucun).
    pub lighting: f32,
    /// Corps de l'animal, en points : demi-largeur et hauteur au-dessus du
    /// point d'appui (le curseur l'attrape là, les clics passent ailleurs).
    pub body: (f32, f32),
    /// Effets déjà dessinés dans les images.
    pub drawn: Drawn,
    /// Dessin vectoriel cuit en images (voir [`crate::bake`]) : le rendu
    /// direct le pose sans Vello. `None` : tracés dessinés tels quels.
    pub baked: Option<Arc<BakedRig>>,
    /// Distance parcourue pendant un cycle de marche (points) : les pattes
    /// d'un dessin vectoriel balaient juste ce qu'il faut pour ne pas glisser.
    pub stride: f32,
}

/// Fenêtre d'un animal : taille en points et position du point d'appui.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub anchor: CoreVec2,
}

impl PetArt {
    pub fn load(def: &PetDef) -> Result<Self, ArtError> {
        match &def.appearance {
            Appearance::Rig { svg, height } => {
                let rig = Rig::parse(svg)?;
                let k = f64::from(*height) / rig.bounds.height().max(1.0);
                let width = (rig.bounds.width() * k) as f32;
                let lighting = 0.6;
                let baked = BakedRig::bake(&rig, k, lighting).map(Arc::new);
                Ok(Self {
                    width,
                    height: *height,
                    look: Look::Rig(Arc::new(rig)),
                    lighting,
                    body: (width * 0.46, *height * 1.02),
                    drawn: Drawn::default(),
                    baked,
                    stride: def.stride(),
                })
            }
            Appearance::Sprites(sheet) => {
                let frames = SpriteFrames::decode(sheet)?;
                let p = def.physics;
                Ok(Self {
                    width: frames.tile_width as f32 * sheet.scale,
                    height: frames.tile_height as f32 * sheet.scale,
                    look: Look::Sprites(Arc::new(frames)),
                    lighting: 0.0,
                    body: (p.width * 0.46, p.height * 1.02),
                    drawn: sheet.drawn,
                    baked: None,
                    stride: def.stride(),
                })
            }
        }
    }

    /// Place du dessin devant et derrière son point d'appui (points), pour
    /// un animal en images ; un dessin vectoriel suit sa largeur physique.
    #[must_use]
    pub fn reach(&self) -> Option<(f32, f32)> {
        match &self.look {
            Look::Sprites(frames) => frames.reach(),
            Look::Rig(_) => None,
        }
    }

    /// Fenêtre assez grande pour l'écrasement, les oreilles, les bulles,
    /// l'ombre et une culbute.
    #[must_use]
    pub fn canvas(&self) -> Canvas {
        let width = (self.width * 2.1).max(self.height * 2.2).max(120.0).ceil();
        let below = (self.height * 0.65).max(16.0).ceil();
        let height = (self.height * 2.25).max(100.0).ceil() + below;
        Canvas {
            width,
            height,
            anchor: CoreVec2::new(width * 0.5, height - below),
        }
    }
}

/// Réglages visuels globaux.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Style {
    pub dark: bool,
}

/// Cible du dessin : coin haut gauche (repère du monde) et pixels par point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub origin: CoreVec2,
    pub scale: f64,
}

impl Frame {
    #[must_use]
    pub fn world_to_px(&self) -> Affine {
        Affine::scale(self.scale)
            * Affine::translate((-f64::from(self.origin.x), -f64::from(self.origin.y)))
    }
}

fn pt(v: CoreVec2) -> Point {
    Point::new(f64::from(v.x), f64::from(v.y))
}

/// Ajoute à `out` le dessin complet d'un animal.
pub fn paint_pet(
    art: &PetArt,
    view: &PetView<'_>,
    frame: &Frame,
    style: &Style,
    out: &mut DisplayList,
) {
    let world = frame.world_to_px();
    paint_shadow(art, view, world, style, out);
    let layered = view.opacity < 0.999;
    if layered {
        let bounds = out.bounds().to_path(0.1);
        out.push_layer(
            bounds,
            Affine::IDENTITY,
            view.opacity.clamp(0.0, 1.0),
            Mix::Normal,
        );
    }
    match &art.look {
        Look::Rig(rig) => paint_rig(rig, art, view, world, out),
        Look::Sprites(frames) => paint_sprite(art, frames, view, world, out),
    }
    if layered {
        out.pop_layer();
    }
    paint_particles(view.particles, world, style, out);
}

/// L'ombre que Mouflon dessine sous l'animal, s'il en dessine une, avec sa
/// force : 1 au sol, de moins en moins en l'air, rien au-delà de 288 points.
fn drawn_shadow(art: &PetArt, view: &PetView<'_>) -> Option<(Shadow, f32)> {
    let shadow = view.shadow?;
    // Les images dessinent déjà leur ombre au sol : la nôtre ne sert qu'en l'air.
    if art.drawn.shadow && matches!(view.support, Support::Ground { .. }) {
        return None;
    }
    let near = (1.0 - (shadow.distance.max(0.0) / 320.0).min(1.0)).powi(2);
    (near >= 0.01).then_some((shadow, near))
}

fn paint_shadow(
    art: &PetArt,
    view: &PetView<'_>,
    world: Affine,
    style: &Style,
    out: &mut DisplayList,
) {
    let Some((shadow, near)) = drawn_shadow(art, view) else {
        return;
    };
    let strength = if style.dark { 0.42 } else { 0.27 } * view.opacity.clamp(0.0, 1.0);
    let body_width = art.body.0 / 0.46;
    let w = f64::from(body_width * (0.5 + 0.36 * near) * (1.0 + 0.2 * view.pose.squash.max(0.0)));
    let h = w * 0.17;
    let cx = f64::from(view.ground.x);
    let cy = f64::from(shadow.y) - h * 0.1;
    out.push(Command::BlurredRect {
        rect: Rect::new(cx - w / 2.0, cy - h / 2.0, cx + w / 2.0, cy + h / 2.0),
        radius: h / 2.0,
        std_dev: h * 0.5,
        color: Color::BLACK.with_alpha(strength * near),
        transform: world,
    });
    if shadow.distance < 12.0 {
        let w2 = w * 0.58;
        let h2 = h * 0.55;
        out.push(Command::BlurredRect {
            rect: Rect::new(
                cx - w2 / 2.0,
                cy - h2 / 2.0 + h * 0.08,
                cx + w2 / 2.0,
                cy + h2 / 2.0 + h * 0.08,
            ),
            radius: h2 / 2.0,
            std_dev: h2 * 0.3,
            color: Color::BLACK.with_alpha(strength * 0.9 * near),
            transform: world,
        });
    }
}

fn paint_rig(rig: &Rig, art: &PetArt, view: &PetView<'_>, world: Affine, out: &mut DisplayList) {
    let k = f64::from(art.height) / rig.bounds.height().max(1.0);
    let poses: PoseTransforms = pose(rig, &view.pose, view.facing, f64::from(art.stride) / k);
    let base = world
        * Affine::translate(pt(view.ground).to_vec2())
        * Affine::scale(k)
        * Affine::translate(-rig.ground.to_vec2())
        * poses.root;
    let canvas = out.bounds().to_path(0.1);
    let mut stack: Vec<Affine> = vec![base];
    let mut parts: Vec<Part> = Vec::new();
    let baked = art.baked.as_deref();
    for (index, item) in rig.items.iter().enumerate() {
        let top = *stack.last().unwrap_or(&base);
        // Cuit : chaque suite de tracés d'une partie est une image posée d'un coup.
        if let Some(baked) = baked
            && matches!(item, Item::Fill { .. } | Item::Stroke { .. })
        {
            if let Some(piece) = baked.piece(index) {
                // Retourné (déterminant négatif), le corps a son reflet de l'autre côté.
                let bitmap = match &piece.mirrored {
                    Some(mirrored) if top.determinant() < 0.0 => mirrored,
                    _ => &piece.bitmap,
                };
                out.push(Command::Image {
                    bitmap: bitmap.clone(),
                    transform: top * piece.placement,
                    alpha: 1.0,
                    smooth: true,
                });
            }
            continue;
        }
        match item {
            Item::BeginPart(part) => {
                stack.push(top * poses.part(*part));
                parts.push(*part);
            }
            Item::EndPart => {
                if parts.pop() == Some(Part::Body) && art.lighting > 0.0 && baked.is_none() {
                    paint_lighting(rig, art, top, out);
                }
                stack.pop();
            }
            Item::BeginLayer { alpha } => {
                out.push_layer(canvas.clone(), Affine::IDENTITY, *alpha, Mix::Normal)
            }
            Item::EndLayer => out.pop_layer(),
            Item::Fill {
                path,
                transform,
                paint,
                brush_transform,
                rule,
            } => out.push(Command::Fill {
                path: path.clone(),
                transform: top * *transform,
                paint: paint.clone(),
                brush_transform: *brush_transform,
                rule: *rule,
            }),
            Item::Stroke {
                path,
                transform,
                paint,
                brush_transform,
                style,
            } => out.push(Command::Stroke {
                path: path.clone(),
                transform: top * *transform,
                paint: paint.clone(),
                brush_transform: *brush_transform,
                style: style.clone(),
            }),
        }
    }
}

/// Lumière venue d'en haut à gauche de l'écran, quel que soit le sens de
/// l'animal : le reflet glisse sur le corps quand il se retourne, ce qui
/// donne du volume au dessin plat.
fn paint_lighting(rig: &Rig, art: &PetArt, body_transform: Affine, out: &mut DisplayList) {
    let Some((silhouette, local)) = &rig.body_silhouette else {
        return;
    };
    let transform = body_transform * *local;
    let bbox = transform.transform_rect_bbox(silhouette.bounding_box());
    if bbox.width() <= 0.0 || bbox.height() <= 0.0 {
        return;
    }
    out.push_layer(
        silhouette.clone(),
        transform,
        art.lighting.clamp(0.0, 1.0),
        Mix::SoftLight,
    );
    let light = Point::new(
        bbox.min_x() + bbox.width() * 0.32,
        bbox.min_y() + bbox.height() * 0.12,
    );
    let radius = bbox.width().max(bbox.height()) * 1.05;
    let paint = Paint::Radial {
        start: light,
        start_radius: 0.0,
        end: light,
        end_radius: radius as f32,
        stops: vec![
            Stop {
                offset: 0.0,
                color: Color::WHITE.with_alpha(0.95),
            },
            Stop {
                offset: 0.42,
                color: Color::WHITE.with_alpha(0.0),
            },
            Stop {
                offset: 0.62,
                color: Color::BLACK.with_alpha(0.0),
            },
            Stop {
                offset: 1.0,
                color: Color::BLACK.with_alpha(0.75),
            },
        ],
        extend: Extend::Pad,
    };
    out.push(Command::Fill {
        path: bbox.inflate(2.0, 2.0).to_path(0.1),
        transform: Affine::IDENTITY,
        paint,
        brush_transform: None,
        rule: Fill::NonZero,
    });
    out.pop_layer();
}

fn paint_sprite(
    art: &PetArt,
    frames: &SpriteFrames,
    view: &PetView<'_>,
    world: Affine,
    out: &mut DisplayList,
) {
    let Some(index) = view.sprite_frame else {
        return;
    };
    let Some(pick) = frames.pick(index, view.facing) else {
        return;
    };
    let frame = pick.frame;
    let bitmap = &frame.bitmap;
    let s = f64::from(frames.scale);
    let g = pt(view.ground);
    let offset = f64::from(view.sprite_offset_y);
    let (ax, ay) = (f64::from(pick.anchor_x), f64::from(frames.anchor.1));
    let (fx, fy, texel) = (
        f64::from(frame.x),
        f64::from(frame.y),
        f64::from(frame.texel),
    );
    let motion = SpriteMotion::of(view, art);
    // L'image est retournée quand son sens dessiné n'est pas celui de l'animal.
    let flipped = motion.mirror < 0.0;
    if motion.is_rest() {
        // Coin de l'image calé sur les pixels de l'écran : le dessin reste net.
        let device = world.as_coeffs()[0];
        let k = s * device;
        let anchor = world * Point::new(g.x, g.y + offset);
        let bw = f64::from(bitmap.width) * texel;
        let left = if flipped {
            anchor.x - (fx + bw - ax) * k
        } else {
            anchor.x + (fx - ax) * k
        };
        let (x, y) = (left.round(), (anchor.y + (fy - ay) * k).round());
        let pixel = texel * k;
        let transform = if flipped {
            Affine::translate((x + bw * k, y)) * Affine::scale_non_uniform(-pixel, pixel)
        } else {
            Affine::translate((x, y)) * Affine::scale(pixel)
        };
        out.push(Command::Image {
            bitmap: bitmap.clone(),
            transform,
            alpha: 1.0,
            // Un pixel d'image par pixel d'écran (ou un multiple entier : le pixel
            // art agrandi) reste franc ; sinon, l'image se lisse.
            smooth: pixel < 0.999 || (pixel - pixel.round()).abs() > 0.01,
        });
        return;
    }
    // En mouvement : écrasement depuis les pattes, balancement sous la main qui
    // le tient, inclinaison en vol, culbute, demi-tour qui pivote.
    // Tenu, il se balance autour du point où le curseur le tient.
    let pivot = match (view.support, view.grip) {
        (Support::Held, Some(grip)) => pt(grip),
        (Support::Held, None) => {
            Point::new(g.x, g.y - f64::from(frames.tile_height) * s * 0.95 + offset)
        }
        _ => Point::new(g.x, g.y - f64::from(art.body.1 / 1.02) * 0.5 + offset),
    };
    let sx = motion.mirror.signum() * motion.mirror.abs().max(0.12) * (1.0 + 0.5 * motion.squash);
    let sy = 1.0 - motion.squash;
    let transform = world
        * Affine::rotate_about(motion.angle, pivot)
        * Affine::translate((g.x, g.y + offset))
        * Affine::scale_non_uniform(sx, sy)
        * Affine::scale(s)
        * Affine::translate((fx - ax, fy - ay))
        * Affine::scale(texel);
    out.push(Command::Image {
        bitmap: bitmap.clone(),
        transform,
        alpha: 1.0,
        smooth: true,
    });
}

/// Mouvement ajouté par Mouflon aux images d'un animal en sprites.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteMotion {
    /// 1 : image telle qu'elle est dessinée ; -1 : en miroir ; entre les deux, il pivote.
    pub mirror: f64,
    /// > 0 : écrasé ; < 0 : étiré.
    pub squash: f64,
    /// Rotation dans le repère de l'écran (radians, sens horaire).
    pub angle: f64,
}

impl SpriteMotion {
    #[must_use]
    pub fn of(view: &PetView<'_>, art: &PetArt) -> Self {
        let facing = f64::from(view.facing.clamp(-1.0, 1.0));
        let lean = f64::from(view.pose.lean) * facing.signum();
        let angle = match view.support {
            Support::Held => f64::from(view.pose.swing) * 0.9,
            Support::Air => lean * 0.8,
            Support::Ground { .. } => lean * 0.3,
        } + f64::from(view.pose.roll);
        let facings = match &art.look {
            Look::Sprites(frames) => frames.facings,
            Look::Rig(_) => Facings::Right,
        };
        let mirror = match facings {
            // Les sprites desktopPet regardent vers la gauche.
            Facings::Left => -facing,
            Facings::Right => facing,
            // Une image par sens : jamais retournée, elle se resserre seulement
            // pendant un demi-tour qui n'est pas dessiné.
            Facings::Both => facing.abs(),
        };
        Self {
            mirror,
            squash: if art.drawn.squash {
                0.0
            } else {
                f64::from(view.pose.squash)
            },
            angle,
        }
    }

    /// Image au repos : on la dessine nette, au pixel près.
    #[must_use]
    pub fn is_rest(&self) -> bool {
        self.squash.abs() < 0.004 && self.angle.abs() < 0.003 && self.mirror.abs() > 0.999
    }

    /// Le mouvement au millième près : deux mouvements de même clé se
    /// dessinent à moins d'un cinquième de pixel l'un de l'autre, même pour
    /// une image de 320 pixels. Au repos, l'image se pose au pixel près quel
    /// que soit ce qui reste du mouvement : une seule clé.
    #[must_use]
    pub fn key(&self) -> [i32; 3] {
        let thousandths = |v: f64| (v * 1000.0).round() as i32;
        if self.is_rest() {
            return [thousandths(self.mirror.signum()), 0, 0];
        }
        [
            thousandths(self.mirror),
            thousandths(self.squash),
            thousandths(self.angle),
        ]
    }
}

/// Ce qui change, à l'œil, l'image d'un animal en sprites. Son déplacement
/// passe par la position de sa fenêtre : on ne le redessine que quand cette
/// clé change, quelques fois par seconde au lieu de 120.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpriteKey {
    frame: Option<u32>,
    /// Décalage vertical de l'image, au quart de point.
    offset: i32,
    opacity: u8,
    mirrored: bool,
    /// L'ombre, tant qu'elle se voit : sa distance à l'animal au demi-point
    /// (elle reste au sol quand la fenêtre bouge), son écrasement au millième.
    shadow: Option<[i32; 2]>,
    /// Le mouvement ajouté par Mouflon : une inclinaison qui ne change pas
    /// (au plafond, contre un mur) ne fait rien redessiner.
    motion: [i32; 3],
}

impl SpriteKey {
    /// La clé de l'image de `view`, ou rien quand l'animal se redessine à
    /// chaque image (dessin vectoriel, particules).
    #[must_use]
    pub fn of(art: &PetArt, view: &PetView<'_>) -> Option<Self> {
        if !matches!(art.look, Look::Sprites(_)) || !view.particles.is_empty() {
            return None;
        }
        Some(Self {
            frame: view.sprite_frame,
            offset: (view.sprite_offset_y * 4.0).round() as i32,
            opacity: (view.opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
            mirrored: view.facing > 0.0,
            shadow: drawn_shadow(art, view).map(|(shadow, _)| {
                [
                    (shadow.distance * 2.0).round() as i32,
                    (view.pose.squash.max(0.0) * 1000.0).round() as i32,
                ]
            }),
            motion: SpriteMotion::of(view, art).key(),
        })
    }
}

fn paint_particles(particles: &[Particle], world: Affine, style: &Style, out: &mut DisplayList) {
    for p in particles {
        let t = f64::from(p.progress());
        let center = pt(p.pos);
        match p.kind {
            ParticleKind::Dust => {
                let r = f64::from(p.size) * (1.0 + 1.3 * t);
                let alpha = 0.5 * (1.0 - t).powf(1.6) as f32;
                let tone = if style.dark {
                    rgba(190, 196, 210, 1.0)
                } else {
                    rgba(168, 160, 150, 1.0)
                };
                let paint = Paint::Radial {
                    start: center,
                    start_radius: 0.0,
                    end: center,
                    end_radius: r as f32,
                    stops: vec![
                        Stop {
                            offset: 0.0,
                            color: tone.with_alpha(alpha),
                        },
                        Stop {
                            offset: 1.0,
                            color: tone.with_alpha(0.0),
                        },
                    ],
                    extend: Extend::Pad,
                };
                out.disc(center, r, world, paint);
            }
            ParticleKind::Bubble => {
                let fade = smooth(0.0, 0.12, t) * (1.0 - smooth(0.72, 1.0, t));
                let r = f64::from(p.size) * (0.75 + 0.7 * t);
                let ink = if style.dark {
                    rgba(220, 232, 255, 1.0)
                } else {
                    rgba(120, 150, 210, 1.0)
                };
                out.disc(
                    center,
                    r,
                    world,
                    Paint::Radial {
                        start: center + (r * 0.25, r * 0.3),
                        start_radius: 0.0,
                        end: center,
                        end_radius: r as f32,
                        stops: vec![
                            Stop {
                                offset: 0.0,
                                color: ink.with_alpha(0.0),
                            },
                            Stop {
                                offset: 0.8,
                                color: ink.with_alpha(0.16 * fade as f32),
                            },
                            Stop {
                                offset: 1.0,
                                color: ink.with_alpha(0.55 * fade as f32),
                            },
                        ],
                        extend: Extend::Pad,
                    },
                );
                let spark = center + (-r * 0.35, -r * 0.38);
                out.disc(
                    spark,
                    r * 0.22,
                    world,
                    Paint::Solid(Color::WHITE.with_alpha(0.85 * fade as f32)),
                );
            }
        }
    }
}

fn smooth(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Chemin d'une forme quelconque (utile aux outils).
#[must_use]
pub fn shape_path(shape: &impl Shape) -> BezPath {
    shape.to_path(0.1)
}
