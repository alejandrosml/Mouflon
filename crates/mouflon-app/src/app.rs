//! Boucle de l'application : simulation, scène (une fenêtre par écran), entrées et menu.

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
use mouflon_render::{PetArt, Style};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy};
use winit::monitor::MonitorHandle;
use winit::window::{Theme, Window, WindowId};

use crate::library::{Entry, Library};
use crate::menu::{Action, PetChoice, PetMenuState};
use crate::scene::Scene;
use crate::settings::{Paths, Settings};
use crate::settings_window::Shown;
use crate::watch::{Snapshot, Watcher};

/// Relevé des écrans (le reste se relève sur un fil à part).
const SCREEN_POLL: Duration = Duration::from_secs(2);
/// Au lancement, les fenêtres des écrans restent vides un court instant :
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
    /// Ce que le système dit de l'ouverture à l'ouverture de session (voir
    /// `App::ask_login_item`).
    LoginItem(platform::LoginItem),
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
    /// Une fenêtre transparente par écran, où se posent tous les animaux.
    scene: Scene,
    /// Le dessin de chaque animal à l'écran (son numéro → son dessin).
    arts: HashMap<u32, Arc<PetArt>>,
    /// Surprises à venir : quand, et laquelle si elle est imposée (un essai,
    /// `MOUFLON_SURPRISE`) ; sinon tirée au sort.
    surprises: Vec<(Instant, Option<Surprise>)>,
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
    /// Ce que fait l'utilisateur, au dernier relevé, et quand ce relevé est
    /// arrivé (voir `App::user_activity`).
    activity: Option<(Instant, platform::UserActivity)>,
    /// Personne devant la machine (essai de la CI) : on ne regarde pas ce
    /// que fait l'utilisateur.
    no_user: bool,
    /// Apps dont les fenêtres qui bougent sont signalées par l'Accessibilité
    /// (au dernier relevé), et l'autorisation d'Accessibilité.
    followed: Option<u32>,
    trusted: bool,
    /// Dernière note au journal d'une fenêtre qui bouge (essais), par fenêtre.
    window_notes: HashMap<u64, Instant>,
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
    /// Génération du troupeau pour laquelle les dessins sont à jour.
    pets_generation: Option<u64>,
    /// Ouverture à l'ouverture de session, telle que le système l'a dite à
    /// la dernière ouverture des réglages (c'est lui qui la garde).
    login: platform::LoginItem,
    /// L'animal dont le menu (clic droit) a été ouvert en dernier, et le
    /// rang de son espèce dans la bibliothèque.
    menu_pet: Option<(u32, Option<usize>)>,
    /// Vitrine de la CI (`MOUFLON_VITRINE`) : images de la fenêtre des réglages.
    showcase: Option<Showcase>,
    /// Instantanés de la scène (`MOUFLON_INSTANTANES`) : le dossier, et
    /// quand les prendre.
    snapshots: Option<(std::path::PathBuf, Vec<Instant>)>,
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

fn local_hour() -> f32 {
    let now = chrono::Local::now();
    now.hour() as f32 + now.minute() as f32 / 60.0
}

/// Le curseur est-il sur le corps de l'animal ? Hors du corps, la fenêtre
/// de l'écran laisse passer les clics vers les applications en dessous.
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
            scene: Scene::new(),
            arts: HashMap::new(),
            surprises: Vec::new(),
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
            activity: None,
            no_user: std::env::var_os("MOUFLON_ESSAI_CI").is_some(),
            followed: None,
            trusted: false,
            window_notes: HashMap::new(),
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
            pets_generation: None,
            login: platform::LoginItem::Unavailable,
            menu_pet: None,
            showcase: None,
            snapshots: None,
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

    /// Ce que fait l'utilisateur (voir `platform::user_activity`), tel que
    /// le dit le dernier relevé (quatre par seconde au moins), vieilli du
    /// temps passé depuis. Pas dans la CI : personne n'y touche à la
    /// machine, les animaux y dormiraient tout l'essai.
    fn user_activity(&self) -> Option<platform::UserActivity> {
        if self.no_user {
            return None;
        }
        let (at, known) = self.activity?;
        let age = at.elapsed().as_secs_f64();
        Some(platform::UserActivity {
            since_key: known.since_key + age,
            idle: known.idle + age,
        })
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
            // Une fenêtre transparente par écran.
            self.sync_overlays(event_loop);
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
        // Ce que fait l'utilisateur, l'app au premier plan (sauf Mouflon
        // lui-même, avec ses réglages) et l'autorisation d'Accessibilité :
        // tout vient du relevé, aucune question au système ici.
        self.activity = snapshot.activity.map(|activity| (now, activity));
        if let Some(pid) = snapshot.frontmost
            && pid != platform::current_pid()
        {
            self.frontmost = Some(pid);
        }
        let trust_changed = snapshot.trusted != self.trusted;
        self.trusted = snapshot.trusted;
        if let Some(stats) = &self.stats
            && (self.followed != Some(snapshot.followed) || trust_changed)
        {
            stats.note(format_args!(
                "fenêtres suivies par l'Accessibilité : {} app(s){}",
                snapshot.followed,
                if snapshot.trusted {
                    ""
                } else {
                    " (pas d'autorisation)"
                }
            ));
        }
        self.followed = Some(snapshot.followed);
        let dock = snapshot.dock.filter(|_| self.settings.walk_under_dock);
        if dock != self.dock_frame {
            self.dock_frame = dock;
            self.refresh_screens(event_loop);
        }
        if trust_changed {
            // La fenêtre des réglages montre ce qui manque (ou plus).
            platform::update_form(&crate::settings_window::values(&self.shown()));
        }
        if self.stats.is_some() {
            self.note_window_changes(&snapshot.windows, now);
        }
        self.os_windows = snapshot.windows;
        self.refresh_windows(now);
    }

    /// Au journal d'un essai : les fenêtres des autres apps qui arrivent,
    /// partent, bougent (au plus deux fois par seconde chacune) ou changent
    /// de niveau.
    fn note_window_changes(&mut self, windows: &[platform::OsWindow], now: Instant) {
        let Some(stats) = &self.stats else {
            return;
        };
        let describe = |w: &platform::OsWindow| {
            format!(
                "n° {} de {} (pid {}, niveau {}, {:.0}×{:.0} en {:.0},{:.0})",
                w.id, w.owner, w.pid, w.layer, w.frame.w, w.frame.h, w.frame.x, w.frame.y
            )
        };
        for w in windows {
            match self.os_windows.iter().find(|old| old.id == w.id) {
                None => stats.note(format_args!("fenêtre + {}", describe(w))),
                Some(old) if old.layer != w.layer => {
                    stats.note(format_args!(
                        "fenêtre niveau {} → {}",
                        old.layer,
                        describe(w)
                    ));
                }
                Some(old) if old.frame != w.frame => {
                    let recent = self
                        .window_notes
                        .get(&w.id)
                        .is_some_and(|at| now.duration_since(*at) < Duration::from_millis(500));
                    if !recent {
                        self.window_notes.insert(w.id, now);
                        stats.note(format_args!("fenêtre ~ {}", describe(w)));
                    }
                }
                Some(_) => {}
            }
        }
        for old in &self.os_windows {
            if !windows.iter().any(|w| w.id == old.id) {
                stats.note(format_args!("fenêtre − {}", describe(old)));
                self.window_notes.remove(&old.id);
            }
        }
    }

    /// Applique le dernier relevé des fenêtres (sans rien redemander au système).
    fn refresh_windows(&mut self, now: Instant) {
        let mut windows = Vec::new();
        let me = platform::current_pid();
        for w in &self.os_windows {
            // Les fenêtres de Mouflon ne sont jamais des rebords (le relevé
            // les écarte déjà).
            if w.pid == me {
                continue;
            }
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

    fn spawn(&mut self, index: usize) {
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
        self.sync_pets();
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

    /// Les images d'une espèce sont compressées : ses animaux prennent le
    /// dessin compressé ; les images d'avant s'oublient.
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
        let mut swapped = 0;
        for pet in self.flock.pets() {
            let Some((_, art)) = ready.iter().find(|(def, _)| Arc::ptr_eq(pet.def(), def)) else {
                continue;
            };
            if self
                .arts
                .get(&pet.id())
                .is_some_and(|old| !Arc::ptr_eq(old, art))
            {
                self.arts.insert(pet.id(), Arc::clone(art));
                swapped += 1;
            }
        }
        self.scene.forget_images();
        if let Some(stats) = &self.stats {
            let bytes: usize = ready.iter().map(|(_, art)| art.image_bytes()).sum();
            stats.note(format_args!(
                "images de {id} compressées ({:.1} Mo, {swapped} animal(aux))",
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
    fn art_ready(&mut self, id: &str) {
        self.next_frame = None;
        self.decoding.remove(id);
        let count = self.pending.remove(id).unwrap_or(0);
        if count == 0 {
            return;
        }
        if let Some(index) = self.library.pets.iter().position(|e| e.def.meta.id == id) {
            for _ in 0..count {
                self.spawn(index);
            }
        }
    }

    /// Le dessin de chaque animal : pour les nouveaux venus (dont les
    /// personnages secondaires des scènes desktopPet), on le cherche et on
    /// règle ce qui en dépend ; ceux des partis s'oublient.
    fn sync_pets(&mut self) {
        // Rien n'a changé dans le troupeau : les dessins sont à jour.
        let generation = self.flock.generation();
        if self.pets_generation == Some(generation) {
            return;
        }
        self.pets_generation = Some(generation);
        let flock = &self.flock;
        self.arts
            .retain(|id, _| flock.pets().iter().any(|p| p.id() == *id));
        let missing: Vec<(u32, Arc<PetArt>)> = self
            .flock
            .pets()
            .iter()
            .filter(|p| !self.arts.contains_key(&p.id()))
            .filter_map(|p| {
                // Dessin de la bibliothèque ; après un rechargement, celui d'un
                // animal déjà montré de la même espèce (personnages secondaires).
                let art = self.art_of(p).or_else(|| {
                    self.flock
                        .pets()
                        .iter()
                        .find(|q| Arc::ptr_eq(q.def(), p.def()))
                        .and_then(|q| self.arts.get(&q.id()))
                        .cloned()
                })?;
                Some((p.id(), art))
            })
            .collect();
        for (id, art) in missing {
            if let Some(stats) = &self.stats
                && let Some(pet) = self.flock.get_mut(id)
            {
                stats.note(format_args!("arrivée de {} (n° {id})", pet.def().meta.name));
            }
            self.fit_pet(id, art);
        }
    }

    /// Un animal prend son dessin (à son arrivée, ou à une autre taille) :
    /// ce qui en dépend pour la simulation suit.
    fn fit_pet(&mut self, id: u32, art: Arc<PetArt>) {
        if let Some(pet) = self.flock.get_mut(id) {
            // Les collisions partent quand les dessins se touchent.
            if let Some((front, back)) = art.reach() {
                pet.set_reach(front, back);
            }
            // Un autre se pose sur ce qu'on voit de lui, même couché.
            if let Some(tops) = art.tops() {
                pet.set_tops(tops);
            }
        }
        self.arts.insert(id, art);
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
                    let mut what = describe(&event, pet);
                    if let PetEvent::Landed { .. } = event
                        && let Support::Ground { surface } = pet.support()
                    {
                        what.push_str(" sur ");
                        what.push_str(&surface_name(&self.world, &self.os_windows, surface));
                    }
                    stats.note(format_args!("n° {} {what}", pet.id()));
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
    fn install_pet(&mut self, path: &std::path::Path, target: Option<u32>) {
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
                self.spawn(index);
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
    /// écrans ne seront jamais libérées (voir `keep_pet_window`).
    pub fn shutdown(&mut self) {
        if let Some(stats) = &mut self.stats {
            stats.finish();
        }
        self.scene.shutdown();
    }

    /// Une étape de l'essai, notée au journal.
    fn trial_step(&mut self, step: TrialStep) {
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
            TrialStep::Add(index) => self.spawn(index),
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
        // Chacun prend tout de suite son dessin à la nouvelle taille.
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
            self.fit_pet(pet, art);
        }
        self.scene.forget_images();
        log::info!(
            "taille des animaux : × {size} ({} à l'écran)",
            self.flock.pets().len()
        );
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
        self.fit_pet(id, art);
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
        self.scene.forget_images();
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
                self.spawn(index);
            }
            Action::Add(index) => {
                self.spawn(index);
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
                self.sync_pets();
            }
            Action::SetPetSize(pet, index) => self.set_pet_size(pet, index),
            Action::SetCollisions(on) => self.set_collisions(on),
            Action::SetRealisticPhysics(on) => self.set_realistic_physics(on),
            Action::SetSurprises(on) => self.set_surprises(on),
            Action::SetFrontmostOnly(on) => self.set_frontmost_only(on),
            Action::SetWalkUnderDock(on) => self.set_walk_under_dock(event_loop, on),
            Action::SetOpenAtLogin(on) => {
                // La case reste comme on l'a cochée en attendant la réponse.
                self.login = if on {
                    platform::LoginItem::On
                } else {
                    platform::LoginItem::Off
                };
                self.ask_login_item(Some(on));
            }
            Action::OpenSettings => self.open_settings(),
            Action::OpenGallery => self.open_gallery(false),
            Action::RefreshSettings => {
                // Relus à côté ; la fenêtre suit quand les réponses arrivent.
                self.ask_login_item(None);
                if let Some(watcher) = &self.watcher {
                    watcher.refresh_now();
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
            trusted: self.trusted,
            login: self.login,
        }
    }

    fn save_settings(&self) {
        if let Some(paths) = &self.paths {
            self.settings.save(&paths.settings);
        }
    }

    /// Demande au système, sur un fil à part, où en est l'ouverture à
    /// l'ouverture de session (`set` : la demander ou la retirer d'abord).
    /// Il met parfois une cinquantaine de millisecondes à répondre ; la
    /// réponse arrive par `UserEvent::LoginItem`.
    fn ask_login_item(&self, set: Option<bool>) {
        if !platform::login_item_available() {
            return;
        }
        let proxy = Arc::clone(&self.proxy);
        let spawned = std::thread::Builder::new()
            .name("mouflon-ouverture".into())
            .spawn(move || {
                let state = match set {
                    Some(on) => platform::set_login_item(on),
                    None => platform::login_item(),
                };
                if let Ok(proxy) = proxy.lock() {
                    let _ = proxy.send_event(UserEvent::LoginItem(state));
                }
            });
        if let Err(e) = spawned {
            log::warn!("ouverture à l'ouverture de session non relue : {e}");
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

    /// Une fenêtre transparente par écran (voir [`Scene`]) : refaites quand
    /// les écrans changent.
    fn sync_overlays(&mut self, event_loop: &ActiveEventLoop) {
        let screens = self.world.screens().to_vec();
        for note in self.scene.sync_screens(event_loop, &screens) {
            log::info!("{note}");
            if let Some(stats) = &self.stats {
                stats.note(&note);
            }
        }
        self.refresh_monitors();
    }

    /// Le dessin d'un animal, s'il est prêt (jamais de décodage ici).
    fn art_of(&self, pet: &Pet) -> Option<Arc<PetArt>> {
        self.library
            .entry_of(pet.def())
            .filter(|entry| entry.art_ready())
            .and_then(crate::library::Entry::art)
    }

    /// L'animal sous le curseur (le plus en avant).
    fn pet_under_cursor(&self) -> Option<u32> {
        self.pet_at(self.cursor()?, 1.0)
    }

    /// L'animal au point `p` (le plus en avant), à l'instant `alpha` entre
    /// deux pas. Les personnages secondaires laissent passer les clics.
    fn pet_at(&self, p: Vec2, alpha: f32) -> Option<u32> {
        self.flock
            .pets()
            .iter()
            .rev()
            .filter(|pet| !pet.is_child())
            .find(|pet| {
                self.arts
                    .get(&pet.id())
                    .is_some_and(|art| hit_test(p, &pet.view(alpha), art))
            })
            .map(Pet::id)
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

    /// Ouvre la fenêtre des réglages, avec ce que le système a dit en
    /// dernier de l'ouverture de session (relue à côté : la fenêtre suit).
    fn open_settings(&mut self) {
        self.ask_login_item(None);
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

    /// L'animal visé par un événement de la fenêtre `window` : celui sous
    /// le curseur, si c'est la fenêtre d'un écran.
    fn pet_of(&self, window: WindowId) -> Option<u32> {
        self.scene.overlay(window)?;
        self.pet_under_cursor()
    }

    /// La fréquence du plus rapide des écrans.
    fn refresh_monitors(&mut self) {
        let fastest = self
            .scene
            .overlays()
            .iter()
            .map(|overlay| window_hz(&overlay.window, overlay.window.current_monitor().as_ref()))
            .fold(0.0, f64::max);
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
            self.trial_step(step);
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
        self.sync_pets();
        self.sync_menu();
        phases.windows = lap(&mut mark);

        // La scène : chaque soucoupe, puis chaque animal, à sa place ; une
        // seule transaction pour tout ce qui change (voir `Scene`).
        self.scene.begin();
        if !settling {
            for saucer in self.flock.saucers() {
                self.scene
                    .show_saucer(saucer.id(), &saucer.view(), &self.style);
            }
            for pet in self.flock.pets() {
                if let Some(art) = self.arts.get(&pet.id()) {
                    self.scene
                        .show_pet(pet.id(), art, &pet.view(alpha), &self.style);
                }
            }
        }
        let timings = self.scene.finish();
        phases.paint += timings.paint + timings.decode;
        phases.moves += timings.layers;
        phases.worst_move = phases.worst_move.max(timings.layers);
        phases.present += timings.commit;
        phases.renders += timings.repainted;
        if let Some(gpu) = self.scene.gpu_mut() {
            let gpu_timings = gpu.take_timings();
            phases.acquire = gpu_timings.acquire;
            phases.worst_acquire = gpu_timings.worst_acquire;
            phases.misses = gpu_timings.misses;
        }

        // Chaque fenêtre d'écran prend la souris au-dessus d'un animal (ou
        // pendant qu'on en tient un), et la laisse passer ailleurs.
        let mark = Instant::now();
        let hits: Vec<bool> = self
            .scene
            .overlays()
            .iter()
            .map(|overlay| {
                self.pointer.dragging.is_some()
                    || cursor.is_some_and(|c| {
                        overlay.rect.contains(c) && self.pet_at(c, alpha).is_some()
                    })
            })
            .collect();
        for (index, hit) in hits.into_iter().enumerate() {
            // Le curseur arrive sur un animal : on va peut-être l'attraper.
            if self.scene.set_hittest(index, hit) && hit && self.pointer.dragging.is_none() {
                self.full_rate_until = Some(now + HOVER_FULL_RATE);
            }
        }
        phases.hittest += mark.elapsed();
        if !settling {
            self.take_snapshots(now);
        }

        self.schedule(event_loop, now, settling);
        if !settling {
            self.showcase(event_loop, now);
        }

        let textures = self.scene.image_usage();
        let (decoded, evicted) = self.scene.take_image_counts();
        if let Some(stats) = &mut self.stats {
            stats.set_textures(textures);
            if decoded + evicted > 0 {
                stats.counters.images_decoded += decoded;
                stats.counters.images_evicted += evicted;
            }
            // Une fois par dessin (les animaux d'une espèce le partagent).
            let mut arts: Vec<&Arc<PetArt>> = Vec::new();
            for art in self.arts.values() {
                if !arts.iter().any(|a| Arc::ptr_eq(a, art)) {
                    arts.push(art);
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

    /// Les instantanés de la scène prévus (voir `snapshots`), à leur heure.
    fn take_snapshots(&mut self, now: Instant) {
        let Some((dir, times)) = &mut self.snapshots else {
            return;
        };
        let Some(&at) = times.first() else {
            return;
        };
        if now < at {
            return;
        }
        times.remove(0);
        let started = self
            .stats
            .as_ref()
            .map_or(now, crate::stats::FrameStats::begun);
        let name = format!(
            "instantane-{:05.1}s",
            now.duration_since(started).as_secs_f32()
        );
        let written = self.scene.snapshot(dir, &name, 1.0);
        let took = now.elapsed();
        log::info!(
            "instantané de la scène : {} image(s) en {} ms",
            written.len(),
            took.as_millis()
        );
        if let Some(stats) = &self.stats {
            stats.note(format_args!(
                "instantané de la scène ({name}, {} image(s), {:.1} ms)",
                written.len(),
                took.as_secs_f32() * 1000.0
            ));
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

/// Ce sur quoi se tient une surface, pour le journal d'un essai.
fn surface_name(
    world: &World,
    windows: &[platform::OsWindow],
    surface: mouflon_core::world::SurfaceId,
) -> String {
    if world
        .surface(surface)
        .is_some_and(|s| s.kind == mouflon_core::SurfaceKind::Floor)
    {
        return "le sol de l'écran".to_owned();
    }
    if let Some(pet) = mouflon_core::physics::surface_pet(surface) {
        return format!("le dos de n° {pet}");
    }
    let window = surface >> 8;
    if window == mouflon_core::DOCK_ID {
        return "le Dock".to_owned();
    }
    match windows.iter().find(|w| w.id == window) {
        Some(w) => format!(
            "la fenêtre n° {} de {} (niveau {}, {:.0}×{:.0} en {:.0},{:.0})",
            w.id, w.owner, w.layer, w.frame.w, w.frame.h, w.frame.x, w.frame.y
        ),
        None => format!("la fenêtre n° {window} (plus au relevé)"),
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
                "REGLAGES taille {} sons {} collisions {} physique_realiste {} premier_plan {} dock {} surprises {} scene {} animaux {:?}",
                self.settings.size_factor(),
                self.settings.sounds,
                self.flock.collisions(),
                self.flock.realistic(),
                self.settings.frontmost_only,
                self.settings.walk_under_dock,
                self.settings.surprises,
                if std::env::var("MOUFLON_GPU").is_ok_and(|v| !v.is_empty() && v != "0") {
                    "gpu"
                } else {
                    "calques"
                },
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
        // Ce que le système dit de l'ouverture de session, relu à côté.
        if platform::login_item_available() {
            self.login = platform::LoginItem::Off;
            self.ask_login_item(None);
        }

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
        if self.settings.sounds {
            self.warm_up_sounds();
        }
        steps.push(("sons", lap(&mut mark)));
        // Le premier relevé, et les écrans : une fenêtre transparente par
        // écran (voir `Scene`).
        let first = watcher.wait_first(Duration::from_millis(500));
        self.watcher = Some(watcher);
        self.refresh_screens(event_loop);
        match first {
            Some(snapshot) => self.apply_snapshot(event_loop, snapshot, Instant::now()),
            None => self.refresh_windows(Instant::now()),
        }
        steps.push(("relevé et scène", lap(&mut mark)));
        // Le GPU, seulement s'il sert (sans calques Core Animation).
        if let Some(gpu) = self.scene.gpu_mut() {
            gpu.prepare_ahead();
            if let Some(gpu) = gpu.describe() {
                log::info!("GPU : {gpu}");
                if let Some(stats) = &self.stats {
                    stats.raw(format!("GPU {gpu}"));
                }
            }
            steps.push(("GPU", lap(&mut mark)));
        } else if let Some(stats) = &self.stats {
            stats.raw("SCENE calques Core Animation, une fenêtre par écran");
        }
        // Les dessins décodés à côté : on attend ceux qui ne sont pas prêts.
        for entry in &species {
            let _ = entry.art();
        }
        steps.push(("fin des dessins", lap(&mut mark)));
        for &i in &starters {
            self.spawn(i);
        }
        steps.push(("animaux", lap(&mut mark)));
        // Leurs premières images, prêtes avant qu'ils n'arrivent.
        for pet in self.flock.pets() {
            if let Some(art) = self.arts.get(&pet.id()) {
                self.scene.warm_up(art, &pet.view(1.0), &self.style);
            }
        }
        steps.push(("premières images", lap(&mut mark)));
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
        // Instantanés de la scène (`MOUFLON_INSTANTANES=dossier`) : pendant
        // un essai, à l'arrivée, au coup de vent, à la soucoupe et vers la
        // fin ; sinon, toutes les trente secondes pendant cinq minutes.
        if let Some(dir) = std::env::var_os("MOUFLON_INSTANTANES").filter(|d| !d.is_empty()) {
            let start = Instant::now();
            let times: Vec<Instant> = match trial {
                Some(d) => [
                    Duration::from_secs(4),
                    Duration::from_secs(12),
                    d / 3 + Duration::from_secs(2),
                    d * 2 / 3 + Duration::from_secs(5),
                    d * 2 / 3 + Duration::from_secs(9),
                    d.saturating_sub(Duration::from_secs(8)),
                ]
                .into_iter()
                .map(|t| start + t)
                .collect(),
                None => (1..=10)
                    .map(|k| start + Duration::from_secs(30 * k))
                    .collect(),
            };
            let mut times = times;
            times.sort();
            self.snapshots = Some((dir.into(), times));
        }
        if let Some(duration) = trial {
            log::info!("essai de {:.1} s", duration.as_secs_f32());
            self.trial_end = Some(Instant::now() + duration);
            // À la moitié, puis aux trois quarts, le dernier arrivé d'une
            // espèce s'en va (sa fenêtre se ferme) et un autre arrive : de la
            // même espèce la première fois, la seconde fois Aquarelle (des
            // morceaux peints sur un squelette), pour passer aussi par elle.
            let start = Instant::now();
            let mut kinds: Vec<usize> = Vec::new();
            for &i in &starters {
                if !kinds.contains(&i) {
                    kinds.push(i);
                }
            }
            let painted = self
                .library
                .pets
                .iter()
                .position(|e| e.def.meta.id == "aquarelle");
            for (k, at) in [duration / 2, duration * 3 / 4].into_iter().enumerate() {
                if let Some(&kind) = kinds.get(k % kinds.len().max(1)) {
                    let arrives = if k == 1 {
                        painted.unwrap_or(kind)
                    } else {
                        kind
                    };
                    self.trial_steps
                        .push((start + at, TrialStep::Goodbye(kind)));
                    self.trial_steps
                        .push((start + at + Duration::from_secs(5), TrialStep::Add(arrives)));
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
            UserEvent::LoginItem(_) => "ouverture de session relue",
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
            UserEvent::ArtReady(id) => self.art_ready(&id),
            UserEvent::ArtCompressed(id) => self.art_compressed(&id),
            UserEvent::PetSizeReady(pet, index) => self.pet_size_ready(pet, index),
            UserEvent::GalleryPreview(index, frames) => {
                platform::set_gallery_preview(index, &frames);
            }
            UserEvent::LoginItem(state) => {
                if state != self.login {
                    self.login = state;
                    self.publish();
                }
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
        _event_loop: &ActiveEventLoop,
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
        self.on_window_event(window_id, event);
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

    fn on_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
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
                if let Some(overlay) = self.scene.overlay(window_id) {
                    let local = position.to_logical::<f64>(overlay.scale);
                    self.pointer.cursor = Some(Vec2::new(
                        overlay.rect.x + local.x as f32,
                        overlay.rect.y + local.y as f32,
                    ));
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scene.scale_changed(window_id, scale_factor);
            }
            WindowEvent::Resized(size) => {
                self.scene.resized(window_id, size.width, size.height);
            }
            WindowEvent::DroppedFile(path) => {
                let target = self.pet_of(window_id);
                self.install_pet(&path, target);
            }
            // Les animaux se redessinent d'eux-mêmes (le thème fait partie
            // de ce qui les identifie).
            WindowEvent::ThemeChanged(theme) => self.style.dark = theme == Theme::Dark,
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
