//! Ereignisse, die die Simulation pro Tick ausgibt (für Audio, Partikel, UI, Chronik).

use crate::data::{BuildingTypeId, DamageType, ProjectileKind, ResourceKind, UnitTypeId};
use crate::entity::{BuildingId, EntityRef, PlayerId, UnitId};
use crate::env::Weather;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SimEvent {
    UnitSpawned { id: UnitId, kind: UnitTypeId, owner: PlayerId, pos: Vec2 },
    UnitDied { id: UnitId, kind: UnitTypeId, owner: PlayerId, pos: Vec2, facing_right: bool },
    MeleeSwing { attacker: UnitId, kind: UnitTypeId, pos: Vec2 },
    Hit { target: EntityRef, pos: Vec2, amount: f32, damage_type: DamageType, blocked: bool },
    ProjectileFired { kind: ProjectileKind, from: Vec2, owner: PlayerId },
    ProjectileImpact { kind: ProjectileKind, pos: Vec2, hit: bool },
    BuildingPlaced { id: BuildingId, kind: BuildingTypeId, owner: PlayerId, pos: Vec2 },
    BuildingCompleted { id: BuildingId, kind: BuildingTypeId, owner: PlayerId, pos: Vec2 },
    BuildingDestroyed { id: BuildingId, kind: BuildingTypeId, owner: PlayerId, pos: Vec2 },
    TrainCompleted { unit: UnitId, kind: UnitTypeId, owner: PlayerId, pos: Vec2 },
    WorkStroke { unit: UnitId, resource: Option<ResourceKind>, pos: Vec2 },
    ResourceDelivered { owner: PlayerId, kind: ResourceKind, amount: u32, pos: Vec2 },
    NodeDepleted { pos: Vec2, tree: bool },
    AbilityCast { unit: UnitId, ability: String, pos: Vec2, target: Vec2 },
    LevelUp { unit: UnitId, level: u32 },
    MoraleBroken { unit: UnitId, pos: Vec2 },
    HeroFallen { kind: UnitTypeId, pos: Vec2 },
    HeroRespawned { unit: UnitId, pos: Vec2 },
    WaveIncoming { wave: u32, from: Vec2, size: u32 },
    NightBegins { night: u32 },
    DayBegins { day: u32 },
    WeatherChanged { weather: Weather },
    Lightning,
    Message { speaker: String, text: String, voice: Option<String> },
    ObjectiveAdded { id: String },
    ObjectiveCompleted { id: String },
    Starvation { owner: PlayerId, active: bool },
    Error { owner: PlayerId, text: String },
    Victory,
    Defeat,
}
