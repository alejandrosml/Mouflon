//! Boucle de l'application : simulation, fenêtres des animaux, entrées et menu.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Timelike;
use mouflon_core::{
    Activity, Flock, Input, LedgeOptions, PetView, Rect, Screen, Support, Vec2, WindowRect, World,
    exact_dock,
};
use mouflon_platform as platform;
use mouflon_render::{Canvas, DisplayList, Frame, Look, PetArt, SpriteMotion, Style, paint_pet};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy};
use winit::window::{Theme, Window, WindowId, WindowLevel};

use crate::gpu::{Gpu, PetSurface};
use crate::library::Library;
use crate::menu::Action;
use crate::settings::{Paths, Settings};

/// Intervalle de relevé des fenêtres quand rien ne bouge, et quand une
/// fenêtre portant un animal est en train d'être déplacée.
const WINDOW_POLL_CALM: Duration = Duration::from_millis(250);
const WINDOW_POLL_FAST: Duration = Duration::from_millis(33);
const SCREEN_POLL: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub enum UserEvent {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Menu(crate::menu::MenuEvent),
    #[allow(dead_code)]
    Wake,
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
    next_window_poll: Instant,
    next_screen_poll: Instant,
    fast_poll_until: Instant,
    frontmost: Option<i32>,
    /// Le Dock traité comme une fenêtre posée devant les autres (« Marcher sous le Dock »).
    dock: Option<WindowRect>,
    /// Le Dock a déjà été signalé au journal.
    dock_logged: bool,
    /// Panneaux flottants déjà signalés au journal (propriétaire, niveau).
    floating_seen: std::collections::HashSet<(String, i32)>,
    seed: u64,
    refresh_hz: f64,
    proxy: EventLoopProxy<UserEvent>,
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    menu: Option<crate::menu::TrayMenu>,
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
            next_window_poll: now,
            next_screen_poll: now,
            fast_poll_until: now,
            frontmost: None,
            dock: None,
            dock_logged: false,
            floating_seen: std::collections::HashSet::new(),
            seed,
            refresh_hz: 60.0,
            proxy,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu: None,
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
        // Le Dock tel qu'il est, si on a le droit de le lire.
        self.dock = None;
        if self.settings.walk_under_dock
            && let Some(frame) = platform::dock_frame()
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
            self.world.set_screens(screens);
        }
    }

    fn refresh_windows(&mut self, now: Instant) {
        let me = platform::current_pid();
        if let Some(pid) = platform::frontmost_pid()
            && pid != me
        {
            self.frontmost = Some(pid);
        }
        let mut windows = Vec::new();
        for w in platform::windows() {
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
        // Dessin préparé à la première apparition de l'animal.
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

    /// Une fenêtre par animal : on en crée pour les nouveaux venus (dont les
    /// personnages secondaires des scènes desktopPet) et on ferme celles des partis.
    fn sync_windows(&mut self, event_loop: &ActiveEventLoop) {
        let alive: Vec<u32> = self.flock.pets().iter().map(|p| p.id()).collect();
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
        let Some(pet) = self.flock.pets().iter().find(|p| p.id() == id) else {
            return;
        };
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
        if let Some(hz) = window
            .current_monitor()
            .and_then(|m| m.refresh_rate_millihertz())
        {
            self.refresh_hz = self
                .refresh_hz
                .max(f64::from(hz) / 1000.0)
                .clamp(30.0, 240.0);
        }
        if window.theme() == Some(Theme::Dark) {
            self.style.dark = true;
        }
        let gpu = self.gpu.get_or_insert_with(Gpu::new);
        let size = window.inner_size();
        let surface = match gpu.create_surface(Arc::clone(&window), size.width, size.height) {
            Ok(s) => s,
            Err(e) => {
                log::error!("rendu indisponible : {e}");
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
            },
        );
    }

    /// Reconstruit le menu après un changement de la bibliothèque.
    fn rebuild_menu(&mut self) {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            self.menu = None;
            match crate::menu::TrayMenu::new(
                &self.library,
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
                self.refresh_screens(event_loop);
                self.refresh_windows(Instant::now());
            }
            Action::SetSize(index) => {
                self.set_size(index);
                self.sync_windows(event_loop);
            }
            Action::TogglePause => {
                self.paused = !self.paused;
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

    /// Une image : simulation, position des fenêtres, rendu, prochain réveil.
    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        if now >= self.next_screen_poll {
            self.refresh_screens(event_loop);
            // Un animal ajouté au dossier apparaît dans le menu sans relancer l'app.
            if self.library.changed() {
                log::info!("dossier des animaux modifié : rechargement");
                self.library.reload();
                self.rebuild_menu();
            }
            self.next_screen_poll = now + SCREEN_POLL;
        }
        if now >= self.next_window_poll {
            self.refresh_windows(now);
            let fast = now < self.fast_poll_until || self.pointer.dragging.is_some();
            self.next_window_poll = now
                + if fast {
                    WINDOW_POLL_FAST
                } else {
                    WINDOW_POLL_CALM
                };
        }

        let input = self.input();
        let cursor = input.cursor;

        // Glisser : on n'attrape l'animal qu'après un petit déplacement,
        // un simple clic reste un « coucou ». Il est pris là où l'on a cliqué,
        // et ce point reste sous le curseur.
        if let (Some(start), Some(c), None) = (self.pointer.pressed, cursor, self.pointer.dragging)
            && (c - start).length() > 3.0
            && let Some(id) = self.pointer.window.and_then(|w| self.pet_of(w))
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.grab(start, &self.world);
            self.pointer.dragging = Some(id);
        }
        if let (Some(id), Some(c)) = (self.pointer.dragging, cursor)
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.drag_to(c);
        }

        let alpha = if self.paused {
            1.0
        } else {
            self.flock.advance(elapsed, &self.world, &input)
        };
        self.sync_windows(event_loop);

        let Self {
            windows,
            flock,
            gpu,
            list,
            style,
            pointer,
            refresh_hz,
            paused,
            ..
        } = self;
        let moving_interval = Duration::from_secs_f64(1.0 / *refresh_hz);
        for pw in windows.values_mut() {
            let Some(pet) = flock.pets().iter().find(|p| p.id() == pw.pet) else {
                continue;
            };
            let view = pet.view(alpha);
            let scale = pw.scale;
            let ox = ((f64::from(view.ground.x) - f64::from(pw.canvas.anchor.x)) * scale).floor()
                / scale;
            let oy = ((f64::from(view.ground.y) - f64::from(pw.canvas.anchor.y)) * scale).floor()
                / scale;
            let moved = pw.position != Some((ox, oy));
            if moved {
                pw.window.set_outer_position(LogicalPosition::new(ox, oy));
                pw.position = Some((ox, oy));
                pw.placed = placed_at(&pw.window, scale).unwrap_or((ox, oy));
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
                pw.last_render.is_none() || sprite != pw.sprite || offset != pw.drawn_offset
            } else {
                let interval = match pet.activity() {
                    Activity::Moving => moving_interval,
                    Activity::Idle => Duration::from_millis(33),
                    Activity::Asleep => Duration::from_millis(50),
                };
                moved
                    || pw.last_render.is_none_or(|t| {
                        now.duration_since(t) + Duration::from_millis(2) >= interval
                    })
            };
            if !needed {
                continue;
            }
            let Some(gpu) = gpu.as_mut() else { continue };
            let size = pw.window.inner_size();
            list.clear(size.width, size.height);
            let frame = Frame {
                origin: Vec2::new(pw.placed.0 as f32, pw.placed.1 as f32),
                scale,
            };
            paint_pet(&pw.art, &view, &frame, style, list);
            if gpu.render(&mut pw.surface, list, &pw.window) {
                pw.last_render = Some(now);
                pw.sprite = sprite;
                pw.drawn_offset = offset;
            }
        }

        let wake = if self.paused {
            Duration::from_millis(250)
        } else if self.pointer.dragging.is_some() {
            moving_interval
        } else {
            match self.flock.activity() {
                Some(Activity::Moving) => moving_interval,
                Some(Activity::Idle) => Duration::from_millis(33),
                Some(Activity::Asleep) => Duration::from_millis(50),
                None => Duration::from_millis(500),
            }
        };
        event_loop.set_control_flow(ControlFlow::WaitUntil(now + wake));
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.started {
            return;
        }
        self.started = true;
        self.refresh_screens(event_loop);
        self.refresh_windows(Instant::now());

        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let proxy = std::sync::Mutex::new(self.proxy.clone());
            crate::menu::MenuEvent::set_event_handler(Some(move |event| {
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::Menu(event));
                }
            }));
            match crate::menu::TrayMenu::new(
                &self.library,
                self.settings.frontmost_only,
                self.settings.collisions,
                self.settings.walk_under_dock,
                self.settings.size_index(),
            ) {
                Ok(menu) => self.menu = Some(menu),
                Err(e) => log::error!("impossible de créer le menu : {e}"),
            }
        }

        let index = self
            .library
            .default_index(self.settings.default_pet.as_deref());
        for _ in 0..self.settings.start_count.clamp(1, 32) {
            self.spawn(event_loop, index);
        }
        self.last_tick = Instant::now();
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
            UserEvent::Wake => {}
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
                }
                ElementState::Released => {
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
        if self.started {
            self.frame(event_loop);
        }
    }
}
