//! L'affichage : le troupeau, le monde, la scène et le rythme des images.
//!
//! Sur Mac, il tourne sur son propre fil, réveillé par l'horloge de l'écran
//! (un `CADisplayLink` inscrit sur la boucle de ce fil) : un menu ouvert, la
//! fenêtre des réglages, une requête d'Accessibilité d'une autre app ou un
//! premier chargement de code sur le fil principal ne retardent plus une
//! image. Le fil principal lui envoie ce que fait l'utilisateur ([`Command`])
//! et reçoit ce qu'il doit montrer dans les menus ([`Report`]).
//!
//! Les demandes sont numérotées dans l'ordre où elles partent ; quand
//! l'affichage décrit le troupeau, il dit combien il en a traité : le fil
//! principal sait ainsi ce que la description sait déjà.
//!
//! Quand la scène passe par le GPU (`MOUFLON_GPU=1`, ou un système sans
//! calques Core Animation), l'affichage reste sur le fil principal, rythmé
//! par la boucle de winit : c'est le même code, avec une autre horloge.

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use chrono::Timelike;
use mouflon_core::{
    Activity, Flock, Input, LedgeOptions, PetDef, PetEvent, PetView, Rect, Screen, Support, Vec2,
    WindowRect, World, exact_dock,
};
use mouflon_platform as platform;
use mouflon_render::{PetArt, Style};
use winit::event_loop::EventLoopProxy;

use crate::app::UserEvent;
use crate::scene::{Overlay, Scene};
use crate::stats::FrameStats;
use crate::watch::{Snapshot, Watcher};

/// Au lancement, la scène reste vide un court instant : le temps que macOS
/// affiche les fenêtres et que tout soit en route avant que le premier
/// animal ne tombe.
pub const SETTLE: Duration = Duration::from_millis(300);
/// Sans battement de l'horloge de l'écran depuis ce délai (écran en veille,
/// débranché…), la minuterie prend le relais ; au-delà du second, l'écran
/// dort sans doute : on ralentit.
const CLOCK_SILENT: Duration = Duration::from_millis(150);
const CLOCK_ASLEEP: Duration = Duration::from_secs(1);
/// Une horloge qui tourne mais ne bat plus depuis ce délai (écran changé,
/// réveil…) est refaite ; pas plus souvent.
const CLOCK_DEAD: Duration = Duration::from_secs(2);
/// Cadences quand les animaux se reposent ou dorment (images par seconde).
const IDLE_FPS: f64 = 30.0;
const ASLEEP_FPS: f64 = 20.0;
/// Après un mouvement vif, la cadence reste pleine ce temps-là avant de
/// ralentir : elle ne change pas d'allure à chaque rebond qui se calme.
const FULL_RATE_HOLD: Duration = Duration::from_millis(300);
/// Le curseur passe sur un animal : la cadence est pleine ce temps-là, au
/// cas où on l'attraperait.
const HOVER_FULL_RATE: Duration = Duration::from_secs(1);
/// Délai entre deux surprises tirées au hasard (minutes).
const SURPRISE_EVERY: (f32, f32) = (15.0, 40.0);
/// Volume de l'ensemble des sons des animaux (de 0 à 1) : discret, c'est un
/// compagnon de bureau.
const VOLUME: f32 = 0.55;
/// Sons lancés au plus par fenêtre de temps (les suivants se taisent).
const SOUNDS_PER_WINDOW: u32 = 4;
const SOUND_WINDOW: Duration = Duration::from_millis(500);

/// Une surprise (voir `mouflon_core::surprise`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surprise {
    Wind,
    Saucer,
}

/// Les réglages qui changent ce que fait l'affichage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Live {
    pub paused: bool,
    pub sounds: bool,
    pub collisions: bool,
    pub realistic: bool,
    pub surprises: bool,
    pub frontmost_only: bool,
    pub walk_under_dock: bool,
}

/// Un changement de dessin : les animaux dessinés d'après `from` prennent
/// `to` (agrandis ou réduits de `factor`) et `art`.
#[derive(Clone, Debug)]
pub struct Redraw {
    pub from: Arc<PetDef>,
    pub to: Arc<PetDef>,
    pub art: Arc<PetArt>,
    pub factor: f32,
}

/// Ce que le fil principal demande à l'affichage.
#[derive(Debug)]
pub enum Command {
    /// Les écrans tels que le système les donne (avant le Dock), et la
    /// fenêtre de chacun.
    Screens {
        screens: Vec<Screen>,
        overlays: Vec<Overlay>,
    },
    /// Un relevé des fenêtres arrivé avant que l'affichage soit en route.
    Survey(Snapshot),
    /// Un animal arrive, avec son dessin.
    Spawn {
        def: Arc<PetDef>,
        art: Arc<PetArt>,
    },
    /// Les premières images se préparent, puis la scène se montre après
    /// une courte mise en route (`settle`) : les animaux arrivent.
    Start {
        settle: Duration,
    },
    /// L'animal sous le curseur s'en va tout de suite (un autre, déposé sur
    /// lui, prend sa place).
    RemoveUnderCursor,
    /// Cet animal dit au revoir.
    Goodbye(u32),
    /// Le dernier arrivé de cette espèce (identifiant de sa définition) dit
    /// au revoir.
    GoodbyeSpecies(String),
    GoodbyeAll,
    Together,
    /// Changements de dessin ; les animaux dessinés d'après une définition
    /// de `gone` (plus dans la bibliothèque) s'en vont.
    Redraw {
        changes: Vec<Redraw>,
        gone: Vec<Arc<PetDef>>,
    },
    /// Cet animal-là change de taille.
    RedrawPet {
        id: u32,
        def: Arc<PetDef>,
        art: Arc<PetArt>,
        factor: f32,
    },
    Live(Live),
    Dark(bool),
    /// La fenêtre de l'écran `key` a changé de taille (GPU).
    Resized {
        key: u64,
        width: u32,
        height: u32,
    },
    /// Bouton gauche appuyé ou relâché sur une fenêtre d'écran (avec la
    /// position du curseur vue par l'événement).
    Press(Option<Vec2>),
    Release(Option<Vec2>),
    /// Le curseur bouge dans une fenêtre d'écran (si le système ne donne
    /// pas sa position).
    Cursor(Vec2),
    /// Le curseur entre dans une fenêtre d'écran : on va peut-être attraper
    /// un animal.
    Hover,
    /// Clic droit (ou Contrôle-clic) : le menu de l'animal sous le curseur.
    Menu,
    MenuClosed,
    /// Une surprise imposée, à son heure (essai, `MOUFLON_SURPRISE`).
    Surprise(Instant, Surprise),
    /// L'essai finit à cette heure.
    TrialEnd(Instant),
    /// Instantanés de la scène : où, et quand.
    Snapshots(std::path::PathBuf, Vec<Instant>),
    /// Le fil de l'affichage s'arrête (l'app quitte).
    Quit,
}

/// Un animal, pour le fil principal : de quoi faire les menus.
#[derive(Clone, Debug)]
pub struct PetSummary {
    pub id: u32,
    pub def: Arc<PetDef>,
    /// Personnage secondaire ou animal qui part : pas compté dans le menu.
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub extra: bool,
}

/// Ce que l'affichage dit au fil principal.
#[derive(Clone, Debug)]
pub enum Report {
    /// Le troupeau a changé (arrivée, départ, dessin) : sa génération, le
    /// nombre de demandes traitées jusque-là, et ses animaux.
    Flock {
        generation: u64,
        done: u64,
        pets: Vec<PetSummary>,
    },
    /// Les fenêtres des écrans (leur clé) qui prennent la souris (au-dessus
    /// d'un animal, ou pendant qu'on en tient un) ou la laissent passer.
    Hittest(Vec<(u64, bool)>),
    /// Le menu de cet animal (sous le curseur) est à ouvrir.
    PetMenu { id: u32, def: Arc<PetDef> },
    /// L'autorisation d'Accessibilité, telle que le relevé la voit.
    Trusted(bool),
    /// L'horloge de l'écran ne bat plus (ou les écrans ont changé) : en
    /// faire une autre.
    NeedClock,
    /// L'essai est fini (son bilan est au journal) : l'app quitte.
    TrialOver,
}

/// Ce dont l'affichage a besoin d'une horloge de l'écran : sur le fil
/// principal ([`platform::FrameClock`]) ou sur le sien ([`platform::ThreadClock`]).
pub trait Clock {
    /// Elle arrive sur le fil de l'affichage (elle s'y inscrit, s'il le faut).
    fn attach(&self) {}
    fn max_fps(&self) -> f32;
    fn is_running(&self) -> bool;
    fn frame_duration(&self) -> f64;
    fn set_running(&self, running: bool);
    fn set_rate(&self, fps: f32);
}

impl Clock for platform::FrameClock {
    fn max_fps(&self) -> f32 {
        Self::max_fps(self)
    }
    fn is_running(&self) -> bool {
        Self::is_running(self)
    }
    fn frame_duration(&self) -> f64 {
        Self::frame_duration(self)
    }
    fn set_running(&self, running: bool) {
        Self::set_running(self, running);
    }
    fn set_rate(&self, fps: f32) {
        Self::set_rate(self, fps);
    }
}

impl Clock for platform::ThreadClock {
    fn attach(&self) {
        self.start_here();
    }
    fn max_fps(&self) -> f32 {
        Self::max_fps(self)
    }
    fn is_running(&self) -> bool {
        Self::is_running(self)
    }
    fn frame_duration(&self) -> f64 {
        Self::frame_duration(self)
    }
    fn set_running(&self, running: bool) {
        Self::set_running(self, running);
    }
    fn set_rate(&self, fps: f32) {
        Self::set_rate(self, fps);
    }
}

#[derive(Debug, Default)]
struct Pointer {
    pressed: Option<Vec2>,
    dragging: Option<u32>,
    /// Dernière position vue par les événements (si le système ne donne
    /// pas la position du curseur).
    cursor: Option<Vec2>,
}

/// Ce qu'il faut à l'affichage pour commencer.
pub struct Init {
    pub live: Live,
    pub dark: bool,
    pub watcher: Arc<Watcher>,
    pub stats: Option<FrameStats>,
    pub proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
}

/// L'affichage : tout ce qui fait une image.
pub struct Display<C: Clock> {
    world: World,
    flock: Flock,
    scene: Scene,
    /// Le dessin de chaque animal (son numéro → son dessin).
    arts: HashMap<u32, Arc<PetArt>>,
    /// Le dessin de chaque définition à l'écran : les personnages secondaires
    /// prennent celui de leur espèce (ils en partagent la définition).
    def_arts: Vec<(Arc<PetDef>, Arc<PetArt>)>,
    /// Demandes traitées (voir le début du module), et depuis la dernière image.
    done: u64,
    commands: u32,
    /// Les animaux sont arrivés (`Command::Start`) : les images commencent.
    started: bool,
    /// Les écrans tels que le système les donne (avant le Dock).
    screens: Vec<Screen>,
    live: Live,
    style: Style,
    pointer: Pointer,
    /// Surprises à venir : quand, et laquelle si elle est imposée ; sinon
    /// tirée au sort.
    surprises: Vec<(Instant, Option<Surprise>)>,
    seed: u64,
    watcher: Arc<Watcher>,
    /// Dernier relevé des fenêtres des autres apps, tel quel.
    os_windows: Vec<platform::OsWindow>,
    /// Dernier cadre du Dock relevé (« Marcher sous le Dock »).
    dock_frame: Option<platform::OsRect>,
    /// Le Dock traité comme une fenêtre posée devant les autres.
    dock: Option<WindowRect>,
    dock_logged: bool,
    /// Panneaux flottants déjà signalés au journal (propriétaire, niveau).
    floating_seen: HashSet<(String, i32)>,
    frontmost: Option<i32>,
    followed: Option<u32>,
    trusted: bool,
    /// Ce que fait l'utilisateur, au dernier relevé, et quand.
    activity: Option<(Instant, platform::UserActivity)>,
    /// Personne devant la machine (essai de la CI).
    no_user: bool,
    /// Dernière note au journal d'une fenêtre qui bouge, par fenêtre.
    window_notes: HashMap<u64, Instant>,
    fast_poll_until: Instant,
    /// L'horloge de l'écran, si elle existe.
    clock: Option<C>,
    /// Dernier battement reçu, synchronisation verticale du précédent, et
    /// heure d'affichage de l'image précédente.
    clock_beat: Option<Instant>,
    last_vsync: Option<f64>,
    shown_at: Option<f64>,
    /// Une autre horloge a été demandée au fil principal ; celle d'à
    /// présent est arrivée à cette heure.
    clock_asked: bool,
    clock_since: Instant,
    last_tick: Instant,
    settle_until: Option<Instant>,
    /// Prochaine image prévue par la minuterie (`None` : tout de suite).
    next_frame: Option<Instant>,
    /// Images par seconde voulues par le dernier rythme (0 : horloge arrêtée).
    cadence: f64,
    full_rate_until: Option<Instant>,
    refresh_hz: f64,
    /// L'animal dont le menu est ouvert (il attend sous le curseur).
    menu_pet: Option<u32>,
    /// Génération du troupeau pour laquelle le fil principal est au courant.
    reported_generation: Option<u64>,
    /// Ce que le fil principal sait des fenêtres qui prennent la souris, et
    /// de quoi le calculer à chaque image sans allouer.
    reported_hits: Vec<(u64, bool)>,
    hits: Vec<(u64, bool)>,
    stats: Option<FrameStats>,
    trial_end: Option<Instant>,
    snapshots: Option<(std::path::PathBuf, Vec<Instant>)>,
    sounds_window: (Instant, u32),
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    quit: bool,
}

fn local_hour() -> f32 {
    let now = chrono::Local::now();
    now.hour() as f32 + now.minute() as f32 / 60.0
}

fn to_rect(r: platform::OsRect) -> Rect {
    Rect::new(r.x as f32, r.y as f32, r.w as f32, r.h as f32)
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

impl<C: Clock> Display<C> {
    #[must_use]
    pub fn new(init: Init) -> Self {
        let now = Instant::now();
        let mut flock = Flock::new();
        flock.set_collisions(init.live.collisions);
        flock.set_realistic(init.live.realistic);
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x5EED);
        let mut display = Self {
            world: World::default(),
            flock,
            scene: Scene::new(),
            arts: HashMap::new(),
            def_arts: Vec::new(),
            done: 0,
            commands: 0,
            started: false,
            screens: Vec::new(),
            live: init.live,
            style: Style { dark: init.dark },
            pointer: Pointer::default(),
            surprises: Vec::new(),
            seed,
            watcher: init.watcher,
            os_windows: Vec::new(),
            dock_frame: None,
            dock: None,
            dock_logged: false,
            floating_seen: HashSet::new(),
            frontmost: None,
            followed: None,
            trusted: false,
            activity: None,
            no_user: std::env::var_os("MOUFLON_ESSAI_CI").is_some(),
            window_notes: HashMap::new(),
            fast_poll_until: now,
            clock: None,
            clock_beat: None,
            last_vsync: None,
            shown_at: None,
            clock_asked: false,
            clock_since: now,
            last_tick: now,
            settle_until: None,
            next_frame: None,
            cadence: 0.0,
            full_rate_until: None,
            refresh_hz: 60.0,
            menu_pet: None,
            reported_generation: None,
            reported_hits: Vec::new(),
            hits: Vec::new(),
            stats: init.stats,
            trial_end: None,
            snapshots: None,
            sounds_window: (now, 0),
            proxy: init.proxy,
            quit: false,
        };
        display.schedule_surprise(now);
        display
    }

    /// La scène (GPU), pour le fil principal quand l'affichage y tourne.
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// Les animaux sont-ils arrivés (les images ont-elles commencé) ?
    #[must_use]
    pub fn started(&self) -> bool {
        self.started
    }

    /// Demandes traitées jusqu'ici.
    #[must_use]
    pub fn done(&self) -> u64 {
        self.done
    }

    #[must_use]
    pub fn quitting(&self) -> bool {
        self.quit
    }

    fn report(&self, report: Report) {
        if let Ok(proxy) = self.proxy.lock() {
            let _ = proxy.send_event(UserEvent::Display(report));
        }
    }

    fn note(&self, what: impl std::fmt::Display) {
        if let Some(stats) = &self.stats {
            stats.note(what);
        }
    }

    // ——— L'horloge ———

    /// Une nouvelle horloge de l'écran (l'ancienne cesse de battre).
    pub fn set_clock(&mut self, clock: Option<C>) {
        self.clock = None;
        self.shown_at = None;
        self.last_vsync = None;
        self.clock_asked = false;
        let now = Instant::now();
        self.clock_since = now;
        // Le silence se compte depuis le dernier battement entendu ; une
        // horloge toute neuve ne mène les images qu'une fois qu'elle bat.
        if self.clock_beat.is_none() {
            self.clock_beat = Some(now.checked_sub(CLOCK_SILENT).unwrap_or(now));
        }
        if let Some(clock) = &clock {
            clock.attach();
            log::info!(
                "horloge de l'écran : jusqu'à {:.0} images par seconde",
                clock.max_fps()
            );
            self.note(format_args!(
                "horloge de l'écran, {:.0} i/s au plus",
                clock.max_fps()
            ));
        }
        self.clock = clock;
    }

    /// Un battement de l'horloge vient d'arriver.
    pub fn beat_received(&mut self) {
        self.clock_beat = Some(Instant::now());
        if let Some(stats) = &self.stats {
            crate::stats::Counters::bump(&stats.counters.beats);
        }
    }

    /// L'horloge de l'écran mène-t-elle les images en ce moment ?
    #[must_use]
    pub fn clocked(&self, now: Instant) -> bool {
        self.clock.as_ref().is_some_and(Clock::is_running)
            && self
                .clock_beat
                .is_some_and(|beat| now.duration_since(beat) < CLOCK_SILENT)
    }

    /// Une image est-elle à faire sur la minuterie maintenant ?
    #[must_use]
    pub fn timer_due(&self, now: Instant) -> bool {
        self.started
            && !self.clocked(now)
            && self
                .next_frame
                .is_none_or(|next| now + Duration::from_millis(1) >= next)
    }

    /// Quand se réveiller au plus tard (minuterie, ou garde de l'horloge).
    #[must_use]
    pub fn wake_at(&self) -> Option<Instant> {
        self.next_frame
    }

    /// L'horloge tourne-t-elle (les battements mènent les images) ?
    #[must_use]
    pub fn clock_running(&self) -> bool {
        self.clock.as_ref().is_some_and(Clock::is_running)
    }

    // ——— Les demandes du fil principal ———

    pub fn command(&mut self, command: Command) {
        self.done += 1;
        self.commands += 1;
        // Ce que l'on vient de demander se voit tout de suite (pas besoin
        // d'une image de plus pour ce qui ne change rien à l'écran).
        let quiet = matches!(
            command,
            Command::Cursor(_)
                | Command::Hover
                | Command::Resized { .. }
                | Command::Surprise(..)
                | Command::TrialEnd(_)
                | Command::Snapshots(..)
        );
        if !quiet {
            self.next_frame = None;
        }
        match command {
            Command::Screens { screens, overlays } => {
                for note in self.scene.set_overlays(overlays) {
                    log::info!("{note}");
                    self.note(&note);
                }
                self.screens = screens;
                self.apply_screens();
                // Ce que le fil principal sait des fenêtres est redit à la
                // prochaine image (les neuves laissent passer la souris).
                self.reported_hits.clear();
            }
            Command::Survey(snapshot) => self.apply_snapshot(snapshot, Instant::now()),
            Command::Spawn { def, art } => self.spawn(def, art),
            Command::Start { settle } => self.start(settle),
            Command::RemoveUnderCursor => {
                if let Some(id) = self.pet_under_cursor() {
                    self.flock.remove(id);
                    if self.pointer.dragging == Some(id) {
                        self.pointer.dragging = None;
                    }
                }
            }
            Command::Goodbye(id) => {
                let input = self.input();
                if self.flock.leave(id, &self.world, &input) && self.pointer.dragging == Some(id) {
                    self.pointer = Pointer::default();
                }
            }
            Command::GoodbyeSpecies(species) => {
                let input = self.input();
                if let Some(id) = self.flock.leave_one(&species, &self.world, &input)
                    && self.pointer.dragging == Some(id)
                {
                    self.pointer = Pointer::default();
                }
            }
            Command::GoodbyeAll => {
                let input = self.input();
                self.flock.leave_all(&self.world, &input);
                self.pointer = Pointer::default();
            }
            Command::Together => {
                let input = self.input();
                let count = self.flock.together(&self.world, &input);
                log::info!("tous ensemble : {count} animal(aux)");
            }
            Command::Redraw { changes, gone } => self.redraw(&changes, &gone),
            Command::RedrawPet {
                id,
                def,
                art,
                factor,
            } => {
                let to = Arc::clone(&def);
                if self
                    .flock
                    .get_mut(id)
                    .is_some_and(|pet| pet.rescale(def, factor))
                {
                    self.remember_art(&to, &art);
                    self.fit_pet(id, art);
                    self.scene.forget_images();
                    // Le fil principal apprend sa nouvelle définition.
                    self.reported_generation = None;
                }
            }
            Command::Live(live) => self.set_live(live),
            Command::Dark(dark) => self.style.dark = dark,
            Command::Resized { key, width, height } => self.scene.resized(key, width, height),
            Command::Press(cursor) => {
                if cursor.is_some() {
                    self.pointer.cursor = cursor;
                }
                self.pointer.pressed = self.cursor();
                self.note(format_args!(
                    "souris appuyée sur n° {:?}",
                    self.pet_under_cursor()
                ));
            }
            Command::Release(cursor) => {
                if cursor.is_some() {
                    self.pointer.cursor = cursor;
                }
                self.release();
            }
            Command::Cursor(cursor) => self.pointer.cursor = Some(cursor),
            Command::Hover => self.full_rate_until = Some(Instant::now() + HOVER_FULL_RATE),
            Command::Menu => self.open_menu(),
            Command::MenuClosed => {
                // L'animal du menu repart d'où il était.
                self.flock.hold_still(None);
                self.menu_pet = None;
                self.note("menu refermé");
            }
            Command::Surprise(at, kind) => {
                self.surprises.push((at, Some(kind)));
                self.surprises.sort_by_key(|(at, _)| *at);
            }
            Command::TrialEnd(at) => self.trial_end = Some(at),
            Command::Snapshots(dir, times) => self.snapshots = Some((dir, times)),
            Command::Quit => self.quit = true,
        }
    }

    /// Les premières images des animaux arrivés, prêtes avant qu'ils ne se
    /// montrent ; puis une courte mise en route, et c'est parti.
    fn start(&mut self, settle: Duration) {
        let began = Instant::now();
        for pet in self.flock.pets() {
            if let Some(art) = self.arts.get(&pet.id()) {
                self.scene.warm_up(art, &pet.view(1.0), &self.style);
            }
        }
        let took = began.elapsed();
        log::info!("premières images prêtes en {} ms", took.as_millis());
        self.note(format_args!(
            "premières images prêtes ({:.1} ms)",
            took.as_secs_f32() * 1000.0
        ));
        let now = Instant::now();
        self.settle_until = Some(now + settle);
        self.last_tick = now;
        self.started = true;
        self.next_frame = None;
    }

    fn set_live(&mut self, live: Live) {
        let before = self.live;
        self.live = live;
        self.flock.set_collisions(live.collisions);
        self.flock.set_realistic(live.realistic);
        if before.paused != live.paused {
            // Au réveil, les animaux repartent d'où ils étaient, sans
            // rattraper le temps de la pause.
            self.last_tick = Instant::now();
            self.shown_at = None;
        }
        if before.surprises != live.surprises {
            // Celles qui n'étaient pas imposées partent ou reviennent.
            self.surprises.retain(|(_, kind)| kind.is_some());
            if live.surprises {
                self.schedule_surprise(Instant::now());
            }
        }
        if before.walk_under_dock != live.walk_under_dock {
            self.dock_logged = false;
            if !live.walk_under_dock {
                self.dock_frame = None;
            }
            self.apply_screens();
        }
        if before.frontmost_only != live.frontmost_only
            || before.walk_under_dock != live.walk_under_dock
        {
            self.refresh_windows(Instant::now());
        }
    }

    // ——— Les animaux ———

    fn spawn(&mut self, def: Arc<PetDef>, art: Arc<PetArt>) {
        self.seed = self
            .seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.remember_art(&def, &art);
        let id = self.flock.spawn(def, self.seed, &self.world, None);
        if let Some(stats) = &self.stats
            && let Some(pet) = self.flock.pets().iter().find(|p| p.id() == id)
        {
            stats.note(format_args!("arrivée de {} (n° {id})", pet.def().meta.name));
        }
        self.fit_pet(id, art);
    }

    /// Le dessin d'une définition (pour les personnages secondaires qui la
    /// partagent).
    fn remember_art(&mut self, def: &Arc<PetDef>, art: &Arc<PetArt>) {
        match self.def_arts.iter_mut().find(|(d, _)| Arc::ptr_eq(d, def)) {
            Some((_, known)) => *known = Arc::clone(art),
            None => self.def_arts.push((Arc::clone(def), Arc::clone(art))),
        }
    }

    /// Un animal prend son dessin : ce qui en dépend pour la simulation suit.
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

    /// Des animaux changent de dessin (autre taille, images compressées) ;
    /// ceux dont l'espèce n'est plus dans la bibliothèque s'en vont.
    fn redraw(&mut self, changes: &[Redraw], gone: &[Arc<PetDef>]) {
        let mut fits = Vec::new();
        let mut removed = Vec::new();
        for pet in self.flock.pets_mut() {
            if gone.iter().any(|def| Arc::ptr_eq(def, pet.def())) {
                removed.push(pet.id());
                continue;
            }
            let Some(change) = changes.iter().find(|c| Arc::ptr_eq(&c.from, pet.def())) else {
                continue;
            };
            let same = Arc::ptr_eq(&change.from, &change.to);
            if same || pet.rescale(Arc::clone(&change.to), change.factor) {
                fits.push((pet.id(), Arc::clone(&change.art)));
            } else {
                removed.push(pet.id());
            }
        }
        for change in changes {
            self.remember_art(&change.to, &change.art);
        }
        for (id, art) in fits {
            self.fit_pet(id, art);
        }
        for id in removed {
            self.flock.remove(id);
            if self.pointer.dragging == Some(id) {
                self.pointer.dragging = None;
            }
        }
        self.scene.forget_images();
        // Le fil principal apprend leurs nouvelles définitions.
        self.reported_generation = None;
    }

    /// Le menu de l'animal sous le curseur : il attend sous le menu, les
    /// autres continuent.
    fn open_menu(&mut self) {
        if self.pointer.dragging.is_some() {
            return;
        }
        let Some(id) = self.pet_under_cursor() else {
            return;
        };
        let Some(pet) = self.flock.pets().iter().find(|p| p.id() == id) else {
            return;
        };
        // Les personnages secondaires et ceux qui partent n'en ont pas.
        if pet.is_child() || pet.is_leaving() {
            return;
        }
        let def = Arc::clone(pet.def());
        // Le clic n'est pas un « coucou », ni le début d'un glisser.
        self.pointer.pressed = None;
        self.note(format_args!("menu de n° {id}"));
        self.flock.hold_still(Some(id));
        self.menu_pet = Some(id);
        self.report(Report::PetMenu { id, def });
    }

    /// Bouton relâché : on lâche l'animal tenu, ou un simple clic le touche.
    fn release(&mut self) {
        self.note("souris relâchée");
        let input = self.input();
        if let Some(id) = self.pointer.dragging.take() {
            if let Some(pet) = self.flock.get_mut(id) {
                pet.release(&self.world);
            }
        } else if self.pointer.pressed.is_some()
            && let Some(id) = self.pet_under_cursor()
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.poke(&self.world, &input);
        }
        self.pointer.pressed = None;
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

    /// Ce que fait l'utilisateur, tel que le dit le dernier relevé (quatre
    /// par seconde au moins), vieilli du temps passé depuis. Pas dans la
    /// CI : personne n'y touche à la machine.
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
                self.arts.get(&pet.id()).is_some_and(|art| {
                    // D'abord sa place au dernier pas, avec de la marge : la
                    // vue entre deux pas n'est calculée que pour ceux qui sont
                    // tout près (une foule en compte des centaines).
                    let (half, height) = art.body;
                    let at = pet.position();
                    let margin = 64.0;
                    (p.x - at.x).abs() <= half + margin
                        && p.y >= at.y - height - margin
                        && p.y <= at.y + margin
                        && hit_test(p, &pet.view(alpha), art)
                })
            })
            .map(mouflon_core::Pet::id)
    }

    // ——— Les écrans, les fenêtres, le Dock ———

    /// Les écrans du système, le Dock en moins (s'il le faut), pour le monde.
    fn apply_screens(&mut self) {
        let mut screens = self.screens.clone();
        self.dock = None;
        if self.live.walk_under_dock
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
            if !first {
                self.ask_clock();
            }
            self.refresh_windows(Instant::now());
        }
        let fastest = self.scene.screens().map(|s| s.hz).fold(0.0, f64::max);
        if fastest > 0.0 {
            self.refresh_hz = fastest.max(30.0);
        }
    }

    fn ask_clock(&mut self) {
        if !self.clock_asked {
            self.clock_asked = true;
            self.report(Report::NeedClock);
        }
    }

    /// Applique ce que le relevé a vu depuis la dernière image.
    fn apply_watch(&mut self, now: Instant) {
        if let Some(snapshot) = self.watcher.take() {
            self.apply_snapshot(snapshot, now);
        }
        // Une fenêtre qui porte un animal bouge, ou on tient un animal : on
        // relève plus souvent pour qu'il suive en douceur.
        self.watcher
            .set_fast(now < self.fast_poll_until || self.pointer.dragging.is_some());
    }

    fn apply_snapshot(&mut self, snapshot: Snapshot, now: Instant) {
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
        if self.followed != Some(snapshot.followed) || trust_changed {
            self.note(format_args!(
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
        if trust_changed {
            self.report(Report::Trusted(snapshot.trusted));
        }
        let dock = snapshot.dock.filter(|_| self.live.walk_under_dock);
        if dock != self.dock_frame {
            self.dock_frame = dock;
            self.apply_screens();
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
            if self.live.frontmost_only && !w.is_floating() && Some(w.pid) != self.frontmost {
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
        // Une fenêtre qui porte un animal bouge : on relève plus souvent
        // pour qu'il suive en douceur.
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

    // ——— Les surprises ———

    /// La prochaine surprise tirée au sort, dans un quart d'heure à
    /// quarante minutes.
    fn schedule_surprise(&mut self, now: Instant) {
        if !self.live.surprises || self.surprises.iter().any(|(_, k)| k.is_none()) {
            return;
        }
        let minutes = SURPRISE_EVERY.0 + (SURPRISE_EVERY.1 - SURPRISE_EVERY.0) * self.random_unit();
        self.surprises
            .push((now + Duration::from_secs_f32(minutes * 60.0), None));
        self.surprises.sort_by_key(|(at, _)| *at);
    }

    /// Un nombre au hasard dans [0, 1) (même suite que les graines des animaux).
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
        self.note(format_args!("surprise : {what}"));
        if imposed.is_none() {
            self.schedule_surprise(now);
        }
    }

    // ——— Les sons et les événements des animaux ———

    /// Joue les sons que les animaux viennent de faire (s'ils ont la parole),
    /// et vide la liste de leurs événements (notés au journal d'un essai).
    fn play_sounds(&mut self) {
        let audible = self.live.sounds && !self.live.paused;
        let now = Instant::now();
        if now.duration_since(self.sounds_window.0) >= SOUND_WINDOW {
            self.sounds_window = (now, 0);
        }
        for (rank, pet) in self.flock.pets_mut().iter_mut().enumerate() {
            for event in pet.take_events() {
                // Au journal : les événements des premiers arrivés seulement
                // (une foule en ferait des milliers par seconde).
                if let Some(stats) = &self.stats
                    && rank < crate::stats::DETAILED_PETS
                {
                    let mut what = describe(&event, pet);
                    if let PetEvent::Landed { .. } = event
                        && let Support::Ground { surface } = pet.support()
                    {
                        what.push_str(" sur ");
                        what.push_str(&surface_name(&self.world, &self.os_windows, surface));
                    }
                    stats.note(format_args!("n° {} {what}", pet.id()));
                }
                // Pas plus de quelques sons à la fois : une foule ne fait
                // qu'un brouhaha (et chaque son coûte).
                if let PetEvent::Sound { sound, clip } = event
                    && audible
                    && self.sounds_window.1 < SOUNDS_PER_WINDOW
                    && let Some(sound) = pet.def().sounds.get(sound)
                    && let Some(data) = sound.clips.get(clip)
                {
                    self.sounds_window.1 += 1;
                    platform::play_sound(data, sound.volume * VOLUME);
                }
            }
        }
    }

    // ——— L'image ———

    /// Une image : simulation, scène, prochain réveil. `beat` : le battement
    /// de l'horloge de l'écran qui l'a réveillée, s'il y en a un, et les
    /// battements fondus depuis l'image d'avant.
    pub fn frame(&mut self, beat: Option<platform::Beat>, coalesced: u32) {
        let now = Instant::now();
        let shown_at = beat.map(|b| b.target);
        let cadence = self.cadence;
        // Pour le journal d'un essai : ce que dit l'horloge, le retard de la
        // minuterie, les allocations de l'image.
        let diagnosing = self.stats.is_some();
        let mut beat_timing = None;
        let mut timer_late = None;
        let allocs_before = diagnosing.then(platform::alloc_counts);
        if diagnosing {
            platform::diag::frame_started();
            match beat {
                Some(t) => {
                    let interval = (t.target - t.vsync).max(1e-4);
                    let missed = self.last_vsync.map_or(0, |last| {
                        (((t.vsync - last) / interval).round() as i64 - 1).max(0) as u32
                    });
                    beat_timing = Some(crate::stats::BeatTiming {
                        latency: ((t.called - t.vsync) * 1000.0) as f32,
                        queue: ((platform::diag::media_time() - t.called) * 1000.0) as f32,
                        missed,
                        coalesced,
                    });
                }
                None => {
                    timer_late = self.next_frame.map(|planned| {
                        now.saturating_duration_since(planned).as_secs_f32() * 1000.0
                    });
                }
            }
        }
        if let Some(t) = beat {
            self.last_vsync = Some(t.vsync);
        }
        // Calé sur l'écran, on avance exactement du temps qui sépare deux
        // images affichées : les petits retards ne font plus varier la
        // vitesse des animaux d'une image à l'autre.
        let elapsed = match (shown_at, self.shown_at) {
            (Some(at), Some(before)) if at > before => (at - before) as f32,
            _ => now.duration_since(self.last_tick).as_secs_f32(),
        };
        self.shown_at = shown_at;
        self.last_tick = now;
        let mut phases = crate::stats::Phases::default();

        // Une horloge qui tourne mais ne bat plus (écran changé, réveil…) :
        // le fil principal en refait une.
        if self.clock.as_ref().is_some_and(Clock::is_running)
            && now.duration_since(self.clock_since) > CLOCK_DEAD
            && self
                .clock_beat
                .is_some_and(|beat| now.duration_since(beat) > CLOCK_DEAD)
        {
            self.ask_clock();
        }
        self.apply_watch(now);
        let mut mark = Instant::now();
        phases.watch = mark - now;

        // Mise en route du lancement : la scène est là mais vide, les
        // animaux attendent.
        let settling = self.settle_until.is_some_and(|until| now < until);
        if !settling && self.settle_until.take().is_some() {
            log::info!("mise en route terminée : les animaux arrivent");
            self.note("mise en route terminée");
        }

        let input = self.input();
        let cursor = input.cursor;
        if !settling && !self.live.paused {
            self.surprise(now, &input);
        }

        // Glisser : on n'attrape l'animal qu'après un petit déplacement, un
        // simple clic reste un « coucou ». Il est pris là où l'on a cliqué,
        // et ce point reste sous le curseur.
        if !settling
            && let (Some(start), Some(c), None) =
                (self.pointer.pressed, cursor, self.pointer.dragging)
            && (c - start).length() > 3.0
            && let Some(id) = self.pet_at(start, 1.0)
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.grab(start, &self.world);
            self.pointer.dragging = Some(id);
            self.note(format_args!(
                "n° {id} attrapé en {:.0},{:.0}",
                start.x, start.y
            ));
        }
        if let (Some(id), Some(c)) = (self.pointer.dragging, cursor)
            && let Some(pet) = self.flock.get_mut(id)
        {
            pet.drag_to(c);
        }

        let alpha = if self.live.paused || settling {
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
        phases.windows = lap(&mut mark);

        // La scène : chaque soucoupe, puis chaque animal, à sa place ; une
        // seule transaction pour tout ce qui change.
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
        // pendant qu'on en tient un), et la laisse passer ailleurs : le fil
        // principal le fait, quand cela change.
        let mark = Instant::now();
        self.update_hits(cursor, alpha, now);
        phases.hittest += mark.elapsed();
        if !settling {
            self.take_snapshots(now);
        }

        self.schedule(now, settling);
        self.record(
            now,
            alpha,
            &phases,
            beat_timing,
            timer_late,
            cadence,
            allocs_before,
        );
    }

    /// Les fenêtres d'écran qui prennent la souris : celle où se trouve le
    /// curseur, s'il est sur un animal ; toutes, pendant qu'on en tient un.
    fn update_hits(&mut self, cursor: Option<Vec2>, alpha: f32, now: Instant) {
        let dragging = self.pointer.dragging.is_some();
        let mut hits = std::mem::take(&mut self.hits);
        hits.clear();
        hits.extend(self.scene.screens().map(|screen| {
            let over =
                cursor.is_some_and(|c| screen.rect.contains(c) && self.pet_at(c, alpha).is_some());
            (screen.key, dragging || over)
        }));
        if hits != self.reported_hits {
            let newly = hits.iter().any(|&(key, on)| {
                on && !self
                    .reported_hits
                    .iter()
                    .any(|&(k, before)| k == key && before)
            });
            // Le curseur arrive sur un animal : on va peut-être l'attraper.
            if newly && !dragging {
                self.full_rate_until = Some(now + HOVER_FULL_RATE);
            }
            self.reported_hits.clone_from(&hits);
            self.report(Report::Hittest(hits.clone()));
        }
        self.hits = hits;
    }

    /// Les nouveaux venus (personnages secondaires des scènes desktopPet
    /// compris) prennent le dessin de leur espèce ; ceux qui sont partis
    /// s'oublient ; le fil principal apprend ce qui a changé.
    fn sync_pets(&mut self) {
        let generation = self.flock.generation();
        if self.reported_generation == Some(generation) {
            return;
        }
        self.reported_generation = Some(generation);
        let pets = self.flock.pets();
        let alive: HashSet<u32> = pets.iter().map(mouflon_core::Pet::id).collect();
        self.arts.retain(|id, _| alive.contains(id));
        self.def_arts
            .retain(|(def, _)| pets.iter().any(|p| Arc::ptr_eq(p.def(), def)));
        // Un personnage secondaire prend le dessin de son espèce.
        let missing: Vec<(u32, Arc<PetArt>)> = pets
            .iter()
            .filter(|p| !self.arts.contains_key(&p.id()))
            .filter_map(|p| {
                let (_, art) = self
                    .def_arts
                    .iter()
                    .find(|(d, _)| Arc::ptr_eq(d, p.def()))?;
                Some((p.id(), Arc::clone(art)))
            })
            .collect();
        let summary = pets
            .iter()
            .map(|p| PetSummary {
                id: p.id(),
                def: Arc::clone(p.def()),
                extra: p.is_child() || p.is_leaving(),
            })
            .collect();
        for (id, art) in missing {
            self.fit_pet(id, art);
        }
        self.report(Report::Flock {
            generation,
            done: self.done,
            pets: summary,
        });
    }

    /// Les instantanés de la scène prévus, à leur heure.
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
        let dir = dir.clone();
        let started = self.stats.as_ref().map_or(now, FrameStats::begun);
        let name = format!(
            "instantane-{:05.1}s",
            now.duration_since(started).as_secs_f32()
        );
        let written = self.scene.snapshot(&dir, &name, 1.0);
        let took = now.elapsed();
        log::info!(
            "instantané de la scène : {} image(s) en {} ms",
            written.len(),
            took.as_millis()
        );
        self.note(format_args!(
            "instantané de la scène ({name}, {} image(s), {:.1} ms)",
            written.len(),
            took.as_secs_f32() * 1000.0
        ));
    }

    /// Prochain réveil : au battement de l'horloge de l'écran, à la cadence
    /// voulue, ou par la minuterie quand l'horloge manque ou se tait.
    fn schedule(&mut self, now: Instant, settling: bool) {
        let moving = self.clock.as_ref().map_or(self.refresh_hz, |clock| {
            self.refresh_hz.max(f64::from(clock.max_fps()))
        });
        // Le curseur sur un animal, ou le bouton appuyé : on va sans doute
        // l'attraper. L'écran passe à pleine cadence d'avance.
        let anticipating =
            self.pointer.pressed.is_some() || self.full_rate_until.is_some_and(|until| now < until);
        let fps = if self.live.paused {
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
            let wake = Duration::from_millis(if self.live.paused { 250 } else { 500 });
            self.next_frame = Some(now + wake);
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
        if self.stats.is_some() {
            // L'horloge qui bat amène l'image suivante dans un intervalle ;
            // sinon, c'est la minuterie.
            let clocked = self.clock.is_some() && wake == CLOCK_SILENT;
            platform::diag::expect_frame(if clocked { interval } else { wake });
        }
    }

    /// Le journal d'un essai : l'image, ses parties, les premiers animaux.
    #[expect(clippy::too_many_arguments, reason = "tout ce que l'image a mesuré")]
    fn record(
        &mut self,
        now: Instant,
        alpha: f32,
        phases: &crate::stats::Phases,
        beat: Option<crate::stats::BeatTiming>,
        timer_late: Option<f32>,
        cadence: f64,
        allocs_before: Option<platform::AllocCounts>,
    ) {
        if self.stats.is_none() {
            return;
        }
        let textures = self.scene.image_usage();
        let (decoded, evicted) = self.scene.take_image_counts();
        let refresh = self
            .clock
            .as_ref()
            .map_or(0.0, |clock| clock.frame_duration() * 1000.0);
        let mut arts: Vec<&Arc<PetArt>> = Vec::new();
        for art in self.arts.values() {
            if !arts.iter().any(|a| Arc::ptr_eq(a, art)) {
                arts.push(art);
            }
        }
        let image_bytes = arts.iter().map(|a| a.image_bytes()).sum();
        let events = std::mem::take(&mut self.commands);
        let Some(stats) = &mut self.stats else {
            return;
        };
        stats.set_textures(textures);
        stats
            .counters
            .images_decoded
            .fetch_add(decoded, std::sync::atomic::Ordering::Relaxed);
        stats
            .counters
            .images_evicted
            .fetch_add(evicted, std::sync::atomic::Ordering::Relaxed);
        stats.set_images(image_bytes);
        let pets = self
            .flock
            .pets()
            .iter()
            .take(crate::stats::DETAILED_PETS)
            .map(|p| {
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
                        Support::Ground { .. } => 's',
                        Support::Air => 'a',
                        Support::Held => 't',
                    },
                    roll: view.pose.roll,
                }
            });
        let allocs = allocs_before.map_or((0, 0), |before| {
            let after = platform::alloc_counts();
            (
                after.main_allocs.saturating_sub(before.main_allocs),
                after.main_bytes.saturating_sub(before.main_bytes),
            )
        });
        let record = crate::stats::FrameRecord {
            started: now,
            beat,
            timer_late,
            cadence,
            refresh,
            phases: *phases,
            allocs,
            events,
        };
        stats.record(&record, pets);
        if self.trial_end.is_some_and(|end| now >= end) {
            self.trial_end = None;
            let bilan = stats.finish();
            log::info!("essai terminé : {bilan}");
            self.report(Report::TrialOver);
        }
    }

    /// Avant de quitter : l'horloge s'arrête (sur son fil), et le journal
    /// d'un essai interrompu reçoit son bilan.
    pub fn shutdown(&mut self) {
        self.clock = None;
        if let Some(stats) = &mut self.stats {
            stats.finish();
        }
    }
}

/// Cadence pour des animaux qui bougent : la plus lente qui divise celle de
/// l'écran (`full`) sans descendre sous celle qu'il leur faut (`needed`).
/// Un diviseur : l'écran montre chaque image le même nombre de fois, et le
/// mouvement reste régulier.
fn moving_rate(needed: f32, full: f64) -> f64 {
    let needed = f64::from(needed);
    if !needed.is_finite() || needed >= full {
        return full;
    }
    let divisor = (full / needed.max(1.0)).floor().max(1.0);
    full / divisor
}

/// Temps écoulé depuis `mark`, qui passe à maintenant.
fn lap(mark: &mut Instant) -> Duration {
    let now = Instant::now();
    let lap = now - *mark;
    *mark = now;
    lap
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

// ——— Le fil de l'affichage ———

/// L'affichage sur son propre fil : de quoi lui parler.
#[derive(Debug)]
pub struct DisplayThread {
    commands: Sender<Command>,
    clocks: Sender<Option<platform::ThreadClock>>,
    wake: Arc<platform::LoopWaker>,
    beats: Arc<platform::Beats>,
    /// Demandes envoyées (voir le début du module).
    sent: u64,
    thread: Option<JoinHandle<()>>,
}

impl DisplayThread {
    /// Lance le fil de l'affichage, qui fait son [`Display`] avec `init`.
    /// `None` s'il n'a pas pu l'être.
    pub fn spawn(init: Init) -> Option<Self> {
        let (commands, command_receiver) = std::sync::mpsc::channel();
        let (clocks, clock_receiver) = std::sync::mpsc::channel();
        let (ready, waiting) = std::sync::mpsc::channel();
        let beats = Arc::new(platform::Beats::default());
        let thread_beats = Arc::clone(&beats);
        let thread = std::thread::Builder::new()
            .name("mouflon-affichage".into())
            .spawn(move || {
                platform::raise_thread_priority();
                let Some(wake) = platform::LoopWaker::current() else {
                    let _ = ready.send(None);
                    return;
                };
                let wake = Arc::new(wake);
                if ready.send(Some(Arc::clone(&wake))).is_err() {
                    return;
                }
                let display = Display::new(init);
                run(display, &command_receiver, &clock_receiver, &thread_beats);
            });
        let thread = match thread {
            Ok(thread) => thread,
            Err(e) => {
                log::error!("impossible de lancer le fil de l'affichage : {e}");
                return None;
            }
        };
        let Some(wake) = waiting.recv().ok().flatten() else {
            log::error!("le fil de l'affichage n'a pas de boucle d'événements");
            return None;
        };
        Some(Self {
            commands,
            clocks,
            wake,
            beats,
            sent: 0,
            thread: Some(thread),
        })
    }

    /// Une demande, que l'affichage traite tout de suite.
    pub fn send(&mut self, command: Command) {
        if self.commands.send(command).is_ok() {
            self.sent += 1;
            self.wake.wake();
        }
    }

    /// Une demande qui attend le prochain réveil de l'affichage.
    pub fn send_quiet(&mut self, command: Command) {
        if self.commands.send(command).is_ok() {
            self.sent += 1;
        }
    }

    /// Demandes envoyées jusqu'ici.
    #[must_use]
    pub fn sent(&self) -> u64 {
        self.sent
    }

    /// Une horloge de l'écran pour ce fil (à créer sur le fil principal,
    /// seul à pouvoir demander l'écran).
    #[must_use]
    pub fn new_clock(&self) -> Option<platform::ThreadClock> {
        platform::ThreadClock::new(Arc::clone(&self.beats), Arc::clone(&self.wake))
    }

    /// Donne une horloge au fil (l'ancienne s'arrête).
    pub fn set_clock(&self, clock: Option<platform::ThreadClock>) {
        if self.clocks.send(clock).is_ok() {
            self.wake.wake();
        }
    }

    /// Arrête le fil et attend qu'il ait fini (journal d'un essai compris).
    pub fn stop(&mut self) {
        self.send(Command::Quit);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// La boucle du fil de l'affichage : les demandes, puis une image à chaque
/// battement (ou à l'heure de la minuterie), et l'attente.
fn run(
    mut display: Display<platform::ThreadClock>,
    commands: &Receiver<Command>,
    clocks: &Receiver<Option<platform::ThreadClock>>,
    beats: &platform::Beats,
) {
    if display.stats.is_some() {
        // Les allocations comptées et la pile relevée quand une image
        // tarde : celles de ce fil, qui fait les images.
        platform::set_alloc_counting(true);
        platform::diag::register_frame_thread();
    }
    loop {
        let quit = platform::with_release_pool(|| {
            while let Ok(clock) = clocks.try_recv() {
                display.set_clock(clock);
            }
            while let Ok(command) = commands.try_recv() {
                display.command(command);
            }
            if display.quitting() {
                return true;
            }
            if let Some(beat) = beats.take() {
                display.beat_received();
                if display.started() && display.clock_running() {
                    display.frame(Some(beat), beats.take_coalesced());
                }
            } else if display.timer_due(Instant::now()) {
                display.frame(None, 0);
            }
            let now = Instant::now();
            let wait = display.wake_at().map_or(Duration::from_millis(250), |at| {
                at.saturating_duration_since(now)
            });
            platform::run_loop_for(wait.min(Duration::from_millis(250)));
            false
        });
        if quit {
            break;
        }
    }
    display.shutdown();
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
