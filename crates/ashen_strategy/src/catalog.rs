//! Statische Kataloge: Einheiten, Eigenschaften, Namen, Rituale, Artefakte.

use crate::types::{Attr, Culture, Religion, School};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------------------------
// Einheiten
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UClass {
    Worker,
    Infantry,
    Ranged,
    Cavalry,
    Siege,
    Support,
    Mage,
    Hero,
    Monster,
    Boss,
}

/// Strategische Einheitendefinition. Die IDs entsprechen `assets/data/units.ron` bzw. GDD 10.3.
#[derive(Clone, Copy, Debug)]
pub struct UnitDef {
    pub id: &'static str,
    pub name: &'static str,
    pub faction: &'static str,
    pub class: UClass,
    pub hp: f32,
    pub armor: f32,
    pub dmg: f32,
    pub interval: f32,
    pub range: f32,
    /// Rekrutierungskosten in Gold (0 = nicht anwerbbar).
    pub cost: f32,
    /// Unterhalt Gold pro Monat.
    pub upkeep: f32,
    /// Bevölkerungspunkte (Frontbreite, Aushebung).
    pub pop: u8,
    /// Zusatzschaden gegen Kavallerie.
    pub vs_cav: f32,
    /// Zusatzschaden gegen Fernkämpfer.
    pub vs_ranged: f32,
    /// Zusatzschaden gegen Untote/Hohle.
    pub vs_undead: f32,
    pub undead: bool,
}

#[allow(clippy::too_many_arguments)]
const fn u(
    id: &'static str,
    name: &'static str,
    faction: &'static str,
    class: UClass,
    hp: f32,
    armor: f32,
    dmg: f32,
    interval: f32,
    range: f32,
    cost: f32,
    upkeep: f32,
    pop: u8,
) -> UnitDef {
    UnitDef { id, name, faction, class, hp, armor, dmg, interval, range, cost, upkeep, pop, vs_cav: 1.0, vs_ranged: 1.0, vs_undead: 1.0, undead: false }
}

const fn cav(mut d: UnitDef, f: f32) -> UnitDef {
    d.vs_cav = f;
    d
}
const fn rng(mut d: UnitDef, f: f32) -> UnitDef {
    d.vs_ranged = f;
    d
}
const fn und(mut d: UnitDef) -> UnitDef {
    d.undead = true;
    d
}
const fn vsund(mut d: UnitDef, f: f32) -> UnitDef {
    d.vs_undead = f;
    d
}

use UClass::*;

pub const UNITS: &[UnitDef] = &[
    // Glutkrone
    u("worker", "Leibeigener", "ember_crown", Worker, 60.0, 0.0, 4.0, 1.5, 0.8, 20.0, 0.2, 1),
    u("swordsman", "Schwertkämpfer", "ember_crown", Infantry, 180.0, 3.0, 14.0, 1.2, 0.8, 60.0, 1.0, 1),
    cav(u("spearman", "Speerträger", "ember_crown", Infantry, 160.0, 2.0, 11.0, 1.3, 1.5, 45.0, 0.8, 1), 2.0),
    u("archer", "Bogenschütze", "ember_crown", Ranged, 100.0, 1.0, 9.0, 1.6, 7.0, 55.0, 0.8, 1),
    u("crossbowman", "Armbrustschütze", "ember_crown", Ranged, 120.0, 2.0, 18.0, 2.6, 7.5, 80.0, 1.2, 1),
    cav(u("halberdier", "Hellebardier", "ember_crown", Infantry, 220.0, 3.0, 16.0, 1.6, 1.5, 90.0, 1.4, 1), 1.75),
    u("scout_rider", "Grenzreiter", "ember_crown", Cavalry, 220.0, 1.0, 10.0, 1.2, 0.8, 100.0, 1.5, 2),
    rng(u("knight", "Ritter", "ember_crown", Cavalry, 380.0, 5.0, 22.0, 1.4, 0.8, 200.0, 3.0, 3), 1.5),
    vsund(u("ember_adept", "Glutklinge", "ember_crown", Infantry, 260.0, 3.0, 18.0, 1.3, 0.8, 150.0, 2.0, 2), 1.3),
    u("flame_priest", "Flammenpriester", "ember_crown", Support, 140.0, 0.0, 8.0, 2.0, 6.0, 110.0, 1.5, 1),
    u("ember_mage", "Glutmagier", "ember_crown", Mage, 120.0, 0.0, 24.0, 2.4, 8.0, 190.0, 2.5, 2),
    u("battering_ram", "Rammbock", "ember_crown", Siege, 600.0, 6.0, 80.0, 3.0, 0.8, 160.0, 2.0, 4),
    u("mangonel", "Mangonel", "ember_crown", Siege, 300.0, 2.0, 55.0, 4.5, 12.0, 220.0, 3.0, 4),
    u("trebuchet", "Tribok", "ember_crown", Siege, 400.0, 3.0, 140.0, 7.0, 18.0, 350.0, 4.0, 4),
    u("hero_isolde", "Isolde Raventhorn", "ember_crown", Hero, 620.0, 5.0, 28.0, 1.1, 0.8, 0.0, 5.0, 0),
    // Skarnheim
    u("skarn_ironguard", "Eisengarde", "skarn_clans", Infantry, 300.0, 6.0, 20.0, 1.6, 0.8, 150.0, 2.0, 2),
    u("skarn_rune_thrower", "Runenwerfer", "skarn_clans", Ranged, 130.0, 1.0, 14.0, 2.0, 5.0, 90.0, 1.2, 1),
    // Dornenbund
    u("veyl_stalker", "Veyl-Pirscher", "thorn_covenant", Ranged, 120.0, 1.0, 12.0, 1.4, 7.0, 80.0, 1.2, 1),
    u("thorn_warden", "Dornenhüter", "thorn_covenant", Infantry, 280.0, 3.0, 15.0, 1.4, 0.8, 120.0, 1.8, 2),
    // Saelith
    u("saelith_marine", "Seegardist", "saelith_compact", Infantry, 200.0, 3.0, 14.0, 1.2, 1.5, 85.0, 1.0, 1),
    u("saelith_arbalest", "Söldner-Arbalestier", "saelith_compact", Ranged, 140.0, 2.0, 26.0, 3.2, 8.5, 100.0, 2.0, 1),
    // Hohle
    und(u("hollow_thrall", "Hohlknecht", "hollowed", Monster, 120.0, 1.0, 8.0, 1.5, 0.8, 0.0, 0.0, 1)),
    und(u("hollow_archer", "Knochenschütze", "hollowed", Ranged, 80.0, 0.0, 8.0, 1.8, 6.5, 0.0, 0.0, 1)),
    und(u("grave_brute", "Grabkoloss", "hollowed", Monster, 650.0, 4.0, 32.0, 2.4, 1.0, 0.0, 0.0, 3)),
    und(u("veil_wraith", "Schleiergeist", "hollowed", Monster, 200.0, 0.0, 14.0, 1.6, 0.8, 0.0, 0.0, 2)),
    und(u("hollow_priest", "Leerpriester", "hollowed", Support, 180.0, 1.0, 12.0, 2.0, 6.0, 0.0, 0.0, 2)),
    // Aschenlegion
    und(u("ashen_legionnaire", "Aschenlegionär", "ashen_legion", Infantry, 340.0, 5.0, 18.0, 1.3, 0.8, 0.0, 0.0, 2)),
    und(u("cinder_knight", "Glutritter Aureths", "ashen_legion", Cavalry, 520.0, 6.0, 26.0, 1.4, 0.8, 0.0, 0.0, 4)),
    // Neutrale Kreaturen (nur strategische Ebene; Werte nach GDD 14.3)
    u("ash_wolf", "Aschwolf", "neutral", Monster, 90.0, 0.0, 9.0, 1.2, 0.5, 0.0, 0.0, 1),
    u("grave_worm", "Grabwurm", "neutral", Monster, 260.0, 2.0, 16.0, 2.0, 0.5, 0.0, 0.0, 2),
    u("glow_salamander", "Glutsalamander", "neutral", Monster, 300.0, 3.0, 20.0, 1.8, 1.0, 0.0, 0.0, 2),
    u("moor_light", "Moorlicht", "neutral", Monster, 70.0, 0.0, 10.0, 1.6, 4.0, 0.0, 0.0, 1),
    // Bosse
    und(u("boss_pale_reeve", "Der Fahle Vogt", "hollowed", Boss, 2200.0, 3.0, 40.0, 1.5, 1.0, 0.0, 0.0, 6)),
    u("boss_ashen_dragon", "Der Aschdrache Vyrmathax", "neutral", Boss, 14000.0, 8.0, 160.0, 2.0, 6.0, 0.0, 0.0, 30),
    u("boss_root_mother", "Die Wurzelmutter", "neutral", Boss, 9000.0, 4.0, 90.0, 2.0, 5.0, 0.0, 0.0, 24),
    und(u("boss_hollow_king", "Der Hohle König", "ashen_legion", Boss, 12000.0, 6.0, 130.0, 1.8, 3.0, 0.0, 0.0, 28)),
    u("boss_stone_oath", "Gharun, der Steinerne Eid", "neutral", Boss, 8000.0, 10.0, 100.0, 3.0, 1.0, 0.0, 0.0, 20),
    und(u("boss_reef_mistress", "Isarre Mol, Herrin des Schwarzen Riffs", "neutral", Boss, 6000.0, 2.0, 70.0, 1.6, 8.0, 0.0, 0.0, 18)),
];

pub fn unit(id: &str) -> Option<&'static UnitDef> {
    UNITS.iter().find(|u| u.id == id)
}

impl UnitDef {
    /// Kampfkraft nach GDD 4.6: (DPS_eff × HP_eff)^0.5.
    pub fn power(&self) -> f32 {
        let dps = self.dmg / self.interval;
        let hp = self.hp * (1.0 + self.armor * 0.06);
        let mut p = (dps * hp).sqrt();
        if self.class == UClass::Boss {
            p *= 1.0;
        }
        p
    }
    pub fn is_ranged(&self) -> bool {
        matches!(self.class, UClass::Ranged | UClass::Mage)
    }
}

/// Anwerbbare Einheiten je Kultur (für KI-Rekrutierung und Spielerliste).
pub fn roster(culture: Culture) -> &'static [&'static str] {
    match culture {
        Culture::Valemarker | Culture::Aschmaerker => &["spearman", "swordsman", "archer", "crossbowman", "halberdier", "knight", "scout_rider", "ember_adept", "mangonel", "battering_ram"],
        Culture::Skarn => &["skarn_ironguard", "skarn_rune_thrower", "spearman", "worker", "battering_ram"],
        Culture::Dornwaldler | Culture::Veyl => &["thorn_warden", "veyl_stalker", "archer", "worker"],
        Culture::Saelith => &["saelith_marine", "saelith_arbalest", "crossbowman", "swordsman", "scout_rider", "mangonel"],
        Culture::GraueNomaden | Culture::Aurethisch => &["scout_rider", "archer", "spearman", "swordsman"],
        Culture::Schlackenbrut => &["spearman", "glow_salamander", "worker"],
        Culture::Hohl => &["hollow_thrall", "hollow_archer", "grave_brute", "veil_wraith"],
    }
}

// ---------------------------------------------------------------------------------------------
// Eigenschaften (Traits)
// ---------------------------------------------------------------------------------------------

/// Auslöser für Stress beim Herrscher.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StressAct {
    Retreat,
    PersonalFight,
    Execute,
    Pardon,
    PeaceNoGain,
    DeclareWar,
    PactOtherFaith,
    Pilgrimage,
    GenerousGift,
    RaiseTaxes,
    DismissSpymaster,
    WarSoonAfterPeace,
    IgnoreInsult,
    Lie,
    Confess,
    FabricateClaim,
}

macro_rules! traits {
    ($( $v:ident, $id:expr, $name:expr, $acquired:expr, [$m:expr,$s:expr,$d:expr,$i:expr,$l:expr,$c:expr], $op:expr, $stress:expr; )*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Trait { $( $v ),* }
        impl Trait {
            pub const ALL: &'static [Trait] = &[ $( Trait::$v ),* ];
            pub fn id(self) -> &'static str { match self { $( Trait::$v => $id ),* } }
            pub fn name(self) -> &'static str { match self { $( Trait::$v => $name ),* } }
            pub fn acquired(self) -> bool { match self { $( Trait::$v => $acquired ),* } }
            /// Attributsmodifikatoren (Kriegskunst, Verwaltung, Diplomatie, Intrige, Gelehrsamkeit, Charisma).
            pub fn attrs(self) -> [i8; 6] { match self { $( Trait::$v => [$m,$s,$d,$i,$l,$c] ),* } }
            /// Allgemeine Meinungsänderung anderer Figuren.
            pub fn opinion(self) -> i8 { match self { $( Trait::$v => $op ),* } }
            pub fn stress_act(self) -> Option<(StressAct, u8)> { match self { $( Trait::$v => $stress ),* } }
            pub fn from_id(s: &str) -> Option<Trait> { match s { $( $id => Some(Trait::$v), )* _ => None } }
        }
    };
}

traits! {
    Brave, "brave", "Tapfer", false, [2,0,0,0,0,0], 0, Some((StressAct::Retreat, 10));
    Craven, "craven", "Feige", false, [-2,0,0,2,0,0], 0, Some((StressAct::PersonalFight, 10));
    Just, "just", "Gerecht", false, [0,2,0,0,0,0], 3, Some((StressAct::FabricateClaim, 15));
    Cruel, "cruel", "Grausam", false, [0,0,0,2,0,0], -10, Some((StressAct::Pardon, 10));
    Merciful, "merciful", "Gnädig", false, [0,0,0,0,0,0], 5, Some((StressAct::Execute, 20));
    Ambitious, "ambitious", "Ehrgeizig", false, [1,1,1,1,1,1], 0, Some((StressAct::PeaceNoGain, 5));
    Content, "content", "Zufrieden", false, [0,0,0,-1,0,0], 3, Some((StressAct::DeclareWar, 10));
    Zealous, "zealous", "Fromm", false, [0,0,0,0,0,0], 0, Some((StressAct::PactOtherFaith, 10));
    Cynical, "cynical", "Zweifler", false, [0,0,0,2,2,0], 0, Some((StressAct::Pilgrimage, 5));
    Greedy, "greedy", "Gierig", false, [0,0,0,0,0,0], -3, Some((StressAct::GenerousGift, 10));
    Generous, "generous", "Großzügig", false, [0,0,2,0,0,0], 4, Some((StressAct::RaiseTaxes, 10));
    Paranoid, "paranoid", "Misstrauisch", false, [0,0,-2,3,0,0], -2, Some((StressAct::DismissSpymaster, 5));
    Patient, "patient", "Geduldig", false, [0,2,0,0,2,0], 2, Some((StressAct::WarSoonAfterPeace, 10));
    Wrathful, "wrathful", "Jähzornig", false, [3,0,-2,0,0,0], -3, Some((StressAct::IgnoreInsult, 10));
    Honest, "honest", "Ehrlich", false, [0,0,2,-4,0,0], 4, Some((StressAct::Lie, 15));
    Deceitful, "deceitful", "Verschlagen", false, [0,0,-2,4,0,0], -4, Some((StressAct::Confess, 10));
    AshFeverScar, "ash_fever_scar", "Aschenfieber-Narbe", true, [0,0,0,0,0,-1], 0, None;
    Genius, "genius", "Genie", true, [5,5,5,5,5,5], 5, None;
    Quick, "quick", "Kluger Kopf", true, [2,2,2,2,2,2], 2, None;
    Slow, "slow", "Einfältig", true, [-3,-3,-3,-3,-3,-3], -2, None;
    Giant, "giant", "Riese", true, [3,0,0,0,0,0], 0, None;
    Sickly, "sickly", "Kränklich", true, [0,0,0,0,0,0], 0, None;
    OneEyed, "one_eyed", "Einäugig", true, [1,0,-1,0,0,0], 0, None;
    Maimed, "maimed", "Verkrüppelt", true, [-3,0,0,0,0,0], 0, None;
    WarHero, "war_hero", "Kriegsheld", true, [0,0,0,0,0,2], 10, None;
    Kinslayer, "kinslayer", "Sippenmörder", true, [0,0,0,0,0,0], -15, None;
    HollowTouched, "hollow_touched", "Hohlberührt", true, [0,0,0,0,2,0], -5, None;
    FlameSworn, "flame_sworn", "Flammengeweiht", true, [0,0,0,0,0,0], 5, None;
    Scholar, "scholar", "Gelehrter", true, [0,0,0,0,3,0], 2, None;
    Strategist, "strategist", "Stratege", true, [3,0,0,0,0,0], 2, None;
    Courtier, "courtier", "Höfling", true, [0,0,3,0,0,0], 3, None;
    Drunkard, "drunkard", "Trinker", true, [0,-1,0,0,0,0], -3, None;
    Melancholic, "melancholic", "Melancholisch", true, [-2,-2,-2,-2,-2,-2], -2, None;
    Lunatic, "lunatic", "Verrückt", true, [0,0,0,0,0,0], -10, None;
    Steward, "steward", "Verwalter", true, [0,3,0,0,0,0], 2, None;
    Schemer, "schemer", "Ränkeschmied", true, [0,0,0,3,0,0], -2, None;
    PiousMind, "pious_mind", "Frommer Geist", true, [0,0,0,0,2,1], 2, None;
}

impl Trait {
    pub fn is_personality(self) -> bool {
        !self.acquired()
    }
    /// Gegensätzliche Persönlichkeitseigenschaft (für die Grundsympathie).
    pub fn opposite(self) -> Option<Trait> {
        use Trait::*;
        Some(match self {
            Brave => Craven,
            Craven => Brave,
            Just => Cruel,
            Cruel => Just,
            Merciful => Cruel,
            Content => Ambitious,
            Ambitious => Content,
            Zealous => Cynical,
            Cynical => Zealous,
            Greedy => Generous,
            Generous => Greedy,
            Honest => Deceitful,
            Deceitful => Honest,
            Patient => Wrathful,
            Wrathful => Patient,
            _ => return None,
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Namen
// ---------------------------------------------------------------------------------------------

pub struct NameSet {
    pub male: &'static [&'static str],
    pub female: &'static [&'static str],
    pub family: &'static [&'static str],
}

const VALEMARK: NameSet = NameSet {
    male: &["Aldric", "Edmund", "Gerold", "Lorens", "Benedikt", "Aurelien", "Roland", "Tomas", "Wilhelm", "Cedric", "Alaric", "Hartmut", "Osric", "Baldwin", "Fenn", "Garrick", "Leofric", "Matthis", "Rainer", "Viktor"],
    female: &["Mireille", "Elyane", "Isolde", "Aveline", "Maelwen", "Rosalind", "Adelheid", "Sibylle", "Clarisse", "Ysolde", "Brigitte", "Lenore", "Odette", "Ilse", "Margarethe", "Thessaly", "Ottilie", "Liesel", "Corinna", "Hedda"],
    family: &["Caldermont", "Corvane", "Melbrecht", "Strathgar", "Ilverholt", "Varenne", "Raventhorn", "Falkenrath", "Harkwald", "Brandt", "Thornfield", "Kestrel", "Marsh", "Wendel", "Aldermere", "Ashby"],
};
const ASCHMARK: NameSet = NameSet {
    male: &["Edran", "Hal", "Brenn", "Tam", "Jorun", "Kaspar", "Wulf", "Dag", "Arnulf", "Berit", "Osk", "Rurik"],
    female: &["Maelwen", "Hanne", "Yrsa", "Greta", "Wenna", "Tove", "Ragna", "Elka", "Birk", "Sunna", "Inga", "Fenja"],
    family: &["Raventhorn", "Brennholt", "Aschbach", "Kaltwind", "Meringer", "Grauwall", "Birkenfeld", "Steinacker"],
};
const SKARN: NameSet = NameSet {
    male: &["Torvald", "Haldor", "Bjarn", "Ragnvar", "Sigurd", "Orm", "Eirik", "Thorgrim", "Gunnar", "Ulfar", "Skarr", "Harek"],
    female: &["Sigrun", "Astrid", "Helga", "Thyra", "Brynja", "Gudrun", "Ingrid", "Solveig", "Ragnhild", "Hildr", "Svanhild", "Freydis"],
    family: &["Eisenhand", "Grauhelm", "Wolfspass", "Steinbrecher", "Frostmantel", "Kharnson", "Runenfaust", "Schildbrecher"],
};
const THORN: NameSet = NameSet {
    male: &["Alder", "Rowan", "Linden", "Fenwick", "Hazel", "Ashwin", "Oakley", "Bramwell", "Elm", "Thistle"],
    female: &["Sylaen", "Willow", "Hazel", "Rowena", "Briar", "Heather", "Myrtle", "Ivy", "Fern", "Sorrel"],
    family: &["Wurzelsohn", "Mooshain", "Dornenkranz", "Eichenfels", "Harzblut", "Birkenwurz"],
};
const VEYL: NameSet = NameSet {
    male: &["Ilvaen", "Taelor", "Nyrr", "Sethil", "Quor", "Vael", "Orlen", "Isk"],
    female: &["Sylaen", "Ilyra", "Naeve", "Thessil", "Aelwyn", "Mirrel", "Oryn", "Veyra"],
    family: &["Mottenflügel", "Silberrinde", "Leuchtauge", "Nachtfalter", "Rindenflüsterer"],
};
const SAELITH: NameSet = NameSet {
    male: &["Corvin", "Matteo", "Lucan", "Dario", "Emeric", "Fabrizio", "Orsino", "Teodor", "Valerio", "Rinaldo"],
    female: &["Ophira", "Isabeau", "Lucrezia", "Serafina", "Chiara", "Mirela", "Viola", "Carmina", "Delphine", "Alessia"],
    family: &["Saelith-Varo", "Saelith-Ombre", "Isquel", "Merenhal", "Thalvern", "Corallo", "Duvessa", "Lirr", "Astrane"],
};
const NOMAD: NameSet = NameSet {
    male: &["Kesh", "Tamar", "Jaric", "Omar", "Zahir", "Rashid", "Idris", "Haldun"],
    female: &["Merrat", "Nasira", "Zuri", "Amara", "Yasmin", "Dalia", "Samira", "Leyla"],
    family: &["Aschwandler", "Glassucher", "Dünenreiter", "Merratsohn", "Sandmantel"],
};
const SLAG: NameSet = NameSet {
    male: &["Ossgar", "Brakk", "Kharn", "Dumm", "Grosh", "Vulk"],
    female: &["Embra", "Kessa", "Slagga", "Orrun", "Pyra", "Cinda"],
    family: &["Schlackenbrut", "Glutzahn", "Aschenhaut", "Basaltfaust"],
};
const AURETH: NameSet = NameSet {
    male: &["Varkhan", "Solvar", "Aurel", "Helion", "Caldus", "Marius", "Lucian", "Octavar"],
    female: &["Solmera", "Aurelia", "Helia", "Cassia", "Livia", "Octavia", "Seraphine", "Valeria"],
    family: &["Aurelith", "Solvarin", "Helionid", "Kesh-Auran"],
};

pub fn names(c: Culture) -> &'static NameSet {
    match c {
        Culture::Valemarker => &VALEMARK,
        Culture::Aschmaerker => &ASCHMARK,
        Culture::Skarn => &SKARN,
        Culture::Dornwaldler => &THORN,
        Culture::Veyl => &VEYL,
        Culture::Saelith => &SAELITH,
        Culture::GraueNomaden => &NOMAD,
        Culture::Schlackenbrut => &SLAG,
        Culture::Hohl | Culture::Aurethisch => &AURETH,
    }
}

// ---------------------------------------------------------------------------------------------
// Rituale und Artefakte
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RitualEffect {
    /// Hohlung einer Provinz senken.
    Purify(f32),
    /// Ernte der Provinz für ein Jahr verbessern.
    HarvestBlessing,
    /// Befestigung der Provinz erhöhen.
    Fortify,
    /// Schleierstärke in der Provinz senken.
    SealVeil(f32),
    /// Forschungspunkte gutschreiben.
    ResearchBurst(f32),
    /// Spionagenetz beschleunigen (Netzwerkstärke).
    ShadowNetwork(f32),
    /// Unruhe in der Provinz senken (Beruhigung, Gezeitenfrieden).
    Calm(f32),
    /// Totenruhe: Hohle erheben sich ein Jahr lang nicht.
    DeadRest,
    /// Schleier reißen (Leermagie): Schleier +, Hohlung +, mächtig.
    RendVeil,
    /// Erzritual: alle Provinzen Hohlung senken.
    GrandPurge(f32),
    /// Erzritual „Siegel“ (Siegbedingung).
    SealOfTheVeil,
    /// Hohlruf: Hohle Einheiten für den Wirker (Seelenasche).
    CallHollow,
}

#[derive(Clone, Copy, Debug)]
pub struct RitualDef {
    pub id: &'static str,
    pub name: &'static str,
    pub school: School,
    pub grade: u8,
    pub ashglass: f32,
    pub gold: f32,
    pub effect: RitualEffect,
    /// Risiko einer Hohlungs-/Schleierfolge (0–1).
    pub risk: f32,
}

pub const RITUALS: &[RitualDef] = &[
    RitualDef { id: "ritual_cleansing_fire", name: "Läuterndes Feuer", school: School::Ember, grade: 2, ashglass: 4.0, gold: 150.0, effect: RitualEffect::Purify(12.0), risk: 0.0 },
    RitualDef { id: "ritual_eternal_watch", name: "Ewige Wacht", school: School::Ember, grade: 4, ashglass: 60.0, gold: 2000.0, effect: RitualEffect::GrandPurge(10.0), risk: 0.05 },
    RitualDef { id: "ritual_binding_circle", name: "Bannkreis", school: School::Order, grade: 2, ashglass: 5.0, gold: 200.0, effect: RitualEffect::SealVeil(0.3), risk: 0.0 },
    RitualDef { id: "ritual_seal", name: "Siegel des Schleiers", school: School::Order, grade: 4, ashglass: 100.0, gold: 5000.0, effect: RitualEffect::SealOfTheVeil, risk: 0.1 },
    RitualDef { id: "ritual_rest_of_dead", name: "Totenruhe", school: School::Passage, grade: 2, ashglass: 3.0, gold: 100.0, effect: RitualEffect::DeadRest, risk: 0.0 },
    RitualDef { id: "ritual_harvest_blessing", name: "Ernteritual", school: School::Growth, grade: 2, ashglass: 3.0, gold: 120.0, effect: RitualEffect::HarvestBlessing, risk: 0.0 },
    RitualDef { id: "ritual_storm_peace", name: "Gezeitenfrieden", school: School::Tide, grade: 2, ashglass: 3.0, gold: 150.0, effect: RitualEffect::Calm(15.0), risk: 0.0 },
    RitualDef { id: "ritual_rune_wall", name: "Runenwall", school: School::Stone, grade: 2, ashglass: 4.0, gold: 180.0, effect: RitualEffect::Fortify, risk: 0.0 },
    RitualDef { id: "ritual_resonance", name: "Resonanzfeld", school: School::Crystal, grade: 2, ashglass: 6.0, gold: 250.0, effect: RitualEffect::ResearchBurst(150.0), risk: 0.08 },
    RitualDef { id: "ritual_whispers", name: "Flüsternetz", school: School::Shadow, grade: 2, ashglass: 4.0, gold: 200.0, effect: RitualEffect::ShadowNetwork(15.0), risk: 0.05 },
    RitualDef { id: "ritual_hollow_call", name: "Ruf aus dem Tief", school: School::Void, grade: 3, ashglass: 20.0, gold: 500.0, effect: RitualEffect::CallHollow, risk: 0.3 },
    RitualDef { id: "ritual_rend_veil", name: "Schleierriss", school: School::Void, grade: 4, ashglass: 80.0, gold: 1500.0, effect: RitualEffect::RendVeil, risk: 0.5 },
];

pub fn ritual(id: &str) -> Option<&'static RitualDef> {
    RITUALS.iter().find(|r| r.id == id)
}

/// Forschungsaufwand und Aschglas je Grad (I–IV).
pub const GRADE_POINTS: [f32; 4] = [200.0, 600.0, 1500.0, 4000.0];
pub const GRADE_ASHGLASS: [f32; 4] = [5.0, 15.0, 40.0, 100.0];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactEffect {
    Attr(Attr, i8),
    HollowResist,
    MilitaryPower,
    ResearchBoost,
    Income,
    Fertility,
}

#[derive(Clone, Copy, Debug)]
pub struct ArtifactDef {
    pub id: &'static str,
    pub name: &'static str,
    pub effect: ArtifactEffect,
    pub amount: f32,
    /// Fundort (Provinzschlüssel).
    pub site: &'static str,
}

pub const ARTIFACTS: &[ArtifactDef] = &[
    ArtifactDef { id: "art_morgenrot_splitter", name: "Morgenrotsplitter", effect: ArtifactEffect::HollowResist, amount: 0.5, site: "arnvale" },
    ArtifactDef { id: "art_sun_crown", name: "Sonnenreif Varkhans", effect: ArtifactEffect::Attr(Attr::Charisma, 3), amount: 3.0, site: "nekropole_der_sonnenkoenige" },
    ArtifactDef { id: "art_glass_eye", name: "Glasauge von Ilum", effect: ArtifactEffect::ResearchBoost, amount: 0.25, site: "glasfelder_von_ilum" },
    ArtifactDef { id: "art_iron_oath", name: "Eisenschwur der Clans", effect: ArtifactEffect::MilitaryPower, amount: 0.08, site: "eisenkehle" },
    ArtifactDef { id: "art_amber_heart", name: "Bernsteinherz", effect: ArtifactEffect::Fertility, amount: 0.15, site: "bernsteinsuempfe" },
    ArtifactDef { id: "art_salt_ledger", name: "Salzbuch des Hohen Kontors", effect: ArtifactEffect::Income, amount: 0.06, site: "selvaris" },
    ArtifactDef { id: "art_dragon_scale", name: "Glutschuppe", effect: ArtifactEffect::MilitaryPower, amount: 0.05, site: "der_kessel" },
    ArtifactDef { id: "art_still_mirror", name: "Spiegel von Ner Moraun", effect: ArtifactEffect::Attr(Attr::Learning, 3), amount: 3.0, site: "ner_moraun" },
];

/// Namen der Seuchen.
pub const PLAGUES: [&str; 4] = ["Aschenfieber", "Grauhusten", "Schwarze Schleimpest", "Magische Seuche"];

/// Religion einer Kultur als Standard.
pub fn default_religion(c: Culture) -> Religion {
    match c {
        Culture::Valemarker | Culture::Aschmaerker => Religion::EternalFlame,
        Culture::Skarn => Religion::AncestorPath,
        Culture::Dornwaldler | Culture::Veyl => Religion::GreenCircle,
        Culture::Saelith => Religion::TideHouse,
        Culture::GraueNomaden | Culture::Aurethisch => Religion::SunHeresy,
        Culture::Schlackenbrut => Religion::HollowLiturgy,
        Culture::Hohl => Religion::HollowLiturgy,
    }
}
