//! La scène : sur chaque écran, une seule fenêtre transparente, créée une
//! fois et jamais déplacée, où se posent tous les animaux et les soucoupes.
//!
//! Chaque animal devient des morceaux d'image (voir
//! [`mouflon_render::pieces`]), refaits seulement quand ce qu'on voit de
//! lui change. Sur Mac, chaque morceau est un calque Core Animation dont
//! l'image est partagée par tous ceux qui la montrent : à chaque image,
//! Mouflon ne change que les propriétés des calques qui bougent, en une
//! seule transaction, et le compositeur du système fait le reste. Ailleurs
//! (ou si les calques manquent), les morceaux se dessinent sur le GPU dans
//! la fenêtre de l'écran.
//!
//! Les fenêtres des écrans sont faites sur le fil principal (voir
//! [`crate::screens`]) ; la scène reçoit de chacune la racine de ses calques
//! (ou la fenêtre elle-même, pour le GPU) et n'y touche plus qu'à travers
//! elle : elle peut ainsi vivre sur le fil de l'affichage.

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mouflon_core::{PetView, Rect, SaucerView, Vec2};
use mouflon_platform as platform;
use mouflon_render::display::{Bitmap, Command, DisplayList};
use mouflon_render::kurbo::Affine;
use mouflon_render::peniko::Mix;
use mouflon_render::pieces::{Pieces, flatten};
use mouflon_render::{Frame, PetArt, SpriteKey, Style, paint_pet};
use winit::window::Window;

use crate::gpu::{Gpu, Job, PetSurface};

/// Images décompressées gardées pour les calques : au-delà, les moins
/// récemment montrées sont oubliées (et refaites si elles reviennent).
const IMAGE_BUDGET: usize = 96 << 20;
/// Temps de décompression d'images nouvelles par image, au plus : le
/// reste attend l'image suivante (le calque garde la sienne en attendant).
/// La première de chaque image passe toujours.
const DECODE_BUDGET: Duration = Duration::from_millis(3);

/// Une table rangée par identifiant (des entiers) : chaque image, chaque
/// animal y passe à chaque image de l'écran, avec un hachage rapide.
type IdMap<K, V> = HashMap<K, V, BuildHasherDefault<mouflon_core::IdHasher>>;

/// Ce que montre la scène.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Entity {
    Pet(u32),
    Saucer(u32),
}

/// Ce qui identifie le dessin d'un animal en images : le même, il n'est
/// pas refait (il ne fait que se déplacer).
#[derive(Clone, Copy, Debug, PartialEq)]
struct ContentKey {
    sprite: SpriteKey,
    art: usize,
    scale: f64,
    dark: bool,
}

/// Une chose montrée à cette image : ses morceaux, dans le repère de sa
/// toile (pixels), et où se trouve sa toile.
#[derive(Debug, Default)]
struct Shown {
    pieces: Pieces,
    /// Coin haut gauche de sa toile, en pixels de l'écran (entiers) dans le
    /// repère commun, à l'échelle `scale`.
    origin: (f64, f64),
    scale: f64,
    /// Taille de sa toile, en points.
    size: (f64, f64),
    key: Option<ContentKey>,
    /// Change à chaque fois que ses morceaux sont refaits.
    generation: u64,
    /// Dernière image où elle a été montrée.
    seen: u64,
}

impl Shown {
    /// Sa toile, en points, dans le repère commun.
    fn rect(&self) -> Rect {
        Rect::new(
            (self.origin.0 / self.scale) as f32,
            (self.origin.1 / self.scale) as f32,
            self.size.0 as f32,
            self.size.1 as f32,
        )
    }
}

/// Une image décompressée, prête pour les calques.
#[derive(Debug)]
struct Cached {
    image: platform::SceneImage,
    used: u64,
}

/// Les images des calques, partagées : une par image d'animal (ou par
/// image cuite), décompressée à sa première apparition.
#[derive(Debug, Default)]
struct Images {
    map: IdMap<u64, Cached>,
    bytes: usize,
    frame: u64,
    spent: Duration,
    /// Décompressées et oubliées depuis le dernier relevé (journal des essais).
    decoded: u32,
    evicted: u32,
}

impl Images {
    /// L'image de `bitmap`, décompressée si besoin (et si le temps de
    /// l'image le permet encore).
    fn get(&mut self, bitmap: &Bitmap) -> Option<&platform::SceneImage> {
        let id = bitmap.id();
        if !self.map.contains_key(&id) {
            if self.spent >= DECODE_BUDGET {
                return None;
            }
            let started = Instant::now();
            let (width, height) = bitmap.texture_size();
            let mut pixels = bitmap.texture_rgba();
            // RVBA → BVRA : le format que le compositeur prend tel quel.
            for px in pixels.as_chunks_mut::<4>().0 {
                px.swap(0, 2);
            }
            let image = platform::SceneImage::from_bgra(width, height, &pixels)?;
            self.spent += started.elapsed();
            self.bytes += image.byte_size();
            self.decoded += 1;
            self.map.insert(id, Cached { image, used: 0 });
        }
        let cached = self.map.get_mut(&id)?;
        cached.used = self.frame;
        Some(&cached.image)
    }

    /// L'image de `bitmap`, décompressée tout de suite si besoin (avant la
    /// première image : pas de budget).
    fn prepare(&mut self, bitmap: &Bitmap) {
        let spent = std::mem::take(&mut self.spent);
        let _ = self.get(bitmap);
        self.spent = spent;
    }

    fn begin(&mut self, frame: u64) {
        self.frame = frame;
        self.spent = Duration::ZERO;
    }

    /// Oublie les moins récemment montrées au-delà du budget (jamais celles
    /// de cette image : les calques les gardent de toute façon).
    fn trim(&mut self) {
        if self.bytes <= IMAGE_BUDGET {
            return;
        }
        let mut old: Vec<(u64, u64, usize)> = self
            .map
            .iter()
            .filter(|(_, c)| c.used < self.frame)
            .map(|(&id, c)| (c.used, id, c.image.byte_size()))
            .collect();
        old.sort_unstable();
        for (_, id, bytes) in old {
            if self.bytes <= IMAGE_BUDGET * 3 / 4 {
                break;
            }
            self.map.remove(&id);
            self.bytes -= bytes;
            self.evicted += 1;
        }
    }

    fn clear(&mut self) {
        self.map.clear();
        self.bytes = 0;
    }
}

/// Un calque d'image d'un morceau, et ce qu'on lui a donné en dernier.
#[derive(Debug)]
struct Slot {
    layer: platform::SceneLayer,
    image: Option<u64>,
    size: (u32, u32),
    transform: [f64; 6],
    alpha: f32,
    smooth: Option<bool>,
}

impl Slot {
    fn new() -> Self {
        Self {
            layer: platform::SceneLayer::new(),
            image: None,
            size: (0, 0),
            transform: [0.0; 6],
            alpha: -1.0,
            smooth: None,
        }
    }
}

/// Les calques d'une chose montrée sur un écran : un calque qui la porte
/// (à la place de sa toile), un calque par groupe, un par morceau.
#[derive(Debug)]
struct Node {
    container: platform::SceneLayer,
    groups: Vec<platform::SceneLayer>,
    group_alpha: Vec<f32>,
    slots: Vec<Slot>,
    /// Groupe de chaque morceau, à la dernière construction.
    shape: Vec<u16>,
    generation: Option<u64>,
    position: Option<(f64, f64)>,
}

impl Node {
    fn new() -> Self {
        Self {
            container: platform::SceneLayer::new(),
            groups: Vec::new(),
            group_alpha: Vec::new(),
            slots: Vec::new(),
            shape: Vec::new(),
            generation: None,
            position: None,
        }
    }

    /// Refait l'arbre de ses calques pour cette suite de groupes.
    fn rebuild(&mut self, shape: &[u16], groups: usize) {
        for slot in &self.slots {
            slot.layer.remove();
        }
        for group in &self.groups {
            group.remove();
        }
        // Les calques déjà faits resservent (dans l'ordre).
        self.slots.resize_with(shape.len(), Slot::new);
        self.groups.resize_with(groups, || {
            let layer = platform::SceneLayer::new();
            layer.set_group_opacity(true);
            layer
        });
        self.group_alpha.resize(groups, -1.0);
        let mut open: u16 = 0;
        for (slot, &group) in self.slots.iter().zip(shape) {
            if group != open && group != 0 {
                self.container.add(&self.groups[usize::from(group) - 1]);
            }
            open = group;
            match group {
                0 => self.container.add(&slot.layer),
                n => self.groups[usize::from(n) - 1].add(&slot.layer),
            }
        }
        self.shape = shape.to_vec();
    }

    /// Met ses calques à jour : sa place sur l'écran dont le coin haut
    /// gauche est `screen` (points), et ses morceaux s'ils ont changé.
    /// `false` si une image n'a pas pu être prête à temps (on réessaie à
    /// l'image suivante).
    ///
    /// Les calques sont dans le repère de la vue de la fenêtre, qu'AppKit
    /// retourne (winit la déclare `isFlipped`) : origine en haut à gauche, y
    /// vers le bas — celui de Mouflon et de ses toiles —, et une image y
    /// est montrée à l'endroit. Le calque qui porte la chose est placé au
    /// coin haut gauche de sa toile ; chaque morceau s'y pose comme sur la
    /// toile.
    fn update(&mut self, shown: &Shown, screen: (f64, f64), images: &mut Images) -> bool {
        let position = (
            shown.origin.0 / shown.scale - screen.0,
            shown.origin.1 / shown.scale - screen.1,
        );
        if self.position != Some(position) {
            self.container.set_position(position.0, position.1);
            self.position = Some(position);
        }
        if self.generation == Some(shown.generation) {
            return true;
        }
        let pieces = &shown.pieces;
        let shape: Vec<u16> = pieces.items.iter().map(|p| p.group).collect();
        if shape != self.shape || self.groups.len() != pieces.groups.len() {
            self.rebuild(&shape, pieces.groups.len());
        }
        for (i, layer) in self.groups.iter().enumerate() {
            let alpha = pieces.groups[i];
            if self.group_alpha[i] != alpha {
                layer.set_opacity(alpha);
                self.group_alpha[i] = alpha;
            }
        }
        // Des pixels de la toile aux points.
        let to_points = Affine::scale(1.0 / shown.scale);
        let mut complete = true;
        for (slot, piece) in self.slots.iter_mut().zip(&pieces.items) {
            let id = piece.bitmap.id();
            if slot.image != Some(id) {
                let Some(image) = images.get(&piece.bitmap) else {
                    // Il garde son image d'avant (et sa place) une image de plus.
                    complete = false;
                    continue;
                };
                slot.layer.set_image(Some(image));
                slot.image = Some(id);
            }
            let size = piece.bitmap.texture_size();
            if slot.size != size {
                slot.layer.set_size(f64::from(size.0), f64::from(size.1));
                slot.size = size;
            }
            // L'image a une bordure transparente d'un pixel (ses bords
            // tournés restent lisses) : son coin est à (-1, -1).
            let transform =
                (to_points * piece.transform * Affine::translate((-1.0, -1.0))).as_coeffs();
            if slot.transform != transform {
                slot.layer.set_transform(transform);
                slot.transform = transform;
            }
            if slot.alpha != piece.alpha {
                slot.layer.set_opacity(piece.alpha);
                slot.alpha = piece.alpha;
            }
            if slot.smooth != Some(piece.smooth) {
                slot.layer.set_smooth(piece.smooth);
                slot.smooth = Some(piece.smooth);
            }
        }
        if complete {
            self.generation = Some(shown.generation);
        }
        complete
    }
}

/// Ce que la scène sait de la fenêtre d'un écran.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenInfo {
    /// La fenêtre (une par écran, jamais déplacée : une autre a une autre clé).
    pub key: u64,
    /// Le cadre de l'écran (repère commun, points).
    pub rect: Rect,
    /// Pixels par point.
    pub scale: f64,
    /// Images par seconde que l'écran sait afficher.
    pub hz: f64,
}

/// Ce dans quoi la scène se montre sur un écran : la racine des calques
/// posée dans sa fenêtre, ou la fenêtre elle-même (GPU, sur le fil
/// principal seulement).
#[derive(Clone, Debug)]
pub enum Surface {
    Layers(platform::SceneLayer),
    Gpu(Arc<Window>),
}

/// La fenêtre d'un écran, telle que le fil principal la donne à la scène.
#[derive(Clone, Debug)]
pub struct Overlay {
    pub info: ScreenInfo,
    pub surface: Surface,
}

/// Comment la scène montre ce qu'elle porte sur un écran.
enum Backend {
    /// Des calques Core Animation (Mac).
    Layers {
        root: platform::SceneLayer,
        nodes: IdMap<Entity, Node>,
    },
    /// Le GPU : tout se redessine dans la fenêtre à chaque image.
    Gpu {
        surface: Box<PetSurface>,
        list: DisplayList,
        window: Arc<Window>,
    },
}

/// Un écran de la scène.
struct Place {
    info: ScreenInfo,
    backend: Backend,
}

impl Place {
    fn uses_layers(&self) -> bool {
        matches!(self.backend, Backend::Layers { .. })
    }
}

/// Le temps passé par la scène à une image (journal des essais).
#[derive(Clone, Copy, Debug, Default)]
pub struct Timings {
    /// Dessin et découpage en morceaux.
    pub paint: Duration,
    /// Images décompressées pour les calques.
    pub decode: Duration,
    /// Mise à jour des calques.
    pub layers: Duration,
    /// Envoi de la transaction au compositeur (ou rendu GPU).
    pub commit: Duration,
    /// Choses refaites (dessinées et découpées) et montrées.
    pub repainted: u32,
    pub shown: u32,
}

/// Toute la scène.
pub struct Scene {
    places: Vec<Place>,
    shown: IdMap<Entity, Shown>,
    /// Ce qui est parti à cette image (gardé pour ne pas réallouer).
    gone: Vec<Entity>,
    /// Ordre d'empilement : les soucoupes d'abord, puis les animaux dans
    /// l'ordre de leur arrivée.
    order: Vec<Entity>,
    images: Images,
    gpu: Option<Gpu>,
    list: DisplayList,
    frame: u64,
    timings: Timings,
}

impl Scene {
    #[must_use]
    pub fn new() -> Self {
        Self {
            places: Vec::new(),
            shown: IdMap::default(),
            gone: Vec::new(),
            order: Vec::new(),
            images: Images::default(),
            gpu: None,
            list: DisplayList::default(),
            frame: 0,
            timings: Timings::default(),
        }
    }

    /// Les écrans où elle se montre.
    pub fn screens(&self) -> impl Iterator<Item = &ScreenInfo> {
        self.places.iter().map(|place| &place.info)
    }

    /// Le GPU, s'il sert (fenêtres d'écran sans calques).
    pub fn gpu_mut(&mut self) -> Option<&mut Gpu> {
        self.gpu.as_mut()
    }

    /// Les fenêtres des écrans : celles qu'elle connaît déjà gardent leurs
    /// calques (leur échelle ou leur fréquence a pu changer), les nouvelles
    /// commencent vides, celles qui sont parties s'oublient. Renvoie de quoi
    /// le dire au journal.
    pub fn set_overlays(&mut self, overlays: Vec<Overlay>) -> Vec<String> {
        let mut old = std::mem::take(&mut self.places);
        let mut notes = Vec::new();
        for overlay in overlays {
            if let Some(i) = old.iter().position(|p| p.info.key == overlay.info.key) {
                let mut place = old.swap_remove(i);
                place.info = overlay.info;
                self.places.push(place);
                continue;
            }
            let rect = overlay.info.rect;
            match self.open(overlay) {
                Ok(place) => {
                    notes.push(format!(
                        "scène : fenêtre de l'écran {:.0}×{:.0} en {:.0},{:.0} ({})",
                        rect.w,
                        rect.h,
                        rect.x,
                        rect.y,
                        if place.uses_layers() {
                            "calques Core Animation"
                        } else {
                            "GPU"
                        }
                    ));
                    self.places.push(place);
                }
                Err(e) => log::error!("fenêtre de l'écran impossible : {e}"),
            }
        }
        // Les calques des fenêtres parties les quittent (le fil principal
        // garde ces fenêtres, fermées, jusqu'à la fin).
        if old.iter().any(Place::uses_layers) {
            platform::scene_transaction(|| {
                for place in &old {
                    if let Backend::Layers { root, .. } = &place.backend {
                        root.remove();
                    }
                }
            });
        }
        notes
    }

    fn open(&mut self, overlay: Overlay) -> Result<Place, String> {
        let backend = match overlay.surface {
            Surface::Layers(root) => Backend::Layers {
                root,
                nodes: IdMap::default(),
            },
            Surface::Gpu(window) => {
                let gpu = self.gpu.get_or_insert_with(Gpu::new);
                let size = window.inner_size();
                let surface = gpu
                    .create_surface(Arc::clone(&window), size.width, size.height)
                    .map_err(|e| format!("rendu indisponible : {e}"))?;
                Backend::Gpu {
                    surface: Box::new(surface),
                    list: DisplayList::default(),
                    window,
                }
            }
        };
        Ok(Place {
            info: overlay.info,
            backend,
        })
    }

    /// La fenêtre `key` a changé de taille (GPU).
    pub fn resized(&mut self, key: u64, width: u32, height: u32) {
        let Some(place) = self.places.iter_mut().find(|p| p.info.key == key) else {
            return;
        };
        if let (Backend::Gpu { surface, .. }, Some(gpu)) = (&mut place.backend, self.gpu.as_ref()) {
            gpu.resize(surface, width, height);
        }
    }

    /// Les images ont changé (compressées, autre taille, thème) : tout est
    /// refait à la prochaine image.
    pub fn forget_images(&mut self) {
        self.images.clear();
        for shown in self.shown.values_mut() {
            shown.key = None;
        }
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.forget_unused_images();
        }
    }

    /// Images prêtes à l'affichage (calques ou GPU) : nombre et octets.
    #[must_use]
    pub fn image_usage(&self) -> (usize, usize) {
        match &self.gpu {
            Some(gpu) if self.places.iter().all(|p| !p.uses_layers()) => gpu.texture_usage(),
            _ => (self.images.map.len(), self.images.bytes),
        }
    }

    /// Images décompressées et oubliées depuis la dernière fois.
    pub fn take_image_counts(&mut self) -> (u32, u32) {
        (
            std::mem::take(&mut self.images.decoded),
            std::mem::take(&mut self.images.evicted),
        )
    }

    /// Ce que montrent les calques de chaque écran, en images PNG dans
    /// `dir` (`<name>-ecran<n>.png`), à `scale` pixels par point : de quoi
    /// vérifier ce qui est à l'écran pendant un essai. Renvoie les fichiers
    /// écrits.
    pub fn snapshot(
        &self,
        dir: &std::path::Path,
        name: &str,
        scale: f64,
    ) -> Vec<std::path::PathBuf> {
        let mut written = Vec::new();
        for (i, place) in self.places.iter().enumerate() {
            let Backend::Layers { root, .. } = &place.backend else {
                continue;
            };
            let rect = place.info.rect;
            let Some((width, height, mut rgba)) =
                platform::render_scene(root, rect.w as u32, rect.h as u32, scale)
            else {
                continue;
            };
            unpremultiply(&mut rgba);
            let png =
                mouflon_render::sprites::encode_png_rgba(&Bitmap::from_rgba(width, height, rgba));
            let path = dir.join(format!("{name}-ecran{i}.png"));
            if std::fs::create_dir_all(dir).is_ok() && std::fs::write(&path, png).is_ok() {
                written.push(path);
            }
        }
        written
    }

    /// Prépare d'avance ce que montrera un animal (avant sa première
    /// apparition) : ses images décompressées pour les calques, ou
    /// envoyées au GPU.
    pub fn warm_up(&mut self, art: &Arc<PetArt>, view: &PetView<'_>, style: &Style) {
        let canvas = art.canvas();
        let scale = self.scale_at(view.ground);
        let (width, height) = (
            (f64::from(canvas.width) * scale).ceil() as u32,
            (f64::from(canvas.height) * scale).ceil() as u32,
        );
        self.list.clear(width, height);
        let frame = Frame {
            origin: view.ground - canvas.anchor,
            scale,
        };
        paint_pet(art, view, &frame, style, &mut self.list);
        let mut pieces = Pieces::default();
        flatten(&self.list, &mut pieces);
        if self.places.iter().any(Place::uses_layers) {
            for piece in &pieces.items {
                self.images.prepare(&piece.bitmap);
            }
        }
        if let Some(gpu) = self.gpu.as_mut() {
            let mut list = DisplayList::new(width, height);
            pieces.write_list(&mut list);
            if let Err(e) = gpu.warm_up(&list) {
                log::warn!("mise en route du GPU : {e}");
            }
        }
    }

    /// Échelle (pixels par point) de l'écran où se trouve `p`.
    fn scale_at(&self, p: Vec2) -> f64 {
        self.places
            .iter()
            .find(|place| place.info.rect.contains(p))
            .or_else(|| self.places.first())
            .map_or(2.0, |place| place.info.scale)
    }

    /// Début d'une image.
    pub fn begin(&mut self) {
        self.frame += 1;
        self.timings = Timings::default();
        self.images.begin(self.frame);
    }

    /// Un animal à cette image.
    pub fn show_pet(&mut self, id: u32, art: &Arc<PetArt>, view: &PetView<'_>, style: &Style) {
        let started = Instant::now();
        let entity = Entity::Pet(id);
        let canvas = art.canvas();
        let scale = self.scale_at(view.ground);
        let sprite = SpriteKey::of(art, view);
        // Le point d'appui au pixel près : un animal en images se déplace
        // de pixel en pixel, sans être redessiné (seul son calque bouge).
        let ground = (
            (f64::from(view.ground.x) * scale).round(),
            (f64::from(view.ground.y) * scale).round(),
        );
        let anchor = (
            (f64::from(canvas.anchor.x) * scale).round(),
            (f64::from(canvas.anchor.y) * scale).round(),
        );
        let origin = (ground.0 - anchor.0, ground.1 - anchor.1);
        let key = sprite.map(|sprite| ContentKey {
            sprite,
            art: Arc::as_ptr(art) as usize,
            scale,
            dark: style.dark,
        });
        let order = &mut self.order;
        let shown = self.shown.entry(entity).or_insert_with(|| {
            order.push(entity);
            Shown::default()
        });
        shown.seen = self.frame;
        shown.origin = origin;
        shown.scale = scale;
        shown.size = (f64::from(canvas.width), f64::from(canvas.height));
        if key.is_some() && key == shown.key {
            self.timings.paint += started.elapsed();
            self.timings.shown += 1;
            return;
        }
        let mut view = view.clone();
        if sprite.is_some() {
            view.ground = Vec2::new((ground.0 / scale) as f32, (ground.1 / scale) as f32);
        }
        self.list.clear(
            (f64::from(canvas.width) * scale).ceil() as u32,
            (f64::from(canvas.height) * scale).ceil() as u32,
        );
        let frame = Frame {
            origin: Vec2::new((origin.0 / scale) as f32, (origin.1 / scale) as f32),
            scale,
        };
        paint_pet(art, &view, &frame, style, &mut self.list);
        flatten(&self.list, &mut shown.pieces);
        shown.key = key;
        shown.generation += 1;
        self.timings.repainted += 1;
        self.timings.shown += 1;
        self.timings.paint += started.elapsed();
    }

    /// Une soucoupe en visite à cette image (sous les animaux).
    pub fn show_saucer(&mut self, id: u32, view: &SaucerView, style: &Style) {
        let started = Instant::now();
        let entity = Entity::Saucer(id);
        let canvas = mouflon_render::saucer::saucer_canvas();
        let scale = self.scale_at(view.pos);
        let origin = (
            (f64::from(view.pos.x - canvas.anchor.x) * scale).round(),
            (f64::from(view.pos.y - canvas.anchor.y) * scale).round(),
        );
        if !self.shown.contains_key(&entity) {
            self.order.insert(0, entity);
        }
        let shown = self.shown.entry(entity).or_default();
        shown.seen = self.frame;
        shown.origin = origin;
        shown.scale = scale;
        shown.size = (f64::from(canvas.width), f64::from(canvas.height));
        self.list.clear(
            (f64::from(canvas.width) * scale).ceil() as u32,
            (f64::from(canvas.height) * scale).ceil() as u32,
        );
        let frame = Frame {
            origin: Vec2::new((origin.0 / scale) as f32, (origin.1 / scale) as f32),
            scale,
        };
        mouflon_render::saucer::paint_saucer(view, &frame, style, &mut self.list);
        flatten(&self.list, &mut shown.pieces);
        shown.key = None;
        shown.generation += 1;
        self.timings.repainted += 1;
        self.timings.shown += 1;
        self.timings.paint += started.elapsed();
    }

    /// Fin d'une image : ce qui n'a pas été montré s'en va, chaque fenêtre
    /// d'écran se met à jour (une seule transaction pour toutes), et les
    /// images les moins récentes sont oubliées au-delà du budget.
    pub fn finish(&mut self) -> Timings {
        let frame = self.frame;
        let mut gone = std::mem::take(&mut self.gone);
        gone.clear();
        self.shown.retain(|entity, shown| {
            let here = shown.seen == frame;
            if !here {
                gone.push(*entity);
            }
            here
        });
        // L'ordre ne se refait que si quelque chose est parti.
        if !gone.is_empty() {
            let shown = &self.shown;
            self.order.retain(|entity| shown.contains_key(entity));
        }
        let decode_before = self.images.spent;
        let started = Instant::now();
        let uses_layers = self.places.iter().any(Place::uses_layers);
        if uses_layers {
            let Self {
                places,
                shown,
                order,
                images,
                ..
            } = self;
            platform::scene_transaction(|| {
                for place in places.iter_mut() {
                    update_layers(place, shown, order, &gone, images);
                }
                self.timings.layers = started.elapsed();
            });
            self.timings.commit = started.elapsed().saturating_sub(self.timings.layers);
        }
        self.gone = gone;
        self.timings.decode = self.images.spent.saturating_sub(decode_before);
        self.timings.layers = self.timings.layers.saturating_sub(self.timings.decode);
        self.render_gpu();
        self.images.trim();
        self.timings
    }

    /// Les fenêtres d'écran sans calques : tout se redessine sur le GPU.
    fn render_gpu(&mut self) {
        let Some(gpu) = self.gpu.as_mut() else {
            return;
        };
        let started = Instant::now();
        let mut jobs: Vec<Job<'_>> = Vec::new();
        for place in &mut self.places {
            let Backend::Gpu {
                surface,
                list,
                window,
            } = &mut place.backend
            else {
                continue;
            };
            let overlay = place.info;
            let size = window.inner_size();
            list.clear(size.width, size.height);
            for entity in &self.order {
                let Some(shown) = self.shown.get(entity) else {
                    continue;
                };
                if !overlaps(shown.rect(), overlay.rect) {
                    continue;
                }
                // Des pixels de la toile à ceux de l'écran.
                let k = overlay.scale / shown.scale;
                let at = Affine::translate((
                    shown.origin.0 * k - f64::from(overlay.rect.x) * overlay.scale,
                    shown.origin.1 * k - f64::from(overlay.rect.y) * overlay.scale,
                )) * Affine::scale(k);
                write_pieces(&shown.pieces, at, list);
            }
            jobs.push(Job {
                surface,
                list,
                window,
            });
        }
        if !jobs.is_empty() {
            let _ = gpu.render_all(&mut jobs);
        }
        self.timings.commit += started.elapsed();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

/// RVBA prémultiplié → alpha non prémultiplié, sur place.
fn unpremultiply(rgba: &mut [u8]) {
    for px in rgba.as_chunks_mut::<4>().0 {
        let a = u32::from(px[3]);
        if a == 0 || a == 255 {
            continue;
        }
        for c in &mut px[..3] {
            *c = ((u32::from(*c) * 255 + a / 2) / a).min(255) as u8;
        }
    }
}

fn overlaps(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

/// Les calques d'une fenêtre d'écran : un nœud par chose qui s'y trouve
/// (même en partie), dans l'ordre d'empilement. `gone` : ce qui n'est plus
/// montré du tout depuis l'image d'avant.
fn update_layers(
    place: &mut Place,
    shown: &IdMap<Entity, Shown>,
    order: &[Entity],
    gone: &[Entity],
    images: &mut Images,
) {
    let overlay = place.info;
    let Backend::Layers { root, nodes } = &mut place.backend else {
        return;
    };
    let screen = (f64::from(overlay.rect.x), f64::from(overlay.rect.y));
    // Ce qui est parti.
    for entity in gone {
        if let Some(node) = nodes.remove(entity) {
            node.container.remove();
        }
    }
    for entity in order {
        let Some(shown) = shown.get(entity) else {
            continue;
        };
        // Ce qui n'est pas (ou plus) sur cet écran.
        if !overlaps(shown.rect(), overlay.rect) {
            if let Some(node) = nodes.remove(entity) {
                node.container.remove();
            }
            continue;
        }
        let node = nodes.entry(*entity).or_insert_with(|| {
            let node = Node::new();
            match entity {
                // Les soucoupes passent sous les animaux.
                Entity::Saucer(_) => root.insert(&node.container, 0),
                Entity::Pet(_) => root.add(&node.container),
            }
            node
        });
        node.update(shown, screen, images);
    }
}

/// Les morceaux, posés par `at`, en liste d'images (et de calques pour les
/// groupes) pour le rendu GPU.
fn write_pieces(pieces: &Pieces, at: Affine, out: &mut DisplayList) {
    let mut open: u16 = 0;
    for piece in &pieces.items {
        if piece.group != open {
            if open != 0 {
                out.pop_layer();
            }
            if piece.group != 0 {
                let bounds = out.bounds();
                out.push_layer(
                    mouflon_render::paint::shape_path(&bounds),
                    Affine::IDENTITY,
                    pieces.group_alpha(piece.group),
                    Mix::Normal,
                );
            }
            open = piece.group;
        }
        out.push(Command::Image {
            bitmap: piece.bitmap.clone(),
            transform: at * piece.transform,
            alpha: piece.alpha,
            smooth: piece.smooth,
        });
    }
    if open != 0 {
        out.pop_layer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deux écrans côte à côte, de 800 × 600 points chacun (clés 1 et 2).
    fn two_screens() -> Vec<Overlay> {
        (0..2u64)
            .map(|i| Overlay {
                info: ScreenInfo {
                    key: i + 1,
                    rect: Rect::new(800.0 * i as f32, 0.0, 800.0, 600.0),
                    scale: 1.0,
                    hz: 60.0,
                },
                surface: Surface::Layers(platform::SceneLayer::new()),
            })
            .collect()
    }

    /// Les choses qui ont des calques sur chaque écran.
    fn on_screens(scene: &Scene) -> Vec<Vec<Entity>> {
        scene
            .places
            .iter()
            .map(|place| match &place.backend {
                Backend::Layers { nodes, .. } => {
                    let mut here: Vec<Entity> = nodes.keys().copied().collect();
                    here.sort_by_key(|e| match e {
                        Entity::Pet(id) | Entity::Saucer(id) => *id,
                    });
                    here
                }
                Backend::Gpu { .. } => Vec::new(),
            })
            .collect()
    }

    /// Un animal a des calques sur l'écran où il est, les quitte quand il
    /// passe sur l'autre, et n'en a plus nulle part quand il s'en va ; les
    /// autres gardent les leurs.
    #[test]
    fn a_pet_has_layers_only_where_it_is() {
        let def = Arc::new(crate::library::builtin_mannequin().expect("mannequin"));
        let art = Arc::new(PetArt::load(&def).expect("dessin"));
        let world = mouflon_core::World::new(vec![mouflon_core::Screen {
            frame: Rect::new(0.0, 0.0, 1600.0, 600.0),
            usable: Rect::new(0.0, 25.0, 1600.0, 560.0),
            scale: 1.0,
        }]);
        let pet = mouflon_core::Pet::spawn(0, Arc::clone(&def), 3, &world, Some(200.0));
        let mut scene = Scene::new();
        assert_eq!(scene.set_overlays(two_screens()).len(), 2, "deux fenêtres");
        let style = Style::default();
        let frame = |scene: &mut Scene, pets: &[(u32, f32)]| {
            scene.begin();
            for &(id, x) in pets {
                let mut view = pet.view(1.0);
                view.ground = Vec2::new(x, 500.0);
                scene.show_pet(id, &art, &view, &style);
            }
            scene.finish();
            on_screens(scene)
        };
        let (a, b) = (Entity::Pet(1), Entity::Pet(2));
        assert_eq!(
            frame(&mut scene, &[(1, 200.0), (2, 1200.0)]),
            [vec![a], vec![b]]
        );
        // Il passe sur l'autre écran (à cheval sur les deux en chemin).
        assert_eq!(
            frame(&mut scene, &[(1, 800.0), (2, 1200.0)]),
            [vec![a], vec![a, b]]
        );
        assert_eq!(
            frame(&mut scene, &[(1, 1400.0), (2, 1200.0)]),
            [vec![], vec![a, b]]
        );
        // Il s'en va.
        assert_eq!(frame(&mut scene, &[(2, 1200.0)]), [vec![], vec![b]]);
        assert_eq!(frame(&mut scene, &[]), [Vec::<Entity>::new(), vec![]]);
    }
}
