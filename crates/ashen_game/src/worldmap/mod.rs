//! Weltkarten-Szene der Grand Campaign: Karte von Eldraeth, Reichsverwaltung, Diplomatie, Heere und der
//! Übergang in die Echtzeitschlacht. Die Simulation liegt vollständig in `ashen_strategy`.

pub mod battle;
pub mod battle_ui;
pub mod mapview;
mod panels;
pub mod store;
mod widgets;
mod windows;

use crate::art::ArtLibrary;
use crate::settings::UserSettings;
use crate::ui_art::UiArt;
use ashen_engine::Engine;
use ashen_engine::audio::Intensity;
use ashen_strategy::*;
use egui::TextureId;
use glam::Vec2;
use mapview::{MapMode, MapView};
use std::collections::VecDeque;
use std::path::PathBuf;
use widgets::{Req, UiState, WinKind};

/// Spieltage pro Sekunde je Geschwindigkeitsstufe 1–5.
pub const DAYS_PER_SECOND: [f32; 5] = [0.7, 1.5, 3.0, 6.0, 12.0];

/// Was die Szene vom übergeordneten Spiel möchte.
#[derive(Clone, Debug, PartialEq)]
pub enum WorldAction {
    None,
    MainMenu,
    Quit,
    Load(PathBuf),
    /// Schlacht selbst kommandieren (Id der wartenden Schlacht).
    PlayBattle(BattleId),
}

/// Einblendung am Bildschirmrand.
pub struct Notice {
    pub text: String,
    pub kind: ChronicleKind,
    pub province: Option<ProvId>,
    pub age: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuPage {
    Main,
    Save,
    Load,
    Settings,
}

/// Optionen für Screenshots und Demos.
#[derive(Clone, Debug, Default)]
pub struct WorldAuto {
    pub mode: Option<String>,
    pub open: Vec<String>,
    pub days: u32,
    pub select: Option<String>,
    pub focus: Option<String>,
    pub zoom: Option<f32>,
    pub event_popup: bool,
    pub battle_dialog: bool,
    pub war: bool,
    pub speed: Option<u8>,
}

pub struct WorldMapScene {
    pub world: StrategyWorld,
    pub view: MapView,
    pub mode: MapMode,
    pub selected: Option<ProvId>,
    pub selected_army: Option<ArmyId>,
    pub speed: u8,
    last_speed: u8,
    day_acc: f32,
    pub open: Vec<WinKind>,
    pub st: UiState,
    pub notices: VecDeque<Notice>,
    pub toast: Option<(String, f32)>,
    pub action: WorldAction,
    pub menu_open: bool,
    pub menu_page: MenuPage,
    pub save_name: String,
    pub status: Option<String>,
    pub last_result: Option<(bool, String)>,
    pub hover: Option<ProvId>,
    t: f32,
    pub(crate) sounds: Vec<&'static str>,
    last_autosave_year: i32,
    seen_event: Option<u32>,
    seen_battle: Option<u32>,
    pub victory_ack: bool,
    pub defeat_ack: bool,
    /// Aktueller Rückmeldetext der letzten Schlacht (aus der Echtzeitebene).
    pub battle_note: Option<String>,
}

impl WorldMapScene {
    pub fn new(world: StrategyWorld) -> WorldMapScene {
        let seed = world.options.seed;
        let mut view = MapView::new(seed);
        let me = world.player;
        let capital = me.map(|m| world.realms[m as usize].capital);
        if let Some(c) = capital {
            view.center_on(world.provinces[c as usize].centroid);
            view.zoom = 1.5;
        }
        let year = world.year();
        WorldMapScene {
            world,
            view,
            mode: MapMode::Realms,
            selected: capital,
            selected_army: None,
            speed: 0,
            last_speed: 2,
            day_acc: 0.0,
            open: Vec::new(),
            st: UiState::default(),
            notices: VecDeque::new(),
            toast: None,
            action: WorldAction::None,
            menu_open: false,
            menu_page: MenuPage::Main,
            save_name: "grosse_kampagne".into(),
            status: None,
            last_result: None,
            hover: None,
            t: 0.0,
            sounds: Vec::new(),
            last_autosave_year: year,
            seen_event: None,
            seen_battle: None,
            victory_ack: false,
            defeat_ack: false,
            battle_note: None,
        }
    }

    pub fn me(&self) -> RealmId {
        self.world.player.unwrap_or(0)
    }

    /// Wartet die Szene auf eine Entscheidung (Ereignis, Schlacht, Friedensangebot, Sieg)?
    pub fn modal_open(&self) -> bool {
        self.pending_event().is_some()
            || self.pending_battle().is_some()
            || self.pending_peace().is_some()
            || self.menu_open
            || self.game_over()
    }

    pub fn pending_event(&self) -> Option<&PendingEvent> {
        let me = self.me();
        self.world.pending_events().iter().find(|e| e.realm == me)
    }

    pub fn pending_battle(&self) -> Option<&BattleRequest> {
        let me = self.me();
        self.world.pending_battles().iter().find(|b| {
            b.attackers
                .iter()
                .chain(b.defenders.iter())
                .any(|a| a.realm == Some(me) && a.army != u32::MAX)
        })
    }

    pub fn pending_peace(&self) -> Option<&PeaceOffer> {
        let me = self.me();
        self.world.peace_offers.iter().find(|o| o.to == me)
    }

    /// Sieg errungen oder Reich untergegangen und noch nicht bestätigt.
    pub fn game_over(&self) -> bool {
        (self.world.victory.is_some() && !self.victory_ack)
            || (!self.world.realms[self.me() as usize].alive && !self.defeat_ack)
    }

    pub fn toast(&mut self, s: impl Into<String>) {
        self.toast = Some((s.into(), 4.0));
    }

    pub fn open_window(&mut self, k: WinKind) {
        if !self.open.contains(&k) {
            self.open.push(k);
            self.sounds.push(if k == WinKind::Chronicle {
                "sfx.ui_chronicle_page"
            } else {
                "sfx.ui_window_open"
            });
        } else {
            // nach vorn holen: ans Ende der Liste (wird zuletzt gezeichnet)
            self.open.retain(|x| *x != k);
            self.open.push(k);
        }
    }

    pub fn close_window(&mut self, k: WinKind) {
        self.open.retain(|x| *x != k);
    }

    pub fn toggle_window(&mut self, k: WinKind) {
        if self.open.contains(&k) {
            self.close_window(k);
        } else {
            self.open_window(k);
        }
    }

    /// Musikintensität der Weltkarte: Frieden, Krieg oder Hohlung.
    pub fn music_intensity(&self) -> Intensity {
        let me = self.me();
        let realm = &self.world.realms[me as usize];
        let provs = self.world.provinces_of(me);
        let avg_hollow = if provs.is_empty() {
            0.0
        } else {
            provs
                .iter()
                .map(|p| self.world.provinces[*p as usize].hollowing)
                .sum::<f32>()
                / provs.len() as f32
        };
        if realm.hollow_path >= 2 || avg_hollow >= 30.0 || self.world.endgame.veil_pressure >= 60.0
        {
            Intensity::WorldHollow
        } else if self.world.in_any_war(me) || self.pending_battle().is_some() {
            Intensity::WorldWar
        } else {
            Intensity::WorldPeace
        }
    }

    /// Zeitfortschritt und Ereignisverarbeitung.
    pub fn update(&mut self, engine: &mut Engine, settings: &UserSettings, dt: f32) {
        self.t += dt;
        engine.audio.set_music_intensity(self.music_intensity());
        for s in std::mem::take(&mut self.sounds) {
            engine.audio.play(s);
        }
        if let Some((_, t)) = &mut self.toast {
            *t -= dt;
        }
        if self.toast.as_ref().is_some_and(|(_, t)| *t <= 0.0) {
            self.toast = None;
        }
        for n in self.notices.iter_mut() {
            n.age += dt;
        }
        while self.notices.front().is_some_and(|n| n.age > 14.0) {
            self.notices.pop_front();
        }
        // Neue Entscheidungsfenster melden (Klang).
        let ev = self.pending_event().map(|e| e.id);
        if ev != self.seen_event {
            if ev.is_some() {
                engine.audio.play("sfx.ui_event_popup");
            }
            self.seen_event = ev;
        }
        let bt = self.pending_battle().map(|b| b.id);
        if bt != self.seen_battle {
            if bt.is_some() {
                engine.audio.play("sfx.ui_battle_alert");
            }
            self.seen_battle = bt;
        }
        if self.speed > 0 && !self.modal_open() {
            self.day_acc += dt * DAYS_PER_SECOND[(self.speed as usize - 1).min(4)];
            let mut guard = 0;
            while self.day_acc >= 1.0 && !self.modal_open() && guard < 40 {
                self.day_acc -= 1.0;
                guard += 1;
                self.world.tick_day();
                self.process_events(engine);
                let year = self.world.year();
                if year != self.last_autosave_year {
                    self.last_autosave_year = year;
                    if settings.autosave_minutes > 0.0 {
                        match store::autosave(&self.world) {
                            Ok(_) => {
                                self.status = Some(format!("Automatisch gespeichert (Jahr {year})"))
                            }
                            Err(e) => log::warn!("Autosave der Grand Campaign fehlgeschlagen: {e}"),
                        }
                    }
                }
            }
            if self.modal_open() {
                self.day_acc = 0.0;
            }
        } else if self.modal_open() {
            self.day_acc = 0.0;
        }
        // Ereignisse auch im Stillstand abholen (z. B. nach Befehlen).
        self.process_events(engine);
    }

    fn push_notice(&mut self, text: String, kind: ChronicleKind, province: Option<ProvId>) {
        self.notices.push_back(Notice {
            text,
            kind,
            province,
            age: 0.0,
        });
        while self.notices.len() > 6 {
            self.notices.pop_front();
        }
    }

    pub fn process_events(&mut self, engine: &mut Engine) {
        let me = self.me();
        for e in self.world.drain_events() {
            match e {
                StrategyEvent::Notice {
                    realm,
                    kind,
                    text,
                    province,
                } => {
                    if realm.is_none_or(|r| r == me) {
                        self.push_notice(text, kind, province);
                    }
                }
                StrategyEvent::EventWindow { .. } => {}
                StrategyEvent::BattleRequested { id } => {
                    if let Some(b) = self.world.pending_battles().iter().find(|b| b.id == id) {
                        let t = format!("Schlacht bei {}!", b.province_name);
                        let p = b.province;
                        self.push_notice(t, ChronicleKind::Battle, Some(p));
                    }
                }
                StrategyEvent::BattleResolved {
                    province,
                    winner_realm,
                } => {
                    if winner_realm == Some(me)
                        || self
                            .world
                            .armies
                            .iter()
                            .any(|a| a.owner == Some(me) && a.location == province)
                    {
                        let t = match winner_realm {
                            Some(w) if w == me => {
                                format!("Sieg bei {}.", self.world.pname(province))
                            }
                            Some(w) => format!(
                                "{} siegt bei {}.",
                                self.world.rname(w),
                                self.world.pname(province)
                            ),
                            None => {
                                format!("Schlacht bei {} ohne Sieger.", self.world.pname(province))
                            }
                        };
                        self.push_notice(t, ChronicleKind::Battle, Some(province));
                    }
                }
                StrategyEvent::RulerChanged { realm, ruler } => {
                    if realm == me {
                        let t = format!("{} besteigt den Thron.", self.world.full_name(ruler));
                        self.push_notice(t, ChronicleKind::Succession, None);
                        engine.audio.play("sfx.ui_ruler_changed");
                    }
                }
                StrategyEvent::WarDeclared { war } => {
                    if let Some(w) = self.world.wars.iter().find(|w| w.id == war)
                        && (w.attackers.contains(&me) || w.defenders.contains(&me))
                    {
                        let t = format!(
                            "Krieg: {} gegen {}!",
                            self.world.rname(w.leader_a),
                            self.world.rname(w.leader_d)
                        );
                        self.push_notice(t, ChronicleKind::War, None);
                        engine.audio.play("sfx.ui_war_declared");
                    }
                }
                StrategyEvent::PeaceMade { war } => {
                    if let Some(w) = self.world.wars.iter().find(|w| w.id == war)
                        && (w.attackers.contains(&me) || w.defenders.contains(&me))
                    {
                        self.push_notice(
                            "Der Krieg ist beendet – Friede.".into(),
                            ChronicleKind::Peace,
                            None,
                        );
                        engine.audio.play("sfx.ui_peace_made");
                    }
                }
                StrategyEvent::RealmFell { realm } => {
                    let t = format!("{} ist untergegangen.", self.world.rname(realm));
                    self.push_notice(t, ChronicleKind::Realm, None);
                    engine.audio.play("sfx.ui_realm_fell");
                }
                StrategyEvent::RealmFounded { realm } => {
                    let t = format!("{} wurde gegründet.", self.world.rname(realm));
                    self.push_notice(t, ChronicleKind::Founding, None);
                }
                StrategyEvent::ProvinceChanged { province, old, new } => {
                    if old == Some(me) || new == Some(me) {
                        let t = if new == Some(me) {
                            format!("{} ist nun Euer.", self.world.pname(province))
                        } else {
                            format!("{} ist verloren.", self.world.pname(province))
                        };
                        self.push_notice(t, ChronicleKind::War, Some(province));
                    }
                }
                StrategyEvent::Victory { realm, kind } => {
                    let t = format!("{}: {}!", self.world.rname(realm), kind.name());
                    self.push_notice(t, ChronicleKind::Victory, None);
                    engine.audio.play_sting("victory_sting");
                }
            }
        }
    }

    /// Wendet die gesammelten Befehle an und verarbeitet Rückmeldung und Klang.
    pub(crate) fn run_commands(&mut self, engine: &mut Engine, cmds: Vec<StrategyCommand>) {
        for c in cmds {
            let tag = match &c {
                StrategyCommand::MoveArmy { .. } => "move",
                StrategyCommand::Recruit { .. } | StrategyCommand::HireMercenaries(_) => "recruit",
                StrategyCommand::Build { .. } | StrategyCommand::UpgradeFortification(_) => "build",
                StrategyCommand::Diplomacy {
                    action: DiplomaticAction::Propose(_) | DiplomaticAction::OfferVassalage,
                    ..
                } => "treaty",
                StrategyCommand::ArrangeMarriage { .. } => "marriage",
                _ => "",
            };
            let r = self.world.execute(c);
            if r.ok {
                match tag {
                    "move" => engine.audio.play("sfx.map_army_move"),
                    "recruit" => engine.audio.play("sfx.map_recruit"),
                    "build" => engine.audio.play("sfx.build_complete"),
                    "treaty" | "marriage" => engine.audio.play("sfx.ui_treaty_signed"),
                    _ => false,
                };
            } else {
                engine.audio.play("sfx.ui_error");
            }
            self.status = Some(r.message.clone());
            self.last_result = Some((r.ok, r.message));
        }
        self.process_events(engine);
    }

    pub(crate) fn handle_reqs(&mut self, reqs: Vec<Req>) {
        for r in reqs {
            match r {
                Req::Open(k) => self.open_window(k),
                Req::Close(k) => self.close_window(k),
                Req::Select(p) => {
                    self.selected = Some(p);
                }
                Req::Center(p) => {
                    let c = self.world.provinces[p as usize].centroid;
                    self.view.center_on(c);
                }
                Req::SelectArmy(a) => self.selected_army = Some(a),
                Req::AutoBattle(id) => match self.world.auto_resolve_pending(id) {
                    Ok(()) => {
                        self.battle_note = Some("Die Schlacht wurde automatisch aufgelöst.".into());
                        self.last_result = Some((true, "Schlacht automatisch aufgelöst".into()));
                    }
                    Err(e) => self.last_result = Some((false, e)),
                },
                Req::PlayBattle(id) => self.action = WorldAction::PlayBattle(id),
            }
        }
    }

    /// Wendet `WorldAuto` an (Screenshots).
    pub fn apply_auto(&mut self, auto: &WorldAuto) {
        if auto.days > 0 {
            self.world.advance_days(auto.days);
            let _ = self.world.drain_events();
        }
        if let Some(m) = auto.mode.as_deref().and_then(MapMode::from_id) {
            self.mode = m;
        }
        if let Some(sel) = &auto.select {
            let found = self.world.prov_by_key(sel).or_else(|| {
                self.world
                    .provinces
                    .iter()
                    .find(|p| p.name.eq_ignore_ascii_case(sel))
                    .map(|p| p.id)
            });
            if let Some(p) = found {
                self.selected = Some(p);
                self.view
                    .center_on(self.world.provinces[p as usize].centroid);
            }
        }
        if let Some(f) = &auto.focus {
            let found = self.world.prov_by_key(f).or_else(|| {
                self.world
                    .provinces
                    .iter()
                    .find(|p| p.name.eq_ignore_ascii_case(f))
                    .map(|p| p.id)
            });
            if let Some(p) = found {
                self.view
                    .center_on(self.world.provinces[p as usize].centroid);
            } else if let Some((x, y)) = f.split_once(',') {
                self.view.center_on(Vec2::new(
                    x.parse().unwrap_or(500.0),
                    y.parse().unwrap_or(350.0),
                ));
            }
        }
        if let Some(z) = auto.zoom {
            self.view.zoom = z;
        }
        for w in &auto.open {
            if let Some(k) = WinKind::from_id(w) {
                self.open_window(k);
            }
        }
        if let Some(s) = auto.speed {
            self.speed = s.min(5);
        }
        self.sounds.clear();
        let me = self.me();
        if auto.war {
            // Krieg gegen die Aschenlegion/Hohlen ist immer möglich; für das Bild genügt ein Schlachtfeld in Grenznähe.
            if let Some(h) = self.world.realm_by_key("hollowed") {
                let loc = self.world.realms[me as usize].capital;
                let _ = self.world.new_army(
                    ArmyKind::Realm,
                    Some(h),
                    loc,
                    vec![Regiment {
                        unit: "hollow_thrall".into(),
                        count: 40,
                        vet: 0,
                        hp: 1.0,
                    }],
                    "Hohle Vorhut",
                );
                let _ = self.world.new_army(
                    ArmyKind::Realm,
                    Some(me),
                    loc,
                    vec![
                        Regiment {
                            unit: "swordsman".into(),
                            count: 30,
                            vet: 1,
                            hp: 1.0,
                        },
                        Regiment {
                            unit: "archer".into(),
                            count: 20,
                            vet: 0,
                            hp: 1.0,
                        },
                    ],
                    "Wächter der Aschmark",
                );
            }
        }
        if auto.event_popup {
            let me = self.me();
            let prov = self.world.realms[me as usize].capital;
            let _ = self.world.force_event("famine_province", me, Some(prov));
        }
        if auto.battle_dialog {
            let hollow = self.world.realm_by_key("hollowed");
            if let Some(h) = hollow {
                let loc = self
                    .selected
                    .unwrap_or(self.world.realms[me as usize].capital);
                let _ = self.world.new_army(
                    ArmyKind::Realm,
                    Some(h),
                    loc,
                    vec![
                        Regiment {
                            unit: "hollow_thrall".into(),
                            count: 60,
                            vet: 0,
                            hp: 1.0,
                        },
                        Regiment {
                            unit: "grave_brute".into(),
                            count: 4,
                            vet: 0,
                            hp: 1.0,
                        },
                    ],
                    "Hohlschar",
                );
                let _ = self.world.new_army(
                    ArmyKind::Realm,
                    Some(me),
                    loc,
                    vec![
                        Regiment {
                            unit: "swordsman".into(),
                            count: 50,
                            vet: 1,
                            hp: 1.0,
                        },
                        Regiment {
                            unit: "archer".into(),
                            count: 30,
                            vet: 0,
                            hp: 1.0,
                        },
                        Regiment {
                            unit: "knight".into(),
                            count: 8,
                            vet: 0,
                            hp: 1.0,
                        },
                    ],
                    "Wächter der Aschmark",
                );
                self.world.tick_day();
                let _ = self.world.drain_events();
            }
        }
    }

    /// Zeichnet die gesamte Weltkarten-Oberfläche.
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        engine: &mut Engine,
        lib: &ArtLibrary,
        tex: TextureId,
        art: &UiArt,
        settings: &mut UserSettings,
    ) {
        panels::draw(self, ui, engine, lib, tex, art, settings);
    }
}
