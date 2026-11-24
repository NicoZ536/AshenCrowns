//! Anwendungskern: Fenster (winit), Hauptschleife, Spiel-Schnittstelle und Headless-Rendering.

use crate::audio::AudioSystem;
use crate::gpu::GpuContext;
use crate::input::{Input, MouseButton};
use crate::render::{FrameData, GraphicsProfile, Renderer};
use crate::time::FrameTimer;
use crate::ui::EguiLayer;
use glam::Vec2;
use std::path::PathBuf;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Fullscreen, Window, WindowId};

/// Startkonfiguration.
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub profile: GraphicsProfile,
    pub audio_output: bool,
    pub assets_dir: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            title: "Ashen Crowns".into(),
            width: 1440,
            height: 900,
            fullscreen: false,
            vsync: true,
            profile: GraphicsProfile::Balanced,
            audio_output: true,
            assets_dir: None,
        }
    }
}

/// Anfragen des Spiels an das Fenster.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowRequest {
    Fullscreen(bool),
    Vsync(bool),
    Exit,
}

/// Engine-Zustand, den das Spiel pro Frame verwenden darf.
pub struct Engine {
    pub gpu: GpuContext,
    pub renderer: Renderer,
    pub audio: AudioSystem,
    pub input: Input,
    pub timer: FrameTimer,
    pub frame: FrameData,
    pub ui: EguiLayer,
    pub window: Option<Arc<Window>>,
    /// Ausgabegröße in physischen Pixeln.
    pub size: (u32, u32),
    pub scale_factor: f32,
    pub requests: Vec<WindowRequest>,
    pub assets_dir: Option<PathBuf>,
    pub headless: bool,
    pub fullscreen: bool,
    pub vsync: bool,
    pub fow_enabled: bool,
    pub fow_size: (u32, u32),
}

impl Engine {
    pub fn request(&mut self, r: WindowRequest) {
        self.requests.push(r);
    }

    /// Mausposition in Weltpixeln (basierend auf dem letzten Frame).
    pub fn mouse_world(&self) -> Vec2 {
        let v = self.frame.camera.view(self.size, self.renderer.settings.min_lines);
        v.screen_to_world(self.input.mouse)
    }
}

/// Schnittstelle zwischen Engine und Spiel.
pub trait Game {
    fn init(&mut self, engine: &mut Engine);
    /// Spiellogik mit variabler Frame-Zeit (feste Simulationsschritte verwaltet das Spiel).
    fn update(&mut self, engine: &mut Engine, dt: f32);
    /// Benutzeroberfläche (egui).
    fn ui(&mut self, engine: &mut Engine, ui: &mut egui::Ui);
    /// Füllt `engine.frame` mit Sprites, Lichtern und Atmosphäre.
    fn render(&mut self, engine: &mut Engine);
    fn on_exit(&mut self, _engine: &mut Engine) {}
}

/// CPU-Zeiten (Millisekunden) der Frame-Abschnitte; wird nur vom Benchmark ausgewertet.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameSample {
    /// `Game::update`: feste Simulationsschritte, Kamera, Eingabe, Ansichts-Update.
    pub update: f64,
    /// egui-Lauf (`Game::ui`).
    pub ui: f64,
    /// `Game::render`: Ansicht aufbauen (Sprites, Lichter, Partikel, Atmosphäre).
    pub view: f64,
    /// Renderer-Aufbereitung und Befehlsaufzeichnung (CPU) inklusive egui-Malen.
    pub submit: f64,
    /// Einreichen und Warten auf die GPU (nur im Benchmark gemessen).
    pub gpu_wait: f64,
}

/// Mittlere GPU-Zeit je Render-Pass über alle gemessenen Frames (Millisekunden).
pub type PassTimes = Vec<(&'static str, f64)>;

fn run_frame<G: Game>(engine: &mut Engine, game: &mut G, dt: f32, output: &wgpu::TextureView, sample: Option<&mut FrameSample>) -> wgpu::CommandEncoder {
    let t0 = std::time::Instant::now();
    engine.renderer.hot_reload(&engine.gpu, dt);
    game.update(engine, dt);
    let t1 = std::time::Instant::now();
    let window = engine.window.clone();
    let raw = engine.ui.take_input(window.as_deref(), engine.size, engine.scale_factor);
    let ctx = engine.ui.ctx.clone();
    let full = ctx.run_ui(raw, |ui| game.ui(engine, ui));
    engine.input.ui_wants_pointer = ctx.is_pointer_over_egui() || ctx.egui_wants_pointer_input();
    engine.input.ui_wants_keyboard = ctx.egui_wants_keyboard_input();
    let platform = full.platform_output.clone();
    let t2 = std::time::Instant::now();
    engine.frame.clear();
    game.render(engine);
    let t3 = std::time::Instant::now();
    let mut encoder = engine.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });
    let size = engine.size;
    engine.renderer.render(&engine.gpu, &mut encoder, &mut engine.frame, output, size, dt, engine.fow_enabled, engine.fow_size);
    engine.ui.paint(&engine.gpu, &mut encoder, output, size, full);
    engine.ui.handle_output(window.as_deref(), platform);
    engine.audio.update(dt);
    engine.input.end_frame();
    if let Some(sm) = sample {
        let ms = |a: std::time::Instant, b: std::time::Instant| b.duration_since(a).as_secs_f64() * 1000.0;
        sm.update = ms(t0, t1);
        sm.ui = ms(t1, t2);
        sm.view = ms(t2, t3);
        sm.submit = ms(t3, std::time::Instant::now());
    }
    encoder
}

struct App<G: Game> {
    config: AppConfig,
    game: G,
    engine: Option<Engine>,
    surface: Option<wgpu::Surface<'static>>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
    error: Option<String>,
}

impl<G: Game> App<G> {
    fn configure_surface(&mut self) {
        let (Some(engine), Some(surface), Some(cfg)) = (&self.engine, &self.surface, &mut self.surface_config) else { return };
        cfg.width = engine.size.0.max(1);
        cfg.height = engine.size.1.max(1);
        cfg.present_mode = if engine.vsync { wgpu::PresentMode::AutoVsync } else { wgpu::PresentMode::AutoNoVsync };
        surface.configure(&engine.gpu.device, cfg);
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let attrs = Window::default_attributes()
            .with_title(self.config.title.clone())
            .with_inner_size(winit::dpi::LogicalSize::new(self.config.width, self.config.height))
            .with_min_inner_size(winit::dpi::LogicalSize::new(960, 600))
            .with_fullscreen(if self.config.fullscreen { Some(Fullscreen::Borderless(None)) } else { None });
        let window = Arc::new(event_loop.create_window(attrs).map_err(|e| e.to_string())?);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle_from_env(Box::new(event_loop.owned_display_handle())));
        let surface = instance.create_surface(window.clone()).map_err(|e| e.to_string())?;
        let gpu = GpuContext::new(instance, Some(&surface)).map_err(|e| e.to_string())?;
        let caps = surface.get_capabilities(&gpu.adapter);
        // Nicht-sRGB-Format bevorzugen: der Ausgabe-Shader kodiert Gamma selbst, egui ebenso.
        let format = caps.formats.iter().copied().find(|f| !f.is_srgb()).unwrap_or(caps.formats[0]);
        let size = window.inner_size();
        let mut cfg = surface.get_default_config(&gpu.adapter, size.width.max(1), size.height.max(1)).ok_or("Oberfläche nicht konfigurierbar")?;
        cfg.format = format;
        cfg.desired_maximum_frame_latency = 2;
        let renderer = Renderer::new(&gpu, format, self.config.profile)?;
        let ui = EguiLayer::new(&gpu, format, Some(&window));
        let assets_dir = self.config.assets_dir.clone().or_else(crate::assets::find_assets_dir);
        let audio = AudioSystem::new(assets_dir.as_ref().map(|a| a.join("audio")).as_deref(), self.config.audio_output);
        let engine = Engine {
            gpu,
            renderer,
            audio,
            input: Input::new(),
            timer: FrameTimer::default(),
            frame: FrameData::default(),
            ui,
            scale_factor: window.scale_factor() as f32,
            window: Some(window.clone()),
            size: (size.width.max(1), size.height.max(1)),
            requests: Vec::new(),
            assets_dir,
            headless: false,
            fullscreen: self.config.fullscreen,
            vsync: self.config.vsync,
            fow_enabled: false,
            fow_size: (1, 1),
        };
        self.engine = Some(engine);
        self.surface = Some(surface);
        self.surface_config = Some(cfg);
        self.configure_surface();
        let engine = self.engine.as_mut().expect("just set");
        self.game.init(engine);
        window.request_redraw();
        Ok(())
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) {
        let Some(engine) = self.engine.as_mut() else { return };
        let dt = engine.timer.tick();
        let surface = self.surface.as_ref().expect("surface");
        let texture = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return,
            other => {
                log::error!("Oberfläche nicht verfügbar: {other:?}");
                return;
            }
        };
        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = run_frame(engine, &mut self.game, dt, &view, None);
        engine.gpu.queue.submit(Some(encoder.finish()));
        if let Some(w) = &engine.window {
            w.pre_present_notify();
        }
        engine.gpu.queue.present(texture);

        let requests = std::mem::take(&mut engine.requests);
        let mut reconfigure = false;
        for r in requests {
            match r {
                WindowRequest::Exit => {
                    self.game.on_exit(engine);
                    event_loop.exit();
                }
                WindowRequest::Fullscreen(on) => {
                    engine.fullscreen = on;
                    if let Some(w) = &engine.window {
                        w.set_fullscreen(if on { Some(Fullscreen::Borderless(None)) } else { None });
                    }
                }
                WindowRequest::Vsync(on) => {
                    engine.vsync = on;
                    reconfigure = true;
                }
            }
        }
        if reconfigure {
            self.configure_surface();
        }
    }
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.engine.is_none()
            && let Err(e) = self.init(event_loop)
        {
            log::error!("Start fehlgeschlagen: {e}");
            self.error = Some(e);
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(engine) = self.engine.as_mut() else { return };
        if let Some(w) = engine.window.clone() {
            let _ = engine.ui.on_window_event(&w, &event);
        }
        match event {
            WindowEvent::CloseRequested => {
                self.game.on_exit(engine);
                event_loop.exit();
            }
            WindowEvent::Resized(s) => {
                engine.size = (s.width.max(1), s.height.max(1));
                self.configure_surface();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => engine.scale_factor = scale_factor as f32,
            WindowEvent::Focused(f) => {
                engine.input.focused = f;
                if !f {
                    engine.input.reset();
                }
            }
            WindowEvent::CursorEntered { .. } => engine.input.cursor_inside = true,
            WindowEvent::CursorLeft { .. } => engine.input.cursor_inside = false,
            WindowEvent::ModifiersChanged(m) => {
                let s = m.state();
                engine.input.shift = s.shift_key();
                engine.input.ctrl = s.control_key();
                engine.input.alt = s.alt_key();
                engine.input.cmd = s.super_key();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    engine.input.on_key(code, event.state == ElementState::Pressed, event.repeat);
                }
            }
            WindowEvent::CursorMoved { position, .. } => engine.input.on_cursor(Vec2::new(position.x as f32, position.y as f32)),
            WindowEvent::MouseInput { state, button, .. } => {
                let b = match button {
                    winit::event::MouseButton::Left => Some(MouseButton::Left),
                    winit::event::MouseButton::Right => Some(MouseButton::Right),
                    winit::event::MouseButton::Middle => Some(MouseButton::Middle),
                    _ => None,
                };
                if let Some(b) = b {
                    engine.input.on_mouse(b, state == ElementState::Pressed);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                engine.input.wheel += match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 40.0,
                };
            }
            WindowEvent::RedrawRequested => self.redraw(event_loop),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = self.engine.as_ref().and_then(|e| e.window.as_ref()) {
            w.request_redraw();
        }
    }
}

/// Startet das Spiel im Fenster. Kehrt zurück, wenn das Fenster geschlossen wird.
pub fn run<G: Game>(config: AppConfig, game: G) -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App { config, game, engine: None, surface: None, surface_config: None, error: None };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())?;
    match app.error {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// Erzeugt Engine und Zieltextur ohne Fenster (gemeinsam für Screenshots und Benchmark).
fn headless_engine(config: &AppConfig) -> Result<(Engine, wgpu::Texture, wgpu::TextureView), String> {
    let gpu = GpuContext::headless().map_err(|e| e.to_string())?;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let size = (config.width, config.height);
    let renderer = Renderer::new(&gpu, format, config.profile)?;
    let ui = EguiLayer::new(&gpu, format, None);
    let assets_dir = config.assets_dir.clone().or_else(crate::assets::find_assets_dir);
    let audio = AudioSystem::new(assets_dir.as_ref().map(|a| a.join("audio")).as_deref(), false);
    let engine = Engine {
        gpu,
        renderer,
        audio,
        input: Input::new(),
        timer: FrameTimer::default(),
        frame: FrameData::default(),
        ui,
        window: None,
        size,
        scale_factor: 1.0,
        requests: Vec::new(),
        assets_dir,
        headless: true,
        fullscreen: false,
        vsync: false,
        fow_enabled: false,
        fow_size: (1, 1),
    };
    let target = engine.gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("headless-target"),
        size: wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());
    Ok((engine, target, view))
}

/// Ergebnis von [`run_bench`]: CPU-Zeiten je Frame-Abschnitt und Last der letzten Frames.
#[derive(Clone, Debug)]
pub struct BenchReport {
    pub init_ms: f64,
    pub samples: Vec<FrameSample>,
    /// Mittlere GPU-Zeit je Pass (leer ohne Zeitstempel-Unterstützung).
    pub pass_times: PassTimes,
    pub stats: crate::render::RenderStats,
    pub adapter: String,
    pub software_gpu: bool,
    /// Spitzen-Arbeitsspeicher des Prozesses in MiB (Linux: VmHWM; sonst `None`).
    pub peak_rss_mib: Option<f64>,
}

impl BenchReport {
    /// Mittelwert, p50, p95 und Maximum je Abschnitt als Text (Millisekunden) samt Last der letzten Frames.
    pub fn summary(&self) -> String {
        fn stat(v: &mut [f64]) -> (f64, f64, f64, f64) {
            v.sort_by(|a, b| a.total_cmp(b));
            let q = |p: f64| v.get(((v.len() as f64 - 1.0) * p).round() as usize).copied().unwrap_or(0.0);
            (v.iter().sum::<f64>() / v.len().max(1) as f64, q(0.5), q(0.95), q(1.0))
        }
        let col = |f: fn(&FrameSample) -> f64| -> Vec<f64> { self.samples.iter().map(f).collect() };
        let rows: [(&str, Vec<f64>); 6] = [
            ("update (Sim-Ticks, Kamera, Eingabe)", col(|s| s.update)),
            ("ui (egui-Lauf)", col(|s| s.ui)),
            ("view (Ansicht aufbauen)", col(|s| s.view)),
            ("submit (Renderer-CPU, Befehle)", col(|s| s.submit)),
            ("CPU gesamt", col(|s| s.update + s.ui + s.view + s.submit)),
            ("GPU-Wartezeit (Software-GPU: nicht repräsentativ)", col(|s| s.gpu_wait)),
        ];
        let mut out = format!(
            "Adapter: {}{}\nStart (Init inkl. Atlas/Bake): {:.0} ms\nFrames: {}\nLast: {} Sprites (Boden {}, Schatten {}, Welt {}, Overlay {}), {} Lichter, {} Draw-Calls, intern {}x{} (Pixelskalierung {})\n",
            self.adapter,
            if self.software_gpu { "  [SOFTWARE-GPU: GPU-Zeiten nicht repräsentativ]" } else { "" },
            self.init_ms,
            self.samples.len(),
            self.stats.sprites,
            self.stats.groups[0],
            self.stats.groups[1],
            self.stats.groups[2],
            self.stats.groups[3],
            self.stats.lights,
            self.stats.draw_calls,
            self.stats.internal.0,
            self.stats.internal.1,
            self.stats.pixel_scale
        );
        if !self.pass_times.is_empty() {
            let parts: Vec<String> = self.pass_times.iter().map(|(l, ms)| format!("{l} {ms:.2}")).collect();
            out += &format!("GPU je Pass (ms, Mittel{}): {}\n", if self.software_gpu { ", Software-GPU: nur Verhältnisse" } else { "" }, parts.join(", "));
        }
        if let Some(mb) = self.peak_rss_mib {
            out += &format!("Spitzen-Arbeitsspeicher (Prozess, inkl. Software-Vulkan-Puffer): {mb:.0} MiB\n");
        }
        out += "Abschnitt                                          mittel     p50     p95     max (ms)\n";
        for (name, mut v) in rows {
            let (m, p50, p95, max) = stat(&mut v);
            out += &format!("{name:<48} {m:>8.3} {p50:>7.3} {p95:>7.3} {max:>7.3}\n");
        }
        out
    }
}

/// Führt `warmup + frames` Frames ohne Fenster aus und misst die CPU-Zeiten je Abschnitt sowie die GPU-Wartezeit.
/// Die ersten `warmup` Frames fließen nicht in das Ergebnis ein. Auf Software-Rastern (llvmpipe) ist `gpu_wait` nicht
/// repräsentativ für echte GPUs.
pub fn run_bench<G: Game>(config: AppConfig, mut game: G, warmup: u32, frames: u32, dt: f32) -> Result<BenchReport, String> {
    let (mut engine, _target, view) = headless_engine(&config)?;
    let info = engine.gpu.adapter.get_info();
    let adapter = format!("{} ({:?}, {:?})", info.name, info.backend, info.device_type);
    let software_gpu = info.device_type == wgpu::DeviceType::Cpu || info.name.to_lowercase().contains("llvmpipe");
    let t0 = std::time::Instant::now();
    game.init(&mut engine);
    let init_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let mut samples = Vec::with_capacity(frames as usize);
    let mut pass_sum: PassTimes = Vec::new();
    for i in 0..warmup + frames {
        let dt = engine.timer.tick_fixed(dt);
        let mut sm = FrameSample::default();
        let encoder = run_frame(&mut engine, &mut game, dt, &view, Some(&mut sm));
        let t = std::time::Instant::now();
        engine.gpu.queue.submit(Some(encoder.finish()));
        engine.gpu.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        sm.gpu_wait = t.elapsed().as_secs_f64() * 1000.0;
        if i >= warmup {
            samples.push(sm);
            if let Some(p) = engine.renderer.read_gpu_pass_times(&engine.gpu) {
                for (label, ms) in p {
                    match pass_sum.iter_mut().find(|(l, _)| *l == label) {
                        Some(e) => e.1 += ms,
                        None => pass_sum.push((label, ms)),
                    }
                }
            }
        }
    }
    let pass_times: PassTimes = pass_sum.into_iter().map(|(l, ms)| (l, ms / frames.max(1) as f64)).collect();
    game.on_exit(&mut engine);
    let peak_rss_mib = std::fs::read_to_string("/proc/self/status").ok().and_then(|t| t.lines().find(|l| l.starts_with("VmHWM:")).and_then(|l| l.split_whitespace().nth(1)?.parse::<f64>().ok())).map(|kb| kb / 1024.0);
    Ok(BenchReport { init_ms, samples, pass_times, stats: engine.renderer.stats, adapter, software_gpu, peak_rss_mib })
}

/// Rendert ohne Fenster eine Anzahl Frames und speichert den letzten als PNG.
/// Dient Sichtprüfungen, automatisierten Screenshots und CI.
pub fn run_headless<G: Game>(config: AppConfig, mut game: G, frames: u32, dt: f32, out: &std::path::Path) -> Result<(), String> {
    let (mut engine, target, view) = headless_engine(&config)?;
    let size = engine.size;
    game.init(&mut engine);
    for i in 0..frames {
        let dt = engine.timer.tick_fixed(dt);
        let mut encoder = run_frame(&mut engine, &mut game, dt, &view, None);
        if i + 1 == frames {
            let bytes_per_row = (size.0 * 4).div_ceil(256) * 256;
            let buf = engine.gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("readback"),
                size: (bytes_per_row * size.1) as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: &target, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bytes_per_row), rows_per_image: Some(size.1) } },
                wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
            );
            engine.gpu.queue.submit(Some(encoder.finish()));
            let slice = buf.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            engine.gpu.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
            let data = slice.get_mapped_range().map_err(|e| format!("{e:?}"))?;
            let mut pixels = Vec::with_capacity((size.0 * size.1 * 4) as usize);
            for row in 0..size.1 {
                let start = (row * bytes_per_row) as usize;
                pixels.extend_from_slice(&data[start..start + (size.0 * 4) as usize]);
            }
            drop(data);
            buf.unmap();
            if let Some(dir) = out.parent() {
                std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            }
            image::save_buffer(out, &pixels, size.0, size.1, image::ColorType::Rgba8).map_err(|e| e.to_string())?;
        } else {
            engine.gpu.queue.submit(Some(encoder.finish()));
        }
        let _ = engine.gpu.device.poll(wgpu::PollType::Poll);
    }
    game.on_exit(&mut engine);
    log::info!("Render-Statistik: {:?}", engine.renderer.stats);
    Ok(())
}
