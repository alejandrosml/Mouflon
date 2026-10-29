//! Mesures de fluidité, pour vérifier une version : `MOUFLON_ESSAI=8` lance
//! Mouflon pour 8 secondes, puis note au journal la régularité des images
//! avant de quitter (l'intégration continue s'en sert sur Mac).

use std::time::{Duration, Instant};

/// Durée d'essai demandée (variable d'environnement `MOUFLON_ESSAI`, en secondes).
#[must_use]
pub fn trial() -> Option<Duration> {
    let seconds: f64 = std::env::var("MOUFLON_ESSAI").ok()?.trim().parse().ok()?;
    (seconds.is_finite() && seconds > 0.0).then(|| Duration::from_secs_f64(seconds))
}

/// Écarts entre les images et temps passé à les préparer.
#[derive(Debug, Default)]
pub struct FrameStats {
    /// Écart avec l'image précédente, en millisecondes.
    gaps: Vec<f32>,
    /// Temps de préparation de chaque image, en millisecondes.
    work: Vec<f32>,
    /// Images données par l'horloge de l'écran (les autres : la minuterie).
    beats: usize,
    last: Option<Instant>,
}

impl FrameStats {
    /// Une image commencée à `started` (et finie maintenant).
    pub fn record(&mut self, started: Instant, beat: bool) {
        if let Some(last) = self.last {
            self.gaps
                .push(started.duration_since(last).as_secs_f32() * 1000.0);
        }
        self.last = Some(started);
        self.work.push(started.elapsed().as_secs_f32() * 1000.0);
        if beat {
            self.beats += 1;
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
        format!(
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
        )
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
        let mut stats = FrameStats::default();
        let start = Instant::now();
        for i in 0..5u64 {
            stats.record(start + Duration::from_millis(i * 8), i > 0);
        }
        let report = stats.report();
        assert!(report.starts_with("5 images (4 sur l'horloge"), "{report}");
        assert!(report.contains("médian 8.0 ms"), "{report}");
        assert!(report.contains("≤ 10 ms : 4"), "{report}");
        assert_eq!(percentile(&[], 0.5), 0.0);
    }
}
