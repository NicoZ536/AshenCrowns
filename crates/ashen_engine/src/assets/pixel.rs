//! Pixel-Leinwand für prozedurale Pixel-Art mit Albedo-, Normalen- und Emissionskanal.
//!
//! Primitive schreiben neben der Farbe auch eine Oberflächennormale, damit der Renderer
//! Sprites dynamisch beleuchten kann. Organische Formen erhalten nachträglich „Kissen-Normalen“
//! aus einer Distanztransformation ([`Canvas::pillow_normals`]).

use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise;
use glam::{Vec2, Vec3};

/// Kodierte Normale (x, y in 0..255, 128 = flach) plus Emission.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NormalPx {
    pub nx: u8,
    pub ny: u8,
    pub emissive: u8,
    /// 1, wenn die Normale explizit gesetzt wurde (wird von `pillow_normals` respektiert).
    pub fixed: u8,
}

impl NormalPx {
    pub const FLAT: NormalPx = NormalPx { nx: 128, ny: 128, emissive: 0, fixed: 0 };

    pub fn from_vec(n: Vec3) -> NormalPx {
        let n = n.normalize_or(Vec3::Z);
        NormalPx { nx: ((n.x * 0.5 + 0.5) * 255.0) as u8, ny: ((n.y * 0.5 + 0.5) * 255.0) as u8, emissive: 0, fixed: 1 }
    }
}

/// Häufig verwendete Flächennormalen im Bildschirmraum (y nach unten).
pub mod normals {
    use glam::Vec3;
    pub const UP: Vec3 = Vec3::new(0.0, -0.75, 0.66);
    pub const LEFT: Vec3 = Vec3::new(-0.8, 0.15, 0.58);
    pub const RIGHT: Vec3 = Vec3::new(0.8, 0.15, 0.58);
    pub const FRONT: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const ROOF_L: Vec3 = Vec3::new(-0.55, -0.55, 0.63);
    pub const ROOF_R: Vec3 = Vec3::new(0.55, -0.55, 0.63);
}

#[derive(Clone)]
pub struct Canvas {
    pub w: i32,
    pub h: i32,
    pub color: Vec<Rgba>,
    pub normal: Vec<NormalPx>,
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Canvas {
        let n = (w * h).max(0) as usize;
        Canvas { w, h, color: vec![Rgba::TRANSPARENT; n], normal: vec![NormalPx::FLAT; n] }
    }

    #[inline]
    pub fn inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h
    }

    #[inline]
    fn i(&self, x: i32, y: i32) -> usize {
        (y * self.w + x) as usize
    }

    pub fn get(&self, x: i32, y: i32) -> Rgba {
        if self.inside(x, y) { self.color[self.i(x, y)] } else { Rgba::TRANSPARENT }
    }

    pub fn opaque(&self, x: i32, y: i32) -> bool {
        self.get(x, y).3 > 0
    }

    /// Setzt einen Pixel (Alpha 0 wird ignoriert, Alpha < 255 wird geblendet).
    #[inline]
    pub fn px(&mut self, x: i32, y: i32, c: Rgba) {
        if !self.inside(x, y) || c.3 == 0 {
            return;
        }
        let i = self.i(x, y);
        if c.3 == 255 || self.color[i].3 == 0 {
            self.color[i] = c;
        } else {
            let base = self.color[i];
            self.color[i] = base.lerp(c.with_alpha(255), c.3 as f32 / 255.0).with_alpha(base.3.max(c.3));
        }
    }

    /// Pixel mit Normale.
    #[inline]
    pub fn pxn(&mut self, x: i32, y: i32, c: Rgba, n: Vec3) {
        if !self.inside(x, y) || c.3 == 0 {
            return;
        }
        self.px(x, y, c);
        let i = self.i(x, y);
        let e = self.normal[i].emissive;
        self.normal[i] = NormalPx::from_vec(n);
        self.normal[i].emissive = e;
    }

    /// Emission eines Pixels setzen (0..1).
    pub fn glow(&mut self, x: i32, y: i32, e: f32) {
        if self.inside(x, y) {
            let i = self.i(x, y);
            self.normal[i].emissive = (e.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }

    pub fn erase(&mut self, x: i32, y: i32) {
        if self.inside(x, y) {
            let i = self.i(x, y);
            self.color[i] = Rgba::TRANSPARENT;
            self.normal[i] = NormalPx::FLAT;
        }
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Rgba) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.px(xx, yy, c);
            }
        }
    }

    pub fn rect_n(&mut self, x: i32, y: i32, w: i32, h: i32, c: Rgba, n: Vec3) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.pxn(xx, yy, c, n);
            }
        }
    }

    /// Linie (Bresenham).
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: Rgba) {
        let (mut x, mut y) = (x0, y0);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.px(x, y, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Gefüllte Ellipse mit Kugelschattierung aus einer Farbrampe (Licht von links oben).
    pub fn ellipse_shaded(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, ramp: &Ramp, light_bias: f32) {
        let x0 = (cx - rx).floor() as i32;
        let x1 = (cx + rx).ceil() as i32;
        let y0 = (cy - ry).floor() as i32;
        let y1 = (cy + ry).ceil() as i32;
        let l = Vec3::new(-0.55, -0.65, 0.52).normalize();
        for y in y0..=y1 {
            for x in x0..=x1 {
                let dx = (x as f32 + 0.5 - cx) / rx;
                let dy = (y as f32 + 0.5 - cy) / ry;
                let d = dx * dx + dy * dy;
                if d > 1.0 {
                    continue;
                }
                let n = Vec3::new(dx, dy, (1.0 - d).max(0.0).sqrt());
                let t = (n.dot(l) * 0.5 + 0.5 + light_bias).clamp(0.0, 1.0);
                self.pxn(x, y, ramp.dither(t, x, y), n);
            }
        }
    }

    /// Gefülltes Polygon (Scanline, gerade/ungerade-Regel).
    pub fn polygon(&mut self, pts: &[Vec2], mut shade: impl FnMut(i32, i32) -> Option<(Rgba, Vec3)>) {
        if pts.len() < 3 {
            return;
        }
        let ymin = pts.iter().map(|p| p.y).fold(f32::MAX, f32::min).floor() as i32;
        let ymax = pts.iter().map(|p| p.y).fold(f32::MIN, f32::max).ceil() as i32;
        let mut xs = Vec::new();
        for y in ymin..=ymax {
            let sy = y as f32 + 0.5;
            xs.clear();
            for i in 0..pts.len() {
                let a = pts[i];
                let b = pts[(i + 1) % pts.len()];
                if (a.y <= sy && b.y > sy) || (b.y <= sy && a.y > sy) {
                    xs.push(a.x + (sy - a.y) / (b.y - a.y) * (b.x - a.x));
                }
            }
            xs.sort_by(f32::total_cmp);
            for pair in xs.chunks(2) {
                if pair.len() < 2 {
                    continue;
                }
                let start = (pair[0] - 0.5).ceil() as i32;
                let end = (pair[1] - 0.5).floor() as i32;
                for x in start..=end {
                    if let Some((c, n)) = shade(x, y) {
                        self.pxn(x, y, c, n);
                    }
                }
            }
        }
    }

    /// Selektive Kontur: transparente Nachbarn opaker Pixel werden mit einer dunklen Variante
    /// der Nachbarfarbe gefüllt (wirkt weicher als eine schwarze Linie).
    pub fn outline(&mut self, darkness: f32) {
        let src = self.color.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                if src[self.i(x, y)].3 != 0 {
                    continue;
                }
                let mut acc: Option<Rgba> = None;
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if self.inside(nx, ny) {
                        let c = src[self.i(nx, ny)];
                        if c.3 != 0 && acc.is_none_or(|a| c.luma() < a.luma()) {
                            acc = Some(c);
                        }
                    }
                }
                if let Some(c) = acc {
                    let o = Rgba(((c.0 as f32 * darkness) as u8).max(6), ((c.1 as f32 * darkness) as u8).max(5), ((c.2 as f32 * darkness) as u8).max(8), 255);
                    let i = self.i(x, y);
                    self.color[i] = o;
                }
            }
        }
    }

    /// Erzeugt gerundete Normalen aus einer Distanztransformation der Silhouette
    /// (für Figuren, Bäume, Felsen). Explizit gesetzte Normalen bleiben erhalten.
    pub fn pillow_normals(&mut self, strength: f32) {
        let (w, h) = (self.w, self.h);
        let mut dist = vec![0.0f32; (w * h) as usize];
        // Zwei-Pass-Chamfer-Distanz.
        let big = 1e4;
        for y in 0..h {
            for x in 0..w {
                let i = self.i(x, y);
                dist[i] = if self.color[i].3 == 0 { 0.0 } else { big };
            }
        }
        let at = |d: &Vec<f32>, x: i32, y: i32| if x < 0 || y < 0 || x >= w || y >= h { 0.0 } else { d[(y * w + x) as usize] };
        for y in 0..h {
            for x in 0..w {
                let i = (y * w + x) as usize;
                if dist[i] == 0.0 {
                    continue;
                }
                let m = (at(&dist, x - 1, y) + 1.0).min(at(&dist, x, y - 1) + 1.0).min(at(&dist, x - 1, y - 1) + 1.414).min(at(&dist, x + 1, y - 1) + 1.414);
                dist[i] = dist[i].min(m);
            }
        }
        for y in (0..h).rev() {
            for x in (0..w).rev() {
                let i = (y * w + x) as usize;
                if dist[i] == 0.0 {
                    continue;
                }
                let m = (at(&dist, x + 1, y) + 1.0).min(at(&dist, x, y + 1) + 1.0).min(at(&dist, x + 1, y + 1) + 1.414).min(at(&dist, x - 1, y + 1) + 1.414);
                dist[i] = dist[i].min(m);
            }
        }
        let height = |x: i32, y: i32| at(&dist, x, y).min(4.0).sqrt();
        for y in 0..h {
            for x in 0..w {
                let i = (y * w + x) as usize;
                if self.color[i].3 == 0 || self.normal[i].fixed == 1 {
                    continue;
                }
                let gx = height(x + 1, y) - height(x - 1, y);
                let gy = height(x, y + 1) - height(x, y - 1);
                // Feinstruktur aus der Helligkeit (dunkle Pixel = Vertiefung).
                let l = self.color[i].luma();
                let lx = self.get(x + 1, y).luma() - self.get(x - 1, y).luma();
                let ly = self.get(x, y + 1).luma() - self.get(x, y - 1).luma();
                let n = Vec3::new(-(gx * strength + lx * 0.6), -(gy * strength + ly * 0.6), 1.0);
                let e = self.normal[i].emissive;
                self.normal[i] = NormalPx::from_vec(n);
                self.normal[i].fixed = 0;
                self.normal[i].emissive = e;
                let _ = l;
            }
        }
    }

    /// Fügt Rauschen in die Helligkeit opaker Pixel ein (Materialstruktur).
    pub fn grain(&mut self, amount: f32, seed: u32) {
        for y in 0..self.h {
            for x in 0..self.w {
                let i = self.i(x, y);
                let c = self.color[i];
                if c.3 == 0 {
                    continue;
                }
                let r = noise::rand2(x, y, seed) - 0.5;
                self.color[i] = c.scale(1.0 + r * amount);
            }
        }
    }

    /// Kopiert eine andere Leinwand an Position (x, y) (Alpha-Test).
    pub fn blit(&mut self, src: &Canvas, x: i32, y: i32, flip: bool) {
        for sy in 0..src.h {
            for sx in 0..src.w {
                let rx = if flip { src.w - 1 - sx } else { sx };
                let c = src.color[(sy * src.w + rx) as usize];
                if c.3 == 0 {
                    continue;
                }
                let (dx, dy) = (x + sx, y + sy);
                if self.inside(dx, dy) {
                    let i = self.i(dx, dy);
                    self.color[i] = c;
                    let mut n = src.normal[(sy * src.w + rx) as usize];
                    if flip {
                        n.nx = 255 - n.nx;
                    }
                    self.normal[i] = n;
                }
            }
        }
    }

    /// Dreht die Leinwand um `angle` (Bogenmaß) um den Punkt `pivot` (nächster Nachbar).
    pub fn rotated(&self, angle: f32, pivot: Vec2) -> Canvas {
        let mut out = Canvas::new(self.w, self.h);
        let (s, c) = angle.sin_cos();
        for y in 0..self.h {
            for x in 0..self.w {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - pivot;
                let src = Vec2::new(p.x * c + p.y * s, -p.x * s + p.y * c) + pivot;
                let (sx, sy) = (src.x.floor() as i32, src.y.floor() as i32);
                if self.inside(sx, sy) {
                    let i = (sy * self.w + sx) as usize;
                    if self.color[i].3 != 0 {
                        let o = (y * self.w + x) as usize;
                        out.color[o] = self.color[i];
                        out.normal[o] = self.normal[i];
                    }
                }
            }
        }
        out
    }

    /// Leinwand als RGBA8-Bytes (Albedo).
    pub fn albedo_bytes(&self) -> Vec<u8> {
        self.color.iter().flat_map(|c| c.to_array()).collect()
    }

    /// Normalen/Emission als RGBA8-Bytes (r = nx, g = ny, b = Emission, a = Abdeckung).
    pub fn normal_bytes(&self) -> Vec<u8> {
        self.normal.iter().zip(&self.color).flat_map(|(n, c)| [n.nx, n.ny, n.emissive, c.3]).collect()
    }

    /// Lädt eine Leinwand aus einem PNG (Albedo), optional mit Normalen-PNG.
    pub fn from_png(path: &std::path::Path, normal_path: Option<&std::path::Path>) -> Result<Canvas, String> {
        let img = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?.to_rgba8();
        let (w, h) = (img.width() as i32, img.height() as i32);
        let mut c = Canvas::new(w, h);
        for (i, p) in img.pixels().enumerate() {
            c.color[i] = Rgba(p[0], p[1], p[2], p[3]);
        }
        if let Some(np) = normal_path.filter(|p| p.exists()) {
            let n = image::open(np).map_err(|e| format!("{}: {e}", np.display()))?.to_rgba8();
            if n.width() as i32 == w && n.height() as i32 == h {
                for (i, p) in n.pixels().enumerate() {
                    c.normal[i] = NormalPx { nx: p[0], ny: p[1], emissive: p[2], fixed: 1 };
                }
            }
        } else {
            c.pillow_normals(0.8);
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_and_outline() {
        let mut c = Canvas::new(16, 16);
        let ramp = Ramp::from_hex(&[0x202020, 0x606060, 0xa0a0a0]);
        c.ellipse_shaded(8.0, 8.0, 4.0, 4.0, &ramp, 0.0);
        assert!(c.opaque(8, 8));
        assert!(!c.opaque(1, 1));
        c.outline(0.5);
        assert!(c.opaque(3, 8) || c.opaque(4, 8));
        c.pillow_normals(1.0);
        let n = c.normal[(8 * 16 + 5) as usize];
        assert!(n.nx < 128, "left side faces left: {}", n.nx);
        let r = c.rotated(std::f32::consts::FRAC_PI_2, Vec2::new(8.0, 8.0));
        assert!(r.opaque(8, 8));
    }
}
