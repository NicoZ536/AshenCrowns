//! Isometrische Projektion (2:1-Diamanten).
//!
//! Weltkoordinaten der Simulation sind Kachel-Einheiten (`x` nach Südosten, `y` nach Südwesten
//! auf dem Bildschirm). Weltpixel sind die Pixel des internen Low-Res-Renderziels, y nach unten.

use glam::Vec2;

/// Breite einer Bodenkachel in Weltpixeln.
pub const TILE_W: f32 = 32.0;
/// Höhe einer Bodenkachel in Weltpixeln.
pub const TILE_H: f32 = 16.0;

/// Kachelkoordinate → Weltpixel (Mittelpunkt der Kachel liegt bei `(x+0.5, y+0.5)`).
#[inline]
pub fn tile_to_world(t: Vec2) -> Vec2 {
    Vec2::new((t.x - t.y) * (TILE_W * 0.5), (t.x + t.y) * (TILE_H * 0.5))
}

/// Weltpixel → Kachelkoordinate (Umkehrung von [`tile_to_world`]).
#[inline]
pub fn world_to_tile(w: Vec2) -> Vec2 {
    let a = w.x / (TILE_W * 0.5);
    let b = w.y / (TILE_H * 0.5);
    Vec2::new((a + b) * 0.5, (b - a) * 0.5)
}

/// Tiefenschlüssel für die Zeichenreihenfolge (größer = weiter vorne).
#[inline]
pub fn depth_of(t: Vec2) -> f32 {
    t.x + t.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for &(x, y) in &[(0.0, 0.0), (3.5, 7.25), (95.0, 1.0), (-2.0, 4.0)] {
            let t = Vec2::new(x, y);
            let back = world_to_tile(tile_to_world(t));
            assert!((back - t).length() < 1e-4, "{t:?} -> {back:?}");
        }
    }

    #[test]
    fn axes() {
        assert_eq!(tile_to_world(Vec2::new(1.0, 0.0)), Vec2::new(16.0, 8.0));
        assert_eq!(tile_to_world(Vec2::new(0.0, 1.0)), Vec2::new(-16.0, 8.0));
    }
}
