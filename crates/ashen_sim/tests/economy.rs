//! Integrationstests der Echtzeit-Wirtschaft: Produktionsketten, Bevölkerung, Steuern, Markt,
//! Ausbaustufen, Befestigungen, Belagerung und Spielstände mit den neuen Feldern.

use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{GameData, Item};
use ashen_sim::entity::{BuildingId, Order, ProdStatus, ENEMY, PLAYER};
use ashen_sim::map::Terrain;
use ashen_sim::mission::{GameMode, MatchSettings, MissionDef};
use ashen_sim::world::SimWorld;
use ashen_sim::SimEvent;
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

/// Sandbox (Nebel aus, alle Ausbaustufen, viele Vorräte) mit geleertem Warenlager.
fn sandbox() -> (GameData, SimWorld) {
    let (data, mut w) = setup(GameMode::Sandbox);
    for g in w.players[0].goods.iter_mut() {
        *g = 0.0;
    }
    // Die Startsiedlung der Mission (Hütten, Holzfällerlager) stört die Messungen.
    let start: Vec<_> = w.buildings.iter().filter(|(_, b)| matches!(data.building(b.kind).id.as_str(), "hut" | "woodcutter")).map(|(id, b)| (id, b.tile, b.size)).collect();
    for (id, tile, size) in start {
        w.buildings.remove(id);
        w.map.occupy(tile, size, false);
    }
    (data, w)
}

fn run(w: &mut SimWorld, data: &GameData, secs: f32) {
    for _ in 0..(secs * 20.0) as u32 {
        w.step(data);
        w.events.clear();
    }
}

fn set_goods(w: &mut SimWorld, data: &GameData, list: &[(&str, f32)]) {
    for (id, n) in list {
        let it = data.item(id).unwrap_or_else(|| panic!("unbekannt: {id}"));
        *w.players[0].stock_mut(it) = *n;
    }
}

fn stock(w: &SimWorld, data: &GameData, id: &str) -> f32 {
    w.players[0].stock(data.item(id).expect("item"))
}

/// Setzt ein fertiges Gebäude auf die nächste freie, platzierbare Kachel nahe `near`.
fn place(w: &mut SimWorld, data: &GameData, id: &str, near: IVec2) -> BuildingId {
    let kind = data.building_id(id).expect("building");
    for r in 0..24i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let t = near + IVec2::new(dx, dy);
                if w.can_place(data, kind, t, PLAYER).is_ok() {
                    return w.place_building(data, kind, PLAYER, t, true);
                }
            }
        }
    }
    panic!("kein Bauplatz für {id}");
}

const HOME: IVec2 = IVec2::new(18, 54);

/// Genug Bewohner für Arbeitskräfte.
fn settle(w: &mut SimWorld, data: &GameData, huts: usize) {
    for _ in 0..huts {
        place(w, data, "hut", HOME);
    }
    run(w, data, 2.0);
}

fn building_status(w: &SimWorld, id: BuildingId) -> ProdStatus {
    w.buildings.get(id).expect("building").status
}

#[test]
fn production_chain_wood_to_planks() {
    let (data, mut w) = sandbox();
    settle(&mut w, &data, 3);
    set_goods(&mut w, &data, &[("wood", 40.0)]);
    let saw = place(&mut w, &data, "sawmill", HOME);
    assert!(w.players[0].workforce >= 2, "Bewohner stellen Arbeitskräfte");
    run(&mut w, &data, 62.0);
    // 40 Holz → je Zyklus (30 s) 2 Holz → 3 Bretter; nach ~62 s mindestens zwei Zyklen.
    let planks = stock(&w, &data, "planks");
    assert!(planks >= 6.0, "Bretter: {planks}");
    assert!(stock(&w, &data, "wood") <= 36.0, "Holz wurde verbraucht");
    assert_eq!(building_status(&w, saw), ProdStatus::Running);
}

#[test]
fn iron_chain_ore_to_weapons() {
    let (data, mut w) = sandbox();
    settle(&mut w, &data, 6);
    set_goods(&mut w, &data, &[("iron_ore", 20.0), ("coal", 20.0), ("planks", 10.0)]);
    let smelter = place(&mut w, &data, "smelter", HOME);
    let smithy = place(&mut w, &data, "smithy", HOME);
    assert_eq!(w.buildings.get(smithy).unwrap().recipe, 0, "Standardrezept der Schmiede: Waffen");
    run(&mut w, &data, 190.0);
    assert!(stock(&w, &data, "iron") > 0.0 || stock(&w, &data, "weapons") > 0.0, "Eisen entsteht");
    let weapons = stock(&w, &data, "weapons");
    assert!(weapons >= 2.0, "Waffen: {weapons}");
    assert!(stock(&w, &data, "iron_ore") < 20.0 && stock(&w, &data, "coal") < 20.0, "Erz und Kohle verbraucht");
    assert_ne!(building_status(&w, smelter), ProdStatus::NoWorkers);
}

#[test]
fn production_stalls_without_inputs_workers_or_depot() {
    let (data, mut w) = sandbox();
    settle(&mut w, &data, 3);
    let saw = place(&mut w, &data, "sawmill", HOME);
    set_goods(&mut w, &data, &[("wood", 0.0)]);
    *w.players[0].stock_mut(Item::Res(ashen_sim::data::ResourceKind::Wood)) = 0.0;
    run(&mut w, &data, 3.0);
    assert_eq!(building_status(&w, saw), ProdStatus::MissingInput);
    let b = w.buildings.get(saw).unwrap();
    assert_eq!(b.stall_item, Some(Item::Res(ashen_sim::data::ResourceKind::Wood)), "es fehlt Holz");
    assert!(!b.cycle_active, "ohne Eingaben läuft kein Zyklus");
    assert_eq!(stock(&w, &data, "planks"), 0.0);

    // Stillgelegt per Befehl.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetWorkers { building: saw, limit: Some(0) } });
    run(&mut w, &data, 2.0);
    assert_eq!(building_status(&w, saw), ProdStatus::Disabled);
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetWorkers { building: saw, limit: None } });

    // Ohne Bewohner keine Arbeiter.
    let (data2, mut w2) = sandbox();
    let saw2 = place(&mut w2, &data2, "sawmill", HOME);
    run(&mut w2, &data2, 3.0);
    assert_eq!(building_status(&w2, saw2), ProdStatus::NoWorkers);

    // Zu weit vom Lager entfernt.
    let (data3, mut w3) = sandbox();
    settle(&mut w3, &data3, 3);
    let far = place(&mut w3, &data3, "sawmill", IVec2::new(44, 14));
    run(&mut w3, &data3, 3.0);
    assert_eq!(building_status(&w3, far), ProdStatus::NoDepot);
}

#[test]
fn woodcutter_needs_trees_and_consumes_them() {
    let (data, mut w) = sandbox();
    let kind = data.building_id("woodcutter").unwrap();
    // Offenes Gelände ohne Bäume: Standortbedingung verletzt.
    let open = w.can_place(&data, kind, IVec2::new(30, 62), PLAYER);
    if open.is_ok() {
        // Wenn dort doch Bäume stehen, ist die Bedingung eben erfüllt – dann gezielt leeren.
        let nodes: Vec<_> = w.nodes.iter().map(|(id, n)| (id, n.tile, n.kind)).collect();
        for (id, tile, k) in nodes {
            if (tile - IVec2::new(30, 62)).abs().max_element() <= 9 {
                w.nodes.remove(id);
                w.map.occupy(tile, k.size(), false);
            }
        }
    }
    assert!(w.can_place(&data, kind, IVec2::new(30, 62), PLAYER).is_err(), "ohne Bäume kein Holzfällerlager");
    // Am Waldrand dagegen schon.
    settle(&mut w, &data, 3);
    let trees_before: u32 = w.nodes.iter().map(|(_, n)| n.amount).sum();
    let wc = place(&mut w, &data, "woodcutter", IVec2::new(4, 54));
    let wood0 = w.players[0].resources.wood;
    run(&mut w, &data, 70.0);
    assert!(w.players[0].resources.wood > wood0, "Holz geliefert");
    let trees_after: u32 = w.nodes.iter().map(|(_, n)| n.amount).sum();
    assert!(trees_after < trees_before, "Baumvorrat sinkt");
    assert_eq!(building_status(&w, wc), ProdStatus::Running);
}

#[test]
fn houses_upgrade_when_needs_are_met() {
    let (data, mut w) = sandbox();
    let hut = place(&mut w, &data, "hut", HOME);
    let temple = place(&mut w, &data, "temple", HOME);
    settle(&mut w, &data, 3);
    set_goods(&mut w, &data, &[("clothing", 200.0), ("herbs", 50.0), ("bread", 200.0), ("beer", 200.0)]);
    assert!(w.players[0].workforce >= 1);
    // Die Hütte steht in Tempelnähe (Faith) – erst wenn Kleidung und Glauben gesichert sind, steigt sie auf.
    let near = w.buildings.get(hut).unwrap().center();
    assert!((w.buildings.get(temple).unwrap().center() - near).length() < 12.0);
    run(&mut w, &data, 40.0);
    let kind_name = |w: &SimWorld, id| data.building(w.buildings.get(id).unwrap().kind).id.clone();
    assert_eq!(kind_name(&w, hut), "house", "Hütte → Haus");
    run(&mut w, &data, 80.0);
    assert_eq!(kind_name(&w, hut), "townhouse", "Haus → Bürgerhaus (Brot, Bier, Kleidung, Glauben, Schutz)");
    assert!(w.players[0].pop_cap >= 10);
}

#[test]
fn house_does_not_upgrade_without_clothing_or_temple() {
    let (data, mut w) = sandbox();
    let hut = place(&mut w, &data, "hut", HOME);
    run(&mut w, &data, 80.0);
    assert_eq!(data.building(w.buildings.get(hut).unwrap().kind).id, "hut", "ohne Kleidung und Tempel kein Aufstieg");
}

#[test]
fn taxes_trade_income_for_contentment() {
    let measure = |level: u8| {
        let (data, mut w) = sandbox();
        // Ein Haus ohne Kleidung und Tempel: Bedürfnisse nur teilweise erfüllt.
        let kind = data.building_id("house").unwrap();
        let mut ids = Vec::new();
        for i in 0..3 {
            ids.push(w.place_building(&data, kind, PLAYER, IVec2::new(18 + i * 3, 56), true));
        }
        w.push_command(PlayerCommand { player: PLAYER, command: Command::SetTax { level } });
        let gold0 = w.players[0].resources.gold;
        run(&mut w, &data, 40.0);
        let p = &w.players[0];
        let mean = ids.iter().map(|id| w.buildings.get(*id).unwrap().happiness).sum::<f32>() / ids.len() as f32;
        (mean, p.resources.gold - gold0, p.residents, p.unrest)
    };
    let (h_none, g_none, _, _) = measure(0);
    let (h_norm, g_norm, r_norm, _) = measure(2);
    let (h_high, _, r_high, unrest_high) = measure(4);
    assert!(h_none > h_norm && h_norm > h_high, "höhere Steuern senken die Zufriedenheit: {h_none} > {h_norm} > {h_high}");
    assert!(g_norm > g_none, "Steuern bringen Gold: {g_norm} > {g_none}");
    assert!(unrest_high, "drückende Steuern lösen Unruhe aus");
    assert!(r_high < r_norm, "bei Unruhe wandern Bewohner ab: {r_high} < {r_norm}");
}

#[test]
fn market_prices_react_to_trade() {
    let (data, mut w) = sandbox();
    let planks = data.item("planks").unwrap();
    // Ohne Markt kein Handel.
    set_goods(&mut w, &data, &[("planks", 100.0)]);
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Trade { item: planks, amount: -10 } });
    run(&mut w, &data, 0.5);
    assert_eq!(stock(&w, &data, "planks"), 100.0, "ohne Markt wird nichts verkauft");

    place(&mut w, &data, "market", HOME);
    let base = w.market_price(&data, PLAYER, planks);
    let gold0 = w.players[0].resources.gold;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Trade { item: planks, amount: -40 } });
    run(&mut w, &data, 0.2);
    let sold_price = w.market_price(&data, PLAYER, planks);
    let income = w.players[0].resources.gold - gold0;
    assert_eq!(stock(&w, &data, "planks"), 60.0);
    assert!(sold_price < base * 0.8, "Verkauf drückt den Preis: {sold_price} vs {base}");
    assert!(income > 0.0 && income < 40.0 * base, "Verkauf unter Marktpreis (Spanne): {income}");

    // Kaufen treibt den Preis wieder hoch; Kaufpreis liegt über dem Verkaufspreis.
    let gold1 = w.players[0].resources.gold;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Trade { item: planks, amount: 40 } });
    run(&mut w, &data, 0.2);
    let spent = gold1 - w.players[0].resources.gold;
    assert!(spent > income, "Rückkauf kostet mehr als der Erlös: {spent} > {income}");
    assert!(w.market_price(&data, PLAYER, planks) > sold_price, "Kauf hebt den Preis");

    // Handelsdruck klingt ab.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Trade { item: planks, amount: -60 } });
    run(&mut w, &data, 0.2);
    let depressed = w.market_price(&data, PLAYER, planks);
    run(&mut w, &data, 200.0);
    assert!(w.market_price(&data, PLAYER, planks) > depressed, "Preis erholt sich");
}

#[test]
fn storage_capacity_limits_stock() {
    let (data, mut w) = sandbox();
    set_goods(&mut w, &data, &[("planks", 0.0)]);
    let planks = data.item("planks").unwrap();
    let cap = w.players[0].capacity;
    assert!(cap >= 500.0, "Bergfried lagert 500: {cap}");
    assert!((w.credit(PLAYER, planks, cap + 100.0) - cap).abs() < 1e-3, "Überschuss verfällt");
    assert_eq!(w.credit(PLAYER, planks, 10.0), 0.0, "Lager voll");
    let before = w.players[0].capacity;
    place(&mut w, &data, "storehouse", HOME);
    run(&mut w, &data, 2.0);
    assert!(w.players[0].capacity >= before + 400.0, "Lagerhaus erhöht die Kapazität");
}

#[test]
fn training_needs_equipment() {
    let (data, mut w) = setup(GameMode::Campaign);
    let barracks = place_for_campaign(&mut w, &data);
    let sword = data.unit_id("swordsman").unwrap();
    let weapons = data.item("weapons").unwrap();
    let armor = data.item("armor").unwrap();
    *w.players[0].stock_mut(weapons) = 0.0;
    *w.players[0].stock_mut(armor) = 0.0;
    let gold0 = w.players[0].resources.gold;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: barracks, unit: sword } });
    w.step(&data);
    assert!(w.buildings.get(barracks).unwrap().queue.is_empty(), "ohne Waffen keine Ausbildung");
    assert!(w.events.iter().any(|e| matches!(e, SimEvent::Error { text, .. } if text.contains("Waffen"))));
    w.events.clear();
    assert!(w.players[0].resources.gold >= gold0, "es wurde kein Gold abgebucht");

    *w.players[0].stock_mut(weapons) = 2.0;
    *w.players[0].stock_mut(armor) = 1.0;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: barracks, unit: sword } });
    w.step(&data);
    assert_eq!(w.buildings.get(barracks).unwrap().queue.len(), 1);
    assert_eq!(w.players[0].stock(weapons), 1.0);
    assert_eq!(w.players[0].stock(armor), 0.0);
    // Abbrechen erstattet die Ausrüstung.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::CancelTrain { building: barracks, index: 0 } });
    w.step(&data);
    assert_eq!(w.players[0].stock(weapons), 2.0);
    assert_eq!(w.players[0].stock(armor), 1.0);
}

fn place_for_campaign(w: &mut SimWorld, data: &GameData) -> BuildingId {
    let kind = data.building_id("barracks").unwrap();
    // Kampagne: Gelände muss erforscht sein; nahe der Basis ist es das.
    for y in 46..62 {
        for x in 16..28 {
            let t = IVec2::new(x, y);
            if w.can_place(data, kind, t, PLAYER).is_ok() {
                return w.place_building(data, kind, PLAYER, t, true);
            }
        }
    }
    panic!("kein Platz");
}

#[test]
fn era_upgrade_unlocks_buildings() {
    let (data, mut w) = setup(GameMode::Campaign);
    let iron_mine = data.building_id("iron_mine").unwrap();
    assert_eq!(w.players[0].era, 0);
    // Gesperrt: Fehlermeldung nennt die Ausbaustufe.
    let tile = (30..60).flat_map(|y| (14..40).map(move |x| IVec2::new(x, y))).find(|t| {
        w.can_place(&data, iron_mine, *t, PLAYER).err().is_some_and(|e| e.contains("Ausbaustufe"))
    });
    assert!(tile.is_some(), "Eisenmine ist vor der Burg gesperrt");

    // Voraussetzungen fehlen.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::AdvanceEra });
    w.step(&data);
    assert!(w.players[0].era_progress.is_none());
    assert!(w.events.iter().any(|e| matches!(e, SimEvent::Error { .. })));
    w.events.clear();

    // Voraussetzungen und Mittel bereitstellen.
    let house = data.building_id("house").unwrap();
    let store = data.building_id("storehouse").unwrap();
    for kind in [house, house, store] {
        let mut placed = false;
        'outer: for r in 0..16i32 {
            for dy in -r..=r {
                for dx in -r..=r {
                    let t = HOME + IVec2::new(dx, dy);
                    if dx.abs().max(dy.abs()) == r && w.can_place(&data, kind, t, PLAYER).is_ok() {
                        w.place_building(&data, kind, PLAYER, t, true);
                        placed = true;
                        break 'outer;
                    }
                }
            }
        }
        assert!(placed);
    }
    w.players[0].resources.gold += 1000.0;
    w.players[0].resources.wood += 500.0;
    w.players[0].resources.stone += 500.0;
    set_goods(&mut w, &data, &[("planks", 100.0), ("bricks", 100.0)]);
    w.push_command(PlayerCommand { player: PLAYER, command: Command::AdvanceEra });
    w.step(&data);
    assert!(w.players[0].era_progress.is_some(), "Ausbau läuft");
    run(&mut w, &data, data.rules.eras[0].build_time + 2.0);
    assert_eq!(w.players[0].era, 1);
    let err = w.can_place(&data, iron_mine, tile.unwrap(), PLAYER).err().unwrap_or_default();
    assert!(!err.contains("Ausbaustufe"), "Eisenmine ist nicht mehr durch die Ausbaustufe gesperrt: {err}");
    assert!(w.players[0].capacity >= 500.0 + 200.0, "Kapazitätsbonus der Burg");
}

/// Freies Rechteck für Mauertests mit Gras und ohne Hindernisse.
fn clear_area(w: &mut SimWorld, min: IVec2, max: IVec2) {
    let ids: Vec<_> = w.nodes.iter().map(|(id, n)| (id, n.tile, n.kind)).collect();
    for (id, tile, k) in ids {
        if tile.x >= min.x - 1 && tile.x <= max.x + 1 && tile.y >= min.y - 1 && tile.y <= max.y + 1 {
            w.nodes.remove(id);
            w.map.occupy(tile, k.size(), false);
        }
    }
    for y in min.y - 1..=max.y + 1 {
        for x in min.x - 1..=max.x + 1 {
            let t = IVec2::new(x, y);
            w.map.set_terrain(t, Terrain::Grass);
            let i = w.map.idx(t);
            w.map.static_block[i] = false;
        }
    }
}

/// Mauerring (Rechteck) mit einem Tor an der Südseite. Liefert (Tor, Innenpunkt, Außenpunkt).
fn wall_ring(w: &mut SimWorld, data: &GameData) -> (BuildingId, Vec2, Vec2) {
    let (min, max) = (IVec2::new(22, 58), IVec2::new(32, 66));
    clear_area(w, min - IVec2::splat(4), max + IVec2::splat(4));
    let wall = data.building_id("wall").unwrap();
    let gate = data.building_id("gate").unwrap();
    let gate_tile = IVec2::new(27, max.y);
    let mut gate_id = None;
    for x in min.x..=max.x {
        for y in [min.y, max.y] {
            let t = IVec2::new(x, y);
            if t == gate_tile {
                gate_id = Some(w.place_building(data, gate, PLAYER, t, true));
            } else {
                w.place_building(data, wall, PLAYER, t, true);
            }
        }
    }
    for y in min.y + 1..max.y {
        for x in [min.x, max.x] {
            w.place_building(data, wall, PLAYER, IVec2::new(x, y), true);
        }
    }
    (gate_id.unwrap(), Vec2::new(27.5, 62.5), Vec2::new(27.5, 72.5))
}

#[test]
fn walls_block_enemies_and_gate_admits_own_units() {
    let (data, mut w) = sandbox();
    let (gate, inside, outside) = wall_ring(&mut w, &data);
    let mut pf = ashen_sim::path::Pathfinder::default();
    // Eigene Einheiten: Weg durch das offene Tor.
    let p = pf.find_for(&w.map, inside, outside, 20_000, PLAYER).expect("Weg");
    assert!(pf.last_found, "eigener Weg ist vollständig");
    assert!(!p.is_empty());
    assert!(w.map.line_walkable_for(inside, Vec2::new(27.5, 66.5), PLAYER) && !w.map.line_walkable_for(inside, Vec2::new(27.5, 66.5), ENEMY), "Tor ist nur für Eigene begehbar");
    // Feinde: kein Weg hinein.
    pf.find_for(&w.map, outside, inside, 20_000, ENEMY);
    assert!(!pf.last_found, "Feinde finden keinen Weg durch Mauer und Tor");
    // Verschlossenes Tor sperrt auch eigene Einheiten.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetGate { building: gate, open: false } });
    w.step(&data);
    pf.find_for(&w.map, inside, outside, 20_000, PLAYER);
    assert!(!pf.last_found, "geschlossenes Tor sperrt alle");
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetGate { building: gate, open: true } });
    w.step(&data);
    pf.find_for(&w.map, inside, outside, 20_000, PLAYER);
    assert!(pf.last_found);

    // Tatsächliche Bewegung: ein eigener Soldat verlässt den Ring durch das Tor.
    let sword = data.unit_id("swordsman").unwrap();
    let u = w.spawn_unit(&data, sword, PLAYER, inside);
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: vec![u], target: outside, attack_move: false, queue: false } });
    run(&mut w, &data, 30.0);
    let pos = w.units.get(u).unwrap().pos;
    assert!((pos - outside).length() < 2.0, "Soldat steht draußen: {pos}");
}

#[test]
fn siege_units_destroy_gate_instead_of_getting_stuck() {
    let (data, mut w) = sandbox();
    let (_gate, inside, outside) = wall_ring(&mut w, &data);
    // Gegnerwelle außerhalb, Ziel im Ring.
    let thrall = data.unit_id("hollow_thrall").unwrap();
    let brute = data.unit_id("grave_brute").unwrap();
    let mut units = Vec::new();
    for i in 0..5 {
        let k = if i == 0 { brute } else { thrall };
        let id = w.spawn_unit(&data, k, ENEMY, outside + Vec2::new(i as f32 * 0.6 - 1.2, 0.0));
        let u = w.units.get_mut(id).unwrap();
        u.order = Order::Move { target: inside, attack_move: true };
        u.request_path(inside);
        units.push(id);
    }
    let forts_before = w.buildings.iter().filter(|(_, b)| data.building(b.kind).is_fort()).count();
    let mut destroyed_fort = false;
    let mut walls_hit = false;
    for _ in 0..(20 * 240) {
        w.step(&data);
        for e in w.events.drain(..) {
            match e {
                SimEvent::BuildingDestroyed { kind, .. } if data.building(kind).is_fort() => destroyed_fort = true,
                SimEvent::Hit { target: ashen_sim::entity::EntityRef::Building(_), .. } => walls_hit = true,
                _ => {}
            }
        }
        if destroyed_fort {
            break;
        }
    }
    assert!(walls_hit, "Belagerer greifen Mauer oder Tor an");
    assert!(destroyed_fort, "Mauer oder Tor wurde zerstört");
    let forts_after = w.buildings.iter().filter(|(_, b)| data.building(b.kind).is_fort()).count();
    assert!(forts_after < forts_before);
    // Nach dem Durchbruch gelangen sie hinein.
    run(&mut w, &data, 40.0);
    let in_ring = units.iter().filter_map(|id| w.units.get(*id)).filter(|u| u.pos.x > 22.5 && u.pos.x < 32.0 && u.pos.y > 58.5 && u.pos.y < 66.0).count();
    assert!(in_ring >= 1, "mindestens ein Angreifer ist durch die Bresche");
}

#[test]
fn destroyed_gate_opens_the_way_and_clears_tile() {
    let (data, mut w) = sandbox();
    let (gate, inside, outside) = wall_ring(&mut w, &data);
    let tile = w.buildings.get(gate).unwrap().tile;
    w.buildings.get_mut(gate).unwrap().hp = 0.0;
    w.step(&data);
    assert!(w.buildings.get(gate).is_none());
    assert!(w.map.walkable(tile), "Torkachsel frei");
    let mut pf = ashen_sim::path::Pathfinder::default();
    pf.find_for(&w.map, outside, inside, 20_000, ENEMY);
    assert!(pf.last_found, "Feinde kommen durch die Bresche");
}

#[test]
fn wall_line_command_places_pieces_and_pays() {
    let (data, mut w) = sandbox();
    clear_area(&mut w, IVec2::new(20, 60), IVec2::new(34, 66));
    let workers: Vec<_> = w.units.iter().filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).build_speed > 0.0).map(|(id, _)| id).collect();
    assert!(!workers.is_empty());
    let wall = data.building_id("wall").unwrap();
    let tiles: Vec<IVec2> = (22..30).map(|x| IVec2::new(x, 62)).collect();
    let stone0 = w.players[0].resources.stone;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::BuildLine { units: workers, kind: wall, tiles } });
    run(&mut w, &data, 1.0);
    assert_eq!(w.buildings.iter().filter(|(_, b)| b.kind == wall).count(), 8, "acht Mauerteile gesetzt");
    assert!((stone0 - w.players[0].resources.stone - 64.0).abs() < 1e-3, "8 × 8 Stein bezahlt");
    run(&mut w, &data, 90.0);
    assert_eq!(w.buildings.iter().filter(|(_, b)| b.kind == wall && b.complete).count(), 8, "Arbeiter errichten die ganze Linie");
}

#[test]
fn save_roundtrip_with_economy_state() {
    let (data, mut w) = sandbox();
    settle(&mut w, &data, 3);
    set_goods(&mut w, &data, &[("wood", 40.0), ("planks", 5.0)]);
    place(&mut w, &data, "sawmill", HOME);
    place(&mut w, &data, "market", HOME);
    let gate = {
        let (g, _, _) = wall_ring(&mut w, &data);
        g
    };
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetGate { building: gate, open: false } });
    w.push_command(PlayerCommand { player: PLAYER, command: Command::SetTax { level: 3 } });
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Trade { item: data.item("planks").unwrap(), amount: -3 } });
    run(&mut w, &data, 45.0);
    assert!(w.players[0].residents > 0.0);
    let bytes = ashen_sim::save::encode(&w, &data, "wirtschaft", String::new()).expect("encode");
    let (_, payload) = ashen_sim::save::decode(&bytes, &data).expect("decode");
    let mut loaded = payload.world;
    assert_eq!(loaded.state_hash(), w.state_hash());
    assert_eq!(loaded.players[0].tax_level, 3);
    assert_eq!(loaded.players[0].goods, w.players[0].goods);
    assert_eq!(loaded.players[0].market, w.players[0].market);
    assert!(!loaded.buildings.get(gate).unwrap().gate_open);
    let gt = loaded.buildings.get(gate).unwrap().tile;
    assert_eq!(loaded.map.gate_pass[loaded.map.idx(gt)], w.map.gate_pass[w.map.idx(gt)]);
    run(&mut w, &data, 30.0);
    run(&mut loaded, &data, 30.0);
    assert_eq!(loaded.state_hash(), w.state_hash(), "geladener Spielstand läuft identisch weiter");
}

#[test]
fn economy_is_deterministic() {
    let build = || {
        let (data, mut w) = sandbox();
        settle(&mut w, &data, 4);
        set_goods(&mut w, &data, &[("wood", 80.0), ("grain", 40.0), ("clay", 20.0), ("coal", 10.0)]);
        place(&mut w, &data, "sawmill", HOME);
        place(&mut w, &data, "windmill", HOME);
        place(&mut w, &data, "brickworks", HOME);
        run(&mut w, &data, 90.0);
        w.state_hash()
    };
    assert_eq!(build(), build());
}

#[test]
fn mission_one_starts_with_a_working_town() {
    let (data, mut w) = setup(GameMode::Campaign);
    let huts = data.building_id("hut").unwrap();
    let woodcutter = data.building_id("woodcutter").unwrap();
    assert_eq!(w.buildings.iter().filter(|(_, b)| b.kind == huts && b.complete).count(), 2, "zwei Startsiedlungs-Hütten");
    assert_eq!(w.players[0].pop_cap, 18, "Versorgung der Einheiten bleibt unverändert");
    let wc = w.buildings.iter().find(|(_, b)| b.kind == woodcutter).map(|(id, _)| id).expect("Holzfällerlager");
    let wood0 = w.players[0].resources.wood;
    run(&mut w, &data, 100.0);
    assert!(w.players[0].residents >= 5.0, "Bewohner ziehen ein: {}", w.players[0].residents);
    assert!(!w.players[0].unrest, "kein Aufruhr beim Standardsteuersatz");
    assert_eq!(building_status(&w, wc), ProdStatus::Running, "das Holzfällerlager arbeitet");
    assert!(w.players[0].resources.wood > wood0 || w.players[0].stats.gathered.wood > 0.0, "Holz kommt herein");
    // Ausrüstung für die ersten Soldaten liegt bereit.
    assert!(stock(&w, &data, "weapons") >= 8.0 && stock(&w, &data, "armor") >= 3.0);
    // Die optionalen Wirtschaftsziele werden angeboten.
    assert!(w.mission.has_objective("obj_town"));
    assert!(w.mission.has_objective("obj_farm"));
}
