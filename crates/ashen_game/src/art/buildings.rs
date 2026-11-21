//! Isometrische Gebäude-Sprites (Bergfried, Gehöft, Kaserne, Wachturm, Hohlschrein, Knochenturm)
//! samt Metadaten für Lichter, Rauch, Feuer und Banner.

use super::palette as pal;
use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise::rand2;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use ashen_engine::assets::pixel::normals;
use glam::{Vec2, Vec3};
use std::collections::HashMap;

/// Effektpunkte relativ zum Anker (obere Ecke der Grundfläche) in Weltpixeln.
#[derive(Clone, Debug, Default)]
pub struct BuildingMeta {
    pub lights: Vec<(Vec2, Vec3, f32)>,
    pub smoke: Vec<Vec2>,
    pub fires: Vec<Vec2>,
    pub glow_green: Vec<Vec2>,
    pub height: f32,
}

/// Hilfsgeometrie für eine Grundfläche w × h Kacheln mit `oy` Pixeln Platz darüber.
struct Iso {
    w: f32,
    h: f32,
    oy: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum Face {
    Left,
    Right,
    Top,
}

impl Iso {
    fn canvas(&self) -> Canvas {
        Canvas::new(((self.w + self.h) * 16.0) as i32, ((self.w + self.h) * 8.0 + self.oy) as i32 + 1)
    }

    fn anchor(&self) -> Vec2 {
        Vec2::new(self.h * 16.0, self.oy)
    }

    /// Kachelkoordinate (relativ zur Grundfläche) + Höhe → Leinwandpixel.
    fn p(&self, tx: f32, ty: f32, z: f32) -> Vec2 {
        Vec2::new(16.0 * (tx - ty) + 16.0 * self.h, 8.0 * (tx + ty) + self.oy - z)
    }

    /// Quader von (x0,y0) bis (x1,y1) in Kacheln, von Höhe z0 bis z0+hgt.
    #[allow(clippy::too_many_arguments)]
    fn quad_box(&self, c: &mut Canvas, x0: f32, y0: f32, x1: f32, y1: f32, z0: f32, hgt: f32, tex: &dyn Fn(i32, i32, Face, f32, f32) -> Option<Rgba>) {
        let l = self.p(x0, y1, z0);
        let b = self.p(x1, y1, z0);
        let r = self.p(x1, y0, z0);
        let t = self.p(x0, y0, z0);
        let up = Vec2::new(0.0, hgt);
        // Linke Fassade (zur Kamera, links).
        let left = [l, b, b - up, l - up];
        c.polygon(&left, |x, y| {
            let base_y = l.y + (x as f32 + 0.5 - l.x) * 0.5;
            let v = base_y - (y as f32 + 0.5);
            let u = (x as f32 + 0.5 - l.x) / 2.0;
            tex(x, y, Face::Left, u, v).map(|col| (col, normals::LEFT))
        });
        let right = [b, r, r - up, b - up];
        c.polygon(&right, |x, y| {
            let base_y = b.y - (x as f32 + 0.5 - b.x) * 0.5;
            let v = base_y - (y as f32 + 0.5);
            let u = (x as f32 + 0.5 - b.x) / 2.0;
            tex(x, y, Face::Right, u, v).map(|col| (col, normals::RIGHT))
        });
        let top = [t - up, r - up, b - up, l - up];
        c.polygon(&top, |x, y| tex(x, y, Face::Top, 0.0, 0.0).map(|col| (col, normals::UP)));
    }

    /// Walmdach über einer Fläche; First entlang der längeren Achse.
    #[allow(clippy::too_many_arguments)]
    fn hip_roof(&self, c: &mut Canvas, x0: f32, y0: f32, x1: f32, y1: f32, z0: f32, rh: f32, ramp: &Ramp, overhang: f32) {
        let (x0, y0, x1, y1) = (x0 - overhang, y0 - overhang, x1 + overhang, y1 + overhang);
        let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let along_x = (x1 - x0) >= (y1 - y0);
        let half = ((x1 - x0).min(y1 - y0)) * 0.5;
        let (a, b) = if along_x { (self.p(x0 + half, cy, z0 + rh), self.p(x1 - half, cy, z0 + rh)) } else { (self.p(cx, y0 + half, z0 + rh), self.p(cx, y1 - half, z0 + rh)) };
        let l = self.p(x0, y1, z0);
        let bt = self.p(x1, y1, z0);
        let r = self.p(x1, y0, z0);
        let t = self.p(x0, y0, z0);
        let shingle = |x: i32, y: i32, base: f32, n: Vec3| {
            let row = (y as f32 * 0.5).floor() as i32;
            let seam = y % 3 == 0 || (x + row * 2) % 5 == 0;
            let tone = base + rand2(x, y, 31) * 0.12 - if seam { 0.18 } else { 0.0 };
            Some((ramp.dither(tone, x, y), n))
        };
        // Hintere Flächen zuerst, dann die zur Kamera gewandten.
        if along_x {
            c.polygon(&[t, r, b, a], |x, y| shingle(x, y, 0.5, normals::ROOF_R));
            c.polygon(&[l, t, a], |x, y| shingle(x, y, 0.4, normals::ROOF_L));
            c.polygon(&[l, bt, b, a], |x, y| shingle(x, y, 0.62, Vec3::new(-0.2, -0.45, 0.87)));
            c.polygon(&[bt, r, b], |x, y| shingle(x, y, 0.78, normals::ROOF_R));
        } else {
            c.polygon(&[t, r, a], |x, y| shingle(x, y, 0.5, normals::ROOF_R));
            c.polygon(&[l, t, a, b], |x, y| shingle(x, y, 0.4, normals::ROOF_L));
            c.polygon(&[bt, r, a, b], |x, y| shingle(x, y, 0.75, Vec3::new(0.3, -0.45, 0.84)));
            c.polygon(&[l, bt, b], |x, y| shingle(x, y, 0.55, normals::ROOF_L));
        }
        // Firstlinie.
        let steps = (a - b).length().ceil() as i32;
        for i in 0..=steps.max(1) {
            let p = a.lerp(b, i as f32 / steps.max(1) as f32);
            c.px(p.x.round() as i32, p.y.round() as i32, ramp.at(0.95));
        }
    }

    /// Kegeldach eines runden Turms.
    fn cone(&self, c: &mut Canvas, center: Vec2, radius: f32, height: f32, ramp: &Ramp) {
        let apex = center - Vec2::new(0.0, height);
        let x0 = (center.x - radius).floor() as i32;
        let x1 = (center.x + radius).ceil() as i32;
        for x in x0..=x1 {
            let dx = (x as f32 + 0.5 - center.x) / radius;
            if dx.abs() > 1.0 {
                continue;
            }
            let base_y = center.y + (1.0 - dx * dx).sqrt() * radius * 0.5;
            let top_y = apex.y + (base_y - apex.y) * dx.abs();
            for y in top_y as i32..=base_y as i32 {
                let t = 0.7 - dx * 0.3 + if y % 3 == 0 { -0.1 } else { 0.0 };
                c.pxn(x, y, ramp.dither(t, x, y), Vec3::new(dx * 0.7, -0.5, 0.6));
            }
        }
    }

    /// Runder Turm (Zylinder).
    fn cylinder(&self, c: &mut Canvas, center: Vec2, radius: f32, height: f32, ramp: &Ramp) {
        let x0 = (center.x - radius).floor() as i32;
        let x1 = (center.x + radius).ceil() as i32;
        for x in x0..=x1 {
            let dx = (x as f32 + 0.5 - center.x) / radius;
            if dx.abs() > 1.0 {
                continue;
            }
            let curve = (1.0 - dx * dx).sqrt() * radius * 0.5;
            let bottom = center.y + curve;
            let top = center.y - height + curve;
            for y in top as i32..=bottom as i32 {
                let v = bottom - y as f32;
                let brick = (v as i32 % 4 == 0) || ((x + (v as i32 / 4) * 3) % 6 == 0);
                let t = 0.62 - dx * 0.35 + rand2(x, y, 5) * 0.08 - if brick { 0.15 } else { 0.0 };
                c.pxn(x, y, ramp.dither(t, x, y), Vec3::new(dx, 0.0, (1.0 - dx * dx).max(0.0).sqrt()));
            }
        }
    }
}

fn stone_wall(ramp: Ramp) -> impl Fn(i32, i32, Face, f32, f32) -> Option<Rgba> {
    move |x, y, face, u, v| {
        let shade = match face {
            Face::Left => 0.42,
            Face::Right => 0.62,
            Face::Top => 0.78,
        };
        if face == Face::Top {
            return Some(ramp.dither(shade + rand2(x, y, 3) * 0.08, x, y));
        }
        let row = (v / 3.0).floor() as i32;
        let joint_v = v - row as f32 * 3.0 < 0.9;
        let joint_u = ((u + if row % 2 == 0 { 0.0 } else { 2.5 }) / 5.0).fract() < 0.18;
        let stone_tone = rand2((u / 5.0) as i32, row, 11) * 0.14;
        let t = shade + stone_tone - if joint_v || joint_u { 0.2 } else { 0.0 } + (v * 0.004);
        Some(ramp.dither(t, x, y))
    }
}

fn timber_wall() -> impl Fn(i32, i32, Face, f32, f32) -> Option<Rgba> {
    move |x, y, face, u, v| {
        let shade = match face {
            Face::Left => 0.35,
            Face::Right => 0.6,
            Face::Top => 0.7,
        };
        let beam = (u % 7.0) < 1.0 || (v % 9.0) < 1.0 || ((u + v) % 14.0 < 1.0 && (v % 9.0) > 1.0);
        if beam {
            Some(pal::dark_wood().dither(shade, x, y))
        } else {
            Some(pal::plaster().dither(shade + rand2(x, y, 2) * 0.1, x, y))
        }
    }
}

fn plank_wall() -> impl Fn(i32, i32, Face, f32, f32) -> Option<Rgba> {
    move |x, y, face, u, _v| {
        let shade = match face {
            Face::Left => 0.35,
            Face::Right => 0.6,
            Face::Top => 0.72,
        };
        let seam = (u % 3.0) < 0.7;
        Some(pal::wood().dither(shade - if seam { 0.15 } else { 0.0 } + rand2(x, y, 8) * 0.08, x, y))
    }
}

fn window(c: &mut Canvas, x: i32, y: i32, w: i32, h: i32, lit: bool) {
    for dy in 0..h {
        for dx in 0..w {
            let col = if lit { pal::WINDOW_WARM.scale(0.75 + 0.25 * (1.0 - dy as f32 / h as f32)) } else { Rgba::hex(0x0d0b0a) };
            c.px(x + dx, y + dy, col);
            if lit {
                c.glow(x + dx, y + dy, 0.85);
            }
        }
    }
}

fn banner(c: &mut Canvas, x: i32, y: i32, len: i32, wave: i32) {
    // Stange
    for i in -2..len + 1 {
        c.px(x, y + i, pal::dark_wood().at(0.5));
    }
    for i in 0..len {
        let off = ((i + wave) as f32 * 0.7).sin().round() as i32;
        for dx in 1..6 {
            let t = 0.55 - dx as f32 * 0.04 + if i % 4 == 0 { -0.08 } else { 0.0 };
            c.pxn(x + dx + off * (dx / 3), y + i, pal::crimson().dither(t, x + dx, y + i), Vec3::new(0.0, 0.0, 1.0));
        }
    }
    // Goldene Flamme als Wappen.
    let cy = y + len / 2;
    for (dx, dy) in [(3, -1), (3, 0), (2, 0), (4, 0), (3, 1), (2, 1), (4, 1)] {
        c.px(x + dx, cy + dy, pal::gold().at(0.75));
    }
    for dx in 1..6 {
        c.px(x + dx, y + len, pal::gold().at(0.5));
    }
}

fn finish(mut c: Canvas) -> Canvas {
    c.outline(pal::OUTLINE);
    c
}

fn crenellate(c: &mut Canvas, iso: &Iso, a: (f32, f32), b: (f32, f32), z: f32, wall: &dyn Fn(i32, i32, Face, f32, f32) -> Option<Rgba>) {
    let n = (((b.0 - a.0).abs() + (b.1 - a.1).abs()) / 0.22).round().max(1.0) as i32;
    for i in (0..=n).step_by(2) {
        let t = i as f32 / n as f32;
        let x = a.0 + (b.0 - a.0) * t;
        let y = a.1 + (b.1 - a.1) * t;
        iso.quad_box(c, x - 0.07, y - 0.07, x + 0.07, y + 0.07, z, 4.0, wall);
    }
}

fn round_tower(c: &mut Canvas, iso: &Iso, tx: f32, ty: f32, z: f32, radius: f32, height: f32) {
    let base = iso.p(tx, ty, z);
    iso.cylinder(c, base, radius, height, &pal::stone());
    iso.cone(c, base - Vec2::new(0.0, height), radius + 1.5, radius * 2.2, &pal::roof_red());
}

fn keep(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 3.0, h: 3.0, oy: 100.0 };
    let mut c = iso.canvas();
    let wall = stone_wall(pal::stone());
    let (lo, hi) = (0.2, 2.8);
    let wall_h = 16.0;
    let t = 0.22;
    // Sockel.
    iso.quad_box(&mut c, 0.05, 0.05, 2.95, 2.95, 0.0, 3.0, &stone_wall(pal::rock()));
    // Hinterer Turm (obere Ecke) und hintere Mauern.
    round_tower(&mut c, &iso, lo, lo, 3.0, 7.0, 30.0);
    iso.quad_box(&mut c, lo, lo, hi, lo + t, 3.0, wall_h, &wall);
    iso.quad_box(&mut c, lo, lo, lo + t, hi, 3.0, wall_h, &wall);
    crenellate(&mut c, &iso, (lo + 0.1, lo + 0.1), (hi, lo + 0.1), 3.0 + wall_h, &wall);
    crenellate(&mut c, &iso, (lo + 0.1, lo + 0.1), (lo + 0.1, hi), 3.0 + wall_h, &wall);
    // Seitliche Ecktürme.
    round_tower(&mut c, &iso, hi, lo, 3.0, 7.0, 30.0);
    round_tower(&mut c, &iso, lo, hi, 3.0, 7.0, 30.0);
    // Donjon im Hof.
    iso.quad_box(&mut c, 0.85, 0.85, 1.95, 1.95, 3.0, 52.0, &wall);
    for k in 0..5 {
        let f = 0.85 + k as f32 * 0.275;
        iso.quad_box(&mut c, f - 0.06, 1.89, f + 0.06, 2.01, 55.0, 4.0, &wall);
        iso.quad_box(&mut c, 1.89, f - 0.06, 2.01, f + 0.06, 55.0, 4.0, &wall);
    }
    iso.hip_roof(&mut c, 1.0, 1.0, 1.8, 1.8, 55.0, 22.0, &pal::roof_red(), 0.0);
    let lf = iso.p(0.85, 1.95, 3.0);
    let rf = iso.p(1.95, 1.95, 3.0);
    window(&mut c, (lf.x + 7.0) as i32, (lf.y - 30.0) as i32, 2, 4, true);
    window(&mut c, (lf.x + 13.0) as i32, (lf.y - 40.0) as i32, 2, 4, false);
    window(&mut c, (rf.x + 8.0) as i32, (rf.y - 41.0) as i32, 2, 4, true);
    window(&mut c, (rf.x + 14.0) as i32, (rf.y - 30.0) as i32, 2, 4, false);
    banner(&mut c, (rf.x - 3.0) as i32, (rf.y - 50.0) as i32, 16, 1);
    // Vordere Mauern (über dem Hof).
    iso.quad_box(&mut c, lo, hi - t, hi, hi, 3.0, wall_h, &wall);
    iso.quad_box(&mut c, hi - t, lo, hi, hi, 3.0, wall_h, &wall);
    crenellate(&mut c, &iso, (lo + 0.1, hi - 0.1), (hi - 0.1, hi - 0.1), 3.0 + wall_h, &wall);
    crenellate(&mut c, &iso, (hi - 0.1, lo + 0.1), (hi - 0.1, hi - 0.1), 3.0 + wall_h, &wall);
    // Torhaus in der rechten Frontmauer.
    let gate = iso.p(hi, 1.5, 3.0);
    for dy in 0..13 {
        for dx in -4i32..5 {
            let arch = dy > 10 && dx.abs() > 2 || dy > 11 && dx.abs() > 1;
            if !arch {
                let col = if dx.abs() == 4 { pal::stone().at(0.8) } else if dy % 3 == 0 || dx % 2 == 0 { pal::iron_dark().at(0.5) } else { pal::dark_wood().at(0.3) };
                c.px(gate.x as i32 + dx, gate.y as i32 - dy + dx / 2 - 1, col);
            }
        }
    }
    // Vorderer Turm (untere Ecke) zuletzt.
    round_tower(&mut c, &iso, hi, hi, 3.0, 7.5, 32.0);
    let torch_a = gate + Vec2::new(-7.0, -13.0);
    let torch_b = gate + Vec2::new(7.0, -9.0);
    for t in [torch_a, torch_b] {
        c.px(t.x as i32, t.y as i32, pal::EMBER_HOT);
        c.glow(t.x as i32, t.y as i32, 1.0);
        c.px(t.x as i32, t.y as i32 - 1, pal::EMBER);
        c.glow(t.x as i32, t.y as i32 - 1, 1.0);
        c.px(t.x as i32, t.y as i32 + 1, pal::dark_wood().at(0.3));
    }
    let anchor = iso.anchor();
    let rel = |p: Vec2| p - anchor;
    meta.insert(
        "keep".into(),
        BuildingMeta {
            lights: vec![
                (rel(torch_a), Vec3::new(1.0, 0.6, 0.25), 70.0),
                (rel(torch_b), Vec3::new(1.0, 0.6, 0.25), 70.0),
                (rel(Vec2::new(lf.x + 8.0, lf.y - 28.0)), Vec3::new(1.0, 0.7, 0.35), 40.0),
                (rel(Vec2::new(rf.x + 9.0, rf.y - 39.0)), Vec3::new(1.0, 0.7, 0.35), 40.0),
            ],
            smoke: vec![rel(iso.p(1.2, 1.2, 70.0))],
            fires: vec![rel(torch_a), rel(torch_b)],
            glow_green: vec![],
            height: 96.0,
        },
    );
    (finish(c), anchor)
}

fn farmstead(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 2.0, h: 2.0, oy: 40.0 };
    let mut c = iso.canvas();
    // Feld mit Getreidereihen auf der ganzen Grundfläche.
    for y in 0..c.h {
        for x in 0..c.w {
            let wx = x as f32 + 0.5 - 32.0;
            let wy = y as f32 + 0.5 - 40.0;
            let tx = (wx / 16.0 + wy / 8.0) * 0.5;
            let ty = (wy / 8.0 - wx / 16.0) * 0.5;
            if !(0.02..1.98).contains(&tx) || !(0.02..1.98).contains(&ty) {
                continue;
            }
            let furrow = ((tx * 6.0).fract()) < 0.35;
            let col = if furrow { pal::dirt().dither(0.35, x, y) } else if rand2(x, y, 4) > 0.3 { pal::thatch().dither(0.55 + rand2(x, y, 9) * 0.3, x, y) } else { pal::grass().dither(0.6, x, y) };
            c.pxn(x, y, col, if furrow { Vec3::Z } else { Vec3::new(0.0, -0.5, 0.85) });
        }
    }
    // Haus.
    iso.quad_box(&mut c, 0.15, 0.1, 1.15, 0.95, 0.0, 15.0, &timber_wall());
    iso.hip_roof(&mut c, 0.15, 0.1, 1.15, 0.95, 15.0, 13.0, &pal::thatch(), 0.12);
    // Schornstein.
    let ch = iso.p(0.45, 0.35, 22.0);
    iso.quad_box(&mut c, 0.38, 0.28, 0.52, 0.42, 22.0, 9.0, &stone_wall(pal::rock()));
    // Tür und Fenster.
    let fl = iso.p(0.15, 0.95, 0.0);
    let fr = iso.p(1.15, 0.95, 0.0);
    for dy in 0..7 {
        for dx in 0..3 {
            c.px(fr.x as i32 + 4 + dx, fr.y as i32 - 2 - dy - (dx + 4) / 2, pal::dark_wood().at(0.35));
        }
    }
    window(&mut c, (fl.x + 6.0) as i32, (fl.y - 9.0) as i32, 2, 3, true);
    window(&mut c, (fr.x + 10.0) as i32, (fr.y - 13.0) as i32, 2, 3, true);
    // Zaun an der Vorderkante.
    for i in 0..9 {
        let p = iso.p(1.95, 0.2 + i as f32 * 0.2, 0.0);
        for dy in 0..4 {
            c.px(p.x as i32, p.y as i32 - dy, pal::wood().at(0.45));
        }
        c.px(p.x as i32 - 1, p.y as i32 - 2, pal::wood().at(0.6));
        c.px(p.x as i32 + 1, p.y as i32 - 2, pal::wood().at(0.6));
    }
    // Heuballen.
    let hay = iso.p(1.6, 1.6, 0.0);
    c.ellipse_shaded(hay.x, hay.y - 3.0, 4.0, 3.0, &pal::thatch(), 0.1);
    let anchor = iso.anchor();
    meta.insert(
        "farmstead".into(),
        BuildingMeta {
            lights: vec![(Vec2::new(fl.x + 7.0, fl.y - 8.0) - anchor, Vec3::new(1.0, 0.65, 0.3), 30.0)],
            smoke: vec![ch - Vec2::new(0.0, 10.0) - anchor],
            height: 40.0,
            ..Default::default()
        },
    );
    (finish(c), anchor)
}

fn barracks(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 3.0, h: 2.0, oy: 54.0 };
    let mut c = iso.canvas();
    iso.quad_box(&mut c, 0.1, 0.1, 2.9, 1.9, 0.0, 4.0, &stone_wall(pal::rock()));
    iso.quad_box(&mut c, 0.25, 0.3, 2.65, 1.6, 4.0, 8.0, &stone_wall(pal::stone()));
    iso.quad_box(&mut c, 0.25, 0.3, 2.65, 1.6, 12.0, 12.0, &timber_wall());
    iso.hip_roof(&mut c, 0.25, 0.3, 2.65, 1.6, 24.0, 18.0, &pal::roof_red(), 0.12);
    let fl = iso.p(0.25, 1.6, 4.0);
    let fr = iso.p(2.65, 1.6, 4.0);
    // Tor auf der linken (langen) Fassade.
    let door = fl + Vec2::new(26.0, 13.0);
    for dy in 0..11 {
        for dx in 0..8 {
            c.px(door.x as i32 + dx, door.y as i32 - dy + dx / 2, if dx == 0 || dx == 7 { pal::dark_wood().at(0.15) } else { pal::dark_wood().at(0.35) });
        }
    }
    for i in 0..4 {
        window(&mut c, (fl.x + 8.0 + i as f32 * 12.0) as i32, (fl.y - 15.0 + i as f32 * 6.0) as i32, 2, 3, i % 2 == 1);
    }
    window(&mut c, (fr.x + 10.0) as i32, (fr.y - 18.0) as i32, 2, 3, true);
    // Waffenständer mit Speeren.
    let rack = iso.p(1.0, 1.85, 0.0);
    for i in 0..4 {
        let x = rack.x as i32 + i * 2;
        for dy in 0..12 {
            c.px(x, rack.y as i32 - dy, pal::wood().at(0.5));
        }
        c.px(x, rack.y as i32 - 12, pal::steel().at(0.9));
    }
    for dx in -1..9 {
        c.px(rack.x as i32 + dx, rack.y as i32 - 4, pal::dark_wood().at(0.4));
    }
    // Übungspuppe.
    let dummy = iso.p(2.6, 1.85, 0.0);
    for dy in 0..10 {
        c.px(dummy.x as i32, dummy.y as i32 - dy, pal::wood().at(0.45));
    }
    c.ellipse_shaded(dummy.x, dummy.y - 9.0, 3.0, 3.5, &pal::thatch(), 0.0);
    for dx in -4..5 {
        c.px(dummy.x as i32 + dx, dummy.y as i32 - 7, pal::wood().at(0.55));
    }
    banner(&mut c, (fr.x - 4.0) as i32, (fr.y - 40.0) as i32, 12, 2);
    let anchor = iso.anchor();
    meta.insert(
        "barracks".into(),
        BuildingMeta {
            lights: vec![(door + Vec2::new(4.0, -12.0) - anchor, Vec3::new(1.0, 0.6, 0.28), 52.0)],
            fires: vec![door + Vec2::new(-2.0, -10.0) - anchor],
            smoke: vec![iso.p(2.2, 0.8, 40.0) - anchor],
            height: 54.0,
            ..Default::default()
        },
    );
    let t = door + Vec2::new(-2.0, -10.0);
    c.px(t.x as i32, t.y as i32, pal::EMBER_HOT);
    c.glow(t.x as i32, t.y as i32, 1.0);
    (finish(c), anchor)
}

fn watchtower(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 1.0, h: 1.0, oy: 66.0 };
    let mut c = iso.canvas();
    iso.quad_box(&mut c, 0.1, 0.1, 0.9, 0.9, 0.0, 6.0, &stone_wall(pal::rock()));
    // Pfosten.
    for (tx, ty) in [(0.2, 0.2), (0.8, 0.2), (0.2, 0.8), (0.8, 0.8)] {
        iso.quad_box(&mut c, tx - 0.05, ty - 0.05, tx + 0.05, ty + 0.05, 6.0, 38.0, &plank_wall());
    }
    // Kreuzstreben.
    for k in 0..3 {
        let z = 10.0 + k as f32 * 11.0;
        let a = iso.p(0.2, 0.8, z);
        let b = iso.p(0.8, 0.8, z + 9.0);
        c.line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, pal::wood().at(0.35));
        let a = iso.p(0.8, 0.8, z);
        let b = iso.p(0.8, 0.2, z + 9.0);
        c.line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, pal::wood().at(0.55));
    }
    // Plattform mit Brüstung.
    iso.quad_box(&mut c, 0.0, 0.0, 1.0, 1.0, 44.0, 3.0, &plank_wall());
    iso.quad_box(&mut c, 0.0, 0.85, 1.0, 1.0, 47.0, 5.0, &plank_wall());
    iso.quad_box(&mut c, 0.85, 0.0, 1.0, 0.85, 47.0, 5.0, &plank_wall());
    // Feuerbecken.
    let fire = iso.p(0.5, 0.5, 49.0);
    for dx in -2..=2 {
        c.px(fire.x as i32 + dx, fire.y as i32, pal::iron_dark().at(0.6));
    }
    for (dx, dy, col) in [(0, -1, pal::EMBER), (-1, -1, pal::FIRE_DEEP), (1, -1, pal::FIRE_DEEP), (0, -2, pal::EMBER_HOT), (0, -3, pal::EMBER)] {
        c.px(fire.x as i32 + dx, fire.y as i32 + dy, col);
        c.glow(fire.x as i32 + dx, fire.y as i32 + dy, 1.0);
    }
    let anchor = iso.anchor();
    meta.insert(
        "watchtower".into(),
        BuildingMeta { lights: vec![(fire - anchor + Vec2::new(0.0, -2.0), Vec3::new(1.0, 0.55, 0.2), 95.0)], fires: vec![fire - anchor + Vec2::new(0.0, -2.0)], height: 66.0, ..Default::default() },
    );
    (finish(c), anchor)
}

fn hollow_shrine(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 3.0, h: 3.0, oy: 92.0 };
    let mut c = iso.canvas();
    let ruin = pal::ash();
    let wall = stone_wall(Ramp::from_hex(&[0x151416, 0x1f1d20, 0x2a272a, 0x363236, 0x433e42, 0x524c50]));
    iso.quad_box(&mut c, 0.1, 0.1, 2.9, 2.9, 0.0, 4.0, &wall);
    // Gebrochene Kapellenmauern an den hinteren Kanten.
    for i in 0..6 {
        let f = 0.2 + i as f32 * 0.45;
        let hgt = [34.0, 22.0, 40.0, 12.0, 28.0, 18.0][i];
        iso.quad_box(&mut c, f, 0.15, f + 0.4, 0.4, 4.0, hgt, &wall);
        let hgt2 = [16.0, 30.0, 10.0, 36.0, 20.0, 26.0][i];
        iso.quad_box(&mut c, 0.15, f, 0.4, f + 0.4, 4.0, hgt2, &wall);
    }
    // Spitzbogenfragment.
    let arch = iso.p(0.3, 0.3, 40.0);
    for i in 0..10 {
        let a = i as f32 / 9.0 * std::f32::consts::PI;
        let p = arch + Vec2::new(a.cos() * 9.0 + 9.0, -a.sin() * 8.0);
        c.px(p.x as i32, p.y as i32, ruin.at(0.6));
        c.px(p.x as i32, p.y as i32 + 1, ruin.at(0.4));
    }
    // Obelisk mit Riss im Schleier.
    iso.quad_box(&mut c, 1.2, 1.2, 1.8, 1.8, 4.0, 54.0, &wall);
    iso.hip_roof(&mut c, 1.2, 1.2, 1.8, 1.8, 58.0, 12.0, &Ramp::from_hex(&[0x121113, 0x1d1b1e, 0x29262a, 0x35313a]), 0.0);
    let base = iso.p(1.8, 1.8, 4.0);
    let rift_x = base.x as i32 - 1;
    let mut glow_pts = Vec::new();
    for dy in 6..48 {
        let wob = ((dy as f32 * 0.5).sin() * 1.5).round() as i32;
        let w = if dy > 14 && dy < 40 { 2 } else { 1 };
        for dx in 0..w {
            let x = rift_x + wob + dx;
            let y = base.y as i32 - dy;
            c.px(x, y, if dx == 0 { pal::HOLLOW_GLOW } else { pal::HOLLOW_DEEP });
            c.glow(x, y, 1.0);
        }
        if dy % 10 == 0 {
            glow_pts.push(Vec2::new(rift_x as f32, base.y - dy as f32));
        }
    }
    // Runen an der linken Fassade.
    let lf = iso.p(1.2, 1.8, 4.0);
    for k in 0..5 {
        let p = lf + Vec2::new(4.0 + k as f32 * 2.0, -10.0 - k as f32 * 7.0);
        c.px(p.x as i32, p.y as i32, pal::HOLLOW_DEEP);
        c.glow(p.x as i32, p.y as i32, 0.7);
    }
    // Knochen und Kerzen auf dem Boden.
    for i in 0..18 {
        let tx = 0.5 + rand2(i, 1, 77) * 2.2;
        let ty = 0.5 + rand2(i, 2, 77) * 2.2;
        if (1.1..1.9).contains(&tx) && (1.1..1.9).contains(&ty) {
            continue;
        }
        let p = iso.p(tx, ty, 4.0);
        if i % 3 == 0 {
            c.px(p.x as i32, p.y as i32, Rgba::hex(0xe8d8a0));
            c.px(p.x as i32, p.y as i32 - 1, pal::EMBER_HOT);
            c.glow(p.x as i32, p.y as i32 - 1, 1.0);
        } else {
            c.px(p.x as i32, p.y as i32, pal::bone().at(0.7));
            c.px(p.x as i32 + 1, p.y as i32, pal::bone().at(0.55));
        }
    }
    let anchor = iso.anchor();
    meta.insert(
        "hollow_shrine".into(),
        BuildingMeta {
            lights: vec![(Vec2::new(rift_x as f32, base.y - 26.0) - anchor, Vec3::new(0.45, 1.0, 0.6), 130.0)],
            glow_green: glow_pts.iter().map(|p| *p - anchor).collect(),
            height: 92.0,
            ..Default::default()
        },
    );
    (finish(c), anchor)
}

fn bone_spire(meta: &mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2) {
    let iso = Iso { w: 1.0, h: 1.0, oy: 66.0 };
    let mut c = iso.canvas();
    let bone = Ramp::from_hex(&[0x1f1d18, 0x34312a, 0x4c4839, 0x67614c, 0x837b60]);
    let base = iso.p(0.5, 0.5, 0.0);
    for i in 0..46 {
        let y = base.y as i32 - i;
        let r = (7.0 - i as f32 * 0.14).max(1.0);
        for dx in -(r as i32)..=(r as i32) {
            let seg = i % 7 < 2;
            let t = 0.65 - dx as f32 / r * 0.3 - if seg { 0.2 } else { 0.0 };
            c.pxn(base.x as i32 + dx, y, bone.dither(t, base.x as i32 + dx, y), Vec3::new(dx as f32 / r, -0.1, 0.8));
        }
        // Rippen-Stacheln.
        if i % 9 == 4 && i < 40 {
            for k in 1..5 {
                c.px(base.x as i32 + r as i32 + k, y - k / 2, bone.at(0.7));
                c.px(base.x as i32 - r as i32 - k, y - k / 2, bone.at(0.5));
            }
        }
    }
    let orb = Vec2::new(base.x, base.y - 49.0);
    for dy in -2..=2 {
        for dx in -2..=2 {
            if dx * dx + dy * dy <= 5 {
                c.px(orb.x as i32 + dx, orb.y as i32 + dy, if dx * dx + dy * dy <= 1 { pal::HOLLOW_GLOW } else { pal::HOLLOW_DEEP });
                c.glow(orb.x as i32 + dx, orb.y as i32 + dy, 1.0);
            }
        }
    }
    let anchor = iso.anchor();
    meta.insert(
        "bone_spire".into(),
        BuildingMeta { lights: vec![(orb - anchor, Vec3::new(0.4, 1.0, 0.55), 70.0)], glow_green: vec![orb - anchor], height: 66.0, ..Default::default() },
    );
    (finish(c), anchor)
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

pub fn build(atlas: &mut AtlasBuilder) -> HashMap<String, BuildingMeta> {
    let mut meta = HashMap::new();
    type Painter = fn(&mut HashMap<String, BuildingMeta>) -> (Canvas, Vec2);
    let list: [(&str, Painter); 6] = [("keep", keep), ("farmstead", farmstead), ("barracks", barracks), ("watchtower", watchtower), ("hollow_shrine", hollow_shrine), ("bone_spire", bone_spire)];
    for (name, f) in list {
        let (c, a) = f(&mut meta);
        atlas.add(format!("bld_{name}"), c, a);
    }
    for (w, h) in [(1.0, 1.0), (2.0, 2.0), (3.0, 2.0), (3.0, 3.0)] {
        let (c, a) = scaffold(w, h);
        atlas.add(format!("scaffold_{}x{}", w as i32, h as i32), c, a);
    }
    // Banner-Animation (drei Phasen) für zusätzliche Fahnen.
    for i in 0..3 {
        let mut c = Canvas::new(10, 20);
        banner(&mut c, 1, 2, 14, i * 2);
        atlas.add(format!("banner_{i}"), finish(c), Vec2::new(1.0, 18.0));
    }
    meta
}
