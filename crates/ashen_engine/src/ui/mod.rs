//! egui-Integration (Eingabe über winit, Zeichnen über wgpu) und Dark-Fantasy-Theme.

pub mod theme;

use crate::gpu::GpuContext;
use winit::window::Window;

pub struct EguiLayer {
    pub ctx: egui::Context,
    state: Option<egui_winit::State>,
    renderer: egui_wgpu::Renderer,
    start: std::time::Instant,
}

impl EguiLayer {
    pub fn new(gpu: &GpuContext, format: wgpu::TextureFormat, window: Option<&Window>) -> EguiLayer {
        let ctx = egui::Context::default();
        theme::apply(&ctx);
        let max_tex = gpu.device.limits().max_texture_dimension_2d as usize;
        let state = window.map(|w| egui_winit::State::new(ctx.clone(), egui::ViewportId::ROOT, w, Some(w.scale_factor() as f32), None, Some(max_tex)));
        let renderer = egui_wgpu::Renderer::new(&gpu.device, format, egui_wgpu::RendererOptions::default());
        EguiLayer { ctx, state, renderer, start: std::time::Instant::now() }
    }

    pub fn on_window_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> egui_winit::EventResponse {
        match &mut self.state {
            Some(s) => s.on_window_event(window, event),
            None => egui_winit::EventResponse { consumed: false, repaint: false },
        }
    }

    /// Eingabe für den nächsten egui-Frame. Ohne Fenster wird sie synthetisch erzeugt (Headless).
    pub fn take_input(&mut self, window: Option<&Window>, size: (u32, u32), ppp: f32) -> egui::RawInput {
        if let (Some(s), Some(w)) = (&mut self.state, window) {
            return s.take_egui_input(w);
        }
        let mut raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size.0 as f32 / ppp, size.1 as f32 / ppp))),
            time: Some(self.start.elapsed().as_secs_f64()),
            ..Default::default()
        };
        raw.viewports.entry(egui::ViewportId::ROOT).or_default().native_pixels_per_point = Some(ppp);
        raw
    }

    pub fn handle_output(&mut self, window: Option<&Window>, output: egui::PlatformOutput) {
        if let (Some(s), Some(w)) = (&mut self.state, window) {
            s.handle_platform_output(w, output);
        }
    }

    /// Registriert eine GPU-Textur für die Verwendung in egui (z. B. Atlas für Porträts).
    pub fn register_texture(&mut self, gpu: &GpuContext, view: &wgpu::TextureView) -> egui::TextureId {
        self.renderer.register_native_texture(&gpu.device, view, wgpu::FilterMode::Nearest)
    }

    pub fn update_texture(&mut self, gpu: &GpuContext, id: egui::TextureId, view: &wgpu::TextureView) {
        self.renderer.update_egui_texture_from_wgpu_texture(&gpu.device, view, wgpu::FilterMode::Nearest, id);
    }

    /// Tesselliert und zeichnet die UI über das bereits gerenderte Bild.
    pub fn paint(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, size: (u32, u32), mut full: egui::FullOutput) {
        let ppp = full.pixels_per_point;
        let jobs = self.ctx.tessellate(full.shapes, ppp);
        for (id, deltas) in &full.textures_delta.set {
            for delta in deltas.iter() {
                self.renderer.update_texture(&gpu.device, &gpu.queue, *id, delta);
            }
        }
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: [size.0, size.1], pixels_per_point: ppp };
        let cmd = self.renderer.update_buffers(&gpu.device, &gpu.queue, encoder, &jobs, &screen);
        if !cmd.is_empty() {
            gpu.queue.submit(cmd);
        }
        {
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            let mut pass = pass.forget_lifetime();
            self.renderer.render(&mut pass, &jobs, &screen);
        }
        for id in &full.textures_delta.free {
            self.renderer.free_texture(id);
        }
        full.textures_delta.clear();
    }
}
