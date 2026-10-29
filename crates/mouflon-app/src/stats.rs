//! Mesures de fluidité, pour vérifier une version : `MOUFLON_ESSAI=8` lance
//! Mouflon pour 8 secondes, puis note au journal la régularité des images
//! avant de quitter (l'intégration continue s'en sert sur Mac).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::{Duration, Instant};

/// Durée d'essai demandée (variable d'environnement `MOUFLON_ESSAI`, en secondes).
#[must_use]
pub fn trial() -> Option<Duration> {
    let seconds: f64 = std::env::var("MOUFLON_ESSAI").ok()?.trim().parse().ok()?;
    (seconds.is_finite() && seconds > 0.0).then(|| Duration::from_secs_f64(seconds))
}

/// Temps passé dans chaque partie d'une image.
#[derive(Clone, Copy, Debug, Default)]
pub struct Phases {
    /// Relevés appliqués (écrans, fenêtres, Dock, dossier des animaux).
    pub watch: Duration,
    /// Simulation des animaux (et prise à la souris).
    pub sim: Duration,
    /// Sons lancés.
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

/// Une image lente : quand (secondes depuis le début), combien (ms), et où.
#[derive(Clone, Copy, Debug)]
struct Slow {
    at: f32,
    work: f32,
    phases: Phases,
}

/// Images préparées en plus de ce temps : notées en détail.
const SLOW_MS: f32 = 12.0;
/// Images lentes gardées pour le rapport (les pires).
const SLOW_KEPT: usize = 12;

/// Écarts entre les images et temps passé à les préparer.
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
    /// Comportement de chaque animal à la dernière image.
    behaviors: HashMap<u32, String>,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self::new(Instant::now())
    }
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
            behaviors: HashMap::new(),
        }
    }

    /// Note au journal ce qui arrive pendant l'essai, avec l'heure depuis
    /// son début : on retrouve ainsi ce qui accompagne une image lente.
    pub fn note(&self, what: impl std::fmt::Display) {
        log::info!("à {:.2} s : {what}", self.begun.elapsed().as_secs_f32());
    }

    /// Note les animaux qui changent de comportement.
    pub fn follow<'a>(&mut self, pets: impl Iterator<Item = (u32, &'a str, &'a str)>) {
        for (id, name, behavior) in pets {
            if self.behaviors.get(&id).is_some_and(|b| b == behavior) {
                continue;
            }
            self.note(format_args!("{name} n° {id} : {behavior}"));
            self.behaviors.insert(id, behavior.to_owned());
        }
    }

    /// Une image commencée à `started` (et finie maintenant).
    pub fn record(&mut self, started: Instant, beat: bool, phases: Phases) {
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
        out
    }
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

    #[test]
    fn reports_gaps_and_work() {
        let start = Instant::now();
        let mut stats = FrameStats::new(start);
        for i in 0..5u64 {
            stats.record(
                start + Duration::from_millis(i * 8),
                i > 0,
                Phases::default(),
            );
        }
        let report = stats.report();
        assert!(report.starts_with("5 images (4 sur l'horloge"), "{report}");
        assert!(report.contains("médian 8.0 ms"), "{report}");
        assert!(report.contains("≤ 10 ms : 4"), "{report}");
        assert!(report.contains(" 0:5/8"), "{report}");
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
        );
        let report = stats.report();
        assert!(report.contains("images les plus lentes"), "{report}");
        assert!(report.contains("sons 900.0"), "{report}");
    }
}
