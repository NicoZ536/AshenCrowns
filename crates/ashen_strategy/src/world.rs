//! Der vollständige Zustand der Weltkarte (`StrategyWorld`) und der Tagestakt.

use crate::chronicle::{Chronicle, ChronicleKind, Entry};
use crate::data::StrategyData;
use crate::endgame::{VictoryKind, VictoryRecord};
use crate::military::BattleRequest;
use crate::model::*;
use crate::types::*;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Wer der Spieler ist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerStart {
    /// Grand Campaign: Isolde Raventhorn, Wächterin der Aschmark, Vasallin der Glutkrone.
    AshmarchWarden,
    /// Sandbox: ein beliebiges Reich der Startwelt (Schlüssel, z. B. `skarn_clans`).
    Realm(String),
    /// Kein Spielerreich (reine KI-Simulation, z. B. für Tests).
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DifficultyMods {
    pub ai_yield: f32,
    pub player_yield: f32,
    pub ai_aggression: f32,
    pub hollow_strength: f32,
    pub veil_rate: f32,
    /// Ereignisse pro Jahr (Multiplikator auf die Auslösewahrscheinlichkeit).
    pub event_density: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Pilgrim,
    Vassal,
    Lord,
    HighKing,
    Ashfall,
    Custom(DifficultyMods),
}

impl Difficulty {
    pub fn mods(self) -> DifficultyMods {
        match self {
            Difficulty::Pilgrim => DifficultyMods { ai_yield: 1.0, player_yield: 1.25, ai_aggression: 0.6, hollow_strength: 0.6, veil_rate: 0.8, event_density: 1.0 },
            Difficulty::Vassal => DifficultyMods { ai_yield: 1.0, player_yield: 1.1, ai_aggression: 0.85, hollow_strength: 0.8, veil_rate: 0.9, event_density: 1.0 },
            Difficulty::Lord => DifficultyMods { ai_yield: 1.0, player_yield: 1.0, ai_aggression: 1.0, hollow_strength: 1.0, veil_rate: 1.0, event_density: 1.0 },
            Difficulty::HighKing => DifficultyMods { ai_yield: 1.15, player_yield: 1.0, ai_aggression: 1.25, hollow_strength: 1.25, veil_rate: 1.2, event_density: 1.0 },
            Difficulty::Ashfall => DifficultyMods { ai_yield: 1.3, player_yield: 1.0, ai_aggression: 1.5, hollow_strength: 1.5, veil_rate: 1.4, event_density: 1.2 },
            Difficulty::Custom(m) => m,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Difficulty::Pilgrim => "Pilger",
            Difficulty::Vassal => "Vasall",
            Difficulty::Lord => "Lehnsherr",
            Difficulty::HighKing => "Hochkönig",
            Difficulty::Ashfall => "Aschenfall",
            Difficulty::Custom(_) => "Benutzerdefiniert",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Campaign,
    Sandbox,
    Challenge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Challenge {
    /// „Kein Blut“: Handelssieg ohne Kriegserklärung des Spielers.
    NoBlood,
    /// „Die Leibeigene“: Spiel als machtloses Kadettenhaus.
    CadetHouse,
    /// Überleben: festgelegte Zeitspanne überstehen.
    Survival(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleMode {
    /// Alle Schlachten werden automatisch aufgelöst.
    AutoResolveAll,
    /// Schlachten mit Spielerbeteiligung erzeugen einen `BattleRequest` und warten auf ein `BattleResult`.
    PlayerBattlesRequested,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleSet {
    pub mode: GameMode,
    pub victories: Vec<VictoryKind>,
    pub challenge: Option<Challenge>,
    pub ironman: bool,
    pub permadeath: bool,
    pub battle_mode: BattleMode,
    /// Ämter und Provinzbau des Spielers übernimmt bei Bedarf die KI-Verwaltung.
    pub auto_govern_player: bool,
    pub strict_time: bool,
}

impl RuleSet {
    pub fn campaign() -> RuleSet {
        RuleSet {
            mode: GameMode::Campaign,
            victories: vec![VictoryKind::CrownOfEldraeth, VictoryKind::HollowKingFalls, VictoryKind::SealOfTheVeil, VictoryKind::TradeDominion, VictoryKind::FaithVictory, VictoryKind::DynasticLegend, VictoryKind::HollowCrown],
            challenge: None,
            ironman: false,
            permadeath: true,
            battle_mode: BattleMode::AutoResolveAll,
            auto_govern_player: true,
            strict_time: false,
        }
    }
    pub fn sandbox(victories: Vec<VictoryKind>) -> RuleSet {
        RuleSet { mode: GameMode::Sandbox, victories, ..RuleSet::campaign() }
    }
    pub fn challenge(c: Challenge) -> RuleSet {
        let victories = match c {
            Challenge::NoBlood => vec![VictoryKind::TradeDominion],
            Challenge::CadetHouse => vec![VictoryKind::DynasticLegend],
            Challenge::Survival(_) => vec![VictoryKind::Survival],
        };
        RuleSet { mode: GameMode::Challenge, victories, challenge: Some(c), ..RuleSet::campaign() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyOptions {
    pub seed: u64,
    pub player: PlayerStart,
    pub difficulty: Difficulty,
    pub rules: RuleSet,
}

impl StrategyOptions {
    /// Standard der Grand Campaign: Isolde als Wächterin der Aschmark.
    pub fn campaign(seed: u64) -> StrategyOptions {
        StrategyOptions { seed, player: PlayerStart::AshmarchWarden, difficulty: Difficulty::Lord, rules: RuleSet::campaign() }
    }
    /// Reine KI-Simulation ohne Spielerreich.
    pub fn observer(seed: u64) -> StrategyOptions {
        StrategyOptions { seed, player: PlayerStart::None, difficulty: Difficulty::Lord, rules: RuleSet::campaign() }
    }
}

/// Ereignisfenster, das auf eine Entscheidung des Spielers wartet.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingEvent {
    pub id: u32,
    pub event_id: String,
    pub realm: RealmId,
    pub province: Option<ProvId>,
    pub title: String,
    pub text: String,
    /// (Options-ID, Text, bezahlbar)
    pub options: Vec<(String, String, bool)>,
    pub day: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventState {
    /// Abklingzeiten (Schlüssel `event@scope`) → Tag, ab dem erneut möglich.
    pub cooldowns: BTreeMap<String, u32>,
    pub pending: Vec<PendingEvent>,
    pub next_id: u32,
    pub fired: u32,
}

/// Ausgabeereignisse für Oberfläche und Spiel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrategyEvent {
    Notice { realm: Option<RealmId>, kind: ChronicleKind, text: String, province: Option<ProvId> },
    EventWindow { pending: u32 },
    BattleRequested { id: BattleId },
    BattleResolved { province: ProvId, winner_realm: Option<RealmId> },
    RulerChanged { realm: RealmId, ruler: CharId },
    WarDeclared { war: WarId },
    PeaceMade { war: WarId },
    RealmFell { realm: RealmId },
    RealmFounded { realm: RealmId },
    ProvinceChanged { province: ProvId, old: Option<RealmId>, new: Option<RealmId> },
    Victory { realm: RealmId, kind: VictoryKind },
}

fn default_data() -> Arc<StrategyData> {
    StrategyData::builtin()
}

#[derive(Serialize, Deserialize)]
pub struct StrategyWorld {
    pub options: StrategyOptions,
    /// Vergangene Tage seit dem 1. Frostmond 897 n.F.
    pub day: u32,
    pub rng: ChaCha8Rng,
    pub regions: Vec<Region>,
    pub provinces: Vec<Province>,
    pub realms: Vec<Realm>,
    pub houses: Vec<NobleHouse>,
    pub dynasties: Vec<Dynasty>,
    pub chars: Vec<Character>,
    pub armies: Vec<Army>,
    pub next_army: ArmyId,
    pub wars: Vec<War>,
    pub next_war: WarId,
    pub treaties: Vec<Treaty>,
    pub next_treaty: u32,
    pub opinion_mods: BTreeMap<(RealmId, RealmId), Vec<OpinionMod>>,
    pub coalitions: Vec<Coalition>,
    pub next_coalition: u32,
    pub factions: Vec<PoliticalFaction>,
    pub next_faction: u32,
    pub plots: Vec<Plot>,
    pub next_plot: PlotId,
    pub markets: Vec<Market>,
    pub routes: Vec<TradeRoute>,
    pub neutrals: Vec<NeutralGroup>,
    pub next_neutral: u32,
    pub mercs: Vec<MercCompany>,
    pub bosses: Vec<Boss>,
    pub mage_orders: Vec<MageOrder>,
    pub artifacts: Vec<ArtifactState>,
    pub chronicle: Chronicle,
    pub endgame: EndgameState,
    pub events: EventState,
    pub victory: Option<VictoryRecord>,
    pub battles: Vec<BattleRequest>,
    pub next_battle: BattleId,
    pub player: Option<RealmId>,
    pub out: Vec<StrategyEvent>,
    /// Kriegsstatistik für Tests/Anzeige.
    pub stats: WorldStats,
    pub peace_offers: Vec<crate::ai::PeaceOffer>,
    pub next_peace_offer: u32,
    /// Verwundete, die nach 30 Tagen in die Bevölkerung zurückkehren: (Tag, Provinz, Anzahl).
    pub returning: Vec<(u32, ProvId, f32)>,
    /// Eifer der sechs Religionen (0–100).
    pub zeal: [f32; 6],
    /// Zorn der Wurzelmutter über Abholzung.
    pub root_anger: f32,
    /// Laufende Schrein-Wiederherstellungen: (Provinz, Reich, Fertigstellungstag).
    pub shrine_quests: Vec<(ProvId, RealmId, u32)>,
    pub league_chest: f32,
    pub veil_break_years: u32,
    /// Monate, die ein Reich die Handelsherrschaft hält.
    pub trade_hold: BTreeMap<RealmId, u32>,
    /// Monate, die ein Reich die Voraussetzungen der Krone von Eldraeth erfüllt.
    #[serde(default)]
    pub crown_hold: BTreeMap<RealmId, u32>,
    #[serde(skip, default = "default_data")]
    pub data: Arc<StrategyData>,
    #[serde(skip)]
    pub realm_provs: Vec<Vec<ProvId>>,
    #[serde(skip)]
    pub realm_dist: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorldStats {
    pub wars_started: u32,
    pub wars_ended: u32,
    pub births: u32,
    pub deaths: u32,
    pub successions: u32,
    pub regencies: u32,
    pub realms_founded: u32,
    pub realms_fallen: u32,
    pub dynasties_extinct: u32,
    pub plots_started: u32,
    pub plots_succeeded: u32,
    pub revolts: u32,
    pub battles: u32,
    pub sieges: u32,
    pub events_fired: u32,
    pub coalitions: u32,
    pub plagues: u32,
    pub famines: u32,
    /// Diagnose: Bevölkerungsänderung je Systemphase (kumuliert).
    pub pop_flow: BTreeMap<String, f32>,
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Zeit
    // ------------------------------------------------------------------------------------
    pub fn date(&self) -> Date {
        Date::from_day(self.day)
    }
    pub fn year(&self) -> i32 {
        START_YEAR + (self.day / DAYS_PER_YEAR) as i32
    }
    pub fn month(&self) -> u32 {
        (self.day / DAYS_PER_MONTH) % 12
    }
    pub fn season(&self) -> Season {
        Season::from_month(self.month())
    }

    // ------------------------------------------------------------------------------------
    // Zufall
    // ------------------------------------------------------------------------------------
    pub fn rf(&mut self) -> f32 {
        self.rng.random::<f32>()
    }
    pub fn chance(&mut self, p: f32) -> bool {
        p > 0.0 && self.rng.random::<f32>() < p
    }
    pub fn ri(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo {
            return lo;
        }
        self.rng.random_range(lo..=hi)
    }
    pub fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        let i = self.rng.random_range(0..v.len());
        &v[i]
    }

    // ------------------------------------------------------------------------------------
    // Namen / Zugriff
    // ------------------------------------------------------------------------------------
    pub fn pname(&self, p: ProvId) -> String {
        self.provinces.get(p as usize).map(|x| x.name.clone()).unwrap_or_default()
    }
    pub fn rname(&self, r: RealmId) -> String {
        self.realms.get(r as usize).map(|x| x.name.clone()).unwrap_or_default()
    }
    pub fn cname(&self, c: CharId) -> String {
        self.chars.get(c as usize).map(|x| x.name.clone()).unwrap_or_default()
    }
    /// Voller Name inkl. Dynastie.
    pub fn full_name(&self, c: CharId) -> String {
        let ch = &self.chars[c as usize];
        match self.dynasties.get(ch.dynasty as usize) {
            Some(d) if !ch.name.contains(&d.name) => format!("{} {}", ch.name, d.name),
            _ => ch.name.clone(),
        }
    }
    pub fn ch(&self, c: CharId) -> &Character {
        &self.chars[c as usize]
    }
    pub fn realm(&self, r: RealmId) -> &Realm {
        &self.realms[r as usize]
    }
    pub fn ruler_of(&self, r: RealmId) -> &Character {
        &self.chars[self.realms[r as usize].ruler as usize]
    }
    pub fn prov_by_key(&self, key: &str) -> Option<ProvId> {
        self.provinces.iter().position(|p| p.key == key).map(|i| i as ProvId)
    }
    pub fn realm_by_key(&self, key: &str) -> Option<RealmId> {
        self.realms.iter().position(|r| r.key == key).map(|i| i as RealmId)
    }
    pub fn is_player(&self, r: RealmId) -> bool {
        self.player == Some(r)
    }
    /// Provinzen eines Reiches (Cache, monatlich und bei Besitzwechsel erneuert).
    pub fn provinces_of(&self, r: RealmId) -> &[ProvId] {
        self.realm_provs.get(r as usize).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn rebuild_caches(&mut self) {
        let n = self.realms.len();
        self.realm_provs = vec![Vec::new(); n];
        for p in &self.provinces {
            if let Some(o) = p.owner {
                self.realm_provs[o as usize].push(p.id);
            }
        }
        // Entfernungsmatrix (Provinzsprünge) zwischen Reichen: 0 = gleich, 1 = Nachbar, 2, 3 = weit
        let mut dist = vec![vec![9u8; n]; n];
        for (a, row) in dist.iter_mut().enumerate() {
            row[a] = 0;
        }
        for p in &self.provinces {
            let Some(a) = p.owner else { continue };
            for l in &p.neighbors {
                if let Some(b) = self.provinces[l.to as usize].owner
                    && a != b {
                        dist[a as usize][b as usize] = 1;
                    }
            }
        }
        for a in 0..n {
            for b in 0..n {
                if dist[a][b] != 1 {
                    continue;
                }
                for c in 0..n {
                    if dist[b][c] == 1 && dist[a][c] > 2 {
                        dist[a][c] = 2;
                    }
                }
            }
        }
        for row in dist.iter_mut() {
            for v in row.iter_mut() {
                if *v == 9 {
                    *v = 3;
                }
            }
        }
        self.realm_dist = dist;
    }
    pub fn realm_distance(&self, a: RealmId, b: RealmId) -> u8 {
        self.realm_dist.get(a as usize).and_then(|r| r.get(b as usize)).copied().unwrap_or(3)
    }
    pub fn living_realms(&self) -> impl Iterator<Item = &Realm> {
        self.realms.iter().filter(|r| r.alive)
    }
    pub fn at_war(&self, a: RealmId, b: RealmId) -> bool {
        if a == b {
            return false;
        }
        let (ka, kb) = (crate::diplomacy::is_antagonist(self.realms[a as usize].kind), crate::diplomacy::is_antagonist(self.realms[b as usize].kind));
        if ka != kb {
            return true;
        }
        if ka && kb {
            return false;
        }
        self.wars.iter().any(|w| w.active && ((w.attackers.contains(&a) && w.defenders.contains(&b)) || (w.attackers.contains(&b) && w.defenders.contains(&a))))
    }
    pub fn in_any_war(&self, r: RealmId) -> bool {
        self.wars.iter().any(|w| w.active && (w.attackers.contains(&r) || w.defenders.contains(&r)))
    }
    pub fn wars_of(&self, r: RealmId) -> Vec<WarId> {
        self.wars.iter().filter(|w| w.active && (w.attackers.contains(&r) || w.defenders.contains(&r))).map(|w| w.id).collect()
    }

    // ------------------------------------------------------------------------------------
    // Chronik und Ausgaben
    // ------------------------------------------------------------------------------------
    pub fn chron(&mut self, e: Entry) -> u64 {
        let realm = e.realm;
        let kind = e.kind;
        let prov = e.province;
        let mag = e.magnitude;
        let id = self.chronicle.push(self.day, e);
        if mag >= 25
            && let Some(entry) = self.chronicle.entries.last() {
                let text = entry.text.clone();
                self.emit(StrategyEvent::Notice { realm, kind, text, province: prov });
            }
        id
    }
    pub fn emit(&mut self, e: StrategyEvent) {
        if self.out.len() > 4000 {
            self.out.drain(0..2000);
        }
        self.out.push(e);
    }
    /// Entnimmt alle aufgelaufenen Ausgabeereignisse.
    pub fn drain_events(&mut self) -> Vec<StrategyEvent> {
        std::mem::take(&mut self.out)
    }

    // ------------------------------------------------------------------------------------
    // Takt
    // ------------------------------------------------------------------------------------
    /// Ein Welttag: Armeen, Belagerungen, Schlachten; am Monatsanfang Monatsverarbeitung, am Jahresanfang Jahresverarbeitung.
    pub fn tick_day(&mut self) {
        if self.victory.is_some() && self.options.rules.ironman {
            return;
        }
        self.day += 1;
        self.daily_military();
        if self.day.is_multiple_of(DAYS_PER_MONTH) {
            self.monthly();
        }
        if self.day.is_multiple_of(DAYS_PER_YEAR) {
            self.yearly();
        }
    }

    /// Mehrere Tage am Stück.
    pub fn advance_days(&mut self, n: u32) {
        for _ in 0..n {
            self.tick_day();
        }
    }

    /// Spielt bis zum Jahresanfang des Zieljahres.
    pub fn advance_to_year(&mut self, year: i32) {
        while self.year() < year {
            self.tick_day();
        }
    }

    fn monthly(&mut self) {
        self.rebuild_caches();
        macro_rules! phase {
            ($name:expr, $call:expr) => {{
                let before: f32 = self.provinces.iter().map(|p| p.population()).sum();
                $call;
                let after: f32 = self.provinces.iter().map(|p| p.population()).sum();
                *self.stats.pop_flow.entry($name.to_string()).or_insert(0.0) += after - before;
            }};
        }
        phase!("weather", self.weather_month());
        phase!("economy", self.economy_month());
        phase!("population", self.population_month());
        phase!("order", self.order_month());
        phase!("religion", self.religion_month());
        phase!("magic", self.magic_month());
        phase!("characters", self.characters_month());
        phase!("realm", self.realm_month());
        phase!("politics", self.politics_month());
        phase!("plots", self.plots_month());
        phase!("diplomacy", self.diplomacy_month());
        phase!("war", self.war_month());
        phase!("neutral", self.neutral_month());
        phase!("ai", self.ai_month());
        phase!("events", self.events_month());
        phase!("endgame", self.endgame_month());
        self.victory_check();
        self.cleanup_month();
    }

    fn yearly(&mut self) {
        self.characters_year();
        self.dynasties_year();
        self.realm_year();
        self.economy_year();
        self.bosses_year();
        self.ai_year();
        self.endgame_year();
    }

    /// Hält Listen klein (alte Meinungsmodifikatoren, beendete Kriege, Armeen ohne Truppen).
    fn cleanup_month(&mut self) {
        let day = self.day;
        for v in self.opinion_mods.values_mut() {
            v.retain(|m| m.expires.is_none_or(|e| e > day));
        }
        self.opinion_mods.retain(|_, v| !v.is_empty());
        self.treaties.retain(|t| t.end.is_none_or(|e| e > day));
        // Beendete Kriege nach zehn Jahren verwerfen (Platz sparen), Statistik bleibt in WorldStats.
        let keep_from = day.saturating_sub(DAYS_PER_YEAR * 10);
        self.wars.retain(|w| w.active || w.start + DAYS_PER_YEAR > keep_from);
        self.plots.retain(|p| !p.done || day.saturating_sub(p.started) < DAYS_PER_YEAR);
        self.coalitions.retain(|c| c.active);
        for ch in self.chars.iter_mut() {
            if ch.alive() {
                ch.memories.retain(|m| m.expires > day);
            }
        }
        for r in self.realms.iter_mut() {
            r.memory.retain(|m| day.saturating_sub(m.day) < DAYS_PER_YEAR * 30);
            r.recent_conquests.retain(|d| day.saturating_sub(*d) < DAYS_PER_YEAR * 30);
        }
        // Bei Tod erloschene Spielerreiche: Spieler zum Nachfolgerreich oder keiner
        if let Some(p) = self.player
            && !self.realms[p as usize].alive {
                let heir = self.realms[p as usize].vassal_of.filter(|o| self.realms[*o as usize].alive);
                self.player = heir;
            }
    }

    /// Prüfsumme des gesamten Zustands (Determinismus-Test).
    pub fn state_hash(&self) -> u64 {
        let bytes = bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap_or_default();
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        h
    }

    pub fn new_rng(seed: u64) -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(seed)
    }
}
