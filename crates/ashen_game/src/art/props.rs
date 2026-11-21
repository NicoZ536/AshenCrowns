//! Bäume, Ressourcenquellen und Dekorationsobjekte.

use super::palette as pal;
use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise::{fbm2, rand2};
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::{Vec2, Vec3};

fn finish(mut c: Canvas) -> Canvas {
    c.outline(pal::OUTLINE);
    c.pillow_normals(1.0);
    c
}

/// Laubklumpen mit Licht von links oben und dunklen Lücken.
fn foliage_blob(c: &mut Canvas, cx: f32, cy: f32, rx: f32, ry: f32, ramp: &Ramp, seed: u32) {
    let l = Vec3::new(-0.6, -0.7, 0.4).normalize();
    for y in (cy - ry) as i32..=(cy + ry) as i32 {
        for x in (cx - rx) as i32..=(cx + rx) as i32 {
            let dx = (x as f32 + 0.5 - cx) / rx;
            let dy = (y as f32 + 0.5 - cy) / ry;
            let edge_noise = fbm2(x as f32 * 0.4, y as f32 * 0.4, seed, 2) * 0.45;
            let d = dx * dx + dy * dy;
            if d > 1.0 - edge_noise * 0.6 + 0.2 {
                continue;
            }
            let n = Vec3::new(dx, dy, (1.0 - d.min(1.0)).sqrt());
            let leaf = rand2(x, y, seed ^ 3);
            let t = (n.dot(l) * 0.5 + 0.45 + (leaf - 0.5) * 0.25).clamp(0.0, 1.0);
            c.pxn(x, y, ramp.dither(t, x, y), n);
        }
    }
}

fn pine(seed: u32) -> Canvas {
    let (w, h) = (26, 46);
    let mut c = Canvas::new(w, h);
    let cx = 13.0;
    let base = h as f32 - 3.0;
    for y in (base as i32 - 7)..=(base as i32) {
        c.px(12, y, pal::bark().at(0.4));
        c.px(13, y, pal::bark().at(0.25));
    }
    let tiers = 5;
    for i in 0..tiers {
        let ty = base - 8.0 - i as f32 * 6.5;
        let r = 11.0 - i as f32 * 1.9 + rand2(i, 0, seed) * 1.5;
        let pts = [Vec2::new(cx - r, ty + 2.0), Vec2::new(cx + r, ty + 2.0), Vec2::new(cx + 0.5, ty - 9.0)];
        let ramp = pal::pine();
        c.polygon(&pts, |x, y| {
            let fx = (x as f32 + 0.5 - cx) / r;
            let droop = ((y as f32 - ty) + 2.0) / 11.0;
            let t = 0.55 - fx * 0.3 + (1.0 - droop) * 0.15 + (rand2(x, y, seed) - 0.5) * 0.25;
            // Gezackte Unterkante.
            if y as f32 > ty + 0.5 && rand2(x, 0, seed + i as u32) > 0.6 {
                return None;
            }
            Some((ramp.dither(t, x, y), Vec3::new(fx * 0.6, -0.4, 0.7)))
        });
    }
    finish(c)
}

fn oak(seed: u32) -> Canvas {
    let (w, h) = (32, 40);
    let mut c = Canvas::new(w, h);
    let base = h - 3;
    for y in base - 12..=base {
        for x in 14..18 {
            c.px(x, y, pal::bark().at(0.25 + (x - 14) as f32 * 0.08));
        }
    }
    c.line(16, base - 10, 22, base - 16, pal::bark().at(0.35));
    c.line(15, base - 9, 9, base - 15, pal::bark().at(0.3));
    let ramp = pal::oak();
    foliage_blob(&mut c, 10.0, 18.0, 8.0, 7.0, &ramp, seed);
    foliage_blob(&mut c, 22.0, 17.0, 8.0, 7.5, &ramp, seed + 1);
    foliage_blob(&mut c, 16.0, 11.0, 9.0, 8.0, &ramp, seed + 2);
    foliage_blob(&mut c, 16.0, 21.0, 9.0, 6.0, &ramp, seed + 3);
    finish(c)
}

fn dead_tree(seed: u32) -> Canvas {
    let (w, h) = (28, 40);
    let mut c = Canvas::new(w, h);
    let base = h - 3;
    let bark = Ramp::from_hex(&[0x121012, 0x1d1a1c, 0x2a2628, 0x383235]);
    for y in base - 18..=base {
        let wob = ((y as f32 * 0.4).sin() * 0.8).round() as i32;
        c.px(13 + wob, y, bark.at(0.5));
        c.px(14 + wob, y, bark.at(0.3));
    }
    let branches = [(14, base - 16, 22, base - 26), (13, base - 12, 5, base - 20), (14, base - 18, 11, base - 30), (13, base - 8, 20, base - 13)];
    for (i, (x0, y0, x1, y1)) in branches.iter().enumerate() {
        c.line(*x0, *y0, *x1, *y1, bark.at(0.45));
        let (tx, ty) = (*x1 + if i % 2 == 0 { 2 } else { -2 }, *y1 - 3);
        c.line(*x1, *y1, tx, ty, bark.at(0.6));
    }
    let _ = seed;
    finish(c)
}

fn gold_mine() -> Canvas {
    let (w, h) = (66, 50);
    let mut c = Canvas::new(w, h);
    let rock = pal::rock();
    // Felshügel.
    c.ellipse_shaded(33.0, 32.0, 28.0, 14.0, &rock, 0.0);
    c.ellipse_shaded(22.0, 26.0, 14.0, 10.0, &rock, 0.05);
    c.ellipse_shaded(44.0, 25.0, 13.0, 11.0, &rock, 0.08);
    // Goldadern.
    for i in 0..26 {
        let x = 10 + (rand2(i, 3, 41) * 46.0) as i32;
        let y = 18 + (rand2(i, 4, 41) * 24.0) as i32;
        if c.opaque(x, y) {
            c.px(x, y, pal::gold().at(0.85));
            c.glow(x, y, 0.35);
            c.px(x + 1, y, pal::gold().at(0.6));
        }
    }
    // Stolleneingang mit Holzrahmen.
    for y in 26..40 {
        for x in 28..38 {
            c.px(x, y, Rgba::hex(0x080706));
        }
    }
    for y in 24..41 {
        c.px(27, y, pal::wood().at(0.55));
        c.px(38, y, pal::wood().at(0.4));
    }
    for x in 26..40 {
        c.px(x, 24, pal::wood().at(0.6));
        c.px(x, 25, pal::wood().at(0.4));
    }
    // Lore auf Schienen.
    for x in 40..54 {
        c.px(x, 44 - (x - 40) / 3, pal::iron_dark().at(0.6));
    }
    for y in 36..41 {
        for x in 43..50 {
            c.px(x, y, pal::wood().at(0.35 + (x - 43) as f32 * 0.04));
        }
    }
    for x in 43..50 {
        c.px(x, 36, pal::gold().at(0.8));
    }
    c.px(44, 41, Rgba::hex(0x101010));
    c.px(49, 41, Rgba::hex(0x101010));
    // Laterne am Eingang.
    c.px(39, 22, pal::WINDOW_WARM);
    c.glow(39, 22, 1.0);
    finish(c)
}

fn quarry() -> Canvas {
    let (w, h) = (64, 44);
    let mut c = Canvas::new(w, h);
    let stone = pal::stone();
    let blocks = [(14.0, 30.0, 10.0, 7.0), (30.0, 26.0, 13.0, 10.0), (46.0, 30.0, 10.0, 8.0), (26.0, 34.0, 9.0, 6.0), (40.0, 36.0, 8.0, 5.0)];
    for (i, (x, y, rx, ry)) in blocks.iter().enumerate() {
        c.ellipse_shaded(*x, *y, *rx, *ry, &stone, 0.05 * i as f32);
    }
    // Geschnittene Quader.
    for (bx, by) in [(8, 36), (50, 38)] {
        for y in 0..5 {
            for x in 0..7 {
                let top = y < 2;
                c.pxn(bx + x - y / 2, by + y, stone.at(if top { 0.75 } else { 0.5 - x as f32 * 0.03 }), if top { Vec3::new(0.0, -0.8, 0.6) } else { Vec3::new(0.6, 0.2, 0.75) });
            }
        }
    }
    // Werkzeug.
    c.line(20, 22, 26, 16, pal::wood().at(0.5));
    c.px(26, 16, pal::steel().at(0.8));
    c.px(27, 16, pal::steel().at(0.6));
    finish(c)
}

fn small_rock(seed: u32) -> Canvas {
    let mut c = Canvas::new(10, 7);
    c.ellipse_shaded(5.0, 4.0, 4.0 + rand2(1, 1, seed) * 1.0, 2.6, &pal::rock(), 0.0);
    finish(c)
}

fn boulder(seed: u32) -> Canvas {
    let mut c = Canvas::new(26, 20);
    c.ellipse_shaded(13.0, 12.0, 11.0, 7.0, &pal::rock(), 0.0);
    c.ellipse_shaded(9.0 + rand2(1, 2, seed) * 3.0, 9.0, 6.0, 5.0, &pal::rock(), 0.08);
    // Moos.
    for i in 0..12 {
        let x = 4 + (rand2(i, 5, seed) * 16.0) as i32;
        let y = 6 + (rand2(i, 6, seed) * 4.0) as i32;
        if c.opaque(x, y) {
            c.px(x, y, pal::grass().at(0.5));
        }
    }
    finish(c)
}

fn bush(seed: u32) -> Canvas {
    let mut c = Canvas::new(16, 12);
    foliage_blob(&mut c, 8.0, 7.0, 7.0, 4.5, &pal::oak(), seed);
    if seed.is_multiple_of(3) {
        for i in 0..4 {
            let x = 3 + (rand2(i, 7, seed) * 10.0) as i32;
            let y = 4 + (rand2(i, 8, seed) * 4.0) as i32;
            if c.opaque(x, y) {
                c.px(x, y, Rgba::hex(0x8a2a2a));
            }
        }
    }
    finish(c)
}

fn grass_tuft(seed: u32) -> Canvas {
    let mut c = Canvas::new(8, 6);
    let ramp = pal::grass();
    for i in 0..5 {
        let x = 1 + i + (rand2(i, 0, seed) * 1.5) as i32;
        let hgt = 2 + (rand2(i, 1, seed) * 3.0) as i32;
        for k in 0..hgt {
            c.px(x + if k == hgt - 1 && i % 2 == 0 { 1 } else { 0 }, 5 - k, ramp.at(0.45 + k as f32 * 0.1));
        }
    }
    c
}

fn flowers(seed: u32) -> Canvas {
    let mut c = Canvas::new(8, 5);
    let cols = [Rgba::hex(0x8a6aa0), Rgba::hex(0xb0a050), Rgba::hex(0xa04a40)];
    for i in 0..4 {
        let x = 1 + (rand2(i, 0, seed) * 6.0) as i32;
        let y = 1 + (rand2(i, 1, seed) * 3.0) as i32;
        c.px(x, y + 1, pal::grass().at(0.5));
        c.px(x, y, cols[(seed as usize + i as usize) % 3]);
    }
    c
}

fn reeds(seed: u32) -> Canvas {
    let mut c = Canvas::new(10, 12);
    for i in 0..5 {
        let x = 1 + i * 2;
        let hgt = 6 + (rand2(i, 2, seed) * 5.0) as i32;
        for k in 0..hgt {
            c.px(x + (k / 4), 11 - k, pal::thatch().at(0.3 + k as f32 * 0.05));
        }
        c.px(x + hgt / 4, 11 - hgt, pal::wood().at(0.4));
    }
    c
}

fn ruin_wall() -> Canvas {
    let mut c = Canvas::new(26, 30);
    let ramp = pal::ash();
    let stone = Ramp::from_hex(&[0x1a191b, 0x262427, 0x343134, 0x433f42, 0x524d50]);
    for x in 2..24 {
        let top = 6 + ((x as f32 * 0.9).sin() * 4.0 + rand2(x, 0, 3) * 6.0) as i32;
        let base = 26 - (x - 2) / 3;
        for y in top..base {
            let row = (base - y) / 3;
            let joint = (base - y) % 3 == 0 || (x + row * 3) % 6 == 0;
            c.pxn(x, y, stone.dither(0.55 - (x - 2) as f32 * 0.01 - if joint { 0.2 } else { 0.0 }, x, y), Vec3::new(-0.6, 0.1, 0.8));
        }
    }
    let _ = ramp;
    finish(c)
}

fn pillar() -> Canvas {
    let mut c = Canvas::new(14, 34);
    let stone = Ramp::from_hex(&[0x1e1c1e, 0x2c292c, 0x3b373a, 0x4c4749, 0x5f595b]);
    for y in 4..31 {
        for x in 3..11 {
            let fx = (x as f32 - 6.5) / 4.0;
            let flute = x % 2 == 0;
            c.pxn(x, y, stone.dither(0.6 - fx * 0.3 - if flute { 0.08 } else { 0.0 }, x, y), Vec3::new(fx, 0.0, 0.8));
        }
    }
    // Abgebrochene Oberkante.
    for x in 3..11 {
        let cut = 4 + (rand2(x, 9, 5) * 4.0) as i32;
        for y in 4..cut {
            c.erase(x, y);
        }
    }
    for x in 1..13 {
        c.pxn(x, 31, stone.at(0.45), Vec3::Z);
        c.pxn(x, 32, stone.at(0.35), Vec3::Z);
    }
    finish(c)
}

fn grave(seed: u32) -> Canvas {
    let mut c = Canvas::new(10, 14);
    let stone = pal::stone();
    if seed.is_multiple_of(2) {
        for y in 2..12 {
            for x in 2..8 {
                let round = y < 4 && (x == 2 || x == 7);
                if !round {
                    c.pxn(x, y, stone.dither(0.55 - (x - 2) as f32 * 0.04, x, y), Vec3::new(-0.3, 0.0, 0.9));
                }
            }
        }
    } else {
        for y in 1..12 {
            c.px(5, y, stone.at(0.6));
            c.px(4, y, stone.at(0.45));
        }
        for x in 2..8 {
            c.px(x, 4, stone.at(0.55));
        }
    }
    c.px(3, 12, pal::dirt().at(0.4));
    c.px(6, 12, pal::dirt().at(0.4));
    finish(c)
}

fn bones(seed: u32) -> Canvas {
    let mut c = Canvas::new(12, 6);
    let b = pal::bone();
    c.line(1, 3, 6, 2, b.at(0.7));
    c.line(5, 4, 10, 4, b.at(0.55));
    if seed.is_multiple_of(2) {
        c.ellipse_shaded(8.0, 2.5, 2.0, 1.6, &b, 0.0);
        c.px(9, 2, Rgba::hex(0x101010));
    }
    c
}

fn brazier() -> Canvas {
    let mut c = Canvas::new(10, 16);
    for y in 7..14 {
        c.px(4, y, pal::iron_dark().at(0.6));
        c.px(5, y, pal::iron_dark().at(0.4));
    }
    for x in 1..9 {
        c.px(x, 6, pal::iron_dark().at(0.7));
        c.px(x, 7, pal::iron_dark().at(0.45));
    }
    c.line(2, 14, 4, 12, pal::iron_dark().at(0.5));
    c.line(7, 14, 5, 12, pal::iron_dark().at(0.5));
    for (x, y, col) in [(3, 5, pal::FIRE_DEEP), (4, 4, pal::EMBER), (5, 3, pal::EMBER_HOT), (6, 5, pal::EMBER), (5, 5, pal::EMBER_HOT), (4, 2, pal::EMBER)] {
        c.px(x, y, col);
        c.glow(x, y, 1.0);
    }
    finish(c)
}

fn lantern() -> Canvas {
    let mut c = Canvas::new(8, 22);
    for y in 4..21 {
        c.px(3, y, pal::dark_wood().at(0.5));
    }
    c.line(3, 4, 6, 4, pal::dark_wood().at(0.6));
    for (x, y) in [(5, 5), (6, 5), (5, 6), (6, 6), (5, 7), (6, 7)] {
        c.px(x, y, pal::WINDOW_WARM);
        c.glow(x, y, 1.0);
    }
    finish(c)
}

fn fence() -> Canvas {
    let mut c = Canvas::new(18, 12);
    for (i, x) in [1, 8, 15].iter().enumerate() {
        for y in 2 + i as i32 * 2..10 + i as i32 * 2 - 2 {
            c.px(*x, y.min(11), pal::wood().at(0.4));
        }
    }
    c.line(1, 4, 15, 8, pal::wood().at(0.6));
    c.line(1, 7, 15, 11, pal::wood().at(0.45));
    finish(c)
}

fn stump() -> Canvas {
    let mut c = Canvas::new(10, 8);
    c.ellipse_shaded(5.0, 5.0, 3.5, 2.0, &pal::bark(), 0.0);
    c.px(4, 4, pal::wood().at(0.8));
    c.px(5, 4, pal::wood().at(0.7));
    finish(c)
}

pub fn build(atlas: &mut AtlasBuilder) {
    let foot = |c: &Canvas| Vec2::new(c.w as f32 * 0.5, c.h as f32 - 3.0);
    for v in 0..4u32 {
        let p = pine(100 + v * 7);
        let a = foot(&p);
        atlas.add(format!("tree_pine_{v}"), p, a);
        let o = oak(200 + v * 11);
        let a = foot(&o);
        atlas.add(format!("tree_oak_{v}"), o, a);
    }
    for v in 0..3u32 {
        let d = dead_tree(300 + v);
        let a = foot(&d);
        atlas.add(format!("tree_dead_{v}"), d, a);
    }
    // Minen/Steinbrüche: Anker an der oberen Ecke der 2×2-Grundfläche.
    let gm = gold_mine();
    atlas.add("node_gold_mine", gm, Vec2::new(33.0, 18.0));
    let q = quarry();
    atlas.add("node_quarry", q, Vec2::new(32.0, 12.0));
    for v in 0..3u32 {
        let r = small_rock(v);
        atlas.add(format!("decor_rock_{v}"), r, Vec2::new(5.0, 5.0));
        let b = boulder(v + 10);
        atlas.add(format!("decor_boulder_{v}"), b, Vec2::new(13.0, 16.0));
        let b = bush(v + 20);
        atlas.add(format!("decor_bush_{v}"), b, Vec2::new(8.0, 10.0));
        atlas.add(format!("decor_tuft_{v}"), grass_tuft(v + 30), Vec2::new(4.0, 5.0));
        atlas.add(format!("decor_flowers_{v}"), flowers(v + 40), Vec2::new(4.0, 4.0));
        atlas.add(format!("decor_reeds_{v}"), reeds(v + 50), Vec2::new(5.0, 11.0));
        atlas.add(format!("decor_grave_{v}"), grave(v), Vec2::new(5.0, 12.0));
        atlas.add(format!("decor_bones_{v}"), bones(v), Vec2::new(6.0, 4.0));
    }
    atlas.add("decor_ruin", ruin_wall(), Vec2::new(13.0, 24.0));
    atlas.add("decor_pillar", pillar(), Vec2::new(7.0, 31.0));
    atlas.add("decor_brazier", brazier(), Vec2::new(5.0, 14.0));
    atlas.add("decor_lantern", lantern(), Vec2::new(3.0, 20.0));
    atlas.add("decor_fence", fence(), Vec2::new(9.0, 9.0));
    atlas.add("decor_stump", stump(), Vec2::new(5.0, 6.0));
}
