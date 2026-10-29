//! Sons des animaux : NSSound joue les fichiers que macOS sait lire (MP3,
//! AAC, WAV, AIFF…), directement depuis la mémoire.

use std::cell::RefCell;

use objc2::AllocAnyThread;
use objc2::rc::Retained;
use objc2_app_kit::NSSound;
use objc2_foundation::NSData;

/// Sons joués en même temps, au plus : au-delà, un nouveau son est ignoré
/// (un troupeau qui bêle d'un coup reste supportable).
const MAX_PLAYING: usize = 6;

thread_local! {
    /// Sons en cours, gardés en vie jusqu'à la fin de leur lecture.
    static PLAYING: RefCell<Vec<Retained<NSSound>>> = const { RefCell::new(Vec::new()) };
}

/// Joue un son (contenu d'un fichier audio) au volume donné (de 0 à 1).
/// Renvoie `false` si macOS ne sait pas le lire, ou si trop de sons jouent déjà.
pub fn play_sound(data: &[u8], volume: f32) -> bool {
    play(data, volume.clamp(0.0, 1.0))
}

/// Met la sortie audio en route en jouant sans volume `sample` (un son des
/// animaux, pour que son décodeur soit chargé aussi), ou un court silence :
/// le premier son joué par l'app charge tout le nécessaire (une fraction de
/// seconde), et ferait sinon sauter l'animation au premier bêlement.
pub fn warm_up_sounds(sample: Option<&[u8]>) {
    if !sample.is_some_and(|data| play(data, 0.0)) {
        let _ = play(&silence(), 0.0);
    }
}

/// Un vingtième de seconde de silence, en WAV (PCM 16 bits mono, 8 kHz).
fn silence() -> Vec<u8> {
    const RATE: u32 = 8_000;
    const SAMPLES: u32 = RATE / 20;
    let data = SAMPLES * 2;
    let mut wav = Vec::with_capacity(44 + data as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&RATE.to_le_bytes());
    wav.extend_from_slice(&(RATE * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data.to_le_bytes());
    wav.resize(44 + data as usize, 0);
    wav
}

fn play(data: &[u8], volume: f32) -> bool {
    PLAYING.with(|playing| {
        let mut playing = playing.borrow_mut();
        playing.retain(|sound| sound.isPlaying());
        if playing.len() >= MAX_PLAYING {
            return false;
        }
        let data = NSData::with_bytes(data);
        let Some(sound) = NSSound::initWithData(NSSound::alloc(), &data) else {
            return false;
        };
        sound.setVolume(volume);
        if !sound.play() {
            return false;
        }
        playing.push(sound);
        true
    })
}
