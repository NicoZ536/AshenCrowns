//! Spielmodi neben der Kampagne: Szenarien, Herausforderungen und die Gefecht-Einrichtung.
//!
//! Dieses Modul enthält die Daten- und Bauhelfer (Gefechtskarte, Wertung, Bestenliste) sowie die
//! Menüseiten, die das Hauptmenü einhängt.

use crate::content::Content;
use crate::editor::doc::EditDoc;
use crate::editor::EditorInit;
use crate::menu::{MenuAction, MenuScene, Page};
use crate::settings::{leaderboard_path, UserSettings};
use crate::ui_art::{self, BtnState, PanelKind, UiArt};
use crate::ui_common;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::challenge::{self, ChallengeDef, ChallengeResult, Leaderboard, LeaderboardEntry, RankBy};
use ashen_sim::data::Cost;
use ashen_sim::genmap::{generate_preset, GenPreset};
use ashen_sim::map::Terrain;
use ashen_sim::mission::{Action, Condition, Difficulty, GameMode, MatchSettings, MissionDef, TriggerDef};
use ashen_sim::scenario::{MapData, ScenarioEntry};
use egui::{Align2, Color32, RichText, Sense, Vec2};

// ---------------------------------------------------------------------------------------------
// Gefecht-Einrichtung: Daten
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapSource {
    Generated,
    Scenario,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WinKind {
    /// Gegnerische Basis zerstören (klassisches Gefecht).
    DestroyBase,
    /// Eine bestimmte Zahl Nächte überleben.
    Survive,
    /// Basis zerstören und alle Gegner besiegen.
    Annihilate,
}

impl WinKind {
    pub const ALL: [WinKind; 3] = [WinKind::DestroyBase, WinKind::Survive, WinKind::Annihilate];

    pub fn label(self) -> &'static str {
        match self {
            WinKind::DestroyBase => "Basis zerstören",
            WinKind::Survive => "Überleben",
            WinKind::Annihilate => "Vernichtung",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            WinKind::DestroyBase => "Zerstört den Hohlschrein des Gegners. Die Wellen kommen schneller als in der Kampagne.",
            WinKind::Survive => "Der Schrein ist unzerstörbar. Haltet die gewählte Zahl Nächte durch.",
            WinKind::Annihilate => "Zerstört den Schrein und besiegt alle Hohlen auf der Karte.",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wealth {
    Poor,
    Normal,
    Rich,
    Lavish,
}

impl Wealth {
    pub const ALL: [Wealth; 4] = [Wealth::Poor, Wealth::Normal, Wealth::Rich, Wealth::Lavish];

    pub fn label(self) -> &'static str {
        match self {
            Wealth::Poor => "Karg",
            Wealth::Normal => "Normal",
            Wealth::Rich => "Reich",
            Wealth::Lavish => "Üppig",
        }
    }

    pub fn factor(self) -> f32 {
        match self {
            Wealth::Poor => 0.5,
            Wealth::Normal => 1.0,
            Wealth::Rich => 2.0,
            Wealth::Lavish => 4.0,
        }
    }
}

pub const SIZES: [(&str, i32); 4] = [("Klein 48", 48), ("Mittel 72", 72), ("Groß 96", 96), ("Riesig 128", 128)];

/// Einstellungen des Gefecht-Bildschirms.
#[derive(Clone, Debug)]
pub struct SkirmishForm {
    pub source: MapSource,
    pub preset: GenPreset,
    pub size_idx: usize,
    pub seed: u64,
    pub scenario: usize,
    pub difficulty: Difficulty,
    pub wealth: Wealth,
    pub win: WinKind,
    pub nights: u32,
}

impl Default for SkirmishForm {
    fn default() -> Self {
        SkirmishForm { source: MapSource::Generated, preset: GenPreset::Aschmark, size_idx: 1, seed: 897, scenario: 0, difficulty: Difficulty::Normal, wealth: Wealth::Normal, win: WinKind::DestroyBase, nights: 5 }
    }
}

fn scale_cost(c: Cost, f: f32) -> Cost {
    c.scaled(f)
}

/// Baut aus der Einrichtung eine spielbare Mission samt Startoptionen. `scenarios` sind alle
/// wählbaren Szenarien (mitgelieferte und eigene), `hero_permadeath` kommt aus den Einstellungen.
/// Die Schwierigkeit wird unverändert als Datum in [`MatchSettings`] durchgereicht; die Gegner-KI
/// des Gefechtsmodus liest sie von dort.
pub fn build_skirmish(form: &SkirmishForm, scenarios: &[&ScenarioEntry], hero_permadeath: bool) -> Result<(MissionDef, MatchSettings), String> {
    let (mut mission, base_seed) = match form.source {
        MapSource::Generated => {
            let size = SIZES[form.size_idx.min(SIZES.len() - 1)].1;
            let doc = EditDoc::generated(size, size, form.seed, form.preset);
            let mut m = doc.to_mission();
            m.id = format!("gefecht_{}_{}_{}", ashen_sim::scenario::slug(form.preset.label()), size, form.seed);
            m.name = format!("Gefecht – {}", form.preset.label());
            m.description = format!("Gefecht auf einer generierten Karte ({} {size}×{size}, Seed {}).", form.preset.label(), form.seed);
            (m, form.seed)
        }
        MapSource::Scenario => {
            let s = scenarios.get(form.scenario).ok_or("Kein Szenario gewählt.")?;
            (s.def.clone(), s.def.map_seed)
        }
    };
    mission.start_resources = scale_cost(mission.start_resources, form.wealth.factor());
    for g in &mut mission.start_goods {
        g.1 *= form.wealth.factor();
    }
    let mode = match (form.source, form.win) {
        (MapSource::Scenario, _) => GameMode::Campaign,
        (_, WinKind::DestroyBase) => GameMode::Skirmish,
        (_, WinKind::Survive) => GameMode::Survival,
        (_, WinKind::Annihilate) => GameMode::Campaign,
    };
    if form.source == MapSource::Generated {
        // Siegbedingung der generierten Karte neu setzen.
        mission.triggers.retain(|t| t.id != "sieg");
        let base = mission.enemy_base_tag.clone();
        let (cond, text) = match form.win {
            WinKind::DestroyBase => (Condition::TagDead(base.clone()), "Zerstört den Hohlschrein.".to_string()),
            WinKind::Survive => (Condition::DayStarted(form.nights + 1), format!("Überlebt {} Nächte.", form.nights)),
            WinKind::Annihilate => (Condition::All(vec![Condition::TagDead(base.clone()), Condition::EnemyUnitsAtMost(0)]), "Zerstört den Schrein und besiegt alle Hohlen.".to_string()),
        };
        mission.objectives.iter_mut().for_each(|o| o.text = text.clone());
        mission.triggers.push(TriggerDef {
            id: "sieg".into(),
            condition: cond,
            actions: vec![Action::CompleteObjective("obj_shrine".into()), Action::Message { speaker: "Erzähler".into(), text: "Das Gefecht ist gewonnen.".into(), voice: None }, Action::Victory],
            modes: vec![],
        });
        // Das Einführungsziel kommt im Gefecht schon bei Spielbeginn.
        if let Some(t) = mission.triggers.iter_mut().find(|t| t.id == "intro") {
            t.condition = Condition::AtTime(1.0);
        }
    }
    let settings = MatchSettings { mode, difficulty: form.difficulty, seed: base_seed, hero_permadeath };
    Ok((mission, settings))
}

// ---------------------------------------------------------------------------------------------
// Herausforderungen: Auswertung und Bestenliste
// ---------------------------------------------------------------------------------------------

/// Ergebnisanzeige nach einer Herausforderung.
#[derive(Clone, Debug)]
pub struct ChallengeReport {
    pub name: String,
    pub result: ChallengeResult,
    pub rank: Option<usize>,
    pub rank_by: RankBy,
    pub top: Vec<LeaderboardEntry>,
    pub new_best: bool,
}

pub fn unix_now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Wertet die Partie aus und trägt das Ergebnis in die Bestenliste unter `path` ein.
pub fn record_result(def: &ChallengeDef, result: &ChallengeResult, name: &str, path: &std::path::Path, now: u64) -> ChallengeReport {
    let mut lb = Leaderboard::load(path);
    let before_best = lb.best(def).map(|e| (e.score, e.time));
    let rank = lb.submit(def, name, result, now);
    if let Err(e) = lb.save(path) {
        log::warn!("Bestenliste nicht gespeichert: {e}");
    }
    let new_best = rank == Some(0) && before_best.is_none_or(|(s, t)| if def.scoring.rank_by == RankBy::Time { result.time < t } else { result.score > s });
    ChallengeReport { name: def.name.clone(), result: result.clone(), rank, rank_by: def.scoring.rank_by, top: lb.top(def, 5).into_iter().cloned().collect(), new_best }
}

/// Wertet eine beendete Partie aus; `None`, solange noch kein Ergebnis feststeht.
pub fn finish_challenge(def: &ChallengeDef, world: &ashen_sim::SimWorld) -> Option<ChallengeReport> {
    let result = challenge::evaluate(def, world)?;
    Some(record_result(def, &result, "Wächterin", &leaderboard_path(), unix_now()))
}

/// Datum `JJJJ-MM-TT` aus Unix-Sekunden (bürgerlicher Kalender, UTC).
pub fn format_date(secs: u64) -> String {
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    format!("{:04}-{:02}-{:02}", if m <= 2 { y + 1 } else { y }, m, d)
}

/// Eine Zeile der Bestenliste als Text.
pub fn entry_text(rank_by: RankBy, e: &LeaderboardEntry) -> String {
    let time = challenge::format_time(e.time);
    match rank_by {
        RankBy::Score => format!("{} Punkte · {time} · {}", e.score, format_date(e.date)),
        RankBy::Time => format!("{time} · {} Punkte · {}", e.score, format_date(e.date)),
    }
}

// ---------------------------------------------------------------------------------------------
// Menüzustand der drei Seiten
// ---------------------------------------------------------------------------------------------

type ThumbKey = (GenPreset, i32, u64);

pub struct ModesState {
    pub scen_sel: usize,
    pub scen_user: Vec<ScenarioEntry>,
    pub scen_errors: Vec<String>,
    pub difficulty: Difficulty,
    pub chal_sel: usize,
    pub leaderboard: Leaderboard,
    pub skirmish: SkirmishForm,
    pub skirmish_error: Option<String>,
    thumb: Option<(ThumbKey, egui::TextureHandle)>,
}

impl ModesState {
    pub fn new() -> ModesState {
        ModesState { scen_sel: 0, scen_user: Vec::new(), scen_errors: Vec::new(), difficulty: Difficulty::Normal, chal_sel: 0, leaderboard: Leaderboard::default(), skirmish: SkirmishForm::default(), skirmish_error: None, thumb: None }
    }

    pub fn refresh_scenarios(&mut self, content: &Content) {
        self.scen_errors.clear();
        self.scen_user = content.user_scenarios(&mut self.scen_errors);
        let n = content.scenarios.len() + self.scen_user.len();
        self.scen_sel = self.scen_sel.min(n.saturating_sub(1));
        self.skirmish.scenario = self.skirmish.scenario.min(n.saturating_sub(1));
    }

    pub fn refresh_challenges(&mut self) {
        self.leaderboard = Leaderboard::load(&leaderboard_path());
    }
}

/// Alle wählbaren Szenarien: mitgelieferte, Mods, dann eigene.
fn all_scenarios<'a>(content: &'a Content, user: &'a [ScenarioEntry]) -> Vec<&'a ScenarioEntry> {
    content.scenarios.iter().chain(user.iter()).collect()
}

fn terrain_rgb(t: Terrain) -> [u8; 3] {
    match t {
        Terrain::Grass => [96, 140, 64],
        Terrain::DarkGrass => [58, 100, 56],
        Terrain::Dirt => [130, 100, 66],
        Terrain::Road => [158, 140, 108],
        Terrain::Stone => [122, 122, 130],
        Terrain::Sand => [204, 186, 132],
        Terrain::Ash => [92, 88, 86],
        Terrain::Ford => [112, 156, 164],
        Terrain::Water => [58, 110, 172],
        Terrain::DeepWater => [28, 58, 118],
    }
}

/// Draufsicht der Karte als kleines Bild (für die Gefechtswahl).
pub fn thumbnail(map: &MapData, starts: &[glam::Vec2]) -> egui::ColorImage {
    let scale = (360 / map.width.max(map.height).max(1)).clamp(2, 6) as usize;
    let (w, h) = (map.width as usize * scale, map.height as usize * scale);
    let mut img = egui::ColorImage::filled([w, h], Color32::BLACK);
    let terrain = map.terrain_vec();
    for y in 0..map.height as usize {
        for x in 0..map.width as usize {
            let c = terrain_rgb(terrain[y * map.width as usize + x]);
            for dy in 0..scale {
                for dx in 0..scale {
                    img.pixels[(y * scale + dy) * w + x * scale + dx] = Color32::from_rgb(c[0], c[1], c[2]);
                }
            }
        }
    }
    let mut dot = |x: i32, y: i32, size: i32, col: Color32| {
        for dy in 0..size * scale as i32 {
            for dx in 0..size * scale as i32 {
                let (px, py) = (x * scale as i32 + dx, y * scale as i32 + dy);
                if px >= 0 && py >= 0 && (px as usize) < w && (py as usize) < h {
                    img.pixels[py as usize * w + px as usize] = col;
                }
            }
        }
    };
    for n in &map.nodes {
        match n.kind {
            ashen_sim::entity::NodeKind::Tree => dot(n.x, n.y, 1, Color32::from_rgb(34, 74, 36)),
            ashen_sim::entity::NodeKind::GoldMine => dot(n.x, n.y, 2, Color32::from_rgb(250, 210, 70)),
            ashen_sim::entity::NodeKind::StoneQuarry => dot(n.x, n.y, 2, Color32::from_rgb(210, 210, 220)),
        }
    }
    for (i, s) in starts.iter().enumerate() {
        dot(s.x as i32 - 2, s.y as i32 - 2, 4, if i == 0 { Color32::from_rgb(90, 170, 255) } else { Color32::from_rgb(235, 90, 76) });
    }
    img
}

// ---------------------------------------------------------------------------------------------
// Gemeinsame UI-Bausteine
// ---------------------------------------------------------------------------------------------

fn label(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).font(theme::caps_font(13.5)).color(Color32::from_rgb(214, 190, 146)));
}

fn list_row(ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, title: &str, sub: &str, selected: bool, width: f32) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 54.0), Sense::click());
    let mut st = BtnState::of(&resp, true);
    if selected && st == BtnState::Normal {
        st = BtnState::Pressed;
    }
    art.button_nine(st).paint(ui.painter(), egui::emath::GuiRounding::round_ui(rect), Color32::WHITE);
    let col = if selected || st == BtnState::Hover { theme::GOLD_BRIGHT } else { theme::GOLD };
    ui.painter().text(rect.left_top() + Vec2::new(20.0, 9.0), Align2::LEFT_TOP, title, theme::heading_font(15.0), col);
    ui.painter().text(rect.left_bottom() + Vec2::new(20.0, -8.0), Align2::LEFT_BOTTOM, sub, theme::italic_font(13.5), theme::PARCHMENT_DIM);
    ui_common::sfx(engine, ui, &resp);
    resp.clicked()
}

fn difficulty_row(ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, current: &mut Difficulty) {
    ui.horizontal(|ui| {
        for d in [Difficulty::Story, Difficulty::Normal, Difficulty::Hard] {
            let r = ui_art::choice(ui, art, d.label(), *current == d);
            ui_common::sfx(engine, ui, &r);
            if r.clicked() {
                *current = d;
            }
        }
    });
}

fn parchment<R>(ui: &mut egui::Ui, art: &UiArt, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    art.panel(ui, PanelKind::Parchment, egui::Margin { left: 26, right: 26, top: 20, bottom: 20 }, |ui| {
        ui.set_width(ui.available_width());
        add(ui)
    })
    .inner
}

fn ink_heading(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).font(theme::title_bold_font(18.0)).color(theme::INK_RED));
}

fn ink_caps(ui: &mut egui::Ui, t: &str) {
    ui.add_space(6.0);
    ui.label(RichText::new(t).font(theme::caps_font(13.0)).color(theme::INK_RED));
}

fn ink_text(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).size(15.5).color(theme::INK));
}

// ---------------------------------------------------------------------------------------------
// Seite „Szenarien“
// ---------------------------------------------------------------------------------------------

pub fn scenarios_page(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, settings: &UserSettings, art: &UiArt) {
    let user = std::mem::take(&mut menu.modes.scen_user);
    scenarios_inner(menu, ui, engine, content, &user, settings, art);
    menu.modes.scen_user = user;
}

fn scenarios_inner(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, user: &[ScenarioEntry], settings: &UserSettings, art: &UiArt) {
    ui.set_width(920.0);
    ui_art::heading(ui, art, "Szenarien", 26.0);
    let all = all_scenarios(content, user);
    if all.is_empty() {
        ui.label(RichText::new("Keine Szenarien gefunden.").font(theme::italic_font(16.0)).color(theme::PARCHMENT_DIM));
    }
    let sel = menu.modes.scen_sel.min(all.len().saturating_sub(1));
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(340.0);
            egui::ScrollArea::vertical().id_salt("scen_list").max_height(430.0).show(ui, |ui| {
                for (i, s) in all.iter().enumerate() {
                    let year = s.def.year.map(|y| format!("{y} n.F. · ")).unwrap_or_default();
                    if list_row(ui, engine, art, &s.def.name, &format!("{year}{}", s.source.label()), i == sel, 322.0) {
                        menu.modes.scen_sel = i;
                    }
                    ui.add_space(2.0);
                }
            });
        });
        ui.add_space(8.0);
        ui.vertical(|ui| {
            let Some(s) = all.get(sel) else { return };
            let d = &s.def;
            parchment(ui, art, |ui| {
                ink_heading(ui, &d.name);
                let (w, h) = d.map.as_ref().map(|m| (m.width, m.height)).unwrap_or((72, 72));
                let year = d.year.map(|y| format!("Jahr {y} n.F. · ")).unwrap_or_default();
                let author = if d.author.is_empty() { String::new() } else { format!(" · von {}", d.author) };
                ui.label(RichText::new(format!("{year}Karte {w}×{h} · {}{author}", s.source.label())).font(theme::italic_font(14.0)).color(theme::INK_FADED));
                ui.add_space(4.0);
                egui::ScrollArea::vertical().id_salt("scen_detail").max_height(250.0).show(ui, |ui| {
                    ink_text(ui, &d.description);
                    if !d.briefing.is_empty() {
                        ink_caps(ui, "Einsatzbesprechung");
                        ink_text(ui, &d.briefing);
                    }
                    let goals: Vec<_> = d.objectives.iter().filter(|o| !o.optional).collect();
                    if !goals.is_empty() {
                        ink_caps(ui, "Ziele");
                        for o in goals {
                            ink_text(ui, &format!("• {}", o.text));
                        }
                    }
                    let opt: Vec<_> = d.objectives.iter().filter(|o| o.optional).collect();
                    if !opt.is_empty() {
                        ink_caps(ui, "Nebenziele");
                        for o in opt {
                            ink_text(ui, &format!("- {}", o.text));
                        }
                    }
                });
            });
            ui.add_space(8.0);
            label(ui, "Schwierigkeit");
            difficulty_row(ui, engine, art, &mut menu.modes.difficulty);
        });
    });
    for e in &menu.modes.scen_errors {
        ui.label(RichText::new(e).color(theme::EMBER).small());
    }
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 700.0).max(0.0) / 2.0);
        if let Some(s) = all.get(sel) {
            if ui_common::button(ui, engine, art, "Starten", 220.0) {
                let settings = MatchSettings { mode: GameMode::Campaign, difficulty: menu.modes.difficulty, seed: s.def.map_seed, hero_permadeath: settings.hero_permadeath };
                menu.action = MenuAction::StartDef { mission: Box::new(s.def.clone()), settings, challenge: None };
            }
            if ui_common::button(ui, engine, art, "Im Editor öffnen", 250.0) {
                menu.action = MenuAction::OpenEditor(EditorInit::Load(s.path.clone()));
            }
        }
        if ui_common::button(ui, engine, art, "Zurück", 200.0) {
            menu.page = Page::Main;
        }
    });
}

// ---------------------------------------------------------------------------------------------
// Seite „Herausforderungen“
// ---------------------------------------------------------------------------------------------

pub fn challenges_page(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, settings: &UserSettings, art: &UiArt) {
    ui.set_width(960.0);
    ui_art::heading(ui, art, "Herausforderungen", 26.0);
    let list = &content.challenges;
    let sel = menu.modes.chal_sel.min(list.len().saturating_sub(1));
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(330.0);
            egui::ScrollArea::vertical().id_salt("chal_list").max_height(470.0).show(ui, |ui| {
                for (i, c) in list.iter().enumerate() {
                    let best = menu.modes.leaderboard.best(c).map(|e| match c.scoring.rank_by {
                        RankBy::Score => format!("Beste: {} Punkte", e.score),
                        RankBy::Time => format!("Bestzeit: {}", challenge::format_time(e.time)),
                    });
                    if list_row(ui, engine, art, &c.name, &best.unwrap_or_else(|| "Noch nicht bestanden".into()), i == sel, 312.0) {
                        menu.modes.chal_sel = i;
                    }
                    ui.add_space(2.0);
                }
            });
        });
        ui.add_space(8.0);
        ui.vertical(|ui| {
            let Some(c) = list.get(sel) else { return };
            parchment(ui, art, |ui| {
                ink_heading(ui, &c.name);
                let base = content.find_base(&c.base).map(|m| m.name).unwrap_or_else(|| "(Basis fehlt)".into());
                ui.label(RichText::new(format!("{} · {} · {}", base, c.mode.label(), c.difficulty.label())).font(theme::italic_font(14.0)).color(theme::INK_FADED));
                ui.add_space(4.0);
                ink_text(ui, &c.description);
                if !c.goal_text.is_empty() {
                    ink_caps(ui, "Aufgabe");
                    ink_text(ui, &c.goal_text);
                }
                if !c.modifiers.is_empty() {
                    ink_caps(ui, "Regeln");
                    for m in &c.modifiers {
                        ink_text(ui, &format!("• {}", m.text()));
                    }
                }
                ink_caps(ui, "Wertung");
                let how = match c.scoring.rank_by {
                    RankBy::Score => "Höchste Punktzahl",
                    RankBy::Time => "Kürzeste Siegzeit",
                };
                ink_text(ui, &format!("{how}. Sieg {} P, Gefallene Feinde {:+.0} P, Verluste {:+.0} P{}.", c.scoring.victory_bonus, c.scoring.per_kill, c.scoring.per_unit_lost, if c.scoring.par_time > 0.0 { format!(", Zeitbonus bis {}", challenge::format_time(c.scoring.par_time)) } else { String::new() }));
            });
            ui.add_space(6.0);
            label(ui, "Bestenliste");
            let top = menu.modes.leaderboard.top(c, 5);
            if top.is_empty() {
                ui.label(RichText::new("Noch keine Einträge.").font(theme::italic_font(14.5)).color(theme::PARCHMENT_DIM));
            }
            for (i, e) in top.iter().enumerate() {
                ui.label(RichText::new(format!("{}. {} – {}", i + 1, e.name, entry_text(c.scoring.rank_by, e))).font(theme::body_font(15.0)).color(if i == 0 { theme::GOLD_BRIGHT } else { theme::PARCHMENT }));
            }
        });
    });
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 440.0).max(0.0) / 2.0);
        if let Some(c) = list.get(sel)
            && ui_common::button(ui, engine, art, "Starten", 220.0)
        {
            match content.find_base(&c.base) {
                Some(base) => {
                    let p = challenge::prepare(c, &base);
                    let mut st = p.settings;
                    st.hero_permadeath |= settings.hero_permadeath;
                    menu.action = MenuAction::StartDef { mission: Box::new(p.mission), settings: st, challenge: Some(Box::new((c.clone(), p.iron_man))) };
                }
                None => menu.error = Some(format!("Basis '{}' der Herausforderung nicht gefunden.", c.base)),
            }
        }
        if ui_common::button(ui, engine, art, "Zurück", 200.0) {
            menu.page = Page::Main;
        }
    });
}

// ---------------------------------------------------------------------------------------------
// Seite „Gefecht-Einrichtung“
// ---------------------------------------------------------------------------------------------

pub fn skirmish_page(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, settings: &mut UserSettings, art: &UiArt) {
    let user = std::mem::take(&mut menu.modes.scen_user);
    skirmish_inner(menu, ui, engine, content, &user, settings, art);
    menu.modes.scen_user = user;
}

fn skirmish_inner(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, user: &[ScenarioEntry], settings: &mut UserSettings, art: &UiArt) {
    ui.set_width(980.0);
    ui_art::heading(ui, art, "Gefecht einrichten", 26.0);
    let all = all_scenarios(content, user);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(520.0);
            let f = &mut menu.modes.skirmish;
            egui::Grid::new("skirmish_grid").num_columns(2).spacing([20.0, 12.0]).min_col_width(130.0).show(ui, |ui| {
                label(ui, "Karte");
                ui.horizontal(|ui| {
                    for (s, name) in [(MapSource::Generated, "Generiert"), (MapSource::Scenario, "Szenario")] {
                        let r = ui_art::choice(ui, art, name, f.source == s);
                        ui_common::sfx(engine, ui, &r);
                        if r.clicked() {
                            f.source = s;
                        }
                    }
                });
                ui.end_row();
                if f.source == MapSource::Generated {
                    label(ui, "Vorlage");
                    ui.vertical(|ui| {
                        for row in GenPreset::ALL.chunks(3) {
                            ui.horizontal(|ui| {
                                for p in row {
                                    let r = ui_art::choice(ui, art, p.label(), f.preset == *p);
                                    ui_common::sfx(engine, ui, &r);
                                    if r.clicked() {
                                        f.preset = *p;
                                    }
                                }
                            });
                        }
                    });
                    ui.end_row();
                    label(ui, "Größe");
                    ui.horizontal(|ui| {
                        for (i, (name, _)) in SIZES.iter().enumerate() {
                            let r = ui_art::choice(ui, art, name, f.size_idx == i);
                            ui_common::sfx(engine, ui, &r);
                            if r.clicked() {
                                f.size_idx = i;
                            }
                        }
                    });
                    ui.end_row();
                    label(ui, "Seed");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut f.seed));
                        if ui.button("Würfeln").clicked() {
                            f.seed = (ui.ctx().input(|i| i.time) * 1000.0) as u64 % 1_000_000;
                        }
                    });
                    ui.end_row();
                } else {
                    label(ui, "Szenario");
                    egui::ComboBox::from_id_salt("skirmish_scen").width(330.0).selected_text(all.get(f.scenario).map(|s| s.def.name.clone()).unwrap_or_else(|| "–".into())).show_ui(ui, |ui| {
                        for (i, s) in all.iter().enumerate() {
                            ui.selectable_value(&mut f.scenario, i, format!("{} ({})", s.def.name, s.source.label()));
                        }
                    });
                    ui.end_row();
                }
                label(ui, "Schwierigkeit");
                difficulty_row(ui, engine, art, &mut f.difficulty);
                ui.end_row();
                label(ui, "Startressourcen");
                ui.horizontal(|ui| {
                    for w in Wealth::ALL {
                        let r = ui_art::choice(ui, art, w.label(), f.wealth == w);
                        ui_common::sfx(engine, ui, &r);
                        if r.clicked() {
                            f.wealth = w;
                        }
                    }
                });
                ui.end_row();
                if f.source == MapSource::Generated {
                    label(ui, "Sieg");
                    ui.horizontal(|ui| {
                        for w in WinKind::ALL {
                            let r = ui_art::choice(ui, art, w.label(), f.win == w);
                            ui_common::sfx(engine, ui, &r);
                            if r.clicked() {
                                f.win = w;
                            }
                        }
                    });
                    ui.end_row();
                    if f.win == WinKind::Survive {
                        label(ui, "Nächte");
                        ui.add(egui::Slider::new(&mut f.nights, 1..=20));
                        ui.end_row();
                    }
                }
                label(ui, "Heldentod");
                ui.checkbox(&mut settings.hero_permadeath, RichText::new("Fällt die Heldin, bleibt sie tot").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                ui.end_row();
            });
            ui.add_space(6.0);
            let text = match f.source {
                MapSource::Generated => format!("{}\n{}", f.preset.description(), f.win.description()),
                MapSource::Scenario => all.get(f.scenario).map(|s| s.def.description.clone()).unwrap_or_default(),
            };
            ui.label(RichText::new(text).font(theme::italic_font(14.5)).color(theme::PARCHMENT_DIM));
            ui.label(RichText::new(format!("Schwierigkeit „{}“: Wellenstärke ×{:.2}", f.difficulty.label(), f.difficulty.wave_mult())).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
        });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            let f = menu.modes.skirmish.clone();
            // Vorschaubild (nur bei Änderungen neu erzeugt).
            match f.source {
                MapSource::Generated => {
                    let size = SIZES[f.size_idx.min(SIZES.len() - 1)].1;
                    let key = (f.preset, size, f.seed);
                    if menu.modes.thumb.as_ref().is_none_or(|(k, _)| *k != key) {
                        let g = generate_preset(size, size, f.seed, f.preset);
                        let tex = ui.ctx().load_texture("skirmish_thumb", thumbnail(&g.map, &g.starts), egui::TextureOptions::NEAREST);
                        menu.modes.thumb = Some((key, tex));
                    }
                }
                MapSource::Scenario => {
                    let key = (GenPreset::Aschmark, -(f.scenario as i32) - 1, 0);
                    if let Some(s) = all.get(f.scenario)
                        && let Some(m) = &s.def.map
                        && menu.modes.thumb.as_ref().is_none_or(|(k, _)| *k != key)
                    {
                        let starts: Vec<glam::Vec2> = s.def.starts.iter().map(|st| glam::Vec2::new(st.x, st.y)).collect();
                        let tex = ui.ctx().load_texture("skirmish_thumb", thumbnail(m, &starts), egui::TextureOptions::NEAREST);
                        menu.modes.thumb = Some((key, tex));
                    }
                }
            }
            art.panel(ui, PanelKind::Thin, egui::Margin::same(10), |ui| {
                if let Some((_, tex)) = &menu.modes.thumb {
                    let size = tex.size_vec2();
                    let scale = (360.0 / size.x.max(size.y)).min(1.0);
                    ui.image((tex.id(), size * scale));
                }
            });
        });
    });
    if let Some(e) = &menu.modes.skirmish_error {
        ui.label(RichText::new(e).color(theme::EMBER));
    }
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 440.0).max(0.0) / 2.0);
        if ui_common::button(ui, engine, art, "Gefecht starten", 240.0) {
            match build_skirmish(&menu.modes.skirmish, &all, settings.hero_permadeath) {
                Ok((mission, st)) => {
                    menu.modes.skirmish_error = None;
                    menu.action = MenuAction::StartDef { mission: Box::new(mission), settings: st, challenge: None };
                }
                Err(e) => menu.modes.skirmish_error = Some(e),
            }
        }
        if ui_common::button(ui, engine, art, "Zurück", 200.0) {
            menu.page = Page::Main;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::validate;
    use ashen_sim::data::GameData;
    use ashen_sim::mission::Outcome;
    use ashen_sim::SimWorld;
    use std::path::PathBuf;

    fn assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    #[test]
    fn skirmish_builds_for_all_presets_sizes_and_wins() {
        let data = GameData::load_dir(&assets().join("data")).unwrap();
        for preset in GenPreset::ALL {
            for (idx, win) in [(0, WinKind::DestroyBase), (1, WinKind::Survive), (2, WinKind::Annihilate), (3, WinKind::DestroyBase)] {
                let form = SkirmishForm { preset, size_idx: idx, win, seed: 31 + idx as u64, wealth: Wealth::Rich, difficulty: Difficulty::Hard, ..Default::default() };
                let (mission, settings) = build_skirmish(&form, &[], false).expect("Mission");
                assert_eq!(settings.difficulty, Difficulty::Hard, "Schwierigkeit wird durchgereicht");
                let doc = EditDoc::from_mission(&data, &mission).unwrap();
                let issues = validate::validate(&doc, &data);
                let errors: Vec<_> = issues.iter().filter(|i| i.level == validate::Level::Error).map(|i| i.text.clone()).collect();
                assert!(errors.is_empty(), "{preset:?}/{idx}/{win:?}: {errors:?}");
                let mut w = SimWorld::new(&data, mission, settings).unwrap();
                for _ in 0..200 {
                    w.step(&data);
                    w.events.clear();
                }
                assert_ne!(w.mission.outcome, Some(Outcome::Defeat));
            }
        }
    }

    #[test]
    fn bundled_scenarios_pass_editor_validation() {
        let data = GameData::load_dir(&assets().join("data")).unwrap();
        let mut errors = Vec::new();
        let list = ashen_sim::scenario::discover(&[(assets().join("data/scenarios"), ashen_sim::scenario::ScenarioSource::Bundled)], &mut errors);
        assert!(errors.is_empty() && list.len() >= 3, "{errors:?}");
        for s in list {
            let doc = EditDoc::from_mission(&data, &s.def).unwrap();
            let issues = validate::validate(&doc, &data);
            let errs: Vec<_> = issues.iter().filter(|i| i.level == validate::Level::Error).map(|i| i.text.clone()).collect();
            assert!(errs.is_empty(), "{}: {errs:?}", s.def.id);
        }
    }

    #[test]
    fn wealth_scales_start_resources() {
        let a = build_skirmish(&SkirmishForm { wealth: Wealth::Poor, ..Default::default() }, &[], false).unwrap().0;
        let b = build_skirmish(&SkirmishForm { wealth: Wealth::Lavish, ..Default::default() }, &[], false).unwrap().0;
        assert!((b.start_resources.gold / a.start_resources.gold - 8.0).abs() < 1e-3);
    }

    #[test]
    fn dates_format_correctly() {
        assert_eq!(format_date(0), "1970-01-01");
        assert_eq!(format_date(1_700_000_000), "2023-11-14");
        assert_eq!(format_date(951_782_400), "2000-02-29");
    }

    #[test]
    fn leaderboard_roundtrip_on_disk_and_report() {
        let dir = std::env::temp_dir().join(format!("ashen_lb_{}", std::process::id()));
        let path = dir.join("bestenliste.ron");
        let _ = std::fs::remove_dir_all(&dir);
        let def = ChallengeDef::load(&assets().join("data/challenges/c03_speedrun.ron")).unwrap();
        assert_eq!(def.scoring.rank_by, RankBy::Time);
        let r = |time: f32, score: i64| ChallengeResult { victory: true, time, kills: 9, units_lost: 1, buildings: 4, residents: 0, waves: 2, nights: 1, score };
        let first = record_result(&def, &r(900.0, 500), "A", &path, 1_700_000_000);
        assert_eq!(first.rank, Some(0));
        assert!(first.new_best);
        let slower = record_result(&def, &r(1200.0, 800), "B", &path, 1_700_000_100);
        assert_eq!(slower.rank, Some(1));
        assert!(!slower.new_best);
        let faster = record_result(&def, &r(700.0, 100), "C", &path, 1_700_000_200);
        assert!(faster.new_best && faster.rank == Some(0));
        // Neu von der Platte laden.
        let lb = Leaderboard::load(&path);
        assert_eq!(lb.entries.len(), 3);
        assert_eq!(lb.best(&def).unwrap().name, "C");
        assert_eq!(faster.top.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["C", "A", "B"]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
