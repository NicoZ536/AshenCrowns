//! Shader-Verwaltung: eingebettete WGSL-Quellen mit optionalem Hot-Reload aus dem Dateisystem.
//!
//! Ist die Umgebungsvariable `ASHEN_SHADER_DIR` gesetzt, werden Shader von dort geladen und bei
//! Dateiänderung automatisch neu übersetzt (Entwicklung). Fehlerhafte Shader werden verworfen,
//! die zuletzt funktionierende Version bleibt aktiv.

use std::path::PathBuf;
use std::time::SystemTime;

pub const COMMON: &str = include_str!("shaders/common.wgsl");

pub const SOURCES: [(&str, &str); 5] = [
    ("sprite", include_str!("shaders/sprite.wgsl")),
    ("light", include_str!("shaders/light.wgsl")),
    ("composite", include_str!("shaders/composite.wgsl")),
    ("bloom", include_str!("shaders/bloom.wgsl")),
    ("final", include_str!("shaders/final.wgsl")),
];

pub struct ShaderManager {
    dir: Option<PathBuf>,
    stamps: Vec<Option<SystemTime>>,
    check_timer: f32,
}

impl ShaderManager {
    pub fn new() -> ShaderManager {
        let dir = std::env::var_os("ASHEN_SHADER_DIR").map(PathBuf::from).filter(|p| p.is_dir());
        if let Some(d) = &dir {
            log::info!("Shader-Hot-Reload aktiv: {}", d.display());
        }
        let mut s = ShaderManager { dir, stamps: vec![None; SOURCES.len() + 1], check_timer: 0.0 };
        s.stamps = s.current_stamps();
        s
    }

    fn current_stamps(&self) -> Vec<Option<SystemTime>> {
        let Some(dir) = &self.dir else { return vec![None; SOURCES.len() + 1] };
        std::iter::once("common")
            .chain(SOURCES.iter().map(|(n, _)| *n))
            .map(|n| std::fs::metadata(dir.join(format!("{n}.wgsl"))).and_then(|m| m.modified()).ok())
            .collect()
    }

    /// Vollständiger Quelltext (gemeinsamer Teil + Shader).
    pub fn source(&self, name: &str) -> String {
        let embedded = SOURCES.iter().find(|(n, _)| *n == name).map(|(_, s)| *s).unwrap_or("");
        if let Some(dir) = &self.dir {
            let common = std::fs::read_to_string(dir.join("common.wgsl")).unwrap_or_else(|_| COMMON.to_string());
            let body = std::fs::read_to_string(dir.join(format!("{name}.wgsl"))).unwrap_or_else(|_| embedded.to_string());
            return format!("{common}\n{body}");
        }
        format!("{COMMON}\n{embedded}")
    }

    /// `true`, wenn sich Shader-Dateien geändert haben (höchstens zweimal pro Sekunde geprüft).
    pub fn poll_changed(&mut self, dt: f32) -> bool {
        if self.dir.is_none() {
            return false;
        }
        self.check_timer -= dt;
        if self.check_timer > 0.0 {
            return false;
        }
        self.check_timer = 0.5;
        let now = self.current_stamps();
        if now != self.stamps {
            self.stamps = now;
            return true;
        }
        false
    }
}

impl Default for ShaderManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Übersetzt einen Shader und meldet Validierungsfehler, statt abzustürzen.
pub fn compile(device: &wgpu::Device, name: &str, source: &str) -> Result<wgpu::ShaderModule, String> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some(name), source: wgpu::ShaderSource::Wgsl(source.into()) });
    match pollster::block_on(scope.pop()) {
        Some(e) => Err(format!("Shader {name}: {e}")),
        None => Ok(module),
    }
}
