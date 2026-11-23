//! RPG- und Militärdefinitionen: Gegenstände, Talente, Schmiede-Upgrades und Formationen.
//!
//! Die Inhalte stammen aus `items.ron`, `talents.ron` und `upgrades.ron`; Mods können Einträge
//! gleicher `id` ersetzen oder ergänzen.

use crate::data::{Cost, ItemAmounts, ItemList, UnitClass};
use serde::{Deserialize, Serialize};

/// Index eines Gegenstands in [`crate::GameData::items`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ItemId(pub u16);

/// Additive Wertänderungen (Ausrüstung, Talente, Stufenaufstieg).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatMods {
    pub hp: f32,
    pub mana: f32,
    /// Fester Zusatzschaden je Treffer.
    pub damage: f32,
    /// Prozentualer Schadensbonus (0.1 = +10 %).
    pub damage_pct: f32,
    pub armor: f32,
    /// Prozentuale Geschwindigkeit (0.1 = +10 %).
    pub speed_pct: f32,
    pub hp_regen: f32,
    pub mana_regen: f32,
    /// Prozentuale Erfahrung (0.1 = +10 %).
    pub xp_pct: f32,
    /// Verkürzung der Fähigkeiten-Abklingzeit (0.1 = −10 %).
    pub cooldown_pct: f32,
}

impl StatMods {
    /// Addiert `o` ein- oder mehrfach (`times` Ränge).
    pub fn add(&mut self, o: &StatMods, times: f32) {
        self.hp += o.hp * times;
        self.mana += o.mana * times;
        self.damage += o.damage * times;
        self.damage_pct += o.damage_pct * times;
        self.armor += o.armor * times;
        self.speed_pct += o.speed_pct * times;
        self.hp_regen += o.hp_regen * times;
        self.mana_regen += o.mana_regen * times;
        self.xp_pct += o.xp_pct * times;
        self.cooldown_pct += o.cooldown_pct * times;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemSlot {
    Weapon,
    Armor,
    Amulet,
    Ring,
    /// Verbrauchsgegenstand (Trank); wird nicht angelegt.
    Potion,
}

impl ItemSlot {
    pub fn label(self) -> &'static str {
        match self {
            ItemSlot::Weapon => "Waffe",
            ItemSlot::Armor => "Rüstung",
            ItemSlot::Amulet => "Amulett",
            ItemSlot::Ring => "Ring",
            ItemSlot::Potion => "Trank",
        }
    }
}

/// Plätze der Heldenausrüstung: Waffe, Rüstung, Amulett, zwei Ringe.
pub const EQUIP_SLOTS: usize = 5;
/// Größe des Heldeninventars.
pub const INVENTORY_SIZE: usize = 12;

/// Index des Ausrüstungsplatzes für einen Gegenstandstyp (bei Ringen der erste freie von beiden).
pub fn equip_index(slot: ItemSlot, equipment: &[Option<ItemId>; EQUIP_SLOTS]) -> Option<usize> {
    match slot {
        ItemSlot::Weapon => Some(0),
        ItemSlot::Armor => Some(1),
        ItemSlot::Amulet => Some(2),
        ItemSlot::Ring => {
            if equipment[3].is_none() {
                Some(3)
            } else if equipment[4].is_none() {
                Some(4)
            } else {
                Some(3)
            }
        }
        ItemSlot::Potion => None,
    }
}

pub const EQUIP_LABELS: [&str; EQUIP_SLOTS] = ["Waffe", "Rüstung", "Amulett", "Ring", "Ring"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    /// Einzigartiges Artefakt aus dem Kanon.
    Unique,
}

impl Rarity {
    pub fn label(self) -> &'static str {
        match self {
            Rarity::Common => "Gewöhnlich",
            Rarity::Uncommon => "Ungewöhnlich",
            Rarity::Rare => "Selten",
            Rarity::Epic => "Episch",
            Rarity::Unique => "Artefakt",
        }
    }

    /// Anzeigefarbe (RGB).
    pub fn color(self) -> [u8; 3] {
        match self {
            Rarity::Common => [200, 196, 186],
            Rarity::Uncommon => [110, 200, 120],
            Rarity::Rare => [96, 150, 240],
            Rarity::Epic => [190, 110, 235],
            Rarity::Unique => [245, 170, 60],
        }
    }
}

/// Wirkung eines Verbrauchsgegenstands (Trank).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemUse {
    pub heal: f32,
    pub mana: f32,
    /// Zeitlich begrenzter Kampfbonus.
    pub damage_mult: f32,
    pub armor_bonus: f32,
    pub duration: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slot: ItemSlot,
    pub rarity: Rarity,
    pub mods: StatMods,
    pub use_effect: Option<ItemUse>,
    /// Verkaufswert in Gold.
    pub value: f32,
}

impl Default for ItemDef {
    fn default() -> Self {
        ItemDef { id: String::new(), name: String::new(), description: String::new(), slot: ItemSlot::Ring, rarity: Rarity::Common, mods: StatMods::default(), use_effect: None, value: 10.0 }
    }
}

/// Ein Talent im Talentbaum eines Helden.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TalentDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Pfad 0..3.
    pub path: u8,
    /// Reihe im Pfad (0 = oben). Reihe `t` verlangt `2·t` ausgegebene Punkte im selben Pfad.
    pub tier: u8,
    pub max_rank: u8,
    /// Wertänderung je Rang.
    pub mods: StatMods,
    /// Fähigkeit, die mit dem ersten Rang freigeschaltet wird.
    pub ability: Option<String>,
}

impl Default for TalentDef {
    fn default() -> Self {
        TalentDef { id: String::new(), name: String::new(), description: String::new(), path: 0, tier: 0, max_rank: 1, mods: StatMods::default(), ability: None }
    }
}

/// Talentbaum eines Helden: drei Pfade.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TalentTree {
    pub hero: String,
    pub paths: Vec<String>,
    pub talents: Vec<TalentDef>,
}

impl TalentTree {
    /// Punkte, die in `path` ausgegeben wurden.
    pub fn spent_in_path(&self, ranks: &[u8], path: u8) -> u32 {
        self.talents.iter().enumerate().filter(|(_, t)| t.path == path).map(|(i, _)| ranks.get(i).copied().unwrap_or(0) as u32).sum()
    }

    pub fn spent(&self, ranks: &[u8]) -> u32 {
        ranks.iter().map(|r| *r as u32).sum()
    }

    /// Verlangte Punkte im Pfad für ein Talent (2 je Reihe).
    pub fn required_points(t: &TalentDef) -> u32 {
        t.tier as u32 * 2
    }
}

/// Wirkung eines Upgrades je Stufe.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpgradeEffect {
    pub damage_pct: f32,
    pub armor: f32,
    pub hp_pct: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpgradeLevel {
    pub cost: Cost,
    pub goods: ItemList,
    #[serde(skip)]
    pub goods_r: ItemAmounts,
    /// Forschungsdauer in Sekunden.
    pub time: f32,
    /// Benötigte Ausbaustufe der Siedlung.
    pub era: u8,
}

/// Schmiede-Upgrade (Stufen 1–3) für eine Gruppe von Einheitenklassen.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpgradeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Gebäude, in dem geforscht wird.
    pub building: String,
    pub applies_to: Vec<UnitClass>,
    /// Wirkung je erreichter Stufe.
    pub per_level: UpgradeEffect,
    pub levels: Vec<UpgradeLevel>,
}

/// Aufstellung einer Gruppe. Wirkungen siehe [`Formation::mods`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formation {
    /// Lockerer Block ohne Boni (Grundverhalten).
    #[default]
    Block,
    Line,
    Wedge,
    Column,
    /// Karree / Schildwall.
    Square,
}

/// Wirkung einer Formation auf Kampfwerte.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormationMods {
    pub damage_mult: f32,
    pub ranged_damage_mult: f32,
    pub armor: f32,
    pub speed_mult: f32,
    /// Faktor auf erlittenen Pfeil- und Bolzenschaden.
    pub pierce_taken: f32,
    /// Faktor auf Moralverlust.
    pub morale_loss: f32,
}

impl Formation {
    pub const ALL: [Formation; 5] = [Formation::Block, Formation::Line, Formation::Wedge, Formation::Column, Formation::Square];

    pub fn label(self) -> &'static str {
        match self {
            Formation::Block => "Block",
            Formation::Line => "Linie",
            Formation::Wedge => "Keil",
            Formation::Column => "Kolonne",
            Formation::Square => "Karree",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Formation::Block => "Lockerer Block ohne Boni.",
            Formation::Line => "Breite Front: Fernkämpfer +10 % Schaden, aber −1 Rüstung (dünne Reihe).",
            Formation::Wedge => "Spitze voran: +15 % Schaden, −1 Rüstung. Stark beim Durchbruch, verwundbar auf Dauer.",
            Formation::Column => "Marschordnung: +15 % Geschwindigkeit, aber −15 % Schaden und −1 Rüstung.",
            Formation::Square => "Karree / Schildwall: +3 Rüstung, −25 % erlittener Pfeilschaden, −30 % Moralverlust; −20 % Tempo, −10 % Schaden.",
        }
    }

    pub fn mods(self) -> FormationMods {
        let one = FormationMods { damage_mult: 1.0, ranged_damage_mult: 1.0, armor: 0.0, speed_mult: 1.0, pierce_taken: 1.0, morale_loss: 1.0 };
        match self {
            Formation::Block => one,
            Formation::Line => FormationMods { ranged_damage_mult: 1.10, armor: -1.0, ..one },
            Formation::Wedge => FormationMods { damage_mult: 1.15, ranged_damage_mult: 1.0, armor: -1.0, ..one },
            Formation::Column => FormationMods { damage_mult: 0.85, ranged_damage_mult: 0.85, armor: -1.0, speed_mult: 1.15, ..one },
            Formation::Square => FormationMods { damage_mult: 0.9, ranged_damage_mult: 0.9, armor: 3.0, speed_mult: 0.8, pierce_taken: 0.75, morale_loss: 0.7 },
        }
    }
}
