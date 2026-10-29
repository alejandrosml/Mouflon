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
        sound.setVolume(volume.clamp(0.0, 1.0));
        if !sound.play() {
            return false;
        }
        playing.push(sound);
        true
    })
}
