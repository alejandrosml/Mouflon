//! Rendu GPU : Vello dessine dans une texture intermédiaire, puis une passe
//! la recopie dans la fenêtre en prémultipliant l'alpha (ce qu'attend Core
//! Animation pour une fenêtre transparente).

use std::num::NonZeroUsize;
use std::sync::Arc;

use mouflon_render::DisplayList;
use mouflon_render::vello::util::RenderContext;
use mouflon_render::vello::wgpu::{self, util::TextureBlitter, util::TextureBlitterBuilder};
use mouflon_render::vello::{
    AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene, peniko::Color,
};
use winit::window::Window;

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

/// Fenêtre de rendu d'un animal.
pub struct PetSurface {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    dev_id: usize,
    /// Gardée en vie pour la vue ci-dessous.
    _target: wgpu::Texture,
    target_view: wgpu::TextureView,
}

impl std::fmt::Debug for PetSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PetSurface")
            .field("size", &(self.config.width, self.config.height))
            .finish()
    }
}

pub struct Gpu {
    context: RenderContext,
    renderer: Option<Renderer>,
    blitter: Option<(wgpu::TextureFormat, TextureBlitter)>,
    scene: Scene,
}

impl std::fmt::Debug for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu")
            .field("devices", &self.context.devices.len())
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

impl Gpu {
    #[must_use]
    pub fn new() -> Self {
        Self {
            context: RenderContext::new(),
            renderer: None,
            blitter: None,
            scene: Scene::new(),
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
        let caps = surface.get_capabilities(handle.adapter());
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
        let (target, target_view) = create_target(&handle.device, config.width, config.height);

        if self.renderer.is_none() {
            let renderer = Renderer::new(
                &handle.device,
                RendererOptions {
                    use_cpu: false,
                    antialiasing_support: AaSupport::area_only(),
                    num_init_threads: NonZeroUsize::new(1),
                    pipeline_cache: None,
                },
            )?;
            self.renderer = Some(renderer);
        }
        if self.blitter.as_ref().is_none_or(|(f, _)| *f != format) {
            // Recopie en prémultipliant : couleur × alpha, sur une cible effacée.
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
            let blitter = TextureBlitterBuilder::new(&handle.device, format)
                .blend_state(premultiply)
                .build();
            self.blitter = Some((format, blitter));
        }
        Ok(PetSurface {
            surface,
            config,
            dev_id,
            _target: target,
            target_view,
        })
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
        let (target, view) = create_target(device, width, height);
        surface._target = target;
        surface.target_view = view;
    }

    /// Dessine une image dans la fenêtre. Renvoie `false` si l'image a été sautée.
    pub fn render(
        &mut self,
        surface: &mut PetSurface,
        list: &DisplayList,
        window: &Window,
    ) -> bool {
        let Some(renderer) = self.renderer.as_mut() else {
            return false;
        };
        let Some((_, blitter)) = self.blitter.as_ref() else {
            return false;
        };
        let handle = &self.context.devices[surface.dev_id];
        self.scene.reset();
        mouflon_render::vello_scene::encode(list, &mut self.scene);
        let params = RenderParams {
            base_color: Color::TRANSPARENT,
            width: surface.config.width,
            height: surface.config.height,
            antialiasing_method: AaConfig::Area,
        };
        if let Err(e) = renderer.render_to_texture(
            &handle.device,
            &handle.queue,
            &self.scene,
            &surface.target_view,
            &params,
        ) {
            log::warn!("rendu Vello : {e}");
            return false;
        }
        let frame = match surface.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                surface.surface.configure(&handle.device, &surface.config);
                return false;
            }
            _ => return false,
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = handle
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mouflon"),
            });
        {
            let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mouflon-effacement"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
        blitter.copy(&handle.device, &mut encoder, &surface.target_view, &view);
        handle.queue.submit([encoder.finish()]);
        window.pre_present_notify();
        handle.queue.present(frame);
        true
    }
}
