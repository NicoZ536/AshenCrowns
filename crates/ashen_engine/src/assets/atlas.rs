//! Textur-Atlas mit Regal-Packer (Shelf Packing). Alle Sprites landen in einem Atlas,
//! damit der Renderer die gesamte Welt mit wenigen Draw Calls zeichnen kann.

use super::pixel::Canvas;
use crate::render::SpriteRect;
use glam::Vec2;
use std::collections::HashMap;

pub struct AtlasBuilder {
    entries: Vec<(String, Canvas, Vec2)>,
}

/// Fertig gepackter Atlas.
pub struct Atlas {
    pub size: (u32, u32),
    pub albedo: Vec<u8>,
    pub normal: Vec<u8>,
    rects: HashMap<String, SpriteRect>,
    missing: SpriteRect,
}

impl Default for AtlasBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AtlasBuilder {
    pub fn new() -> AtlasBuilder {
        AtlasBuilder { entries: Vec::new() }
    }

    /// Fügt ein Sprite hinzu. `anchor` in Pixeln relativ zur linken oberen Ecke.
    pub fn add(&mut self, name: impl Into<String>, canvas: Canvas, anchor: Vec2) {
        let name = name.into();
        if let Some(e) = self.entries.iter_mut().find(|e| e.0 == name) {
            e.1 = canvas;
            e.2 = anchor;
        } else {
            self.entries.push((name, canvas, anchor));
        }
    }

    /// Ersetzt ein vorhandenes Sprite (z. B. handgezeichneter Override). Bei gleicher Größe bleibt
    /// der Anker des Originals erhalten, sonst wird `fallback_anchor` verwendet.
    pub fn replace(&mut self, name: &str, canvas: Canvas, fallback_anchor: Vec2) {
        let anchor = match self.entries.iter().find(|e| e.0 == name) {
            Some(e) if e.1.w == canvas.w && e.1.h == canvas.h => e.2,
            _ => fallback_anchor,
        };
        self.add(name, canvas, anchor);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.iter().any(|e| e.0 == name)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Packt alle Sprites (1 Pixel Abstand gegen Ausbluten). Die Atlasbreite wächst bei Bedarf.
    pub fn build(mut self) -> Atlas {
        // Fallback-Sprite (magenta Schachbrett) für fehlende Namen.
        let mut miss = Canvas::new(8, 8);
        for y in 0..8 {
            for x in 0..8 {
                let c = if (x / 2 + y / 2) % 2 == 0 { ashen_core::color::Rgba::hex(0xff00ff) } else { ashen_core::color::Rgba::hex(0x200020) };
                miss.px(x, y, c);
            }
        }
        self.entries.push(("__missing".into(), miss, Vec2::new(4.0, 8.0)));
        let mut order: Vec<usize> = (0..self.entries.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse((self.entries[i].1.h, self.entries[i].1.w)));
        let area: i64 = self.entries.iter().map(|e| ((e.1.w + 2) * (e.1.h + 2)) as i64).sum();
        let mut width = 512;
        while (width as i64 * width as i64) < area * 2 && width < 8192 {
            width *= 2;
        }
        let max_w = self.entries.iter().map(|e| e.1.w + 2).max().unwrap_or(1);
        width = width.max((max_w as u32).next_power_of_two() as i32);
        let (mut x, mut y, mut row_h) = (1, 1, 0);
        let mut places = vec![(0, 0); self.entries.len()];
        for &i in &order {
            let c = &self.entries[i].1;
            if x + c.w + 1 > width {
                x = 1;
                y += row_h + 1;
                row_h = 0;
            }
            places[i] = (x, y);
            x += c.w + 1;
            row_h = row_h.max(c.h);
        }
        let height = ((y + row_h + 1) as u32).next_power_of_two();
        let (w, h) = (width as u32, height);
        let mut albedo = vec![0u8; (w * h * 4) as usize];
        let mut normal = vec![0u8; (w * h * 4) as usize];
        let mut rects = HashMap::new();
        for (i, (name, c, anchor)) in self.entries.iter().enumerate() {
            let (px, py) = places[i];
            let ab = c.albedo_bytes();
            let nb = c.normal_bytes();
            for row in 0..c.h {
                let src = (row * c.w * 4) as usize..((row + 1) * c.w * 4) as usize;
                let dst_start = (((py + row) as u32 * w + px as u32) * 4) as usize;
                albedo[dst_start..dst_start + (c.w * 4) as usize].copy_from_slice(&ab[src.clone()]);
                normal[dst_start..dst_start + (c.w * 4) as usize].copy_from_slice(&nb[src]);
            }
            let uv = [px as f32 / w as f32, py as f32 / h as f32, (px + c.w) as f32 / w as f32, (py + c.h) as f32 / h as f32];
            rects.insert(name.clone(), SpriteRect { uv, size: Vec2::new(c.w as f32, c.h as f32), anchor: *anchor });
        }
        let missing = rects["__missing"];
        Atlas { size: (w, h), albedo, normal, rects, missing }
    }
}

impl Atlas {
    /// Sprite nach Namen; fehlende Namen liefern ein auffälliges Ersatz-Sprite (und eine Logmeldung).
    pub fn get(&self, name: &str) -> SpriteRect {
        match self.rects.get(name) {
            Some(r) => *r,
            None => {
                log::warn!("Sprite '{name}' fehlt im Atlas");
                self.missing
            }
        }
    }

    pub fn try_get(&self, name: &str) -> Option<SpriteRect> {
        self.rects.get(name).copied()
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.rects.keys()
    }

    /// Speichert den Atlas als PNG (Debug/Artists).
    pub fn save_png(&self, path: &std::path::Path) -> Result<(), String> {
        image::save_buffer(path, &self.albedo, self.size.0, self.size.1, image::ColorType::Rgba8).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_without_overlap() {
        let mut b = AtlasBuilder::new();
        for i in 0..50 {
            let mut c = Canvas::new(10 + i % 7, 12 + i % 5);
            c.px(0, 0, ashen_core::color::Rgba::hex(0xffffff));
            b.add(format!("s{i}"), c, Vec2::ZERO);
        }
        let a = b.build();
        let rects: Vec<SpriteRect> = (0..50).map(|i| a.get(&format!("s{i}"))).collect();
        for (i, r) in rects.iter().enumerate() {
            for s in rects.iter().skip(i + 1) {
                let overlap = r.uv[0] < s.uv[2] && s.uv[0] < r.uv[2] && r.uv[1] < s.uv[3] && s.uv[1] < r.uv[3];
                assert!(!overlap);
            }
        }
        assert!(a.try_get("nope").is_none());
    }
}
