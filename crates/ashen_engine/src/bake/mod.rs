//! 3D→Pixel-Bake (Dead-Cells-Prinzip): Low-Poly-Modelle mit Skelettanimation werden in niedriger
//! Auflösung ohne Kantenglättung zu Sprites gerastert. Albedo, Normalen und Leuchtkraft gehen in
//! den Atlas; der Laufzeit-Renderer beleuchtet die Sprites anschließend dynamisch.

pub mod mesh;
pub mod skeleton;

pub use mesh::{hex_rgb, Mat, Mesh, Paint};
pub use skeleton::{Pose, Skeleton};

use crate::gpu::GpuContext;
use bytemuck::{Pod, Zeroable};
use glam::{Mat3, Mat4, Vec2, Vec3};

/// Isometrische Bake-Kamera. Neigung 30° ergibt das 2:1-Kachelraster.
#[derive(Clone, Copy, Debug)]
pub struct BakeCamera {
    /// Drehung des Modells um die Hochachse (Bogenmaß).
    pub yaw: f32,
    /// Pixel pro Welteinheit (1 Einheit = 1 Kachel ⇒ 32/√2).
    pub ppu: f32,
    /// Pixelposition des Modellursprungs in der Zelle.
    pub anchor: Vec2,
}

pub const PITCH: f32 = std::f32::consts::FRAC_PI_6;
pub const PPU: f32 = 22.627_417;

impl BakeCamera {
    /// Projiziert einen Weltpunkt in Zellpixel.
    pub fn project_point(&self, world: Vec3) -> Vec2 {
        let p = self.view() * world;
        Vec2::new(self.anchor.x + p.x * self.ppu, self.anchor.y + p.y * self.ppu)
    }

    fn view(&self) -> Mat3 {
        let yaw = Mat3::from_rotation_y(self.yaw);
        let (s, c) = PITCH.sin_cos();
        // Bildschirmbasis: x rechts, y unten, z zum Betrachter.
        let to_view = Mat3::from_cols(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -c, s), Vec3::new(0.0, s, c));
        to_view * yaw
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BakeVertex {
    pub px: [f32; 2],
    pub depth: f32,
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub obj: [f32; 3],
    pub mat: u32,
    pub cell: [f32; 2],
}

/// Eine zu rasternde Sprite-Zelle.
pub struct BakeCell {
    pub w: u32,
    pub h: u32,
    pub verts: Vec<BakeVertex>,
}

/// Gerastertes Ergebnis (RGBA8, sRGB-Albedo bzw. Normale/Leuchtkraft).
pub struct BakedImage {
    pub w: u32,
    pub h: u32,
    pub albedo: Vec<u8>,
    pub normal: Vec<u8>,
}

/// Posiert ein Mesh und projiziert es in eine Zelle.
pub fn project(mesh: &Mesh, bones: &[Mat4], model: Mat4, cam: &BakeCamera, cell: (u32, u32)) -> Vec<BakeVertex> {
    let view = cam.view();
    let mut out = Vec::with_capacity(mesh.verts.len());
    for v in &mesh.verts {
        let m = model * bones.get(v.bone).copied().unwrap_or(Mat4::IDENTITY);
        let world = m.transform_point3(v.pos);
        let wn = m.transform_vector3(v.normal).normalize_or(Vec3::Y);
        let p = view * world;
        let n = (view * wn).normalize_or(Vec3::Z);
        out.push(BakeVertex {
            px: [cam.anchor.x + p.x * cam.ppu, cam.anchor.y + p.y * cam.ppu],
            depth: (0.5 - p.z / 40.0).clamp(0.0, 1.0),
            normal: n.to_array(),
            color: [v.color.x, v.color.y, v.color.z, v.glow],
            obj: v.pos.to_array(),
            mat: v.mat as u32,
            cell: [cell.0 as f32, cell.1 as f32],
        });
    }
    out
}

const PAGE: u32 = 2048;
const FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Rastert alle Zellen auf der GPU (seitenweise in 2048²-Texturen) und liest sie zurück.
pub fn bake(gpu: &GpuContext, cells: &[BakeCell]) -> Result<Vec<BakedImage>, String> {
    let device = &gpu.device;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("bake"), source: wgpu::ShaderSource::Wgsl(include_str!("bake.wgsl").into()) });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("bake"), bind_group_layouts: &[], immediate_size: 0 });
    let attrs = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32, 2 => Float32x3, 3 => Float32x4, 4 => Float32x3, 5 => Uint32, 6 => Float32x2];
    let vb = wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<BakeVertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &attrs };
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("bake"),
        layout: Some(&layout),
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[Some(vb)] },
        primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState { format: FMT, blend: None, write_mask: wgpu::ColorWrites::ALL }), Some(wgpu::ColorTargetState { format: FMT, blend: None, write_mask: wgpu::ColorWrites::ALL })],
        }),
        multiview_mask: None,
        cache: None,
    });

    // Regal-Packing in Seiten.
    let mut placement = Vec::with_capacity(cells.len());
    let (mut page, mut x, mut y, mut row) = (0u32, 0u32, 0u32, 0u32);
    for c in cells {
        if c.w > PAGE || c.h > PAGE {
            return Err(format!("Bake-Zelle {}×{} zu groß", c.w, c.h));
        }
        if x + c.w > PAGE {
            x = 0;
            y += row + 1;
            row = 0;
        }
        if y + c.h > PAGE {
            page += 1;
            x = 0;
            y = 0;
            row = 0;
        }
        placement.push((page, x, y));
        x += c.w + 1;
        row = row.max(c.h);
    }
    let pages = if cells.is_empty() { 0 } else { page + 1 };
    let mut results: Vec<Option<BakedImage>> = (0..cells.len()).map(|_| None).collect();
    for pg in 0..pages {
        let idx: Vec<usize> = (0..cells.len()).filter(|&i| placement[i].0 == pg).collect();
        let mk = |label: &str, format: wgpu::TextureFormat, usage: wgpu::TextureUsages| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d { width: PAGE, height: PAGE, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        let rt = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        let albedo = mk("bake-albedo", FMT, rt);
        let normal = mk("bake-normal", FMT, rt);
        let depth = mk("bake-depth", wgpu::TextureFormat::Depth32Float, wgpu::TextureUsages::RENDER_ATTACHMENT);
        let mut verts: Vec<BakeVertex> = Vec::new();
        let mut ranges = Vec::new();
        for &i in &idx {
            let s = verts.len() as u32;
            verts.extend_from_slice(&cells[i].verts);
            ranges.push((i, s, verts.len() as u32));
        }
        if verts.is_empty() {
            continue;
        }
        let vbuf = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bake-verts"), size: (verts.len() * std::mem::size_of::<BakeVertex>()) as u64, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        gpu.queue.write_buffer(&vbuf, 0, bytemuck::cast_slice(&verts));
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("bake") });
        {
            let av = albedo.create_view(&Default::default());
            let nv = normal.create_view(&Default::default());
            let dv = depth.create_view(&Default::default());
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bake"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { view: &av, depth_slice: None, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),
                    Some(wgpu::RenderPassColorAttachment { view: &nv, depth_slice: None, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.5, g: 0.5, b: 0.0, a: 0.0 }), store: wgpu::StoreOp::Store } }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &dv, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_vertex_buffer(0, vbuf.slice(..));
            for (i, s, e) in &ranges {
                let (_, x, y) = placement[*i];
                let c = &cells[*i];
                pass.set_viewport(x as f32, y as f32, c.w as f32, c.h as f32, 0.0, 1.0);
                pass.set_scissor_rect(x, y, c.w, c.h);
                pass.draw(*s..*e, 0..1);
            }
        }
        let bpr = PAGE * 4;
        let read = |enc: &mut wgpu::CommandEncoder, tex: &wgpu::Texture| {
            let buf = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bake-read"), size: (bpr * PAGE) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(PAGE) } },
                wgpu::Extent3d { width: PAGE, height: PAGE, depth_or_array_layers: 1 },
            );
            buf
        };
        let ab = read(&mut enc, &albedo);
        let nb = read(&mut enc, &normal);
        gpu.queue.submit(Some(enc.finish()));
        for b in [&ab, &nb] {
            b.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        }
        device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        let adata = ab.slice(..).get_mapped_range().map_err(|e| format!("{e:?}"))?;
        let ndata = nb.slice(..).get_mapped_range().map_err(|e| format!("{e:?}"))?;
        for (i, _, _) in &ranges {
            let (_, x, y) = placement[*i];
            let c = &cells[*i];
            let mut a = Vec::with_capacity((c.w * c.h * 4) as usize);
            let mut n = Vec::with_capacity((c.w * c.h * 4) as usize);
            for row in 0..c.h {
                let start = (((y + row) * PAGE + x) * 4) as usize;
                let end = start + (c.w * 4) as usize;
                a.extend_from_slice(&adata[start..end]);
                n.extend_from_slice(&ndata[start..end]);
            }
            results[*i] = Some(BakedImage { w: c.w, h: c.h, albedo: a, normal: n });
        }
        drop(adata);
        drop(ndata);
        ab.unmap();
        nb.unmap();
    }
    Ok(results.into_iter().map(|r| r.expect("alle Zellen gerastert")).collect())
}
