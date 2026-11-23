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
    Humanoid {
        bones: Bones,
        look: Look,
    },
    /// Reiter auf Pferd (Ritter, Spähreiter, Berittene Bogenschützen).
    Knight {
        horse: HorseBones,
        rider: Bones,
        look: Look,
    },
    /// Belagerungsmaschine (Rammbock, Katapult).
    Machine {
        bones: MachineBones,
    },
    /// Fledermausschwarm.
    Swarm {
        bones: SwarmBones,
    },
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
        big_shield: false,
    }
}

/// Einheiten-ID, Aussehen, Zellgröße und Animationen.
pub type UnitSpec = (&'static str, Look, (u32, u32), Vec<Anim>);

pub fn looks() -> Vec<UnitSpec> {
    let std_anims = vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death];
    let cast_anims = vec![
        Anim::Idle,
        Anim::Walk,
        Anim::Attack,
        Anim::Death,
        Anim::Cast,
    ];
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
    let crossbowman = Look {
        torso: (0x4a3d2a, Mat::Leather),
        arms: (0x5a4630, Mat::Leather),
        legs: (0x3a3a2c, Mat::Cloth),
        tabard: Some(0x7a1c1c),
        trim: Some(0xcfa23f),
        helm: Helm::Kettle,
        helm_color: 0x6a7480,
        weapon: Weapon::Crossbow,
        pack: true,
        pauldrons: Some(0x5c646c),
        ..base(1.7)
    };
    let halberdier = Look {
        torso: (0x8b969e, Mat::Metal),
        arms: (0x657079, Mat::Metal),
        legs: (0x47505a, Mat::Metal),
        tabard: Some(0x8f2220),
        trim: Some(0xcfa23f),
        helm: Helm::Great,
        helm_color: 0x9aa4ac,
        weapon: Weapon::Halberd,
        pauldrons: Some(0xb4bec4),
        ..base(1.78)
    };
    let shieldbearer = Look {
        torso: (0x657079, Mat::Metal),
        arms: (0x47505a, Mat::Metal),
        tabard: Some(0x6c1717),
        trim: Some(0xcfa23f),
        helm: Helm::Kettle,
        helm_color: 0x8b969e,
        shield: Some(0x7a1c1c),
        big_shield: true,
        weapon: Weapon::Sword,
        pauldrons: Some(0x9aa4ac),
        bulk: 1.12,
        ..base(1.7)
    };
    let priest = Look {
        torso: (0xe8dcc0, Mat::Cloth),
        arms: (0xe8dcc0, Mat::Cloth),
        legs: (0xe8dcc0, Mat::Cloth),
        boots: 0x5a4630,
        tabard: Some(0x8f2220),
        trim: Some(0xcfa23f),
        helm: Helm::Mitre,
        helm_color: 0xf2ead2,
        hair: Some(0x6a5a46),
        cape: Some(0xb8a47a),
        weapon: Weapon::Scepter,
        body: Body::Gown,
        ..base(1.7)
    };
    let battle_mage = Look {
        torso: (0x8f2a22, Mat::Cloth),
        arms: (0x7a231c, Mat::Cloth),
        legs: (0x6a1f1a, Mat::Cloth),
        boots: 0x2c1c13,
        tabard: Some(0x2a2018),
        trim: Some(0xf08a2c),
        helm: Helm::PointedHat,
        helm_color: 0x5a1a14,
        hair: Some(0x2a1a10),
        cape: Some(0x3a1510),
        weapon: Weapon::FireStaff,
        body: Body::Gown,
        ..base(1.68)
    };
    let hal_venn = Look {
        torso: (0xe0d4b4, Mat::Cloth),
        arms: (0xd0c4a4, Mat::Cloth),
        legs: (0xe0d4b4, Mat::Cloth),
        boots: 0x4a3a28,
        tabard: Some(0x8f2220),
        trim: Some(0xeccb6c),
        helm: Helm::Circlet,
        hair: None,
        cape: Some(0x7a1c1c),
        weapon: Weapon::Mace,
        body: Body::Gown,
        pauldrons: Some(0xcfa23f),
        ..base(1.82)
    };
    let torvald = Look {
        skin: 0xc4906c,
        torso: (0x6a7d91, Mat::Metal),
        arms: (0x586a7c, Mat::Metal),
        legs: (0x3a4654, Mat::Cloth),
        tabard: Some(0x2f4a66),
        trim: Some(0xc9d4de),
        helm: Helm::Horned,
        helm_color: 0x8a9aaa,
        cape: Some(0x3d5a73),
        weapon: Weapon::GreatAxe,
        pauldrons: Some(0xaab4bc),
        bulk: 1.28,
        ..base(1.9)
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
    let hollow_archer = Look {
        helm: Helm::Hood,
        helm_color: 0x212428,
        weapon: Weapon::BoneBow,
        cape: Some(0x17191c),
        ..thrall.clone()
    };
    let digger = Look {
        torso: (0x3a3228, Mat::Cloth),
        arms: (0x8f8a6e, Mat::Bone),
        legs: (0x4a4234, Mat::Cloth),
        helm: Helm::Cap,
        helm_color: 0x2c2820,
        weapon: Weapon::Pick,
        pack: true,
        hunch: 14.0,
        ..thrall.clone()
    };
    let ghoul = Look {
        skin: 0x7d8a72,
        torso: (0x5b6350, Mat::Skin),
        arms: (0x7d8a72, Mat::Skin),
        legs: (0x4e5846, Mat::Skin),
        boots: 0x3a4034,
        weapon: Weapon::Claws,
        undead: true,
        hunch: 42.0,
        bulk: 0.95,
        ..base(1.55)
    };
    let witch = Look {
        skin: 0xcbd0b4,
        torso: (0x2b2e38, Mat::Cloth),
        arms: (0x24262e, Mat::Cloth),
        legs: (0x2b2e38, Mat::Cloth),
        tabard: Some(0x1f4a30),
        trim: Some(0x86f2a0),
        helm: Helm::Hood,
        helm_color: 0x1c1e26,
        weapon: Weapon::Staff,
        undead: true,
        body: Body::Robe,
        cape: Some(0x17191c),
        bulk: 0.95,
        ..base(1.72)
    };
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
        (
            "worker",
            worker,
            (56, 64),
            vec![
                Anim::Idle,
                Anim::Walk,
                Anim::Attack,
                Anim::Death,
                Anim::Work,
                Anim::Carry,
            ],
        ),
        ("swordsman", swordsman, (60, 66), std_anims.clone()),
        ("spearman", spearman, (76, 70), std_anims.clone()),
        ("archer", archer, (58, 66), std_anims.clone()),
        ("crossbowman", crossbowman, (60, 66), std_anims.clone()),
        ("halberdier", halberdier, (80, 74), std_anims.clone()),
        ("shieldbearer", shieldbearer, (64, 68), cast_anims.clone()),
        ("priest", priest, (60, 70), cast_anims.clone()),
        ("battle_mage", battle_mage, (62, 72), cast_anims.clone()),
        ("hero_isolde", isolde, (66, 74), cast_anims.clone()),
        ("hero_hal_venn", hal_venn, (66, 76), cast_anims.clone()),
        ("hero_torvald", torvald, (72, 78), cast_anims.clone()),
        ("hollow_thrall", thrall, (60, 66), std_anims.clone()),
        ("hollow_archer", hollow_archer, (58, 66), std_anims.clone()),
        (
            "hollow_digger",
            digger,
            (58, 66),
            vec![
                Anim::Idle,
                Anim::Walk,
                Anim::Attack,
                Anim::Death,
                Anim::Work,
                Anim::Carry,
            ],
        ),
        ("ghoul", ghoul, (62, 62), std_anims.clone()),
        ("ash_witch", witch, (60, 72), cast_anims.clone()),
        ("grave_brute", brute, (84, 84), std_anims.clone()),
        ("boss_pale_reeve", reeve, (92, 108), cast_anims),
    ]
}

/// Modelle, die nicht auf dem Humanoiden-Rig beruhen oder beritten sind: (ID, Animationen).
pub fn special_anims() -> Vec<(&'static str, Vec<Anim>)> {
    let std = vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death];
    vec![
        ("knight", std.clone()),
        ("scout_rider", std.clone()),
        ("horse_archer", std.clone()),
        ("battering_ram", std.clone()),
        ("catapult", std.clone()),
        ("bat_swarm", std),
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
            UnitModel {
                id,
                skeleton: sk,
                mesh,
                kind: Kind::Humanoid { bones, look },
                cell,
                anims,
            }
        })
        .collect();
    out.push(knight());
    out.push(scout_rider());
    out.push(horse_archer());
    out.push(ram());
    out.push(catapult());
    out.push(bat_swarm());
    out
}

fn knight() -> UnitModel {
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
    mounted("knight", look, 0x3d2a1c, 0x8f2220, (96, 92))
}

fn scout_rider() -> UnitModel {
    let look = Look {
        torso: (0x6b4a2c, Mat::Leather),
        arms: (0x5a4630, Mat::Leather),
        legs: (0x3a3a2c, Mat::Cloth),
        helm: Helm::Coif,
        helm_color: 0x7a838c,
        cape: Some(0x3d5a3a),
        weapon: Weapon::Sword,
        ..base(1.42)
    };
    mounted("scout_rider", look, 0xb08a5a, 0x3d5a3a, (92, 88))
}

fn horse_archer() -> UnitModel {
    let look = Look {
        torso: (0x3d3a32, Mat::Leather),
        arms: (0x4a3a28, Mat::Leather),
        legs: (0x2c2a24, Mat::Cloth),
        helm: Helm::Hood,
        helm_color: 0x2d3823,
        cape: Some(0x20281a),
        weapon: Weapon::Bow,
        ..base(1.44)
    };
    mounted("horse_archer", look, 0x2f2823, 0x8f2220, (96, 90))
}

/// Reiter auf Pferd: `coat` = Fellfarbe, `cloth` = Schabracke.
fn mounted(
    id: &'static str,
    look: Look,
    coat_hex: u32,
    cloth_hex: u32,
    cell: (u32, u32),
) -> UnitModel {
    let mut sk = Skeleton::default();
    let body = sk.add("horse_body", None, Vec3::new(0.0, 0.95, 0.0));
    let neck = sk.add("horse_neck", Some(body), Vec3::new(0.0, 0.12, 0.55));
    let head = sk.add("horse_head", Some(neck), Vec3::new(0.0, 0.42, 0.16));
    let mut legs = [(0, 0); 4];
    for (i, (x, z)) in [
        (-0.17f32, 0.46f32),
        (0.17, 0.46),
        (-0.17, -0.48),
        (0.17, -0.48),
    ]
    .iter()
    .enumerate()
    {
        let up = sk.add("leg_up", Some(body), Vec3::new(*x, -0.14, *z));
        let low = sk.add("leg_low", Some(up), Vec3::new(0.0, -0.38, 0.0));
        legs[i] = (up, low);
    }
    let tail = sk.add("horse_tail", Some(body), Vec3::new(0.0, 0.1, -0.5));
    let rider = rig::build_skeleton(
        &mut sk,
        &look,
        Some(body),
        Vec3::new(0.0, 0.2 - 0.46 * look.height, -0.04),
    );
    let mut m = rig::build_mesh(&look, &rider);
    let hb = HorseBones {
        body,
        neck,
        head,
        legs,
        tail,
    };
    let coat = |b| Paint::new(coat_hex, Mat::Leather).bone(b);
    m.ellipsoid(&coat(body), Vec3::ZERO, Vec3::new(0.25, 0.27, 0.7), 10, 6);
    // Schabracke mit Goldsaum.
    m.frustum(
        &Paint::new(cloth_hex, Mat::Cloth).bone(body),
        Vec3::new(0.0, 0.06, -0.42),
        Vec3::new(0.0, 0.06, 0.34),
        0.29,
        0.29,
        10,
    );
    m.frustum(
        &Paint::new(0xcfa23f, Mat::Gold).bone(body),
        Vec3::new(0.0, -0.16, -0.43),
        Vec3::new(0.0, -0.16, 0.35),
        0.295,
        0.295,
        10,
    );
    m.frustum(
        &coat(neck),
        Vec3::ZERO,
        Vec3::new(0.0, 0.42, 0.14),
        0.14,
        0.1,
        7,
    );
    m.frustum(
        &Paint::new(0x1a110c, Mat::Hair).bone(neck),
        Vec3::new(0.0, 0.05, -0.06),
        Vec3::new(0.0, 0.4, 0.05),
        0.04,
        0.03,
        4,
    );
    m.frustum(
        &coat(head),
        Vec3::ZERO,
        Vec3::new(0.0, -0.1, 0.38),
        0.09,
        0.06,
        6,
    );
    m.ellipsoid(
        &Paint::new(0x8b969e, Mat::Metal).bone(head),
        Vec3::new(0.0, 0.03, 0.12),
        Vec3::new(0.075, 0.06, 0.14),
        6,
        4,
    );
    for ((up, low), _) in legs.iter().zip(0..) {
        m.frustum(
            &coat(*up),
            Vec3::ZERO,
            Vec3::new(0.0, -0.38, 0.0),
            0.08,
            0.055,
            6,
        );
        m.frustum(
            &coat(*low),
            Vec3::ZERO,
            Vec3::new(0.0, -0.36, 0.0),
            0.05,
            0.042,
            6,
        );
        m.frustum(
            &Paint::new(0x101010, Mat::Plain).bone(*low),
            Vec3::new(0.0, -0.34, 0.0),
            Vec3::new(0.0, -0.4, 0.0),
            0.06,
            0.06,
            6,
        );
    }
    m.frustum(
        &Paint::new(0x1a110c, Mat::Hair).bone(tail),
        Vec3::ZERO,
        Vec3::new(0.0, -0.45, -0.12),
        0.05,
        0.03,
        4,
    );
    UnitModel {
        id,
        skeleton: sk,
        mesh: m,
        kind: Kind::Knight {
            horse: hb,
            rider,
            look,
        },
        cell,
        anims: vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death],
    }
}

// ------------------------------------------------------------------------------------------------
// Belagerungsmaschinen
// ------------------------------------------------------------------------------------------------

/// Knochen einer Maschine: Körper, beweglicher Teil (Rammbaum bzw. Wurfarm) und Räder.
#[derive(Clone, Copy)]
pub struct MachineBones {
    pub body: usize,
    pub moving: usize,
    pub wheels: [usize; 4],
    pub catapult: bool,
}

fn wheel_mesh(m: &mut Mesh, bone: usize, r: f32, left: bool) {
    let wood = Paint::new(0x5a3a22, Mat::WoodPlank).bone(bone);
    let dark = Paint::new(0x2c1c13, Mat::WoodPlank).bone(bone);
    let iron = Paint::new(0x4a4a52, Mat::Metal).bone(bone);
    let s = if left { -1.0 } else { 1.0 };
    m.frustum(
        &wood,
        Vec3::new(s * -0.05, 0.0, 0.0),
        Vec3::new(s * 0.05, 0.0, 0.0),
        r,
        r,
        10,
    );
    m.frustum(
        &iron,
        Vec3::new(s * 0.05, 0.0, 0.0),
        Vec3::new(s * 0.065, 0.0, 0.0),
        r,
        r,
        10,
    );
    m.bx(
        &dark,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.065, 0.018, r * 0.97),
    );
    m.bx(
        &dark,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.065, r * 0.97, 0.018),
    );
    m.frustum(
        &iron,
        Vec3::new(s * -0.07, 0.0, 0.0),
        Vec3::new(s * 0.09, 0.0, 0.0),
        0.04,
        0.04,
        6,
    );
}

fn ram() -> UnitModel {
    let mut sk = Skeleton::default();
    let body = sk.add("body", None, Vec3::ZERO);
    let moving = sk.add("log", Some(body), Vec3::new(0.0, 0.78, 0.0));
    let mut wheels = [0; 4];
    for (i, (x, z)) in [(-0.4f32, 0.5f32), (0.4, 0.5), (-0.4, -0.5), (0.4, -0.5)]
        .iter()
        .enumerate()
    {
        wheels[i] = sk.add("wheel", Some(body), Vec3::new(*x, 0.17, *z));
    }
    let mut m = Mesh::default();
    let wood = Paint::new(0x5a3a22, Mat::WoodPlank).bone(body);
    let dark = Paint::new(0x2c1c13, Mat::WoodPlank).bone(body);
    let hide = Paint::new(0x5a4632, Mat::Leather).bone(body);
    let iron = Paint::new(0x4a4a52, Mat::Metal).bone(body);
    // Rahmen, Pfosten und Dach.
    for sx in [-1.0f32, 1.0] {
        m.bx(
            &wood,
            Vec3::new(sx * 0.34, 0.3, 0.0),
            Vec3::new(0.05, 0.05, 0.82),
        );
        for sz in [-1.0f32, 1.0] {
            m.bx(
                &dark,
                Vec3::new(sx * 0.34, 0.55, sz * 0.55),
                Vec3::new(0.04, 0.27, 0.04),
            );
        }
        let t = Mat4::from_translation(Vec3::new(sx * 0.25, 0.9, 0.0))
            * Mat4::from_rotation_z(sx * 0.5);
        m.cuboid(
            &hide,
            Vec3::new(-0.27, -0.025, -0.78),
            Vec3::new(0.27, 0.025, 0.78),
            t,
        );
        m.cuboid(
            &wood,
            Vec3::new(-0.27, 0.02, -0.78),
            Vec3::new(0.27, 0.04, -0.74),
            t,
        );
        m.cuboid(
            &wood,
            Vec3::new(-0.27, 0.02, 0.74),
            Vec3::new(0.27, 0.04, 0.78),
            t,
        );
    }
    m.bx(
        &dark,
        Vec3::new(0.0, 0.3, 0.55),
        Vec3::new(0.34, 0.04, 0.04),
    );
    m.bx(
        &dark,
        Vec3::new(0.0, 0.3, -0.55),
        Vec3::new(0.34, 0.04, 0.04),
    );
    m.bx(&dark, Vec3::new(0.0, 1.03, 0.0), Vec3::new(0.03, 0.03, 0.8));
    // Eisenbeschläge und Banner.
    for sz in [-0.55f32, 0.55] {
        m.bx(&iron, Vec3::new(0.0, 0.84, sz), Vec3::new(0.3, 0.02, 0.025));
    }
    m.bx(
        &Paint::new(0x8f2220, Mat::Cloth).bone(body),
        Vec3::new(0.0, 1.05, -0.8),
        Vec3::new(0.12, 0.1, 0.012),
    );
    for (i, w) in wheels.iter().enumerate() {
        wheel_mesh(&mut m, *w, 0.17, i % 2 == 0);
    }
    // Rammbaum mit Eisenkopf, an Seilen aufgehängt (Drehpunkt unter dem Dach).
    let bark = Paint::new(0x4a3322, Mat::Bark).bone(moving);
    let band = Paint::new(0x3a3a42, Mat::Metal).bone(moving);
    m.frustum(
        &bark,
        Vec3::new(0.0, -0.4, -0.85),
        Vec3::new(0.0, -0.4, 0.9),
        0.1,
        0.1,
        8,
    );
    for z in [-0.4f32, 0.0, 0.4, 0.8] {
        m.frustum(
            &band,
            Vec3::new(0.0, -0.4, z - 0.03),
            Vec3::new(0.0, -0.4, z + 0.03),
            0.112,
            0.112,
            8,
        );
    }
    m.frustum(
        &band,
        Vec3::new(0.0, -0.4, 0.9),
        Vec3::new(0.0, -0.4, 1.1),
        0.13,
        0.1,
        8,
    );
    m.frustum(
        &band,
        Vec3::new(0.0, -0.4, 1.1),
        Vec3::new(0.0, -0.4, 1.22),
        0.07,
        0.0,
        6,
    );
    for z in [-0.3f32, 0.55] {
        m.bx(
            &Paint::new(0x7a6a50, Mat::Thatch).bone(moving),
            Vec3::new(0.0, -0.1, z),
            Vec3::new(0.012, 0.3, 0.012),
        );
    }
    UnitModel {
        id: "battering_ram",
        skeleton: sk,
        mesh: m,
        kind: Kind::Machine {
            bones: MachineBones {
                body,
                moving,
                wheels,
                catapult: false,
            },
        },
        cell: (96, 84),
        anims: vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death],
    }
}

fn catapult() -> UnitModel {
    let mut sk = Skeleton::default();
    let body = sk.add("body", None, Vec3::ZERO);
    let moving = sk.add("arm", Some(body), Vec3::new(0.0, 0.8, 0.0));
    let mut wheels = [0; 4];
    for (i, (x, z)) in [
        (-0.45f32, 0.55f32),
        (0.45, 0.55),
        (-0.45, -0.55),
        (0.45, -0.55),
    ]
    .iter()
    .enumerate()
    {
        wheels[i] = sk.add("wheel", Some(body), Vec3::new(*x, 0.18, *z));
    }
    let mut m = Mesh::default();
    let wood = Paint::new(0x6e4a2e, Mat::WoodPlank).bone(body);
    let dark = Paint::new(0x2c1c13, Mat::WoodPlank).bone(body);
    let iron = Paint::new(0x4a4a52, Mat::Metal).bone(body);
    for sx in [-1.0f32, 1.0] {
        m.bx(
            &wood,
            Vec3::new(sx * 0.34, 0.28, 0.0),
            Vec3::new(0.055, 0.055, 0.85),
        );
        m.bx(
            &dark,
            Vec3::new(sx * 0.34, 0.55, 0.0),
            Vec3::new(0.045, 0.3, 0.07),
        );
        m.bx(
            &iron,
            Vec3::new(sx * 0.34, 0.35, 0.0),
            Vec3::new(0.06, 0.02, 0.09),
        );
    }
    for z in [-0.8f32, -0.3, 0.3, 0.8] {
        m.bx(&dark, Vec3::new(0.0, 0.28, z), Vec3::new(0.34, 0.035, 0.04));
    }
    // Querachse und Anschlagbalken.
    m.bx(
        &dark,
        Vec3::new(0.0, 0.8, 0.0),
        Vec3::new(0.38, 0.035, 0.035),
    );
    m.bx(
        &wood,
        Vec3::new(0.0, 0.5, 0.32),
        Vec3::new(0.3, 0.04, 0.035),
    );
    m.bx(
        &Paint::new(0x8f2220, Mat::Cloth).bone(body),
        Vec3::new(0.0, 0.32, -0.85),
        Vec3::new(0.14, 0.08, 0.012),
    );
    for (i, w) in wheels.iter().enumerate() {
        wheel_mesh(&mut m, *w, 0.18, i % 2 == 0);
    }
    // Wurfarm (zeigt in Ruhe nach oben), Gegengewicht und Schleuder mit Felsbrocken.
    let arm = Paint::new(0x6e4a2e, Mat::WoodPlank).bone(moving);
    m.bx(
        &arm,
        Vec3::new(0.0, 0.3, 0.0),
        Vec3::new(0.045, 0.62, 0.045),
    );
    m.bx(
        &Paint::new(0x4a4a52, Mat::Metal).bone(moving),
        Vec3::new(0.0, -0.02, 0.0),
        Vec3::new(0.06, 0.05, 0.06),
    );
    m.bx(
        &Paint::new(0x4c4a43, Mat::Rock).bone(moving),
        Vec3::new(0.0, -0.36, 0.0),
        Vec3::new(0.15, 0.13, 0.13),
    );
    m.bx(
        &Paint::new(0x4a4a52, Mat::Metal).bone(moving),
        Vec3::new(0.0, -0.36, 0.0),
        Vec3::new(0.16, 0.02, 0.14),
    );
    m.frustum(
        &Paint::new(0x3a2c1c, Mat::Leather).bone(moving),
        Vec3::new(0.0, 0.88, 0.0),
        Vec3::new(0.0, 0.96, 0.0),
        0.08,
        0.13,
        8,
    );
    m.ellipsoid(
        &Paint::new(0x5a5a60, Mat::Rock).bone(moving),
        Vec3::new(0.0, 1.02, 0.0),
        Vec3::splat(0.1),
        7,
        4,
    );
    UnitModel {
        id: "catapult",
        skeleton: sk,
        mesh: m,
        kind: Kind::Machine {
            bones: MachineBones {
                body,
                moving,
                wheels,
                catapult: true,
            },
        },
        cell: (100, 100),
        anims: vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death],
    }
}

/// Pose einer Belagerungsmaschine.
fn machine_pose(model: &UnitModel, b: &MachineBones, anim: Anim, t: f32) -> Pose {
    let mut ps = model.skeleton.rest();
    let ph = t * TAU;
    // Ruhestellung: Wurfarm gespannt nach hinten.
    let cocked = if b.catapult { -72.0 } else { 0.0 };
    ps.euler(b.moving, cocked, 0.0, 0.0);
    match anim {
        Anim::Walk => {
            for w in b.wheels {
                ps.euler(w, ph.to_degrees(), 0.0, 0.0);
            }
            ps.shift(b.body, Vec3::new(0.0, (ph * 2.0).sin().abs() * 0.025, 0.0));
            ps.euler(b.body, (ph * 2.0).sin() * 1.2, 0.0, (ph).sin() * 0.8);
        }
        Anim::Attack => {
            if b.catapult {
                // Spannen, Auslösen, Nachschwingen.
                let a = keys6(t, &[-72.0, -72.0, -40.0, 28.0, 62.0, 14.0]);
                ps.euler(b.moving, a - cocked, 0.0, 0.0);
                ps.shift(
                    b.body,
                    Vec3::new(0.0, 0.0, -0.05 * keys6(t, &[0.0, 0.0, 0.0, 1.0, 0.4, 0.0])),
                );
            } else {
                // Rammbaum zurückziehen, vorschnellen, auspendeln (Vorwärtsschwung = negative Drehung).
                let a = keys6(t, &[0.0, 24.0, 30.0, -34.0, -8.0, 0.0]);
                ps.euler(b.moving, a, 0.0, 0.0);
                ps.shift(
                    b.body,
                    Vec3::new(0.0, 0.0, 0.1 * keys6(t, &[0.0, 0.0, 0.0, 1.0, 0.5, 0.0])),
                );
            }
        }
        Anim::Death => {
            let f = keys6(t, &[0.0, 0.3, 0.7, 0.95, 1.0, 1.0]);
            ps.euler(b.body, 4.0 * f, 0.0, 24.0 * f);
            ps.shift(b.body, Vec3::new(0.0, -0.1 * f, 0.0));
            ps.euler(
                b.moving,
                if b.catapult { 140.0 * f } else { -70.0 * f },
                0.0,
                12.0 * f,
            );
            ps.euler(b.wheels[1], 0.0, 0.0, -30.0 * f);
            ps.euler(b.wheels[2], 25.0 * f, 0.0, 0.0);
        }
        _ => {
            ps.euler(b.moving, (ph).sin() * 1.5, 0.0, 0.0);
        }
    }
    ps
}

fn keys6(t: f32, k: &[f32]) -> f32 {
    let n = k.len() - 1;
    let x = (t.clamp(0.0, 1.0) * n as f32).min(n as f32 - 1e-4);
    let i = x.floor() as usize;
    let f = x - i as f32;
    let f = f * f * (3.0 - 2.0 * f);
    k[i] + (k[i + 1] - k[i]) * f
}

// ------------------------------------------------------------------------------------------------
// Fledermausschwarm
// ------------------------------------------------------------------------------------------------

/// Knochen eines Fledermausschwarms: je Tier ein Körper und zwei Flügel.
#[derive(Clone)]
pub struct SwarmBones {
    pub root: usize,
    pub bats: Vec<(usize, usize, usize, Vec3)>,
}

const BAT_SPOTS: [(f32, f32, f32); 4] = [
    (0.0, 1.0, 0.0),
    (0.75, 0.75, 0.25),
    (-0.75, 0.85, 0.0),
    (0.05, 1.35, -0.6),
];

fn bat_swarm() -> UnitModel {
    let mut sk = Skeleton::default();
    let root = sk.add("root", None, Vec3::ZERO);
    let mut m = Mesh::default();
    let mut bats = Vec::new();
    // Tiere sind größer als natürlich, damit der Schwarm im Pixelraster lesbar bleibt.
    let k = 1.8;
    for (x, y, z) in BAT_SPOTS.iter() {
        let pos = Vec3::new(*x, *y, *z);
        let body = sk.add("bat", Some(root), pos);
        let wl = sk.add("wing_l", Some(body), Vec3::new(-0.035 * k, 0.0, 0.0));
        let wr = sk.add("wing_r", Some(body), Vec3::new(0.035 * k, 0.0, 0.0));
        let fur = Paint::new(0x4a3c5c, Mat::Cloth).bone(body);
        m.ellipsoid(
            &fur,
            Vec3::ZERO,
            Vec3::new(0.06 * k, 0.05 * k, 0.1 * k),
            6,
            4,
        );
        m.ellipsoid(
            &fur,
            Vec3::new(0.0, 0.02 * k, 0.1 * k),
            Vec3::splat(0.045 * k),
            5,
            3,
        );
        for sx in [-1.0f32, 1.0] {
            m.frustum(
                &fur,
                Vec3::new(sx * 0.025 * k, 0.05 * k, 0.11 * k),
                Vec3::new(sx * 0.04 * k, 0.11 * k, 0.1 * k),
                0.016 * k,
                0.0,
                3,
            );
            m.ellipsoid(
                &Paint::new(0x86f2a0, Mat::Glow).bone(body).glow(1.0),
                Vec3::new(sx * 0.02 * k, 0.035 * k, 0.14 * k),
                Vec3::splat(0.016 * k),
                3,
                2,
            );
        }
        // Flügel: Membran mit Spitze und hellen Fingerstreben.
        for (bone, s) in [(wl, -1.0f32), (wr, 1.0)] {
            let w = Paint::new(0x8a78a4, Mat::Cloth).bone(bone);
            let (x0, x1) = if s < 0.0 {
                (-0.34 * k, 0.0)
            } else {
                (0.0, 0.34 * k)
            };
            m.cuboid(
                &w,
                Vec3::new(x0, -0.01 * k, -0.13 * k),
                Vec3::new(x1, 0.01 * k, 0.15 * k),
                Mat4::IDENTITY,
            );
            m.tri_flat(
                &w,
                Vec3::new(s * 0.34 * k, 0.0, -0.13 * k),
                Vec3::new(s * 0.34 * k, 0.0, 0.15 * k),
                Vec3::new(s * 0.5 * k, 0.0, -0.05 * k),
            );
            let strut = Paint::new(0xa898bc, Mat::Bone).bone(bone);
            m.frustum(
                &strut,
                Vec3::new(0.0, 0.012 * k, -0.08 * k),
                Vec3::new(s * 0.47 * k, 0.012 * k, -0.04 * k),
                0.016 * k,
                0.006 * k,
                3,
            );
        }
        bats.push((body, wl, wr, pos));
    }
    UnitModel {
        id: "bat_swarm",
        skeleton: sk,
        mesh: m,
        kind: Kind::Swarm {
            bones: SwarmBones { root, bats },
        },
        cell: (96, 92),
        anims: vec![Anim::Idle, Anim::Walk, Anim::Attack, Anim::Death],
    }
}

fn swarm_pose(model: &UnitModel, b: &SwarmBones, anim: Anim, t: f32) -> Pose {
    let mut ps = model.skeleton.rest();
    let ph = t * TAU;
    let cycles = match anim {
        Anim::Walk => 3.0,
        Anim::Attack => 2.0,
        _ => 1.0,
    };
    for (i, (body, wl, wr, pos)) in b.bats.iter().enumerate() {
        let off = i as f32 * 1.1;
        let f = match anim {
            Anim::Death => -85.0,
            _ => (ph * cycles + off).sin() * 35.0,
        };
        // Flügel stehen in V-Form (30° Grundwinkel) und schlagen um ±35° darum.
        ps.euler(*wl, 0.0, 0.0, -(30.0 + f));
        ps.euler(*wr, 0.0, 0.0, 30.0 + f);
        match anim {
            Anim::Death => {
                let g = keys6(t, &[0.0, 0.3, 0.7, 1.0, 1.0, 1.0]);
                ps.shift(*body, Vec3::new(0.0, -(pos.y - 0.08) * g, 0.0));
                ps.euler(*body, 70.0 * g, 0.0, 30.0 * g * (i as f32 - 2.5).signum());
            }
            Anim::Attack => {
                let dive = (t * std::f32::consts::PI).sin();
                ps.shift(
                    *body,
                    Vec3::new(0.0, -0.45 * dive * (0.6 + (i % 3) as f32 * 0.2), 0.3 * dive),
                );
                ps.euler(*body, 35.0 * dive, 0.0, 0.0);
            }
            _ => {
                ps.shift(
                    *body,
                    Vec3::new(
                        (ph + off).cos() * 0.07,
                        (ph * 2.0 + off).sin() * 0.05,
                        (ph + off * 2.0).sin() * 0.06,
                    ),
                );
                ps.euler(
                    *body,
                    (ph * cycles + off).sin() * 6.0,
                    0.0,
                    (ph + off).cos() * 8.0,
                );
            }
        }
    }
    ps
}

/// Pose eines Modells.
pub fn pose(model: &UnitModel, anim: Anim, t: f32) -> Pose {
    match &model.kind {
        Kind::Humanoid { bones, look } => {
            let r = rig::Rig {
                skeleton: model.skeleton.clone(),
                bones: *bones,
                look: look.clone(),
            };
            rig::pose(&r, anim, t)
        }
        Kind::Machine { bones } => machine_pose(model, bones, anim, t),
        Kind::Swarm { bones } => swarm_pose(model, bones, anim, t),
        Kind::Knight { horse, rider, look } => {
            let r = rig::Rig {
                skeleton: model.skeleton.clone(),
                bones: *rider,
                look: look.clone(),
            };
            // Reiterhaltung: Grundpose des Rigs (Idle/Angriff), Beine gespreizt.
            let rider_anim = match anim {
                Anim::Walk => Anim::Idle,
                other => other,
            };
            let mut ps = rig::pose(
                &r,
                if rider_anim == Anim::Death {
                    Anim::Idle
                } else {
                    rider_anim
                },
                t,
            );
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
            ps
        }
    }
}

/// Weltposition des Kopfes in der Idle-Pose (für Porträtausschnitte).
pub fn head_position(model: &UnitModel) -> Vec3 {
    let ps = pose(model, Anim::Idle, 0.0);
    let w = model.skeleton.world(&ps);
    match &model.kind {
        Kind::Humanoid { bones, look }
        | Kind::Knight {
            rider: bones, look, ..
        } => w[bones.head].transform_point3(Vec3::new(0.0, 0.075 * look.height, 0.0)),
        Kind::Machine { bones } => w[bones.body].transform_point3(Vec3::new(
            0.0,
            if bones.catapult { 0.85 } else { 0.7 },
            0.1,
        )),
        Kind::Swarm { bones } => w[bones.root].transform_point3(Vec3::new(0.0, 0.95, 0.0)),
    }
}
