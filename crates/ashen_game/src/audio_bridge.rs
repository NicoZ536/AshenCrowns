//! Verbindet Simulationsereignisse mit Audio-Events und steuert die Musikintensität.

use ashen_core::iso::tile_to_world;
use ashen_engine::audio::{AudioSystem, Intensity};
use ashen_engine::render::ViewParams;
use ashen_sim::data::{GameData, ProjectileKind, ResourceKind};
use ashen_sim::entity::{ENEMY, EntityRef, PLAYER};
use ashen_sim::env::Weather;
use ashen_sim::{SimEvent, SimWorld};
use glam::Vec2;

/// Deterministische, gleichmäßig verteilte Auswahl (1 aus n) ohne Zufallszustand.
fn chance(world: &SimWorld, pos: Vec2, n: u32) -> bool {
    let h = ashen_core::noise::hash2(
        (pos.x * 97.0) as i32,
        (pos.y * 89.0) as i32,
        world.tick as u32,
    );
    h.is_multiple_of(n)
}

/// Stereo-Panorama und Dämpfung relativ zur Bildschirmmitte.
fn spatial(view: &ViewParams, tile_pos: Vec2) -> (f32, f32) {
    let w = tile_to_world(tile_pos);
    let size = Vec2::new(view.internal.0 as f32, view.internal.1 as f32);
    let center = view.origin + size * 0.5;
    let d = (w - center) / (size * 0.5);
    let pan = d.x.clamp(-1.0, 1.0) * 0.7;
    let att = ((d.length() - 0.8) / 1.5).clamp(0.0, 0.95);
    (pan, att)
}

pub fn on_event(
    audio: &mut AudioSystem,
    e: &SimEvent,
    world: &SimWorld,
    data: &GameData,
    view: &ViewParams,
) {
    let at = |audio: &mut AudioSystem, id: &str, p: Vec2| {
        let (pan, att) = spatial(view, p);
        if att < 0.95 {
            audio.play_at(id, pan, att);
        }
    };
    match e {
        SimEvent::MeleeSwing { kind, pos, .. } => {
            let id = &data.unit(*kind).id;
            let sfx = match id.as_str() {
                "knight" => {
                    at(audio, "sfx.horse_charge", *pos);
                    "sfx.spear_hit"
                }
                "spearman" | "halberdier" => "sfx.spear_hit",
                "grave_brute" => "sfx.bone_crunch",
                "battering_ram" => "sfx.ram_hit",
                "ghoul" | "bat_swarm" | "hollow_digger" => "sfx.bone_crunch",
                _ => "sfx.sword_hit",
            };
            at(audio, sfx, *pos);
            if chance(world, *pos, 5) {
                at(audio, &format!("voice.{id}.attack"), *pos);
            }
        }
        SimEvent::Hit {
            target,
            pos,
            blocked,
            ..
        } => {
            if *blocked {
                at(audio, "sfx.shield_block", *pos);
            }
            if let EntityRef::Building(b) = target {
                let fort = world
                    .buildings
                    .get(*b)
                    .is_some_and(|x| data.building(x.kind).is_fort());
                at(
                    audio,
                    if fort {
                        "sfx.wall_hit"
                    } else {
                        "sfx.building_hit"
                    },
                    *pos,
                );
            } else if let EntityRef::Unit(u) = target
                && let Some(unit) = world.units.get(*u)
                && data.unit(unit.kind).class == ashen_sim::data::UnitClass::Hero
                && unit.owner == PLAYER
                && unit.hp < unit.max_hp * 0.25
                && chance(world, *pos, 6)
            {
                at(
                    audio,
                    &format!("voice.{}.lowhp", data.unit(unit.kind).id),
                    *pos,
                );
            } else if let EntityRef::Unit(u) = target
                && let Some(unit) = world.units.get(*u)
                && unit.hp < unit.max_hp * 0.3
                && chance(world, *pos, 4)
            {
                at(
                    audio,
                    &format!("voice.{}.hit", data.unit(unit.kind).id),
                    *pos,
                );
            }
        }
        SimEvent::ProjectileFired { kind, from, .. } => match kind {
            ProjectileKind::Arrow | ProjectileKind::BoneBolt => at(audio, "sfx.arrow_fire", *from),
            ProjectileKind::SoulBolt => at(audio, "sfx.reeve_soul_drain", *from),
            ProjectileKind::Bolt => at(audio, "sfx.crossbow_fire", *from),
            ProjectileKind::Fireball => at(audio, "sfx.fire_spell", *from),
            ProjectileKind::Boulder => at(audio, "sfx.catapult_launch", *from),
        },
        SimEvent::Splash { pos, kind, .. } => at(
            audio,
            if *kind == ProjectileKind::Boulder {
                "sfx.boulder_impact"
            } else {
                "sfx.fireball_impact"
            },
            *pos,
        ),
        SimEvent::ProjectileImpact {
            kind: ProjectileKind::Boulder | ProjectileKind::Fireball,
            ..
        } => {}
        SimEvent::ProjectileImpact { pos, hit: true, .. } => at(audio, "sfx.arrow_impact", *pos),
        SimEvent::UnitDied {
            kind, owner, pos, ..
        } => {
            let def = data.unit(*kind);
            at(
                audio,
                if *owner == ENEMY {
                    "sfx.unit_death_hollow"
                } else {
                    "sfx.unit_death_human"
                },
                *pos,
            );
            at(audio, &format!("voice.{}.death", def.id), *pos);
        }
        SimEvent::BuildingPlaced {
            owner: PLAYER, pos, ..
        } => at(audio, "sfx.build_place", *pos),
        SimEvent::BuildingCompleted {
            owner: PLAYER, pos, ..
        } => {
            at(audio, "sfx.build_complete", *pos);
            audio.play("voice.worker.build_done");
        }
        SimEvent::BuildingDestroyed { pos, kind, .. } => at(
            audio,
            if data.building(*kind).is_fort() {
                "sfx.wall_collapse"
            } else {
                "sfx.building_destroyed"
            },
            *pos,
        ),
        SimEvent::ProductionCycle { kind, pos, .. } => {
            let sfx = match data.building(*kind).id.as_str() {
                "smithy" | "armory" | "smelter" => Some("sfx.smithy_hammer"),
                "sawmill" | "woodcutter" => Some("sfx.sawmill_saw"),
                "windmill" | "bakery" | "brewery" => Some("sfx.mill_grind"),
                _ => None,
            };
            if let Some(id) = sfx {
                at(audio, id, *pos);
            }
        }
        SimEvent::Traded { owner: PLAYER, .. } => {
            audio.play("sfx.market_trade");
        }
        SimEvent::GateToggled { pos, open, .. } => at(
            audio,
            if *open {
                "sfx.gate_open"
            } else {
                "sfx.gate_close"
            },
            *pos,
        ),
        SimEvent::HouseChanged {
            pos,
            up: true,
            owner: PLAYER,
            ..
        } => at(audio, "sfx.upgrade_complete", *pos),
        SimEvent::EraAdvanced { owner: PLAYER, .. } => {
            audio.play("sfx.upgrade_complete");
        }
        SimEvent::Unrest { owner: PLAYER } => {
            audio.play("sfx.chapel_bell");
        }
        SimEvent::TrainCompleted {
            kind,
            owner: PLAYER,
            ..
        } => {
            audio.play("sfx.train_complete");
            audio.play(&format!("voice.{}.select", data.unit(*kind).id));
        }
        SimEvent::WorkStroke { resource, pos, .. } => {
            let id = match resource {
                Some(ResourceKind::Wood) => "sfx.wood_chop",
                Some(ResourceKind::Gold) => "sfx.mine_pick",
                Some(ResourceKind::Stone) => "sfx.stone_quarry",
                Some(ResourceKind::Food) => "sfx.harvest_grain",
                None => "sfx.build_progress",
            };
            at(audio, id, *pos);
        }
        SimEvent::AbilityCast {
            ability, pos, unit, ..
        } => {
            let sfx = match ability.as_str() {
                "soul_drain" => "sfx.reeve_soul_drain".to_string(),
                "summon_thralls" | "raise_dead" => "sfx.hollow_summon".to_string(),
                "dread_wail" => "voice.boss_pale_reeve.taunt".to_string(),
                // Fähigkeitsfamilien teilen sich ihre Klänge.
                "fireball" | "flame_nova" | "sacred_flame" | "phoenix_cut" | "pillar_of_fire" => {
                    "sfx.fire_spell".to_string()
                }
                "holy_heal" | "flame_mend" | "resurgence" => "sfx.heal_spell".to_string(),
                "blessing" | "beacon_of_faith" | "war_cry" => "sfx.bless_spell".to_string(),
                "clan_roar" | "war_horns" => "sfx.war_horn".to_string(),
                "shield_wall" | "iron_vow" | "last_stand" | "aegis_of_flame" | "stone_skin" => {
                    "sfx.shield_spell".to_string()
                }
                "ground_slam" | "ancestral_fury" => "sfx.ground_slam".to_string(),
                other => format!("sfx.{other}"),
            };
            at(audio, &sfx, *pos);
            if let Some(u) = world.units.get(*unit) {
                let id = &data.unit(u.kind).id;
                at(audio, &format!("voice.{id}.ability"), *pos);
            }
        }
        SimEvent::LevelUp { unit, .. } => {
            if let Some(u) = world.units.get(*unit) {
                audio.play(&format!("voice.{}.levelup", data.unit(u.kind).id));
            }
            audio.play("sfx.talent_learn");
        }
        SimEvent::TalentLearned { .. } => {
            audio.play("sfx.talent_learn");
        }
        SimEvent::ItemPickedUp { pos, .. } => at(audio, "sfx.item_pickup", *pos),
        SimEvent::ItemUsed { pos, .. } => at(audio, "sfx.potion_drink", *pos),
        SimEvent::UpgradeResearched { owner: PLAYER, .. } => {
            audio.play("sfx.upgrade_complete");
        }
        SimEvent::HeroRespawned { pos, .. } => at(audio, "sfx.hero_revive", *pos),
        SimEvent::WaveIncoming { size, .. } => {
            if *size == 0 {
                audio.play("sfx.horn_alarm");
            }
        }
        SimEvent::Message { voice: Some(v), .. } => {
            audio.play(v);
        }
        SimEvent::ObjectiveAdded { .. } => {
            audio.play("sfx.objective_new");
        }
        SimEvent::ObjectiveCompleted { .. } => {
            audio.play("sfx.objective_complete");
        }
        SimEvent::Error { owner: PLAYER, .. } => {
            audio.play("sfx.ui_error");
        }
        SimEvent::Lightning => {
            audio.play("sfx.thunder");
        }
        SimEvent::Victory => {
            audio.play_sting("victory_sting");
            audio.play("voice.hero_isolde.victory");
        }
        SimEvent::Defeat => {
            audio.play_sting("defeat_sting");
            audio.play("narrator.m01_defeat");
        }
        SimEvent::MoraleBroken { unit, pos } => {
            if let Some(u) = world.units.get(*unit) {
                at(
                    audio,
                    &format!("voice.{}.retreat", data.unit(u.kind).id),
                    *pos,
                );
            }
        }
        SimEvent::HeroFallen { kind, .. } => {
            audio.play(&format!("voice.{}.death", data.unit(*kind).id));
        }
        _ => {}
    }
}

/// Musikintensität aus Kampfgeschehen, Bedrohung und Boss-Nähe.
pub fn music_intensity(world: &SimWorld, data: &GameData) -> Intensity {
    let mut fighting = 0;
    let mut enemies_near_base = 0;
    let mut boss_fight = false;
    let base = world
        .building_with_tag(&world.mission_def.defeat_tag)
        .and_then(|b| world.buildings.get(b))
        .map(|b| b.center());
    for (_, u) in world.units.iter() {
        if u.combat_timer > 0.0 && u.owner == PLAYER {
            fighting += 1;
        }
        if u.owner == ENEMY {
            if let Some(b) = base
                && (u.pos - b).length() < 18.0
            {
                enemies_near_base += 1;
            }
            if data.unit(u.kind).class == ashen_sim::data::UnitClass::Boss && u.combat_timer > 0.0 {
                boss_fight = true;
            }
        }
    }
    if boss_fight {
        Intensity::Boss
    } else if fighting >= 10 || enemies_near_base >= 10 {
        Intensity::Battle
    } else if fighting >= 1 {
        Intensity::Combat
    } else if world.director.enabled && world.director.next_wave_in < 30.0
        || world.env.is_night()
        || enemies_near_base > 0
    {
        Intensity::Tension
    } else {
        Intensity::Calm
    }
}

/// Ambience-Schichten passend zu Tageszeit, Wetter und Kameraposition.
pub fn ambience(audio: &mut AudioSystem, world: &SimWorld, camera_tile: Vec2) {
    let night = world.env.is_night();
    let near_ruins = (camera_tile - ashen_sim::mapgen::ENEMY_BASE).length() < 22.0;
    let near_village = (camera_tile - ashen_sim::mapgen::PLAYER_BASE).length() < 16.0;
    let mut layers: Vec<(&str, f32)> = vec![(
        if night {
            "ambience_night"
        } else {
            "ambience_day"
        },
        1.0,
    )];
    if world.env.weather.is_rainy() {
        layers.push(("ambience_rain", world.env.weather_intensity));
    }
    if world.env.weather == Weather::Ashfall || near_ruins {
        layers.push(("ambience_ash_wind", 0.8));
    }
    if near_ruins {
        layers.push(("ambience_ruins", 0.9));
    }
    if near_village {
        layers.push(("ambience_village", 0.8));
    }
    audio.set_ambience(&layers);
}
