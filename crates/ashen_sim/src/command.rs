//! Spielerbefehle. Befehle sind reine Daten, damit sie gespeichert, über das Netzwerk
//! gesendet (Lockstep) oder für Wiederholungen aufgezeichnet werden können.

use crate::data::{BuildingTypeId, Item, UnitTypeId};
use crate::rpg::Formation;
use crate::entity::{BuildingId, EntityRef, NodeId, PlayerId, UnitId};
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    Move { units: Vec<UnitId>, target: Vec2, attack_move: bool, queue: bool },
    Attack { units: Vec<UnitId>, target: EntityRef },
    Stop { units: Vec<UnitId> },
    Hold { units: Vec<UnitId> },
    Patrol { units: Vec<UnitId>, target: Vec2 },
    Gather { units: Vec<UnitId>, node: NodeId },
    Build { units: Vec<UnitId>, kind: BuildingTypeId, tile: IVec2 },
    Repair { units: Vec<UnitId>, building: BuildingId },
    Train { building: BuildingId, unit: UnitTypeId },
    CancelTrain { building: BuildingId, index: usize },
    SetRally { building: BuildingId, point: Vec2 },
    UseAbility { unit: UnitId, ability: u8, point: Vec2 },
    Demolish { building: BuildingId },
    /// Mehrere Bauteile (Mauerlinie) auf einmal platzieren; Arbeiter verteilen sich darauf.
    BuildLine { units: Vec<UnitId>, kind: BuildingTypeId, tiles: Vec<IVec2> },
    /// Arbeiterlimit eines Produktionsgebäudes (`None` = voll besetzen, `Some(0)` = stilllegen).
    SetWorkers { building: BuildingId, limit: Option<u32> },
    /// Rezept eines Gebäudes mit mehreren Rezepten wählen.
    SetRecipe { building: BuildingId, recipe: u8 },
    /// Tor für eigene Einheiten öffnen oder verschließen.
    SetGate { building: BuildingId, open: bool },
    /// Steuersatz (Index in den Steuerstufen).
    SetTax { level: u8 },
    /// Handel am Markt: positiv = kaufen, negativ = verkaufen.
    Trade { item: Item, amount: i32 },
    /// Siedlung zur nächsten Ausbaustufe ausbauen.
    AdvanceEra,
    /// Aufstellung der Einheiten setzen (wirkt auf Kampfwerte und spätere Bewegungsbefehle).
    SetFormation { units: Vec<UnitId>, formation: Formation },
    /// Talentpunkt verteilen (Index im Talentbaum des Helden).
    LearnTalent { unit: UnitId, talent: u16 },
    /// Gegenstand aus dem Inventar anlegen (Index im Inventar).
    EquipItem { unit: UnitId, index: u8 },
    /// Ausrüstung ablegen (Ausrüstungsplatz 0..5).
    UnequipItem { unit: UnitId, slot: u8 },
    /// Gegenstand aus dem Inventar auf den Boden werfen.
    DropItem { unit: UnitId, index: u8 },
    /// Trank aus dem Inventar verwenden.
    UseItem { unit: UnitId, index: u8 },
    /// Schmiede-Upgrade erforschen (Index in `GameData::upgrades`).
    Research { building: BuildingId, upgrade: u8 },
    CancelResearch { building: BuildingId },
    /// Gefallenen Helden am Bergfried oder Altar gegen Gold wiederbeleben.
    ReviveHero { building: BuildingId, hero: UnitTypeId },
}

/// Ein Befehl mit Absender.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCommand {
    pub player: PlayerId,
    pub command: Command,
}
