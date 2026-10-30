//! Rendu direct des animaux en images.
//!
//! Un animal en sprites, c'est une image (une case de sa planche), une ombre
//! floue et quelques particules rondes. Vello sait tout dessiner, mais au
//! prix d'une dizaine de passes de calcul sur toute la fenêtre à chaque
//! image, de tampons de travail et d'une texture intermédiaire. Ici, chaque
//! élément est un simple quadrilatère dessiné droit dans la fenêtre, et son
//! programme calcule sa couleur pixel par pixel :
//!
//! - **image** : la case, téléversée une fois (alpha prémultiplié, bordée
//!   d'un pixel transparent pour que ses bords tournés restent lisses),
//!   lue au plus proche ou par interpolation bilinéaire ;
//! - **ombre** : rectangle arrondi flouté, par la même approximation
//!   analytique que Vello (Raph Levien, « Blurred rounded rectangles ») ;
//! - **disque** : cercle antialiasé, rempli d'une couleur ou d'un dégradé
//!   (linéaire, ou radial à deux cercles comme en SVG).
//!
//! Un calque d'opacité qui ne contient qu'un élément revient à multiplier
//! l'opacité de cet élément : c'est le cas d'un animal qui s'efface. Toute
//! autre liste d'affichage (dessins vectoriels, calques de fusion) reste à
//! Vello : voir [`Quick::supports`].

use std::collections::HashMap;
use std::num::NonZeroU64;

use mouflon_render::display::{Command, DisplayList, Paint, Stop};
use mouflon_render::vello::kurbo::{Affine, Point, Rect, Shape, Vec2};
use mouflon_render::vello::peniko::{Color, Extend, Mix};
use mouflon_render::vello::wgpu;

/// Place d'un élément dans le tampon de paramètres : l'alignement exigé
/// pour un décalage dynamique (256 octets sur tous les GPU).
const SLOT: usize = 256;
/// Éléments par image, au plus (un animal en a rarement plus d'une dizaine).
const MAX_DRAWS: usize = 64;
/// Arrêts de couleur d'un dégradé, au plus.
const MAX_STOPS: usize = 4;
/// Images gardées sur le GPU, en octets : au-delà, les moins récemment
/// montrées sont oubliées (et renvoyées si elles reservent).
const TEXTURE_BUDGET: usize = 32 << 20;

const SHADER: &str = r"
struct Draw {
    // Quadrilatère, en pixels de la fenêtre : origine et deux côtés.
    quad0: vec4<f32>,   // origine x, y ; côté u x, y
    quad1: vec4<f32>,   // côté v x, y ; largeur, hauteur de la fenêtre
    // Pixel → repère de l'élément : local = M · pixel + t.
    inv0: vec4<f32>,    // M : colonne x (x, y), colonne y (x, y)
    inv1: vec4<f32>,    // t x, y ; opacité ; drapeau (image lisse, sorte de remplissage du disque)
    color: vec4<f32>,   // ombre : couleur prémultipliée ; disque : nombre d'arrêts en x
    shape: vec4<f32>,   // ombre : largeur, hauteur, rayon, écart type ; disque : centre x, y, rayon, pixels par unité
    grad0: vec4<f32>,   // dégradé : départ x, y, rayon de départ, extension
    grad1: vec4<f32>,   // dégradé : arrivée x, y, rayon d'arrivée, -
    offsets: vec4<f32>,
    stops: array<vec4<f32>, 4>,
}

@group(0) @binding(0) var<uniform> draw: Draw;
@group(0) @binding(1) var nearest: sampler;
@group(0) @binding(2) var bilinear: sampler;
@group(1) @binding(0) var image: texture_2d<f32>;

struct Varyings {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs(@builtin(vertex_index) index: u32) -> Varyings {
    let uv = vec2(f32(index & 1u), f32(index >> 1u));
    let p = draw.quad0.xy + draw.quad0.zw * uv.x + draw.quad1.xy * uv.y;
    let ndc = vec2(p.x / draw.quad1.z * 2.0 - 1.0, 1.0 - p.y / draw.quad1.w * 2.0);
    return Varyings(vec4(ndc, 0.0, 1.0), uv);
}

fn to_local(pixel: vec2<f32>) -> vec2<f32> {
    return draw.inv0.xy * pixel.x + draw.inv0.zw * pixel.y + draw.inv1.xy;
}

@fragment
fn fs_image(in: Varyings) -> @location(0) vec4<f32> {
    var c: vec4<f32>;
    if draw.inv1.w > 0.5 {
        c = textureSampleLevel(image, bilinear, in.uv, 0.0);
    } else {
        c = textureSampleLevel(image, nearest, in.uv, 0.0);
    }
    return c * draw.inv1.z;
}

// Fonction d'erreur approchée (celle de Vello).
fn erf7(x: f32) -> f32 {
    let y = clamp(x * 1.1283791671, -100.0, 100.0);
    let yy = y * y;
    let z = y + (0.24295 + (0.03395 + 0.0104 * yy) * yy) * (y * yy);
    return z / sqrt(1.0 + z * z);
}

@fragment
fn fs_shadow(in: Varyings) -> @location(0) vec4<f32> {
    // Vello évalue l'ombre au coin du pixel.
    let local = to_local(in.position.xy - vec2(0.5));
    let w = draw.shape.x;
    let h = draw.shape.y;
    let radius = draw.shape.z;
    let std_dev = max(draw.shape.w, 1e-5);
    let inv_std_dev = 1.0 / std_dev;
    let min_edge = min(w, h);
    let radius_max = 0.5 * min_edge;
    let r0 = min(length(vec2(radius, std_dev * 1.15)), radius_max);
    let r1 = min(length(vec2(radius, std_dev * 2.0)), radius_max);
    let exponent = 2.0 * r1 / r0;
    let delta = 1.25 * std_dev
        * (exp(-pow(0.5 * inv_std_dev * w, 2.0)) - exp(-pow(0.5 * inv_std_dev * h, 2.0)));
    let width = w + min(delta, 0.0);
    let height = h - max(delta, 0.0);
    let scale = 0.5 * erf7(inv_std_dev * 0.5 * (max(width, height) - 0.5 * radius));
    let y0 = abs(local.y) - (height * 0.5 - r1);
    let x0 = abs(local.x) - (width * 0.5 - r1);
    let d_pos = pow(pow(max(x0, 0.0), exponent) + pow(max(y0, 0.0), exponent), 1.0 / exponent);
    let d_neg = min(max(x0, y0), 0.0);
    let d = d_pos + d_neg - r1;
    let alpha = scale * (erf7(inv_std_dev * (min_edge + d)) - erf7(inv_std_dev * d));
    return draw.color * clamp(alpha, 0.0, 1.0) * draw.inv1.z;
}

fn extend(t: f32, mode: f32) -> f32 {
    if mode > 1.5 {
        // Miroir.
        let r = t - 2.0 * floor(t * 0.5);
        return select(r, 2.0 - r, r > 1.0);
    }
    if mode > 0.5 {
        return t - floor(t);
    }
    return clamp(t, 0.0, 1.0);
}

fn ramp(t: f32) -> vec4<f32> {
    let n = i32(draw.color.x);
    if t <= draw.offsets[0] || n < 2 {
        return draw.stops[0];
    }
    for (var k = 1; k < n; k++) {
        if t <= draw.offsets[k] {
            let span = max(draw.offsets[k] - draw.offsets[k - 1], 1e-6);
            return mix(draw.stops[k - 1], draw.stops[k], (t - draw.offsets[k - 1]) / span);
        }
    }
    return draw.stops[n - 1];
}

// Dégradé radial à deux cercles : le plus grand t pour lequel le point est
// sur le cercle de centre départ + t (arrivée - départ), de rayon
// r0 + t (r1 - r0) positif.
fn radial(p: vec2<f32>) -> f32 {
    let f = draw.grad0.xy;
    let r0 = draw.grad0.z;
    let d = draw.grad1.xy - f;
    let dr = draw.grad1.z - r0;
    let q = p - f;
    let a = dot(d, d) - dr * dr;
    let b = dot(q, d) + r0 * dr;
    let c = dot(q, q) - r0 * r0;
    if abs(a) < 1e-6 {
        return c / max(2.0 * b, 1e-6);
    }
    let s = sqrt(max(b * b - a * c, 0.0));
    let t1 = (b + s) / a;
    let t2 = (b - s) / a;
    let high = max(t1, t2);
    return select(min(t1, t2), high, r0 + high * dr >= 0.0);
}

@fragment
fn fs_disc(in: Varyings) -> @location(0) vec4<f32> {
    let p = to_local(in.position.xy);
    let center = draw.shape.xy;
    let distance = (length(p - center) - draw.shape.z) * draw.shape.w;
    let cover = clamp(0.5 - distance, 0.0, 1.0);
    var color = draw.stops[0];
    if draw.inv1.w > 1.5 {
        color = ramp(extend(radial(p), draw.grad0.w));
    } else if draw.inv1.w > 0.5 {
        let d = draw.grad1.xy - draw.grad0.xy;
        let t = dot(p - draw.grad0.xy, d) / max(dot(d, d), 1e-12);
        color = ramp(extend(t, draw.grad0.w));
    }
    return color * cover * draw.inv1.z;
}
";

/// Sorte d'élément, donc de programme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Image,
    Shadow,
    Disc,
}

/// Un élément à dessiner : son programme, sa place dans le tampon de
/// paramètres et son image.
#[derive(Clone, Copy, Debug)]
struct Call {
    kind: Kind,
    image: Option<u64>,
}

/// Les trois programmes, pour un format de fenêtre.
#[derive(Debug)]
struct Pipelines {
    image: wgpu::RenderPipeline,
    shadow: wgpu::RenderPipeline,
    disc: wgpu::RenderPipeline,
}

/// Une image sur le GPU.
#[derive(Debug)]
struct Texture {
    bind_group: wgpu::BindGroup,
    _texture: wgpu::Texture,
    bytes: usize,
    last_used: u64,
}

/// Le moteur de rendu direct : programmes, paramètres et images.
#[derive(Debug)]
pub struct Quick {
    module: wgpu::ShaderModule,
    image_layout: wgpu::BindGroupLayout,
    shapes_layout: wgpu::PipelineLayout,
    images_layout: wgpu::PipelineLayout,
    params: wgpu::Buffer,
    params_group: wgpu::BindGroup,
    pipelines: HashMap<wgpu::TextureFormat, Pipelines>,
    textures: HashMap<u64, Texture>,
    texture_bytes: usize,
    /// Numéro de l'image en cours (pour oublier les images les plus anciennes).
    tick: u64,
    /// Paramètres et éléments de l'image en cours (gardés pour ne pas
    /// réallouer à chaque image).
    bytes: Vec<u8>,
    calls: Vec<Call>,
}

impl Quick {
    #[must_use]
    pub fn new(device: &wgpu::Device) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mouflon-rapide"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let params_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mouflon-rapide-parametres"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: NonZeroU64::new(SLOT as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let image_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mouflon-rapide-image"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
        let shapes_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mouflon-rapide-formes"),
            bind_group_layouts: &[Some(&params_layout)],
            immediate_size: 0,
        });
        let images_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mouflon-rapide-images"),
            bind_group_layouts: &[Some(&params_layout), Some(&image_layout)],
            immediate_size: 0,
        });
        let params = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mouflon-rapide-parametres"),
            size: (SLOT * MAX_DRAWS) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let sampler = |filter: wgpu::FilterMode| {
            device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("mouflon-rapide"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                mag_filter: filter,
                min_filter: filter,
                ..Default::default()
            })
        };
        let (nearest, bilinear) = (
            sampler(wgpu::FilterMode::Nearest),
            sampler(wgpu::FilterMode::Linear),
        );
        let params_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mouflon-rapide-parametres"),
            layout: &params_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &params,
                        offset: 0,
                        size: NonZeroU64::new(SLOT as u64),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&nearest),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&bilinear),
                },
            ],
        });
        Self {
            module,
            image_layout,
            shapes_layout,
            images_layout,
            params,
            params_group,
            pipelines: HashMap::new(),
            textures: HashMap::new(),
            texture_bytes: 0,
            tick: 0,
            bytes: Vec::with_capacity(SLOT * 8),
            calls: Vec::with_capacity(8),
        }
    }

    /// Les programmes pour des fenêtres au format `format` (compilés à la
    /// première demande).
    pub fn prepare(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        if self.pipelines.contains_key(&format) {
            return;
        }
        let pipeline = |layout: &wgpu::PipelineLayout, entry: &str| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(entry),
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: &self.module,
                    entry_point: Some("vs"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &self.module,
                    entry_point: Some(entry),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    // Couleurs prémultipliées, comme les attend Core Animation.
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipelines = Pipelines {
            image: pipeline(&self.images_layout, "fs_image"),
            shadow: pipeline(&self.shapes_layout, "fs_shadow"),
            disc: pipeline(&self.shapes_layout, "fs_disc"),
        };
        self.pipelines.insert(format, pipelines);
    }

    /// La liste se dessine-t-elle ici ? Des images, des ombres, des disques,
    /// et des calques d'opacité qui couvrent toute la fenêtre sans rien
    /// fondre ni découper, chacun avec un seul élément.
    #[must_use]
    pub fn supports(list: &DisplayList) -> bool {
        let canvas = list.bounds();
        let mut depth = 0usize;
        let mut inside = 0usize;
        let mut draws = 0usize;
        for command in &list.commands {
            match command {
                Command::Image { .. } | Command::BlurredRect { .. } => {}
                Command::Disc { paint, .. } => {
                    if !paint_supported(paint) {
                        return false;
                    }
                }
                Command::PushLayer {
                    clip,
                    transform,
                    mix,
                    ..
                } => {
                    let covered = transform.transform_rect_bbox(clip.bounding_box());
                    if *mix != Mix::Normal || !contains(covered, canvas) || depth > 0 {
                        return false;
                    }
                    depth += 1;
                    inside = 0;
                    continue;
                }
                Command::PopLayer => {
                    depth = depth.saturating_sub(1);
                    continue;
                }
                Command::Fill { .. } | Command::Stroke { .. } => return false,
            }
            draws += 1;
            if depth > 0 {
                inside += 1;
                if inside > 1 {
                    return false;
                }
            }
        }
        draws <= MAX_DRAWS
    }

    /// Dessine la liste (que [`Self::supports`] accepte) dans `target`, une
    /// texture au format `format` d'abord effacée. Les images manquantes
    /// sont téléversées au passage.
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        list: &DisplayList,
    ) {
        self.prepare(device, format);
        self.tick += 1;
        self.bytes.clear();
        self.calls.clear();
        let size = (f64::from(list.width.max(1)), f64::from(list.height.max(1)));
        let mut alpha = 1.0f32;
        for command in &list.commands {
            if self.calls.len() >= MAX_DRAWS {
                break;
            }
            match command {
                Command::Image {
                    bitmap,
                    transform,
                    alpha: own,
                    smooth,
                } => {
                    let key = bitmap.rgba.id();
                    self.upload(
                        device,
                        queue,
                        key,
                        bitmap.width,
                        bitmap.height,
                        bitmap.rgba.data(),
                    );
                    let (w, h) = (f64::from(bitmap.width), f64::from(bitmap.height));
                    // L'image et sa bordure transparente d'un pixel.
                    let mut p =
                        Params::quad(size, *transform, Rect::new(-1.0, -1.0, w + 1.0, h + 1.0));
                    p.set(ALPHA, own * alpha);
                    p.set(FLAG, if *smooth { 1.0 } else { 0.0 });
                    self.push(Kind::Image, Some(key), &p);
                }
                Command::BlurredRect {
                    rect,
                    radius,
                    std_dev,
                    color,
                    transform,
                } => {
                    // Comme Vello : la gaussienne est coupée à 2,5 écarts types.
                    let kernel = 2.5 * std_dev;
                    let mut p = Params::quad(size, *transform, rect.inflate(kernel, kernel));
                    p.inverse(*transform * Affine::translate(rect.center().to_vec2()));
                    p.set(ALPHA, alpha);
                    p.vec4(4, premultiplied(*color, 1.0));
                    p.vec4(
                        5,
                        [
                            rect.width() as f32,
                            rect.height() as f32,
                            *radius as f32,
                            *std_dev as f32,
                        ],
                    );
                    self.push(Kind::Shadow, None, &p);
                }
                Command::Disc {
                    center,
                    radius,
                    paint,
                    transform,
                } => {
                    let scale = transform.determinant().abs().sqrt().max(1e-9);
                    // Un pixel et demi de plus autour, pour le bord antialiasé.
                    let margin = 1.5 / scale;
                    let r = radius + margin;
                    let bounds = Rect::new(center.x - r, center.y - r, center.x + r, center.y + r);
                    let mut p = Params::quad(size, *transform, bounds);
                    p.inverse(*transform);
                    p.set(ALPHA, alpha);
                    p.vec4(
                        5,
                        [
                            center.x as f32,
                            center.y as f32,
                            *radius as f32,
                            scale as f32,
                        ],
                    );
                    p.paint(paint);
                    self.push(Kind::Disc, None, &p);
                }
                Command::PushLayer { alpha: layer, .. } => alpha = layer.clamp(0.0, 1.0),
                Command::PopLayer => alpha = 1.0,
                Command::Fill { .. } | Command::Stroke { .. } => {}
            }
        }
        if !self.bytes.is_empty() {
            queue.write_buffer(&self.params, 0, &self.bytes);
        }
        let Some(pipelines) = self.pipelines.get(&format) else {
            return;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mouflon-rapide"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
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
        let mut current: Option<Kind> = None;
        for (i, call) in self.calls.iter().enumerate() {
            if current != Some(call.kind) {
                pass.set_pipeline(match call.kind {
                    Kind::Image => &pipelines.image,
                    Kind::Shadow => &pipelines.shadow,
                    Kind::Disc => &pipelines.disc,
                });
                current = Some(call.kind);
            }
            pass.set_bind_group(0, &self.params_group, &[(i * SLOT) as u32]);
            if let Some(texture) = call.image.and_then(|key| self.textures.get(&key)) {
                pass.set_bind_group(1, &texture.bind_group, &[]);
            } else if call.kind == Kind::Image {
                continue;
            }
            pass.draw(0..4, 0..1);
        }
        drop(pass);
        self.forget_old();
    }

    fn push(&mut self, kind: Kind, image: Option<u64>, params: &Params) {
        for value in params.0 {
            self.bytes.extend_from_slice(&value.to_le_bytes());
        }
        self.bytes.resize(self.calls.len() * SLOT + SLOT, 0);
        self.calls.push(Call { kind, image });
    }

    /// Téléverse une image (alpha non prémultiplié) si elle n'est pas déjà
    /// sur le GPU : prémultipliée, et bordée d'un pixel transparent.
    fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        key: u64,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) {
        let tick = self.tick;
        if let Some(texture) = self.textures.get_mut(&key) {
            texture.last_used = tick;
            return;
        }
        let (w, h) = (width as usize, height as usize);
        if rgba.len() < w * h * 4 {
            return;
        }
        let (pw, ph) = (w + 2, h + 2);
        let mut padded = vec![0u8; pw * ph * 4];
        for y in 0..h {
            let src = &rgba[y * w * 4..(y + 1) * w * 4];
            let dst = &mut padded[((y + 1) * pw + 1) * 4..((y + 1) * pw + 1 + w) * 4];
            for (d, s) in dst
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(src.as_chunks::<4>().0)
            {
                let a = u16::from(s[3]);
                // (c · a + 127) / 255, arrondi.
                let mul = |c: u8| ((u16::from(c) * a + 127) / 255) as u8;
                *d = [mul(s[0]), mul(s[1]), mul(s[2]), s[3]];
            }
        }
        let size = wgpu::Extent3d {
            width: pw as u32,
            height: ph as u32,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mouflon-image"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &padded,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pw as u32 * 4),
                rows_per_image: Some(ph as u32),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mouflon-image"),
            layout: &self.image_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });
        self.texture_bytes += padded.len();
        self.textures.insert(
            key,
            Texture {
                bind_group,
                _texture: texture,
                bytes: padded.len(),
                last_used: tick,
            },
        );
    }

    /// Au-delà du budget, oublie les images les moins récemment montrées
    /// (jamais celles de l'image en cours), jusqu'aux trois quarts du budget.
    fn forget_old(&mut self) {
        if self.texture_bytes <= TEXTURE_BUDGET {
            return;
        }
        let mut ages: Vec<(u64, u64)> = self
            .textures
            .iter()
            .filter(|(_, t)| t.last_used < self.tick)
            .map(|(key, t)| (t.last_used, *key))
            .collect();
        ages.sort_unstable();
        for (_, key) in ages {
            if self.texture_bytes <= TEXTURE_BUDGET / 4 * 3 {
                break;
            }
            if let Some(texture) = self.textures.remove(&key) {
                self.texture_bytes -= texture.bytes;
            }
        }
    }

    /// Images sur le GPU et octets qu'elles occupent.
    #[must_use]
    pub fn texture_usage(&self) -> (usize, usize) {
        (self.textures.len(), self.texture_bytes)
    }
}

/// `inner` est-il entièrement dans `outer` (au demi-pixel près) ?
fn contains(outer: Rect, inner: Rect) -> bool {
    outer.x0 <= inner.x0 + 0.5
        && outer.y0 <= inner.y0 + 0.5
        && outer.x1 >= inner.x1 - 0.5
        && outer.y1 >= inner.y1 - 0.5
}

fn paint_supported(paint: &Paint) -> bool {
    match paint {
        Paint::Solid(_) => true,
        Paint::Linear { stops, .. } | Paint::Radial { stops, .. } => {
            !stops.is_empty() && stops.len() <= MAX_STOPS
        }
    }
}

/// Couleur prémultipliée, multipliée par `alpha`.
fn premultiplied(color: Color, alpha: f32) -> [f32; 4] {
    let [r, g, b, a] = color.components;
    let a = a.clamp(0.0, 1.0) * alpha;
    [r * a, g * a, b * a, a]
}

/// Paramètres d'un élément, tels que les lit le programme (voir `Draw` :
/// treize `vec4`, dans cet ordre).
#[derive(Clone, Copy, Debug)]
struct Params([f32; 52]);

/// Places de quelques paramètres isolés dans [`Params`].
const ALPHA: usize = 14;
const FLAG: usize = 15;
const STOP_COUNT: usize = 16;
const OFFSETS: usize = 32;

impl Params {
    /// Quadrilatère couvrant `rect` (dans le repère de l'élément), transformé
    /// par `transform` vers les pixels d'une fenêtre de taille `size`.
    fn quad(size: (f64, f64), transform: Affine, rect: Rect) -> Self {
        let mut p = Self([0.0; 52]);
        let origin = transform * Point::new(rect.x0, rect.y0);
        let [a, b, c, d, _, _] = transform.as_coeffs();
        let u = Vec2::new(a * rect.width(), b * rect.width());
        let v = Vec2::new(c * rect.height(), d * rect.height());
        p.vec4(
            0,
            [origin.x as f32, origin.y as f32, u.x as f32, u.y as f32],
        );
        p.vec4(1, [v.x as f32, v.y as f32, size.0 as f32, size.1 as f32]);
        p
    }

    /// Passage des pixels de la fenêtre au repère de l'élément : l'inverse de `to_pixels`.
    fn inverse(&mut self, to_pixels: Affine) {
        let [a, b, c, d, e, f] = to_pixels.inverse().as_coeffs();
        self.vec4(2, [a as f32, b as f32, c as f32, d as f32]);
        self.set(12, e as f32);
        self.set(13, f as f32);
    }

    /// Remplissage d'un disque : couleur unie (drapeau 0), dégradé linéaire
    /// (1) ou radial (2).
    fn paint(&mut self, paint: &Paint) {
        match paint {
            Paint::Solid(color) => {
                self.set(FLAG, 0.0);
                self.set(STOP_COUNT, 1.0);
                self.vec4(9, premultiplied(*color, 1.0));
            }
            Paint::Linear {
                start,
                end,
                stops,
                extend,
            } => {
                self.set(FLAG, 1.0);
                self.vec4(
                    6,
                    [start.x as f32, start.y as f32, 0.0, extend_code(*extend)],
                );
                self.vec4(7, [end.x as f32, end.y as f32, 0.0, 0.0]);
                self.stops(stops);
            }
            Paint::Radial {
                start,
                start_radius,
                end,
                end_radius,
                stops,
                extend,
            } => {
                self.set(FLAG, 2.0);
                self.vec4(
                    6,
                    [
                        start.x as f32,
                        start.y as f32,
                        *start_radius,
                        extend_code(*extend),
                    ],
                );
                self.vec4(7, [end.x as f32, end.y as f32, *end_radius, 0.0]);
                self.stops(stops);
            }
        }
    }

    /// Arrêts de couleur (au plus [`MAX_STOPS`]) : positions et couleurs prémultipliées.
    fn stops(&mut self, stops: &[Stop]) {
        let n = stops.len().min(MAX_STOPS);
        self.set(STOP_COUNT, n as f32);
        for (k, stop) in stops.iter().take(MAX_STOPS).enumerate() {
            self.set(OFFSETS + k, stop.offset);
            self.vec4(9 + k, premultiplied(stop.color, 1.0));
        }
    }

    fn vec4(&mut self, index: usize, values: [f32; 4]) {
        self.0[index * 4..index * 4 + 4].copy_from_slice(&values);
    }

    fn set(&mut self, index: usize, value: f32) {
        self.0[index] = value;
    }
}

fn extend_code(extend: Extend) -> f32 {
    match extend {
        Extend::Pad => 0.0,
        Extend::Repeat => 1.0,
        Extend::Reflect => 2.0,
    }
}
