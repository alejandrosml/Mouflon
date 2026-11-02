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
//! La fenêtre d'un écran laisse passer la souris, sauf au-dessus d'un
//! animal : l'app le décide à chaque image (voir [`Scene::set_hittest`]).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mouflon_core::{PetView, Rect, SaucerView, Screen, Vec2};
use mouflon_platform as platform;
use mouflon_render::display::{Bitmap, Command, DisplayList};
use mouflon_render::pieces::{Pieces, flatten};
use mouflon_render::vello::kurbo::Affine;
use mouflon_render::vello::peniko::Mix;
use mouflon_render::{Frame, PetArt, SpriteKey, Style, paint_pet};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId, WindowLevel};

use crate::gpu::{Gpu, Job, PetSurface};

/// Images décompressées gardées pour les calques : au-delà, les moins
/// récemment montrées sont oubliées (et refaites si elles reviennent).
const IMAGE_BUDGET: usize = 96 << 20;
/// Temps de décompression d'images nouvelles par image, au plus : le
/// reste attend l'image suivante (le calque garde la sienne en attendant).
/// La première de chaque image passe toujours.
const DECODE_BUDGET: Duration = Duration::from_millis(3);

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
    map: HashMap<u64, Cached>,
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

    /// Met ses calques à jour : sa place sur l'écran `screen` (coin haut
    /// gauche et hauteur, points), et ses morceaux s'ils ont changé.
    /// `false` si une image n'a pas pu être prête à temps (on réessaie à
    /// l'image suivante).
    ///
    /// Les calques suivent le repère ordinaire de Core Animation sur Mac :
    /// origine en bas à gauche, y vers le haut (une image s'y affiche à
    /// l'endroit). Le calque qui porte la chose est placé au coin haut
    /// gauche de sa toile ; chaque morceau y descend (y négatif).
    fn update(&mut self, shown: &Shown, screen: (f64, f64, f64), images: &mut Images) -> bool {
        let position = (
            shown.origin.0 / shown.scale - screen.0,
            screen.2 - (shown.origin.1 / shown.scale - screen.1),
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
            let transform = upright(
                (to_points * piece.transform * Affine::translate((-1.0, -1.0))).as_coeffs(),
                f64::from(size.1),
            );
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

/// Comment une fenêtre d'écran montre ce qu'elle porte.
enum Backend {
    /// Des calques Core Animation (Mac).
    Layers {
        root: platform::SceneLayer,
        nodes: HashMap<Entity, Node>,
    },
    /// Le GPU : tout se redessine dans la fenêtre à chaque image.
    Gpu {
        surface: Box<PetSurface>,
        list: DisplayList,
    },
}

/// La fenêtre transparente d'un écran.
pub struct Overlay {
    /// Le cadre de l'écran (repère commun, points).
    pub rect: Rect,
    pub scale: f64,
    pub window: Arc<Window>,
    /// Prend-elle la souris en ce moment ?
    pub hittest: bool,
    backend: Backend,
}

impl Overlay {
    #[must_use]
    pub fn uses_layers(&self) -> bool {
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
    overlays: Vec<Overlay>,
    shown: HashMap<Entity, Shown>,
    /// Ordre d'empilement : les soucoupes d'abord, puis les animaux dans
    /// l'ordre de leur arrivée.
    order: Vec<Entity>,
    images: Images,
    gpu: Option<Gpu>,
    list: DisplayList,
    frame: u64,
    timings: Timings,
    /// Calques Core Animation quand le système en a (sinon le GPU).
    layers_wanted: bool,
}

impl Scene {
    #[must_use]
    pub fn new() -> Self {
        Self {
            overlays: Vec::new(),
            shown: HashMap::new(),
            order: Vec::new(),
            images: Images::default(),
            gpu: None,
            list: DisplayList::default(),
            frame: 0,
            timings: Timings::default(),
            // `MOUFLON_GPU=1` : le GPU même sur Mac (pour comparer).
            layers_wanted: !std::env::var("MOUFLON_GPU").is_ok_and(|v| !v.is_empty() && v != "0"),
        }
    }

    pub fn overlays(&self) -> &[Overlay] {
        &self.overlays
    }

    /// La fenêtre d'écran `window`, s'il en est une.
    pub fn overlay(&self, window: WindowId) -> Option<&Overlay> {
        self.overlays.iter().find(|o| o.window.id() == window)
    }

    /// Le GPU, s'il sert (fenêtres d'écran sans calques).
    pub fn gpu_mut(&mut self) -> Option<&mut Gpu> {
        self.gpu.as_mut()
    }

    /// Une fenêtre par écran : refaites quand les écrans changent. Renvoie
    /// de quoi le dire au journal (rien si elles n'ont pas changé).
    pub fn sync_screens(
        &mut self,
        event_loop: &ActiveEventLoop,
        screens: &[Screen],
    ) -> Vec<String> {
        let rects: Vec<(Rect, f32)> = screens.iter().map(|s| (s.frame, s.scale)).collect();
        let same = self.overlays.len() == rects.len()
            && self
                .overlays
                .iter()
                .all(|o| rects.iter().any(|(r, _)| *r == o.rect));
        if same {
            return Vec::new();
        }
        for overlay in self.overlays.drain(..) {
            if let Backend::Layers { root, .. } = &overlay.backend {
                root.remove();
            }
            platform::keep_pet_window(&*overlay.window);
        }
        let mut notes = Vec::new();
        for (rect, _) in rects {
            match self.open_overlay(event_loop, rect) {
                Ok(overlay) => {
                    notes.push(format!(
                        "scène : fenêtre de l'écran {:.0}×{:.0} en {:.0},{:.0} ({})",
                        rect.w,
                        rect.h,
                        rect.x,
                        rect.y,
                        if overlay.uses_layers() {
                            "calques Core Animation"
                        } else {
                            "GPU"
                        }
                    ));
                    self.overlays.push(overlay);
                }
                Err(e) => log::error!("fenêtre de l'écran impossible : {e}"),
            }
        }
        notes
    }

    fn open_overlay(
        &mut self,
        event_loop: &ActiveEventLoop,
        rect: Rect,
    ) -> Result<Overlay, String> {
        let attrs = Window::default_attributes()
            .with_title("Mouflon")
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(LogicalSize::new(f64::from(rect.w), f64::from(rect.h)))
            .with_position(LogicalPosition::new(f64::from(rect.x), f64::from(rect.y)))
            .with_active(false);
        #[cfg(target_os = "macos")]
        let attrs = {
            use winit::platform::macos::WindowAttributesExtMacOS;
            attrs.with_has_shadow(false).with_accepts_first_mouse(true)
        };
        let window = Arc::new(event_loop.create_window(attrs).map_err(|e| e.to_string())?);
        platform::prepare_pet_window(&*window);
        // Elle laisse passer la souris, sauf au-dessus d'un animal.
        let _ = window.set_cursor_hittest(false);
        let scale = window.scale_factor();
        let layers = self
            .layers_wanted
            .then(|| platform::attach_scene(&*window))
            .flatten();
        let backend = match layers {
            Some(root) => Backend::Layers {
                root,
                nodes: HashMap::new(),
            },
            None => {
                let gpu = self.gpu.get_or_insert_with(Gpu::new);
                let size = window.inner_size();
                match gpu.create_surface(Arc::clone(&window), size.width, size.height) {
                    Ok(surface) => Backend::Gpu {
                        surface: Box::new(surface),
                        list: DisplayList::default(),
                    },
                    Err(e) => {
                        platform::keep_pet_window(&*window);
                        return Err(format!("rendu indisponible : {e}"));
                    }
                }
            }
        };
        Ok(Overlay {
            rect,
            scale,
            window,
            hittest: false,
            backend,
        })
    }

    /// La fenêtre `index` prend la souris (au-dessus d'un animal) ou la
    /// laisse passer.
    pub fn set_hittest(&mut self, index: usize, on: bool) -> bool {
        let Some(overlay) = self.overlays.get_mut(index) else {
            return false;
        };
        if overlay.hittest == on {
            return false;
        }
        let _ = overlay.window.set_cursor_hittest(on);
        overlay.hittest = on;
        true
    }

    pub fn scale_changed(&mut self, window: WindowId, scale: f64) {
        if let Some(overlay) = self.overlays.iter_mut().find(|o| o.window.id() == window) {
            overlay.scale = scale;
        }
    }

    pub fn resized(&mut self, window: WindowId, width: u32, height: u32) {
        let Some(overlay) = self.overlays.iter_mut().find(|o| o.window.id() == window) else {
            return;
        };
        if let (Backend::Gpu { surface, .. }, Some(gpu)) = (&mut overlay.backend, self.gpu.as_ref())
        {
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
            Some(gpu) if self.overlays.iter().all(|o| !o.uses_layers()) => gpu.texture_usage(),
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
        for (i, overlay) in self.overlays.iter().enumerate() {
            let Backend::Layers { root, .. } = &overlay.backend else {
                continue;
            };
            let Some((width, height, mut rgba)) =
                platform::render_scene(root, overlay.rect.w as u32, overlay.rect.h as u32, scale)
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

    /// Avant de quitter : les fenêtres ne seront jamais libérées (voir
    /// `platform::keep_pet_window`).
    pub fn shutdown(&mut self) {
        for overlay in &self.overlays {
            platform::keep_pet_window(&*overlay.window);
        }
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
        if self.overlays.iter().any(Overlay::uses_layers) {
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
        self.overlays
            .iter()
            .find(|o| o.rect.contains(p))
            .or_else(|| self.overlays.first())
            .map_or(2.0, |o| o.scale)
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
        if !self.shown.contains_key(&entity) {
            self.order.push(entity);
        }
        let shown = self.shown.entry(entity).or_default();
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
        self.shown.retain(|_, shown| shown.seen == frame);
        let shown = &self.shown;
        self.order.retain(|entity| shown.contains_key(entity));
        let decode_before = self.images.spent;
        let started = Instant::now();
        let uses_layers = self.overlays.iter().any(Overlay::uses_layers);
        if uses_layers {
            let Self {
                overlays,
                shown,
                order,
                images,
                ..
            } = self;
            platform::scene_transaction(|| {
                for overlay in overlays.iter_mut() {
                    update_layers(overlay, shown, order, images);
                }
                self.timings.layers = started.elapsed();
            });
            self.timings.commit = started.elapsed().saturating_sub(self.timings.layers);
        }
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
        for overlay in &mut self.overlays {
            let Backend::Gpu { surface, list } = &mut overlay.backend else {
                continue;
            };
            let size = overlay.window.inner_size();
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
                window: &overlay.window,
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

/// La transformation d'un calque d'image de `height` pixels de haut, dans
/// le repère de Core Animation (y vers le haut, l'image à l'endroit : sa
/// première ligne en haut du calque), pour qu'il tombe là où `toward`
/// (des pixels de l'image, y vers le bas, aux points du calque parent, y
/// vers le bas depuis son origine) pose l'image.
///
/// Un point du calque (u, w) est la ligne `height - w` de l'image ; il va
/// en `toward(u, height - w)`, puis y change de signe.
fn upright([a, b, c, d, e, f]: [f64; 6], height: f64) -> [f64; 6] {
    [a, -b, -c, d, c * height + e, -(d * height + f)]
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
/// (même en partie), dans l'ordre d'empilement.
fn update_layers(
    overlay: &mut Overlay,
    shown: &HashMap<Entity, Shown>,
    order: &[Entity],
    images: &mut Images,
) {
    let Backend::Layers { root, nodes } = &mut overlay.backend else {
        return;
    };
    let screen = (
        f64::from(overlay.rect.x),
        f64::from(overlay.rect.y),
        f64::from(overlay.rect.h),
    );
    // Ce qui est parti, ou n'est plus sur cet écran.
    nodes.retain(|entity, node| {
        let here = shown
            .get(entity)
            .is_some_and(|s| overlaps(s.rect(), overlay.rect));
        if !here {
            node.container.remove();
        }
        here
    });
    for entity in order {
        let Some(shown) = shown.get(entity) else {
            continue;
        };
        if !overlaps(shown.rect(), overlay.rect) {
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

    /// Applique une transformation `[a, b, c, d, tx, ty]` à un point.
    fn apply([a, b, c, d, tx, ty]: [f64; 6], (x, y): (f64, f64)) -> (f64, f64) {
        (a * x + c * y + tx, b * x + d * y + ty)
    }

    #[test]
    fn images_land_where_they_should_in_the_upward_frame() {
        // Une image de 10 × 6 pixels, tournée, agrandie, déplacée (y vers le bas).
        let toward = Affine::translate((40.0, 25.0)) * Affine::rotate(0.4) * Affine::scale(1.5);
        let t = upright(toward.as_coeffs(), 6.0);
        for (u, v) in [
            (0.0, 0.0),
            (10.0, 0.0),
            (0.0, 6.0),
            (10.0, 6.0),
            (3.5, 2.25),
        ] {
            // La ligne `v` de l'image est à la hauteur `6 - v` du calque.
            let (x, y) = apply(t, (u, 6.0 - v));
            let p = toward * mouflon_render::vello::kurbo::Point::new(u, v);
            assert!(
                (x - p.x).abs() < 1e-9 && (y + p.y).abs() < 1e-9,
                "({u}, {v})"
            );
        }
    }
}
