//! # ashen_sim – deterministische RTS-Simulation von ASHEN CROWNS
//!
//! Die Simulation ist vollständig vom Rendering entkoppelt: sie kennt weder Fenster noch GPU.
//! Sie läuft mit festem Takt (`Rules::tick_rate`), verarbeitet Befehle als Daten und meldet
//! alles Sichtbare/Hörbare als [`events::SimEvent`].

pub mod arena;
pub mod balance;
pub mod campaign;
pub mod challenge;
pub mod chronicle;
pub mod command;
pub mod data;
pub mod entity;
pub mod env;
pub mod events;
pub mod fog;
pub mod map;
pub mod genmap;
pub mod mapgen;
pub mod mission;
pub mod path;
pub mod rpg;
pub mod save;
pub mod scenario;
pub mod spatial;
pub mod systems;
pub mod world;

pub use command::{Command, PlayerCommand};
pub use data::GameData;
pub use events::SimEvent;
pub use world::SimWorld;
