//! Missions-Trigger-Editor: Bedingungen und Aktionen aus `mission.rs` per Oberfläche zusammenstellen.

use super::doc::EditDoc;
use ashen_sim::data::{Cost, GameData};
use ashen_sim::env::Weather;
use ashen_sim::mission::{Action, Condition, DecisionDef, DecisionOption, DialogueLine, GameMode, ObjectiveDef, TriggerDef};
use egui::{Color32, RichText, Ui};

use ashen_engine::ui::theme;

/// Auswahllisten für Verweise (Bereiche, Tags, Ziele, Einheiten, Gebäude, Waren).
#[derive(Default)]
pub struct Lists {
    pub areas: Vec<String>,
    pub objectives: Vec<String>,
    pub tags: Vec<String>,
    pub units: Vec<(String, String)>,
    pub buildings: Vec<(String, String)>,
    pub items: Vec<(String, String)>,
}

impl Lists {
    pub fn from_doc(doc: &EditDoc, data: &GameData) -> Lists {
        let mut tags: Vec<String> = doc.def.placements.iter().filter_map(|p| p.tag.clone()).collect();
        tags.sort();
        tags.dedup();
        let mut items: Vec<(String, String)> = ashen_sim::data::ResourceKind::ALL.iter().map(|k| (data.item_id(ashen_sim::data::Item::Res(*k)).to_string(), data.item_name(ashen_sim::data::Item::Res(*k)).to_string())).collect();
        items.extend(data.goods.iter().map(|g| (g.id.clone(), g.name.clone())));
        Lists {
            areas: doc.def.areas.iter().map(|a| a.id.clone()).collect(),
            objectives: doc.def.objectives.iter().map(|o| o.id.clone()).collect(),
            tags,
            units: data.units.iter().map(|u| (u.id.clone(), u.name.clone())).collect(),
            buildings: data.buildings.iter().map(|b| (b.id.clone(), b.name.clone())).collect(),
            items,
        }
    }
}

const COND_KINDS: [&str; 27] = [
    "Zeit erreicht",
    "Gebäude fertig",
    "Gebäudeanzahl",
    "Einheitenzahl",
    "Soldatenzahl",
    "Ziel erledigt",
    "Nacht beginnt",
    "Tag beginnt",
    "Tag-Ziel zerstört",
    "Tag-Ziel entdeckt",
    "Heldenstufe",
    "Wellen überstanden",
    "Ausbaustufe",
    "Bewohnerzahl",
    "Vorrat",
    "Einheiten im Bereich",
    "Gegner höchstens",
    "Tag-Ziel beschädigt",
    "Nicht …",
    "Alle …",
    "Eine von …",
    "Flag gesetzt",
    "Markierte Einheit im Bereich",
    "Soldaten höchstens",
    "Markierte Einheit verwundet",
    "Einheiten einer Art im Bereich",
    "Dialog beendet",
];

fn cond_kind(c: &Condition) -> usize {
    match c {
        Condition::AtTime(_) => 0,
        Condition::BuildingCompleted(_) => 1,
        Condition::BuildingCount { .. } => 2,
        Condition::UnitCount { .. } => 3,
        Condition::MilitaryCount(_) => 4,
        Condition::ObjectiveCompleted(_) => 5,
        Condition::NightStarted(_) => 6,
        Condition::DayStarted(_) => 7,
        Condition::TagDead(_) => 8,
        Condition::TagDiscovered(_) => 9,
        Condition::HeroLevel(_) => 10,
        Condition::WavesSurvived(_) => 11,
        Condition::EraReached(_) => 12,
        Condition::Residents(_) => 13,
        Condition::Stock { .. } => 14,
        Condition::UnitsInArea { .. } => 15,
        Condition::EnemyUnitsAtMost(_) => 16,
        Condition::TagDamaged(_) => 17,
        Condition::Not(_) => 18,
        Condition::All(_) => 19,
        Condition::Any(_) => 20,
        Condition::Flag(_) => 21,
        Condition::TagInArea { .. } => 22,
        Condition::ArmyAtMost(_) => 23,
        Condition::TagHealthBelow { .. } => 24,
        Condition::UnitKindInArea { .. } => 25,
        Condition::DialogueDone => 26,
    }
}

fn first_or(v: &[String], fallback: &str) -> String {
    v.first().cloned().unwrap_or_else(|| fallback.to_string())
}

fn first_id(v: &[(String, String)], fallback: &str) -> String {
    v.first().map(|x| x.0.clone()).unwrap_or_else(|| fallback.to_string())
}

fn cond_default(kind: usize, l: &Lists) -> Condition {
    match kind {
        0 => Condition::AtTime(60.0),
        1 => Condition::BuildingCompleted(first_id(&l.buildings, "keep")),
        2 => Condition::BuildingCount { building: first_id(&l.buildings, "keep"), count: 2 },
        3 => Condition::UnitCount { unit: first_id(&l.units, "worker"), count: 5 },
        4 => Condition::MilitaryCount(8),
        5 => Condition::ObjectiveCompleted(first_or(&l.objectives, "")),
        6 => Condition::NightStarted(1),
        7 => Condition::DayStarted(2),
        8 => Condition::TagDead(first_or(&l.tags, "")),
        9 => Condition::TagDiscovered(first_or(&l.tags, "")),
        10 => Condition::HeroLevel(2),
        11 => Condition::WavesSurvived(3),
        12 => Condition::EraReached(1),
        13 => Condition::Residents(20),
        14 => Condition::Stock { item: "gold".into(), amount: 500.0 },
        15 => Condition::UnitsInArea { area: first_or(&l.areas, ""), count: 1 },
        16 => Condition::EnemyUnitsAtMost(0),
        17 => Condition::TagDamaged(first_or(&l.tags, "")),
        18 => Condition::Not(Box::new(Condition::AtTime(60.0))),
        19 => Condition::All(vec![Condition::AtTime(60.0)]),
        21 => Condition::Flag(String::new()),
        22 => Condition::TagInArea { tag: first_or(&l.tags, "hero"), area: first_or(&l.areas, "") },
        23 => Condition::ArmyAtMost(0),
        26 => Condition::DialogueDone,
        25 => Condition::UnitKindInArea { unit: first_id(&l.units, "worker"), area: first_or(&l.areas, ""), count: 5 },
        24 => Condition::TagHealthBelow { tag: first_or(&l.tags, "boss"), percent: 30.0 },
        _ => Condition::Any(vec![Condition::AtTime(60.0)]),
    }
}

const ACTION_KINDS: [&str; 16] = ["Nachricht", "Ziel hinzufügen", "Ziel abschließen", "Sieg", "Niederlage", "Welle auslösen", "Ressourcen schenken", "Wellen ein/aus", "Wetter setzen", "Chronik-Eintrag", "Einheiten erscheinen", "Flag setzen", "Dialogfolge", "Entscheidung", "Einheiten übergeben", "Einheiten losschicken"];

fn action_kind(a: &Action) -> usize {
    match a {
        Action::Message { .. } => 0,
        Action::AddObjective(_) => 1,
        Action::CompleteObjective(_) => 2,
        Action::Victory => 3,
        Action::Defeat => 4,
        Action::SpawnWave(_) => 5,
        Action::Grant(_) => 6,
        Action::EnableWaves(_) => 7,
        Action::SetWeather(_) => 8,
        Action::Chronicle(_) => 9,
        Action::SpawnUnits { .. } => 10,
        Action::SetFlag(_) => 11,
        Action::Dialogue(_) => 12,
        Action::Decision(_) => 13,
        Action::ChangeOwner { .. } => 14,
        Action::MoveTag { .. } => 15,
    }
}

fn action_default(kind: usize, l: &Lists) -> Action {
    match kind {
        0 => Action::Message { speaker: "Erzähler".into(), text: "…".into(), voice: None },
        1 => Action::AddObjective(first_or(&l.objectives, "")),
        2 => Action::CompleteObjective(first_or(&l.objectives, "")),
        3 => Action::Victory,
        4 => Action::Defeat,
        5 => Action::SpawnWave(6.0),
        6 => Action::Grant(Cost { gold: 100.0, wood: 100.0, stone: 0.0, food: 0.0 }),
        7 => Action::EnableWaves(true),
        8 => Action::SetWeather(Weather::Fog),
        9 => Action::Chronicle("Ein Ereignis wurde festgehalten.".into()),
        11 => Action::SetFlag(String::new()),
        12 => Action::Dialogue(vec![DialogueLine { speaker: "Erzähler".into(), text: "…".into(), voice: None }]),
        13 => Action::Decision(DecisionDef {
            id: "entscheidung".into(),
            speaker: "Erzähler".into(),
            title: "Eine Entscheidung".into(),
            text: "…".into(),
            voice: None,
            options: vec![DecisionOption { flag: "wahl_a".into(), label: "Option A".into(), detail: String::new(), requires: vec![], actions: vec![] }, DecisionOption { flag: "wahl_b".into(), label: "Option B".into(), detail: String::new(), requires: vec![], actions: vec![] }],
        }),
        14 => Action::ChangeOwner { tag: first_or(&l.tags, ""), owner: 0 },
        15 => Action::MoveTag { tag: first_or(&l.tags, ""), area: first_or(&l.areas, ""), attack: false },
        _ => Action::SpawnUnits { unit: first_id(&l.units, "hollow_thrall"), owner: 1, area: first_or(&l.areas, ""), count: 5, attack: true, tag: None },
    }
}

const WEATHERS: [Weather; 6] = [Weather::Clear, Weather::Overcast, Weather::Fog, Weather::Rain, Weather::Storm, Weather::Ashfall];

/// Auswahl aus einer Liste von Zeichenketten (mit Hinweis bei unbekanntem Wert).
fn pick(ui: &mut Ui, salt: impl std::hash::Hash + std::fmt::Debug, value: &mut String, options: &[String]) -> bool {
    let mut changed = false;
    let unknown = !options.contains(value);
    let text = if value.is_empty() {
        "(nichts gewählt)".to_string()
    } else if unknown {
        format!("{value} (unbekannt)")
    } else {
        value.clone()
    };
    egui::ComboBox::from_id_salt(salt).selected_text(RichText::new(text).color(if unknown { theme::EMBER } else { theme::PARCHMENT })).width(170.0).show_ui(ui, |ui| {
        for o in options {
            if ui.selectable_label(value == o, o).clicked() {
                *value = o.clone();
                changed = true;
            }
        }
        if options.is_empty() {
            ui.label(RichText::new("Noch nichts angelegt.").italics().color(theme::PARCHMENT_DIM));
        }
    });
    changed
}

/// Auswahl mit Anzeigename (ID, Name).
fn pick_named(ui: &mut Ui, salt: impl std::hash::Hash + std::fmt::Debug, value: &mut String, options: &[(String, String)]) -> bool {
    let mut changed = false;
    let known = options.iter().find(|o| &o.0 == value);
    let text = match known {
        Some(o) => format!("{} ({})", o.1, o.0),
        None if value.is_empty() => "(nichts gewählt)".to_string(),
        None => format!("{value} (unbekannt)"),
    };
    egui::ComboBox::from_id_salt(salt).selected_text(RichText::new(text).color(if known.is_none() { theme::EMBER } else { theme::PARCHMENT })).width(210.0).show_ui(ui, |ui| {
        egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
            for (id, name) in options {
                if ui.selectable_label(value == id, format!("{name} ({id})")).clicked() {
                    *value = id.clone();
                    changed = true;
                }
            }
        });
    });
    changed
}

fn num_u32(ui: &mut Ui, v: &mut u32, range: std::ops::RangeInclusive<u32>) -> bool {
    ui.add(egui::DragValue::new(v).range(range).speed(0.2)).changed()
}

/// Bedingungsbaum bearbeiten. Liefert `true`, wenn etwas geändert wurde.
pub fn condition_ui(ui: &mut Ui, salt: u64, c: &mut Condition, l: &Lists, depth: usize) -> bool {
    let mut changed = false;
    let mut kind = cond_kind(c);
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt((salt, "ck")).selected_text(RichText::new(COND_KINDS[kind]).color(theme::GOLD)).width(190.0).show_ui(ui, |ui| {
            for (i, k) in COND_KINDS.iter().enumerate() {
                if ui.selectable_label(kind == i, *k).clicked() && kind != i {
                    kind = i;
                    *c = cond_default(i, l);
                    changed = true;
                }
            }
        });
        match c {
            Condition::AtTime(t) => {
                changed |= ui.add(egui::DragValue::new(t).range(0.0..=36000.0).speed(1.0).suffix(" s")).changed();
            }
            Condition::BuildingCompleted(b) => changed |= pick_named(ui, (salt, "b"), b, &l.buildings),
            Condition::BuildingCount { building, count } => {
                changed |= pick_named(ui, (salt, "b"), building, &l.buildings);
                ui.label("mindestens");
                changed |= num_u32(ui, count, 1..=500);
            }
            Condition::UnitCount { unit, count } => {
                changed |= pick_named(ui, (salt, "u"), unit, &l.units);
                ui.label("mindestens");
                changed |= num_u32(ui, count, 1..=500);
            }
            Condition::MilitaryCount(n) | Condition::NightStarted(n) | Condition::DayStarted(n) | Condition::HeroLevel(n) | Condition::WavesSurvived(n) | Condition::Residents(n) | Condition::EnemyUnitsAtMost(n) => {
                changed |= num_u32(ui, n, 0..=10_000);
            }
            Condition::ObjectiveCompleted(o) => changed |= pick(ui, (salt, "o"), o, &l.objectives),
            Condition::TagDead(t) | Condition::TagDiscovered(t) | Condition::TagDamaged(t) => {
                changed |= pick(ui, (salt, "t"), t, &l.tags);
                changed |= ui.add(egui::TextEdit::singleline(t).desired_width(110.0).hint_text("Tag")).changed();
            }
            Condition::EraReached(e) => {
                let mut v = *e as u32;
                if num_u32(ui, &mut v, 0..=9) {
                    *e = v as u8;
                    changed = true;
                }
            }
            Condition::Stock { item, amount } => {
                changed |= pick_named(ui, (salt, "i"), item, &l.items);
                ui.label("mindestens");
                changed |= ui.add(egui::DragValue::new(amount).range(0.0..=100_000.0).speed(5.0)).changed();
            }
            Condition::UnitsInArea { area, count } => {
                changed |= pick(ui, (salt, "a"), area, &l.areas);
                ui.label("mindestens");
                changed |= num_u32(ui, count, 1..=500);
            }
            Condition::Flag(f) => changed |= ui.add(egui::TextEdit::singleline(f).desired_width(160.0).hint_text("Flag")).changed(),
            Condition::TagInArea { tag, area } => {
                changed |= pick(ui, (salt, "t"), tag, &l.tags);
                ui.label("in");
                changed |= pick(ui, (salt, "a"), area, &l.areas);
            }
            Condition::ArmyAtMost(n) => changed |= num_u32(ui, n, 0..=10_000),
            Condition::UnitKindInArea { unit, area, count } => {
                changed |= pick_named(ui, (salt, "u"), unit, &l.units);
                ui.label("in");
                changed |= pick(ui, (salt, "a"), area, &l.areas);
                changed |= num_u32(ui, count, 1..=500);
            }
            Condition::TagHealthBelow { tag, percent } => {
                changed |= pick(ui, (salt, "t"), tag, &l.tags);
                changed |= ui.add(egui::DragValue::new(percent).range(1.0..=99.0).suffix(" %")).changed();
            }
            Condition::DialogueDone | Condition::Not(_) | Condition::All(_) | Condition::Any(_) => {}
        }
    });
    // Verschachtelte Bedingungen.
    match c {
        Condition::Not(inner) if depth < 5 => {
            ui.indent((salt, "not"), |ui| changed |= condition_ui(ui, salt.wrapping_mul(31).wrapping_add(1), inner, l, depth + 1));
        }
        Condition::All(v) | Condition::Any(v) if depth < 5 => {
            let mut remove = None;
            ui.indent((salt, "list"), |ui| {
                for (i, sub) in v.iter_mut().enumerate() {
                    ui.horizontal_top(|ui| {
                        if ui.small_button("×").on_hover_text("Bedingung entfernen").clicked() {
                            remove = Some(i);
                        }
                        ui.vertical(|ui| changed |= condition_ui(ui, salt.wrapping_mul(31).wrapping_add(i as u64 + 2), sub, l, depth + 1));
                    });
                }
                if ui.small_button("+ Bedingung").clicked() {
                    v.push(Condition::AtTime(60.0));
                    changed = true;
                }
            });
            if let Some(i) = remove {
                v.remove(i);
                changed = true;
            }
        }
        _ => {}
    }
    changed
}

/// Aktion bearbeiten.
pub fn action_ui(ui: &mut Ui, salt: u64, a: &mut Action, l: &Lists) -> bool {
    let mut changed = false;
    let mut kind = action_kind(a);
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt((salt, "ak")).selected_text(RichText::new(ACTION_KINDS[kind]).color(theme::EMBER_HOT)).width(170.0).show_ui(ui, |ui| {
            for (i, k) in ACTION_KINDS.iter().enumerate() {
                if ui.selectable_label(kind == i, *k).clicked() && kind != i {
                    kind = i;
                    *a = action_default(i, l);
                    changed = true;
                }
            }
        });
        match a {
            Action::Message { speaker, .. } => {
                changed |= ui.add(egui::TextEdit::singleline(speaker).desired_width(130.0).hint_text("Sprecher")).changed();
            }
            Action::AddObjective(o) | Action::CompleteObjective(o) => changed |= pick(ui, (salt, "o"), o, &l.objectives),
            Action::SpawnWave(b) => {
                ui.label("Budget");
                changed |= ui.add(egui::DragValue::new(b).range(1.0..=200.0).speed(0.2)).changed();
            }
            Action::Grant(c) => {
                for (label, v) in [("Gold", &mut c.gold), ("Holz", &mut c.wood), ("Stein", &mut c.stone), ("Nahrung", &mut c.food)] {
                    ui.label(label);
                    changed |= ui.add(egui::DragValue::new(v).range(0.0..=100_000.0).speed(5.0)).changed();
                }
            }
            Action::EnableWaves(on) => changed |= ui.checkbox(on, "Wellen aktiv").changed(),
            Action::SetWeather(w) => {
                egui::ComboBox::from_id_salt((salt, "w")).selected_text(w.label()).show_ui(ui, |ui| {
                    for x in WEATHERS {
                        if ui.selectable_label(*w == x, x.label()).clicked() {
                            *w = x;
                            changed = true;
                        }
                    }
                });
            }
            Action::SpawnUnits { unit, owner, area, count, attack, .. } => {
                changed |= pick_named(ui, (salt, "u"), unit, &l.units);
                ui.label("×");
                changed |= num_u32(ui, count, 1..=200);
                ui.label("in");
                changed |= pick(ui, (salt, "a"), area, &l.areas);
                let mut side = *owner == 0;
                if ui.checkbox(&mut side, "für den Spieler").changed() {
                    *owner = if side { 0 } else { 1 };
                    changed = true;
                }
                changed |= ui.checkbox(attack, "greift an").changed();
            }
            Action::SetFlag(f) => changed |= ui.add(egui::TextEdit::singleline(f).desired_width(160.0).hint_text("Flag")).changed(),
            Action::ChangeOwner { tag, owner } => {
                changed |= pick(ui, (salt, "t"), tag, &l.tags);
                let mut side = *owner == 0;
                if ui.checkbox(&mut side, "an den Spieler").changed() {
                    *owner = if side { 0 } else { 1 };
                    changed = true;
                }
            }
            Action::MoveTag { tag, area, attack } => {
                changed |= pick(ui, (salt, "t"), tag, &l.tags);
                ui.label("nach");
                changed |= pick(ui, (salt, "a"), area, &l.areas);
                changed |= ui.checkbox(attack, "kämpfend").changed();
            }
            Action::Decision(d) => {
                changed |= ui.add(egui::TextEdit::singleline(&mut d.id).desired_width(120.0).hint_text("ID")).changed();
                changed |= ui.add(egui::TextEdit::singleline(&mut d.speaker).desired_width(120.0).hint_text("Sprecher")).changed();
            }
            Action::Victory | Action::Defeat | Action::Chronicle(_) | Action::Dialogue(_) => {}
        }
    });
    match a {
        Action::Dialogue(lines) => {
            let mut remove = None;
            for (i, line) in lines.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= ui.add(egui::TextEdit::singleline(&mut line.speaker).desired_width(120.0).hint_text("Sprecher")).changed();
                    changed |= ui.add(egui::TextEdit::singleline(&mut line.text).desired_width(f32::INFINITY).hint_text("Text")).changed();
                    if ui.small_button("×").clicked() {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                lines.remove(i);
                changed = true;
            }
            if ui.small_button("+ Zeile").clicked() {
                lines.push(DialogueLine { speaker: "Erzähler".into(), text: "…".into(), voice: None });
                changed = true;
            }
        }
        Action::Decision(d) => {
            changed |= ui.add(egui::TextEdit::singleline(&mut d.title).desired_width(f32::INFINITY).hint_text("Titel")).changed();
            changed |= ui.add(egui::TextEdit::multiline(&mut d.text).desired_rows(2).desired_width(f32::INFINITY).hint_text("Frage")).changed();
            for o in d.options.iter_mut() {
                ui.horizontal(|ui| {
                    changed |= ui.add(egui::TextEdit::singleline(&mut o.flag).desired_width(120.0).hint_text("Flag")).changed();
                    changed |= ui.add(egui::TextEdit::singleline(&mut o.label).desired_width(f32::INFINITY).hint_text("Antwort")).changed();
                });
            }
        }
        _ => {}
    }
    match a {
        Action::Message { text, .. } | Action::Chronicle(text) => {
            changed |= ui.add(egui::TextEdit::multiline(text).desired_rows(2).desired_width(f32::INFINITY).hint_text("Text")).changed();
        }
        _ => {}
    }
    changed
}

/// Kurzbeschreibung einer Bedingung für Kopfzeilen.
pub fn describe_condition(c: &Condition) -> String {
    match c {
        Condition::AtTime(t) => format!("Zeit ≥ {}:{:02}", *t as u32 / 60, *t as u32 % 60),
        Condition::BuildingCompleted(b) => format!("{b} fertig"),
        Condition::BuildingCount { building, count } => format!("{count}× {building}"),
        Condition::UnitCount { unit, count } => format!("{count}× {unit}"),
        Condition::MilitaryCount(n) => format!("{n} Soldaten"),
        Condition::ObjectiveCompleted(o) => format!("Ziel {o} erledigt"),
        Condition::NightStarted(n) => format!("Nacht {n}"),
        Condition::DayStarted(n) => format!("Tag {n}"),
        Condition::TagDead(t) => format!("{t} zerstört"),
        Condition::TagDiscovered(t) => format!("{t} entdeckt"),
        Condition::HeroLevel(n) => format!("Heldenstufe {n}"),
        Condition::WavesSurvived(n) => format!("{n} Wellen überstanden"),
        Condition::EraReached(n) => format!("Ausbaustufe {n}"),
        Condition::Residents(n) => format!("{n} Bewohner"),
        Condition::Stock { item, amount } => format!("{amount:.0} {item}"),
        Condition::UnitsInArea { area, count } => format!("{count} Einheiten in {area}"),
        Condition::EnemyUnitsAtMost(n) => format!("höchstens {n} Gegner"),
        Condition::TagDamaged(t) => format!("{t} beschädigt"),
        Condition::Not(c) => format!("nicht ({})", describe_condition(c)),
        Condition::All(v) => v.iter().map(describe_condition).collect::<Vec<_>>().join(" UND "),
        Condition::Any(v) => format!("({})", v.iter().map(describe_condition).collect::<Vec<_>>().join(" ODER ")),
        Condition::Flag(f) => format!("Flag {f}"),
        Condition::TagInArea { tag, area } => format!("{tag} in {area}"),
        Condition::ArmyAtMost(n) => format!("höchstens {n} Soldaten"),
        Condition::TagHealthBelow { tag, percent } => format!("{tag} unter {percent:.0} % Leben"),
        Condition::UnitKindInArea { unit, area, count } => format!("{count}× {unit} in {area}"),
        Condition::DialogueDone => "Dialog beendet".into(),
    }
}

pub fn describe_action(a: &Action) -> String {
    match a {
        Action::Message { speaker, .. } => format!("Nachricht ({})", if speaker.is_empty() { "–" } else { speaker }),
        Action::AddObjective(o) => format!("Ziel neu: {o}"),
        Action::CompleteObjective(o) => format!("Ziel erledigt: {o}"),
        Action::Victory => "SIEG".into(),
        Action::Defeat => "NIEDERLAGE".into(),
        Action::SpawnWave(b) => format!("Welle {b:.0}"),
        Action::Grant(_) => "Ressourcen".into(),
        Action::EnableWaves(on) => format!("Wellen {}", if *on { "an" } else { "aus" }),
        Action::SetWeather(w) => format!("Wetter {}", w.label()),
        Action::Chronicle(_) => "Chronik".into(),
        Action::SpawnUnits { unit, count, area, .. } => format!("{count}× {unit} in {area}"),
        Action::SetFlag(f) => format!("Flag {f}"),
        Action::Dialogue(l) => format!("Dialogfolge ({} Zeilen)", l.len()),
        Action::Decision(d) => format!("Entscheidung {}", d.id),
        Action::ChangeOwner { tag, owner } => format!("{tag} → Besitzer {owner}"),
        Action::MoveTag { tag, area, .. } => format!("{tag} nach {area}"),
    }
}

/// Zustand der Trigger-Oberfläche.
#[derive(Default)]
pub struct TriggerUi {
    pub selected: Option<usize>,
}

/// Registerkarte „Auslöser“: Liste links, Bearbeitung rechts.
pub fn triggers_tab(ui: &mut Ui, doc: &mut EditDoc, l: &Lists, st: &mut TriggerUi) -> bool {
    let mut changed = false;
    let n = doc.def.triggers.len();
    if st.selected.is_none() && n > 0 {
        st.selected = Some(0);
    }
    if st.selected.is_some_and(|s| s >= n) {
        st.selected = n.checked_sub(1);
    }
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(260.0);
            ui.label(RichText::new("Auslöser").font(theme::heading_font(15.0)).color(theme::GOLD));
            egui::ScrollArea::vertical().id_salt("trg_list").max_height(430.0).show(ui, |ui| {
                for (i, t) in doc.def.triggers.iter().enumerate() {
                    let sel = st.selected == Some(i);
                    let victory = t.actions.iter().any(|a| matches!(a, Action::Victory));
                    let defeat = t.actions.iter().any(|a| matches!(a, Action::Defeat));
                    let mark = if victory { " (Sieg)" } else if defeat { " (Niederlage)" } else { "" };
                    let acts = t.actions.iter().map(describe_action).collect::<Vec<_>>().join(", ");
                    let text = format!("{}{}\nWENN {}\nDANN {}", t.id, mark, describe_condition(&t.condition), acts);
                    if ui.selectable_label(sel, RichText::new(text).size(13.5)).clicked() {
                        st.selected = Some(i);
                    }
                }
            });
            ui.horizontal(|ui| {
                if ui.button("+ Auslöser").clicked() {
                    let ids: Vec<&str> = doc.def.triggers.iter().map(|t| t.id.as_str()).collect();
                    let id = EditDoc::fresh_id(&ids, "trigger");
                    doc.def.triggers.push(TriggerDef { id, condition: Condition::AtTime(30.0), actions: vec![Action::Message { speaker: "Erzähler".into(), text: "…".into(), voice: None }], modes: vec![] });
                    st.selected = Some(doc.def.triggers.len() - 1);
                    changed = true;
                }
                if let Some(s) = st.selected {
                    if ui.button("Kopie").clicked() {
                        let mut t = doc.def.triggers[s].clone();
                        let ids: Vec<&str> = doc.def.triggers.iter().map(|t| t.id.as_str()).collect();
                        t.id = EditDoc::fresh_id(&ids, &format!("{}_kopie", t.id));
                        doc.def.triggers.insert(s + 1, t);
                        st.selected = Some(s + 1);
                        changed = true;
                    }
                    if ui.small_button("Auf").clicked() && s > 0 {
                        doc.def.triggers.swap(s, s - 1);
                        st.selected = Some(s - 1);
                        changed = true;
                    }
                    if ui.small_button("Ab").clicked() && s + 1 < doc.def.triggers.len() {
                        doc.def.triggers.swap(s, s + 1);
                        st.selected = Some(s + 1);
                        changed = true;
                    }
                    if ui.button("Löschen").clicked() {
                        doc.def.triggers.remove(s);
                        st.selected = if doc.def.triggers.is_empty() { None } else { Some(s.min(doc.def.triggers.len() - 1)) };
                        changed = true;
                    }
                }
            });
        });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            let Some(s) = st.selected else {
                ui.label(RichText::new("Wählt links einen Auslöser oder legt einen neuen an.").font(theme::italic_font(15.0)).color(theme::PARCHMENT_DIM));
                return;
            };
            egui::ScrollArea::vertical().id_salt("trg_edit").max_height(470.0).show(ui, |ui| {
                let t = &mut doc.def.triggers[s];
                ui.horizontal(|ui| {
                    ui.label("ID");
                    changed |= ui.add(egui::TextEdit::singleline(&mut t.id).desired_width(150.0)).changed();
                    ui.label("Gilt in:");
                    for m in [GameMode::Campaign, GameMode::Skirmish, GameMode::Survival, GameMode::Sandbox] {
                        let mut on = t.modes.contains(&m);
                        if ui.checkbox(&mut on, m.label()).changed() {
                            if on {
                                t.modes.push(m);
                            } else {
                                t.modes.retain(|x| *x != m);
                            }
                            changed = true;
                        }
                    }
                });
                ui.label(RichText::new("Keine Modus-Auswahl bedeutet: gilt in allen Modi.").font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
                ui.add_space(6.0);
                ui.label(RichText::new("WENN").font(theme::caps_font(14.0)).color(Color32::from_rgb(214, 190, 146)));
                changed |= condition_ui(ui, (s as u64 + 1) * 1000, &mut t.condition, l, 0);
                ui.add_space(8.0);
                ui.label(RichText::new("DANN").font(theme::caps_font(14.0)).color(Color32::from_rgb(214, 190, 146)));
                let mut remove = None;
                let mut swap = None;
                let na = t.actions.len();
                for (i, a) in t.actions.iter_mut().enumerate() {
                    ui.horizontal_top(|ui| {
                        ui.vertical(|ui| {
                            if ui.small_button("×").on_hover_text("Aktion entfernen").clicked() {
                                remove = Some(i);
                            }
                            if i > 0 && ui.small_button("Auf").clicked() {
                                swap = Some((i, i - 1));
                            }
                            if i + 1 < na && ui.small_button("Ab").clicked() {
                                swap = Some((i, i + 1));
                            }
                        });
                        ui.vertical(|ui| changed |= action_ui(ui, (s as u64 + 1) * 1000 + 500 + i as u64, a, l));
                    });
                    ui.separator();
                }
                if let Some(i) = remove {
                    t.actions.remove(i);
                    changed = true;
                }
                if let Some((a, b)) = swap {
                    t.actions.swap(a, b);
                    changed = true;
                }
                if ui.button("+ Aktion").clicked() {
                    t.actions.push(action_default(0, l));
                    changed = true;
                }
            });
        });
    });
    changed
}

/// Registerkarte „Ziele“.
pub fn objectives_tab(ui: &mut Ui, doc: &mut EditDoc) -> bool {
    let mut changed = false;
    let mut remove = None;
    ui.label(RichText::new("Ziele erscheinen in der Zielliste. Auslöser fügen sie hinzu (Aktion „Ziel hinzufügen“) und schließen sie ab.").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
    ui.add_space(4.0);
    egui::ScrollArea::vertical().id_salt("obj_list").max_height(420.0).show(ui, |ui| {
        for (i, o) in doc.def.objectives.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui.small_button("×").clicked() {
                    remove = Some(i);
                }
                changed |= ui.add(egui::TextEdit::singleline(&mut o.id).desired_width(130.0)).changed();
                changed |= ui.add(egui::TextEdit::singleline(&mut o.text).desired_width(430.0)).changed();
                changed |= ui.checkbox(&mut o.optional, "optional").changed();
            });
        }
    });
    if let Some(i) = remove {
        doc.def.objectives.remove(i);
        changed = true;
    }
    if ui.button("+ Ziel").clicked() {
        let ids: Vec<&str> = doc.def.objectives.iter().map(|o| o.id.as_str()).collect();
        let id = EditDoc::fresh_id(&ids, "obj");
        doc.def.objectives.push(ObjectiveDef { id, text: "Neues Ziel.".into(), optional: false });
        changed = true;
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_condition_and_action_kind_roundtrips() {
        let l = Lists { areas: vec!["a".into()], objectives: vec!["o".into()], tags: vec!["t".into()], units: vec![("worker".into(), "Arbeiter".into())], buildings: vec![("keep".into(), "Bergfried".into())], items: vec![] };
        for k in 0..COND_KINDS.len() {
            let c = cond_default(k, &l);
            assert_eq!(cond_kind(&c), k, "Bedingung {k}");
            assert!(!describe_condition(&c).is_empty());
        }
        for k in 0..ACTION_KINDS.len() {
            let a = action_default(k, &l);
            assert_eq!(action_kind(&a), k, "Aktion {k}");
            assert!(!describe_action(&a).is_empty());
        }
    }
}
