//! Boucle de l'application : simulation, fenêtres des animaux, entrées et menu.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Timelike;
use mouflon_core::{
    Activity, Flock, Input, LedgeOptions, PetEvent, PetView, Rect, Screen, Support, Vec2,
    WindowRect, World, exact_dock,
};
use mouflon_platform as platform;
use mouflon_render::{Canvas, DisplayList, Frame, Look, PetArt, SpriteMotion, Style, paint_pet};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy};
use winit::monitor::MonitorHandle;
use winit::window::{Theme, Window, WindowId, WindowLevel};

use crate::gpu::{Gpu, Job, PetSurface};
use crate::library::{Entry, Library};
use crate::menu::Action;
use crate::settings::{Paths, Settings};
use crate::watch::{Snapshot, Watcher};

/// Relevé des écrans (le reste se relève sur un fil à part).
const SCREEN_POLL: Duration = Duration::from_secs(2);
/// Au lancement, les fenêtres des animaux restent vides un court instant :
/// le temps que macOS les affiche et que tout soit en route avant que le
/// premier animal ne tombe.
const SETTLE: Duration = Duration::from_millis(300);
/// Sans battement de l'horloge de l'écran depuis ce délai (écran en veille,
/// débranché…), la minuterie prend le relais ; au-delà du second, l'écran
/// dort sans doute : on ralentit, et l'horloge est recréée de temps en temps.
const CLOCK_SILENT: Duration = Duration::from_millis(150);
const CLOCK_ASLEEP: Duration = Duration::from_secs(1);
/// Cadences quand les animaux se reposent ou dorment (images par seconde).
const IDLE_FPS: f64 = 30.0;
const ASLEEP_FPS: f64 = 20.0;

#[derive(Debug)]
pub enum UserEvent {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Menu(crate::menu::MenuEvent),
    /// Battement de l'horloge de l'écran (voir `App::beat`).
    Frame,
    /// Le dessin de cet animal (identifiant) est prêt.
    ArtReady(String),
}

/// Ce qu'un essai fait de lui-même à mi-parcours, pour passer aussi par un
/// départ et une arrivée (une fenêtre fermée, une autre ouverte).
#[derive(Clone, Copy, Debug)]
enum TrialStep {
    /// Le dernier arrivé de cette espèce dit au revoir.
    Goodbye(usize),
    /// Un animal de cette espèce arrive.
    Add(usize),
}

struct PetWindow {
    pet: u32,
    window: Arc<Window>,
    surface: PetSurface,
    canvas: Canvas,
    art: Arc<PetArt>,
    scale: f64,
    /// Coin haut gauche demandé pour la fenêtre (repère commun, points).
    position: Option<(f64, f64)>,
    /// Coin haut gauche où la fenêtre se trouve vraiment : macOS refuse de
    /// placer une fenêtre plus haut que la barre des menus.
    placed: (f64, f64),
    /// Écart entre les deux au dernier dessin.
    drawn_offset: (f64, f64),
    hittest: bool,
    last_render: Option<Instant>,
    /// Dernière image dessinée d'un animal en sprites.
    sprite: Option<SpriteKey>,
    /// Écran où se trouve la fenêtre, et sa fréquence (images par seconde) :
    /// sur un écran plus lent que l'horloge, on dessine moins souvent (sinon
    /// l'affichage ferait attendre toute l'app).
    monitor: Option<MonitorHandle>,
    hz: f64,
    /// Ce qu'on y dessine à cette image (gardé d'une image à l'autre pour ne
    /// pas réallouer) : toutes les fenêtres partent ensuite au GPU d'un coup.
    list: DisplayList,
}

/// Ce qui change l'image d'un animal en sprites. Le déplacement passe par la
/// position de la fenêtre : on ne redessine que quand l'image elle-même change,
/// quelques fois par seconde au lieu de 120.
#[derive(Clone, Copy, Debug, PartialEq)]
struct SpriteKey {
    frame: Option<u32>,
    offset: i32,
    opacity: u8,
    mirrored: bool,
    shadow: Option<i32>,
}

impl SpriteKey {
    fn of(art: &PetArt, view: &PetView<'_>) -> Option<Self> {
        // En mouvement (écrasement, balancement…) ou avec des particules : chaque image change.
        if !matches!(art.look, Look::Sprites(_))
            || !view.particles.is_empty()
            || !SpriteMotion::of(view, art).is_rest()
        {
            return None;
        }
        Some(Self {
            frame: view.sprite_frame,
            offset: (view.sprite_offset_y * 4.0).round() as i32,
            opacity: (view.opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
            mirrored: view.facing > 0.0,
            shadow: view.shadow.map(|s| (s.distance * 2.0).round() as i32),
        })
    }
}

#[derive(Debug, Default)]
struct Pointer {
    window: Option<WindowId>,
    pressed: Option<Vec2>,
    dragging: Option<u32>,
    /// Dernière position connue par les événements (si le système ne fournit pas la position globale).
    cursor: Option<Vec2>,
}

#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub struct App {
    settings: Settings,
    paths: Option<Paths>,
    library: Library,
    world: World,
    flock: Flock,
    windows: HashMap<WindowId, PetWindow>,
    gpu: Option<Gpu>,
    list: DisplayList,
    style: Style,
    pointer: Pointer,
    paused: bool,
    started: bool,
    last_tick: Instant,
    next_screen_poll: Instant,
    fast_poll_until: Instant,
    /// Relevé des fenêtres, du Dock et du dossier des animaux (fil à part).
    watcher: Option<Watcher>,
    /// Dernier relevé des fenêtres des autres apps, tel quel (refiltré quand
    /// un réglage change).
    os_windows: Vec<platform::OsWindow>,
    /// Dernier cadre du Dock relevé (« Marcher sous le Dock »).
    dock_frame: Option<platform::OsRect>,
    /// Horloge de l'écran (macOS 14 et plus) ; sinon, une minuterie.
    clock: Option<platform::FrameClock>,
    /// Dernier battement de l'horloge pas encore traité : l'heure (en
    /// secondes) à laquelle l'image préparée maintenant sera affichée. Les
    /// battements se remplacent : si l'app a pris du retard (une fenêtre
    /// modale…), une seule image rattrape le tout.
    beat: Rc<Cell<Option<f64>>>,
    /// Dernier battement reçu de l'horloge.
    clock_beat: Option<Instant>,
    /// Heure d'affichage (horloge de l'écran) de l'image précédente.
    shown_at: Option<f64>,
    /// Fin de la mise en route du lancement.
    settle_until: Option<Instant>,
    /// Prochaine image prévue par la minuterie (`None` : tout de suite). Les
    /// autres événements (souris, fenêtres déplacées…) ne la font pas venir
    /// plus tôt, sauf ceux qui changent quelque chose à l'écran.
    next_frame: Option<Instant>,
    /// Animaux demandés dont le dessin se prépare encore (identifiant → nombre).
    pending: HashMap<String, usize>,
    /// Dessins en préparation sur un fil à part (identifiants).
    decoding: HashSet<String>,
    /// Essai (`MOUFLON_ESSAI`) : fin prévue, étapes à venir et mesures de fluidité.
    trial_end: Option<Instant>,
    trial_steps: Vec<(Instant, TrialStep)>,
    stats: Option<crate::stats::FrameStats>,
    frontmost: Option<i32>,
    /// Le Dock traité comme une fenêtre posée devant les autres (« Marcher sous le Dock »).
    dock: Option<WindowRect>,
    /// Le Dock a déjà été signalé au journal.
    dock_logged: bool,
    /// Panneaux flottants déjà signalés au journal (propriétaire, niveau).
    floating_seen: std::collections::HashSet<(String, i32)>,
    seed: u64,
    refresh_hz: f64,
    /// Images par seconde voulues par le dernier `schedule` (0 : horloge
    /// arrêtée) : notées au journal d'un essai.
    cadence: f64,
    /// Pour réveiller la boucle depuis l'horloge, le menu ou un autre fil.
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    menu: Option<crate::menu::TrayMenu>,
    /// Ce que le menu montre du troupeau (le préféré, le nombre d'animaux de
    /// chaque espèce) : il n'est mis à jour que quand cela change, ce que dit
    /// la génération du troupeau.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    menu_flock: Option<(usize, Vec<usize>)>,
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    menu_generation: Option<u64>,
    /// Génération du troupeau pour laquelle les fenêtres sont à jour.
    windows_generation: Option<u64>,
}

fn to_rect(r: platform::OsRect) -> Rect {
    Rect::new(r.x as f32, r.y as f32, r.w as f32, r.h as f32)
}

/// Ouvre la fenêtre « À propos » : version, description, page du projet (et
/// site de l'auteur quand le `homepage` du workspace en sera un autre),
/// crédits, licence et copyright.
fn show_about() {
    use platform::AboutLine;
    let t = crate::i18n::texts();
    let shown = |url: &'static str| url.trim_start_matches("https://").trim_end_matches('/');
    let repository = env!("CARGO_PKG_REPOSITORY");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    let mut lines = vec![
        AboutLine::Text(&t.description),
        AboutLine::Link(shown(repository), repository),
    ];
    if !homepage.is_empty() && homepage != repository {
        lines.push(AboutLine::Link(shown(homepage), homepage));
    }
    lines.push(AboutLine::Text(&t.credits));
    lines.push(AboutLine::Text(&t.license));
    platform::show_about(&platform::About {
        name: "Mouflon",
        version: env!("CARGO_PKG_VERSION"),
        copyright: COPYRIGHT,
        lines: &lines,
    });
}

/// Volume de l'ensemble des sons des animaux (de 0 à 1) : discret, c'est un
/// compagnon de bureau.
const VOLUME: f32 = 0.55;

/// Mention de copyright de la fenêtre « À propos ».
const COPYRIGHT: &str = "© 2026 Alejandro San Martin Lamas";

/// Coin haut gauche où se trouve vraiment la fenêtre (repère commun).
/// Sur macOS, un déplacement est immédiat et peut être corrigé par le
/// système (une fenêtre ne monte pas au-dessus de la barre des menus) ;
/// ailleurs, il peut n'être fait qu'un peu plus tard : on s'en tient à la
/// position demandée.
fn placed_at(window: &Window, scale: f64) -> Option<(f64, f64)> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    let p = window.outer_position().ok()?.to_logical::<f64>(scale);
    Some((p.x, p.y))
}

fn local_hour() -> f32 {
    let now = chrono::Local::now();
    now.hour() as f32 + now.minute() as f32 / 60.0
}

/// Le curseur est-il sur le corps de l'animal ? Hors du corps, la fenêtre
/// laisse passer les clics vers les applications en dessous.
/// Le dessin d'un animal en images doit-il suivre un nouvel écart entre sa
/// fenêtre et lui ? Seulement d'un point au moins : macOS arrondit la place
/// des fenêtres, et redessiner pour un demi-point (un pixel) coûtait, à un
/// animal qui marche, trois ou quatre rendus de plus que ses propres images.
/// L'écart exact revient à l'image suivante de l'animal.
fn shifted(offset: (f64, f64), drawn: (f64, f64)) -> bool {
    (offset.0 - drawn.0).abs() >= 1.0 || (offset.1 - drawn.1).abs() >= 1.0
}

fn hit_test(cursor: Vec2, view: &PetView<'_>, art: &PetArt) -> bool {
    let g = view.ground;
    let (half, height) = art.body;
    cursor.x >= g.x - half
        && cursor.x <= g.x + half
        && cursor.y >= g.y - height
        && cursor.y <= g.y + 3.0
}

#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
impl App {
    #[must_use]
    pub fn new(
        settings: Settings,
        paths: Option<Paths>,
        library: Library,
        proxy: EventLoopProxy<UserEvent>,
    ) -> Self {
        let now = Instant::now();
        let mut flock = Flock::new();
        flock.set_collisions(settings.collisions);
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x5EED);
        Self {
            settings,
            paths,
            library,
            world: World::default(),
            flock,
            windows: HashMap::new(),
            gpu: None,
            list: DisplayList::default(),
            style: Style::default(),
            pointer: Pointer::default(),
            paused: false,
            started: false,
            last_tick: now,
            next_screen_poll: now,
            fast_poll_until: now,
            watcher: None,
            os_windows: Vec::new(),
            dock_frame: None,
            clock: None,
            beat: Rc::new(Cell::new(None)),
            clock_beat: None,
            shown_at: None,
            settle_until: None,
            next_frame: None,
            pending: HashMap::new(),
            decoding: HashSet::new(),
            trial_end: None,
            trial_steps: Vec::new(),
            stats: None,
            frontmost: None,
            dock: None,
            dock_logged: false,
            floating_seen: std::collections::HashSet::new(),
            seed,
            refresh_hz: 60.0,
            cadence: 0.0,
            proxy: Arc::new(Mutex::new(proxy)),
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu_flock: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu_generation: None,
            windows_generation: None,
        }
    }

    fn cursor(&self) -> Option<Vec2> {
        platform::cursor_position()
            .map(|(x, y)| Vec2::new(x as f32, y as f32))
            .or(self.pointer.cursor)
    }

    fn input(&self) -> Input {
        Input {
            cursor: self.cursor(),
            local_hour: local_hour(),
        }
    }

    fn refresh_screens(&mut self, event_loop: &ActiveEventLoop) {
        let mut screens: Vec<Screen> = platform::screens()
            .into_iter()
            .map(|s| Screen {
                frame: to_rect(s.frame),
                usable: to_rect(s.visible),
                scale: s.scale as f32,
            })
            .collect();
        if screens.is_empty() {
            // Systèmes sans couche dédiée : on se contente des écrans de winit.
            screens = event_loop
                .available_monitors()
                .map(|m| {
                    let scale = m.scale_factor();
                    let p = m.position().to_logical::<f64>(scale);
                    let s = m.size().to_logical::<f64>(scale);
                    let frame = Rect::new(p.x as f32, p.y as f32, s.width as f32, s.height as f32);
                    let usable = Rect::new(frame.x, frame.y, frame.w, (frame.h - 48.0).max(100.0));
                    Screen {
                        frame,
                        usable,
                        scale: scale as f32,
                    }
                })
                .collect();
        }
        // Le Dock tel que le fil de relevé l'a lu (s'il en a le droit).
        self.dock = None;
        if self.settings.walk_under_dock
            && let Some(frame) = self.dock_frame
        {
            self.dock = exact_dock(&mut screens, to_rect(frame));
            if !self.dock_logged {
                self.dock_logged = true;
                log::info!(
                    "Dock : {:.0} × {:.0} en {:.0}, {:.0}{}",
                    frame.w,
                    frame.h,
                    frame.x,
                    frame.y,
                    if self.dock.is_some() {
                        ""
                    } else {
                        " (sur le côté, masqué ou sur toute la largeur : ignoré)"
                    }
                );
            }
        }
        if screens != self.world.screens() {
            let first = self.world.screens().is_empty();
            self.world.set_screens(screens);
            // Écran branché, débranché ou réglé autrement : l'horloge se cale
            // de nouveau sur le plus rapide.
            if !first && self.clock.is_some() {
                self.start_clock();
            }
        }
    }

    /// (Re)crée l'horloge de l'écran, sur l'écran le plus rapide. Elle
    /// démarre à la prochaine image.
    fn start_clock(&mut self) {
        // L'ancienne d'abord : elle cesse de battre.
        self.clock = None;
        self.shown_at = None;
        // Le silence se compte depuis le dernier battement entendu ; une
        // horloge toute neuve ne mène les images qu'une fois qu'elle bat.
        if self.clock_beat.is_none() {
            let now = Instant::now();
            self.clock_beat = Some(now.checked_sub(CLOCK_SILENT).unwrap_or(now));
        }
        self.beat.set(None);
        let beat = Rc::clone(&self.beat);
        let proxy = Arc::clone(&self.proxy);
        self.clock = platform::FrameClock::new(move |shown_at| {
            // Un seul battement en attente à la fois.
            if beat.replace(Some(shown_at)).is_none()
                && let Ok(proxy) = proxy.lock()
            {
                let _ = proxy.send_event(UserEvent::Frame);
            }
        });
        if let Some(clock) = &self.clock {
            log::info!(
                "horloge de l'écran : jusqu'à {:.0} images par seconde",
                clock.max_fps()
            );
            if let Some(stats) = &self.stats {
                stats.note(format_args!(
                    "horloge de l'écran, {:.0} i/s au plus",
                    clock.max_fps()
                ));
            }
        }
    }

    /// L'horloge de l'écran mène-t-elle les images en ce moment ?
    fn clocked(&self, now: Instant) -> bool {
        self.clock
            .as_ref()
            .is_some_and(platform::FrameClock::is_running)
            && self
                .clock_beat
                .is_some_and(|beat| now.duration_since(beat) < CLOCK_SILENT)
    }

    /// Applique ce que le fil de relevé a vu depuis la dernière image.
    fn apply_watch(&mut self, event_loop: &ActiveEventLoop, now: Instant) {
        let Some(watcher) = &self.watcher else {
            return;
        };
        let snapshot = watcher.take();
        let pets = watcher.take_pets();
        if let Some(snapshot) = snapshot {
            self.apply_snapshot(event_loop, snapshot, now);
        }
        // Un animal ajouté au dossier apparaît dans le menu sans relancer l'app.
        if let Some(stamp) = pets
            && self.library.changed_since(&stamp)
        {
            log::info!("dossier des animaux modifié : rechargement");
            self.library.reload();
            self.rebuild_menu();
        }
        // Une fenêtre qui porte un animal bouge, ou on tient un animal : on
        // relève plus souvent pour qu'il suive en douceur.
        if let Some(watcher) = &self.watcher {
            watcher.set_fast(now < self.fast_poll_until || self.pointer.dragging.is_some());
        }
    }

    fn apply_snapshot(&mut self, event_loop: &ActiveEventLoop, snapshot: Snapshot, now: Instant) {
        let dock = snapshot.dock.filter(|_| self.settings.walk_under_dock);
        if dock != self.dock_frame {
            self.dock_frame = dock;
            self.refresh_screens(event_loop);
        }
        if let Some(stats) = &self.stats
            && snapshot.windows.len() != self.os_windows.len()
        {
            stats.note(format_args!(
                "fenêtres des autres apps : {} → {}",
                self.os_windows.len(),
                snapshot.windows.len()
            ));
        }
        self.os_windows = snapshot.windows;
        self.refresh_windows(now);
    }

    /// Applique le dernier relevé des fenêtres (sans rien redemander au système,
    /// sauf l'app au premier plan).
    fn refresh_windows(&mut self, now: Instant) {
        let me = platform::current_pid();
        if let Some(pid) = platform::frontmost_pid()
            && pid != me
        {
            self.frontmost = Some(pid);
        }
        let mut windows = Vec::new();
        for w in &self.os_windows {
            // Calque transparent posé sur tout un écran (le clavier d'accessibilité
            // en a deux) : ni sol, ni obstacle.
            let overlay = w.is_floating()
                && self.world.screens().iter().any(|s| {
                    f64::from(s.frame.w) * f64::from(s.frame.h) * 0.9 <= w.frame.w * w.frame.h
                });
            let walkable = w.is_walkable() && !overlay;
            // Panneaux flottants (visualiseur de clavier…) : notés une fois au journal.
            if w.is_floating() && self.floating_seen.insert((w.owner.clone(), w.layer)) {
                log::info!(
                    "fenêtre flottante : {} (niveau {}, {:.0} × {:.0} en {:.0}, {:.0}){}",
                    w.owner,
                    w.layer,
                    w.frame.w,
                    w.frame.h,
                    w.frame.x,
                    w.frame.y,
                    if walkable { "" } else { ", ignorée" }
                );
            }
            if !walkable {
                continue;
            }
            // Un panneau flottant reste visible quelle que soit l'app au premier plan.
            if self.settings.frontmost_only && !w.is_floating() && Some(w.pid) != self.frontmost {
                continue;
            }
            windows.push(WindowRect {
                id: w.id,
                frame: to_rect(w.frame),
            });
        }

        // Le Dock passe devant toutes les fenêtres.
        if let Some(dock) = self.dock {
            windows.insert(0, dock);
        }

        // Une fenêtre qui porte un animal bouge : on relève plus souvent pour qu'il suive en douceur.
        let supports: Vec<(u64, f32, f32)> = self
            .flock
            .pets()
            .iter()
            .filter_map(|p| match p.support() {
                Support::Ground { surface } => {
                    self.world.surface(surface).map(|s| (surface, s.x0, s.y))
                }
                _ => None,
            })
            .collect();
        self.world.set_windows(&windows, LedgeOptions::default());
        let moved = supports.iter().any(|&(id, x0, y)| {
            self.world
                .surface(id)
                .is_some_and(|s| (s.x0 - x0).abs() > 0.5 || (s.y - y).abs() > 0.5)
        });
        if moved {
            self.fast_poll_until = now + Duration::from_millis(1500);
        }
    }

    fn spawn(&mut self, event_loop: &ActiveEventLoop, index: usize) {
        let Some(entry) = self.library.pets.get(index).cloned() else {
            return;
        };
        // Les dessins d'un animal se préparent à sa première apparition, sur
        // un fil à part : les autres continuent de bouger pendant ce temps, et
        // il arrive dès que c'est prêt.
        if !entry.art_ready() && self.prepare_art(&entry) {
            *self.pending.entry(entry.def.meta.id.clone()).or_default() += 1;
            return;
        }
        // Illisible : rien à montrer (l'erreur est au journal).
        if entry.art().is_none() {
            return;
        }
        self.seed = self
            .seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.flock
            .spawn(Arc::clone(&entry.def), self.seed, &self.world, None);
        self.sync_windows(event_loop);
    }

    /// Lance le décodage des dessins de ces animaux, une espèce par fil, sans
    /// les attendre : `Entry::art` attend ensuite ceux qui ne sont pas prêts
    /// (ou décode sur place si un fil n'a pas pu être lancé).
    fn decode_ahead(&self, starters: &[usize]) -> Vec<Entry> {
        let mut species: Vec<Entry> = Vec::new();
        for entry in starters.iter().filter_map(|&i| self.library.pets.get(i)) {
            if !species.iter().any(|e| e.def.meta.id == entry.def.meta.id) {
                species.push(entry.clone());
            }
        }
        for entry in species.iter().filter(|e| !e.art_ready()) {
            let entry = entry.clone();
            let spawned = std::thread::Builder::new()
                .name("mouflon-dessins".into())
                .spawn(move || {
                    let _ = entry.art();
                });
            if let Err(e) = spawned {
                log::warn!("dessins préparés sur le fil principal : {e}");
            }
        }
        species
    }

    /// Prépare les dessins d'un animal sur un fil à part ; `UserEvent::ArtReady`
    /// arrive quand c'est fait. `false` si le fil n'a pas pu être lancé.
    fn prepare_art(&mut self, entry: &Entry) -> bool {
        let id = entry.def.meta.id.clone();
        if self.decoding.contains(&id) {
            return true;
        }
        let job = (entry.clone(), id.clone(), Arc::clone(&self.proxy));
        let spawned = std::thread::Builder::new()
            .name("mouflon-dessins".into())
            .spawn(move || {
                let (entry, id, proxy) = job;
                let _ = entry.art();
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::ArtReady(id));
                }
            });
        match spawned {
            Ok(_) => {
                self.decoding.insert(id);
                true
            }
            Err(e) => {
                log::warn!("dessins préparés sur le fil principal : {e}");
                false
            }
        }
    }

    /// Les dessins d'un animal sont prêts : ceux qui l'attendaient arrivent.
    fn art_ready(&mut self, event_loop: &ActiveEventLoop, id: &str) {
        self.next_frame = None;
        self.decoding.remove(id);
        let count = self.pending.remove(id).unwrap_or(0);
        if count == 0 {
            return;
        }
        if let Some(index) = self.library.pets.iter().position(|e| e.def.meta.id == id) {
            for _ in 0..count {
                self.spawn(event_loop, index);
            }
        }
    }

    /// Une fenêtre par animal : on en crée pour les nouveaux venus (dont les
    /// personnages secondaires des scènes desktopPet) et on ferme celles des partis.
    fn sync_windows(&mut self, event_loop: &ActiveEventLoop) {
        // Rien n'a changé dans le troupeau : les fenêtres sont à jour.
        let generation = self.flock.generation();
        if self.windows_generation == Some(generation) {
            return;
        }
        self.windows_generation = Some(generation);
        let alive: Vec<u32> = self.flock.pets().iter().map(|p| p.id()).collect();
        for w in self.windows.values().filter(|w| !alive.contains(&w.pet)) {
            if let Some(stats) = &self.stats {
                stats.note(format_args!("fenêtre fermée (n° {})", w.pet));
            }
            // Fermée mais jamais libérée (voir `keep_pet_window`).
            platform::keep_pet_window(&*w.window);
        }
        self.windows.retain(|_, w| alive.contains(&w.pet));
        if self.windows.len() == alive.len() {
            return;
        }
        let pets = self.flock.pets();
        let missing: Vec<(u32, String, Arc<PetArt>)> = pets
            .iter()
            .filter(|p| !self.windows.values().any(|w| w.pet == p.id()))
            .filter_map(|p| {
                // Dessin de la bibliothèque ; après un rechargement, celui d'un
                // animal déjà affiché de la même espèce (personnages secondaires).
                let art = self
                    .library
                    .pets
                    .iter()
                    .find(|e| Arc::ptr_eq(&e.def, p.def()))
                    .and_then(crate::library::Entry::art)
                    .or_else(|| {
                        self.windows
                            .values()
                            .find(|w| {
                                pets.iter()
                                    .any(|q| q.id() == w.pet && Arc::ptr_eq(q.def(), p.def()))
                            })
                            .map(|w| Arc::clone(&w.art))
                    })?;
                Some((p.id(), p.def().meta.name.clone(), art))
            })
            .collect();
        for (id, name, art) in missing {
            self.open_window(event_loop, id, &name, art);
        }
    }

    fn open_window(&mut self, event_loop: &ActiveEventLoop, id: u32, name: &str, art: Arc<PetArt>) {
        if let Some(stats) = &self.stats {
            stats.note(format_args!("fenêtre ouverte pour {name} (n° {id})"));
        }
        // Les collisions partent quand les dessins se touchent.
        let Some(pet) = self.flock.get_mut(id) else {
            return;
        };
        if let Some((front, back)) = art.reach() {
            pet.set_reach(front, back);
        }
        let canvas = art.canvas();
        let ground = pet.position();
        let origin = ground - canvas.anchor;

        let attrs = Window::default_attributes()
            .with_title(format!("Mouflon — {name}"))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(LogicalSize::new(
                f64::from(canvas.width),
                f64::from(canvas.height),
            ))
            .with_position(LogicalPosition::new(
                f64::from(origin.x),
                f64::from(origin.y),
            ))
            .with_active(false);
        #[cfg(target_os = "macos")]
        let attrs = {
            use winit::platform::macos::WindowAttributesExtMacOS;
            attrs.with_has_shadow(false).with_accepts_first_mouse(true)
        };
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                log::error!("impossible de créer la fenêtre de l'animal : {e}");
                self.flock.remove(id);
                return;
            }
        };
        platform::prepare_pet_window(&*window);
        let monitor = window.current_monitor();
        let hz = window_hz(&window, monitor.as_ref());
        self.refresh_hz = self.refresh_hz.max(hz);
        if window.theme() == Some(Theme::Dark) {
            self.style.dark = true;
        }
        let gpu = self.gpu.get_or_insert_with(Gpu::new);
        let size = window.inner_size();
        let surface = match gpu.create_surface(Arc::clone(&window), size.width, size.height) {
            Ok(s) => s,
            Err(e) => {
                log::error!("rendu indisponible : {e}");
                platform::keep_pet_window(&*window);
                self.flock.remove(id);
                return;
            }
        };
        let scale = window.scale_factor();
        self.windows.insert(
            window.id(),
            PetWindow {
                pet: id,
                window,
                surface,
                canvas,
                art,
                scale,
                position: None,
                placed: (f64::from(origin.x), f64::from(origin.y)),
                drawn_offset: (0.0, 0.0),
                hittest: true,
                last_render: None,
                sprite: None,
                monitor,
                hz,
                list: DisplayList::default(),
            },
        );
    }

    /// Met le GPU en route avant la première image : une image d'essai de
    /// chaque animal du lancement, dessinée hors écran jusqu'au bout.
    fn warm_up_gpu(&mut self) {
        let started = Instant::now();
        let gpu = self.gpu.get_or_insert_with(Gpu::new);
        let mut done: Vec<Arc<PetArt>> = Vec::new();
        for pw in self.windows.values() {
            if done.iter().any(|art| Arc::ptr_eq(art, &pw.art)) {
                continue;
            }
            done.push(Arc::clone(&pw.art));
            let Some(pet) = self.flock.pets().iter().find(|p| p.id() == pw.pet) else {
                continue;
            };
            let view = pet.view(1.0);
            let size = pw.window.inner_size();
            self.list.clear(size.width, size.height);
            let frame = Frame {
                origin: view.ground - pw.canvas.anchor,
                scale: pw.scale,
            };
            paint_pet(&pw.art, &view, &frame, &self.style, &mut self.list);
            if let Err(e) = gpu.warm_up(&self.list) {
                log::warn!("mise en route du GPU : {e}");
            }
        }
        log::info!("GPU en route en {} ms", started.elapsed().as_millis());
    }

    /// Reconstruit le menu après un changement de la bibliothèque.
    fn rebuild_menu(&mut self) {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            self.menu = None;
            match crate::menu::TrayMenu::new(
                &self.library,
                self.settings.sounds,
                self.settings.frontmost_only,
                self.settings.collisions,
                self.settings.walk_under_dock,
                self.settings.size_index(),
            ) {
                Ok(menu) => {
                    menu.set_paused(self.paused);
                    self.menu = Some(menu);
                }
                Err(e) => log::error!("impossible de recréer le menu : {e}"),
            }
            self.menu_flock = None;
            self.menu_generation = None;
            self.sync_menu();
        }
    }

    /// Le menu suit le troupeau : « Ajouter Lainou » nomme l'animal préféré,
    /// et chaque espèce dit combien des siens sont à l'écran (ceux qui
    /// partent déjà et les personnages secondaires ne comptent pas).
    fn sync_menu(&mut self) {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let Some(menu) = &self.menu else {
                return;
            };
            // Le préféré ne change qu'avec un animal ajouté : la génération
            // du troupeau suffit à savoir s'il y a du nouveau.
            let generation = self.flock.generation();
            if self.menu_generation == Some(generation) {
                return;
            }
            self.menu_generation = Some(generation);
            let favorite = self
                .library
                .default_index(self.settings.default_pet.as_deref());
            let mut counts = vec![0; self.library.pets.len()];
            for pet in self.flock.pets() {
                if pet.is_child() || pet.is_leaving() {
                    continue;
                }
                let id = pet.def().meta.id.as_str();
                if let Some(i) = self.library.pets.iter().position(|e| e.def.meta.id == id) {
                    counts[i] += 1;
                }
            }
            if self
                .menu_flock
                .as_ref()
                .is_some_and(|(f, c)| *f == favorite && *c == counts)
            {
                return;
            }
            menu.set_flock(favorite, &counts);
            self.menu_flock = Some((favorite, counts));
        }
    }

    /// Met la sortie audio en route avant le premier son, avec un son de
    /// l'animal du lancement (joué sans volume) : son décodeur se charge aussi.
    fn warm_up_sounds(&self) {
        let index = self
            .library
            .default_index(self.settings.default_pet.as_deref());
        let sample = self
            .library
            .pets
            .get(index)
            .and_then(|entry| entry.def.sounds.first())
            .and_then(|sound| sound.clips.first());
        platform::warm_up_sounds(sample);
    }

    /// Joue les sons que les animaux viennent de faire (s'ils ont la parole),
    /// et vide la liste de leurs événements (notés au journal d'un essai).
    fn play_sounds(&mut self) {
        let audible = self.settings.sounds && !self.paused;
        for pet in self.flock.pets_mut() {
            for event in pet.take_events() {
                if let Some(stats) = &self.stats {
                    stats.note(format_args!("n° {} {}", pet.id(), describe(&event, pet)));
                }
                if let PetEvent::Sound { sound, clip } = event
                    && audible
                    && let Some(sound) = pet.def().sounds.get(sound)
                    && let Some(data) = sound.clips.get(clip)
                {
                    platform::play_sound(data, sound.volume * VOLUME);
                }
            }
        }
    }

    /// Un fichier ou un dossier d'animal déposé sur un animal : on l'installe,
    /// et l'animal visé laisse sa place au nouveau venu (comme dans desktopPet).
    fn install_pet(
        &mut self,
        event_loop: &ActiveEventLoop,
        path: &std::path::Path,
        target: Option<u32>,
    ) {
        match self.library.install(path) {
            Ok(id) => {
                log::info!("animal installé : {id} ({})", path.display());
                self.rebuild_menu();
                if let Some(old) = target {
                    self.flock.remove(old);
                    if self.pointer.dragging == Some(old) {
                        self.pointer.dragging = None;
                    }
                }
                let index = self.library.index_of(&id);
                if self.settings.default_pet.as_deref() != Some(id.as_str()) {
                    self.settings.default_pet = Some(id);
                    if let Some(paths) = &self.paths {
                        self.settings.save(&paths.settings);
                    }
                }
                self.spawn(event_loop, index);
            }
            Err(e) => log::warn!("impossible d'installer {} : {e}", path.display()),
        }
    }

    /// Le dernier arrivé de cette espèce (rang dans la bibliothèque) dit au
    /// revoir ; sa fenêtre se ferme quand il est parti.
    fn goodbye_one(&mut self, index: usize) {
        if let Some(entry) = self.library.pets.get(index) {
            let input = self.input();
            let species = entry.def.meta.id.clone();
            if let Some(id) = self.flock.leave_one(&species, &self.world, &input)
                && self.pointer.dragging == Some(id)
            {
                self.pointer = Pointer::default();
            }
        }
    }

    /// Avant de quitter : le journal d'un essai interrompu (Quitter pendant
    /// l'essai) reçoit son bilan et part sur le disque, et les fenêtres des
    /// animaux ne seront jamais libérées (voir `keep_pet_window`).
    pub fn shutdown(&mut self) {
        if let Some(stats) = &mut self.stats {
            stats.finish();
        }
        for pw in self.windows.values() {
            platform::keep_pet_window(&*pw.window);
        }
    }

    /// Une étape de l'essai, notée au journal.
    fn trial_step(&mut self, event_loop: &ActiveEventLoop, step: TrialStep) {
        if let Some(stats) = &self.stats {
            let (TrialStep::Goodbye(index) | TrialStep::Add(index)) = step;
            let name = self
                .library
                .pets
                .get(index)
                .map_or("?", |entry| entry.def.meta.name.as_str());
            match step {
                TrialStep::Goodbye(_) => {
                    stats.note(format_args!("essai : au revoir à un(e) {name}"))
                }
                TrialStep::Add(_) => stats.note(format_args!("essai : un(e) {name} de plus")),
            }
        }
        match step {
            TrialStep::Goodbye(index) => self.goodbye_one(index),
            TrialStep::Add(index) => self.spawn(event_loop, index),
        }
    }

    /// Tous les animaux disent au revoir : chacun joue son animation d'adieu
    /// et s'efface ; leurs fenêtres se ferment quand ils sont partis.
    fn remove_all(&mut self) {
        let input = self.input();
        self.flock.leave_all(&self.world, &input);
        self.pointer = Pointer::default();
    }

    /// Change la taille de tous les animaux, sans les interrompre.
    fn set_size(&mut self, index: usize) {
        let size = crate::settings::SIZES[index.min(crate::settings::SIZES.len() - 1)];
        let old = self.settings.size_factor();
        if (size - old).abs() < f32::EPSILON {
            return;
        }
        self.settings.size = size;
        if let Some(paths) = &self.paths {
            self.settings.save(&paths.settings);
        }
        self.library.set_size(size);
        // Les dessins de chaque espèce à l'écran, à la nouvelle taille,
        // préparés en parallèle.
        let mut species = HashSet::new();
        let needed: Vec<Entry> = self
            .flock
            .pets()
            .iter()
            .filter_map(|pet| self.library.current(pet.def()))
            .filter(|entry| species.insert(entry.def.meta.id.clone()))
            .cloned()
            .collect();
        std::thread::scope(|scope| {
            for entry in &needed {
                scope.spawn(move || entry.art());
            }
        });
        let mut gone = Vec::new();
        for pet in self.flock.pets_mut() {
            let resized = self
                .library
                .current(pet.def())
                .is_some_and(|entry| pet.rescale(Arc::clone(&entry.def), size / old));
            if !resized {
                // Plus dans la bibliothèque : on n'a plus de quoi le dessiner.
                gone.push(pet.id());
            }
        }
        for id in gone {
            self.flock.remove(id);
        }
        // Chaque fenêtre prend tout de suite la nouvelle taille, sans se fermer.
        for pw in self.windows.values_mut() {
            let Some(pet) = self.flock.pets().iter().find(|p| p.id() == pw.pet) else {
                continue;
            };
            let Some(art) = self
                .library
                .pets
                .iter()
                .find(|e| Arc::ptr_eq(&e.def, pet.def()))
                .and_then(crate::library::Entry::art)
            else {
                continue;
            };
            pw.canvas = art.canvas();
            if let Some((front, back)) = art.reach()
                && let Some(pet) = self.flock.get_mut(pw.pet)
            {
                pet.set_reach(front, back);
            }
            pw.art = art;
            let requested =
                LogicalSize::new(f64::from(pw.canvas.width), f64::from(pw.canvas.height));
            if let Some(size) = pw.window.request_inner_size(requested)
                && let Some(gpu) = self.gpu.as_ref()
            {
                gpu.resize(&mut pw.surface, size.width, size.height);
            }
            pw.position = None;
            pw.last_render = None;
            pw.sprite = None;
        }
        log::info!(
            "taille des animaux : × {size} ({} à l'écran)",
            self.flock.pets().len()
        );
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        if let Some(menu) = &self.menu {
            menu.set_size(index);
        }
    }

    fn handle_action(&mut self, event_loop: &ActiveEventLoop, action: Action) {
        if let Some(stats) = &self.stats {
            stats.note(format_args!("menu {action:?}"));
        }
        // Ce que l'on vient de choisir se voit tout de suite.
        self.next_frame = None;
        match action {
            Action::AddDefault => {
                let index = self
                    .library
                    .default_index(self.settings.default_pet.as_deref());
                self.spawn(event_loop, index);
            }
            Action::Add(index) => {
                self.spawn(event_loop, index);
                // Le dernier animal choisi devient celui du lancement et du premier article.
                if let Some(entry) = self.library.pets.get(index)
                    && self.settings.default_pet.as_deref() != Some(entry.def.meta.id.as_str())
                {
                    self.settings.default_pet = Some(entry.def.meta.id.clone());
                    if let Some(paths) = &self.paths {
                        self.settings.save(&paths.settings);
                    }
                }
            }
            Action::RemoveOne(index) => self.goodbye_one(index),
            Action::RemoveAll => self.remove_all(),
            Action::Together => {
                let input = self.input();
                let count = self.flock.together(&self.world, &input);
                log::info!("tous ensemble : {count} animal(aux)");
            }
            Action::ToggleWalkUnderDock => {
                self.settings.walk_under_dock = !self.settings.walk_under_dock;
                if let Some(paths) = &self.paths {
                    self.settings.save(&paths.settings);
                }
                // Lire le Dock demande l'autorisation d'Accessibilité : macOS
                // propose de l'accorder si Mouflon ne l'a pas encore.
                if self.settings.walk_under_dock && !platform::request_accessibility() {
                    log::info!(
                        "Marcher sous le Dock : en attente de l'autorisation d'Accessibilité"
                    );
                }
                self.dock_logged = false;
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    menu.set_walk_under_dock(self.settings.walk_under_dock);
                }
                // Le fil de relevé lit le Dock tout de suite (ou cesse de le lire).
                if let Some(watcher) = &self.watcher {
                    watcher.set_dock(self.settings.walk_under_dock);
                }
                if !self.settings.walk_under_dock {
                    self.dock_frame = None;
                }
                self.refresh_screens(event_loop);
                self.refresh_windows(Instant::now());
            }
            Action::SetSize(index) => {
                self.set_size(index);
                self.sync_windows(event_loop);
            }
            Action::ToggleSounds => {
                self.settings.sounds = !self.settings.sounds;
                if let Some(paths) = &self.paths {
                    self.settings.save(&paths.settings);
                }
                if self.settings.sounds {
                    self.warm_up_sounds();
                }
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    menu.set_sounds(self.settings.sounds);
                }
            }
            Action::TogglePause => {
                self.paused = !self.paused;
                // Au réveil, les animaux repartent d'où ils étaient, sans
                // rattraper le temps de la pause.
                self.last_tick = Instant::now();
                self.shown_at = None;
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    menu.set_paused(self.paused);
                }
            }
            Action::ToggleFrontmostOnly => {
                self.settings.frontmost_only = !self.settings.frontmost_only;
                if let Some(paths) = &self.paths {
                    self.settings.save(&paths.settings);
                }
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    menu.set_frontmost_only(self.settings.frontmost_only);
                }
                self.refresh_windows(Instant::now());
            }
            Action::ToggleCollisions => {
                self.settings.collisions = !self.settings.collisions;
                self.flock.set_collisions(self.settings.collisions);
                if let Some(paths) = &self.paths {
                    self.settings.save(&paths.settings);
                }
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    menu.set_collisions(self.settings.collisions);
                }
            }
            Action::OpenPetsFolder => {
                if let Some(paths) = &self.paths {
                    let _ = std::fs::create_dir_all(&paths.pets);
                    if !platform::open_folder(&paths.pets) {
                        log::warn!("impossible d'ouvrir {}", paths.pets.display());
                    }
                }
            }
            Action::DockInfo =>
            {
                #[cfg(any(target_os = "macos", target_os = "windows"))]
                if let Some(menu) = &self.menu {
                    let t = crate::i18n::texts();
                    menu.show_note(&t.walk_under_dock, &t.dock_info_text);
                }
            }
            Action::About => show_about(),
            Action::Quit => event_loop.exit(),
        }
    }

    fn pet_of(&self, window: WindowId) -> Option<u32> {
        self.windows.get(&window).map(|w| w.pet)
    }

    /// Fréquence de l'écran de chaque fenêtre (un animal a pu changer d'écran).
    fn refresh_monitors(&mut self) {
        let mut fastest: f64 = 0.0;
        for pw in self.windows.values_mut() {
            let monitor = pw.window.current_monitor();
            if monitor != pw.monitor {
                pw.hz = window_hz(&pw.window, monitor.as_ref());
                pw.monitor = monitor;
            }
            fastest = fastest.max(pw.hz);
        }
        if fastest > 0.0 {
            self.refresh_hz = fastest;
        }
    }

    /// Une image : simulation, position des fenêtres, rendu, prochain réveil.
    /// `shown_at` : heure d'affichage de l'image, donnée par l'horloge de l'écran.
    fn frame(&mut self, event_loop: &ActiveEventLoop, shown_at: Option<f64>) {
        let now = Instant::now();
        let cadence = self.cadence;
        // Calé sur l'écran, on avance exactement du temps qui sépare deux
        // images affichées : les petits retards du fil principal ne font plus
        // varier la vitesse des animaux d'une image à l'autre.
        let elapsed = match (shown_at, self.shown_at) {
            (Some(at), Some(before)) if at > before => (at - before) as f32,
            _ => now.duration_since(self.last_tick).as_secs_f32(),
        };
        self.shown_at = shown_at;
        self.last_tick = now;
        let mut phases = crate::stats::Phases::default();

        if now >= self.next_screen_poll {
            self.refresh_screens(event_loop);
            self.refresh_monitors();
            // Une horloge qui tourne mais ne bat plus (écran changé, réveil…) :
            // on en refait une.
            if self
                .clock
                .as_ref()
                .is_some_and(platform::FrameClock::is_running)
                && self
                    .clock_beat
                    .is_some_and(|beat| now.duration_since(beat) > SCREEN_POLL)
            {
                self.start_clock();
            }
            self.next_screen_poll = now + SCREEN_POLL;
        }
        self.apply_watch(event_loop, now);
        let mut mark = Instant::now();
        phases.watch = mark - now;

        // Mise en route du lancement : les fenêtres sont ouvertes mais vides,
        // les animaux attendent.
        let settling = self.settle_until.is_some_and(|until| now < until);
        if !settling && self.settle_until.take().is_some() {
            log::info!("mise en route terminée : les animaux arrivent");
            if let Some(stats) = &self.stats {
                stats.note("mise en route terminée");
            }
        }

        // Un essai fait partir un animal à mi-parcours, puis en fait venir un.
        while !settling && self.trial_steps.first().is_some_and(|(at, _)| now >= *at) {
            let (_, step) = self.trial_steps.remove(0);
            self.trial_step(event_loop, step);
        }

        let input = self.input();
        let cursor = input.cursor;

        // Glisser : on n'attrape l'animal qu'après un petit déplacement,
        // un simple clic reste un « coucou ». Il est pris là où l'on a cliqué,
        // et ce point reste sous le curseur.
        if !settling
            && let (Some(start), Some(c), None) =
                (self.pointer.pressed, cursor, self.pointer.dragging)
            && (c - start).length() > 3.0
            && let Some(id) = self.pointer.window.and_then(|w| self.pet_of(w))
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.grab(start, &self.world);
            self.pointer.dragging = Some(id);
            if let Some(stats) = &self.stats {
                stats.note(format_args!(
                    "n° {id} attrapé en {:.0},{:.0}",
                    start.x, start.y
                ));
            }
        }
        if let (Some(id), Some(c)) = (self.pointer.dragging, cursor)
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.drag_to(c);
        }

        let alpha = if self.paused || settling {
            1.0
        } else {
            self.flock.advance(elapsed, &self.world, &input)
        };
        phases.sim = lap(&mut mark);
        if !settling {
            self.play_sounds();
        }
        phases.sounds = lap(&mut mark);
        self.sync_windows(event_loop);
        self.sync_menu();
        phases.windows = lap(&mut mark);

        let Self {
            windows,
            flock,
            gpu,
            style,
            pointer,
            paused,
            ..
        } = self;
        // Fenêtres à redessiner, avec l'image et le décalage qu'elles montreront.
        let mut redraw: Vec<(&mut PetWindow, Option<SpriteKey>, (f64, f64))> = Vec::new();
        for pw in windows.values_mut() {
            let Some(pet) = flock.pets().iter().find(|p| p.id() == pw.pet) else {
                continue;
            };
            let view = pet.view(alpha);
            let scale = pw.scale;
            // macOS ne place une fenêtre qu'au point entier (deux pixels d'un
            // écran Retina) : on la demande directement là, sans déplacement
            // inutile d'un demi-point.
            let ox = (f64::from(view.ground.x) - f64::from(pw.canvas.anchor.x)).floor();
            let oy = (f64::from(view.ground.y) - f64::from(pw.canvas.anchor.y)).floor();
            let moved = pw.position != Some((ox, oy));
            if moved {
                let started = Instant::now();
                pw.window.set_outer_position(LogicalPosition::new(ox, oy));
                pw.position = Some((ox, oy));
                pw.placed = placed_at(&pw.window, scale).unwrap_or((ox, oy));
                phases.moves += started.elapsed();
            }
            if settling {
                // Une image vide : la fenêtre s'affiche et l'affichage se met en
                // route sans que l'animal se voie encore.
                let size = pw.window.inner_size();
                pw.list.clear(size.width, size.height);
                redraw.push((pw, None, (0.0, 0.0)));
                continue;
            }
            // L'animal est dessiné là où il est, même si sa fenêtre n'a pas pu
            // monter jusque-là : il reste sous le curseur, et ce qui dépasse
            // de la fenêtre passe derrière la barre des menus (et l'encoche).
            let offset = (pw.placed.0 - ox, pw.placed.1 - oy);
            // Les personnages secondaires laissent passer les clics.
            let hit = pointer.dragging == Some(pw.pet)
                || (!pet.is_child() && cursor.is_none_or(|c| hit_test(c, &view, &pw.art)));
            if hit != pw.hittest {
                let _ = pw.window.set_cursor_hittest(hit);
                pw.hittest = hit;
            }
            if *paused && pw.last_render.is_some() {
                continue;
            }
            let sprite = SpriteKey::of(&pw.art, &view);
            let needed = if sprite.is_some() {
                pw.last_render.is_none() || sprite != pw.sprite || shifted(offset, pw.drawn_offset)
            } else {
                // En mouvement, au rythme de l'écran où se trouve la fenêtre.
                let interval = match pet.activity() {
                    Activity::Moving => Duration::from_secs_f64(1.0 / pw.hz),
                    Activity::Idle => Duration::from_secs_f64(1.0 / IDLE_FPS),
                    Activity::Asleep => Duration::from_secs_f64(1.0 / ASLEEP_FPS),
                };
                moved
                    || pw.last_render.is_none_or(|t| {
                        now.duration_since(t) + Duration::from_millis(2) >= interval
                    })
            };
            if !needed || gpu.is_none() {
                continue;
            }
            let size = pw.window.inner_size();
            pw.list.clear(size.width, size.height);
            let frame = Frame {
                origin: Vec2::new(pw.placed.0 as f32, pw.placed.1 as f32),
                scale,
            };
            let started = Instant::now();
            paint_pet(&pw.art, &view, &frame, style, &mut pw.list);
            phases.paint += started.elapsed();
            redraw.push((pw, sprite, offset));
        }
        if let Some(gpu) = gpu.as_mut()
            && !redraw.is_empty()
        {
            let mut jobs: Vec<Job<'_>> = redraw
                .iter_mut()
                .map(|(pw, _, _)| Job {
                    surface: &mut pw.surface,
                    list: &pw.list,
                    window: &pw.window,
                })
                .collect();
            let done = gpu.render_all(&mut jobs);
            drop(jobs);
            phases.renders += done.iter().filter(|&&ok| ok).count() as u32;
            for ((pw, sprite, offset), ok) in redraw.into_iter().zip(done) {
                if settling {
                    pw.last_render = None;
                } else if ok {
                    pw.last_render = Some(now);
                    pw.sprite = sprite;
                    pw.drawn_offset = offset;
                }
            }
        }
        if let Some(gpu) = self.gpu.as_mut() {
            let timings = gpu.take_timings();
            phases.paint += timings.paint;
            phases.acquire = timings.acquire;
            phases.present = timings.present;
        }

        self.schedule(event_loop, now, settling);

        let textures = self.gpu.as_ref().map_or((0, 0), Gpu::texture_usage);
        if let Some(stats) = &mut self.stats {
            stats.set_textures(textures);
            let pets = self.flock.pets().iter().map(|p| {
                let view = p.view(alpha);
                crate::stats::PetLine {
                    id: p.id(),
                    name: p.def().meta.name.as_str(),
                    behavior: p.behavior_name(),
                    frame: view.sprite_frame,
                    x: view.ground.x,
                    y: view.ground.y,
                    vx: p.velocity().x,
                    vy: p.velocity().y,
                    support: match p.support() {
                        mouflon_core::Support::Ground { .. } => 's',
                        mouflon_core::Support::Air => 'a',
                        mouflon_core::Support::Held => 't',
                    },
                    roll: view.pose.roll,
                }
            });
            // Durée d'une image de l'écran : ProMotion la change selon ce qui bouge.
            let refresh = self
                .clock
                .as_ref()
                .map_or(0.0, |clock| clock.frame_duration() * 1000.0);
            stats.record(now, shown_at.is_some(), (cadence, refresh), phases, pets);
            if self.trial_end.is_some_and(|end| now >= end) {
                log::info!("essai terminé : {}", stats.finish());
                event_loop.exit();
            }
        }
    }

    /// Prochain réveil : au battement de l'horloge de l'écran, à la cadence
    /// voulue, ou par la minuterie quand l'horloge manque ou se tait.
    fn schedule(&mut self, event_loop: &ActiveEventLoop, now: Instant, settling: bool) {
        let moving = self.clock.as_ref().map_or(self.refresh_hz, |clock| {
            self.refresh_hz.max(f64::from(clock.max_fps()))
        });
        let fps = if self.paused {
            None
        } else if settling || self.pointer.dragging.is_some() {
            Some(moving)
        } else {
            match self.flock.activity() {
                Some(Activity::Moving) => Some(moving),
                Some(Activity::Idle) => Some(IDLE_FPS),
                Some(Activity::Asleep) => Some(ASLEEP_FPS),
                None => None,
            }
        };
        self.cadence = fps.unwrap_or(0.0);
        let Some(fps) = fps else {
            // Rien ne bouge : l'horloge s'arrête, on passe de temps en temps.
            if let Some(clock) = &self.clock {
                clock.set_running(false);
            }
            self.shown_at = None;
            let wake = Duration::from_millis(if self.paused { 250 } else { 500 });
            self.next_frame = Some(now + wake);
            event_loop.set_control_flow(ControlFlow::WaitUntil(now + wake));
            return;
        };
        let interval = Duration::from_secs_f64(1.0 / fps);
        let wake = match &self.clock {
            Some(clock) => {
                clock.set_rate(fps as f32);
                clock.set_running(true);
                match self.clock_beat.map(|beat| now.duration_since(beat)) {
                    // Tant que l'horloge bat, c'est elle qui réveille.
                    Some(silent) if silent < CLOCK_SILENT => CLOCK_SILENT,
                    // Muette depuis longtemps : l'écran dort, rien ne se voit.
                    Some(silent) if silent > CLOCK_ASLEEP => Duration::from_millis(100),
                    _ => interval,
                }
            }
            None => interval,
        };
        self.next_frame = Some(now + wake);
        event_loop.set_control_flow(ControlFlow::WaitUntil(now + wake));
    }
}

/// Un événement d'un animal, pour le journal d'un essai.
fn describe(event: &PetEvent, pet: &mouflon_core::Pet) -> String {
    match event {
        PetEvent::Landed { impact } => format!("atterrit à {impact:.0} pt/s"),
        PetEvent::Poked => "touché du bout de la souris".to_owned(),
        PetEvent::Thrown { speed } => format!("lancé à {speed:.0} pt/s"),
        PetEvent::Bumped { speed } => format!("heurté, repart à {speed:.0} pt/s"),
        PetEvent::Met { other } => format!("rencontre n° {other}"),
        PetEvent::Sound { sound, clip } => format!(
            "son {} ({})",
            pet.def()
                .sounds
                .get(*sound)
                .map_or("?", |s| s.name.as_str()),
            clip + 1
        ),
    }
}

/// Temps écoulé depuis `mark`, qui passe à maintenant.
fn lap(mark: &mut Instant) -> Duration {
    let now = Instant::now();
    let lap = now - *mark;
    *mark = now;
    lap
}

/// Fréquence de l'écran d'une fenêtre, en images par seconde : celle que
/// donne macOS, sinon celle de winit, sinon 60.
fn window_hz(window: &Window, monitor: Option<&MonitorHandle>) -> f64 {
    platform::window_max_fps(window)
        .or_else(|| {
            monitor
                .and_then(MonitorHandle::refresh_rate_millihertz)
                .map(|mhz| f64::from(mhz) / 1000.0)
        })
        .map_or(60.0, |hz| hz.clamp(30.0, 240.0))
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.started {
            return;
        }
        self.started = true;
        let begun = Instant::now();
        // Essai (MOUFLON_ESSAI) : journal complet dès le début.
        let trial = crate::stats::trial();
        if let Some(duration) = trial {
            let stats = crate::stats::FrameStats::new(begun).with_journal();
            stats.raw(format!(
                "ESSAI Mouflon {} ({} s)",
                env!("CARGO_PKG_VERSION"),
                duration.as_secs_f32()
            ));
            if crate::stats::trial_collisions() {
                self.flock.set_collisions(true);
            }
            stats.raw(format!(
                "REGLAGES taille {} sons {} collisions {} premier_plan {} dock {} animaux {:?}",
                self.settings.size_factor(),
                self.settings.sounds,
                self.flock.collisions(),
                self.settings.frontmost_only,
                self.settings.walk_under_dock,
                crate::stats::trial_pets(),
            ));
            self.stats = Some(stats);
        }
        // Le relevé des fenêtres (et du Dock) commence tout de suite, à côté.
        let watcher = Watcher::start(
            self.settings.walk_under_dock,
            self.library.dir().map(std::path::Path::to_path_buf),
        );

        // Animaux du lancement : ceux de l'essai s'il en demande, sinon le préféré.
        let mut starters: Vec<usize> = Vec::new();
        if trial.is_some() {
            for id in crate::stats::trial_pets() {
                match self.library.pets.iter().position(|e| e.def.meta.id == id) {
                    Some(i) => starters.push(i),
                    None => log::warn!("essai : animal « {id} » introuvable"),
                }
            }
        }
        if starters.is_empty() {
            let index = self
                .library
                .default_index(self.settings.default_pet.as_deref());
            starters = vec![index; self.settings.start_count.clamp(1, 32) as usize];
        }
        // Leurs dessins se décodent dès maintenant, une espèce par fil, pendant
        // que le fil principal crée le menu et met le GPU en route.
        let species = self.decode_ahead(&starters);

        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let proxy = Arc::clone(&self.proxy);
            crate::menu::MenuEvent::set_event_handler(Some(move |event| {
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::Menu(event));
                }
            }));
            match crate::menu::TrayMenu::new(
                &self.library,
                self.settings.sounds,
                self.settings.frontmost_only,
                self.settings.collisions,
                self.settings.walk_under_dock,
                self.settings.size_index(),
            ) {
                Ok(menu) => self.menu = Some(menu),
                Err(e) => log::error!("impossible de créer le menu : {e}"),
            }
        }
        // Les noms et les nombres du menu sont justes avant même la première image.
        self.sync_menu();

        // Préchargement, avant la première image : dessins des animaux du
        // lancement, GPU, sons, premier relevé des fenêtres. Rien de tout
        // cela ne doit tomber pendant la première chute.
        let mut mark = Instant::now();
        let mut steps = vec![("menu", mark - begun)];
        self.gpu.get_or_insert_with(Gpu::new).prepare_ahead();
        steps.push(("GPU", lap(&mut mark)));
        if self.settings.sounds {
            self.warm_up_sounds();
        }
        steps.push(("sons", lap(&mut mark)));
        let first = watcher.wait_first(Duration::from_millis(500));
        self.watcher = Some(watcher);
        self.refresh_screens(event_loop);
        match first {
            Some(snapshot) => self.apply_snapshot(event_loop, snapshot, Instant::now()),
            None => self.refresh_windows(Instant::now()),
        }
        steps.push(("relevé des fenêtres", lap(&mut mark)));
        // Les dessins décodés à côté : on attend ceux qui ne sont pas prêts.
        for entry in &species {
            let _ = entry.art();
        }
        steps.push(("fin des dessins", lap(&mut mark)));
        for &i in &starters {
            self.spawn(event_loop, i);
        }
        steps.push(("fenêtres", lap(&mut mark)));
        self.warm_up_gpu();
        steps.push(("image d'essai", lap(&mut mark)));
        self.start_clock();
        self.settle_until = Some(Instant::now() + SETTLE);
        self.last_tick = Instant::now();
        let detail: Vec<String> = steps
            .iter()
            .map(|(step, time)| format!("{step} {}", time.as_millis()))
            .collect();
        log::info!(
            "préchargement : {} ms ({})",
            begun.elapsed().as_millis(),
            detail.join(", ")
        );
        if let Some(duration) = trial {
            log::info!("essai de {:.1} s", duration.as_secs_f32());
            self.trial_end = Some(Instant::now() + duration);
            // À la moitié, puis aux trois quarts, le dernier arrivé d'une
            // espèce s'en va (sa fenêtre se ferme) et un autre arrive.
            let start = Instant::now();
            let mut kinds: Vec<usize> = Vec::new();
            for &i in &starters {
                if !kinds.contains(&i) {
                    kinds.push(i);
                }
            }
            for (k, at) in [duration / 2, duration * 3 / 4].into_iter().enumerate() {
                if let Some(&kind) = kinds.get(k % kinds.len().max(1)) {
                    self.trial_steps
                        .push((start + at, TrialStep::Goodbye(kind)));
                    self.trial_steps
                        .push((start + at + Duration::from_secs(5), TrialStep::Add(kind)));
                }
            }
            if let Some(stats) = &self.stats {
                for (i, screen) in self.world.screens().iter().enumerate() {
                    stats.raw(format!(
                        "ECRAN {i} {:.0}x{:.0} en {:.0},{:.0} echelle {}",
                        screen.frame.w,
                        screen.frame.h,
                        screen.frame.x,
                        screen.frame.y,
                        screen.scale
                    ));
                }
                stats.raw(format!(
                    "PRECHARGEMENT {} ms ({})",
                    begun.elapsed().as_millis(),
                    detail.join(", ")
                ));
            }
        }
    }

    #[cfg_attr(
        not(any(target_os = "macos", target_os = "windows")),
        allow(unused_variables)
    )]
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            UserEvent::Menu(event) => {
                if let Some(action) = self.menu.as_ref().and_then(|m| m.action(&event)) {
                    self.handle_action(event_loop, action);
                }
            }
            UserEvent::Frame => {
                let Some(shown_at) = self.beat.take() else {
                    return;
                };
                self.clock_beat = Some(Instant::now());
                if self.started
                    && self
                        .clock
                        .as_ref()
                        .is_some_and(platform::FrameClock::is_running)
                {
                    self.frame(event_loop, Some(shown_at));
                }
            }
            UserEvent::ArtReady(id) => self.art_ready(event_loop, &id),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => {
                    self.pointer.window = Some(window_id);
                    self.pointer.pressed = self.cursor();
                    self.next_frame = None;
                    if let Some(stats) = &self.stats {
                        stats.note(format_args!(
                            "souris appuyée sur n° {:?}",
                            self.pet_of(window_id)
                        ));
                    }
                }
                ElementState::Released => {
                    self.next_frame = None;
                    if let Some(stats) = &self.stats {
                        stats.note("souris relâchée");
                    }
                    let input = self.input();
                    if let Some(id) = self.pointer.dragging.take() {
                        if let Some(pet) = self.flock.get_mut(id) {
                            pet.release(&self.world);
                        }
                    } else if self.pointer.pressed.is_some()
                        && let Some(id) = self.pet_of(window_id)
                        && let Some(pet) = self.flock.get_mut(id)
                    {
                        pet.poke(&self.world, &input);
                    }
                    self.pointer.pressed = None;
                    self.pointer.window = None;
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(pw) = self.windows.get(&window_id)
                    && pw.position.is_some()
                {
                    // Par rapport à l'endroit où la fenêtre est vraiment.
                    let (ox, oy) = pw.placed;
                    let local = position.to_logical::<f64>(pw.scale);
                    self.pointer.cursor =
                        Some(Vec2::new((ox + local.x) as f32, (oy + local.y) as f32));
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let Some(pw) = self.windows.get_mut(&window_id) {
                    pw.scale = scale_factor;
                    pw.position = None;
                    pw.last_render = None;
                }
            }
            WindowEvent::Resized(size) => {
                if let (Some(pw), Some(gpu)) = (self.windows.get_mut(&window_id), self.gpu.as_ref())
                {
                    gpu.resize(&mut pw.surface, size.width, size.height);
                    pw.last_render = None;
                }
            }
            WindowEvent::DroppedFile(path) => {
                let target = self.pet_of(window_id);
                self.install_pet(event_loop, &path, target);
            }
            WindowEvent::ThemeChanged(theme) => {
                self.style.dark = theme == Theme::Dark;
                for pw in self.windows.values_mut() {
                    pw.last_render = None;
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pw) = self.windows.get_mut(&window_id) {
                    pw.last_render = None;
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // L'horloge de l'écran donne le rythme ; sans elle (ou si elle se
        // tait), on avance ici, à l'heure prévue par la minuterie.
        let now = Instant::now();
        if self.started
            && !self.clocked(now)
            && self
                .next_frame
                .is_none_or(|next| now + Duration::from_millis(1) >= next)
        {
            self.frame(event_loop, None);
        }
    }
}
