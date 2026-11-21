//! Entitätstypen der Simulation: Einheiten, Gebäude, Ressourcenquellen, Projektile.

use crate::arena::Id;
use crate::data::{BuildingTypeId, DamageType, ProjectileKind, ResourceKind, UnitTypeId};
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

pub type UnitId = Id<Unit>;
pub type BuildingId = Id<Building>;
pub type NodeId = Id<ResourceNode>;

/// Spieler-Index. 0 = menschlicher Spieler (Glutkrone), 1 = Die Hohlen, 2 = neutral.
pub type PlayerId = u8;
pub const PLAYER: PlayerId = 0;
pub const ENEMY: PlayerId = 1;
pub const NEUTRAL: PlayerId = 2;

/// Verweis auf ein angreifbares Ziel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityRef {
    Unit(UnitId),
    Building(BuildingId),
}

/// Befehl, den eine Einheit gerade ausführt.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Order {
    #[default]
    Idle,
    Move { target: Vec2, attack_move: bool },
    Attack { target: EntityRef },
    Gather { node: NodeId },
    ReturnCargo { node: Option<NodeId> },
    Build { building: BuildingId },
    Hold,
    Patrol { a: Vec2, b: Vec2, to_b: bool },
    Cast { ability: u8, point: Vec2 },
    /// Wache halten an einem Posten (KI): angreifen im Radius, danach zurückkehren.
    Guard { post: Vec2, radius: f32 },
}

/// Grobe Aktivität für Animation und Audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Activity {
    #[default]
    Idle,
    Walk,
    Attack,
    Work,
    Cast,
    Flee,
}

/// Zeitlich begrenzter Effekt.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Buff {
    pub damage_mult: f32,
    pub armor_bonus: f32,
    pub remaining: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub kind: UnitTypeId,
    pub owner: PlayerId,
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub vel: Vec2,
    /// Blickrichtung (Bildschirm): `true` = nach rechts.
    pub facing_right: bool,
    pub hp: f32,
    pub max_hp: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub morale: f32,
    pub order: Order,
    pub queue: Vec<Order>,
    pub path: Vec<Vec2>,
    pub path_goal: Option<Vec2>,
    pub path_pending: bool,
    pub repath_timer: f32,
    pub stuck_timer: f32,
    pub attack_cd: f32,
    /// Laufender Schlag: verbleibende Ausholzeit und Ziel.
    pub windup: Option<(f32, EntityRef)>,
    pub target: Option<EntityRef>,
    pub carry: Option<(ResourceKind, u32)>,
    pub work_timer: f32,
    pub level: u32,
    pub xp: u32,
    pub ability_cd: Vec<f32>,
    pub buffs: Vec<Buff>,
    pub shield: f32,
    pub shield_time: f32,
    pub routing: f32,
    pub combat_timer: f32,
    pub activity: Activity,
    pub activity_time: f32,
    pub last_attacker: Option<EntityRef>,
    pub tag: Option<String>,
    /// Gehört zu einer Angriffswelle (KI).
    pub wave: Option<u32>,
    pub alive_time: f32,
}

impl Unit {
    pub fn new(kind: UnitTypeId, owner: PlayerId, pos: Vec2, hp: f32, mana: f32, morale: f32, abilities: usize) -> Self {
        Unit {
            kind,
            owner,
            pos,
            prev_pos: pos,
            vel: Vec2::ZERO,
            facing_right: true,
            hp,
            max_hp: hp,
            mana,
            max_mana: mana,
            morale,
            order: Order::Idle,
            queue: Vec::new(),
            path: Vec::new(),
            path_goal: None,
            path_pending: false,
            repath_timer: 0.0,
            stuck_timer: 0.0,
            attack_cd: 0.0,
            windup: None,
            target: None,
            carry: None,
            work_timer: 0.0,
            level: 1,
            xp: 0,
            ability_cd: vec![0.0; abilities],
            buffs: Vec::new(),
            shield: 0.0,
            shield_time: 0.0,
            routing: 0.0,
            combat_timer: 0.0,
            activity: Activity::Idle,
            activity_time: 0.0,
            last_attacker: None,
            tag: None,
            wave: None,
            alive_time: 0.0,
        }
    }

    pub fn set_activity(&mut self, a: Activity) {
        if self.activity != a {
            self.activity = a;
            self.activity_time = 0.0;
        }
    }

    pub fn damage_mult(&self) -> f32 {
        self.buffs.iter().fold(1.0, |m, b| m * b.damage_mult)
    }

    pub fn armor_bonus(&self) -> f32 {
        self.buffs.iter().map(|b| b.armor_bonus).sum()
    }

    /// Setzt einen neuen Pfadwunsch; die eigentliche Suche erfolgt budgetiert im Tick.
    pub fn request_path(&mut self, goal: Vec2) {
        self.path_goal = Some(goal);
        self.path_pending = true;
        self.path.clear();
        self.stuck_timer = 0.0;
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
        self.path_goal = None;
        self.path_pending = false;
    }

    pub fn is_moving(&self) -> bool {
        !self.path.is_empty() || self.path_pending
    }
}

/// Eintrag der Trainingswarteschlange.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TrainItem {
    pub unit: UnitTypeId,
    pub progress: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    pub kind: BuildingTypeId,
    pub owner: PlayerId,
    /// Obere linke Kachel der Grundfläche.
    pub tile: IVec2,
    pub size: IVec2,
    pub hp: f32,
    pub max_hp: f32,
    /// Baufortschritt 0..1.
    pub progress: f32,
    pub complete: bool,
    pub queue: Vec<TrainItem>,
    pub rally: Option<Vec2>,
    pub attack_cd: f32,
    pub income_acc: f32,
    pub tag: Option<String>,
    pub invulnerable: bool,
    pub last_hit_time: f32,
}

impl Building {
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.tile.x as f32 + self.size.x as f32 * 0.5, self.tile.y as f32 + self.size.y as f32 * 0.5)
    }

    /// Ungefährer Radius für Reichweitenberechnungen.
    pub fn radius(&self) -> f32 {
        (self.size.x.max(self.size.y) as f32) * 0.5
    }

    /// Abstand eines Punktes zur Grundfläche (0 = innen).
    pub fn distance_to(&self, p: Vec2) -> f32 {
        let min = self.tile.as_vec2();
        let max = min + self.size.as_vec2();
        let c = p.clamp(min, max);
        (p - c).length()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    Tree,
    GoldMine,
    StoneQuarry,
}

impl NodeKind {
    pub fn resource(self) -> ResourceKind {
        match self {
            NodeKind::Tree => ResourceKind::Wood,
            NodeKind::GoldMine => ResourceKind::Gold,
            NodeKind::StoneQuarry => ResourceKind::Stone,
        }
    }

    pub fn size(self) -> IVec2 {
        match self {
            NodeKind::Tree => IVec2::ONE,
            NodeKind::GoldMine | NodeKind::StoneQuarry => IVec2::splat(2),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceNode {
    pub kind: NodeKind,
    pub tile: IVec2,
    pub amount: u32,
    pub max_amount: u32,
    pub variant: u8,
}

impl ResourceNode {
    pub fn center(&self) -> Vec2 {
        let s = self.kind.size();
        Vec2::new(self.tile.x as f32 + s.x as f32 * 0.5, self.tile.y as f32 + s.y as f32 * 0.5)
    }

    pub fn distance_to(&self, p: Vec2) -> f32 {
        let min = self.tile.as_vec2();
        let max = min + self.kind.size().as_vec2();
        (p - p.clamp(min, max)).length()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projectile {
    pub kind: ProjectileKind,
    pub owner: PlayerId,
    pub origin: Vec2,
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub target: EntityRef,
    pub target_pos: Vec2,
    pub speed: f32,
    pub damage: f32,
    pub damage_type: DamageType,
    pub bonus_class_mult: Vec<(crate::data::UnitClass, f32)>,
    pub source: Option<EntityRef>,
    /// Flugbogenhöhe in Kacheln (nur Darstellung).
    pub arc: f32,
    pub progress: f32,
    pub total: f32,
}
