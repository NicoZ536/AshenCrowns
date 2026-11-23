//! 2D-Ergänzungen zu den 3D-Gebäuden: Baugerüste sowie der Metadatentyp für Effektpunkte.

use super::palette as pal;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::{Vec2, Vec3};

/// Effektpunkte relativ zum Anker (obere Ecke der Grundfläche) in Weltpixeln.
#[derive(Clone, Debug, Default)]
pub struct BuildingMeta {
    pub lights: Vec<(Vec2, Vec3, f32)>,
    pub smoke: Vec<Vec2>,
    pub fires: Vec<Vec2>,
    pub glow_green: Vec<Vec2>,
    pub height: f32,
}

/// Minimale Isometrie-Hilfe für 2D-Gerüste.
struct Iso {
    w: f32,
    h: f32,
    oy: f32,
}

impl Iso {
    fn canvas(&self) -> Canvas {
        Canvas::new(((self.w + self.h) * 16.0) as i32, ((self.w + self.h) * 8.0 + self.oy) as i32 + 1)
    }

    fn anchor(&self) -> Vec2 {
        Vec2::new(self.h * 16.0, self.oy)
    }

    fn p(&self, tx: f32, ty: f32, z: f32) -> Vec2 {
        Vec2::new(16.0 * (tx - ty) + 16.0 * self.h, 8.0 * (tx + ty) + self.oy - z)
    }
}

fn finish(mut c: Canvas) -> Canvas {
    c.outline(pal::OUTLINE);
    c
}

fn scaffold(w: f32, h: f32) -> (Canvas, Vec2) {
    let iso = Iso { w, h, oy: 36.0 };
    let mut c = iso.canvas();
    let wood = pal::wood();
    let corners = [(0.15, 0.15), (w - 0.15, 0.15), (0.15, h - 0.15), (w - 0.15, h - 0.15)];
    for &(tx, ty) in &corners {
        let p = iso.p(tx, ty, 0.0);
        for dy in 0..32 {
            c.px(p.x as i32, p.y as i32 - dy, wood.at(0.5));
        }
    }
    for z in [10.0, 22.0] {
        for (a, b) in [((0.15, h - 0.15), (w - 0.15, h - 0.15)), ((w - 0.15, h - 0.15), (w - 0.15, 0.15))] {
            let pa = iso.p(a.0, a.1, z);
            let pb = iso.p(b.0, b.1, z);
            c.line(pa.x as i32, pa.y as i32, pb.x as i32, pb.y as i32, wood.at(0.65));
            c.line(pa.x as i32, pa.y as i32 + 1, pb.x as i32, pb.y as i32 + 1, wood.at(0.35));
        }
    }
    // Diagonalstrebe.
    let a = iso.p(0.15, h - 0.15, 0.0);
    let b = iso.p(w - 0.15, h - 0.15, 22.0);
    c.line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, wood.at(0.4));
    (finish(c), iso.anchor())
}

pub fn build(atlas: &mut AtlasBuilder) {
    for (w, h) in [(1.0, 1.0), (2.0, 2.0), (3.0, 2.0), (3.0, 3.0)] {
        let (c, a) = scaffold(w, h);
        atlas.add(format!("scaffold_{}x{}", w as i32, h as i32), c, a);
    }
}
