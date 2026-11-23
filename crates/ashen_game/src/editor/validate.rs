//! Validierung eines Szenarios: erkennt unspielbare Karten und Skripte und gibt Hinweise.

use super::doc::EditDoc;
use ashen_sim::SimWorld;
use ashen_sim::data::GameData;
use ashen_sim::entity::NodeKind;
use ashen_sim::map::TileMap;
use ashen_sim::mission::{Action, Condition, GameMode, MatchSettings};
use ashen_sim::path::Pathfinder;
use glam::{IVec2, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warning,
    Info,
}

impl Level {
    pub fn label(self) -> &'static str {
        match self {
            Level::Error => "Fehler",
            Level::Warning => "Hinweis",
            Level::Info => "Info",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Issue {
    pub level: Level,
    pub text: String,
    /// Kachelposition für „Hinspringen“.
    pub at: Option<Vec2>,
}

impl Issue {
    fn new(level: Level, text: impl Into<String>, at: Option<Vec2>) -> Issue {
        Issue {
            level,
            text: text.into(),
            at,
        }
    }
}

/// Alles, was Trigger über Bereiche, Tags, Ziele und IDs referenzieren.
#[derive(Default)]
struct Refs {
    areas: Vec<String>,
    tags: Vec<String>,
    objectives_added: Vec<String>,
    objectives_completed: Vec<String>,
    objectives_read: Vec<String>,
    units: Vec<String>,
    buildings: Vec<String>,
    victory: bool,
    defeat: bool,
    items: Vec<String>,
}

fn collect_condition(c: &Condition, r: &mut Refs) {
    match c {
        Condition::BuildingCompleted(b) | Condition::BuildingCount { building: b, .. } => {
            r.buildings.push(b.clone())
        }
        Condition::UnitCount { unit, .. } => r.units.push(unit.clone()),
        Condition::ObjectiveCompleted(o) => r.objectives_read.push(o.clone()),
        Condition::TagDead(t) | Condition::TagDiscovered(t) | Condition::TagDamaged(t) => {
            r.tags.push(t.clone())
        }
        Condition::Stock { item, .. } => r.items.push(item.clone()),
        Condition::UnitsInArea { area, .. } => r.areas.push(area.clone()),
        Condition::Not(c) => collect_condition(c, r),
        Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| collect_condition(c, r)),
        _ => {}
    }
}

fn collect(doc: &EditDoc) -> Refs {
    let mut r = Refs::default();
    for t in &doc.def.triggers {
        collect_condition(&t.condition, &mut r);
        for a in &t.actions {
            match a {
                Action::AddObjective(o) => r.objectives_added.push(o.clone()),
                Action::CompleteObjective(o) => r.objectives_completed.push(o.clone()),
                Action::Victory => r.victory = true,
                Action::Defeat => r.defeat = true,
                Action::SpawnUnits { unit, area, .. } => {
                    r.units.push(unit.clone());
                    r.areas.push(area.clone());
                }
                _ => {}
            }
        }
    }
    r
}

/// Prüft das Dokument. Fehler machen das Szenario unspielbar oder unfair, Hinweise sind Anregungen.
pub fn validate(doc: &EditDoc, data: &GameData) -> Vec<Issue> {
    let mut out: Vec<Issue> = Vec::new();
    let def = &doc.def;
    if def.name.trim().is_empty() {
        out.push(Issue::new(
            Level::Warning,
            "Das Szenario hat keinen Namen.",
            None,
        ));
    }
    if def.id.trim().is_empty() {
        out.push(Issue::new(Level::Error, "Die Szenario-ID ist leer.", None));
    }
    // Platzierungen: bekannte IDs, Gelände.
    for p in &def.placements {
        let at = Some(Vec2::new(p.x, p.y));
        if data.unit_id(&p.id).is_none() && data.building_id(&p.id).is_none() {
            out.push(Issue::new(
                Level::Error,
                format!("Unbekannte ID '{}' in der Aufstellung.", p.id),
                at,
            ));
        }
    }
    for (i, p) in def.placements.iter().enumerate() {
        if data.building_id(&p.id).is_some()
            && let Err(e) =
                doc.can_place_building(data, &p.id, IVec2::new(p.x as i32, p.y as i32), Some(i))
        {
            out.push(Issue::new(
                Level::Error,
                format!("Gebäude '{}' steht ungültig: {e}", p.id),
                Some(Vec2::new(p.x, p.y)),
            ));
        } else if data.unit_id(&p.id).is_some()
            && let Err(e) = doc.can_place_unit(Vec2::new(p.x, p.y))
        {
            out.push(Issue::new(
                Level::Error,
                format!("Einheit '{}' steht ungültig: {e}", p.id),
                Some(Vec2::new(p.x, p.y)),
            ));
        }
    }
    let own = def.placements.iter().filter(|p| p.owner == 0).count();
    if own == 0 {
        out.push(Issue::new(
            Level::Error,
            "Die Spielerseite hat weder Einheiten noch Gebäude.",
            None,
        ));
    }
    // Startpositionen.
    let player_starts: Vec<&ashen_sim::scenario::StartPos> =
        def.starts.iter().filter(|s| s.owner == 0).collect();
    if player_starts.is_empty() {
        out.push(Issue::new(
            Level::Error,
            "Keine Startposition für die Spielerseite gesetzt.",
            None,
        ));
    }
    for s in &def.starts {
        let t = IVec2::new(s.x.floor() as i32, s.y.floor() as i32);
        match doc.map.terrain_at(t) {
            None => out.push(Issue::new(
                Level::Error,
                "Eine Startposition liegt außerhalb der Karte.",
                Some(Vec2::new(s.x, s.y)),
            )),
            Some(tr) if !tr.walkable() => out.push(Issue::new(
                Level::Error,
                format!(
                    "Startposition liegt auf unbegehbarem Gelände ({}).",
                    tr.label()
                ),
                Some(Vec2::new(s.x, s.y)),
            )),
            _ => {}
        }
    }
    // Hauptgebäude und Basis.
    let tag_pos = |tag: &str| -> Option<Vec2> {
        def.placements
            .iter()
            .enumerate()
            .find(|(_, p)| p.tag.as_deref() == Some(tag))
            .map(|(i, _)| doc.placement_center(data, i))
    };
    if !def.defeat_tag.is_empty() && tag_pos(&def.defeat_tag).is_none() {
        out.push(Issue::new(
            Level::Error,
            format!(
                "Niederlage-Tag '{}' gehört keiner Platzierung: Die Partie wäre sofort verloren.",
                def.defeat_tag
            ),
            None,
        ));
    }
    if def.waves.enabled
        && (def.enemy_base_tag.is_empty() || tag_pos(&def.enemy_base_tag).is_none())
    {
        out.push(Issue::new(Level::Warning, "Angriffswellen sind aktiv, aber das gegnerische Hauptgebäude (Tag der gegnerischen Basis) fehlt: Es kommen keine Wellen.", None));
    }

    // Ziele und Skript.
    let refs = collect(doc);
    if def.objectives.is_empty() {
        out.push(Issue::new(
            Level::Error,
            "Es sind keine Ziele definiert.",
            None,
        ));
    }
    if !refs.victory {
        out.push(Issue::new(Level::Error, "Kein Auslöser führt zum Sieg: Das Szenario kann nicht gewonnen werden (Aktion „Sieg“ fehlt).", None));
    }
    if !refs.defeat && def.defeat_tag.is_empty() {
        out.push(Issue::new(
            Level::Info,
            "Es gibt keine Niederlagebedingung (weder Niederlage-Tag noch Auslöser).",
            None,
        ));
    }
    for o in &def.objectives {
        if !refs.objectives_added.contains(&o.id) && !refs.objectives_completed.contains(&o.id) {
            out.push(Issue::new(
                Level::Warning,
                format!(
                    "Ziel '{}' wird von keinem Auslöser hinzugefügt oder abgeschlossen.",
                    o.id
                ),
                None,
            ));
        }
    }
    for o in refs
        .objectives_added
        .iter()
        .chain(&refs.objectives_completed)
        .chain(&refs.objectives_read)
    {
        if def.objective(o).is_none() {
            out.push(Issue::new(
                Level::Warning,
                format!("Ein Auslöser verweist auf das unbekannte Ziel '{o}'."),
                None,
            ));
        }
    }
    let mut seen: Vec<&str> = Vec::new();
    for t in &def.triggers {
        if seen.contains(&t.id.as_str()) {
            out.push(Issue::new(
                Level::Error,
                format!("Auslöser-ID '{}' kommt doppelt vor.", t.id),
                None,
            ));
        }
        seen.push(&t.id);
    }
    for a in &refs.areas {
        if def.area(a).is_none() {
            out.push(Issue::new(
                Level::Error,
                format!("Ein Auslöser verweist auf den unbekannten Bereich '{a}'."),
                None,
            ));
        }
    }
    for t in &refs.tags {
        if !def
            .placements
            .iter()
            .any(|p| p.tag.as_deref() == Some(t.as_str()))
        {
            out.push(Issue::new(
                Level::Warning,
                format!("Tag '{t}' gehört keiner Platzierung: Die Bedingung wird nie wahr."),
                None,
            ));
        }
    }
    for u in &refs.units {
        if data.unit_id(u).is_none() {
            out.push(Issue::new(
                Level::Error,
                format!("Ein Auslöser verweist auf die unbekannte Einheit '{u}'."),
                None,
            ));
        }
    }
    for b in &refs.buildings {
        if data.building_id(b).is_none() {
            out.push(Issue::new(
                Level::Error,
                format!("Ein Auslöser verweist auf das unbekannte Gebäude '{b}'."),
                None,
            ));
        }
    }
    for i in &refs.items {
        if data.item(i).is_none() {
            out.push(Issue::new(
                Level::Warning,
                format!("Ein Auslöser verweist auf die unbekannte Ware '{i}'."),
                None,
            ));
        }
    }
    for a in &def.areas {
        if a.w <= 0
            || a.h <= 0
            || a.x < 0
            || a.y < 0
            || a.x + a.w > doc.map.width
            || a.y + a.h > doc.map.height
        {
            out.push(Issue::new(
                Level::Warning,
                format!("Bereich '{}' liegt nicht vollständig auf der Karte.", a.id),
                Some(a.center()),
            ));
        }
    }

    // Weltprobe und Erreichbarkeit.
    let mission = doc.to_mission();
    match SimWorld::new(
        data,
        mission,
        MatchSettings {
            mode: GameMode::Sandbox,
            ..Default::default()
        },
    ) {
        Err(e) => out.push(Issue::new(
            Level::Error,
            format!("Die Welt lässt sich nicht aufbauen: {e}"),
            None,
        )),
        Ok(w) => {
            // Ziele der Erreichbarkeit: gegnerische Starts, Basis, Tag-Ziele, Bereiche.
            let mut targets: Vec<(String, Vec2)> = Vec::new();
            for s in def.starts.iter().filter(|s| s.owner != 0) {
                targets.push(("Gegnerstart".into(), Vec2::new(s.x, s.y)));
            }
            if let Some(p) = tag_pos(&def.enemy_base_tag).filter(|_| !def.enemy_base_tag.is_empty())
            {
                targets.push(("Gegnerische Basis".into(), p));
            }
            for t in refs.tags.iter() {
                if let Some(i) = def
                    .placements
                    .iter()
                    .position(|p| p.tag.as_deref() == Some(t.as_str()) && p.owner != 0)
                {
                    targets.push((format!("Ziel '{t}'"), doc.placement_center(data, i)));
                }
            }
            for a in refs.areas.iter().filter_map(|a| def.area(a)) {
                targets.push((format!("Bereich '{}'", a.id), a.center()));
            }
            targets.dedup_by(|a, b| (a.1 - b.1).length() < 0.5);
            let mut pf = Pathfinder::default();
            for st in player_starts {
                let from = Vec2::new(st.x, st.y);
                let Some(from_t) = w.map.nearest_walkable(TileMap::tile_of(from), 4) else {
                    out.push(Issue::new(
                        Level::Error,
                        "Die Startposition der Spielerseite ist von Hindernissen umschlossen.",
                        Some(from),
                    ));
                    continue;
                };
                for (name, to) in &targets {
                    let Some(to_t) = w.map.nearest_walkable(TileMap::tile_of(*to), 6) else {
                        out.push(Issue::new(
                            Level::Warning,
                            format!("{name} ist von Hindernissen umschlossen."),
                            Some(*to),
                        ));
                        continue;
                    };
                    let (a, b) = (TileMap::center_of(from_t), TileMap::center_of(to_t));
                    let reach = pf
                        .find_for(&w.map, a, b, 200_000, ashen_sim::entity::PLAYER)
                        .is_some_and(|p| p.last().is_some_and(|e| (*e - b).length() < 1.6));
                    if !reach {
                        out.push(Issue::new(Level::Error, format!("{name} ist von der Startposition der Spielerseite aus nicht erreichbar (Wasser oder Hindernisse im Weg)."), Some(*to)));
                    }
                }
            }
            // Wirtschaft: Gold in Reichweite des Spielers.
            if let Some(st) = def.starts.iter().find(|s| s.owner == 0) {
                let near_gold = doc.map.nodes.iter().any(|n| {
                    n.kind == NodeKind::GoldMine
                        && (Vec2::new(n.x as f32, n.y as f32) - Vec2::new(st.x, st.y)).length()
                            < 30.0
                });
                if !near_gold {
                    out.push(Issue::new(
                        Level::Info,
                        "Keine Goldmine im Umkreis von 30 Kacheln um den Spielerstart.",
                        Some(Vec2::new(st.x, st.y)),
                    ));
                }
                if !doc.map.nodes.iter().any(|n| {
                    n.kind == NodeKind::Tree
                        && (Vec2::new(n.x as f32, n.y as f32) - Vec2::new(st.x, st.y)).length()
                            < 25.0
                }) {
                    out.push(Issue::new(Level::Info, "Keine Bäume im Umkreis von 25 Kacheln um den Spielerstart: Holz muss gehandelt werden.", Some(Vec2::new(st.x, st.y))));
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|i| seen.insert(i.text.clone()));
    out.sort_by_key(|i| i.level);
    out
}

pub fn has_errors(issues: &[Issue]) -> bool {
    issues.iter().any(|i| i.level == Level::Error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::doc::BrushShape;
    use ashen_sim::genmap::GenPreset;
    use ashen_sim::map::Terrain;
    use ashen_sim::mission::Action;
    use std::path::PathBuf;

    fn data() -> GameData {
        GameData::load_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data"))
            .expect("data")
    }

    fn texts(issues: &[Issue], level: Level) -> Vec<&str> {
        issues
            .iter()
            .filter(|i| i.level == level)
            .map(|i| i.text.as_str())
            .collect()
    }

    #[test]
    fn generated_map_validates_cleanly() {
        let d = data();
        let doc = EditDoc::generated(64, 64, 55, GenPreset::Aschmark);
        let issues = validate(&doc, &d);
        assert!(!has_errors(&issues), "{:?}", texts(&issues, Level::Error));
    }

    #[test]
    fn unreachable_start_and_missing_goals_are_reported() {
        let d = data();
        let mut doc = EditDoc::generated(64, 64, 55, GenPreset::Riverlands);
        // Ein tiefer Graben trennt die Spielerseite vom Rest der Karte.
        let p = doc.def.starts[0].clone();
        for y in 0..doc.map.height {
            for dx in 0..3 {
                let t = IVec2::new(p.x as i32 + 12 + dx, y);
                doc.paint_terrain(&BrushShape::Square.tiles(t, 1), Terrain::DeepWater);
            }
        }
        let issues = validate(&doc, &d);
        let errors = texts(&issues, Level::Error);
        assert!(
            errors.iter().any(|e| e.contains("nicht erreichbar")),
            "{errors:?}"
        );
        // Ziele und Sieg entfernen.
        doc.def.objectives.clear();
        for t in &mut doc.def.triggers {
            t.actions.retain(|a| !matches!(a, Action::Victory));
        }
        let issues = validate(&doc, &d);
        let errors = texts(&issues, Level::Error);
        assert!(
            errors.iter().any(|e| e.contains("keine Ziele")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("nicht gewonnen")),
            "{errors:?}"
        );
    }

    #[test]
    fn invalid_start_and_references_are_reported() {
        let d = data();
        let mut doc = EditDoc::generated(48, 48, 9, GenPreset::Heartland);
        let s = doc.def.starts[0].clone();
        doc.paint_terrain(
            &BrushShape::Square.tiles(IVec2::new(s.x as i32, s.y as i32), 2),
            Terrain::Water,
        );
        doc.def.defeat_tag = "gibt_es_nicht".into();
        doc.def.triggers.push(ashen_sim::mission::TriggerDef {
            id: "kaputt".into(),
            condition: Condition::UnitsInArea {
                area: "nirgends".into(),
                count: 1,
            },
            actions: vec![Action::SpawnUnits {
                unit: "drache".into(),
                owner: 1,
                area: "nirgends".into(),
                count: 1,
                attack: false,
            }],
            modes: vec![],
        });
        let issues = validate(&doc, &d);
        let errors = texts(&issues, Level::Error);
        assert!(
            errors.iter().any(|e| e.contains("unbegehbarem Gelände")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("Niederlage-Tag")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("Bereich 'nirgends'")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("Einheit 'drache'")),
            "{errors:?}"
        );
    }
}
