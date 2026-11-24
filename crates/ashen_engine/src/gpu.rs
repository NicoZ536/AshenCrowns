//! GPU-Kontext: Instanz, Adapter, Gerät und Warteschlange.

use std::sync::Arc;

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

#[derive(Debug)]
pub struct GpuError(pub String);

impl std::fmt::Display for GpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GPU-Initialisierung fehlgeschlagen: {}", self.0)
    }
}

impl std::error::Error for GpuError {}

impl GpuContext {
    /// Erstellt den Kontext. `surface` sorgt dafür, dass ein zur Fensteroberfläche
    /// kompatibler Adapter gewählt wird. Auf Apple Silicon wird Metal verwendet.
    pub fn new(instance: wgpu::Instance, surface: Option<&wgpu::Surface<'_>>) -> Result<GpuContext, GpuError> {
        Self::with_timing(instance, surface, false)
    }

    /// Wie [`GpuContext::new`]; `want_timing` fordert GPU-Zeitstempelabfragen an (falls unterstützt).
    pub fn with_timing(instance: wgpu::Instance, surface: Option<&wgpu::Surface<'_>>, want_timing: bool) -> Result<GpuContext, GpuError> {
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            // Energieeffizient: integrierte GPU reicht für den Pixel-Art-Renderer und schont den Akku.
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: surface,
            apply_limit_buckets: false,
        }))
        .map_err(|e| GpuError(format!("kein passender Grafikadapter: {e}")))?;
        let info = adapter.get_info();
        log::info!("Grafikadapter: {} ({:?}, {:?})", info.name, info.backend, info.device_type);
        // GPU-Zeitstempel nur auf Wunsch (Benchmark, `ASHEN_GPU_TIMING=1`) und wenn der Adapter sie kennt.
        let timing = (want_timing || std::env::var_os("ASHEN_GPU_TIMING").is_some()) && adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("ashen-device"),
            required_features: if timing { wgpu::Features::TIMESTAMP_QUERY } else { wgpu::Features::empty() },
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            ..Default::default()
        }))
        .map_err(|e| GpuError(format!("Gerät konnte nicht erstellt werden: {e}")))?;
        device.on_uncaptured_error(Arc::new(|e| log::error!("wgpu: {e}")));
        Ok(GpuContext { instance, adapter, device, queue })
    }

    /// Kontext ohne Fenster (Screenshots, Tests).
    pub fn headless() -> Result<GpuContext, GpuError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        Self::with_timing(instance, None, true)
    }
}
