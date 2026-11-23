//! Startzustand der Welt Eldraeth im Jahr 897 n.F. (Kanon): Reiche, Adelshäuser, Herrscher,
//! Provinzbesitz, Beziehungen, Armeen. Provinzen, Wirtschaft und Monster kommen aus dem Seed.

use crate::catalog::{self, Trait, UClass};
use crate::chronicle::{Chronicle, ChronicleKind, Entry};
use crate::data::StrategyData;
use crate::mapgen;
use crate::model::*;
use crate::types::*;
use crate::world::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

struct Person {
    name: &'static str,
    family: &'static str,
    gender: Gender,
    age: i32,
    traits: &'static [Trait],
    /// Kriegskunst, Verwaltung, Diplomatie, Intrige, Gelehrsamkeit, Charisma
    attrs: [i8; 6],
    health: f32,
}

struct RealmSpec {
    key: &'static str,
    name: &'static str,
    faction: &'static str,
    kind: RealmKind,
    culture: Culture,
    religion: Religion,
    gov: Government,
    rank: Rank,
    capital: &'static str,
    provinces: &'static [&'static str],
    vassal_of: Option<&'static str>,
    gold: f32,
    succession: SuccLaw,
    ruler: Person,
    personality: Personality,
    immortal: bool,
}

struct HouseSpec {
    realm: &'static str,
    name: &'static str,
    head: Person,
    provinces: &'static [&'static str],
    leaning: HouseLeaning,
    loyalty: f32,
    claim: bool,
    retinue: u32,
    wealth: f32,
}

use Trait::*;

const fn pr(name: &'static str, family: &'static str, gender: Gender, age: i32, traits: &'static [Trait], attrs: [i8; 6], health: f32) -> Person {
    Person { name, family, gender, age, traits, attrs, health }
}

fn realm_specs() -> Vec<RealmSpec> {
    use Gender::*;
    vec![
        RealmSpec {
            key: "ember_crown",
            name: "Die Glutkrone von Valemark",
            faction: "ember_crown",
            kind: RealmKind::Kingdom,
            culture: Culture::Valemarker,
            religion: Religion::EternalFlame,
            gov: Government::Feudal,
            rank: Rank::King,
            capital: "arnvale",
            provinces: &["arnvale", "kaldris", "koenigsfurt", "harkenmoor", "falkenmark", "corvanshold", "rabental", "rotsandfeste", "vellmund", "melbrechts_auen", "weissfurt", "strathhaven", "ilverwacht", "eschenhain", "sonnfall"],
            vassal_of: None,
            gold: 6000.0,
            succession: SuccLaw::Primogeniture,
            ruler: pr("Aldric", "Caldermont", Male, 68, &[Paranoid, Patient], [5, 9, 7, 10, 8, 6], 4.0),
            personality: Personality::Keeper,
            immortal: false,
        },
        RealmSpec {
            key: "ashmarch_march",
            name: "Markgrafschaft Aschmark",
            faction: "ember_crown",
            kind: RealmKind::March,
            culture: Culture::Aschmaerker,
            religion: Religion::EternalFlame,
            gov: Government::Feudal,
            rank: Rank::Marcher,
            capital: "brennhollow",
            provinces: &["brennhollow", "grauwall", "kaltherd", "kraehenmoore", "letzter_meilenstein"],
            vassal_of: Some("ember_crown"),
            gold: 300.0,
            succession: SuccLaw::Primogeniture,
            ruler: pr("Isolde", "Raventhorn", Female, 34, &[Brave, Just, AshFeverScar], [12, 8, 7, 5, 6, 10], 7.0),
            personality: Personality::Keeper,
            immortal: false,
        },
        RealmSpec {
            key: "skarn_clans",
            name: "Die Clans von Skarnheim",
            faction: "skarn_clans",
            kind: RealmKind::Clan,
            culture: Culture::Skarn,
            religion: Religion::AncestorPath,
            gov: Government::ClanCouncil,
            rank: Rank::Duke,
            capital: "kharnfels",
            provinces: &["kharnfels", "eisenkehle", "waechtertor", "vhalmirsee", "nordhafen", "grauhelm"],
            vassal_of: None,
            gold: 3500.0,
            succession: SuccLaw::Elective,
            ruler: pr("Torvald", "Eisenhand", Male, 56, &[Wrathful, Brave], [13, 7, 5, 6, 5, 9], 6.0),
            personality: Personality::Conqueror,
            immortal: false,
        },
        RealmSpec {
            key: "wolfspass_clan",
            name: "Clan Wolfspass",
            faction: "skarn_clans",
            kind: RealmKind::Clan,
            culture: Culture::Skarn,
            religion: Religion::AncestorPath,
            gov: Government::ClanCouncil,
            rank: Rank::Count,
            capital: "wolfspass",
            provinces: &["wolfspass", "frostgrat"],
            vassal_of: Some("skarn_clans"),
            gold: 400.0,
            succession: SuccLaw::Primogeniture,
            ruler: pr("Ragnvar", "Wolfspass", Male, 44, &[Ambitious, Brave], [11, 6, 6, 7, 4, 7], 7.0),
            personality: Personality::Conqueror,
            immortal: false,
        },
        RealmSpec {
            key: "thorn_covenant",
            name: "Der Dornenbund",
            faction: "thorn_covenant",
            kind: RealmKind::Covenant,
            culture: Culture::Veyl,
            religion: Religion::GreenCircle,
            gov: Government::Druidic,
            rank: Rank::Duke,
            capital: "ilvaen",
            provinces: &["ilvaen", "herzhain", "calyxruh", "dornpforte", "wurzelmark", "mottenwacht"],
            vassal_of: None,
            gold: 2500.0,
            succession: SuccLaw::Elective,
            ruler: pr("Sylaen", "Veyl", Female, 52, &[Zealous, Patient, Honest], [6, 8, 9, 6, 13, 9], 7.0),
            personality: Personality::Zealot,
            immortal: false,
        },
        RealmSpec {
            key: "veyl_free",
            name: "Ungebundene Veyl-Sippen",
            faction: "thorn_covenant",
            kind: RealmKind::Free,
            culture: Culture::Veyl,
            religion: Religion::GreenCircle,
            gov: Government::Druidic,
            rank: Rank::Count,
            capital: "bernsteinsuempfe",
            provinces: &["bernsteinsuempfe", "brandsaum"],
            vassal_of: None,
            gold: 500.0,
            succession: SuccLaw::Elective,
            ruler: pr("Orlen", "Mottenflügel", Male, 41, &[Content, Merciful], [6, 7, 8, 6, 9, 8], 7.0),
            personality: Personality::Keeper,
            immortal: false,
        },
        RealmSpec {
            key: "saelith_compact",
            name: "Der Saelith-Pakt",
            faction: "saelith_compact",
            kind: RealmKind::Republic,
            culture: Culture::Saelith,
            religion: Religion::TideHouse,
            gov: Government::MerchantRepublic,
            rank: Rank::Duke,
            capital: "selvaris",
            provinces: &["selvaris", "thalmoor", "isk", "varo", "thals_auge", "schwarzes_riff", "merenhal", "corallo"],
            vassal_of: None,
            gold: 9000.0,
            succession: SuccLaw::Elective,
            ruler: pr("Ophira", "Saelith-Varo", Female, 29, &[Ambitious, Greedy], [5, 11, 14, 11, 8, 12], 8.0),
            personality: Personality::Merchant,
            immortal: false,
        },
        RealmSpec {
            key: "free_port_emberth",
            name: "Freihafen Emberth",
            faction: "saelith_compact",
            kind: RealmKind::Free,
            culture: Culture::Saelith,
            religion: Religion::TideHouse,
            gov: Government::MerchantRepublic,
            rank: Rank::Count,
            capital: "emberth",
            provinces: &["emberth"],
            vassal_of: None,
            gold: 600.0,
            succession: SuccLaw::Elective,
            ruler: pr("Dario", "Vess", Male, 47, &[Deceitful, Greedy], [8, 8, 8, 10, 6, 7], 6.0),
            personality: Personality::Schemer,
            immortal: false,
        },
        RealmSpec {
            key: "grey_nomads",
            name: "Graue Nomaden von Merrat",
            faction: "grey_nomads",
            kind: RealmKind::Free,
            culture: Culture::GraueNomaden,
            religion: Religion::SunHeresy,
            gov: Government::ClanCouncil,
            rank: Rank::Count,
            capital: "merrat",
            provinces: &["merrat", "gebrochene_saeule", "sonnenstrasse_posten", "duenenmeer"],
            vassal_of: None,
            gold: 400.0,
            succession: SuccLaw::Elective,
            ruler: pr("Zahir", "Aschwandler", Male, 49, &[Patient, Zealous], [8, 7, 7, 7, 8, 8], 7.0),
            personality: Personality::Zealot,
            immortal: false,
        },
        RealmSpec {
            key: "slag_brood",
            name: "Schlackenbrut von Khar Ossun",
            faction: "slag_brood",
            kind: RealmKind::Free,
            culture: Culture::Schlackenbrut,
            religion: Religion::SunHeresy,
            gov: Government::ClanCouncil,
            rank: Rank::Count,
            capital: "khar_ossun",
            provinces: &["khar_ossun", "brennender_pass", "feuerquellen", "obsidiankueste"],
            vassal_of: None,
            gold: 700.0,
            succession: SuccLaw::Primogeniture,
            ruler: pr("Ossgar", "Glutzahn", Male, 38, &[Cruel, Brave], [12, 5, 4, 7, 4, 6], 8.0),
            personality: Personality::Conqueror,
            immortal: false,
        },
        RealmSpec {
            key: "hollowed",
            name: "Die Hohlen",
            faction: "hollowed",
            kind: RealmKind::Hollowed,
            culture: Culture::Hohl,
            religion: Religion::HollowLiturgy,
            gov: Government::HollowCourt,
            rank: Rank::King,
            capital: "hohles_auge",
            provinces: &["hohles_auge", "ner_moraun", "vael_ithar", "schwebende_steine", "stillwasser", "vael_dorn"],
            vassal_of: None,
            gold: 1.0e9,
            succession: SuccLaw::Seniority,
            ruler: pr("Die Stimme aus dem Tief", "Das Tief", Male, 900, &[Cruel], [10, 10, 10, 15, 15, 10], 10.0),
            personality: Personality::Conqueror,
            immortal: true,
        },
        RealmSpec {
            key: "ashen_legion",
            name: "Die Aschenlegion",
            faction: "ashen_legion",
            kind: RealmKind::Legion,
            culture: Culture::Aurethisch,
            religion: Religion::SunHeresy,
            gov: Government::Legion,
            rank: Rank::HighKing,
            capital: "solmere",
            provinces: &["solmere", "glasfelder_von_ilum", "kesh_auran", "nekropole_der_sonnenkoenige"],
            vassal_of: None,
            gold: 1.0e9,
            succession: SuccLaw::Seniority,
            ruler: pr("Varkhan", "Aurelith", Male, 900, &[Ambitious, Cruel], [15, 12, 8, 10, 16, 14], 10.0),
            personality: Personality::Conqueror,
            immortal: true,
        },
    ]
}

fn house_specs() -> Vec<HouseSpec> {
    use Gender::*;
    use HouseLeaning::*;
    vec![
        HouseSpec { realm: "ember_crown", name: "Corvane", head: pr("Mireille", "Corvane", Female, 41, &[Ambitious, Deceitful], [7, 9, 10, 12, 9, 13], 7.0), provinces: &["corvanshold", "rabental", "rotsandfeste"], leaning: Separatist, loyalty: 24.0, claim: true, retinue: 120, wealth: 1200.0 },
        HouseSpec { realm: "ember_crown", name: "Melbrecht", head: pr("Gerold", "Melbrecht", Male, 63, &[Greedy, Content], [4, 11, 8, 8, 6, 8], 5.0), provinces: &["vellmund", "melbrechts_auen", "weissfurt"], leaning: Trade, loyalty: 55.0, claim: false, retinue: 70, wealth: 2500.0 },
        HouseSpec { realm: "ember_crown", name: "Strathgar", head: pr("Lorens", "Strathgar", Male, 37, &[Deceitful, Generous], [7, 9, 11, 10, 8, 10], 8.0), provinces: &["strathhaven"], leaning: Trade, loyalty: 60.0, claim: false, retinue: 60, wealth: 1800.0 },
        HouseSpec { realm: "ember_crown", name: "Ilverholt", head: pr("Benedikt", "Ilverholt", Male, 55, &[Zealous, Honest], [8, 8, 9, 4, 7, 8], 6.0), provinces: &["ilverwacht", "eschenhain"], leaning: Faith, loyalty: 85.0, claim: false, retinue: 80, wealth: 700.0 },
        HouseSpec { realm: "ember_crown", name: "Varenne", head: pr("Aurelien", "Varenne", Male, 46, &[Ambitious, Greedy], [8, 7, 8, 11, 7, 9], 6.0), provinces: &["sonnfall"], leaning: Separatist, loyalty: 36.0, claim: false, retinue: 55, wealth: 900.0 },
        HouseSpec { realm: "skarn_clans", name: "Grauhelm", head: pr("Haldor", "Grauhelm", Male, 49, &[Brave, Zealous], [12, 6, 5, 5, 6, 8], 7.0), provinces: &["grauhelm"], leaning: War, loyalty: 60.0, claim: false, retinue: 90, wealth: 400.0 },
        HouseSpec { realm: "thorn_covenant", name: "Zirkel der Wurzeln", head: pr("Alder", "Wurzelsohn", Male, 58, &[Zealous, Cynical], [5, 8, 8, 7, 12, 8], 6.0), provinces: &["herzhain"], leaning: Faith, loyalty: 70.0, claim: false, retinue: 40, wealth: 300.0 },
        HouseSpec { realm: "thorn_covenant", name: "Hain von Calyxruh", head: pr("Willow", "Mooshain", Female, 44, &[Patient, Just], [5, 9, 8, 6, 10, 9], 7.0), provinces: &["calyxruh"], leaning: Crown, loyalty: 65.0, claim: false, retinue: 35, wealth: 250.0 },
        HouseSpec { realm: "saelith_compact", name: "Saelith-Ombre", head: pr("Corvin", "Saelith-Ombre", Male, 55, &[Greedy, Patient], [4, 13, 9, 9, 8, 8], 6.0), provinces: &["thals_auge"], leaning: Trade, loyalty: 60.0, claim: false, retinue: 30, wealth: 6000.0 },
        HouseSpec { realm: "saelith_compact", name: "Isquel", head: pr("Valerio", "Isquel", Male, 51, &[Greedy, Deceitful], [5, 10, 8, 9, 7, 8], 6.0), provinces: &["isk"], leaning: Trade, loyalty: 55.0, claim: false, retinue: 25, wealth: 3500.0 },
        HouseSpec { realm: "saelith_compact", name: "Merenhal", head: pr("Chiara", "Merenhal", Female, 43, &[Content, Generous], [4, 10, 10, 7, 7, 10], 7.0), provinces: &["merenhal"], leaning: Trade, loyalty: 65.0, claim: false, retinue: 20, wealth: 2500.0 },
        HouseSpec { realm: "saelith_compact", name: "Thalvern", head: pr("Matteo", "Thalvern", Male, 60, &[Zealous, Patient], [4, 9, 9, 6, 9, 8], 5.0), provinces: &["thalmoor"], leaning: Faith, loyalty: 70.0, claim: false, retinue: 20, wealth: 1500.0 },
        HouseSpec { realm: "saelith_compact", name: "Corallo", head: pr("Serafina", "Corallo", Female, 38, &[Ambitious, Courtier], [4, 9, 12, 8, 7, 11], 8.0), provinces: &["corallo"], leaning: Trade, loyalty: 55.0, claim: false, retinue: 20, wealth: 1800.0 },
        HouseSpec { realm: "saelith_compact", name: "Duvessa", head: pr("Teodor", "Duvessa", Male, 48, &[Brave, Greedy], [11, 8, 7, 8, 5, 8], 7.0), provinces: &[], leaning: War, loyalty: 50.0, claim: false, retinue: 150, wealth: 2500.0 },
        HouseSpec { realm: "saelith_compact", name: "Lirr", head: pr("Orsino", "Lirr", Male, 52, &[Patient, Honest], [5, 9, 11, 5, 8, 8], 6.0), provinces: &["schwarzes_riff"], leaning: Trade, loyalty: 60.0, claim: false, retinue: 20, wealth: 2200.0 },
        HouseSpec { realm: "saelith_compact", name: "Astrane", head: pr("Delphine", "Astrane", Female, 45, &[Cynical, Deceitful], [4, 12, 8, 11, 9, 9], 7.0), provinces: &[], leaning: Trade, loyalty: 55.0, claim: false, retinue: 10, wealth: 4000.0 },
    ]
}

/// Verwandtschaftsdaten kanonischer Figuren, die nicht aus Zufall entstehen.
fn canon_note() -> &'static str {
    "Aldric III. (68), Erbin Elyane (9); Isolde Raventhorn (34), Wächterin der Aschmark"
}

/// Monate des wirtschaftlichen Einschwingens vor Spielbeginn.
const WARMUP_MONTHS: u32 = 180;

impl StrategyWorld {
    /// Erzeugt die Welt: Karte, Reiche, Figuren, Wirtschaft, Bedrohungen.
    pub fn new(options: StrategyOptions) -> StrategyWorld {
        Self::new_with_data(options, StrategyData::builtin())
    }

    pub fn new_with_data(options: StrategyOptions, data: std::sync::Arc<StrategyData>) -> StrategyWorld {
        let seed = options.seed;
        let map = mapgen::generate(seed);
        let mut w = StrategyWorld {
            options: options.clone(),
            day: 0,
            rng: ChaCha8Rng::seed_from_u64(seed ^ 0xC0FF_EE00_1234_ABCD),
            regions: map.regions,
            provinces: map.provinces,
            realms: Vec::new(),
            houses: Vec::new(),
            dynasties: Vec::new(),
            chars: Vec::new(),
            armies: Vec::new(),
            next_army: 1,
            wars: Vec::new(),
            next_war: 1,
            treaties: Vec::new(),
            next_treaty: 1,
            opinion_mods: BTreeMap::new(),
            coalitions: Vec::new(),
            next_coalition: 1,
            factions: Vec::new(),
            next_faction: 1,
            plots: Vec::new(),
            next_plot: 1,
            markets: Vec::new(),
            routes: Vec::new(),
            neutrals: Vec::new(),
            next_neutral: 1,
            mercs: Vec::new(),
            bosses: Vec::new(),
            mage_orders: Vec::new(),
            artifacts: Vec::new(),
            chronicle: Chronicle::default(),
            endgame: EndgameState::default(),
            events: EventState::default(),
            victory: None,
            battles: Vec::new(),
            next_battle: 1,
            player: None,
            out: Vec::new(),
            stats: WorldStats::default(),
            peace_offers: Vec::new(),
            next_peace_offer: 1,
            returning: Vec::new(),
            zeal: [50.0; 6],
            root_anger: 0.0,
            shrine_quests: Vec::new(),
            league_chest: 0.0,
            veil_break_years: 0,
            trade_hold: BTreeMap::new(),
            data,
            realm_provs: Vec::new(),
            realm_dist: Vec::new(),
        };
        w.build_realms();
        w.finish_setup();
        w
    }

    fn mk_person(&mut self, p: &Person, culture: Culture, dynasty: DynId, realm: Option<RealmId>, house: Option<HouseId>, loc: ProvId) -> CharId {
        let id = self.spawn_person(Some(p.name.to_string()), culture, dynasty, p.gender, p.age, realm, house, loc);
        let ch = &mut self.chars[id as usize];
        ch.attrs = p.attrs;
        ch.traits = p.traits.to_vec();
        ch.health = p.health;
        ch.prestige = 80.0 + p.age as f32 * 2.0;
        id
    }

    fn blank_realm(&self, id: RealmId, spec: &RealmSpec, ruler: CharId, dynasty: DynId, capital: ProvId) -> Realm {
        Realm {
            id,
            key: spec.key.to_string(),
            name: spec.name.to_string(),
            faction: spec.faction.to_string(),
            kind: spec.kind,
            alive: true,
            culture: spec.culture,
            religion: spec.religion,
            gov: spec.gov,
            rank: spec.rank,
            ruler,
            dynasty,
            capital,
            vassal_of: None,
            gold: spec.gold,
            tax_level: 2,
            trade_tax: 5.0,
            tariff_import: 5.0,
            tariff_transit: 2.0,
            loans: Vec::new(),
            credit_blocked_until: 0,
            laws: Laws {
                succession: spec.succession,
                faith_law: match spec.faction {
                    "ember_crown" => 0,
                    "saelith_compact" => 2,
                    _ => 1,
                },
                serfdom: match spec.faction {
                    "ember_crown" => 0,
                    "skarn_clans" => 1,
                    "saelith_compact" => 2,
                    _ => 1,
                },
                town_rights: if spec.faction == "saelith_compact" { 2 } else { 1 },
                magic_law: match spec.faction {
                    "ember_crown" => 1,
                    "saelith_compact" => 2,
                    "thorn_covenant" => 2,
                    "skarn_clans" => 1,
                    _ => 1,
                },
                crown_authority: if spec.kind == RealmKind::Kingdom { 1 } else { 0 },
                ..Laws::default()
            },
            offices: [None; 7],
            office_task: [0; 7],
            prestige: 200.0,
            piety: 120.0,
            tyranny: 0.0,
            war_weariness: 0.0,
            legitimacy: 60.0,
            strength: 0.0,
            threat: 0.0,
            recent_conquests: Vec::new(),
            treaty_breaks: 0,
            goals: Vec::new(),
            personality: spec.personality,
            regency: None,
            heir: None,
            research: [ResearchState::default(); 9],
            research_focus: None,
            ashglass: 10.0,
            soul_ash: 0.0,
            hollow_path: 0,
            hollow_value: 0.0,
            void_uses: Vec::new(),
            spy_net: BTreeMap::new(),
            court: Vec::new(),
            fleet: 0,
            is_player: false,
            embargo: Vec::new(),
            income: 0.0,
            expenses: 0.0,
            great_power: false,
            memory: Vec::new(),
            stats: RealmStats { rulers: 1, ..Default::default() },
            founded: 0,
            fallen: None,
            peace_reputation_until: 0,
            last_peace_day: 0,
            forbidden_discovered: false,
            ritual_cooldown: 0,
            last_ruler_change: 0,
            tribute_paid: 0.0,
            banner: 0,
            last_event: BTreeMap::new(),
            heresy_pressure: 0.0,
            recruit_budget: 0.0,
            excommunicated: false,
            shrines_restored: 0,
            mage_orders: Vec::new(),
            artifacts: Vec::new(),
            pending_regime: false,
        }
    }

    fn build_realms(&mut self) {
        let specs = realm_specs();
        // 1. Reiche mit Herrschern anlegen
        for (i, s) in specs.iter().enumerate() {
            let cap = self.prov_by_key(s.capital).or_else(|| s.provinces.iter().find_map(|k| self.prov_by_key(k))).unwrap_or(0);
            let culture = s.ruler_culture();
            let dynasty = self.create_dynasty(s.ruler.family, culture);
            let id = i as RealmId;
            // Platzhalter-Herrscher ID 0 wird gleich ersetzt
            let mut realm = self.blank_realm(id, s, 0, dynasty, cap);
            realm.banner = self.rng_u32();
            self.realms.push(realm);
            let ruler = self.mk_person(&s.ruler, culture, dynasty, Some(id), None, cap);
            self.realms[i].ruler = ruler;
            self.realms[i].culture = s.culture;
            self.chars[ruler as usize].realm = Some(id);
            self.chars[ruler as usize].religion = s.religion;
            if s.immortal {
                self.chars[ruler as usize].immortal = true;
                self.chars[ruler as usize].born = -(s.ruler.age as i64) * DAYS_PER_YEAR as i64;
            }
            self.chars[ruler as usize].reign_start = Some(0);
            self.chars[ruler as usize].reigns.push(id);
        }
        // Vasallenbeziehungen
        for (i, s) in specs.iter().enumerate() {
            if let Some(v) = s.vassal_of {
                self.realms[i].vassal_of = self.realm_by_key(v);
            }
        }
        // Provinzbesitz
        for (i, s) in specs.iter().enumerate() {
            for k in s.provinces {
                if let Some(p) = self.prov_by_key(k) {
                    self.provinces[p as usize].owner = Some(i as RealmId);
                }
            }
        }
        // Hauptstadt muss existieren; sonst erste Provinz
        for (i, s) in specs.iter().enumerate() {
            if self.provinces.iter().all(|p| p.owner != Some(i as RealmId)) {
                // Reich ohne Provinz (selten durch Seeds nicht möglich) – Hauptstadt der Vorlage zuweisen
                if let Some(p) = self.prov_by_key(s.capital) {
                    self.provinces[p as usize].owner = Some(i as RealmId);
                }
            }
            let owned = self.provinces.iter().find(|p| p.owner == Some(i as RealmId)).map(|p| p.id);
            if let Some(o) = owned
                && self.provinces[self.realms[i].capital as usize].owner != Some(i as RealmId) {
                    self.realms[i].capital = o;
                }
        }
        // Hauptstädte sind Hauptstädte
        for r in 0..self.realms.len() {
            let cap = self.realms[r].capital;
            self.provinces[cap as usize].settlement = match (self.provinces[cap as usize].settlement, self.realms[r].kind) {
                (_, RealmKind::Kingdom | RealmKind::Republic) => SettlementLevel::Capital,
                (SettlementLevel::Village, _) => SettlementLevel::Village,
                (s, _) => s,
            };
        }
        // Adelshäuser
        for hs in house_specs() {
            let Some(r) = self.realm_by_key(hs.realm) else { continue };
            let culture = self.realms[r as usize].culture;
            let hculture = if hs.realm == "thorn_covenant" { Culture::Dornwaldler } else { culture };
            let dyn_id = self.create_dynasty(hs.head.family, hculture);
            let hid = self.houses.len() as HouseId;
            self.dynasties[dyn_id as usize].house = Some(hid);
            let seat = hs.provinces.iter().find_map(|k| self.prov_by_key(k)).unwrap_or(self.realms[r as usize].capital);
            let head = self.mk_person(&hs.head, hculture, dyn_id, Some(r), Some(hid), seat);
            self.houses.push(NobleHouse { id: hid, name: hs.name.to_string(), realm: r, dynasty: dyn_id, head, alive: true, power: 5.0, loyalty: hs.loyalty, wealth: hs.wealth, retinue: hs.retinue, leaning: hs.leaning, rights: vec![HouseRight::CouncilSeat], throne_claim: hs.claim, faction: None, seat, cadet: false, grudge: 0.0 });
            if hs.claim {
                self.chars[head as usize].claims.push(r);
            }
            for k in hs.provinces {
                if let Some(p) = self.prov_by_key(k)
                    && self.provinces[p as usize].owner == Some(r) && self.realms[r as usize].capital != p {
                        self.provinces[p as usize].fief = Some(hid);
                    }
            }
        }
        // Familien: Herrscher und Hausoberhäupter
        self.build_families();
        // Besondere Figuren des Kanons
        self.canon_court();
        // Gesetze/Spielregeln: Spieler
        self.rebuild_caches();
    }

    fn build_families(&mut self) {
        let specs = realm_specs();
        // Herrscherfamilien (nicht Isolde/Aldric, siehe canon_court)
        for (i, s) in specs.iter().enumerate() {
            if matches!(s.key, "ember_crown" | "ashmarch_march" | "hollowed" | "ashen_legion") {
                continue;
            }
            let r = i as RealmId;
            let head = self.realms[i].ruler;
            self.random_family(head, r);
        }
        let hs: Vec<(HouseId, CharId, RealmId)> = self.houses.iter().map(|h| (h.id, h.head, h.realm)).collect();
        for (_, head, r) in hs {
            self.random_family(head, r);
        }
    }

    /// Ehepartner und Kinder für eine Figur anlegen (Wahrscheinlichkeiten nach Alter).
    fn random_family(&mut self, head: CharId, r: RealmId) {
        let ch = self.chars[head as usize].clone();
        let age = ch.age(self.day);
        if age < 22 || ch.immortal {
            return;
        }
        let married = self.chance(if age > 28 { 0.8 } else { 0.4 });
        if !married {
            return;
        }
        let g = if ch.gender == Gender::Male { Gender::Female } else { Gender::Male };
        let fam = self.random_family_name(ch.culture);
        let d = self.create_dynasty(&fam, ch.culture);
        let sage = (age + self.ri(-6, 4)).clamp(20, 70);
        let spouse = self.spawn_person(None, ch.culture, d, g, sage, Some(r), ch.house, ch.location);
        self.chars[head as usize].spouse = Some(spouse);
        self.chars[spouse as usize].spouse = Some(head);
        let max_kids = ((age - 20) / 8).clamp(0, 4);
        let n = self.ri(0, max_kids);
        for _ in 0..n {
            let kage = self.ri(1, (age - 18).max(1)).min(30);
            let gender = if self.chance(0.5) { Gender::Male } else { Gender::Female };
            let father = if ch.gender == Gender::Male { head } else { spouse };
            let mother = if ch.gender == Gender::Male { spouse } else { head };
            let dyn_id = self.chars[father as usize].dynasty;
            let kid = self.spawn_person(None, ch.culture, dyn_id, gender, kage, Some(r), ch.house, ch.location);
            self.chars[kid as usize].father = Some(father);
            self.chars[kid as usize].mother = Some(mother);
            self.chars[father as usize].children.push(kid);
            self.chars[mother as usize].children.push(kid);
            for i in 0..6 {
                let avg = (self.chars[father as usize].attrs[i] as i32 + self.chars[mother as usize].attrs[i] as i32) / 2;
                self.chars[kid as usize].attrs[i] = (avg + self.ri(-2, 2)).clamp(1, 18) as i8;
            }
        }
    }

    /// Kanonische Figuren: Königshaus, Kassian Vale, Hal Venn.
    fn canon_court(&mut self) {
        let crown = self.realm_by_key("ember_crown").unwrap();
        let aldric = self.realms[crown as usize].ruler;
        let cald = self.realms[crown as usize].dynasty;
        let cap = self.realms[crown as usize].capital;
        // Kronprinz Edric (†884) und Erbin Elyane (9)
        let edric = self.spawn_person(Some("Edric".into()), Culture::Valemarker, cald, Gender::Male, 30, Some(crown), None, cap);
        self.chars[edric as usize].died = Some((0, DeathCause::Murder));
        self.chars[edric as usize].born = -(30 + 13) * DAYS_PER_YEAR as i64;
        self.chars[edric as usize].father = Some(aldric);
        self.chars[aldric as usize].children.push(edric);
        self.court_remove(crown, edric);
        let elyane = self.spawn_person(Some("Elyane".into()), Culture::Valemarker, cald, Gender::Female, 9, Some(crown), None, cap);
        self.chars[elyane as usize].father = Some(edric);
        self.chars[edric as usize].children.push(elyane);
        self.chars[elyane as usize].attrs = [5, 8, 9, 6, 9, 10];
        self.chars[elyane as usize].traits = vec![Trait::Quick, Trait::Content];
        self.chars[elyane as usize].education = Some(Attr::Diplomacy);
        // Kassian Vale, Spionagemeister der Krone
        let vale = self.create_dynasty("Vale", Culture::Valemarker);
        let kassian = self.spawn_person(Some("Kassian".into()), Culture::Valemarker, vale, Gender::Male, 48, Some(crown), None, cap);
        self.chars[kassian as usize].attrs = [6, 8, 8, 16, 8, 7];
        self.chars[kassian as usize].traits = vec![Trait::Deceitful, Trait::Paranoid];
        let _ = self.appoint_office(crown, Office::Spymaster, kassian);
        self.realms[crown as usize].office_task[Office::Spymaster.idx()] = 1;
        // Hohepriester: Erzflammenwahrer-Vertrauter
        let pr = self.spawn_person(Some("Aveline".into()), Culture::Valemarker, self.dynasties.len() as DynId, Gender::Female, 57, Some(crown), None, cap);
        let _ = pr;
        // Isolde und Hal Venn
        let march = self.realm_by_key("ashmarch_march").unwrap();
        let isolde = self.realms[march as usize].ruler;
        self.chars[isolde as usize].is_hero = true;
        self.chars[isolde as usize].abilities = vec![Ability::Bold, Ability::Tactician];
        self.chars[isolde as usize].prestige = 450.0;
        self.chars[isolde as usize].piety = 80.0;
        let rav = self.realms[march as usize].dynasty;
        let mcap = self.realms[march as usize].capital;
        let hal_d = self.create_dynasty("Venn", Culture::Aschmaerker);
        let hal = self.spawn_person(Some("Hal".into()), Culture::Aschmaerker, hal_d, Gender::Male, 52, Some(march), None, mcap);
        self.chars[hal as usize].traits = vec![Trait::Zealous, Trait::Merciful];
        self.chars[hal as usize].attrs = [4, 7, 8, 5, 12, 8];
        let _ = self.appoint_office(march, Office::HighPriest, hal);
        self.realms[march as usize].office_task[Office::HighPriest.idx()] = 1;
        // Ein erfahrener Waffenmeister und ein Schreiber
        let dm = self.create_dynasty("Brennholt", Culture::Aschmaerker);
        let steward = self.spawn_person(None, Culture::Aschmaerker, dm, Gender::Female, 41, Some(march), None, mcap);
        self.chars[steward as usize].attrs = [6, 11, 7, 5, 8, 7];
        let _ = self.appoint_office(march, Office::Steward, steward);
        let _ = rav;
        // Höfe auffüllen
        for r in 0..self.realms.len() as RealmId {
            if matches!(self.realms[r as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            // Mindestens drei Höflinge für Ämter
            let mut n_adult = self.realms[r as usize].court.iter().filter(|c| self.chars[**c as usize].adult(self.day)).count();
            while n_adult < 6 {
                let culture = self.realms[r as usize].culture;
                let fam = self.random_family_name(culture);
                let d = self.create_dynasty(&fam, culture);
                let g = if self.chance(0.5) { Gender::Male } else { Gender::Female };
                let age = 24 + self.ri(0, 28);
                let c = self.spawn_person(None, culture, d, g, age, Some(r), None, self.realms[r as usize].capital);
                let _ = c;
                n_adult += 1;
            }
            self.fill_offices(r);
        }
        let _ = canon_note();
    }

    fn finish_setup(&mut self) {
        let opts = self.options.clone();
        // Spieler
        let player = match &opts.player {
            PlayerStart::AshmarchWarden => self.realm_by_key("ashmarch_march"),
            PlayerStart::Realm(k) => self.realm_by_key(k).filter(|r| !matches!(self.realms[*r as usize].kind, RealmKind::Hollowed | RealmKind::Legion)),
            PlayerStart::None => None,
        };
        // Herausforderung: machtloses Kadettenhaus
        self.player = player;
        if let Some(p) = player {
            self.realms[p as usize].is_player = true;
        }
        self.rebuild_caches();
        // Wirtschaft
        self.init_markets();
        for p in 0..self.provinces.len() as ProvId {
            self.populate_province(p);
            // Aschmark: Brennhollow ist die letzte Siedlung mit etwa 30 Einwohnern
            if self.provinces[p as usize].key == "brennhollow" {
                let scale = 36.0 / self.provinces[p as usize].population().max(1.0);
                for s in self.provinces[p as usize].pop.iter_mut() {
                    s.children *= scale;
                    s.adults *= scale;
                    s.old *= scale;
                }
            }
        }
        self.init_economy_warmup();
        self.init_magic();
        self.init_bosses();
        self.init_neutrals();
        self.seed_relations();
        self.seed_armies();
        // Ziele und Einstufung
        self.rebuild_caches();
        self.threat_month();
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| !matches!(r.kind, RealmKind::Hollowed | RealmKind::Legion)).map(|r| r.id).collect();
        for r in ids {
            self.choose_goals(r);
            let cap = self.realms[r as usize].capital;
            self.realms[r as usize].stats.peak_provinces = self.provinces_of(r).len() as u32;
            let _ = cap;
        }
        if let Some(p) = self.player {
            // Herausforderung „Die Leibeigene“: Spiel als machtloses Kadettenhaus
            if self.options.rules.challenge == Some(Challenge::CadetHouse) {
                self.make_cadet_start(p);
            }
        }
        // Schwierigkeit: Spieler-Startvorteil gering
        self.chronicle = Chronicle::default();
        self.events = EventState::default();
        self.stats = WorldStats::default();
        let e = Entry::new(ChronicleKind::Founding, "event_generic", 40).var("a", "Eldraeth").var("p", "Eldraeth").var("c", "Das Jahr 897 n.F. beginnt; die Hohlen nähern sich der Aschmark");
        self.chron(e);
        if let Some(p) = self.player {
            let rn = self.rname(p);
            let e = Entry::new(ChronicleKind::Founding, "event_generic", 30).realm(p).var("a", rn.clone()).var("p", rn.clone()).var("c", format!("Das Haus {} übernimmt die Verantwortung für {rn}", self.dynasties[self.realms[p as usize].dynasty as usize].name));
            self.chron(e);
        }
        self.out.clear();
    }

    /// Kadetten-Start: das Spielerreich schrumpft auf eine Provinz, Gold fast null.
    fn make_cadet_start(&mut self, p: RealmId) {
        let cap = self.realms[p as usize].capital;
        let provs: Vec<ProvId> = self.provinces_of(p).to_vec();
        for pr in provs {
            if pr != cap
                && let Some(lord) = self.realms[p as usize].vassal_of {
                    self.provinces[pr as usize].owner = Some(lord);
                }
        }
        self.realms[p as usize].gold = 80.0;
        self.rebuild_caches();
    }

    /// Wirtschaftliches Einschwingen: baut Gebäude nach Bedarf und füllt Lager, ohne die Spielzeit zu verbrauchen.
    fn init_economy_warmup(&mut self) {
        let saved_gold: Vec<f32> = self.realms.iter().map(|r| r.gold).collect();
        for r in self.realms.iter_mut() {
            r.gold = 1.0e7;
        }
        let saved_rules = self.options.rules.auto_govern_player;
        self.options.rules.auto_govern_player = true;
        // Anfangsbestände: zwei Monate Verbrauch
        for pi in 0..self.provinces.len() {
            for s in STRATA {
                let n100 = self.provinces[pi].pop[s.idx()].total() / 100.0;
                for nd in crate::goods::needs_of(s) {
                    match nd.kind {
                        crate::goods::NeedKind::Food => self.provinces[pi].stock[crate::goods::Good::Grain.idx()] += nd.per100 * n100 * 1.5,
                        crate::goods::NeedKind::Item(g) => self.provinces[pi].stock[g.idx()] += nd.per100 * n100 * 1.5,
                        _ => {}
                    }
                }
            }
        }
        for round in 0..WARMUP_MONTHS {
            self.day = (round % 12) * DAYS_PER_MONTH;
            if round % 2 == 0 {
                for pi in 0..self.provinces.len() {
                    let Some(o) = self.provinces[pi].owner else { continue };
                    if matches!(self.realms[o as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                        continue;
                    }
                    for _ in 0..6 {
                        self.govern_province(pi as ProvId, true);
                    }
                }
            }
            self.economy_month();
            self.population_core();
        }
        self.day = 0;
        // Brennhollow ist die letzte Siedlung der Aschmark: ca. 30 Einwohner
        if let Some(b) = self.prov_by_key("brennhollow") {
            let scale = 34.0 / self.provinces[b as usize].population().max(1.0);
            for s in self.provinces[b as usize].pop.iter_mut() {
                s.children *= scale;
                s.adults *= scale;
                s.old *= scale;
            }
        }
        self.options.rules.auto_govern_player = saved_rules;
        for (r, g) in self.realms.iter_mut().zip(saved_gold) {
            r.gold = g;
            r.income = 0.0;
            r.expenses = 0.0;
        }
        for p in self.provinces.iter_mut() {
            p.states.clear();
            p.chain = ChainMemory::default();
            p.plague = None;
            p.disease = 5.0;
            p.unrest = 5.0;
            p.tension = [0.0; 5];
            p.refugees = 0.0;
            p.harvest = 1.0;
            p.rainfall = 1.0;
            p.dry_months = 0;
            p.devastation = 0.0;
            p.turnover = 0.0;
            p.garrison = (p.pop[Stratum::Warriors.idx()].adults * 0.5) as u32;
            p.food_ratio = 1.0;
        }
        for m in self.markets.iter_mut() {
            m.volume = 0.0;
            m.event_factor = vec![(1.0, 0); crate::goods::Good::COUNT];
        }
        self.chronicle = Chronicle::default();
        self.stats = WorldStats::default();
        self.out.clear();
    }

    /// Startbeziehungen nach Weltbibel 4.0 und Verträge.
    fn seed_relations(&mut self) {
        let pairs: [(&str, &str, f32); 6] = [("ember_crown", "skarn_clans", -20.0), ("ember_crown", "thorn_covenant", -30.0), ("ember_crown", "saelith_compact", 10.0), ("skarn_clans", "thorn_covenant", 10.0), ("skarn_clans", "saelith_compact", 5.0), ("thorn_covenant", "saelith_compact", -10.0)];
        for (a, b, v) in pairs {
            let (Some(ra), Some(rb)) = (self.realm_by_key(a), self.realm_by_key(b)) else { continue };
            self.add_opinion(ra, rb, OpKind::Historic, v, None);
            self.add_opinion(rb, ra, OpKind::Historic, v, None);
            // alle Vasallen teilen die Haltung ihres Herrn
            for (x, y) in [(ra, rb), (rb, ra)] {
                let vs: Vec<RealmId> = self.realms.iter().filter(|r| r.vassal_of == Some(x)).map(|r| r.id).collect();
                for vv in vs {
                    self.add_opinion(vv, y, OpKind::Historic, v, None);
                    self.add_opinion(y, vv, OpKind::Historic, v, None);
                }
            }
        }
        // Handelsabkommen Krone–Saelith (Handelspartner), Nichtangriff Dornenbund–Veyl
        if let (Some(c), Some(s)) = (self.realm_by_key("ember_crown"), self.realm_by_key("saelith_compact")) {
            self.add_treaty(TreatyKind::Trade, c, s, Some(20), 0);
            self.add_opinion(c, s, OpKind::TradePact, 20.0, None);
            self.add_opinion(s, c, OpKind::TradePact, 20.0, None);
        }
        if let (Some(t), Some(v)) = (self.realm_by_key("thorn_covenant"), self.realm_by_key("veyl_free")) {
            self.add_treaty(TreatyKind::Alliance, t, v, None, 0);
        }
        // Familien- und Treuebindungen: Aschmark ist Vasall der Krone (+Meinung)
        if let (Some(m), Some(c)) = (self.realm_by_key("ashmarch_march"), self.realm_by_key("ember_crown")) {
            self.add_opinion(m, c, OpKind::Historic, 15.0, None);
            self.add_opinion(c, m, OpKind::Historic, 15.0, None);
        }
        // Raventhorn–Corvane-Streit um das Rabenwappen
        if let (Some(m), Some(c)) = (self.realm_by_key("ashmarch_march"), self.realm_by_key("ember_crown")) {
            let _ = (m, c);
        }
    }

    fn seed_armies(&mut self) {
        for r in 0..self.realms.len() as RealmId {
            let kind = self.realms[r as usize].kind;
            if matches!(kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            let provs = self.provinces_of(r).to_vec();
            if provs.is_empty() {
                continue;
            }
            let total_pop: f32 = provs.iter().map(|p| self.provinces[*p as usize].population()).sum();
            let frac = if self.realms[r as usize].kind == RealmKind::March { 0.0 } else { 0.014 };
            let mut soldiers = (total_pop * frac) as u32;
            if self.realms[r as usize].key == "ashmarch_march" {
                soldiers = 41; // „einundvierzig Bewaffnete“
            }
            if soldiers < 20 {
                continue;
            }
            let culture = self.realms[r as usize].culture;
            let roster = catalog::roster(culture);
            let pick = |class: UClass| -> Vec<&str> { roster.iter().copied().filter(|id| catalog::unit(id).is_some_and(|u| u.class == class && u.cost > 0.0)).collect() };
            let (inf, rng, cav) = (pick(UClass::Infantry), pick(UClass::Ranged), pick(UClass::Cavalry));
            let mut regs = Vec::new();
            let mut left = soldiers;
            if let Some(u) = inf.first() {
                let n = soldiers * 5 / 10;
                regs.push(Regiment { unit: u.to_string(), count: n, vet: 1, hp: 1.0 });
                left -= n;
            }
            if let Some(u) = inf.get(1) {
                let n = soldiers * 2 / 10;
                regs.push(Regiment { unit: u.to_string(), count: n, vet: 0, hp: 1.0 });
                left -= n;
            }
            if let Some(u) = rng.first() {
                let n = soldiers * 2 / 10;
                regs.push(Regiment { unit: u.to_string(), count: n, vet: 0, hp: 1.0 });
                left -= n;
            }
            if let Some(u) = cav.first() {
                regs.push(Regiment { unit: u.to_string(), count: left.max(1).min(soldiers / 10 + 1), vet: 1, hp: 1.0 });
            }
            if regs.is_empty() {
                regs.push(Regiment { unit: "worker".into(), count: soldiers, vet: 0, hp: 1.0 });
            }
            regs.retain(|g| g.count > 0);
            let cap = self.realms[r as usize].capital;
            let name = format!("Heer von {}", self.rname(r));
            // Isolde führt ihr Aufgebot selbst
            let aid = self.new_army(ArmyKind::Realm, Some(r), cap, regs, &name);
            if self.realms[r as usize].key == "ashmarch_march" {
                let isolde = self.realms[r as usize].ruler;
                if let Some(a) = self.armies.iter_mut().find(|a| a.id == aid) {
                    a.commander = Some(isolde);
                }
                self.chars[isolde as usize].army = Some(aid);
            } else {
                self.assign_commander(aid);
            }
            // Die Hälfte der Truppen wird aus der Bevölkerung genommen
            let p = &mut self.provinces[cap as usize];
            p.pop[Stratum::Warriors.idx()].adults = (p.pop[Stratum::Warriors.idx()].adults - soldiers as f32 * 0.3).max(0.0);
        }
    }
}

impl RealmSpec {
    fn ruler_culture(&self) -> Culture {
        match self.key {
            "thorn_covenant" | "veyl_free" => Culture::Veyl,
            _ => self.culture,
        }
    }
}
