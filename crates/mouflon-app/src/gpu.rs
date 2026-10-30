//! Rendu GPU. Les animaux en images (le cas courant) sont dessinés
//! directement dans leur fenêtre par [`Quick`] : quelques quadrilatères par
//! image. Les dessins vectoriels passent par Vello, mis en route seulement
//! s'il sert : il dessine dans une texture intermédiaire, puis une passe la
//! recopie dans la fenêtre en prémultipliant l'alpha (ce qu'attend Core
//! Animation pour une fenêtre transparente).

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mouflon_render::DisplayList;
use mouflon_render::vello::kurbo::Affine;
use mouflon_render::vello::peniko::{
    Blob, Color, ImageAlphaType, ImageBrush, ImageData, ImageFormat,
};
use mouflon_render::vello::wgpu::{self, util::TextureBlitter, util::TextureBlitterBuilder};
use mouflon_render::vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::window::Window;

use crate::quick::Quick;

/// Sans dessin vectoriel depuis ce temps, Vello s'arrête et rend sa mémoire.
const VELLO_IDLE: Duration = Duration::from_secs(20);

/// Format des fenêtres sur Mac (et le plus souvent ailleurs) : ses programmes
/// se préparent avant d'ouvrir la première fenêtre.
const USUAL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

#[derive(Debug, thiserror::Error)]
pub enum GpuError {
    #[error("impossible de créer la surface : {0}")]
    Surface(#[from] wgpu::CreateSurfaceError),
    #[error("aucun GPU compatible")]
    NoDevice,
    #[error("format de surface non pris en charge")]
    Format,
    #[error("Vello : {0}")]
    Vello(#[from] mouflon_render::vello::Error),
}

/// Instance wgpu et appareils, comme `vello::util::RenderContext`, mais
/// l'appareil demande aussi la lecture des textures compressées BC (BC7 pour
/// les images des animaux) quand le GPU la connaît : tous les Mac.
struct GpuContext {
    instance: wgpu::Instance,
    devices: Vec<DeviceHandle>,
}

struct DeviceHandle {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GpuContext {
    fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            display: None,
            backends: wgpu::Backends::from_env().unwrap_or_default(),
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::from_env_or_default(),
        });
        Self {
            instance,
            devices: Vec::new(),
        }
    }

    /// Un appareil qui peut dessiner dans `compatible_surface` (ou le premier
    /// venu), créé au besoin : son rang dans `devices`.
    async fn device(&mut self, compatible_surface: Option<&wgpu::Surface<'_>>) -> Option<usize> {
        let found = match compatible_surface {
            Some(surface) => self
                .devices
                .iter()
                .position(|d| d.adapter.is_surface_supported(surface)),
            None => (!self.devices.is_empty()).then_some(0),
        };
        if found.is_some() {
            return found;
        }
        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&self.instance, compatible_surface)
                .await
                .ok()?;
        let wanted = wgpu::Features::CLEAR_TEXTURE
            | wgpu::Features::PIPELINE_CACHE
            | wgpu::Features::TEXTURE_COMPRESSION_BC
            | wgpu::Features::IMMEDIATES;
        let features = adapter.features() & wanted;
        // Les paramètres d'un élément du rendu direct (208 octets) partent
        // avec sa commande de dessin quand le GPU le permet.
        let immediates = if features.contains(wgpu::Features::IMMEDIATES) {
            adapter.limits().max_immediate_size.min(256)
        } else {
            0
        };
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: features,
                required_limits: wgpu::Limits {
                    max_immediate_size: immediates,
                    ..wgpu::Limits::default()
                },
                ..Default::default()
            })
            .await
            .ok()?;
        self.devices.push(DeviceHandle {
            adapter,
            device,
            queue,
        });
        Some(self.devices.len() - 1)
    }
}

/// Une fenêtre à redessiner : sa surface, ce qu'on y dessine, la fenêtre.
#[derive(Debug)]
pub struct Job<'a> {
    pub surface: &'a mut PetSurface,
    pub list: &'a DisplayList,
    pub window: &'a Window,
}

/// Fenêtre de rendu d'un animal.
pub struct PetSurface {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    dev_id: usize,
    /// Texture intermédiaire de Vello (et sa vue), créée à son premier usage.
    target: Option<(wgpu::Texture, wgpu::TextureView)>,
}

impl std::fmt::Debug for PetSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PetSurface")
            .field("size", &(self.config.width, self.config.height))
            .finish()
    }
}

/// Vello et ce qui ne sert qu'à lui.
struct VelloPath {
    renderer: Renderer,
    blitter: Option<(wgpu::TextureFormat, TextureBlitter)>,
    scene: Scene,
    /// Un pixel transparent, ajouté à chaque scène : devant une scène sans
    /// aucune image (le mannequin, une fenêtre vide), Vello 0.11 détruit son
    /// atlas d'images sans l'oublier, et les images qu'il croit encore y avoir
    /// s'afficheraient ensuite vides.
    keepalive: ImageBrush,
}

pub struct Gpu {
    context: GpuContext,
    quick: Option<Quick>,
    vello: Option<VelloPath>,
    /// Dernier dessin passé par Vello : sans dessin vectoriel depuis un
    /// moment, ses tampons de travail (plus de 150 Mo) sont rendus.
    vello_used: Option<Instant>,
    /// Format des fenêtres (le premier vu) : les programmes sont prêts pour lui.
    format: Option<wgpu::TextureFormat>,
    timings: Timings,
}

/// Temps passé à dessiner (voir [`Gpu::take_timings`]).
#[derive(Clone, Copy, Debug, Default)]
pub struct Timings {
    /// Commandes de dessin préparées et envoyées au GPU.
    pub paint: Duration,
    /// Attente d'une image d'écran libre.
    pub acquire: Duration,
    /// Envoi à l'écran.
    pub present: Duration,
}

impl std::fmt::Debug for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu")
            .field("devices", &self.context.devices.len())
            .field("vello", &self.vello.is_some())
            .finish()
    }
}

fn create_target(
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("mouflon-cible"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        format: wgpu::TextureFormat::Rgba8Unorm,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

impl VelloPath {
    fn new(device: &wgpu::Device) -> Result<Self, GpuError> {
        let started = Instant::now();
        let renderer = Renderer::new(
            device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::area_only(),
                num_init_threads: NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )?;
        log::info!(
            "Vello mis en route (dessin vectoriel) en {} ms",
            started.elapsed().as_millis()
        );
        Ok(Self {
            renderer,
            blitter: None,
            scene: Scene::new(),
            keepalive: ImageBrush::new(ImageData {
                data: Blob::from(vec![0u8; 4]),
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width: 1,
                height: 1,
            }),
        })
    }

    /// Recopie vers des fenêtres au format `format`, en prémultipliant :
    /// couleur × alpha, sur une cible effacée.
    fn blitter(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) -> &TextureBlitter {
        if self.blitter.as_ref().is_none_or(|(f, _)| *f != format) {
            let premultiply = wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
            };
            let blitter = TextureBlitterBuilder::new(device, format)
                .blend_state(premultiply)
                .build();
            self.blitter = Some((format, blitter));
        }
        match &self.blitter {
            Some((_, blitter)) => blitter,
            None => unreachable!("recopie créée juste au-dessus"),
        }
    }

    /// Dessine la liste dans la texture intermédiaire `target`.
    fn paint(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        list: &DisplayList,
        target: &wgpu::TextureView,
        (width, height): (u32, u32),
    ) -> Result<(), GpuError> {
        self.scene.reset();
        mouflon_render::vello_scene::encode(list, &mut self.scene);
        self.scene.draw_image(&self.keepalive, Affine::IDENTITY);
        self.renderer.render_to_texture(
            device,
            queue,
            &self.scene,
            target,
            &RenderParams {
                base_color: Color::TRANSPARENT,
                width,
                height,
                antialiasing_method: AaConfig::Area,
            },
        )?;
        Ok(())
    }
}

/// Efface `view` en transparent, puis y recopie `source` en prémultipliant.
fn clear_and_blit(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    blitter: &TextureBlitter,
    source: &wgpu::TextureView,
    view: &wgpu::TextureView,
) {
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mouflon-effacement"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
    blitter.copy(device, encoder, source, view);
}

impl Gpu {
    #[must_use]
    pub fn new() -> Self {
        Self {
            context: GpuContext::new(),
            quick: None,
            vello: None,
            vello_used: None,
            format: None,
            timings: Timings::default(),
        }
    }

    /// Temps passé à dessiner depuis le dernier appel.
    pub fn take_timings(&mut self) -> Timings {
        std::mem::take(&mut self.timings)
    }

    /// Images des animaux sur le GPU, et octets qu'elles occupent.
    #[must_use]
    pub fn texture_usage(&self) -> (usize, usize) {
        self.quick.as_ref().map_or((0, 0), Quick::texture_usage)
    }

    /// Oublie les images qui n'ont pas servi à la dernière image (voir
    /// [`Quick::forget_unused`]).
    pub fn forget_unused_images(&mut self) {
        if let Some(quick) = self.quick.as_mut() {
            quick.forget_unused();
        }
    }

    /// Crée la surface d'une fenêtre (et le GPU au premier appel).
    pub fn create_surface(
        &mut self,
        window: Arc<Window>,
        width: u32,
        height: u32,
    ) -> Result<PetSurface, GpuError> {
        let surface = self.context.instance.create_surface(window)?;
        let dev_id =
            pollster::block_on(self.context.device(Some(&surface))).ok_or(GpuError::NoDevice)?;
        let handle = &self.context.devices[dev_id];
        let caps = surface.get_capabilities(&handle.adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| {
                matches!(
                    f,
                    wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm
                )
            })
            .ok_or(GpuError::Format)?;
        let alpha_mode = [
            wgpu::CompositeAlphaMode::PostMultiplied,
            wgpu::CompositeAlphaMode::PreMultiplied,
        ]
        .into_iter()
        .find(|m| caps.alpha_modes.contains(m))
        .unwrap_or(wgpu::CompositeAlphaMode::Auto);
        if alpha_mode == wgpu::CompositeAlphaMode::Auto {
            log::warn!("pas de mode de transparence de fenêtre : le fond risque d'être opaque");
        }
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            color_space: wgpu::SurfaceColorSpace::Auto,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&handle.device, &config);
        self.prepare(dev_id, format);
        self.format.get_or_insert(format);
        Ok(PetSurface {
            surface,
            config,
            dev_id,
            target: None,
        })
    }

    /// Le GPU et l'interface graphique employés (« Apple M2, Metal, intégré »),
    /// une fois l'appareil créé.
    #[must_use]
    pub fn describe(&self) -> Option<String> {
        let info = self.context.devices.first()?.adapter.get_info();
        let kind = match info.device_type {
            wgpu::DeviceType::IntegratedGpu => "intégré",
            wgpu::DeviceType::DiscreteGpu => "séparé",
            wgpu::DeviceType::VirtualGpu => "virtuel",
            wgpu::DeviceType::Cpu => "logiciel",
            wgpu::DeviceType::Other => "autre",
        };
        let bc7 = self
            .context
            .devices
            .first()?
            .device
            .features()
            .contains(wgpu::Features::TEXTURE_COMPRESSION_BC);
        Some(format!(
            "{}, {:?}, {kind}{}",
            info.name,
            info.backend,
            if bc7 { ", images BC7" } else { "" }
        ))
    }

    /// Programmes du rendu direct pour des fenêtres au format `format`.
    fn prepare(&mut self, dev_id: usize, format: wgpu::TextureFormat) {
        let device = &self.context.devices[dev_id].device;
        self.quick
            .get_or_insert_with(|| Quick::new(device))
            .prepare(device, format);
    }

    /// Met le GPU en route avant même la première fenêtre, pendant que les
    /// dessins se décodent à côté : appareil, puis programmes du rendu direct
    /// pour le format habituel des fenêtres.
    pub fn prepare_ahead(&mut self) {
        match pollster::block_on(self.context.device(None)) {
            Some(dev_id) => self.prepare(dev_id, USUAL_FORMAT),
            None => log::warn!("pas de GPU pour l'instant"),
        }
    }

    /// Met le GPU en route avant d'ouvrir la moindre fenêtre : appareil,
    /// programmes, puis une image d'essai (les dessins des animaux du
    /// lancement) faite hors écran jusqu'au bout. La première chute ne paie
    /// plus ces mises en route. Vello n'est mis en route que si un dessin
    /// en a besoin.
    pub fn warm_up(&mut self, list: &DisplayList) -> Result<(), GpuError> {
        let dev_id = pollster::block_on(self.context.device(None)).ok_or(GpuError::NoDevice)?;
        // Le format des fenêtres déjà ouvertes, sinon le format habituel.
        let format = self.format.unwrap_or(USUAL_FORMAT);
        self.prepare(dev_id, format);
        let handle = &self.context.devices[dev_id];
        let (width, height) = (list.width.max(1), list.height.max(1));
        let window = handle.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mouflon-essai"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            view_formats: &[],
        });
        let view = window.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = handle
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mouflon-essai"),
            });
        if Quick::supports(list) {
            if let Some(quick) = self.quick.as_mut() {
                quick.render(
                    &handle.device,
                    &handle.queue,
                    &mut encoder,
                    &view,
                    format,
                    list,
                );
            }
        } else {
            if self.vello.is_none() {
                self.vello = Some(VelloPath::new(&handle.device)?);
            }
            self.vello_used = Some(Instant::now());
            let Some(vello) = self.vello.as_mut() else {
                return Ok(());
            };
            let (_target, target_view) = create_target(&handle.device, width, height);
            vello.paint(
                &handle.device,
                &handle.queue,
                list,
                &target_view,
                (width, height),
            )?;
            let blitter = vello.blitter(&handle.device, format);
            clear_and_blit(&handle.device, &mut encoder, blitter, &target_view, &view);
        }
        handle.queue.submit([encoder.finish()]);
        if let Err(e) = handle.device.poll(wgpu::PollType::wait_indefinitely()) {
            log::warn!("mise en route du GPU : {e}");
        }
        Ok(())
    }

    pub fn resize(&self, surface: &mut PetSurface, width: u32, height: u32) {
        let (width, height) = (width.max(1), height.max(1));
        if surface.config.width == width && surface.config.height == height {
            return;
        }
        let device = &self.context.devices[surface.dev_id].device;
        surface.config.width = width;
        surface.config.height = height;
        surface.surface.configure(device, &surface.config);
        // Refaite à la bonne taille au prochain dessin vectoriel.
        surface.target = None;
    }

    /// Dessine les fenêtres de l'image en cours. Celles que le rendu direct
    /// sait dessiner (toutes, pour des animaux en images) partagent un seul
    /// envoi au GPU : leurs passes s'enchaînent dans une même liste de
    /// commandes, puis chacune s'affiche. Les autres passent par Vello, une à
    /// une. Renvoie, fenêtre par fenêtre, si elle a été dessinée.
    ///
    /// Toutes les fenêtres sont sur le même GPU (un Mac n'en montre qu'un aux
    /// fenêtres) : c'est le sien qui reçoit l'envoi.
    pub fn render_all(&mut self, jobs: &mut [Job<'_>]) -> Vec<bool> {
        let mut done = vec![false; jobs.len()];
        let mut direct = Vec::with_capacity(jobs.len());
        for (i, job) in jobs.iter_mut().enumerate() {
            if Quick::supports(job.list) {
                direct.push(i);
            } else {
                done[i] = self.render_vello(job.surface, job.list, job.window);
                self.vello_used = Some(Instant::now());
            }
        }
        if self.vello.is_some()
            && self
                .vello_used
                .is_none_or(|used| used.elapsed() > VELLO_IDLE)
        {
            self.vello = None;
            log::info!("Vello arrêté : plus de dessin vectoriel à l'écran");
        }
        let Some(quick) = self.quick.as_mut() else {
            return done;
        };
        let started = Instant::now();
        let frames: Vec<(usize, wgpu::SurfaceTexture)> = direct
            .into_iter()
            .filter_map(|i| Some((i, Self::acquire(&self.context, jobs[i].surface)?)))
            .collect();
        let acquired = Instant::now();
        self.timings.acquire += acquired - started;
        let Some(&(first, _)) = frames.first() else {
            return done;
        };
        let handle = &self.context.devices[jobs[first].surface.dev_id];
        let draws = frames
            .iter()
            .map(|&(i, _)| Quick::draws(jobs[i].list))
            .sum();
        quick.begin(&handle.device, draws);
        let mut encoder = handle
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mouflon"),
            });
        for (i, frame) in &frames {
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let job = &jobs[*i];
            quick.draw(
                &handle.device,
                &handle.queue,
                &mut encoder,
                &view,
                job.surface.config.format,
                job.list,
            );
        }
        quick.finish(&handle.queue);
        handle.queue.submit([encoder.finish()]);
        let painted = Instant::now();
        self.timings.paint += painted - acquired;
        for (i, frame) in frames {
            jobs[i].window.pre_present_notify();
            handle.queue.present(frame);
            done[i] = true;
        }
        self.timings.present += painted.elapsed();
        done
    }

    /// Image de la fenêtre à dessiner, ou `None` (surface à refaire, écran perdu…).
    fn acquire(context: &GpuContext, surface: &PetSurface) -> Option<wgpu::SurfaceTexture> {
        match surface.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => Some(t),
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                let device = &context.devices[surface.dev_id].device;
                surface.surface.configure(device, &surface.config);
                None
            }
            _ => None,
        }
    }

    fn render_vello(
        &mut self,
        surface: &mut PetSurface,
        list: &DisplayList,
        window: &Window,
    ) -> bool {
        let handle = &self.context.devices[surface.dev_id];
        if self.vello.is_none() {
            match VelloPath::new(&handle.device) {
                Ok(vello) => self.vello = Some(vello),
                Err(e) => {
                    log::warn!("Vello indisponible : {e}");
                    return false;
                }
            }
        }
        let Some(vello) = self.vello.as_mut() else {
            return false;
        };
        let size = (surface.config.width, surface.config.height);
        let (_, target) = surface
            .target
            .get_or_insert_with(|| create_target(&handle.device, size.0, size.1));
        let started = Instant::now();
        if let Err(e) = vello.paint(&handle.device, &handle.queue, list, target, size) {
            log::warn!("rendu Vello : {e}");
            return false;
        }
        let painted = Instant::now();
        self.timings.paint += painted - started;
        let Some(frame) = Self::acquire(&self.context, surface) else {
            return false;
        };
        let acquired = Instant::now();
        self.timings.acquire += acquired - painted;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = handle
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mouflon"),
            });
        let blitter = vello.blitter(&handle.device, surface.config.format);
        let Some((_, target)) = surface.target.as_ref() else {
            return false;
        };
        clear_and_blit(&handle.device, &mut encoder, blitter, target, &view);
        handle.queue.submit([encoder.finish()]);
        window.pre_present_notify();
        handle.queue.present(frame);
        self.timings.present += acquired.elapsed();
        true
    }
}

#[cfg(test)]
mod tests {
    //! Le rendu direct donne la même image que Vello, à l'arrondi près.
    use std::path::Path;
    use std::sync::Arc;

    use mouflon_core::{Particle, ParticleKind, Shadow, Support, Vec2, load_pet};
    use mouflon_render::{DisplayList, Frame, PetArt, Style, paint_pet};

    use super::*;

    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

    /// Lit une texture RGBA 8 bits (lignes alignées comme l'exige wgpu).
    fn read(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture) -> Vec<u8> {
        let (w, h) = (texture.width(), texture.height());
        let row = (w * 4).div_ceil(256) * 256;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(row * h),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row),
                    rows_per_image: Some(h),
                },
            },
            texture.size(),
        );
        queue.submit([encoder.finish()]);
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, |r| r.expect("lecture"));
        device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("attente du GPU");
        let data = buffer.slice(..).get_mapped_range().expect("tampon lisible");
        let mut out = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            let start = (y * row) as usize;
            out.extend_from_slice(&data[start..start + (w * 4) as usize]);
        }
        out
    }

    fn target(device: &wgpu::Device, list: &DisplayList) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: list.width,
                height: list.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format: FORMAT,
            view_formats: &[],
        })
    }

    /// Écarts entre deux images : le plus grand, la moyenne, et la part des
    /// octets qui s'écartent de plus de 6/255.
    fn compare(a: &[u8], b: &[u8]) -> (u8, f64, f64) {
        let mut max = 0u8;
        let mut sum = 0u64;
        let mut far = 0usize;
        for (x, y) in a.iter().zip(b) {
            let d = x.abs_diff(*y);
            max = max.max(d);
            sum += u64::from(d);
            far += usize::from(d > 6);
        }
        (
            max,
            sum as f64 / a.len() as f64,
            far as f64 / a.len() as f64,
        )
    }

    /// Lainou et Pelote, images compressées ou non.
    fn pets(compressed: bool) -> Vec<(Arc<mouflon_core::PetDef>, PetArt)> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets");
        ["lainou", "pelote"]
            .iter()
            .map(|name| {
                let def = Arc::new(load_pet(&root.join(name).join("pet.toml")).expect("pet.toml"));
                let mut art = PetArt::load(&def).expect("dessin");
                if compressed {
                    art = art.compressed(8, &|| {}).expect("images à compresser");
                }
                (def, art)
            })
            .collect()
    }

    /// Listes d'affichage de Lainou et Pelote dans des situations variées, à
    /// deux échelles : (nom, liste).
    fn sample_lists(compressed: bool) -> Vec<(String, DisplayList)> {
        let world = mouflon_core::World::new(vec![mouflon_core::Screen {
            frame: mouflon_core::Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: mouflon_core::Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }]);
        let dust: Vec<Particle> = (0..5)
            .map(|i| Particle {
                kind: if i % 2 == 0 {
                    ParticleKind::Dust
                } else {
                    ParticleKind::Bubble
                },
                pos: Vec2::new(590.0 + i as f32 * 9.0, 760.0 - i as f32 * 7.0),
                vel: Vec2::new(0.0, 0.0),
                age: 0.2 + i as f32 * 0.15,
                life: 1.2,
                size: 4.0 + i as f32,
                phase: 0.0,
            })
            .collect();
        let mut lists = Vec::new();
        for (def, art) in pets(compressed) {
            let mut pet = mouflon_core::Pet::spawn(0, Arc::clone(&def), 7, &world, Some(600.0));
            for _ in 0..600 {
                pet.step(mouflon_core::TICK, &world, &mouflon_core::Input::default());
            }
            let canvas = art.canvas();
            for scale in [2.0, 1.0] {
                for case in 0..7 {
                    let mut view = pet.view(1.0);
                    match case {
                        // Au repos, tourné d'un côté puis de l'autre.
                        0 => view.facing = 1.0,
                        1 => view.facing = -1.0,
                        // Culbute : l'image tourne (lissée).
                        2 => view.pose.roll = 0.9,
                        // En l'air : ombre floue sous lui.
                        3 => {
                            view.support = Support::Air;
                            view.shadow = Some(Shadow {
                                y: view.ground.y + 30.0,
                                distance: 6.0,
                            });
                            view.pose.lean = 0.3;
                        }
                        // Il s'efface.
                        4 => view.opacity = 0.45,
                        // Écrasé à l'atterrissage.
                        5 => view.pose.squash = 0.25,
                        // Poussière et bulles.
                        _ => view.particles = &dust,
                    }
                    let mut list = DisplayList::new(
                        (f64::from(canvas.width) * scale) as u32,
                        (f64::from(canvas.height) * scale) as u32,
                    );
                    let frame = Frame {
                        origin: view.ground - canvas.anchor,
                        scale,
                    };
                    paint_pet(&art, &view, &frame, &Style::default(), &mut list);
                    let name = format!("{} ×{scale} cas {case}", def.meta.name);
                    assert!(Quick::supports(&list), "{name}");
                    lists.push((name, list));
                }
            }
        }
        lists
    }

    #[test]
    fn direct_rendering_matches_vello() {
        let mut context = GpuContext::new();
        let Some(dev_id) = pollster::block_on(context.device(None)) else {
            eprintln!("pas de GPU ici : comparaison sautée");
            return;
        };
        let handle = &context.devices[dev_id];
        let (device, queue) = (&handle.device, &handle.queue);
        let mut quick = Quick::new(device);
        let mut vello = VelloPath::new(device).expect("Vello");
        let mut worst = (0u8, 0.0f64, 0.0f64);
        for (name, list) in sample_lists(false) {
            let fast = target(device, &list);
            let fast_view = fast.create_view(&Default::default());
            let mut encoder = device.create_command_encoder(&Default::default());
            quick.render(device, queue, &mut encoder, &fast_view, FORMAT, &list);
            queue.submit([encoder.finish()]);

            let slow = target(device, &list);
            let slow_view = slow.create_view(&Default::default());
            let (_storage, storage_view) = create_target(device, list.width, list.height);
            vello
                .paint(
                    device,
                    queue,
                    &list,
                    &storage_view,
                    (list.width, list.height),
                )
                .expect("Vello");
            let mut encoder = device.create_command_encoder(&Default::default());
            let blitter = vello.blitter(device, FORMAT);
            clear_and_blit(device, &mut encoder, blitter, &storage_view, &slow_view);
            queue.submit([encoder.finish()]);

            let (a, b) = (read(device, queue, &fast), read(device, queue, &slow));
            if let Ok(dir) = std::env::var("MOUFLON_COMPARAISON") {
                let file = format!("{dir}/{}", name.replace([' ', '×'], "-"));
                for (suffix, pixels) in [("rapide", &a), ("vello", &b)] {
                    let bitmap = mouflon_render::display::Bitmap::from_rgba(
                        list.width,
                        list.height,
                        pixels.clone(),
                    );
                    let png = mouflon_render::sprites::encode_png_rgba(&bitmap);
                    std::fs::write(format!("{file}-{suffix}.png"), png).expect("png");
                }
            }
            let (max, mean, far) = compare(&a, &b);
            eprintln!(
                "{name} : écart max {max}, moyen {mean:.4}, {:.3} % au-delà de 6",
                far * 100.0
            );
            worst = (worst.0.max(max), worst.1.max(mean), worst.2.max(far));
            assert!(
                a.iter().skip(3).step_by(4).any(|&alpha| alpha > 0),
                "image vide"
            );
        }
        eprintln!("pire : {worst:?}");
        assert!(worst.1 < 0.35, "écart moyen {}", worst.1);
        assert!(worst.2 < 0.01, "trop de pixels différents : {}", worst.2);
    }

    /// Paramètres passés avec chaque commande (« immediates ») ou par un
    /// tampon uniforme : exactement la même image.
    #[test]
    fn immediates_and_uniforms_draw_the_same() {
        let mut context = GpuContext::new();
        let Some(dev_id) = pollster::block_on(context.device(None)) else {
            eprintln!("pas de GPU ici : comparaison sautée");
            return;
        };
        let handle = &context.devices[dev_id];
        let (device, queue) = (&handle.device, &handle.queue);
        let mut fast = Quick::new(device);
        let mut slow = Quick::with_uniforms(device);
        eprintln!(
            "« immediates » : {} (jusqu'à {} octets)",
            device.features().contains(wgpu::Features::IMMEDIATES),
            device.limits().max_immediate_size
        );
        for (name, list) in sample_lists(false) {
            let images = [&mut fast, &mut slow].map(|quick| {
                let texture = target(device, &list);
                let view = texture.create_view(&Default::default());
                let mut encoder = device.create_command_encoder(&Default::default());
                quick.render(device, queue, &mut encoder, &view, FORMAT, &list);
                queue.submit([encoder.finish()]);
                read(device, queue, &texture)
            });
            assert!(images[0] == images[1], "{name} : images différentes");
        }
    }

    /// Les images compressées en BC7 donnent, au rendu direct, presque
    /// exactement les mêmes images que les originales : lues par le GPU, ou
    /// décompressées avant de partir quand il ne sait pas les lire.
    #[test]
    fn compressed_images_look_like_the_originals() {
        let mut context = GpuContext::new();
        let Some(dev_id) = pollster::block_on(context.device(None)) else {
            eprintln!("pas de GPU ici : comparaison sautée");
            return;
        };
        let handle = &context.devices[dev_id];
        let (device, queue) = (&handle.device, &handle.queue);
        let bc7 = device
            .features()
            .contains(wgpu::Features::TEXTURE_COMPRESSION_BC);
        eprintln!("textures BC7 lues par le GPU : {bc7}");
        let originals = sample_lists(false);
        let compressed = sample_lists(true);
        for (gpu_reads_bc7, mut quick) in [
            (bc7, Quick::new(device)),
            (false, Quick::new(device).without_bc7()),
        ] {
            let mut worst = (0u8, 0.0f64, 0.0f64);
            for ((name, original), (_, packed)) in originals.iter().zip(&compressed) {
                let images = [original, packed].map(|list| {
                    let texture = target(device, list);
                    let view = texture.create_view(&Default::default());
                    let mut encoder = device.create_command_encoder(&Default::default());
                    quick.render(device, queue, &mut encoder, &view, FORMAT, list);
                    queue.submit([encoder.finish()]);
                    read(device, queue, &texture)
                });
                let (max, mean, far) = compare(&images[0], &images[1]);
                eprintln!(
                    "{name} (BC7 lu par le GPU : {gpu_reads_bc7}) : écart max {max}, moyen \
                     {mean:.4}, {:.3} % au-delà de 6",
                    far * 100.0
                );
                worst = (worst.0.max(max), worst.1.max(mean), worst.2.max(far));
            }
            eprintln!("pire : {worst:?}");
            assert!(worst.1 < 0.2, "écart moyen {}", worst.1);
            assert!(worst.2 < 0.01, "trop de pixels différents : {}", worst.2);
        }
    }

    /// Un dessin vectoriel cuit en images (le Mannequin), posé par le rendu
    /// direct, ressemble à ses tracés dessinés par Vello : mêmes formes, même
    /// éclairage, aux bords près (les images sont posées tournées et étirées).
    #[test]
    fn a_baked_rig_looks_like_its_vector_drawing() {
        let mut context = GpuContext::new();
        let Some(dev_id) = pollster::block_on(context.device(None)) else {
            eprintln!("pas de GPU ici : comparaison sautée");
            return;
        };
        let handle = &context.devices[dev_id];
        let (device, queue) = (&handle.device, &handle.queue);
        let mut quick = Quick::new(device);
        let mut vello = VelloPath::new(device).expect("Vello");
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/mannequin/pet.toml");
        let def = Arc::new(load_pet(&root).expect("pet.toml"));
        let baked = PetArt::load(&def).expect("dessin");
        assert!(baked.baked.is_some(), "le mannequin se cuit");
        let mut vector = baked.clone();
        vector.baked = None;
        let world = mouflon_core::World::new(vec![mouflon_core::Screen {
            frame: mouflon_core::Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: mouflon_core::Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }]);
        let mut pet = mouflon_core::Pet::spawn(0, Arc::clone(&def), 5, &world, Some(600.0));
        let mut worst = (0u8, 0.0f64, 0.0f64);
        for case in 0..6 {
            for _ in 0..90 {
                pet.step(mouflon_core::TICK, &world, &mouflon_core::Input::default());
            }
            let mut view = pet.view(1.0);
            match case {
                0 => view.facing = 1.0,
                1 => view.facing = -1.0,
                2 => view.pose.squash = 0.2,
                3 => view.pose.roll = 0.5,
                // Il s'efface : ses morceaux ne se voient pas les uns à travers les autres.
                4 => view.opacity = 0.45,
                _ => {}
            }
            let canvas = vector.canvas();
            let mut lists = [&baked, &vector].map(|art| {
                let mut list = DisplayList::new(
                    (f64::from(canvas.width) * 2.0) as u32,
                    (f64::from(canvas.height) * 2.0) as u32,
                );
                let frame = Frame {
                    origin: view.ground - canvas.anchor,
                    scale: 2.0,
                };
                paint_pet(art, &view, &frame, &Style::default(), &mut list);
                list
            });
            let [fast_list, slow_list] = &mut lists;
            assert!(Quick::supports(fast_list), "cas {case} : rendu direct");
            let fast = target(device, fast_list);
            let fast_view = fast.create_view(&Default::default());
            let mut encoder = device.create_command_encoder(&Default::default());
            quick.render(device, queue, &mut encoder, &fast_view, FORMAT, fast_list);
            queue.submit([encoder.finish()]);
            let slow = target(device, slow_list);
            let slow_view = slow.create_view(&Default::default());
            let (_storage, storage_view) = create_target(device, slow_list.width, slow_list.height);
            vello
                .paint(
                    device,
                    queue,
                    slow_list,
                    &storage_view,
                    (slow_list.width, slow_list.height),
                )
                .expect("Vello");
            let mut encoder = device.create_command_encoder(&Default::default());
            let blitter = vello.blitter(device, FORMAT);
            clear_and_blit(device, &mut encoder, blitter, &storage_view, &slow_view);
            queue.submit([encoder.finish()]);
            let (a, b) = (read(device, queue, &fast), read(device, queue, &slow));
            if let Ok(dir) = std::env::var("MOUFLON_COMPARAISON") {
                for (suffix, pixels) in [("cuit", &a), ("vello", &b)] {
                    let bitmap = mouflon_render::display::Bitmap::from_rgba(
                        fast_list.width,
                        fast_list.height,
                        pixels.clone(),
                    );
                    let png = mouflon_render::sprites::encode_png_rgba(&bitmap);
                    std::fs::write(format!("{dir}/mannequin-{case}-{suffix}.png"), png)
                        .expect("png");
                }
            }
            let (max, mean, far) = compare_far(&a, &b, 24);
            eprintln!(
                "mannequin cas {case} ({}) : écart max {max}, moyen {mean:.3}, {:.3} % au-delà de 24",
                pet.behavior_name(),
                far * 100.0
            );
            worst = (worst.0.max(max), worst.1.max(mean), worst.2.max(far));
        }
        eprintln!("pire : {worst:?}");
        assert!(worst.1 < 1.5, "écart moyen {}", worst.1);
        assert!(worst.2 < 0.02, "trop de pixels différents : {}", worst.2);
    }

    /// Comme [`compare`], avec un seuil d'écart choisi.
    fn compare_far(a: &[u8], b: &[u8], threshold: u8) -> (u8, f64, f64) {
        let (max, mean, _) = compare(a, b);
        let far = a
            .iter()
            .zip(b)
            .filter(|(x, y)| x.abs_diff(**y) > threshold)
            .count();
        (max, mean, far as f64 / a.len() as f64)
    }

    /// Plusieurs fenêtres en un seul envoi : chacune reçoit exactement l'image
    /// qu'elle aurait eue seule, même quand le tampon de paramètres doit
    /// grandir en route.
    #[test]
    fn one_submit_draws_each_window_as_if_alone() {
        let mut context = GpuContext::new();
        let Some(dev_id) = pollster::block_on(context.device(None)) else {
            eprintln!("pas de GPU ici : essai sauté");
            return;
        };
        let handle = &context.devices[dev_id];
        let (device, queue) = (&handle.device, &handle.queue);
        let mut quick = Quick::new(device);
        // Trois fois chaque situation : bien plus d'éléments que le tampon
        // de départ n'en reçoit.
        let lists: Vec<(String, DisplayList)> =
            sample_lists(false).into_iter().cycle().take(84).collect();
        let draws: usize = lists.iter().map(|(_, list)| Quick::draws(list)).sum();
        assert!(
            draws > crate::quick::FIRST_CAPACITY,
            "{draws} éléments seulement"
        );

        let alone: Vec<Vec<u8>> = lists
            .iter()
            .map(|(_, list)| {
                let texture = target(device, list);
                let view = texture.create_view(&Default::default());
                let mut encoder = device.create_command_encoder(&Default::default());
                quick.render(device, queue, &mut encoder, &view, FORMAT, list);
                queue.submit([encoder.finish()]);
                read(device, queue, &texture)
            })
            .collect();

        let textures: Vec<wgpu::Texture> =
            lists.iter().map(|(_, list)| target(device, list)).collect();
        quick.begin(device, draws);
        let mut encoder = device.create_command_encoder(&Default::default());
        for ((_, list), texture) in lists.iter().zip(&textures) {
            let view = texture.create_view(&Default::default());
            quick.draw(device, queue, &mut encoder, &view, FORMAT, list);
        }
        quick.finish(queue);
        queue.submit([encoder.finish()]);
        for (((name, _), texture), expected) in lists.iter().zip(&textures).zip(&alone) {
            assert!(
                read(device, queue, texture) == *expected,
                "{name} : image différente"
            );
        }
    }
}
