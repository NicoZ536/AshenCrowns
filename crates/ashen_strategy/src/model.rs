//! Datenmodell der Weltkarten-Simulation (alles serialisierbar, alles IDs/Indizes, keine Referenzen).

use crate::catalog::{ArtifactEffect, Trait};
use crate::goods::Good;
use crate::types::*;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------------------------
// Karte und Provinzen
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkKind {
    Land,
    Sea,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Link {
    pub to: ProvId,
    pub kind: LinkKind,
    /// Entfernung in Kartenkoordinaten.
    pub dist: f32,
}

/// Region der Kanon-Welt (handgefertigt; Provinzen variieren je Seed).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub provinces: Vec<ProvId>,
    /// Marktgebiet (None = kein Markt, Schleiernarbe).
    pub market: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct StratumPop {
    pub children: f32,
    pub adults: f32,
    pub old: f32,
    /// Zufriedenheit 0–100.
    pub satisfaction: f32,
    /// Erfüllungsgrade der Bedürfnisse (0–1).
    pub basic: f32,
    pub comfort: f32,
    pub luxury: f32,
}

impl StratumPop {
    pub fn total(&self) -> f32 {
        self.children + self.adults + self.old
    }
    /// Arbeitskraft (Alte arbeiten zu 30 %).
    pub fn labor(&self) -> f32 {
        self.adults + self.old * 0.3
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plague {
    pub kind: u8,
    pub start_day: u32,
    pub severity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Siege {
    pub besieger: RealmId,
    pub army: ArmyId,
    /// Fortschritt in Prozent (0–100).
    pub progress: f32,
    pub started: u32,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct ChainMemory {
    pub drought: Option<u64>,
    pub famine: Option<u64>,
    pub price_spike: Option<u64>,
    pub unrest: Option<u64>,
    pub revolt: Option<u64>,
    pub plague: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    pub id: ProvId,
    pub key: String,
    pub name: String,
    pub region: u8,
    pub centroid: Vec2,
    pub polygon: Vec<Vec2>,
    pub neighbors: Vec<Link>,
    pub biome: Biome,
    pub coastal: bool,
    pub deposits: Vec<(Good, u8)>,
    pub fertility: f32,
    /// Niederschlag als Anteil des Klimamittels (0–2).
    pub rainfall: f32,
    pub dry_months: u8,
    pub settlement: SettlementLevel,
    /// (Gebäudeindex in `goods::BUILDINGS`, Anzahl)
    pub buildings: Vec<(u8, u16)>,
    pub fortification: u8,
    pub garrison: u32,
    pub pop: [StratumPop; 8],
    pub housing: f32,
    pub culture: Culture,
    pub faith: [f32; 6],
    pub owner: Option<RealmId>,
    /// Besatzer (wenn abweichend vom Besitzer).
    pub controller: Option<RealmId>,
    pub fief: Option<HouseId>,
    /// Von Rebellen gehalten (Besitzer bleibt formal erhalten).
    pub rebel_held: bool,
    pub unrest: f32,
    pub hollowing: f32,
    pub veil: f32,
    pub devastation: f32,
    pub disease: f32,
    pub hygiene: f32,
    pub plague: Option<Plague>,
    /// Spannungsachsen: Leibeigene↔Adel, Bürger↔Adel, Klerus↔Gelehrte, Krieger↔Bürger, Einheimische↔Flüchtlinge.
    pub tension: [f32; 5],
    pub stock: Vec<f32>,
    pub unmet: Vec<f32>,
    pub produced: Vec<f32>,
    pub food_ratio: f32,
    /// Nahrungsvielfalt-Bonus (Zufriedenheit).
    pub variety: f32,
    pub harvest: f32,
    pub shrine: u8,
    pub hollow_force: f32,
    pub holy_site: Option<Religion>,
    pub god_shrine: Option<u8>,
    pub siege: Option<Siege>,
    /// Zustände (Name → Ablauftag).
    pub states: BTreeMap<String, u32>,
    pub chain: ChainMemory,
    pub conversion: f32,
    pub refugees: f32,
    pub forest_cover: f32,
    /// Letzter Besitzerwechsel (für Rückeroberungsziele).
    pub original_owner: Option<RealmId>,
    pub captured_day: u32,
    pub artifact: Option<u8>,
    /// Monatliche Statistik: Umsatz am Markt (Gold).
    pub turnover: f32,
    pub trade_post: bool,
}

impl Province {
    pub fn population(&self) -> f32 {
        self.pop.iter().map(|s| s.total()).sum()
    }
    pub fn adults(&self) -> f32 {
        self.pop.iter().map(|s| s.adults).sum()
    }
    pub fn avg_satisfaction(&self) -> f32 {
        let t = self.population().max(1.0);
        self.pop.iter().map(|s| s.satisfaction * s.total()).sum::<f32>() / t
    }
    pub fn faith_majority(&self) -> Religion {
        let mut best = 0;
        for i in 1..6 {
            if self.faith[i] > self.faith[best] {
                best = i;
            }
        }
        RELIGIONS[best]
    }
    pub fn building_count(&self, idx: usize) -> u16 {
        self.buildings.iter().find(|(i, _)| *i as usize == idx).map(|(_, c)| *c).unwrap_or(0)
    }
    pub fn add_building(&mut self, idx: usize, n: u16) {
        if let Some(e) = self.buildings.iter_mut().find(|(i, _)| *i as usize == idx) {
            e.1 += n;
        } else {
            self.buildings.push((idx as u8, n));
        }
    }
    pub fn deposit(&self, g: Good) -> u8 {
        self.deposits.iter().find(|(x, _)| *x == g).map(|(_, r)| *r).unwrap_or(0)
    }
    pub fn has_state(&self, name: &str, day: u32) -> bool {
        self.states.get(name).is_some_and(|&d| d > day)
    }
    /// Wer die Provinz tatsächlich kontrolliert.
    pub fn holder(&self) -> Option<RealmId> {
        self.controller.or(self.owner)
    }
}

// ---------------------------------------------------------------------------------------------
// Charaktere und Dynastien
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeathCause {
    Natural,
    Battle,
    Murder,
    Disease,
    Executed,
    Deposed,
    Abdicated,
    Accident,
}

impl DeathCause {
    pub fn label(self) -> &'static str {
        match self {
            DeathCause::Natural => "natürlicher Tod",
            DeathCause::Battle => "im Kampf gefallen",
            DeathCause::Murder => "ermordet",
            DeathCause::Disease => "an einer Seuche gestorben",
            DeathCause::Executed => "hingerichtet",
            DeathCause::Deposed => "abgesetzt",
            DeathCause::Abdicated => "abgedankt",
            DeathCause::Accident => "durch einen Unfall gestorben",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretKind {
    Affair,
    Murder,
    Heresy,
    HollowCult,
    Illegitimate,
    Embezzlement,
}

impl SecretKind {
    pub fn label(self) -> &'static str {
        match self {
            SecretKind::Affair => "Affäre",
            SecretKind::Murder => "Mord",
            SecretKind::Heresy => "Ketzerei",
            SecretKind::HollowCult => "Hohlkult",
            SecretKind::Illegitimate => "Unehelichkeit",
            SecretKind::Embezzlement => "Unterschlagung",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    pub kind: SecretKind,
    pub public: bool,
    /// Wer das Geheimnis kennt (Erpresser).
    pub known_by: Vec<CharId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ability {
    Tactician,
    SiegeMaster,
    Logistician,
    Healer,
    Orator,
    Spy,
    Mage(School),
    Bold,
    Administrator,
}

impl Ability {
    pub fn name(self) -> &'static str {
        match self {
            Ability::Tactician => "Taktiker",
            Ability::SiegeMaster => "Belagerungsmeister",
            Ability::Logistician => "Versorgungskünstler",
            Ability::Healer => "Heiler",
            Ability::Orator => "Redner",
            Ability::Spy => "Späher",
            Ability::Mage(_) => "Magier",
            Ability::Bold => "Draufgänger",
            Ability::Administrator => "Verwalter",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKind {
    FamilyKilled,
    Betrayed,
    Helped,
    Insulted,
    Plotted,
    Gifted,
    Executed,
    Spared,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub kind: MemoryKind,
    pub subject: CharId,
    pub value: f32,
    pub expires: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: CharId,
    pub name: String,
    pub dynasty: DynId,
    pub house: Option<HouseId>,
    pub realm: Option<RealmId>,
    pub culture: Culture,
    pub religion: Religion,
    pub gender: Gender,
    /// Geburtstag (absolute Spieltage, kann negativ sein → i64).
    pub born: i64,
    pub died: Option<(u32, DeathCause)>,
    pub father: Option<CharId>,
    pub mother: Option<CharId>,
    pub spouse: Option<CharId>,
    pub children: Vec<CharId>,
    pub attrs: [i8; 6],
    pub health: f32,
    pub fertility: f32,
    pub traits: Vec<Trait>,
    pub abilities: Vec<Ability>,
    pub prestige: f32,
    pub piety: f32,
    pub hollowing: f32,
    pub stress: f32,
    pub location: ProvId,
    pub office: Option<(RealmId, Office)>,
    pub secrets: Vec<Secret>,
    pub captive_of: Option<RealmId>,
    pub army: Option<ArmyId>,
    pub education: Option<Attr>,
    pub guardian: Option<CharId>,
    pub claims: Vec<RealmId>,
    pub memories: Vec<Memory>,
    pub epithet: Option<String>,
    pub immortal: bool,
    pub wounded_until: u32,
    pub is_hero: bool,
    /// Karrierewerte für Beinamen.
    pub deeds: Deeds,
    pub reign_start: Option<u32>,
    pub reigns: Vec<RealmId>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Deeds {
    pub defensive_wins: u16,
    pub battles_won: u16,
    pub famines_averted: u16,
    pub alliances: u16,
    pub conquests: u16,
    pub plots: u16,
    pub purges: u16,
}

impl Character {
    pub fn alive(&self) -> bool {
        self.died.is_none()
    }
    pub fn age(&self, day: u32) -> i32 {
        ((day as i64 - self.born) / DAYS_PER_YEAR as i64) as i32
    }
    pub fn has(&self, t: Trait) -> bool {
        self.traits.contains(&t)
    }
    /// Attributwert inkl. Eigenschaften (0–30 begrenzt).
    pub fn attr(&self, a: Attr) -> i32 {
        let mut v = self.attrs[a.idx()] as i32;
        for t in &self.traits {
            v += t.attrs()[a.idx()] as i32;
        }
        v.clamp(0, 30)
    }
    pub fn adult(&self, day: u32) -> bool {
        self.age(day) >= 16
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynasty {
    pub id: DynId,
    pub name: String,
    pub culture: Culture,
    pub founded: u32,
    pub prestige: f32,
    pub extinct: bool,
    pub extinct_day: Option<u32>,
    pub cadet_of: Option<DynId>,
    pub house: Option<HouseId>,
    /// Freigeschaltete Vermächtnisse.
    pub legacies: Vec<String>,
    pub crest: u32,
}

// ---------------------------------------------------------------------------------------------
// Reiche, Häuser, Gesetze
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealmKind {
    Kingdom,
    March,
    Clan,
    Covenant,
    Republic,
    Duchy,
    Free,
    Hollowed,
    Legion,
    Rebel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Government {
    Feudal,
    ClanCouncil,
    Druidic,
    MerchantRepublic,
    HollowCourt,
    Legion,
    Tyranny,
}

impl Government {
    pub fn name(self) -> &'static str {
        match self {
            Government::Feudal => "Feudalmonarchie",
            Government::ClanCouncil => "Clanrat",
            Government::Druidic => "Druidenzirkel",
            Government::MerchantRepublic => "Handelsrepublik",
            Government::HollowCourt => "Hohler Hof",
            Government::Legion => "Ewige Legion",
            Government::Tyranny => "Tyrannis",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccLaw {
    Primogeniture,
    Ultimogeniture,
    Partition,
    Elective,
    Seniority,
    Designation,
}

impl SuccLaw {
    pub fn name(self) -> &'static str {
        match self {
            SuccLaw::Primogeniture => "Primogenitur",
            SuccLaw::Ultimogeniture => "Ultimogenitur",
            SuccLaw::Partition => "Teilung",
            SuccLaw::Elective => "Wahlmonarchie",
            SuccLaw::Seniority => "Seniorat",
            SuccLaw::Designation => "Ernennung",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Laws {
    /// 0 schwach … 3 absolut.
    pub crown_authority: u8,
    pub succession: SuccLaw,
    /// 0 Leibeigenschaft, 1 gebundene Pacht, 2 freie Bauern.
    pub serfdom: u8,
    /// 0 keine, 1 Marktrecht, 2 freie Städte.
    pub town_rights: u8,
    /// 0 Staatskirche, 1 Duldung, 2 Glaubensfreiheit.
    pub faith_law: u8,
    /// 0 Verbot, 1 Lizenz, 2 Freiheit.
    pub magic_law: u8,
    /// 0 Friedensordnung, 1 Wehrbereitschaft, 2 Kriegsrecht.
    pub martial_law: u8,
    /// Abgabe der Vasallen (Prozent).
    pub vassal_tax: f32,
    /// Heerfolge der Vasallen (Prozent).
    pub levy: f32,
    pub last_succession_change: u32,
}

impl Default for Laws {
    fn default() -> Self {
        Laws { crown_authority: 1, succession: SuccLaw::Primogeniture, serfdom: 0, town_rights: 1, faith_law: 0, magic_law: 1, martial_law: 0, vassal_tax: 20.0, levy: 25.0, last_succession_change: 0 }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Loan {
    pub lender: LoanSource,
    pub principal: f32,
    pub rate: f32,
    pub months_left: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanSource {
    SaelithBanks,
    Church,
    House(HouseId),
    War,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Regency {
    pub regent: CharId,
    pub ward: CharId,
    pub loyalty: f32,
    pub since: u32,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct ResearchState {
    pub points: f32,
    /// Erforschter Grad 0–4.
    pub grade: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Personality {
    Conqueror,
    Merchant,
    Zealot,
    Schemer,
    Keeper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalKind {
    Survive,
    Expand,
    ThroneOfValemark,
    Wealth,
    FaithPurity,
    Independence,
    Revenge,
    SealVeil,
    Balance,
    /// Verlorene Provinzen zurückerobern.
    Reconquer,
    /// Handelsroute/Marktanbindung etablieren.
    TradeRoute,
    /// Eigene Religion verbreiten.
    SpreadFaith,
    /// Rivalen schwächen (Komplotte, Boykott, Krieg).
    WeakenRival,
    /// Dynastie sichern (Heirat, Erben).
    SecureDynasty,
    /// Erzfeind vernichten.
    DestroyNemesis,
}

impl GoalKind {
    pub fn name(self) -> &'static str {
        match self {
            GoalKind::Survive => "Überleben",
            GoalKind::Expand => "Expansion",
            GoalKind::ThroneOfValemark => "Thron von Valemark",
            GoalKind::Wealth => "Reichtum",
            GoalKind::FaithPurity => "Glaubensreinheit",
            GoalKind::Independence => "Unabhängigkeit",
            GoalKind::Revenge => "Rache",
            GoalKind::SealVeil => "Schleier schließen",
            GoalKind::Balance => "Gleichgewicht",
            GoalKind::Reconquer => "Rückeroberung",
            GoalKind::TradeRoute => "Handelsweg",
            GoalKind::SpreadFaith => "Religion verbreiten",
            GoalKind::WeakenRival => "Rivalen schwächen",
            GoalKind::SecureDynasty => "Dynastie sichern",
            GoalKind::DestroyNemesis => "Erzfeind vernichten",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AiGoal {
    pub kind: GoalKind,
    pub target_realm: Option<RealmId>,
    pub target_prov: Option<ProvId>,
    pub religion: Option<Religion>,
    pub started: u32,
    pub weight: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealmMemKind {
    Betrayed,
    Helped,
    Attacked,
    Insulted,
    FamilyKilled,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RealmMemory {
    pub kind: RealmMemKind,
    pub other: RealmId,
    pub day: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RealmStats {
    pub wars_declared: u32,
    pub wars_won: u32,
    pub wars_lost: u32,
    pub battles_won: u32,
    pub battles_lost: u32,
    pub provinces_conquered: u32,
    pub provinces_lost: u32,
    pub plots_run: u32,
    pub rulers: u32,
    pub peak_provinces: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Realm {
    pub id: RealmId,
    pub key: String,
    pub name: String,
    /// Kanon-Fraktion (`ember_crown`, `skarn_clans` …).
    pub faction: String,
    pub kind: RealmKind,
    pub alive: bool,
    pub culture: Culture,
    pub religion: Religion,
    pub gov: Government,
    pub rank: Rank,
    pub ruler: CharId,
    pub dynasty: DynId,
    pub capital: ProvId,
    pub vassal_of: Option<RealmId>,
    pub gold: f32,
    /// Kopfsteuerstufe: 0 keine, 1 niedrig, 2 mittel, 3 hoch, 4 erdrückend.
    pub tax_level: u8,
    pub trade_tax: f32,
    pub tariff_import: f32,
    pub tariff_transit: f32,
    pub loans: Vec<Loan>,
    pub credit_blocked_until: u32,
    pub laws: Laws,
    pub offices: [Option<CharId>; 7],
    pub office_task: [u8; 7],
    pub prestige: f32,
    pub piety: f32,
    pub tyranny: f32,
    pub war_weariness: f32,
    pub legitimacy: f32,
    pub strength: f32,
    pub threat: f32,
    pub recent_conquests: Vec<u32>,
    pub treaty_breaks: u32,
    pub goals: Vec<AiGoal>,
    pub personality: Personality,
    pub regency: Option<Regency>,
    pub heir: Option<CharId>,
    pub research: [ResearchState; 9],
    pub research_focus: Option<School>,
    pub ashglass: f32,
    pub soul_ash: f32,
    pub hollow_path: u8,
    pub hollow_value: f32,
    pub void_uses: Vec<u32>,
    pub spy_net: BTreeMap<RealmId, f32>,
    pub court: Vec<CharId>,
    pub fleet: u32,
    pub is_player: bool,
    pub embargo: Vec<RealmId>,
    pub income: f32,
    pub expenses: f32,
    pub great_power: bool,
    pub memory: Vec<RealmMemory>,
    pub stats: RealmStats,
    pub founded: u32,
    pub fallen: Option<u32>,
    pub peace_reputation_until: u32,
    pub last_peace_day: u32,
    pub forbidden_discovered: bool,
    pub ritual_cooldown: u32,
    pub last_ruler_change: u32,
    pub tribute_paid: f32,
    pub banner: u32,
    pub last_event: BTreeMap<String, u32>,
    pub heresy_pressure: f32,
    pub recruit_budget: f32,
    pub excommunicated: bool,
    pub shrines_restored: u8,
    pub mage_orders: Vec<u8>,
    pub artifacts: Vec<u8>,
    pub pending_regime: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseLeaning {
    War,
    Trade,
    Faith,
    Crown,
    Separatist,
}

impl HouseLeaning {
    pub fn name(self) -> &'static str {
        match self {
            HouseLeaning::War => "Kriegspartei",
            HouseLeaning::Trade => "Handelspartei",
            HouseLeaning::Faith => "Glaubenspartei",
            HouseLeaning::Crown => "Kronpartei",
            HouseLeaning::Separatist => "Separatisten",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseRight {
    TaxExempt,
    OwnArmy,
    CouncilSeat,
    TownCharter,
    Creditor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NobleHouse {
    pub id: HouseId,
    pub name: String,
    pub realm: RealmId,
    pub dynasty: DynId,
    pub head: CharId,
    pub alive: bool,
    pub power: f32,
    pub loyalty: f32,
    pub wealth: f32,
    pub retinue: u32,
    pub leaning: HouseLeaning,
    pub rights: Vec<HouseRight>,
    pub throne_claim: bool,
    pub faction: Option<u32>,
    pub seat: ProvId,
    pub cadet: bool,
    pub grudge: f32,
}

/// Unzufriedene Häuser bündeln sich zu Fraktionen mit einer Forderung.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionDemand {
    LessCrownAuthority,
    Independence,
    OtherHeir,
    WarOn(RealmId),
    ReligiousReform,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoliticalFaction {
    pub id: u32,
    pub realm: RealmId,
    pub demand: FactionDemand,
    pub members: Vec<HouseId>,
    pub leader: HouseId,
    pub ultimatum: bool,
    pub started: u32,
    pub candidate: Option<CharId>,
}

// ---------------------------------------------------------------------------------------------
// Armeen
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmyKind {
    Realm,
    Rebels,
    Monster,
    Bandits,
    Mercenary,
    Cultists,
    Legion,
    Hollow,
    Boss,
    Refugees,
}

impl ArmyKind {
    pub fn is_neutral(self) -> bool {
        !matches!(self, ArmyKind::Realm | ArmyKind::Mercenary)
    }
}

/// Truppenverband eines Einheitentyps (1 Figur = 1 RTS-Einheit).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Regiment {
    pub unit: String,
    pub count: u32,
    /// Veteranenstufe 0–3 (Rekrut, Erfahren, Veteran, Elite).
    pub vet: u8,
    /// Mittlerer Anteil Lebenspunkte (0–1).
    pub hp: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmyOrder {
    Idle,
    Move,
    /// Belagert/besetzt die Zielprovinz.
    Siege,
    Retreat,
    /// Patrouille / Garnison.
    Garrison,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Army {
    pub id: ArmyId,
    pub name: String,
    pub kind: ArmyKind,
    pub owner: Option<RealmId>,
    /// Bei Rebellen: gegen welches Reich; bei Söldnern: Auftraggeber = owner.
    pub target_realm: Option<RealmId>,
    pub house: Option<HouseId>,
    pub commander: Option<CharId>,
    pub regiments: Vec<Regiment>,
    pub location: ProvId,
    pub path: Vec<ProvId>,
    /// Fortschritt (0–1) auf der Kante zum nächsten Pfadpunkt.
    pub progress: f32,
    pub order: ArmyOrder,
    pub morale: f32,
    pub supply: f32,
    pub in_battle: Option<BattleId>,
    pub locked_until: u32,
    pub war: Option<WarId>,
    pub created: u32,
    pub unpaid_months: u8,
    pub boss: Option<String>,
    pub came_from: Option<ProvId>,
    pub wounded_pop: f32,
}

impl Army {
    pub fn size(&self) -> u32 {
        self.regiments.iter().map(|r| r.count).sum()
    }
}

// ---------------------------------------------------------------------------------------------
// Diplomatie
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatyKind {
    Alliance,
    DefensivePact,
    NonAggression,
    Trade,
    Marriage,
    Tribute,
    MilitaryAccess,
    Guarantee,
    JointCampaign,
    Truce,
    SecretAlliance,
    AgainstHollow,
}

impl TreatyKind {
    pub fn name(self) -> &'static str {
        match self {
            TreatyKind::Alliance => "Bündnis",
            TreatyKind::DefensivePact => "Verteidigungspakt",
            TreatyKind::NonAggression => "Nichtangriffspakt",
            TreatyKind::Trade => "Handelsabkommen",
            TreatyKind::Marriage => "Heiratsbündnis",
            TreatyKind::Tribute => "Tributvertrag",
            TreatyKind::MilitaryAccess => "Militärischer Zugang",
            TreatyKind::Guarantee => "Garantie",
            TreatyKind::JointCampaign => "Gemeinsamer Feldzug",
            TreatyKind::Truce => "Waffenstillstand",
            TreatyKind::SecretAlliance => "Geheimes Bündnis",
            TreatyKind::AgainstHollow => "Gemeinsame Front gegen das Tief",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Treaty {
    pub id: u32,
    pub kind: TreatyKind,
    /// a = Antragsteller/Senior (bei Tribut: Empfänger, bei Garantie: Garant).
    pub a: RealmId,
    pub b: RealmId,
    pub start: u32,
    pub end: Option<u32>,
    /// Zusatzdatum: bei JointCampaign das Zielreich, bei Marriage die Figur.
    pub data: i64,
    pub secret: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpKind {
    Gift,
    Envoy,
    CommonEnemy,
    WarAgainst,
    BrokenTreaty,
    AllianceCallRefused,
    PrisonerReturned,
    FamilyExecuted,
    KinMurdered,
    HelpedAgainstHollow,
    PeaceReputation,
    TradePact,
    Betrayal,
    Boycott,
    Plot,
    Marriage,
    Insult,
    TreasonToLight,
    Tribute,
    Claim,
    Historic,
    Helped,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OpinionMod {
    pub kind: OpKind,
    pub value: f32,
    pub start: u32,
    /// None = dauerhaft, sonst Ablauftag (linearer Abklang).
    pub expires: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CasusBelli {
    Claim,
    HolyWar,
    Independence,
    Border,
    Revenge,
    LiberateHollowland,
    Coalition,
    Throne,
    Unjustified,
    Subjugation,
}

impl CasusBelli {
    pub fn name(self) -> &'static str {
        match self {
            CasusBelli::Claim => "Anspruch",
            CasusBelli::HolyWar => "Heiliger Krieg",
            CasusBelli::Independence => "Unabhängigkeit",
            CasusBelli::Border => "Grenzkonflikt",
            CasusBelli::Revenge => "Vergeltung",
            CasusBelli::LiberateHollowland => "Befreiung von Hohlland",
            CasusBelli::Coalition => "Koalitionskrieg",
            CasusBelli::Throne => "Thronanspruch",
            CasusBelli::Unjustified => "Eroberungskrieg ohne Grund",
            CasusBelli::Subjugation => "Unterwerfung",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarGoal {
    TakeProvince(ProvId),
    Tribute,
    Independence,
    Throne,
    Subjugate,
    LiberateHollowland,
    RevertConquests,
    Destroy,
    ConvertFaith,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct War {
    pub id: WarId,
    pub attackers: Vec<RealmId>,
    pub defenders: Vec<RealmId>,
    pub leader_a: RealmId,
    pub leader_d: RealmId,
    pub cb: CasusBelli,
    pub goal: WarGoal,
    /// Kriegspunkte aus Sicht der Angreifer (−100 … 100).
    pub score: f32,
    pub start: u32,
    pub active: bool,
    pub battles: u32,
    pub casualties_a: u32,
    pub casualties_d: u32,
    pub civil: bool,
    /// Bei Thronkrieg: Prätendent.
    pub claimant: Option<CharId>,
    pub chronicle: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coalition {
    pub id: u32,
    pub target: RealmId,
    pub members: Vec<RealmId>,
    pub founded: u32,
    pub forced: bool,
    pub active: bool,
}

// ---------------------------------------------------------------------------------------------
// Intrigen
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotKind {
    Murder,
    Abduct,
    Slander,
    Fabricate,
    Incite,
    Usurp,
    Steal,
    Seduce,
    Espionage,
    Bribery,
    Treason,
    SecretAlliance,
    Blackmail,
    Conspiracy,
}

impl PlotKind {
    pub fn id(self) -> &'static str {
        match self {
            PlotKind::Murder => "plot_murder",
            PlotKind::Abduct => "plot_abduct",
            PlotKind::Slander => "plot_slander",
            PlotKind::Fabricate => "plot_fabricate",
            PlotKind::Incite => "plot_incite",
            PlotKind::Usurp => "plot_usurp",
            PlotKind::Steal => "plot_steal",
            PlotKind::Seduce => "plot_seduce",
            PlotKind::Espionage => "plot_espionage",
            PlotKind::Bribery => "plot_bribery",
            PlotKind::Treason => "plot_treason",
            PlotKind::SecretAlliance => "plot_secret_alliance",
            PlotKind::Blackmail => "plot_blackmail",
            PlotKind::Conspiracy => "plot_conspiracy",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            PlotKind::Murder => "Attentat",
            PlotKind::Abduct => "Entführung",
            PlotKind::Slander => "Rufmord",
            PlotKind::Fabricate => "Anspruch fälschen",
            PlotKind::Incite => "Aufwiegeln",
            PlotKind::Usurp => "Putsch",
            PlotKind::Steal => "Diebstahl",
            PlotKind::Seduce => "Verführung",
            PlotKind::Espionage => "Spionage",
            PlotKind::Bribery => "Bestechung",
            PlotKind::Treason => "Verrat",
            PlotKind::SecretAlliance => "Geheimes Bündnis",
            PlotKind::Blackmail => "Erpressung",
            PlotKind::Conspiracy => "Verschwörung",
        }
    }
    /// Grundschwierigkeit nach GDD 6.6 (neue Arten sinnvoll ergänzt).
    pub fn difficulty(self) -> f32 {
        match self {
            PlotKind::Murder => 60.0,
            PlotKind::Abduct => 70.0,
            PlotKind::Slander => 30.0,
            PlotKind::Fabricate => 40.0,
            PlotKind::Incite => 45.0,
            PlotKind::Usurp => 90.0,
            PlotKind::Steal => 50.0,
            PlotKind::Seduce => 35.0,
            PlotKind::Espionage => 25.0,
            PlotKind::Bribery => 35.0,
            PlotKind::Treason => 65.0,
            PlotKind::SecretAlliance => 40.0,
            PlotKind::Blackmail => 45.0,
            PlotKind::Conspiracy => 55.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotTarget {
    Char(CharId),
    Prov(ProvId),
    Realm(RealmId),
    Throne(RealmId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plot {
    pub id: PlotId,
    pub kind: PlotKind,
    pub leader: CharId,
    pub conspirators: Vec<CharId>,
    pub target: PlotTarget,
    /// Reich, in dem das Komplott spielt (Opfer).
    pub realm: RealmId,
    /// Auftraggeberreich (bei Auslandsintrigen).
    pub sponsor: Option<RealmId>,
    pub progress: f32,
    pub started: u32,
    pub candidate: Option<CharId>,
    pub discovered: bool,
    pub done: bool,
}

// ---------------------------------------------------------------------------------------------
// Märkte
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    pub region: usize,
    pub name: String,
    pub prices: Vec<f32>,
    pub supply: Vec<f32>,
    pub demand: Vec<f32>,
    /// Importbestand im Marktgebiet.
    pub pool: Vec<f32>,
    pub inflation: f32,
    /// Ereignisfaktor je Ware (1 = neutral) mit Ablauftag.
    pub event_factor: Vec<(f32, u32)>,
    pub volume: f32,
    pub shortage: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub a: usize,
    pub b: usize,
    /// Provinz-Eintrittspunkte (für Zölle, Blockade).
    pub from_prov: ProvId,
    pub to_prov: ProvId,
    pub days: f32,
    pub sea: bool,
    pub capacity: f32,
    pub blocked_until: u32,
    pub volume_last: f32,
    pub profit_last: f32,
}

// ---------------------------------------------------------------------------------------------
// Neutrale Akteure, Bosse, Magie
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeutralKind {
    AshWolves,
    GraveWorms,
    Salamanders,
    MoorLights,
    Bandits,
    Cultists,
    GlassSeekers,
    VeylClans,
    Refugees,
    Ruins,
}

impl NeutralKind {
    pub fn name(self) -> &'static str {
        match self {
            NeutralKind::AshWolves => "Aschwolfrudel",
            NeutralKind::GraveWorms => "Grabwürmer",
            NeutralKind::Salamanders => "Glutsalamander",
            NeutralKind::MoorLights => "Moorlichter",
            NeutralKind::Bandits => "Raubritter und Banditen",
            NeutralKind::Cultists => "Kultisten der Leeren Liturgie",
            NeutralKind::GlassSeekers => "Aschglas-Sucher",
            NeutralKind::VeylClans => "Ungebundene Veyl-Sippen",
            NeutralKind::Refugees => "Flüchtlingszug",
            NeutralKind::Ruins => "Ruinen und Gewölbe",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeutralGroup {
    pub id: u32,
    pub kind: NeutralKind,
    pub province: ProvId,
    pub strength: f32,
    pub aggression: f32,
    pub founded: u32,
    pub active: bool,
    pub last_raid: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MercCompany {
    pub id: u32,
    pub name: String,
    pub size: u32,
    pub home: ProvId,
    pub hired_by: Option<RealmId>,
    pub army: Option<ArmyId>,
    pub monthly_pay: f32,
    pub unpaid_months: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BossState {
    Dormant,
    Active,
    Defeated,
    Allied,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Boss {
    pub id: String,
    pub name: String,
    pub province: ProvId,
    pub state: BossState,
    pub army: Option<ArmyId>,
    pub last_action: u32,
    pub next_action: u32,
    pub tribute_until: u32,
    pub defeated_by: Option<RealmId>,
    pub awakened: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MageOrder {
    pub id: u8,
    pub name: String,
    pub school: School,
    pub realm: Option<RealmId>,
    pub members: u32,
    pub wealth: f32,
    pub forbidden: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactState {
    pub def: u8,
    pub holder: Option<RealmId>,
    pub site: ProvId,
    pub effect: ArtifactEffect,
}

// ---------------------------------------------------------------------------------------------
// Sonstiges
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndgameState {
    pub started: bool,
    pub started_day: Option<u32>,
    pub veil_pressure: f32,
    pub great_power: Option<RealmId>,
    pub legion_awake: bool,
    pub hollow_king_awake: Option<u32>,
    pub dragon_wrath: bool,
    pub withering: bool,
    pub veil_break: bool,
    pub last_legion_march_year: i32,
    pub last_veil_break_year: i32,
    pub league_of_last_flame: Vec<RealmId>,
    /// Reiche, gegen die sich der Bund der Letzten Flamme (Hohlungsstufe 5) gebildet hat.
    pub league_targets: Vec<RealmId>,
    pub gods_shrines: u8,
    pub seal_cast: Option<RealmId>,
    pub plague_magic_year: Option<i32>,
    pub portals: Vec<ProvId>,
    pub great_power_since: Option<u32>,
}

impl Default for EndgameState {
    fn default() -> Self {
        EndgameState {
            started: false,
            started_day: None,
            veil_pressure: 5.0,
            great_power: None,
            legion_awake: false,
            hollow_king_awake: None,
            dragon_wrath: false,
            withering: false,
            veil_break: false,
            last_legion_march_year: -1,
            last_veil_break_year: -1,
            league_of_last_flame: Vec::new(),
            league_targets: Vec::new(),
            gods_shrines: 0,
            seal_cast: None,
            plague_magic_year: None,
            portals: Vec::new(),
            great_power_since: None,
        }
    }
}
