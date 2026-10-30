//! Boucle de l'application : simulation, fenêtres des animaux, entrées et menu.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Timelike;
use mouflon_core::{
    Activity, Flock, Input, LedgeOptions, Pet, PetEvent, PetView, Rect, Screen, Support, Vec2,
    WindowRect, World, exact_dock,
};
use mouflon_platform as platform;
use mouflon_render::{Canvas, DisplayList, Frame, PetArt, SpriteKey, Style, paint_pet};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy};
use winit::monitor::MonitorHandle;
use winit::window::{Theme, Window, WindowId, WindowLevel};

use crate::gpu::{Gpu, Job, PetSurface};
use crate::library::{Entry, Library};
use crate::menu::{Action, PetChoice, PetMenuState};
use crate::settings::{Paths, Settings};
use crate::settings_window::Shown;
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
/// Après un mouvement vif, la cadence reste pleine ce temps-là avant de
/// ralentir : elle ne change pas d'allure à chaque rebond qui se calme.
const FULL_RATE_HOLD: Duration = Duration::from_millis(300);
/// Le curseur passe sur un animal : la cadence est pleine ce temps-là, au
/// cas où on l'attraperait.
const HOVER_FULL_RATE: Duration = Duration::from_secs(1);
/// La compression des images d'une espèce attend ce temps après son
/// arrivée : les premières chutes passent avant.
const COMPRESS_DELAY: Duration = Duration::from_secs(3);
/// Images gardées sur le disque qui n'ont pas servi depuis ce temps : effacées.
const IMAGES_KEPT: Duration = Duration::from_secs(90 * 24 * 3600);

#[derive(Debug)]
pub enum UserEvent {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Menu(crate::menu::MenuEvent),
    /// Battement de l'horloge de l'écran (voir `App::beat`).
    Frame,
    /// Le dessin de cet animal (identifiant) est prêt.
    ArtReady(String),
    /// Les images de cet animal sont compressées (voir `App::compress_later`).
    ArtCompressed(String),
    /// Le dessin à sa nouvelle taille de cet animal (son numéro, le rang de
    /// la taille) est prêt (voir `App::set_pet_size`).
    PetSizeReady(u32, usize),
    /// L'aperçu d'une espèce (son rang) pour la galerie est prêt.
    GalleryPreview(u32, Vec<platform::GalleryImage>),
    /// Un article du menu d'un animal, un contrôle de la fenêtre des réglages.
    Ui(platform::UiEvent),
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

/// La fenêtre d'une soucoupe en visite (voir `mouflon_core::Saucer`) : elle
/// laisse passer les clics.
struct SaucerWindow {
    saucer: u32,
    window: Arc<Window>,
    surface: PetSurface,
    scale: f64,
    position: Option<(f64, f64)>,
    placed: (f64, f64),
    list: DisplayList,
}

/// Une surprise (voir `mouflon_core::surprise`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Surprise {
    Wind,
    Saucer,
}

/// Délai entre deux surprises tirées au hasard (minutes).
const SURPRISE_EVERY: (f32, f32) = (15.0, 40.0);

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
    saucer_windows: HashMap<WindowId, SaucerWindow>,
    /// Surprises à venir : quand, et laquelle si elle est imposée (un essai,
    /// `MOUFLON_SURPRISE`) ; sinon tirée au sort.
    surprises: Vec<(Instant, Option<Surprise>)>,
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
    /// Dernier battement de l'horloge pas encore traité (dont l'heure, en
    /// secondes, à laquelle l'image préparée maintenant sera affichée). Les
    /// battements se remplacent : si l'app a pris du retard (une fenêtre
    /// modale…), une seule image rattrape le tout.
    beat: Rc<Cell<Option<platform::Beat>>>,
    /// Battements remplacés avant d'être traités, depuis la dernière image.
    coalesced: Rc<Cell<u32>>,
    /// Synchronisation verticale du battement précédent (secondes).
    last_vsync: Option<f64>,
    /// Événements des fenêtres reçus depuis la dernière image (essais).
    window_events: u32,
    /// Temps passé à préparer des images pendant le gestionnaire en cours :
    /// le reste du gestionnaire est noté s'il est long (essais).
    frame_time: Duration,
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
    /// Animaux qui attendent leur dessin à une autre taille, et cette entrée.
    resizing: Vec<(u32, Entry)>,
    /// Essai (`MOUFLON_ESSAI`) : fin prévue, étapes à venir et mesures de fluidité.
    trial_end: Option<Instant>,
    /// Ce que fait l'utilisateur, au dernier relevé (voir `App::user_activity`).
    activity: Cell<Option<(Instant, platform::UserActivity)>>,
    /// Personne devant la machine (essai de la CI) : on ne regarde pas ce
    /// que fait l'utilisateur.
    no_user: bool,
    /// Apps dont les fenêtres bougeant sont signalées, et si l'Accessibilité
    /// le permettait (voir `App::follow_moves`).
    followed: Option<(bool, Vec<i32>)>,
    /// L'autorisation d'Accessibilité, et quand elle a été relue.
    trust: Option<(Instant, bool)>,
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
    /// Jusqu'à quand la cadence reste pleine après un mouvement vif.
    full_rate_until: Option<Instant>,
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
    /// Ouverture à l'ouverture de session, telle que le système l'a dite à
    /// la dernière ouverture des réglages (c'est lui qui la garde).
    login: platform::LoginItem,
    /// L'animal dont le menu (clic droit) a été ouvert en dernier, et le
    /// rang de son espèce dans la bibliothèque.
    menu_pet: Option<(u32, Option<usize>)>,
    /// Vitrine de la CI (`MOUFLON_VITRINE`) : images de la fenêtre des réglages.
    showcase: Option<Showcase>,
}

/// La vitrine de la CI : à l'heure dite, la fenêtre des réglages s'ouvre,
/// puis son image est enregistrée (claire, puis avec ses remarques, claire
/// et sombre), avec le texte des menus ; l'app quitte ensuite.
#[derive(Debug)]
struct Showcase {
    dir: std::path::PathBuf,
    at: Instant,
    opened: bool,
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
        flock.set_realistic(settings.realistic_physics);
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
            saucer_windows: HashMap::new(),
            surprises: Vec::new(),
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
            coalesced: Rc::new(Cell::new(0)),
            last_vsync: None,
            window_events: 0,
            frame_time: Duration::ZERO,
            clock_beat: None,
            shown_at: None,
            settle_until: None,
            next_frame: None,
            pending: HashMap::new(),
            decoding: HashSet::new(),
            resizing: Vec::new(),
            trial_end: None,
            activity: Cell::new(None),
            no_user: std::env::var_os("MOUFLON_ESSAI_CI").is_some(),
            followed: None,
            trust: None,
            trial_steps: Vec::new(),
            stats: None,
            frontmost: None,
            dock: None,
            dock_logged: false,
            floating_seen: std::collections::HashSet::new(),
            seed,
            refresh_hz: 60.0,
            cadence: 0.0,
            full_rate_until: None,
            proxy: Arc::new(Mutex::new(proxy)),
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu_flock: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            menu_generation: None,
            windows_generation: None,
            login: platform::LoginItem::Unavailable,
            menu_pet: None,
            showcase: None,
        }
    }

    fn cursor(&self) -> Option<Vec2> {
        platform::cursor_position()
            .map(|(x, y)| Vec2::new(x as f32, y as f32))
            .or(self.pointer.cursor)
    }

    fn input(&self) -> Input {
        let activity = self.user_activity();
        Input {
            cursor: self.cursor(),
            local_hour: local_hour(),
            since_key: activity.map(|a| a.since_key as f32),
            idle: activity.map(|a| a.idle as f32),
        }
    }

    /// Ce que fait l'utilisateur (voir `platform::user_activity`), demandé
    /// au système deux fois par seconde au plus. Pas dans la CI : personne
    /// n'y touche à la machine, les animaux y dormiraient tout l'essai.
    fn user_activity(&self) -> Option<platform::UserActivity> {
        if self.no_user {
            return None;
        }
        let now = Instant::now();
        if let Some((at, known)) = self.activity.get() {
            let age = now.duration_since(at).as_secs_f64();
            if age < 0.5 {
                return Some(platform::UserActivity {
                    since_key: known.since_key + age,
                    idle: known.idle + age,
                });
            }
        }
        let fresh = platform::user_activity();
        self.activity.set(fresh.map(|a| (now, a)));
        fresh
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
        self.last_vsync = None;
        let beat = Rc::clone(&self.beat);
        let coalesced = Rc::clone(&self.coalesced);
        let proxy = Arc::clone(&self.proxy);
        self.clock = platform::FrameClock::new(move |tick| {
            // Un seul battement en attente à la fois.
            if beat.replace(Some(tick)).is_some() {
                coalesced.set(coalesced.get() + 1);
            } else if let Ok(proxy) = proxy.lock() {
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
            // Le Dock se lit (ou plus) : l'autorisation d'Accessibilité a pu
            // changer, la fenêtre des réglages le montre.
            if dock.is_some() != self.dock_frame.is_some() {
                platform::update_form(&crate::settings_window::values(&self.shown()));
            }
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
        self.follow_moves();
        self.refresh_windows(now);
    }

    /// Les apps dont les fenêtres servent de rebords préviennent quand l'une
    /// d'elles bouge (avec l'autorisation d'Accessibilité seulement).
    fn follow_moves(&mut self) {
        let mut pids: Vec<i32> = self
            .os_windows
            .iter()
            .filter(|w| w.is_walkable())
            .map(|w| w.pid)
            .collect();
        pids.sort_unstable();
        pids.dedup();
        // L'autorisation peut arriver en cours de route : relue toutes les
        // deux secondes.
        let now = Instant::now();
        let trusted = match self.trust {
            Some((at, trusted)) if now.duration_since(at) < SCREEN_POLL => trusted,
            _ => {
                let trusted = platform::accessibility_trusted();
                self.trust = Some((now, trusted));
                trusted
            }
        };
        if self.followed.as_ref() == Some(&(trusted, pids.clone())) {
            return;
        }
        let count = platform::follow_window_moves(&pids);
        if let Some(stats) = &self.stats
            && self.followed.as_ref().is_none_or(|(t, _)| *t != trusted)
        {
            stats.note(format_args!(
                "fenêtres suivies par l'Accessibilité : {count} app(s){}",
                if trusted { "" } else { " (pas d'autorisation)" }
            ));
        }
        self.followed = Some((trusted, pids));
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
        self.compress_later(&entry);
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

    /// Compresse les images d'une espèce sur des fils de fond, un peu après
    /// son arrivée (voir [`crate::library::Entry::compress`]) ;
    /// `UserEvent::ArtCompressed` arrive quand c'est fait. Rien à faire pour
    /// un dessin vectoriel, du pixel art agrandi, ou des images reprises
    /// déjà compressées du disque.
    fn compress_later(&self, entry: &Entry) {
        if !entry.compressible() {
            return;
        }
        let job = (entry.clone(), Arc::clone(&self.proxy));
        let spawned = std::thread::Builder::new()
            .name("mouflon-compression".into())
            .spawn(move || {
                let (entry, proxy) = job;
                platform::lower_thread_priority();
                std::thread::sleep(COMPRESS_DELAY);
                let threads =
                    std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).clamp(1, 4));
                let Some(done) = entry.compress(threads, &platform::lower_thread_priority) else {
                    return;
                };
                log::info!(
                    "images de {} compressées en {} ms sur {threads} fils : {:.1} → {:.1} Mo",
                    entry.def.meta.name,
                    done.took.as_millis(),
                    done.before as f64 / 1e6,
                    done.after as f64 / 1e6,
                );
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::ArtCompressed(entry.def.meta.id.clone()));
                }
            });
        if let Err(e) = spawned {
            log::warn!("images non compressées : {e}");
        }
    }

    /// Les images d'une espèce sont compressées : ses fenêtres prennent le
    /// dessin compressé, et le GPU oublie les images d'avant.
    fn art_compressed(&mut self, id: &str) {
        // Toutes les tailles de l'espèce dont le dessin est prêt (la
        // bibliothèque a pu être rechargée entre-temps : jamais de décodage
        // ici, sur le fil de l'affichage).
        let ready: Vec<(Arc<mouflon_core::PetDef>, Arc<PetArt>)> = self
            .library
            .sizes_of(id)
            .filter(|entry| entry.art_ready())
            .filter_map(|entry| Some((Arc::clone(&entry.def), entry.art()?)))
            .collect();
        if ready.is_empty() {
            return;
        }
        let pets = self.flock.pets();
        let mut swapped = 0;
        for (def, art) in &ready {
            for pw in self.windows.values_mut() {
                let same = pets
                    .iter()
                    .any(|p| p.id() == pw.pet && Arc::ptr_eq(p.def(), def));
                if same && !Arc::ptr_eq(&pw.art, art) {
                    pw.art = Arc::clone(art);
                    // Redessinée à la prochaine image, avec les images compressées.
                    pw.sprite = None;
                    swapped += 1;
                }
            }
        }
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.forget_unused_images();
        }
        if let Some(stats) = &self.stats {
            let bytes: usize = ready.iter().map(|(_, art)| art.image_bytes()).sum();
            stats.note(format_args!(
                "images de {id} compressées ({:.1} Mo, {swapped} fenêtre(s))",
                bytes as f64 / 1e6
            ));
        }
        // Les images d'origine sont libérées : leurs pages retournent au
        // système tout de suite, plutôt que dans quelques minutes.
        let _ = std::thread::Builder::new()
            .name("mouflon-menage".into())
            .spawn(|| {
                platform::lower_thread_priority();
                let released = platform::release_free_memory();
                if released >= 1 << 20 {
                    log::info!(
                        "{:.1} Mo de mémoire libérée rendus au système",
                        released as f64 / 1e6
                    );
                }
            });
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
                    .entry_of(p.def())
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
            match crate::menu::TrayMenu::new(&self.library, self.settings.sounds) {
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

    /// Taille d'un animal : la sienne s'il en a une à lui, celle de tous sinon.
    fn size_of(&self, pet: &Pet) -> f32 {
        self.library
            .entry_of(pet.def())
            .map_or(self.library.size(), |entry| entry.size)
    }

    /// Change la taille de tous les animaux (ceux qui en avaient une à eux
    /// compris), sans les interrompre.
    fn set_size(&mut self, index: usize) {
        let size = crate::settings::SIZES[index.min(crate::settings::SIZES.len() - 1)];
        let before: HashMap<u32, f32> = self
            .flock
            .pets()
            .iter()
            .map(|pet| (pet.id(), self.size_of(pet)))
            .collect();
        let unchanged = (size - self.settings.size_factor()).abs() < f32::EPSILON
            && before.values().all(|s| (s - size).abs() < 1e-3);
        if unchanged {
            return;
        }
        self.resizing.clear();
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
            let old = before.get(&pet.id()).copied().unwrap_or(size);
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
        let refits: Vec<(u32, Arc<PetArt>)> = self
            .flock
            .pets()
            .iter()
            .filter_map(|pet| {
                let art = self
                    .library
                    .entry_of(pet.def())
                    .and_then(crate::library::Entry::art)?;
                Some((pet.id(), art))
            })
            .collect();
        for (pet, art) in refits {
            self.refit_window(pet, art);
        }
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.forget_unused_images();
        }
        log::info!(
            "taille des animaux : × {size} ({} à l'écran)",
            self.flock.pets().len()
        );
    }

    /// La fenêtre d'un animal prend son nouveau dessin (une autre taille),
    /// sans se fermer.
    fn refit_window(&mut self, pet: u32, art: Arc<PetArt>) {
        let Some(pw) = self.windows.values_mut().find(|w| w.pet == pet) else {
            return;
        };
        pw.canvas = art.canvas();
        if let Some((front, back)) = art.reach()
            && let Some(pet) = self.flock.get_mut(pet)
        {
            pet.set_reach(front, back);
        }
        pw.art = art;
        let requested = LogicalSize::new(f64::from(pw.canvas.width), f64::from(pw.canvas.height));
        if let Some(size) = pw.window.request_inner_size(requested)
            && let Some(gpu) = self.gpu.as_ref()
        {
            gpu.resize(&mut pw.surface, size.width, size.height);
        }
        pw.position = None;
        pw.last_render = None;
        pw.sprite = None;
    }

    /// Change la taille d'un seul animal (son menu). Son dessin à cette
    /// taille se prépare sur un fil à part s'il ne l'est pas déjà : les
    /// animaux continuent de bouger, et il grandit (ou rapetisse) dès que
    /// c'est prêt (`UserEvent::PetSizeReady`).
    fn set_pet_size(&mut self, id: u32, index: usize) {
        let index = index.min(crate::settings::SIZES.len() - 1);
        let size = crate::settings::SIZES[index];
        let Some(pet) = self.flock.pets().iter().find(|p| p.id() == id) else {
            return;
        };
        if (self.size_of(pet) - size).abs() < 1e-3 {
            return;
        }
        let species = pet.def().meta.id.clone();
        let Some(entry) = self
            .library
            .pets
            .iter()
            .position(|e| e.def.meta.id == species)
            .and_then(|i| self.library.at_size(i, size))
        else {
            return;
        };
        self.resizing.retain(|(pet, _)| *pet != id);
        if entry.art_ready() {
            self.apply_pet_size(id, &entry);
            return;
        }
        self.resizing.push((id, entry.clone()));
        let job = (entry, Arc::clone(&self.proxy));
        let spawned = std::thread::Builder::new()
            .name("mouflon-dessins".into())
            .spawn(move || {
                let (entry, proxy) = job;
                let _ = entry.art();
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::PetSizeReady(id, index));
                }
            });
        if let Err(e) = spawned {
            log::warn!("dessin préparé sur le fil principal : {e}");
            self.pet_size_ready(id, index);
        }
    }

    /// Le dessin attendu par un animal pour sa nouvelle taille est prêt.
    fn pet_size_ready(&mut self, id: u32, index: usize) {
        let size = crate::settings::SIZES[index.min(crate::settings::SIZES.len() - 1)];
        // Toujours voulu (une autre taille a pu être choisie entre-temps) ?
        let Some(at) = self
            .resizing
            .iter()
            .position(|(pet, entry)| *pet == id && (entry.size - size).abs() < 1e-3)
        else {
            return;
        };
        let (_, entry) = self.resizing.remove(at);
        self.apply_pet_size(id, &entry);
    }

    fn apply_pet_size(&mut self, id: u32, entry: &Entry) {
        let Some(art) = entry.art() else {
            return;
        };
        let Some(old) = self
            .flock
            .pets()
            .iter()
            .find(|p| p.id() == id)
            .map(|pet| self.size_of(pet))
        else {
            return;
        };
        let rescaled = self
            .flock
            .get_mut(id)
            .is_some_and(|pet| pet.rescale(Arc::clone(&entry.def), entry.size / old));
        if !rescaled {
            return;
        }
        self.refit_window(id, art);
        self.compress_later(entry);
        // Les tailles dont plus personne ne se sert sont oubliées, et leurs
        // images avec.
        let used: Vec<Arc<mouflon_core::PetDef>> = self
            .flock
            .pets()
            .iter()
            .map(|p| Arc::clone(p.def()))
            .chain(self.resizing.iter().map(|(_, e)| Arc::clone(&e.def)))
            .collect();
        self.library
            .forget_unused_sizes(|def| used.iter().any(|u| Arc::ptr_eq(u, def)));
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.forget_unused_images();
        }
        log::info!("n° {id} : taille × {}", entry.size);
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
                self.make_favorite(index);
            }
            Action::RemoveOne(index) => self.goodbye_one(index),
            Action::RemoveAll => self.remove_all(),
            Action::Goodbye(id) => self.goodbye_pet(id),
            Action::Together => {
                let input = self.input();
                let count = self.flock.together(&self.world, &input);
                log::info!("tous ensemble : {count} animal(aux)");
            }
            Action::SetPaused(paused) => self.set_paused(paused),
            Action::SetSounds(on) => self.set_sounds(on),
            Action::SetSize(index) => {
                self.set_size(index);
                self.sync_windows(event_loop);
            }
            Action::SetPetSize(pet, index) => self.set_pet_size(pet, index),
            Action::SetCollisions(on) => self.set_collisions(on),
            Action::SetRealisticPhysics(on) => self.set_realistic_physics(on),
            Action::SetSurprises(on) => self.set_surprises(on),
            Action::SetFrontmostOnly(on) => self.set_frontmost_only(on),
            Action::SetWalkUnderDock(on) => self.set_walk_under_dock(event_loop, on),
            Action::SetOpenAtLogin(on) => self.login = platform::set_login_item(on),
            Action::OpenSettings => self.open_settings(),
            Action::OpenGallery => self.open_gallery(false),
            Action::RefreshSettings => self.login = platform::login_item(),
            Action::OpenPetsFolder => {
                if let Some(paths) = &self.paths {
                    let _ = std::fs::create_dir_all(&paths.pets);
                    if !platform::open_folder(&paths.pets) {
                        log::warn!("impossible d'ouvrir {}", paths.pets.display());
                    }
                }
            }
            Action::OpenAccessibilitySettings => {
                if !platform::open_accessibility_settings() {
                    log::warn!("impossible d'ouvrir les réglages d'Accessibilité");
                }
            }
            Action::OpenLoginItemsSettings => platform::open_login_items_settings(),
            Action::About => show_about(),
            Action::Quit => event_loop.exit(),
        }
        // La barre des menus et la fenêtre des réglages montrent l'état
        // qui en résulte, d'où que vienne le changement.
        self.publish();
    }

    /// Montre l'état de l'app partout où il se voit : la barre des menus
    /// (pause, sons, animaux à l'écran) et la fenêtre des réglages. Le menu
    /// d'un animal, lui, est refait à chaque clic droit.
    fn publish(&mut self) {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            if let Some(menu) = &self.menu {
                menu.set_paused(self.paused);
                menu.set_sounds(self.settings.sounds);
            }
            // Le préféré a pu changer sans qu'un animal arrive encore.
            self.menu_generation = None;
        }
        self.sync_menu();
        platform::update_form(&crate::settings_window::values(&self.shown()));
    }

    /// Ce que la fenêtre des réglages montre de l'état de l'app et du système.
    fn shown(&self) -> Shown {
        Shown {
            size: self.settings.size_index(),
            sounds: self.settings.sounds,
            surprises: self.settings.surprises,
            // Celles du troupeau : un essai peut les imposer.
            collisions: self.flock.collisions(),
            realistic_physics: self.flock.realistic(),
            frontmost_only: self.settings.frontmost_only,
            walk_under_dock: self.settings.walk_under_dock,
            trusted: platform::accessibility_trusted(),
            login: self.login,
        }
    }

    fn save_settings(&self) {
        if let Some(paths) = &self.paths {
            self.settings.save(&paths.settings);
        }
    }

    /// Le dernier animal choisi devient celui du lancement et du premier
    /// article du menu.
    fn make_favorite(&mut self, index: usize) {
        if let Some(entry) = self.library.pets.get(index)
            && self.settings.default_pet.as_deref() != Some(entry.def.meta.id.as_str())
        {
            self.settings.default_pet = Some(entry.def.meta.id.clone());
            self.save_settings();
        }
    }

    fn set_paused(&mut self, paused: bool) {
        if self.paused == paused {
            return;
        }
        self.paused = paused;
        // Au réveil, les animaux repartent d'où ils étaient, sans rattraper
        // le temps de la pause.
        self.last_tick = Instant::now();
        self.shown_at = None;
    }

    fn set_sounds(&mut self, on: bool) {
        if self.settings.sounds == on {
            return;
        }
        self.settings.sounds = on;
        self.save_settings();
        if on {
            self.warm_up_sounds();
        }
    }

    fn set_surprises(&mut self, on: bool) {
        if self.settings.surprises == on {
            return;
        }
        self.settings.surprises = on;
        self.save_settings();
        // Celles qui n'étaient pas imposées partent ou reviennent.
        self.surprises.retain(|(_, kind)| kind.is_some());
        if on {
            self.schedule_surprise(Instant::now());
        }
    }

    /// La prochaine surprise tirée au sort, dans un quart d'heure à
    /// quarante minutes.
    fn schedule_surprise(&mut self, now: Instant) {
        if !self.settings.surprises || self.surprises.iter().any(|(_, k)| k.is_none()) {
            return;
        }
        let minutes = SURPRISE_EVERY.0 + (SURPRISE_EVERY.1 - SURPRISE_EVERY.0) * self.random_unit();
        self.surprises
            .push((now + Duration::from_secs_f32(minutes * 60.0), None));
        self.surprises.sort_by_key(|(at, _)| *at);
    }

    /// Un nombre au hasard dans [0, 1) (même suite que les graines des
    /// animaux).
    fn random_unit(&mut self) -> f32 {
        self.seed = self
            .seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self.seed >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Une surprise à son heure : un coup de vent, ou une soucoupe (si un
    /// animal est là pour elle). Pas devant personne : elle attend son retour.
    fn surprise(&mut self, now: Instant, input: &Input) {
        let Some((next, &(at, imposed))) = self
            .surprises
            .iter()
            .enumerate()
            .min_by_key(|(_, (at, _))| *at)
        else {
            return;
        };
        if now < at {
            return;
        }
        self.surprises.remove(next);
        if imposed.is_none() && (input.away() || self.flock.pets().is_empty()) {
            // Personne pour la voir : un peu plus tard.
            self.surprises.push((now + Duration::from_secs(60), None));
            self.surprises.sort_by_key(|(at, _)| *at);
            return;
        }
        let kind = imposed.unwrap_or(if self.random_unit() < 0.55 {
            Surprise::Wind
        } else {
            Surprise::Saucer
        });
        let saucer = match kind {
            Surprise::Saucer => {
                let seed = (self.random_unit() * 1e6) as u64;
                self.flock.invite_saucer(&self.world, seed)
            }
            Surprise::Wind => None,
        };
        let what = if saucer.is_some() {
            "une soucoupe volante".to_owned()
        } else {
            // Pas d'animal pour la soucoupe : un coup de vent à la place.
            let dir = if self.random_unit() < 0.5 { -1.0 } else { 1.0 };
            let seconds = 6.0 + 3.0 * self.random_unit();
            self.flock.blow(dir, seconds);
            format!(
                "un coup de vent vers la {} ({seconds:.1} s)",
                if dir > 0.0 { "droite" } else { "gauche" }
            )
        };
        log::info!("surprise : {what}");
        if let Some(stats) = &self.stats {
            stats.note(format_args!("surprise : {what}"));
        }
        if imposed.is_none() {
            self.schedule_surprise(now);
        }
    }

    /// Une fenêtre par soucoupe en visite ; on ferme celles des parties.
    fn sync_saucers(&mut self, event_loop: &ActiveEventLoop) {
        let visiting: Vec<u32> = self
            .flock
            .saucers()
            .iter()
            .map(mouflon_core::Saucer::id)
            .collect();
        for w in self
            .saucer_windows
            .values()
            .filter(|w| !visiting.contains(&w.saucer))
        {
            platform::keep_pet_window(&*w.window);
        }
        self.saucer_windows
            .retain(|_, w| visiting.contains(&w.saucer));
        for id in visiting {
            if self.saucer_windows.values().any(|w| w.saucer == id) {
                continue;
            }
            let canvas = mouflon_render::saucer::saucer_canvas();
            let attrs = Window::default_attributes()
                .with_title("Mouflon — soucoupe")
                .with_decorations(false)
                .with_transparent(true)
                .with_resizable(false)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_inner_size(LogicalSize::new(
                    f64::from(canvas.width),
                    f64::from(canvas.height),
                ))
                .with_position(LogicalPosition::new(-10_000.0, -10_000.0))
                .with_active(false);
            #[cfg(target_os = "macos")]
            let attrs = {
                use winit::platform::macos::WindowAttributesExtMacOS;
                attrs.with_has_shadow(false)
            };
            let window = match event_loop.create_window(attrs) {
                Ok(w) => Arc::new(w),
                Err(e) => {
                    log::error!("impossible de créer la fenêtre de la soucoupe : {e}");
                    continue;
                }
            };
            platform::prepare_pet_window(&*window);
            let _ = window.set_cursor_hittest(false);
            let gpu = self.gpu.get_or_insert_with(Gpu::new);
            let size = window.inner_size();
            let surface = match gpu.create_surface(Arc::clone(&window), size.width, size.height) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("rendu de la soucoupe indisponible : {e}");
                    platform::keep_pet_window(&*window);
                    continue;
                }
            };
            let scale = window.scale_factor();
            self.saucer_windows.insert(
                window.id(),
                SaucerWindow {
                    saucer: id,
                    window,
                    surface,
                    scale,
                    position: None,
                    placed: (-10_000.0, -10_000.0),
                    list: DisplayList::default(),
                },
            );
        }
    }

    fn set_realistic_physics(&mut self, on: bool) {
        if self.settings.realistic_physics == on && self.flock.realistic() == on {
            return;
        }
        self.settings.realistic_physics = on;
        self.flock.set_realistic(on);
        self.save_settings();
    }

    fn set_collisions(&mut self, on: bool) {
        if self.settings.collisions == on && self.flock.collisions() == on {
            return;
        }
        self.settings.collisions = on;
        self.flock.set_collisions(on);
        self.save_settings();
    }

    fn set_frontmost_only(&mut self, on: bool) {
        if self.settings.frontmost_only == on {
            return;
        }
        self.settings.frontmost_only = on;
        self.save_settings();
        self.refresh_windows(Instant::now());
    }

    fn set_walk_under_dock(&mut self, event_loop: &ActiveEventLoop, on: bool) {
        if self.settings.walk_under_dock == on {
            return;
        }
        self.settings.walk_under_dock = on;
        self.save_settings();
        // Lire le Dock demande l'autorisation d'Accessibilité : macOS
        // propose de l'accorder si Mouflon ne l'a pas encore.
        if on && !platform::request_accessibility() {
            log::info!("Marcher sous le Dock : en attente de l'autorisation d'Accessibilité");
        }
        self.dock_logged = false;
        // Le fil de relevé lit le Dock tout de suite (ou cesse de le lire).
        if let Some(watcher) = &self.watcher {
            watcher.set_dock(on);
        }
        if !on {
            self.dock_frame = None;
        }
        self.refresh_screens(event_loop);
        self.refresh_windows(Instant::now());
    }

    /// Cet animal-là dit au revoir (le menu de son clic droit).
    fn goodbye_pet(&mut self, id: u32) {
        let input = self.input();
        if self.flock.leave(id, &self.world, &input) && self.pointer.dragging == Some(id) {
            self.pointer = Pointer::default();
        }
    }

    /// Ouvre la fenêtre des réglages, avec ce que le système dit de
    /// l'ouverture de session à cet instant.
    fn open_settings(&mut self) {
        self.login = platform::login_item();
        let form = crate::settings_window::form(crate::i18n::texts(), self.login);
        let values = crate::settings_window::values(&self.shown());
        if !platform::show_form(&form, &values)
            && let Some(paths) = &self.paths
        {
            // Pas de fenêtre des réglages sur ce système : le fichier, à
            // modifier à la main (il est lu au lancement).
            self.settings.save(&paths.settings);
            platform::open_folder(&paths.settings);
        }
    }

    /// La galerie des animaux : une carte par espèce de la bibliothèque. Les
    /// aperçus se dessinent sur un fil à part et arrivent un à un
    /// (`UserEvent::GalleryPreview`) ; `now` : tout de suite, sur ce fil (la
    /// vitrine de la CI).
    fn open_gallery(&mut self, now: bool) {
        let t = crate::i18n::texts();
        let cards: Vec<platform::GalleryCard> = self
            .library
            .pets
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let meta = &entry.def.meta;
                let mut detail = meta.description.clone().unwrap_or_default();
                if let Some(author) = meta.author.as_deref().filter(|a| !a.is_empty()) {
                    if !detail.is_empty() {
                        detail.push('\n');
                    }
                    detail.push_str(&t.gallery_by.replace("{author}", author));
                }
                platform::GalleryCard {
                    id: i as u32,
                    name: meta.name.clone(),
                    detail,
                    button: t.gallery_add.clone(),
                }
            })
            .collect();
        if !platform::show_gallery(&t.gallery_title, &cards) {
            return;
        }
        let entries: Vec<(u32, Entry)> = self
            .library
            .pets
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, e)| (i as u32, e))
            .collect();
        let previews = move |send: &dyn Fn(u32, Vec<platform::GalleryImage>)| {
            for (index, entry) in &entries {
                let Some(art) = entry.art() else {
                    continue;
                };
                let frames = mouflon_render::thumbnail::preview(&entry.def, &art, 10, 10.0, 2.0)
                    .into_iter()
                    .map(|thumb| platform::GalleryImage {
                        width: thumb.width,
                        height: thumb.height,
                        rgba: thumb.rgba,
                        scale: 2.0,
                    })
                    .collect();
                send(*index, frames);
            }
        };
        if now {
            previews(&|index, frames| platform::set_gallery_preview(index, &frames));
            return;
        }
        let proxy = Arc::clone(&self.proxy);
        let spawned = std::thread::Builder::new()
            .name("mouflon-galerie".into())
            .spawn(move || {
                platform::lower_thread_priority();
                previews(&|index, frames| {
                    if let Ok(proxy) = proxy.lock() {
                        let _ = proxy.send_event(UserEvent::GalleryPreview(index, frames));
                    }
                });
            });
        if let Err(e) = spawned {
            log::warn!("aperçus de la galerie indisponibles : {e}");
        }
    }

    /// Le menu d'un animal, au clic droit (ou Contrôle-clic) sur lui.
    fn open_pet_menu(&mut self, window: WindowId) {
        if self.pointer.dragging.is_some() {
            return;
        }
        let Some(id) = self.pet_of(window) else {
            return;
        };
        let Some(pet) = self.flock.pets().iter().find(|p| p.id() == id) else {
            return;
        };
        // Les personnages secondaires et ceux qui partent n'en ont pas.
        if pet.is_child() || pet.is_leaving() {
            return;
        }
        let name = pet.def().meta.name.clone();
        let species = self
            .library
            .pets
            .iter()
            .position(|e| e.def.meta.id == pet.def().meta.id);
        let state = PetMenuState {
            paused: self.paused,
            sounds: self.settings.sounds,
            size: crate::settings::size_index_of(self.size_of(pet)),
        };
        let items = crate::menu::pet_menu(&name, state, crate::i18n::texts());
        self.menu_pet = Some((id, species));
        // Le clic n'est pas un « coucou », ni le début d'un glisser.
        self.pointer.pressed = None;
        self.pointer.window = None;
        if let Some(stats) = &self.stats {
            stats.note(format_args!("menu de n° {id}"));
        }
        // Cet animal attend qu'on ait choisi, sous le menu ; les autres
        // continuent (le menu s'ouvre au prochain tour de la boucle, voir
        // `platform::pop_up_menu`).
        self.flock.hold_still(Some(id));
        platform::pop_up_menu(&items);
    }

    /// Ce que fait l'article choisi dans le menu d'un animal.
    fn pet_menu_action(&self, id: u32) -> Option<Action> {
        let (pet, species) = self.menu_pet?;
        PetChoice::from_id(id)?.action(pet, species)
    }

    /// La vitrine de la CI, à son heure (voir [`Showcase`]).
    fn showcase(&mut self, event_loop: &ActiveEventLoop, now: Instant) {
        let Some(showcase) = self.showcase.as_mut() else {
            return;
        };
        if now < showcase.at {
            return;
        }
        if !showcase.opened {
            showcase.opened = true;
            showcase.at = now + Duration::from_secs(1);
            self.open_settings();
            return;
        }
        let Some(showcase) = self.showcase.take() else {
            return;
        };
        let t = crate::i18n::texts();
        let code = crate::i18n::code();
        let _ = std::fs::create_dir_all(&showcase.dir);
        let image = |name: &str| showcase.dir.join(format!("reglages-{code}-{name}.png"));
        let mut saved = vec![platform::snapshot_form(&image("clair"), false)];
        // Avec ses remarques (autorisation et accord en attente), et la
        // section « Démarrage » même si le système ne la permet pas ici.
        let full = Shown {
            walk_under_dock: true,
            trusted: false,
            login: platform::LoginItem::NeedsApproval,
            ..self.shown()
        };
        let form = crate::settings_window::form(t, platform::LoginItem::Off);
        platform::show_form(&form, &crate::settings_window::values(&full));
        saved.push(platform::snapshot_form(&image("remarques-clair"), false));
        saved.push(platform::snapshot_form(&image("remarques-sombre"), true));
        let state = PetMenuState {
            paused: false,
            sounds: true,
            size: 1,
        };
        // La galerie, avec ses aperçus.
        self.open_gallery(true);
        saved.push(platform::snapshot_gallery(
            &showcase.dir.join(format!("galerie-{code}-clair.png")),
            false,
        ));
        saved.push(platform::snapshot_gallery(
            &showcase.dir.join(format!("galerie-{code}-sombre.png")),
            true,
        ));
        let menu = crate::menu::pet_menu("Lainou", state, t);
        let text = format!("{menu:#?}\n");
        let _ = std::fs::write(showcase.dir.join(format!("menu-animal-{code}.txt")), text);
        log::info!("vitrine : images enregistrées {saved:?}");
        event_loop.exit();
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
    /// `tick` : le battement de l'horloge de l'écran qui l'a réveillée (avec
    /// l'heure d'affichage de l'image), s'il y en a un.
    fn frame(&mut self, event_loop: &ActiveEventLoop, tick: Option<platform::Beat>) {
        let now = Instant::now();
        self.frame_body(event_loop, tick, now);
        self.frame_time += now.elapsed();
    }

    fn frame_body(
        &mut self,
        event_loop: &ActiveEventLoop,
        tick: Option<platform::Beat>,
        now: Instant,
    ) {
        let shown_at = tick.map(|t| t.target);
        let cadence = self.cadence;
        // Pour le journal d'un essai : ce que dit l'horloge, le retard de la
        // minuterie, les allocations de l'image.
        let diagnosing = self.stats.is_some();
        let mut beat_timing = None;
        let mut timer_late = None;
        let allocs_before = diagnosing.then(platform::alloc_counts);
        if diagnosing {
            platform::diag::frame_started();
            match tick {
                Some(t) => {
                    let interval = (t.target - t.vsync).max(1e-4);
                    let missed = self.last_vsync.map_or(0, |last| {
                        (((t.vsync - last) / interval).round() as i64 - 1).max(0) as u32
                    });
                    beat_timing = Some(crate::stats::BeatTiming {
                        latency: ((t.called - t.vsync) * 1000.0) as f32,
                        queue: ((platform::diag::media_time() - t.called) * 1000.0) as f32,
                        missed,
                        coalesced: self.coalesced.replace(0),
                    });
                }
                None => {
                    timer_late = self.next_frame.map(|planned| {
                        now.saturating_duration_since(planned).as_secs_f32() * 1000.0
                    });
                }
            }
        }
        if let Some(t) = tick {
            self.last_vsync = Some(t.vsync);
        }
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
        if !settling && !self.paused {
            self.surprise(now, &input);
        }

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
        self.sync_saucers(event_loop);
        self.sync_menu();
        phases.windows = lap(&mut mark);

        let Self {
            windows,
            saucer_windows,
            flock,
            gpu,
            style,
            pointer,
            paused,
            full_rate_until,
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
                let took = started.elapsed();
                phases.moves += took;
                phases.worst_move = phases.worst_move.max(took);
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
                let started = Instant::now();
                let _ = pw.window.set_cursor_hittest(hit);
                pw.hittest = hit;
                phases.hittest += started.elapsed();
                // Le curseur arrive sur un animal : on va peut-être l'attraper.
                if hit && pointer.dragging.is_none() {
                    *full_rate_until = Some(now + HOVER_FULL_RATE);
                }
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
        // Les soucoupes en visite, à chaque image.
        let mut saucers: Vec<&mut SaucerWindow> = Vec::new();
        for sw in saucer_windows.values_mut() {
            let Some(saucer) = flock.saucers().iter().find(|s| s.id() == sw.saucer) else {
                continue;
            };
            if settling || *paused || gpu.is_none() {
                continue;
            }
            let view = saucer.view();
            let canvas = mouflon_render::saucer::saucer_canvas();
            let ox = (f64::from(view.pos.x) - f64::from(canvas.anchor.x)).floor();
            let oy = (f64::from(view.pos.y) - f64::from(canvas.anchor.y)).floor();
            if sw.position != Some((ox, oy)) {
                let started = Instant::now();
                sw.window.set_outer_position(LogicalPosition::new(ox, oy));
                sw.position = Some((ox, oy));
                sw.placed = placed_at(&sw.window, sw.scale).unwrap_or((ox, oy));
                phases.moves += started.elapsed();
            }
            let size = sw.window.inner_size();
            sw.list.clear(size.width, size.height);
            let frame = Frame {
                origin: Vec2::new(sw.placed.0 as f32, sw.placed.1 as f32),
                scale: sw.scale,
            };
            let started = Instant::now();
            mouflon_render::saucer::paint_saucer(&view, &frame, style, &mut sw.list);
            phases.paint += started.elapsed();
            saucers.push(sw);
        }
        if let Some(gpu) = gpu.as_mut()
            && !(redraw.is_empty() && saucers.is_empty())
        {
            let mut jobs: Vec<Job<'_>> = redraw
                .iter_mut()
                .map(|(pw, _, _)| Job {
                    surface: &mut pw.surface,
                    list: &pw.list,
                    window: &pw.window,
                })
                .collect();
            jobs.extend(saucers.iter_mut().map(|sw| Job {
                surface: &mut sw.surface,
                list: &sw.list,
                window: &sw.window,
            }));
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
            phases.worst_acquire = timings.worst_acquire;
            phases.misses = timings.misses;
        }

        self.schedule(event_loop, now, settling);
        if !settling {
            self.showcase(event_loop, now);
        }

        let textures = self.gpu.as_ref().map_or((0, 0), Gpu::texture_usage);
        if let Some(stats) = &mut self.stats {
            stats.set_textures(textures);
            // Une fois par espèce (ses fenêtres partagent le même dessin).
            let mut arts: Vec<&Arc<PetArt>> = Vec::new();
            for pw in self.windows.values() {
                if !arts.iter().any(|a| Arc::ptr_eq(a, &pw.art)) {
                    arts.push(&pw.art);
                }
            }
            stats.set_images(arts.iter().map(|a| a.image_bytes()).sum());
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
            let allocs = allocs_before.map_or((0, 0), |before| {
                let after = platform::alloc_counts();
                (
                    after.main_allocs.saturating_sub(before.main_allocs),
                    after.main_bytes.saturating_sub(before.main_bytes),
                )
            });
            let record = crate::stats::FrameRecord {
                started: now,
                beat: beat_timing,
                timer_late,
                cadence,
                refresh,
                phases,
                allocs,
                events: std::mem::take(&mut self.window_events),
            };
            stats.record(&record, pets);
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
        // Le curseur sur un animal, ou le bouton appuyé : on va sans doute
        // l'attraper. L'écran passe à pleine cadence d'avance (ProMotion met
        // quelques images à monter de 60 à 120 Hz).
        let anticipating =
            self.pointer.pressed.is_some() || self.full_rate_until.is_some_and(|until| now < until);
        let fps = if self.paused {
            None
        } else if settling || self.pointer.dragging.is_some() || anticipating {
            Some(moving)
        } else {
            match self.flock.activity() {
                Some(Activity::Moving) => {
                    // Toute la cadence de l'écran pour un mouvement vif ; la
                    // moitié (ProMotion) quand tous vont leur petit train.
                    let needed = self.flock.smooth_rate();
                    if f64::from(needed) >= moving {
                        let hold = now + FULL_RATE_HOLD;
                        self.full_rate_until =
                            Some(self.full_rate_until.map_or(hold, |until| until.max(hold)));
                    }
                    if self.full_rate_until.is_some_and(|until| now < until) {
                        Some(moving)
                    } else {
                        Some(moving_rate(needed, moving))
                    }
                }
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
            if self.stats.is_some() {
                platform::diag::expect_frame(wake);
            }
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
        if self.stats.is_some() {
            // L'horloge qui bat amène l'image suivante dans un intervalle ;
            // sinon, c'est la minuterie.
            let clocked = self.clock.is_some() && wake == CLOCK_SILENT;
            platform::diag::expect_frame(if clocked { interval } else { wake });
        }
    }
}

/// Cadence pour des animaux qui bougent : la plus lente qui divise celle de
/// l'écran (`full`) sans descendre sous celle qu'il leur faut (`needed`,
/// voir `Pet::smooth_rate`). Un diviseur : l'écran montre chaque image le
/// même nombre de fois, et le mouvement reste régulier.
fn moving_rate(needed: f32, full: f64) -> f64 {
    let needed = f64::from(needed);
    if !needed.is_finite() || needed >= full {
        return full;
    }
    let divisor = (full / needed.max(1.0)).floor().max(1.0);
    full / divisor
}

/// Un événement d'un animal, pour le journal d'un essai.
fn describe(event: &PetEvent, pet: &mouflon_core::Pet) -> String {
    match event {
        PetEvent::Landed { impact } => format!("atterrit à {impact:.0} pt/s"),
        PetEvent::Poked => "touché du bout de la souris".to_owned(),
        PetEvent::Thrown { speed } => format!("lancé à {speed:.0} pt/s"),
        PetEvent::Bumped { speed } => format!("heurté, repart à {speed:.0} pt/s"),
        PetEvent::Met { other } => format!("rencontre n° {other}"),
        PetEvent::Startled => "sursaute (pointeur rapide)".to_owned(),
        PetEvent::Napping => "s'endort (utilisateur parti)".to_owned(),
        PetEvent::Woken => "se réveille (utilisateur revenu)".to_owned(),
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
        // Les images gardées pour des animaux ou des tailles délaissés s'effacent.
        if let Some(dir) = self.paths.as_ref().map(|p| p.images.clone()) {
            let _ = std::thread::Builder::new()
                .name("mouflon-menage".into())
                .spawn(move || {
                    platform::lower_thread_priority();
                    let removed = mouflon_render::cache::prune(&dir, IMAGES_KEPT);
                    if removed > 0 {
                        log::info!("{removed} fichier(s) d'images délaissés effacés");
                    }
                });
        }
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
                "VERSION {} commit {} {}",
                env!("CARGO_PKG_VERSION"),
                option_env!("GITHUB_SHA").map_or("local", |sha| &sha[..sha.len().min(12)]),
                if cfg!(debug_assertions) {
                    "debug"
                } else {
                    "release"
                },
            ));
            let wall = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            stats.raw(format!(
                "DEBUT {:.3} {}",
                wall.as_secs_f64(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f %z")
            ));
            for line in platform::diag::system_info() {
                stats.raw(line);
            }
            stats.raw(format!(
                "REGLAGES taille {} sons {} collisions {} physique_realiste {} premier_plan {} dock {} animaux {:?}",
                self.settings.size_factor(),
                self.settings.sounds,
                self.flock.collisions(),
                self.flock.realistic(),
                self.settings.frontmost_only,
                self.settings.walk_under_dock,
                crate::stats::trial_pets(),
            ));
            // Diagnostics : pile du fil principal quand une image tarde,
            // ressources du processus, de ses fils et du système, allocations,
            // notifications du système (voir `platform::diag`).
            platform::set_alloc_counting(true);
            platform::diag::register_main_thread();
            if let Some(sender) = stats.sender() {
                crate::stats::install_panic_hook(sender.clone(), begun);
                let to_journal = sender.clone();
                platform::diag::start(
                    begun,
                    Box::new(move |line| {
                        let _ = to_journal.send(line);
                    }),
                );
                platform::diag::observe_system(move |line| {
                    let _ = sender.send(line);
                });
            }
            self.stats = Some(stats);
        }
        // Le relevé des fenêtres (et du Dock) commence tout de suite, à côté.
        let watcher = Watcher::start(
            self.settings.walk_under_dock,
            self.library.dir().map(std::path::Path::to_path_buf),
            self.stats
                .as_ref()
                .and_then(|s| Some((s.sender()?, s.begun()))),
        );
        // Avec l'Accessibilité, une fenêtre qui bouge est relevée aussitôt
        // (voir `follow_window_moves`).
        let nudger = watcher.nudger();
        platform::set_window_moves_handler(move || nudger.nudge());

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

        // Le menu d'un animal et la fenêtre des réglages : leurs événements
        // arrivent par la boucle, comme ceux de la barre des menus.
        let proxy = Arc::clone(&self.proxy);
        platform::set_ui_handler(move |event| {
            if let Ok(proxy) = proxy.lock() {
                let _ = proxy.send_event(UserEvent::Ui(event));
            }
        });
        // Vitrine de la CI : la fenêtre des réglages s'ouvre après l'arrivée
        // des animaux.
        if let Some(dir) = crate::stats::showcase() {
            self.showcase = Some(Showcase {
                dir,
                at: begun + Duration::from_secs(3),
                opened: false,
            });
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let proxy = Arc::clone(&self.proxy);
            crate::menu::MenuEvent::set_event_handler(Some(move |event| {
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::Menu(event));
                }
            }));
            match crate::menu::TrayMenu::new(&self.library, self.settings.sounds) {
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
        let gpu = self.gpu.get_or_insert_with(Gpu::new);
        gpu.prepare_ahead();
        if let Some(gpu) = gpu.describe() {
            log::info!("GPU : {gpu}");
            if let Some(stats) = &self.stats {
                stats.raw(format!("GPU {gpu}"));
            }
        }
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
        // Une surprise tout de suite, pour la voir (`MOUFLON_SURPRISE=vent`
        // ou `soucoupe`) ; sinon, la première au hasard.
        let soon = Instant::now() + SETTLE + Duration::from_secs(4);
        match std::env::var("MOUFLON_SURPRISE").ok().as_deref() {
            Some("vent") => self.surprises.push((soon, Some(Surprise::Wind))),
            Some("soucoupe") => self.surprises.push((soon, Some(Surprise::Saucer))),
            _ => {}
        }
        self.schedule_surprise(Instant::now());
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
            // Les surprises passent aussi : un coup de vent au premier tiers,
            // une soucoupe aux deux tiers.
            self.surprises
                .push((start + duration / 3, Some(Surprise::Wind)));
            self.surprises
                .push((start + duration * 2 / 3, Some(Surprise::Saucer)));
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
        let started = self.handler_start();
        let kind = match &event {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            UserEvent::Menu(_) => "menu de la barre",
            UserEvent::Frame => "battement",
            UserEvent::ArtReady(_) => "dessin prêt",
            UserEvent::ArtCompressed(_) => "images compressées",
            UserEvent::PetSizeReady(..) => "dessin à une autre taille prêt",
            UserEvent::GalleryPreview(..) => "aperçu de la galerie prêt",
            UserEvent::Ui(_) => "menu d'un animal ou réglages",
        };
        if let Some(stats) = &mut self.stats {
            if matches!(event, UserEvent::Frame) {
                stats.counters.beats += 1;
            } else {
                stats.counters.user += 1;
            }
        }
        match event {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            UserEvent::Menu(event) => {
                if let Some(action) = self.menu.as_ref().and_then(|m| m.action(&event)) {
                    self.handle_action(event_loop, action);
                }
            }
            UserEvent::Frame => {
                if let Some(tick) = self.beat.take() {
                    self.clock_beat = Some(Instant::now());
                    if self.started
                        && self
                            .clock
                            .as_ref()
                            .is_some_and(platform::FrameClock::is_running)
                    {
                        self.frame(event_loop, Some(tick));
                    }
                }
            }
            UserEvent::ArtReady(id) => self.art_ready(event_loop, &id),
            UserEvent::ArtCompressed(id) => self.art_compressed(&id),
            UserEvent::PetSizeReady(pet, index) => self.pet_size_ready(pet, index),
            UserEvent::GalleryPreview(index, frames) => {
                platform::set_gallery_preview(index, &frames);
            }
            UserEvent::Ui(platform::UiEvent::MenuClosed) => {
                // L'animal du menu repart d'où il était.
                self.flock.hold_still(None);
                if let Some(stats) = &self.stats {
                    stats.note("menu refermé");
                }
            }
            UserEvent::Ui(event) => {
                let action = match event {
                    platform::UiEvent::Menu(id) => self.pet_menu_action(id),
                    // « Ajouter » sur une carte de la galerie : son espèce.
                    platform::UiEvent::GalleryAdd(index) => Some(Action::Add(index as usize)),
                    other => crate::settings_window::action(other),
                };
                if let Some(action) = action {
                    self.handle_action(event_loop, action);
                }
            }
        }
        self.handler_done(kind, started);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let started = self.handler_start();
        let kind = window_event_kind(&event);
        self.window_events += 1;
        if let Some(stats) = &mut self.stats {
            let c = &mut stats.counters;
            match &event {
                WindowEvent::CursorMoved { .. } => c.cursor_moves += 1,
                WindowEvent::MouseInput { .. } => c.clicks += 1,
                WindowEvent::CursorEntered { .. } => c.hovers += 1,
                WindowEvent::RedrawRequested => c.redraws += 1,
                _ => c.other_window += 1,
            }
        }
        self.on_window_event(event_loop, window_id, event);
        self.handler_done(kind, started);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let started = self.handler_start();
        if let Some(stats) = &mut self.stats {
            stats.counters.waits += 1;
        }
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
        self.handler_done("avant l'attente", started);
    }
}

impl App {
    /// Début d'un gestionnaire d'événements (pour noter ceux qui sont lents).
    fn handler_start(&mut self) -> Option<Instant> {
        self.frame_time = Duration::ZERO;
        self.stats.is_some().then(Instant::now)
    }

    /// Fin d'un gestionnaire : au-delà de 4 ms hors de la préparation des
    /// images (notée à part), il est noté au journal d'un essai.
    fn handler_done(&mut self, kind: &str, started: Option<Instant>) {
        let (Some(started), Some(stats)) = (started, self.stats.as_mut()) else {
            return;
        };
        let other = started.elapsed().saturating_sub(self.frame_time);
        if other > Duration::from_millis(4) {
            let ms = other.as_secs_f32() * 1000.0;
            stats.counters.slow_handlers += 1;
            stats.counters.slowest_handler = stats.counters.slowest_handler.max(ms);
            stats.note(format_args!("gestionnaire lent : {kind} {ms:.1} ms"));
        }
    }

    fn on_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            // Le curseur arrive sur un animal : on va peut-être l'attraper.
            WindowEvent::CursorEntered { .. } => {
                self.full_rate_until = Some(Instant::now() + HOVER_FULL_RATE);
            }
            // Clic droit : le menu de l'animal.
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => self.open_pet_menu(window_id),
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                // Contrôle-clic : le menu de l'animal aussi, comme partout sur
                // Mac. Rien n'est appuyé ensuite : le bouton relâché (si le
                // menu le laisse passer) n'est pas un « coucou ».
                ElementState::Pressed if platform::control_key_down() => {
                    self.open_pet_menu(window_id);
                }
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
}

/// Le nom d'un événement de fenêtre, pour le journal d'un essai.
fn window_event_kind(event: &WindowEvent) -> &'static str {
    match event {
        WindowEvent::MouseInput { .. } => "clic",
        WindowEvent::CursorMoved { .. } => "souris",
        WindowEvent::CursorEntered { .. } => "souris entrée",
        WindowEvent::CursorLeft { .. } => "souris sortie",
        WindowEvent::RedrawRequested => "redessin demandé",
        WindowEvent::Resized(_) => "taille",
        WindowEvent::Moved(_) => "déplacée",
        WindowEvent::ScaleFactorChanged { .. } => "échelle",
        WindowEvent::ThemeChanged(_) => "thème",
        WindowEvent::DroppedFile(_) => "fichier déposé",
        WindowEvent::Focused(_) => "focus",
        WindowEvent::Occluded(_) => "occultée",
        _ => "autre événement de fenêtre",
    }
}

#[cfg(test)]
mod tests {
    use super::moving_rate;

    #[test]
    fn slow_pets_halve_a_promotion_screen_but_never_drop_below_their_need() {
        // ProMotion : la moitié pour un animal au pas.
        assert_eq!(moving_rate(60.0, 120.0), 60.0);
        // Un écran à 60 Hz ne change pas.
        assert_eq!(moving_rate(60.0, 60.0), 60.0);
        // Toujours un diviseur de la cadence de l'écran, jamais sous le besoin.
        assert_eq!(moving_rate(60.0, 144.0), 72.0);
        assert_eq!(moving_rate(60.0, 240.0), 60.0);
        assert_eq!(moving_rate(60.0, 90.0), 90.0);
        // Un mouvement vif : toute la cadence.
        assert_eq!(moving_rate(f32::INFINITY, 120.0), 120.0);
    }
}
