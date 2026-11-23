//! Ereignisse, die die Simulation pro Tick ausgibt (für Audio, Partikel, UI, Chronik).

use crate::data::{BuildingTypeId, DamageType, Item, ProjectileKind, ResourceKind, UnitTypeId};
use crate::entity::{BuildingId, EntityRef, PlayerId, UnitId};
use crate::env::Weather;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SimEvent {
    UnitSpawned { id: UnitId, kind: UnitTypeId, owner: PlayerId, pos: Vec2 },
    UnitDied { id: UnitId, kind: UnitTypeId, owner: PlayerId, pos: Vec2, facing_right: bool, facing: f32 },
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
    /// Ein Produktionszyklus wurde abgeschlossen (Ton/Effekt am Gebäude).
    ProductionCycle { building: BuildingId, kind: BuildingTypeId, owner: PlayerId, pos: Vec2 },
    /// Handel am Markt: `amount > 0` = gekauft, `< 0` = verkauft; `gold` = gezahlter bzw. erhaltener Betrag.
    Traded { owner: PlayerId, item: Item, amount: i32, gold: f32, pos: Vec2 },
    GateToggled { id: BuildingId, pos: Vec2, open: bool },
    /// Wohnhaus ist auf- (`up`) oder abgestiegen.
    HouseChanged { id: BuildingId, kind: BuildingTypeId, owner: PlayerId, pos: Vec2, up: bool },
    EraAdvanced { owner: PlayerId, era: u8 },
    /// Unruhe in der Bevölkerung (zu hohe Steuern oder Mangel).
    Unrest { owner: PlayerId },
    Error { owner: PlayerId, text: String },
    Victory,
    Defeat,
}
