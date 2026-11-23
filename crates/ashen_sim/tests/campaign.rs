//! Tests der Echtzeit-Kampagne: Plan, Missionsdaten, Entscheidungen, Heldenfortschritt, Speicherstand
//! und je Mission ein deterministischer Testspieler, der gewinnt.

use ashen_sim::campaign::{read_slot, slot_path, CampaignDef, CampaignProgress, MissionStatus, SlotState};
use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{GameData, UnitClass};
use ashen_sim::entity::{EntityRef, UnitId, ENEMY, PLAYER};
use ashen_sim::mission::{Action, Condition, DecisionDef, GameMode, MatchSettings, MissionDef, Outcome};
use ashen_sim::world::SimWorld;
use glam::Vec2;
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn data() -> GameData {
    GameData::load_dir(&assets().join("data")).expect("data")
}

fn campaign() -> CampaignDef {
    CampaignDef::load(&assets().join("data/campaign/campaign.ron")).expect("Kampagnenplan")
}

fn mission(id: &str) -> MissionDef {
    let p = if id == "m01_last_settlement" { assets().join("data/missions/m01_last_settlement.ron") } else { assets().join(format!("data/campaign/{id}.ron")) };
    MissionDef::load(&p).unwrap_or_else(|e| panic!("{e}"))
}

fn all_missions() -> Vec<MissionDef> {
    campaign().missions().map(|(_, m)| mission(&m.id)).collect()
}

fn start(def: &MissionDef, flags: &[&str]) -> (MissionDef, MatchSettings) {
    let mut p = CampaignProgress::new("Test");
    p.flags = flags.iter().map(|s| s.to_string()).collect();
    p.start(def)
}

fn world(def: &MissionDef, flags: &[&str]) -> SimWorld {
    let (m, s) = start(def, flags);
    SimWorld::new(&data(), m, s).unwrap_or_else(|e| panic!("{}: {e}", def.id))
}

// ------------------------------------------------------------------------------------------ Plan

#[test]
fn plan_has_seven_ages_with_two_or_three_missions_each() {
    let c = campaign();
    assert_eq!(c.chapters.len(), 7);
    for ch in &c.chapters {
        assert!((2..=3).contains(&ch.missions.len()), "{}: {} Missionen", ch.id, ch.missions.len());
    }
    assert!(c.mission_count() >= 16, "mindestens 16 Missionen, gefunden {}", c.mission_count());
    assert_eq!(c.chapters[0].missions[0].id, "m01_last_settlement", "Mission 1 bleibt der Auftakt");
    assert!(c.endings.len() >= 2);
    let missions = all_missions();
    let errs = c.validate(&missions);
    assert!(errs.is_empty(), "{errs:?}");
}

#[test]
fn campaign_unlocks_in_order() {
    let c = campaign();
    let mut p = CampaignProgress::new("Test");
    let mut order = Vec::new();
    let mut guard = 0;
    while let Some(next) = c.next(&p).map(|s| s.to_string()) {
        order.push(next.clone());
        let def = mission(&next);
        let _ = def;
        p.completed.push(ashen_sim::campaign::MissionResult { id: next, ..Default::default() });
        guard += 1;
        assert!(guard < 40);
    }
    assert!(c.is_finished(&p));
    assert_eq!(order.len(), c.mission_count(), "alle Missionen sind auf dem Standardweg spielbar");
    assert_eq!(c.status(&p, "m01_last_settlement"), MissionStatus::Completed);
}

// -------------------------------------------------------------------------- Missionen im Einzelnen

fn collect_actions<'a>(actions: &'a [Action], out: &mut Vec<&'a Action>) {
    for a in actions {
        out.push(a);
        if let Action::Decision(d) = a {
            for o in &d.options {
                collect_actions(&o.actions, out);
            }
        }
    }
}

fn mission_actions(def: &MissionDef) -> Vec<&Action> {
    let mut out = Vec::new();
    for t in def.triggers.iter().chain(def.variants.iter().flat_map(|v| v.triggers.iter())) {
        collect_actions(&t.actions, &mut out);
    }
    out
}

fn check_condition(def: &MissionDef, c: &Condition, tags: &[String]) {
    match c {
        Condition::UnitsInArea { area, .. } | Condition::UnitKindInArea { area, .. } => assert!(def.area(area).is_some(), "{}: Bereich {area}", def.id),
        Condition::TagInArea { tag, area } => {
            assert!(def.area(area).is_some(), "{}: Bereich {area}", def.id);
            assert!(tags.contains(tag), "{}: Tag {tag}", def.id);
        }
        Condition::TagDead(t) | Condition::TagDiscovered(t) | Condition::TagDamaged(t) | Condition::TagHealthBelow { tag: t, .. } => assert!(tags.contains(t), "{}: Tag {t}", def.id),
        Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| check_condition(def, c, tags)),
        Condition::Not(c) => check_condition(def, c, tags),
        _ => {}
    }
}

#[test]
fn every_mission_is_well_formed() {
    let d = data();
    for def in all_missions() {
        assert!(!def.briefing.is_empty() && !def.description.is_empty(), "{}: Briefing", def.id);
        assert!(!def.kind.is_empty() && !def.victory_text.is_empty() && !def.defeat_text.is_empty(), "{}: Typ und Schlusstexte", def.id);
        assert!(def.id == "m01_last_settlement" || def.map.is_some(), "{}: eigene Karte", def.id);
        assert!(!def.objectives.is_empty() && !def.triggers.is_empty(), "{}: Ziele und Trigger", def.id);
        // Tags: aus allen Platzierungen (auch der Varianten) und aus SpawnUnits.
        let mut tags: Vec<String> = def.placements.iter().chain(def.variants.iter().flat_map(|v| v.placements.iter())).filter_map(|p| p.tag.clone()).collect();
        let actions = mission_actions(&def);
        for a in &actions {
            if let Action::SpawnUnits { tag: Some(t), .. } = a {
                tags.push(t.clone());
            }
        }
        let areas: Vec<&str> = def.areas.iter().chain(def.variants.iter().flat_map(|v| v.areas.iter())).map(|a| a.id.as_str()).collect();
        let has_victory = actions.iter().any(|a| matches!(a, Action::Victory));
        assert!(has_victory, "{}: Siegauslöser fehlt", def.id);
        // Bedingungen und Aktionen verweisen auf Vorhandenes.
        let mut full = def.clone();
        for v in &def.variants {
            full.areas.extend(v.areas.iter().cloned());
        }
        for t in def.triggers.iter().chain(def.variants.iter().flat_map(|v| v.triggers.iter())) {
            check_condition(&full, &t.condition, &tags);
        }
        for a in &actions {
            match a {
                Action::SpawnUnits { unit, area, .. } => {
                    assert!(areas.contains(&area.as_str()), "{}: Bereich {area}", def.id);
                    assert!(d.unit_id(unit).is_some(), "{}: Einheit {unit}", def.id);
                }
                Action::ChangeOwner { tag, .. } => assert!(tags.contains(tag), "{}: Tag {tag}", def.id),
                Action::MoveTag { tag, area, .. } => {
                    assert!(tags.contains(tag), "{}: Tag {tag}", def.id);
                    assert!(areas.contains(&area.as_str()), "{}: Bereich {area}", def.id);
                }
                Action::Decision(dd) => {
                    assert!(dd.options.len() >= 2, "{}: Entscheidung {} braucht mindestens zwei Optionen", def.id, dd.id);
                    assert!(dd.voice.is_some(), "{}: Entscheidung {} vertont", def.id, dd.id);
                }
                _ => {}
            }
        }
        for o in &def.objectives {
            assert!(!o.text.is_empty());
        }
        for a in &def.areas {
            let m = def.map.as_ref();
            if let Some(m) = m {
                assert!(a.x >= 0 && a.y >= 0 && a.x + a.w <= m.width && a.y + a.h <= m.height, "{}: Bereich {} liegt auf der Karte", def.id, a.id);
            }
        }
        // Weltaufbau mit und ohne Entscheidungen.
        let w = world(&def, &[]);
        assert!(w.units.iter().any(|(_, u)| u.owner == PLAYER), "{}: Spielereinheiten", def.id);
        let all_flags: Vec<String> = def.variants.iter().flat_map(|v| v.requires.iter().cloned()).collect();
        let refs: Vec<&str> = all_flags.iter().map(|s| s.as_str()).collect();
        let w = world(&def, &refs);
        assert!(w.units.iter().any(|(_, u)| u.owner == PLAYER), "{}: Spielereinheiten mit Flags", def.id);
        // Alle Startplatzierungen auf begehbaren Kacheln.
        let mut pf = ashen_sim::path::Pathfinder::default();
        for st in def.starts.iter().filter(|s| s.owner == 0) {
            for en in def.starts.iter().filter(|s| s.owner != 0) {
                let from = w.map.nearest_walkable(glam::IVec2::new(st.x as i32, st.y as i32), 6).expect("Start begehbar");
                let to = w.map.nearest_walkable(glam::IVec2::new(en.x as i32, en.y as i32), 8).expect("Gegnerstart begehbar");
                let goal = ashen_sim::map::TileMap::center_of(to);
                let p = pf.find_for(&w.map, ashen_sim::map::TileMap::center_of(from), goal, 400_000, PLAYER);
                assert!(p.is_some_and(|p| p.last().is_some_and(|e| (*e - goal).length() < 1.6)), "{}: Start nicht mit Gegner verbunden", def.id);
            }
        }
    }
}

#[test]
fn flags_used_by_missions_are_set_somewhere_earlier() {
    let c = campaign();
    let mut set: Vec<String> = Vec::new();
    for (_, cm) in c.missions() {
        let def = mission(&cm.id);
        // Gelesene Flags müssen in einer früheren oder derselben Mission gesetzt werden.
        for a in mission_actions(&def) {
            match a {
                Action::SetFlag(f) => set.push(f.clone()),
                Action::Decision(d) => set.extend(d.options.iter().map(|o| o.flag.clone())),
                _ => {}
            }
        }
        fn conds(c: &Condition, out: &mut Vec<String>) {
            match c {
                Condition::Flag(f) => out.push(f.clone()),
                Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| conds(c, out)),
                Condition::Not(c) => conds(c, out),
                _ => {}
            }
        }
        let mut read = Vec::new();
        for t in def.triggers.iter().chain(def.variants.iter().flat_map(|v| v.triggers.iter())) {
            conds(&t.condition, &mut read);
        }
        for v in &def.variants {
            read.extend(v.requires.iter().chain(v.forbids.iter()).cloned());
        }
        for a in mission_actions(&def) {
            if let Action::Decision(d) = a {
                for o in &d.options {
                    read.extend(o.requires.iter().cloned());
                }
            }
        }
        for f in read {
            assert!(set.contains(&f), "{}: Flag '{f}' wird nirgends zuvor gesetzt", def.id);
        }
    }
}

#[test]
fn at_least_six_decisions_and_two_endings() {
    let c = campaign();
    let mut decisions: Vec<(String, DecisionDef)> = Vec::new();
    for (_, cm) in c.missions() {
        for a in mission_actions(&mission(&cm.id)) {
            if let Action::Decision(d) = a {
                decisions.push((cm.id.clone(), d.clone()));
            }
        }
    }
    assert!(decisions.len() >= 6, "Entscheidungen: {}", decisions.len());
    // Eindeutige Flags je Option.
    let mut flags: Vec<&str> = decisions.iter().flat_map(|(_, d)| d.options.iter().map(|o| o.flag.as_str())).collect();
    let n = flags.len();
    flags.sort();
    flags.dedup();
    assert_eq!(flags.len(), n, "Flags der Entscheidungen sind eindeutig");
    let finals = c.endings.iter().filter(|e| !e.requires.is_empty()).count();
    assert!(c.endings.len() >= 2 && finals >= 1);
}

// ------------------------------------------------------------------- Entscheidungen verändern Missionen

fn military(w: &SimWorld, d: &GameData) -> usize {
    w.units.iter().filter(|(_, u)| u.owner == PLAYER && !matches!(d.unit(u.kind).class, UnitClass::Worker)).count()
}

fn enemies(w: &SimWorld) -> usize {
    w.units.iter().filter(|(_, u)| u.owner == ENEMY).count()
}

/// Je Entscheidungsflag mindestens eine spätere Mission, deren Aufbau sich dadurch ändert.
#[test]
fn decisions_change_later_missions() {
    let d = data();
    let c = campaign();
    let mut all_flags: Vec<(String, String)> = Vec::new(); // (Mission der Entscheidung, Flag)
    for (_, cm) in c.missions() {
        for a in mission_actions(&mission(&cm.id)) {
            if let Action::Decision(dd) = a {
                for o in &dd.options {
                    all_flags.push((cm.id.clone(), o.flag.clone()));
                }
            }
        }
    }
    // Flags der letzten Entscheidung (Enden) wirken über den Plan.
    let mut changed = 0;
    for (src, flag) in &all_flags {
        // Die Wahl der Schlussmission entscheidet über das Ende der Kampagne.
        if c.entry(src).is_some_and(|(_, m)| m.finale) {
            continue;
        }
        let mut effects: Vec<String> = Vec::new();
        for (_, cm) in c.missions() {
            let def = mission(&cm.id);
            if cm.id == *src {
                continue;
            }
            let (m0, s0) = start(&def, &[]);
            let (m1, s1) = start(&def, &[flag.as_str()]);
            let w0 = SimWorld::new(&d, m0, s0).unwrap();
            let w1 = SimWorld::new(&d, m1, s1).unwrap();
            let differs = military(&w0, &d) != military(&w1, &d)
                || enemies(&w0) != enemies(&w1)
                || w0.players[0].resources != w1.players[0].resources
                || w0.mission_def.triggers.len() != w1.mission_def.triggers.len()
                || w0.units.iter().count() != w1.units.iter().count();
            if differs {
                effects.push(cm.id.clone());
            }
        }
        // Verfügbarkeit von Missionen und Enden zählt ebenfalls.
        let ends = c.endings.iter().any(|e| e.requires.contains(flag));
        let gates = c.missions().any(|(_, m)| m.if_flags.contains(flag) || m.unless_flags.contains(flag));
        if effects.is_empty() && !ends && !gates {
            // Entscheidungen in der letzten Mission wirken über Optionen und Enden; andere müssen später wirken.
            let opens_option = all_flags.iter().any(|_| false);
            assert!(opens_option, "Flag {flag} (Mission {src}) hat keine Folgen");
        } else {
            changed += 1;
        }
    }
    assert!(changed >= 6);
    // Konkrete, nachweisbare Folgen.
    let m07 = mission("m07_iron_hills");
    let base = world(&m07, &[]);
    let corvane = world(&m07, &["oath_corvane"]);
    assert!(military(&corvane, &d) > military(&base, &d), "Mireilles Speere verstärken die Belagerer");
    let m28 = mission("m28_gates_of_solmere");
    let w0 = world(&m28, &[]);
    let w1 = world(&m28, &["torvald_blood", "rootmother_pact", "fleet_pact"]);
    assert!(military(&w1, &d) > military(&w0, &d) + 20, "Verbündete des Bundes");
    assert!(w1.units.iter().any(|(_, u)| d.unit(u.kind).id == "hero_torvald" && u.owner == PLAYER), "Torvald tritt bei");
    let w2 = world(&m28, &["sun_truth"]);
    assert!(enemies(&w2) < enemies(&w0), "Die Wahrheit spaltet die Häretiker");
    let m11 = mission("m11_marrowfen_rift");
    assert!(enemies(&world(&m11, &["nel_sheltered"])) < enemies(&world(&m11, &[])), "Nel weist den Weg");
    let m14 = mission("m14_black_reef");
    assert!(enemies(&world(&m14, &["rift_harvested"])) > enemies(&world(&m14, &[])), "Die Hohlung wächst");
}

#[test]
fn decisions_set_flags_and_options_can_be_gated() {
    let d = data();
    let def = mission("m29_hollow_throne");
    let (m, s) = start(&def, &[]);
    let mut w = SimWorld::new(&d, m, s).unwrap();
    // Die letzte Entscheidung direkt auslösen.
    let dec = mission_actions(&def).into_iter().find_map(|a| if let Action::Decision(x) = a { Some(x.clone()) } else { None }).unwrap();
    w.mission.decisions.push(dec.clone());
    w.time = 1000.0;
    // Gesperrte Option (Erlösung) wird abgelehnt, Siegel gilt.
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Decide { decision: dec.id.clone(), option: 2 } });
    w.step(&d);
    assert!(!w.mission.has_flag("final_redeem"), "ohne Nel und die Wahrheit keine Erlösung");
    assert_eq!(w.mission.decisions.len(), 1);
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Decide { decision: dec.id.clone(), option: 0 } });
    w.step(&d);
    assert!(w.mission.has_flag("final_seal"));
    assert!(w.mission.decisions.is_empty());
    assert_eq!(w.mission.choices, vec![("final".to_string(), "final_seal".to_string())]);
    // Mit den richtigen Flags ist sie offen.
    let (m, s) = start(&def, &["nel_sheltered", "sun_truth"]);
    let mut w = SimWorld::new(&d, m, s).unwrap();
    w.mission.decisions.push(dec.clone());
    w.time = 1000.0;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Decide { decision: dec.id.clone(), option: 2 } });
    w.step(&d);
    assert!(w.mission.has_flag("final_redeem"));
}

#[test]
fn endings_depend_on_flags() {
    let c = campaign();
    let mut p = CampaignProgress::new("Test");
    assert_eq!(c.ending(&p).unwrap().id, "end_seal");
    p.flags = vec!["final_crown".into()];
    assert_eq!(c.ending(&p).unwrap().id, "end_crown");
    p.flags = vec!["final_redeem".into()];
    assert_eq!(c.ending(&p).unwrap().id, "end_redeem");
    p.flags = vec!["final_destroy".into()];
    assert_eq!(c.ending(&p).unwrap().id, "end_destroy");
}

// -------------------------------------------------------------------------- Heldenfortschritt, Speichern

#[test]
fn hero_progress_carries_between_missions_and_progress_saves() {
    let d = data();
    let c = campaign();
    let m02 = mission("m02_harrowmere");
    let (m, s) = start(&m02, &[]);
    let mut w = SimWorld::new(&d, m, s).unwrap();
    let hero = w.hero_of(&d, PLAYER).expect("Heldin");
    // Stufe 5, Talentpunkte, Ausrüstung aus dem RPG-System.
    {
        let sword = d.item_def_id("embersteel_blade").expect("Item");
        let u = w.units.get_mut(hero).unwrap();
        u.level = 5;
        u.xp = 400;
        u.talents[0] = 1;
        u.equipment[0] = Some(sword);
        u.inventory.push(d.item_def_id("potion_heal").unwrap());
    }
    ashen_sim::systems::heroes::recompute(&mut w, &d, hero);
    let hp5 = w.units.get(hero).unwrap().max_hp;
    w.mission.outcome = Some(Outcome::Victory);
    w.mission.flags.push("nel_sheltered".into());
    let mut p = CampaignProgress::new("Isolde");
    assert!(p.record(&m02, &w, &d));
    assert!(p.is_completed("m02_harrowmere") && p.has_flag("nel_sheltered"));
    assert_eq!(p.hero("hero_isolde").unwrap().level, 5);
    // Speichern und Laden.
    let dir = std::env::temp_dir().join(format!("ashen_campaign_flow_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    p.save(&slot_path(&dir, 1)).unwrap();
    let SlotState::Ready(q) = read_slot(&dir, 1, &c) else { panic!("Profil lesbar") };
    assert_eq!(*q, p);
    // Nächste Mission übernimmt Stufe, Talente, Ausrüstung.
    let m03 = mission("m03_grey_road");
    let (m, s) = q.start(&m03);
    let w3 = SimWorld::new(&d, m, s).unwrap();
    let h3 = w3.hero_of(&d, PLAYER).unwrap();
    let u = w3.units.get(h3).unwrap();
    assert_eq!(u.level, 5);
    assert_eq!(u.talents[0], 1);
    assert_eq!(u.equipment[0], d.item_def_id("embersteel_blade"));
    assert_eq!(u.inventory.len(), 1);
    assert!((u.max_hp - hp5).abs() < 0.5, "Lebenspunkte nach Stufe und Ausrüstung: {} gegenüber {hp5}", u.max_hp);
    // Niederlagen schalten nichts frei.
    let mut lost = w3;
    lost.mission.outcome = Some(Outcome::Defeat);
    let mut q2 = (*q).clone();
    assert!(!q2.record(&m03, &lost, &d));
    assert!(!q2.is_completed("m03_grey_road"));
    // Wiederholung ersetzt die alten Entscheidungen.
    let (m, s) = q2.start(&m02);
    let mut again = SimWorld::new(&d, m, s).unwrap();
    again.mission.outcome = Some(Outcome::Victory);
    again.mission.flags.retain(|f| f != "nel_sheltered");
    again.mission.flags.push("nel_sent".into());
    assert!(q2.record(&m02, &again, &d));
    assert!(q2.has_flag("nel_sent") && !q2.has_flag("nel_sheltered"));
    assert_eq!(q2.completed.iter().filter(|r| r.id == "m02_harrowmere").count(), 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn mission_variants_apply_from_saved_world() {
    // Die Flags bleiben im Spielstand erhalten.
    let d = data();
    let def = mission("m04_long_night");
    let (m, s) = start(&def, &["nel_sheltered"]);
    let w = SimWorld::new(&d, m, s).unwrap();
    assert!(w.mission.has_flag("nel_sheltered"));
    let tmp = std::env::temp_dir().join(format!("ashen_campaign_save_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let p = ashen_sim::save::save_to(&tmp, "kampagne", &w, &d, String::new()).unwrap();
    let (_, payload) = ashen_sim::save::load_from(&p, &d).unwrap();
    assert!(payload.world.mission.has_flag("nel_sheltered"));
    let _ = std::fs::remove_dir_all(&tmp);
}

// ------------------------------------------------------------------------------------------ Audio

#[test]
fn all_voice_events_exist_in_the_manifest() {
    let manifest = std::fs::read_to_string(assets().join("audio/manifest.ron")).unwrap();
    let mut ids: Vec<String> = Vec::new();
    for (i, _) in manifest.match_indices("id: \"") {
        let rest = &manifest[i + 5..];
        if let Some(end) = rest.find('"') {
            ids.push(rest[..end].to_string());
        }
    }
    let exists = |v: &Option<String>, what: &str| {
        if let Some(v) = v {
            assert!(ids.contains(v), "{what}: Audio-Event {v} fehlt im Manifest");
        }
    };
    let c = campaign();
    for (ch, _) in c.missions() {
        exists(&ch.voice, &ch.id);
    }
    for e in &c.endings {
        exists(&e.voice, &e.id);
        assert!(e.voice.is_some(), "Ende {} ist vertont", e.id);
    }
    let mut checked = 0;
    for (_, cm) in c.missions() {
        let def = mission(&cm.id);
        let mut acts = mission_actions(&def);
        for t in &def.triggers {
            let mut v = Vec::new();
            collect_actions(&t.actions, &mut v);
            acts.extend(v);
        }
        for a in acts {
            match a {
                Action::Message { voice, .. } => exists(voice, &def.id),
                Action::Dialogue(lines) => {
                    for l in lines {
                        assert!(l.voice.is_some(), "{}: Dialogzeile ohne Audio-Event: {}", def.id, l.text);
                        exists(&l.voice, &def.id);
                        checked += 1;
                    }
                }
                Action::Decision(d) => {
                    exists(&d.voice, &def.id);
                    checked += 1;
                }
                _ => {}
            }
        }
        for t in def.variants.iter().flat_map(|v| v.triggers.iter()) {
            let mut v = Vec::new();
            collect_actions(&t.actions, &mut v);
            for a in v {
                if let Action::Dialogue(lines) = a {
                    for l in lines {
                        exists(&l.voice, &def.id);
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(checked > 150, "{checked} Sprechzeilen geprüft");
    // Musikwünsche der Missionen stehen im Manifest.
    for def in all_missions() {
        for m in &def.music {
            assert!(ids.contains(m), "{}: Musikstück {m} fehlt im Manifest", def.id);
        }
    }
}

// ------------------------------------------------------------------------------------- Testspieler

fn verbose() -> bool {
    std::env::var_os("ASHEN_TEST_LOG").is_some()
}

fn run_ticks(w: &mut SimWorld, d: &GameData, ticks: u32, mut each: impl FnMut(&mut SimWorld, u32)) {
    for t in 0..ticks {
        each(w, t);
        w.step(d);
        if verbose() {
            for e in &w.events {
                if let ashen_sim::SimEvent::Message { text, .. } = e {
                    eprintln!("  {:.0}s {}", w.time, text);
                }
            }
        }
        w.events.clear();
        if w.mission.outcome.is_some() {
            break;
        }
    }
}

#[derive(Clone)]
enum Plan {
    /// Streitmacht hält die Mitte des Bergfrieds.
    Defend,
    /// Soldaten brechen den Weg zum Bereich, Zivilisten folgen dem Heer.
    Escort(&'static str),
    /// Heer zieht nacheinander durch die Bereiche (Bereich, frühester Zeitpunkt).
    March(Vec<(&'static str, f32)>),
    /// Belagerung: Maschinen gegen das Tor (Gebäude-ID), dann alles gegen das Ziel-Tag.
    Siege { gate: &'static str, keep_tag: &'static str, camp: &'static str },
}

struct Bot {
    plan: Plan,
    choices: Vec<(&'static str, usize)>,
    wp: usize,
}

impl Bot {
    fn new(plan: Plan, choices: &[(&'static str, usize)]) -> Bot {
        Bot { plan, choices: choices.to_vec(), wp: 0 }
    }

    fn step(&mut self, w: &mut SimWorld, d: &GameData, t: u32) {
        if !t.is_multiple_of(20) {
            return;
        }
        // Entscheidungen.
        if let Some(dec) = w.mission.open_decision(w.time).cloned() {
            let opt = self.choices.iter().find(|(id, _)| *id == dec.id).map(|c| c.1).unwrap_or(0);
            w.push_command(PlayerCommand { player: PLAYER, command: Command::Decide { decision: dec.id, option: opt } });
            return;
        }
        // Ausbildung in vorhandenen Kasernen.
        if t.is_multiple_of(40) {
            let kinds = ["swordsman", "archer", "spearman"];
            let barracks: Vec<_> = w.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete && d.building(b.kind).id == "barracks" && b.queue.len() < 2).map(|(id, _)| id).collect();
            for (i, b) in barracks.into_iter().enumerate() {
                let unit = d.unit_id(kinds[(t as usize / 40 + i) % 3]).unwrap();
                w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: b, unit } });
            }
        }
        let army: Vec<UnitId> = w.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && !matches!(d.unit(u.kind).class, UnitClass::Worker)).map(|(id, _)| id).collect();
        let workers: Vec<UnitId> = w.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && d.unit(u.kind).class == UnitClass::Worker).map(|(id, _)| id).collect();
        if army.is_empty() {
            return;
        }
        let center = army.iter().filter_map(|i| w.units.get(*i)).map(|u| u.pos).sum::<Vec2>() / army.len() as f32;
        if verbose() && t.is_multiple_of(100) {
            let tagged: Vec<String> = w.units.iter().filter(|(_, u)| u.tag.as_deref().is_some_and(|t| t != "hero")).map(|(_, u)| format!("{}@{:.0},{:.0}/{:.0}hp", u.tag.clone().unwrap_or_default(), u.pos.x, u.pos.y, u.hp)).collect();
            let first = w.units.get(army[0]).map(|u| format!("{:?} path {} pending {}", u.order, u.path.len(), u.path_pending)).unwrap_or_default();
            eprintln!("  [{:.0}s] Heer {} bei {:.0},{:.0} wp {} {:?} {}", w.time, army.len(), center.x, center.y, self.wp, tagged, first);
        }
        match self.plan.clone() {
            Plan::Defend => {
                let home = w.building_with_tag("player_keep").and_then(|b| w.buildings.get(b)).map(|b| b.center()).unwrap_or(center);
                let threat = w.units.iter().filter(|(_, u)| u.owner == ENEMY && (u.pos - home).length() < 26.0).map(|(_, u)| u.pos).min_by(|a, b| (*a - home).length().total_cmp(&(*b - home).length()));
                let target = threat.map(|p| home + (p - home).clamp_length_max(9.0)).unwrap_or(home + Vec2::new(0.0, 3.0));
                w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army, target, attack_move: true, queue: false } });
            }
            Plan::Escort(area) => {
                let a = w.mission_def.area(area).expect("Bereich").center();
                w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army, target: a, attack_move: true, queue: false } });
                w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: workers, target: center, attack_move: false, queue: false } });
            }
            Plan::March(points) => {
                let (area, not_before) = points[self.wp.min(points.len() - 1)];
                let a = w.mission_def.area(area).unwrap_or_else(|| panic!("Bereich {area}")).center();
                // Befehle nur erneuern, wenn jemand untätig ist (ständiges Neu-Befehlen setzt die Wegsuche zurück).
                let idle = army.iter().any(|i| w.units.get(*i).is_some_and(|u| u.order == ashen_sim::entity::Order::Idle && u.target.is_none()));
                if w.time >= not_before && (idle || t.is_multiple_of(200)) {
                    w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army.clone(), target: a, attack_move: true, queue: false } });
                    // Zivilisten (gerettete Dörfer) folgen dem Heer.
                    if !workers.is_empty() {
                        w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: workers, target: center, attack_move: false, queue: false } });
                    }
                }
                let clear = !w.units.iter().any(|(_, u)| u.owner == ENEMY && u.hp > 0.0 && (u.pos - center).length() < 11.0);
                if (center - a).length() < 7.0 && clear && self.wp + 1 < points.len() && w.time >= not_before {
                    self.wp += 1;
                }
            }
            Plan::Siege { gate, keep_tag, camp } => {
                let gate_id = w.buildings.iter().find(|(_, b)| b.owner == ENEMY && b.hp > 0.0 && d.building(b.kind).id == gate).map(|(id, _)| id);
                let keep = w.building_with_tag(keep_tag);
                let siege: Vec<UnitId> = army.iter().copied().filter(|i| w.units.get(*i).is_some_and(|u| d.unit(u.kind).class == UnitClass::Siege)).collect();
                let rest: Vec<UnitId> = army.iter().copied().filter(|i| !siege.contains(i)).collect();
                let camp_c = w.mission_def.area(camp).expect("Bereich").center();
                match (gate_id, keep) {
                    (Some(g), _) => {
                        w.push_command(PlayerCommand { player: PLAYER, command: Command::Attack { units: siege, target: EntityRef::Building(g) } });
                        w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: rest, target: camp_c, attack_move: true, queue: false } });
                    }
                    (None, Some(k)) => {
                        let kc = w.buildings.get(k).map(|b| b.center()).unwrap_or(camp_c);
                        w.push_command(PlayerCommand { player: PLAYER, command: Command::Attack { units: siege, target: EntityRef::Building(k) } });
                        w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: rest, target: kc, attack_move: true, queue: false } });
                    }
                    _ => {}
                }
            }
        }
    }
}

struct Case {
    id: &'static str,
    flags: &'static [&'static str],
    plan: Plan,
    choices: &'static [(&'static str, usize)],
    seconds: u32,
}

fn play(case: &Case) -> SimWorld {
    let d = data();
    let def = mission(case.id);
    let mut w = world(&def, case.flags);
    let mut bot = Bot::new(case.plan.clone(), case.choices);
    run_ticks(&mut w, &d, case.seconds * 20, |w, t| bot.step(w, &d, t));
    if w.mission.outcome.is_none() && verbose() {
        for (_, u) in w.units.iter().filter(|(_, u)| u.owner == ENEMY) {
            eprintln!("  uebrig: {} bei {:.0},{:.0} hp {:.0} tag {:?} order {:?}", d.unit(u.kind).id, u.pos.x, u.pos.y, u.hp, u.tag, u.order);
        }
    }
    eprintln!(
        "{}: {:?} nach {:.0}s, Verluste {}, Gefallene Gegner {}, Gegner übrig {}, Flags {:?}",
        case.id,
        w.mission.outcome,
        w.time,
        w.players[0].stats.units_lost,
        w.players[0].stats.kills,
        enemies(&w),
        w.mission.flags
    );
    w
}

fn assert_wins(case: Case) -> SimWorld {
    let w = play(&case);
    assert_eq!(w.mission.outcome, Some(Outcome::Victory), "{}: Sieg erwartet (Zeit {:.0}s, Verluste {})", case.id, w.time, w.players[0].stats.units_lost);
    w
}

#[test]
fn m01_is_winnable_with_a_scripted_assault() {
    // Mission 1: Der Schrein fällt unter einem Ansturm von Rittern (wie im Simulationstest, hier mit Entscheidungslogik des Bots).
    let d = data();
    let def = mission("m01_last_settlement");
    let (m, s) = start(&def, &[]);
    let mut w = SimWorld::new(&d, m, s).unwrap();
    let shrine = w.building_with_tag("enemy_shrine").unwrap();
    let center = w.buildings.get(shrine).unwrap().center();
    let knight = d.unit_id("knight").unwrap();
    let army: Vec<_> = (0..24).map(|i| w.spawn_unit(&d, knight, PLAYER, center + Vec2::new(-3.5 + (i % 6) as f32 * 0.6, 3.0 + (i / 6) as f32 * 0.6))).collect();
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Attack { units: army, target: EntityRef::Building(shrine) } });
    run_ticks(&mut w, &d, 20 * 300, |_, _| {});
    assert_eq!(w.mission.outcome, Some(Outcome::Victory));
    // Die Heldin wird für die Kampagne übernommen.
    let mut p = CampaignProgress::new("T");
    assert!(p.record(&def, &w, &d));
    assert!(p.hero("hero_isolde").is_some());
}

#[test]
fn m02_harrowmere_escort_is_winnable() {
    let w = assert_wins(Case { id: "m02_harrowmere", flags: &[], plan: Plan::Escort("brennhollow"), choices: &[("core", 0), ("nel", 0)], seconds: 900 });
    assert!(w.mission.has_flag("core_smashed") && w.mission.has_flag("nel_sheltered"));
}

#[test]
fn m02_other_choices_are_winnable_too() {
    let w = assert_wins(Case { id: "m02_harrowmere", flags: &[], plan: Plan::Escort("brennhollow"), choices: &[("core", 1), ("nel", 1)], seconds: 900 });
    assert!(w.mission.has_flag("core_kept") && w.mission.has_flag("nel_sent"));
}

#[test]
fn m03_grey_road_is_winnable() {
    let w = assert_wins(Case { id: "m03_grey_road", flags: &["core_smashed"], plan: Plan::March(vec![("posten1", 0.0), ("posten2", 0.0), ("wendel_zugang", 0.0)]), choices: &[("wendel", 1)], seconds: 900 });
    assert!(w.mission.has_flag("wendel_recruited"));
    // Die Männer treten in den Dienst der Heldin.
    assert!(w.units.iter().any(|(_, u)| u.owner == PLAYER && u.tag.as_deref() == Some("wendel_men")));
}

#[test]
fn m03_hanging_wendel_is_winnable() {
    let w = assert_wins(Case { id: "m03_grey_road", flags: &[], plan: Plan::March(vec![("posten1", 0.0), ("posten2", 0.0), ("wendel_zugang", 0.0)]), choices: &[("wendel", 0)], seconds: 900 });
    assert!(w.mission.has_flag("wendel_hanged"));
}

#[test]
fn m04_long_night_is_winnable() {
    let w = assert_wins(Case { id: "m04_long_night", flags: &["nel_sheltered", "wendel_recruited"], plan: Plan::Defend, choices: &[("oath", 1)], seconds: 700 });
    assert!(w.mission.has_flag("oath_corvane"));
    assert!(w.players[0].stats.kills >= 40, "echte Schlacht: {} Gefallene", w.players[0].stats.kills);
}

#[test]
fn m06_tourney_is_winnable() {
    let w = assert_wins(Case { id: "m06_tourney_kaldris", flags: &["oath_crown"], plan: Plan::March(vec![("arena", 0.0), ("weg", 45.0)]), choices: &[("kassian_secret", 1)], seconds: 600 });
    assert!(w.mission.has_flag("kassian_leverage"));
}

#[test]
fn m07_iron_hills_siege_is_winnable() {
    assert_wins(Case { id: "m07_iron_hills", flags: &["oath_corvane", "wendel_recruited"], plan: Plan::Siege { gate: "gate", keep_tag: "kargsted_keep", camp: "tor" }, choices: &[], seconds: 900 });
}

#[test]
fn m09_black_winter_is_winnable() {
    let w = assert_wins(Case { id: "m09_black_winter", flags: &["oath_crown", "core_kept"], plan: Plan::Defend, choices: &[("torvald_oath", 0)], seconds: 700 });
    assert!(w.mission.has_flag("torvald_blood"));
}

#[test]
fn m09_with_gold_is_winnable() {
    let w = assert_wins(Case { id: "m09_black_winter", flags: &["oath_corvane"], plan: Plan::Defend, choices: &[("torvald_oath", 1)], seconds: 700 });
    assert!(w.mission.has_flag("torvald_gold"));
}

#[test]
fn m10_root_and_ash_burning_is_winnable() {
    let w = assert_wins(Case { id: "m10_root_and_ash", flags: &["wendel_recruited"], plan: Plan::March(vec![("lager1", 0.0), ("lager2", 0.0), ("lager3", 0.0), ("hain_mitte", 0.0)]), choices: &[("rootmother", 0)], seconds: 1200 });
    assert!(w.mission.has_flag("rootmother_burned"));
}

#[test]
fn m10_root_and_ash_dream_is_winnable() {
    let w = assert_wins(Case { id: "m10_root_and_ash", flags: &["nel_sheltered", "core_kept"], plan: Plan::March(vec![("lager1", 0.0), ("lager2", 0.0), ("lager3", 0.0), ("hain_mitte", 0.0)]), choices: &[("rootmother", 1)], seconds: 1200 });
    assert!(w.mission.has_flag("rootmother_pact"));
}

#[test]
fn m11_marrowfen_is_winnable() {
    let w = assert_wins(Case {
        id: "m11_marrowfen_rift",
        flags: &["nel_sheltered", "rootmother_pact"],
        plan: Plan::March(vec![("dorf1", 0.0), ("dorf2", 0.0), ("dorf3", 0.0), ("riss", 0.0), ("damm", 0.0)]),
        choices: &[("rift", 0)],
        seconds: 1500,
    });
    assert!(w.mission.has_flag("rift_sealed"));
}

#[test]
fn m11_harvesting_the_rift_is_winnable() {
    let w = assert_wins(Case {
        id: "m11_marrowfen_rift",
        flags: &["rootmother_burned"],
        plan: Plan::March(vec![("dorf1", 0.0), ("dorf2", 0.0), ("dorf3", 0.0), ("riss", 0.0), ("damm", 0.0)]),
        choices: &[("rift", 1)],
        seconds: 1500,
    });
    assert!(w.mission.has_flag("rift_harvested"));
}

#[test]
fn m13_ashglass_fields_is_winnable() {
    assert_wins(Case { id: "m13_ashglass_fields", flags: &["wendel_recruited", "torvald_blood", "oath_crown"], plan: Plan::March(vec![("posten1", 0.0), ("posten2", 0.0), ("posten3", 0.0)]), choices: &[], seconds: 1200 });
}

#[test]
fn m14_black_reef_is_winnable() {
    let w = assert_wins(Case { id: "m14_black_reef", flags: &["torvald_blood"], plan: Plan::March(vec![("strand", 0.0), ("damm", 0.0), ("insel", 0.0)]), choices: &[("fleet", 0)], seconds: 1200 });
    assert!(w.mission.has_flag("fleet_pact"));
}

#[test]
fn m17_death_of_a_king_is_winnable() {
    let w = assert_wins(Case { id: "m17_death_of_a_king", flags: &["oath_crown"], plan: Plan::March(vec![("keller", 0.0), ("kapelle", 0.0), ("krypta", 0.0), ("dom", 0.0)]), choices: &[("blame", 3)], seconds: 1200 });
    assert!(w.mission.has_flag("blame_truth"), "mit allen drei Beweisen ist die Wahrheit wählbar");
}

#[test]
fn m17_without_the_crypt_the_truth_is_locked() {
    // Die Wahl der Wahrheit setzt alle Beweise voraus; ohne Krypta bleibt sie gesperrt.
    let d = data();
    let def = mission("m17_death_of_a_king");
    let mut w = world(&def, &[]);
    let dec = mission_actions(&def).into_iter().find_map(|a| if let Action::Decision(x) = a { Some(x.clone()) } else { None }).unwrap();
    w.mission.decisions.push(dec.clone());
    w.time = 1000.0;
    w.push_command(PlayerCommand { player: PLAYER, command: Command::Decide { decision: dec.id.clone(), option: 3 } });
    w.step(&d);
    assert!(!w.mission.has_flag("blame_truth"));
}

#[test]
fn m19_kaldris_ford_is_winnable() {
    let w = assert_wins(Case { id: "m19_kaldris_ford", flags: &["blame_heretics", "torvald_blood", "kassian_warned_king"], plan: Plan::March(vec![("furt", 0.0), ("feind", 0.0)]), choices: &[("throne", 0)], seconds: 900 });
    assert!(w.mission.has_flag("throne_mireille"));
}

#[test]
fn m19_on_the_kings_side_is_winnable() {
    let w = assert_wins(Case { id: "m19_kaldris_ford", flags: &["blame_mireille"], plan: Plan::March(vec![("furt", 0.0), ("feind", 0.0)]), choices: &[("throne", 1)], seconds: 900 });
    assert!(w.mission.has_flag("throne_aldren"));
}

#[test]
fn m21_shrine_oruun_is_winnable() {
    assert_wins(Case { id: "m21_shrine_oruun", flags: &["torvald_blood", "rootmother_pact"], plan: Plan::March(vec![("vorraum", 0.0), ("ahnenhalle", 0.0), ("schreinsaal", 0.0)]), choices: &[], seconds: 1500 });
}

#[test]
fn m25_ithren_light_is_winnable() {
    let w = assert_wins(Case { id: "m25_ithren_light", flags: &["torvald_blood", "blame_truth"], plan: Plan::March(vec![("allee", 0.0), ("schrein", 0.0)]), choices: &[("sun", 0)], seconds: 1500 });
    assert!(w.mission.has_flag("sun_truth"));
}

#[test]
fn m26_ashen_march_holding_is_winnable() {
    let w = assert_wins(Case { id: "m26_ashen_march", flags: &["torvald_blood", "rootmother_pact", "fleet_pact"], plan: Plan::Defend, choices: &[("march", 0)], seconds: 700 });
    assert!(w.mission.has_flag("march_hold"));
}

#[test]
fn m26_ashen_march_evacuation_is_winnable() {
    let w = assert_wins(Case { id: "m26_ashen_march", flags: &["nel_sheltered"], plan: Plan::Escort("pass"), choices: &[("march", 1)], seconds: 700 });
    assert!(w.mission.has_flag("march_evacuate"));
}

#[test]
fn m28_gates_of_solmere_siege_is_winnable() {
    assert_wins(Case {
        id: "m28_gates_of_solmere",
        flags: &["torvald_blood", "rootmother_pact", "fleet_pact", "throne_mireille", "sun_truth", "core_kept", "march_hold"],
        plan: Plan::Siege { gate: "gate", keep_tag: "gate_keep", camp: "tor" },
        choices: &[],
        seconds: 1500,
    });
}

#[test]
fn m29_hollow_throne_is_winnable_with_every_ending() {
    let flags: &[&str] = &["torvald_blood", "rootmother_pact", "fleet_pact", "throne_mireille", "sun_truth", "nel_sheltered", "core_smashed", "rift_sealed", "march_hold"];
    for (option, flag) in [(0usize, "final_seal"), (1, "final_crown"), (2, "final_redeem"), (3, "final_destroy")] {
        let choices: &'static [(&'static str, usize)] = Box::leak(vec![("final", option)].into_boxed_slice());
        let w = assert_wins(Case { id: "m29_hollow_throne", flags, plan: Plan::March(vec![("mauerhof", 0.0), ("sonnenhof", 0.0), ("schleier", 0.0), ("thronsaal", 0.0)]), choices, seconds: 1800 });
        assert!(w.mission.has_flag(flag), "Ende {flag}");
    }
    // Die Enden folgen dem gewählten Flag.
    let c = campaign();
    let mut p = CampaignProgress::new("T");
    p.flags = vec!["final_crown".into()];
    assert_eq!(c.ending(&p).map(|e| e.id.as_str()), Some("end_crown"));
}

#[test]
fn missions_are_deterministic() {
    let case = || Case { id: "m03_grey_road", flags: &[], plan: Plan::March(vec![("posten1", 0.0), ("posten2", 0.0), ("wendel_zugang", 0.0)]), choices: &[("wendel", 1)], seconds: 900 };
    let a = play(&case());
    let b = play(&case());
    assert_eq!(a.tick, b.tick);
    assert_eq!(a.mission.outcome, b.mission.outcome);
    assert_eq!(a.players[0].stats.kills, b.players[0].stats.kills);
    assert_eq!(a.players[0].stats.units_lost, b.players[0].stats.units_lost);
}

#[test]
fn campaign_mode_is_default_for_missions() {
    let d = data();
    let def = mission("m02_harrowmere");
    let (m, s) = start(&def, &[]);
    assert_eq!(s.mode, GameMode::Campaign);
    let w = SimWorld::new(&d, m, s).unwrap();
    // Gegnerische Verstärkungen skalieren mit der Schwierigkeit.
    assert_eq!(w.settings.difficulty.wave_mult(), 1.0);
}
