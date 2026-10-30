//! Relevé des fenêtres des autres apps, du cadre du Dock et du dossier des
//! animaux, sur un fil à part : ces questions au système prennent parfois
//! plusieurs millisecondes (le Dock peut mettre bien plus à répondre), de quoi
//! faire sauter une image si on les posait pendant l'animation.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
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
/// Essais : un relevé plus long est noté au journal ; un résumé toutes les…
const SLOW_POLL: Duration = Duration::from_millis(15);
const SUMMARY_EVERY: Duration = Duration::from_secs(10);

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
    /// dossier des animaux à surveiller ; `journal` : celui d'un essai (et
    /// son début), où noter la durée des relevés.
    pub fn start(
        dock: bool,
        pets: Option<PathBuf>,
        journal: Option<(Sender<String>, Instant)>,
    ) -> Self {
        let shared = Arc::new(Shared::default());
        shared.dock.store(dock, Ordering::Relaxed);
        let worker = Arc::clone(&shared);
        let spawned = std::thread::Builder::new()
            .name("mouflon-releve".into())
            .spawn(move || watch(&worker, pets.as_deref(), journal.map(Timing::new)));
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

/// Durée des relevés, pour le journal d'un essai.
struct Timing {
    journal: Sender<String>,
    begun: Instant,
    since: Instant,
    polls: u32,
    total: Duration,
    worst: Duration,
    dock_worst: Duration,
    folder_worst: Duration,
    windows: usize,
}

impl Timing {
    fn new((journal, begun): (Sender<String>, Instant)) -> Self {
        Self {
            journal,
            begun,
            since: Instant::now(),
            polls: 0,
            total: Duration::ZERO,
            worst: Duration::ZERO,
            dock_worst: Duration::ZERO,
            folder_worst: Duration::ZERO,
            windows: 0,
        }
    }

    fn at(&self) -> f64 {
        self.begun.elapsed().as_secs_f64()
    }

    /// Un relevé des fenêtres (et, de temps en temps, du Dock et du dossier).
    fn poll(&mut self, windows: Duration, count: usize, dock: Duration, folder: Duration) {
        self.polls += 1;
        self.total += windows;
        self.worst = self.worst.max(windows);
        self.dock_worst = self.dock_worst.max(dock);
        self.folder_worst = self.folder_worst.max(folder);
        self.windows = count;
        let ms = |d: Duration| d.as_secs_f64() * 1000.0;
        if windows + dock + folder > SLOW_POLL {
            let _ = self.journal.send(format!(
                "E {:.4} relevé lent : fenêtres {:.1} ms ({count}), Dock {:.1} ms, dossier {:.1} ms",
                self.at(),
                ms(windows),
                ms(dock),
                ms(folder)
            ));
        }
        if self.since.elapsed() >= SUMMARY_EVERY {
            let _ = self.journal.send(format!(
                "SW {:.4} releves {} moyen_ms {:.2} max_ms {:.2} dock_max_ms {:.2} dossier_max_ms {:.2} fenetres {}",
                self.at(),
                self.polls,
                ms(self.total) / f64::from(self.polls.max(1)),
                ms(self.worst),
                ms(self.dock_worst),
                ms(self.folder_worst),
                self.windows,
            ));
            *self = Self::new((self.journal.clone(), self.begun));
        }
    }
}

fn watch(shared: &Shared, pets: Option<&std::path::Path>, mut timing: Option<Timing>) {
    let mut dock = None;
    let mut slow_read: Option<Instant> = None;
    while !shared.stop.load(Ordering::Relaxed) {
        let started = Instant::now();
        let windows = platform::windows();
        let windows_took = started.elapsed();
        let mut dock_took = Duration::ZERO;
        let mut folder_took = Duration::ZERO;
        let with_dock = shared.dock.load(Ordering::Relaxed);
        if slow_read.is_none_or(|t| t.elapsed() >= SLOW) {
            let asked = Instant::now();
            dock = if with_dock {
                platform::dock_frame()
            } else {
                None
            };
            dock_took = asked.elapsed();
            if let Some(dir) = pets
                && let Ok(mut latest) = shared.pets.lock()
            {
                let asked = Instant::now();
                *latest = Some(stamp(dir));
                folder_took = asked.elapsed();
            }
            slow_read = Some(Instant::now());
        } else if !with_dock {
            dock = None;
        }
        if let Some(timing) = timing.as_mut() {
            timing.poll(windows_took, windows.len(), dock_took, folder_took);
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
