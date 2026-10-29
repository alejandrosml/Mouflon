//! Le « monde » vu par les animaux : écrans, sols et rebords de fenêtres.
//!
//! Repère : points logiques, origine en haut à gauche de l'écran principal,
//! y vers le bas. La couche système convertit depuis le repère de l'OS.

use crate::math::{Rect, Vec2};

/// Identifiant stable d'une surface, conservé tant que la fenêtre existe :
/// c'est ce qui permet à un animal de « voyager » sur une fenêtre qu'on déplace.
pub type SurfaceId = u64;

const FLOOR_ID_BIT: u64 = 1 << 63;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SurfaceKind {
    /// Bas de la zone utile d'un écran (au-dessus du Dock ou de la barre des tâches).
    Floor,
    /// Bord supérieur visible d'une fenêtre.
    Window,
}

/// Segment horizontal sur lequel un animal peut se tenir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Surface {
    pub id: SurfaceId,
    pub kind: SurfaceKind,
    pub x0: f32,
    pub x1: f32,
    pub y: f32,
}

impl Surface {
    #[must_use]
    pub fn width(&self) -> f32 {
        self.x1 - self.x0
    }

    #[must_use]
    pub fn contains_x(&self, x: f32, margin: f32) -> bool {
        x >= self.x0 - margin && x <= self.x1 + margin
    }
}

/// Partie visible du côté vertical d'une fenêtre : elle barre le chemin d'un
/// animal qui marche vers la fenêtre (`windowSide` de desktopPet).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wall {
    /// Fenêtre dont c'est le côté.
    pub window: u64,
    pub x: f32,
    /// Haut et bas de la partie visible.
    pub y0: f32,
    pub y1: f32,
    /// Côté vers lequel il regarde : -1 pour le côté gauche de la fenêtre
    /// (il arrête un animal qui va vers la droite), 1 pour le côté droit.
    pub facing: f32,
}

/// Un côté plus court que ceci (le reste est caché) ne barre rien.
const MIN_WALL: f32 = 4.0;

/// Un écran : son cadre complet et sa zone utile (sans barre des menus ni Dock).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Screen {
    pub frame: Rect,
    pub usable: Rect,
    pub scale: f32,
}

/// Fenêtre d'une autre application, telle que fournie par la couche système,
/// dans l'ordre d'empilement (la première est au premier plan).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowRect {
    pub id: u64,
    pub frame: Rect,
}

/// Réglages du calcul des rebords de fenêtres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LedgeOptions {
    /// Largeur minimale d'un morceau de rebord visible pour qu'on y marche.
    pub min_span: f32,
    /// Hauteur libre exigée entre le haut de la zone utile et le rebord :
    /// une fenêtre plein écran n'offre pas de place pour se tenir dessus.
    pub headroom: f32,
}

impl Default for LedgeOptions {
    fn default() -> Self {
        Self {
            min_span: 28.0,
            headroom: 24.0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct World {
    screens: Vec<Screen>,
    surfaces: Vec<Surface>,
    walls: Vec<Wall>,
    arena: Rect,
    bounds: Rect,
}

impl World {
    #[must_use]
    pub fn new(screens: Vec<Screen>) -> Self {
        let mut world = Self::default();
        world.set_screens(screens);
        world
    }

    /// Remplace les écrans et régénère les sols. Les rebords de fenêtres sont conservés.
    pub fn set_screens(&mut self, screens: Vec<Screen>) {
        self.screens = screens;
        self.arena = self
            .screens
            .iter()
            .map(|s| s.usable)
            .reduce(|a, b| a.union(&b))
            .unwrap_or_default();
        self.bounds = self
            .screens
            .iter()
            .map(|s| s.frame)
            .reduce(|a, b| a.union(&b))
            .unwrap_or_default();
        self.surfaces.retain(|s| s.kind != SurfaceKind::Floor);
        for (i, screen) in self.screens.iter().enumerate() {
            self.surfaces.push(Surface {
                id: FLOOR_ID_BIT | i as u64,
                kind: SurfaceKind::Floor,
                x0: screen.usable.left(),
                x1: screen.usable.right(),
                y: screen.usable.bottom(),
            });
        }
    }

    /// Remplace les rebords et les côtés des fenêtres à partir de la liste
    /// fournie par le système.
    pub fn set_windows(&mut self, windows: &[WindowRect], options: LedgeOptions) {
        let ledges = visible_ledges(windows, &self.screens, options);
        self.surfaces.retain(|s| s.kind == SurfaceKind::Floor);
        self.surfaces.extend(ledges);
        self.walls = visible_walls(windows);
    }

    /// Côtés visibles des fenêtres.
    #[must_use]
    pub fn walls(&self) -> &[Wall] {
        &self.walls
    }

    /// Premier côté de fenêtre que franchit l'avant d'un animal qui va de
    /// l'abscisse `from` à `to`, son corps couvrant les hauteurs `top` à
    /// `bottom`. Seul compte un côté tourné vers lui : on sort librement
    /// d'une fenêtre devant laquelle on se trouve.
    #[must_use]
    pub fn wall_crossed(&self, from: f32, to: f32, top: f32, bottom: f32) -> Option<&Wall> {
        if to == from {
            return None;
        }
        let dir = (to - from).signum();
        self.walls
            .iter()
            .filter(|w| {
                w.facing == -dir
                    && (w.x - from) * dir >= 0.0
                    && (to - w.x) * dir > 0.0
                    && w.y0 < bottom - 1.0
                    && w.y1 > top + 1.0
            })
            .min_by(|a, b| ((a.x - from) * dir).total_cmp(&((b.x - from) * dir)))
    }

    #[must_use]
    pub fn screens(&self) -> &[Screen] {
        &self.screens
    }

    #[must_use]
    pub fn surfaces(&self) -> &[Surface] {
        &self.surfaces
    }

    #[must_use]
    pub fn surface(&self, id: SurfaceId) -> Option<&Surface> {
        self.surfaces.iter().find(|s| s.id == id)
    }

    /// Union des zones utiles de tous les écrans.
    #[must_use]
    pub fn arena(&self) -> Rect {
        self.arena
    }

    /// Union des cadres complets de tous les écrans.
    #[must_use]
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    /// Écran qui contient le point, ou à défaut le plus proche.
    #[must_use]
    pub fn screen_at(&self, p: Vec2) -> Option<&Screen> {
        self.screens
            .iter()
            .find(|s| s.frame.contains(p))
            .or_else(|| {
                self.screens.iter().min_by(|a, b| {
                    let da = (a.frame.center() - p).length();
                    let db = (b.frame.center() - p).length();
                    da.total_cmp(&db)
                })
            })
    }

    /// Surface franchie en descendant de `y0` à `y1` à l'abscisse `x` :
    /// la plus haute dont le niveau est compris dans l'intervalle.
    #[must_use]
    pub fn landing_surface(&self, x: f32, margin: f32, y0: f32, y1: f32) -> Option<&Surface> {
        self.surfaces
            .iter()
            .filter(|s| s.contains_x(x, margin) && s.y >= y0 - 0.01 && s.y <= y1 + 0.01)
            .min_by(|a, b| a.y.total_cmp(&b.y))
    }

    /// Surface la plus proche sous le point (pour l'ombre portée pendant un saut).
    #[must_use]
    pub fn surface_below(&self, p: Vec2, margin: f32) -> Option<&Surface> {
        self.surfaces
            .iter()
            .filter(|s| s.contains_x(p.x, margin) && s.y >= p.y - 0.5)
            .min_by(|a, b| a.y.total_cmp(&b.y))
    }

    /// Sol d'un écran qui s'étend au-dessus de l'abscisse `x`, le plus proche
    /// de la hauteur `y` (pour passer d'un écran à l'autre).
    #[must_use]
    pub fn floor_at(&self, x: f32, y: f32) -> Option<&Surface> {
        self.surfaces
            .iter()
            .filter(|s| s.kind == SurfaceKind::Floor && s.contains_x(x, 0.0))
            .min_by(|a, b| (a.y - y).abs().total_cmp(&(b.y - y).abs()))
    }

    /// Sol de l'écran situé sous l'abscisse `x` (ou du plus proche).
    #[must_use]
    pub fn floor_under(&self, x: f32) -> Option<&Surface> {
        self.surfaces
            .iter()
            .filter(|s| s.kind == SurfaceKind::Floor)
            .min_by(|a, b| {
                let da = if a.contains_x(x, 0.0) {
                    0.0
                } else {
                    (a.x0 - x).abs().min((a.x1 - x).abs())
                };
                let db = if b.contains_x(x, 0.0) {
                    0.0
                } else {
                    (b.x0 - x).abs().min((b.x1 - x).abs())
                };
                da.total_cmp(&db)
            })
    }
}

/// Calcule les morceaux visibles du bord supérieur de chaque fenêtre.
///
/// Une fenêtre placée devant en masque une partie : on soustrait sa largeur
/// au rebord si elle couvre la hauteur de ce rebord. On ne garde que les
/// morceaux assez larges et assez bas sous la barre des menus.
#[must_use]
pub fn visible_ledges(
    windows: &[WindowRect],
    screens: &[Screen],
    options: LedgeOptions,
) -> Vec<Surface> {
    let mut out = Vec::new();
    for (index, window) in windows.iter().enumerate() {
        let y = window.frame.top();
        let Some(screen) = screens.iter().find(|s| {
            s.frame
                .contains(Vec2::new(window.frame.center().x, y + 1.0))
        }) else {
            continue;
        };
        if y < screen.usable.top() + options.headroom || y >= screen.usable.bottom() - 1.0 {
            continue;
        }
        let mut spans = vec![(
            window.frame.left().max(screen.usable.left()),
            window.frame.right().min(screen.usable.right()),
        )];
        for front in &windows[..index] {
            let f = front.frame;
            // On masque aussi un peu au-dessus du rebord : l'animal s'y tient debout.
            if f.top() <= y + 1.0 && f.bottom() >= y - 8.0 {
                spans = subtract_span(&spans, f.left(), f.right());
                if spans.is_empty() {
                    break;
                }
            }
        }
        for (span_index, (x0, x1)) in spans.into_iter().enumerate() {
            if x1 - x0 >= options.min_span {
                out.push(Surface {
                    id: (window.id << 8) | (span_index as u64 & 0xFF),
                    kind: SurfaceKind::Window,
                    x0,
                    x1,
                    y,
                });
            }
        }
    }
    out
}

/// Calcule les parties visibles des côtés de chaque fenêtre : une fenêtre
/// placée devant cache la partie d'un côté qu'elle recouvre.
#[must_use]
pub fn visible_walls(windows: &[WindowRect]) -> Vec<Wall> {
    let mut out = Vec::new();
    for (index, window) in windows.iter().enumerate() {
        let f = window.frame;
        for (x, facing) in [(f.left(), -1.0), (f.right(), 1.0)] {
            let mut spans = vec![(f.top(), f.bottom())];
            for front in &windows[..index] {
                let g = front.frame;
                if g.left() < x && g.right() > x {
                    spans = subtract_span(&spans, g.top(), g.bottom());
                    if spans.is_empty() {
                        break;
                    }
                }
            }
            out.extend(
                spans
                    .into_iter()
                    .filter(|(y0, y1)| y1 - y0 >= MIN_WALL)
                    .map(|(y0, y1)| Wall {
                        window: window.id,
                        x,
                        y0,
                        y1,
                        facing,
                    }),
            );
        }
    }
    out
}

fn subtract_span(spans: &[(f32, f32)], cut0: f32, cut1: f32) -> Vec<(f32, f32)> {
    let mut out = Vec::with_capacity(spans.len() + 1);
    for &(a, b) in spans {
        if cut1 <= a || cut0 >= b {
            out.push((a, b));
            continue;
        }
        if cut0 > a {
            out.push((a, cut0));
        }
        if cut1 < b {
            out.push((cut1, b));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen() -> Screen {
        Screen {
            frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }
    }

    #[test]
    fn floor_is_bottom_of_usable_area() {
        let world = World::new(vec![screen()]);
        let floor = world.floor_under(100.0).unwrap();
        assert_eq!(floor.y, 825.0);
        assert_eq!(floor.kind, SurfaceKind::Floor);
    }

    #[test]
    fn front_window_hides_part_of_ledge() {
        let windows = [
            WindowRect {
                id: 1,
                frame: Rect::new(300.0, 150.0, 200.0, 300.0),
            },
            WindowRect {
                id: 2,
                frame: Rect::new(100.0, 200.0, 600.0, 400.0),
            },
        ];
        let ledges = visible_ledges(&windows, &[screen()], LedgeOptions::default());
        let back: Vec<_> = ledges
            .iter()
            .filter(|s| s.id >> 8 == 2)
            .map(|s| (s.x0, s.x1))
            .collect();
        assert_eq!(back, vec![(100.0, 300.0), (500.0, 700.0)]);
    }

    #[test]
    fn maximized_window_offers_no_ledge() {
        let windows = [WindowRect {
            id: 3,
            frame: Rect::new(0.0, 25.0, 1440.0, 800.0),
        }];
        assert!(visible_ledges(&windows, &[screen()], LedgeOptions::default()).is_empty());
    }

    #[test]
    fn front_window_hides_part_of_a_side() {
        let windows = [
            WindowRect {
                id: 1,
                frame: Rect::new(50.0, 300.0, 200.0, 100.0),
            },
            WindowRect {
                id: 2,
                frame: Rect::new(100.0, 200.0, 600.0, 400.0),
            },
        ];
        let walls = visible_walls(&windows);
        let back_left: Vec<_> = walls
            .iter()
            .filter(|w| w.window == 2 && w.facing < 0.0)
            .map(|w| (w.x, w.y0, w.y1))
            .collect();
        assert_eq!(
            back_left,
            vec![(100.0, 200.0, 300.0), (100.0, 400.0, 600.0)]
        );
        let back_right: Vec<_> = walls
            .iter()
            .filter(|w| w.window == 2 && w.facing > 0.0)
            .map(|w| (w.x, w.y0, w.y1))
            .collect();
        assert_eq!(back_right, vec![(700.0, 200.0, 600.0)]);
        // La fenêtre de devant garde ses deux côtés entiers.
        assert_eq!(walls.iter().filter(|w| w.window == 1).count(), 2);
    }

    #[test]
    fn a_side_stops_only_who_walks_into_it() {
        let mut world = World::new(vec![screen()]);
        world.set_windows(
            &[WindowRect {
                id: 7,
                frame: Rect::new(500.0, 600.0, 400.0, 400.0),
            }],
            LedgeOptions::default(),
        );
        // Au sol (corps de 765 à 825), vers la droite : le côté gauche l'arrête.
        let wall = world.wall_crossed(498.0, 501.0, 765.0, 825.0).unwrap();
        assert_eq!((wall.x, wall.facing), (500.0, -1.0));
        // Vers la gauche, de l'autre côté : le côté droit.
        let wall = world.wall_crossed(902.0, 899.0, 765.0, 825.0).unwrap();
        assert_eq!((wall.x, wall.facing), (900.0, 1.0));
        // Devant la fenêtre, il en sort librement.
        assert!(world.wall_crossed(899.0, 902.0, 765.0, 825.0).is_none());
        assert!(world.wall_crossed(501.0, 498.0, 765.0, 825.0).is_none());
        // Au-dessus de la fenêtre (sur un rebord plus haut), rien ne l'arrête.
        assert!(world.wall_crossed(498.0, 501.0, 500.0, 560.0).is_none());
        // Pas encore arrivé au côté.
        assert!(world.wall_crossed(480.0, 483.0, 765.0, 825.0).is_none());
    }

    #[test]
    fn landing_picks_highest_crossed_surface() {
        let mut world = World::new(vec![screen()]);
        world.set_windows(
            &[WindowRect {
                id: 9,
                frame: Rect::new(0.0, 400.0, 500.0, 200.0),
            }],
            LedgeOptions::default(),
        );
        let hit = world.landing_surface(200.0, 0.0, 100.0, 900.0).unwrap();
        assert_eq!(hit.y, 400.0);
        assert!(world.landing_surface(800.0, 0.0, 100.0, 500.0).is_none());
    }
}
