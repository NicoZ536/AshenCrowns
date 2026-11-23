//! Menüseiten der Echtzeit-Kampagne „Das Zeitalter der Gefallenen“: Profile, Zeitalter-Übersicht auf der
//! Pergamentkarte, Briefing, Zusammenfassung nach der Mission und das Ende.
//!
//! Der Zustand liegt in [`CampaignUi`] (im Hauptmenü); Spielstände der Kampagne (mehrere Profile) liegen im
//! Benutzerordner und werden robust gelesen: fehlende Dateien gelten als leer, beschädigte werden gemeldet.

use crate::content::Content;
use crate::menu::{MenuAction, MenuScene, Page};
use crate::play_modes::{ink_caps, ink_heading, ink_text, label, list_row, parchment};
use crate::settings::campaign_dir;
use crate::ui_art::{self, PanelKind, UiArt};
use crate::ui_common;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::campaign::{self, CampaignDef, CampaignProgress, MissionStatus, SlotState, SLOT_COUNT};
use ashen_sim::mission::{Action, Difficulty, MissionDef};
use egui::{Align2, Color32, Pos2, Rect, RichText, Sense, Shape, Stroke, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Stage {
    #[default]
    Slots,
    Overview,
    Briefing,
    Summary,
    Ending,
}

/// Ergebnis einer Mission für die Zusammenfassung.
#[derive(Clone, Debug, Default)]
pub struct Summary {
    pub mission: String,
    pub victory: bool,
    pub time: f32,
    pub kills: u32,
    pub lost: u32,
    pub trained: u32,
    /// Getroffene Entscheidungen: (Titel, Antwort).
    pub choices: Vec<(String, String)>,
    pub hero: Option<(String, u32)>,
    pub unlocked: Vec<String>,
    /// Schlusstext der Mission.
    pub text: String,
}

#[derive(Default)]
pub struct CampaignUi {
    pub stage: Stage,
    slots: Vec<SlotState>,
    loaded: bool,
    pub slot: usize,
    pub progress: CampaignProgress,
    chapter: usize,
    pub selected: Option<String>,
    name_edit: String,
    confirm_delete: Option<usize>,
    pub summary: Option<Summary>,
    pub message: Option<String>,
    ending_voiced: bool,
}

impl CampaignUi {
    fn reload(&mut self, plan: &CampaignDef) {
        self.slots = (0..SLOT_COUNT).map(|i| campaign::read_slot(&campaign_dir(), i, plan)).collect();
        self.loaded = true;
    }

    fn save(&mut self) {
        if let Err(e) = self.progress.save(&campaign::slot_path(&campaign_dir(), self.slot)) {
            self.message = Some(format!("Kampagnenstand nicht gespeichert: {e}"));
        }
    }

    /// Öffnet ein Profil (für automatische Läufe und nach der Mission).
    pub fn open_slot(&mut self, plan: &CampaignDef, slot: usize, progress: CampaignProgress) {
        self.slot = slot;
        self.progress = progress;
        self.chapter = self.current_chapter(plan);
        self.selected = plan.next(&self.progress).map(str::to_string);
        self.stage = Stage::Overview;
        self.loaded = false;
    }

    /// Öffnet das Briefing einer Mission.
    pub fn show_briefing(&mut self, id: &str) {
        self.selected = Some(id.to_string());
        self.stage = Stage::Briefing;
    }

    /// Zeigt das Ende der Kampagne.
    pub fn show_ending(&mut self) {
        self.stage = Stage::Ending;
    }

    fn current_chapter(&self, plan: &CampaignDef) -> usize {
        let id = self.selected.clone().or_else(|| plan.next(&self.progress).map(str::to_string));
        id.and_then(|id| plan.chapters.iter().position(|c| c.missions.iter().any(|m| m.id == id))).unwrap_or(0)
    }

    /// Zeigt die Zusammenfassung einer beendeten Mission.
    pub fn show_summary(&mut self, plan: &CampaignDef, slot: usize, progress: CampaignProgress, summary: Summary) {
        self.slot = slot;
        self.progress = progress;
        self.selected = plan.next(&self.progress).map(str::to_string);
        self.chapter = self.current_chapter(plan);
        self.summary = Some(summary);
        self.stage = Stage::Summary;
        self.loaded = false;
    }
}

fn status_color(s: MissionStatus) -> Color32 {
    match s {
        MissionStatus::Completed => Color32::from_rgb(46, 92, 34),
        MissionStatus::Available => theme::INK_RED,
        MissionStatus::Locked => theme::INK_FADED,
        MissionStatus::Skipped => Color32::from_rgb(120, 110, 100),
    }
}

fn status_label(s: MissionStatus) -> &'static str {
    match s {
        MissionStatus::Completed => "abgeschlossen",
        MissionStatus::Available => "offen",
        MissionStatus::Locked => "gesperrt",
        MissionStatus::Skipped => "entfällt",
    }
}

fn mission_def<'a>(content: &'a Content, id: &str) -> Option<&'a MissionDef> {
    content.campaign_mission(id)
}

fn difficulty_row(ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, current: &mut Difficulty) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        for d in Difficulty::ALL {
            let r = ui_art::choice(ui, art, d.label(), *current == d);
            ui_common::sfx(engine, ui, &r);
            if r.clicked() && *current != d {
                *current = d;
                changed = true;
            }
        }
    });
    changed
}

/// Die Kampagnenseite des Hauptmenüs.
pub fn campaign_page(menu: &mut MenuScene, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, art: &UiArt) {
    let mut st = std::mem::take(&mut menu.campaign);
    ui.set_width(1040.0);
    ui_art::heading(ui, art, "Das Zeitalter der Gefallenen", 26.0);
    match &content.campaign {
        None => {
            ui.label(RichText::new("Der Kampagnenplan wurde nicht gefunden (assets/data/campaign/campaign.ron).").font(theme::italic_font(16.0)).color(theme::EMBER));
            ui.add_space(10.0);
            if ui_common::button(ui, engine, art, "Zurück", 200.0) {
                menu.page = Page::Main;
            }
        }
        Some(plan) => {
            if !st.loaded && st.stage == Stage::Slots {
                st.reload(plan);
            }
            match st.stage {
                Stage::Slots => slots(menu, &mut st, ui, engine, plan, art),
                Stage::Overview => overview(menu, &mut st, ui, engine, content, plan, art),
                Stage::Briefing => briefing(menu, &mut st, ui, engine, content, plan, art),
                Stage::Summary => summary(&mut st, ui, engine, plan, art),
                Stage::Ending => ending(&mut st, ui, engine, plan, art),
            }
        }
    }
    if let Some(m) = &st.message {
        ui.label(RichText::new(m).font(theme::italic_font(14.0)).color(theme::EMBER));
    }
    menu.campaign = st;
}

// ---------------------------------------------------------------------------------------------
// Profile
// ---------------------------------------------------------------------------------------------

fn slots(menu: &mut MenuScene, st: &mut CampaignUi, ui: &mut egui::Ui, engine: &mut Engine, plan: &CampaignDef, art: &UiArt) {
    ui.label(RichText::new(&plan.intro).font(theme::italic_font(15.5)).color(theme::PARCHMENT_DIM));
    ui.add_space(8.0);
    let mut action: Option<(usize, &'static str)> = None;
    for i in 0..SLOT_COUNT {
        let state = st.slots.get(i).cloned().unwrap_or(SlotState::Empty);
        art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(18, 10), |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(560.0);
                    match &state {
                        SlotState::Empty => {
                            ui.label(RichText::new(format!("Profil {} – leer", i + 1)).font(theme::heading_font(17.0)).color(theme::PARCHMENT_DIM));
                            ui.label(RichText::new("Ein neues Kapitel in der Chronik von Eldraeth beginnen.").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                        }
                        SlotState::Ready(p) => {
                            ui.label(RichText::new(format!("Profil {} – {}", i + 1, p.name)).font(theme::heading_font(17.0)).color(theme::GOLD));
                            let done = plan.completed_count(p);
                            let next = plan.next(p).and_then(|id| plan.entry(id)).map(|(c, m)| format!("nächste Mission: {} (Zeitalter {})", m.id.split_once('_').map(|x| x.1).unwrap_or(&m.id).replace('_', " "), c.numeral)).unwrap_or_else(|| if plan.is_finished(p) { "Kampagne abgeschlossen".into() } else { "—".into() });
                            ui.label(RichText::new(format!("{done} von {} Missionen · {} · {next}", plan.mission_count(), p.difficulty.label())).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                        }
                        SlotState::Damaged(e) => {
                            ui.label(RichText::new(format!("Profil {} – beschädigt", i + 1)).font(theme::heading_font(17.0)).color(theme::EMBER));
                            ui.label(RichText::new(format!("Die Datei ist nicht lesbar und bleibt unberührt: {}", e.chars().take(110).collect::<String>())).font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
                        }
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| match &state {
                    SlotState::Empty => {
                        if ui_common::button(ui, engine, art, "Neu beginnen", 180.0) {
                            action = Some((i, "new"));
                        }
                    }
                    SlotState::Ready(_) => {
                        if st.confirm_delete == Some(i) {
                            if ui_common::button(ui, engine, art, "Wirklich löschen", 190.0) {
                                action = Some((i, "delete"));
                            }
                        } else {
                            if ui_common::button(ui, engine, art, "Löschen", 120.0) {
                                st.confirm_delete = Some(i);
                            }
                            if ui_common::button(ui, engine, art, "Fortsetzen", 150.0) {
                                action = Some((i, "open"));
                            }
                        }
                    }
                    SlotState::Damaged(_) => {
                        if ui_common::button(ui, engine, art, "Beiseitelegen", 180.0) {
                            action = Some((i, "quarantine"));
                        }
                    }
                });
            });
        });
        ui.add_space(3.0);
    }
    if let Some((i, what)) = action {
        match what {
            "new" => {
                st.progress = CampaignProgress::new(&st.name_edit);
                st.slot = i;
                st.name_edit.clear();
                st.save();
                let p = st.progress.clone();
                st.open_slot(plan, i, p);
            }
            "open" => {
                if let Some(SlotState::Ready(p)) = st.slots.get(i).cloned() {
                    st.open_slot(plan, i, *p);
                }
            }
            "delete" => {
                if let Err(e) = campaign::delete_slot(&campaign_dir(), i) {
                    st.message = Some(e);
                }
                st.confirm_delete = None;
                st.loaded = false;
            }
            _ => {
                if let Err(e) = campaign::quarantine_slot(&campaign_dir(), i) {
                    st.message = Some(e);
                }
                st.loaded = false;
            }
        }
    }
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        label(ui, "Name für ein neues Profil");
        ui.add(egui::TextEdit::singleline(&mut st.name_edit).desired_width(200.0).hint_text("Wächterin").font(theme::body_font(16.0)));
    });
    ui.add_space(8.0);
    ui.vertical_centered(|ui| {
        if ui_common::button(ui, engine, art, "Zurück", 200.0) {
            menu.page = Page::Main;
        }
    });
}

// ---------------------------------------------------------------------------------------------
// Übersicht: Zeitalter, Kapitelseite, Karte
// ---------------------------------------------------------------------------------------------

/// Umriss des Kontinents Eldraeth (Verhältnisse 0..1) für die Pergamentkarte.
fn continent() -> Vec<(f32, f32)> {
    let pts = [(0.10, 0.34), (0.16, 0.20), (0.30, 0.12), (0.44, 0.06), (0.60, 0.10), (0.74, 0.18), (0.88, 0.30), (0.93, 0.46), (0.88, 0.62), (0.78, 0.70), (0.72, 0.84), (0.60, 0.94), (0.46, 0.92), (0.36, 0.84), (0.24, 0.80), (0.14, 0.68), (0.08, 0.50)];
    pts.to_vec()
}

fn map_panel(ui: &mut egui::Ui, st: &mut CampaignUi, plan: &CampaignDef, engine: &mut Engine, content: &Content) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(316.0, 400.0), Sense::hover());
    let p = ui.painter();
    let at = |x: f32, y: f32| Pos2::new(rect.min.x + x * rect.width(), rect.min.y + y * rect.height());
    // Küste und Regionen.
    let outline: Vec<Pos2> = continent().iter().map(|(x, y)| at(*x, *y)).collect();
    p.add(Shape::convex_polygon(outline.clone(), Color32::from_rgba_unmultiplied(120, 88, 50, 40), Stroke::new(1.6, theme::INK_FADED)));
    for (name, x, y) in [("Skarnheim", 0.50, 0.12), ("Dornwald", 0.20, 0.36), ("Valemark", 0.46, 0.38), ("Aschmark", 0.44, 0.58), ("Saelith", 0.80, 0.44), ("Aurethwüste", 0.56, 0.76), ("Schleiernarbe", 0.50, 0.90)] {
        p.text(at(x, y), Align2::CENTER_CENTER, name, theme::italic_font(12.5), theme::INK_FADED.gamma_multiply(0.85));
    }
    // Weg der Chronik.
    let order: Vec<(&str, (f32, f32))> = plan.missions().map(|(_, m)| (m.id.as_str(), m.pos)).collect();
    for w in order.windows(2) {
        let (a, b) = (at(w[0].1 .0, w[0].1 .1), at(w[1].1 .0, w[1].1 .1));
        let done = st.progress.is_completed(w[0].0);
        let col = if done { theme::INK_RED.gamma_multiply(0.9) } else { theme::INK_FADED.gamma_multiply(0.55) };
        let n = ((b - a).length() / 8.0).max(1.0) as usize;
        for i in 0..n {
            if done || i % 2 == 0 {
                let (u, v) = (i as f32 / n as f32, (i as f32 + 0.6) / n as f32);
                p.line_segment([a.lerp(b, u), a.lerp(b, v.min(1.0))], Stroke::new(if done { 2.0 } else { 1.2 }, col));
            }
        }
    }
    // Missionsknoten.
    let mut clicked: Option<String> = None;
    let mut hovered: Option<(String, String)> = None;
    for (i, (ch, m)) in plan.missions().enumerate() {
        let c = at(m.pos.0, m.pos.1);
        let status = plan.status(&st.progress, &m.id);
        let r = Rect::from_center_size(c, Vec2::splat(22.0));
        let resp = ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| ui.allocate_rect(r, Sense::click())).inner;
        let sel = st.selected.as_deref() == Some(m.id.as_str());
        let p = ui.painter();
        if sel {
            p.circle_stroke(c, 14.0, Stroke::new(2.0, theme::GOLD));
        }
        let col = status_color(status);
        p.circle_filled(c, 9.5, if status == MissionStatus::Skipped { Color32::from_rgb(190, 178, 156) } else { Color32::from_rgb(232, 214, 176) });
        p.circle_stroke(c, 9.5, Stroke::new(if resp.hovered() { 2.4 } else { 1.6 }, col));
        if status == MissionStatus::Completed {
            p.line_segment([c + Vec2::new(-4.5, 0.5), c + Vec2::new(-1.5, 4.0)], Stroke::new(2.2, col));
            p.line_segment([c + Vec2::new(-1.5, 4.0), c + Vec2::new(5.0, -4.5)], Stroke::new(2.2, col));
        } else {
            p.text(c, Align2::CENTER_CENTER, (i + 1).to_string(), theme::heading_font(11.0), col);
        }
        if resp.hovered() {
            let name = mission_def(content, &m.id).map(|d| d.name.clone()).unwrap_or_else(|| m.id.clone());
            hovered = Some((name, format!("Zeitalter {} · {}", ch.numeral, status_label(status))));
        }
        if resp.clicked() && status != MissionStatus::Locked {
            clicked = Some(m.id.clone());
            engine.audio.play("sfx.ui_click");
        }
    }
    if let Some((n, s)) = hovered {
        let p = ui.painter();
        let pos = rect.left_bottom() + Vec2::new(8.0, -8.0);
        p.text(pos + Vec2::new(0.0, -18.0), Align2::LEFT_BOTTOM, n, theme::heading_font(13.5), theme::INK);
        p.text(pos, Align2::LEFT_BOTTOM, s, theme::italic_font(12.5), theme::INK_FADED);
    }
    if let Some(id) = clicked {
        st.chapter = plan.chapters.iter().position(|c| c.missions.iter().any(|m| m.id == id)).unwrap_or(st.chapter);
        st.selected = Some(id);
    }
}

fn overview(menu: &mut MenuScene, st: &mut CampaignUi, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, plan: &CampaignDef, art: &UiArt) {
    let chapter = st.chapter.min(plan.chapters.len() - 1);
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("Profil: {}", st.progress.name)).font(theme::heading_font(15.0)).color(theme::GOLD));
        ui.label(RichText::new(format!("· {} von {} Missionen abgeschlossen", plan.completed_count(&st.progress), plan.mission_count())).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
    });
    ui.add_space(4.0);
    ui.horizontal_top(|ui| {
        // Zeitalter als Kapitelreiter.
        ui.vertical(|ui| {
            ui.set_width(292.0);
            for (i, c) in plan.chapters.iter().enumerate() {
                let reachable = c.missions.iter().any(|m| plan.status(&st.progress, &m.id) != MissionStatus::Locked);
                let done = c.missions.iter().all(|m| st.progress.is_completed(&m.id) || plan.is_skipped(&st.progress, &m.id));
                let sub = format!("{} · {}", c.years, if done { "vollendet" } else if reachable { "offen" } else { "verschlossen" });
                if list_row(ui, engine, art, &format!("{}  {}", c.numeral, c.title), &sub, i == chapter, 286.0) && reachable {
                    st.chapter = i;
                    if let Some(first) = c.missions.iter().find(|m| plan.status(&st.progress, &m.id) == MissionStatus::Available).or(c.missions.first()) {
                        st.selected = Some(first.id.clone());
                    }
                    if let Some(v) = &c.voice {
                        engine.audio.play(v);
                    }
                }
                ui.add_space(2.0);
            }
        });
        ui.add_space(6.0);
        // Kapitelseite.
        ui.vertical(|ui| {
            ui.set_width(380.0);
            let c = &plan.chapters[chapter];
            parchment(ui, art, |ui| {
                ui.set_min_height(330.0);
                ui.label(RichText::new(format!("Zeitalter {} – {}", c.numeral, c.subtitle)).font(theme::caps_font(12.5)).color(theme::INK_FADED));
                ink_heading(ui, &c.title);
                ui.label(RichText::new(&c.blurb).font(theme::italic_font(14.5)).color(theme::INK_FADED));
                ui.add_space(6.0);
                for m in &c.missions {
                    let status = plan.status(&st.progress, &m.id);
                    let Some(def) = mission_def(content, &m.id) else { continue };
                    let sel = st.selected.as_deref() == Some(m.id.as_str());
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 52.0), Sense::click());
                    let p = ui.painter();
                    let bg = if sel { Color32::from_rgba_unmultiplied(150, 40, 24, 36) } else if resp.hovered() { Color32::from_rgba_unmultiplied(90, 60, 30, 30) } else { Color32::TRANSPARENT };
                    p.rect_filled(rect, 3.0, bg);
                    p.line_segment([rect.left_bottom(), rect.right_bottom()], Stroke::new(1.0, theme::INK_FADED.gamma_multiply(0.4)));
                    let col = status_color(status);
                    p.circle_filled(rect.left_center() + Vec2::new(12.0, 0.0), 6.0, col);
                    p.text(rect.left_top() + Vec2::new(30.0, 8.0), Align2::LEFT_TOP, &def.name, theme::heading_font(16.0), if status == MissionStatus::Locked { theme::INK_FADED } else { theme::INK });
                    let year = def.year.map(|y| format!("{y} n.F. · ")).unwrap_or_default();
                    p.text(rect.left_bottom() + Vec2::new(30.0, -7.0), Align2::LEFT_BOTTOM, format!("{year}{} · {}", def.kind, status_label(status)), theme::italic_font(13.0), col);
                    if resp.clicked() && status != MissionStatus::Locked {
                        st.selected = Some(m.id.clone());
                        engine.audio.play("sfx.ui_click");
                    }
                }
            });
        });
        ui.add_space(6.0);
        // Pergamentkarte.
        ui.vertical(|ui| {
            ui.set_width(344.0);
            art.panel(ui, PanelKind::Parchment, egui::Margin::symmetric(14, 14), |ui| {
                map_panel(ui, st, plan, engine, content);
            });
        });
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        label(ui, "Schwierigkeit");
        let mut d = st.progress.difficulty;
        if difficulty_row(ui, engine, art, &mut d) {
            st.progress.difficulty = d;
            st.save();
        }
        let mut perma = st.progress.hero_permadeath;
        if ui.checkbox(&mut perma, RichText::new("Dauerhafter Heldentod").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM)).changed() {
            st.progress.hero_permadeath = perma;
            st.save();
        }
    });
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 700.0).max(0.0) / 2.0);
        let can = st.selected.as_deref().is_some_and(|id| matches!(plan.status(&st.progress, id), MissionStatus::Available | MissionStatus::Completed));
        let r = ui_art::button(ui, art, "Briefing öffnen", Vec2::new(230.0, 36.0), can);
        ui_common::sfx(engine, ui, &r);
        if r.clicked() {
            st.stage = Stage::Briefing;
        }
        if plan.is_finished(&st.progress) && ui_common::button(ui, engine, art, "Das Ende lesen", 200.0) {
            st.stage = Stage::Ending;
        }
        if ui_common::button(ui, engine, art, "Profile", 160.0) {
            st.stage = Stage::Slots;
            st.loaded = false;
        }
        if ui_common::button(ui, engine, art, "Hauptmenü", 180.0) {
            menu.page = Page::Main;
            st.stage = Stage::Slots;
            st.loaded = false;
        }
    });
}

// ---------------------------------------------------------------------------------------------
// Briefing
// ---------------------------------------------------------------------------------------------

fn briefing(menu: &mut MenuScene, st: &mut CampaignUi, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, plan: &CampaignDef, art: &UiArt) {
    let Some(id) = st.selected.clone() else {
        st.stage = Stage::Overview;
        return;
    };
    let Some(def) = mission_def(content, &id).cloned() else {
        st.message = Some(format!("Mission {id} nicht gefunden."));
        st.stage = Stage::Overview;
        return;
    };
    let chapter = plan.entry(&id).map(|(c, _)| c.clone());
    let flags = st.progress.flags.clone();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(660.0);
            parchment(ui, art, |ui| {
                if let Some(c) = &chapter {
                    ui.label(RichText::new(format!("Zeitalter {} – {}", c.numeral, c.title)).font(theme::caps_font(12.5)).color(theme::INK_FADED));
                }
                ink_heading(ui, &def.name);
                let year = def.year.map(|y| format!("Jahr {y} n.F. · ")).unwrap_or_default();
                ui.label(RichText::new(format!("{year}{}", def.kind)).font(theme::italic_font(14.5)).color(theme::INK_FADED));
                ui.add_space(4.0);
                egui::ScrollArea::vertical().id_salt("brief_text").max_height(300.0).show(ui, |ui| {
                    ink_text(ui, &def.briefing);
                    let notes: Vec<&str> = def.active_variants(&flags).into_iter().map(|v| v.note.as_str()).filter(|n| !n.is_empty()).collect();
                    if !notes.is_empty() {
                        ink_caps(ui, "Folgen Eurer Entscheidungen");
                        for n in notes {
                            ui.label(RichText::new(format!("• {n}")).font(theme::italic_font(15.0)).color(theme::INK_RED));
                        }
                    }
                });
            });
        });
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.set_width(340.0);
            parchment(ui, art, |ui| {
                ui.set_min_height(300.0);
                ink_caps(ui, "Hauptziele");
                for o in def.objectives.iter().filter(|o| !o.optional && !o.text.is_empty()) {
                    ui.label(RichText::new(format!("• {}", o.text)).size(14.5).color(theme::INK));
                }
                if def.objectives.iter().any(|o| o.optional) {
                    ink_caps(ui, "Nebenziele");
                    for o in def.objectives.iter().filter(|o| o.optional) {
                        ui.label(RichText::new(format!("- {}", o.text)).font(theme::italic_font(14.0)).color(theme::INK_FADED));
                    }
                }
                let decisions = decisions_of(&def);
                if !decisions.is_empty() {
                    ink_caps(ui, "Entscheidungen");
                    for d in decisions {
                        ui.label(RichText::new(format!("◆ {d}")).font(theme::italic_font(14.0)).color(theme::INK_RED));
                    }
                }
                if let Some(h) = st.progress.hero("hero_isolde") {
                    ink_caps(ui, "Heldin");
                    ui.label(RichText::new(format!("Isolde Raventhorn – Stufe {}", h.level)).size(14.5).color(theme::INK));
                }
            });
        });
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        label(ui, "Schwierigkeit");
        let mut d = st.progress.difficulty;
        if difficulty_row(ui, engine, art, &mut d) {
            st.progress.difficulty = d;
            st.save();
        }
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 480.0).max(0.0) / 2.0);
        if ui_common::button(ui, engine, art, "Mission beginnen", 250.0) {
            let (m, settings) = st.progress.start(&def);
            menu.action = MenuAction::StartCampaign { slot: st.slot, mission: Box::new(m), settings };
        }
        if ui_common::button(ui, engine, art, "Zurück", 200.0) {
            st.stage = Stage::Overview;
        }
    });
}

/// Titel der Entscheidungen einer Mission (für das Briefing).
fn decisions_of(def: &MissionDef) -> Vec<String> {
    fn walk(a: &[Action], out: &mut Vec<String>) {
        for x in a {
            if let Action::Decision(d) = x {
                out.push(d.title.clone());
            }
        }
    }
    let mut out = Vec::new();
    for t in &def.triggers {
        walk(&t.actions, &mut out);
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Zusammenfassung und Ende
// ---------------------------------------------------------------------------------------------

fn summary(st: &mut CampaignUi, ui: &mut egui::Ui, engine: &mut Engine, plan: &CampaignDef, art: &UiArt) {
    let Some(s) = st.summary.clone() else {
        st.stage = Stage::Overview;
        return;
    };
    ui.set_width(760.0);
    parchment(ui, art, |ui| {
        ui.label(RichText::new(if s.victory { "Mission abgeschlossen" } else { "Mission gescheitert" }).font(theme::caps_font(13.0)).color(theme::INK_FADED));
        ink_heading(ui, &s.mission);
        ui.label(RichText::new(&s.text).font(theme::italic_font(15.5)).color(theme::INK));
        ui.add_space(6.0);
        ui.label(RichText::new(format!("Spielzeit {}:{:02} · Besiegte Feinde {} · Verluste {} · Ausgebildet {}", s.time as u32 / 60, s.time as u32 % 60, s.kills, s.lost, s.trained)).size(15.0).color(theme::INK));
        if let Some((n, l)) = &s.hero {
            ui.label(RichText::new(format!("{n} – Stufe {l}")).size(15.0).color(theme::INK));
        }
        if !s.choices.is_empty() {
            ink_caps(ui, "Eure Entscheidungen");
            for (t, a) in &s.choices {
                ui.label(RichText::new(format!("◆ {t}: {a}")).font(theme::italic_font(15.0)).color(theme::INK_RED));
            }
        }
        if !s.unlocked.is_empty() {
            ink_caps(ui, "Neu freigeschaltet");
            for u in &s.unlocked {
                ink_text(ui, &format!("• {u}"));
            }
        }
    });
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        let finished = plan.is_finished(&st.progress) && s.victory;
        let label = if finished { "Das Ende" } else { "Weiter" };
        if ui_common::button(ui, engine, art, label, 220.0) {
            st.summary = None;
            st.stage = if finished { Stage::Ending } else { Stage::Overview };
        }
    });
}

fn ending(st: &mut CampaignUi, ui: &mut egui::Ui, engine: &mut Engine, plan: &CampaignDef, art: &UiArt) {
    ui.set_width(760.0);
    match plan.ending(&st.progress) {
        Some(e) => {
            parchment(ui, art, |ui| {
                ui.label(RichText::new("Das Ende der Chronik").font(theme::caps_font(13.0)).color(theme::INK_FADED));
                ink_heading(ui, &e.title);
                ui.add_space(4.0);
                ui.label(RichText::new(&e.text).font(theme::italic_font(17.0)).color(theme::INK));
            });
            if !st.ending_voiced {
                if let Some(v) = &e.voice {
                    engine.audio.play(v);
                }
                st.ending_voiced = true;
            }
        }
        None => ink_text(ui, "Noch kein Ende erreicht."),
    }
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        if ui_common::button(ui, engine, art, "Zur Übersicht", 220.0) {
            st.ending_voiced = false;
            st.stage = Stage::Overview;
        }
    });
}

/// Beschriftung getroffener Entscheidungen (Titel, Antwort) aus den Missionsdaten.
fn describe_choices(def: &MissionDef, choices: &[(String, String)]) -> Vec<(String, String)> {
    fn find<'a>(actions: &'a [Action], id: &str, flag: &str) -> Option<(&'a str, &'a str)> {
        for a in actions {
            if let Action::Decision(d) = a
                && d.id == id
                && let Some(o) = d.options.iter().find(|o| o.flag == flag)
            {
                return Some((d.title.as_str(), o.label.as_str()));
            }
        }
        None
    }
    choices
        .iter()
        .map(|(id, flag)| {
            let hit = def.triggers.iter().find_map(|t| find(&t.actions, id, flag));
            match hit {
                Some((t, l)) => (t.to_string(), l.to_string()),
                None => (id.clone(), flag.clone()),
            }
        })
        .collect()
}

/// Wertet eine beendete Kampagnenmission aus: übernimmt bei Sieg Ergebnis, Flags und Helden in `progress`
/// und liefert die Zusammenfassung.
pub fn summarize(content: &Content, plan: &CampaignDef, def: &MissionDef, world: &ashen_sim::SimWorld, progress: &mut CampaignProgress, before: &[String]) -> Summary {
    let victory = world.mission.outcome == Some(ashen_sim::mission::Outcome::Victory);
    if victory {
        progress.record(def, world, &content.data);
    }
    let s = &world.players[0].stats;
    let hero = progress.hero("hero_isolde").map(|h| ("Isolde Raventhorn".to_string(), h.level));
    let unlocked = plan
        .available(progress)
        .into_iter()
        .filter(|id| !before.iter().any(|b| b == id))
        .filter_map(|id| content.campaign_mission(id).map(|m| m.name.clone()))
        .collect();
    Summary {
        mission: def.name.clone(),
        victory,
        time: world.time,
        kills: s.kills,
        lost: s.units_lost,
        trained: s.units_trained,
        choices: describe_choices(def, &world.mission.choices),
        hero,
        unlocked,
        text: if victory { def.victory_text.clone() } else { def.defeat_text.clone() },
    }
}
