//! Waren, Produktionsketten, Gebäude und Bedürfnisse (GDD Kapitel 8 und 9).
//!
//! Alle Tabellen sind statisch. Mengen gelten pro Monat (1 Welttag = 8 s RTS-Zeit, 1 Monat = 240 s).

use crate::types::Stratum;
use serde::{Deserialize, Serialize};

macro_rules! goods {
    ($( $v:ident, $id:expr, $name:expr, $tier:expr, $price:expr, $nut:expr, $decay:expr; )*) => {
        /// Eine Ware der Wirtschaft (IDs wie im GDD).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Good { $( $v ),* }
        impl Good {
            pub const ALL: &'static [Good] = &[ $( Good::$v ),* ];
            pub const COUNT: usize = Good::ALL.len();
            pub fn idx(self) -> usize { self as usize }
            pub fn id(self) -> &'static str { match self { $( Good::$v => $id ),* } }
            pub fn name(self) -> &'static str { match self { $( Good::$v => $name ),* } }
            pub fn tier(self) -> u8 { match self { $( Good::$v => $tier ),* } }
            pub fn base_price(self) -> f32 { match self { $( Good::$v => $price ),* } }
            pub fn nutrition(self) -> f32 { match self { $( Good::$v => $nut ),* } }
            /// Lagerverlust pro Monat.
            pub fn decay(self) -> f32 { match self { $( Good::$v => $decay ),* } }
            pub fn from_id(s: &str) -> Option<Good> { match s { $( $id => Some(Good::$v), )* _ => None } }
        }
    };
}

goods! {
    Wood, "wood", "Holz", 1, 2.0, 0.0, 0.005;
    Stone, "stone", "Stein", 1, 3.0, 0.0, 0.0;
    Grain, "grain", "Getreide", 1, 2.0, 1.0, 0.005;
    Fish, "fish", "Fisch", 1, 2.0, 1.0, 0.10;
    Game, "game", "Wild", 1, 2.0, 1.0, 0.10;
    Livestock, "livestock", "Vieh", 1, 8.0, 0.0, 0.01;
    Wool, "wool", "Wolle", 1, 3.0, 0.0, 0.005;
    Flax, "flax", "Flachs", 1, 2.0, 0.0, 0.01;
    Hops, "hops", "Hopfen", 1, 3.0, 0.0, 0.02;
    Grapes, "grapes", "Trauben", 1, 3.0, 0.0, 0.10;
    Herbs, "herbs", "Kräuter", 1, 3.0, 0.0, 0.04;
    Wax, "wax", "Wachs", 1, 4.0, 0.0, 0.0;
    Salt, "salt", "Salz", 1, 4.0, 0.0, 0.0;
    Clay, "clay", "Ton", 1, 1.0, 0.0, 0.0;
    IronOre, "iron_ore", "Eisenerz", 1, 4.0, 0.0, 0.0;
    Coal, "coal", "Kohle", 1, 3.0, 0.0, 0.0;
    SilverOre, "silver_ore", "Silbererz", 1, 20.0, 0.0, 0.0;
    Sulfur, "sulfur", "Schwefel", 1, 6.0, 0.0, 0.0;
    AshglassRaw, "ashglass_raw", "Rohaschglas", 1, 40.0, 0.0, 0.0;
    Planks, "planks", "Bretter", 2, 4.0, 0.0, 0.0;
    CutStone, "cut_stone", "Werkstein", 2, 6.0, 0.0, 0.0;
    Bricks, "bricks", "Ziegel", 2, 3.0, 0.0, 0.0;
    Flour, "flour", "Mehl", 2, 3.0, 0.0, 0.02;
    Malt, "malt", "Malz", 2, 4.0, 0.0, 0.01;
    Meat, "meat", "Fleisch", 2, 5.0, 2.0, 0.08;
    Hides, "hides", "Häute", 2, 4.0, 0.0, 0.01;
    Leather, "leather", "Leder", 2, 9.0, 0.0, 0.0;
    Yarn, "yarn", "Garn", 2, 5.0, 0.0, 0.0;
    Linen, "linen", "Leinen", 2, 6.0, 0.0, 0.0;
    Charcoal, "charcoal", "Holzkohle", 2, 3.0, 0.0, 0.0;
    Iron, "iron", "Eisenbarren", 2, 12.0, 0.0, 0.0;
    Silver, "silver", "Silberbarren", 2, 60.0, 0.0, 0.0;
    Parchment, "parchment", "Pergament", 2, 5.0, 0.0, 0.0;
    Ashglass, "ashglass", "Aschglas", 2, 110.0, 0.0, 0.0;
    Bread, "bread", "Brot", 3, 3.0, 2.0, 0.05;
    PreservedFood, "preserved_food", "Pökelware", 3, 5.0, 1.5, 0.005;
    Beer, "beer", "Bier", 3, 4.0, 0.0, 0.02;
    Wine, "wine", "Wein", 3, 12.0, 0.0, 0.0;
    Cloth, "cloth", "Tuch", 3, 12.0, 0.0, 0.0;
    Clothing, "clothing", "Kleidung", 3, 20.0, 0.0, 0.0;
    Tools, "tools", "Werkzeug", 3, 14.0, 0.0, 0.0;
    Weapons, "weapons", "Waffen", 3, 30.0, 0.0, 0.0;
    Armor, "armor", "Rüstung", 3, 55.0, 0.0, 0.0;
    Candles, "candles", "Kerzen", 3, 4.0, 0.0, 0.0;
    Books, "books", "Bücher", 3, 45.0, 0.0, 0.0;
    Remedies, "remedies", "Heilmittel", 3, 10.0, 0.0, 0.0;
    Furniture, "furniture", "Möbel", 3, 30.0, 0.0, 0.0;
    FineGarments, "fine_garments", "Feingewänder", 4, 160.0, 0.0, 0.0;
    Jewelry, "jewelry", "Schmuck", 4, 220.0, 0.0, 0.0;
    Spices, "spices", "Gewürze", 4, 35.0, 0.0, 0.0;
    Silk, "silk", "Seide", 4, 50.0, 0.0, 0.0;
    Relics, "relics", "Reliquien", 4, 300.0, 0.0, 0.0;
    AshglassFocus, "ashglass_focus", "Aschglas-Fokus", 4, 400.0, 0.0, 0.0;
    Warhorse, "warhorse", "Schlachtross", 4, 120.0, 0.0, 0.0;
    SiegeParts, "siege_parts", "Belagerungsteile", 4, 60.0, 0.0, 0.0;
    AlchemicalFire, "alchemical_fire", "Alchemistenfeuer", 4, 70.0, 0.0, 0.0;
    SoulAsh, "soul_ash", "Seelenasche", 4, 80.0, 0.0, 0.0;
}

/// Nahrungsmittel in Verbrauchsreihenfolge (beste zuerst).
pub const FOODS: [Good; 6] = [Good::Bread, Good::Meat, Good::PreservedFood, Good::Fish, Good::Game, Good::Grain];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceKind {
    Temple,
    Hospital,
    Library,
    Marketplace,
    Harbor,
    Barracks,
    Watchtower,
    Housing,
    MageTower,
    Granary,
    Wells,
}

#[derive(Clone, Copy, Debug)]
pub enum BKind {
    Production,
    Service(ServiceKind),
}

/// Gebäudedefinition: Produktion (mit Rezept) oder Dienstgebäude.
#[derive(Clone, Copy, Debug)]
pub struct BuildingDef {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: BKind,
    pub inputs: &'static [(Good, f32)],
    pub outputs: &'static [(Good, f32)],
    /// Zykluszeit in RTS-Sekunden.
    pub time: f32,
    pub stratum: Stratum,
    pub workers: u8,
    /// Benötigtes Vorkommen in der Provinz.
    pub deposit: Option<Good>,
    pub cost: f32,
    pub upkeep: f32,
}

#[allow(clippy::too_many_arguments)]
const fn prod(
    id: &'static str,
    name: &'static str,
    inputs: &'static [(Good, f32)],
    outputs: &'static [(Good, f32)],
    time: f32,
    stratum: Stratum,
    workers: u8,
    deposit: Option<Good>,
    cost: f32,
) -> BuildingDef {
    BuildingDef { id, name, kind: BKind::Production, inputs, outputs, time, stratum, workers, deposit, cost, upkeep: 1.0 }
}

const fn svc(id: &'static str, name: &'static str, k: ServiceKind, cost: f32, upkeep: f32, stratum: Stratum, workers: u8) -> BuildingDef {
    BuildingDef { id, name, kind: BKind::Service(k), inputs: &[], outputs: &[], time: 0.0, stratum, workers, deposit: None, cost, upkeep }
}

use Good::*;
use Stratum::*;

pub const BUILDINGS: &[BuildingDef] = &[
    // Gewinnung
    prod("woodcutter", "Holzfällerhütte", &[], &[(Wood, 4.0)], 30.0, Freeholders, 2, Some(Wood), 40.0),
    prod("quarry", "Steinbruch", &[], &[(Stone, 3.0)], 40.0, Serfs, 3, Some(Stone), 50.0),
    prod("farmstead", "Gehöft", &[], &[(Grain, 4.0)], 40.0, Serfs, 4, Some(Grain), 40.0),
    prod("fishery", "Fischerhütte", &[], &[(Fish, 3.0)], 30.0, Freeholders, 2, Some(Fish), 35.0),
    prod("hunting_lodge", "Jagdhütte", &[], &[(Game, 2.0)], 35.0, Freeholders, 1, Some(Game), 30.0),
    prod("pasture", "Viehweide", &[(Grain, 2.0)], &[(Livestock, 2.0)], 60.0, Freeholders, 2, Some(Livestock), 45.0),
    prod("sheep_farm", "Schäferei", &[], &[(Wool, 2.0)], 50.0, Freeholders, 2, Some(Wool), 45.0),
    prod("flax_farm", "Flachsfeld", &[], &[(Flax, 3.0)], 50.0, Serfs, 2, Some(Flax), 35.0),
    prod("hop_garden", "Hopfengarten", &[], &[(Hops, 2.0)], 45.0, Freeholders, 2, Some(Hops), 40.0),
    prod("vineyard", "Weinberg", &[], &[(Grapes, 3.0)], 60.0, Freeholders, 3, Some(Grapes), 60.0),
    prod("herb_garden", "Kräutergarten", &[], &[(Herbs, 2.0)], 40.0, Freeholders, 1, Some(Herbs), 30.0),
    prod("apiary", "Imkerei", &[], &[(Wax, 2.0)], 60.0, Freeholders, 1, Some(Wax), 30.0),
    prod("saltworks", "Saline", &[], &[(Salt, 2.0)], 45.0, Serfs, 2, Some(Salt), 45.0),
    prod("clay_pit", "Tongrube", &[], &[(Clay, 3.0)], 30.0, Serfs, 2, Some(Clay), 30.0),
    prod("iron_mine", "Eisenmine", &[], &[(IronOre, 2.0)], 40.0, Serfs, 4, Some(IronOre), 80.0),
    prod("coal_mine", "Kohlenmine", &[], &[(Coal, 3.0)], 40.0, Serfs, 4, Some(Coal), 80.0),
    prod("silver_mine", "Silbermine", &[], &[(SilverOre, 1.0)], 60.0, Serfs, 4, Some(SilverOre), 120.0),
    prod("sulfur_pit", "Schwefelgrube", &[], &[(Sulfur, 2.0)], 50.0, Serfs, 3, Some(Sulfur), 70.0),
    prod("ashglass_mine", "Aschglasschacht", &[], &[(AshglassRaw, 1.0)], 90.0, Serfs, 3, Some(AshglassRaw), 150.0),
    prod("spice_plantation", "Gewürzplantage", &[], &[(Spices, 2.0)], 60.0, Freeholders, 3, Some(Spices), 120.0),
    prod("silk_house", "Seidenhaus", &[], &[(Silk, 1.0)], 60.0, Artisans, 2, Some(Silk), 130.0),
    // Verarbeitung
    prod("windmill", "Windmühle", &[(Grain, 2.0)], &[(Flour, 2.0)], 20.0, Freeholders, 1, None, 60.0),
    prod("bakery", "Bäckerei", &[(Flour, 2.0)], &[(Bread, 4.0)], 30.0, Artisans, 2, None, 60.0),
    prod("smokehouse", "Räucherei", &[(Fish, 2.0), (Salt, 1.0)], &[(PreservedFood, 3.0)], 40.0, Artisans, 2, None, 55.0),
    prod("butcher", "Metzgerei", &[(Livestock, 1.0)], &[(Meat, 3.0), (Hides, 1.0)], 30.0, Artisans, 2, None, 55.0),
    prod("tannery", "Gerberei", &[(Hides, 2.0), (Salt, 1.0)], &[(Leather, 2.0)], 40.0, Artisans, 2, None, 60.0),
    prod("malthouse", "Mälzerei", &[(Grain, 2.0)], &[(Malt, 2.0)], 30.0, Artisans, 1, None, 45.0),
    prod("brewery", "Brauerei", &[(Malt, 2.0), (Hops, 1.0)], &[(Beer, 4.0)], 60.0, Artisans, 2, None, 70.0),
    prod("winery", "Kelterei", &[(Grapes, 3.0)], &[(Wine, 2.0)], 90.0, Artisans, 2, None, 70.0),
    prod("sawmill", "Sägewerk", &[(Wood, 2.0)], &[(Planks, 3.0)], 30.0, Artisans, 2, None, 60.0),
    prod("stonemason", "Steinmetz", &[(Stone, 2.0)], &[(CutStone, 2.0)], 40.0, Artisans, 2, None, 60.0),
    prod("charcoal_burner", "Köhlerei", &[(Wood, 3.0)], &[(Charcoal, 2.0)], 60.0, Freeholders, 1, None, 35.0),
    prod("brickworks", "Ziegelei", &[(Clay, 2.0), (Charcoal, 1.0)], &[(Bricks, 4.0)], 45.0, Artisans, 2, None, 60.0),
    prod("carpenter", "Schreinerei", &[(Planks, 3.0), (Tools, 1.0)], &[(Furniture, 2.0)], 60.0, Artisans, 2, None, 65.0),
    prod("smelter", "Schmelzhütte", &[(IronOre, 2.0), (Charcoal, 2.0)], &[(Iron, 2.0)], 50.0, Artisans, 2, None, 90.0),
    prod("toolsmith", "Werkzeugschmiede", &[(Iron, 1.0), (Planks, 1.0)], &[(Tools, 2.0)], 45.0, Artisans, 2, None, 80.0),
    prod("weaponsmith", "Waffenschmiede", &[(Iron, 2.0), (Planks, 1.0)], &[(Weapons, 2.0)], 60.0, Artisans, 2, None, 90.0),
    prod("armorer", "Rüstungsschmiede", &[(Iron, 3.0), (Leather, 1.0)], &[(Armor, 1.0)], 75.0, Artisans, 3, None, 100.0),
    prod("silver_smelter", "Silberhütte", &[(SilverOre, 2.0), (Charcoal, 1.0)], &[(Silver, 1.0)], 60.0, Artisans, 2, None, 90.0),
    prod("jeweler", "Juwelier", &[(Silver, 1.0), (Ashglass, 1.0)], &[(Jewelry, 1.0)], 120.0, Artisans, 2, None, 110.0),
    prod("spinnery", "Spinnerei", &[(Wool, 2.0)], &[(Yarn, 2.0)], 30.0, Artisans, 2, None, 50.0),
    prod("weaver", "Weberei", &[(Yarn, 2.0)], &[(Cloth, 1.0)], 40.0, Artisans, 2, None, 55.0),
    prod("linen_weaver", "Leinenweberei", &[(Flax, 3.0)], &[(Linen, 2.0)], 45.0, Artisans, 2, None, 55.0),
    prod("tailor", "Schneiderei", &[(Cloth, 1.0), (Leather, 1.0)], &[(Clothing, 2.0)], 50.0, Artisans, 2, None, 55.0),
    prod("couturier", "Gewandmeisterei", &[(Linen, 1.0), (Silk, 1.0)], &[(FineGarments, 1.0)], 90.0, Artisans, 2, None, 120.0),
    prod("chandler", "Kerzenzieherei", &[(Wax, 2.0)], &[(Candles, 4.0)], 40.0, Artisans, 1, None, 40.0),
    prod("parchment_maker", "Pergamenterei", &[(Hides, 2.0)], &[(Parchment, 3.0)], 50.0, Artisans, 1, None, 45.0),
    prod("scriptorium", "Skriptorium", &[(Parchment, 2.0), (Candles, 1.0)], &[(Books, 1.0)], 120.0, Scholars, 2, None, 120.0),
    prod("apothecary", "Apotheke", &[(Herbs, 2.0), (Beer, 1.0)], &[(Remedies, 2.0)], 60.0, Scholars, 1, None, 70.0),
    prod("ashglass_cutter", "Aschglasschleiferei", &[(AshglassRaw, 2.0)], &[(Ashglass, 1.0)], 120.0, Scholars, 1, None, 130.0),
    prod("arcane_workshop", "Arkanwerkstatt", &[(Ashglass, 2.0), (Silver, 1.0)], &[(AshglassFocus, 1.0)], 180.0, Scholars, 2, None, 200.0),
    prod("alchemist", "Alchemistenlabor", &[(Sulfur, 2.0), (AshglassRaw, 1.0), (Charcoal, 1.0)], &[(AlchemicalFire, 2.0)], 120.0, Scholars, 2, None, 160.0),
    prod("stud_farm", "Gestüt", &[(Grain, 4.0), (Livestock, 1.0)], &[(Warhorse, 1.0)], 240.0, Freeholders, 2, None, 140.0),
    prod("siege_workshop", "Belagerungswerkstatt", &[(Planks, 4.0), (Iron, 2.0), (Tools, 1.0)], &[(SiegeParts, 1.0)], 120.0, Artisans, 3, None, 150.0),
    prod("reliquary", "Reliquienschrein", &[(Candles, 1.0)], &[(Relics, 1.0)], 240.0, Clergy, 2, None, 150.0),
    // Dienstgebäude
    svc("temple", "Tempel", ServiceKind::Temple, 150.0, 3.0, Clergy, 3),
    svc("hospital", "Hospital", ServiceKind::Hospital, 160.0, 4.0, Clergy, 3),
    svc("library", "Bibliothek", ServiceKind::Library, 200.0, 4.0, Scholars, 3),
    svc("marketplace", "Marktplatz", ServiceKind::Marketplace, 120.0, 2.0, Burghers, 2),
    svc("harbor", "Hafen", ServiceKind::Harbor, 220.0, 4.0, Freeholders, 4),
    svc("barracks", "Kaserne", ServiceKind::Barracks, 140.0, 3.0, Warriors, 4),
    svc("watchtower", "Wachturm", ServiceKind::Watchtower, 60.0, 1.0, Warriors, 2),
    svc("housing", "Wohnviertel", ServiceKind::Housing, 80.0, 1.0, Serfs, 0),
    svc("mage_tower", "Magierturm", ServiceKind::MageTower, 260.0, 6.0, Scholars, 3),
    svc("granary", "Kornspeicher", ServiceKind::Granary, 90.0, 1.0, Serfs, 1),
    svc("wells", "Brunnen und Badehaus", ServiceKind::Wells, 70.0, 1.0, Burghers, 1),
];

pub fn building_index(id: &str) -> Option<usize> {
    BUILDINGS.iter().position(|b| b.id == id)
}

pub fn service_index(k: ServiceKind) -> usize {
    BUILDINGS.iter().position(|b| matches!(b.kind, BKind::Service(s) if s == k)).unwrap_or(0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeedKind {
    Food,
    Item(Good),
    Faith,
    Security,
    Market,
}

#[derive(Clone, Copy, Debug)]
pub struct Need {
    pub kind: NeedKind,
    /// Verbrauch pro 100 Bewohner und Monat (bei Gütern) bzw. Gewicht.
    pub per100: f32,
    /// 0 = Grund-, 1 = Komfort-, 2 = Luxusbedürfnis.
    pub tier: u8,
}

const fn n(kind: NeedKind, per100: f32, tier: u8) -> Need {
    Need { kind, per100, tier }
}

const SERF_NEEDS: &[Need] = &[
    n(NeedKind::Food, 60.0, 0),
    n(NeedKind::Item(Wood), 10.0, 0),
    n(NeedKind::Security, 1.0, 0),
    n(NeedKind::Item(Beer), 5.0, 1),
    n(NeedKind::Faith, 1.0, 1),
];
const FREE_NEEDS: &[Need] = &[
    n(NeedKind::Food, 60.0, 0),
    n(NeedKind::Item(Wood), 10.0, 0),
    n(NeedKind::Item(Clothing), 2.0, 0),
    n(NeedKind::Faith, 1.0, 0),
    n(NeedKind::Item(Beer), 8.0, 1),
    n(NeedKind::Item(Tools), 1.0, 1),
    n(NeedKind::Item(Meat), 10.0, 2),
    n(NeedKind::Market, 1.0, 2),
];
const ARTISAN_NEEDS: &[Need] = &[
    n(NeedKind::Food, 55.0, 0),
    n(NeedKind::Item(Clothing), 4.0, 0),
    n(NeedKind::Item(Tools), 2.0, 0),
    n(NeedKind::Faith, 1.0, 0),
    n(NeedKind::Item(Beer), 10.0, 1),
    n(NeedKind::Item(Meat), 12.0, 1),
    n(NeedKind::Item(Candles), 4.0, 1),
    n(NeedKind::Item(Furniture), 1.0, 2),
    n(NeedKind::Item(Wine), 3.0, 2),
];
const BURGHER_NEEDS: &[Need] = &[
    n(NeedKind::Food, 50.0, 0),
    n(NeedKind::Item(Clothing), 5.0, 0),
    n(NeedKind::Item(Candles), 5.0, 0),
    n(NeedKind::Faith, 1.0, 0),
    n(NeedKind::Market, 1.0, 0),
    n(NeedKind::Item(Wine), 6.0, 1),
    n(NeedKind::Item(Furniture), 2.0, 1),
    n(NeedKind::Item(Books), 1.0, 1),
    n(NeedKind::Item(Jewelry), 0.5, 2),
    n(NeedKind::Item(FineGarments), 1.0, 2),
    n(NeedKind::Item(Spices), 2.0, 2),
];
const CLERGY_NEEDS: &[Need] = &[
    n(NeedKind::Food, 45.0, 0),
    n(NeedKind::Item(Candles), 8.0, 0),
    n(NeedKind::Faith, 1.0, 0),
    n(NeedKind::Item(Books), 3.0, 1),
    n(NeedKind::Item(Wine), 4.0, 1),
    n(NeedKind::Item(Relics), 0.1, 2),
];
const SCHOLAR_NEEDS: &[Need] = &[
    n(NeedKind::Food, 45.0, 0),
    n(NeedKind::Item(Candles), 8.0, 0),
    n(NeedKind::Item(Books), 4.0, 0),
    n(NeedKind::Item(Wine), 4.0, 1),
    n(NeedKind::Item(Remedies), 2.0, 1),
    n(NeedKind::Item(Ashglass), 0.5, 1),
    n(NeedKind::Item(Jewelry), 0.5, 2),
];
const WARRIOR_NEEDS: &[Need] = &[
    n(NeedKind::Food, 70.0, 0),
    n(NeedKind::Item(Beer), 10.0, 0),
    n(NeedKind::Item(Weapons), 2.0, 0),
    n(NeedKind::Item(Meat), 15.0, 1),
    n(NeedKind::Item(Clothing), 4.0, 1),
];
const NOBLE_NEEDS: &[Need] = &[
    n(NeedKind::Food, 40.0, 0),
    n(NeedKind::Item(FineGarments), 3.0, 0),
    n(NeedKind::Item(Wine), 8.0, 0),
    n(NeedKind::Item(Jewelry), 1.0, 1),
    n(NeedKind::Item(Spices), 4.0, 1),
    n(NeedKind::Item(Furniture), 3.0, 1),
    n(NeedKind::Item(Silk), 2.0, 2),
    n(NeedKind::Item(Relics), 0.2, 2),
];

pub fn needs_of(s: Stratum) -> &'static [Need] {
    match s {
        Stratum::Serfs => SERF_NEEDS,
        Stratum::Freeholders => FREE_NEEDS,
        Stratum::Artisans => ARTISAN_NEEDS,
        Stratum::Burghers => BURGHER_NEEDS,
        Stratum::Clergy => CLERGY_NEEDS,
        Stratum::Scholars => SCHOLAR_NEEDS,
        Stratum::Warriors => WARRIOR_NEEDS,
        Stratum::Nobility => NOBLE_NEEDS,
    }
}
