//! 3D-Modelle aller Einheiten der Vertical Slice.

use super::rig::{self, Anim, Body, Bones, Helm, Look, Weapon};
use ashen_engine::bake::{Mat, Mesh, Paint, Pose, Skeleton};
use glam::{Mat4, Vec3};
use std::f32::consts::TAU;

/// Ein animierbares Einheitenmodell.
pub struct UnitModel {
    pub id: &'static str,
    pub skeleton: Skeleton,
    pub mesh: Mesh,
    pub kind: Kind,
    /// Zellgröße in Pixeln und Höhe des Kopfes (für Porträts) in Welteinheiten.
    pub cell: (u32, u32),
    pub anims: Vec<Anim>,
}

pub enum Kind {
    Humanoid { bones: Bones, look: Look },
    Knight { horse: HorseBones, rider: Bones, look: Look },
}

#[derive(Clone, Copy)]
pub struct HorseBones {
    pub body: usize,
    pub neck: usize,
    pub head: usize,
    pub legs: [(usize, usize); 4],
    pub tail: usize,
}

fn base(height: f32) -> Look {
    Look {
        height,
        bulk: 1.0,
        skin: 0xad7c5c,
        torso: (0x47505a, Mat::Metal),
        arms: (0x47505a, Mat::Metal),
        legs: (0x2c2f34, Mat::Cloth),
        boots: 0x332216,
        tabard: None,
        trim: None,
        helm: Helm::None,
        helm_color: 0x657079,
        hair: None,
        cape: None,
        shield: None,
        weapon: Weapon::None,
        pauldrons: None,
        undead: false,
        hunch: 0.0,
        body: Body::Legs,
        pack: false,
    }
}

/// Einheiten-ID, Aussehen, Zellgröße und Animationen.
pub type UnitSpec = (&'static str, Look, (u32, u32), Vec<Anim>);

pub fn looks() -> Vec<UnitSpec> {
    let std_anims = vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death];
    let worker = Look {
        torso: (0x93804f, Mat::Cloth),
        arms: (0xad7c5c, Mat::Skin),
        legs: (0x5d4630, Mat::Cloth),
        helm: Helm::Cap,
        helm_color: 0x3b492d,
        hair: Some(0x4a3020),
        weapon: Weapon::Axe,
        ..base(1.6)
    };
    let swordsman = Look {
        torso: (0x657079, Mat::Metal),
        arms: (0x47505a, Mat::Metal),
        tabard: Some(0x8f2220),
        trim: Some(0xcfa23f),
        helm: Helm::Kettle,
        helm_color: 0x8b969e,
        shield: Some(0x6c1717),
        weapon: Weapon::Sword,
        pauldrons: Some(0x8b969e),
        ..base(1.72)
    };
    let spearman = Look {
        torso: (0x4a3220, Mat::Leather),
        arms: (0x5f432c, Mat::Leather),
        legs: (0x483625, Mat::Cloth),
        tabard: Some(0x7a1c1c),
        helm: Helm::Kettle,
        helm_color: 0x3c4046,
        weapon: Weapon::Spear,
        ..base(1.7)
    };
    let archer = Look {
        torso: (0x2d3823, Mat::Cloth),
        arms: (0x4a3220, Mat::Leather),
        legs: (0x34261a, Mat::Cloth),
        helm: Helm::Hood,
        helm_color: 0x2d3823,
        cape: Some(0x20281a),
        weapon: Weapon::Bow,
        ..base(1.68)
    };
    let isolde = Look {
        torso: (0x8b969e, Mat::Metal),
        arms: (0x657079, Mat::Metal),
        legs: (0x47505a, Mat::Metal),
        tabard: Some(0x8f2220),
        trim: Some(0xeccb6c),
        helm: Helm::Circlet,
        hair: Some(0x8a3a1e),
        cape: Some(0x7a1c1c),
        weapon: Weapon::EmberSword,
        pauldrons: Some(0xb4bec4),
        ..base(1.85)
    };
    let thrall = Look {
        skin: 0x8f8a6e,
        torso: (0x17191c, Mat::Cloth),
        arms: (0x8f8a6e, Mat::Bone),
        legs: (0x6b6752, Mat::Bone),
        boots: 0x2c3035,
        weapon: Weapon::RustBlade,
        undead: true,
        hunch: 10.0,
        ..base(1.7)
    };
    let hollow_archer = Look { helm: Helm::Hood, helm_color: 0x212428, weapon: Weapon::BoneBow, cape: Some(0x17191c), ..thrall.clone() };
    let brute = Look {
        skin: 0x485644,
        torso: (0x374233, Mat::Skin),
        arms: (0x485644, Mat::Skin),
        legs: (0x283024, Mat::Skin),
        boots: 0x1a1e18,
        weapon: Weapon::Club,
        undead: true,
        hunch: 28.0,
        bulk: 1.75,
        pauldrons: Some(0xb5ae8d),
        ..base(2.1)
    };
    let reeve = Look {
        skin: 0xd2cbaa,
        torso: (0x3a3f48, Mat::Cloth),
        arms: (0x2e323a, Mat::Cloth),
        legs: (0x343842, Mat::Cloth),
        tabard: Some(0x1f4a30),
        trim: Some(0x86f2a0),
        helm: Helm::SpikedCrown,
        weapon: Weapon::Staff,
        undead: true,
        body: Body::Robe,
        cape: Some(0x24272d),
        bulk: 1.2,
        ..base(2.5)
    };
    vec![
        ("worker", worker, (56, 64), vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death, Anim::Work, Anim::Carry]),
        ("swordsman", swordsman, (60, 66), std_anims.clone()),
        ("spearman", spearman, (76, 70), std_anims.clone()),
        ("archer", archer, (58, 66), std_anims.clone()),
        ("hero_isolde", isolde, (66, 74), vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death, Anim::Cast]),
        ("hollow_thrall", thrall, (60, 66), std_anims.clone()),
        ("hollow_archer", hollow_archer, (58, 66), std_anims.clone()),
        ("grave_brute", brute, (84, 84), std_anims.clone()),
        ("boss_pale_reeve", reeve, (92, 108), vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death, Anim::Cast]),
    ]
}

pub fn build_all() -> Vec<UnitModel> {
    let mut out: Vec<UnitModel> = looks()
        .into_iter()
        .map(|(id, look, cell, anims)| {
            let mut sk = Skeleton::default();
            let bones = rig::build_skeleton(&mut sk, &look, None, Vec3::ZERO);
            let mut mesh = rig::build_mesh(&look, &bones);
            mesh.verts.shrink_to_fit();
            UnitModel { id, skeleton: sk, mesh, kind: Kind::Humanoid { bones, look }, cell, anims }
        })
        .collect();
    out.push(knight());
    out
}

fn knight() -> UnitModel {
    let mut sk = Skeleton::default();
    let body = sk.add("horse_body", None, Vec3::new(0.0, 0.95, 0.0));
    let neck = sk.add("horse_neck", Some(body), Vec3::new(0.0, 0.12, 0.55));
    let head = sk.add("horse_head", Some(neck), Vec3::new(0.0, 0.42, 0.16));
    let mut legs = [(0, 0); 4];
    for (i, (x, z)) in [(-0.17f32, 0.46f32), (0.17, 0.46), (-0.17, -0.48), (0.17, -0.48)].iter().enumerate() {
        let up = sk.add("leg_up", Some(body), Vec3::new(*x, -0.14, *z));
        let low = sk.add("leg_low", Some(up), Vec3::new(0.0, -0.38, 0.0));
        legs[i] = (up, low);
    }
    let tail = sk.add("horse_tail", Some(body), Vec3::new(0.0, 0.1, -0.5));
    let look = Look {
        torso: (0x8b969e, Mat::Metal),
        arms: (0x657079, Mat::Metal),
        legs: (0x47505a, Mat::Metal),
        tabard: Some(0x8f2220),
        trim: Some(0xcfa23f),
        helm: Helm::Great,
        helm_color: 0x8b969e,
        cape: Some(0x7a1c1c),
        weapon: Weapon::Lance,
        shield: Some(0x6c1717),
        pauldrons: Some(0xb4bec4),
        ..base(1.45)
    };
    let rider = rig::build_skeleton(&mut sk, &look, Some(body), Vec3::new(0.0, 0.2 - 0.46 * 1.45, -0.04));
    let mut m = rig::build_mesh(&look, &rider);
    let hb = HorseBones { body, neck, head, legs, tail };
    let coat = |b| Paint::new(0x3d2a1c, Mat::Leather).bone(b);
    m.ellipsoid(&coat(body), Vec3::ZERO, Vec3::new(0.25, 0.27, 0.7), 10, 6);
    // Schabracke in Glutkronenrot mit Goldsaum.
    m.frustum(&Paint::new(0x8f2220, Mat::Cloth).bone(body), Vec3::new(0.0, 0.06, -0.42), Vec3::new(0.0, 0.06, 0.34), 0.29, 0.29, 10);
    m.frustum(&Paint::new(0xcfa23f, Mat::Gold).bone(body), Vec3::new(0.0, -0.16, -0.43), Vec3::new(0.0, -0.16, 0.35), 0.295, 0.295, 10);
    m.frustum(&coat(neck), Vec3::ZERO, Vec3::new(0.0, 0.42, 0.14), 0.14, 0.1, 7);
    m.frustum(&Paint::new(0x1a110c, Mat::Hair).bone(neck), Vec3::new(0.0, 0.05, -0.06), Vec3::new(0.0, 0.4, 0.05), 0.04, 0.03, 4);
    m.frustum(&coat(head), Vec3::ZERO, Vec3::new(0.0, -0.1, 0.38), 0.09, 0.06, 6);
    m.ellipsoid(&Paint::new(0x8b969e, Mat::Metal).bone(head), Vec3::new(0.0, 0.03, 0.12), Vec3::new(0.075, 0.06, 0.14), 6, 4);
    for ((up, low), _) in legs.iter().zip(0..) {
        m.frustum(&coat(*up), Vec3::ZERO, Vec3::new(0.0, -0.38, 0.0), 0.08, 0.055, 6);
        m.frustum(&coat(*low), Vec3::ZERO, Vec3::new(0.0, -0.36, 0.0), 0.05, 0.042, 6);
        m.frustum(&Paint::new(0x101010, Mat::Plain).bone(*low), Vec3::new(0.0, -0.34, 0.0), Vec3::new(0.0, -0.4, 0.0), 0.06, 0.06, 6);
    }
    m.frustum(&Paint::new(0x1a110c, Mat::Hair).bone(tail), Vec3::ZERO, Vec3::new(0.0, -0.45, -0.12), 0.05, 0.03, 4);
    UnitModel { id: "knight", skeleton: sk, mesh: m, kind: Kind::Knight { horse: hb, rider, look }, cell: (96, 92), anims: vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death] }
}

/// Pose eines Modells.
pub fn pose(model: &UnitModel, anim: Anim, t: f32) -> Pose {
    match &model.kind {
        Kind::Humanoid { bones, look } => {
            let r = rig::Rig { skeleton: model.skeleton.clone(), bones: *bones, look: look.clone() };
            rig::pose(&r, anim, t)
        }
        Kind::Knight { horse, rider, look } => {
            let r = rig::Rig { skeleton: model.skeleton.clone(), bones: *rider, look: look.clone() };
            // Reiterhaltung: Grundpose des Rigs (Idle/Angriff), Beine gespreizt.
            let rider_anim = match anim {
                Anim::Walk => Anim::Idle,
                other => other,
            };
            let mut ps = rig::pose(&r, if rider_anim == Anim::Death { Anim::Idle } else { rider_anim }, t);
            ps.euler(rider.thigh_l, -70.0, 0.0, -28.0);
            ps.euler(rider.thigh_r, -70.0, 0.0, 28.0);
            ps.euler(rider.shin_l, 60.0, 0.0, 0.0);
            ps.euler(rider.shin_r, 60.0, 0.0, 0.0);
            let ph = t * TAU;
            match anim {
                Anim::Walk => {
                    // Galopp: diagonal versetzte Beinpaare, Körper wippt.
                    for (i, (up, low)) in horse.legs.iter().enumerate() {
                        let off = [0.0, 0.5, 0.25, 0.75][i] * TAU;
                        let s = (ph + off).sin();
                        ps.euler(*up, s * 35.0, 0.0, 0.0);
                        ps.euler(*low, ((ph + off).cos()).max(0.0) * 50.0, 0.0, 0.0);
                    }
                    ps.shift(horse.body, Vec3::new(0.0, (ph * 2.0).sin() * 0.035, 0.0));
                    ps.euler(horse.body, (ph).sin() * 4.0, 0.0, 0.0);
                    ps.euler(horse.neck, (ph).cos() * 8.0, 0.0, 0.0);
                    ps.euler(horse.head, -(ph).cos() * 6.0, 0.0, 0.0);
                    ps.euler(horse.tail, 20.0 + (ph).sin() * 10.0, 0.0, 0.0);
                }
                Anim::Death => {
                    let f = (t * 1.4).min(1.0);
                    ps.euler(horse.body, 0.0, 0.0, 80.0 * f);
                    ps.shift(horse.body, Vec3::new(0.0, -0.45 * f, 0.0));
                    ps.euler(horse.neck, 30.0 * f, 0.0, 0.0);
                }
                _ => {
                    ps.euler(horse.neck, (ph).sin() * 4.0, 0.0, 0.0);
                    ps.euler(horse.tail, 10.0 + (ph).sin() * 6.0, 0.0, 0.0);
                    if anim == Anim::Attack {
                        let rear = (t * std::f32::consts::PI).sin();
                        ps.euler(horse.body, -rear * 8.0, 0.0, 0.0);
                    }
                }
            }
            let _ = Mat4::IDENTITY;
            ps
        }
    }
}

/// Weltposition des Kopfes in der Idle-Pose (für Porträtausschnitte).
pub fn head_position(model: &UnitModel) -> Vec3 {
    let ps = pose(model, Anim::Idle, 0.0);
    let w = model.skeleton.world(&ps);
    let head = match &model.kind {
        Kind::Humanoid { bones, .. } => bones.head,
        Kind::Knight { rider, .. } => rider.head,
    };
    let h = match &model.kind {
        Kind::Humanoid { look, .. } | Kind::Knight { look, .. } => look.height,
    };
    w[head].transform_point3(Vec3::new(0.0, 0.075 * h, 0.0))
}
