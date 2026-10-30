//! Squelettes à morceaux peints (voir [`mouflon_core::skeleton`]).
//!
//! Au chargement, chaque morceau est décodé, recadré sur ce qu'il montre (une
//! image exportée à la taille de toute la toile est surtout transparente) et
//! réduit, avec un filtre de surface, à la finesse de l'écran. À chaque image,
//! les os prennent leur pose : les animations à clés, mêlées selon ce que fait
//! l'animal, puis les mouvements du moteur pour les os qui ont un rôle (celui
//! des dessins vectoriels, voir [`crate::animate`]). Chaque morceau se pose
//! avec son os, comme une image du rendu direct : pas de Vello.
//!
//! Les mouvements du moteur sont calculés dans le repère de la toile, comme
//! si chaque os avec un rôle était libre (le corps monte et descend, les
//! pattes restent posées au sol), puis ramenés à son parent : un squelette
//! peut ranger ses os comme il veut, les sabots ne glissent pas pour autant.

use std::sync::Arc;

use mouflon_core::{BonePose, BoneRole, PoseSignals, RUN_STRIDE, Shown, Skeleton};
use vello::kurbo::{Affine, Point, Rect, Vec2};

use crate::animate::{about, ease, leg_phase, root_transform, rotate_about, scale_about, stepping};
use crate::display::{Bitmap, Command, DisplayList};
use crate::sprites::{SpriteError, coverage, decode_png_rgba};

/// Pixels d'image par point affiché : deux (écrans Retina), et un quart de
/// plus pour les morceaux étirés et tournés.
const PIXELS_PER_POINT: f64 = 2.5;

#[derive(Debug, thiserror::Error)]
pub enum SkeletonArtError {
    #[error("{image} : {source}")]
    Image { image: String, source: SpriteError },
    #[error("le squelette ne montre rien : ses morceaux sont vides")]
    Empty,
}

/// Un morceau prêt à poser.
#[derive(Clone, Debug)]
struct PieceArt {
    bone: usize,
    bitmap: Bitmap,
    /// Des pixels de l'image vers ceux de la toile.
    placement: Affine,
    shown: Shown,
}

/// Un squelette prêt à dessiner.
#[derive(Clone, Debug)]
pub struct SkeletonArt {
    pub skeleton: Arc<Skeleton>,
    pieces: Vec<PieceArt>,
    /// Ce qui est dessiné, dans la toile (pixels).
    pub bounds: Rect,
    /// Ce que montrent les morceaux posés sur chaque os, dans la toile.
    own: Vec<Option<Rect>>,
    /// Ce que montrent un os et tous ceux qu'il porte.
    carried: Vec<Option<Rect>>,
    /// Des paupières peintes : les yeux ne s'écrasent pas pour se fermer.
    painted_lids: bool,
    /// Rang de chaque os parmi ceux de son rôle (oreilles, pattes…).
    rank: Vec<usize>,
    /// Pour une oreille : 1 si elle part vers l'avant de l'animal (la
    /// droite de la toile), -1 vers l'arrière ; elle tombe en tournant de
    /// ce côté-là.
    side: Vec<f64>,
}

/// Les transformations d'une image.
#[derive(Clone, Debug)]
pub struct SkeletonPose {
    /// L'animal entier (écrasement, inclinaison, retournement, culbute).
    pub root: Affine,
    /// Chaque os, de la toile au repos vers la toile en mouvement.
    pub bones: Vec<Affine>,
}

/// Pixels RGBA (non prémultipliés) d'une image décodée.
struct Decoded {
    width: usize,
    height: usize,
    rgba: Vec<u8>,
}

impl Decoded {
    /// Le rectangle des pixels qui se voient, s'il y en a.
    fn visible(&self) -> Option<(usize, usize, usize, usize)> {
        let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
        for y in 0..self.height {
            let row = &self.rgba[y * self.width * 4..(y + 1) * self.width * 4];
            let mut any = false;
            for (x, p) in row.as_chunks::<4>().0.iter().enumerate() {
                if p[3] > 0 {
                    x0 = x0.min(x);
                    x1 = x1.max(x + 1);
                    any = true;
                }
            }
            if any {
                y0 = y0.min(y);
                y1 = y1.max(y + 1);
            }
        }
        (x0 < x1 && y0 < y1).then_some((x0, y0, x1, y1))
    }
}

/// La partie `(x0, y0, x1, y1)` de l'image, réduite à `out_w` × `out_h` par
/// un filtre de surface, couleurs pondérées par leur opacité (pas de liseré
/// sombre) : RGBA non prémultiplié.
fn reduce(image: &Decoded, area: (usize, usize, usize, usize), out_w: u32, out_h: u32) -> Vec<u8> {
    let (x0, y0, x1, y1) = area;
    let (w, h) = (x1 - x0, y1 - y0);
    let columns = coverage(w as u32, out_w);
    let rows = coverage(h as u32, out_h);
    let (ow, oh) = (out_w as usize, out_h as usize);
    // Passe horizontale, en prémultiplié.
    let mut wide = vec![[0.0f32; 4]; ow * h];
    for y in 0..h {
        let start = ((y0 + y) * image.width + x0) * 4;
        let row = image.rgba[start..start + w * 4].as_chunks::<4>().0;
        for (i, taps) in columns.iter().enumerate() {
            let mut acc = [0.0f32; 4];
            for &(sx, weight) in taps {
                let p = row[sx];
                let a = f32::from(p[3]) / 255.0 * weight;
                acc[0] += f32::from(p[0]) * a;
                acc[1] += f32::from(p[1]) * a;
                acc[2] += f32::from(p[2]) * a;
                acc[3] += a;
            }
            wide[y * ow + i] = acc;
        }
    }
    // Passe verticale, puis retour au non prémultiplié.
    let mut out = Vec::with_capacity(ow * oh * 4);
    for taps in &rows {
        for i in 0..ow {
            let mut acc = [0.0f32; 4];
            for &(sy, weight) in taps {
                let p = wide[sy * ow + i];
                for c in 0..4 {
                    acc[c] += p[c] * weight;
                }
            }
            let a = acc[3];
            if a <= 1e-6 {
                out.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            let channel = |v: f32| (v / a).round().clamp(0.0, 255.0) as u8;
            out.extend_from_slice(&[
                channel(acc[0]),
                channel(acc[1]),
                channel(acc[2]),
                (a * 255.0).round().clamp(0.0, 255.0) as u8,
            ]);
        }
    }
    out
}

fn union(a: Option<Rect>, b: Rect) -> Rect {
    a.map_or(b, |a| a.union(b))
}

impl SkeletonArt {
    /// Prépare `skeleton`, affiché à `height` points de haut.
    pub fn load(skeleton: Arc<Skeleton>, height: f32) -> Result<Self, SkeletonArtError> {
        // Décodés et recadrés : ce que chaque morceau montre, dans la toile.
        let mut decoded = Vec::with_capacity(skeleton.pieces.len());
        let mut bounds: Option<Rect> = None;
        for piece in &skeleton.pieces {
            let image_error = |source| SkeletonArtError::Image {
                image: piece.image.clone(),
                source,
            };
            let bytes = piece.png.bytes().map_err(|e| image_error(e.into()))?;
            let (width, height, rgba) = decode_png_rgba(&bytes).map_err(image_error)?;
            let image = Decoded {
                width: width as usize,
                height: height as usize,
                rgba,
            };
            let visible = image.visible();
            if let Some((x0, y0, x1, y1)) = visible {
                let at = Point::new(f64::from(piece.at.x), f64::from(piece.at.y));
                let rect = Rect::new(x0 as f64, y0 as f64, x1 as f64, y1 as f64) + at.to_vec2();
                bounds = Some(union(bounds, rect));
            }
            decoded.push((image, visible));
        }
        let bounds = bounds.ok_or(SkeletonArtError::Empty)?;
        // Points par pixel de la toile, puis pixels d'image par pixel de la
        // toile : jamais agrandi, réduit s'il est plus fin que l'écran.
        let k = f64::from(height) / bounds.height().max(1.0);
        let ratio = (k * PIXELS_PER_POINT).min(1.0);
        let bones = skeleton.bones.len();
        let mut own: Vec<Option<Rect>> = vec![None; bones];
        let mut pieces = Vec::with_capacity(decoded.len());
        for (piece, (image, visible)) in skeleton.pieces.iter().zip(&decoded) {
            let Some(area) = *visible else {
                continue;
            };
            let (x0, y0, x1, y1) = area;
            let (w, h) = (x1 - x0, y1 - y0);
            let out_w = ((w as f64 * ratio).ceil() as u32).max(1);
            let out_h = ((h as f64 * ratio).ceil() as u32).max(1);
            let rgba = if out_w as usize == w && out_h as usize == h {
                let mut rgba = Vec::with_capacity(w * h * 4);
                for y in y0..y1 {
                    let start = (y * image.width + x0) * 4;
                    rgba.extend_from_slice(&image.rgba[start..start + w * 4]);
                }
                rgba
            } else {
                reduce(image, area, out_w, out_h)
            };
            let origin = Vec2::new(
                f64::from(piece.at.x) + x0 as f64,
                f64::from(piece.at.y) + y0 as f64,
            );
            let placement = Affine::translate(origin)
                * Affine::scale_non_uniform(
                    w as f64 / f64::from(out_w),
                    h as f64 / f64::from(out_h),
                );
            if let Some(slot) = own.get_mut(piece.bone) {
                let rect = Rect::from_origin_size(origin.to_point(), (w as f64, h as f64));
                *slot = Some(union(*slot, rect));
            }
            pieces.push(PieceArt {
                bone: piece.bone,
                bitmap: Bitmap::from_rgba(out_w, out_h, rgba),
                placement,
                shown: piece.shown,
            });
        }
        // Ce que porte chaque os : les enfants viennent après leur parent.
        let mut carried = own.clone();
        for i in (0..bones).rev() {
            if let (Some(parent), Some(rect)) = (skeleton.bones[i].parent, carried[i]) {
                carried[parent] = Some(union(carried[parent], rect));
            }
        }
        let mut rank = vec![0; bones];
        let mut seen: Vec<BoneRole> = Vec::new();
        for (i, bone) in skeleton.bones.iter().enumerate() {
            if let Some(role) = bone.role {
                rank[i] = seen.iter().filter(|r| **r == role).count();
                seen.push(role);
            }
        }
        let side = skeleton
            .bones
            .iter()
            .zip(&carried)
            .map(|(bone, rect)| match rect {
                Some(rect) if rect.center().x < f64::from(bone.at.x) => -1.0,
                _ => 1.0,
            })
            .collect();
        let painted_lids = skeleton.pieces.iter().any(|p| p.shown == Shown::EyesClosed);
        Ok(Self {
            skeleton,
            pieces,
            bounds,
            own,
            carried,
            painted_lids,
            rank,
            side,
        })
    }

    /// Octets des images des morceaux.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        self.pieces
            .iter()
            .map(|p| p.bitmap.width as usize * p.bitmap.height as usize * 4)
            .sum()
    }

    /// Le point d'appui, dans la toile.
    #[must_use]
    pub fn ground(&self) -> Point {
        let g = self.skeleton.ground;
        Point::new(f64::from(g.x), f64::from(g.y))
    }

    /// L'os qui a ce rôle (le premier), s'il y en a un.
    fn role(&self, wanted: BoneRole) -> Option<usize> {
        self.skeleton
            .bones
            .iter()
            .position(|b| b.role == Some(wanted))
    }

    /// La pose d'une image. `stride` : distance parcourue pendant un cycle de
    /// marche, en pixels de la toile (0 : 0,9 fois sa hauteur).
    #[must_use]
    pub fn pose(&self, s: &PoseSignals, facing: f32, stride: f64) -> SkeletonPose {
        let sk = &*self.skeleton;
        let count = sk.bones.len();
        let h = self.bounds.height().max(1.0);
        let u = h / 100.0;
        let g = self.ground();

        // --- Animations à clés : le calme, puis ce que fait l'animal par-dessus.
        let mut clips = vec![BonePose::default(); count];
        let m = &sk.motions;
        let on_clock = |clip: usize| s.clock / sk.clips[clip].length;
        let on_gait = s.gait_phase / std::f32::consts::TAU;
        if let Some(idle) = m.idle {
            sk.clips[idle].blend_into(on_clock(idle), 1.0, &mut clips);
        }
        for (clip, weight, gait) in [
            (m.sit, s.sit, false),
            (m.sleep, s.sleep, false),
            (m.graze, s.graze, false),
            (m.walk, s.walk, true),
            (m.run, s.run, true),
            (m.air, s.air, false),
            (m.dangle, s.dangle, false),
        ] {
            if let Some(clip) = clip {
                let t = if gait { on_gait } else { on_clock(clip) };
                sk.clips[clip].blend_into(t, weight, &mut clips);
            }
        }
        // Ce que les animations jouent, le moteur ne le refait pas.
        let mut p = *s;
        let skip = |clip: Option<usize>, signal: &mut f32| {
            if clip.is_some() {
                *signal = 0.0;
            }
        };
        skip(m.walk, &mut p.walk);
        skip(m.run, &mut p.run);
        skip(m.sit, &mut p.sit);
        skip(m.sleep, &mut p.sleep);
        skip(m.graze, &mut p.graze);
        skip(m.air, &mut p.air);
        skip(m.dangle, &mut p.dangle);

        // --- L'animal entier.
        let hold_x = self
            .role(BoneRole::Body)
            .map_or(g.x, |b| f64::from(sk.bones[b].at.x));
        let hold = Point::new(hold_x, self.bounds.min_y() + h * 0.12);
        let root = root_transform(s, facing, g, h, hold);

        // --- Les os avec un rôle, comme s'ils étaient libres (repère de la toile).
        let engine = self.engine(&p, s, stride, g, h, u);
        let mut free = vec![Affine::IDENTITY; count];
        let mut bones = vec![Affine::IDENTITY; count];
        for (i, bone) in sk.bones.iter().enumerate() {
            let parent_free = bone.parent.map_or(Affine::IDENTITY, |b| free[b]);
            let at = Point::new(f64::from(bone.at.x), f64::from(bone.at.y));
            free[i] = match bone.role {
                None => parent_free,
                Some(role) => engine.bone(role, at, self.rank[i], parent_free, self, i),
            };
            // Ramené à son parent, puis l'animation à clés, autour de l'articulation.
            let engine_local = parent_free.inverse() * free[i];
            let clip = clips[i];
            let keyed = about(
                at,
                Affine::translate((f64::from(clip.moves.x), f64::from(clip.moves.y)))
                    * Affine::rotate(f64::from(clip.rotate))
                    * Affine::scale_non_uniform(f64::from(clip.scale.x), f64::from(clip.scale.y)),
            );
            let parent = bone.parent.map_or(Affine::IDENTITY, |b| bones[b]);
            bones[i] = parent * engine_local * keyed;
        }
        SkeletonPose { root, bones }
    }

    /// Les mouvements du moteur pour cette image (voir [`Engine::bone`]).
    fn engine(
        &self,
        p: &PoseSignals,
        s: &PoseSignals,
        stride: f64,
        g: Point,
        h: f64,
        u: f64,
    ) -> Engine {
        let f = |v: f32| f64::from(v);
        let phase = f(p.gait_phase);
        let (walk, run, sit, sleep, air) = (f(p.walk), f(p.run), f(p.sit), f(p.sleep), f(p.air));
        let graze = ease(0.0, 1.0, f(p.graze));
        let body = self
            .role(BoneRole::Body)
            .and_then(|b| self.own[b])
            .unwrap_or(self.bounds);
        let clearance = (g.y - body.max_y() - 0.6 * u).max(0.0);
        let front_down = ease(0.0, 0.65, sit);
        let back_down = ease(0.35, 1.0, sit);
        let drop_front = clearance * front_down;
        let drop_back = clearance * back_down;
        let half_length = (body.width() * 0.5).max(u);
        let pitch = ((drop_front - drop_back) / (2.0 * half_length)).atan();
        let bob = -(walk * (phase * 2.0).sin().abs() + run * 2.6 * phase.sin().abs()) * u;
        let rock = run * 0.07 * phase.cos();
        let body_t = Affine::translate((0.0, bob + 0.5 * (drop_front + drop_back)))
            * rotate_about(body.center(), pitch + rock + 0.05 * graze)
            * scale_about(g, 1.0 + 0.04 * sleep, 1.0 - 0.05 * sleep);
        // Les pattes : le balayage de l'appui couvre exactement la distance
        // que le corps parcourt pendant ce temps.
        let stride =
            if stride > 0.0 { stride } else { h * 0.9 } * (1.0 + f64::from(RUN_STRIDE) * run);
        let duty = 0.65 + (0.38 - 0.65) * run;
        let rise = f(s.rise);
        Engine {
            signals: *p,
            all: *s,
            g,
            u,
            h,
            phase,
            bob,
            drop_front,
            drop_back,
            graze,
            body_t,
            stride,
            duty,
            gait: walk.max(run),
            tuck: air * ease(0.0, 1.0, rise),
            reach: air * ease(0.0, 1.0, -rise),
            splay: f(s.squash).clamp(-0.15, 0.16).max(0.0) * 3.0,
            front_down,
            back_down,
        }
    }
}

/// Ce que le moteur fait des os pour une image : les grandeurs partagées
/// par tous, calculées une fois.
struct Engine {
    /// Les signaux, sans ce que jouent les animations à clés.
    signals: PoseSignals,
    /// Tous les signaux (le regard, les clignements, la respiration…).
    all: PoseSignals,
    g: Point,
    u: f64,
    h: f64,
    phase: f64,
    bob: f64,
    drop_front: f64,
    drop_back: f64,
    graze: f64,
    body_t: Affine,
    stride: f64,
    duty: f64,
    gait: f64,
    tuck: f64,
    reach: f64,
    splay: f64,
    front_down: f64,
    back_down: f64,
}

impl Engine {
    /// Où le moteur met l'os `index` (articulation `at`, `rank`-ième de son
    /// rôle), libre dans la toile ; `parent` : où est son parent.
    fn bone(
        &self,
        role: BoneRole,
        at: Point,
        rank: usize,
        parent: Affine,
        art: &SkeletonArt,
        index: usize,
    ) -> Affine {
        let f = |v: f32| f64::from(v);
        let (p, s) = (&self.signals, &self.all);
        let (sleep, air, dangle) = (f(p.sleep), f(p.air), f(p.dangle));
        let breath = f(s.breath);
        let look = Vec2::new(f(s.look.x), f(s.look.y));
        let u = self.u;
        match role {
            BoneRole::Body => self.body_t,
            BoneRole::Head => {
                let rise = f(s.rise);
                let mut angle = look.y * 0.16 + 0.42 * sleep - 0.06 * air
                    + f(s.lean) * 0.3
                    + (self.phase * 2.0).sin() * 0.035 * f(p.walk)
                    - 0.12 * dangle
                    - 0.12 * air * ease(0.0, 1.0, rise)
                    + 0.1 * air * ease(0.0, 1.0, -rise);
                let mut shift = Vec2::new(
                    look.x * u * 1.4,
                    look.y * u + self.bob * 1.15 + self.drop_front + sleep * self.h * 0.06,
                );
                if self.graze > 0.0
                    && let Some(head) = art.carried[index]
                {
                    // Bouche au ras du sol, juste devant les pattes de devant.
                    let bow = 0.85 + 0.05 * f(p.nibble);
                    let turn = rotate_about(at, bow);
                    let lowest = [
                        Point::new(head.max_x(), head.max_y()),
                        Point::new(head.max_x(), head.min_y()),
                        Point::new(head.min_x(), head.max_y()),
                    ]
                    .into_iter()
                    .map(|corner| (turn * corner).y)
                    .fold(f64::NEG_INFINITY, f64::max);
                    let lower = (self.g.y - 1.5 * u - lowest).max(0.0);
                    angle += self.graze * bow;
                    shift += Vec2::new(
                        self.graze * 1.5 * u,
                        self.graze * (lower + 0.5 * u * f(p.nibble)),
                    );
                }
                Affine::translate(shift) * rotate_about(at, angle)
            }
            BoneRole::Ear => {
                // La première oreille (derrière) bouge un peu moins ; elles
                // tombent vers le bas, de quelque côté qu'elles partent.
                let k = if rank == 0 { 0.8 } else { 1.0 } * art.side[index];
                let droop = 0.3 * sleep + 0.15 * dangle + 0.18 * self.graze;
                parent * rotate_about(at, k * (f(s.ear) + 0.05 * breath + droop))
            }
            BoneRole::Eye => {
                if art.painted_lids {
                    parent
                } else {
                    let open = (1.0 - 0.94 * f(s.blink)).max(0.06);
                    parent * scale_about(at, 1.0, open)
                }
            }
            BoneRole::Pupil => parent * Affine::translate((look.x * u * 2.6, look.y * u * 2.0)),
            BoneRole::Tail => {
                Affine::translate((0.0, self.bob + self.drop_back))
                    * rotate_about(at, f(s.tail) - 0.2 * dangle)
            }
            BoneRole::Leg { front, near } => {
                let leg_len = (self.g.y - at.y).max(u * 8.0);
                let amplitude = (self.stride * self.duty / (2.0 * leg_len)).clamp(0.1, 0.75);
                let lift_amount = (2.2 + 3.0 * f(p.run)) * u * self.gait;
                let cycle = self.phase + leg_phase(front, near, f(p.run));
                let (step, raised, support) = stepping(cycle, self.duty, amplitude);
                let mut angle = self.gait * step;
                // En l'air : repliées en montant, tendues vers le sol en descendant.
                angle += if front {
                    -0.15 * air + self.tuck - 0.3 * self.reach
                } else {
                    0.15 * air + 0.55 * self.tuck - 0.12 * self.reach
                };
                angle += dangle * (if front { -0.1 } else { 0.12 } + f(s.swing) * 0.4);
                // À l'atterrissage, elles s'écartent ; couché, elles se replient.
                angle += if front { -self.splay } else { self.splay };
                angle += if front {
                    1.3 * self.front_down
                } else {
                    -1.25 * self.back_down
                };
                let lift = raised * lift_amount;
                let drop = if front {
                    self.drop_front
                } else {
                    self.drop_back
                };
                // En appui, la patte penchée s'allonge juste assez pour rester au sol.
                let planted = self.gait * support * (1.0 / (self.gait * step).cos().max(0.5) - 1.0);
                let stretch = 1.0 + planted + 0.07 * dangle + 0.04 * air;
                Affine::translate((0.0, drop - lift))
                    * rotate_about(at, angle)
                    * scale_about(at, 1.0, stretch)
            }
        }
    }
}

/// Ajoute à `out` les morceaux du squelette, posés. `world` : du repère du
/// monde vers les pixels de la fenêtre ; `ground` : le point d'appui de
/// l'animal dans le monde ; `k` : points par pixel de la toile.
pub fn paint_skeleton(
    art: &SkeletonArt,
    pose: &SkeletonPose,
    blink: f32,
    world: Affine,
    ground: Point,
    k: f64,
    out: &mut DisplayList,
) {
    let base = world
        * Affine::translate(ground.to_vec2())
        * Affine::scale(k)
        * Affine::translate(-art.ground().to_vec2())
        * pose.root;
    let closed = blink >= 0.5;
    for piece in &art.pieces {
        match piece.shown {
            Shown::EyesOpen if closed => continue,
            Shown::EyesClosed if !closed => continue,
            _ => {}
        }
        let Some(bone) = pose.bones.get(piece.bone) else {
            continue;
        };
        out.push(Command::Image {
            bitmap: piece.bitmap.clone(),
            transform: base * *bone * piece.placement,
            alpha: 1.0,
            smooth: true,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU as CYCLE;

    use super::*;
    use mouflon_core::{PngData, parse_skeleton};

    /// Un PNG plein de `color` sur `w` × `h`, posé dans une toile plus grande
    /// (transparente ailleurs) à `(x, y)`.
    fn png(canvas: (u32, u32), rect: (u32, u32, u32, u32), color: [u8; 4]) -> PngData {
        let (cw, ch) = canvas;
        let mut rgba = vec![0u8; (cw * ch * 4) as usize];
        let (x, y, w, h) = rect;
        for yy in y..y + h {
            for xx in x..x + w {
                let i = ((yy * cw + xx) * 4) as usize;
                rgba[i..i + 4].copy_from_slice(&color);
            }
        }
        let mut out = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut out, cw, ch);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&rgba).unwrap();
        }
        PngData::from(out)
    }

    const TOML: &str = r#"
        size = [400, 300]
        ground = [200, 280]

        [[bone]]
        name = "racine"
        at = [200, 280]

        [[bone]]
        name = "corps"
        parent = "racine"
        at = [200, 180]
        role = "body"

        [[bone]]
        name = "tete"
        parent = "corps"
        at = [280, 150]
        role = "head"

        [[bone]]
        name = "patte"
        parent = "corps"
        at = [260, 210]
        role = "leg-front-near"

        [[bone]]
        name = "oeil"
        parent = "tete"
        at = [300, 130]
        role = "eye"

        [[piece]]
        bone = "corps"
        image = "corps.png"

        [[piece]]
        bone = "tete"
        image = "tete.png"

        [[piece]]
        bone = "patte"
        image = "patte.png"
    "#;

    fn art(toml: &str) -> SkeletonArt {
        let skeleton = parse_skeleton(toml, |name| {
            Ok(match name {
                // Le corps : de 120 à 280 en largeur, de 140 à 210 en hauteur.
                "corps.png" => png((400, 300), (120, 140, 160, 70), [240, 240, 240, 255]),
                "tete.png" => png((400, 300), (270, 110, 50, 50), [90, 80, 80, 255]),
                // La patte : de la hanche (260, 210) au sol (280).
                "patte.png" => png((400, 300), (255, 200, 10, 80), [60, 50, 50, 255]),
                // Une paupière fermée, sur l'œil.
                "ferme.png" => png((400, 300), (295, 128, 12, 4), [60, 50, 50, 255]),
                _ => png((10, 10), (0, 0, 1, 1), [0, 0, 0, 255]),
            })
        })
        .unwrap();
        SkeletonArt::load(Arc::new(skeleton), 60.0).unwrap()
    }

    #[test]
    fn pieces_are_cropped_and_reduced_to_the_screen() {
        let art = art(TOML);
        // De la tête (110) au sol (280) : 170 pixels pour 60 points.
        assert_eq!(art.bounds, Rect::new(120.0, 110.0, 320.0, 280.0));
        assert_eq!(art.pieces.len(), 3);
        // 60 / 170 points par pixel, 2,5 pixels par point : un peu réduit.
        let body = &art.pieces[0];
        assert_eq!((body.bitmap.width, body.bitmap.height), (142, 62));
        // Son coin se pose au coin de ce qu'il montre dans la toile.
        let corner = body.placement * Point::ZERO;
        assert!((corner - Point::new(120.0, 140.0)).hypot() < 1e-9);
        let far = body.placement * Point::new(142.0, 62.0);
        assert!((far - Point::new(280.0, 210.0)).hypot() < 1e-9);
    }

    #[test]
    fn at_rest_nothing_moves() {
        let art = art(TOML);
        let pose = art.pose(&PoseSignals::default(), 1.0, 0.0);
        assert!((pose.root * art.ground() - art.ground()).hypot() < 1e-9);
        for bone in &pose.bones {
            assert!((*bone * Point::new(250.0, 200.0) - Point::new(250.0, 200.0)).hypot() < 1e-6);
        }
    }

    #[test]
    fn walking_keeps_the_hooves_on_the_ground_and_swings_the_leg() {
        let art = art(TOML);
        let foot = Point::new(260.0, 280.0);
        let mut lowest = f64::NEG_INFINITY;
        let mut angles = Vec::new();
        for i in 0..24 {
            let signals = PoseSignals {
                walk: 1.0,
                gait_phase: (i as f64 / 24.0 * CYCLE) as f32,
                ..PoseSignals::default()
            };
            let pose = art.pose(&signals, 1.0, 0.0);
            let leg = pose.bones[3];
            let moved = leg * foot;
            lowest = lowest.max(moved.y);
            let hip = leg * Point::new(260.0, 210.0);
            angles.push((moved.x - hip.x).atan2(moved.y - hip.y));
        }
        // Le sabot ne passe jamais sous le sol, et la patte balaie d'avant en arrière.
        assert!(lowest <= 280.5, "{lowest}");
        let swing = angles.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - angles.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(swing > 0.2, "{swing}");
    }

    #[test]
    fn the_head_follows_the_body_and_bows_to_graze() {
        let art = art(TOML);
        let signals = PoseSignals {
            graze: 1.0,
            ..PoseSignals::default()
        };
        let pose = art.pose(&signals, 1.0, 0.0);
        // Le bas du museau descend près du sol.
        let muzzle = pose.bones[2] * Point::new(320.0, 160.0);
        assert!(muzzle.y > 240.0, "{muzzle:?}");
        // L'œil suit la tête.
        let eye = pose.bones[4] * Point::new(300.0, 130.0);
        let head = pose.bones[2] * Point::new(300.0, 130.0);
        assert!((eye - head).hypot() < 1e-6);
    }

    #[test]
    fn keyed_animations_replace_the_engine_for_their_motion() {
        let toml = format!(
            "{TOML}
            [[clip]]
            name = \"marche\"
            [clip.bones.patte]
            rotate = [[0, 30], [0.5, -30], [1, 30]]

            [motions]
            walk = \"marche\"
            "
        );
        let art = art(&toml);
        let at = |phase: f64| {
            let signals = PoseSignals {
                walk: 1.0,
                gait_phase: (phase * CYCLE) as f32,
                ..PoseSignals::default()
            };
            let pose = art.pose(&signals, 1.0, 0.0);
            let foot = pose.bones[3] * Point::new(260.0, 280.0);
            let hip = pose.bones[3] * Point::new(260.0, 210.0);
            (foot.x - hip.x).atan2(foot.y - hip.y)
        };
        // 30° dans le sens horaire au départ : le sabot part vers l'arrière.
        assert!(
            (at(0.0).to_degrees() + 30.0).abs() < 0.5,
            "{}",
            at(0.0).to_degrees()
        );
        assert!(
            (at(0.5).to_degrees() - 30.0).abs() < 0.5,
            "{}",
            at(0.5).to_degrees()
        );
        // Le corps ne monte plus et ne descend plus de lui-même.
        let signals = PoseSignals {
            walk: 1.0,
            gait_phase: 0.8,
            ..PoseSignals::default()
        };
        let pose = art.pose(&signals, 1.0, 0.0);
        assert!(
            (pose.bones[1] * Point::new(200.0, 180.0) - Point::new(200.0, 180.0)).hypot() < 1e-6
        );
    }

    #[test]
    fn painted_lids_replace_squashed_eyes() {
        let lids = TOML.replace(
            "[[piece]]\n        bone = \"patte\"",
            "[[piece]]\n        bone = \"oeil\"\n        image = \"ferme.png\"\n        eyes = \"closed\"\n\n        [[piece]]\n        bone = \"patte\"",
        );
        let art = art(&lids);
        assert!(art.painted_lids);
        let signals = PoseSignals {
            blink: 1.0,
            ..PoseSignals::default()
        };
        let pose = art.pose(&signals, 1.0, 0.0);
        // L'œil ne s'écrase pas : la paupière peinte se montre.
        let top = pose.bones[4] * Point::new(300.0, 120.0);
        assert!((top.y - 120.0).abs() < 1e-6);
        let mut out = DisplayList::new(100, 100);
        paint_skeleton(
            &art,
            &pose,
            1.0,
            Affine::IDENTITY,
            Point::new(50.0, 90.0),
            0.3,
            &mut out,
        );
        assert_eq!(out.commands.len(), 4);
        let mut open = DisplayList::new(100, 100);
        paint_skeleton(
            &art,
            &pose,
            0.0,
            Affine::IDENTITY,
            Point::new(50.0, 90.0),
            0.3,
            &mut open,
        );
        assert_eq!(open.commands.len(), 3);
    }
}
