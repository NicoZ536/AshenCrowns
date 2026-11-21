//! 2D-Kamera mit pixelgenauer, ganzzahliger Hochskalierung und Subpixel-Scrolling.

use glam::{Mat4, Vec2, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Camera2D {
    /// Mittelpunkt in Weltpixeln.
    pub center: Vec2,
    /// Gewünschte sichtbare Bildzeilen (kleiner = näher). Wird auf ganzzahlige Skalierung gerundet.
    pub target_lines: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Camera2D { center: Vec2::ZERO, target_lines: 340.0 }
    }
}

/// Aus Kamera und Ausgabegröße abgeleitete Ansichtsparameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewParams {
    pub pixel_scale: u32,
    /// Interne Renderauflösung inklusive 2 Pixel Rand.
    pub internal: (u32, u32),
    /// Weltpixel der linken oberen Ecke des internen Ziels (ganzzahlig).
    pub origin: Vec2,
    /// Subpixel-Versatz in internen Pixeln (0..1) + 1 Pixel Rand.
    pub subpixel: Vec2,
    pub output: (u32, u32),
}

impl Camera2D {
    pub fn view(&self, output: (u32, u32), min_lines: f32) -> ViewParams {
        let (w, h) = (output.0.max(1), output.1.max(1));
        let lines = self.target_lines.max(min_lines);
        let scale = ((h as f32 / lines).round() as u32).clamp(1, 16);
        let iw = w.div_ceil(scale) + 2;
        let ih = h.div_ceil(scale) + 2;
        let visible = Vec2::new(w as f32 / scale as f32, h as f32 / scale as f32);
        let tl = self.center - visible * 0.5;
        let origin = tl.floor() - Vec2::ONE;
        let subpixel = tl - tl.floor() + Vec2::ONE;
        ViewParams { pixel_scale: scale, internal: (iw, ih), origin, subpixel, output: (w, h) }
    }
}

impl ViewParams {
    pub fn view_proj(&self) -> Mat4 {
        let (w, h) = (self.internal.0 as f32, self.internal.1 as f32);
        // Weltpixel → Clip-Raum (y nach unten, z unverändert 0).
        let (l, t) = (self.origin.x, self.origin.y);
        Mat4::from_cols(
            Vec4::new(2.0 / w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -2.0 / h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(-1.0 - 2.0 * l / w, 1.0 + 2.0 * t / h, 0.0, 1.0),
        )
    }

    /// Bildschirmpixel (Ausgabe) → Weltpixel.
    pub fn screen_to_world(&self, p: Vec2) -> Vec2 {
        self.origin + p / self.pixel_scale as f32 + self.subpixel
    }

    /// Weltpixel → Bildschirmpixel (Ausgabe).
    pub fn world_to_screen(&self, w: Vec2) -> Vec2 {
        (w - self.origin - self.subpixel) * self.pixel_scale as f32
    }

    /// Sichtbarer Weltbereich (min, max) mit Rand.
    pub fn visible_world(&self, margin: f32) -> (Vec2, Vec2) {
        let size = Vec2::new(self.internal.0 as f32, self.internal.1 as f32);
        (self.origin - Vec2::splat(margin), self.origin + size + Vec2::splat(margin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_world_roundtrip() {
        let cam = Camera2D { center: Vec2::new(100.3, -50.7), target_lines: 300.0 };
        let v = cam.view((1440, 900), 200.0);
        assert_eq!(v.pixel_scale, 3);
        let p = Vec2::new(321.0, 123.0);
        let w = v.screen_to_world(p);
        assert!((v.world_to_screen(w) - p).length() < 1e-3);
        // Bildmitte entspricht Kameramittelpunkt.
        let c = v.screen_to_world(Vec2::new(720.0, 450.0));
        assert!((c - cam.center).length() < 0.01);
    }
}
