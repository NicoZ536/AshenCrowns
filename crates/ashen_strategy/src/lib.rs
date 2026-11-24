//! # ashen_strategy – Grand-Campaign-Simulation von ASHEN CROWNS (Strategieebene „Weltkarte“)
//!
//! Vollständig deterministisch (ChaCha8, Befehle als Daten) und ohne Grafik. Die Welt Eldraeth besteht
//! aus 50–64 Provinzen in den acht Kanon-Regionen; Reiche, Dynastien, Adelshäuser, Wirtschaft,
//! Bevölkerung, Religion, Magie, Ereignisse, Bedrohungen, Endgame und Chronik laufen im Tagestakt.
//!
//! Einstieg: [`StrategyWorld::new`] mit [`StrategyOptions`], dann [`StrategyWorld::tick_day`] und
//! [`StrategyWorld::execute`] mit einem [`StrategyCommand`]. Schlachten lassen sich automatisch lösen
//! oder über [`BattleRequest`]/[`BattleResult`] an die Echtzeitebene übergeben. Siehe
//! `docs/tech/strategy_layer.md`.

// Indexschleifen über mehrere parallele Provinz-/Reichstabellen sind hier bewusst gewählt.
#![allow(clippy::needless_range_loop)]

pub mod ai;
pub mod catalog;
pub mod character;
pub mod chronicle;
pub mod command;
pub mod data;
pub mod diplomacy;
pub mod economy;
pub mod endgame;
pub mod events;
pub mod goods;
pub mod magic;
pub mod mapgen;
pub mod military;
pub mod model;
pub mod neutral;
pub mod observe;
pub mod politics;
pub mod population;
pub mod religion;
pub mod save;
pub mod scenario;
pub mod types;
pub mod world;

pub use ai::PeaceOffer;
pub use chronicle::{Chronicle, ChronicleEntry, ChronicleKind};
pub use command::{CommandResult, LawChange, StrategyCommand};
pub use data::StrategyData;
pub use diplomacy::{DiplomacyOutcome, DiplomaticAction, PeaceTerms};
pub use endgame::{VictoryKind, VictoryProgress, VictoryRecord};
pub use goods::Good;
pub use military::{ArmyResult, BattleArmy, BattleRequest, BattleResult, BattleSide, HeroOutcome, Weather};
pub use model::*;
pub use politics::HouseAction;
pub use types::*;
pub use world::{BattleMode, Challenge, Difficulty, DifficultyMods, GameMode, PendingEvent, PlayerStart, RuleSet, StrategyEvent, StrategyOptions, StrategyWorld, WorldStats};
