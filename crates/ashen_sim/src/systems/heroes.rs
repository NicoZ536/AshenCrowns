//! Helden: Werte aus Ausrüstung und Talenten, Inventar, Tränke, Beute, Wiederbelebung.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::rpg::{equip_index, ItemId, StatMods, TalentTree, EQUIP_SLOTS, INVENTORY_SIZE};
use crate::systems::economy;
use crate::world::SimWorld;
use glam::Vec2;
use rand::RngExt;

/// Alle Fähigkeiten einer Einheit: Grundfähigkeiten, danach durch Talente freigeschaltete.
pub fn ability_list<'a>(data: &'a GameData, u: &'a Unit) -> impl Iterator<Item = &'a String> {
    data.unit(u.kind).abilities.iter().chain(u.extra_abilities.iter())
}

/// Höchste erreichbare Heldenstufe.
pub fn max_level(data: &GameData) -> u32 {
    data.rules.xp_levels.len() as u32
}

/// Noch nicht verteilte Talentpunkte (ein Punkt je Stufe über 1).
pub fn free_points(data: &GameData, u: &Unit) -> u32 {
    let spent: u32 = u.talents.iter().map(|r| *r as u32).sum();
    u.level.saturating_sub(1).saturating_sub(spent).min(max_level(data).saturating_sub(1))
}

/// Berechnet die Zusatzwerte eines Helden neu (Ausrüstung, Talente, Stufe) und passt Leben,
/// Mana und Fähigkeitenliste an. Für Nicht-Helden ohne Wirkung.
pub fn recompute(world: &mut SimWorld, data: &GameData, id: UnitId) {
    let Some(u) = world.units.get(id) else { return };
    let def = data.unit(u.kind);
    if def.class != UnitClass::Hero {
        return;
    }
    let tree: Option<&TalentTree> = data.talent_tree(&def.id);
    let mut mods = StatMods::default();
    for it in u.equipment.iter().flatten() {
        mods.add(&data.item_def(*it).mods, 1.0);
    }
    let mut extra: Vec<String> = Vec::new();
    if let Some(tree) = tree {
        for (i, t) in tree.talents.iter().enumerate() {
            let r = u.talents.get(i).copied().unwrap_or(0);
            if r > 0 {
                mods.add(&t.mods, r as f32);
                if let Some(a) = &t.ability {
                    extra.push(a.clone());
                }
            }
        }
    }
    let lvl = u.level.saturating_sub(1) as f32;
    let new_max_hp = (def.hp + lvl * def.hp_per_level + mods.hp).max(1.0);
    let new_max_mana = (def.mana + lvl * 15.0 + mods.mana).max(0.0);
    let n_abilities = def.abilities.len() + extra.len();
    let u = world.units.get_mut(id).expect("exists");
    let dh = new_max_hp - u.max_hp;
    u.max_hp = new_max_hp;
    u.hp = (u.hp + dh).clamp(1.0, new_max_hp);
    let dm = new_max_mana - u.max_mana;
    u.max_mana = new_max_mana;
    u.mana = (u.mana + dm).clamp(0.0, new_max_mana);
    u.mods = mods;
    u.extra_abilities = extra;
    u.ability_cd.resize(n_abilities, 0.0);
}

/// Bereitet einen frisch erzeugten Helden vor (Talentliste, Werte).
pub fn init_hero(world: &mut SimWorld, data: &GameData, id: UnitId) {
    let Some(u) = world.units.get_mut(id) else { return };
    let def = data.unit(u.kind);
    if def.class != UnitClass::Hero {
        return;
    }
    let n = data.talent_tree(&def.id).map(|t| t.talents.len()).unwrap_or(0);
    u.talents = vec![0; n];
    recompute(world, data, id);
}

fn own_hero(world: &SimWorld, data: &GameData, player: PlayerId, id: UnitId) -> bool {
    world.units.get(id).is_some_and(|u| u.owner == player && u.hp > 0.0 && data.unit(u.kind).class == UnitClass::Hero)
}

/// Verteilt einen Talentpunkt.
pub fn learn_talent(world: &mut SimWorld, data: &GameData, player: PlayerId, id: UnitId, talent: usize) {
    if !own_hero(world, data, player, id) {
        return;
    }
    let u = world.units.get(id).expect("checked");
    let def = data.unit(u.kind);
    let Some(tree) = data.talent_tree(&def.id) else { return };
    let Some(t) = tree.talents.get(talent) else { return };
    let ranks = &u.talents;
    let rank = ranks.get(talent).copied().unwrap_or(0);
    if free_points(data, u) == 0 {
        world.error(player, "Keine Talentpunkte verfügbar.");
        return;
    }
    if rank >= t.max_rank {
        world.error(player, "Talent ist bereits auf dem höchsten Rang.");
        return;
    }
    let need = TalentTree::required_points(t);
    if tree.spent_in_path(ranks, t.path) < need {
        world.error(player, format!("Benötigt {need} Punkte in diesem Pfad."));
        return;
    }
    let u = world.units.get_mut(id).expect("checked");
    if u.talents.len() < tree.talents.len() {
        u.talents.resize(tree.talents.len(), 0);
    }
    u.talents[talent] += 1;
    recompute(world, data, id);
    let pos = world.units.get(id).map(|u| u.pos).unwrap_or(Vec2::ZERO);
    world.events.push(SimEvent::TalentLearned { unit: id, talent: t.id.clone(), pos });
}

/// Fügt einen Gegenstand ins Inventar ein. `false`, wenn kein Platz ist.
pub fn give_item(world: &mut SimWorld, id: UnitId, item: ItemId) -> bool {
    let Some(u) = world.units.get_mut(id) else { return false };
    if u.inventory.len() >= INVENTORY_SIZE {
        return false;
    }
    u.inventory.push(item);
    true
}

/// Legt einen Gegenstand aus dem Inventar an; der bisherige kehrt ins Inventar zurück.
pub fn equip(world: &mut SimWorld, data: &GameData, player: PlayerId, id: UnitId, inv_index: usize) {
    if !own_hero(world, data, player, id) {
        return;
    }
    let u = world.units.get(id).expect("checked");
    let Some(&item) = u.inventory.get(inv_index) else { return };
    let slot = data.item_def(item).slot;
    let Some(si) = equip_index(slot, &u.equipment) else {
        world.error(player, "Dieser Gegenstand kann nicht angelegt werden.");
        return;
    };
    let u = world.units.get_mut(id).expect("checked");
    u.inventory.remove(inv_index);
    if let Some(old) = u.equipment[si].replace(item) {
        u.inventory.push(old);
    }
    recompute(world, data, id);
}

/// Legt den Gegenstand eines Ausrüstungsplatzes ab (ins Inventar).
pub fn unequip(world: &mut SimWorld, data: &GameData, player: PlayerId, id: UnitId, slot: usize) {
    if !own_hero(world, data, player, id) || slot >= EQUIP_SLOTS {
        return;
    }
    let u = world.units.get_mut(id).expect("checked");
    if u.equipment[slot].is_none() {
        return;
    }
    if u.inventory.len() >= INVENTORY_SIZE {
        world.error(player, "Inventar voll.");
        return;
    }
    if let Some(it) = u.equipment[slot].take() {
        u.inventory.push(it);
    }
    recompute(world, data, id);
}

/// Wirft einen Gegenstand des Inventars auf den Boden.
pub fn drop_item(world: &mut SimWorld, data: &GameData, player: PlayerId, id: UnitId, inv_index: usize) {
    if !own_hero(world, data, player, id) {
        return;
    }
    let u = world.units.get_mut(id).expect("checked");
    if inv_index >= u.inventory.len() {
        return;
    }
    let item = u.inventory.remove(inv_index);
    let pos = u.pos + Vec2::new(0.6, 0.4);
    world.drops.push(GroundItem { pos, item });
}

/// Verbraucht einen Trank des Inventars.
pub fn use_item(world: &mut SimWorld, data: &GameData, player: PlayerId, id: UnitId, inv_index: usize) {
    if !own_hero(world, data, player, id) {
        return;
    }
    let u = world.units.get(id).expect("checked");
    let Some(&item) = u.inventory.get(inv_index) else { return };
    let def = data.item_def(item);
    let Some(effect) = def.use_effect else {
        world.error(player, "Dieser Gegenstand ist kein Verbrauchsgegenstand.");
        return;
    };
    if u.potion_cd > 0.0 {
        world.error(player, "Ihr könnt noch keinen Trank nehmen.");
        return;
    }
    let u = world.units.get_mut(id).expect("checked");
    u.inventory.remove(inv_index);
    u.potion_cd = 8.0;
    u.hp = (u.hp + effect.heal).min(u.max_hp);
    u.mana = (u.mana + effect.mana).min(u.max_mana);
    if effect.duration > 0.0 {
        u.buffs.push(Buff { damage_mult: if effect.damage_mult > 0.0 { effect.damage_mult } else { 1.0 }, armor_bonus: effect.armor_bonus, remaining: effect.duration });
    }
    let pos = u.pos;
    world.events.push(SimEvent::ItemUsed { unit: id, item, pos });
}

/// Wirft Beute einer gefallenen Einheit ab (Beutetabelle der Einheit, Lagerbeute).
pub fn on_unit_death(world: &mut SimWorld, data: &GameData, dead: &Unit) {
    if dead.owner == PLAYER {
        return;
    }
    let def = data.unit(dead.kind);
    for (id, chance) in &def.loot {
        let Some(item) = data.item_def_id(id) else { continue };
        let roll: f32 = world.rng.random();
        if roll < *chance {
            let jitter = Vec2::new(world.rng.random::<f32>() - 0.5, world.rng.random::<f32>() - 0.5) * 0.8;
            world.drops.push(GroundItem { pos: dead.pos + jitter, item });
            world.events.push(SimEvent::ItemDropped { item, pos: dead.pos });
        }
    }
    if let Some(camp) = &dead.camp
        && !world.units.iter().any(|(_, u)| u.hp > 0.0 && u.camp.as_ref() == Some(camp))
    {
        let table = data.rules.camp_loot.clone();
        let total: f32 = table.iter().map(|(_, w)| *w).sum();
        if total > 0.0 {
            for k in 0..2 {
                let mut r = world.rng.random::<f32>() * total;
                for (id, w) in &table {
                    if r < *w {
                        if let Some(item) = data.item_def_id(id) {
                            world.drops.push(GroundItem { pos: dead.pos + Vec2::new(k as f32 * 0.7 - 0.35, 0.5), item });
                            world.events.push(SimEvent::ItemDropped { item, pos: dead.pos });
                        }
                        break;
                    }
                    r -= *w;
                }
            }
            world.record(ChronicleKind::Battle, "Ein feindliches Lager wurde vernichtet; die Beute liegt am Boden.");
        }
    }
}

/// Helden nehmen Beute in Reichweite auf; untätige Helden in der Nähe laufen hin.
fn pickup_tick(world: &mut SimWorld, data: &GameData) {
    if world.drops.is_empty() {
        return;
    }
    let radius = data.rules.pickup_radius;
    let heroes: Vec<UnitId> = world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && data.unit(u.kind).class == UnitClass::Hero).map(|(id, _)| id).collect();
    for hid in heroes {
        let Some(h) = world.units.get(hid) else { continue };
        let pos = h.pos;
        let idle = h.order == Order::Idle && h.target.is_none();
        let mut i = 0;
        while i < world.drops.len() {
            let d = (world.drops[i].pos - pos).length();
            if d <= radius {
                let item = world.drops[i].item;
                if give_item(world, hid, item) {
                    world.drops.swap_remove(i);
                    world.events.push(SimEvent::ItemPickedUp { unit: hid, item, pos });
                    continue;
                } else if world.tick.is_multiple_of(100) {
                    world.error(PLAYER, "Inventar voll.");
                }
            } else if idle && d <= 7.0 && world.tick.is_multiple_of(10) {
                let goal = world.drops[i].pos;
                if let Some(u) = world.units.get_mut(hid) {
                    u.order = Order::Move { target: goal, attack_move: false };
                    u.request_path(goal);
                }
            }
            i += 1;
        }
    }
}

/// Kosten der Wiederbelebung (Gold).
pub fn revive_cost(data: &GameData, level: u32) -> f32 {
    data.rules.hero_revive_base + data.rules.hero_revive_per_level * level.saturating_sub(1) as f32
}

fn revive_at(world: &mut SimWorld, data: &GameData, index: usize, building: BuildingId, hp_frac: f32) {
    let Some(b) = world.buildings.get(building).cloned() else { return };
    let fallen = world.fallen.remove(index);
    let p = economy::exit_point(world, &b, None);
    let mut u = fallen.unit;
    u.pos = p;
    u.prev_pos = p;
    u.order = Order::Idle;
    u.queue.clear();
    u.path.clear();
    u.target = None;
    u.windup = None;
    u.buffs.clear();
    u.shield = 0.0;
    u.routing = 0.0;
    u.last_attacker = None;
    u.hp = u.max_hp * hp_frac;
    u.mana = u.max_mana * 0.5;
    let id = world.units.insert(u);
    world.events.push(SimEvent::HeroRespawned { unit: id, pos: p });
    economy::recompute_population(world, data);
}

/// Wiederbelebung gegen Gold am Bergfried oder Altar.
pub fn revive(world: &mut SimWorld, data: &GameData, player: PlayerId, building: BuildingId, hero: crate::data::UnitTypeId) {
    let Some(b) = world.buildings.get(building) else { return };
    if b.owner != player || !b.complete {
        return;
    }
    let is_keep = b.tag.as_deref() == Some(world.mission_def.defeat_tag.as_str());
    let is_altar = data.building(b.kind).id == "altar";
    if !is_keep && !is_altar {
        return;
    }
    let Some(idx) = world.fallen.iter().position(|f| f.unit.kind == hero && f.unit.owner == player) else { return };
    let cost = revive_cost(data, world.fallen[idx].unit.level);
    let p = &mut world.players[player as usize];
    if p.resources.gold + 1e-3 < cost {
        world.error(player, "Nicht genug Gold zur Wiederbelebung.");
        return;
    }
    p.resources.gold -= cost;
    let frac = if is_altar { data.rules.hero_altar_hp } else { data.rules.hero_keep_hp };
    revive_at(world, data, idx, building, frac);
}

/// Zähler und automatische Rückkehr gefallener Helden im Bergfried; Beute einsammeln.
pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    for u in world.units.iter_mut().map(|(_, u)| u) {
        if u.potion_cd > 0.0 {
            u.potion_cd = (u.potion_cd - dt).max(0.0);
        }
    }
    pickup_tick(world, data);
    if world.fallen.is_empty() {
        return;
    }
    for f in &mut world.fallen {
        f.timer -= dt;
    }
    let Some(keep) = world.building_with_tag(&world.mission_def.defeat_tag) else { return };
    let ready = world.fallen.iter().position(|f| f.timer <= 0.0);
    if let Some(i) = ready {
        revive_at(world, data, i, keep, data.rules.hero_keep_hp);
    }
}
