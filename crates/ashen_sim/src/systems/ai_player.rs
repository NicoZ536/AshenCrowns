//! KI-Spieler der Hohlen für das Gefecht: baut eine Basis wirtschaftlich auf (Sammeln, Bauen,
//! Ausbilden), stellt Armeen nach Konterlogik zusammen, greift an, verteidigt und zieht sich zurück.
//!
//! Die KI nutzt dieselben Befehle wie der Spieler (`Command`) und dieselben Regeln (Kosten, Bauzeiten,
//! Ausbildungswarteschlangen). Entscheidungen fallen alle `AiLevel::think` Sekunden; Zufall stammt
//! ausschließlich aus dem deterministischen Zufallsgenerator der Partie.

use crate::command::{Command, PlayerCommand};
use crate::data::{AiLevel, ArmorType, BuildingTypeId, Faction, GameData, UnitClass, UnitTypeId};
use crate::entity::*;
use crate::events::SimEvent;
use crate::map::TileMap;
use crate::mission::Difficulty;
use crate::rpg::Formation;
use crate::systems::commands;
use crate::world::SimWorld;
use glam::{IVec2, Vec2};
use rand::RngExt;
use serde::{Deserialize, Serialize};

const AI: PlayerId = ENEMY;
/// Startvorrat der KI (Gold, Holz).
const START_GOLD: f32 = 260.0;
const START_WOOD: f32 = 200.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AiPhase {
    /// Wirtschaft und Armee aufbauen.
    #[default]
    Build,
    /// Ein Angriff läuft.
    Attack,
    /// Rückzug nach verlorenem Gefecht.
    Retreat,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AiState {
    pub enabled: bool,
    /// Index in `Rules::ai_levels`.
    pub level: usize,
    /// Faktor auf eingesammelte Ressourcen (Schwierigkeit).
    pub income_mult: f32,
    pub initialized: bool,
    pub think_in: f32,
    /// Spielzeit der KI in Sekunden.
    pub time: f32,
    pub phase: AiPhase,
    pub next_attack_at: f32,
    pub attacks: u32,
    pub tech_at: f32,
    pub tech_steps: u32,
    pub target: Option<Vec2>,
    pub retreat_until: f32,
    pub defending: bool,
    pub launched_size: u32,
    /// Anzahl der Bauaufträge insgesamt.
    pub builds: u32,
    pub trained: u32,
}

impl AiState {
    pub fn enable(&mut self, data: &GameData, difficulty: Difficulty) {
        let idx = difficulty.index().min(data.rules.ai_levels.len().saturating_sub(1));
        let l = &data.rules.ai_levels[idx];
        self.enabled = true;
        self.level = idx;
        self.income_mult = l.income_mult;
        self.next_attack_at = l.first_attack;
        self.tech_at = l.tech_interval;
    }
}

/// Reihenfolge der Wirtschafts- und Militärgebäude: (Gebäude-ID, Mindestanzahl Arbeiter, Mindestzeit, Zielanzahl).
const BUILD_ORDER: [(&str, usize, f32, u32); 8] = [
    ("bone_pit", 4, 0.0, 1),
    ("ghoul_den", 5, 60.0, 1),
    ("bone_pit", 6, 150.0, 2),
    ("ash_altar", 6, 240.0, 1),
    ("grave_forge", 7, 330.0, 1),
    ("ghoul_den", 8, 420.0, 2),
    ("ash_altar", 8, 520.0, 2),
    ("grave_forge", 9, 640.0, 2),
];

fn cmd(world: &mut SimWorld, data: &GameData, command: Command) {
    commands::apply(world, data, PlayerCommand { player: AI, command });
}

fn base_center(world: &SimWorld) -> Option<(BuildingId, Vec2)> {
    let id = world.building_with_tag(&world.mission_def.enemy_base_tag)?;
    world.buildings.get(id).map(|b| (id, b.center()))
}

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    if !world.ai.enabled {
        return;
    }
    world.ai.time += dt;
    if !world.ai.initialized {
        world.ai.initialized = true;
        let p = &mut world.players[AI as usize];
        p.resources.gold += START_GOLD;
        p.resources.wood += START_WOOD;
    }
    world.ai.think_in -= dt;
    if world.ai.think_in > 0.0 {
        return;
    }
    let lvl = data.rules.ai_levels[world.ai.level.min(data.rules.ai_levels.len() - 1)].clone();
    world.ai.think_in = lvl.think;
    let Some((shrine, center)) = base_center(world) else { return };
    economy_step(world, data, &lvl, shrine, center);
    build_step(world, data, &lvl, center);
    train_step(world, data, &lvl, shrine, center);
    tech_step(world, data, &lvl);
    military_step(world, data, &lvl, center);
}

// ------------------------------------------------------------------------------------------ Wirtschaft

fn worker_kind(data: &GameData) -> Option<UnitTypeId> {
    data.units.iter().position(|u| u.faction == Faction::Hollowed && u.class == UnitClass::Worker).map(|i| UnitTypeId(i as u16))
}

fn economy_step(world: &mut SimWorld, data: &GameData, lvl: &AiLevel, shrine: BuildingId, center: Vec2) {
    let Some(wk) = worker_kind(data) else { return };
    let workers: Vec<UnitId> = world.units.iter().filter(|(_, u)| u.owner == AI && u.kind == wk && u.hp > 0.0).map(|(id, _)| id).collect();
    let queued = world.buildings.iter().filter(|(_, b)| b.owner == AI).flat_map(|(_, b)| b.queue.iter()).filter(|q| q.unit == wk).count();
    let qlen = world.buildings.get(shrine).map(|b| b.queue.len()).unwrap_or(5);
    let def = data.unit(wk);
    if workers.len() + queued < lvl.workers as usize && qlen < 2 && world.afford(AI, &def.cost, &def.goods_cost_r).is_ok() {
        cmd(world, data, Command::Train { building: shrine, unit: wk });
    }
    // Welche Ressource wird bereits eingesammelt?
    let (mut gold_ws, mut wood_ws) = (0usize, 0usize);
    for &w in &workers {
        let Some(u) = world.units.get(w) else { continue };
        let node = match u.order {
            Order::Gather { node } => Some(node),
            Order::ReturnCargo { node } => node,
            _ => None,
        };
        match node.and_then(|n| world.nodes.get(n)).map(|n| n.kind) {
            Some(NodeKind::GoldMine) => gold_ws += 1,
            Some(NodeKind::Tree) => wood_ws += 1,
            _ => {}
        }
    }
    let idle: Vec<UnitId> = workers.iter().copied().filter(|&w| world.units.get(w).is_some_and(|u| u.order == Order::Idle && u.queue.is_empty())).collect();
    let res = world.players[AI as usize].resources;
    // Holzmangel bei Goldüberschuss: Goldsammler zu den Bäumen umschichten.
    if res.wood < 150.0 && res.gold > 400.0 && wood_ws * 2 < workers.len() {
        let movers: Vec<UnitId> = workers
            .iter()
            .copied()
            .filter(|&w| {
                world.units.get(w).is_some_and(|u| {
                    u.carry.is_none() && matches!(u.order, Order::Gather { node } if world.nodes.get(node).is_some_and(|n| n.kind == NodeKind::GoldMine))
                })
            })
            .take(2)
            .collect();
        if let Some(node) = nearest_node(world, NodeKind::Tree, center) {
            for w in movers {
                wood_ws += 1;
                gold_ws = gold_ws.saturating_sub(1);
                cmd(world, data, Command::Gather { units: vec![w], node });
            }
        }
    }
    // Wunschanteil der Holzfäller: bei knappem Holz mehr, bei vollem Lager weniger.
    let share = if res.wood < 120.0 {
        0.7
    } else if res.wood < 300.0 {
        0.5
    } else {
        0.3
    };
    for w in idle {
        let total = (wood_ws + gold_ws).max(1) as f32;
        let prefer_wood = (wood_ws as f32) < share * total + 0.5 && !(res.gold < 60.0 && res.wood > 200.0);
        let kind = if prefer_wood { NodeKind::Tree } else { NodeKind::GoldMine };
        let Some(node) = nearest_node(world, kind, center).or_else(|| nearest_node(world, NodeKind::Tree, center)) else { continue };
        if kind == NodeKind::Tree || world.nodes.get(node).is_some_and(|n| n.kind == NodeKind::Tree) {
            wood_ws += 1;
        } else {
            gold_ws += 1;
        }
        cmd(world, data, Command::Gather { units: vec![w], node });
    }
}

fn nearest_node(world: &SimWorld, kind: NodeKind, from: Vec2) -> Option<NodeId> {
    world.nodes.iter().filter(|(_, n)| n.kind == kind && n.amount > 0).min_by(|a, b| a.1.distance_to(from).total_cmp(&b.1.distance_to(from))).map(|(id, _)| id)
}

// ------------------------------------------------------------------------------------------ Bauen

fn count_buildings(world: &SimWorld, kind: BuildingTypeId) -> u32 {
    world.buildings.iter().filter(|(_, b)| b.owner == AI && b.kind == kind).count() as u32
}

/// Sucht einen freien Bauplatz mit einer Kachel Abstand ringförmig um `around`.
fn find_site(world: &SimWorld, data: &GameData, kind: BuildingTypeId, around: Vec2, r_min: i32, r_max: i32, toward: Option<Vec2>) -> Option<IVec2> {
    let def = data.building(kind);
    let (w, h) = (def.size.0 as i32, def.size.1 as i32);
    let c = IVec2::new(around.x as i32, around.y as i32);
    let mut best: Option<(f32, IVec2)> = None;
    for r in r_min..=r_max {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let tile = c + IVec2::new(dx, dy) - IVec2::new(w / 2, h / 2);
                // Abstand: eine freie Kachel Rand um die Grundfläche.
                let mut free = true;
                'scan: for y in -1..=h {
                    for x in -1..=w {
                        if !world.map.buildable(tile + IVec2::new(x, y)) {
                            free = false;
                            break 'scan;
                        }
                    }
                }
                if !free || world.can_place(data, kind, tile, AI).is_err() {
                    continue;
                }
                let score = toward.map(|t| (t - (tile.as_vec2() + Vec2::new(w as f32, h as f32) * 0.5)).length()).unwrap_or(r as f32) + r as f32 * 0.4;
                if best.is_none_or(|(s, _)| score < s) {
                    best = Some((score, tile));
                }
            }
        }
        if best.is_some() {
            break;
        }
    }
    best.map(|b| b.1)
}

fn nearest_worker(world: &SimWorld, data: &GameData, to: Vec2) -> Option<UnitId> {
    let wk = worker_kind(data)?;
    world
        .units
        .iter()
        .filter(|(_, u)| u.owner == AI && u.kind == wk && u.hp > 0.0 && !matches!(u.order, Order::Build { .. }))
        .min_by(|a, b| (a.1.pos - to).length().total_cmp(&(b.1.pos - to).length()))
        .map(|(id, _)| id)
}

fn try_build(world: &mut SimWorld, data: &GameData, kind: BuildingTypeId, around: Vec2, r_min: i32, r_max: i32, toward: Option<Vec2>) -> bool {
    let def = data.building(kind);
    if world.afford(AI, &def.cost, &def.goods_cost_r).is_err() {
        return false;
    }
    let Some(tile) = find_site(world, data, kind, around, r_min, r_max, toward) else { return false };
    let Some(w) = nearest_worker(world, data, tile.as_vec2()) else { return false };
    let before = count_buildings(world, kind);
    cmd(world, data, Command::Build { units: vec![w], kind, tile });
    let ok = count_buildings(world, kind) > before;
    if ok {
        world.ai.builds += 1;
    }
    ok
}

fn build_step(world: &mut SimWorld, data: &GameData, lvl: &AiLevel, center: Vec2) {
    let Some(wk) = worker_kind(data) else { return };
    let n_workers = world.units.iter().filter(|(_, u)| u.owner == AI && u.kind == wk && u.hp > 0.0).count();
    let player_dir = crate::mapgen::PLAYER_BASE;
    // Ein Bauauftrag nach dem anderen: erst wenn die letzte Baustelle steht, folgt die nächste.
    let unfinished: Vec<BuildingId> = world.buildings.iter().filter(|(_, b)| b.owner == AI && !b.complete).map(|(id, _)| id).collect();
    if !unfinished.is_empty() {
        // Ist der Bauarbeiter gefallen, übernimmt ein anderer die Baustelle.
        for bid in unfinished {
            let has_builder = world.units.iter().any(|(_, u)| u.owner == AI && matches!(u.order, Order::Build { building } if building == bid));
            if !has_builder
                && let Some(c) = world.buildings.get(bid).map(|b| b.center())
                && let Some(w) = nearest_worker(world, data, c)
            {
                cmd(world, data, Command::Repair { units: vec![w], building: bid });
            }
        }
        return;
    }
    // Knochenlager am Vorkommen verkürzt die Wege der Arbeiter.
    if let Some(cache) = data.building_id("bone_cache")
        && n_workers >= 4
        && count_buildings(world, cache) < 2
    {
        let kind = if count_buildings(world, cache) == 0 { NodeKind::GoldMine } else { NodeKind::Tree };
        if let Some(node) = nearest_node(world, kind, center) {
            let nc = world.nodes.get(node).map(|n| n.center()).unwrap_or(center);
            let far = world.buildings.iter().filter(|(_, b)| b.owner == AI && b.complete && data.building(b.kind).dropoff).all(|(_, b)| (b.center() - nc).length() > 9.0);
            if far && try_build(world, data, cache, nc, 2, 5, Some(center)) {
                return;
            }
        }
    }
    for (id, min_workers, min_time, want) in BUILD_ORDER {
        let Some(kind) = data.building_id(id) else { continue };
        if n_workers < min_workers || world.ai.time < min_time * level_pace(lvl) || count_buildings(world, kind) >= want {
            continue;
        }
        if try_build(world, data, kind, center, 4, 14, Some(player_dir)) {
            return;
        }
        // Dieses Gebäude ist noch nicht bezahlbar: sparen, nichts anderes davor bauen.
        return;
    }
    // Überschüsse: zusätzliche Produktionsgebäude, damit Gold nicht ungenutzt liegt.
    let res = world.players[AI as usize].resources;
    if res.gold > 600.0 && res.wood > 250.0 && lvl.max_army >= 24 {
        for id in ["bone_pit", "ghoul_den", "ash_altar", "grave_forge"] {
            let Some(kind) = data.building_id(id) else { continue };
            if count_buildings(world, kind) < 4 && try_build(world, data, kind, center, 4, 16, Some(player_dir)) {
                return;
            }
        }
    }
    // Verteidigungstürme.
    if let Some(tower) = data.building_id("bone_spire") {
        let have = count_buildings(world, tower);
        let existing_defs = world.buildings.iter().filter(|(_, b)| b.owner == AI && b.kind == tower).count() as u32;
        let _ = existing_defs;
        if have < lvl.towers + 2 && world.ai.time > 300.0 * level_pace(lvl) {
            let _ = try_build(world, data, tower, center, 6, 12, Some(player_dir));
        }
    }
}

/// Schwierigere Stufen bauen früher (Skalierung der Mindestzeiten).
fn level_pace(lvl: &AiLevel) -> f32 {
    (lvl.first_attack / 420.0).clamp(0.4, 1.6)
}

// ------------------------------------------------------------------------------------------ Ausbilden

/// Gegnerprofil für die Konterlogik: Anteile je Einheitenmerkmal an der Spielerarmee.
#[derive(Default, Clone, Copy, Debug)]
pub struct Profile {
    pub cavalry: f32,
    pub ranged: f32,
    pub heavy: f32,
    pub medium: f32,
    pub light: f32,
    pub siege: f32,
    pub support: f32,
    pub walls: f32,
}

/// Misst die Spielerarmee (Anteile 0..1) und die Befestigung.
pub fn player_profile(world: &SimWorld, data: &GameData) -> Profile {
    let mut p = Profile::default();
    let mut n = 0.0f32;
    for (_, u) in world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0) {
        let d = data.unit(u.kind);
        if matches!(d.class, UnitClass::Worker) {
            continue;
        }
        n += 1.0;
        match d.class {
            UnitClass::Cavalry => p.cavalry += 1.0,
            UnitClass::Ranged => p.ranged += 1.0,
            UnitClass::Siege => p.siege += 1.0,
            UnitClass::Support => p.support += 1.0,
            _ => {}
        }
        match d.armor_type {
            ArmorType::Heavy | ArmorType::Hero => p.heavy += 1.0,
            ArmorType::Medium => p.medium += 1.0,
            ArmorType::Light | ArmorType::Unarmored => p.light += 1.0,
            ArmorType::Fortified => {}
        }
    }
    if n > 0.0 {
        p.cavalry /= n;
        p.ranged /= n;
        p.heavy /= n;
        p.medium /= n;
        p.light /= n;
        p.siege /= n;
        p.support /= n;
    }
    let forts = world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && (data.building(b.kind).is_fort() || data.building(b.kind).attack.is_some())).count() as f32;
    p.walls = (forts / 12.0).min(1.0);
    p
}

/// Gewicht einer Hohlen-Einheit gegen das Spielerprofil (Konter-Matrix, siehe docs/design/rts_military.md).
pub fn counter_weight(id: &str, p: &Profile) -> f32 {
    match id {
        "hollow_thrall" => 1.0 * (1.0 + 1.5 * p.medium),
        "ghoul" => 0.8 * (1.0 + 2.0 * p.ranged + 1.0 * p.siege + 1.0 * p.support),
        "hollow_archer" => 0.8 * (1.0 + 1.5 * p.light),
        "ash_witch" => 0.5 * (1.0 + 2.0 * p.heavy + 1.5 * p.cavalry),
        "grave_brute" => 0.35 * (1.0 + 4.0 * p.walls + 1.0 * p.heavy),
        "bat_swarm" => 0.45 * (1.0 + 1.5 * p.siege + p.support) * (1.2 - p.ranged).max(0.2),
        _ => 0.5,
    }
}

fn train_step(world: &mut SimWorld, data: &GameData, lvl: &AiLevel, shrine: BuildingId, center: Vec2) {
    // Sammelpunkt vor der Basis, in Richtung des Gegners.
    let toward = (crate::mapgen::PLAYER_BASE - center).try_normalize().unwrap_or(Vec2::new(-1.0, 1.0));
    let muster = world.map.nearest_walkable(TileMap::tile_of(center + toward * 7.0), 6).map(TileMap::center_of).unwrap_or(center);
    let soldiers = ai_soldiers(world, data);
    let queued_soldiers = world.buildings.iter().filter(|(_, b)| b.owner == AI).flat_map(|(_, b)| b.queue.iter()).filter(|q| data.unit(q.unit).class != UnitClass::Worker).count();
    // Die Armee wächst mit dem nächsten Angriffsziel (nicht unbegrenzt): Schwierigkeit über `army_mult`.
    let cap = (attack_size(world, lvl) as f32 * 1.5).round().min(lvl.max_army as f32) as usize;
    if soldiers.len() + queued_soldiers >= cap {
        return;
    }
    let profile = player_profile(world, data);
    let producers: Vec<(BuildingId, Vec<UnitTypeId>)> = world
        .buildings
        .iter()
        .filter(|(id, b)| b.owner == AI && b.complete && *id != shrine && b.queue.len() < 2 && !data.building(b.kind).trains.is_empty())
        .map(|(id, b)| (id, data.building(b.kind).trains.iter().filter_map(|t| data.unit_id(t)).collect()))
        .collect();
    for (bid, units) in producers {
        if let Some(b) = world.buildings.get(bid)
            && b.rally != Some(muster)
        {
            cmd(world, data, Command::SetRally { building: bid, point: muster });
        }
        let options: Vec<UnitTypeId> = units.into_iter().filter(|u| data.unit(*u).class != UnitClass::Worker && world.afford(AI, &data.unit(*u).cost, &data.unit(*u).goods_cost_r).is_ok()).collect();
        if options.is_empty() {
            continue;
        }
        let pick = if world.rng.random::<f32>() < lvl.counter_skill {
            let weights: Vec<f32> = options.iter().map(|u| counter_weight(&data.unit(*u).id, &profile).powi(2)).collect();
            weighted(world, &weights)
        } else {
            world.rng.random_range(0..options.len())
        };
        cmd(world, data, Command::Train { building: bid, unit: options[pick] });
        world.ai.trained += 1;
    }
    // Der Schrein bildet Notreserven aus, solange keine Produktionsgebäude stehen.
    if world.ai.builds == 0 && world.ai.time > 200.0 {
        let t = data.unit_id("hollow_thrall");
        if let (Some(t), Some(b)) = (t, world.buildings.get(shrine)) {
            let ok = b.queue.is_empty() && data.building(b.kind).trains.iter().any(|x| *x == data.unit(t).id);
            if ok && world.afford(AI, &data.unit(t).cost, &data.unit(t).goods_cost_r).is_ok() {
                cmd(world, data, Command::Train { building: shrine, unit: t });
            }
        }
    }
}

/// Benötigte Armeegröße für den nächsten Angriff.
fn attack_size(world: &SimWorld, lvl: &AiLevel) -> usize {
    (((8 + world.ai.attacks * 3) as f32) * lvl.army_mult).round().clamp(3.0, lvl.max_army as f32) as usize
}

fn weighted(world: &mut SimWorld, weights: &[f32]) -> usize {
    let total: f32 = weights.iter().sum();
    if total <= 0.0 {
        return 0;
    }
    let mut r = world.rng.random::<f32>() * total;
    for (i, w) in weights.iter().enumerate() {
        if r < *w {
            return i;
        }
        r -= *w;
    }
    weights.len() - 1
}

// ------------------------------------------------------------------------------------------ Schmiede

fn tech_step(world: &mut SimWorld, data: &GameData, lvl: &AiLevel) {
    if world.ai.time < world.ai.tech_at || world.ai.tech_steps >= 9 {
        return;
    }
    world.ai.tech_at = world.ai.time + lvl.tech_interval;
    // Abwechselnd Waffen und Rüstung (die ersten beiden Upgrades), je höchstens Stufe 3.
    let idx = (world.ai.tech_steps % (data.upgrades.len().min(3) as u32).max(1)) as usize;
    world.ai.tech_steps += 1;
    if let Some(up) = data.upgrades.get(idx) {
        let cur = world.players[AI as usize].upgrades.get(idx).copied().unwrap_or(0) as usize;
        if cur < up.levels.len() {
            world.players[AI as usize].upgrades[idx] += 1;
            world.events.push(SimEvent::UpgradeResearched { owner: AI, upgrade: up.id.clone(), level: (cur + 1) as u8 });
        }
    }
}

// ------------------------------------------------------------------------------------------ Militär

/// Soldaten der KI: keine Arbeiter, keine Wachen der Mission, keine Wellen, keine Diener, keine Bosse.
pub fn ai_soldiers(world: &SimWorld, data: &GameData) -> Vec<UnitId> {
    world
        .units
        .iter()
        .filter(|(_, u)| {
            let c = data.unit(u.kind).class;
            u.owner == AI && u.hp > 0.0 && !matches!(c, UnitClass::Worker | UnitClass::Boss) && u.wave.is_none() && u.master.is_none() && !matches!(u.order, Order::Guard { .. }) && u.tag.is_none()
        })
        .map(|(id, _)| id)
        .collect()
}

fn power(world: &SimWorld, data: &GameData, id: UnitId) -> f32 {
    let Some(u) = world.units.get(id) else { return 0.0 };
    let d = data.unit(u.kind);
    let dps = (d.damage_min + d.damage_max) * 0.5 / d.cooldown.max(0.3);
    (u.hp.max(0.0) * dps).sqrt()
}

fn centroid(world: &SimWorld, ids: &[UnitId]) -> Vec2 {
    let pts: Vec<Vec2> = ids.iter().filter_map(|&i| world.units.get(i).map(|u| u.pos)).collect();
    if pts.is_empty() { Vec2::ZERO } else { pts.iter().sum::<Vec2>() / pts.len() as f32 }
}

fn set_formation(world: &mut SimWorld, data: &GameData, ids: &[UnitId], f: Formation) {
    cmd(world, data, Command::SetFormation { units: ids.to_vec(), formation: f });
}

/// Angriffsziel: nächstes Spielergebäude zur Basis (Albtraum zuerst Arbeiter und Außenposten).
fn pick_target(world: &SimWorld, data: &GameData, lvl: &AiLevel, from: Vec2) -> Option<Vec2> {
    let _ = data;
    let mut best: Option<(f32, Vec2)> = None;
    for (_, b) in world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.hp > 0.0) {
        let mut d = (b.center() - from).length();
        // Mauern und Tore sind keine Ziele an sich.
        if data.building(b.kind).is_fort() {
            d += 30.0;
        }
        if lvl.harass && data.building(b.kind).dropoff {
            d += 10.0;
        }
        if best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, b.center()));
        }
    }
    best.map(|b| b.1).or_else(|| world.units.iter().find(|(_, u)| u.owner == PLAYER).map(|(_, u)| u.pos))
}

fn military_step(world: &mut SimWorld, data: &GameData, lvl: &AiLevel, center: Vec2) {
    let soldiers = ai_soldiers(world, data);
    let now = world.ai.time;
    // Diener der Beschwörer folgen ihrem Herrn.
    let followers: Vec<(UnitId, Vec2)> = world
        .units
        .iter()
        .filter(|(_, u)| u.owner == AI && u.hp > 0.0)
        .filter_map(|(id, u)| u.master.and_then(|m| world.units.get(m)).map(|m| (id, m.pos, u.pos)))
        .filter(|(_, mp, up)| (*mp - *up).length() > 7.0)
        .map(|(id, mp, _)| (id, mp))
        .collect();
    for (id, mp) in followers {
        if let Some(u) = world.units.get_mut(id) {
            u.order = Order::Move { target: mp, attack_move: true };
            u.request_path(mp);
        }
    }

    // Verteidigung: Spielereinheiten nahe der Basis werden gemeinsam bekämpft.
    let threats: Vec<Vec2> = world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && (u.pos - center).length() < 16.0).map(|(_, u)| u.pos).collect();
    if !threats.is_empty() && world.ai.phase != AiPhase::Attack {
        let t = threats.iter().sum::<Vec2>() / threats.len() as f32;
        world.ai.defending = true;
        for &id in &soldiers {
            if let Some(u) = world.units.get_mut(id)
                && !matches!(u.order, Order::Move { attack_move: true, .. })
            {
                u.order = Order::Move { target: t, attack_move: true };
                u.request_path(t);
            }
        }
        return;
    }
    if world.ai.defending && threats.is_empty() {
        world.ai.defending = false;
        for &id in &soldiers {
            if let Some(u) = world.units.get_mut(id)
                && let Some(p) = world.map.nearest_walkable(TileMap::tile_of(center + Vec2::new(-4.0, 6.0)), 6)
            {
                u.order = Order::Move { target: TileMap::center_of(p), attack_move: true };
                u.request_path(TileMap::center_of(p));
            }
        }
    }

    match world.ai.phase {
        AiPhase::Build => {
            let need = attack_size(world, lvl);
            if now >= world.ai.next_attack_at && soldiers.len() >= need {
                launch_attack(world, data, lvl, &soldiers, center);
            }
        }
        AiPhase::Attack => {
            if soldiers.len() < 3 {
                end_attack(world, lvl);
                return;
            }
            let c = centroid(world, &soldiers);
            // Kräftevergleich in der Nähe des Heeres.
            let my: f32 = soldiers.iter().filter(|&&s| world.units.get(s).is_some_and(|u| (u.pos - c).length() < 14.0)).map(|&s| power(world, data, s)).sum();
            let theirs: f32 = world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && (u.pos - c).length() < 12.0 && data.unit(u.kind).class != UnitClass::Worker).map(|(id, _)| power(world, data, id)).sum();
            if theirs > 1.0 && my < theirs * lvl.retreat_ratio {
                world.ai.phase = AiPhase::Retreat;
                world.ai.retreat_until = now + 25.0;
                for &id in &soldiers {
                    if let Some(u) = world.units.get_mut(id) {
                        u.order = Order::Move { target: center + Vec2::new(-3.0, 6.0), attack_move: false };
                        u.request_path(center + Vec2::new(-3.0, 6.0));
                    }
                }
                world.record(crate::chronicle::ChronicleKind::Battle, "Die Hohlen wichen vor der Übermacht zurück.");
                return;
            }
            // Ziel erreicht oder zerstört: nächstes Ziel suchen.
            let idle = soldiers.iter().filter(|&&s| world.units.get(s).is_some_and(|u| u.order == Order::Idle && u.target.is_none())).count();
            if idle * 2 >= soldiers.len() {
                match pick_target(world, data, lvl, c) {
                    Some(t) => {
                        world.ai.target = Some(t);
                        for &id in &soldiers {
                            if let Some(u) = world.units.get_mut(id) {
                                u.order = Order::Move { target: t, attack_move: true };
                                u.request_path(t);
                            }
                        }
                    }
                    None => end_attack(world, lvl),
                }
            }
        }
        AiPhase::Retreat => {
            if now >= world.ai.retreat_until {
                end_attack(world, lvl);
            }
        }
    }
}

fn end_attack(world: &mut SimWorld, lvl: &AiLevel) {
    world.ai.phase = AiPhase::Build;
    world.ai.attacks += 1;
    world.ai.next_attack_at = world.ai.time + lvl.attack_interval;
    world.ai.target = None;
}

fn launch_attack(world: &mut SimWorld, data: &GameData, lvl: &AiLevel, soldiers: &[UnitId], center: Vec2) {
    let Some(target) = pick_target(world, data, lvl, center) else { return };
    world.ai.phase = AiPhase::Attack;
    world.ai.target = Some(target);
    world.ai.launched_size = soldiers.len() as u32;
    set_formation(world, data, soldiers, if lvl.name == "Leicht" { Formation::Block } else { Formation::Wedge });
    // Albtraum: schnelle Einheiten belästigen zuerst Arbeiter.
    let mut main: Vec<UnitId> = soldiers.to_vec();
    if lvl.harass {
        let fast: Vec<UnitId> = soldiers.iter().copied().filter(|&s| world.units.get(s).is_some_and(|u| data.unit(u.kind).speed >= 3.2)).collect();
        let worker_pos = world.units.iter().filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Worker).map(|(_, u)| u.pos).next();
        if let Some(wp) = worker_pos {
            for &f in &fast {
                if let Some(u) = world.units.get_mut(f) {
                    u.order = Order::Move { target: wp, attack_move: true };
                    u.request_path(wp);
                }
            }
            main.retain(|m| !fast.contains(m));
        }
    }
    // Die Hauptgruppe sammelt sich als Block auf dem Weg: Befehl mit Formationsplätzen.
    cmd_move_group(world, data, &main, target);
    let n = soldiers.len() as u32;
    world.events.push(SimEvent::WaveIncoming { wave: world.ai.attacks + 1, from: center, size: 0 });
    world.events.push(SimEvent::WaveIncoming { wave: world.ai.attacks + 1, from: center, size: n });
    world.record(crate::chronicle::ChronicleKind::Battle, format!("Die Hohlen marschierten mit {n} Kriegern gegen Brennhollow."));
}

/// Gruppenbefehl der KI: Angriffsbewegung mit Formationsplätzen (wie ein Spielerbefehl).
fn cmd_move_group(world: &mut SimWorld, data: &GameData, ids: &[UnitId], target: Vec2) {
    cmd(world, data, Command::Move { units: ids.to_vec(), target, attack_move: true, queue: false });
}
