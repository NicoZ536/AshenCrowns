//! Integrationstests der Simulation mit den echten Spieldaten und Mission 1.

use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{GameData, UnitClass};
use ashen_sim::entity::{NodeKind, PLAYER};
use ashen_sim::mission::{GameMode, MatchSettings, MissionDef};
use ashen_sim::world::SimWorld;
use glam::{IVec2, Vec2};
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data")
}

fn setup(mode: GameMode) -> (GameData, SimWorld) {
    let data = GameData::load_dir(&assets()).expect("data");
    let mission = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).expect("mission");
    let world = SimWorld::new(&data, mission, MatchSettings { mode, ..Default::default() }).expect("world");
    (data, world)
}

fn player_units(w: &SimWorld, data: &GameData, class: UnitClass) -> Vec<ashen_sim::entity::UnitId> {
    w.units.iter().filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == class).map(|(id, _)| id).collect()
}

fn run(w: &mut SimWorld, data: &GameData, ticks: u32) {
    for _ in 0..ticks {
        w.step(data);
        w.events.clear();
    }
}

#[test]
fn mission_starts_with_canonical_setup() {
    let (data, w) = setup(GameMode::Campaign);
    assert_eq!(player_units(&w, &data, UnitClass::Worker).len(), 5);
    assert_eq!(player_units(&w, &data, UnitClass::Infantry).len(), 3);
    assert_eq!(player_units(&w, &data, UnitClass::Hero).len(), 1);
    assert!(w.building_with_tag("player_keep").is_some());
    assert!(w.building_with_tag("enemy_shrine").is_some());
    assert_eq!(w.players[0].pop_cap, 18);
    assert!(w.nodes.iter().any(|(_, n)| n.kind == NodeKind::GoldMine));
    assert!(w.nodes.iter().filter(|(_, n)| n.kind == NodeKind::Tree).count() > 200);
}

#[test]
fn workers_gather_gold_and_wood() {
    let (data, mut w) = setup(GameMode::Campaign);
    let workers = player_units(&w, &data, UnitClass::Worker);
    let keep = w.buildings.get(w.building_with_tag("player_keep").unwrap()).unwrap().center();
    let mine = w.nodes.iter().filter(|(_, n)| n.kind == NodeKind::GoldMine).min_by(|a, b| a.1.distance_to(keep).total_cmp(&b.1.distance_to(keep))).unwrap().0;
    let tree = w.nodes.iter().filter(|(_, n)| n.kind == NodeKind::Tree).min_by(|a, b| a.1.distance_to(keep).total_cmp(&b.1.distance_to(keep))).unwrap().0;
    let gold0 = w.players[0].stats.gathered.gold;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Gather { units: workers[..3].to_vec(), node: mine } });
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Gather { units: workers[3..].to_vec(), node: tree } });
    run(&mut w, &data, 20 * 60);
    let p = &w.players[0];
    assert!(p.stats.gathered.gold - gold0 >= 60.0, "gold gathered: {}", p.stats.gathered.gold - gold0);
    assert!(p.stats.gathered.wood >= 20.0, "wood gathered: {}", p.stats.gathered.wood);
}

#[test]
fn build_and_train() {
    let (data, mut w) = setup(GameMode::Campaign);
    let workers = player_units(&w, &data, UnitClass::Worker);
    let farm = data.building_id("farmstead").unwrap();
    // Freien Bauplatz nahe der Basis suchen.
    let mut tile = None;
    'outer: for y in 46..62 {
        for x in 14..26 {
            let t = IVec2::new(x, y);
            if w.can_place(&data, farm, t, PLAYER).is_ok() {
                tile = Some(t);
                break 'outer;
            }
        }
    }
    let tile = tile.expect("free build site");
    let wood0 = w.players[0].resources.wood;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Build { units: workers[..2].to_vec(), kind: farm, tile } });
    run(&mut w, &data, 1);
    assert!(w.players[0].resources.wood < wood0, "cost deducted");
    run(&mut w, &data, 20 * 45);
    let farms = w.buildings.iter().filter(|(_, b)| b.kind == farm && b.complete).count();
    assert_eq!(farms, 2, "second farmstead completed");
    assert_eq!(w.players[0].pop_cap, 26);

    let keep = w.building_with_tag("player_keep").unwrap();
    let worker = data.unit_id("worker").unwrap();
    let pop0 = w.players[0].pop;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: keep, unit: worker } });
    run(&mut w, &data, 20 * 13);
    assert_eq!(w.players[0].pop, pop0 + 1);
}

#[test]
fn simulation_is_deterministic() {
    let (data, mut a) = setup(GameMode::Campaign);
    let (_, mut b) = setup(GameMode::Campaign);
    let sw = player_units(&a, &data, UnitClass::Infantry);
    let cmd = Command::Move { units: sw, target: Vec2::new(30.0, 40.0), attack_move: true, queue: false };
    a.push_command(PlayerCommand { player: PLAYER, command: cmd.clone() });
    b.push_command(PlayerCommand { player: PLAYER, command: cmd });
    for _ in 0..(20 * 120) {
        a.step(&data);
        b.step(&data);
        a.events.clear();
        b.events.clear();
    }
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn waves_attack_and_game_runs_long() {
    let (data, mut w) = setup(GameMode::Campaign);
    let mut waves = 0;
    let mut deaths = 0;
    for _ in 0..(20 * 420) {
        w.step(&data);
        for e in w.events.drain(..) {
            match e {
                ashen_sim::SimEvent::WaveIncoming { size, .. } if size > 0 => waves += 1,
                ashen_sim::SimEvent::UnitDied { .. } => deaths += 1,
                _ => {}
            }
        }
        if w.mission.outcome.is_some() {
            break;
        }
    }
    assert!(waves >= 1, "at least one wave spawned");
    assert!(deaths >= 1, "combat happened");
    assert!(w.env.night >= 1);
    assert!(!w.chronicle.entries.is_empty());
}

#[test]
fn save_roundtrip_preserves_state() {
    let (data, mut w) = setup(GameMode::Skirmish);
    run(&mut w, &data, 20 * 20);
    let bytes = ashen_sim::save::encode(&w, &data, "test", "camera".into()).expect("encode");
    let (header, payload) = ashen_sim::save::decode(&bytes, &data).expect("decode");
    assert_eq!(header.mission_id, "m01_last_settlement");
    assert_eq!(payload.client_state, "camera");
    let mut loaded = payload.world;
    assert_eq!(loaded.state_hash(), w.state_hash());
    // Weiterlaufen nach dem Laden muss identisch bleiben.
    run(&mut w, &data, 200);
    run(&mut loaded, &data, 200);
    assert_eq!(loaded.state_hash(), w.state_hash());
    // Beschädigte Daten werden erkannt.
    let mut broken = bytes.clone();
    let n = broken.len();
    broken[n - 10] ^= 0xFF;
    assert!(matches!(ashen_sim::save::decode(&broken, &data), Err(ashen_sim::save::SaveError::Checksum)));
}

#[test]
fn hero_ability_damages_enemies() {
    let (data, mut w) = setup(GameMode::Sandbox);
    let hero = player_units(&w, &data, UnitClass::Hero)[0];
    let pos = w.units.get(hero).unwrap().pos;
    let thrall = data.unit_id("hollow_thrall").unwrap();
    let enemy = w.spawn_unit(&data, thrall, 1, pos + Vec2::new(1.0, 0.0));
    run(&mut w, &data, 1);
    let hp0 = w.units.get(enemy).map(|u| u.hp).unwrap();
    w.push_command(PlayerCommand { player: PLAYER, command: Command::UseAbility { unit: hero, ability: 0, point: pos } });
    run(&mut w, &data, 2);
    let hp1 = w.units.get(enemy).map(|u| u.hp).unwrap_or(0.0);
    assert!(hp1 < hp0 - 50.0, "ember cleave hit: {hp0} -> {hp1}");
}

#[test]
fn mod_overlay_overrides_and_adds() {
    let mut data = GameData::load_dir(&assets()).expect("data");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../mods/example_mod/data");
    let changes = data.apply_overlay(&dir).expect("overlay");
    assert!(!changes.is_empty());
    assert!(data.unit_id("ember_militia").is_some());
}

#[test]
fn destroying_the_shrine_wins_campaign_and_skirmish() {
    for mode in [GameMode::Campaign, GameMode::Skirmish] {
        let (data, mut w) = setup(mode);
        let shrine = w.building_with_tag("enemy_shrine").expect("shrine");
        let center = w.buildings.get(shrine).unwrap().center();
        let knight = data.unit_id("knight").unwrap();
        let army: Vec<_> = (0..24).map(|i| w.spawn_unit(&data, knight, PLAYER, center + Vec2::new(-3.5 + (i % 6) as f32 * 0.6, 3.0 + (i / 6) as f32 * 0.6))).collect();
        w.push_command(PlayerCommand { player: PLAYER, command: Command::Attack { units: army, target: ashen_sim::entity::EntityRef::Building(shrine) } });
        let mut won = false;
        for _ in 0..(20 * 240) {
            w.step(&data);
            if w.events.iter().any(|e| matches!(e, ashen_sim::SimEvent::Victory)) {
                won = true;
                break;
            }
            w.events.clear();
        }
        assert!(won, "{mode:?}: Sieg nach Zerstörung des Schreins");
        assert_eq!(w.mission.outcome, Some(ashen_sim::mission::Outcome::Victory));
    }
}

#[test]
fn losing_the_keep_is_defeat() {
    let (data, mut w) = setup(GameMode::Campaign);
    let keep = w.building_with_tag("player_keep").unwrap();
    w.buildings.get_mut(keep).unwrap().hp = 0.0;
    run(&mut w, &data, 20);
    assert_eq!(w.mission.outcome, Some(ashen_sim::mission::Outcome::Defeat));
}

#[test]
fn faulty_mod_leaves_data_untouched() {
    let mut data = GameData::load_dir(&assets()).expect("data");
    let dir = std::env::temp_dir().join(format!("ashen_badmod_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::copy(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../mods/example_mod/data/units.ron"), dir.join("units.ron")).unwrap();
    std::fs::write(dir.join("buildings.ron"), "( buildings: [ kaputt ] )").unwrap();
    let before = data.type_table();
    assert!(data.apply_overlay(&dir).is_err());
    assert_eq!(data.type_table(), before, "fehlerhafter Mod darf nichts teilweise ändern");
    assert!(data.unit_id("ember_militia").is_none());
    let _ = std::fs::remove_dir_all(&dir);
}
