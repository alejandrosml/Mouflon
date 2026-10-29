//! Rig vectoriel : un dessin SVG découpé en calques nommés que le moteur
//! anime de façon procédurale.
//!
//! # Contrat pour l'artiste
//!
//! - Le personnage regarde vers la **droite**.
//! - Chaque partie animée est un groupe `<g>` dont l'`id` est l'un des noms
//!   de [`Part`] : `body`, `head`, `ear-near`, `ear-far`, `eyes`, `pupils`,
//!   `tail`, `leg-front-near`, `leg-front-far`, `leg-back-near`, `leg-back-far`.
//!   Toutes sont facultatives ; une partie peut en contenir d'autres
//!   (`eyes` dans `head`, `pupils` dans `eyes`).
//! - Le pivot de chaque partie (épaule, hanche, cou, base d'oreille…) est un
//!   petit cercle d'`id` `pivot-<partie>`, par exemple `pivot-head`. Le point
//!   d'appui au sol est `pivot-ground`. Les pivots ne sont jamais dessinés.
//! - L'ordre du document est l'ordre de dessin : pattes du fond, corps,
//!   pattes de devant, tête.

use std::collections::HashMap;

use vello::kurbo::{Affine, BezPath, Point, Rect};
use vello::peniko::{Color, Extend, Fill};

use crate::display::{LineCap, LineJoin, Paint, Stop, StrokeStyle};

/// Parties animables d'un personnage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Part {
    Body,
    Head,
    EarNear,
    EarFar,
    Eyes,
    Pupils,
    Tail,
    LegFrontNear,
    LegFrontFar,
    LegBackNear,
    LegBackFar,
}

impl Part {
    pub const ALL: [Self; 11] = [
        Self::Body,
        Self::Head,
        Self::EarNear,
        Self::EarFar,
        Self::Eyes,
        Self::Pupils,
        Self::Tail,
        Self::LegFrontNear,
        Self::LegFrontFar,
        Self::LegBackNear,
        Self::LegBackFar,
    ];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Head => "head",
            Self::EarNear => "ear-near",
            Self::EarFar => "ear-far",
            Self::Eyes => "eyes",
            Self::Pupils => "pupils",
            Self::Tail => "tail",
            Self::LegFrontNear => "leg-front-near",
            Self::LegFrontFar => "leg-front-far",
            Self::LegBackNear => "leg-back-near",
            Self::LegBackFar => "leg-back-far",
        }
    }

    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        let id = id.replace('_', "-").to_ascii_lowercase();
        Self::ALL.into_iter().find(|p| p.name() == id)
    }

    #[must_use]
    pub fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub fn is_leg(self) -> bool {
        matches!(
            self,
            Self::LegFrontNear | Self::LegFrontFar | Self::LegBackNear | Self::LegBackFar
        )
    }
}

/// Élément du dessin, dans l'ordre du document.
#[derive(Clone, Debug)]
pub enum Item {
    BeginPart(Part),
    EndPart,
    BeginLayer {
        alpha: f32,
    },
    EndLayer,
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
}

#[derive(Clone, Debug)]
pub struct Rig {
    /// Taille du document SVG (unités utilisateur).
    pub size: (f64, f64),
    /// Boîte englobante du dessin (sans les pivots).
    pub bounds: Rect,
    /// Point d'appui au sol, dans le repère du document.
    pub ground: Point,
    pivots: HashMap<Part, Point>,
    part_bounds: HashMap<Part, Rect>,
    /// Silhouette du corps, pour l'éclairage dynamique.
    pub body_silhouette: Option<(BezPath, Affine)>,
    pub items: Vec<Item>,
}

#[derive(Debug, thiserror::Error)]
pub enum RigError {
    #[error("SVG invalide : {0}")]
    Svg(#[from] usvg::Error),
    #[error("le dessin est vide")]
    Empty,
}

impl Rig {
    /// Analyse un SVG suivant le contrat décrit dans ce module.
    pub fn parse(svg: &[u8]) -> Result<Self, RigError> {
        let tree = usvg::Tree::from_data(svg, &usvg::Options::default())?;
        let mut builder = Builder::default();
        builder.walk(tree.root(), None);
        let bounds = builder.bounds.ok_or(RigError::Empty)?;
        let ground = builder
            .named_pivots
            .get("ground")
            .copied()
            .unwrap_or_else(|| Point::new(bounds.center().x, bounds.max_y()));
        let mut pivots = HashMap::new();
        for part in Part::ALL {
            if let Some(p) = builder.named_pivots.get(part.name()) {
                pivots.insert(part, *p);
            }
        }
        let size = tree.size();
        Ok(Self {
            size: (f64::from(size.width()), f64::from(size.height())),
            bounds,
            ground,
            pivots,
            part_bounds: builder.part_bounds,
            body_silhouette: builder.body_silhouette,
            items: builder.items,
        })
    }

    #[must_use]
    pub fn has_part(&self, part: Part) -> bool {
        self.part_bounds.contains_key(&part)
    }

    /// Pivot d'une partie : celui fourni par l'artiste, sinon une valeur
    /// raisonnable déduite de la forme (haut des pattes, base de la tête…).
    #[must_use]
    pub fn pivot(&self, part: Part) -> Point {
        if let Some(p) = self.pivots.get(&part) {
            return *p;
        }
        let b = self.part_bounds.get(&part).copied().unwrap_or(self.bounds);
        match part {
            p if p.is_leg() => Point::new(b.center().x, b.min_y() + b.height() * 0.15),
            Part::Head => Point::new(b.min_x() + b.width() * 0.3, b.max_y() - b.height() * 0.2),
            Part::EarNear | Part::EarFar => Point::new(b.center().x, b.max_y()),
            Part::Tail => Point::new(b.max_x(), b.center().y),
            Part::Body => Point::new(b.center().x, b.max_y()),
            _ => b.center(),
        }
    }

    /// Boîte d'une partie (ou du dessin entier si elle est absente).
    #[must_use]
    pub fn part_bounds(&self, part: Part) -> Rect {
        self.part_bounds.get(&part).copied().unwrap_or(self.bounds)
    }
}

#[derive(Default)]
struct Builder {
    items: Vec<Item>,
    bounds: Option<Rect>,
    named_pivots: HashMap<String, Point>,
    part_bounds: HashMap<Part, Rect>,
    open_parts: Vec<Part>,
    body_silhouette: Option<(BezPath, Affine)>,
}

impl Builder {
    fn walk(&mut self, group: &usvg::Group, part_hint: Option<Part>) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => {
                    if g.clip_path().is_some() || g.mask().is_some() || !g.filters().is_empty() {
                        log::warn!(
                            "groupe « {} » : masques, découpes et filtres SVG ne sont pas encore gérés",
                            g.id()
                        );
                    }
                    let part = Part::from_id(g.id());
                    if let Some(part) = part {
                        self.items.push(Item::BeginPart(part));
                        self.open_parts.push(part);
                    }
                    let alpha = g.opacity().get();
                    let layered = alpha < 0.999;
                    if layered {
                        self.items.push(Item::BeginLayer { alpha });
                    }
                    self.walk(g, part.or(part_hint));
                    if layered {
                        self.items.push(Item::EndLayer);
                    }
                    if part.is_some() {
                        self.items.push(Item::EndPart);
                        self.open_parts.pop();
                    }
                }
                usvg::Node::Path(path) => self.path(path),
                usvg::Node::Image(_) => {
                    log::warn!("les images bitmap dans un rig SVG sont ignorées")
                }
                usvg::Node::Text(_) => {
                    log::warn!("le texte d'un rig SVG doit être converti en tracés")
                }
            }
        }
    }

    fn path(&mut self, path: &usvg::Path) {
        let id = path.id();
        if let Some(name) = id.strip_prefix("pivot-") {
            let b = path.abs_bounding_box();
            let c = Point::new(
                f64::from(b.x() + b.width() / 2.0),
                f64::from(b.y() + b.height() / 2.0),
            );
            self.named_pivots
                .insert(name.replace('_', "-").to_ascii_lowercase(), c);
            return;
        }
        if !path.is_visible() {
            return;
        }
        let bez = to_bez(path.data());
        let transform = to_affine(path.abs_transform());
        let b = path.abs_stroke_bounding_box();
        let rect = Rect::new(
            f64::from(b.left()),
            f64::from(b.top()),
            f64::from(b.right()),
            f64::from(b.bottom()),
        );
        self.bounds = Some(self.bounds.map_or(rect, |r| r.union(rect)));
        for part in &self.open_parts {
            self.part_bounds
                .entry(*part)
                .and_modify(|r| *r = r.union(rect))
                .or_insert(rect);
        }

        let in_body = self.open_parts.last() == Some(&Part::Body);
        if let Some(fill) = path.fill() {
            if in_body {
                let (sil, t) = self
                    .body_silhouette
                    .get_or_insert_with(|| (BezPath::new(), transform));
                // Les tracés du corps sont regroupés dans le repère du premier.
                let local = t.inverse() * transform;
                for el in bez.elements() {
                    sil.push(local * *el);
                }
            }
            if let Some((paint, brush)) = to_paint(fill.paint(), fill.opacity().get()) {
                let rule = match fill.rule() {
                    usvg::FillRule::NonZero => Fill::NonZero,
                    usvg::FillRule::EvenOdd => Fill::EvenOdd,
                };
                self.items.push(Item::Fill {
                    path: bez.clone(),
                    transform,
                    paint,
                    brush_transform: brush,
                    rule,
                });
            }
        }
        if let Some(stroke) = path.stroke()
            && let Some((paint, brush)) = to_paint(stroke.paint(), stroke.opacity().get())
        {
            let style = StrokeStyle {
                width: f64::from(stroke.width().get()),
                cap: match stroke.linecap() {
                    usvg::LineCap::Butt => LineCap::Butt,
                    usvg::LineCap::Round => LineCap::Round,
                    usvg::LineCap::Square => LineCap::Square,
                },
                join: match stroke.linejoin() {
                    usvg::LineJoin::Round => LineJoin::Round,
                    usvg::LineJoin::Bevel => LineJoin::Bevel,
                    usvg::LineJoin::Miter | usvg::LineJoin::MiterClip => LineJoin::Miter,
                },
                miter_limit: f64::from(stroke.to_tiny_skia().miter_limit),
            };
            self.items.push(Item::Stroke {
                path: bez,
                transform,
                paint,
                brush_transform: brush,
                style,
            });
        }
    }
}

fn to_affine(t: usvg::Transform) -> Affine {
    Affine::new([
        f64::from(t.sx),
        f64::from(t.ky),
        f64::from(t.kx),
        f64::from(t.sy),
        f64::from(t.tx),
        f64::from(t.ty),
    ])
}

fn to_bez(path: &usvg::tiny_skia_path::Path) -> BezPath {
    use usvg::tiny_skia_path::PathSegment;
    let p = |pt: usvg::tiny_skia_path::Point| Point::new(f64::from(pt.x), f64::from(pt.y));
    let mut bez = BezPath::new();
    for seg in path.segments() {
        match seg {
            PathSegment::MoveTo(a) => bez.move_to(p(a)),
            PathSegment::LineTo(a) => bez.line_to(p(a)),
            PathSegment::QuadTo(a, b) => bez.quad_to(p(a), p(b)),
            PathSegment::CubicTo(a, b, c) => bez.curve_to(p(a), p(b), p(c)),
            PathSegment::Close => bez.close_path(),
        }
    }
    bez
}

fn to_color(c: usvg::Color, alpha: f32) -> Color {
    Color::new([
        f32::from(c.red) / 255.0,
        f32::from(c.green) / 255.0,
        f32::from(c.blue) / 255.0,
        alpha,
    ])
}

fn to_stops(stops: &[usvg::Stop], alpha: f32) -> Vec<Stop> {
    stops
        .iter()
        .map(|s| Stop {
            offset: s.offset().get(),
            color: to_color(s.color(), s.opacity().get() * alpha),
        })
        .collect()
}

fn to_extend(spread: usvg::SpreadMethod) -> Extend {
    match spread {
        usvg::SpreadMethod::Pad => Extend::Pad,
        usvg::SpreadMethod::Reflect => Extend::Reflect,
        usvg::SpreadMethod::Repeat => Extend::Repeat,
    }
}

fn to_paint(paint: &usvg::Paint, alpha: f32) -> Option<(Paint, Option<Affine>)> {
    match paint {
        usvg::Paint::Color(c) => Some((Paint::Solid(to_color(*c, alpha)), None)),
        usvg::Paint::LinearGradient(g) => Some((
            Paint::Linear {
                start: Point::new(f64::from(g.x1()), f64::from(g.y1())),
                end: Point::new(f64::from(g.x2()), f64::from(g.y2())),
                stops: to_stops(g.stops(), alpha),
                extend: to_extend(g.spread_method()),
            },
            Some(to_affine(g.transform())),
        )),
        usvg::Paint::RadialGradient(g) => Some((
            Paint::Radial {
                start: Point::new(f64::from(g.fx()), f64::from(g.fy())),
                start_radius: g.fr().get(),
                end: Point::new(f64::from(g.cx()), f64::from(g.cy())),
                end_radius: g.r().get(),
                stops: to_stops(g.stops(), alpha),
                extend: to_extend(g.spread_method()),
            },
            Some(to_affine(g.transform())),
        )),
        usvg::Paint::Pattern(_) => {
            log::warn!("les motifs SVG (pattern) ne sont pas gérés");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
        <defs><linearGradient id="g" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#fff"/><stop offset="1" stop-color="#000"/></linearGradient></defs>
        <g id="leg-back-far"><rect x="20" y="60" width="8" height="36" rx="4" fill="#333"/><circle id="pivot-leg-back-far" cx="24" cy="62" r="1"/></g>
        <g id="body"><ellipse cx="50" cy="55" rx="35" ry="22" fill="url(#g)"/></g>
        <g id="head"><circle cx="80" cy="35" r="15" fill="#888"/>
            <g id="eyes"><circle cx="85" cy="32" r="3" fill="#fff"/><g id="pupils"><circle cx="86" cy="32" r="1.5" fill="#000"/></g></g>
        </g>
        <circle id="pivot-ground" cx="50" cy="96" r="1"/>
    </svg>"##;

    #[test]
    fn parses_parts_pivots_and_ground() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        assert_eq!(rig.ground, Point::new(50.0, 96.0));
        assert_eq!(rig.pivot(Part::LegBackFar), Point::new(24.0, 62.0));
        assert!(rig.has_part(Part::Head) && rig.has_part(Part::Pupils));
        assert!(!rig.has_part(Part::Tail));
        assert!(rig.body_silhouette.is_some());
        let begins = rig
            .items
            .iter()
            .filter(|i| matches!(i, Item::BeginPart(_)))
            .count();
        let ends = rig
            .items
            .iter()
            .filter(|i| matches!(i, Item::EndPart))
            .count();
        assert_eq!(begins, 5);
        assert_eq!(begins, ends);
        // Les pivots ne font pas partie du dessin.
        assert!(rig.bounds.max_y() <= 96.5);
    }

    #[test]
    fn gradients_become_linear_paints() {
        let rig = Rig::parse(SVG.as_bytes()).unwrap();
        let has_linear = rig.items.iter().any(|i| {
            matches!(
                i,
                Item::Fill {
                    paint: Paint::Linear { .. },
                    ..
                }
            )
        });
        assert!(has_linear);
    }
}
