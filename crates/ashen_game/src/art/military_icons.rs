//! Icons des Echtzeit-Militärs: Fähigkeiten, Formationen, Gegenstände, Upgrades und die neuen Geschosse
//! (16×16-Pixel-Art, prozedural gezeichnet).

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

fn hex(h: u32) -> Rgba {
    Rgba::hex(h)
}

/// Mischt zwei Farben (t = 0 → a, 1 → b).
fn mix(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let ch = |s: u32| (((a >> s) & 0xff) as f32 * (1.0 - t) + ((b >> s) & 0xff) as f32 * t).round().clamp(0.0, 255.0) as u32;
    (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

fn disc(c: &mut Canvas, cx: f32, cy: f32, r: f32, col: u32, glow: bool) {
    for y in 0..16 {
        for x in 0..16 {
            let d = ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt();
            if d <= r {
                c.px(x, y, hex(mix(col, 0xffffff, ((r - d) / r).max(0.0) * 0.25 - (d / r) * 0.1)));
                if glow {
                    c.glow(x, y, 1.0);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Grundmotive
// ---------------------------------------------------------------------------------------------

fn flame(c: &mut Canvas, main: u32, hi: u32) {
    let rows = [(2, 7, 8), (3, 6, 9), (4, 5, 10), (5, 5, 10), (6, 4, 11), (7, 4, 11), (8, 4, 11), (9, 4, 11), (10, 5, 10), (11, 5, 10), (12, 6, 9), (13, 7, 8)];
    for (y, x0, x1) in rows {
        for x in x0..x1 {
            let t = (y as f32 - 2.0) / 11.0;
            c.px(x, y, hex(mix(hi, main, t)));
            c.glow(x, y, 1.0);
        }
    }
    for y in 7..13 {
        c.px(7, y, hex(mix(0xffffff, hi, 0.4)));
        c.px(8, y, hex(mix(0xffffff, hi, 0.4)));
    }
}

fn cross(c: &mut Canvas, col: u32) {
    for i in 3..13 {
        for w in 0..3 {
            c.px(6 + w, i, hex(mix(col, 0xffffff, 0.15 * (w as f32))));
            c.px(i, 6 + w, hex(mix(col, 0xffffff, 0.15 * (w as f32))));
        }
    }
    c.glow(7, 7, 1.0);
    c.glow(8, 8, 1.0);
}

fn banner(c: &mut Canvas, col: u32, trim: u32) {
    for y in 2..15 {
        c.px(4, y, hex(0x6e4a2e));
    }
    for y in 3..11 {
        for x in 5..13 - (y - 3) / 3 {
            c.px(x, y, hex(mix(col, 0x000000, (y - 3) as f32 * 0.03)));
        }
    }
    for (x, y) in [(8, 5), (8, 6), (7, 6), (9, 6), (8, 7), (7, 7), (9, 7)] {
        c.px(x, y, hex(trim));
    }
}

fn shield(c: &mut Canvas, col: u32, boss: u32) {
    for y in 2..14 {
        let w = if y < 9 { 5 } else { 5 - (y - 8) };
        for x in 8 - w..8 + w {
            let edge = x == 8 - w || x == 7 + w || y == 2;
            c.px(x, y, hex(if edge { mix(col, 0xffffff, 0.45) } else { mix(col, 0x000000, (y - 2) as f32 * 0.025) }));
        }
    }
    disc(c, 8.0, 7.0, 2.0, boss, false);
}

fn skull(c: &mut Canvas, col: u32, eye: u32) {
    for y in 2..11 {
        for x in 4..12 {
            let corner = (y == 2 || y == 10) && (x == 4 || x == 11);
            if !corner {
                c.px(x, y, hex(mix(col, 0x000000, (y - 2) as f32 * 0.03)));
            }
        }
    }
    for x in 5..11 {
        c.px(x, 11, hex(mix(col, 0x000000, 0.2)));
    }
    for x in [6, 8, 10] {
        c.px(x, 12, hex(mix(col, 0x000000, 0.35)));
    }
    for (x, y) in [(5, 5), (6, 5), (5, 6), (6, 6), (9, 5), (10, 5), (9, 6), (10, 6)] {
        c.px(x, y, hex(eye));
        c.glow(x, y, 1.0);
    }
    c.px(7, 8, hex(0x2a2a26));
    c.px(8, 8, hex(0x2a2a26));
}

fn horn(c: &mut Canvas, col: u32) {
    let pts = [(3, 11), (4, 10), (5, 9), (6, 8), (7, 7), (8, 6), (9, 5), (10, 4), (11, 3)];
    for (i, (x, y)) in pts.iter().enumerate() {
        let w = (i as f32 / 3.0) as i32;
        for k in 0..=w {
            c.px(*x, y + k, hex(mix(col, 0x000000, 0.08 * k as f32)));
            c.px(x + 1, *y + k, hex(mix(col, 0xffffff, 0.12)));
        }
    }
    c.line(11, 3, 13, 2, hex(0xc9c6ad));
    for (x, y) in [(13, 6), (14, 8), (13, 10)] {
        c.px(x, y, hex(0xe8dcb0));
    }
}

fn axe(c: &mut Canvas, col: u32) {
    c.line(3, 13, 11, 4, hex(0x6e4a2e));
    c.line(4, 13, 12, 4, hex(0x5a3a22));
    for y in 2..8 {
        for x in 8..14 {
            if (x - 8) + (y - 2) < 8 && (x - 8) + (7 - y) < 7 {
                c.px(x, y, hex(mix(col, 0xffffff, ((13 - x) as f32) * 0.05)));
            }
        }
    }
}

fn burst(c: &mut Canvas, col: u32, ring: u32) {
    for i in 0..16 {
        let a = i as f32 / 16.0 * std::f32::consts::TAU;
        c.px((8.0 + a.cos() * 5.5) as i32, (8.0 + a.sin() * 5.0) as i32, hex(ring));
        c.glow((8.0 + a.cos() * 5.5) as i32, (8.0 + a.sin() * 5.0) as i32, 0.8);
    }
    disc(c, 8.0, 8.0, 3.0, col, true);
}

fn mountain_fist(c: &mut Canvas, col: u32) {
    for y in 4..13 {
        for x in 4..12 {
            let o = (y - 4) as f32 * 0.2;
            if (x as f32) > 5.0 - o * 0.3 && (x as f32) < 10.5 + o * 0.5 {
                c.px(x, y, hex(mix(col, 0x000000, (y - 4) as f32 * 0.05)));
            }
        }
    }
    for x in 3..13 {
        c.px(x, 13, hex(0x3a3a3e));
        c.px(x, 14, hex(0x2a2a2e));
    }
    c.line(4, 12, 2, 14, hex(0x2a2a2e));
    c.line(12, 12, 14, 14, hex(0x2a2a2e));
}

fn column(c: &mut Canvas, main: u32, hi: u32) {
    for y in 1..14 {
        let w = 2 + (y as f32 * 0.25) as i32;
        for x in 8 - w..8 + w {
            c.px(x, y, hex(mix(hi, main, (y as f32 - 1.0) / 13.0)));
            c.glow(x, y, 1.0);
        }
    }
    for x in 3..13 {
        c.px(x, 14, hex(mix(main, 0x000000, 0.2)));
    }
}

fn crescent(c: &mut Canvas, main: u32, hi: u32) {
    for y in 2..14 {
        for x in 2..14 {
            let d1 = ((x as f32 - 7.5).powi(2) + (y as f32 - 8.0).powi(2)).sqrt();
            let d2 = ((x as f32 - 9.5).powi(2) + (y as f32 - 8.0).powi(2)).sqrt();
            if d1 <= 6.0 && d2 > 5.0 {
                c.px(x, y, hex(mix(hi, main, d1 / 6.0)));
                c.glow(x, y, 1.0);
            }
        }
    }
}

/// Zeichnet das Motiv zu einer Fähigkeit.
fn ability_icon(id: &str) -> Canvas {
    let mut c = icon();
    match id {
        "fireball" => {
            disc(&mut c, 8.0, 8.0, 4.5, 0xff7a20, true);
            disc(&mut c, 7.0, 7.0, 2.0, 0xffe09a, true);
            c.line(12, 3, 14, 1, hex(0xff9a40));
            c.line(11, 4, 13, 2, hex(0xffb050));
        }
        "flame_nova" | "phoenix_cut" => {
            if id == "phoenix_cut" {
                crescent(&mut c, 0xff6a20, 0xffe09a);
            } else {
                burst(&mut c, 0xffa030, 0xff6a20);
            }
        }
        "sacred_flame" => flame(&mut c, 0xffb040, 0xffe9b0),
        "pillar_of_fire" => column(&mut c, 0xff6a20, 0xffe09a),
        "holy_heal" | "flame_mend" => cross(&mut c, 0x5ee08a),
        "resurgence" => {
            cross(&mut c, 0xffe9a0);
            burst(&mut c, 0x5ee08a, 0xb4ffd0);
            cross(&mut c, 0xffe9a0);
        }
        "blessing" => banner(&mut c, 0xdcc060, 0xffffff),
        "beacon_of_faith" => {
            flame(&mut c, 0xffc040, 0xfff0c0);
            for x in [2, 13] {
                c.px(x, 8, hex(0xffe9a0));
            }
        }
        "war_cry" => banner(&mut c, 0xb3201c, 0xffe9a0),
        "war_horns" => horn(&mut c, 0xd6c27a),
        "shield_wall" => shield(&mut c, 0x8b969e, 0xcfa23f),
        "iron_vow" => shield(&mut c, 0x5a6a7c, 0xc9d4de),
        "last_stand" => shield(&mut c, 0xb3201c, 0xffe9a0),
        "aegis_of_flame" => {
            shield(&mut c, 0xd86a1c, 0xffe09a);
            c.glow(8, 7, 1.0);
        }
        "stone_skin" => mountain_fist(&mut c, 0x8a867c),
        "ground_slam" => mountain_fist(&mut c, 0x6a5a48),
        "ancestral_fury" => axe(&mut c, 0xaab4bc),
        "clan_roar" => {
            burst(&mut c, 0xcfa23f, 0xe8dcb0);
            horn(&mut c, 0xd6c27a);
        }
        "raise_dead" => skull(&mut c, 0xc9c6ad, 0x86f2a0),
        _ => burst(&mut c, 0x9aa4ac, 0xcfd6dc),
    }
    c
}

pub const ABILITY_IDS: [&str; 21] = [
    "shield_wall", "fireball", "flame_nova", "holy_heal", "blessing", "raise_dead", "flame_mend", "sacred_flame", "beacon_of_faith", "clan_roar", "ground_slam", "iron_vow", "phoenix_cut", "war_cry",
    "last_stand", "pillar_of_fire", "resurgence", "aegis_of_flame", "ancestral_fury", "stone_skin", "war_horns",
];

// ---------------------------------------------------------------------------------------------
// Formationen
// ---------------------------------------------------------------------------------------------

fn formation_icon(name: &str) -> Canvas {
    let mut c = icon();
    let dots: Vec<(i32, i32)> = match name {
        "line" => (0..6).map(|i| (2 + i * 2, 5)).chain((0..6).map(|i| (2 + i * 2, 9))).collect(),
        "wedge" => vec![(8, 2), (6, 5), (10, 5), (4, 8), (8, 8), (12, 8), (2, 11), (6, 11), (10, 11), (14, 11)],
        "column" => (0..5).flat_map(|i| [(6, 2 + i * 3), (10, 2 + i * 3)]).collect(),
        "square" => vec![(3, 3), (6, 3), (9, 3), (12, 3), (3, 6), (12, 6), (3, 9), (12, 9), (3, 12), (6, 12), (9, 12), (12, 12)],
        _ => vec![(4, 4), (8, 4), (12, 4), (4, 8), (8, 8), (12, 8), (4, 12), (8, 12), (12, 12)],
    };
    for (x, y) in dots {
        c.rect(x - 1, y - 1, 2, 2, hex(0xe8dcb0));
        c.px(x - 1, y - 1, hex(0xffffff));
        c.px(x, y, hex(0x8a7a50));
    }
    c
}

// ---------------------------------------------------------------------------------------------
// Gegenstände
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Glyph {
    Sword,
    Axe,
    Staff,
    Armor,
    Amulet,
    Ring,
    Potion,
}

fn item_glyph(g: Glyph, accent: u32) -> Canvas {
    let mut c = icon();
    match g {
        Glyph::Sword => {
            c.line(3, 13, 12, 3, hex(0xcfd6dc));
            c.line(4, 13, 13, 3, hex(0x8b969e));
            c.line(2, 10, 6, 14, hex(accent));
            c.line(3, 10, 6, 13, hex(mix(accent, 0x000000, 0.3)));
            c.px(2, 14, hex(0x5a3a22));
            c.px(1, 15, hex(0x5a3a22));
        }
        Glyph::Axe => axe(&mut c, 0xaab4bc),
        Glyph::Staff => {
            c.line(3, 14, 11, 5, hex(0x6e4a2e));
            c.line(4, 14, 12, 5, hex(0x4a3220));
            disc(&mut c, 12.0, 4.0, 2.6, accent, true);
        }
        Glyph::Armor => {
            for y in 3..14 {
                let w = if y < 6 { 5 } else if y < 12 { 4 } else { 3 };
                for x in 8 - w..8 + w {
                    c.px(x, y, hex(mix(0x8b969e, 0x000000, (y - 3) as f32 * 0.03)));
                }
            }
            for x in 2..5 {
                c.px(x, 4, hex(0x9aa4ac));
                c.px(x + 9, 4, hex(0x9aa4ac));
            }
            for y in 5..13 {
                c.px(8, y, hex(accent));
            }
            for x in 4..12 {
                c.px(x, 9, hex(mix(accent, 0x000000, 0.35)));
            }
        }
        Glyph::Amulet => {
            for i in 0..8 {
                c.px(4 + i / 2, 2 + i, hex(0xcfa23f));
                c.px(11 - i / 2, 2 + i, hex(0xcfa23f));
            }
            disc(&mut c, 8.0, 11.0, 3.2, accent, true);
            disc(&mut c, 7.0, 10.0, 1.0, 0xffffff, false);
        }
        Glyph::Ring => {
            for y in 5..14 {
                for x in 3..13 {
                    let d = ((x as f32 - 7.5).powi(2) + (y as f32 - 9.5).powi(2)).sqrt();
                    if d <= 4.6 && d > 2.6 {
                        c.px(x, y, hex(mix(0xcfa23f, 0xffffff, (9.5 - y as f32) * 0.05)));
                    }
                }
            }
            disc(&mut c, 7.5, 4.0, 2.2, accent, true);
        }
        Glyph::Potion => {
            for y in 7..14 {
                let w = if y < 9 { 3 } else { 4 };
                for x in 8 - w..8 + w {
                    c.px(x, y, hex(mix(accent, 0xffffff, if x < 7 { 0.25 } else { 0.0 })));
                    c.glow(x, y, 0.5);
                }
            }
            for y in 3..7 {
                c.px(7, y, hex(0xc8d0d8));
                c.px(8, y, hex(0xaab4bc));
            }
            c.px(7, 2, hex(0x8a5a30));
            c.px(8, 2, hex(0x8a5a30));
        }
    }
    c
}

/// Zeichnet der Gegenstandskennung entsprechend (bekannte Inhalte), sonst nach Platz.
fn item_icon_for(id: &str) -> Canvas {
    use Glyph::*;
    match id {
        "rusty_sword" => item_glyph(Sword, 0x7a5a3e),
        "steel_sword" => item_glyph(Sword, 0x8b969e),
        "embersteel_blade" => item_glyph(Sword, 0xf08a2c),
        "edran_blade" => item_glyph(Sword, 0xe8c050),
        "skarn_greataxe" => item_glyph(Axe, 0xaab4bc),
        "pilgrim_staff" => item_glyph(Staff, 0xffd35a),
        "padded_jerkin" => item_glyph(Armor, 0xb8a47a),
        "chain_hauberk" => item_glyph(Armor, 0x8b969e),
        "ember_plate" => item_glyph(Armor, 0xd86a1c),
        "grauwall_aegis" => item_glyph(Armor, 0xe8c050),
        "bone_charm" => item_glyph(Amulet, 0xc9c6ad),
        "flamekeeper_pendant" => item_glyph(Amulet, 0xffa030),
        "oath_medal" => item_glyph(Amulet, 0xb3201c),
        "ashglass_heart" => item_glyph(Amulet, 0x86f2a0),
        "copper_ring" => item_glyph(Ring, 0xb4693a),
        "iron_band" => item_glyph(Ring, 0x8b969e),
        "wolf_signet" => item_glyph(Ring, 0x6a7d91),
        "ember_ring" => item_glyph(Ring, 0xf08a2c),
        "varkhan_signet" => item_glyph(Ring, 0xe8c050),
        "vael_dorn_seal" => item_glyph(Ring, 0x86f2a0),
        "potion_heal" => item_glyph(Potion, 0xd0303a),
        "potion_heal_greater" => item_glyph(Potion, 0xff4060),
        "potion_mana" => item_glyph(Potion, 0x3a70e0),
        "potion_ember" => item_glyph(Potion, 0xff8a2c),
        "potion_ward" => item_glyph(Potion, 0x8a8e96),
        _ => item_glyph(Ring, 0x9aa4ac),
    }
}

const ITEM_IDS: [&str; 25] = [
    "rusty_sword", "steel_sword", "pilgrim_staff", "embersteel_blade", "skarn_greataxe", "edran_blade", "padded_jerkin", "chain_hauberk", "ember_plate", "grauwall_aegis", "bone_charm", "flamekeeper_pendant", "oath_medal",
    "ashglass_heart", "copper_ring", "iron_band", "wolf_signet", "ember_ring", "varkhan_signet", "vael_dorn_seal", "potion_heal", "potion_heal_greater", "potion_mana", "potion_ember", "potion_ward",
];

/// Ersatzicons für Gegenstände aus Mods: Platz × Glyph.
const FALLBACK: [(&str, Glyph, u32); 5] = [("weapon", Glyph::Sword, 0x8b969e), ("armor", Glyph::Armor, 0x8b969e), ("amulet", Glyph::Amulet, 0x9aa4ac), ("ring", Glyph::Ring, 0x9aa4ac), ("potion", Glyph::Potion, 0x9aa4ac)];

// ---------------------------------------------------------------------------------------------
// Upgrades und sonstige Symbole
// ---------------------------------------------------------------------------------------------

fn upgrade_icon(id: &str) -> Canvas {
    let mut c = icon();
    match id {
        "forge_weapons" => {
            c = item_glyph(Glyph::Sword, 0xcfa23f);
            c.px(13, 12, hex(0xffe09a));
            c.px(12, 13, hex(0xffe09a));
            c.px(14, 13, hex(0xffe09a));
            c.px(13, 14, hex(0xffe09a));
        }
        "forge_armor" => {
            c = item_glyph(Glyph::Armor, 0xcfa23f);
            c.px(13, 12, hex(0xffe09a));
            c.px(12, 13, hex(0xffe09a));
            c.px(14, 13, hex(0xffe09a));
            c.px(13, 14, hex(0xffe09a));
        }
        "fletching" => {
            c.line(2, 13, 13, 2, hex(0x9a7a50));
            c.line(2, 12, 12, 2, hex(0x6e4a2e));
            for (x, y) in [(12, 2), (13, 3), (11, 3), (13, 1), (14, 2)] {
                c.px(x, y, hex(0xcfd6dc));
            }
            c.line(2, 13, 5, 13, hex(0xe8dcc0));
            c.line(2, 13, 2, 10, hex(0xe8dcc0));
        }
        _ => burst(&mut c, 0xcfa23f, 0xe8dcb0),
    }
    c
}

fn hero_icon() -> Canvas {
    // Heldenhelm mit Federbusch.
    let mut c = icon();
    for y in 4..14 {
        for x in 4..12 {
            let corner = (y == 4) && (x == 4 || x == 11);
            if !corner {
                c.px(x, y, hex(mix(0xaab4bc, 0x000000, (y - 4) as f32 * 0.04)));
            }
        }
    }
    for x in 5..11 {
        c.px(x, 8, hex(0x17130f));
        c.px(x, 9, hex(0x17130f));
    }
    for y in 9..14 {
        c.px(8, y, hex(0x17130f));
    }
    for y in 1..5 {
        c.px(7, y, hex(0xb3201c));
        c.px(8, y, hex(0xd0303a));
    }
    c
}

fn talent_icon() -> Canvas {
    let mut c = icon();
    // Stern.
    for i in 0..8 {
        let a = i as f32 / 8.0 * std::f32::consts::TAU;
        let r = if i % 2 == 0 { 6.5 } else { 3.0 };
        c.line(8, 8, (8.0 + a.cos() * r) as i32, (8.0 + a.sin() * r) as i32, hex(0xe8c050));
    }
    disc(&mut c, 8.0, 8.0, 2.4, 0xffe9a0, true);
    c
}

fn projectiles(atlas: &mut AtlasBuilder) {
    // Armbrustbolzen: kurz, dick, mit Eisenspitze.
    let mut b = Canvas::new(8, 3);
    for x in 0..6 {
        b.px(x, 1, hex(0x7a5a3a));
    }
    b.px(0, 0, hex(0xd8d0c0));
    b.px(0, 2, hex(0xd8d0c0));
    b.px(6, 1, hex(0xcfd6dc));
    b.px(7, 1, hex(0x9aa4ac));
    b.px(6, 0, hex(0x8b969e));
    b.px(6, 2, hex(0x8b969e));
    atlas.add("proj_bolt", b, Vec2::new(4.0, 1.5));
    // Feuerball: leuchtende Kugel mit Schweif.
    let mut f = Canvas::new(10, 10);
    for y in 0..10 {
        for x in 0..10 {
            let d = ((x as f32 - 4.5).powi(2) + (y as f32 - 4.5).powi(2)).sqrt();
            if d <= 4.2 {
                let col = if d < 1.8 { 0xfff0b0 } else if d < 3.0 { 0xffb040 } else { 0xff6a20 };
                f.px(x, y, hex(col));
                f.glow(x, y, 1.0);
            }
        }
    }
    atlas.add("proj_fireball", f, Vec2::new(5.0, 5.0));
    // Felsbrocken.
    let mut r = Canvas::new(12, 12);
    for y in 0..12 {
        for x in 0..12 {
            let d = ((x as f32 - 5.5).powi(2) + (y as f32 - 5.5).powi(2)).sqrt();
            if d <= 5.3 {
                let shade = 0.35 + (11 - y) as f32 * 0.03 + if (x + y * 3) % 5 == 0 { -0.08 } else { 0.0 };
                r.px(x, y, hex(mix(0x2a2a2e, 0xa8a49a, shade.clamp(0.0, 1.0))));
            }
        }
    }
    atlas.add("proj_boulder", r, Vec2::new(6.0, 6.0));
}

pub fn build(atlas: &mut AtlasBuilder) {
    for id in ABILITY_IDS {
        atlas.add(format!("icon_{id}"), finish(ability_icon(id)), Vec2::ZERO);
    }
    for f in ["block", "line", "wedge", "column", "square"] {
        atlas.add(format!("icon_formation_{f}"), finish(formation_icon(f)), Vec2::ZERO);
    }
    for id in ITEM_IDS {
        atlas.add(format!("icon_item_{id}"), finish(item_icon_for(id)), Vec2::ZERO);
    }
    for (slot, g, accent) in FALLBACK {
        atlas.add(format!("icon_itemslot_{slot}"), finish(item_glyph(g, accent)), Vec2::ZERO);
    }
    for id in ["forge_weapons", "forge_armor", "fletching"] {
        atlas.add(format!("icon_up_{id}"), finish(upgrade_icon(id)), Vec2::ZERO);
    }
    atlas.add("icon_hero", finish(hero_icon()), Vec2::ZERO);
    atlas.add("icon_talent", finish(talent_icon()), Vec2::ZERO);
    projectiles(atlas);
}
