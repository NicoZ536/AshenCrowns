//! Grundtypen: IDs, Kalender, Biome, Kulturen, Religionen, Schichten, Ämter, Schulen.

use serde::{Deserialize, Serialize};

pub type ProvId = u16;
pub type RealmId = u16;
pub type HouseId = u16;
pub type DynId = u16;
pub type CharId = u32;
pub type ArmyId = u32;
pub type WarId = u32;
pub type PlotId = u32;
pub type BattleId = u32;

pub const DAYS_PER_MONTH: u32 = 30;
pub const DAYS_PER_YEAR: u32 = 360;
/// Startjahr der Kanon-Welt (n.F.).
pub const START_YEAR: i32 = 897;

pub const MONTH_NAMES: [&str; 12] = [
    "Frostmond",
    "Schmelzmond",
    "Saatmond",
    "Blütenmond",
    "Grünmond",
    "Sonnenmond",
    "Heumond",
    "Erntemond",
    "Fahlmond",
    "Nebelmond",
    "Aschmond",
    "Dunkelmond",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

impl Season {
    pub fn from_month(m: u32) -> Season {
        match m % 12 {
            11 | 0 | 1 => Season::Winter,
            2..=4 => Season::Spring,
            5..=7 => Season::Summer,
            _ => Season::Autumn,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Season::Winter => "Winter",
            Season::Spring => "Frühling",
            Season::Summer => "Sommer",
            Season::Autumn => "Herbst",
        }
    }
    /// Saisonfaktor der Feldfrüchte (Mittelwert über das Jahr = 1).
    pub fn crop_factor(self, month: u32) -> f32 {
        match self {
            Season::Winter => 0.0,
            Season::Spring => 0.8,
            Season::Summer => 1.2,
            Season::Autumn => {
                if (month % 12) >= 8 {
                    2.0
                } else {
                    1.6
                }
            }
        }
    }
}

/// Kalenderdatum in Spielzeit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn from_day(day: u32) -> Date {
        Date { year: START_YEAR + (day / DAYS_PER_YEAR) as i32, month: ((day / DAYS_PER_MONTH) % 12) as u8, day: (day % DAYS_PER_MONTH) as u8 }
    }
    pub fn month_name(&self) -> &'static str {
        MONTH_NAMES[self.month as usize % 12]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biome {
    Plains,
    Hills,
    Forest,
    Mountain,
    Swamp,
    Desert,
    Coast,
    Island,
    Volcanic,
    Tundra,
    Ruins,
    Hollowland,
}

impl Biome {
    pub fn label(self) -> &'static str {
        match self {
            Biome::Plains => "Ebene",
            Biome::Hills => "Hügelland",
            Biome::Forest => "Wald",
            Biome::Mountain => "Gebirge",
            Biome::Swamp => "Sumpf",
            Biome::Desert => "Wüste",
            Biome::Coast => "Küste",
            Biome::Island => "Insel",
            Biome::Volcanic => "Vulkanland",
            Biome::Tundra => "Tundra",
            Biome::Ruins => "Ruinenland",
            Biome::Hollowland => "Hohlland",
        }
    }
    /// Geländekosten für Märsche (Tage-Multiplikator).
    pub fn move_cost(self) -> f32 {
        match self {
            Biome::Plains | Biome::Coast => 1.0,
            Biome::Hills | Biome::Tundra => 1.3,
            Biome::Forest | Biome::Ruins => 1.35,
            Biome::Mountain => 1.7,
            Biome::Swamp => 1.6,
            Biome::Desert => 1.4,
            Biome::Volcanic => 1.7,
            Biome::Island => 1.0,
            Biome::Hollowland => 1.5,
        }
    }
    /// Verteidigungsbonus im Auto-Resolve (Gelände).
    pub fn defense_bonus(self) -> f32 {
        match self {
            Biome::Mountain => 1.25,
            Biome::Hills | Biome::Forest | Biome::Swamp => 1.12,
            Biome::Ruins | Biome::Volcanic => 1.08,
            _ => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Culture {
    Valemarker,
    Aschmaerker,
    Skarn,
    Dornwaldler,
    Veyl,
    Saelith,
    GraueNomaden,
    Schlackenbrut,
    Hohl,
    Aurethisch,
}

impl Culture {
    pub fn label(self) -> &'static str {
        match self {
            Culture::Valemarker => "Valemarker",
            Culture::Aschmaerker => "Aschmärker",
            Culture::Skarn => "Skarn",
            Culture::Dornwaldler => "Dornwaldler",
            Culture::Veyl => "Veyl",
            Culture::Saelith => "Saelith",
            Culture::GraueNomaden => "Graue Nomaden",
            Culture::Schlackenbrut => "Schlackenbrut",
            Culture::Hohl => "Hohle",
            Culture::Aurethisch => "Aurethisch",
        }
    }
    /// Verwandte Kulturen zählen für die Meinung als „halb gleich“.
    pub fn related(self, o: Culture) -> bool {
        use Culture::*;
        matches!((self, o), (Valemarker, Aschmaerker) | (Aschmaerker, Valemarker) | (Dornwaldler, Veyl) | (Veyl, Dornwaldler) | (Aurethisch, GraueNomaden) | (GraueNomaden, Aurethisch))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Religion {
    EternalFlame,
    AncestorPath,
    GreenCircle,
    TideHouse,
    HollowLiturgy,
    SunHeresy,
}

pub const RELIGIONS: [Religion; 6] = [Religion::EternalFlame, Religion::AncestorPath, Religion::GreenCircle, Religion::TideHouse, Religion::HollowLiturgy, Religion::SunHeresy];

impl Religion {
    pub fn idx(self) -> usize {
        self as usize
    }
    pub fn id(self) -> &'static str {
        match self {
            Religion::EternalFlame => "eternal_flame",
            Religion::AncestorPath => "ancestor_path",
            Religion::GreenCircle => "green_circle",
            Religion::TideHouse => "tide_house",
            Religion::HollowLiturgy => "hollow_liturgy",
            Religion::SunHeresy => "sun_heresy",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Religion::EternalFlame => "Glaube der Ewigen Flamme",
            Religion::AncestorPath => "Ahnenpfad",
            Religion::GreenCircle => "Grüner Kreis",
            Religion::TideHouse => "Haus der Gezeiten",
            Religion::HollowLiturgy => "Leere Liturgie",
            Religion::SunHeresy => "Sonnenhäresie",
        }
    }
    /// Feiertag (Monat, Name).
    pub fn feast(self) -> (u8, &'static str) {
        match self {
            Religion::EternalFlame => (10, "Fest der Ewigen Flamme"),
            Religion::AncestorPath => (0, "Ahnennacht"),
            Religion::GreenCircle => (4, "Tag der Wurzeln"),
            Religion::TideHouse => (6, "Gezeitenfest"),
            Religion::HollowLiturgy => (11, "Nacht der Leere"),
            Religion::SunHeresy => (5, "Sonnenwende des Erlösers"),
        }
    }
    /// 1 = gleich, 0 = verwandt, -1 = verfeindet, 2 = neutral/andere.
    pub fn relation(self, o: Religion) -> i8 {
        use Religion::*;
        if self == o {
            return 1;
        }
        match (self, o) {
            (EternalFlame, HollowLiturgy) | (HollowLiturgy, EternalFlame) => -1,
            (EternalFlame, SunHeresy) | (SunHeresy, EternalFlame) => 0,
            (GreenCircle, HollowLiturgy) | (HollowLiturgy, GreenCircle) => -1,
            (AncestorPath, HollowLiturgy) | (HollowLiturgy, AncestorPath) => -1,
            (TideHouse, HollowLiturgy) | (HollowLiturgy, TideHouse) => -1,
            (SunHeresy, HollowLiturgy) | (HollowLiturgy, SunHeresy) => -1,
            (AncestorPath, GreenCircle) | (GreenCircle, AncestorPath) => 0,
            _ => 2,
        }
    }
    pub fn forbidden_by_default(self) -> bool {
        self == Religion::HollowLiturgy
    }
}

/// Die acht Bevölkerungsschichten.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Stratum {
    Serfs,
    Freeholders,
    Artisans,
    Burghers,
    Clergy,
    Scholars,
    Warriors,
    Nobility,
}

pub const STRATA: [Stratum; 8] = [Stratum::Serfs, Stratum::Freeholders, Stratum::Artisans, Stratum::Burghers, Stratum::Clergy, Stratum::Scholars, Stratum::Warriors, Stratum::Nobility];

impl Stratum {
    pub fn idx(self) -> usize {
        self as usize
    }
    pub fn id(self) -> &'static str {
        match self {
            Stratum::Serfs => "serfs",
            Stratum::Freeholders => "freeholders",
            Stratum::Artisans => "artisans",
            Stratum::Burghers => "burghers",
            Stratum::Clergy => "clergy",
            Stratum::Scholars => "scholars",
            Stratum::Warriors => "warriors",
            Stratum::Nobility => "nobility",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Stratum::Serfs => "Leibeigene",
            Stratum::Freeholders => "Freibauern",
            Stratum::Artisans => "Handwerker",
            Stratum::Burghers => "Bürger",
            Stratum::Clergy => "Klerus",
            Stratum::Scholars => "Gelehrte",
            Stratum::Warriors => "Krieger",
            Stratum::Nobility => "Adel",
        }
    }
    /// Kopfsteuer pro Monat bei mittlerem Satz (Gold je 10 Bewohner, damit Provinzzahlen sinnvoll bleiben).
    pub fn head_tax(self) -> f32 {
        match self {
            Stratum::Serfs => 0.1,
            Stratum::Freeholders => 0.3,
            Stratum::Artisans => 0.6,
            Stratum::Burghers => 1.2,
            Stratum::Clergy => 0.0,
            Stratum::Scholars => 0.8,
            Stratum::Warriors => 0.0,
            Stratum::Nobility => 3.0,
        }
    }
}

/// Die sieben Ämter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Office {
    Chancellor,
    Marshal,
    Treasurer,
    Diplomat,
    Spymaster,
    HighPriest,
    Steward,
}

pub const OFFICES: [Office; 7] = [Office::Chancellor, Office::Marshal, Office::Treasurer, Office::Diplomat, Office::Spymaster, Office::HighPriest, Office::Steward];

impl Office {
    pub fn idx(self) -> usize {
        self as usize
    }
    pub fn id(self) -> &'static str {
        match self {
            Office::Chancellor => "office_chancellor",
            Office::Marshal => "office_marshal",
            Office::Treasurer => "office_treasurer",
            Office::Diplomat => "office_diplomat",
            Office::Spymaster => "office_spymaster",
            Office::HighPriest => "office_high_priest",
            Office::Steward => "office_steward",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Office::Chancellor => "Kanzler",
            Office::Marshal => "Marschall",
            Office::Treasurer => "Schatzmeister",
            Office::Diplomat => "Diplomat",
            Office::Spymaster => "Spionagemeister",
            Office::HighPriest => "Hohepriester",
            Office::Steward => "Statthalter",
        }
    }
    /// Maßgebliches Attribut (Index in `Attr`).
    pub fn attr(self) -> Attr {
        match self {
            Office::Chancellor => Attr::Diplomacy,
            Office::Marshal => Attr::Martial,
            Office::Treasurer => Attr::Stewardship,
            Office::Diplomat => Attr::Diplomacy,
            Office::Spymaster => Attr::Intrigue,
            Office::HighPriest => Attr::Learning,
            Office::Steward => Attr::Stewardship,
        }
    }
    /// Aufgaben (aktiv ist immer eine) – Namen für die Oberfläche.
    pub fn tasks(self) -> &'static [&'static str] {
        match self {
            Office::Chancellor => &["Beziehungen pflegen", "Ansprüche erarbeiten", "Gesetze verwalten"],
            Office::Marshal => &["Truppen drillen", "Rekrutierung leiten", "Befestigungen inspizieren"],
            Office::Treasurer => &["Steuern eintreiben", "Handel fördern", "Korruption bekämpfen"],
            Office::Diplomat => &["Gesandtschaft", "Bündnis schmieden", "Frieden vermitteln"],
            Office::Spymaster => &["Komplotte aufdecken", "Spionagenetz", "Sabotage"],
            Office::HighPriest => &["Glauben verbreiten", "Hohlung reinigen", "Häresie verfolgen"],
            Office::Steward => &["Ausbau", "Ordnung", "Erntepflege"],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Attr {
    Martial,
    Stewardship,
    Diplomacy,
    Intrigue,
    Learning,
    Charisma,
}

pub const ATTRS: [Attr; 6] = [Attr::Martial, Attr::Stewardship, Attr::Diplomacy, Attr::Intrigue, Attr::Learning, Attr::Charisma];

impl Attr {
    pub fn idx(self) -> usize {
        self as usize
    }
    pub fn name(self) -> &'static str {
        match self {
            Attr::Martial => "Kriegskunst",
            Attr::Stewardship => "Verwaltung",
            Attr::Diplomacy => "Diplomatie",
            Attr::Intrigue => "Intrige",
            Attr::Learning => "Gelehrsamkeit",
            Attr::Charisma => "Charisma",
        }
    }
}

/// Die neun Magieschulen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum School {
    Ember,
    Order,
    Passage,
    Growth,
    Tide,
    Stone,
    Crystal,
    Shadow,
    Void,
}

pub const SCHOOLS: [School; 9] = [School::Ember, School::Order, School::Passage, School::Growth, School::Tide, School::Stone, School::Crystal, School::Shadow, School::Void];

impl School {
    pub fn idx(self) -> usize {
        self as usize
    }
    pub fn id(self) -> &'static str {
        match self {
            School::Ember => "school_ember",
            School::Order => "school_order",
            School::Passage => "school_passage",
            School::Growth => "school_growth",
            School::Tide => "school_tide",
            School::Stone => "school_stone",
            School::Crystal => "school_crystal",
            School::Shadow => "school_shadow",
            School::Void => "school_void",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            School::Ember => "Glutmagie",
            School::Order => "Ordnungsmagie Ithrens",
            School::Passage => "Übergangskunst Morvaines",
            School::Growth => "Wachstumsmagie Calyx'",
            School::Tide => "Gezeitenmagie Thals",
            School::Stone => "Steinkunst Oruuns",
            School::Crystal => "Aschglas-Arkanistik",
            School::Shadow => "Schattenkunst",
            School::Void => "Leermagie",
        }
    }
    /// Verbotene Schulen: Entdeckungsrisiko bei der Forschung.
    pub fn forbidden(self) -> bool {
        matches!(self, School::Shadow | School::Void)
    }
    /// Kultur, die diese Schule verehrt (25 % billiger).
    pub fn revered_by(self) -> Option<Culture> {
        match self {
            School::Ember => Some(Culture::Valemarker),
            School::Growth => Some(Culture::Dornwaldler),
            School::Tide => Some(Culture::Saelith),
            School::Stone => Some(Culture::Skarn),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementLevel {
    Village,
    Town,
    City,
    Capital,
}

impl SettlementLevel {
    pub fn name(self) -> &'static str {
        match self {
            SettlementLevel::Village => "Dorf",
            SettlementLevel::Town => "Stadt",
            SettlementLevel::City => "Großstadt",
            SettlementLevel::Capital => "Hauptstadt",
        }
    }
    pub fn rank(self) -> u8 {
        self as u8
    }
    pub fn housing(self) -> f32 {
        match self {
            SettlementLevel::Village => 700.0,
            SettlementLevel::Town => 2400.0,
            SettlementLevel::City => 6000.0,
            SettlementLevel::Capital => 11000.0,
        }
    }
    pub fn next(self) -> Option<SettlementLevel> {
        match self {
            SettlementLevel::Village => Some(SettlementLevel::Town),
            SettlementLevel::Town => Some(SettlementLevel::City),
            SettlementLevel::City => Some(SettlementLevel::Capital),
            SettlementLevel::Capital => None,
        }
    }
}

/// Rangstufe eines Herrschers (Heiratsformel).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Marcher = 1,
    Count = 2,
    Duke = 3,
    King = 4,
    HighKing = 5,
}

impl Rank {
    pub fn name(self) -> &'static str {
        match self {
            Rank::Marcher => "Grenzherr",
            Rank::Count => "Graf",
            Rank::Duke => "Herzog",
            Rank::King => "König",
            Rank::HighKing => "Hochkönig",
        }
    }
}

pub fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    v.max(lo).min(hi)
}
