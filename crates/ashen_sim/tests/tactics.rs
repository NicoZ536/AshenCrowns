//! Tests der taktischen Kampf-KI (`systems::tactics`): Fokusfeuer, Fernkämpfer hinter dem Nahkampf, Rückzug
//! Angeschlagener, Belagerung von Toren, Bemannen von Türmen, Determinismus und Wirkung im Gefecht.

use ashen_sim::data::GameData;
use ashen_sim::entity::{EntityRef, Order, UnitId, ENEMY, PLAYER};
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings, MissionDef};
use ashen_sim::systems::tactics::{drive, Role};
use ashen_sim::world::SimWorld;
use glam::{IVec2, Vec2};
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data")
}

fn arena() -> (GameData, SimWorld) {
    let data = GameData::load_dir(&assets()).expect("data");
    let mission = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).expect("mission");
    let mut w = SimWorld::new(&data, mission, MatchSettings { mode: GameMode::Sandbox, difficulty: Difficulty::Normal, ..Default::default() }).expect("world");
    w.fog.disabled = true;
    // Nur die Testeinheiten bleiben.
    let ids: Vec<UnitId> = w.units.iter().map(|(id, _)| id).collect();
    for id in ids {
        w.units.remove(id);
    }
    let bs: Vec<_> = w.buildings.iter().map(|(id, _)| id).collect();
    for id in bs {
        w.buildings.remove(id);
    }
    (data, w)
}

fn spawn(w: &mut SimWorld, data: &GameData, id: &str, owner: u8, pos: Vec2) -> UnitId {
    w.spawn_unit(data, data.unit_id(id).unwrap_or_else(|| panic!("unbekannte Einheit {id}")), owner, pos)
}

const CENTER: Vec2 = Vec2::new(36.0, 34.0);

/// Beide Seiten bestehen aus Schwertkämpfern, Bogenschützen und einem Priester der Glutkrone.
fn army(w: &mut SimWorld, data: &GameData, owner: u8, at: Vec2, swords: usize, archers: usize, priests: usize) -> Vec<UnitId> {
    let facing = if owner == PLAYER { 1.0 } else { -1.0 };
    let mut v = Vec::new();
    for i in 0..swords {
        v.push(spawn(w, data, "swordsman", owner, at + Vec2::new(facing * 1.0, (i as f32 - swords as f32 / 2.0) * 1.1)));
    }
    for i in 0..archers {
        v.push(spawn(w, data, "archer", owner, at + Vec2::new(-facing * 0.5, (i as f32 - archers as f32 / 2.0) * 1.2)));
    }
    for i in 0..priests {
        v.push(spawn(w, data, "priest", owner, at + Vec2::new(-facing * 2.0, i as f32)));
    }
    v
}

fn march(w: &mut SimWorld, ids: &[UnitId], target: Vec2) {
    for id in ids {
        if let Some(u) = w.units.get_mut(*id) {
            u.order = Order::Move { target, attack_move: true };
            u.request_path(target);
        }
    }
}

fn hp(w: &SimWorld, ids: &[UnitId]) -> f32 {
    ids.iter().filter_map(|i| w.units.get(*i)).map(|u| u.hp.max(0.0)).sum()
}

fn alive(w: &SimWorld, ids: &[UnitId]) -> usize {
    ids.iter().filter(|i| w.units.get(**i).is_some_and(|u| u.hp > 0.0)).count()
}

/// Lässt zwei gleich große Heere aufeinandertreffen; `side` wird taktisch geführt, die Gegenseite marschiert nur.
fn duel(side: u8, swords: usize, archers: usize, priests: usize, ticks: u32) -> (f32, f32, usize, usize) {
    let (data, mut w) = arena();
    let (a, b) = (CENTER + Vec2::new(-7.0, 0.0), CENTER + Vec2::new(7.0, 0.0));
    let left = army(&mut w, &data, PLAYER, a, swords, archers, priests);
    let right = army(&mut w, &data, ENEMY, b, swords, archers, priests);
    march(&mut w, &left, b);
    march(&mut w, &right, a);
    let (tactical, plain) = if side == PLAYER { (&left, &right) } else { (&right, &left) };
    for t in 0..ticks {
        w.step(&data);
        w.events.clear();
        if t % 30 == 0 {
            drive(&mut w, &data, side, Role::Attacker);
        }
    }
    (hp(&w, tactical), hp(&w, plain), alive(&w, tactical), alive(&w, plain))
}

#[test]
fn tactics_beat_plain_attack_move_with_equal_armies() {
    let mut wins = 0;
    let mut report = Vec::new();
    // Verschiedene Zusammensetzungen und beide Seiten (die Spielerseite sieht mit 75 % Sichtweite etwas weniger).
    for (side, swords, archers, priests) in [(ENEMY, 8, 6, 1), (PLAYER, 8, 6, 1), (ENEMY, 6, 8, 2), (PLAYER, 6, 8, 2), (ENEMY, 10, 4, 0), (PLAYER, 10, 4, 0)] {
        let (mine, theirs, a, b) = duel(side, swords, archers, priests, 20 * 100);
        report.push(format!("Seite {side}: {swords}/{archers}/{priests} -> LP {mine:.0} gegen {theirs:.0} ({a} gegen {b} Überlebende)"));
        if mine > theirs {
            wins += 1;
        }
    }
    assert!(wins >= 5, "Taktik gewinnt mindestens 5 von 6 gleichen Duellen, tatsächlich {wins}:\n{}", report.join("\n"));
}

#[test]
fn ranged_units_stay_behind_the_melee_line() {
    let (data, mut w) = arena();
    let swords: Vec<UnitId> = (0..6).map(|i| spawn(&mut w, &data, "swordsman", ENEMY, CENTER + Vec2::new(10.0, i as f32 * 1.2))).collect();
    let archers: Vec<UnitId> = (0..4).map(|i| spawn(&mut w, &data, "archer", ENEMY, CENTER + Vec2::new(10.0, i as f32 * 1.2))).collect();
    // Der Gegner steht links; hinten liegt also rechts (größeres x).
    for i in 0..4 {
        spawn(&mut w, &data, "swordsman", PLAYER, CENTER + Vec2::new(-12.0, i as f32 * 1.2));
    }
    drive(&mut w, &data, ENEMY, Role::Attacker);
    let front_x = swords.iter().map(|i| w.units.get(*i).unwrap().pos.x).sum::<f32>() / swords.len() as f32;
    for a in &archers {
        let u = w.units.get(*a).unwrap();
        match u.order {
            Order::Move { target, .. } => assert!(target.x > front_x + 1.5, "Bogenschütze zieht hinter die Front: Ziel {target:?}, Front x {front_x}"),
            other => panic!("Bogenschütze erhält eine Haltebewegung statt {other:?}"),
        }
    }
    for s in &swords {
        assert!(matches!(w.units.get(*s).unwrap().order, Order::Move { attack_move: true, .. }), "Nahkämpfer rücken vor");
    }
}

#[test]
fn focus_fire_prefers_weak_dangerous_targets_and_converges() {
    let (data, mut w) = arena();
    let mine: Vec<UnitId> = (0..5).map(|i| spawn(&mut w, &data, "swordsman", ENEMY, CENTER + Vec2::new(3.0, i as f32 * 0.9))).collect();
    let foe_sword = spawn(&mut w, &data, "swordsman", PLAYER, CENTER + Vec2::new(0.0, 1.5));
    let foe_archer = spawn(&mut w, &data, "archer", PLAYER, CENTER + Vec2::new(0.0, 2.5));
    w.units.get_mut(foe_archer).unwrap().hp *= 0.5;
    drive(&mut w, &data, ENEMY, Role::Attacker);
    for m in &mine {
        assert_eq!(w.units.get(*m).unwrap().order, Order::Attack { target: EntityRef::Unit(foe_archer) }, "Fokusfeuer auf den angeschlagenen Schützen");
    }
    assert!(w.units.get(foe_sword).is_some());
}

#[test]
fn wounded_units_retreat_behind_the_line_and_hold() {
    let (data, mut w) = arena();
    let healthy: Vec<UnitId> = (0..5).map(|i| spawn(&mut w, &data, "swordsman", ENEMY, CENTER + Vec2::new(4.0, i as f32 * 1.0))).collect();
    let hurt = spawn(&mut w, &data, "swordsman", ENEMY, CENTER + Vec2::new(2.0, 2.0));
    w.units.get_mut(hurt).unwrap().hp *= 0.15;
    for i in 0..4 {
        spawn(&mut w, &data, "swordsman", PLAYER, CENTER + Vec2::new(-3.0, i as f32 * 1.0));
    }
    drive(&mut w, &data, ENEMY, Role::Attacker);
    let pos = w.units.get(hurt).unwrap().pos;
    match w.units.get(hurt).unwrap().order {
        Order::Move { target, attack_move: false } => assert!(target.x > pos.x + 5.0, "Rückzug weg vom Feind: {pos:?} -> {target:?}"),
        other => panic!("Verwundeter zieht sich zurück, Befehl war {other:?}"),
    }
    for h in &healthy {
        assert!(matches!(w.units.get(*h).unwrap().order, Order::Attack { .. }), "Gesunde greifen an");
    }
    // Ohne genug gesunde Kameraden kämpft jeder weiter.
    let (data2, mut w2) = arena();
    let lone = spawn(&mut w2, &data2, "swordsman", ENEMY, CENTER);
    w2.units.get_mut(lone).unwrap().hp *= 0.1;
    spawn(&mut w2, &data2, "swordsman", PLAYER, CENTER + Vec2::new(-3.0, 0.0));
    drive(&mut w2, &data2, ENEMY, Role::Attacker);
    assert!(matches!(w2.units.get(lone).unwrap().order, Order::Attack { .. }), "Letzter Kämpfer flieht nicht");
}

#[test]
fn siege_weapons_break_the_gate_first_then_towers_and_walls() {
    let (data, mut w) = arena();
    let gate_kind = data.building_id("gate").expect("Tor");
    let wall_kind = data.building_id("wall").expect("Mauer");
    let tower_kind = data.building_id("wall_tower").expect("Turm");
    let wall = w.place_building(&data, wall_kind, PLAYER, IVec2::new(30, 30), true);
    let tower = w.place_building(&data, tower_kind, PLAYER, IVec2::new(30, 34), true);
    let gate = w.place_building(&data, gate_kind, PLAYER, IVec2::new(30, 38), true);
    let ram = spawn(&mut w, &data, "battering_ram", ENEMY, CENTER + Vec2::new(12.0, 3.0));
    let cat = spawn(&mut w, &data, "catapult", ENEMY, CENTER + Vec2::new(14.0, -3.0));
    spawn(&mut w, &data, "hollow_thrall", ENEMY, CENTER + Vec2::new(11.0, 0.0));
    drive(&mut w, &data, ENEMY, Role::Attacker);
    for s in [ram, cat] {
        assert_eq!(w.units.get(s).unwrap().order, Order::Attack { target: EntityRef::Building(gate) }, "Belagerungswaffe greift das Tor an");
    }
    w.buildings.remove(gate);
    drive(&mut w, &data, ENEMY, Role::Attacker);
    assert_eq!(w.units.get(ram).unwrap().order, Order::Attack { target: EntityRef::Building(tower) }, "danach der Turm");
    w.buildings.remove(tower);
    drive(&mut w, &data, ENEMY, Role::Attacker);
    assert_eq!(w.units.get(ram).unwrap().order, Order::Attack { target: EntityRef::Building(wall) }, "danach die Mauer");
}

#[test]
fn defenders_man_the_towers_and_hold_the_choke_point() {
    let (data, mut w) = arena();
    let wall_kind = data.building_id("wall").expect("Mauer");
    let tower_kind = data.building_id("wall_tower").expect("Turm");
    // Kleine Mauerlinie mit zwei Türmen auf der Feindseite (links, x = 30).
    let tower_a = w.place_building(&data, tower_kind, ENEMY, IVec2::new(30, 28), true);
    let tower_b = w.place_building(&data, tower_kind, ENEMY, IVec2::new(30, 40), true);
    for y in 30..40 {
        w.place_building(&data, wall_kind, ENEMY, IVec2::new(30, y), true);
    }
    for y in 28..42 {
        w.place_building(&data, wall_kind, ENEMY, IVec2::new(42, y), true);
    }
    let archers: Vec<UnitId> = (0..4).map(|i| spawn(&mut w, &data, "hollow_archer", ENEMY, Vec2::new(37.0, 32.0 + i as f32))).collect();
    let thralls: Vec<UnitId> = (0..4).map(|i| spawn(&mut w, &data, "hollow_thrall", ENEMY, Vec2::new(38.0, 32.0 + i as f32))).collect();
    for i in 0..3 {
        spawn(&mut w, &data, "swordsman", PLAYER, Vec2::new(20.0, 33.0 + i as f32));
    }
    let towers: Vec<Vec2> = [tower_a, tower_b].iter().map(|b| w.buildings.get(*b).unwrap().center()).collect();
    drive(&mut w, &data, ENEMY, Role::Defender);
    for a in &archers {
        let Order::Move { target, .. } = w.units.get(*a).unwrap().order else { panic!("Schütze bezieht Posten: {:?}", w.units.get(*a).unwrap().order) };
        let d = towers.iter().map(|t| (*t - target).length()).fold(f32::MAX, f32::min);
        assert!(d < 4.0, "Posten liegt an einem Turm: Abstand {d:.1}, Ziel {target:?}, Türme {towers:?}");
    }
    for t in &thralls {
        assert!(matches!(w.units.get(*t).unwrap().order, Order::Guard { radius, .. } if radius > 0.0), "Nahkämpfer halten den Einfallspunkt");
    }
    // Nach dem Erreichen des Postens halten die Schützen die Stellung.
    for (i, a) in archers.iter().enumerate() {
        let Order::Move { target, .. } = w.units.get(*a).unwrap().order else { unreachable!() };
        let _ = i;
        w.units.get_mut(*a).unwrap().pos = target;
    }
    drive(&mut w, &data, ENEMY, Role::Defender);
    assert!(archers.iter().all(|a| w.units.get(*a).unwrap().order == Order::Hold), "Schützen halten Posten");
}

#[test]
fn tactics_are_deterministic() {
    let run = || {
        let (data, mut w) = arena();
        let (a, b) = (CENTER + Vec2::new(-7.0, 0.0), CENTER + Vec2::new(7.0, 0.0));
        let left = army(&mut w, &data, PLAYER, a, 6, 5, 1);
        let right = army(&mut w, &data, ENEMY, b, 6, 5, 1);
        march(&mut w, &left, b);
        march(&mut w, &right, a);
        for t in 0..20 * 40 {
            w.step(&data);
            w.events.clear();
            if t % 30 == 0 {
                drive(&mut w, &data, ENEMY, Role::Attacker);
            }
        }
        w.state_hash()
    };
    assert_eq!(run(), run());
}
