//! Effekt-Sprites: Partikel, Projektile, Schatten, Auswahlringe, Sammelpunkt-Fahne.

use super::palette as pal;
use ashen_core::color::Rgba;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::Vec2;

fn soft_circle(r: f32, col: Rgba, falloff: f32) -> Canvas {
    let s = (r * 2.0).ceil() as i32 + 1;
    let mut c = Canvas::new(s, s);
    let cx = s as f32 * 0.5;
    for y in 0..s {
        for x in 0..s {
            let d = (Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - Vec2::splat(cx)).length() / r;
            if d <= 1.0 {
                let a = ((1.0 - d).powf(falloff) * 255.0) as u8;
                c.color[(y * s + x) as usize] = col.with_alpha(a.max(1));
            }
        }
    }
    c
}

pub fn build(atlas: &mut AtlasBuilder) {
    let center = |c: &Canvas| Vec2::new(c.w as f32 * 0.5, c.h as f32 * 0.5);
    // Rauch, Staub, Glut, Funken, Magie.
    for (name, r, col, f) in [
        ("fx_smoke", 4.0, Rgba::hex(0x6a6766), 0.6),
        ("fx_smoke_big", 6.0, Rgba::hex(0x55524f), 0.7),
        ("fx_dust", 3.0, Rgba::hex(0x6e5a44), 0.8),
        ("fx_heat", 5.0, Rgba::hex(0xffffff), 1.0),
        ("fx_glow", 3.0, Rgba::hex(0xffffff), 1.2),
    ] {
        let c = soft_circle(r, col, f);
        let a = center(&c);
        atlas.add(name, c, a);
    }
    let mut px = Canvas::new(2, 2);
    px.rect(0, 0, 2, 2, Rgba::hex(0xffffff));
    atlas.add("fx_pixel", px, Vec2::new(1.0, 1.0));
    let mut ember = Canvas::new(1, 1);
    ember.px(0, 0, Rgba::hex(0xffffff));
    atlas.add("fx_dot", ember, Vec2::new(0.5, 0.5));
    let mut spark = Canvas::new(5, 5);
    for i in 0..5 {
        spark.px(2, i, Rgba::hex(0xffffff));
        spark.px(i, 2, Rgba::hex(0xffffff));
    }
    atlas.add("fx_spark", spark, Vec2::new(2.5, 2.5));
    // Flammen (4 Phasen).
    for f in 0..4 {
        let mut c = Canvas::new(8, 12);
        for y in 0..12 {
            let w =
                ((12 - y) as f32 / 12.0 * 3.5 + ((y + f * 3) as f32 * 1.3).sin() * 0.8).max(0.0);
            for x in 0..8 {
                let dx = (x as f32 + 0.5 - 4.0 - ((y + f * 2) as f32 * 0.9).sin() * 0.8).abs();
                if dx < w {
                    let t = 1.0 - y as f32 / 12.0;
                    let col = if dx < w * 0.4 && t > 0.3 {
                        pal::EMBER_HOT
                    } else if t > 0.25 {
                        pal::EMBER
                    } else {
                        pal::FIRE_DEEP
                    };
                    c.px(x, y, col);
                    c.glow(x, y, 1.0);
                }
            }
        }
        atlas.add(format!("fx_flame_{f}"), c, Vec2::new(4.0, 11.0));
    }
    // Projektile.
    let mut arrow = Canvas::new(8, 3);
    for x in 0..6 {
        arrow.px(x, 1, pal::wood().at(0.6));
    }
    arrow.px(6, 1, pal::steel().at(0.9));
    arrow.px(7, 1, pal::steel().at(0.7));
    arrow.px(0, 0, Rgba::hex(0xd8d0c0));
    arrow.px(0, 2, Rgba::hex(0xd8d0c0));
    atlas.add("proj_arrow", arrow, Vec2::new(4.0, 1.5));
    let mut bolt = Canvas::new(6, 3);
    for x in 0..6 {
        bolt.px(x, 1, pal::bone().at(0.5 + x as f32 * 0.08));
    }
    bolt.px(5, 0, pal::bone().at(0.8));
    bolt.px(5, 2, pal::bone().at(0.8));
    atlas.add("proj_bone", bolt, Vec2::new(3.0, 1.5));
    let mut soul = soft_circle(3.0, pal::HOLLOW_GLOW, 0.5);
    for i in 0..soul.normal.len() {
        if soul.color[i].3 > 0 {
            soul.normal[i].emissive = 255;
        }
    }
    atlas.add("proj_soul", soul, Vec2::new(3.5, 3.5));
    // Schatten (weich) in drei Größen.
    for (name, rx, ry) in [
        ("shadow_s", 6.0, 2.5),
        ("shadow_m", 9.0, 3.5),
        ("shadow_l", 14.0, 5.0),
    ] {
        let w = (rx * 2.0) as i32 + 2;
        let h = (ry * 2.0) as i32 + 2;
        let mut c = Canvas::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let dx = (x as f32 + 0.5 - w as f32 * 0.5) / rx;
                let dy = (y as f32 + 0.5 - h as f32 * 0.5) / ry;
                let d = dx * dx + dy * dy;
                if d <= 1.0 {
                    c.color[(y * w + x) as usize] =
                        Rgba(0, 0, 0, ((1.0 - d).powf(0.7) * 255.0) as u8);
                }
            }
        }
        let a = center(&c);
        atlas.add(name, c, a);
    }
    // Auswahlring (Ellipse) in drei Größen.
    for (name, rx, ry) in [
        ("ring_s", 7.0, 3.5),
        ("ring_m", 10.0, 5.0),
        ("ring_l", 16.0, 8.0),
    ] {
        let w = (rx * 2.0) as i32 + 3;
        let h = (ry * 2.0) as i32 + 3;
        let mut c = Canvas::new(w, h);
        let steps = 160;
        for i in 0..steps {
            let a = i as f32 / steps as f32 * std::f32::consts::TAU;
            let x = (w as f32 * 0.5 + a.cos() * rx) as i32;
            let y = (h as f32 * 0.5 + a.sin() * ry) as i32;
            c.px(x, y, Rgba::hex(0xffffff));
        }
        let a = center(&c);
        atlas.add(name, c, a);
    }
    // Sammelpunkt-Fahne.
    let mut flag = Canvas::new(8, 14);
    for y in 0..13 {
        flag.px(1, y, pal::wood().at(0.5));
    }
    for y in 0..5 {
        for x in 2..7 - y / 2 {
            flag.px(x, y + 1, pal::crimson().at(0.7));
        }
    }
    atlas.add("rally_flag", flag, Vec2::new(1.5, 13.0));
    // Zielmarkierung für Befehle.
    for (name, col) in [
        ("marker_move", Rgba::hex(0x8fe08f)),
        ("marker_attack", Rgba::hex(0xe05040)),
    ] {
        let mut c = Canvas::new(11, 7);
        for i in 0..40 {
            let a = i as f32 / 40.0 * std::f32::consts::TAU;
            c.px(
                (5.5 + a.cos() * 4.5) as i32,
                (3.5 + a.sin() * 2.5) as i32,
                col,
            );
        }
        c.px(5, 3, col);
        atlas.add(name, c, Vec2::new(5.5, 3.5));
    }
}
