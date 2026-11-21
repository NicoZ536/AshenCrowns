//! Spielerbefehle. Befehle sind reine Daten, damit sie gespeichert, über das Netzwerk
//! gesendet (Lockstep) oder für Wiederholungen aufgezeichnet werden können.

use crate::data::{BuildingTypeId, UnitTypeId};
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
}

/// Ein Befehl mit Absender.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCommand {
    pub player: PlayerId,
    pub command: Command,
}
