//! Asset-Verwaltung: Pfadauflösung, prozedurale Pixel-Art, Atlas, PNG-Overrides.

pub mod atlas;
pub mod pixel;

pub use atlas::{Atlas, AtlasBuilder};
pub use pixel::Canvas;

use std::path::{Path, PathBuf};

/// Findet das `assets`-Verzeichnis: Umgebungsvariable `ASHEN_ASSETS`, neben der ausführbaren Datei,
/// im macOS-App-Bundle (`Contents/Resources/assets`) oder aufwärts vom Arbeitsverzeichnis.
pub fn find_assets_dir() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("ASHEN_ASSETS").map(PathBuf::from)
        && p.join("data").is_dir()
    {
        return Some(p);
    }
    let mut candidates = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        candidates.push(dir.join("assets"));
        candidates.push(dir.join("../Resources/assets"));
        let mut d = dir.to_path_buf();
        for _ in 0..4 {
            if !d.pop() {
                break;
            }
            candidates.push(d.join("assets"));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let mut d = cwd.clone();
        candidates.push(d.join("assets"));
        for _ in 0..4 {
            if !d.pop() {
                break;
            }
            candidates.push(d.join("assets"));
        }
    }
    candidates.into_iter().find(|p| p.join("data").is_dir())
}

/// Optionale handgezeichnete Overrides: `assets/sprites/<name>.png` (+ `<name>_n.png` Normalen)
/// ersetzen gleichnamige prozedurale Sprites. So können Artists Sprites schrittweise austauschen.
pub fn load_override(assets: &Path, name: &str) -> Option<Canvas> {
    let file = assets.join("sprites").join(format!("{name}.png"));
    if !file.exists() {
        return None;
    }
    let normal = assets.join("sprites").join(format!("{name}_n.png"));
    match Canvas::from_png(&file, Some(&normal)) {
        Ok(c) => Some(c),
        Err(e) => {
            log::warn!("Sprite-Override {name} nicht ladbar: {e}");
            None
        }
    }
}
