//! Aperçu hors écran de Mouflon.
//!
//! Simule une scène de bureau (fenêtres, Dock, curseur) avec le vrai moteur,
//! dessine chaque image avec le vrai rendu, puis l'exporte en PNG via SVG et
//! resvg. Sert à juger le rendu et la fluidité sans lancer l'app.
//!
//! ```text
//! cargo run -p mouflon-preview --release -- <pet.toml|animations.xml> <dossier> [video|poses|all|icon]
//! cargo run -p mouflon-preview --release -- <animal> <dossier> troupeau [secondes] [nombre]
//! ```
//!
//! Le mode `troupeau` fait apparaître plusieurs animaux à leurs points
//! d'apparition (utile pour les animaux desktopPet et leurs scènes).
//!
//! ```text
//! cargo run -p mouflon-preview --release -- <animal> <dossier> rencontre [comportement de marche]
//! cargo run -p mouflon-preview --release -- <animal> <dossier> planche
//! ```
//!
//! `rencontre` : deux animaux marchent l'un vers l'autre en mode collisions,
//! puis le premier est lancé sur le second. `planche` : toutes les images de
//! chaque comportement, dessinées par Mouflon dans les deux sens, sur la
//! ligne du sol (pour vérifier le point d'appui et le rognage).
#![forbid(unsafe_code)]

use std::f32::consts::TAU;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use mouflon_core::{
    Flock, Input, LedgeOptions, Particle, ParticleKind, Pet, PetEvent, PetView, PoseSignals, Rect,
    Screen, Shadow, Support, Vec2, WindowRect, World, load_pet,
};
use mouflon_render::display::{Command, DisplayList, Paint, Stop, rgba};
use mouflon_render::svg::to_svg;
use mouflon_render::vello::kurbo::{
    Affine, BezPath, Circle, Point, Rect as KRect, RoundedRect, Shape,
};
use mouflon_render::vello::peniko::{Color, Extend};
use mouflon_render::{Frame, PetArt, Style, paint_pet};
use resvg::{tiny_skia, usvg};

type Error = Box<dyn std::error::Error>;

const SCREEN_W: f32 = 960.0;
const SCREEN_H: f32 = 600.0;
const MENU_H: f32 = 26.0;
const DOCK_H: f32 = 64.0;
const SCALE: f64 = 1.5;
const FPS: u32 = 60;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let pet_path = PathBuf::from(
        args.get(1)
            .map_or("pets/mannequin/pet.toml", String::as_str),
    );
    let out_dir = PathBuf::from(args.get(2).map_or("apercu", String::as_str));
    let mode = args.get(3).map_or("all", String::as_str);
    std::fs::create_dir_all(&out_dir)?;

    let def = Arc::new(load_pet(&pet_path)?);
    let mut art = PetArt::load(&def)?;
    // L'aperçu (SVG) garde les tracés vectoriels plutôt que les images cuites.
    art.baked = None;
    println!(
        "{} : {} comportements, {:.0}×{:.0} pt",
        def.meta.name,
        def.behaviors.len(),
        art.width,
        art.height
    );

    if mode == "poses" || mode == "all" {
        let path = out_dir.join("poses.png");
        render_poses(&art, &path)?;
        println!("planche de poses : {}", path.display());
    }
    if mode == "video" || mode == "all" || mode == "troupeau" {
        let frames = out_dir.join("frames");
        std::fs::create_dir_all(&frames)?;
        let options = if mode == "troupeau" {
            Scene {
                duration: args.get(4).and_then(|a| a.parse().ok()).unwrap_or(30.0),
                count: args.get(5).and_then(|a| a.parse().ok()).unwrap_or(3),
                spawn_x: None,
            }
        } else {
            Scene {
                duration: 18.0,
                count: 1,
                spawn_x: Some(300.0),
            }
        };
        let count = render_video(&def, &art, &frames, &options)?;
        println!("{count} images dans {}", frames.display());
    }
    if mode == "icon" {
        let path = out_dir.join("AppIcon.png");
        render_icon(&art, &path)?;
        println!("icône : {}", path.display());
    }
    if mode == "rencontre" {
        let frames = out_dir.join("frames");
        std::fs::create_dir_all(&frames)?;
        let walk = args.get(4).map_or_else(
            || {
                ["walk", "marche"]
                    .into_iter()
                    .find(|n| def.behavior_index(n).is_some())
                    .unwrap_or("walk")
                    .to_owned()
            },
            Clone::clone,
        );
        let count = render_meeting(&def, &art, &frames, &walk)?;
        println!("{count} images dans {}", frames.display());
    }
    if mode == "salut" {
        let frames = out_dir.join("frames");
        std::fs::create_dir_all(&frames)?;
        let count = render_greeting(&def, &art, &frames)?;
        println!("{count} images dans {}", frames.display());
    }
    if mode == "planche" {
        for (facing, name) in [(-1.0, "planche-gauche.png"), (1.0, "planche-droite.png")] {
            let path = out_dir.join(name);
            render_sheet(&def, &art, facing, &path)?;
            println!("planche : {}", path.display());
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- rencontre

/// Comme l'app : les animaux se touchent quand leurs dessins se touchent.
fn give_reach(flock: &mut Flock, art: &PetArt) {
    if let Some((front, back)) = art.reach() {
        for pet in flock.pets_mut() {
            pet.set_reach(front, back);
        }
    }
}

/// Deux animaux qui se rentrent dedans (mode collisions), puis un lancer.
fn render_meeting(
    def: &Arc<mouflon_core::PetDef>,
    art: &PetArt,
    dir: &Path,
    walk: &str,
) -> Result<u32, Error> {
    const RATE: u32 = 30;
    let mut world = World::new(vec![screen()]);
    world.set_windows(&[], LedgeOptions::default());
    let mut flock = Flock::new();
    flock.set_collisions(true);
    let a = flock.spawn(Arc::clone(def), 3, &world, Some(400.0));
    let b = flock.spawn(Arc::clone(def), 5, &world, Some(580.0));
    give_reach(&mut flock, art);
    let duration = 11.0;
    let frames = (duration * RATE as f32) as u32;
    let mut grabbed_at: Option<Vec2> = None;
    for frame in 0..frames {
        let t = frame as f32 / RATE as f32;
        let at = |s: f32| (t - s).abs() < 0.5 / RATE as f32;
        // Ils se font face et marchent l'un vers l'autre.
        if at(1.6) {
            for (id, facing) in [(a, 1.0), (b, -1.0)] {
                if let Some(pet) = flock.get_mut(id) {
                    pet.face(facing);
                    pet.force(walk);
                }
            }
        }
        // Puis le premier est attrapé et lancé sur le second.
        let cursor = flock.pets().iter().find(|p| p.id() == a).and_then(|pet| {
            let head = pet.position() + Vec2::new(0.0, -30.0);
            match t {
                t if (6.5..7.2).contains(&t) => Some(head),
                t if (7.2..8.2).contains(&t) => {
                    // Il le soulève, recule, puis le lance à ras du sol vers l'autre.
                    let u = (t - 7.2) / 1.0;
                    let up = Vec2::new(150.0, 420.0);
                    let to = Vec2::new(330.0, 470.0);
                    let start = *grabbed_at.get_or_insert(head);
                    if u < 0.5 {
                        Some(start.lerp(up, ease(u * 2.0)))
                    } else {
                        let v = (u - 0.5) * 2.0;
                        Some(up.lerp(to, v * v))
                    }
                }
                _ => None,
            }
        });
        if let Some(pet) = flock.get_mut(a) {
            if at(7.2)
                && let Some(c) = cursor
            {
                pet.grab(c, &world);
            }
            if pet.is_held()
                && let Some(c) = cursor
            {
                pet.drag_to(c);
            }
            if at(8.2) {
                pet.release(&world);
            }
        }
        let input = Input {
            cursor,
            local_hour: 15.0,
        };
        let alpha = flock.advance(1.0 / RATE as f32, &world, &input);
        let mut list = DisplayList::new(
            (f64::from(SCREEN_W) * SCALE) as u32,
            (f64::from(SCREEN_H) * SCALE) as u32,
        );
        let script = Script {
            w1: Rect::new(-1000.0, 0.0, 1.0, 1.0),
            w2: Rect::new(-1000.0, 0.0, 1.0, 1.0),
        };
        paint_desktop(&mut list, &script, 0.0);
        for pet in flock.pets() {
            paint_pet(
                art,
                &pet.view(alpha),
                &Frame {
                    origin: Vec2::ZERO,
                    scale: SCALE,
                },
                &Style::default(),
                &mut list,
            );
        }
        if let Some(c) = cursor {
            let held = flock.pets().iter().any(Pet::is_held);
            paint_cursor(&mut list, c, held);
        }
        rasterize(&list, &dir.join(format!("frame_{frame:05}.png")))?;
        if frame % RATE == 0 {
            let states: Vec<String> = flock
                .pets()
                .iter()
                .map(|p| {
                    format!(
                        "{}@({:.0},{:.0})",
                        p.behavior_name(),
                        p.position().x,
                        p.position().y
                    )
                })
                .collect();
            println!("{t:4.1} s : {}", states.join(", "));
        }
    }
    Ok(frames)
}

// ---------------------------------------------------------------- salut

/// Sans mode collisions : deux animaux se rencontrent et réagissent, un
/// troisième bute contre le côté d'une fenêtre, puis « Tous ensemble » et
/// « Au revoir ».
fn render_greeting(
    def: &Arc<mouflon_core::PetDef>,
    art: &PetArt,
    dir: &Path,
) -> Result<u32, Error> {
    const RATE: u32 = 30;
    let mut world = World::new(vec![screen()]);
    // Une fenêtre qui descend jusqu'au Dock, à droite.
    let window = Rect::new(700.0, 300.0, 240.0, SCREEN_H - DOCK_H - 300.0 + 20.0);
    world.set_windows(
        &[WindowRect {
            id: 1,
            frame: window,
        }],
        LedgeOptions::default(),
    );
    let walk = ["walk", "marche"]
        .into_iter()
        .find(|n| def.behavior_index(n).is_some())
        .unwrap_or("walk");
    let mut flock = Flock::new();
    let a = flock.spawn(Arc::clone(def), 3, &world, Some(250.0));
    let b = flock.spawn(Arc::clone(def), 5, &world, Some(430.0));
    let c = flock.spawn(Arc::clone(def), 8, &world, Some(600.0));
    give_reach(&mut flock, art);
    let input = Input {
        cursor: None,
        local_hour: 15.0,
    };
    // Une fois tout le monde posé (la chute d'un animal desktopPet est
    // lente), les deux premiers marchent l'un vers l'autre et le troisième
    // vers la fenêtre ; puis « Tous ensemble » et « Au revoir ».
    let landing = [
        "fall",
        "fall soft",
        "fall hard",
        "arrivee",
        "atterrit",
        "atterrit_sonne",
    ];
    let mut start: Option<f32> = None;
    let mut met = false;
    let mut frame = 0;
    loop {
        let t = frame as f32 / RATE as f32;
        let settled = flock.pets().iter().all(|p| {
            matches!(p.support(), Support::Ground { .. }) && !landing.contains(&p.behavior_name())
        });
        if start.is_none() && t >= 1.0 && (settled || t > 12.0) {
            start = Some(t);
            for (id, facing) in [(a, 1.0), (b, -1.0), (c, 1.0)] {
                if let Some(pet) = flock.get_mut(id) {
                    pet.face(facing);
                    pet.force(walk);
                }
            }
        }
        // Jusqu'à leur rencontre, les deux premiers continuent de marcher.
        if start.is_some() && !met {
            for (id, facing) in [(a, 1.0), (b, -1.0)] {
                if let Some(pet) = flock.get_mut(id)
                    && pet.behavior_name() != walk
                {
                    pet.face(facing);
                    pet.force(walk);
                }
            }
        }
        let since = start.map_or(-1.0, |s| t - s);
        let at = |s: f32| (since - s).abs() < 0.5 / RATE as f32;
        if since > 12.5 || flock.pets().is_empty() && since > 0.0 {
            break;
        }
        if at(7.0) {
            println!(
                "{t:4.1} s : tous ensemble ({})",
                flock.together(&world, &input)
            );
        }
        if at(9.5) {
            println!("{t:4.1} s : au revoir");
            flock.leave_all(&world, &input);
        }
        let alpha = flock.advance(1.0 / RATE as f32, &world, &input);
        for pet in flock.pets_mut() {
            met |= pet
                .take_events()
                .iter()
                .any(|e| matches!(e, PetEvent::Met { .. }));
        }
        let mut list = DisplayList::new(
            (f64::from(SCREEN_W) * SCALE) as u32,
            (f64::from(SCREEN_H) * SCALE) as u32,
        );
        let script = Script {
            w1: window,
            w2: Rect::new(-1000.0, 0.0, 1.0, 1.0),
        };
        paint_desktop(&mut list, &script, 0.0);
        for pet in flock.pets() {
            paint_pet(
                art,
                &pet.view(alpha),
                &Frame {
                    origin: Vec2::ZERO,
                    scale: SCALE,
                },
                &Style::default(),
                &mut list,
            );
        }
        rasterize(&list, &dir.join(format!("frame_{frame:05}.png")))?;
        if frame % (RATE / 2) == 0 {
            let states: Vec<String> = flock
                .pets()
                .iter()
                .map(|p| {
                    format!(
                        "{}@{:.0}{}",
                        p.behavior_name(),
                        p.position().x,
                        if p.is_leaving() { " (s'en va)" } else { "" }
                    )
                })
                .collect();
            println!("{t:4.1} s : {}", states.join(", "));
        }
        frame += 1;
    }
    Ok(frame)
}

// ---------------------------------------------------------------- planche

/// Toutes les images de chaque comportement, une ligne par comportement.
fn render_sheet(
    def: &mouflon_core::PetDef,
    art: &PetArt,
    facing: f32,
    path: &Path,
) -> Result<(), Error> {
    let rows: Vec<(&str, &mouflon_core::def::SpriteClip)> = def
        .behaviors
        .iter()
        .filter_map(|b| Some((b.name.as_str(), b.clip.as_ref()?)))
        .collect();
    let cell_w = art.width.max(art.body.0 * 2.2) * 1.1;
    let cell_h = art.height * 1.25;
    let cols = rows.iter().map(|(_, c)| c.frames.len()).max().unwrap_or(1);
    let zoom = 2.0;
    let width = (cell_w * cols as f32 * zoom as f32).ceil() as u32;
    let height = (cell_h * rows.len().max(1) as f32 * zoom as f32).ceil() as u32;
    let mut list = DisplayList::new(width, height);
    list.fill(
        rounded(Rect::new(0.0, 0.0, width as f32, height as f32), 0.0),
        Affine::IDENTITY,
        Paint::Solid(rgba(236, 240, 232, 1.0)),
    );
    for (row, (name, clip)) in rows.iter().enumerate() {
        println!(
            "ligne {:2} : {name} ({} images)",
            row + 1,
            clip.frames.len()
        );
        let ground_y = (row as f32 + 0.86) * cell_h;
        list.fill(
            rounded(
                Rect::new(0.0, ground_y * zoom as f32, width as f32, 1.0),
                0.0,
            ),
            Affine::IDENTITY,
            Paint::Solid(rgba(220, 120, 120, 0.6)),
        );
        for (col, frame) in clip.frames.iter().enumerate() {
            let ground = Vec2::new((col as f32 + 0.5) * cell_w, ground_y);
            let view = PetView {
                id: 0,
                ground,
                facing,
                pose: PoseSignals::default(),
                shadow: Some(Shadow {
                    y: ground.y,
                    distance: 0.0,
                }),
                sprite_frame: Some(*frame),
                sprite_offset_y: 0.0,
                opacity: 1.0,
                particles: &[],
                support: Support::Ground { surface: 0 },
                behavior: name,
                grip: None,
            };
            paint_pet(
                art,
                &view,
                &Frame {
                    origin: Vec2::ZERO,
                    scale: zoom,
                },
                &Style::default(),
                &mut list,
            );
        }
    }
    rasterize(&list, path)
}

/// Icône d'application 1024×1024 : carré arrondi façon macOS et personnage au centre.
fn render_icon(art: &PetArt, path: &Path) -> Result<(), Error> {
    let size = 1024u32;
    let mut list = DisplayList::new(size, size);
    // Gabarit des icônes macOS : carré de 824 px centré, rayon ≈ 185 px.
    let tile = Rect::new(100.0, 100.0, 824.0, 824.0);
    list.push(Command::BlurredRect {
        rect: KRect::new(100.0, 118.0, 924.0, 942.0),
        radius: 185.0,
        std_dev: 22.0,
        color: Color::BLACK.with_alpha(0.28),
        transform: Affine::IDENTITY,
    });
    list.fill(
        rounded(tile, 185.0),
        Affine::IDENTITY,
        Paint::Linear {
            start: Point::new(0.0, 100.0),
            end: Point::new(0.0, 924.0),
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: rgba(226, 255, 245, 1.0),
                },
                Stop {
                    offset: 0.55,
                    color: rgba(150, 228, 210, 1.0),
                },
                Stop {
                    offset: 1.0,
                    color: rgba(62, 168, 160, 1.0),
                },
            ],
            extend: Extend::Pad,
        },
    );
    list.fill(
        Circle::new((400.0, 260.0), 420.0).to_path(0.1),
        Affine::IDENTITY,
        Paint::Radial {
            start: Point::new(400.0, 260.0),
            start_radius: 0.0,
            end: Point::new(400.0, 260.0),
            end_radius: 420.0,
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: Color::WHITE.with_alpha(0.35),
                },
                Stop {
                    offset: 1.0,
                    color: Color::WHITE.with_alpha(0.0),
                },
            ],
            extend: Extend::Pad,
        },
    );
    let zoom = 560.0 / f64::from(art.height.max(art.width));
    let ground = Vec2::new(512.0, 760.0);
    let frame = Frame {
        origin: Vec2::new(-ground.x / zoom as f32, -ground.y / zoom as f32),
        scale: zoom,
    };
    let view = PetView {
        id: 0,
        ground: Vec2::ZERO,
        facing: 1.0,
        pose: PoseSignals {
            look: Vec2::new(0.45, 0.1),
            breath: 0.3,
            ..PoseSignals::default()
        },
        shadow: Some(Shadow {
            y: 0.0,
            distance: 0.0,
        }),
        sprite_frame: Some(0),
        sprite_offset_y: 0.0,
        opacity: 1.0,
        particles: &[],
        support: Support::Air,
        behavior: "",
        grip: None,
    };
    paint_pet(art, &view, &frame, &Style::default(), &mut list);
    rasterize(&list, path)
}

// ---------------------------------------------------------------- scénario

fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

struct Script {
    w1: Rect,
    w2: Rect,
}

impl Script {
    fn back_window(&self, t: f32) -> Rect {
        // La fenêtre du fond glisse vers la droite : le personnage voyage avec elle.
        let shift = 90.0 * ease((t - 3.0) / 1.6);
        Rect::new(self.w1.x + shift, self.w1.y, self.w1.w, self.w1.h)
    }

    fn windows(&self, t: f32) -> Vec<WindowRect> {
        vec![
            WindowRect {
                id: 2,
                frame: self.w2,
            },
            WindowRect {
                id: 1,
                frame: self.back_window(t),
            },
        ]
    }
}

fn screen() -> Screen {
    Screen {
        frame: Rect::new(0.0, 0.0, SCREEN_W, SCREEN_H),
        usable: Rect::new(0.0, MENU_H, SCREEN_W, SCREEN_H - MENU_H - DOCK_H),
        scale: SCALE as f32,
    }
}

/// Réglages d'une scène d'aperçu.
struct Scene {
    duration: f32,
    count: u32,
    /// Abscisse d'apparition ; sinon les points d'apparition de l'animal.
    spawn_x: Option<f32>,
}

fn render_video(
    def: &Arc<mouflon_core::PetDef>,
    art: &PetArt,
    dir: &Path,
    scene: &Scene,
) -> Result<u32, Error> {
    let script = Script {
        w1: Rect::new(110.0, 300.0, 400.0, 220.0),
        w2: Rect::new(560.0, 190.0, 340.0, 300.0),
    };
    let mut world = World::new(vec![screen()]);
    world.set_windows(&script.windows(0.0), LedgeOptions::default());
    let mut flock = Flock::new();
    let first = flock.spawn(Arc::clone(def), 7, &world, scene.spawn_x);
    for i in 1..scene.count {
        flock.spawn(
            Arc::clone(def),
            7 + u64::from(i) * 7919,
            &world,
            scene.spawn_x,
        );
    }

    let frames = (scene.duration * FPS as f32) as u32;
    let mut grab_start: Option<Vec2> = None;
    let mut cursor_path: Option<(Vec2, Vec2)> = None;

    for frame in 0..frames {
        let t = frame as f32 / FPS as f32;
        world.set_windows(&script.windows(t), LedgeOptions::default());

        // Curseur : il s'approche du premier animal, l'attrape et le lance.
        let cursor = flock
            .pets()
            .iter()
            .find(|p| p.id() == first)
            .and_then(|pet| cursor_at(t, pet, &mut grab_start, &mut cursor_path));
        let input = Input {
            cursor,
            local_hour: 15.0,
        };

        let alpha = flock.advance(1.0 / FPS as f32, &world, &input);
        if let Some(pet) = flock.get_mut(first) {
            script_events(t, pet, &world, &input, cursor);
        }

        let mut list = DisplayList::new(
            (f64::from(SCREEN_W) * SCALE) as u32,
            (f64::from(SCREEN_H) * SCALE) as u32,
        );
        paint_desktop(&mut list, &script, t);
        for pet in flock.pets() {
            paint_pet(
                art,
                &pet.view(alpha),
                &Frame {
                    origin: Vec2::ZERO,
                    scale: SCALE,
                },
                &Style::default(),
                &mut list,
            );
        }
        if let Some(c) = cursor {
            let held = flock.pets().iter().any(Pet::is_held);
            paint_cursor(&mut list, c, held);
        }
        rasterize(&list, &dir.join(format!("frame_{frame:05}.png")))?;
    }
    Ok(frames)
}

fn cursor_at(
    t: f32,
    pet: &Pet,
    grab_start: &mut Option<Vec2>,
    path: &mut Option<(Vec2, Vec2)>,
) -> Option<Vec2> {
    let head = pet.position() + Vec2::new(0.0, -44.0);
    if (5.0..6.6).contains(&t) {
        // Le curseur arrive par la droite et s'arrête au-dessus du personnage.
        let from = Vec2::new(SCREEN_W - 60.0, 140.0);
        let k = ease((t - 5.0) / 1.2);
        return Some(
            from.lerp(head + Vec2::new(30.0, -40.0), k)
                .lerp(head, ease((t - 6.2) / 0.4)),
        );
    }
    if (6.6..7.75).contains(&t) {
        let start = *grab_start.get_or_insert(head);
        let end = Vec2::new(500.0, 110.0);
        *path = Some((start, end));
        // Un petit arc, puis un geste vif au moment du lâcher : l'animal part avec élan.
        let u = ((t - 6.6) / 1.15).clamp(0.0, 1.0);
        let k = if u < 0.8 {
            0.5 * ease(u / 0.8)
        } else {
            0.5 + 0.5 * (u - 0.8) / 0.2
        };
        let arc = Vec2::new(0.0, -60.0 * (k * std::f32::consts::PI).sin());
        return Some(start.lerp(end, k) + arc);
    }
    None
}

fn script_events(t: f32, pet: &mut Pet, world: &World, input: &Input, cursor: Option<Vec2>) {
    let at = |s: f32| (t - s).abs() < 0.5 / FPS as f32;
    if at(1.0) {
        pet.force("walk");
    }
    if at(5.0) {
        pet.force("look");
    }
    if at(6.6)
        && let Some(c) = cursor
    {
        pet.grab(c, world);
    }
    if (6.6..7.75).contains(&t)
        && let Some(c) = cursor
    {
        pet.drag_to(c);
    }
    if at(7.75) {
        pet.release(world);
    }
    if at(10.2) && matches!(pet.support(), Support::Ground { .. }) {
        pet.force("walk");
    }
    if at(12.4) {
        pet.force("sit");
    }
    if at(13.6) {
        pet.force("sleep");
    }
    if at(17.0) {
        pet.poke(world, input);
    }
}

// ---------------------------------------------------------------- décor

fn rounded(r: Rect, radius: f64) -> BezPath {
    RoundedRect::new(
        f64::from(r.x),
        f64::from(r.y),
        f64::from(r.right()),
        f64::from(r.bottom()),
        radius,
    )
    .to_path(0.05)
}

fn world_to_px() -> Affine {
    Affine::scale(SCALE)
}

fn paint_desktop(list: &mut DisplayList, script: &Script, t: f32) {
    let w = Affine::scale(SCALE);
    let screen = Rect::new(0.0, 0.0, SCREEN_W, SCREEN_H);
    list.fill(
        rounded(screen, 0.0),
        w,
        Paint::Linear {
            start: Point::new(0.0, 0.0),
            end: Point::new(f64::from(SCREEN_W) * 0.4, f64::from(SCREEN_H)),
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: rgba(40, 60, 120, 1.0),
                },
                Stop {
                    offset: 0.55,
                    color: rgba(108, 96, 176, 1.0),
                },
                Stop {
                    offset: 1.0,
                    color: rgba(232, 150, 170, 1.0),
                },
            ],
            extend: Extend::Pad,
        },
    );
    list.fill(
        Circle::new((720.0, 120.0), 420.0).to_path(0.1),
        w,
        Paint::Radial {
            start: Point::new(720.0, 120.0),
            start_radius: 0.0,
            end: Point::new(720.0, 120.0),
            end_radius: 420.0,
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: rgba(150, 190, 255, 0.45),
                },
                Stop {
                    offset: 1.0,
                    color: rgba(150, 190, 255, 0.0),
                },
            ],
            extend: Extend::Pad,
        },
    );
    // Barre des menus.
    list.fill(
        rounded(Rect::new(0.0, 0.0, SCREEN_W, MENU_H), 0.0),
        w,
        Paint::Solid(Color::WHITE.with_alpha(0.16)),
    );

    paint_window(list, script.back_window(t));
    paint_window(list, script.w2);

    // Dock.
    let dock = Rect::new(
        SCREEN_W / 2.0 - 250.0,
        SCREEN_H - DOCK_H + 8.0,
        500.0,
        DOCK_H - 16.0,
    );
    list.push(Command::BlurredRect {
        rect: KRect::new(
            f64::from(dock.x),
            f64::from(dock.y) + 6.0,
            f64::from(dock.right()),
            f64::from(dock.bottom()) + 6.0,
        ),
        radius: 18.0,
        std_dev: 10.0,
        color: Color::BLACK.with_alpha(0.22),
        transform: w,
    });
    list.fill(
        rounded(dock, 18.0),
        w,
        Paint::Solid(Color::WHITE.with_alpha(0.28)),
    );
    let icons = [
        (255, 94, 87),
        (255, 189, 46),
        (40, 200, 64),
        (64, 156, 255),
        (175, 82, 222),
        (255, 105, 180),
        (90, 200, 250),
    ];
    for (i, (r, g, b)) in icons.into_iter().enumerate() {
        let x = dock.x + 22.0 + i as f32 * 66.0;
        let icon = Rect::new(x, dock.y + 6.0, 36.0, 36.0);
        list.fill(
            rounded(icon, 9.0),
            w,
            Paint::Linear {
                start: Point::new(0.0, f64::from(icon.y)),
                end: Point::new(0.0, f64::from(icon.bottom())),
                stops: vec![
                    Stop {
                        offset: 0.0,
                        color: rgba(r, g, b, 1.0),
                    },
                    Stop {
                        offset: 1.0,
                        color: rgba((r / 4) * 3, (g / 4) * 3, (b / 4) * 3, 1.0),
                    },
                ],
                extend: Extend::Pad,
            },
        );
    }
}

fn paint_window(list: &mut DisplayList, r: Rect) {
    let w = world_to_px();
    list.push(Command::BlurredRect {
        rect: KRect::new(
            f64::from(r.x),
            f64::from(r.y) + 12.0,
            f64::from(r.right()),
            f64::from(r.bottom()) + 12.0,
        ),
        radius: 16.0,
        std_dev: 18.0,
        color: Color::BLACK.with_alpha(0.32),
        transform: w,
    });
    list.fill(rounded(r, 16.0), w, Paint::Solid(rgba(246, 247, 251, 1.0)));
    list.fill(
        rounded(Rect::new(r.x, r.y, r.w, 40.0), 16.0),
        w,
        Paint::Linear {
            start: Point::new(0.0, f64::from(r.y)),
            end: Point::new(0.0, f64::from(r.y) + 40.0),
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: rgba(236, 238, 245, 1.0),
                },
                Stop {
                    offset: 1.0,
                    color: rgba(246, 247, 251, 1.0),
                },
            ],
            extend: Extend::Pad,
        },
    );
    for (i, (cr, cg, cb)) in [(255, 95, 87), (254, 188, 46), (40, 200, 64)]
        .into_iter()
        .enumerate()
    {
        let c = Point::new(
            f64::from(r.x) + 20.0 + i as f64 * 20.0,
            f64::from(r.y) + 20.0,
        );
        list.fill(
            Circle::new(c, 6.0).to_path(0.05),
            w,
            Paint::Solid(rgba(cr, cg, cb, 1.0)),
        );
    }
    let mut y = r.y + 64.0;
    let mut k = 0;
    while y + 14.0 < r.bottom() - 16.0 {
        let width = r.w * [0.72, 0.55, 0.8, 0.4, 0.66][k % 5] - 32.0;
        list.fill(
            rounded(Rect::new(r.x + 24.0, y, width, 10.0), 5.0),
            w,
            Paint::Solid(rgba(222, 226, 236, 1.0)),
        );
        y += 26.0;
        k += 1;
    }
}

fn paint_cursor(list: &mut DisplayList, c: Vec2, grabbing: bool) {
    let mut path = BezPath::new();
    if grabbing {
        list.fill(
            Circle::new((f64::from(c.x), f64::from(c.y)), 9.0).to_path(0.05),
            world_to_px(),
            Paint::Solid(Color::WHITE.with_alpha(0.95)),
        );
        list.fill(
            Circle::new((f64::from(c.x), f64::from(c.y)), 6.5).to_path(0.05),
            world_to_px(),
            Paint::Solid(rgba(30, 30, 40, 0.9)),
        );
        return;
    }
    let p = |x: f64, y: f64| Point::new(f64::from(c.x) + x, f64::from(c.y) + y);
    path.move_to(p(0.0, 0.0));
    path.line_to(p(0.0, 18.0));
    path.line_to(p(4.6, 13.8));
    path.line_to(p(8.2, 21.0));
    path.line_to(p(11.0, 19.6));
    path.line_to(p(7.6, 12.8));
    path.line_to(p(13.4, 12.8));
    path.close_path();
    list.push(Command::Stroke {
        path: path.clone(),
        transform: world_to_px(),
        paint: Paint::Solid(Color::WHITE),
        brush_transform: None,
        style: mouflon_render::display::StrokeStyle {
            width: 2.4,
            cap: mouflon_render::display::LineCap::Round,
            join: mouflon_render::display::LineJoin::Round,
            miter_limit: 4.0,
        },
    });
    list.fill(path, world_to_px(), Paint::Solid(rgba(20, 20, 24, 1.0)));
}

// ---------------------------------------------------------------- poses

fn render_poses(art: &PetArt, path: &Path) -> Result<(), Error> {
    let cell_w = 420.0_f32;
    let cell_h = 380.0_f32;
    let (cols, rows) = (3u32, 2u32);
    let zoom = 3.4_f64;
    let width = (cell_w * cols as f32) as u32;
    let height = (cell_h * rows as f32) as u32;
    let mut list = DisplayList::new(width, height);
    list.fill(
        rounded(Rect::new(0.0, 0.0, width as f32, height as f32), 0.0),
        Affine::IDENTITY,
        Paint::Linear {
            start: Point::new(0.0, 0.0),
            end: Point::new(0.0, f64::from(height)),
            stops: vec![
                Stop {
                    offset: 0.0,
                    color: rgba(244, 245, 250, 1.0),
                },
                Stop {
                    offset: 1.0,
                    color: rgba(226, 229, 240, 1.0),
                },
            ],
            extend: Extend::Pad,
        },
    );

    let bubbles = [
        Particle {
            kind: ParticleKind::Bubble,
            pos: Vec2::new(30.0, -70.0),
            vel: Vec2::ZERO,
            age: 1.0,
            life: 3.0,
            size: 4.0,
            phase: 0.0,
        },
        Particle {
            kind: ParticleKind::Bubble,
            pos: Vec2::new(44.0, -96.0),
            vel: Vec2::ZERO,
            age: 1.9,
            life: 3.0,
            size: 3.4,
            phase: 0.0,
        },
    ];
    let dust: Vec<Particle> = (0..8)
        .map(|i| {
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            Particle {
                kind: ParticleKind::Dust,
                pos: Vec2::new(side * (18.0 + i as f32 * 5.0), -4.0 - (i % 3) as f32 * 3.0),
                vel: Vec2::ZERO,
                age: 0.18,
                life: 0.7,
                size: 4.0,
                phase: 0.0,
            }
        })
        .collect();

    type PoseCase = (PoseSignals, f32, Option<f32>, Vec<Particle>);
    let base = PoseSignals {
        look: Vec2::new(0.35, 0.05),
        ..PoseSignals::default()
    };
    let cases: Vec<PoseCase> = vec![
        (
            PoseSignals {
                breath: 0.4,
                ..base
            },
            1.0,
            Some(0.0),
            vec![],
        ),
        (
            PoseSignals {
                walk: 1.0,
                gait_phase: TAU * 0.2,
                lean: 0.03,
                ..base
            },
            1.0,
            Some(0.0),
            vec![],
        ),
        (
            PoseSignals {
                squash: 0.3,
                ear: 0.6,
                ..base
            },
            -1.0,
            Some(0.0),
            dust,
        ),
        (
            PoseSignals {
                air: 1.0,
                squash: -0.07,
                lean: 0.22,
                ear: -0.55,
                look: Vec2::new(0.6, 0.4),
                ..base
            },
            1.0,
            Some(20.0),
            vec![],
        ),
        (
            PoseSignals {
                dangle: 1.0,
                swing: 0.32,
                ear: 0.3,
                look: Vec2::new(0.2, 0.5),
                ..base
            },
            -1.0,
            Some(34.0),
            vec![],
        ),
        (
            PoseSignals {
                sit: 1.0,
                sleep: 1.0,
                blink: 1.0,
                breath: 0.8,
                ear: -0.25,
                ..base
            },
            1.0,
            Some(0.0),
            bubbles.to_vec(),
        ),
    ];

    for (i, (pose, facing, shadow, particles)) in cases.into_iter().enumerate() {
        let col = i as u32 % cols;
        let row = i as u32 / cols;
        let ground_px = Vec2::new(
            col as f32 * cell_w + cell_w / 2.0,
            row as f32 * cell_h + cell_h - 70.0,
        );
        let lift = shadow.unwrap_or(0.0);
        let frame = Frame {
            origin: Vec2::new(-ground_px.x / zoom as f32, -ground_px.y / zoom as f32),
            scale: zoom,
        };
        let particles: Vec<Particle> = particles
            .into_iter()
            .map(|p| Particle {
                pos: p.pos + Vec2::new(0.0, -lift),
                ..p
            })
            .collect();
        let view = PetView {
            id: i as u32,
            ground: Vec2::new(0.0, -lift),
            facing,
            pose,
            shadow: shadow.map(|d| Shadow {
                y: 0.0,
                distance: d,
            }),
            sprite_frame: None,
            sprite_offset_y: 0.0,
            opacity: 1.0,
            particles: &particles,
            support: Support::Air,
            behavior: "",
            grip: None,
        };
        paint_pet(art, &view, &frame, &Style::default(), &mut list);
    }
    rasterize(&list, path)
}

// ---------------------------------------------------------------- export

fn rasterize(list: &DisplayList, path: &Path) -> Result<(), Error> {
    let svg = to_svg(list);
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default())?;
    let mut pixmap =
        tiny_skia::Pixmap::new(list.width, list.height).ok_or("taille d'image invalide")?;
    resvg::render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(path)?;
    Ok(())
}
