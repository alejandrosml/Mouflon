//! Images des animaux déjà prêtes, gardées sur le disque.
//!
//! Décoder les planches PNG, réduire chaque case à la finesse de l'écran
//! puis la compresser en BC7 prend quelques secondes de calcul pour un
//! grand animal. Le résultat ne dépend que des planches et de la taille
//! choisie : il est gardé dans un fichier, et les lancements suivants le
//! lisent tel quel (quelques millisecondes), sans toucher aux planches.
//!
//! Le fichier porte une clé faite de tout ce qui donne les images : la
//! version de ce format, celle du codeur BC7, l'échelle, et pour chaque
//! planche son découpage et son identité (chemin, taille et date du
//! fichier ; ou ses octets, pour une planche intégrée au fichier de
//! l'animal). Une planche modifiée change la clé : le fichier est refait.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use mouflon_core::{PngData, SpriteSheet};
use vello::peniko::Blob;

use crate::bc7;
use crate::display::{Bitmap, Pixels};
use crate::sprites::{SpriteFrame, SpriteFrames};

/// Version du format et de la réduction des images : à changer quand l'un
/// ou l'autre change, pour que les anciens fichiers soient refaits.
const VERSION: u32 = 1;
const MAGIC: [u8; 8] = *b"MFLNIMG\0";
/// Côté d'une image au-delà duquel un fichier est jugé abîmé.
const MAX_SIDE: u32 = 16_384;

/// Fichier du cache pour les images d'un animal à cette échelle (nom de
/// fichier sûr, quel que soit l'identifiant de l'animal).
#[must_use]
pub fn path(dir: &Path, id: &str, sheet: &SpriteSheet) -> PathBuf {
    let safe: String = id
        .chars()
        .take(40)
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let mut h = Fnv::new();
    h.bytes(id.as_bytes());
    let scale = (f64::from(sheet.scale) * 10_000.0).round() as u64;
    dir.join(format!("{safe}-{:08x}-{scale}.images", h.0 as u32))
}

/// Clé des images d'une planche : `None` si une planche est illisible (le
/// décodage le dira).
#[must_use]
pub fn key(sheet: &SpriteSheet) -> Option<u64> {
    let mut h = Fnv::new();
    h.u32(VERSION);
    h.u32(bc7::VERSION);
    h.u32(sheet.scale.to_bits());
    match sheet.key_color {
        Some(rgb) => h.bytes(&[1, rgb[0], rgb[1], rgb[2]]),
        None => h.bytes(&[0]),
    }
    let pages = sheet
        .pages
        .iter()
        .chain(sheet.inbetweens.iter().flat_map(|i| i.pages.iter()));
    for page in pages {
        h.u32(page.tiles_x);
        h.u32(page.tiles_y);
        match &page.png {
            PngData::Bytes(bytes) => {
                h.bytes(b"octets");
                h.u64(bytes.len() as u64);
                h.bytes(bytes);
            }
            PngData::File { path, .. } => {
                let meta = std::fs::metadata(path).ok()?;
                let modified = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .unwrap_or_default();
                h.bytes(b"fichier");
                h.bytes(path.as_os_str().as_encoded_bytes());
                h.u64(meta.len());
                h.u64(modified.as_secs());
                h.u32(modified.subsec_nanos());
            }
        }
    }
    Some(h.0)
}

/// Les images gardées pour cette planche, si le fichier existe et qu'il
/// correspond à la clé ; il est alors daté du jour (voir [`prune`]).
#[must_use]
pub fn load(path: &Path, key: u64, sheet: &SpriteSheet) -> Option<SpriteFrames> {
    let bytes = std::fs::read(path).ok()?;
    let frames = parse(bytes, key, sheet)?;
    if let Ok(file) = std::fs::File::options().write(true).open(path) {
        let _ = file.set_modified(SystemTime::now());
    }
    Some(frames)
}

/// Garde les images d'une planche (écrites à côté, puis mises en place d'un
/// coup : un lancement simultané ne lit jamais un fichier à moitié écrit).
pub fn save(path: &Path, key: u64, frames: &SpriteFrames) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut out = Vec::with_capacity(64 + frames.frames.len() * 32 + frames.byte_size());
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&key.to_le_bytes());
    out.extend_from_slice(&frames.tile_width.to_le_bytes());
    out.extend_from_slice(&frames.tile_height.to_le_bytes());
    out.extend_from_slice(&(frames.frames.len() as u32).to_le_bytes());
    for frame in &frames.frames {
        let b = &frame.bitmap;
        let kind: u32 = u32::from(b.is_compressed());
        for value in [b.width, b.height] {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in [frame.x, frame.y, frame.texel] {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&(b.byte_size() as u64).to_le_bytes());
    }
    for frame in &frames.frames {
        match &frame.bitmap.pixels {
            Pixels::Rgba(blob) | Pixels::Bc7(blob) => out.extend_from_slice(blob.data()),
        }
    }
    let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
    std::fs::write(&temporary, &out)?;
    std::fs::rename(&temporary, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&temporary);
    })
}

/// Efface les fichiers du cache qui n'ont pas servi depuis `age` (animaux
/// retirés, tailles délaissées). Renvoie le nombre de fichiers effacés.
pub fn prune(dir: &Path, age: Duration) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let now = SystemTime::now();
    let mut removed = 0;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let ours = path
            .extension()
            .is_some_and(|e| e == "images" || e == "tmp");
        let old = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| now.duration_since(t).ok())
            .is_some_and(|elapsed| elapsed > age);
        if ours && old && std::fs::remove_file(&path).is_ok() {
            removed += 1;
        }
    }
    removed
}

/// Lit un fichier du cache ; `None` au moindre doute (autre version, autre
/// clé, longueurs incohérentes) : les images seront refaites.
fn parse(bytes: Vec<u8>, key: u64, sheet: &SpriteSheet) -> Option<SpriteFrames> {
    let mut r = Reader {
        bytes: &bytes,
        at: 0,
    };
    if r.take(8)? != MAGIC || r.u32()? != VERSION || r.u64()? != key {
        return None;
    }
    let tile = (r.u32()?, r.u32()?);
    if sheet.tile_size().is_some_and(|t| t != tile) {
        return None;
    }
    let count = r.u32()? as usize;
    let expected = sheet.tile_count() + sheet.inbetweens.as_ref().map_or(0, |i| i.tile_count());
    if count != expected as usize {
        return None;
    }
    let mut table = Vec::with_capacity(count);
    for _ in 0..count {
        let (width, height) = (r.u32()?, r.u32()?);
        let (x, y, texel) = (r.f32()?, r.f32()?, r.f32()?);
        let (kind, len) = (r.u32()?, usize::try_from(r.u64()?).ok()?);
        let sane = (1..=MAX_SIDE).contains(&width)
            && (1..=MAX_SIDE).contains(&height)
            && x.is_finite()
            && y.is_finite()
            && texel.is_finite()
            && texel > 0.0;
        let expected_len = match kind {
            0 => width as usize * height as usize * 4,
            1 => {
                let (tw, th) = Bitmap::bc7_size(width, height);
                tw as usize * th as usize
            }
            _ => return None,
        };
        if !sane || len != expected_len {
            return None;
        }
        table.push((width, height, x, y, texel, kind, len));
    }
    let start = r.at;
    let total: usize = table.iter().map(|t| t.6).sum();
    if bytes.len() != start.checked_add(total)? {
        return None;
    }
    // Toutes les images partagent le fichier lu, sans copie.
    let shared = Arc::new(bytes);
    let mut at = start;
    let frames = table
        .into_iter()
        .map(|(width, height, x, y, texel, kind, len)| {
            let blob = Blob::new(Arc::new(Slice {
                bytes: Arc::clone(&shared),
                range: at..at + len,
            }));
            at += len;
            SpriteFrame {
                bitmap: Bitmap {
                    width,
                    height,
                    pixels: if kind == 1 {
                        Pixels::Bc7(blob)
                    } else {
                        Pixels::Rgba(blob)
                    },
                },
                x,
                y,
                texel,
            }
        })
        .collect();
    Some(SpriteFrames::assemble(sheet, tile, frames))
}

/// Une partie du fichier lu.
struct Slice {
    bytes: Arc<Vec<u8>>,
    range: std::ops::Range<usize>,
}

impl AsRef<[u8]> for Slice {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[self.range.clone()]
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, count: usize) -> Option<&[u8]> {
        let slice = self.bytes.get(self.at..self.at.checked_add(count)?)?;
        self.at += count;
        Some(slice)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }

    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
}

/// Empreinte FNV-1a sur 64 bits : simple, et la même d'une version de Rust
/// à l'autre (ce que `DefaultHasher` ne promet pas).
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn bytes(&mut self, data: &[u8]) {
        for &byte in data {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }

    fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprites::encode_png_rgba;

    /// Une planche de 2 × 2 cases de 40 pixels, à moitié de sa taille.
    fn sheet() -> SpriteSheet {
        let (w, h) = (80u32, 80u32);
        let mut rgba = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let inside = (x % 40).abs_diff(20) < 14 && (y % 40).abs_diff(22) < 12;
                let a = if inside { 255 } else { 0 };
                rgba.extend_from_slice(&[(x * 3) as u8, (y * 3) as u8, 90, a]);
            }
        }
        let png = encode_png_rgba(&Bitmap::from_rgba(w, h, rgba));
        let mut sheet = SpriteSheet::single(PngData::from(png), 2, 2);
        sheet.scale = 0.25;
        sheet
    }

    #[test]
    fn compressed_frames_come_back_from_the_disk_unchanged() {
        let sheet = sheet();
        let frames = SpriteFrames::decode(&sheet).expect("planche");
        assert!(frames.compressible());
        let packed = frames.compressed(2, &|| {});
        assert!(!packed.compressible());
        assert!(packed.byte_size() < frames.byte_size());

        let dir = std::env::temp_dir().join(format!("mouflon-cache-{}", std::process::id()));
        let file = path(&dir, "essai/é", &sheet);
        assert!(file.file_name().unwrap().to_str().unwrap().is_ascii());
        let k = key(&sheet).expect("clé");
        save(&file, k, &packed).expect("écrit");
        let back = load(&file, k, &sheet).expect("relu");
        assert_eq!(back.frames.len(), packed.frames.len());
        for (a, b) in back.frames.iter().zip(&packed.frames) {
            assert_eq!((a.x, a.y, a.texel), (b.x, b.y, b.texel));
            assert_eq!(
                (a.bitmap.width, a.bitmap.height),
                (b.bitmap.width, b.bitmap.height)
            );
            assert_eq!(a.bitmap.is_compressed(), b.bitmap.is_compressed());
            assert_eq!(a.bitmap.to_rgba(), b.bitmap.to_rgba());
        }
        // Une autre clé (planche changée, autre taille) : le fichier est ignoré.
        assert!(load(&file, k ^ 1, &sheet).is_none());
        let mut other = sheet.clone();
        other.scale = 0.5;
        assert_ne!(key(&other), Some(k));
        // Un fichier tronqué aussi.
        let bytes = std::fs::read(&file).unwrap();
        std::fs::write(&file, &bytes[..bytes.len() - 1]).unwrap();
        assert!(load(&file, k, &sheet).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
