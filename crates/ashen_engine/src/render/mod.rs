//! Deferred-2D-Renderer für isometrische Pixel-Art.
//!
//! Frame-Ablauf (alle Szenen-Passes in niedriger interner Auflösung):
//! 1. **G-Buffer**: Boden, Schatten, sortierte Weltobjekte, Hitze, Overlays → Albedo + Normalen/Emission/Flags
//! 2. **Licht**: instanzierte Punktlichter, additiv in HDR
//! 3. **Komposition**: Umgebung + Sonne/Mond + Lichter + Emission, Kriegsnebel, Nebel, Wetter
//! 4. **Bloom**: Helligkeitsfilter + Down-/Upsample-Kette (profilabhängig)
//! 5. **Ausgabe**: ganzzahlige Hochskalierung mit Subpixel-Scrolling, Tonemapping, Farbkorrektur

pub mod camera;
pub mod particles;
pub mod profile;
pub mod shaders;
pub mod types;

pub use camera::{Camera2D, ViewParams};
pub use profile::{GraphicsProfile, ProfileSettings};
pub use types::{Atmosphere, Globals, Light, Material, SpriteInstance, SpriteRect};

use crate::gpu::GpuContext;
use glam::Vec2;
use shaders::ShaderManager;

const ALBEDO_FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const NORMAL_FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const HDR_FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

/// Alles, was das Spiel für einen Frame zeichnen möchte.
#[derive(Default)]
pub struct FrameData {
    pub camera: Camera2D,
    /// Bodenkacheln, in Reihenfolge gezeichnet.
    pub ground: Vec<SpriteInstance>,
    /// Geworfene Schatten (nur Schattenmaske).
    pub shadows: Vec<SpriteInstance>,
    /// Weltobjekte mit Tiefenschlüssel (größer = weiter vorne).
    pub world: Vec<(f32, SpriteInstance)>,
    /// Hitzeflimmern (nur Verzerrungsmaske).
    pub heat: Vec<SpriteInstance>,
    /// Unbeleuchtete Overlays, zuletzt gezeichnet.
    pub overlay: Vec<SpriteInstance>,
    pub lights: Vec<Light>,
    pub atmosphere: Atmosphere,
}

impl FrameData {
    pub fn clear(&mut self) {
        self.ground.clear();
        self.shadows.clear();
        self.world.clear();
        self.heat.clear();
        self.overlay.clear();
        self.lights.clear();
    }
}

/// GPU-Zeitstempel je Render-Pass (nur mit `TIMESTAMP_QUERY`, siehe `GpuContext::with_timing`).
struct GpuTimer {
    set: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    read: wgpu::Buffer,
    labels: Vec<&'static str>,
    period_ns: f32,
}

const MAX_TIMED_PASSES: u32 = 24;

impl GpuTimer {
    fn new(gpu: &GpuContext) -> Option<GpuTimer> {
        if !gpu.device.features().contains(wgpu::Features::TIMESTAMP_QUERY) {
            return None;
        }
        let bytes = MAX_TIMED_PASSES as u64 * 2 * 8;
        Some(GpuTimer {
            set: gpu.device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("pass-times"), ty: wgpu::QueryType::Timestamp, count: MAX_TIMED_PASSES * 2 }),
            resolve: buffer(&gpu.device, "pass-times-resolve", bytes, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC),
            read: buffer(&gpu.device, "pass-times-read", bytes, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST),
            labels: Vec::new(),
            period_ns: gpu.queue.get_timestamp_period(),
        })
    }

    /// Reserviert den nächsten Zeitstempelplatz für einen Pass.
    fn alloc(&mut self, label: &'static str) -> Option<u32> {
        let i = self.labels.len() as u32;
        if i >= MAX_TIMED_PASSES {
            return None;
        }
        self.labels.push(label);
        Some(i)
    }

    fn writes(&self, slot: Option<u32>) -> Option<wgpu::RenderPassTimestampWrites<'_>> {
        slot.map(|i| wgpu::RenderPassTimestampWrites { query_set: &self.set, beginning_of_pass_write_index: Some(i * 2), end_of_pass_write_index: Some(i * 2 + 1) })
    }

    fn resolve(&self, encoder: &mut wgpu::CommandEncoder) {
        let n = self.labels.len() as u32 * 2;
        if n > 0 {
            encoder.resolve_query_set(&self.set, 0..n, &self.resolve, 0);
            encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.read, 0, n as u64 * 8);
        }
    }
}

/// Statistik des letzten Frames.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderStats {
    pub sprites: usize,
    pub lights: usize,
    pub draw_calls: usize,
    /// Sichtbare Instanzen je Gruppe: Boden, Schatten, Welt, Hitzeflimmern+Overlay.
    pub groups: [usize; 4],
    pub internal: (u32, u32),
    pub pixel_scale: u32,
}

struct Tex {
    _tex: wgpu::Texture,
    /// Nicht-sRGB-Ansicht (nur Atlas): für egui, das Farben selbst im Gammaraum behandelt.
    ui_view: Option<wgpu::TextureView>,
    view: wgpu::TextureView,
    size: (u32, u32),
}

fn make_tex(device: &wgpu::Device, label: &str, size: (u32, u32), format: wgpu::TextureFormat, usage: wgpu::TextureUsages) -> Tex {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width: size.0.max(1), height: size.1.max(1), depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
        view_formats: if format == ALBEDO_FMT && !usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT) { &[wgpu::TextureFormat::Rgba8Unorm] } else { &[] },
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let ui_view = (format == ALBEDO_FMT && !usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT))
        .then(|| tex.create_view(&wgpu::TextureViewDescriptor { format: Some(wgpu::TextureFormat::Rgba8Unorm), ..Default::default() }));
    Tex { _tex: tex, view, ui_view, size }
}

const RT: wgpu::TextureUsages = wgpu::TextureUsages::RENDER_ATTACHMENT.union(wgpu::TextureUsages::TEXTURE_BINDING);

struct Targets {
    size: (u32, u32),
    bloom_levels: u32,
    albedo: Tex,
    normal: Tex,
    light: Tex,
    scene: Tex,
    bloom: Vec<Tex>,
    light_bg: wgpu::BindGroup,
    composite_bg: wgpu::BindGroup,
    final_bg: wgpu::BindGroup,
    /// Bindgruppen der Bloom-Passes: [Prefilter(scene), Down(b0→b1), …, Up(b_n→b_n-1), …]
    bloom_down_bgs: Vec<wgpu::BindGroup>,
    bloom_up_bgs: Vec<wgpu::BindGroup>,
    bloom_bufs: Vec<wgpu::Buffer>,
}

struct Pipelines {
    gbuffer: wgpu::RenderPipeline,
    shadow: wgpu::RenderPipeline,
    heat: wgpu::RenderPipeline,
    light: wgpu::RenderPipeline,
    composite: wgpu::RenderPipeline,
    bloom_prefilter: wgpu::RenderPipeline,
    bloom_down: wgpu::RenderPipeline,
    bloom_up: wgpu::RenderPipeline,
    final_: wgpu::RenderPipeline,
}

struct Layouts {
    globals: wgpu::BindGroupLayout,
    sprite: wgpu::BindGroupLayout,
    light: wgpu::BindGroupLayout,
    composite: wgpu::BindGroupLayout,
    bloom: wgpu::BindGroupLayout,
    final_: wgpu::BindGroupLayout,
}

pub struct Renderer {
    output_format: wgpu::TextureFormat,
    profile: GraphicsProfile,
    pub settings: ProfileSettings,
    shaders: ShaderManager,
    layouts: Layouts,
    pipelines: Pipelines,
    globals_buf: wgpu::Buffer,
    globals_bg: wgpu::BindGroup,
    sampler_point: wgpu::Sampler,
    sampler_linear: wgpu::Sampler,
    atlas_albedo: Tex,
    atlas_normal: Tex,
    sprite_bg: wgpu::BindGroup,
    fow: Tex,
    fow_revision: Option<u32>,
    dummy_black: Tex,
    targets: Option<Targets>,
    instance_buf: wgpu::Buffer,
    instance_cap: usize,
    light_buf: wgpu::Buffer,
    light_cap: usize,
    scratch: Vec<SpriteInstance>,
    time: f32,
    frame: u64,
    pub last_view: Option<ViewParams>,
    pub stats: RenderStats,
    gpu_timer: Option<GpuTimer>,
}

fn tex_entry(binding: u32, vis: wgpu::ShaderStages, filterable: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: vis,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn sampler_entry(binding: u32, filtering: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(if filtering { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering }),
        count: None,
    }
}

fn uniform_entry(binding: u32, vis: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: vis,
        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

fn create_layouts(device: &wgpu::Device) -> Layouts {
    let f = wgpu::ShaderStages::FRAGMENT;
    let vf = wgpu::ShaderStages::VERTEX_FRAGMENT;
    let bgl = |label: &str, entries: &[wgpu::BindGroupLayoutEntry]| device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(label), entries });
    Layouts {
        globals: bgl("globals", &[uniform_entry(0, vf)]),
        sprite: bgl("sprite", &[tex_entry(0, f, false), tex_entry(1, f, false), sampler_entry(2, false)]),
        light: bgl("light", &[tex_entry(0, f, false)]),
        composite: bgl(
            "composite",
            &[tex_entry(0, f, false), tex_entry(1, f, false), tex_entry(2, f, false), tex_entry(3, f, true), sampler_entry(4, false), sampler_entry(5, true)],
        ),
        bloom: bgl("bloom", &[tex_entry(0, f, true), sampler_entry(1, true), uniform_entry(2, f)]),
        final_: bgl("final", &[tex_entry(0, f, false), tex_entry(1, f, true), sampler_entry(2, false), sampler_entry(3, true)]),
    }
}

const SPRITE_ATTRS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
    0 => Float32x2, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4
];
const LIGHT_ATTRS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4];

fn create_pipelines(device: &wgpu::Device, shaders: &ShaderManager, l: &Layouts, output: wgpu::TextureFormat) -> Result<Pipelines, String> {
    let sprite_sh = shaders::compile(device, "sprite", &shaders.source("sprite"))?;
    let light_sh = shaders::compile(device, "light", &shaders.source("light"))?;
    let comp_sh = shaders::compile(device, "composite", &shaders.source("composite"))?;
    let bloom_sh = shaders::compile(device, "bloom", &shaders.source("bloom"))?;
    let final_sh = shaders::compile(device, "final", &shaders.source("final"))?;

    let layout = |label: &str, groups: &[&wgpu::BindGroupLayout]| {
        let groups: Vec<Option<&wgpu::BindGroupLayout>> = groups.iter().map(|g| Some(*g)).collect();
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some(label), bind_group_layouts: &groups, immediate_size: 0 })
    };
    let sprite_layout = layout("sprite", &[&l.globals, &l.sprite]);
    let light_layout = layout("light", &[&l.globals, &l.light]);
    let comp_layout = layout("composite", &[&l.globals, &l.composite]);
    let bloom_layout = layout("bloom", &[&l.globals, &l.bloom]);
    let final_layout = layout("final", &[&l.globals, &l.final_]);

    let sprite_vb = [wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<SpriteInstance>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SPRITE_ATTRS,
    }];
    let light_vb = [wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<Light>() as u64, step_mode: wgpu::VertexStepMode::Instance, attributes: &LIGHT_ATTRS }];

    let sprite_pipe = |label: &str, entry: &str, w0: wgpu::ColorWrites, w1: wgpu::ColorWrites| {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&sprite_layout),
            vertex: wgpu::VertexState { module: &sprite_sh, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[Some(sprite_vb[0].clone())] },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &sprite_sh,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                targets: &[
                    Some(wgpu::ColorTargetState { format: ALBEDO_FMT, blend: None, write_mask: w0 }),
                    Some(wgpu::ColorTargetState { format: NORMAL_FMT, blend: None, write_mask: w1 }),
                ],
            }),
            multiview_mask: None,
            cache: None,
        })
    };
    let gbuffer = sprite_pipe("sprite-gbuffer", "fs_gbuffer", wgpu::ColorWrites::ALL, wgpu::ColorWrites::ALL);
    let shadow = sprite_pipe("sprite-shadow", "fs_shadow", wgpu::ColorWrites::empty(), wgpu::ColorWrites::ALPHA);
    let heat = sprite_pipe("sprite-heat", "fs_heat", wgpu::ColorWrites::ALPHA, wgpu::ColorWrites::empty());

    let additive = wgpu::BlendState {
        color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add },
        alpha: wgpu::BlendComponent::OVER,
    };
    let light = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("light"),
        layout: Some(&light_layout),
        vertex: wgpu::VertexState { module: &light_sh, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[Some(light_vb[0].clone())] },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &light_sh,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState { format: HDR_FMT, blend: Some(additive), write_mask: wgpu::ColorWrites::ALL })],
        }),
        multiview_mask: None,
        cache: None,
    });

    let fullscreen = |label: &str, pl: &wgpu::PipelineLayout, module: &wgpu::ShaderModule, entry: &str, format: wgpu::TextureFormat, blend: Option<wgpu::BlendState>| {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(pl),
            vertex: wgpu::VertexState { module, entry_point: Some("vs_fullscreen"), compilation_options: Default::default(), buffers: &[] },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })],
            }),
            multiview_mask: None,
            cache: None,
        })
    };
    Ok(Pipelines {
        gbuffer,
        shadow,
        heat,
        light,
        composite: fullscreen("composite", &comp_layout, &comp_sh, "fs_main", HDR_FMT, None),
        bloom_prefilter: fullscreen("bloom-prefilter", &bloom_layout, &bloom_sh, "fs_prefilter", HDR_FMT, None),
        bloom_down: fullscreen("bloom-down", &bloom_layout, &bloom_sh, "fs_down", HDR_FMT, None),
        bloom_up: fullscreen("bloom-up", &bloom_layout, &bloom_sh, "fs_up", HDR_FMT, Some(additive)),
        final_: fullscreen("final", &final_layout, &final_sh, "fs_main", output, None),
    })
}

fn buffer(device: &wgpu::Device, label: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size, usage, mapped_at_creation: false })
}

impl Renderer {
    pub fn new(gpu: &GpuContext, output_format: wgpu::TextureFormat, profile: GraphicsProfile) -> Result<Renderer, String> {
        let device = &gpu.device;
        let shaders = ShaderManager::new();
        let layouts = create_layouts(device);
        let pipelines = create_pipelines(device, &shaders, &layouts, output_format)?;
        let globals_buf = buffer(device, "globals", std::mem::size_of::<Globals>() as u64, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let globals_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("globals"),
            layout: &layouts.globals,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: globals_buf.as_entire_binding() }],
        });
        let sampler_point = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("point"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let sampler_linear = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("linear"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let tex_usage = wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST;
        let atlas_albedo = make_tex(device, "atlas-albedo", (1, 1), ALBEDO_FMT, tex_usage);
        let atlas_normal = make_tex(device, "atlas-normal", (1, 1), NORMAL_FMT, tex_usage);
        let sprite_bg = Self::make_sprite_bg(device, &layouts, &atlas_albedo, &atlas_normal, &sampler_point);
        let fow = make_tex(device, "fow", (1, 1), wgpu::TextureFormat::R8Unorm, tex_usage);
        gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &fow._tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &[255],
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1), rows_per_image: Some(1) },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        let dummy_black = make_tex(device, "black", (1, 1), HDR_FMT, tex_usage);
        let instance_cap = 8192;
        let light_cap = 256;
        Ok(Renderer {
            output_format,
            profile,
            settings: profile.settings(),
            shaders,
            pipelines,
            globals_buf,
            globals_bg,
            sampler_point,
            sampler_linear,
            sprite_bg,
            atlas_albedo,
            atlas_normal,
            fow,
            fow_revision: None,
            dummy_black,
            targets: None,
            instance_buf: buffer(device, "instances", (instance_cap * std::mem::size_of::<SpriteInstance>()) as u64, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST),
            instance_cap,
            light_buf: buffer(device, "lights", (light_cap * std::mem::size_of::<Light>()) as u64, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST),
            light_cap,
            layouts,
            scratch: Vec::new(),
            time: 0.0,
            frame: 0,
            last_view: None,
            stats: RenderStats::default(),
            gpu_timer: GpuTimer::new(gpu),
        })
    }

    fn make_sprite_bg(device: &wgpu::Device, l: &Layouts, a: &Tex, n: &Tex, s: &wgpu::Sampler) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite"),
            layout: &l.sprite,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&a.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&n.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(s) },
            ],
        })
    }

    pub fn profile(&self) -> GraphicsProfile {
        self.profile
    }

    pub fn set_profile(&mut self, p: GraphicsProfile) {
        if p != self.profile {
            self.profile = p;
            self.settings = p.settings();
            self.targets = None;
        }
    }

    pub fn output_format(&self) -> wgpu::TextureFormat {
        self.output_format
    }

    /// Lädt den Sprite-Atlas (Albedo sRGB + Normalen/Emission) auf die GPU.
    pub fn set_atlas(&mut self, gpu: &GpuContext, size: (u32, u32), albedo: &[u8], normal: &[u8]) {
        let device = &gpu.device;
        let usage = wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST;
        self.atlas_albedo = make_tex(device, "atlas-albedo", size, ALBEDO_FMT, usage);
        self.atlas_normal = make_tex(device, "atlas-normal", size, NORMAL_FMT, usage);
        for (t, data) in [(&self.atlas_albedo, albedo), (&self.atlas_normal, normal)] {
            gpu.queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &t._tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                data,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(size.0 * 4), rows_per_image: Some(size.1) },
                wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
            );
        }
        self.sprite_bg = Self::make_sprite_bg(device, &self.layouts, &self.atlas_albedo, &self.atlas_normal, &self.sampler_point);
    }

    /// Textur-View des Albedo-Atlas (z. B. für UI-Porträts über egui).
    pub fn atlas_view(&self) -> &wgpu::TextureView {
        self.atlas_albedo.ui_view.as_ref().unwrap_or(&self.atlas_albedo.view)
    }

    /// Kriegsnebel hochladen (0 = unerforscht, 1 = erforscht, 2 = sichtbar). Nur bei neuer Revision.
    pub fn set_fog_of_war(&mut self, gpu: &GpuContext, size: (u32, u32), cells: &[u8], revision: u32) {
        if self.fow_revision == Some(revision) && self.fow.size == size {
            return;
        }
        if self.fow.size != size {
            self.fow = make_tex(&gpu.device, "fow", size, wgpu::TextureFormat::R8Unorm, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST);
            self.targets = None;
        }
        let data: Vec<u8> = cells.iter().map(|&c| match c {
            0 => 0,
            1 => 128,
            _ => 255,
        }).collect();
        gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.fow._tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &data,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(size.0), rows_per_image: Some(size.1) },
            wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
        );
        self.fow_revision = Some(revision);
    }

    /// Kriegsnebel ausschalten (z. B. Hauptmenü).
    pub fn clear_fog_of_war(&mut self) {
        self.fow_revision = None;
    }

    fn ensure_targets(&mut self, device: &wgpu::Device, size: (u32, u32)) {
        let levels = self.settings.bloom_levels;
        if self.targets.as_ref().is_some_and(|t| t.size == size && t.bloom_levels == levels) {
            return;
        }
        let albedo = make_tex(device, "g-albedo", size, ALBEDO_FMT, RT);
        let normal = make_tex(device, "g-normal", size, NORMAL_FMT, RT);
        let light = make_tex(device, "light", size, HDR_FMT, RT);
        let scene = make_tex(device, "scene", size, HDR_FMT, RT);
        let mut bloom = Vec::new();
        let mut s = (size.0 / 2, size.1 / 2);
        for i in 0..levels {
            bloom.push(make_tex(device, &format!("bloom{i}"), (s.0.max(1), s.1.max(1)), HDR_FMT, RT));
            s = (s.0 / 2, s.1 / 2);
        }
        let light_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light"),
            layout: &self.layouts.light,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&normal.view) }],
        });
        let composite_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("composite"),
            layout: &self.layouts.composite,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&albedo.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&normal.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&light.view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&self.fow.view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler_point) },
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::Sampler(&self.sampler_linear) },
            ],
        });
        let bloom_view = bloom.first().map(|b| &b.view).unwrap_or(&self.dummy_black.view);
        let final_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("final"),
            layout: &self.layouts.final_,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&scene.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(bloom_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.sampler_point) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler_linear) },
            ],
        });
        // Bloom-Bindgruppen: Quelle + Texelgröße der Quelle.
        let mut bloom_bufs = Vec::new();
        let mut bloom_down_bgs = Vec::new();
        let mut bloom_up_bgs = Vec::new();
        let mk = |src: &Tex, bufs: &mut Vec<wgpu::Buffer>| {
            let buf = buffer(device, "bloom-pass", 16, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
            let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bloom"),
                layout: &self.layouts.bloom,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&src.view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler_linear) },
                    wgpu::BindGroupEntry { binding: 2, resource: buf.as_entire_binding() },
                ],
            });
            bufs.push(buf);
            bg
        };
        if !bloom.is_empty() {
            bloom_down_bgs.push(mk(&scene, &mut bloom_bufs));
            for b in &bloom[..bloom.len() - 1] {
                bloom_down_bgs.push(mk(b, &mut bloom_bufs));
            }
            for i in (1..bloom.len()).rev() {
                bloom_up_bgs.push(mk(&bloom[i], &mut bloom_bufs));
            }
        }
        self.targets = Some(Targets { size, bloom_levels: levels, albedo, normal, light, scene, bloom, light_bg, composite_bg, final_bg, bloom_down_bgs, bloom_up_bgs, bloom_bufs });
    }

    /// Prüft Shader-Hot-Reload und baut Pipelines bei Änderung neu.
    pub fn hot_reload(&mut self, gpu: &GpuContext, dt: f32) {
        if self.shaders.poll_changed(dt) {
            match create_pipelines(&gpu.device, &self.shaders, &self.layouts, self.output_format) {
                Ok(p) => {
                    self.pipelines = p;
                    log::info!("Shader neu geladen");
                }
                Err(e) => log::error!("Shader-Neuladen fehlgeschlagen, alte Version bleibt aktiv: {e}"),
            }
        }
    }

    /// Zeichnet einen Frame in `output` (Größe `output_size`).
    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, frame: &mut FrameData, output: &wgpu::TextureView, output_size: (u32, u32), dt: f32, fow_enabled: bool, fow_size: (u32, u32)) {
        self.time += dt;
        self.frame += 1;
        let view = frame.camera.view(output_size, self.settings.min_lines);
        self.last_view = Some(view);
        self.ensure_targets(&gpu.device, view.internal);
        let a = frame.atmosphere;
        let s = &self.settings;
        let (iw, ih) = (view.internal.0 as f32, view.internal.1 as f32);
        let globals = Globals {
            view_proj: view.view_proj().to_cols_array_2d(),
            screen: [iw, ih, 1.0 / iw, 1.0 / ih],
            time: [self.time, dt, self.frame as f32, view.pixel_scale as f32],
            camera: [view.origin.x, view.origin.y, view.subpixel.x, view.subpixel.y],
            ambient: [a.ambient.x, a.ambient.y, a.ambient.z, a.ambient_intensity],
            sun_color: [a.sun_color.x, a.sun_color.y, a.sun_color.z, a.sun_intensity],
            sun_dir: [a.sun_dir.x, a.sun_dir.y, a.sun_dir.z, a.shadow_strength],
            fog: [if s.fog { a.fog_density } else { 0.0 }, a.ground_fog, a.wind.x, a.wind.y],
            fog_color: [a.fog_color.x, a.fog_color.y, a.fog_color.z, a.fog_light_scatter],
            weather: [a.rain, a.ash, a.lightning, a.wetness],
            fow: [fow_size.0 as f32, fow_size.1 as f32, if fow_enabled && self.fow_revision.is_some() { 1.0 } else { 0.0 }, 1.0],
            grade: [a.exposure, a.contrast, a.saturation, a.vignette],
            grade_tint: [a.tint.x, a.tint.y, a.tint.z, s.grain],
            bloom: [a.bloom_intensity, a.bloom_threshold, 0.5, if s.bloom_levels > 0 { 1.0 } else { 0.0 }],
            output: [output_size.0 as f32, output_size.1 as f32, if self.output_format.is_srgb() { 1.0 } else { 0.0 }, view.pixel_scale as f32],
            tile: [ashen_core::iso::TILE_W * 0.5, ashen_core::iso::TILE_H * 0.5, if s.normal_mapping { 1.0 } else { 0.0 }, 1.0],
        };
        gpu.queue.write_buffer(&self.globals_buf, 0, bytemuck::bytes_of(&globals));

        // Sichtbarkeitsprüfung und Zusammenstellung aller Instanzen in einen Puffer.
        let (vmin, vmax) = view.visible_world(4.0);
        let visible = |i: &SpriteInstance| {
            let shear = (i.p1[0] * i.size[1]).abs();
            i.pos[0] - shear <= vmax.x && i.pos[0] + i.size[0] + shear >= vmin.x && i.pos[1] <= vmax.y && i.pos[1] + i.size[1] >= vmin.y
        };
        frame.world.sort_by(|x, y| x.0.total_cmp(&y.0));
        self.scratch.clear();
        let mut ranges = [0u32; 6];
        self.scratch.extend(frame.ground.iter().filter(|i| visible(i)));
        ranges[1] = self.scratch.len() as u32;
        self.scratch.extend(frame.shadows.iter().filter(|i| visible(i)));
        ranges[2] = self.scratch.len() as u32;
        self.scratch.extend(frame.world.iter().map(|(_, i)| *i).filter(visible));
        ranges[3] = self.scratch.len() as u32;
        self.scratch.extend(frame.heat.iter().filter(|i| visible(i)));
        ranges[4] = self.scratch.len() as u32;
        self.scratch.extend(frame.overlay.iter().filter(|i| visible(i)));
        ranges[5] = self.scratch.len() as u32;
        if self.scratch.len() > self.instance_cap {
            self.instance_cap = self.scratch.len().next_power_of_two();
            self.instance_buf = buffer(&gpu.device, "instances", (self.instance_cap * std::mem::size_of::<SpriteInstance>()) as u64, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST);
        }
        if !self.scratch.is_empty() {
            gpu.queue.write_buffer(&self.instance_buf, 0, bytemuck::cast_slice(&self.scratch));
        }

        // Lichter: sichtbare auswählen, auf Profil-Maximum begrenzen (hellste zuerst).
        let mut lights: Vec<Light> = frame
            .lights
            .iter()
            .copied()
            .filter(|l| {
                let r = l.pos_radius[2];
                l.pos_radius[0] + r >= vmin.x && l.pos_radius[0] - r <= vmax.x && l.pos_radius[1] + r >= vmin.y && l.pos_radius[1] - r <= vmax.y
            })
            .collect();
        if lights.len() > s.max_lights {
            lights.sort_by(|x, y| (y.color[3] * y.pos_radius[2]).total_cmp(&(x.color[3] * x.pos_radius[2])));
            lights.truncate(s.max_lights);
        }
        if lights.len() > self.light_cap {
            self.light_cap = lights.len().next_power_of_two();
            self.light_buf = buffer(&gpu.device, "lights", (self.light_cap * std::mem::size_of::<Light>()) as u64, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST);
        }
        if !lights.is_empty() {
            gpu.queue.write_buffer(&self.light_buf, 0, bytemuck::cast_slice(&lights));
        }

        let mut timer = self.gpu_timer.take();
        if let Some(tm) = timer.as_mut() {
            tm.labels.clear();
        }
        let slot = |tm: &mut Option<GpuTimer>, label: &'static str| tm.as_mut().and_then(|t| t.alloc(label));
        let t = self.targets.as_ref().expect("targets ensured");
        let mut draws = 0;
        // 1) G-Buffer
        let ts = slot(&mut timer, "gbuffer");
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gbuffer"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &t.albedo.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.004, g: 0.004, b: 0.008, a: 0.0 }), store: wgpu::StoreOp::Store },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &t.normal.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.5, g: 0.5, b: 0.0, a: 0.0 }), store: wgpu::StoreOp::Store },
                    }),
                ],
                depth_stencil_attachment: None,
                timestamp_writes: timer.as_ref().and_then(|tm| tm.writes(ts)),
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_bind_group(0, &self.globals_bg, &[]);
            pass.set_bind_group(1, &self.sprite_bg, &[]);
            pass.set_vertex_buffer(0, self.instance_buf.slice(..));
            let segments = [
                (&self.pipelines.gbuffer, 0, ranges[1]),
                (&self.pipelines.shadow, ranges[1], ranges[2]),
                (&self.pipelines.gbuffer, ranges[2], ranges[3]),
                (&self.pipelines.heat, ranges[3], ranges[4]),
                (&self.pipelines.gbuffer, ranges[4], ranges[5]),
            ];
            for (pipe, a, b) in segments {
                if b > a {
                    pass.set_pipeline(pipe);
                    pass.draw(0..6, a..b);
                    draws += 1;
                }
            }
        }
        // 2) Lichter
        let ts = slot(&mut timer, "lights");
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lights"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.light.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: timer.as_ref().and_then(|tm| tm.writes(ts)),
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if !lights.is_empty() {
                pass.set_pipeline(&self.pipelines.light);
                pass.set_bind_group(0, &self.globals_bg, &[]);
                pass.set_bind_group(1, &t.light_bg, &[]);
                pass.set_vertex_buffer(0, self.light_buf.slice(..));
                pass.draw(0..6, 0..lights.len() as u32);
                draws += 1;
            }
        }
        // 3) Komposition
        let ts = slot(&mut timer, "composite");
        fullscreen_pass(encoder, "composite", &t.scene.view, &self.pipelines.composite, &self.globals_bg, &t.composite_bg, None, timer.as_ref().and_then(|tm| tm.writes(ts)));
        draws += 1;
        // 4) Bloom
        if !t.bloom.is_empty() {
            let knee = 0.5;
            let mut bi = 0;
            let mut src_size = t.size;
            for (i, dst) in t.bloom.iter().enumerate() {
                let data = [1.0 / src_size.0 as f32, 1.0 / src_size.1 as f32, a.bloom_threshold, knee];
                gpu.queue.write_buffer(&t.bloom_bufs[bi], 0, bytemuck::cast_slice(&data));
                let pipe = if i == 0 { &self.pipelines.bloom_prefilter } else { &self.pipelines.bloom_down };
                let ts = slot(&mut timer, "bloom");
                fullscreen_pass(encoder, "bloom-down", &dst.view, pipe, &self.globals_bg, &t.bloom_down_bgs[i], None, timer.as_ref().and_then(|tm| tm.writes(ts)));
                src_size = dst.size;
                bi += 1;
                draws += 1;
            }
            for (k, i) in (1..t.bloom.len()).rev().enumerate() {
                let src = &t.bloom[i];
                let data = [1.0 / src.size.0 as f32, 1.0 / src.size.1 as f32, 0.0, 0.0];
                gpu.queue.write_buffer(&t.bloom_bufs[bi], 0, bytemuck::cast_slice(&data));
                let ts = slot(&mut timer, "bloom");
                fullscreen_pass(encoder, "bloom-up", &t.bloom[i - 1].view, &self.pipelines.bloom_up, &self.globals_bg, &t.bloom_up_bgs[k], Some(wgpu::LoadOp::Load), timer.as_ref().and_then(|tm| tm.writes(ts)));
                bi += 1;
                draws += 1;
            }
        }
        // 5) Ausgabe
        let ts = slot(&mut timer, "final");
        fullscreen_pass(encoder, "final", output, &self.pipelines.final_, &self.globals_bg, &t.final_bg, None, timer.as_ref().and_then(|tm| tm.writes(ts)));
        draws += 1;
        if let Some(tm) = timer.as_ref() {
            tm.resolve(encoder);
        }
        self.gpu_timer = timer;

        self.stats = RenderStats { sprites: self.scratch.len(), lights: lights.len(), draw_calls: draws, groups: [ranges[1] as usize, (ranges[2] - ranges[1]) as usize, (ranges[3] - ranges[2]) as usize, (ranges[5] - ranges[3]) as usize], internal: view.internal, pixel_scale: view.pixel_scale };
    }

    /// GPU-Zeit je Pass des zuletzt eingereichten Frames in Millisekunden (Bloom-Stufen aufsummiert). Blockiert, bis die GPU
    /// fertig ist; `None` ohne Zeitstempel-Unterstützung. Nur für Messungen gedacht.
    pub fn read_gpu_pass_times(&mut self, gpu: &GpuContext) -> Option<Vec<(&'static str, f64)>> {
        let tm = self.gpu_timer.as_ref()?;
        let n = tm.labels.len() * 2;
        if n == 0 {
            return None;
        }
        let slice = tm.read.slice(..n as u64 * 8);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
        let mut out: Vec<(&'static str, f64)> = Vec::new();
        {
            let data = slice.get_mapped_range().ok()?;
            let ticks: &[u64] = bytemuck::cast_slice(&data);
            for (i, label) in tm.labels.iter().enumerate() {
                let ms = ticks[i * 2 + 1].saturating_sub(ticks[i * 2]) as f64 * tm.period_ns as f64 / 1.0e6;
                match out.iter_mut().find(|(l, _)| l == label) {
                    Some(e) => e.1 += ms,
                    None => out.push((label, ms)),
                }
            }
        }
        tm.read.unmap();
        Some(out)
    }

    /// Welt-zu-Bildschirm-Umrechnung des zuletzt gerenderten Frames.
    pub fn world_to_screen(&self, w: Vec2) -> Option<Vec2> {
        self.last_view.map(|v| v.world_to_screen(w))
    }
}

#[allow(clippy::too_many_arguments)]
fn fullscreen_pass(encoder: &mut wgpu::CommandEncoder, label: &str, target: &wgpu::TextureView, pipe: &wgpu::RenderPipeline, globals: &wgpu::BindGroup, bg: &wgpu::BindGroup, load: Option<wgpu::LoadOp<wgpu::Color>>, timestamps: Option<wgpu::RenderPassTimestampWrites<'_>>) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations { load: load.unwrap_or(wgpu::LoadOp::Clear(wgpu::Color::BLACK)), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: timestamps,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipe);
    pass.set_bind_group(0, globals, &[]);
    pass.set_bind_group(1, bg, &[]);
    pass.draw(0..3, 0..1);
}
