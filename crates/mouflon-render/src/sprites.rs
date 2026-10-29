//! Images d'un animal en sprites : planches découpées en cases, réduites
//! d'avance à la finesse de l'écran et rognées à leur contenu.

use std::io::Cursor;

use mouflon_core::{Facings, SpritePage, SpriteSheet};
use vello::peniko::Blob;

use crate::display::Bitmap;

#[derive(Debug, thiserror::Error)]
pub enum SpriteError {
    #[error("PNG illisible : {0}")]
    Png(#[from] png::DecodingError),
    #[error("planche de sprites invalide : {0}")]
    Invalid(String),
}

/// Pixels d'image par point visés (écrans Retina) : une planche plus fine est
/// réduite d'avance, avec un vrai filtre, plutôt qu'à chaque image par le GPU.
const PIXELS_PER_POINT: f32 = 2.0;

/// Une case, rognée à ce qu'elle contient.
#[derive(Clone, Debug)]
pub struct SpriteFrame {
    pub bitmap: Bitmap,
    /// Coin haut gauche de l'image dans sa case, en pixels de la planche.
    pub x: f32,
    pub y: f32,
    /// Pixels de la planche par pixel de l'image (plus de 1 : image réduite).
    pub texel: f32,
}

/// Case à dessiner pour un sens donné.
#[derive(Clone, Copy, Debug)]
pub struct Pick<'a> {
    pub frame: &'a SpriteFrame,
    /// Abscisse du point d'appui dans la case (pixels de la planche).
    pub anchor_x: f32,
    /// L'image est tournée vers la droite (dessinée ainsi, ou à retourner).
    pub faces_right: bool,
}

#[derive(Clone, Debug)]
pub struct SpriteFrames {
    /// Taille d'une case, en pixels de la planche.
    pub tile_width: u32,
    pub tile_height: u32,
    /// Points par pixel de la planche.
    pub scale: f32,
    pub frames: Vec<SpriteFrame>,
    pub facings: Facings,
    /// Point d'appui dans une case, image tournée vers la gauche (pixels).
    pub anchor: (f32, f32),
    /// Cases d'une variante de couleur, les deux sens compris.
    skin_block: u32,
    /// Cases de la planche principale : les images intermédiaires suivent.
    regular: u32,
}

impl SpriteFrames {
    pub fn decode(sheet: &SpriteSheet) -> Result<Self, SpriteError> {
        if sheet.pages.is_empty() {
            return Err(SpriteError::Invalid("aucune planche".into()));
        }
        // Les planches se décodent en parallèle (une bande par animation : il
        // y en a souvent des dizaines).
        let workers = std::thread::available_parallelism()
            .map_or(1, std::num::NonZero::get)
            .clamp(1, 8);
        // Les images intermédiaires suivent celles de la planche principale.
        let pages: Vec<&SpritePage> = sheet
            .pages
            .iter()
            .chain(sheet.inbetweens.iter().flat_map(|i| i.pages.iter()))
            .collect();
        let per_worker = pages.len().div_ceil(workers);
        let decoded: Vec<Result<Vec<DecodedPage>, SpriteError>> = std::thread::scope(|scope| {
            let jobs: Vec<_> = pages
                .chunks(per_worker)
                .map(|pages| {
                    scope.spawn(move || {
                        pages
                            .iter()
                            .map(|page| decode_page(page, sheet))
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect();
            jobs.into_iter()
                .map(|job| {
                    job.join()
                        .unwrap_or_else(|_| Err(SpriteError::Invalid("décodage interrompu".into())))
                })
                .collect()
        });
        let mut tile: Option<(u32, u32)> = None;
        let regular = sheet.tile_count();
        let mut frames =
            Vec::with_capacity(pages.iter().map(|p| p.tiles_x * p.tiles_y).sum::<u32>() as usize);
        for page in decoded
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
        {
            if tile.is_some_and(|t| t != page.tile) {
                return Err(SpriteError::Invalid(format!(
                    "cases de {} × {} pixels, différentes des autres planches",
                    page.tile.0, page.tile.1
                )));
            }
            tile = Some(page.tile);
            frames.extend(page.frames);
        }
        let (tile_width, tile_height) = tile.unwrap_or((1, 1));
        let [ax, ay] = sheet
            .anchor
            .unwrap_or([tile_width as f32 * 0.5, tile_height as f32]);
        Ok(Self {
            tile_width,
            tile_height,
            scale: sheet.scale,
            skin_block: (regular / sheet.skins.max(1)).max(1),
            regular,
            frames,
            facings: sheet.facings,
            anchor: (ax, ay),
        })
    }

    #[must_use]
    pub fn frame(&self, index: u32) -> Option<&SpriteFrame> {
        self.frames.get(index as usize)
    }

    /// Case à dessiner pour l'image `index` (désignée dans le premier sens)
    /// quand l'animal regarde dans le sens `facing` (1 : vers la droite).
    #[must_use]
    pub fn pick(&self, index: u32, facing: f32) -> Option<Pick<'_>> {
        let right = facing > 0.0;
        let (ax, _) = self.anchor;
        match self.facings {
            Facings::Left | Facings::Right => Some(Pick {
                frame: self.frame(index)?,
                anchor_x: ax,
                faces_right: self.facings == Facings::Right,
            }),
            Facings::Both => {
                // Images de la planche principale, par variante de couleur ;
                // images intermédiaires à la suite, elles aussi dans les deux sens.
                let (base, block) = if index >= self.regular {
                    (
                        self.regular,
                        (self.frames.len() as u32 - self.regular).max(1),
                    )
                } else {
                    (index / self.skin_block * self.skin_block, self.skin_block)
                };
                let half = block / 2;
                let within = index - base;
                let tile = if right && within < half {
                    base + half + within
                } else {
                    index
                };
                // Les images tournées vers la droite sont dessinées en miroir des
                // autres : leur point d'appui aussi.
                let faces_right = tile - base >= half;
                Some(Pick {
                    frame: self.frame(tile)?,
                    anchor_x: if faces_right {
                        self.tile_width as f32 - ax
                    } else {
                        ax
                    },
                    faces_right,
                })
            }
        }
    }
}

/// Une planche découpée.
struct DecodedPage {
    tile: (u32, u32),
    frames: Vec<SpriteFrame>,
}

fn decode_page(page: &SpritePage, sheet: &SpriteSheet) -> Result<DecodedPage, SpriteError> {
    let (width, height, mut rgba) = decode_png_rgba(&page.png)?;
    if let Some([kr, kg, kb]) = sheet.key_color {
        for px in rgba.as_chunks_mut::<4>().0 {
            if px[0] == kr && px[1] == kg && px[2] == kb {
                *px = [0, 0, 0, 0];
            }
        }
    }
    if page.tiles_x == 0 || page.tiles_y == 0 {
        return Err(SpriteError::Invalid("nombre de cases nul".into()));
    }
    let (tile_w, tile_h) = (width / page.tiles_x, height / page.tiles_y);
    if tile_w == 0 || tile_h == 0 {
        return Err(SpriteError::Invalid(format!(
            "image {width} × {height} trop petite pour {} × {} cases",
            page.tiles_x, page.tiles_y
        )));
    }
    let target = reduced_size(tile_w, tile_h, sheet.scale);
    let mut frames = Vec::with_capacity((page.tiles_x * page.tiles_y) as usize);
    for ty in 0..page.tiles_y {
        for tx in 0..page.tiles_x {
            let mut cell = Vec::with_capacity((tile_w * tile_h * 4) as usize);
            for y in 0..tile_h {
                let row = ((ty * tile_h + y) * width + tx * tile_w) as usize * 4;
                cell.extend_from_slice(&rgba[row..row + tile_w as usize * 4]);
            }
            let (w, h, cell, texel) = match target {
                Some((w, h)) => (
                    w,
                    h,
                    resample(&cell, tile_w, tile_h, w, h),
                    tile_w as f32 / w as f32,
                ),
                None => (tile_w, tile_h, cell, 1.0),
            };
            frames.push(trimmed(&cell, w, h, texel));
        }
    }
    Ok(DecodedPage {
        tile: (tile_w, tile_h),
        frames,
    })
}

/// Taille des images réduites d'avance, ou rien si la planche n'est pas plus
/// fine que l'écran (le pixel art agrandi garde alors ses pixels francs).
fn reduced_size(tile_w: u32, tile_h: u32, scale: f32) -> Option<(u32, u32)> {
    let k = scale * PIXELS_PER_POINT;
    if !(k > 0.0 && k < 0.999) {
        return None;
    }
    let w = ((tile_w as f32 * k).round() as u32).max(1);
    let h = ((tile_h as f32 * k).round() as u32).max(1);
    Some((w, h))
}

/// Part de chaque pixel source dans chaque pixel réduit (filtre de surface).
fn coverage(source: u32, target: u32) -> Vec<Vec<(usize, f32)>> {
    let ratio = source as f32 / target as f32;
    (0..target)
        .map(|o| {
            let start = o as f32 * ratio;
            let end = start + ratio;
            let mut taps = Vec::with_capacity(ratio.ceil() as usize + 1);
            let mut s = start.floor() as usize;
            while (s as f32) < end && s < source as usize {
                let cover = end.min(s as f32 + 1.0) - start.max(s as f32);
                if cover > 1e-6 {
                    taps.push((s, cover / ratio));
                }
                s += 1;
            }
            taps
        })
        .collect()
}

/// Réduit une image RGBA (alpha non prémultiplié) en moyennant les surfaces
/// couvertes, couleurs pondérées par leur opacité : pas de liseré sombre.
fn resample(rgba: &[u8], width: u32, height: u32, out_w: u32, out_h: u32) -> Vec<u8> {
    let premultiplied: Vec<[f32; 4]> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|p| {
            let a = f32::from(p[3]) / 255.0;
            [
                f32::from(p[0]) * a,
                f32::from(p[1]) * a,
                f32::from(p[2]) * a,
                a,
            ]
        })
        .collect();
    let columns = coverage(width, out_w);
    let rows = coverage(height, out_h);
    let mut wide = vec![[0.0f32; 4]; (out_w * height) as usize];
    for y in 0..height as usize {
        for (ox, taps) in columns.iter().enumerate() {
            let mut acc = [0.0f32; 4];
            for &(sx, weight) in taps {
                let p = premultiplied[y * width as usize + sx];
                for c in 0..4 {
                    acc[c] += p[c] * weight;
                }
            }
            wide[y * out_w as usize + ox] = acc;
        }
    }
    let mut out = Vec::with_capacity((out_w * out_h * 4) as usize);
    for taps in &rows {
        for ox in 0..out_w as usize {
            let mut acc = [0.0f32; 4];
            for &(sy, weight) in taps {
                let p = wide[sy * out_w as usize + ox];
                for c in 0..4 {
                    acc[c] += p[c] * weight;
                }
            }
            let a = acc[3];
            if a <= 1e-6 {
                out.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                let channel = |v: f32| (v / a).round().clamp(0.0, 255.0) as u8;
                out.extend_from_slice(&[
                    channel(acc[0]),
                    channel(acc[1]),
                    channel(acc[2]),
                    (a * 255.0).round().clamp(0.0, 255.0) as u8,
                ]);
            }
        }
    }
    out
}

/// Rogne une image à ses pixels visibles.
fn trimmed(rgba: &[u8], width: u32, height: u32, texel: f32) -> SpriteFrame {
    let (w, h) = (width as usize, height as usize);
    let visible = |x: usize, y: usize| rgba[(y * w + x) * 4 + 3] != 0;
    let rows: Vec<usize> = (0..h).filter(|&y| (0..w).any(|x| visible(x, y))).collect();
    let (Some(&top), Some(&bottom)) = (rows.first(), rows.last()) else {
        // Case vide : un pixel transparent.
        return SpriteFrame {
            bitmap: Bitmap {
                width: 1,
                height: 1,
                rgba: Blob::from(vec![0u8; 4]),
            },
            x: 0.0,
            y: 0.0,
            texel,
        };
    };
    let left = (0..w)
        .find(|&x| (top..=bottom).any(|y| visible(x, y)))
        .unwrap_or(0);
    let right = (0..w)
        .rev()
        .find(|&x| (top..=bottom).any(|y| visible(x, y)))
        .unwrap_or(w - 1);
    let mut out = Vec::with_capacity((right - left + 1) * (bottom - top + 1) * 4);
    for y in top..=bottom {
        out.extend_from_slice(&rgba[(y * w + left) * 4..(y * w + right + 1) * 4]);
    }
    SpriteFrame {
        bitmap: Bitmap {
            width: (right - left + 1) as u32,
            height: (bottom - top + 1) as u32,
            rgba: Blob::from(out),
        },
        x: left as f32 * texel,
        y: top as f32 * texel,
        texel,
    }
}

/// Décode un PNG quelconque en RGBA 8 bits.
pub fn decode_png_rgba(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), SpriteError> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info()?;
    let size = reader
        .output_buffer_size()
        .ok_or_else(|| SpriteError::Invalid("image trop grande".into()))?;
    let mut buf = vec![0; size];
    let info = reader.next_frame(&mut buf)?;
    let pixels = (info.width * info.height) as usize;
    let rgba = match info.color_type {
        png::ColorType::Rgba => buf[..pixels * 4].to_vec(),
        png::ColorType::Rgb => buf[..pixels * 3]
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|&[r, g, b]| [r, g, b, 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => buf[..pixels * 2]
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|&[g, a]| [g, g, g, a])
            .collect(),
        png::ColorType::Grayscale => buf[..pixels].iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => {
            return Err(SpriteError::Invalid("palette non développée".into()));
        }
    };
    Ok((info.width, info.height, rgba))
}

/// Encode une image RGBA en PNG (utilisé par l'aperçu SVG).
#[must_use]
pub fn encode_png_rgba(bitmap: &Bitmap) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, bitmap.width, bitmap.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        if let Ok(mut writer) = encoder.write_header() {
            let _ = writer.write_image_data(bitmap.rgba.data());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use mouflon_core::Inbetweens;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn png(width: u32, height: u32, pixel: impl Fn(u32, u32) -> [u8; 4]) -> Vec<u8> {
        let mut rgba = Vec::new();
        for y in 0..height {
            for x in 0..width {
                rgba.extend_from_slice(&pixel(x, y));
            }
        }
        encode_png_rgba(&Bitmap {
            width,
            height,
            rgba: Blob::from(rgba),
        })
    }

    #[test]
    fn a_fine_sheet_is_reduced_without_dark_fringes() {
        // Bloc 2×2 : deux pixels rouges opaques, deux transparents (noirs).
        let rgba = [
            255, 0, 0, 255, 0, 0, 0, 0, //
            0, 0, 0, 0, 255, 0, 0, 255,
        ];
        assert_eq!(resample(&rgba, 2, 2, 1, 1), vec![255, 0, 0, 128]);
    }

    #[test]
    fn reduction_by_a_fraction_keeps_coverage() {
        // Trois colonnes réduites à deux : celle du milieu se partage.
        let taps = coverage(3, 2);
        assert_eq!(taps[0].len(), 2);
        let total: f32 = taps.iter().flatten().map(|(_, w)| w).sum();
        assert!((total - 2.0).abs() < 1e-5, "{total}");
        // Un blanc opaque reste blanc et opaque.
        let white = vec![255u8; 3 * 3 * 4];
        assert!(resample(&white, 3, 3, 2, 2).iter().all(|&v| v == 255));
    }

    #[test]
    fn slices_tiles_trims_them_and_applies_key_color() {
        // Planche 4×2 pixels, 2×1 cases : la case de gauche est magenta,
        // celle de droite n'a qu'une colonne visible.
        let sheet = SpriteSheet {
            key_color: Some([255, 0, 255]),
            ..SpriteSheet::single(
                Arc::from(png(4, 2, |x, _| match x {
                    0 | 1 => [255, 0, 255, 255],
                    3 => [10, 20, 30, 255],
                    _ => [0, 0, 0, 0],
                })),
                2,
                1,
            )
        };
        let frames = SpriteFrames::decode(&sheet).unwrap();
        assert_eq!(frames.frames.len(), 2);
        assert_eq!((frames.tile_width, frames.tile_height), (2, 2));
        let empty = &frames.frames[0];
        assert_eq!((empty.bitmap.width, empty.bitmap.height), (1, 1));
        let column = &frames.frames[1];
        assert_eq!((column.bitmap.width, column.bitmap.height), (1, 2));
        assert_eq!((column.x, column.y), (1.0, 0.0));
        assert_eq!(&column.bitmap.rgba.data()[..4], &[10, 20, 30, 255]);
    }

    #[test]
    fn both_facings_pick_their_own_frames() {
        // Deux bandes de 2 images de 8 px : vers la gauche, puis vers la droite.
        let strip = |shade: u8| SpritePage {
            png: Arc::from(png(16, 8, move |x, _| [shade, (x / 8) as u8, 0, 255])),
            tiles_x: 2,
            tiles_y: 1,
        };
        let sheet = SpriteSheet {
            pages: vec![strip(10), strip(200)],
            facings: Facings::Both,
            anchor: Some([2.0, 7.0]),
            ..SpriteSheet::single(Arc::from(Vec::new()), 1, 1)
        };
        let frames = SpriteFrames::decode(&sheet).unwrap();
        let left = frames.pick(1, -1.0).unwrap();
        assert!(!left.faces_right);
        assert_eq!(&left.frame.bitmap.rgba.data()[..2], &[10, 1]);
        assert_eq!(left.anchor_x, 2.0);
        let right = frames.pick(1, 1.0).unwrap();
        assert!(right.faces_right);
        assert_eq!(&right.frame.bitmap.rgba.data()[..2], &[200, 1]);
        assert_eq!(right.anchor_x, 6.0, "point d'appui en miroir");
    }

    #[test]
    fn inbetweens_follow_the_sheet_in_both_facings() {
        let strip = |shade: u8| SpritePage {
            png: Arc::from(png(16, 8, move |x, _| [shade, (x / 8) as u8, 0, 255])),
            tiles_x: 2,
            tiles_y: 1,
        };
        let sheet = SpriteSheet {
            pages: vec![strip(10), strip(200)],
            facings: Facings::Both,
            inbetweens: Some(Arc::new(Inbetweens {
                pages: vec![strip(30), strip(230)],
                pairs: HashMap::from([((0, 1), (4, 1)), ((1, 0), (5, 1))]),
            })),
            ..SpriteSheet::single(Arc::from(Vec::new()), 1, 1)
        };
        let frames = SpriteFrames::decode(&sheet).unwrap();
        assert_eq!(frames.frames.len(), 8);
        // Seconde image intermédiaire : vers la gauche, puis vers la droite.
        let left = frames.pick(5, -1.0).unwrap();
        assert!(!left.faces_right);
        assert_eq!(&left.frame.bitmap.rgba.data()[..2], &[30, 1]);
        let right = frames.pick(5, 1.0).unwrap();
        assert!(right.faces_right);
        assert_eq!(&right.frame.bitmap.rgba.data()[..2], &[230, 1]);
        // La planche principale ne change pas.
        let main = frames.pick(1, 1.0).unwrap();
        assert_eq!(&main.frame.bitmap.rgba.data()[..2], &[200, 1]);
    }

    #[test]
    fn hd_frames_are_reduced_to_two_pixels_per_point() {
        // Case de 8 px affichée à 0,375 point par pixel : 6 px à l'écran Retina.
        let sheet = SpriteSheet {
            scale: 0.375,
            ..SpriteSheet::single(Arc::from(png(8, 8, |_, _| [90, 90, 90, 255])), 1, 1)
        };
        let frames = SpriteFrames::decode(&sheet).unwrap();
        let f = &frames.frames[0];
        assert_eq!((f.bitmap.width, f.bitmap.height), (6, 6));
        assert!((f.texel - 8.0 / 6.0).abs() < 1e-6);
        assert_eq!((frames.tile_width, frames.tile_height), (8, 8));
    }
}
