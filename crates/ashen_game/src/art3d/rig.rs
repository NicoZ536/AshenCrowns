//! Humanoides 3D-Rig: Skelett, modulare Low-Poly-Körperteile, Ausrüstung und prozedurale Animationen.
//!
//! Konventionen: 1 Einheit = 1 Kachel, Y nach oben, Blickrichtung +Z. Gliedmaßen hängen in Ruhe
//! entlang −Y. Negative X-Rotation schwingt einen Arm nach vorne.

use ashen_engine::bake::{Mat, Mesh, Paint, Pose, Skeleton};
use glam::{Mat4, Vec3};
use std::f32::consts::{PI, TAU};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Helm {
    None,
    Hood,
    Cap,
    Kettle,
    Great,
    Circlet,
    SpikedCrown,
    /// Spitzhut der Magier.
    PointedHat,
    /// Kettenhaube der Späher.
    Coif,
    /// Gehörnter Nordhelm.
    Horned,
    /// Priestermitra.
    Mitre,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Weapon {
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
    Crossbow,
    Halberd,
    /// Magierstab mit Feuerkugel.
    FireStaff,
    /// Priesterstab mit goldener Flamme.
    Scepter,
    Mace,
    GreatAxe,
    /// Krallen an den Händen (Ghule).
    Claws,
    Pick,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Body {
    /// Normale Figur mit Beinen.
    Legs,
    /// Bodenlange Robe (schwebend, zerfetzt).
    Robe,
    /// Langes Gewand lebender Figuren: Beine bleiben animiert, ein Rock verdeckt sie.
    Gown,
}

/// Aussehen und Proportionen einer Figur.
#[derive(Clone, Debug)]
pub struct Look {
    pub height: f32,
    pub bulk: f32,
    pub skin: u32,
    pub torso: (u32, Mat),
    pub arms: (u32, Mat),
    pub legs: (u32, Mat),
    pub boots: u32,
    pub tabard: Option<u32>,
    pub trim: Option<u32>,
    pub helm: Helm,
    pub helm_color: u32,
    pub hair: Option<u32>,
    pub cape: Option<u32>,
    pub shield: Option<u32>,
    pub weapon: Weapon,
    pub pauldrons: Option<u32>,
    pub undead: bool,
    pub hunch: f32,
    pub body: Body,
    pub pack: bool,
    /// Großer Turmschild statt kleinem Rundschild.
    pub big_shield: bool,
}

/// Knochenindizes des humanoiden Skeletts.
#[derive(Clone, Copy, Debug)]
pub struct Bones {
    pub root: usize,
    pub pelvis: usize,
    pub chest: usize,
    pub head: usize,
    pub arm_l: usize,
    pub fore_l: usize,
    pub hand_l: usize,
    pub arm_r: usize,
    pub fore_r: usize,
    pub hand_r: usize,
    pub thigh_l: usize,
    pub shin_l: usize,
    pub thigh_r: usize,
    pub shin_r: usize,
    pub cape: usize,
    pub weapon: usize,
    pub offhand: usize,
}

pub struct Rig {
    pub skeleton: Skeleton,
    pub bones: Bones,
    pub look: Look,
}

fn p(hex: u32, mat: Mat, bone: usize) -> Paint {
    Paint::new(hex, mat).bone(bone)
}

/// Baut ein humanoides Skelett; `parent` erlaubt das Einhängen (z. B. Reiter auf Pferd).
pub fn build_skeleton(sk: &mut Skeleton, look: &Look, parent: Option<usize>, origin: Vec3) -> Bones {
    let h = look.height;
    let leg = 0.46 * h;
    let root = sk.add("root", parent, origin);
    let pelvis = sk.add("pelvis", Some(root), Vec3::new(0.0, leg, 0.0));
    let chest = sk.add("chest", Some(pelvis), Vec3::new(0.0, 0.08 * h, 0.0));
    let head = sk.add("head", Some(chest), Vec3::new(0.0, 0.3 * h, 0.02 * h));
    let sw = 0.135 * h * look.bulk;
    let arm_l = sk.add("arm_l", Some(chest), Vec3::new(-sw, 0.26 * h, 0.0));
    let fore_l = sk.add("fore_l", Some(arm_l), Vec3::new(0.0, -0.16 * h, 0.0));
    let hand_l = sk.add("hand_l", Some(fore_l), Vec3::new(0.0, -0.15 * h, 0.0));
    let arm_r = sk.add("arm_r", Some(chest), Vec3::new(sw, 0.26 * h, 0.0));
    let fore_r = sk.add("fore_r", Some(arm_r), Vec3::new(0.0, -0.16 * h, 0.0));
    let hand_r = sk.add("hand_r", Some(fore_r), Vec3::new(0.0, -0.15 * h, 0.0));
    let hw = 0.06 * h * look.bulk;
    let thigh_l = sk.add("thigh_l", Some(pelvis), Vec3::new(-hw, -0.01 * h, 0.0));
    let shin_l = sk.add("shin_l", Some(thigh_l), Vec3::new(0.0, -0.23 * h, 0.0));
    let thigh_r = sk.add("thigh_r", Some(pelvis), Vec3::new(hw, -0.01 * h, 0.0));
    let shin_r = sk.add("shin_r", Some(thigh_r), Vec3::new(0.0, -0.23 * h, 0.0));
    let cape = sk.add("cape", Some(chest), Vec3::new(0.0, 0.27 * h, -0.07 * h * look.bulk));
    let weapon = sk.add("weapon", Some(hand_r), Vec3::new(0.0, -0.02 * h, 0.0));
    let offhand = sk.add("offhand", Some(hand_l), Vec3::new(0.0, -0.02 * h, 0.0));
    Bones { root, pelvis, chest, head, arm_l, fore_l, hand_l, arm_r, fore_r, hand_r, thigh_l, shin_l, thigh_r, shin_r, cape, weapon, offhand }
}

fn limb(m: &mut Mesh, paint: &Paint, len: f32, r0: f32, r1: f32) {
    m.frustum(paint, Vec3::ZERO, Vec3::new(0.0, -len, 0.0), r0, r1, 7);
    m.ellipsoid(paint, Vec3::ZERO, Vec3::splat(r0 * 1.02), 7, 4);
}

fn weapon_mesh(m: &mut Mesh, w: Weapon, bone: usize, h: f32) {
    let steel = p(0x8b969e, Mat::Metal, bone);
    let wood = p(0x5a3a22, Mat::WoodPlank, bone);
    let gold = p(0xcfa23f, Mat::Gold, bone);
    // Klingen zeigen in Ruhe nach vorne (+Z) und leicht nach oben.
    match w {
        Weapon::None => {}
        Weapon::Sword | Weapon::RustBlade | Weapon::EmberSword => {
            let blade = match w {
                Weapon::EmberSword => p(0xf08a2c, Mat::Glow, bone).glow(0.95),
                Weapon::RustBlade => p(0x7a5a3e, Mat::Metal, bone),
                _ => steel,
            };
            let len = if w == Weapon::EmberSword { 0.62 } else { 0.5 } * h / 1.75;
            m.bx(&p(0x3a261a, Mat::Leather, bone), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.022, 0.022, 0.06));
            m.bx(&gold, Vec3::new(0.0, 0.0, 0.07), Vec3::new(0.09, 0.02, 0.02));
            m.cuboid(&blade, Vec3::new(-0.025, -0.008, 0.08), Vec3::new(0.025, 0.008, 0.08 + len), Mat4::IDENTITY);
            m.frustum(&blade, Vec3::new(0.0, 0.0, 0.08 + len), Vec3::new(0.0, 0.0, 0.14 + len), 0.025, 0.0, 4);
        }
        Weapon::Spear | Weapon::Lance => {
            let len = if w == Weapon::Lance { 1.55 } else { 1.25 } * h / 1.75;
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.35), Vec3::new(0.0, 0.0, len), 0.022, 0.018, 5);
            m.frustum(&steel, Vec3::new(0.0, 0.0, len), Vec3::new(0.0, 0.0, len + 0.16), 0.04, 0.0, 4);
            if w == Weapon::Lance {
                m.frustum(&p(0x8f2220, Mat::Cloth, bone), Vec3::new(0.0, 0.0, len - 0.3), Vec3::new(0.0, 0.0, len - 0.1), 0.06, 0.02, 3);
            }
        }
        Weapon::Axe => {
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.08), Vec3::new(0.0, 0.0, 0.42), 0.02, 0.02, 5);
            m.cuboid(&steel, Vec3::new(-0.012, -0.02, 0.32), Vec3::new(0.012, 0.11, 0.44), Mat4::IDENTITY);
        }
        Weapon::Club => {
            let bone_p = p(0xb5ae8d, Mat::Bone, bone);
            m.frustum(&bone_p, Vec3::new(0.0, 0.0, -0.05), Vec3::new(0.0, 0.0, 0.75), 0.04, 0.09, 6);
            m.ellipsoid(&bone_p, Vec3::new(0.0, 0.0, 0.8), Vec3::splat(0.12), 6, 4);
        }
        Weapon::Staff => {
            m.frustum(&p(0x2c1c13, Mat::WoodPlank, bone), Vec3::new(0.0, 0.0, -0.9), Vec3::new(0.0, 0.0, 0.7), 0.025, 0.02, 5);
            m.ellipsoid(&p(0x86f2a0, Mat::Glow, bone).glow(1.0), Vec3::new(0.0, 0.0, 0.8), Vec3::splat(0.08), 6, 4);
            for i in 0..4 {
                let a = i as f32 / 4.0 * TAU;
                m.frustum(&p(0xb5ae8d, Mat::Bone, bone), Vec3::new(0.0, 0.0, 0.66), Vec3::new(a.cos() * 0.1, a.sin() * 0.1, 0.9), 0.015, 0.0, 3);
            }
        }
        Weapon::Crossbow => {
            // Armbrust waagerecht vor der Hand: Schaft, Bogen quer, Bolzen.
            m.bx(&wood, Vec3::new(0.0, 0.0, 0.2), Vec3::new(0.025, 0.03, 0.26));
            m.bx(&p(0x3a261a, Mat::Leather, bone), Vec3::new(0.0, 0.03, 0.1), Vec3::new(0.02, 0.015, 0.1));
            m.bx(&p(0x6e4a2e, Mat::WoodPlank, bone), Vec3::new(0.0, 0.0, 0.44), Vec3::new(0.19, 0.016, 0.02));
            for sx in [-1.0f32, 1.0] {
                m.bx(&steel, Vec3::new(sx * 0.19, 0.0, 0.43), Vec3::new(0.012, 0.012, 0.03));
            }
            m.frustum(&p(0xd8d0c0, Mat::Plain, bone), Vec3::new(0.0, 0.035, 0.1), Vec3::new(0.0, 0.035, 0.52), 0.01, 0.01, 4);
            m.frustum(&steel, Vec3::new(0.0, 0.035, 0.52), Vec3::new(0.0, 0.035, 0.58), 0.022, 0.0, 4);
        }
        Weapon::Halberd => {
            let len = 1.45 * h / 1.75;
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.4), Vec3::new(0.0, 0.0, len), 0.022, 0.02, 5);
            m.cuboid(&steel, Vec3::new(-0.012, -0.02, len - 0.32), Vec3::new(0.012, 0.15, len - 0.06), Mat4::IDENTITY);
            m.frustum(&steel, Vec3::new(0.0, 0.0, len), Vec3::new(0.0, 0.0, len + 0.22), 0.035, 0.0, 4);
            m.cuboid(&steel, Vec3::new(-0.01, -0.12, len - 0.16), Vec3::new(0.01, -0.02, len - 0.02), Mat4::IDENTITY);
        }
        Weapon::FireStaff | Weapon::Scepter => {
            let (orb, col) = if w == Weapon::FireStaff { (p(0xff8a2c, Mat::Glow, bone).glow(1.0), 0x3a1c10) } else { (p(0xffd35a, Mat::Glow, bone).glow(1.0), 0xe8dcc0) };
            m.frustum(&p(col, Mat::WoodPlank, bone), Vec3::new(0.0, 0.0, -0.9), Vec3::new(0.0, 0.0, 0.7), 0.025, 0.02, 5);
            m.ellipsoid(&orb, Vec3::new(0.0, 0.0, 0.82), Vec3::splat(0.085), 6, 4);
            for i in 0..4 {
                let a = i as f32 / 4.0 * TAU;
                m.frustum(&gold, Vec3::new(0.0, 0.0, 0.68), Vec3::new(a.cos() * 0.09, a.sin() * 0.09, 0.92), 0.014, 0.0, 3);
            }
            if w == Weapon::Scepter {
                m.frustum(&gold, Vec3::new(0.0, 0.0, 0.86), Vec3::new(0.0, 0.0, 1.0), 0.05, 0.0, 4);
            }
        }
        Weapon::Mace => {
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.05), Vec3::new(0.0, 0.0, 0.5), 0.022, 0.02, 5);
            m.ellipsoid(&p(0xcfa23f, Mat::Gold, bone), Vec3::new(0.0, 0.0, 0.58), Vec3::splat(0.09), 6, 4);
            for i in 0..6 {
                let a = i as f32 / 6.0 * TAU;
                m.frustum(&steel, Vec3::new(a.cos() * 0.08, a.sin() * 0.08, 0.58), Vec3::new(a.cos() * 0.13, a.sin() * 0.13, 0.58), 0.02, 0.0, 3);
            }
            m.ellipsoid(&p(0xf08a2c, Mat::Glow, bone).glow(0.9), Vec3::new(0.0, 0.0, 0.58), Vec3::splat(0.045), 5, 3);
        }
        Weapon::GreatAxe => {
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.25), Vec3::new(0.0, 0.0, 0.95), 0.028, 0.026, 5);
            m.cuboid(&steel, Vec3::new(-0.012, 0.02, 0.7), Vec3::new(0.012, 0.22, 0.92), Mat4::IDENTITY);
            m.cuboid(&steel, Vec3::new(-0.012, -0.22, 0.7), Vec3::new(0.012, -0.02, 0.92), Mat4::IDENTITY);
            m.frustum(&steel, Vec3::new(0.0, 0.0, 0.95), Vec3::new(0.0, 0.0, 1.08), 0.04, 0.0, 4);
        }
        Weapon::Claws => {
            let claw = p(0xd8d2b4, Mat::Bone, bone);
            for i in 0..3 {
                let x = (i as f32 - 1.0) * 0.035;
                m.frustum(&claw, Vec3::new(x, 0.0, 0.03), Vec3::new(x, -0.02, 0.2), 0.014, 0.0, 3);
            }
        }
        Weapon::Pick => {
            m.frustum(&wood, Vec3::new(0.0, 0.0, -0.08), Vec3::new(0.0, 0.0, 0.46), 0.02, 0.02, 5);
            m.cuboid(&steel, Vec3::new(-0.012, -0.16, 0.38), Vec3::new(0.012, 0.16, 0.44), Mat4::IDENTITY);
            m.frustum(&steel, Vec3::new(0.0, 0.16, 0.41), Vec3::new(0.0, 0.26, 0.41), 0.02, 0.0, 3);
            m.frustum(&steel, Vec3::new(0.0, -0.16, 0.41), Vec3::new(0.0, -0.26, 0.41), 0.02, 0.0, 3);
        }
        Weapon::Bow | Weapon::BoneBow => {}
    }
}

fn bow_mesh(m: &mut Mesh, w: Weapon, bone: usize, h: f32) {
    if !matches!(w, Weapon::Bow | Weapon::BoneBow) {
        return;
    }
    let mat = if w == Weapon::Bow { p(0x6e4a2e, Mat::WoodPlank, bone) } else { p(0xb5ae8d, Mat::Bone, bone) };
    // Bogen als gebogene Kette vertikal vor der Hand.
    let r = 0.42 * h / 1.75;
    let n = 8;
    for i in 0..n {
        let a0 = -0.9 + i as f32 / n as f32 * 1.8;
        let a1 = -0.9 + (i + 1) as f32 / n as f32 * 1.8;
        let q = |a: f32| Vec3::new(0.0, a.sin() * r, a.cos() * r * 0.35 - r * 0.25);
        m.frustum(&mat, q(a0), q(a1), 0.018, 0.018, 4);
    }
}

fn helm_mesh(m: &mut Mesh, look: &Look, bone: usize, hr: f32) {
    let c = look.helm_color;
    match look.helm {
        Helm::None => {}
        Helm::Hood => {
            m.ellipsoid(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.05, -hr * 0.12), Vec3::new(hr * 1.18, hr * 1.12, hr * 1.2), 8, 5);
            m.frustum(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.6, -hr * 0.6), Vec3::new(0.0, hr * 1.2, -hr * 1.6), hr * 0.45, 0.0, 5);
        }
        Helm::Cap => {
            m.ellipsoid(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.45, 0.0), Vec3::new(hr * 1.05, hr * 0.55, hr * 1.05), 8, 4);
        }
        Helm::Kettle => {
            m.ellipsoid(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 1.35, 0.0), Vec3::new(hr * 1.1, hr * 0.85, hr * 1.1), 8, 4);
            m.frustum(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 1.1, 0.0), Vec3::new(0.0, hr * 1.18, 0.0), hr * 1.75, hr * 1.1, 10);
        }
        Helm::Great => {
            m.frustum(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 0.15, 0.0), Vec3::new(0.0, hr * 2.0, 0.0), hr * 1.15, hr * 1.05, 8);
            m.bx(&p(0x0c0c0e, Mat::Plain, bone), Vec3::new(0.0, hr * 1.2, hr * 1.08), Vec3::new(hr * 0.75, hr * 0.08, hr * 0.06));
            m.frustum(&p(0x8f2220, Mat::Cloth, bone), Vec3::new(0.0, hr * 2.0, 0.0), Vec3::new(0.0, hr * 2.6, -hr * 0.6), hr * 0.3, hr * 0.1, 4);
        }
        Helm::Circlet => {
            m.frustum(&p(0xcfa23f, Mat::Gold, bone), Vec3::new(0.0, hr * 1.35, 0.0), Vec3::new(0.0, hr * 1.5, 0.0), hr * 1.08, hr * 1.06, 10);
            m.ellipsoid(&p(0xf08a2c, Mat::Glow, bone).glow(1.0), Vec3::new(0.0, hr * 1.45, hr * 1.06), Vec3::splat(hr * 0.16), 5, 3);
        }
        Helm::PointedHat => {
            m.frustum(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.5, 0.0), Vec3::new(0.0, hr * 1.55, 0.0), hr * 1.9, hr * 1.9, 10);
            m.frustum(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.5, 0.0), Vec3::new(hr * 0.1, hr * 3.5, -hr * 0.5), hr * 1.0, hr * 0.12, 8);
            m.frustum(&p(0xcfa23f, Mat::Gold, bone), Vec3::new(0.0, hr * 1.55, 0.0), Vec3::new(0.0, hr * 1.7, 0.0), hr * 1.12, hr * 1.1, 10);
        }
        Helm::Coif => {
            m.ellipsoid(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 1.05, -hr * 0.05), Vec3::new(hr * 1.12, hr * 1.15, hr * 1.12), 8, 5);
            m.bx(&p(0x17130f, Mat::Plain, bone), Vec3::new(0.0, hr * 1.0, hr * 0.9), Vec3::new(hr * 0.55, hr * 0.45, hr * 0.2));
        }
        Helm::Horned => {
            m.ellipsoid(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 1.3, 0.0), Vec3::new(hr * 1.12, hr * 0.9, hr * 1.12), 8, 4);
            m.frustum(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 1.0, 0.0), Vec3::new(0.0, hr * 1.2, 0.0), hr * 1.14, hr * 1.12, 10);
            m.bx(&p(c, Mat::Metal, bone), Vec3::new(0.0, hr * 0.85, hr * 1.05), Vec3::new(hr * 0.12, hr * 0.5, hr * 0.06));
            for sx in [-1.0f32, 1.0] {
                m.frustum(&p(0xe0d8c0, Mat::Bone, bone), Vec3::new(sx * hr * 1.0, hr * 1.5, 0.0), Vec3::new(sx * hr * 1.9, hr * 1.9, 0.0), hr * 0.3, hr * 0.12, 5);
                m.frustum(&p(0xe0d8c0, Mat::Bone, bone), Vec3::new(sx * hr * 1.9, hr * 1.9, 0.0), Vec3::new(sx * hr * 2.2, hr * 2.7, 0.0), hr * 0.12, 0.0, 5);
            }
        }
        Helm::Mitre => {
            m.frustum(&p(c, Mat::Cloth, bone), Vec3::new(0.0, hr * 1.45, 0.0), Vec3::new(0.0, hr * 2.4, 0.0), hr * 1.05, hr * 0.15, 6);
            m.frustum(&p(0xcfa23f, Mat::Gold, bone), Vec3::new(0.0, hr * 1.4, 0.0), Vec3::new(0.0, hr * 1.6, 0.0), hr * 1.1, hr * 1.08, 10);
        }
        Helm::SpikedCrown => {
            for i in 0..7 {
                let a = i as f32 / 7.0 * TAU;
                let base = Vec3::new(a.cos() * hr * 0.95, hr * 1.4, a.sin() * hr * 0.95);
                let len = if i % 2 == 0 { 0.55 } else { 0.32 };
                m.frustum(&p(0xd2cbaa, Mat::Bone, bone), base, base + Vec3::new(a.cos() * 0.15, len, a.sin() * 0.15) * hr * 2.0, hr * 0.14, 0.0, 4);
            }
        }
    }
}

/// Erzeugt das Mesh einer Figur für ein vorhandenes Skelett.
pub fn build_mesh(look: &Look, b: &Bones) -> Mesh {
    let mut m = Mesh::default();
    let h = look.height;
    let bulk = look.bulk;
    let skin = |bone| p(look.skin, if look.undead { Mat::Bone } else { Mat::Skin }, bone);
    // Becken und Torso.
    m.ellipsoid(&p(look.legs.0, look.legs.1, b.pelvis), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.12 * h * bulk, 0.07 * h, 0.08 * h * bulk), 8, 4);
    let torso = p(look.torso.0, look.torso.1, b.chest);
    m.frustum(&torso, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.27 * h, 0.0), 0.1 * h * bulk, 0.145 * h * bulk, 8);
    m.ellipsoid(&torso, Vec3::new(0.0, 0.25 * h, 0.0), Vec3::new(0.16 * h * bulk, 0.06 * h, 0.1 * h * bulk), 8, 4);
    if look.undead {
        // Rippen als helle Querstreben.
        for i in 0..3 {
            let y = 0.08 * h + i as f32 * 0.055 * h;
            m.bx(&p(0xc9c6ad, Mat::Bone, b.chest), Vec3::new(0.0, y, 0.105 * h * bulk), Vec3::new(0.09 * h * bulk, 0.012 * h, 0.012));
        }
    }
    if let Some(t) = look.tabard {
        let tab = p(t, Mat::Cloth, b.chest);
        m.cuboid(&tab, Vec3::new(-0.07 * h * bulk, -0.12 * h, 0.105 * h * bulk), Vec3::new(0.07 * h * bulk, 0.22 * h, 0.125 * h * bulk), Mat4::IDENTITY);
        if let Some(tr) = look.trim {
            m.cuboid(&p(tr, Mat::Gold, b.chest), Vec3::new(-0.02 * h, 0.06 * h, 0.124 * h * bulk), Vec3::new(0.02 * h, 0.14 * h, 0.13 * h * bulk), Mat4::IDENTITY);
        }
    }
    // Gürtel.
    m.frustum(&p(0x2e1d13, Mat::Leather, b.chest), Vec3::new(0.0, -0.005 * h, 0.0), Vec3::new(0.0, 0.03 * h, 0.0), 0.108 * h * bulk, 0.11 * h * bulk, 8);
    if let Some(pc) = look.pauldrons {
        for (x, bone) in [(-1.0, b.arm_l), (1.0, b.arm_r)] {
            let _ = x;
            m.ellipsoid(&p(pc, Mat::Metal, bone), Vec3::new(0.0, 0.01 * h, 0.0), Vec3::new(0.075 * h * bulk, 0.05 * h, 0.07 * h * bulk), 7, 4);
        }
    }
    if look.pack {
        m.ellipsoid(&p(0x5d4630, Mat::Cloth, b.chest), Vec3::new(0.0, 0.16 * h, -0.14 * h), Vec3::new(0.09 * h, 0.1 * h, 0.06 * h), 7, 4);
    }
    // Kopf.
    let hr = 0.075 * h;
    if look.undead {
        m.ellipsoid(&p(0xc9c6ad, Mat::Bone, b.head), Vec3::new(0.0, hr, 0.0), Vec3::new(hr * 0.95, hr * 1.05, hr), 8, 5);
        for x in [-0.4f32, 0.4] {
            m.ellipsoid(&p(0x86f2a0, Mat::Glow, b.head).glow(1.0), Vec3::new(x * hr, hr * 1.05, hr * 0.85), Vec3::splat(hr * 0.2), 4, 3);
        }
        m.bx(&p(0x2e2c24, Mat::Plain, b.head), Vec3::new(0.0, hr * 0.45, hr * 0.85), Vec3::new(hr * 0.45, hr * 0.08, hr * 0.12));
    } else {
        m.ellipsoid(&skin(b.head), Vec3::new(0.0, hr, 0.0), Vec3::new(hr, hr * 1.08, hr), 8, 5);
        for x in [-0.38f32, 0.38] {
            m.bx(&p(0x1a1210, Mat::Plain, b.head), Vec3::new(x * hr, hr * 1.1, hr * 0.94), Vec3::new(hr * 0.12, hr * 0.1, hr * 0.06));
        }
        if let Some(hair) = look.hair {
            m.ellipsoid(&p(hair, Mat::Hair, b.head), Vec3::new(0.0, hr * 1.3, -hr * 0.2), Vec3::new(hr * 1.08, hr * 0.85, hr * 1.0), 8, 4);
            m.frustum(&p(hair, Mat::Hair, b.head), Vec3::new(0.0, hr * 1.1, -hr * 0.8), Vec3::new(0.0, -hr * 1.6, -hr * 1.1), hr * 0.55, hr * 0.2, 5);
        }
    }
    helm_mesh(&mut m, look, b.head, hr);
    // Arme.
    let arm = |bone| p(look.arms.0, look.arms.1, bone);
    for (a, f, hd) in [(b.arm_l, b.fore_l, b.hand_l), (b.arm_r, b.fore_r, b.hand_r)] {
        limb(&mut m, &arm(a), 0.16 * h, 0.045 * h * bulk, 0.038 * h * bulk);
        limb(&mut m, &arm(f), 0.15 * h, 0.038 * h * bulk, 0.032 * h * bulk);
        m.ellipsoid(&skin(hd), Vec3::new(0.0, -0.005 * h, 0.0), Vec3::splat(0.03 * h * bulk), 6, 4);
    }
    // Beine bzw. Robe.
    match look.body {
        Body::Legs => {
            let leg = |bone| p(look.legs.0, look.legs.1, bone);
            for (t, s) in [(b.thigh_l, b.shin_l), (b.thigh_r, b.shin_r)] {
                limb(&mut m, &leg(t), 0.23 * h, 0.055 * h * bulk, 0.045 * h * bulk);
                limb(&mut m, &p(look.boots, Mat::Leather, s), 0.22 * h, 0.045 * h * bulk, 0.04 * h * bulk);
                m.cuboid(&p(look.boots, Mat::Leather, s), Vec3::new(-0.035 * h, -0.235 * h, -0.03 * h), Vec3::new(0.035 * h, -0.2 * h, 0.08 * h), Mat4::IDENTITY);
            }
        }
        Body::Gown => {
            let leg = |bone| p(look.legs.0, look.legs.1, bone);
            for (t, s) in [(b.thigh_l, b.shin_l), (b.thigh_r, b.shin_r)] {
                limb(&mut m, &leg(t), 0.23 * h, 0.055 * h * bulk, 0.045 * h * bulk);
                limb(&mut m, &p(look.boots, Mat::Leather, s), 0.22 * h, 0.045 * h * bulk, 0.04 * h * bulk);
            }
            let robe = p(look.legs.0, look.legs.1, b.pelvis);
            m.frustum(&robe, Vec3::new(0.0, 0.04 * h, 0.0), Vec3::new(0.0, -0.4 * h, 0.0), 0.125 * h * bulk, 0.2 * h * bulk, 10);
            if let Some(t) = look.trim {
                m.frustum(&p(t, Mat::Gold, b.pelvis), Vec3::new(0.0, -0.4 * h, 0.0), Vec3::new(0.0, -0.37 * h, 0.0), 0.2 * h * bulk, 0.195 * h * bulk, 10);
            }
        }
        Body::Robe => {
            let robe = p(look.legs.0, look.legs.1, b.pelvis);
            m.frustum(&robe, Vec3::new(0.0, 0.05 * h, 0.0), Vec3::new(0.0, -0.5 * h, 0.0), 0.13 * h * bulk, 0.24 * h * bulk, 9);
            // Zerfetzter Saum.
            for i in 0..9 {
                let a = i as f32 / 9.0 * TAU;
                let base = Vec3::new(a.cos() * 0.23 * h * bulk, -0.48 * h, a.sin() * 0.23 * h * bulk);
                m.frustum(&robe, base, base + Vec3::new(a.cos() * 0.03, -0.08 * h * (0.6 + (i % 3) as f32 * 0.3), a.sin() * 0.03), 0.04 * h, 0.0, 3);
            }
        }
    }
    if let Some(c) = look.cape {
        let cape = p(c, Mat::Cloth, b.cape);
        let len = 0.52 * h;
        let w = 0.15 * h * bulk;
        m.cuboid(&cape, Vec3::new(-w, -len, -0.02), Vec3::new(w, 0.0, 0.0), Mat4::from_rotation_x(0.12));
    }
    if let Some(s) = look.shield {
        let sh = p(s, Mat::WoodPlank, b.fore_l);
        let t = Mat4::from_translation(Vec3::new(-0.05 * h, -0.09 * h, 0.03 * h)) * Mat4::from_rotation_y(-0.35);
        if look.big_shield {
            // Turmschild: hoch und breit, mit Metallrand und Buckel.
            let t = Mat4::from_translation(Vec3::new(-0.07 * h, -0.07 * h, 0.05 * h)) * Mat4::from_rotation_y(-0.25);
            m.cuboid(&sh, Vec3::new(-0.02, -0.26 * h, -0.13 * h), Vec3::new(0.02, 0.12 * h, 0.13 * h), t);
            let rim = p(0x8b969e, Mat::Metal, b.fore_l);
            m.cuboid(&rim, Vec3::new(-0.03, -0.27 * h, -0.14 * h), Vec3::new(0.03, -0.25 * h, 0.14 * h), t);
            m.cuboid(&rim, Vec3::new(-0.03, 0.11 * h, -0.14 * h), Vec3::new(0.03, 0.13 * h, 0.14 * h), t);
            m.cuboid(&p(0xcfa23f, Mat::Gold, b.fore_l), Vec3::new(-0.04, -0.1 * h, -0.04 * h), Vec3::new(-0.025, 0.0, 0.04 * h), t);
        } else {
            m.cuboid(&sh, Vec3::new(-0.015, -0.13 * h, -0.1 * h), Vec3::new(0.015, 0.1 * h, 0.1 * h), t);
            m.cuboid(&p(0xcfa23f, Mat::Gold, b.fore_l), Vec3::new(-0.025, -0.04 * h, -0.02 * h), Vec3::new(-0.01, 0.04 * h, 0.02 * h), t);
            m.cuboid(&p(0x8b969e, Mat::Metal, b.fore_l), Vec3::new(-0.02, -0.135 * h, -0.105 * h), Vec3::new(0.02, -0.12 * h, 0.105 * h), t);
        }
    }
    weapon_mesh(&mut m, look.weapon, b.weapon, h);
    bow_mesh(&mut m, look.weapon, b.offhand, h);
    m
}

/// Animationsarten der Bake-Pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anim {
    Idle,
    Walk,
    Attack,
    Work,
    Cast,
    Death,
    Carry,
}

impl Anim {
    pub fn name(self) -> &'static str {
        match self {
            Anim::Idle => "idle",
            Anim::Walk => "walk",
            Anim::Attack => "attack",
            Anim::Work => "work",
            Anim::Cast => "cast",
            Anim::Death => "death",
            Anim::Carry => "carry",
        }
    }

    pub fn frames(self) -> usize {
        match self {
            Anim::Idle => 4,
            Anim::Walk | Anim::Carry => 8,
            Anim::Attack | Anim::Work | Anim::Cast | Anim::Death => 6,
        }
    }

    pub fn fps(self) -> f32 {
        match self {
            Anim::Idle => 4.0,
            Anim::Walk | Anim::Carry => 12.0,
            Anim::Attack => 12.0,
            Anim::Work => 9.0,
            Anim::Cast => 10.0,
            Anim::Death => 10.0,
        }
    }

    pub fn looped(self) -> bool {
        matches!(self, Anim::Idle | Anim::Walk | Anim::Carry | Anim::Work)
    }
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Interpoliert stückweise zwischen Schlüsselwerten über t ∈ [0,1].
fn keys(t: f32, k: &[f32]) -> f32 {
    let n = k.len() - 1;
    let x = (t.clamp(0.0, 1.0) * n as f32).min(n as f32 - 1e-4);
    let i = x.floor() as usize;
    let f = smooth(x - i as f32);
    k[i] + (k[i + 1] - k[i]) * f
}

/// Grundhaltung: Waffe bereit, Arme leicht angewinkelt.
fn ready(pose: &mut Pose, b: &Bones, look: &Look) {
    pose.euler(b.chest, look.hunch, 0.0, 0.0);
    pose.euler(b.head, -look.hunch * 0.6, 0.0, 0.0);
    match look.weapon {
        Weapon::Crossbow => {
            pose.euler(b.arm_r, -45.0, 0.0, 10.0);
            pose.euler(b.fore_r, -55.0, 0.0, 0.0);
            pose.euler(b.weapon, 40.0, 0.0, 0.0);
            pose.euler(b.arm_l, -50.0, 0.0, -14.0);
            pose.euler(b.fore_l, -55.0, 0.0, 0.0);
        }
        Weapon::Spear | Weapon::Lance | Weapon::Halberd => {
            pose.euler(b.arm_r, -25.0, 0.0, 8.0);
            pose.euler(b.fore_r, -60.0, 0.0, 0.0);
            pose.euler(b.weapon, 60.0, 0.0, 0.0);
            pose.euler(b.arm_l, -35.0, 0.0, -10.0);
            pose.euler(b.fore_l, -50.0, 0.0, 0.0);
        }
        Weapon::Bow | Weapon::BoneBow => {
            pose.euler(b.arm_l, -55.0, 0.0, -8.0);
            pose.euler(b.fore_l, -20.0, 0.0, 0.0);
            pose.euler(b.offhand, 35.0, 0.0, 0.0);
            pose.euler(b.arm_r, -20.0, 0.0, 10.0);
            pose.euler(b.fore_r, -40.0, 0.0, 0.0);
        }
        Weapon::Staff | Weapon::FireStaff | Weapon::Scepter => {
            pose.euler(b.arm_r, -25.0, 0.0, 10.0);
            pose.euler(b.fore_r, -45.0, 0.0, 0.0);
            pose.euler(b.weapon, -10.0, 0.0, 0.0);
            pose.euler(b.weapon, 60.0, 0.0, 0.0);
            pose.euler(b.arm_l, -15.0, 0.0, -15.0);
        }
        Weapon::None => {
            pose.euler(b.arm_l, -5.0, 0.0, -8.0);
            pose.euler(b.arm_r, -5.0, 0.0, 8.0);
            pose.euler(b.fore_l, -15.0, 0.0, 0.0);
            pose.euler(b.fore_r, -15.0, 0.0, 0.0);
        }
        _ => {
            pose.euler(b.arm_r, -20.0, 0.0, 12.0);
            pose.euler(b.fore_r, -55.0, 0.0, 0.0);
            pose.euler(b.weapon, 25.0, 0.0, 0.0);
            pose.euler(b.arm_l, -15.0, 0.0, -12.0);
            pose.euler(b.fore_l, -45.0, 0.0, 0.0);
        }
    }
}

/// Pose einer Animation zum Zeitpunkt t ∈ [0,1).
pub fn pose(rig: &Rig, anim: Anim, t: f32) -> Pose {
    let b = &rig.bones;
    let look = &rig.look;
    let mut ps = rig.skeleton.rest();
    let ph = t * TAU;
    let robe = look.body == Body::Robe;
    match anim {
        Anim::Idle => {
            ready(&mut ps, b, look);
            let breath = (ph).sin();
            ps.shift(b.chest, Vec3::new(0.0, breath * 0.008 * look.height, 0.0));
            ps.euler(b.arm_l, breath * 2.0, 0.0, 0.0);
            ps.euler(b.head, 0.0, breath * 4.0, 0.0);
            ps.euler(b.cape, breath * 3.0 + 6.0, 0.0, 0.0);
            if robe {
                ps.shift(b.root, Vec3::new(0.0, 0.06 + breath * 0.03, 0.0));
            }
        }
        Anim::Walk | Anim::Carry => {
            let s = ph.sin();
            let c = ph.cos();
            if anim == Anim::Carry {
                ps.euler(b.arm_l, -40.0, 0.0, -20.0);
                ps.euler(b.arm_r, -40.0, 0.0, 20.0);
                ps.euler(b.fore_l, -70.0, 0.0, 0.0);
                ps.euler(b.fore_r, -70.0, 0.0, 0.0);
                ps.euler(b.chest, 8.0, 0.0, 0.0);
            } else {
                ready(&mut ps, b, look);
                ps.euler(b.arm_l, s * 28.0, 0.0, 0.0);
                if matches!(look.weapon, Weapon::None) {
                    ps.euler(b.arm_r, -s * 28.0, 0.0, 0.0);
                }
            }
            if robe {
                ps.shift(b.root, Vec3::new(0.0, 0.06 + (ph * 2.0).sin() * 0.025, 0.0));
                ps.euler(b.pelvis, 6.0 + s * 3.0, 0.0, 0.0);
            } else {
                ps.euler(b.thigh_l, -s * 32.0, 0.0, 0.0);
                ps.euler(b.thigh_r, s * 32.0, 0.0, 0.0);
                ps.euler(b.shin_l, (c.max(0.0)) * 45.0 + 5.0, 0.0, 0.0);
                ps.euler(b.shin_r, ((-c).max(0.0)) * 45.0 + 5.0, 0.0, 0.0);
                ps.shift(b.pelvis, Vec3::new(0.0, -(s * s) * 0.03 * look.height + 0.01, 0.0));
            }
            ps.euler(b.chest, 5.0, s * 6.0, 0.0);
            ps.euler(b.cape, 18.0 + c * 8.0, 0.0, 0.0);
        }
        Anim::Attack => {
            match look.weapon {
                Weapon::Crossbow => {
                    ready(&mut ps, b, look);
                    // Zielen, Abdruck bei Bild 3, Rückstoß.
                    let kick = keys(t, &[0.0, 0.0, 0.0, 1.0, 0.2, 0.0]);
                    ps.euler(b.arm_r, -50.0 + kick * 8.0, 0.0, 8.0);
                    ps.euler(b.arm_l, -55.0 + kick * 6.0, 0.0, -14.0);
                    ps.euler(b.chest, look.hunch - kick * 6.0, 8.0, 0.0);
                    ps.euler(b.head, kick * 4.0, 0.0, 0.0);
                }
                Weapon::Spear | Weapon::Lance | Weapon::Halberd => {
                    ready(&mut ps, b, look);
                    let thrust = keys(t, &[0.0, -0.4, 1.0, 0.6, 0.0]);
                    ps.euler(b.arm_r, -thrust * 40.0, 0.0, 0.0);
                    ps.euler(b.fore_r, thrust * 45.0, 0.0, 0.0);
                    ps.euler(b.chest, 0.0, -thrust * 18.0, 0.0);
                    ps.shift(b.pelvis, Vec3::new(0.0, 0.0, thrust * 0.08));
                    ps.euler(b.thigh_l, -thrust * 25.0, 0.0, 0.0);
                    ps.euler(b.thigh_r, thrust * 15.0, 0.0, 0.0);
                }
                Weapon::Bow | Weapon::BoneBow => {
                    ready(&mut ps, b, look);
                    let draw = keys(t, &[0.2, 0.7, 1.0, 1.0, 0.0, 0.0]);
                    ps.euler(b.arm_l, -35.0, 0.0, 0.0);
                    ps.euler(b.arm_r, -70.0 + draw * 10.0, -draw * 30.0, 0.0);
                    ps.euler(b.fore_r, -90.0 * draw, 0.0, 0.0);
                    ps.euler(b.chest, 0.0, 25.0, 0.0);
                }
                Weapon::Staff | Weapon::FireStaff | Weapon::Scepter => {
                    let raise = keys(t, &[0.0, 1.0, 1.0, 0.3, 0.0]);
                    ps.euler(b.arm_r, -60.0 - raise * 80.0, 0.0, 10.0);
                    ps.euler(b.fore_r, -20.0, 0.0, 0.0);
                    ps.euler(b.weapon, 60.0 - raise * 40.0, 0.0, 0.0);
                    ps.euler(b.arm_l, -40.0 - raise * 50.0, 0.0, -25.0);
                    ps.shift(b.root, Vec3::new(0.0, 0.06 + raise * 0.05, 0.0));
                }
                _ => {
                    // Hieb: ausholen, schlagen, durchziehen.
                    let arm = keys(t, &[-20.0, -150.0, -165.0, -30.0, 15.0, -20.0]);
                    let twist = keys(t, &[0.0, 25.0, 28.0, -30.0, -20.0, 0.0]);
                    let fore = keys(t, &[-55.0, -40.0, -30.0, -10.0, -20.0, -55.0]);
                    ps.euler(b.chest, look.hunch + 6.0, twist, 0.0);
                    ps.euler(b.arm_r, arm, 0.0, 12.0);
                    ps.euler(b.fore_r, fore, 0.0, 0.0);
                    ps.euler(b.weapon, 30.0, 0.0, 0.0);
                    ps.euler(b.arm_l, -20.0 - twist * 0.5, 0.0, -14.0);
                    ps.euler(b.fore_l, -45.0, 0.0, 0.0);
                    let lunge = keys(t, &[0.0, 0.0, 0.2, 1.0, 0.6, 0.0]);
                    ps.euler(b.thigh_l, -lunge * 28.0, 0.0, 0.0);
                    ps.euler(b.shin_l, lunge * 20.0, 0.0, 0.0);
                    ps.euler(b.thigh_r, lunge * 18.0, 0.0, 0.0);
                    ps.shift(b.pelvis, Vec3::new(0.0, -lunge * 0.03, lunge * 0.06));
                }
            }
            if robe {
                ps.shift(b.root, Vec3::new(0.0, 0.06, 0.0));
            }
        }
        Anim::Work => {
            let swing = keys(t, &[-160.0, -170.0, -40.0, -10.0, -90.0, -160.0]);
            ps.euler(b.chest, 18.0 * keys(t, &[0.0, 0.0, 1.0, 1.0, 0.4, 0.0]), 0.0, 0.0);
            ps.euler(b.arm_r, swing, 0.0, 6.0);
            ps.euler(b.arm_l, swing, 0.0, -6.0);
            ps.euler(b.fore_r, -20.0, 0.0, 0.0);
            ps.euler(b.fore_l, -20.0, 0.0, 0.0);
            ps.euler(b.weapon, 20.0, 0.0, 0.0);
            ps.euler(b.thigh_l, -12.0, 0.0, 0.0);
            ps.euler(b.thigh_r, 8.0, 0.0, 0.0);
        }
        Anim::Cast => {
            ready(&mut ps, b, look);
            let r = keys(t, &[0.0, 0.7, 1.0, 1.0, 0.4, 0.0]);
            ps.euler(b.arm_l, -r * 120.0, 0.0, -r * 30.0);
            ps.euler(b.arm_r, -r * 140.0, 0.0, r * 20.0);
            ps.euler(b.chest, -r * 10.0, 0.0, 0.0);
            ps.euler(b.head, -r * 15.0, 0.0, 0.0);
            ps.euler(b.cape, 25.0 * r, 0.0, 0.0);
            if robe {
                ps.shift(b.root, Vec3::new(0.0, 0.06 + r * 0.08, 0.0));
            }
        }
        Anim::Death => {
            let f = keys(t, &[0.0, 0.25, 0.65, 0.95, 1.0, 1.0]);
            // Nach hinten fallen, um die Fersen kippen.
            ps.euler(b.root, 82.0 * f, 0.0, 0.0);
            ps.shift(b.root, Vec3::new(0.0, 0.02 * f, -0.05 * f));
            ps.euler(b.arm_l, -60.0 * f, 0.0, -40.0 * f);
            ps.euler(b.arm_r, -40.0 * f, 0.0, 50.0 * f);
            ps.euler(b.thigh_l, -20.0 * f, 0.0, 0.0);
            ps.euler(b.shin_l, 40.0 * f, 0.0, 0.0);
            ps.euler(b.head, 25.0 * f, 20.0 * f, 0.0);
            ps.euler(b.weapon, 60.0 * f, 0.0, 30.0 * f);
            if robe {
                ps.shift(b.root, Vec3::new(0.0, 0.06 * (1.0 - f), 0.0));
            }
        }
    }
    let _ = PI;
    ps
}
