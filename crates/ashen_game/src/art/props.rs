//! Kleine 2D-Bodendetails (Grasbüschel, Blumen, Schilf). Bäume, Felsen und Bauwerke sind 3D-gebacken.

use super::palette as pal;
use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise::rand2;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::Vec2;

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

/// Kieselhäufchen: wenige helle Steinchen mit Schattenpixel (Lichtkante oben links).
fn pebbles(seed: u32) -> Canvas {
    let mut c = Canvas::new(10, 6);
    let ramp = pal::rock();
    for i in 0..4 {
        let x = 1 + (rand2(i, 0, seed) * 6.0) as i32;
        let y = 1 + (rand2(i, 1, seed) * 3.0) as i32;
        let wide = rand2(i, 2, seed) > 0.45;
        c.px(x, y, ramp.at(0.78));
        if wide {
            c.px(x + 1, y, ramp.at(0.62));
        }
        c.px(x, y + 1, ramp.at(0.18));
        if wide {
            c.px(x + 1, y + 1, ramp.at(0.28));
        }
    }
    c
}

/// Pfütze: flache Ellipse, helle Himmelsspiegelung oben, dunkler Rand.
fn puddle(seed: u32) -> Canvas {
    let mut c = Canvas::new(14, 7);
    let ramp = Ramp::from_hex(&[0x1c2428, 0x2c3a40, 0x45575e, 0x6d8088, 0x9db0b4]);
    let rx = 5.2 + rand2(0, 0, seed) * 1.5;
    for y in 0..7 {
        for x in 0..14 {
            let (dx, dy) = ((x as f32 - 6.5) / rx, (y as f32 - 3.0) / 2.6);
            let d = dx * dx + dy * dy;
            if d <= 1.0 {
                let t = if d > 0.7 { 0.1 } else { 0.35 + (1.0 - d) * 0.25 + (3.0 - y as f32) * 0.06 };
                c.pxn(x, y, ramp.at(t), glam::Vec3::new(0.0, 0.0, 1.0));
            }
        }
    }
    c.px(4, 2, ramp.at(0.99));
    c.px(5, 2, ramp.at(0.9));
    c
}

/// Ascheglut: dunkle Brocken mit glimmenden Rissen (leuchtet schwach).
fn embers(seed: u32) -> Canvas {
    let mut c = Canvas::new(10, 6);
    let ash = pal::ash();
    for i in 0..3 {
        let x = 1 + (rand2(i, 0, seed) * 6.0) as i32;
        let y = 1 + (rand2(i, 1, seed) * 3.0) as i32;
        c.px(x, y, ash.at(0.15));
        c.px(x + 1, y, ash.at(0.3));
        c.px(x, y + 1, ash.at(0.05));
        c.px(x + 1, y + 1, ash.at(0.15));
    }
    for i in 0..3 {
        let x = 1 + (rand2(i + 5, 0, seed) * 7.0) as i32;
        let y = 1 + (rand2(i + 5, 1, seed) * 4.0) as i32;
        let hot = rand2(i + 5, 2, seed) > 0.6;
        c.px(x, y, if hot { pal::EMBER_HOT } else { pal::EMBER });
        c.glow(x, y, if hot { 0.9 } else { 0.6 });
    }
    c
}

pub fn build(atlas: &mut AtlasBuilder) {
    for v in 0..3u32 {
        atlas.add(format!("decor_tuft_{v}"), grass_tuft(v + 30), Vec2::new(4.0, 5.0));
        atlas.add(format!("decor_flowers_{v}"), flowers(v + 40), Vec2::new(4.0, 4.0));
        atlas.add(format!("decor_reeds_{v}"), reeds(v + 50), Vec2::new(5.0, 11.0));
        atlas.add(format!("decor_pebbles_{v}"), pebbles(v + 60), Vec2::new(5.0, 4.0));
        atlas.add(format!("decor_embers_{v}"), embers(v + 70), Vec2::new(5.0, 4.0));
        atlas.add(format!("decor_puddle_{v}"), puddle(v + 80), Vec2::new(7.0, 4.0));
    }
}
