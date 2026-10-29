//! Liste d'affichage indépendante du moteur graphique.
//!
//! Le rendu de l'animal produit une liste de commandes simples (formes,
//! dégradés, flous, calques). Deux traducteurs la consomment : Vello pour
//! l'écran, SVG pour l'outil d'aperçu. Une seule description, deux sorties
//! identiques.

use vello::kurbo::{Affine, BezPath, Point, Rect};
use vello::peniko::{Blob, Color, Extend, Fill, Mix};

#[derive(Clone, Debug, PartialEq)]
pub struct Stop {
    pub offset: f32,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Paint {
    Solid(Color),
    Linear {
        start: Point,
        end: Point,
        stops: Vec<Stop>,
        extend: Extend,
    },
    /// Dégradé radial à deux cercles (le cercle de départ joue le rôle du foyer).
    Radial {
        start: Point,
        start_radius: f32,
        end: Point,
        end_radius: f32,
        stops: Vec<Stop>,
        extend: Extend,
    },
}

impl Paint {
    /// Multiplie l'opacité de toutes les couleurs.
    #[must_use]
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        let scale = |c: &mut Color| *c = c.multiply_alpha(alpha);
        match &mut self {
            Self::Solid(c) => scale(c),
            Self::Linear { stops, .. } | Self::Radial { stops, .. } => {
                stops.iter_mut().for_each(|s| scale(&mut s.color))
            }
        }
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StrokeStyle {
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub miter_limit: f64,
}

/// Image RGBA (alpha non prémultiplié). Le `Blob` garde un identifiant
/// stable : Vello ne la téléverse qu'une fois sur le GPU.
#[derive(Clone, Debug)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub rgba: Blob<u8>,
}

impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.rgba.id() == other.rgba.id()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Fill {
        path: BezPath,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        rule: Fill,
    },
    Stroke {
        path: BezPath,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        style: StrokeStyle,
    },
    /// Rectangle arrondi flouté (ombres douces).
    BlurredRect {
        rect: Rect,
        radius: f64,
        std_dev: f64,
        color: Color,
        transform: Affine,
    },
    Image {
        bitmap: Bitmap,
        transform: Affine,
        alpha: f32,
        smooth: bool,
    },
    /// Calque : tout ce qui suit jusqu'au `PopLayer` est découpé par `clip`,
    /// puis composé avec l'opacité et le mode de fusion donnés.
    PushLayer {
        clip: BezPath,
        transform: Affine,
        alpha: f32,
        mix: Mix,
    },
    PopLayer,
}

/// Une image à dessiner : taille en pixels et commandes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayList {
    pub width: u32,
    pub height: u32,
    pub commands: Vec<Command>,
}

impl DisplayList {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.commands.clear();
    }

    #[must_use]
    pub fn bounds(&self) -> Rect {
        Rect::new(0.0, 0.0, f64::from(self.width), f64::from(self.height))
    }

    pub fn fill(&mut self, path: BezPath, transform: Affine, paint: Paint) {
        self.commands.push(Command::Fill {
            path,
            transform,
            paint,
            brush_transform: None,
            rule: Fill::NonZero,
        });
    }

    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn push_layer(&mut self, clip: BezPath, transform: Affine, alpha: f32, mix: Mix) {
        self.commands.push(Command::PushLayer {
            clip,
            transform,
            alpha,
            mix,
        });
    }

    pub fn pop_layer(&mut self) {
        self.commands.push(Command::PopLayer);
    }
}

/// Couleur sRGB depuis des octets.
#[must_use]
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::new([
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
        a,
    ])
}
