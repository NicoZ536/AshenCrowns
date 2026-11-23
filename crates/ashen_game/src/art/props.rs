//! Kleine 2D-Bodendetails (Grasbüschel, Blumen, Schilf). Bäume, Felsen und Bauwerke sind 3D-gebacken.

use super::palette as pal;
use ashen_core::color::Rgba;
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
            c.px(
                x + if k == hgt - 1 && i % 2 == 0 { 1 } else { 0 },
                5 - k,
                ramp.at(0.45 + k as f32 * 0.1),
            );
        }
    }
    c
}

fn flowers(seed: u32) -> Canvas {
    let mut c = Canvas::new(8, 5);
    let cols = [
        Rgba::hex(0x8a6aa0),
        Rgba::hex(0xb0a050),
        Rgba::hex(0xa04a40),
    ];
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

pub fn build(atlas: &mut AtlasBuilder) {
    for v in 0..3u32 {
        atlas.add(
            format!("decor_tuft_{v}"),
            grass_tuft(v + 30),
            Vec2::new(4.0, 5.0),
        );
        atlas.add(
            format!("decor_flowers_{v}"),
            flowers(v + 40),
            Vec2::new(4.0, 4.0),
        );
        atlas.add(
            format!("decor_reeds_{v}"),
            reeds(v + 50),
            Vec2::new(5.0, 11.0),
        );
    }
}
