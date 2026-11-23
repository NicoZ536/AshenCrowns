//! Tests des Echtzeit-Militärs: Konter, Belagerung, Magie, Helden, Formationen, Upgrades, Gegner-KI.

use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{GameData, UnitClass};
use ashen_sim::entity::{EntityRef, Order, UnitId, ENEMY, PLAYER};
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings, MissionDef};
use ashen_sim::rpg::Formation;
use ashen_sim::world::SimWorld;
use ashen_sim::SimEvent;
use glam::Vec2;
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data")
}

fn setup(mode: GameMode, difficulty: Difficulty) -> (GameData, SimWorld) {
    let data = GameData::load_dir(&assets()).expect("data");
    let mission = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).expect("mission");
    let world = SimWorld::new(&data, mission, MatchSettings { mode, difficulty, ..Default::default() }).expect("world");
    (data, world)
}

fn sandbox() -> (GameData, SimWorld) {
    setup(GameMode::Sandbox, Difficulty::Normal)
}

fn run(w: &mut SimWorld, data: &GameData, ticks: u32) {
    for _ in 0..ticks {
        w.step(data);
        w.events.clear();
    }
}

/// Läuft und sammelt alle Ereignisse.
fn run_events(w: &mut SimWorld, data: &GameData, ticks: u32) -> Vec<SimEvent> {
    let mut all = Vec::new();
    for _ in 0..ticks {
        w.step(data);
        all.append(&mut w.events);
    }
    all
}

fn spawn(w: &mut SimWorld, data: &GameData, id: &str, owner: u8, pos: Vec2) -> UnitId {
    let k = data.unit_id(id).unwrap_or_else(|| panic!("unbekannte Einheit {id}"));
    w.spawn_unit(data, k, owner, pos)
}

fn cmd(w: &mut SimWorld, player: u8, command: Command) {
    w.push_command(PlayerCommand { player, command });
}

/// Ein freier Kampfplatz in der Kartenmitte.
const ARENA: Vec2 = Vec2::new(36.0, 34.0);

fn hp_sum(w: &SimWorld, ids: &[UnitId]) -> f32 {
    ids.iter().filter_map(|i| w.units.get(*i)).map(|u| u.hp.max(0.0)).sum()
}

fn alive(w: &SimWorld, ids: &[UnitId]) -> usize {
    ids.iter().filter(|i| w.units.get(**i).is_some_and(|u| u.hp > 0.0)).count()
}

fn clear_hostiles(w: &mut SimWorld) {
    // Missionseinheiten der Hohlen entfernen, damit nichts den Test stört.
    let ids: Vec<UnitId> = w.units.iter().filter(|(_, u)| u.owner == ENEMY).map(|(id, _)| id).collect();
    for id in ids {
        w.units.remove(id);
    }
}

// ------------------------------------------------------------------------------------ Konter

#[test]
fn halberdier_beats_knight() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let mut halb = Vec::new();
    let mut knights = Vec::new();
    for i in 0..5 {
        halb.push(spawn(&mut w, &data, "halberdier", PLAYER, ARENA + Vec2::new(0.0, i as f32 * 0.9)));
        knights.push(spawn(&mut w, &data, "knight", ENEMY, ARENA + Vec2::new(9.0, i as f32 * 0.9)));
    }
    cmd(&mut w, PLAYER, Command::Move { units: halb.clone(), target: ARENA + Vec2::new(9.0, 2.0), attack_move: true, queue: false });
    for &k in &knights {
        let u = w.units.get_mut(k).unwrap();
        u.order = Order::Move { target: ARENA + Vec2::new(0.0, 2.0), attack_move: true };
        u.request_path(ARENA + Vec2::new(0.0, 2.0));
    }
    run(&mut w, &data, 20 * 40);
    let (h, k) = (alive(&w, &halb), alive(&w, &knights));
    assert!(h > k, "Hellebardiere {h} gegen Ritter {k}: die Anti-Kavallerie muss gewinnen");
    assert!(hp_sum(&w, &halb) > hp_sum(&w, &knights) * 1.3, "deutlicher Vorteil");
}

#[test]
fn counter_matrix_is_consistent() {
    let (data, _) = sandbox();
    let u = |id: &str| data.unit(data.unit_id(id).unwrap());
    // Anti-Kavallerie: Hellebardier > Speerträger > Normalfall.
    assert!(u("halberdier").bonus_against(UnitClass::Cavalry) > u("spearman").bonus_against(UnitClass::Cavalry));
    assert!(u("spearman").bonus_against(UnitClass::Cavalry) >= 2.0);
    // Ritter jagen Fernkämpfer, Ghule jagen Fernkämpfer und Heiler.
    assert!(u("knight").bonus_against(UnitClass::Ranged) > 1.0);
    assert!(u("ghoul").bonus_against(UnitClass::Support) > 1.0);
    // Armbrust gegen Panzer und Flieger.
    assert!(u("crossbowman").bonus_against(UnitClass::Flyer) > 1.0);
    // Feuer verbrennt leichte Ziele stärker als Panzer.
    use ashen_sim::data::{ArmorType, DamageType};
    assert!(DamageType::Fire.multiplier(ArmorType::Light) > DamageType::Fire.multiplier(ArmorType::Heavy));
    // Magie bricht schwere Rüstung, Pfeile nicht.
    assert!(DamageType::Magic.multiplier(ArmorType::Heavy) > DamageType::Pierce.multiplier(ArmorType::Heavy));
    // Belagerung gegen Befestigung am stärksten.
    assert!(DamageType::Siege.multiplier(ArmorType::Fortified) > DamageType::Pierce.multiplier(ArmorType::Fortified));
}

#[test]
fn flyers_are_only_hit_by_ranged_and_long_weapons() {
    let (data, _) = sandbox();
    let bat = data.unit(data.unit_id("bat_swarm").unwrap());
    let can = |id: &str| ashen_sim::systems::units::can_hit(data.unit(data.unit_id(id).unwrap()), bat);
    assert!(!can("swordsman"));
    assert!(!can("knight"));
    assert!(can("archer"));
    assert!(can("crossbowman"));
    assert!(can("halberdier"));
    assert!(can("spearman"));
}

#[test]
fn shieldbearer_blocks_arrows() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let shield = spawn(&mut w, &data, "shieldbearer", PLAYER, ARENA);
    let sword = spawn(&mut w, &data, "swordsman", PLAYER, ARENA + Vec2::new(3.0, 0.0));
    // Beide haben Armortyp Heavy, aber der Schildträger hat 5 statt 3 Rüstung und 40 % Pfeilabwehr.
    let hp0 = (w.units.get(shield).unwrap().hp, w.units.get(sword).unwrap().hp);
    for id in [shield, sword] {
        w.deal_damage(&data, EntityRef::Unit(id), 100.0, ashen_sim::data::DamageType::Pierce, &[], None);
    }
    let (ds, dw) = (hp0.0 - w.units.get(shield).unwrap().hp, hp0.1 - w.units.get(sword).unwrap().hp);
    assert!(ds < dw * 0.7, "Schildträger nimmt weniger Pfeilschaden: {ds:.1} gegen {dw:.1}");
}

// ------------------------------------------------------------------------------------ Belagerung

#[test]
fn catapult_destroys_buildings_with_splash() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let cache = data.building_id("bone_cache").unwrap();
    let bid = w.place_building(&data, cache, ENEMY, glam::IVec2::new(36, 30), true);
    let center = w.buildings.get(bid).unwrap().center();
    let hp0 = w.buildings.get(bid).unwrap().hp;
    // Drei Hohle stehen (haltend) an der Mauer des Gebäudes: Flächenschaden trifft sie mit.
    let mut thralls = Vec::new();
    for i in 0..3 {
        let t = spawn(&mut w, &data, "hollow_thrall", ENEMY, center + Vec2::new(1.6, -0.6 + i as f32 * 0.6));
        w.units.get_mut(t).unwrap().order = Order::Hold;
        thralls.push(t);
    }
    let cat = spawn(&mut w, &data, "catapult", PLAYER, center + Vec2::new(-8.5, 0.5));
    cmd(&mut w, PLAYER, Command::Attack { units: vec![cat], target: EntityRef::Building(bid) });
    let events = run_events(&mut w, &data, 20 * 25);
    let fired = events.iter().filter(|e| matches!(e, SimEvent::ProjectileFired { kind: ashen_sim::data::ProjectileKind::Boulder, .. })).count();
    assert!(fired >= 2, "Katapult feuert Felsbrocken ({fired})");
    let hp1 = w.buildings.get(bid).map(|b| b.hp).unwrap_or(0.0);
    assert!(hp1 < hp0 * 0.4, "Gebäude schwer beschädigt oder zerstört: {hp0} -> {hp1}");
    let hurt = thralls.iter().filter_map(|t| w.units.get(*t)).any(|u| u.hp < 200.0 - 20.0) || alive(&w, &thralls) < 3;
    assert!(hurt, "Flächenschaden traf auch die Einheiten am Gebäude");
    // Bogenschützen richten gegen Gebäude kaum etwas aus (Pierce gegen Fortified ×0.35).
    let (data2, mut w2) = sandbox();
    clear_hostiles(&mut w2);
    let b2 = w2.place_building(&data2, data2.building_id("bone_cache").unwrap(), ENEMY, glam::IVec2::new(36, 30), true);
    let archers: Vec<UnitId> = (0..2).map(|i| spawn(&mut w2, &data2, "archer", PLAYER, center + Vec2::new(-4.0, i as f32 * 0.8))).collect();
    cmd(&mut w2, PLAYER, Command::Attack { units: archers, target: EntityRef::Building(b2) });
    run(&mut w2, &data2, 20 * 25);
    let lost = w2.buildings.get(b2).map(|b| hp0 - b.hp).unwrap_or(hp0);
    assert!(lost < hp0 - hp1, "Katapult ist gegen Gebäude deutlich wirksamer als Bogenschützen");
}

#[test]
fn catapult_has_minimum_range() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let cat = spawn(&mut w, &data, "catapult", PLAYER, ARENA);
    let target = spawn(&mut w, &data, "hollow_thrall", ENEMY, ARENA + Vec2::new(2.0, 0.0));
    w.units.get_mut(target).unwrap().order = Order::Hold;
    let min_range = data.unit(data.unit_id("catapult").unwrap()).min_range;
    cmd(&mut w, PLAYER, Command::Attack { units: vec![cat], target: EntityRef::Unit(target) });
    // Zu nahe: nach einer Sekunde darf noch kein Felsbrocken geflogen sein.
    let early = run_events(&mut w, &data, 20);
    assert!(!early.iter().any(|e| matches!(e, SimEvent::ProjectileFired { .. })), "innerhalb der Mindestreichweite wird nicht geschossen");
    // Danach weicht das Katapult zurück und feuert erst aus ausreichender Entfernung.
    let later = run_events(&mut w, &data, 20 * 12);
    let from = later.iter().find_map(|e| match e {
        SimEvent::ProjectileFired { from, .. } => Some(*from),
        _ => None,
    });
    let from = from.expect("Katapult feuert nach dem Zurückweichen");
    let tpos = w.units.get(target).map(|u| u.pos).unwrap_or(ARENA + Vec2::new(2.0, 0.0));
    assert!((from - tpos).length() >= min_range - 0.6, "Schussentfernung {:.2} ≥ Mindestreichweite {min_range}", (from - tpos).length());
}

#[test]
fn ram_breaks_walls_fast() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let wall = data.building_id("wall").unwrap();
    let bid = w.place_building(&data, wall, ENEMY, glam::IVec2::new(40, 30), true);
    let ram = spawn(&mut w, &data, "battering_ram", PLAYER, Vec2::new(38.0, 30.5));
    let sword = spawn(&mut w, &data, "swordsman", PLAYER, Vec2::new(38.0, 31.5));
    let hp0 = w.buildings.get(bid).unwrap().hp;
    cmd(&mut w, PLAYER, Command::Attack { units: vec![ram], target: EntityRef::Building(bid) });
    run(&mut w, &data, 20 * 12);
    let after_ram = w.buildings.get(bid).map(|b| b.hp).unwrap_or(0.0);
    assert!(after_ram < hp0 * 0.7, "Rammbock schädigt die Mauer schwer: {hp0} -> {after_ram}");
    let _ = sword;
}

// ------------------------------------------------------------------------------------ Magie und Beschwörung

#[test]
fn priest_heals_wounded_allies_automatically() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let sword = spawn(&mut w, &data, "swordsman", PLAYER, ARENA);
    let priest = spawn(&mut w, &data, "priest", PLAYER, ARENA + Vec2::new(1.5, 0.0));
    w.units.get_mut(sword).unwrap().hp = 90.0;
    w.units.get_mut(priest).unwrap().mana = 140.0;
    let events = run_events(&mut w, &data, 20 * 4);
    assert!(events.iter().any(|e| matches!(e, SimEvent::Healed { amount, .. } if *amount > 20.0)), "Heilung wurde gewirkt");
    assert!(w.units.get(sword).unwrap().hp > 160.0, "Verwundeter wurde geheilt: {}", w.units.get(sword).unwrap().hp);
    assert!(w.units.get(priest).unwrap().mana < 140.0, "Heilung kostet Mana");
}

#[test]
fn ash_witch_summons_and_servants_vanish_with_her() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let witch = spawn(&mut w, &data, "ash_witch", ENEMY, ARENA);
    // Ein zäher, aber harmloser Arbeiter lockt die Hexe, ohne ihre Diener zu töten.
    let victim = spawn(&mut w, &data, "worker", PLAYER, ARENA + Vec2::new(5.0, 0.0));
    w.units.get_mut(victim).unwrap().hp = 5000.0;
    w.units.get_mut(victim).unwrap().max_hp = 5000.0;
    let events = run_events(&mut w, &data, 20 * 30);
    let summoned = events.iter().filter(|e| matches!(e, SimEvent::Summoned { .. })).count();
    assert!(summoned >= 2, "Aschhexe beschwört Diener ({summoned})");
    let servants: Vec<UnitId> = w.units.iter().filter(|(_, u)| u.master == Some(witch)).map(|(id, _)| id).collect();
    assert!(!servants.is_empty() && servants.len() <= 4, "höchstens vier Diener gleichzeitig: {}", servants.len());
    // Stirbt die Hexe, vergehen ihre Diener.
    w.units.get_mut(witch).unwrap().hp = 0.0;
    run(&mut w, &data, 3);
    assert!(servants.iter().all(|s| w.units.get(*s).is_none()), "Diener vergehen mit der Hexe");
}

#[test]
fn fireball_hits_an_area_with_fire() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let mage = spawn(&mut w, &data, "battle_mage", PLAYER, ARENA);
    let a = spawn(&mut w, &data, "ghoul", ENEMY, ARENA + Vec2::new(5.0, 0.0));
    let b = spawn(&mut w, &data, "ghoul", ENEMY, ARENA + Vec2::new(5.8, 0.4));
    for g in [a, b] {
        w.units.get_mut(g).unwrap().order = Order::Hold;
    }
    run(&mut w, &data, 2);
    cmd(&mut w, PLAYER, Command::UseAbility { unit: mage, ability: 0, point: ARENA + Vec2::new(5.4, 0.2) });
    run(&mut w, &data, 4);
    let (ha, hb) = (w.units.get(a).unwrap().hp, w.units.get(b).unwrap().hp);
    assert!(ha < 170.0 - 60.0 && hb < 170.0 - 60.0, "beide Ghule verbrannt: {ha}, {hb}");
}

// ------------------------------------------------------------------------------------ Helden

fn hero(w: &SimWorld, data: &GameData) -> UnitId {
    w.hero_of(data, PLAYER).expect("Heldin")
}

#[test]
fn talent_points_work_and_unlock_abilities() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    assert_eq!(ashen_sim::systems::heroes::free_points(&data, w.units.get(h).unwrap()), 0, "Stufe 1: keine Punkte");
    // Auf Stufe 6 aufsteigen → fünf Punkte.
    ashen_sim::systems::combat::grant_xp(&mut w, &data, h, 1000);
    let u = w.units.get(h).unwrap();
    assert_eq!(u.level, 6);
    assert_eq!(ashen_sim::systems::heroes::free_points(&data, u), 5);
    let base_factor = w.damage_factor(&data, u);
    let tree = data.talent_tree("hero_isolde").unwrap();
    let idx = |id: &str| tree.talents.iter().position(|t| t.id == id).unwrap() as u16;
    // Reihe 2 ohne Vorbedingung scheitert.
    cmd(&mut w, PLAYER, Command::LearnTalent { unit: h, talent: idx("phoenix_cut") });
    run(&mut w, &data, 1);
    assert!(w.units.get(h).unwrap().extra_abilities.is_empty(), "Capstone braucht vier Punkte im Pfad");
    // Drei Ränge Glühende Schneide wirken auf den Schaden.
    for _ in 0..3 {
        cmd(&mut w, PLAYER, Command::LearnTalent { unit: h, talent: idx("edge_of_embers") });
    }
    run(&mut w, &data, 1);
    let u = w.units.get(h).unwrap();
    assert_eq!(u.talents[idx("edge_of_embers") as usize], 3);
    assert!((u.mods.damage_pct - 0.18).abs() < 1e-4);
    let boosted = w.damage_factor(&data, u);
    assert!((boosted / base_factor - 1.18).abs() < 0.01, "+18 % Schaden: {base_factor} -> {boosted}");
    // Ein weiterer Punkt in Reihe 1, dann Rang 4 und der Capstone sind erreichbar.
    cmd(&mut w, PLAYER, Command::LearnTalent { unit: h, talent: idx("searing_strikes") });
    run(&mut w, &data, 1);
    assert_eq!(ashen_sim::systems::heroes::free_points(&data, w.units.get(h).unwrap()), 1);
    cmd(&mut w, PLAYER, Command::LearnTalent { unit: h, talent: idx("phoenix_cut") });
    run(&mut w, &data, 1);
    let u = w.units.get(h).unwrap();
    assert_eq!(u.extra_abilities, vec!["phoenix_cut".to_string()], "Fähigkeit freigeschaltet");
    assert_eq!(u.ability_cd.len(), 4);
    // Die neue Fähigkeit ist als vierte Fähigkeit wirksam.
    let pos = u.pos;
    let enemy = spawn(&mut w, &data, "hollow_thrall", ENEMY, pos + Vec2::new(1.2, 0.0));
    run(&mut w, &data, 2);
    w.units.get_mut(h).unwrap().mana = 300.0;
    let hp0 = w.units.get(enemy).unwrap().hp;
    cmd(&mut w, PLAYER, Command::UseAbility { unit: h, ability: 3, point: pos });
    run(&mut w, &data, 2);
    assert!(w.units.get(enemy).map(|e| e.hp).unwrap_or(0.0) < hp0 - 100.0, "Phönixhieb trifft");
}

#[test]
fn equipment_changes_stats() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    let before = {
        let u = w.units.get(h).unwrap();
        (u.max_hp, w.armor_of(&data, u), w.damage_factor(&data, u), u.mods.damage)
    };
    for id in ["embersteel_blade", "ember_plate", "ashglass_heart", "ember_ring", "ember_ring", "iron_band"] {
        let item = data.item_def_id(id).unwrap();
        assert!(ashen_sim::systems::heroes::give_item(&mut w, h, item));
    }
    for _ in 0..6 {
        cmd(&mut w, PLAYER, Command::EquipItem { unit: h, index: 0 });
    }
    run(&mut w, &data, 1);
    let u = w.units.get(h).unwrap();
    assert!(u.equipment.iter().filter(|e| e.is_some()).count() == 5, "5 Plätze belegt (Waffe, Rüstung, Amulett, 2 Ringe)");
    assert_eq!(u.inventory.len(), 1, "der überzählige Ring bleibt im Inventar");
    let after = (u.max_hp, w.armor_of(&data, u), w.damage_factor(&data, u), u.mods.damage);
    assert!(after.0 > before.0 + 100.0, "mehr Leben");
    assert!(after.1 >= before.1 + 4.0, "mehr Rüstung");
    assert!(after.2 > before.2 * 1.15, "mehr Schaden");
    assert!(after.3 >= before.3 + 10.0, "fester Zusatzschaden");
    // Ablegen stellt die Werte wieder her.
    for slot in 0..5 {
        cmd(&mut w, PLAYER, Command::UnequipItem { unit: h, slot });
    }
    run(&mut w, &data, 1);
    let u = w.units.get(h).unwrap();
    assert!((u.max_hp - before.0).abs() < 0.5);
    assert!((w.armor_of(&data, u) - before.1).abs() < 0.01);
}

#[test]
fn potions_heal_and_buff() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    for id in ["potion_heal", "potion_ember"] {
        ashen_sim::systems::heroes::give_item(&mut w, h, data.item_def_id(id).unwrap());
    }
    w.units.get_mut(h).unwrap().hp = 100.0;
    cmd(&mut w, PLAYER, Command::UseItem { unit: h, index: 0 });
    run(&mut w, &data, 1);
    assert!(w.units.get(h).unwrap().hp >= 300.0);
    // Trank-Abklingzeit: der zweite Trank geht erst nach Ablauf.
    cmd(&mut w, PLAYER, Command::UseItem { unit: h, index: 0 });
    run(&mut w, &data, 1);
    assert_eq!(w.units.get(h).unwrap().inventory.len(), 1);
    run(&mut w, &data, 20 * 9);
    cmd(&mut w, PLAYER, Command::UseItem { unit: h, index: 0 });
    run(&mut w, &data, 1);
    assert!(w.units.get(h).unwrap().inventory.is_empty());
    assert!(w.units.get(h).unwrap().damage_mult() > 1.2);
}

#[test]
fn boss_and_camps_drop_items_that_heroes_pick_up() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    let boss = spawn(&mut w, &data, "boss_pale_reeve", ENEMY, ARENA);
    w.deal_damage(&data, EntityRef::Unit(boss), 1.0e6, ashen_sim::data::DamageType::Hero, &[], None);
    run(&mut w, &data, 1);
    let seal = data.item_def_id("vael_dorn_seal").unwrap();
    assert!(w.drops.iter().any(|d| d.item == seal), "der Fahle Vogt lässt das Siegel von Vael Dorn fallen");
    assert!(w.drops.len() >= 3, "Boss-Beute: Siegel, Panzer, Trank");
    // Die Heldin nimmt die Beute auf.
    w.units.get_mut(h).unwrap().pos = ARENA;
    run(&mut w, &data, 40);
    let u = w.units.get(h).unwrap();
    assert!(u.inventory.contains(&seal), "Beute im Inventar");
    // Lagerbeute: erst die letzte Einheit löst sie aus.
    let a = spawn(&mut w, &data, "hollow_thrall", ENEMY, ARENA + Vec2::new(10.0, 0.0));
    let b = spawn(&mut w, &data, "hollow_thrall", ENEMY, ARENA + Vec2::new(11.0, 0.0));
    for id in [a, b] {
        w.units.get_mut(id).unwrap().camp = Some("camp_test".into());
    }
    let drops0 = w.drops.len();
    w.units.get_mut(a).unwrap().hp = 0.0;
    run(&mut w, &data, 1);
    let after_first = w.drops.len();
    w.units.get_mut(b).unwrap().hp = 0.0;
    run(&mut w, &data, 1);
    assert!(w.drops.len() >= after_first + 2 || w.drops.len() >= drops0 + 2, "Lager hinterlässt Beute (zwei Würfe)");
}

#[test]
fn hero_revives_at_altar_with_level_and_items() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    ashen_sim::systems::combat::grant_xp(&mut w, &data, h, 450);
    ashen_sim::systems::heroes::give_item(&mut w, h, data.item_def_id("steel_sword").unwrap());
    cmd(&mut w, PLAYER, Command::EquipItem { unit: h, index: 0 });
    run(&mut w, &data, 1);
    let level = w.units.get(h).unwrap().level;
    w.units.get_mut(h).unwrap().hp = -1.0;
    run(&mut w, &data, 2);
    assert_eq!(w.fallen.len(), 1);
    assert!(w.hero_of(&data, PLAYER).is_none());
    // Altar errichten und gegen Gold wiederbeleben.
    let altar = w.place_building(&data, data.building_id("altar").unwrap(), PLAYER, glam::IVec2::new(30, 40), true);
    let gold0 = w.players[0].resources.gold;
    let hk = data.unit_id("hero_isolde").unwrap();
    cmd(&mut w, PLAYER, Command::ReviveHero { building: altar, hero: hk });
    run(&mut w, &data, 2);
    let back = w.hero_of(&data, PLAYER).expect("Heldin kehrt zurück");
    let u = w.units.get(back).unwrap();
    assert_eq!(u.level, level, "Stufe bleibt erhalten");
    assert!(u.equipment[0].is_some(), "Ausrüstung bleibt erhalten");
    assert!((u.hp / u.max_hp - 0.8).abs() < 0.05, "80 % Leben am Altar");
    assert!(gold0 - w.players[0].resources.gold >= 150.0, "Gold wurde bezahlt");
    assert!(w.fallen.is_empty());
}

#[test]
fn new_heroes_are_trained_at_the_altar_once() {
    let (data, mut w) = sandbox();
    let altar = w.place_building(&data, data.building_id("altar").unwrap(), PLAYER, glam::IVec2::new(30, 40), true);
    let hal = data.unit_id("hero_hal_venn").unwrap();
    cmd(&mut w, PLAYER, Command::Train { building: altar, unit: hal });
    cmd(&mut w, PLAYER, Command::Train { building: altar, unit: hal });
    run(&mut w, &data, 1);
    assert_eq!(w.buildings.get(altar).unwrap().queue.len(), 1, "jeder Held nur einmal");
    run(&mut w, &data, 20 * 50);
    assert!(w.units.iter().any(|(_, u)| u.kind == hal && u.owner == PLAYER));
    let tree = data.talent_tree("hero_hal_venn").expect("Talentbaum");
    assert_eq!(tree.paths.len(), 3);
}

#[test]
fn save_roundtrip_keeps_hero_inventory() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let h = hero(&w, &data);
    ashen_sim::systems::combat::grant_xp(&mut w, &data, h, 1000);
    for id in ["steel_sword", "potion_heal", "iron_band"] {
        ashen_sim::systems::heroes::give_item(&mut w, h, data.item_def_id(id).unwrap());
    }
    cmd(&mut w, PLAYER, Command::EquipItem { unit: h, index: 0 });
    let tree = data.talent_tree("hero_isolde").unwrap();
    cmd(&mut w, PLAYER, Command::LearnTalent { unit: h, talent: tree.talents.iter().position(|t| t.id == "ashen_plate").unwrap() as u16 });
    w.drops.push(ashen_sim::entity::GroundItem { pos: Vec2::new(5.0, 5.0), item: data.item_def_id("ember_ring").unwrap() });
    run(&mut w, &data, 5);
    let bytes = ashen_sim::save::encode(&w, &data, "t", String::new()).expect("encode");
    let (_, payload) = ashen_sim::save::decode(&bytes, &data).expect("decode");
    let loaded = payload.world;
    let (a, b) = (w.units.get(h).unwrap(), loaded.units.get(h).unwrap());
    assert_eq!(a.inventory, b.inventory);
    assert_eq!(a.equipment, b.equipment);
    assert_eq!(a.talents, b.talents);
    assert_eq!(a.level, b.level);
    assert_eq!(a.mods, b.mods);
    assert_eq!(loaded.drops.len(), 1);
    assert_eq!(loaded.state_hash(), w.state_hash());
    let mut loaded = loaded;
    run(&mut w, &data, 100);
    run(&mut loaded, &data, 100);
    assert_eq!(loaded.state_hash(), w.state_hash());
}

// ------------------------------------------------------------------------------------ Formationen und Upgrades

#[test]
fn formation_keeps_spacing_and_has_effects() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    for formation in [Formation::Line, Formation::Wedge, Formation::Column, Formation::Square] {
        let units: Vec<UnitId> = (0..14).map(|i| spawn(&mut w, &data, if i % 3 == 2 { "archer" } else { "swordsman" }, PLAYER, ARENA + Vec2::new((i % 5) as f32 * 0.9, (i / 5) as f32 * 0.9))).collect();
        cmd(&mut w, PLAYER, Command::SetFormation { units: units.clone(), formation });
        cmd(&mut w, PLAYER, Command::Move { units: units.clone(), target: ARENA + Vec2::new(8.0, 4.0), attack_move: false, queue: false });
        run(&mut w, &data, 20 * 16);
        let pos: Vec<Vec2> = units.iter().filter_map(|u| w.units.get(*u)).map(|u| u.pos).collect();
        let mut min = f32::MAX;
        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                min = min.min((pos[i] - pos[j]).length());
            }
        }
        assert!(min >= 0.55, "{formation:?}: kleinster Abstand {min:.2}");
        let c = pos.iter().sum::<Vec2>() / pos.len() as f32;
        assert!((c - (ARENA + Vec2::new(8.0, 4.0))).length() < 4.0, "{formation:?}: Gruppe erreicht das Ziel");
        for u in units {
            w.units.remove(u);
        }
    }
    // Wirkungen: Karree = Rüstung, Kolonne = Tempo, Keil = Schaden.
    let s = spawn(&mut w, &data, "swordsman", PLAYER, ARENA);
    let base = (w.armor_of(&data, w.units.get(s).unwrap()), w.speed_factor(w.units.get(s).unwrap()), w.damage_factor(&data, w.units.get(s).unwrap()));
    for (f, check) in [(Formation::Square, 0), (Formation::Column, 1), (Formation::Wedge, 2)] {
        cmd(&mut w, PLAYER, Command::SetFormation { units: vec![s], formation: f });
        run(&mut w, &data, 1);
        let u = w.units.get(s).unwrap();
        match check {
            0 => assert!(w.armor_of(&data, u) >= base.0 + 3.0),
            1 => assert!(w.speed_factor(u) > base.1 * 1.1),
            _ => assert!(w.damage_factor(&data, u) > base.2 * 1.1),
        }
    }
}

#[test]
fn square_reduces_arrow_damage_and_morale_loss() {
    let (data, mut w) = sandbox();
    clear_hostiles(&mut w);
    let a = spawn(&mut w, &data, "swordsman", PLAYER, ARENA);
    let b = spawn(&mut w, &data, "swordsman", PLAYER, ARENA + Vec2::new(3.0, 0.0));
    cmd(&mut w, PLAYER, Command::SetFormation { units: vec![b], formation: Formation::Square });
    run(&mut w, &data, 1);
    for id in [a, b] {
        w.deal_damage(&data, EntityRef::Unit(id), 100.0, ashen_sim::data::DamageType::Pierce, &[], None);
    }
    let (da, db) = (300.0 - w.units.get(a).unwrap().hp, 300.0 - w.units.get(b).unwrap().hp);
    assert!(db < da * 0.7, "Karree: {db:.1} gegen {da:.1}");
    assert!(w.units.get(b).unwrap().morale > w.units.get(a).unwrap().morale, "weniger Moralverlust");
}

#[test]
fn smithy_research_raises_damage_and_respects_era() {
    // Im Gefecht ohne Burg: Stufe 2 ist gesperrt.
    let (data, mut w) = setup(GameMode::Skirmish, Difficulty::Easy);
    w.players[0].resources.gold += 3000.0;
    for g in w.players[0].goods.iter_mut() {
        *g += 100.0;
    }
    w.players[0].extra_capacity = 5000.0;
    let smithy = w.place_building(&data, data.building_id("smithy").unwrap(), PLAYER, glam::IVec2::new(30, 44), true);
    let up = data.upgrade_id("forge_weapons").unwrap() as u8;
    let s = spawn(&mut w, &data, "swordsman", PLAYER, ARENA);
    let f0 = w.damage_factor(&data, w.units.get(s).unwrap());
    cmd(&mut w, PLAYER, Command::Research { building: smithy, upgrade: up });
    run(&mut w, &data, 20 * 41);
    assert_eq!(w.players[0].upgrades[up as usize], 1);
    let f1 = w.damage_factor(&data, w.units.get(s).unwrap());
    assert!((f1 / f0 - 1.10).abs() < 0.01, "+10 % Schaden: {f0} -> {f1}");
    // Stufe 2 verlangt die Burg.
    cmd(&mut w, PLAYER, Command::Research { building: smithy, upgrade: up });
    run(&mut w, &data, 2);
    assert!(w.buildings.get(smithy).unwrap().research.is_none(), "ohne Burg gesperrt");
    w.players[0].era = 1;
    cmd(&mut w, PLAYER, Command::Research { building: smithy, upgrade: up });
    run(&mut w, &data, 2);
    assert!(w.buildings.get(smithy).unwrap().research.is_some());
    // Abbruch erstattet die Kosten.
    let gold = w.players[0].resources.gold;
    cmd(&mut w, PLAYER, Command::CancelResearch { building: smithy });
    run(&mut w, &data, 1);
    assert!(w.players[0].resources.gold >= gold + 200.0);
}

#[test]
fn burg_unlocks_advanced_units() {
    let (data, mut w) = setup(GameMode::Campaign, Difficulty::Normal);
    let barracks = w.place_building(&data, data.building_id("barracks").unwrap(), PLAYER, glam::IVec2::new(30, 44), true);
    w.players[0].resources.gold += 1000.0;
    w.players[0].resources.wood += 500.0;
    w.players[0].pop_cap = 60;
    for g in w.players[0].goods.iter_mut() {
        *g += 20.0;
    }
    let halb = data.unit_id("halberdier").unwrap();
    cmd(&mut w, PLAYER, Command::Train { building: barracks, unit: halb });
    let events = run_events(&mut w, &data, 2);
    assert!(events.iter().any(|e| matches!(e, SimEvent::Error { text, .. } if text.contains("Burg"))), "Hellebardier erst ab der Burg");
    assert!(w.buildings.get(barracks).unwrap().queue.is_empty());
    w.players[0].era = 1;
    cmd(&mut w, PLAYER, Command::Train { building: barracks, unit: halb });
    run(&mut w, &data, 2);
    assert_eq!(w.buildings.get(barracks).unwrap().queue.len(), 1);
}

#[test]
fn new_unit_costs_use_equipment_goods() {
    let (data, _) = sandbox();
    for id in ["crossbowman", "halberdier", "shieldbearer", "scout_rider", "horse_archer", "battle_mage", "priest", "battering_ram", "catapult", "hero_hal_venn", "hero_torvald"] {
        let d = data.unit(data.unit_id(id).unwrap());
        assert!(!d.goods_cost_r.is_empty(), "{id} braucht Ausrüstung aus dem Lager");
        assert!(d.cost.gold > 0.0, "{id} kostet Gold");
    }
    let mage = data.unit(data.unit_id("battle_mage").unwrap());
    assert!(mage.goods_cost.iter().any(|(g, _)| g == "cloth"));
}

// ------------------------------------------------------------------------------------ Gegner-KI

fn count_ai_buildings(w: &SimWorld, data: &GameData) -> usize {
    ["bone_pit", "ghoul_den", "ash_altar", "grave_forge", "bone_cache", "bone_spire"]
        .iter()
        .filter_map(|id| data.building_id(id))
        .map(|bt| w.buildings.iter().filter(|(_, b)| b.owner == ENEMY && b.kind == bt && b.complete).count())
        .sum()
}

#[test]
fn enemy_ai_builds_a_base_and_attacks() {
    let (data, mut w) = setup(GameMode::Skirmish, Difficulty::Hard);
    // Der Spieler verhält sich passiv; die KI muss von selbst Wirtschaft und Armee aufbauen.
    let keep = w.buildings.get(w.building_with_tag("player_keep").unwrap()).unwrap().center();
    let mut closest = f32::MAX;
    let mut launched = false;
    for t in 0..(20 * 60 * 9) {
        w.step(&data);
        for e in w.events.drain(..) {
            if matches!(e, SimEvent::WaveIncoming { size, .. } if size > 0) {
                launched = true;
            }
        }
        if t % 20 == 0 {
            for u in ashen_sim::systems::ai_player::ai_soldiers(&w, &data) {
                if let Some(u) = w.units.get(u) {
                    closest = closest.min((u.pos - keep).length());
                }
            }
        }
        if w.mission.outcome.is_some() {
            break;
        }
    }
    let workers = w.units.iter().filter(|(_, u)| u.owner == ENEMY && data.unit(u.kind).class == UnitClass::Worker).count();
    assert!(workers >= 6, "KI bildet Arbeiter aus: {workers}");
    assert!(w.players[ENEMY as usize].stats.gathered.gold > 300.0, "KI sammelt Gold: {}", w.players[ENEMY as usize].stats.gathered.gold);
    assert!(w.players[ENEMY as usize].stats.gathered.wood > 100.0, "KI sammelt Holz");
    let b = count_ai_buildings(&w, &data);
    assert!(b >= 4, "KI baut innerhalb von 9 Minuten mindestens 4 Gebäude: {b}");
    assert!(w.ai.trained >= 8, "KI bildet Soldaten aus: {}", w.ai.trained);
    assert!(launched || w.ai.attacks >= 1 || w.ai.launched_size > 0, "KI startet einen Angriff");
    assert!(closest < 25.0 || w.players[0].stats.units_lost > 0, "Angriffstruppen erreichen Brennhollow (nächster Abstand {closest:.1})");
}

#[test]
fn enemy_ai_is_deterministic_and_levels_differ() {
    let hash = |d: Difficulty| {
        let (data, mut w) = setup(GameMode::Skirmish, d);
        run(&mut w, &data, 20 * 300);
        (w.state_hash(), w.ai.trained, w.ai.builds, w.players[ENEMY as usize].stats.gathered.gold)
    };
    assert_eq!(hash(Difficulty::Normal), hash(Difficulty::Normal), "deterministisch");
    let easy = hash(Difficulty::Easy);
    let nightmare = hash(Difficulty::Nightmare);
    assert!(nightmare.3 > easy.3 * 1.3, "Albtraum sammelt deutlich mehr Gold als Leicht: {} gegen {}", nightmare.3, easy.3);
    assert!(nightmare.1 >= easy.1, "Albtraum bildet mindestens so viele Einheiten aus: {} gegen {}", nightmare.1, easy.1);
    let (data, _) = sandbox();
    let l = &data.rules.ai_levels;
    assert_eq!(l.len(), 4);
    assert!(l[0].first_attack > l[1].first_attack && l[1].first_attack > l[2].first_attack && l[2].first_attack > l[3].first_attack);
    assert!(l[0].counter_skill < l[3].counter_skill && l[3].harass && !l[0].harass);
}

#[test]
fn counter_logic_prefers_answers_to_the_players_army() {
    use ashen_sim::systems::ai_player::{counter_weight, Profile};
    let cav_heavy = Profile { cavalry: 0.6, heavy: 0.7, ..Default::default() };
    let archers = Profile { ranged: 0.7, light: 0.7, ..Default::default() };
    let sieging = Profile { walls: 1.0, ..Default::default() };
    // Gegen schwere Reiter: Hexe (Magie) vor Hohlknecht.
    assert!(counter_weight("ash_witch", &cav_heavy) > counter_weight("hollow_thrall", &cav_heavy) * 0.9);
    // Gegen Bogenschützen: Ghule vor Hexen.
    assert!(counter_weight("ghoul", &archers) > counter_weight("ash_witch", &archers));
    // Gegen Befestigungen: Grabkolosse gewinnen an Gewicht.
    assert!(counter_weight("grave_brute", &sieging) > counter_weight("grave_brute", &Profile::default()) * 3.0);
}

#[test]
fn ai_is_off_outside_skirmish() {
    for mode in [GameMode::Campaign, GameMode::Survival] {
        let (data, mut w) = setup(mode, Difficulty::Normal);
        run(&mut w, &data, 20 * 30);
        assert!(!w.ai.enabled);
        assert!(w.director.enabled, "{mode:?} nutzt weiter den Wellen-Direktor");
    }
    let (_, w) = setup(GameMode::Skirmish, Difficulty::Normal);
    assert!(w.ai.enabled && !w.director.enabled);
}
