//! Prozedurale Einheiten-Sprites mit Animationen (idle, walk, attack, work, cast, death).
//!
//! Figuren werden aus Körperteilen gezeichnet; jede Animationsphase ist eine eigene Pose,
//! sodass Gehen, Schlagen und Fallen tatsächlich animiert sind.

use super::palette as pal;
use ashen_core::color::{Ramp, Rgba};
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::{Vec2, Vec3};
use std::f32::consts::{PI, TAU};

#[derive(Clone, Copy, PartialEq)]
enum Helmet {
    Hood,
    Kettle,
    Great,
    Circlet,
    Skull,
    Cap,
}

#[derive(Clone, Copy, PartialEq)]
enum Weapon {
    None,
    Sword,
    EmberSword,
    Spear,
    Bow,
    Axe,
    Lance,
    RustBlade,
    BoneBow,
    Club,
    Staff,
}

#[derive(Clone)]
struct Figure {
    skin: Ramp,
    body: Ramp,
    legs: Ramp,
    accent: Option<Ramp>,
    helmet: Helmet,
    helmet_ramp: Ramp,
    weapon: Weapon,
    shield: Option<Ramp>,
    cape: Option<Ramp>,
    width: i32,
    undead: bool,
    hair: Option<Rgba>,
}

#[derive(Clone, Copy, Default)]
struct Pose {
    leg_phase: f32,
    walking: bool,
    bob: i32,
    arm: f32,
    reach: f32,
    draw: f32,
    cast: f32,
    carry: bool,
}

const W: i32 = 26;
const H: i32 = 30;
const FOOT: i32 = H - 3;

fn draw_legs(c: &mut Canvas, cx: i32, hip: i32, pose: &Pose, f: &Figure) {
    let swing = if pose.walking { (pose.leg_phase * TAU).sin() * 2.0 } else { 0.0 };
    for (side, dir) in [(-1, 1.0f32), (1, -1.0f32)] {
        let off = (swing * dir).round() as i32;
        let lift = if pose.walking && (swing * dir) > 0.8 { 1 } else { 0 };
        // Vorderes Bein (rechts) heller, hinteres dunkler.
        let x = cx + side + off.clamp(-2, 2);
        let t = if side < 0 { 0.55 } else { 0.3 };
        for y in hip..=FOOT - lift {
            c.pxn(x, y, f.legs.at(t), Vec3::new(side as f32 * 0.3, 0.0, 1.0));
        }
        // Stiefel.
        let boot = if f.undead { pal::bone().at(0.4) } else { pal::leather().at(0.25) };
        c.px(x, FOOT - lift, boot);
        c.px(x + 1, FOOT - lift, boot);
    }
}

fn draw_cape(c: &mut Canvas, cx: i32, shoulder: i32, hip: i32, pose: &Pose, ramp: &Ramp) {
    let flutter = if pose.walking { ((pose.leg_phase * TAU).cos() * 1.5).round() as i32 } else { 0 };
    let bottom = hip + 4;
    let pts = [
        Vec2::new(cx as f32 - 2.5, shoulder as f32),
        Vec2::new(cx as f32 + 1.5, shoulder as f32),
        Vec2::new(cx as f32 - 1.0 - flutter as f32, bottom as f32 + 1.0),
        Vec2::new(cx as f32 - 5.0 - flutter as f32, bottom as f32),
    ];
    c.polygon(&pts, |x, y| {
        let t = 0.25 + (y - shoulder) as f32 * 0.02 + if (x + y) % 3 == 0 { 0.1 } else { 0.0 };
        Some((ramp.dither(t, x, y), Vec3::new(-0.4, 0.1, 0.9)))
    });
}

fn draw_torso(c: &mut Canvas, cx: i32, shoulder: i32, hip: i32, f: &Figure) {
    let hw = f.width / 2;
    for y in shoulder..=hip {
        let row_w = if y == shoulder { hw - 1 } else { hw };
        for x in cx - row_w..=cx + row_w - 1 + (f.width % 2) {
            let fx = (x - cx) as f32 / hw.max(1) as f32;
            let fy = (y - shoulder) as f32 / (hip - shoulder).max(1) as f32;
            let n = Vec3::new(fx * 0.7, -0.3 + fy * 0.3, 0.8);
            let t = 0.65 - fx * 0.25 - fy * 0.2;
            let mut col = f.body.dither(t, x, y);
            if let Some(acc) = &f.accent
                && (x - cx).abs() <= 1
                && y > shoulder + 1
            {
                col = acc.dither(t + 0.1, x, y);
            }
            if f.undead && (y - shoulder) % 2 == 1 && (x - cx).abs() < hw {
                // Rippen sichtbar.
                col = pal::bone().dither(0.55 - fx * 0.2, x, y);
            }
            c.pxn(x, y, col, n);
        }
    }
    // Gürtel.
    if !f.undead {
        for x in cx - hw..cx + hw + (f.width % 2) {
            c.px(x, hip - 1, pal::leather().at(0.2));
        }
        c.px(cx, hip - 1, pal::gold().at(0.6));
    }
}

fn draw_head(c: &mut Canvas, cx: i32, top: i32, f: &Figure) {
    // Kopf 4×4 mit Schattierung.
    for y in top..top + 4 {
        for x in cx - 2..cx + 2 {
            let t = 0.7 - (x - cx + 2) as f32 * 0.12 - (y - top) as f32 * 0.05;
            c.pxn(x, y, f.skin.dither(t, x, y), Vec3::new((x - cx) as f32 * 0.3, (y - top - 2) as f32 * 0.3, 0.9));
        }
    }
    // Augen (Blickrichtung rechts).
    if f.undead {
        c.px(cx + 1, top + 1, pal::HOLLOW_GLOW);
        c.glow(cx + 1, top + 1, 1.0);
        c.px(cx - 1, top + 1, pal::HOLLOW_DEEP);
        c.glow(cx - 1, top + 1, 0.6);
    } else {
        c.px(cx + 1, top + 1, Rgba::hex(0x1a1210));
    }
    if let Some(h) = f.hair {
        for x in cx - 2..cx + 1 {
            c.px(x, top - 1, h);
        }
        c.px(cx - 2, top, h);
        c.px(cx - 2, top + 1, h);
        c.px(cx - 3, top + 2, h);
        c.px(cx - 3, top + 3, h.scale(0.8));
    }
    let hr = &f.helmet_ramp;
    match f.helmet {
        Helmet::Hood => {
            for x in cx - 3..cx + 2 {
                c.px(x, top - 1, hr.at(0.4));
            }
            for y in top..top + 4 {
                c.px(cx - 3, y, hr.at(0.25));
                c.px(cx - 2, y, hr.at(0.35));
            }
            c.px(cx + 1, top - 1, hr.at(0.55));
        }
        Helmet::Cap => {
            for x in cx - 2..cx + 2 {
                c.px(x, top - 1, hr.at(0.5));
            }
            c.px(cx + 2, top, hr.at(0.35));
        }
        Helmet::Kettle => {
            for x in cx - 3..cx + 3 {
                c.pxn(x, top, hr.at(if x == cx + 2 || x == cx - 3 { 0.35 } else { 0.7 }), Vec3::new(0.0, -0.6, 0.8));
            }
            for x in cx - 2..cx + 2 {
                c.pxn(x, top - 1, hr.at(0.8), Vec3::new(-0.3, -0.7, 0.6));
            }
        }
        Helmet::Great => {
            for y in top - 1..top + 4 {
                for x in cx - 2..cx + 2 {
                    let t = 0.75 - (x - cx + 2) as f32 * 0.12;
                    c.pxn(x, y, hr.dither(t, x, y), Vec3::new((x - cx) as f32 * 0.35, -0.2, 0.9));
                }
            }
            c.px(cx + 1, top + 1, Rgba::hex(0x0c0c0e));
            c.px(cx, top + 1, Rgba::hex(0x0c0c0e));
            c.px(cx - 1, top - 2, pal::crimson().at(0.7));
            c.px(cx - 2, top - 2, pal::crimson().at(0.6));
        }
        Helmet::Circlet => {
            for x in cx - 2..cx + 2 {
                c.px(x, top, pal::gold().at(0.7));
            }
            c.px(cx, top, pal::EMBER);
            c.glow(cx, top, 0.7);
        }
        Helmet::Skull => {
            // Gesichtsknochen über dem Kopf.
            for y in top..top + 3 {
                for x in cx - 2..cx + 2 {
                    c.px(x, y, pal::bone().at(0.65 - (x - cx + 2) as f32 * 0.08));
                }
            }
            c.px(cx + 1, top + 1, pal::HOLLOW_GLOW);
            c.glow(cx + 1, top + 1, 1.0);
            c.px(cx - 1, top + 1, pal::HOLLOW_DEEP);
            c.px(cx, top + 3, pal::bone().at(0.3));
        }
    }
}

/// Zeichnet die Waffe von der Hand aus mit Winkel (0 = waagerecht nach vorne, negativ = nach oben).
fn draw_weapon(c: &mut Canvas, hand: Vec2, angle: f32, weapon: Weapon, pose: &Pose) {
    let dir = Vec2::new(angle.cos(), angle.sin());
    let line = |c: &mut Canvas, from: Vec2, len: f32, ramp: &Ramp, glow: f32| {
        let steps = (len * 2.0) as i32;
        for i in 0..=steps {
            let p = from + dir * (i as f32 * 0.5);
            let t = 0.85 - i as f32 / steps.max(1) as f32 * 0.4;
            let (x, y) = (p.x.round() as i32, p.y.round() as i32);
            c.pxn(x, y, ramp.at(t), Vec3::new(-dir.y, dir.x, 0.6));
            if glow > 0.0 {
                c.glow(x, y, glow * (0.4 + 0.6 * i as f32 / steps.max(1) as f32));
            }
        }
    };
    match weapon {
        Weapon::None => {}
        Weapon::Sword | Weapon::RustBlade => {
            let ramp = if weapon == Weapon::Sword { pal::steel() } else { Ramp::from_hex(&[0x2a1d14, 0x4a3020, 0x6a4a30, 0x7d6448]) };
            line(c, hand + dir * 1.0, 7.0, &ramp, 0.0);
            // Parierstange.
            let n = Vec2::new(-dir.y, dir.x);
            for k in [-1.0, 1.0] {
                let p = hand + n * k;
                c.px(p.x.round() as i32, p.y.round() as i32, pal::gold().at(0.5));
            }
        }
        Weapon::EmberSword => {
            let ramp = Ramp::from_hex(&[0x6a2a12, 0xb34a1a, 0xf08a2c, 0xffd36a, 0xfff2c0]);
            line(c, hand + dir * 1.0, 8.5, &ramp, 0.9);
            let n = Vec2::new(-dir.y, dir.x);
            for k in [-1.5f32, -0.5, 0.5, 1.5] {
                let p = hand + n * k;
                c.px(p.x.round() as i32, p.y.round() as i32, pal::gold().at(0.7));
            }
        }
        Weapon::Spear | Weapon::Lance => {
            let len = if weapon == Weapon::Lance { 15.0 } else { 12.0 };
            let back = hand - dir * 4.0;
            line(c, back, len, &pal::wood(), 0.0);
            let tip = back + dir * len;
            for i in 0..3 {
                let p = tip + dir * i as f32;
                c.px(p.x.round() as i32, p.y.round() as i32, pal::steel().at(0.9 - i as f32 * 0.1));
            }
            if weapon == Weapon::Lance {
                let p = back + dir * (len * 0.7);
                c.px(p.x.round() as i32, p.y.round() as i32 + 1, pal::crimson().at(0.7));
                c.px(p.x.round() as i32 - 1, p.y.round() as i32 + 1, pal::crimson().at(0.5));
            }
        }
        Weapon::Axe => {
            line(c, hand - dir * 1.0, 6.0, &pal::wood(), 0.0);
            let head = hand + dir * 4.5;
            let n = Vec2::new(-dir.y, dir.x);
            for k in 0..3 {
                let p = head + n * (k as f32 - 1.0);
                c.px(p.x.round() as i32, p.y.round() as i32, pal::steel().at(0.75));
            }
        }
        Weapon::Club => {
            line(c, hand, 8.0, &pal::bone(), 0.0);
            let head = hand + dir * 8.0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    c.px(head.x.round() as i32 + dx, head.y.round() as i32 + dy, pal::bone().at(0.6 - dy as f32 * 0.1));
                }
            }
        }
        Weapon::Staff => {
            let top = hand + Vec2::new(0.0, -10.0);
            for y in (top.y as i32)..=(hand.y as i32 + 6) {
                c.px(hand.x.round() as i32, y, pal::dark_wood().at(0.6));
            }
            for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, -2), (-1, -1), (1, -1)] {
                let (x, y) = (top.x.round() as i32 + dx, top.y.round() as i32 + dy);
                c.px(x, y, if dy == -2 { pal::HOLLOW_GLOW } else { pal::HOLLOW_DEEP });
                c.glow(x, y, 0.8 + pose.cast * 0.2);
            }
        }
        Weapon::Bow | Weapon::BoneBow => {
            let ramp = if weapon == Weapon::Bow { pal::wood() } else { pal::bone() };
            // Bogen vertikal vor der Hand, Sehne zieht nach hinten.
            let pull = (pose.draw * 3.0).round() as i32;
            let (hx, hy) = (hand.x.round() as i32 + 1, hand.y.round() as i32);
            for dy in -5i32..=5 {
                let bend = if dy.abs() > 3 { 0 } else { 1 };
                c.px(hx + bend, hy + dy, ramp.at(0.6 - dy.abs() as f32 * 0.04));
            }
            for dy in -5i32..=5 {
                let sx = hx - (pull as f32 * (1.0 - dy.abs() as f32 / 5.0)).round() as i32;
                c.px(sx, hy + dy, Rgba::hex(0xb8b0a0).with_alpha(200));
            }
            if pose.draw > 0.2 {
                for i in 0..6 {
                    c.px(hx - pull + i, hy, pal::wood().at(0.7));
                }
                c.px(hx - pull + 6, hy, pal::steel().at(0.9));
            }
        }
    }
}

fn draw_shield(c: &mut Canvas, x: i32, y: i32, ramp: &Ramp) {
    for dy in 0..6 {
        let w = if dy == 5 { 1 } else if dy == 4 { 2 } else { 3 };
        for dx in -w + 1..w {
            let t = 0.6 - dx as f32 * 0.12 - dy as f32 * 0.04;
            c.pxn(x + dx, y + dy, ramp.dither(t, x + dx, y + dy), Vec3::new(dx as f32 * 0.25, -0.2, 0.9));
        }
    }
    // Rand und Wappen (goldene Flamme).
    c.px(x, y + 1, pal::gold().at(0.8));
    c.px(x, y + 2, pal::gold().at(0.6));
}

/// Zeichnet eine Fußfigur in eine Leinwand.
fn paint_figure(f: &Figure, pose: &Pose) -> Canvas {
    let mut c = Canvas::new(W, H);
    let cx = W / 2;
    let hip = FOOT - 5 + pose.bob;
    let shoulder = hip - 6;
    let head = shoulder - 4;
    if let Some(cape) = &f.cape {
        draw_cape(&mut c, cx, shoulder + 1, hip, pose, cape);
    }
    draw_legs(&mut c, cx, hip + 1, pose, f);
    draw_torso(&mut c, cx, shoulder, hip, f);
    draw_head(&mut c, cx, head, f);
    // Hinterer Arm.
    c.px(cx - f.width / 2 - 1, shoulder + 1, f.body.at(0.25));
    c.px(cx - f.width / 2 - 1, shoulder + 2, f.body.at(0.2));
    if pose.carry {
        // Sack mit Rohstoffen auf dem Rücken.
        for y in shoulder - 1..shoulder + 4 {
            for x in cx - f.width / 2 - 3..cx - f.width / 2 {
                c.pxn(x, y, pal::cloth_brown().dither(0.6 - (y - shoulder) as f32 * 0.05, x, y), Vec3::new(-0.5, -0.2, 0.8));
            }
        }
    }
    if let Some(sh) = &f.shield {
        draw_shield(&mut c, cx - 1, shoulder + 1, sh);
    }
    // Vorderer Arm + Waffe.
    let shoulder_pt = Vec2::new(cx as f32 + f.width as f32 * 0.5, shoulder as f32 + 1.0);
    let arm_dir = Vec2::new(pose.arm.cos(), pose.arm.sin());
    let hand = if matches!(f.weapon, Weapon::Bow | Weapon::BoneBow) {
        shoulder_pt + Vec2::new(2.0 + pose.reach, 1.0)
    } else if f.weapon == Weapon::Staff {
        shoulder_pt + Vec2::new(1.0, 2.0 - pose.cast * 3.0)
    } else {
        shoulder_pt + arm_dir * 2.5 + Vec2::new(pose.reach, 0.0)
    };
    for i in 0..3 {
        let p = shoulder_pt.lerp(hand, i as f32 / 2.0);
        c.pxn(p.x.round() as i32, p.y.round() as i32, f.body.at(0.55), Vec3::new(0.5, 0.0, 0.85));
    }
    c.px(hand.x.round() as i32, hand.y.round() as i32, f.skin.at(0.6));
    let weapon_angle = if matches!(f.weapon, Weapon::Spear | Weapon::Lance) { pose.arm * 0.4 } else { pose.arm - 0.6 };
    draw_weapon(&mut c, hand, weapon_angle, f.weapon, pose);
    if pose.cast > 0.0 {
        // Glutaura beim Zaubern.
        for i in 0..10 {
            let a = i as f32 / 10.0 * TAU + pose.cast * 3.0;
            let p = Vec2::new(cx as f32, (shoulder + 2) as f32) + Vec2::new(a.cos() * 7.0, a.sin() * 4.0);
            c.px(p.x.round() as i32, p.y.round() as i32, pal::EMBER_HOT);
            c.glow(p.x.round() as i32, p.y.round() as i32, 1.0);
        }
    }
    c
}

/// Pferd mit Reiter (Ritter).
fn paint_knight(pose: &Pose) -> Canvas {
    let (w, h) = (36, 34);
    let mut c = Canvas::new(w, h);
    let foot = h - 3;
    let cx = 17;
    let horse = pal::horse();
    let bob = pose.bob;
    let body_y = foot - 9 + bob;
    // Beine (4) mit Galopp-Phasen.
    for (i, lx) in [-6, -3, 3, 6].iter().enumerate() {
        let ph = pose.leg_phase * TAU + i as f32 * PI * 0.5;
        let off = if pose.walking { (ph.sin() * 2.0).round() as i32 } else { 0 };
        let lift = if pose.walking && ph.cos() > 0.3 { 1 } else { 0 };
        let shade = if i % 2 == 0 { 0.3 } else { 0.45 };
        for y in body_y + 3..=foot - lift {
            c.px(cx + lx + off, y, horse.at(shade));
        }
        c.px(cx + lx + off, foot - lift, Rgba::hex(0x101010));
    }
    // Körper.
    c.ellipse_shaded(cx as f32, body_y as f32 + 1.0, 9.0, 4.0, &horse, 0.05);
    // Schabracke (Karmesin mit Goldsaum).
    for x in cx - 8..=cx + 6 {
        for y in body_y..body_y + 5 {
            if c.opaque(x, y) && y > body_y {
                let gold_trim = y == body_y + 4;
                c.pxn(x, y, if gold_trim { pal::gold().at(0.6) } else { pal::crimson().dither(0.55 - (x - cx) as f32 * 0.02, x, y) }, Vec3::new((x - cx) as f32 * 0.05, 0.1, 0.95));
            }
        }
    }
    // Hals und Kopf.
    for i in 0..6 {
        let x = cx + 7 + i / 2;
        let y = body_y - i;
        c.px(x, y, horse.at(0.5));
        c.px(x + 1, y, horse.at(0.6));
    }
    for (dx, dy) in [(0, 0), (1, 0), (2, 0), (3, 1), (2, 1), (1, 1), (0, 1)] {
        c.px(cx + 10 + dx, body_y - 6 + dy, horse.at(0.55));
    }
    c.px(cx + 11, body_y - 6, Rgba::hex(0x0a0a0a));
    // Schweif.
    for i in 0..4 {
        c.px(cx - 9 - i / 2, body_y + i, horse.at(0.2));
    }
    // Reiter.
    let rider = Figure {
        skin: pal::skin(),
        body: pal::steel(),
        legs: pal::steel(),
        accent: Some(pal::crimson()),
        helmet: Helmet::Great,
        helmet_ramp: pal::steel(),
        weapon: Weapon::Lance,
        shield: None,
        cape: Some(pal::crimson()),
        width: 6,
        undead: false,
        hair: None,
    };
    let mut r = paint_figure(&rider, &Pose { arm: pose.arm, reach: pose.reach, ..Default::default() });
    // Beine des Reiters ausblenden (sitzt).
    for y in FOOT - 5..H {
        for x in 0..W {
            r.erase(x, y);
        }
    }
    c.blit(&r, cx - W / 2 - 1, body_y - FOOT + 5 - 2, false);
    c
}

/// Grabkoloss: massige, gebeugte Kreatur.
fn paint_brute(pose: &Pose) -> Canvas {
    let (w, h) = (34, 34);
    let mut c = Canvas::new(w, h);
    let foot = h - 3;
    let cx = 16;
    let rot = pal::rot();
    let swing = if pose.walking { ((pose.leg_phase * TAU).sin() * 2.0).round() as i32 } else { 0 };
    for (lx, s) in [(-4, 1), (3, -1)] {
        for y in foot - 7..=foot {
            for dx in 0..3 {
                c.px(cx + lx + dx + swing * s, y, rot.at(0.3 + dx as f32 * 0.08));
            }
        }
    }
    let by = foot - 13 + pose.bob;
    c.ellipse_shaded(cx as f32, by as f32, 9.5, 8.0, &rot, 0.0);
    // Nähte und Knochenstacheln.
    for i in 0..5 {
        let x = cx - 6 + i * 3;
        for k in 0..3 {
            c.px(x, by - 8 - k + (i % 2), pal::bone().at(0.7 - k as f32 * 0.1));
        }
    }
    for y in by - 4..by + 5 {
        c.px(cx - 2 + (y % 2), y, Rgba::hex(0x1a1d17));
    }
    // Kopf klein, tief sitzend.
    c.ellipse_shaded(cx as f32 + 6.0, by as f32 - 5.0, 3.0, 2.5, &pal::bone(), 0.0);
    c.px(cx + 7, by - 5, pal::HOLLOW_GLOW);
    c.glow(cx + 7, by - 5, 1.0);
    // Arm + Keule.
    let hand = Vec2::new(cx as f32 + 8.0, by as f32 + 2.0);
    for i in 0..4 {
        c.px(cx + 5 + i, by - 1 + i, rot.at(0.5));
        c.px(cx + 5 + i, by + i, rot.at(0.4));
    }
    draw_weapon(&mut c, hand, pose.arm - 0.3, Weapon::Club, pose);
    c
}

/// Der Fahle Vogt: schwebende Robengestalt mit Maske, Dornenkrone und Seelenstab.
fn paint_reeve(pose: &Pose) -> Canvas {
    let (w, h) = (36, 48);
    let mut c = Canvas::new(w, h);
    let cx = 17;
    let hover = (pose.leg_phase * TAU).sin().round() as i32 + pose.bob;
    let bottom = h - 4 + hover;
    let top = bottom - 26;
    let robe = pal::hollow_cloth();
    let pts = [
        Vec2::new(cx as f32 - 4.0, top as f32 + 6.0),
        Vec2::new(cx as f32 + 4.0, top as f32 + 6.0),
        Vec2::new(cx as f32 + 8.0, bottom as f32),
        Vec2::new(cx as f32 - 8.0, bottom as f32),
    ];
    c.polygon(&pts, |x, y| {
        // Zerfetzter Saum.
        if y > bottom - 3 && ashen_core::noise::rand2(x, 0, 7) > 0.55 && y > bottom - 1 - (x % 3) {
            return None;
        }
        let t = 0.55 - (x - cx) as f32 * 0.03 - (y - top) as f32 * 0.01;
        Some((robe.dither(t, x, y), Vec3::new((x - cx) as f32 * 0.08, 0.1, 0.95)))
    });
    // Seelenfäden im Gewand.
    for y in (top + 8..bottom - 2).step_by(3) {
        c.px(cx - 1 + (y % 4) / 2, y, pal::HOLLOW_DEEP);
        c.glow(cx - 1 + (y % 4) / 2, y, 0.5);
    }
    // Maske.
    for y in top..top + 6 {
        for x in cx - 3..cx + 3 {
            let t = 0.85 - (x - cx + 3) as f32 * 0.06;
            c.pxn(x, y, pal::bone().dither(t, x, y), Vec3::new((x - cx) as f32 * 0.25, -0.2, 0.9));
        }
    }
    for x in [cx - 2, cx + 1] {
        c.px(x, top + 2, pal::HOLLOW_GLOW);
        c.glow(x, top + 2, 1.0);
    }
    c.px(cx - 1, top + 4, Rgba::hex(0x101010));
    c.px(cx, top + 4, Rgba::hex(0x101010));
    // Dornenkrone.
    for (i, x) in (cx - 3..cx + 3).enumerate() {
        let hgt = if i % 2 == 0 { 3 } else { 1 };
        for k in 0..hgt {
            c.px(x, top - 1 - k, pal::bone().at(0.5 + k as f32 * 0.1));
        }
    }
    let hand = Vec2::new(cx as f32 + 6.0, top as f32 + 12.0);
    draw_weapon(&mut c, hand, 0.0, Weapon::Staff, pose);
    c
}

fn walk_pose(i: usize) -> Pose {
    Pose { leg_phase: i as f32 / 4.0, walking: true, bob: if i % 2 == 1 { -1 } else { 0 }, arm: -0.9, ..Default::default() }
}

fn attack_pose(i: usize, weapon: Weapon) -> Pose {
    match weapon {
        Weapon::Spear | Weapon::Lance => {
            let reach = [-2.0, -1.0, 3.0, 1.0][i];
            Pose { arm: 0.0, reach, ..Default::default() }
        }
        Weapon::Bow | Weapon::BoneBow => Pose { arm: 0.0, draw: [0.3, 0.8, 1.0, 0.0][i], ..Default::default() },
        Weapon::Staff => Pose { cast: [0.3, 0.7, 1.0, 0.4][i], arm: -0.5, ..Default::default() },
        _ => Pose { arm: [-2.3, -1.6, 0.4, 0.9][i], reach: [0.0, 0.0, 1.0, 0.5][i], ..Default::default() },
    }
}

fn death_frames(base: &Canvas, undead: bool, pivot: Vec2) -> Vec<Canvas> {
    let mut out = Vec::new();
    for i in 0..5 {
        let t = i as f32 / 4.0;
        let angle = -t * 1.45;
        let mut c = base.rotated(angle, pivot);
        if i >= 3 {
            // Lache bzw. Knochenstaub.
            let col = if undead { pal::bone().at(0.35) } else { Rgba::hex(0x3a0c0c) };
            for dx in -4..5 {
                let y = pivot.y as i32;
                if (dx + i) % 3 != 0 {
                    c.px(pivot.x as i32 + dx - 3, y, col);
                }
            }
        }
        out.push(c);
    }
    out
}

fn finish(mut c: Canvas) -> Canvas {
    c.outline(pal::OUTLINE);
    c.pillow_normals(0.9);
    c
}

fn add_figure_set(atlas: &mut AtlasBuilder, id: &str, f: &Figure, with_work: bool, with_cast: bool) {
    let anchor = Vec2::new((W / 2) as f32, (FOOT + 1) as f32);
    for i in 0..2 {
        let p = Pose { bob: if i == 1 { -1 } else { 0 }, arm: -0.9, ..Default::default() };
        atlas.add(format!("{id}_idle_{i}"), finish(paint_figure(f, &p)), anchor);
    }
    for i in 0..4 {
        atlas.add(format!("{id}_walk_{i}"), finish(paint_figure(f, &walk_pose(i))), anchor);
    }
    for i in 0..4 {
        atlas.add(format!("{id}_attack_{i}"), finish(paint_figure(f, &attack_pose(i, f.weapon))), anchor);
    }
    if with_work {
        let mut wf = f.clone();
        wf.weapon = Weapon::Axe;
        for i in 0..4 {
            let p = Pose { arm: [-2.4, -1.8, 0.3, 0.8][i], ..Default::default() };
            atlas.add(format!("{id}_work_{i}"), finish(paint_figure(&wf, &p)), anchor);
        }
        for i in 0..4 {
            let mut p = walk_pose(i);
            p.carry = true;
            atlas.add(format!("{id}_carry_{i}"), finish(paint_figure(f, &p)), anchor);
        }
    }
    if with_cast {
        for i in 0..4 {
            let p = Pose { cast: (i as f32 + 1.0) / 4.0, arm: -1.8, ..Default::default() };
            atlas.add(format!("{id}_cast_{i}"), finish(paint_figure(f, &p)), anchor);
        }
    }
    let base = paint_figure(f, &Pose { arm: 0.6, ..Default::default() });
    for (i, d) in death_frames(&base, f.undead, anchor).into_iter().enumerate() {
        atlas.add(format!("{id}_death_{i}"), finish(d), anchor);
    }
    // Porträt: Kopf- und Schulterausschnitt.
    let idle = finish(paint_figure(f, &Pose { arm: -0.9, ..Default::default() }));
    let mut portrait = Canvas::new(14, 14);
    let head_top = FOOT - 5 - 6 - 4;
    portrait.blit(&crop(&idle, W / 2 - 7, head_top - 4, 14, 14), 0, 0, false);
    atlas.add(format!("{id}_portrait"), portrait, Vec2::ZERO);
}

fn crop(c: &Canvas, x0: i32, y0: i32, w: i32, h: i32) -> Canvas {
    let mut out = Canvas::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let (sx, sy) = (x0 + x, y0 + y);
            if c.inside(sx, sy) {
                let i = (sy * c.w + sx) as usize;
                let o = (y * w + x) as usize;
                out.color[o] = c.color[i];
                out.normal[o] = c.normal[i];
            }
        }
    }
    out
}

fn add_custom_set(atlas: &mut AtlasBuilder, id: &str, size: (i32, i32), paint: &dyn Fn(&Pose) -> Canvas, weapon: Weapon, undead: bool) {
    let anchor = Vec2::new((size.0 / 2) as f32, (size.1 - 2) as f32);
    for i in 0..2 {
        let p = Pose { bob: if i == 1 { -1 } else { 0 }, arm: -0.9, leg_phase: i as f32 * 0.25, ..Default::default() };
        atlas.add(format!("{id}_idle_{i}"), finish(paint(&p)), anchor);
    }
    for i in 0..4 {
        atlas.add(format!("{id}_walk_{i}"), finish(paint(&walk_pose(i))), anchor);
    }
    for i in 0..4 {
        atlas.add(format!("{id}_attack_{i}"), finish(paint(&attack_pose(i, weapon))), anchor);
    }
    for i in 0..4 {
        let p = Pose { cast: (i as f32 + 1.0) / 4.0, arm: -1.5, ..Default::default() };
        atlas.add(format!("{id}_cast_{i}"), finish(paint(&p)), anchor);
    }
    let base = paint(&Pose { arm: 0.4, ..Default::default() });
    for (i, d) in death_frames(&base, undead, anchor).into_iter().enumerate() {
        atlas.add(format!("{id}_death_{i}"), finish(d), anchor);
    }
    let idle = finish(paint(&Pose { arm: -0.9, ..Default::default() }));
    // Porträt aus dem oberen Bereich der Figur.
    let mut top = size.1;
    for y in 0..size.1 {
        if (0..size.0).any(|x| idle.opaque(x, y)) {
            top = y;
            break;
        }
    }
    let p = crop(&idle, size.0 / 2 - 7 + if id == "knight" { 3 } else { 0 }, top - 1, 14, 14);
    atlas.add(format!("{id}_portrait"), p, Vec2::ZERO);
}

pub fn build(atlas: &mut AtlasBuilder) {
    let worker = Figure {
        skin: pal::skin(),
        body: pal::linen(),
        legs: pal::leather(),
        accent: None,
        helmet: Helmet::Cap,
        helmet_ramp: pal::cloth_green(),
        weapon: Weapon::None,
        shield: None,
        cape: None,
        width: 5,
        undead: false,
        hair: None,
    };
    add_figure_set(atlas, "worker", &worker, true, false);

    let swordsman = Figure {
        skin: pal::skin(),
        body: pal::steel(),
        legs: pal::iron_dark(),
        accent: Some(pal::crimson()),
        helmet: Helmet::Kettle,
        helmet_ramp: pal::steel(),
        weapon: Weapon::Sword,
        shield: Some(pal::crimson()),
        cape: None,
        width: 6,
        undead: false,
        hair: None,
    };
    add_figure_set(atlas, "swordsman", &swordsman, false, false);

    let spearman = Figure {
        skin: pal::skin(),
        body: pal::leather(),
        legs: pal::cloth_brown(),
        accent: Some(pal::crimson()),
        helmet: Helmet::Kettle,
        helmet_ramp: pal::iron_dark(),
        weapon: Weapon::Spear,
        shield: None,
        cape: None,
        width: 6,
        undead: false,
        hair: None,
    };
    add_figure_set(atlas, "spearman", &spearman, false, false);

    let archer = Figure {
        skin: pal::skin(),
        body: pal::cloth_green(),
        legs: pal::leather(),
        accent: None,
        helmet: Helmet::Hood,
        helmet_ramp: pal::cloth_green(),
        weapon: Weapon::Bow,
        shield: None,
        cape: Some(pal::cloth_green()),
        width: 5,
        undead: false,
        hair: None,
    };
    add_figure_set(atlas, "archer", &archer, false, false);

    let isolde = Figure {
        skin: pal::skin(),
        body: pal::steel(),
        legs: pal::steel(),
        accent: Some(pal::crimson()),
        helmet: Helmet::Circlet,
        helmet_ramp: pal::gold(),
        weapon: Weapon::EmberSword,
        shield: None,
        cape: Some(pal::crimson()),
        width: 6,
        undead: false,
        hair: Some(Rgba::hex(0x7a2a18)),
    };
    add_figure_set(atlas, "hero_isolde", &isolde, false, true);

    let thrall = Figure {
        skin: pal::bone(),
        body: pal::hollow_cloth(),
        legs: pal::bone(),
        accent: None,
        helmet: Helmet::Skull,
        helmet_ramp: pal::bone(),
        weapon: Weapon::RustBlade,
        shield: None,
        cape: None,
        width: 5,
        undead: true,
        hair: None,
    };
    add_figure_set(atlas, "hollow_thrall", &thrall, false, false);

    let hollow_archer = Figure { weapon: Weapon::BoneBow, helmet: Helmet::Hood, helmet_ramp: pal::hollow_cloth(), ..thrall.clone() };
    add_figure_set(atlas, "hollow_archer", &hollow_archer, false, false);

    add_custom_set(atlas, "knight", (36, 34), &paint_knight, Weapon::Lance, false);
    add_custom_set(atlas, "grave_brute", (34, 34), &paint_brute, Weapon::Club, true);
    add_custom_set(atlas, "boss_pale_reeve", (36, 48), &paint_reeve, Weapon::Staff, true);
}
