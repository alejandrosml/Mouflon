//! Essais de fluidité : `MOUFLON_ESSAI=300` lance Mouflon pour 300 secondes
//! et écrit un journal complet (`MOUFLON_JOURNAL`, par défaut
//! `mouflon-essai.log` dans le dossier courant) : chaque image (temps, parties,
//! état de chaque animal), chaque événement (collisions, atterrissages, sons,
//! fenêtres, menu…), un résumé toutes les dix secondes et un bilan à la fin.
//!
//! Options : `MOUFLON_ESSAI_ANIMAUX=lainou,lainou,pelote` (animaux du
//! lancement, plusieurs fois le même si on veut) et
//! `MOUFLON_ESSAI_COLLISIONS=1` (mode collisions pendant l'essai).

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Durée d'essai demandée (variable d'environnement `MOUFLON_ESSAI`, en secondes).
#[must_use]
pub fn trial() -> Option<Duration> {
    let seconds: f64 = std::env::var("MOUFLON_ESSAI").ok()?.trim().parse().ok()?;
    (seconds.is_finite() && seconds > 0.0).then(|| Duration::from_secs_f64(seconds))
}

/// Animaux du lancement demandés pour l'essai (identifiants).
#[must_use]
pub fn trial_pets() -> Vec<String> {
    std::env::var("MOUFLON_ESSAI_ANIMAUX")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect()
}

/// L'essai met-il le mode collisions ?
#[must_use]
pub fn trial_collisions() -> bool {
    std::env::var("MOUFLON_ESSAI_COLLISIONS").is_ok_and(|v| v.trim() == "1")
}

/// Temps passé dans chaque partie d'une image.
#[derive(Clone, Copy, Debug, Default)]
pub struct Phases {
    /// Relevés appliqués (écrans, fenêtres, Dock, dossier des animaux).
    pub watch: Duration,
    /// Simulation des animaux (et prise à la souris).
    pub sim: Duration,
    /// Sons lancés (et événements des animaux).
    pub sounds: Duration,
    /// Fenêtres ouvertes ou fermées.
    pub windows: Duration,
    /// Fenêtres déplacées.
    pub moves: Duration,
    /// Dessin : listes d'affichage et travail de Vello.
    pub paint: Duration,
    /// Attente d'une image d'écran libre.
    pub acquire: Duration,
    /// Recopie dans la fenêtre et envoi à l'écran.
    pub present: Duration,
    /// Fenêtres redessinées.
    pub renders: u32,
}

impl Phases {
    fn add(&mut self, other: &Self) {
        self.watch += other.watch;
        self.sim += other.sim;
        self.sounds += other.sounds;
        self.windows += other.windows;
        self.moves += other.moves;
        self.paint += other.paint;
        self.acquire += other.acquire;
        self.present += other.present;
        self.renders += other.renders;
    }

    /// Forme courte pour le journal : millisecondes de chaque partie.
    fn compact(&self) -> String {
        let ms = |d: Duration| d.as_secs_f32() * 1000.0;
        format!(
            "rel {:.2} sim {:.2} son {:.2} fen {:.2} dep {:.2} des {:.2} att {:.2} env {:.2} r {}",
            ms(self.watch),
            ms(self.sim),
            ms(self.sounds),
            ms(self.windows),
            ms(self.moves),
            ms(self.paint),
            ms(self.acquire),
            ms(self.present),
            self.renders,
        )
    }
}

impl std::fmt::Display for Phases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ms = |d: Duration| d.as_secs_f32() * 1000.0;
        write!(
            f,
            "relevés {:.1}, simulation {:.1}, sons {:.1}, fenêtres {:.1}, déplacements {:.1}, \
             dessin {:.1}, attente de l'écran {:.1}, envoi {:.1} ({} rendu(s))",
            ms(self.watch),
            ms(self.sim),
            ms(self.sounds),
            ms(self.windows),
            ms(self.moves),
            ms(self.paint),
            ms(self.acquire),
            ms(self.present),
            self.renders,
        )
    }
}

/// Un animal à une image : ce que le journal en garde.
#[derive(Clone, Debug)]
pub struct PetLine<'a> {
    pub id: u32,
    pub name: &'a str,
    pub behavior: &'a str,
    pub frame: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    /// `s` au sol, `a` en l'air, `t` tenu.
    pub support: char,
    pub roll: f32,
}

/// Une image lente : quand (secondes depuis le début), combien (ms), et où.
#[derive(Clone, Copy, Debug)]
struct Slow {
    at: f32,
    work: f32,
    phases: Phases,
}

/// Images préparées en plus de ce temps : notées en détail.
const SLOW_MS: f32 = 12.0;
/// Images lentes gardées pour le bilan (les pires).
const SLOW_KEPT: usize = 12;
/// Un résumé dans le journal toutes les…
const SUMMARY_EVERY: Duration = Duration::from_secs(10);

/// Par comportement : temps passé, images dessinées, images de la séquence.
#[derive(Clone, Copy, Debug, Default)]
struct BehaviorStats {
    time: f32,
    frames: u32,
    changes: u32,
    worst_gap: f32,
}

/// Écarts entre les images, temps passé à les préparer, et journal.
#[derive(Debug)]
pub struct FrameStats {
    begun: Instant,
    /// Écart avec l'image précédente, en millisecondes.
    gaps: Vec<f32>,
    /// Temps de préparation de chaque image, en millisecondes.
    work: Vec<f32>,
    /// Images et plus grand écart, seconde par seconde.
    seconds: Vec<(u32, f32)>,
    slow: Vec<Slow>,
    /// Images données par l'horloge de l'écran (les autres : la minuterie).
    beats: usize,
    last: Option<Instant>,
    /// Comportement et image de chaque animal à la dernière image.
    seen: HashMap<u32, (String, Option<u32>)>,
    behaviors: BTreeMap<String, BehaviorStats>,
    journal: Option<Journal>,
    /// Depuis le dernier résumé.
    period: Period,
}

#[derive(Debug, Default)]
struct Period {
    started: Option<Instant>,
    frames: u32,
    worst_gap: f32,
    worst_work: f32,
    late: u32,
    phases: Phases,
}

impl FrameStats {
    #[must_use]
    pub fn new(begun: Instant) -> Self {
        Self {
            begun,
            gaps: Vec::new(),
            work: Vec::new(),
            seconds: Vec::new(),
            slow: Vec::new(),
            beats: 0,
            last: None,
            seen: HashMap::new(),
            behaviors: BTreeMap::new(),
            journal: None,
            period: Period::default(),
        }
    }

    /// Avec un journal complet : le fichier `MOUFLON_JOURNAL`, ou
    /// `mouflon-essai.log` dans le dossier courant.
    #[must_use]
    pub fn with_journal(self) -> Self {
        let path = std::env::var_os("MOUFLON_JOURNAL")
            .map_or_else(|| PathBuf::from("mouflon-essai.log"), PathBuf::from);
        self.with_journal_at(path)
    }

    #[must_use]
    pub fn with_journal_at(mut self, path: PathBuf) -> Self {
        self.journal = Journal::open(path);
        self
    }

    /// Secondes depuis le début de l'essai.
    fn at(&self) -> f32 {
        self.begun.elapsed().as_secs_f32()
    }

    /// Note ce qui arrive pendant l'essai, avec l'heure depuis son début : on
    /// retrouve ainsi ce qui accompagne une image lente.
    pub fn note(&self, what: impl std::fmt::Display) {
        let line = format!("E {:.4} {what}", self.at());
        match &self.journal {
            Some(journal) => journal.write(line),
            None => log::info!("{line}"),
        }
    }

    /// Une ligne telle quelle dans le journal (en-tête, état du système…).
    pub fn raw(&self, line: impl Into<String>) {
        if let Some(journal) = &self.journal {
            journal.write(line.into());
        }
    }

    /// Une image commencée à `started` (et finie maintenant), et l'état de
    /// chaque animal à ce moment.
    pub fn record<'a>(
        &mut self,
        started: Instant,
        beat: bool,
        phases: Phases,
        pets: impl Iterator<Item = PetLine<'a>>,
    ) {
        let gap = self
            .last
            .map(|last| started.duration_since(last).as_secs_f32() * 1000.0);
        self.last = Some(started);
        let work = started.elapsed().as_secs_f32() * 1000.0;
        self.work.push(work);
        if beat {
            self.beats += 1;
        }
        let at = started.duration_since(self.begun).as_secs_f32();
        let second = at as usize;
        if self.seconds.len() <= second {
            self.seconds.resize(second + 1, (0, 0.0));
        }
        self.seconds[second].0 += 1;
        if let Some(gap) = gap {
            self.gaps.push(gap);
            self.seconds[second].1 = self.seconds[second].1.max(gap);
        }
        if work >= SLOW_MS {
            self.slow.push(Slow { at, work, phases });
            self.slow.sort_by(|a, b| b.work.total_cmp(&a.work));
            self.slow.truncate(SLOW_KEPT);
        }

        // L'image dans le journal : temps, parties, puis chaque animal.
        let mut line = format!(
            "F {at:.4} ecart {:.2} prep {work:.2} {} {}",
            gap.unwrap_or(0.0),
            if beat { "clk" } else { "tmr" },
            phases.compact(),
        );
        let mut changes = Vec::new();
        for pet in pets {
            let _ = write!(
                line,
                " | {} {} {} {:.1},{:.1} {:.0},{:.0} {} {:.2}",
                pet.id,
                pet.behavior,
                pet.frame.map_or(-1, i64::from),
                pet.x,
                pet.y,
                pet.vx,
                pet.vy,
                pet.support,
                pet.roll,
            );
            let stats = self.behaviors.entry(pet.behavior.to_owned()).or_default();
            stats.frames += 1;
            stats.time += gap.unwrap_or(0.0) / 1000.0;
            stats.worst_gap = stats.worst_gap.max(gap.unwrap_or(0.0));
            match self.seen.get(&pet.id) {
                Some((behavior, frame)) if behavior == pet.behavior => {
                    if *frame != pet.frame {
                        stats.changes += 1;
                    }
                }
                _ => changes.push(format!("{} n° {} : {}", pet.name, pet.id, pet.behavior)),
            }
            self.seen
                .insert(pet.id, (pet.behavior.to_owned(), pet.frame));
        }
        for change in changes {
            self.note(change);
        }
        if let Some(journal) = &self.journal {
            journal.write(line);
        }

        // Résumé de la période.
        let period = &mut self.period;
        let period_start = *period.started.get_or_insert(started);
        period.frames += 1;
        period.worst_gap = period.worst_gap.max(gap.unwrap_or(0.0));
        period.worst_work = period.worst_work.max(work);
        if work >= SLOW_MS {
            period.late += 1;
        }
        period.phases.add(&phases);
        if started.duration_since(period_start) >= SUMMARY_EVERY {
            let frames = period.frames.max(1);
            let mean = |d: Duration| d.as_secs_f32() * 1000.0 / frames as f32;
            let line = format!(
                "S {at:.4} images {} plus_grand_ecart {:.1} plus_longue_prep {:.1} lentes {} \
                 moyennes rel {:.3} sim {:.3} son {:.3} fen {:.3} dep {:.3} des {:.3} att {:.3} \
                 env {:.3} rendus {} {}",
                period.frames,
                period.worst_gap,
                period.worst_work,
                period.late,
                mean(period.phases.watch),
                mean(period.phases.sim),
                mean(period.phases.sounds),
                mean(period.phases.windows),
                mean(period.phases.moves),
                mean(period.phases.paint),
                mean(period.phases.acquire),
                mean(period.phases.present),
                period.phases.renders,
                process_usage(),
            );
            *period = Period::default();
            match &self.journal {
                Some(journal) => journal.write(line),
                None => log::info!("{line}"),
            }
        }
    }

    #[must_use]
    pub fn report(&self) -> String {
        let gaps = sorted(&self.gaps);
        let work = sorted(&self.work);
        // Écarts par tranche : à 120 Hz, presque tout doit tomber sous 10 ms
        // pendant que les animaux bougent (30 i/s au repos : vers 33 ms).
        let buckets = [10.0, 20.0, 40.0, 60.0];
        let mut counts = [0usize; 5];
        for gap in &gaps {
            let i = buckets.iter().position(|b| gap <= b).unwrap_or(4);
            counts[i] += 1;
        }
        let mut out = format!(
            "{} images ({} sur l'horloge de l'écran) ; écarts : médian {:.1} ms, \
             99 % sous {:.1} ms, max {:.1} ms (≤ 10 ms : {}, ≤ 20 : {}, ≤ 40 : {}, \
             ≤ 60 : {}, au-delà : {}) ; préparation : médiane {:.2} ms, 99 % sous \
             {:.2} ms, max {:.1} ms",
            self.work.len(),
            self.beats,
            percentile(&gaps, 0.5),
            percentile(&gaps, 0.99),
            gaps.last().copied().unwrap_or(0.0),
            counts[0],
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            percentile(&work, 0.5),
            percentile(&work, 0.99),
            work.last().copied().unwrap_or(0.0),
        );
        out.push_str("\n  seconde par seconde (images, plus grand écart en ms) :");
        for (i, (frames, gap)) in self.seconds.iter().enumerate() {
            let _ = write!(out, " {i}:{frames}/{gap:.0}");
        }
        if !self.slow.is_empty() {
            out.push_str("\n  images les plus lentes à préparer :");
            for slow in &self.slow {
                let _ = write!(
                    out,
                    "\n    à {:.2} s : {:.1} ms ({})",
                    slow.at, slow.work, slow.phases
                );
            }
        }
        if !self.behaviors.is_empty() {
            out.push_str(
                "\n  par comportement (temps, images de la séquence par seconde, plus grand écart) :",
            );
            for (name, stats) in &self.behaviors {
                let rate = if stats.time > 0.0 {
                    stats.changes as f32 / stats.time
                } else {
                    0.0
                };
                let _ = write!(
                    out,
                    "\n    {name} : {:.1} s, {rate:.1} images/s, {:.0} ms",
                    stats.time, stats.worst_gap
                );
            }
        }
        out
    }

    /// Fin de l'essai : le bilan au journal (et au fichier, attendu jusqu'au bout).
    pub fn finish(&mut self) -> String {
        let report = self.report();
        if let Some(journal) = self.journal.take() {
            journal.write(format!("BILAN {report}"));
            let path = journal.finish();
            log::info!("journal de l'essai : {}", path.display());
        }
        report
    }
}

/// Mémoire et processeur de Mouflon, d'après `ps` (essais seulement).
fn process_usage() -> String {
    let pid = std::process::id().to_string();
    std::process::Command::new("ps")
        .args(["-o", "rss=,%cpu=", "-p", &pid])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|text| {
            let mut parts = text.split_whitespace();
            let rss: f32 = parts.next()?.parse().ok()?;
            let cpu: f32 = parts.next()?.replace(',', ".").parse().ok()?;
            Some(format!("memoire_mo {:.0} cpu {cpu:.0}", rss / 1024.0))
        })
        .unwrap_or_default()
}

/// Fichier du journal, écrit sur un fil à part : l'animation n'attend
/// jamais le disque.
#[derive(Debug)]
struct Journal {
    sender: Sender<String>,
    writer: JoinHandle<()>,
    path: PathBuf,
}

impl Journal {
    fn open(path: PathBuf) -> Option<Self> {
        let (file, path) = match std::fs::File::create(&path) {
            Ok(file) => (file, path),
            Err(e) => {
                // Lancé depuis le Finder, le dossier courant n'est pas à nous.
                let fallback = std::env::temp_dir().join("mouflon-essai.log");
                log::warn!(
                    "journal {} : {e} ; écrit dans {}",
                    path.display(),
                    fallback.display()
                );
                (std::fs::File::create(&fallback).ok()?, fallback)
            }
        };
        let (sender, receiver) = channel();
        let writer = std::thread::Builder::new()
            .name("mouflon-journal".into())
            .spawn(move || write_all(file, &receiver))
            .ok()?;
        log::info!("journal de l'essai : {}", path.display());
        Some(Self {
            sender,
            writer,
            path,
        })
    }

    fn write(&self, line: String) {
        let _ = self.sender.send(line);
    }

    /// Ferme le journal et attend qu'il soit écrit en entier.
    fn finish(self) -> PathBuf {
        let Self {
            sender,
            writer,
            path,
        } = self;
        drop(sender);
        let _ = writer.join();
        path
    }
}

fn write_all(file: std::fs::File, lines: &Receiver<String>) {
    let mut out = std::io::BufWriter::new(file);
    loop {
        match lines.recv_timeout(Duration::from_millis(500)) {
            Ok(line) => {
                let _ = writeln!(out, "{line}");
            }
            Err(RecvTimeoutError::Timeout) => {
                let _ = out.flush();
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    let _ = out.flush();
}

fn sorted(values: &[f32]) -> Vec<f32> {
    let mut values = values.to_vec();
    values.sort_by(f32::total_cmp);
    values
}

/// Valeur sous laquelle tombe la part `p` des valeurs triées.
fn percentile(sorted: &[f32], p: f32) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    let i = ((sorted.len() - 1) as f32 * p).round() as usize;
    sorted[i.min(sorted.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pet(behavior: &str, frame: u32) -> PetLine<'_> {
        PetLine {
            id: 0,
            name: "Laine",
            behavior,
            frame: Some(frame),
            x: 10.0,
            y: 20.0,
            vx: 0.0,
            vy: 0.0,
            support: 's',
            roll: 0.0,
        }
    }

    #[test]
    fn reports_gaps_and_work() {
        let start = Instant::now();
        let mut stats = FrameStats::new(start);
        for i in 0..5u64 {
            stats.record(
                start + Duration::from_millis(i * 8),
                i > 0,
                Phases::default(),
                [pet("marche", i as u32 / 2)].into_iter(),
            );
        }
        let report = stats.report();
        assert!(report.starts_with("5 images (4 sur l'horloge"), "{report}");
        assert!(report.contains("médian 8.0 ms"), "{report}");
        assert!(report.contains("≤ 10 ms : 4"), "{report}");
        assert!(report.contains(" 0:5/8"), "{report}");
        // Deux changements d'image en 32 ms : 62,5 images par seconde.
        assert!(report.contains("marche : 0.0 s, 62.5 images/s"), "{report}");
        assert_eq!(percentile(&[], 0.5), 0.0);
    }

    #[test]
    fn keeps_the_slowest_frames() {
        let start = Instant::now() - Duration::from_secs(1);
        let mut stats = FrameStats::new(start);
        stats.record(
            start,
            false,
            Phases {
                sounds: Duration::from_millis(900),
                ..Phases::default()
            },
            std::iter::empty(),
        );
        let report = stats.report();
        assert!(report.contains("images les plus lentes"), "{report}");
        assert!(report.contains("sons 900.0"), "{report}");
    }

    #[test]
    fn writes_a_full_journal() {
        let path = std::env::temp_dir().join(format!("mouflon-journal-{}.log", std::process::id()));
        let start = Instant::now();
        let mut stats = FrameStats::new(start).with_journal_at(path.clone());
        stats.raw("ESSAI test");
        stats.record(
            start,
            true,
            Phases::default(),
            [pet("marche", 3)].into_iter(),
        );
        stats.note("heurt 0 1");
        stats.finish();
        let text = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert!(text.starts_with("ESSAI test\n"), "{text}");
        assert!(text.contains("| 0 marche 3 10.0,20.0 0,0 s 0.00"), "{text}");
        assert!(text.contains("E ") && text.contains("heurt 0 1"), "{text}");
        assert!(text.contains("BILAN 1 images"), "{text}");
    }
}
