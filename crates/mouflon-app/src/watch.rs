//! Relevé des fenêtres des autres apps, du cadre du Dock et du dossier des
//! animaux, sur un fil à part : ces questions au système prennent parfois
//! plusieurs millisecondes (le Dock peut mettre bien plus à répondre), de quoi
//! faire sauter une image si on les posait pendant l'animation.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use mouflon_platform as platform;

use crate::library::{Stamp, stamp};

/// Relevé des fenêtres au calme, et quand une fenêtre qui porte un animal
/// bouge (ou qu'on tient un animal).
const CALM: Duration = Duration::from_millis(250);
const FAST: Duration = Duration::from_millis(33);
/// Le Dock et le dossier des animaux changent rarement : on les relit de
/// temps en temps.
const SLOW: Duration = Duration::from_secs(2);

/// Un relevé : fenêtres visibles des autres apps, cadre du Dock.
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub windows: Vec<platform::OsWindow>,
    pub dock: Option<platform::OsRect>,
}

#[derive(Debug, Default)]
struct Shared {
    latest: Mutex<Option<Snapshot>>,
    /// Dernier état relevé du dossier des animaux.
    pets: Mutex<Option<Stamp>>,
    /// Relever souvent.
    fast: AtomicBool,
    /// Lire aussi le cadre du Dock (« Marcher sous le Dock »).
    dock: AtomicBool,
    /// Demande de relevé immédiat.
    now: Mutex<bool>,
    wake: Condvar,
    stop: AtomicBool,
}

#[derive(Debug)]
pub struct Watcher {
    shared: Arc<Shared>,
}

impl Watcher {
    /// Lance le fil de relevé ; il s'arrête avec le `Watcher`. `pets` : le
    /// dossier des animaux à surveiller.
    pub fn start(dock: bool, pets: Option<PathBuf>) -> Self {
        let shared = Arc::new(Shared::default());
        shared.dock.store(dock, Ordering::Relaxed);
        let worker = Arc::clone(&shared);
        let spawned = std::thread::Builder::new()
            .name("mouflon-releve".into())
            .spawn(move || watch(&worker, pets.as_deref()));
        if let Err(e) = spawned {
            log::error!("impossible de lancer le relevé des fenêtres : {e}");
        }
        Self { shared }
    }

    /// Le dernier relevé, s'il est nouveau.
    pub fn take(&self) -> Option<Snapshot> {
        self.shared.latest.lock().ok()?.take()
    }

    /// Attend le premier relevé, au plus `timeout`.
    pub fn wait_first(&self, timeout: Duration) -> Option<Snapshot> {
        let started = Instant::now();
        loop {
            if let Some(snapshot) = self.take() {
                return Some(snapshot);
            }
            if started.elapsed() >= timeout {
                return None;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// Le dernier état relevé du dossier des animaux, s'il est nouveau.
    pub fn take_pets(&self) -> Option<Stamp> {
        self.shared.pets.lock().ok()?.take()
    }

    pub fn set_fast(&self, fast: bool) {
        self.shared.fast.store(fast, Ordering::Relaxed);
    }

    pub fn set_dock(&self, dock: bool) {
        self.shared.dock.store(dock, Ordering::Relaxed);
        self.refresh_now();
    }

    /// Relève tout (Dock compris) sans attendre le prochain tour.
    pub fn refresh_now(&self) {
        if let Ok(mut now) = self.shared.now.lock() {
            *now = true;
            self.shared.wake.notify_one();
        }
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.refresh_now();
    }
}

fn watch(shared: &Shared, pets: Option<&std::path::Path>) {
    let mut dock = None;
    let mut slow_read: Option<Instant> = None;
    while !shared.stop.load(Ordering::Relaxed) {
        let windows = platform::windows();
        let with_dock = shared.dock.load(Ordering::Relaxed);
        if slow_read.is_none_or(|t| t.elapsed() >= SLOW) {
            dock = if with_dock {
                platform::dock_frame()
            } else {
                None
            };
            if let Some(dir) = pets
                && let Ok(mut latest) = shared.pets.lock()
            {
                *latest = Some(stamp(dir));
            }
            slow_read = Some(Instant::now());
        } else if !with_dock {
            dock = None;
        }
        if let Ok(mut latest) = shared.latest.lock() {
            *latest = Some(Snapshot { windows, dock });
        }
        let pause = if shared.fast.load(Ordering::Relaxed) {
            FAST
        } else {
            CALM
        };
        let Ok(guard) = shared.now.lock() else {
            return;
        };
        let Ok((mut guard, _)) = shared.wake.wait_timeout_while(guard, pause, |now| !*now) else {
            return;
        };
        if *guard {
            // Une demande de relevé immédiat relit aussi le Dock.
            slow_read = None;
        }
        *guard = false;
    }
}
