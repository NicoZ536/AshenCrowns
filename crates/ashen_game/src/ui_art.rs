//! Prozedurale UI-Grafik: geschmiedetes Eisen, Bronze, Leder, Pergament, Goldfiligran und
//! Edelsteine als Pixel-Art-Texturen für egui – plus die darauf aufbauenden Widgets
//! (9-Slice-Rahmen, Schaltflächen, Befehlsfelder, Balken, Tooltips, Zierschrift mit Glut).
//!
//! Alle Texturen werden beim Start mit der [`Canvas`]-API gemalt (≈ 20 ms) und als
//! egui-Texturen registriert. Pixel-Art wird mit [`PX`] Punkten pro Pixel gezeichnet.

use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise;
use ashen_engine::assets::Canvas;
use ashen_engine::ui::theme;
use egui::emath::GuiRounding;
use egui::{Align2, Color32, ColorImage, FontId, Mesh, Painter, Pos2, Rect, Response, Sense, Shape, TextureHandle, TextureId, TextureOptions, Ui, Vec2};
use std::f32::consts::{PI, TAU};
use std::sync::Arc;

/// Punkte pro Texturpixel für Rahmen und Schaltflächen.
pub const PX: f32 = 2.0;

// ---------------------------------------------------------------------------------------------
// Materialien
// ---------------------------------------------------------------------------------------------

fn iron() -> Ramp {
    Ramp::from_hex(&[0x09090c, 0x121317, 0x1b1d22, 0x25282e, 0x31353c, 0x40454d, 0x555b64, 0x737a84, 0x9aa1aa])
}
fn bronze() -> Ramp {
    Ramp::from_hex(&[0x150d07, 0x26180c, 0x3b2611, 0x543617, 0x6f481e, 0x8c5d27, 0xa97533, 0xc79246, 0xe2b466])
}
fn gold() -> Ramp {
    Ramp::from_hex(&[0x3a2508, 0x5f3f10, 0x8a6018, 0xb68424, 0xd9a83a, 0xf0cb62, 0xfde79c, 0xfff7da])
}
fn stone() -> Ramp {
    Ramp::from_hex(&[0x0c0c0f, 0x131317, 0x1a1a1f, 0x222228, 0x2b2b32, 0x35353d, 0x41414a, 0x51515b, 0x66666f])
}
fn leather() -> Ramp {
    Ramp::from_hex(&[0x0e0907, 0x150e0a, 0x1c130e, 0x231812, 0x2b1e16, 0x35251a, 0x402d20])
}
fn parchment() -> Ramp {
    Ramp::from_hex(&[0x5a4024, 0x775733, 0x927046, 0xab8a5a, 0xc1a26f, 0xd3b884, 0xe1cb99, 0xecdbb0])
}
fn ruby() -> Ramp {
    Ramp::from_hex(&[0x1a0205, 0x3d060b, 0x690c13, 0x9b1519, 0xcc2a24, 0xf2563a, 0xff9a72])
}
fn emerald() -> Ramp {
    Ramp::from_hex(&[0x03140c, 0x062b19, 0x0b4a2a, 0x157040, 0x24995a, 0x4fc983, 0x9cf0bd])
}
fn wax() -> Ramp {
    Ramp::from_hex(&[0x2a0507, 0x4a0a0d, 0x6e1214, 0x8f1c1a, 0xb02c24, 0xd04a36])
}
fn wood() -> Ramp {
    Ramp::from_hex(&[0x120b07, 0x1d130c, 0x2a1b11, 0x3a2617, 0x4b331f, 0x5d4128])
}

const OUTLINE: Rgba = Rgba(7, 6, 8, 255);
const EMBER: Rgba = Rgba(240, 120, 40, 255);
const EMBER_HOT: Rgba = Rgba(255, 206, 112, 255);
const RUST: Rgba = Rgba(84, 44, 22, 255);

/// Farbe aus einer Rampe mit stufenloser Interpolation.
fn shade(r: &Ramp, t: f32) -> Rgba {
    let n = r.0.len();
    let v = t.clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (v.floor() as usize).min(n - 2);
    r.0[i].lerp(r.0[i + 1], v - i as f32)
}

/// Periodisches Wertrauschen: Gitter wiederholt sich nach `pw`×`ph` Zellen.
fn pnoise(x: f32, y: f32, pw: i32, ph: i32, seed: u32) -> f32 {
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sy) = (s(fx), s(fy));
    let h = |ix: i32, iy: i32| noise::rand2(ix.rem_euclid(pw.max(1)), iy.rem_euclid(ph.max(1)), seed);
    let (ix, iy) = (x0 as i32, y0 as i32);
    let a = h(ix, iy) + (h(ix + 1, iy) - h(ix, iy)) * sx;
    let b = h(ix, iy + 1) + (h(ix + 1, iy + 1) - h(ix, iy + 1)) * sx;
    a + (b - a) * sy
}

/// Periodisches fBm über eine Kachel von `w`×`h` Pixeln mit `cells` Grundzellen.
fn pfbm(x: f32, y: f32, w: f32, h: f32, cells: i32, seed: u32, octaves: u32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 0.5;
    let mut norm = 0.0;
    let mut c = cells;
    for o in 0..octaves {
        sum += pnoise(x / w * c as f32, y / h * c as f32, c, c, seed.wrapping_add(o * 977)) * amp;
        norm += amp;
        amp *= 0.5;
        c *= 2;
    }
    sum / norm
}

fn put(c: &mut Canvas, x: i32, y: i32, col: Rgba) {
    if c.inside(x, y) {
        let i = (y * c.w + x) as usize;
        c.color[i] = col;
    }
}

fn image(c: &Canvas) -> ColorImage {
    ColorImage::from_rgba_unmultiplied([c.w as usize, c.h as usize], &c.albedo_bytes())
}

/// Facettierter Edelstein in Goldfassung.
fn gem(c: &mut Canvas, cx: f32, cy: f32, r: f32, ramp: &Ramp) {
    let g = gold();
    let rr = r + 1.2;
    for y in (cy - rr - 1.0) as i32..=(cy + rr + 1.0) as i32 {
        for x in (cx - rr - 1.0) as i32..=(cx + rr + 1.0) as i32 {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let d = (dx * dx + dy * dy).sqrt();
            if d <= rr + 0.6 && d > r - 0.2 {
                let l = 0.55 - (dx + dy) / rr * 0.28;
                put(c, x, y, shade(&g, l));
            } else if d <= rr + 1.4 && d > rr + 0.6 {
                put(c, x, y, OUTLINE);
            }
        }
    }
    for y in (cy - r) as i32 - 1..=(cy + r) as i32 + 1 {
        for x in (cx - r) as i32 - 1..=(cx + r) as i32 + 1 {
            let (dx, dy) = ((x as f32 + 0.5 - cx) / r, (y as f32 + 0.5 - cy) / r);
            let d = dx * dx + dy * dy;
            if d > 1.0 {
                continue;
            }
            // Facetten: Licht von links oben, Tiefe in der Mitte, Glanz unten rechts.
            let facet = if dx + dy < -0.35 { 0.7 } else if dx + dy > 0.6 { 0.62 } else { 0.38 };
            let t = facet - d * 0.18 + (1.0 - d).sqrt() * 0.15;
            put(c, x, y, shade(ramp, t));
        }
    }
    put(c, (cx - r * 0.45) as i32, (cy - r * 0.5) as i32, Rgba(255, 246, 236, 255));
    if r >= 2.5 {
        put(c, (cx - r * 0.45) as i32 + 1, (cy - r * 0.5) as i32, shade(ramp, 0.95));
        put(c, (cx + r * 0.35) as i32, (cy + r * 0.4) as i32, shade(ramp, 0.88));
    }
}

/// Kleine Niete (Kuppel, Licht von links oben).
fn rivet(c: &mut Canvas, x: i32, y: i32, ramp: &Ramp) {
    put(c, x + 1, y + 1, Rgba(4, 4, 5, 200));
    put(c, x, y - 1, shade(ramp, 0.72));
    put(c, x - 1, y, shade(ramp, 0.66));
    put(c, x, y, shade(ramp, 0.95));
    put(c, x + 1, y, shade(ramp, 0.42));
    put(c, x, y + 1, shade(ramp, 0.3));
}

/// Goldene Zierlinie mit Schatten.
fn gold_px(c: &mut Canvas, x: i32, y: i32, t: f32) {
    if c.get(x + 1, y + 1) != shade(&gold(), 0.62) {
        c.px(x + 1, y + 1, Rgba(10, 6, 3, 170));
    }
    put(c, x, y, shade(&gold(), t));
}

/// Spirale (Filigran-Ranke) als Punktfolge relativ zu (0,0).
fn curl(len: f32, r0: f32, turns: f32) -> Vec<(f32, f32)> {
    let mut v = Vec::new();
    for i in 0..=(len * 3.0) as i32 {
        v.push((i as f32 / 3.0, 0.0));
    }
    let steps = 80;
    for i in 0..=steps {
        let a = i as f32 / steps as f32 * turns * TAU;
        let r = r0 * (1.0 - i as f32 / steps as f32 * 0.85);
        v.push((len + a.sin() * r, r0 - a.cos() * r));
    }
    v
}

// ---------------------------------------------------------------------------------------------
// Rahmen (9-Slice)
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Corner {
    Stud,
    Plate { size: i32, gem_r: f32 },
    Ornate,
}

struct FrameSpec {
    size: i32,
    border: i32,
    band: (i32, i32),
    bronze: bool,
    /// Äußere Goldkante auf dem Band.
    gold_edge: bool,
    gold_line: Option<i32>,
    shadow: (i32, i32, f32),
    rivets: Option<(i32, f32)>,
    corner: Corner,
    gem: fn() -> Ramp,
    seed: u32,
}

fn frame_canvas(s: &FrameSpec) -> Canvas {
    let n = s.size;
    let period = (n - 2 * s.border).max(1) as f32;
    let mut c = Canvas::new(n, n);
    let ramp = if s.bronze { bronze() } else { iron() };
    for y in 0..n {
        for x in 0..n {
            let (tx, ty) = (x.min(n - 1 - x), y.min(n - 1 - y));
            let t = tx.min(ty);
            // Seite: 0 oben, 1 links, 2 rechts, 3 unten.
            let side = if ty <= tx {
                if y < n / 2 { 0 } else { 3 }
            } else if x < n / 2 {
                1
            } else {
                2
            };
            let a = if side == 0 || side == 3 { x } else { y } as f32;
            if t < s.band.0 {
                if t == s.band.0 - 1 {
                    put(&mut c, x, y, OUTLINE);
                }
                continue;
            }
            if t < s.band.1 {
                let w = (s.band.1 - s.band.0) as f32;
                let u = (t - s.band.0) as f32 / (w - 1.0).max(1.0);
                let hump = (u * PI).sin();
                let side_l = [0.12, 0.05, -0.07, -0.15][side];
                let mut l = 0.34 + hump * 0.26 + side_l;
                if t == s.band.0 {
                    l += if side <= 1 { 0.22 } else { -0.08 };
                }
                if t == s.band.1 - 1 {
                    l -= 0.14;
                }
                let tar = pfbm(a, t as f32 * 3.0, period, 64.0, 4, s.seed, 3);
                l += (tar - 0.5) * 0.22 + (noise::rand2(a as i32 % period as i32, t, s.seed) - 0.5) * 0.08;
                if s.gold_edge && t == s.band.0 {
                    put(&mut c, x, y, shade(&gold(), 0.5 + side_l * 1.5 + if (a as i32).rem_euclid(4) == 0 { 0.1 } else { 0.0 }));
                    continue;
                }
                if s.gold_edge && t == s.band.0 + 1 {
                    l -= 0.18;
                }
                let mut col = shade(&ramp, l);
                if noise::rand2(a as i32 % period as i32, t, s.seed + 7) < 0.03 {
                    col = shade(&ramp, l + 0.3);
                }
                let rust = pfbm(a + 13.0, t as f32 * 2.0, period, 64.0, 2, s.seed + 3, 2);
                if rust > 0.62 && !s.bronze {
                    col = col.lerp(RUST, (rust - 0.62) * 2.2);
                }
                if s.bronze && rust > 0.64 {
                    col = col.lerp(Rgba(46, 78, 64, 255), (rust - 0.64) * 1.6);
                }
                put(&mut c, x, y, col);
                continue;
            }
            if t == s.band.1 {
                put(&mut c, x, y, Rgba(10, 8, 9, 255));
                continue;
            }
            if Some(t) == s.gold_line {
                let sparkle = if (a as i32).rem_euclid(4) == 0 { 0.12 } else { 0.0 };
                let l = 0.55 + [0.15, 0.08, -0.05, -0.12][side] + sparkle;
                put(&mut c, x, y, shade(&gold(), l));
                continue;
            }
            if let Some(g) = s.gold_line
                && t == g + 1
            {
                put(&mut c, x, y, Rgba(6, 4, 3, 200));
                continue;
            }
            let (s0, len, max) = s.shadow;
            if t >= s0 && t < s0 + len {
                let k = 1.0 - (t - s0) as f32 / len as f32;
                put(&mut c, x, y, Rgba(0, 0, 0, (k * k * max * 255.0) as u8));
            }
        }
    }
    if let Some((spacing, tc)) = s.rivets {
        let r = if s.bronze || s.gold_edge { gold() } else { bronze() };
        let mut a = s.border + spacing / 2;
        while a < n - s.border {
            let t = tc.round() as i32;
            rivet(&mut c, a, t, &r);
            rivet(&mut c, a, n - 1 - t, &r);
            rivet(&mut c, t, a, &r);
            rivet(&mut c, n - 1 - t, a, &r);
            a += spacing;
        }
    }
    let gem_ramp = (s.gem)();
    let corners = [(false, false), (true, false), (false, true), (true, true)];
    match s.corner {
        Corner::Stud => {
            for (fx, fy) in corners {
                let cx = if fx { n as f32 - 3.0 } else { 3.0 };
                let cy = if fy { n as f32 - 3.0 } else { 3.0 };
                gem(&mut c, cx, cy, 1.4, &gem_ramp);
            }
        }
        Corner::Plate { size, gem_r } => {
            let b = bronze();
            for (fx, fy) in corners {
                let x0 = if fx { n - size } else { 0 };
                let y0 = if fy { n - size } else { 0 };
                for y in 0..size {
                    for x in 0..size {
                        let (px, py) = (x0 + x, y0 + y);
                        let edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
                        // Abgeschrägte Ecken der Platte.
                        let (lx, ly) = (x.min(size - 1 - x), y.min(size - 1 - y));
                        if lx + ly < 2 {
                            continue;
                        }
                        let col = if edge || lx + ly == 2 {
                            OUTLINE
                        } else {
                            let mut l = 0.5 + (noise::rand2(px, py, 91) - 0.5) * 0.1;
                            if x == 1 || y == 1 {
                                l += 0.25;
                            }
                            if x == size - 2 || y == size - 2 {
                                l -= 0.25;
                            }
                            shade(&b, l)
                        };
                        put(&mut c, px, py, col);
                    }
                }
                gem(&mut c, x0 as f32 + size as f32 / 2.0, y0 as f32 + size as f32 / 2.0, gem_r, &gem_ramp);
            }
        }
        Corner::Ornate => {
            let g = gold();
            for (fx, fy) in corners {
                let map = |x: f32, y: f32| -> (i32, i32) {
                    let px = if fx { n as f32 - 1.0 - x } else { x };
                    let py = if fy { n as f32 - 1.0 - y } else { y };
                    (px.round() as i32, py.round() as i32)
                };
                // Rautenförmige Eckplatte.
                let (cx, cy, r) = (8.0, 8.0, 8.5);
                for y in 0..18 {
                    for x in 0..18 {
                        let d = (x as f32 + 0.5 - cx).abs() + (y as f32 + 0.5 - cy).abs();
                        if d > r + 0.5 {
                            continue;
                        }
                        let (px, py) = map(x as f32, y as f32);
                        if d > r - 0.6 {
                            put(&mut c, px, py, OUTLINE);
                            continue;
                        }
                        let lx = (px as f32 + 0.5) - if fx { n as f32 - cx } else { cx };
                        let ly = (py as f32 + 0.5) - if fy { n as f32 - cy } else { cy };
                        let l = 0.55 - (lx + ly) / r * 0.3 + (noise::rand2(px, py, 5) - 0.5) * 0.08;
                        let ring = d > r - 2.2;
                        put(&mut c, px, py, if ring { shade(&g, l) } else { shade(&bronze(), l - 0.05) });
                    }
                }
                let (gx, gy) = map(8.0, 8.0);
                gem(&mut c, gx as f32 + 0.5, gy as f32 + 0.5, 3.2, &gem_ramp);
                // Ranken entlang beider Kanten und eine Lilienspitze zur Mitte.
                for (k, swap) in [(0, false), (1, true)] {
                    for (i, (u, v)) in curl(5.0, 2.6, 1.05).into_iter().enumerate() {
                        let (x, y) = (16.0 + u, 10.5 + v);
                        let (x, y) = if swap { (y, x) } else { (x, y) };
                        let (px, py) = map(x, y);
                        gold_px(&mut c, px, py, 0.55 + if i % 5 == 0 { 0.2 } else { 0.0 });
                    }
                    let _ = k;
                }
                for i in 0..5 {
                    let w: i32 = 2 - i / 2;
                    for j in -w..=w {
                        let (px, py) = map(15.0 + i as f32 + j as f32 * 0.5, 15.0 + i as f32 - j as f32 * 0.5);
                        gold_px(&mut c, px, py, 0.5 + 0.1 * (2 - j.abs()) as f32);
                    }
                }
                let (px, py) = map(21.0, 21.0);
                gem(&mut c, px as f32 + 0.5, py as f32 + 0.5, 1.3, &gem_ramp);
            }
        }
    }
    c
}

/// Textur mit 9-Slice-Geometrie; Kanten und Mitte werden gekachelt (ganzzahlig gestreckt),
/// damit Nieten und Muster nicht verzerren und keine Nähte entstehen.
pub struct NineSlice {
    tex: TextureHandle,
    size: Vec2,
    border: [f32; 4],
    pub scale: f32,
}

fn spans(start: f32, len: f32, total: f32, b0: f32, b1: f32, scale: f32) -> Vec<(f32, f32, f32, f32)> {
    let (mut d0, mut d1) = (b0 * scale, b1 * scale);
    if d0 + d1 > len {
        let k = len / (d0 + d1).max(1e-3);
        d0 *= k;
        d1 *= k;
    }
    let mid = len - d0 - d1;
    let tile = (total - b0 - b1) * scale;
    let mut v = vec![(start, start + d0, 0.0, b0 / total)];
    if mid > 0.5 && tile > 0.0 {
        let n = (mid / tile).round().max(1.0);
        let seg = mid / n;
        for i in 0..n as i32 {
            let a = start + d0 + seg * i as f32;
            v.push((a, a + seg, b0 / total, (total - b1) / total));
        }
    }
    v.push((start + len - d1, start + len, (total - b1) / total, 1.0));
    v
}

impl NineSlice {
    fn new(ctx: &egui::Context, name: &str, c: &Canvas, border: [i32; 4], scale: f32) -> NineSlice {
        let tex = ctx.load_texture(name, image(c), TextureOptions::NEAREST);
        NineSlice { tex, size: Vec2::new(c.w as f32, c.h as f32), border: border.map(|b| b as f32), scale }
    }

    pub fn mesh(&self, rect: Rect, tint: Color32, center: bool) -> Mesh {
        let mut m = Mesh::with_texture(self.tex.id());
        let cols = spans(rect.min.x, rect.width(), self.size.x, self.border[0], self.border[2], self.scale);
        let rows = spans(rect.min.y, rect.height(), self.size.y, self.border[1], self.border[3], self.scale);
        for (ri, r) in rows.iter().enumerate() {
            for (ci, col) in cols.iter().enumerate() {
                if !center && ri != 0 && ri != rows.len() - 1 && ci != 0 && ci != cols.len() - 1 {
                    continue;
                }
                if col.1 - col.0 <= 0.0 || r.1 - r.0 <= 0.0 {
                    continue;
                }
                m.add_rect_with_uv(Rect::from_min_max(Pos2::new(col.0, r.0), Pos2::new(col.1, r.1)), Rect::from_min_max(Pos2::new(col.2, r.2), Pos2::new(col.3, r.3)), tint);
            }
        }
        m
    }

    pub fn paint(&self, painter: &Painter, rect: Rect, tint: Color32) {
        painter.add(Shape::mesh(self.mesh(rect, tint, true)));
    }
}

// ---------------------------------------------------------------------------------------------
// Flächen (kachelbar)
// ---------------------------------------------------------------------------------------------

fn leather_tile() -> Canvas {
    let n = 64;
    let mut c = Canvas::new(n, n);
    let r = leather();
    for y in 0..n {
        for x in 0..n {
            let (fx, fy) = (x as f32, y as f32);
            let mottle = pfbm(fx, fy, 64.0, 64.0, 4, 11, 4);
            let grain = noise::rand2(x, y, 12);
            let mut l = 0.42 + (mottle - 0.5) * 0.55 + (grain - 0.5) * 0.12;
            // Geprägtes Rautenmuster (Lederpunzierung).
            let d1 = (x + y).rem_euclid(16);
            let d2 = (x - y).rem_euclid(16);
            if d1 == 0 || d2 == 0 {
                l -= 0.2;
            } else if d1 == 1 || d2 == 1 {
                l += 0.1;
            }
            // Punzen in der Rautenmitte.
            let (cx, cy) = ((x + y).rem_euclid(16), (x - y).rem_euclid(16));
            if (cx == 8 && cy == 8) || ((cx == 7 || cx == 9) && cy == 8) || (cx == 8 && (cy == 7 || cy == 9)) {
                l -= 0.12;
            }
            // Kratzer.
            if pnoise(fx / 64.0 * 16.0, fy / 64.0 * 2.0, 16, 2, 13) > 0.83 {
                l += 0.12;
            }
            put(&mut c, x, y, shade(&r, l));
        }
    }
    c
}

fn stone_tile() -> Canvas {
    let n = 64;
    let mut c = Canvas::new(n, n);
    let r = stone();
    for y in 0..n {
        for x in 0..n {
            let (fx, fy) = (x as f32, y as f32);
            let m = pfbm(fx, fy, 64.0, 64.0, 4, 21, 4);
            let cr = pfbm(fx, fy, 64.0, 64.0, 8, 22, 2);
            let mut l = 0.3 + (m - 0.5) * 0.4 + (noise::rand2(x, y, 23) - 0.5) * 0.1;
            if (cr - 0.5).abs() < 0.018 {
                l -= 0.18;
            }
            put(&mut c, x, y, shade(&r, l));
        }
    }
    c
}

/// Pergament mit Fasern, Flecken und Brandrändern (9-Slice mit gekachelter Mitte).
fn parchment_canvas(n: i32, border: i32, seed: u32) -> Canvas {
    let p = (n - 2 * border) as f32;
    let mut c = Canvas::new(n, n);
    let r = parchment();
    for y in 0..n {
        for x in 0..n {
            let (fx, fy) = (x as f32, y as f32);
            let (tx, ty) = (x.min(n - 1 - x), y.min(n - 1 - y));
            let t = tx.min(ty) as f32;
            let along = if ty <= tx { fx } else { fy };
            let rag = pfbm(along, if ty <= tx { (y < n / 2) as i32 as f32 * 40.0 } else { 20.0 + (x < n / 2) as i32 as f32 * 40.0 }, p, 200.0, 8, seed + 1, 3);
            let e = t + (rag - 0.5) * 7.0 + (noise::rand2(x, y, seed + 2) - 0.5) * 0.8;
            if e < 1.2 {
                continue;
            }
            let m = pfbm(fx, fy, p, p, 4, seed + 3, 4);
            let fib = pnoise(fx / p * 32.0, fy / p * 4.0, 32, 4, seed + 4);
            let fib2 = pnoise(fx / p * 4.0, fy / p * 24.0, 4, 24, seed + 5);
            let stain = pfbm(fx + 7.0, fy + 3.0, p, p, 2, seed + 6, 3);
            let mut l = 0.66 + (m - 0.5) * 0.3 + (fib - 0.5) * 0.12 + (fib2 - 0.5) * 0.06 + (noise::rand2(x, y, seed + 7) - 0.5) * 0.05;
            if stain > 0.6 {
                l -= (stain - 0.6) * 0.9;
            }
            let mut col = shade(&r, l);
            if e < 2.2 {
                col = Rgba(22, 12, 7, 255);
            } else if e < 3.4 {
                col = Rgba(48, 26, 12, 255).lerp(Rgba(80, 44, 20, 255), noise::rand2(x, y, seed + 8));
                if noise::rand2(x, y, seed + 9) < 0.08 {
                    col = EMBER.lerp(EMBER_HOT, noise::rand2(x, y, seed + 10) * 0.5);
                }
            } else if e < 9.0 {
                let k = (1.0 - (e - 3.4) / 5.6).powf(1.6);
                col = col.lerp(Rgba(92, 54, 24, 255), k * 0.85);
            }
            put(&mut c, x, y, col);
        }
    }
    c
}

/// Rolle (oberer/unterer Abschluss der Pergamentrolle), horizontal 3-Slice:
/// eingerollter Pergamentzylinder mit sichtbaren Wickellagen und Holzknäufen mit Goldkappe.
fn roll_canvas() -> Canvas {
    let (w, h) = (48, 13);
    let mut c = Canvas::new(w, h);
    let p = parchment();
    let wd = wood();
    let g = gold();
    let cy = h as f32 / 2.0;
    for y in 0..h {
        for x in 0..w {
            let v = (y as f32 + 0.5 - cy) / (h as f32 / 2.0);
            let kx = x.min(w - 1 - x);
            if kx < 6 {
                // Knauf: gedrechselte Holzkugel mit Goldkappe außen.
                let r = [0.42, 0.62, 0.78, 0.9, 0.9, 0.55][kx as usize];
                if v.abs() > r {
                    continue;
                }
                let sh = (1.0 - (v / r).powi(2)).max(0.0).sqrt();
                let edge = v.abs() > r - 0.15 || kx == 0;
                let col = if edge {
                    OUTLINE
                } else if kx <= 2 {
                    shade(&g, 0.2 + sh * 0.6 - v * 0.25)
                } else {
                    shade(&wd, 0.15 + sh * 0.75 - v * 0.3)
                };
                put(&mut c, x, y, col);
                continue;
            }
            let rv = v / 0.72;
            if rv.abs() > 1.0 {
                continue;
            }
            if rv.abs() > 0.86 {
                put(&mut c, x, y, OUTLINE);
                continue;
            }
            let sh = (1.0 - rv * rv).max(0.0).sqrt();
            // Zylinder: Glanz oben, tiefer Schatten unten.
            let mut l = 0.05 + sh * 0.55 - rv.max(0.0) * 0.45 + (0.3 - (rv + 0.4).abs()).max(0.0) * 1.1;
            l += (pnoise(x as f32 / 36.0 * 12.0, y as f32 * 0.6, 12, 64, 51) - 0.5) * 0.1;
            let mut col = shade(&p, l);
            // Wickellagen an den Rollenenden (Spirale angedeutet).
            match kx {
                6 => col = shade(&p, l * 0.4),
                7 if rv.abs() < 0.55 => col = shade(&p, l * 0.75),
                8 if rv.abs() < 0.25 => col = Rgba(46, 28, 14, 255),
                _ => {}
            }
            if rv.abs() > 0.7 {
                col = col.lerp(Rgba(64, 36, 16, 255), 0.55);
            }
            put(&mut c, x, y, col);
        }
    }
    c
}

// ---------------------------------------------------------------------------------------------
// Schaltflächen
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BtnState {
    Normal = 0,
    Hover = 1,
    Pressed = 2,
    Disabled = 3,
}

impl BtnState {
    pub fn of(resp: &Response, enabled: bool) -> BtnState {
        if !enabled {
            BtnState::Disabled
        } else if resp.is_pointer_button_down_on() {
            BtnState::Pressed
        } else if resp.hovered() {
            BtnState::Hover
        } else {
            BtnState::Normal
        }
    }
}

/// Steinplakette mit Eisenrand und Bronzenieten.
fn button_canvas(state: BtnState) -> Canvas {
    let (w, h) = (48, 24);
    let mut c = Canvas::new(w, h);
    let rim_r = match state {
        BtnState::Hover => bronze(),
        _ => iron(),
    };
    let st = stone();
    for y in 0..h {
        for x in 0..w {
            let (tx, ty) = (x.min(w - 1 - x), y.min(h - 1 - y));
            if tx + ty < 3 {
                continue;
            }
            let t = tx.min(ty).min(tx + ty - 3);
            let top_left = (ty <= tx && y < h / 2) || (tx < ty && x < w / 2);
            let col = if t == 0 {
                OUTLINE
            } else if t <= 2 {
                let mut l = if t == 1 { 0.62 } else { 0.46 };
                let lit = if state == BtnState::Pressed { !top_left } else { top_left };
                l += if lit { 0.16 } else { -0.2 };
                l += (noise::rand2(x % 28, y, 3) - 0.5) * 0.1;
                if state == BtnState::Disabled {
                    l -= 0.18;
                }
                let mut col = shade(&rim_r, l);
                if state == BtnState::Hover {
                    col = col.lerp(if t == 2 { EMBER } else { EMBER_HOT }, if t == 2 { 0.55 } else { 0.25 });
                }
                col
            } else if t == 3 {
                match state {
                    BtnState::Hover => EMBER.scale(0.75),
                    _ => Rgba(8, 7, 8, 255),
                }
            } else {
                let fy = y as f32 / h as f32;
                let m = pfbm(x as f32, y as f32 * 2.0, 28.0, 64.0, 4, 31, 3);
                let mut l = 0.5 + (0.5 - fy) * 0.22 + (m - 0.5) * 0.28 + (noise::rand2(x % 28, y, 32) - 0.5) * 0.06;
                if t == 4 {
                    l += if top_left { 0.14 } else { -0.12 };
                }
                match state {
                    BtnState::Normal => {}
                    BtnState::Hover => l += 0.08,
                    BtnState::Pressed => l = 0.32 - (0.5 - fy) * 0.18 + (m - 0.5) * 0.2,
                    BtnState::Disabled => l -= 0.2,
                }
                let mut col = shade(&st, l);
                match state {
                    BtnState::Hover => {
                        let glow = ((6 - t).max(0) as f32 / 3.0).min(1.0) * 0.35;
                        col = col.lerp(Rgba(92, 44, 18, 255), 0.22 + glow);
                    }
                    BtnState::Pressed => col = col.lerp(Rgba(70, 30, 12, 255), 0.25),
                    _ => {}
                }
                col
            };
            put(&mut c, x, y, col);
        }
    }
    if state != BtnState::Disabled {
        let r = if state == BtnState::Hover { gold() } else { bronze() };
        for (x, y) in [(5, 5), (w - 6, 5), (5, h - 6), (w - 6, h - 6)] {
            rivet(&mut c, x, y, &r);
        }
    }
    c
}

/// Quadratisches Befehlsfeld: eingelassene Steinmulde mit Eisenrand.
fn slot_canvas(n: i32, state: u8) -> Canvas {
    // state: 0 normal, 1 hover, 2 pressed, 3 disabled, 4 leer
    let mut c = Canvas::new(n, n);
    let st = stone();
    let rim = if state == 1 { bronze() } else { iron() };
    for y in 0..n {
        for x in 0..n {
            let (tx, ty) = (x.min(n - 1 - x), y.min(n - 1 - y));
            if tx + ty < 2 {
                continue;
            }
            let t = tx.min(ty);
            let top_left = (ty <= tx && y < n / 2) || (tx < ty && x < n / 2);
            let col = if t == 0 || tx + ty == 2 {
                OUTLINE
            } else if t <= 2 {
                let lit = if state == 2 { !top_left } else { top_left };
                let mut l = if t == 1 { 0.6 } else { 0.45 } + if lit { 0.17 } else { -0.2 };
                l += (noise::rand2(x, y, 41) - 0.5) * 0.1;
                if state >= 3 {
                    l -= 0.2;
                }
                let mut col = shade(&rim, l);
                if state == 1 {
                    col = col.lerp(if t == 2 { EMBER } else { EMBER_HOT }, if t == 2 { 0.6 } else { 0.3 });
                }
                col
            } else if t == 3 {
                if state == 1 { EMBER.scale(0.8) } else { Rgba(6, 5, 6, 255) }
            } else {
                let m = pfbm(x as f32, y as f32, n as f32, n as f32, 3, 42 + n as u32, 3);
                let mut l = 0.2 + (m - 0.5) * 0.22 + (noise::rand2(x, y, 43) - 0.5) * 0.05;
                // Mulde: Schatten oben links, Licht unten rechts.
                if t <= 5 {
                    l += if top_left { -0.12 } else { 0.06 } * (6 - t) as f32 / 2.0;
                }
                if state == 2 || state == 4 {
                    l -= 0.08;
                }
                let mut col = shade(&st, l);
                if state == 1 {
                    let k = ((7 - t).max(0) as f32 / 4.0).min(1.0);
                    col = col.lerp(Rgba(120, 52, 16, 255), 0.12 + k * 0.4);
                }
                col
            };
            put(&mut c, x, y, col);
        }
    }
    if state == 4 {
        // Eingeritzte Raute als Platzhalter.
        let m = n / 2;
        for i in 0..4 {
            for (x, y) in [(m - 4 + i, m - i), (m + i, m - 4 + i), (m + 4 - i, m + i), (m - i, m + 4 - i)] {
                put(&mut c, x, y, shade(&st, 0.08));
                put(&mut c, x + 1, y + 1, shade(&st, 0.32));
            }
        }
    }
    if state != 3 && state != 4 {
        let r = if state == 1 { gold() } else { bronze() };
        for (x, y) in [(2, 2), (n - 3, 2), (2, n - 3), (n - 3, n - 3)] {
            put(&mut c, x, y, shade(&r, 0.85));
        }
    }
    c
}

/// Tastenkürzel-Rune: kleiner Steinknopf mit Bronzerand.
fn rune_canvas() -> Canvas {
    let n = 11;
    let mut c = Canvas::new(n, n);
    let (st, b) = (stone(), bronze());
    for y in 0..n {
        for x in 0..n {
            let (dx, dy) = (x as f32 - 5.0, y as f32 - 5.0);
            let d = (dx * dx + dy * dy).sqrt();
            if d > 5.6 {
                continue;
            }
            let col = if d > 4.8 {
                OUTLINE
            } else if d > 3.7 {
                shade(&b, 0.55 - (dx + dy) * 0.06)
            } else {
                shade(&st, 0.2 + (dx + dy) * 0.02)
            };
            put(&mut c, x, y, col);
        }
    }
    c
}

/// Rote Siegelwachs-Marke („Geplant“).
fn seal_canvas() -> Canvas {
    let n = 18;
    let mut c = Canvas::new(n, n);
    let w = wax();
    for y in 0..n {
        for x in 0..n {
            let (dx, dy) = (x as f32 + 0.5 - 9.0, y as f32 + 0.5 - 9.0);
            let a = dy.atan2(dx);
            let r = 7.4 + (a * 7.0).sin() * 0.6 + (noise::rand2(x, y, 61) - 0.5) * 0.8;
            let d = (dx * dx + dy * dy).sqrt();
            if d > r {
                continue;
            }
            let mut l = 0.55 - (dx + dy) / 18.0;
            if (d - 5.0).abs() < 0.6 {
                l -= 0.25;
            } else if (d - 4.2).abs() < 0.5 {
                l += 0.15;
            }
            if d > r - 1.0 {
                l -= 0.2;
            }
            put(&mut c, x, y, shade(&w, l));
        }
    }
    c
}

// ---------------------------------------------------------------------------------------------
// Bänder, Balken, Minimap- und Porträtfassung
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Layer {
    Outline,
    Iron,
    Groove,
    Gold,
    Shade(u8),
}

/// Horizontales Band (gekachelt), Schichten von oben nach unten.
fn band_canvas(len: i32, layers: &[Layer], seed: u32) -> Canvas {
    let h = layers.len() as i32;
    let mut c = Canvas::new(len, h);
    let ir = iron();
    let iron_rows: Vec<i32> = (0..h).filter(|&y| matches!(layers[y as usize], Layer::Iron)).collect();
    let (i0, i1) = (iron_rows.first().copied().unwrap_or(0), iron_rows.last().copied().unwrap_or(0));
    for y in 0..h {
        for x in 0..len {
            let col = match layers[y as usize] {
                Layer::Outline => OUTLINE,
                Layer::Groove => Rgba(9, 8, 9, 255),
                Layer::Gold => shade(&gold(), 0.6 + if x % 4 == 0 { 0.12 } else { 0.0 }),
                Layer::Shade(a) => Rgba(0, 0, 0, a),
                Layer::Iron => {
                    let u = (y - i0) as f32 / (i1 - i0).max(1) as f32;
                    let mut l = 0.36 + (u * PI).sin() * 0.24;
                    if y == i0 {
                        l += 0.2;
                    }
                    if y == i1 {
                        l -= 0.15;
                    }
                    l += (pfbm(x as f32, y as f32 * 3.0, len as f32, 64.0, 4, seed, 3) - 0.5) * 0.22;
                    l += (noise::rand2(x, y, seed + 1) - 0.5) * 0.08;
                    let mut col = shade(&ir, l);
                    let rust = pfbm(x as f32 + 5.0, y as f32 * 2.0, len as f32, 64.0, 2, seed + 2, 2);
                    if rust > 0.62 {
                        col = col.lerp(RUST, (rust - 0.62) * 2.2);
                    }
                    col
                }
            };
            put(&mut c, x, y, col);
        }
    }
    let mid = (i0 + i1) / 2;
    let mut x = 8;
    while x < len {
        rivet(&mut c, x, mid, &bronze());
        x += 16;
    }
    c
}

fn transpose(c: &Canvas) -> Canvas {
    let mut o = Canvas::new(c.h, c.w);
    for y in 0..c.h {
        for x in 0..c.w {
            put(&mut o, y, x, c.get(x, y));
        }
    }
    o
}

/// Rahmen eines Lebens-/Manabalkens: Eisenrohr mit Bronzekappen, Mitte offen.
fn bar_frame_canvas() -> Canvas {
    let (w, h) = (24, 8);
    let mut c = Canvas::new(w, h);
    let (ir, b) = (iron(), bronze());
    for y in 0..h {
        for x in 0..w {
            let cap = x < 4 || x >= w - 4;
            if cap {
                let kx = if x < 4 { x } else { w - 1 - x };
                let v = (y as f32 + 0.5) / h as f32 * 2.0 - 1.0;
                let col = if kx == 0 || y == 0 || y == h - 1 { OUTLINE } else { shade(&b, 0.3 + (1.0 - v * v) * 0.45 - v * 0.15 + if kx == 2 { 0.15 } else { 0.0 }) };
                put(&mut c, x, y, col);
                continue;
            }
            let col = match y {
                0 => OUTLINE,
                1 => shade(&ir, 0.62),
                6 => shade(&ir, 0.28),
                7 => OUTLINE,
                _ => continue,
            };
            put(&mut c, x, y, col);
        }
    }
    c
}

/// Rautenförmige Fassung der Minimap (Eisen, Goldfiligran, vier Edelsteine).
pub struct MinimapFrame {
    tex: TextureHandle,
    size: Vec2,
    /// Halbachsen der inneren Raute in Texturpixeln.
    pub inner: Vec2,
}

fn minimap_canvas(hw: f32, hh: f32) -> (Canvas, Vec2) {
    let k = hw * hh / (hw * hw + hh * hh).sqrt();
    let thick = 7.0;
    let (w, h) = (((hw * (1.0 + thick / k)) + 8.0) as i32 * 2, ((hh * (1.0 + thick / k)) + 8.0) as i32 * 2);
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let mut c = Canvas::new(w, h);
    let ir = iron();
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let e = ((dx.abs() / hw + dy.abs() / hh) - 1.0) * k;
            if e < -4.0 || e > thick + 1.0 {
                continue;
            }
            if e < 0.0 {
                // Innenschatten auf der Karte.
                let a = (1.0 + e / 4.0).powf(2.0) * 0.75;
                put(&mut c, x, y, Rgba(0, 0, 0, (a * 255.0) as u8));
                continue;
            }
            let lit = match (dx < 0.0, dy < 0.0) {
                (true, true) => 0.16,
                (false, true) => 0.04,
                (true, false) => -0.06,
                (false, false) => -0.18,
            };
            let col = if e < 1.0 {
                Rgba(8, 6, 6, 255)
            } else if e < 2.0 {
                shade(&gold(), 0.58 + lit)
            } else if e < 2.8 {
                Rgba(10, 8, 8, 255)
            } else if e < thick {
                let u = (e - 2.8) / (thick - 2.8);
                let mut l = 0.34 + (u * PI).sin() * 0.26 + lit;
                if u < 0.2 {
                    l += 0.12;
                }
                l += (noise::fbm2(x as f32 * 0.15, y as f32 * 0.4, 71, 3) - 0.5) * 0.25 + (noise::rand2(x, y, 72) - 0.5) * 0.08;
                let mut col = shade(&ir, l);
                let rust = noise::fbm2(x as f32 * 0.08, y as f32 * 0.2, 73, 2);
                if rust > 0.6 {
                    col = col.lerp(RUST, (rust - 0.6) * 2.0);
                }
                col
            } else {
                OUTLINE
            };
            put(&mut c, x, y, col);
        }
    }
    // Nieten entlang der Kanten.
    let mid = (2.8 + thick) / 2.0;
    for i in 2..11 {
        if i == 6 {
            continue;
        }
        let f = i as f32 / 12.0;
        for (sx, sy) in [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
            let n = Vec2::new(sx / hw, sy / hh).normalized() * mid;
            let px = cx + sx * hw * (1.0 - f) + n.x;
            let py = cy + sy * hh * f + n.y;
            rivet(&mut c, px.floor() as i32, py.floor() as i32, &bronze());
        }
    }
    // Edelsteine an den vier Spitzen, oben der größte.
    let o = mid / k;
    gem(&mut c, cx, cy - hh * (1.0 + o) + 0.5, 4.2, &ruby());
    gem(&mut c, cx, cy + hh * (1.0 + o) - 0.5, 3.0, &ruby());
    gem(&mut c, cx - hw * (1.0 + o * 0.8), cy, 3.4, &emerald());
    gem(&mut c, cx + hw * (1.0 + o * 0.8), cy, 3.4, &emerald());
    (c, Vec2::new(w as f32, h as f32))
}


/// Geborstene, angesengte Krone (Emblem des Titels).
fn crown_canvas() -> Canvas {
    let (w, h) = (36, 24);
    let mut c = Canvas::new(w, h);
    let g = gold();
    // Zacken: (Mitte x, Spitze y); der zweite ist abgebrochen.
    let spikes = [(4.0, 6.0), (11.0, 9.5), (18.0, 1.5), (25.0, 4.5), (32.0, 6.0)];
    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;
            let band = (14.0..21.0).contains(&fy) && (2.0..34.0).contains(&fx);
            let mut spike = false;
            for (sx, top) in spikes {
                let half = 3.2 * ((fy - top) / (14.0 - top)).clamp(0.0, 1.0) + 0.6;
                if fy >= top && fy < 14.5 && (fx - sx).abs() < half {
                    spike = true;
                }
            }
            if !(band || spike) {
                continue;
            }
            let mut l = 0.5 - (fx - 18.0) / 60.0 - (fy - 12.0) / 40.0 + (noise::rand2(x, y, 101) - 0.5) * 0.12;
            if band && (fy - 14.5).abs() < 0.6 {
                l += 0.3;
            }
            if band && (fy - 20.5).abs() < 0.6 {
                l -= 0.25;
            }
            let mut col = shade(&g, l);
            // Asche und Brandspuren von links.
            let burn = noise::fbm2(fx * 0.18, fy * 0.18, 102, 3) + (12.0 - fx).max(0.0) / 30.0;
            if burn > 0.62 {
                col = col.lerp(Rgba(40, 34, 32, 255), ((burn - 0.62) * 2.5).min(0.85));
            }
            put(&mut c, x, y, col);
        }
    }
    gem(&mut c, 18.0, 6.5, 1.3, &ruby());
    gem(&mut c, 18.0, 17.5, 2.0, &ruby());
    gem(&mut c, 9.5, 17.5, 1.1, &emerald());
    gem(&mut c, 26.5, 17.5, 1.1, &emerald());
    // Riss im Reif.
    for (x, y) in [(13, 14), (13, 15), (14, 16), (13, 17), (14, 18), (14, 19), (13, 20)] {
        put(&mut c, x, y, Rgba(12, 8, 6, 255));
    }
    c.outline(0.25);
    c
}

// ---------------------------------------------------------------------------------------------
// Sammlung aller UI-Texturen
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PanelKind {
    /// Prunkrahmen mit Bronze, Goldranken und Edelsteinen (Menüs, Dialoge).
    Ornate,
    /// Schmaler Bronzerahmen (Tooltips, Hinweise).
    Thin,
    /// Bronzeplakette (Ressourcen).
    Plaque,
    /// Pergament mit Brandrändern.
    Parchment,
}

pub struct UiArt {
    leather: TextureHandle,
    stone: TextureHandle,
    ornate: NineSlice,
    thin: NineSlice,
    plaque: NineSlice,
    parchment: NineSlice,
    roll: NineSlice,
    pub portrait: NineSlice,
    buttons: [NineSlice; 4],
    slots: Vec<TextureHandle>,
    mini_slots: Vec<TextureHandle>,
    bar_frame: NineSlice,
    glass: TextureHandle,
    glow: TextureHandle,
    divider: TextureHandle,
    rune: TextureHandle,
    seal: TextureHandle,
    crown: TextureHandle,
    band_top: TextureHandle,
    band_bottom: TextureHandle,
    pillar: TextureHandle,
    pub minimap: MinimapFrame,
}

fn repeat() -> TextureOptions {
    TextureOptions { wrap_mode: egui::TextureWrapMode::Repeat, ..TextureOptions::NEAREST }
}

impl UiArt {
    pub fn new(ctx: &egui::Context) -> UiArt {
        let start = std::time::Instant::now();
        let ornate_spec = FrameSpec { size: 80, border: 24, band: (1, 8), bronze: false, gold_edge: true, gold_line: Some(9), shadow: (11, 7, 0.6), rivets: Some((16, 4.0)), corner: Corner::Ornate, gem: ruby, seed: 2 };
        let thin_spec = FrameSpec { size: 28, border: 7, band: (1, 3), bronze: true, gold_edge: false, gold_line: Some(4), shadow: (6, 3, 0.45), rivets: None, corner: Corner::Stud, gem: ruby, seed: 3 };
        let plaque_spec = FrameSpec { size: 32, border: 8, band: (1, 4), bronze: true, gold_edge: false, gold_line: Some(5), shadow: (7, 3, 0.5), rivets: None, corner: Corner::Stud, gem: emerald, seed: 4 };
        let portrait_spec = FrameSpec { size: 56, border: 7, band: (1, 5), bronze: false, gold_edge: true, gold_line: Some(6), shadow: (7, 4, 0.7), rivets: None, corner: Corner::Plate { size: 9, gem_r: 2.3 }, gem: ruby, seed: 5 };
        let glow = {
            let n = 64;
            let mut px = Vec::with_capacity(n * n * 4);
            for y in 0..n {
                for x in 0..n {
                    let (dx, dy) = ((x as f32 + 0.5) / n as f32 * 2.0 - 1.0, (y as f32 + 0.5) / n as f32 * 2.0 - 1.0);
                    let d = (dx * dx + dy * dy).sqrt().min(1.0);
                    let a = ((1.0 - d).powf(2.2) * 255.0) as u8;
                    px.extend_from_slice(&[255, 255, 255, a]);
                }
            }
            ctx.load_texture("ui_glow", ColorImage::from_rgba_unmultiplied([n, n], &px), TextureOptions::LINEAR)
        };
        let glass = {
            let rows: [u8; 16] = [150, 110, 70, 40, 18, 6, 0, 0, 0, 0, 0, 0, 8, 18, 30, 40];
            let px: Vec<u8> = rows.iter().flat_map(|&a| [255, 255, 255, a]).collect();
            ctx.load_texture("ui_glass", ColorImage::from_rgba_unmultiplied([1, 16], &px), TextureOptions::LINEAR)
        };
        let divider = {
            let mut c = Canvas::new(48, 11);
            let g = gold();
            for x in 0..48 {
                let line = (6..42).contains(&x);
                if line {
                    put(&mut c, x, 4, shade(&g, 0.75));
                    put(&mut c, x, 5, shade(&g, 0.5));
                    put(&mut c, x, 6, Rgba(8, 5, 3, 200));
                }
            }
            for (cx, s) in [(3, 1.0), (44, -1.0)] {
                for d in 0..=2i32 {
                    for (x, y) in [(cx - d, 5 - (2 - d)), (cx - d, 5 + (2 - d)), (cx + d, 5 - (2 - d)), (cx + d, 5 + (2 - d))] {
                        put(&mut c, x, y, shade(&g, 0.7 - d as f32 * 0.1));
                    }
                }
                put(&mut c, cx, 5, shade(&ruby(), 0.6));
                let _ = s;
            }
            // Mittlere Raute mit Rubin.
            for y in 0..11 {
                for x in 17..31 {
                    let d = (x as f32 - 23.5).abs() * 0.8 + (y as f32 - 5.0).abs();
                    if d > 5.6 {
                        continue;
                    }
                    let col = if d > 4.8 { OUTLINE } else if d > 3.2 { shade(&g, 0.62 - (x as f32 - 23.5 + y as f32 - 5.0) * 0.05) } else { shade(&ruby(), 0.55 - (x as f32 - 23.5 + y as f32 - 5.0) * 0.08) };
                    put(&mut c, x, y, col);
                }
            }
            put(&mut c, 22, 4, Rgba(255, 236, 220, 255));
            ctx.load_texture("ui_divider", image(&c), TextureOptions::NEAREST)
        };
        let band_top = band_canvas(32, &[Layer::Gold, Layer::Groove, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Outline, Layer::Shade(120), Layer::Shade(50)], 81);
        let band_bottom = band_canvas(32, &[Layer::Shade(50), Layer::Shade(120), Layer::Outline, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Groove, Layer::Gold, Layer::Shade(160)], 82);
        let pillar = transpose(&band_canvas(32, &[Layer::Shade(90), Layer::Outline, Layer::Gold, Layer::Groove, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Iron, Layer::Groove, Layer::Gold, Layer::Outline, Layer::Shade(90)], 83));
        let (mm, mm_size) = minimap_canvas(58.0, 29.0);
        let art = UiArt {
            leather: ctx.load_texture("ui_leather", image(&leather_tile()), repeat()),
            stone: ctx.load_texture("ui_stone", image(&stone_tile()), repeat()),
            ornate: NineSlice::new(ctx, "ui_frame_ornate", &frame_canvas(&ornate_spec), [24; 4], PX),
            thin: NineSlice::new(ctx, "ui_frame_thin", &frame_canvas(&thin_spec), [7; 4], PX),
            plaque: NineSlice::new(ctx, "ui_frame_plaque", &frame_canvas(&plaque_spec), [8; 4], PX),
            parchment: NineSlice::new(ctx, "ui_parchment", &parchment_canvas(96, 16, 7), [16; 4], PX),
            roll: NineSlice::new(ctx, "ui_roll", &roll_canvas(), [9, 0, 9, 0], PX),
            portrait: NineSlice::new(ctx, "ui_portrait", &frame_canvas(&portrait_spec), [7; 4], PX),
            buttons: [BtnState::Normal, BtnState::Hover, BtnState::Pressed, BtnState::Disabled].map(|s| NineSlice::new(ctx, &format!("ui_button_{s:?}"), &button_canvas(s), [10, 8, 10, 8], PX)),
            slots: (0..5).map(|s| ctx.load_texture(format!("ui_slot_{s}"), image(&slot_canvas(24, s)), TextureOptions::NEAREST)).collect(),
            mini_slots: (0..5).map(|s| ctx.load_texture(format!("ui_mslot_{s}"), image(&slot_canvas(19, s)), TextureOptions::NEAREST)).collect(),
            bar_frame: NineSlice::new(ctx, "ui_bar", &bar_frame_canvas(), [4, 2, 4, 2], PX),
            glass,
            glow,
            divider,
            rune: ctx.load_texture("ui_rune", image(&rune_canvas()), TextureOptions::NEAREST),
            seal: ctx.load_texture("ui_seal", image(&seal_canvas()), TextureOptions::NEAREST),
            crown: ctx.load_texture("ui_crown", image(&crown_canvas()), TextureOptions::NEAREST),
            band_top: ctx.load_texture("ui_band_top", image(&band_top), repeat()),
            band_bottom: ctx.load_texture("ui_band_bottom", image(&band_bottom), repeat()),
            pillar: ctx.load_texture("ui_pillar", image(&pillar), repeat()),
            minimap: MinimapFrame { tex: ctx.load_texture("ui_minimap_frame", image(&mm), TextureOptions::NEAREST), size: mm_size, inner: Vec2::new(58.0, 29.0) },
        };
        log::info!("UI-Grafik in {:.0} ms erzeugt", start.elapsed().as_secs_f32() * 1000.0);
        art
    }

    fn nine(&self, kind: PanelKind) -> &NineSlice {
        match kind {
            PanelKind::Ornate => &self.ornate,
            PanelKind::Thin => &self.thin,
            PanelKind::Plaque => &self.plaque,
            PanelKind::Parchment => &self.parchment,
        }
    }

    /// Gekachelte Füllfläche (Leder oder Stein).
    pub fn fill_shape(&self, rect: Rect, stone: bool, tint: Color32) -> Shape {
        let tex = if stone { &self.stone } else { &self.leather };
        let s = 64.0 * PX;
        let uv = Rect::from_min_max(Pos2::new(rect.min.x / s, rect.min.y / s), Pos2::new(rect.max.x / s, rect.max.y / s));
        let mut m = Mesh::with_texture(tex.id());
        m.add_rect_with_uv(rect, uv, tint);
        Shape::mesh(m)
    }

    /// Alle Formen eines Panels (Schatten, Füllung, Rahmen).
    pub fn panel_shapes(&self, kind: PanelKind, rect: Rect, shadow: bool) -> Vec<Shape> {
        self.panel_shapes_tinted(kind, rect, shadow, Color32::WHITE)
    }

    /// Wie [`Self::panel_shapes`], Rahmen und Füllung mit `tint` multipliziert.
    pub fn panel_shapes_tinted(&self, kind: PanelKind, rect: Rect, shadow: bool, frame_tint: Color32) -> Vec<Shape> {
        let rect = rect.round_ui();
        let mut v = Vec::new();
        if shadow {
            let s = egui::Shadow { offset: [0, 8], blur: 24, spread: 0, color: Color32::from_black_alpha(170) };
            v.push(Shape::from(s.as_shape(rect.shrink(6.0), egui::CornerRadius::same(8))));
        }
        let nine = self.nine(kind);
        match kind {
            PanelKind::Parchment => v.push(Shape::mesh(nine.mesh(rect, frame_tint, true))),
            _ => {
                let inset = match kind {
                    PanelKind::Ornate => 8.0,
                    PanelKind::Thin => 3.0,
                    _ => 4.0,
                } * PX;
                let tint = match kind {
                    PanelKind::Thin => Color32::from_gray(170),
                    PanelKind::Plaque => Color32::from_gray(150),
                    _ => Color32::WHITE,
                };
                v.push(self.fill_shape(rect.shrink(inset), false, tint));
                v.push(Shape::mesh(nine.mesh(rect, frame_tint, false)));
            }
        }
        v
    }

    /// Zeichnet ein Panel hinter die Inhalte von `add` (Größe ergibt sich aus dem Inhalt).
    pub fn panel<R>(&self, ui: &mut Ui, kind: PanelKind, margin: impl Into<egui::Margin>, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        self.panel_tinted(ui, kind, margin, Color32::WHITE, add)
    }

    pub fn panel_tinted<R>(&self, ui: &mut Ui, kind: PanelKind, margin: impl Into<egui::Margin>, tint: Color32, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        let idx = ui.painter().add(Shape::Noop);
        let inner = egui::Frame::new().inner_margin(margin).show(ui, add);
        let rect = inner.response.rect;
        ui.painter().set(idx, Shape::Vec(self.panel_shapes_tinted(kind, rect, kind != PanelKind::Plaque, tint)));
        inner
    }

    /// Pergamentrolle: Pergament mit Rollen oben und unten.
    pub fn scroll<R>(&self, ui: &mut Ui, margin: impl Into<egui::Margin>, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        let idx = ui.painter().add(Shape::Noop);
        let inner = egui::Frame::new().inner_margin(margin).show(ui, add);
        let rect = inner.response.rect.round_ui();
        let body = Rect::from_min_max(rect.min + Vec2::new(4.0, 10.0), rect.max - Vec2::new(4.0, 10.0));
        let mut shapes = self.panel_shapes(PanelKind::Parchment, body, true);
        let rh = 13.0 * PX;
        for y in [rect.min.y + 1.0, rect.max.y - rh - 1.0] {
            shapes.push(Shape::mesh(self.roll.mesh(Rect::from_min_size(Pos2::new(rect.min.x - 6.0, y), Vec2::new(rect.width() + 12.0, rh)), Color32::WHITE, true)));
        }
        ui.painter().set(idx, Shape::Vec(shapes));
        inner
    }

    pub fn button_nine(&self, s: BtnState) -> &NineSlice {
        &self.buttons[s as usize]
    }

    pub fn slot_tex(&self, state: u8, mini: bool) -> TextureId {
        let v = if mini { &self.mini_slots } else { &self.slots };
        v[state.min(4) as usize].id()
    }

    pub fn image(&self, painter: &Painter, tex: TextureId, rect: Rect, tint: Color32) {
        painter.image(tex, rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), tint);
    }

    /// Additiver Glutschein.
    pub fn glow(&self, painter: &Painter, rect: Rect, color: Color32) {
        painter.image(self.glow.id(), rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), color.additive());
    }

    /// Dunkler weicher Schatten (normale Mischung).
    pub fn soft_shadow(&self, painter: &Painter, rect: Rect, alpha: f32) {
        painter.image(self.glow.id(), rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::from_black_alpha((alpha * 255.0) as u8));
    }

    /// Kronen-Emblem, `scale` Punkte pro Pixel, zentriert auf `center`.
    pub fn crown(&self, painter: &Painter, center: Pos2, scale: f32) {
        let s = self.crown.size_vec2() * scale;
        self.image(painter, self.crown.id(), Rect::from_center_size(center, s).round_ui(), Color32::WHITE);
    }

    pub fn seal(&self, painter: &Painter, center: Pos2, size: f32) {
        self.image(painter, self.seal.id(), Rect::from_center_size(center, Vec2::splat(size)), Color32::WHITE);
    }

    /// Horizontales Band mit Nieten (oberer Leistenabschluss = `top`, Konsolenkante sonst).
    pub fn band(&self, painter: &Painter, x0: f32, x1: f32, y: f32, top: bool) {
        let tex = if top { &self.band_top } else { &self.band_bottom };
        let h = tex.size()[1] as f32 * PX;
        let rect = Rect::from_min_max(Pos2::new(x0, y), Pos2::new(x1, y + h));
        let mut m = Mesh::with_texture(tex.id());
        m.add_rect_with_uv(rect, Rect::from_min_max(Pos2::new(x0 / (32.0 * PX), 0.0), Pos2::new(x1 / (32.0 * PX), 1.0)), Color32::WHITE);
        painter.add(Shape::mesh(m));
    }

    pub fn band_height(&self, top: bool) -> f32 {
        (if top { &self.band_top } else { &self.band_bottom }).size()[1] as f32 * PX
    }

    /// Senkrechte Trennsäule.
    pub fn pillar(&self, painter: &Painter, x: f32, y0: f32, y1: f32) {
        let w = self.pillar.size()[0] as f32 * PX;
        let rect = Rect::from_min_max(Pos2::new(x - w / 2.0, y0), Pos2::new(x + w / 2.0, y1));
        let mut m = Mesh::with_texture(self.pillar.id());
        m.add_rect_with_uv(rect, Rect::from_min_max(Pos2::new(0.0, y0 / (32.0 * PX)), Pos2::new(1.0, y1 / (32.0 * PX))), Color32::WHITE);
        painter.add(Shape::mesh(m));
    }

    /// Zierleiste mit Mittelrubin von `x0` bis `x1`.
    pub fn divider(&self, painter: &Painter, x0: f32, x1: f32, cy: f32) {
        let h = 11.0 * PX;
        let y0 = (cy - h / 2.0).round();
        let tw = 48.0;
        let parts: [(f32, f32, f32, f32); 5] = {
            let cap = 7.0 * PX;
            let mid = 14.0 * PX;
            let c = (x0 + x1) / 2.0;
            [(x0, x0 + cap, 0.0, 7.0), (x0 + cap, c - mid / 2.0, 7.0, 8.0), (c - mid / 2.0, c + mid / 2.0, 17.0, 31.0), (c + mid / 2.0, x1 - cap, 40.0, 41.0), (x1 - cap, x1, 41.0, 48.0)]
        };
        let mut m = Mesh::with_texture(self.divider.id());
        for (a, b, u0, u1) in parts {
            if b > a {
                m.add_rect_with_uv(Rect::from_min_max(Pos2::new(a.round(), y0), Pos2::new(b.round(), y0 + h)), Rect::from_min_max(Pos2::new(u0 / tw, 0.0), Pos2::new(u1 / tw, 1.0)), Color32::WHITE);
            }
        }
        painter.add(Shape::mesh(m));
    }

    /// Glas-/Metallbalken mit Verlauf und Glanzlicht.
    pub fn bar(&self, painter: &Painter, rect: Rect, frac: f32, color: Color32, text: &str) {
        let rect = rect.round_ui();
        let inner = rect.shrink2(Vec2::new(3.0 * PX, PX));
        painter.rect_filled(inner, 0.0, Color32::from_rgb(10, 7, 6));
        let f = frac.clamp(0.0, 1.0);
        if f > 0.0 {
            let mut fill = inner;
            fill.set_width(inner.width() * f);
            let top = color.lerp_to_gamma(Color32::WHITE, 0.25);
            let bottom = color.lerp_to_gamma(Color32::BLACK, 0.45);
            let mut m = Mesh::default();
            let i = m.vertices.len() as u32;
            m.colored_vertex(fill.left_top(), top);
            m.colored_vertex(fill.right_top(), top);
            m.colored_vertex(fill.right_bottom(), bottom);
            m.colored_vertex(fill.left_bottom(), bottom);
            m.add_triangle(i, i + 1, i + 2);
            m.add_triangle(i, i + 2, i + 3);
            painter.add(Shape::mesh(m));
            // Leuchtende Kante am Füllstand.
            if f < 1.0 {
                painter.rect_filled(Rect::from_min_max(Pos2::new(fill.max.x - 1.0, fill.min.y), fill.max), 0.0, color.lerp_to_gamma(Color32::WHITE, 0.55));
            }
        }
        painter.image(self.glass.id(), inner, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        self.bar_frame.paint(painter, rect, Color32::WHITE);
        if !text.is_empty() {
            let font = theme::body_bold_font((rect.height() * 0.72).clamp(11.0, 15.0));
            let p = rect.center() + Vec2::new(0.0, 0.5);
            painter.text(p + Vec2::new(1.0, 1.0), Align2::CENTER_CENTER, text, font.clone(), Color32::from_black_alpha(220));
            painter.text(p, Align2::CENTER_CENTER, text, font, Color32::from_rgb(250, 240, 220));
        }
    }

    /// Tastenkürzel-Rune mit leuchtendem Buchstaben.
    pub fn rune(&self, painter: &Painter, center: Pos2, key: &str, lit: bool) {
        let size = 11.0 * PX;
        let r = Rect::from_center_size(center, Vec2::splat(size)).round_ui();
        self.image(painter, self.rune.id(), r, Color32::WHITE);
        let col = if lit { theme::EMBER_HOT } else { Color32::from_rgb(150, 120, 80) };
        if lit {
            self.glow(painter, r.expand(4.0), Color32::from_rgb(90, 40, 10));
        }
        let c = r.center();
        match key {
            "+" | "-" | "–" => {
                let st = egui::Stroke::new(2.0, col);
                painter.line_segment([c - Vec2::new(4.0, 0.0), c + Vec2::new(4.0, 0.0)], st);
                if key == "+" {
                    painter.line_segment([c - Vec2::new(0.0, 4.0), c + Vec2::new(0.0, 4.0)], st);
                }
            }
            _ => {
                let fs = if key.chars().count() > 1 { 9.0 } else { 12.0 };
                painter.text(c + Vec2::new(0.0, 0.5), Align2::CENTER_CENTER, key, theme::heading_font(fs), col);
            }
        }
    }

    /// Minimapfassung um die Kartenfläche `map_rect` (Raute in deren Umriss).
    pub fn minimap_frame(&self, painter: &Painter, map_rect: Rect) {
        let m = &self.minimap;
        let scale = map_rect.width() / (m.inner.x * 2.0);
        let size = m.size * scale;
        let r = Rect::from_center_size(map_rect.center(), size);
        painter.image(m.tex.id(), r, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
    }

}

// ---------------------------------------------------------------------------------------------
// Text mit Gold-Verlauf, Kontur und Glut
// ---------------------------------------------------------------------------------------------

pub struct TextFx {
    pub top: Color32,
    pub bottom: Color32,
    pub outline: Color32,
    pub glow: Option<Color32>,
    pub shadow: f32,
    /// Zusätzlicher Buchstabenabstand (Punkte).
    pub spacing: f32,
}

impl TextFx {
    pub fn gold() -> TextFx {
        TextFx { top: Color32::from_rgb(255, 240, 190), bottom: Color32::from_rgb(196, 128, 40), outline: Color32::from_rgb(20, 10, 4), glow: Some(Color32::from_rgb(120, 52, 10)), shadow: 3.0, spacing: 0.0 }
    }
    pub fn ash() -> TextFx {
        TextFx { top: Color32::from_rgb(196, 188, 180), bottom: Color32::from_rgb(110, 100, 96), outline: Color32::from_rgb(14, 8, 8), glow: Some(Color32::from_rgb(120, 14, 10)), shadow: 3.0, spacing: 0.0 }
    }
}

/// Text mit vertikalem Farbverlauf (Galley-Vertices werden umgefärbt).
fn job(text: &str, font: FontId, color: Color32, spacing: f32) -> egui::text::LayoutJob {
    egui::text::LayoutJob::single_section(text.to_string(), egui::TextFormat { font_id: font, color, extra_letter_spacing: spacing, ..Default::default() })
}

pub fn gradient_galley(painter: &Painter, text: &str, font: FontId, top: Color32, bottom: Color32, spacing: f32) -> Arc<egui::Galley> {
    let galley = painter.layout_job(job(text, font, Color32::WHITE, spacing));
    let mut g = (*galley).clone();
    let b = g.mesh_bounds;
    for row in &mut g.rows {
        let oy = row.pos.y;
        let r = Arc::make_mut(&mut row.row);
        for v in &mut r.visuals.mesh.vertices {
            let t = ((v.pos.y + oy - b.min.y) / b.height().max(1.0)).clamp(0.0, 1.0);
            // Glanzkante im oberen Drittel.
            let t2 = if t < 0.45 { t / 0.45 * 0.35 } else { 0.35 + (t - 0.45) / 0.55 * 0.65 };
            v.color = top.lerp_to_gamma(bottom, t2);
        }
    }
    Arc::new(g)
}

/// Zierschrift mit Kontur, Schlagschatten, Glutschein und Verlauf. Liefert das Rechteck.
pub fn fx_text(art: &UiArt, painter: &Painter, pos: Pos2, anchor: Align2, text: &str, font: FontId, fx: &TextFx) -> Rect {
    let g = gradient_galley(painter, text, font.clone(), fx.top, fx.bottom, fx.spacing);
    let rect = anchor.anchor_size(pos, g.size()).round_ui();
    if let Some(glow) = fx.glow {
        let gr = rect.expand2(Vec2::new(rect.height() * 0.9, rect.height() * 0.7));
        art.glow(painter, gr, glow);
    }
    let dark = painter.layout_job(job(text, font, fx.outline, fx.spacing));
    if fx.shadow > 0.0 {
        painter.galley(rect.min + Vec2::new(0.0, fx.shadow), dark.clone(), fx.outline);
    }
    let o = (rect.height() / 40.0).clamp(1.0, 2.0);
    for (dx, dy) in [(-o, 0.0), (o, 0.0), (0.0, -o), (0.0, o), (-o, -o), (o, o), (-o, o), (o, -o)] {
        painter.galley(rect.min + Vec2::new(dx, dy), dark.clone(), fx.outline);
    }
    painter.galley(rect.min, g, Color32::WHITE);
    rect
}

/// Schwebende Glutpartikel (zeitbasiert, zustandslos).
pub fn embers(art: &UiArt, painter: &Painter, area: Rect, t: f32, count: usize, seed: u32) {
    for i in 0..count {
        let r = |k: u32| noise::rand2(i as i32, k as i32, seed);
        let speed = 18.0 + r(1) * 46.0;
        let life = area.height() * (0.45 + r(2) * 0.55);
        let phase = r(3) * life;
        let p = (t * speed + phase) % life;
        let k = p / life;
        let x = area.min.x + r(4) * area.width() + (t * (0.4 + r(5)) + r(6) * TAU).sin() * (8.0 + r(7) * 16.0);
        let y = area.max.y - p;
        let fade = (k * 6.0).min(1.0) * (1.0 - k).powf(1.4);
        let flicker = 0.75 + 0.25 * (t * (7.0 + r(8) * 9.0) + i as f32).sin();
        let a = fade * flicker;
        let size = 1.5 + r(9) * 2.5;
        let p = Pos2::new(x, y).round();
        art.glow(painter, Rect::from_center_size(p, Vec2::splat(size * 10.0)), Color32::from_rgb((170.0 * a) as u8, (62.0 * a) as u8, (12.0 * a) as u8));
        painter.rect_filled(Rect::from_center_size(p, Vec2::splat(size.round())), 0.0, Color32::from_rgb(255, (150.0 + 80.0 * r(10)) as u8, 70).additive().gamma_multiply(a));
        if size > 2.6 {
            painter.rect_filled(Rect::from_center_size(p, Vec2::splat(1.0)), 0.0, Color32::from_rgb(255, 240, 200).additive().gamma_multiply(a));
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Widgets
// ---------------------------------------------------------------------------------------------

/// Klang bei Hover-Beginn und Klick; liefert `true` bei Hover-Beginn.
pub fn hover_started(ui: &Ui, resp: &Response) -> bool {
    let id = resp.id.with("hover_snd");
    let now = resp.hovered();
    let before = ui.ctx().data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));
    if now != before {
        ui.ctx().data_mut(|d| d.insert_temp(id, now));
    }
    now && !before
}

/// Steinplaketten-Schaltfläche mit Cinzel-Beschriftung.
pub fn button(ui: &mut Ui, art: &UiArt, text: &str, size: Vec2, enabled: bool) -> Response {
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });
    if ui.is_rect_visible(rect) {
        paint_button(ui, art, rect, &resp, text, enabled, false);
    }
    resp
}

fn paint_button(ui: &Ui, art: &UiArt, rect: Rect, resp: &Response, text: &str, enabled: bool, selected: bool) {
    let painter = ui.painter();
    let mut st = BtnState::of(resp, enabled);
    if selected && st == BtnState::Normal {
        st = BtnState::Pressed;
    }
    let r = rect.round_ui();
    art.button_nine(st).paint(painter, r, Color32::WHITE);
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("hov"), st == BtnState::Hover, 0.15);
    if hov > 0.0 {
        art.glow(painter, Rect::from_center_size(r.center(), Vec2::new(r.width() * 0.9, r.height() * 1.4)), Color32::from_rgb((70.0 * hov) as u8, (28.0 * hov) as u8, (6.0 * hov) as u8));
    }
    let col = match st {
        BtnState::Disabled => Color32::from_rgb(110, 102, 92),
        BtnState::Hover => theme::GOLD_BRIGHT,
        BtnState::Pressed if selected => theme::EMBER_HOT,
        BtnState::Pressed => Color32::from_rgb(255, 226, 170),
        BtnState::Normal => Color32::from_rgb(226, 206, 166),
    };
    let off = if st == BtnState::Pressed { Vec2::new(0.0, 1.0) } else { Vec2::ZERO };
    let fs = (r.height() * 0.42).clamp(12.0, 18.0);
    let font = theme::caps_font(fs);
    let c = r.center() + off;
    painter.text(c + Vec2::new(1.0, 1.5), Align2::CENTER_CENTER, text, font.clone(), Color32::from_black_alpha(230));
    painter.text(c, Align2::CENTER_CENTER, text, font, col);
}

/// Auswahlknopf (gedrückt dargestellt, wenn gewählt).
pub fn choice(ui: &mut Ui, art: &UiArt, text: &str, selected: bool) -> Response {
    let font = theme::caps_font(13.0);
    let w = ui.painter().layout_no_wrap(text.to_string(), font, Color32::WHITE).size().x + 28.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w.max(46.0), 30.0), Sense::click());
    if ui.is_rect_visible(rect) {
        paint_button(ui, art, rect, &resp, text, true, selected);
        if selected {
            let p = ui.painter();
            let y = rect.max.y - 5.0;
            p.line_segment([Pos2::new(rect.min.x + 12.0, y), Pos2::new(rect.max.x - 12.0, y)], egui::Stroke::new(1.5, theme::EMBER));
        }
    }
    resp
}

/// Großer Menüpunkt: Hover mit Glutlinie, Versatz und Rautenmarken; „Geplant“-Siegel.
pub fn menu_item(ui: &mut Ui, art: &UiArt, text: &str, width: f32, enabled: bool, planned: bool) -> Response {
    let size = Vec2::new(width, 42.0);
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });
    if !ui.is_rect_visible(rect) {
        return resp;
    }
    let painter = ui.painter();
    let st = BtnState::of(&resp, enabled);
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("mi"), st == BtnState::Hover || st == BtnState::Pressed, 0.18);
    let r = rect.round_ui();
    art.button_nine(st).paint(painter, r, Color32::WHITE);
    if hov > 0.0 {
        art.glow(painter, Rect::from_center_size(r.center() + Vec2::new(0.0, 6.0), Vec2::new(r.width() * 1.05, r.height() * 1.6)), Color32::from_rgb((90.0 * hov) as u8, (36.0 * hov) as u8, (8.0 * hov) as u8));
    }
    let shift = hov * 6.0 + if st == BtnState::Pressed { 1.0 } else { 0.0 };
    let font = theme::caps_font(17.0);
    let col = match st {
        BtnState::Disabled => Color32::from_rgb(106, 98, 90),
        BtnState::Normal => Color32::from_rgb(230, 212, 172),
        _ => theme::GOLD_BRIGHT,
    };
    let c = r.center() + Vec2::new(shift, if st == BtnState::Pressed { 1.0 } else { 0.0 });
    let g = painter.layout_no_wrap(text.to_string(), font.clone(), col);
    let tw = g.size().x;
    painter.text(c + Vec2::new(1.0, 1.5), Align2::CENTER_CENTER, text, font.clone(), Color32::from_black_alpha(230));
    painter.text(c, Align2::CENTER_CENTER, text, font, col);
    if hov > 0.0 {
        // Glutlinie unter dem Text und Rautenmarken links/rechts.
        let y = r.max.y - 9.0;
        let half = (tw / 2.0 + 18.0) * hov;
        let ember = theme::EMBER.gamma_multiply(hov);
        painter.line_segment([Pos2::new(c.x - half, y), Pos2::new(c.x + half, y)], egui::Stroke::new(1.5, ember));
        painter.line_segment([Pos2::new(c.x - half * 0.6, y + 1.5), Pos2::new(c.x + half * 0.6, y + 1.5)], egui::Stroke::new(1.0, theme::EMBER_HOT.gamma_multiply(hov * 0.6)));
        for sx in [-1.0, 1.0] {
            let p = Pos2::new(c.x + sx * (tw / 2.0 + 16.0), c.y);
            let d = 4.0 * hov;
            painter.add(Shape::convex_polygon(vec![p + Vec2::new(0.0, -d), p + Vec2::new(d, 0.0), p + Vec2::new(0.0, d), p + Vec2::new(-d, 0.0)], theme::EMBER_HOT.gamma_multiply(hov), egui::Stroke::NONE));
        }
    }
    if planned {
        let sc = Pos2::new(r.max.x - 26.0, r.center().y);
        art.seal(painter, sc + Vec2::new(-32.0, 0.0), 28.0);
        painter.text(sc + Vec2::new(4.0, 0.0), Align2::CENTER_CENTER, "Geplant", theme::italic_font(13.0), Color32::from_rgb(168, 132, 110));
    }
    resp
}

/// Gestylter Tooltip (schmaler Bronzerahmen, Lederfüllung).
pub fn tooltip(resp: Response, art: &UiArt, add: impl FnOnce(&mut Ui)) -> Response {
    tooltip_at(resp, art, None, add)
}

/// Tooltip mit bevorzugter Seite (`None`: automatisch – in der unteren Bildhälfte nach oben).
pub fn tooltip_at(resp: Response, art: &UiArt, side: Option<egui::RectAlign>, add: impl FnOnce(&mut Ui)) -> Response {
    let mut t = egui::Tooltip::for_enabled(&resp);
    t.popup = t.popup.frame(egui::Frame::NONE).gap(6.0);
    if let Some(a) = side {
        t.popup = t.popup.align(a);
    } else if resp.rect.center().y > resp.ctx.content_rect().center().y {
        // HUD-Konsole: nach oben aufklappen, damit keine Befehle verdeckt werden.
        t.popup = t.popup.align(egui::RectAlign::TOP_END).align_alternatives(&[egui::RectAlign::TOP_START, egui::RectAlign::TOP]);
    }
    t.show(|ui| {
        ui.set_max_width(340.0);
        art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(16, 13), add);
    });
    resp
}

/// Tooltip mit Titel und Text.
pub fn tip(resp: Response, art: &UiArt, title: &str, body: &str) -> Response {
    tooltip(resp, art, |ui| {
        ui.label(egui::RichText::new(title).font(theme::heading_font(15.0)).color(theme::GOLD));
        if !body.is_empty() {
            ui.label(egui::RichText::new(body).size(14.5).color(theme::PARCHMENT));
        }
    })
}

/// Überschrift mit Zierleiste darunter.
pub fn heading(ui: &mut Ui, art: &UiArt, text: &str, size: f32) {
    ui.vertical_centered(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), size * 1.3), Sense::hover());
        fx_text(art, ui.painter(), rect.center(), Align2::CENTER_CENTER, text, theme::heading_font(size), &TextFx { glow: Some(Color32::from_rgb(60, 26, 6)), shadow: 2.0, ..TextFx::gold() });
        let (r2, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 22.0), Sense::hover());
        let w = (rect.width() * 0.7).min(360.0);
        art.divider(ui.painter(), r2.center().x - w / 2.0, r2.center().x + w / 2.0, r2.center().y);
    });
}

/// Abklingzeit-Überlagerung: dunkler Kreissektor im Uhrzeigersinn plus Restzeit.
pub fn cooldown_overlay(painter: &Painter, rect: Rect, remaining_frac: f32, seconds: f32) {
    let f = remaining_frac.clamp(0.0, 1.0);
    if f <= 0.0 {
        return;
    }
    let c = rect.center();
    let r = rect.width().max(rect.height());
    let mut m = Mesh::default();
    let col = Color32::from_black_alpha(170);
    m.colored_vertex(c, col);
    let steps = 48;
    let start = -PI / 2.0 + (1.0 - f) * TAU;
    for i in 0..=steps {
        let a = start + f * TAU * i as f32 / steps as f32;
        m.colored_vertex(c + Vec2::new(a.cos(), a.sin()) * r, col);
    }
    for i in 0..steps as u32 {
        m.add_triangle(0, i + 1, i + 2);
    }
    painter.with_clip_rect(rect.intersect(painter.clip_rect())).add(Shape::mesh(m));
    // Leuchtende Kante des Sektors.
    let a = start;
    painter.with_clip_rect(rect.intersect(painter.clip_rect())).line_segment([c, c + Vec2::new(a.cos(), a.sin()) * r], egui::Stroke::new(1.5, theme::EMBER.gamma_multiply(0.8)));
    let txt = if seconds >= 10.0 { format!("{seconds:.0}") } else { format!("{seconds:.1}") };
    let font = theme::heading_font(17.0);
    painter.text(c + Vec2::new(1.0, 1.5), Align2::CENTER_CENTER, &txt, font.clone(), Color32::BLACK);
    painter.text(c, Align2::CENTER_CENTER, &txt, font, theme::GOLD_BRIGHT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_slice_spans_cover_rect() {
        for len in [10.0, 47.0, 96.0, 333.0] {
            let v = spans(5.0, len, 64.0, 16.0, 16.0, PX);
            assert!((v.first().unwrap().0 - 5.0).abs() < 1e-3);
            assert!((v.last().unwrap().1 - (5.0 + len)).abs() < 1e-3);
            for w in v.windows(2) {
                assert!((w[0].1 - w[1].0).abs() < 1e-3, "lückenlos: {v:?}");
            }
        }
    }

    #[test]
    fn periodic_noise_wraps() {
        for (x, y) in [(0.3, 0.7), (2.5, 1.25), (3.9, 0.1)] {
            let a = pnoise(x, y, 4, 4, 9);
            let b = pnoise(x + 4.0, y + 8.0, 4, 4, 9);
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn textures_have_expected_shape() {
        let spec = FrameSpec { size: 80, border: 24, band: (1, 8), bronze: false, gold_edge: true, gold_line: Some(9), shadow: (11, 7, 0.6), rivets: Some((16, 4.0)), corner: Corner::Ornate, gem: ruby, seed: 2 };
        let f = frame_canvas(&spec);
        assert_eq!((f.w, f.h), (80, 80));
        // Rand deckend, Mitte transparent (dort liegt die Lederfüllung).
        assert!(f.opaque(40, 2));
        assert!(!f.opaque(40, 40));
        let p = parchment_canvas(96, 16, 7);
        assert!(p.opaque(48, 48));
        assert!(!p.opaque(0, 0));
        for s in [BtnState::Normal, BtnState::Hover, BtnState::Pressed, BtnState::Disabled] {
            let b = button_canvas(s);
            assert!(b.opaque(24, 12) && !b.opaque(0, 0));
        }
        let (mm, size) = minimap_canvas(58.0, 29.0);
        assert_eq!(size, Vec2::new(mm.w as f32, mm.h as f32));
        assert!(!mm.opaque(mm.w / 2, mm.h / 2), "Kartenfläche bleibt frei");
    }
}
