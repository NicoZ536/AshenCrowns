//! Balancemessung des Echtzeit-Gefechts: ein skriptgesteuerter, aktiver Testspieler (Glutkrone) tritt gegen die
//! Gegner-KI der Hohlen an; außerdem 1v1-Gruppenduelle gleicher Kosten für die Konter-Matrix.
//!
//! Der Testspieler spielt wie ein durchschnittlicher aktiver Mensch: Arbeiter ausbilden und Gold, Holz und Stein
//! sammeln lassen, Gehöfte (Bevölkerung) und Militärgebäude bauen, eine gemischte Armee ausbilden, die Basis verteidigen
//! und in Wellen angreifen. Die Waffen- und Rüstungsproduktion (Eisenmine, Schmelze, Schmiede) ersetzt eine feste Lieferung,
//! die einer eingespielten Schmiedekette entspricht (siehe [`BotStyle`]). Er nutzt keine Taktik-KI und keine Heldenfähigkeiten.
//!
//! Werkzeug: `cargo run --release -p ashen_tools -- rts-balance`; Tests in `tests/balance.rs`.

use crate::command::{Command, PlayerCommand};
use crate::data::{GameData, UnitClass, UnitTypeId};
use crate::entity::{BuildingId, NodeKind, Order, UnitId, ENEMY, PLAYER};
use crate::mission::{Difficulty, GameMode, MatchSettings, MissionDef, Outcome};
use crate::systems::commands;
use crate::world::SimWorld;
use glam::{IVec2, Vec2};

/// Spielstil des Testspielers (variiert je Seed).
#[derive(Clone, Copy, Debug)]
pub struct BotStyle {
    /// Heeresgröße des ersten Angriffs.
    pub first_attack: usize,
    /// Zuwachs je weiterem Angriff.
    pub growth: usize,
    /// Arbeiter-Zielzahl.
    pub workers: usize,
    /// Waffen je 10 Sekunden aus der (angenommenen) Schmiedekette.
    pub weapons_per_10s: f32,
    /// Rüstung je 10 Sekunden.
    pub armor_per_10s: f32,
}

impl BotStyle {
    pub fn for_seed(seed: u64) -> BotStyle {
        match seed % 3 {
            0 => BotStyle { first_attack: 12, growth: 4, workers: 12, weapons_per_10s: 1.3, armor_per_10s: 0.8 },
            1 => BotStyle { first_attack: 16, growth: 5, workers: 14, weapons_per_10s: 1.3, armor_per_10s: 0.8 },
            _ => BotStyle { first_attack: 10, growth: 3, workers: 10, weapons_per_10s: 1.3, armor_per_10s: 0.8 },
        }
    }
}

/// Ergebnis eines Gefechts.
#[derive(Clone, Debug)]
pub struct MatchReport {
    pub difficulty: Difficulty,
    pub seed: u64,
    /// `Some(true)` = Testspieler siegt, `Some(false)` = KI siegt, `None` = Zeitlimit.
    pub player_won: Option<bool>,
    pub seconds: f32,
    pub player_units_lost: u32,
    pub enemy_units_lost: u32,
    pub attacks: u32,
}

struct Bot {
    style: BotStyle,
    attacks: u32,
    next_attack_at: f32,
    attacking_until: f32,
    cycle: usize,
    goods_acc: (f32, f32),
}

fn cmd(world: &mut SimWorld, data: &GameData, command: Command) {
    commands::apply(world, data, PlayerCommand { player: PLAYER, command });
}

fn nearest_node(world: &SimWorld, kind: NodeKind, from: Vec2) -> Option<crate::entity::NodeId> {
    world.nodes.iter().filter(|(_, n)| n.kind == kind && n.amount > 0).min_by(|a, b| a.1.distance_to(from).total_cmp(&b.1.distance_to(from))).map(|(id, _)| id)
}

fn count_buildings(world: &SimWorld, id: &str, data: &GameData) -> usize {
    let Some(k) = data.building_id(id) else { return 0 };
    world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.kind == k).count()
}

fn find_site(world: &SimWorld, data: &GameData, id: &str, around: Vec2, toward: Vec2) -> Option<IVec2> {
    let kind = data.building_id(id)?;
    let def = data.building(kind);
    let (w, h) = (def.size.0 as i32, def.size.1 as i32);
    let c = IVec2::new(around.x as i32, around.y as i32);
    let mut best: Option<(f32, IVec2)> = None;
    for r in 3..=16i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let tile = c + IVec2::new(dx, dy) - IVec2::new(w / 2, h / 2);
                let free = (-1..=h).all(|y| (-1..=w).all(|x| world.map.buildable(tile + IVec2::new(x, y))));
                if !free || world.can_place(data, kind, tile, PLAYER).is_err() {
                    continue;
                }
                let score = (toward - tile.as_vec2()).length() * 0.3 + r as f32;
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

fn try_build(world: &mut SimWorld, data: &GameData, id: &str, around: Vec2, toward: Vec2) -> bool {
    let Some(kind) = data.building_id(id) else { return false };
    let def = data.building(kind);
    if world.afford(PLAYER, &def.cost, &def.goods_cost_r).is_err() {
        return false;
    }
    let Some(tile) = find_site(world, data, id, around, toward) else { return false };
    let builder = world
        .units
        .iter()
        .filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Worker && !matches!(u.order, Order::Build { .. }))
        .min_by(|a, b| (a.1.pos - tile.as_vec2()).length().total_cmp(&(b.1.pos - tile.as_vec2()).length()))
        .map(|(i, _)| i);
    let Some(w) = builder else { return false };
    let before = count_buildings(world, id, data);
    cmd(world, data, Command::Build { units: vec![w], kind, tile });
    count_buildings(world, id, data) > before
}

impl Bot {
    fn new(style: BotStyle) -> Bot {
        Bot { style, attacks: 0, next_attack_at: 0.0, attacking_until: 0.0, cycle: 0, goods_acc: (0.0, 0.0) }
    }

    fn think(&mut self, world: &mut SimWorld, data: &GameData, enemy_base: Vec2) {
        let Some(keep_id) = world.building_with_tag(&world.mission_def.defeat_tag) else { return };
        let Some(keep) = world.buildings.get(keep_id).map(|b| b.center()) else { return };
        let now = world.time;
        // Schmiedekette: gleichmäßige Lieferung von Waffen und Rüstung ab Minute 1.
        if now > 60.0 {
            self.goods_acc.0 += self.style.weapons_per_10s * 0.2;
            self.goods_acc.1 += self.style.armor_per_10s * 0.2;
            for (name, acc) in [("weapons", &mut self.goods_acc.0), ("armor", &mut self.goods_acc.1)] {
                if *acc >= 1.0
                    && let Some(g) = data.good_id(name)
                {
                    let add = acc.floor();
                    *acc -= add;
                    let slot = &mut world.players[PLAYER as usize].goods[g.0 as usize];
                    *slot = (*slot + add).min(40.0);
                }
            }
        }
        self.economy(world, data, keep_id, keep);
        self.build(world, data, keep, enemy_base);
        self.train(world, data, keep);
        self.army(world, data, keep, enemy_base);
    }

    fn economy(&mut self, world: &mut SimWorld, data: &GameData, keep_id: BuildingId, keep: Vec2) {
        let workers: Vec<UnitId> = world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && data.unit(u.kind).class == UnitClass::Worker).map(|(i, _)| i).collect();
        let Some(wk) = data.unit_id("worker") else { return };
        let queued = world.buildings.iter().filter(|(_, b)| b.owner == PLAYER).flat_map(|(_, b)| b.queue.iter()).filter(|q| q.unit == wk).count();
        let qlen = world.buildings.get(keep_id).map_or(9, |b| b.queue.len());
        if workers.len() + queued < self.style.workers && qlen < 2 {
            cmd(world, data, Command::Train { building: keep_id, unit: wk });
        }
        let res = world.players[PLAYER as usize].resources;
        let (mut gold_w, mut wood_w, mut stone_w) = (0, 0, 0);
        for &w in &workers {
            if let Some(Order::Gather { node }) = world.units.get(w).map(|u| u.order)
                && let Some(n) = world.nodes.get(node)
            {
                match n.kind {
                    NodeKind::GoldMine => gold_w += 1,
                    NodeKind::Tree => wood_w += 1,
                    NodeKind::StoneQuarry => stone_w += 1,
                }
            }
        }
        for w in workers {
            let Some(u) = world.units.get(w) else { continue };
            if u.order != Order::Idle || !u.queue.is_empty() {
                continue;
            }
            let kind = if res.stone < 60.0 && stone_w < 2 {
                stone_w += 1;
                NodeKind::StoneQuarry
            } else if wood_w <= gold_w {
                wood_w += 1;
                NodeKind::Tree
            } else {
                gold_w += 1;
                NodeKind::GoldMine
            };
            if let Some(node) = nearest_node(world, kind, keep).or_else(|| nearest_node(world, NodeKind::Tree, keep)) {
                cmd(world, data, Command::Gather { units: vec![w], node });
            }
        }
    }

    fn build(&mut self, world: &mut SimWorld, data: &GameData, keep: Vec2, enemy: Vec2) {
        // Nur eine Baustelle zugleich; verwaiste Baustellen bekommen einen Arbeiter.
        let unfinished: Vec<BuildingId> = world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && !b.complete).map(|(i, _)| i).collect();
        for bid in &unfinished {
            let has = world.units.iter().any(|(_, u)| u.owner == PLAYER && matches!(u.order, Order::Build { building } if building == *bid));
            if !has
                && let Some(c) = world.buildings.get(*bid).map(|b| b.center())
                && let Some(w) = world.units.iter().filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Worker).min_by(|a, b| (a.1.pos - c).length().total_cmp(&(b.1.pos - c).length())).map(|(i, _)| i)
            {
                cmd(world, data, Command::Repair { units: vec![w], building: *bid });
            }
        }
        if unfinished.len() >= 2 {
            return;
        }
        let p = &world.players[PLAYER as usize];
        let (pop, cap) = (p.pop, p.pop_cap);
        let res = p.resources;
        let t = world.time;
        let barracks = count_buildings(world, "barracks", data);
        let ranges = count_buildings(world, "shooting_range", data);
        let stables = count_buildings(world, "stable", data);
        let farms = count_buildings(world, "farmstead", data);
        // Bevölkerungsgrenze zuerst.
        if cap < pop + 6 && farms < 8 && try_build(world, data, "farmstead", keep, enemy) {
            return;
        }
        let wanted: [(&str, usize, f32); 5] = [("barracks", 1, 5.0), ("shooting_range", 1, 120.0), ("barracks", 2, 200.0), ("stable", 1, 300.0), ("barracks", 3, 420.0)];
        for (id, n, after) in wanted {
            let have = match id {
                "barracks" => barracks,
                "shooting_range" => ranges,
                _ => stables,
            };
            if have >= n || t < after {
                continue;
            }
            if try_build(world, data, id, keep, enemy) {
                return;
            }
            // Sparen für das nächste Gebäude der Reihenfolge.
            if res.gold < 300.0 {
                return;
            }
        }
        if res.wood > 160.0 && count_buildings(world, "hut", data) < 6 && try_build(world, data, "hut", keep, enemy) {
            return;
        }
        if t > 360.0 && res.gold > 450.0 && res.stone > 100.0 && count_buildings(world, "watchtower", data) < 2 {
            let _ = try_build(world, data, "watchtower", keep, enemy);
        }
    }

    fn train(&mut self, world: &mut SimWorld, data: &GameData, _keep: Vec2) {
        let producers: Vec<(BuildingId, Vec<UnitTypeId>)> = world
            .buildings
            .iter()
            .filter(|(_, b)| b.owner == PLAYER && b.complete && b.queue.len() < 2)
            .filter_map(|(i, b)| {
                let d = data.building(b.kind);
                let army: Vec<UnitTypeId> = d.trains.iter().filter_map(|t| data.unit_id(t)).filter(|u| !matches!(data.unit(*u).class, UnitClass::Worker | UnitClass::Hero | UnitClass::Siege)).collect();
                (!army.is_empty()).then_some((i, army))
            })
            .collect();
        for (bid, units) in producers {
            // Gemischte Armee: die Reihenfolge der Ausbildungsliste wird reihum durchlaufen; Teures nur mit Überschuss.
            self.cycle += 1;
            let list: Vec<UnitTypeId> = units.into_iter().filter(|u| world.afford(PLAYER, &data.unit(*u).cost, &data.unit(*u).goods_cost_r).is_ok()).collect();
            if list.is_empty() {
                continue;
            }
            let pick = list[self.cycle % list.len()];
            cmd(world, data, Command::Train { building: bid, unit: pick });
        }
    }

    fn soldiers(&self, world: &SimWorld, data: &GameData) -> Vec<UnitId> {
        world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && !matches!(data.unit(u.kind).class, UnitClass::Worker | UnitClass::Siege)).map(|(i, _)| i).collect()
    }

    fn army(&mut self, world: &mut SimWorld, data: &GameData, keep: Vec2, enemy_base: Vec2) {
        let soldiers = self.soldiers(world, data);
        let now = world.time;
        // Verteidigung: Feinde in der Nähe der Siedlung werden gemeinsam bekämpft.
        let threat = world.units.iter().filter(|(_, u)| u.owner == ENEMY && u.hp > 0.0 && (u.pos - keep).length() < 20.0).map(|(_, u)| u.pos).next();
        if let Some(t) = threat {
            cmd(world, data, Command::Move { units: soldiers.clone(), target: t, attack_move: true, queue: false });
            self.attacking_until = 0.0;
            return;
        }
        let need = self.style.first_attack + self.attacks as usize * self.style.growth;
        if now >= self.attacking_until && now >= self.next_attack_at && soldiers.len() >= need.min(44) {
            self.attacks += 1;
            self.attacking_until = now + 120.0;
            self.next_attack_at = now + 100.0;
            cmd(world, data, Command::SetFormation { units: soldiers.clone(), formation: crate::rpg::Formation::Block });
            cmd(world, data, Command::Move { units: soldiers, target: enemy_base, attack_move: true, queue: false });
            return;
        }
        if now < self.attacking_until {
            // Untätige Angreifer ziehen weiter zum Ziel.
            let idle: Vec<UnitId> = soldiers.iter().copied().filter(|i| world.units.get(*i).is_some_and(|u| u.order == Order::Idle && (u.pos - enemy_base).length() > 8.0)).collect();
            if !idle.is_empty() {
                cmd(world, data, Command::Move { units: idle, target: enemy_base, attack_move: true, queue: false });
            }
            if soldiers.len() < 4 {
                self.attacking_until = 0.0;
            }
        } else {
            // Sammelpunkt vor der Siedlung.
            let muster = keep + (enemy_base - keep).normalize_or_zero() * 8.0;
            let far: Vec<UnitId> = soldiers.iter().copied().filter(|i| world.units.get(*i).is_some_and(|u| u.order == Order::Idle && (u.pos - muster).length() > 6.0)).collect();
            if !far.is_empty() {
                cmd(world, data, Command::Move { units: far, target: muster, attack_move: true, queue: false });
            }
        }
    }
}

/// Spielt ein Gefecht des Testspielers gegen die KI (Mission `mission`, Modus Gefecht) bis zum Ende oder Zeitlimit.
pub fn play_match(data: &GameData, mission: &MissionDef, difficulty: Difficulty, seed: u64, max_seconds: f32) -> Result<MatchReport, String> {
    let mut w = SimWorld::new(data, mission.clone(), MatchSettings { mode: GameMode::Skirmish, difficulty, seed, hero_permadeath: false })?;
    let enemy_base = w.building_with_tag(&mission.enemy_base_tag).and_then(|b| w.buildings.get(b)).map(|b| b.center()).unwrap_or(Vec2::new(60.0, 12.0));
    let mut bot = Bot::new(BotStyle::for_seed(seed));
    let rate = data.rules.tick_rate as f32;
    let ticks = (max_seconds * rate) as u32;
    let think_every = (rate * 2.0) as u32;
    for t in 0..ticks {
        w.step(data);
        w.events.clear();
        if t % think_every == 0 {
            bot.think(&mut w, data, enemy_base);
        }
        if w.mission.outcome.is_some() {
            break;
        }
    }
    Ok(MatchReport {
        difficulty,
        seed,
        player_won: w.mission.outcome.map(|o| o == Outcome::Victory),
        seconds: w.time,
        player_units_lost: w.players[PLAYER as usize].stats.units_lost,
        enemy_units_lost: w.players[ENEMY as usize].stats.units_lost,
        attacks: w.ai.attacks,
    })
}

/// Ergebnis eines 1v1-Gruppenduells.
#[derive(Clone, Debug)]
pub struct DuelResult {
    pub a: String,
    pub b: String,
    pub count_a: u32,
    pub count_b: u32,
    /// Verbleibende Lebenspunkte als Anteil der Startlebenspunkte.
    pub left_a: f32,
    pub left_b: f32,
}

impl DuelResult {
    /// Sieger: positiv = A, negativ = B (Differenz der Restanteile).
    pub fn margin(&self) -> f32 {
        self.left_a - self.left_b
    }
}

/// Zwei Gruppen gleichen Goldwerts (Gold + Holz + Stein, Nahrung zählt nicht) treffen mit Angriffsbewegung aufeinander.
pub fn duel(data: &GameData, mission: &MissionDef, a: &str, b: &str, budget: f32) -> Result<DuelResult, String> {
    let mut w = SimWorld::new(data, mission.clone(), MatchSettings { mode: GameMode::Sandbox, ..Default::default() })?;
    w.fog.disabled = true;
    let ids: Vec<UnitId> = w.units.iter().map(|(i, _)| i).collect();
    for i in ids {
        w.units.remove(i);
    }
    let bs: Vec<_> = w.buildings.iter().map(|(i, _)| i).collect();
    for i in bs {
        w.buildings.remove(i);
    }
    let (ka, kb) = (data.unit_id(a).ok_or(format!("Einheit {a}"))?, data.unit_id(b).ok_or(format!("Einheit {b}"))?);
    let value = |k: UnitTypeId| {
        let c = data.unit(k).cost;
        (c.gold + c.wood + c.stone).max(1.0)
    };
    let (na, nb) = (((budget / value(ka)).round() as u32).max(1), ((budget / value(kb)).round() as u32).max(1));
    let centre = Vec2::new(36.0, 34.0);
    let place = |w: &mut SimWorld, k: UnitTypeId, n: u32, owner: u8, side: f32| -> Vec<UnitId> {
        (0..n)
            .map(|i| {
                let (col, row) = ((i % 4) as f32, (i / 4) as f32);
                w.spawn_unit(data, k, owner, centre + Vec2::new(side * (6.0 + row * 1.1), (col - 1.5) * 1.1))
            })
            .collect()
    };
    let ua = place(&mut w, ka, na, PLAYER, -1.0);
    let ub = place(&mut w, kb, nb, ENEMY, 1.0);
    for (ids, target) in [(&ua, centre + Vec2::new(8.0, 0.0)), (&ub, centre + Vec2::new(-8.0, 0.0))] {
        for i in ids {
            if let Some(u) = w.units.get_mut(*i) {
                u.order = Order::Move { target, attack_move: true };
                u.request_path(target);
            }
        }
    }
    let total = |w: &SimWorld, ids: &[UnitId]| ids.iter().filter_map(|i| w.units.get(*i)).map(|u| u.hp.max(0.0)).sum::<f32>();
    let (hp_a, hp_b) = (total(&w, &ua), total(&w, &ub));
    for _ in 0..(data.rules.tick_rate * 120) {
        w.step(data);
        w.events.clear();
        if total(&w, &ua) <= 0.0 || total(&w, &ub) <= 0.0 {
            break;
        }
    }
    Ok(DuelResult { a: a.into(), b: b.into(), count_a: na, count_b: nb, left_a: total(&w, &ua) / hp_a.max(1.0), left_b: total(&w, &ub) / hp_b.max(1.0) })
}
