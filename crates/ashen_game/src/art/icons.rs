//! UI-Icons (Ressourcen, Befehle, Fähigkeiten) als 16×16-Pixel-Art im Atlas.

use super::palette as pal;
use ashen_core::color::Rgba;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::Vec2;

fn icon() -> Canvas {
    Canvas::new(16, 16)
}

fn finish(mut c: Canvas) -> Canvas {
    c.outline(0.3);
    c
}

pub fn build(atlas: &mut AtlasBuilder) {
    let mut add = |name: &str, c: Canvas| atlas.add(format!("icon_{name}"), finish(c), Vec2::ZERO);
    // Gold: Münzstapel.
    let mut c = icon();
    for (i, y) in [11, 9, 7, 5].iter().enumerate() {
        for x in 4..12 {
            c.px(x, *y, pal::gold().at(0.45 + i as f32 * 0.1));
            c.px(x, y + 1, pal::gold().at(0.3));
        }
        c.px(4 + i as i32 % 2, *y, pal::gold().at(0.95));
    }
    add("gold", c);
    // Holz: Stämme.
    let mut c = icon();
    for (i, (x, y)) in [(3, 9), (8, 9), (5, 5)].iter().enumerate() {
        for dx in 0..6 {
            for dy in 0..4 {
                c.px(x + dx, y + dy, pal::wood().at(0.35 + dy as f32 * 0.1));
            }
        }
        c.px(x + 5, y + 1, pal::wood().at(0.85));
        c.px(x + 5, y + 2, pal::bark().at(0.4));
        let _ = i;
    }
    add("wood", c);
    // Stein: Quader.
    let mut c = icon();
    for y in 5..12 {
        for x in 3..13 {
            let top = y < 7;
            c.px(x - (y - 5) / 3, y, pal::stone().at(if top { 0.8 } else { 0.5 - (x - 3) as f32 * 0.02 }));
        }
    }
    add("stone", c);
    // Nahrung: Ähre.
    let mut c = icon();
    for y in 3..14 {
        c.px(8, y, pal::thatch().at(0.5));
    }
    for k in 0..4 {
        let y = 3 + k * 2;
        c.px(7, y, pal::thatch().at(0.85));
        c.px(9, y + 1, pal::thatch().at(0.75));
        c.px(6, y + 1, pal::thatch().at(0.6));
        c.px(10, y + 2, pal::thatch().at(0.6));
    }
    add("food", c);
    // Bevölkerung: Figur.
    let mut c = icon();
    c.ellipse_shaded(8.0, 5.0, 2.5, 2.5, &pal::skin(), 0.0);
    for y in 8..14 {
        for x in 5..11 {
            c.px(x, y, pal::cloth_brown().at(0.6 - (x - 5) as f32 * 0.05));
        }
    }
    add("pop", c);
    // Befehle.
    let mut c = icon();
    c.line(3, 12, 12, 3, pal::steel().at(0.8));
    c.line(3, 11, 11, 3, pal::steel().at(0.6));
    c.line(10, 12, 13, 9, pal::gold().at(0.6));
    add("attack", c);
    let mut c = icon();
    for (x0, y0, x1, y1) in [(3, 8, 12, 8), (9, 5, 12, 8), (9, 11, 12, 8)] {
        c.line(x0, y0, x1, y1, Rgba::hex(0x9fe09f));
    }
    add("move", c);
    let mut c = icon();
    for y in 4..12 {
        for x in 4..12 {
            c.px(x, y, pal::crimson().at(0.6));
        }
    }
    add("stop", c);
    let mut c = icon();
    for y in 3..13 {
        for x in 5..11 {
            let edge = x == 5 || x == 10 || y == 3;
            c.px(x, y, if edge { pal::steel().at(0.9) } else { pal::crimson().at(0.55) });
        }
    }
    add("hold", c);
    let mut c = icon();
    for i in 0..10 {
        let a = i as f32 / 10.0 * std::f32::consts::TAU;
        c.px((8.0 + a.cos() * 5.0) as i32, (8.0 + a.sin() * 5.0) as i32, Rgba::hex(0xd0c080));
    }
    c.line(12, 4, 13, 8, Rgba::hex(0xd0c080));
    add("patrol", c);
    let mut c = icon();
    c.line(3, 13, 10, 6, pal::wood().at(0.6));
    for y in 3..8 {
        for x in 9..14 {
            c.px(x, y, pal::steel().at(0.7));
        }
    }
    add("build", c);
    let mut c = icon();
    for y in 3..14 {
        c.px(5, y, pal::wood().at(0.5));
    }
    for y in 3..8 {
        for x in 6..12 - (y - 3) / 2 {
            c.px(x, y, pal::crimson().at(0.7));
        }
    }
    add("rally", c);
    let mut c = icon();
    c.line(4, 4, 12, 12, Rgba::hex(0xe05040));
    c.line(12, 4, 4, 12, Rgba::hex(0xe05040));
    add("cancel", c);
    let mut c = icon();
    for y in 4..13 {
        c.px(5, y, pal::wood().at(0.5));
        c.px(6, y, pal::wood().at(0.4));
    }
    c.line(5, 4, 11, 7, pal::steel().at(0.8));
    add("demolish", c);
    // Fähigkeiten.
    let mut c = icon();
    for i in 0..22 {
        let a = i as f32 / 22.0 * std::f32::consts::TAU * 1.3;
        let r = 2.0 + i as f32 * 0.25;
        let x = (8.0 + a.cos() * r) as i32;
        let y = (8.0 + a.sin() * r * 0.8) as i32;
        c.px(x, y, if i > 14 { pal::EMBER } else { pal::EMBER_HOT });
        c.glow(x, y, 1.0);
    }
    add("ember_cleave", c);
    let mut c = icon();
    for y in 2..15 {
        c.px(4, y, pal::wood().at(0.55));
    }
    for y in 3..11 {
        for x in 5..13 {
            c.px(x, y, pal::crimson().at(0.65 - (y - 3) as f32 * 0.02));
        }
    }
    for (x, y) in [(8, 5), (8, 6), (7, 6), (9, 6), (8, 7), (7, 7), (9, 7)] {
        c.px(x, y, pal::gold().at(0.8));
    }
    add("rallying_banner", c);
    let mut c = icon();
    for y in 3..14 {
        let w = if y < 10 { 5 } else { 5 - (y - 9) };
        for x in 8 - w..8 + w {
            c.px(x, y, Rgba::hex(0x6a6870).lerp(pal::EMBER, ((y - 3) as f32 / 11.0) * 0.5));
        }
    }
    c.px(8, 7, pal::EMBER_HOT);
    add("ashen_ward", c);
}
