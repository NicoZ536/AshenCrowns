//! Gemeinsame Oberflächenbausteine der Weltkarten-Fenster (Rahmen, Abschnitte, Balken, Schaltflächen).

use crate::ui_art::{self, PanelKind, UiArt};
use crate::ui_common;
use ashen_engine::Engine;
use ashen_engine::ui::theme;
use ashen_strategy::*;
use egui::{Align2, Color32, Id, Order, Pos2, RichText, Sense, Ui, Vec2};

/// Fenstertypen der Weltkarte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WinKind {
    Province,
    Realm,
    Family,
    Court,
    Diplomacy,
    Military,
    Economy,
    Religion,
    Magic,
    Chronicle,
    Victory,
}

impl WinKind {
    pub const ALL: [WinKind; 11] = [
        WinKind::Province,
        WinKind::Realm,
        WinKind::Family,
        WinKind::Court,
        WinKind::Diplomacy,
        WinKind::Military,
        WinKind::Economy,
        WinKind::Religion,
        WinKind::Magic,
        WinKind::Chronicle,
        WinKind::Victory,
    ];
    pub fn title(self) -> &'static str {
        match self {
            WinKind::Province => "Provinz",
            WinKind::Realm => "Reich",
            WinKind::Family => "Dynastie",
            WinKind::Court => "Adel & Intrigen",
            WinKind::Diplomacy => "Diplomatie",
            WinKind::Military => "Heere",
            WinKind::Economy => "Wirtschaft",
            WinKind::Religion => "Glaube",
            WinKind::Magic => "Magie",
            WinKind::Chronicle => "Chronik",
            WinKind::Victory => "Siegziele",
        }
    }
    pub fn short(self) -> &'static str {
        match self {
            WinKind::Province => "Provinz",
            WinKind::Realm => "Reich",
            WinKind::Family => "Dynastie",
            WinKind::Court => "Hof",
            WinKind::Diplomacy => "Diplomatie",
            WinKind::Military => "Heere",
            WinKind::Economy => "Wirtschaft",
            WinKind::Religion => "Glaube",
            WinKind::Magic => "Magie",
            WinKind::Chronicle => "Chronik",
            WinKind::Victory => "Siege",
        }
    }
    pub fn key(self) -> egui::Key {
        match self {
            WinKind::Province => egui::Key::P,
            WinKind::Realm => egui::Key::R,
            WinKind::Family => egui::Key::F,
            WinKind::Court => egui::Key::H,
            WinKind::Diplomacy => egui::Key::T,
            WinKind::Military => egui::Key::M,
            WinKind::Economy => egui::Key::E,
            WinKind::Religion => egui::Key::G,
            WinKind::Magic => egui::Key::Z,
            WinKind::Chronicle => egui::Key::C,
            WinKind::Victory => egui::Key::V,
        }
    }
    pub fn key_label(self) -> &'static str {
        match self {
            WinKind::Province => "P",
            WinKind::Realm => "R",
            WinKind::Family => "F",
            WinKind::Court => "H",
            WinKind::Diplomacy => "T",
            WinKind::Military => "M",
            WinKind::Economy => "E",
            WinKind::Religion => "G",
            WinKind::Magic => "Z",
            WinKind::Chronicle => "C",
            WinKind::Victory => "V",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            WinKind::Province => "province",
            WinKind::Realm => "realm",
            WinKind::Family => "family",
            WinKind::Court => "court",
            WinKind::Diplomacy => "diplomacy",
            WinKind::Military => "military",
            WinKind::Economy => "economy",
            WinKind::Religion => "religion",
            WinKind::Magic => "magic",
            WinKind::Chronicle => "chronicle",
            WinKind::Victory => "victory",
        }
    }
    pub fn from_id(s: &str) -> Option<WinKind> {
        WinKind::ALL.iter().copied().find(|k| k.id() == s)
    }
    fn width(self) -> f32 {
        match self {
            WinKind::Province => 470.0,
            WinKind::Realm => 560.0,
            WinKind::Family => 760.0,
            WinKind::Court => 640.0,
            WinKind::Diplomacy => 780.0,
            WinKind::Military => 620.0,
            WinKind::Economy => 600.0,
            WinKind::Religion => 520.0,
            WinKind::Magic => 560.0,
            WinKind::Chronicle => 640.0,
            WinKind::Victory => 520.0,
        }
    }
    fn default_pos(self) -> Pos2 {
        match self {
            WinKind::Province => Pos2::new(14.0, 66.0),
            WinKind::Realm
            | WinKind::Economy
            | WinKind::Religion
            | WinKind::Magic
            | WinKind::Victory => Pos2::new(300.0, 66.0),
            WinKind::Family | WinKind::Diplomacy | WinKind::Court | WinKind::Chronicle => {
                Pos2::new(240.0, 66.0)
            }
            WinKind::Military => Pos2::new(14.0, 66.0),
        }
    }
}

/// Anfragen der Fenster an die Szene (werden nach dem Zeichnen verarbeitet).
#[derive(Clone, Debug)]
pub enum Req {
    Open(WinKind),
    Close(WinKind),
    Select(ProvId),
    Center(ProvId),
    SelectArmy(ArmyId),
    AutoBattle(BattleId),
    PlayBattle(BattleId),
}

/// Zustände der Fenster (Auswahl, Eingabefelder).
pub struct UiState {
    pub dipl_target: Option<RealmId>,
    pub gift: f32,
    pub war_goal: usize,
    pub war_goal_prov: Option<ProvId>,
    pub war_confirm: bool,
    pub peace_cede: Vec<ProvId>,
    pub peace_tribute: bool,
    pub peace_vassal: bool,
    pub chron_kind: Option<ChronicleKind>,
    pub chron_min: u8,
    pub chron_mine: bool,
    pub chron_search: String,
    pub family_sel: Option<CharId>,
    pub dowry: f32,
    pub plot_kind: usize,
    pub recruit_unit: usize,
    pub recruit_count: u32,
    pub loan_amount: f32,
    pub loan_years: u32,
    pub reward: f32,
    pub court_house: Option<HouseId>,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            dipl_target: None,
            gift: 100.0,
            war_goal: 0,
            war_goal_prov: None,
            war_confirm: false,
            peace_cede: Vec::new(),
            peace_tribute: false,
            peace_vassal: false,
            chron_kind: None,
            chron_min: 0,
            chron_mine: false,
            chron_search: String::new(),
            family_sel: None,
            dowry: 100.0,
            plot_kind: 0,
            recruit_unit: 0,
            recruit_count: 10,
            loan_amount: 500.0,
            loan_years: 5,
            reward: 100.0,
            court_house: None,
        }
    }
}

/// Zeichenkontext der Fenster.
pub struct Cx<'a> {
    pub w: &'a StrategyWorld,
    pub me: RealmId,
    pub art: &'a UiArt,
    pub engine: &'a mut Engine,
    pub st: &'a mut UiState,
    pub cmds: &'a mut Vec<StrategyCommand>,
    pub reqs: &'a mut Vec<Req>,
    pub selected: Option<ProvId>,
    pub selected_army: Option<ArmyId>,
    pub last_result: &'a Option<(bool, String)>,
}

impl Cx<'_> {
    pub fn button(&mut self, ui: &mut Ui, text: &str, w: f32, enabled: bool) -> bool {
        // Breite passt sich dem Text an (nie Abschneiden der Beschriftung).
        let tw = ui.painter().layout_no_wrap(text.to_string(), theme::caps_font(12.0), Color32::WHITE).size().x + 32.0;
        let r = ui_art::button(ui, self.art, text, Vec2::new(w.max(tw), 28.0), enabled);
        if enabled {
            ui_common::sfx(self.engine, ui, &r);
        }
        r.clicked() && enabled
    }

    pub fn choice(&mut self, ui: &mut Ui, text: &str, selected: bool) -> bool {
        let r = ui_art::choice(ui, self.art, text, selected);
        ui_common::sfx(self.engine, ui, &r);
        r.clicked()
    }

    pub fn cmd(&mut self, c: StrategyCommand) {
        self.cmds.push(c);
    }
}

pub fn text(ui: &mut Ui, s: impl Into<String>, size: f32, col: Color32) {
    ui.label(
        RichText::new(s.into())
            .font(theme::body_font(size))
            .color(col),
    );
}

pub fn dim(ui: &mut Ui, s: impl Into<String>) {
    ui.label(
        RichText::new(s.into())
            .font(theme::italic_font(13.5))
            .color(theme::PARCHMENT_DIM),
    );
}

pub fn heading(ui: &mut Ui, s: impl Into<String>) {
    ui.label(
        RichText::new(s.into())
            .font(theme::heading_font(16.0))
            .color(theme::GOLD),
    );
}

/// Abschnittstitel mit Trennlinie.
pub fn section(ui: &mut Ui, art: &UiArt, title: &str) {
    ui.add_space(6.0);
    ui.label(
        RichText::new(title)
            .font(theme::caps_font(12.5))
            .color(theme::GOLD_DIM),
    );
    let r = ui.available_rect_before_wrap();
    art.divider(ui.painter(), r.min.x, r.max.x, r.min.y + 3.0);
    ui.add_space(8.0);
}

/// Zeile „Schlüssel … Wert“.
pub fn kv(ui: &mut Ui, k: &str, v: impl Into<String>) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(k)
                .font(theme::body_font(14.0))
                .color(theme::PARCHMENT_DIM),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(v.into())
                    .font(theme::body_bold_font(14.5))
                    .color(theme::PARCHMENT),
            );
        });
    });
}

/// Farbiger Wert in einer Zeile.
pub fn kv_col(ui: &mut Ui, k: &str, v: impl Into<String>, col: Color32) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(k)
                .font(theme::body_font(14.0))
                .color(theme::PARCHMENT_DIM),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(v.into())
                    .font(theme::body_bold_font(14.5))
                    .color(col),
            );
        });
    });
}

/// Metallbalken mit Beschriftung in der Zeile.
pub fn bar_row(ui: &mut Ui, art: &UiArt, label: &str, frac: f32, color: Color32, value: &str) {
    ui.horizontal(|ui| {
        let lw = ui.painter().layout_no_wrap(label.to_string(), theme::body_font(14.0), Color32::WHITE).size().x + 10.0;
        let (lr, _) = ui.allocate_exact_size(Vec2::new(lw.max(128.0), 20.0), Sense::hover());
        ui.painter().text(
            Pos2::new(lr.min.x, lr.center().y),
            Align2::LEFT_CENTER,
            label,
            theme::body_font(14.0),
            theme::PARCHMENT_DIM,
        );
        let w = (ui.available_width() - 4.0).max(60.0);
        let (r, _) = ui.allocate_exact_size(Vec2::new(w, 18.0), Sense::hover());
        art.bar(ui.painter(), r, frac, color, value);
    });
}

pub fn win_title(ui: &mut Ui, title: &str, sub: &str) {
    ui.label(
        RichText::new(title)
            .font(theme::title_bold_font(21.0))
            .color(theme::GOLD),
    );
    if !sub.is_empty() {
        ui.label(
            RichText::new(sub)
                .font(theme::italic_font(14.0))
                .color(theme::PARCHMENT_DIM),
        );
    }
}

/// Fenster mit Bronzerahmen, Titel und Schließrune. Liefert `true`, wenn geschlossen werden soll.
pub fn window(
    ctx: &egui::Context,
    cx: &mut Cx,
    kind: WinKind,
    title: &str,
    sub: &str,
    add: impl FnOnce(&mut Cx, &mut Ui),
) -> bool {
    let mut close = false;
    let art = cx.art;
    // Unterhalb der Oberleiste und oberhalb der Fensterleiste bleiben (Titel, Rand und Abstand eingerechnet).
    let max_h = (ctx.content_rect().height() - 238.0).max(260.0);
    egui::Area::new(Id::unique(("wm_win", kind.id())))
        .order(Order::Middle)
        .movable(true)
        .default_pos(kind.default_pos())
        .constrain(true)
        .show(ctx, |ui| {
            art.panel(
                ui,
                PanelKind::Thin,
                egui::Margin {
                    left: 22,
                    right: 22,
                    top: 16,
                    bottom: 18,
                },
                |ui| {
                    ui.set_width(kind.width().min(ctx.content_rect().width() - 60.0));
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| win_title(ui, title, sub));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            let (r, resp) =
                                ui.allocate_exact_size(Vec2::splat(24.0), Sense::click());
                            art.rune(ui.painter(), r.center(), "x", resp.hovered());
                            ui_common::sfx(cx.engine, ui, &resp);
                            if resp.clicked() {
                                close = true;
                            }
                        });
                    });
                    ui.add_space(2.0);
                    egui::ScrollArea::vertical()
                        .id_salt(("wm_scroll", kind.id()))
                        .max_height(max_h)
                        .min_scrolled_height(max_h)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            add(cx, ui);
                        });
                },
            );
        });
    close
}

/// Realmname mit Farbquadrat.
pub fn realm_chip(ui: &mut Ui, w: &StrategyWorld, r: RealmId) {
    let realm = &w.realms[r as usize];
    ui.horizontal(|ui| {
        let (rc, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
        ui.painter()
            .rect_filled(rc, 0.0, super::mapview::realm_color(realm));
        ui.painter().rect_stroke(
            rc,
            0.0,
            egui::Stroke::new(1.0, Color32::BLACK),
            egui::StrokeKind::Outside,
        );
        ui.label(
            RichText::new(&realm.name)
                .font(theme::body_bold_font(15.0))
                .color(theme::PARCHMENT),
        );
    });
}

/// Farbe nach Wert (0 = rot … 100 = grün) für Zufriedenheit/Loyalität.
pub fn good_col(v: f32) -> Color32 {
    let t = (v / 100.0).clamp(0.0, 1.0);
    if t < 0.5 {
        Color32::from_rgb(200, (60.0 + 150.0 * t) as u8, 50)
    } else {
        Color32::from_rgb((200.0 - 150.0 * (t - 0.5)) as u8, 180, 60)
    }
}

/// Umgekehrte Skala (hoch = schlecht), z. B. Unruhe oder Tyrannei.
pub fn bad_col(v: f32) -> Color32 {
    good_col(100.0 - v)
}

/// Zeigt eine Befehlsrückmeldung als kleine Zeile.
pub fn feedback(ui: &mut Ui, last: &Option<(bool, String)>) {
    if let Some((ok, m)) = last {
        ui.label(
            RichText::new(m)
                .font(theme::italic_font(13.5))
                .color(if *ok {
                    Color32::from_rgb(150, 210, 120)
                } else {
                    Color32::from_rgb(236, 120, 90)
                }),
        );
    }
}
