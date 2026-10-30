//! Peint les morceaux de l'animal d'exemple à squelette (`pets/aquarelle`) :
//! une brebis à l'aquarelle, sans pinceau ni logiciel de dessin.
//!
//! Chaque morceau est un lavis (une forme remplie de pigment) : la lumière
//! vient d'en haut à gauche, le pigment s'accumule sur les bords en séchant
//! (ils sont plus sombres), sa densité varie doucement (le lavis n'est jamais
//! uni) et le grain du papier le pique ; un trait de crayon léger cerne la
//! forme. Les morceaux sont exportés à la taille de toute la toile, comme
//! le ferait Blender (Grease Pencil) ou un logiciel de dessin : Mouflon les
//! recadre et les réduit lui-même.
//!
//! `cargo run --release -p mouflon-peintre -- pets/aquarelle` repeint les
//! morceaux (et un aperçu de l'ensemble, `apercu.png`, qui n'est pas livré).

use std::path::{Path, PathBuf};

use tiny_skia::{
    FillRule, LineCap, LineJoin, Paint, Path as SkPath, PathBuilder, Pixmap, Stroke, Transform,
};

/// Taille de la toile (pixels).
const W: usize = 1200;
const H: usize = 900;

type Rgb = [f32; 3];

fn hex(v: u32) -> Rgb {
    [
        ((v >> 16) & 0xff) as f32 / 255.0,
        ((v >> 8) & 0xff) as f32 / 255.0,
        (v & 0xff) as f32 / 255.0,
    ]
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// ------------------------------------------------------------------ bruit

/// Bruit de valeurs lissé, de -1 à 1, et sa somme sur plusieurs octaves.
#[derive(Clone, Copy, Debug)]
struct Noise {
    seed: u32,
}

impl Noise {
    fn hash(&self, x: i32, y: i32) -> f32 {
        let mut h = (x as u32)
            .wrapping_mul(0x8da6_b343)
            .wrapping_add((y as u32).wrapping_mul(0xd816_3841))
            .wrapping_add(self.seed.wrapping_mul(0xcb1a_b31f));
        h ^= h >> 13;
        h = h.wrapping_mul(0x5bd1_e995);
        h ^= h >> 15;
        (h & 0xffff) as f32 / 32767.5 - 1.0
    }

    fn value(&self, x: f32, y: f32) -> f32 {
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
        let (ix, iy) = (x0 as i32, y0 as i32);
        let a = self.hash(ix, iy);
        let b = self.hash(ix + 1, iy);
        let c = self.hash(ix, iy + 1);
        let d = self.hash(ix + 1, iy + 1);
        let top = a + (b - a) * sx;
        let bottom = c + (d - c) * sx;
        top + (bottom - top) * sy
    }

    /// Plusieurs octaves, la première de `scale` pixels.
    fn fbm(&self, x: f32, y: f32, scale: f32, octaves: u32) -> f32 {
        let (mut sum, mut amp, mut norm, mut f) = (0.0, 1.0, 0.0, 1.0 / scale);
        for o in 0..octaves {
            let shifted = Noise {
                seed: self.seed.wrapping_add(o * 1013),
            };
            sum += shifted.value(x * f, y * f) * amp;
            norm += amp;
            amp *= 0.5;
            f *= 2.0;
        }
        sum / norm
    }
}

// ----------------------------------------------------------------- masques

/// Ce que couvre un chemin, de 0 à 1 par pixel (antialiasé).
fn mask_of(path: &SkPath) -> Vec<f32> {
    let mut pixmap = Pixmap::new(W as u32, H as u32).expect("toile");
    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;
    pixmap.fill_path(path, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|p| f32::from(p[3]) / 255.0)
        .collect()
}

/// Ce que couvre un trait le long d'un chemin.
fn stroke_mask(path: &SkPath, width: f32) -> Vec<f32> {
    let mut pixmap = Pixmap::new(W as u32, H as u32).expect("toile");
    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;
    let stroke = Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Stroke::default()
    };
    pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
    pixmap
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|p| f32::from(p[3]) / 255.0)
        .collect()
}

/// Flou : trois passes de moyenne glissante, horizontales puis verticales
/// (presque une gaussienne).
fn blur(src: &[f32], radius: usize) -> Vec<f32> {
    if radius == 0 {
        return src.to_vec();
    }
    let mut a = src.to_vec();
    let mut b = vec![0.0; src.len()];
    let window = (2 * radius + 1) as f32;
    for _ in 0..3 {
        for y in 0..H {
            let row = &a[y * W..(y + 1) * W];
            let mut acc: f32 =
                row[..=radius.min(W - 1)].iter().sum::<f32>() + row[0] * radius as f32;
            for x in 0..W {
                b[y * W + x] = acc / window;
                let add = row[(x + radius + 1).min(W - 1)];
                let sub = row[x.saturating_sub(radius)];
                acc += add - sub;
            }
        }
        for x in 0..W {
            let mut acc: f32 =
                (0..=radius.min(H - 1)).map(|y| b[y * W + x]).sum::<f32>() + b[x] * radius as f32;
            for y in 0..H {
                a[y * W + x] = acc / window;
                let add = b[(y + radius + 1).min(H - 1) * W + x];
                let sub = b[y.saturating_sub(radius) * W + x];
                acc += add - sub;
            }
        }
    }
    a
}

// ------------------------------------------------------------------ calques

/// Un morceau en train d'être peint : couleurs prémultipliées.
struct Layer {
    px: Vec<[f32; 4]>,
}

impl Layer {
    fn new() -> Self {
        Self {
            px: vec![[0.0; 4]; W * H],
        }
    }

    /// Pose `color` avec l'opacité `alpha` sur le pixel `i`.
    fn over(&mut self, i: usize, color: Rgb, alpha: f32) {
        let a = alpha.clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let p = &mut self.px[i];
        let keep = 1.0 - a;
        p[0] = color[0] * a + p[0] * keep;
        p[1] = color[1] * a + p[1] * keep;
        p[2] = color[2] * a + p[2] * keep;
        p[3] = a + p[3] * keep;
    }

    /// Assombrit (multiplie) ce qui est déjà peint, sans rien ajouter
    /// ailleurs : une ombre posée par-dessus.
    fn multiply(&mut self, i: usize, color: Rgb, amount: f32) {
        let k = amount.clamp(0.0, 1.0);
        let p = &mut self.px[i];
        for c in 0..3 {
            p[c] *= 1.0 - k + k * color[c];
        }
    }

    /// Ajoute un autre calque par-dessus.
    fn stack(&mut self, other: &Layer) {
        for (p, q) in self.px.iter_mut().zip(&other.px) {
            let keep = 1.0 - q[3];
            for c in 0..4 {
                p[c] = q[c] + p[c] * keep;
            }
        }
    }

    fn save(&self, path: &Path) {
        let mut rgba = Vec::with_capacity(W * H * 4);
        for p in &self.px {
            let a = p[3].clamp(0.0, 1.0);
            if a <= 1.0 / 512.0 {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            let c = |v: f32| ((v / a).clamp(0.0, 1.0) * 255.0).round() as u8;
            rgba.extend_from_slice(&[c(p[0]), c(p[1]), c(p[2]), (a * 255.0).round() as u8]);
        }
        let file = std::fs::File::create(path).expect("fichier");
        let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), W as u32, H as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::High);
        let mut writer = encoder.write_header().expect("en-tête PNG");
        writer.write_image_data(&rgba).expect("PNG");
    }
}

// ------------------------------------------------------------------- lavis

/// Un lavis : une forme remplie de pigment.
#[derive(Clone, Copy, Debug)]
struct Wash {
    /// Teintes au soleil, au milieu, à l'ombre.
    light: Rgb,
    base: Rgb,
    shadow: Rgb,
    /// D'où vient la lumière (vers où va l'ombre), dans la toile.
    toward_shadow: (f32, f32),
    /// Pigment accumulé sur les bords en séchant : sa teinte, sa force, sa
    /// largeur (pixels).
    pigment: Rgb,
    edge: f32,
    edge_width: usize,
    /// Le lavis n'est jamais uni : variations douces de densité.
    mottling: f32,
    /// Grain du papier.
    grain: f32,
    /// Trait de crayon autour : teinte, opacité, largeur (pixels).
    pencil: Option<(Rgb, f32, usize)>,
    seed: u32,
}

impl Wash {
    /// Peint la forme `mask` dans `layer`.
    fn paint(&self, mask: &[f32], layer: &mut Layer) {
        // La boîte de la forme : la lumière la traverse d'un bord à l'autre.
        let (mut x0, mut y0, mut x1, mut y1) = (W, H, 0, 0);
        for (i, m) in mask.iter().enumerate() {
            if *m > 0.0 {
                let (x, y) = (i % W, i / W);
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
        }
        if x0 > x1 {
            return;
        }
        let (cx, cy) = ((x0 + x1) as f32 * 0.5, (y0 + y1) as f32 * 0.5);
        let (dx, dy) = self.toward_shadow;
        let norm = (dx * dx + dy * dy).sqrt().max(1e-6);
        let (dx, dy) = (dx / norm, dy / norm);
        let extent = ((x1 - x0) as f32 * dx.abs() + (y1 - y0) as f32 * dy.abs()).max(1.0) * 0.5;
        // Bords : où le pigment s'accumule, et le volume qui s'arrondit.
        let soft = blur(mask, self.edge_width);
        let deep = blur(mask, self.edge_width * 4);
        let pencil = self.pencil.map(|(color, alpha, width)| {
            let inner = blur(mask, width);
            (color, alpha, inner)
        });
        let noise = Noise { seed: self.seed };
        let grain = Noise {
            seed: self.seed.wrapping_add(77),
        };
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = y * W + x;
                let m = mask[i];
                if m <= 0.0 {
                    continue;
                }
                let (fx, fy) = (x as f32, y as f32);
                let t = (((fx - cx) * dx + (fy - cy) * dy) / extent).clamp(-1.0, 1.0);
                let mut color = if t < 0.0 {
                    mix(self.base, self.light, -t)
                } else {
                    mix(self.base, self.shadow, t)
                };
                // Le volume : les bords tournent vers l'ombre.
                let round = (1.0 - deep[i]).clamp(0.0, 1.0);
                color = mix(color, self.shadow, round * 0.55);
                // Densité du lavis.
                let wash = noise.fbm(fx, fy, 110.0, 4);
                let k = 1.0 + self.mottling * wash;
                color = [color[0] * k, color[1] * k, color[2] * k];
                // Le pigment des bords.
                let rim = ((m - soft[i]) * 2.0).clamp(0.0, 1.0);
                color = mix(color, self.pigment, rim * self.edge);
                // Le grain du papier creuse le lavis par endroits.
                let g = grain.fbm(fx, fy, 5.0, 2).max(0.0);
                let k = 1.0 - self.grain * g;
                color = [color[0] * k, color[1] * k, color[2] * k];
                layer.over(i, color, m);
                if let Some((ink, alpha, inner)) = &pencil {
                    let band = (m * (1.0 - smoothstep(0.55, 0.98, inner[i]))).clamp(0.0, 1.0);
                    if band > 0.0 {
                        let line = noise.fbm(fx + 400.0, fy, 9.0, 2);
                        layer.over(i, *ink, band * alpha * (0.78 + 0.22 * line));
                    }
                }
            }
        }
    }
}

/// Pose `color` là où est `mask`, avec l'opacité `alpha`, seulement sur ce
/// que `clip` couvre déjà (un détail peint dans une forme).
fn dab(layer: &mut Layer, mask: &[f32], clip: Option<&[f32]>, color: Rgb, alpha: f32) {
    for (i, m) in mask.iter().enumerate() {
        let c = clip.map_or(1.0, |c| c[i]);
        let a = m * c * alpha;
        if a > 0.0 {
            layer.over(i, color, a);
        }
    }
}

// ------------------------------------------------------------------ formes

/// Une forme fermée passant par `points`, lissée (Catmull-Rom).
fn smooth_closed(points: &[(f32, f32)]) -> SkPath {
    let n = points.len();
    let mut pb = PathBuilder::new();
    pb.move_to(points[0].0, points[0].1);
    for i in 0..n {
        let p0 = points[(i + n - 1) % n];
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        let p3 = points[(i + 2) % n];
        let c1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
        let c2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
        pb.cubic_to(c1.0, c1.1, c2.0, c2.1, p2.0, p2.1);
    }
    pb.close();
    pb.finish().expect("forme")
}

/// Une courbe ouverte passant par `points`, lissée.
fn smooth_open(points: &[(f32, f32)]) -> SkPath {
    let n = points.len();
    let mut pb = PathBuilder::new();
    pb.move_to(points[0].0, points[0].1);
    for i in 0..n - 1 {
        let p0 = points[i.saturating_sub(1)];
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = points[(i + 2).min(n - 1)];
        let c1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
        let c2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
        pb.cubic_to(c1.0, c1.1, c2.0, c2.1, p2.0, p2.1);
    }
    pb.finish().expect("courbe")
}

/// Des cercles qui se fondent en une seule forme (une toison).
fn cloud(circles: &[(f32, f32, f32)]) -> SkPath {
    let mut pb = PathBuilder::new();
    for &(x, y, r) in circles {
        pb.push_circle(x, y, r);
    }
    pb.finish().expect("nuage")
}

fn ellipse(cx: f32, cy: f32, rx: f32, ry: f32) -> SkPath {
    let rect = tiny_skia::Rect::from_xywh(cx - rx, cy - ry, 2.0 * rx, 2.0 * ry).expect("ellipse");
    PathBuilder::from_oval(rect).expect("ellipse")
}

/// Une feuille (une oreille) de `base` à `tip`, de largeur `width`.
fn leaf(base: (f32, f32), tip: (f32, f32), width: f32, bulge: f32) -> SkPath {
    let (dx, dy) = (tip.0 - base.0, tip.1 - base.1);
    let len = (dx * dx + dy * dy).sqrt();
    let (nx, ny) = (-dy / len, dx / len);
    let at = |t: f32, side: f32| {
        let w = width * 0.5 * (std::f32::consts::PI * t.powf(0.8)).sin().max(0.0) * side;
        (
            base.0 + dx * t + nx * (w + bulge * t * (1.0 - t) * 4.0),
            base.1 + dy * t + ny * (w + bulge * t * (1.0 - t) * 4.0),
        )
    };
    let mut points = Vec::new();
    for k in 0..=8 {
        points.push(at(k as f32 / 8.0, 1.0));
    }
    for k in (1..8).rev() {
        points.push(at(k as f32 / 8.0, -1.0));
    }
    smooth_closed(&points)
}

// ---------------------------------------------------------------- la brebis

/// Palette : une laine crème aux ombres lavande, un visage cacao.
struct Palette {
    wool: Wash,
    face: Wash,
    leg: Wash,
    far_leg: Wash,
}

fn palette() -> Palette {
    let shadow_dir = (0.45, 1.0);
    let wool = Wash {
        light: hex(0xFFFDF7),
        base: hex(0xF3EDE2),
        shadow: hex(0xCFC6C9),
        toward_shadow: shadow_dir,
        pigment: hex(0x9E95B4),
        edge: 0.7,
        edge_width: 6,
        mottling: 0.05,
        grain: 0.06,
        pencil: Some((hex(0x5F5672), 0.75, 4)),
        seed: 11,
    };
    let face = Wash {
        light: hex(0xC09E8E),
        base: hex(0xA27E6E),
        shadow: hex(0x765A4F),
        toward_shadow: shadow_dir,
        pigment: hex(0x5E423B),
        edge: 0.55,
        edge_width: 5,
        mottling: 0.08,
        grain: 0.08,
        pencil: Some((hex(0x3F2A26), 0.8, 3)),
        seed: 23,
    };
    let leg = Wash {
        light: hex(0x8F7268),
        base: hex(0x7A5F56),
        shadow: hex(0x57423C),
        toward_shadow: (1.0, 0.25),
        pigment: hex(0x45322E),
        edge: 0.5,
        edge_width: 4,
        mottling: 0.07,
        grain: 0.08,
        pencil: Some((hex(0x33241F), 0.75, 3)),
        seed: 31,
    };
    let far_leg = Wash {
        light: hex(0x6F5852),
        base: hex(0x5F4A44),
        shadow: hex(0x46352F),
        seed: 37,
        ..leg
    };
    Palette {
        wool,
        face,
        leg,
        far_leg,
    }
}

/// Une patte, de la hanche (cachée dans la toison) au sabot posé au sol.
fn paint_leg(hip: (f32, f32), hoof_x: f32, wash: &Wash, hoof: Rgb) -> Layer {
    let ground = GROUND.1;
    let (top_w, ankle_w) = (40.0, 29.0);
    let ankle_y = ground - 34.0;
    let knee = (
        hip.0 + (hoof_x - hip.0) * 0.55 + 4.0,
        hip.1 + (ankle_y - hip.1) * 0.55,
    );
    let points = [
        (hip.0 - top_w * 0.5, hip.1 - 30.0),
        (hip.0 + top_w * 0.5, hip.1 - 30.0),
        (knee.0 + top_w * 0.42, knee.1),
        (hoof_x + ankle_w * 0.5, ankle_y),
        (hoof_x + ankle_w * 0.5, ground - 18.0),
        (hoof_x - ankle_w * 0.5, ground - 18.0),
        (hoof_x - ankle_w * 0.5, ankle_y),
        (knee.0 - top_w * 0.4, knee.1),
    ];
    let mut layer = Layer::new();
    let mask = mask_of(&smooth_closed(&points));
    wash.paint(&mask, &mut layer);
    // Le sabot : sombre, luisant sur le devant, fendu.
    let hoof_shape = smooth_closed(&[
        (hoof_x - 21.0, ground),
        (hoof_x - 19.0, ground - 22.0),
        (hoof_x - 8.0, ground - 28.0),
        (hoof_x + 10.0, ground - 28.0),
        (hoof_x + 22.0, ground - 18.0),
        (hoof_x + 24.0, ground),
    ]);
    let hoof_wash = Wash {
        light: mix(hoof, hex(0xFFFFFF), 0.18),
        base: hoof,
        shadow: mix(hoof, hex(0x000000), 0.35),
        toward_shadow: (1.0, 0.4),
        pigment: mix(hoof, hex(0x000000), 0.5),
        edge: 0.4,
        edge_width: 3,
        mottling: 0.04,
        grain: 0.05,
        pencil: Some((mix(hoof, hex(0x000000), 0.6), 0.6, 2)),
        seed: wash.seed + 5,
    };
    let hoof_mask = mask_of(&hoof_shape);
    hoof_wash.paint(&hoof_mask, &mut layer);
    let cleft = stroke_mask(
        &smooth_open(&[(hoof_x + 3.0, ground - 1.0), (hoof_x + 4.0, ground - 14.0)]),
        2.5,
    );
    dab(
        &mut layer,
        &cleft,
        Some(&hoof_mask),
        mix(hoof, hex(0x000000), 0.5),
        0.8,
    );
    let shine = mask_of(&ellipse(hoof_x + 12.0, ground - 17.0, 6.0, 3.5));
    dab(
        &mut layer,
        &blur(&shine, 2),
        Some(&hoof_mask),
        hex(0xFFFFFF),
        0.35,
    );
    layer
}

/// Le point d'appui (milieu des sabots au sol).
const GROUND: (f32, f32) = (560.0, 820.0);

/// Les hanches et les sabots : (hanche, x du sabot).
const LEGS: [((f32, f32), f32); 4] = [
    // Arrière loin, avant loin, arrière près, avant près.
    ((418.0, 650.0), 402.0),
    ((706.0, 655.0), 716.0),
    ((452.0, 665.0), 452.0),
    ((742.0, 668.0), 750.0),
];

fn body_circles() -> Vec<(f32, f32, f32)> {
    let noise = Noise { seed: 5 };
    let (cx, cy, rx, ry) = (560.0, 560.0, 205.0, 108.0);
    let mut circles = vec![(cx, cy, 0.0)];
    // Le cœur de la toison, puis ses boucles tout autour.
    circles[0] = (cx, cy + 6.0, 1.0);
    let count = 17;
    for k in 0..count {
        let a = k as f32 / count as f32 * std::f32::consts::TAU + 0.2;
        let wobble = noise.value(k as f32 * 1.7, 0.5);
        let (x, y) = (cx + rx * a.cos(), cy + ry * a.sin());
        // Les boucles du dessous sont plus petites (le ventre).
        let below = a.sin().max(0.0);
        let r = 64.0 + 10.0 * wobble - 16.0 * below;
        circles.push((x, y, r));
    }
    circles
}

fn paint_body(p: &Palette) -> Layer {
    let mut circles = body_circles();
    circles.remove(0);
    let mut pb = PathBuilder::new();
    for &(x, y, r) in &circles {
        pb.push_circle(x, y, r);
    }
    let rect =
        tiny_skia::Rect::from_xywh(560.0 - 222.0, 560.0 - 120.0, 444.0, 252.0).expect("corps");
    pb.push_oval(rect);
    let shape = pb.finish().expect("toison");
    let mask = mask_of(&shape);
    let mut layer = Layer::new();
    p.wool.paint(&mask, &mut layer);
    let inner = blur(&mask, 6);
    let clip: Vec<f32> = inner.iter().map(|v| smoothstep(0.55, 1.0, *v)).collect();
    // Le bord de la toison : seules ses boucles y sont ombrées.
    let rim: Vec<f32> = blur(&mask, 34)
        .iter()
        .map(|v| 1.0 - smoothstep(0.72, 0.97, *v))
        .collect();
    // Chaque boucle de la toison est un petit nuage : éclairée en haut à
    // gauche, dans l'ombre en bas à droite.
    for &(x, y, r) in &circles {
        let lit = blur(
            &mask_of(&ellipse(x - r * 0.25, y - r * 0.3, r * 0.55, r * 0.42)),
            10,
        );
        dab(&mut layer, &lit, Some(&clip), hex(0xFFFFFF), 0.42);
        let crescent = mask_of(&ellipse(x + r * 0.12, y + r * 0.2, r * 0.9, r * 0.8));
        let cut = mask_of(&ellipse(x - r * 0.05, y - r * 0.05, r * 0.86, r * 0.78));
        let shade: Vec<f32> = crescent
            .iter()
            .zip(&cut)
            .map(|(a, b)| (a - b).max(0.0))
            .collect();
        let shade = blur(&shade, 5);
        for (i, v) in shade.iter().enumerate() {
            if *v > 0.0 && mask[i] > 0.0 {
                layer.multiply(i, hex(0xC2B8CC), v * clip[i] * rim[i] * 0.7);
            }
        }
    }
    // Quelques boucles plus petites au milieu.
    let noise = Noise { seed: 9 };
    for k in 0..14 {
        let u = noise.hash(k, 1) * 0.5 + 0.5;
        let v = noise.hash(k, 2) * 0.5 + 0.5;
        let (x, y) = (400.0 + u * 330.0, 490.0 + v * 150.0);
        let r = 24.0 + 10.0 * (noise.hash(k, 3) * 0.5 + 0.5);
        let lit = blur(
            &mask_of(&ellipse(x - r * 0.2, y - r * 0.25, r * 0.55, r * 0.4)),
            8,
        );
        dab(&mut layer, &lit, Some(&clip), hex(0xFFFFFF), 0.3);
        let arc = stroke_mask(
            &smooth_open(&[
                (x - r * 0.8, y + r * 0.1),
                (x - r * 0.2, y + r * 0.75),
                (x + r * 0.7, y + r * 0.35),
            ]),
            4.0,
        );
        dab(&mut layer, &blur(&arc, 2), Some(&clip), hex(0xB7ACC0), 0.35);
    }
    // La lumière accroche le haut de la toison.
    let light = mask_of(&cloud(&[
        (470.0, 470.0, 46.0),
        (540.0, 452.0, 50.0),
        (615.0, 462.0, 40.0),
    ]));
    dab(
        &mut layer,
        &blur(&light, 26),
        Some(&clip),
        hex(0xFFFFFF),
        0.4,
    );
    // Le dessous, dans l'ombre.
    let belly = mask_of(&ellipse(575.0, 690.0, 230.0, 60.0));
    let belly = blur(&belly, 26);
    for (i, b) in belly.iter().enumerate() {
        if mask[i] > 0.0 && *b > 0.0 {
            layer.multiply(i, hex(0xB4A9BD), b * mask[i] * 0.55);
        }
    }
    layer
}

fn paint_tail(p: &Palette) -> Layer {
    let shape = cloud(&[
        (292.0, 528.0, 30.0),
        (276.0, 552.0, 26.0),
        (288.0, 576.0, 21.0),
    ]);
    let mut layer = Layer::new();
    p.wool.paint(&mask_of(&shape), &mut layer);
    layer
}

/// Décalage de la tête et de tout ce qu'elle porte (dessinés autour de
/// (900, 450)) : posée en avant et en haut de la toison.
const HO: (f32, f32) = (-40.0, 40.0);

/// Un point de la tête, décalé avec elle.
fn hp(x: f32, y: f32) -> (f32, f32) {
    (x + HO.0, y + HO.1)
}

fn shifted(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    points.iter().map(|&(x, y)| hp(x, y)).collect()
}

/// La tête de profil, tournée vers la droite.
const HEAD: [(f32, f32); 12] = [
    (800.0, 400.0),
    (846.0, 372.0),
    (902.0, 368.0),
    (946.0, 388.0),
    (974.0, 426.0),
    (986.0, 468.0),
    (978.0, 503.0),
    (950.0, 524.0),
    (906.0, 530.0),
    (858.0, 518.0),
    (818.0, 492.0),
    (797.0, 450.0),
];

fn paint_head(p: &Palette) -> Layer {
    let mut layer = Layer::new();
    let head = mask_of(&smooth_closed(&shifted(&HEAD)));
    p.face.paint(&head, &mut layer);
    // Le front, éclairé ; le dessous de la mâchoire, dans l'ombre.
    let (fx, fy) = hp(885.0, 405.0);
    let brow = blur(&mask_of(&ellipse(fx, fy, 46.0, 26.0)), 16);
    dab(&mut layer, &brow, Some(&head), hex(0xC9A898), 0.45);
    // Le museau, plus clair.
    let (mx, my) = hp(948.0, 484.0);
    let muzzle = blur(&mask_of(&ellipse(mx, my, 40.0, 33.0)), 9);
    dab(&mut layer, &muzzle, Some(&head), hex(0xD2B6A7), 0.8);
    // Les joues roses.
    let (bx, by) = hp(915.0, 476.0);
    let blush = blur(&mask_of(&ellipse(bx, by, 22.0, 14.0)), 8);
    dab(&mut layer, &blush, Some(&head), hex(0xF2A09B), 0.55);
    // La truffe et la bouche.
    let nose = mask_of(&smooth_closed(&shifted(&[
        (966.0, 461.0),
        (981.0, 457.0),
        (985.0, 468.0),
        (977.0, 476.0),
    ])));
    dab(&mut layer, &blur(&nose, 1), Some(&head), hex(0x553734), 0.9);
    let mouth = stroke_mask(
        &smooth_open(&shifted(&[(978.0, 489.0), (969.0, 499.0), (955.0, 502.0)])),
        3.0,
    );
    dab(&mut layer, &mouth, Some(&head), hex(0x4A302D), 0.8);
    layer
}

/// Le toupet de laine sur le front.
fn paint_topknot(p: &Palette) -> Layer {
    let circles: Vec<(f32, f32, f32)> = [
        (826.0, 384.0, 22.0),
        (850.0, 364.0, 26.0),
        (880.0, 358.0, 23.0),
        (904.0, 370.0, 17.0),
        (858.0, 386.0, 20.0),
    ]
    .iter()
    .map(|&(x, y, r)| {
        let (x, y) = hp(x, y);
        (x, y, r)
    })
    .collect();
    let shape = cloud(&circles);
    let mut layer = Layer::new();
    let wash = Wash {
        edge_width: 5,
        seed: 41,
        ..p.wool
    };
    wash.paint(&mask_of(&shape), &mut layer);
    layer
}

/// Les oreilles : leur attache et leur pointe (tombantes, vers l'arrière).
const EAR_FAR: ((f32, f32), (f32, f32)) =
    ((834.0 + HO.0, 400.0 + HO.1), (760.0 + HO.0, 404.0 + HO.1));
const EAR_NEAR: ((f32, f32), (f32, f32)) =
    ((846.0 + HO.0, 414.0 + HO.1), (752.0 + HO.0, 446.0 + HO.1));

fn paint_ear(p: &Palette, base: (f32, f32), tip: (f32, f32), far: bool) -> Layer {
    let mut layer = Layer::new();
    let shape = leaf(base, tip, if far { 46.0 } else { 56.0 }, -7.0);
    let mask = mask_of(&shape);
    let wash = if far {
        Wash {
            light: hex(0x8F7064),
            base: hex(0x7C6055),
            shadow: hex(0x5E463F),
            seed: 51,
            ..p.face
        }
    } else {
        Wash { seed: 53, ..p.face }
    };
    wash.paint(&mask, &mut layer);
    if !far {
        // L'intérieur rose.
        let inner = leaf(
            (
                base.0 + (tip.0 - base.0) * 0.12,
                base.1 + (tip.1 - base.1) * 0.12 + 3.0,
            ),
            (
                base.0 + (tip.0 - base.0) * 0.86,
                base.1 + (tip.1 - base.1) * 0.86 + 2.0,
            ),
            26.0,
            -3.0,
        );
        let inner = blur(&mask_of(&inner), 3);
        dab(&mut layer, &inner, Some(&mask), hex(0xE7A9A3), 0.8);
    }
    layer
}

const EYE: (f32, f32) = (918.0 + HO.0, 440.0 + HO.1);

fn paint_eye() -> Layer {
    let mut layer = Layer::new();
    let (x, y) = EYE;
    let eye = mask_of(&ellipse(x, y, 15.0, 19.0));
    dab(&mut layer, &eye, None, hex(0x2A2125), 1.0);
    // Un iris brun, à peine visible, en bas de l'œil.
    let iris = blur(&mask_of(&ellipse(x + 1.0, y + 7.0, 10.0, 8.0)), 3);
    dab(&mut layer, &iris, Some(&eye), hex(0x6B4A3F), 0.55);
    let big = mask_of(&ellipse(x + 5.0, y - 7.0, 6.0, 6.5));
    dab(&mut layer, &big, None, hex(0xFFFFFF), 0.95);
    let small = mask_of(&ellipse(x - 5.0, y + 8.0, 2.8, 2.8));
    dab(&mut layer, &small, None, hex(0xFFFFFF), 0.8);
    layer
}

/// L'œil fermé : une paupière en arc, deux cils.
fn paint_closed_eye() -> Layer {
    let mut layer = Layer::new();
    let (x, y) = EYE;
    let lid = stroke_mask(
        &smooth_open(&[(x - 16.0, y + 2.0), (x, y + 10.0), (x + 16.0, y + 1.0)]),
        5.0,
    );
    dab(&mut layer, &lid, None, hex(0x2A2125), 0.95);
    for (dx, len) in [(-8.0, 7.0), (2.0, 8.0)] {
        let lash = stroke_mask(
            &smooth_open(&[(x + dx, y + 8.0), (x + dx - 2.0, y + 8.0 + len)]),
            3.0,
        );
        dab(&mut layer, &lash, None, hex(0x2A2125), 0.8);
    }
    layer
}

fn main() {
    let out: PathBuf = std::env::args()
        .nth(1)
        .map_or_else(|| PathBuf::from("pets/aquarelle"), PathBuf::from);
    std::fs::create_dir_all(&out).expect("dossier");
    let p = palette();
    let hoof = hex(0x3B302E);
    let far_hoof = hex(0x2F2725);
    // Dans l'ordre où ils se posent, de l'arrière vers l'avant.
    let parts: Vec<(&str, Layer)> = vec![
        (
            "patte-arriere-loin",
            paint_leg(LEGS[0].0, LEGS[0].1, &p.far_leg, far_hoof),
        ),
        (
            "patte-avant-loin",
            paint_leg(LEGS[1].0, LEGS[1].1, &p.far_leg, far_hoof),
        ),
        ("queue", paint_tail(&p)),
        (
            "patte-arriere-pres",
            paint_leg(LEGS[2].0, LEGS[2].1, &p.leg, hoof),
        ),
        (
            "patte-avant-pres",
            paint_leg(LEGS[3].0, LEGS[3].1, &p.leg, hoof),
        ),
        ("corps", paint_body(&p)),
        ("oreille-loin", paint_ear(&p, EAR_FAR.0, EAR_FAR.1, true)),
        ("tete", paint_head(&p)),
        ("oreille-pres", paint_ear(&p, EAR_NEAR.0, EAR_NEAR.1, false)),
        ("toupet", paint_topknot(&p)),
        ("oeil", paint_eye()),
        ("oeil-ferme", paint_closed_eye()),
    ];
    let mut preview = Layer::new();
    for (name, layer) in &parts {
        layer.save(&out.join(format!("{name}.png")));
        if *name != "oeil-ferme" {
            preview.stack(layer);
        }
        println!("{name}.png");
    }
    preview.save(&out.join("apercu.png"));
    let _ = p.leg;
}
