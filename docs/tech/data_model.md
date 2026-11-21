# ASHEN CROWNS – Datenmodell

Alle Typen stammen aus `crates/ashen_sim/src` (Simulation) bzw. `crates/ashen_engine/src/audio` (Audio-Manifest) und
beschreiben den **tatsächlichen Stand** der Vertical Slice. RON-Beispiele zeigen die Schreibweise der Dateien unter
`assets/data/`. Überblick: [`architecture.md`](architecture.md); Modul-Zuordnung: [`modules.md`](modules.md);
Mod-Regeln: [`modding.md`](modding.md).

Konventionen:

* **Einheiten:** Längen und Reichweiten in **Kacheln**, Zeiten in **Sekunden**, Geschwindigkeiten in Kacheln pro
  Sekunde, Unterhalt in Nahrung pro Sekunde.
* **Koordinaten:** Weltkoordinaten der Simulation sind Kachel-Einheiten als `Vec2`/`IVec2`. Kachel-x läuft auf dem
  Bildschirm nach rechts unten, Kachel-y nach links unten (isometrische 2:1-Projektion, `ashen_core::iso`).
* **RON:** Optionale Felder werden explizit als `Some(...)`/`None` geschrieben (z. B. `hotkey: Some('S')`,
  `projectile: Some(Arrow)`). Tupelstrukturen wie `Cost` als `(gold: 50.0)`.
* **`#[serde(default)]`:** Bei Strukturen mit dieser Markierung dürfen Felder fehlen; sie erhalten den
  dokumentierten Standardwert (nicht den Wert des Basiseintrags bei Mod-Überschreibung).

---

## 1. Spieldefinitionen (`GameData`)

`GameData` (`data.rs`) hält alle Inhalte in Vektoren plus Index-Tabellen (ID-String → Index). Typisierte Indizes
`UnitTypeId(u16)`, `BuildingTypeId(u16)`, `AbilityId(u16)` sind Positionen in diesen Vektoren und damit von der
Ladereihenfolge abhängig (relevant für Spielstände, siehe Typtabelle in Abschnitt 8).

| Feld | Typ | Quelle |
|---|---|---|
| `units` | `Vec<UnitDef>` | `assets/data/units.ron` (Hülle `(units: [...])`) |
| `buildings` | `Vec<BuildingDef>` | `assets/data/buildings.ron` (Hülle `(buildings: [...])`) |
| `abilities` | `Vec<AbilityDef>` | `assets/data/abilities.ron` (Hülle `(abilities: [...])`) |
| `rules` | `Rules` | `assets/data/rules.ron` |

Funktionen: `GameData::load_dir(dir)` (lädt alle vier Dateien, `reindex`, `validate`),
`apply_overlay(dir)` (Mods; gibt Liste wie `unit:spearman` zurück), `validate()`, `unit_id`/`building_id`/`ability_id`,
`unit`/`building`/`ability`, `type_table()`.

`validate()` prüft: Einheiten (`hp > 0`, `speed >= 0`, `damage_max >= damage_min`, Fähigkeiten existieren),
Gebäude (`trains` verweist auf Einheiten, `requires` auf Gebäude, Größe > 0), Fähigkeiten (`Summon` verweist auf
existierende Einheit).

### 1.1 Aufzählungen

| Typ | Werte |
|---|---|
| `Faction` | `EmberCrown`, `Hollowed`, `Neutral` |
| `UnitClass` | `Worker`, `Infantry`, `Ranged`, `Cavalry`, `Hero`, `Monster`, `Boss` |
| `ArmorType` | `Unarmored`, `Light`, `Medium`, `Heavy`, `Fortified`, `Hero` |
| `DamageType` | `Normal`, `Pierce`, `Siege`, `Magic`, `Hero` |
| `ResourceKind` | `Gold`, `Wood`, `Stone`, `Food` |
| `ProjectileKind` | `Arrow`, `BoneBolt`, `SoulBolt` |
| `AbilityTargeting` | `SelfCentered`, `Point` (Hinweis: Alle Fähigkeiten der Vertical Slice sind `SelfCentered`; `Point` ist im Datenmodell vorhanden, wird von keiner Fähigkeit genutzt) |

### 1.2 `Cost`

`Cost { gold, wood, stone, food }`, alle `f32`, `#[serde(default)]` (nicht genannte Felder = 0).
Hilfsfunktionen: `get`, `scaled`, `covers`, `add`, `sub`, `first_missing`. Wird sowohl für Kosten als auch für
Ressourcenmengen (Spielerbestand, Belohnungen) verwendet.

### 1.3 `UnitDef` (`#[serde(default)]`)

| Feld | Typ | Standard | Bedeutung |
|---|---|---|---|
| `id` | `String` | leer | Eindeutige ID (z. B. `swordsman`); Schlüssel für Mod-Überschreibung |
| `name`, `description` | `String` | leer | Anzeigename und Beschreibung |
| `faction` | `Faction` | `Neutral` | Zugehörigkeit |
| `class` | `UnitClass` | `Infantry` | Klasse (Boni, Auswahl, Darstellung) |
| `hp` | `f32` | 100 | Lebenspunkte |
| `mana`, `mana_regen` | `f32` | 0 | Maximales Mana, Regeneration pro Sekunde |
| `hp_regen` | `f32` | 0 | LP pro Sekunde (außerhalb des Kampfes doppelt) |
| `armor` | `f32` | 0 | Rüstungswert |
| `armor_type` | `ArmorType` | `Medium` | Rüstungsart |
| `damage_min`, `damage_max` | `f32` | 5 / 7 | Schaden pro Treffer (`max >= min`) |
| `damage_type` | `DamageType` | `Normal` | Schadensart |
| `range` | `f32` | 0,6 | Angriffsreichweite in Kacheln (zusätzlich zu den Radien) |
| `cooldown` | `f32` | 1,5 | Angriffsintervall (s) |
| `windup` | `f32` | 0,3 | Ausholzeit bis zum Treffer (s) |
| `speed` | `f32` | 2,5 | Kacheln pro Sekunde |
| `sight` | `f32` | 7,0 | Sichtweite in Kacheln |
| `radius` | `f32` | 0,3 | Körperradius (Kollision, Reichweitenberechnung) |
| `cost` | `Cost` | 0 | Ausbildungskosten |
| `train_time` | `f32` | 15 | Ausbildungsdauer (s) |
| `pop` | `u32` | 1 | Bevölkerungsplätze |
| `upkeep` | `f32` | 0,05 | Nahrungsverbrauch pro Sekunde (nur Spieler) |
| `morale` | `f32` | 100 | Maximale Moral |
| `fearless` | `bool` | `false` | Immun gegen Moralverlust |
| `projectile` | `Option<ProjectileKind>` | `None` | Gesetzt = Fernkämpfer |
| `bonus_vs` | `Vec<(UnitClass, f32)>` | leer | Schadensmultiplikator gegen Klassen |
| `abilities` | `Vec<String>` | leer | Fähigkeits-IDs in Reihenfolge (Taste Q, W, E für den ausgewählten Helden) |
| `carry` | `u32` | 0 | Tragekapazität (> 0 = kann sammeln) |
| `gather_speed` | `f32` | 0 | Sammelgeschwindigkeit |
| `build_speed` | `f32` | 0 | Baugeschwindigkeit (> 0 = kann bauen) |
| `xp_value` | `u32` | 10 | Erfahrung beim Töten |
| `hp_per_level`, `damage_per_level` | `f32` | 0 | Zuwächse je Stufe (Helden) |
| `hotkey` | `Option<char>` | `None` | Taste zum Ausbilden (nur Buchstaben A B C D E F G K M P Q S T V W X Z werden vom Spiel verarbeitet, siehe `mission::letter_key`) |

Einheiten der Vertical Slice (`assets/data/units.ron`):

| ID | Name | Fraktion | Klasse | LP | Rüstung/Art | Schaden/Art | Reichweite | Tempo | Kosten | Pop |
|---|---|---|---|---|---|---|---|---|---|---|
| `worker` | Leibeigener | EmberCrown | Worker | 160 | 0 / Medium | 4–6 Normal | 0,5 | 2,4 | 50 Gold | 1 |
| `swordsman` | Schwertkämpfer | EmberCrown | Infantry | 300 | 3 / Heavy | 14–17 Normal | 0,55 | 2,5 | 110 Gold, 10 Holz | 1 |
| `spearman` | Speerträger | EmberCrown | Infantry | 260 | 2 / Medium | 11–14 Pierce | 1,0 | 2,6 | 80 Gold, 30 Holz | 1 |
| `archer` | Bogenschütze | EmberCrown | Ranged | 180 | 0 / Light | 12–15 Pierce | 5,5 | 2,7 | 90 Gold, 40 Holz | 1 |
| `knight` | Ritter | EmberCrown | Cavalry | 520 | 4 / Heavy | 22–28 Normal | 0,7 | 3,8 | 220 Gold, 40 Holz, 20 Stein | 2 |
| `hero_isolde` | Isolde Raventhorn | EmberCrown | Hero | 650 | 4 / Hero | 26–32 Hero | 0,7 | 3,0 | – (Mission) | 2 |
| `hollow_thrall` | Hohlknecht | Hollowed | Monster | 200 | 1 / Medium | 10–13 Normal | 0,55 | 2,3 | – | 0 |
| `hollow_archer` | Knochenschütze | Hollowed | Ranged | 150 | 0 / Light | 10–13 Pierce | 5,0 | 2,4 | – | 0 |
| `grave_brute` | Grabkoloss | Hollowed | Monster | 700 | 2 / Heavy | 28–36 Siege | 0,8 | 1,9 | – | 0 |
| `boss_pale_reeve` | Der Fahle Vogt | Hollowed | Boss | 2 400 | 5 / Hero | 34–42 Hero | 0,9 | 2,6 | – | 0 |

### 1.4 `BuildingDef` (`#[serde(default)]`) und `TowerAttack`

| Feld | Typ | Standard | Bedeutung |
|---|---|---|---|
| `id`, `name`, `description` | `String` | leer | ID, Anzeigename, Beschreibung |
| `faction` | `Faction` | `EmberCrown` | Zugehörigkeit |
| `size` | `(u8, u8)` | (2, 2) | Grundfläche in Kacheln (x, y); 0 ist ungültig |
| `hp` | `f32` | 800 | Lebenspunkte |
| `armor`, `armor_type` | `f32`, `ArmorType` | 5 / `Fortified` | Rüstung |
| `cost` | `Cost` | 0 | Baukosten |
| `build_time` | `f32` | 30 | Bauzeit (s) |
| `sight` | `f32` | 6 | Sichtweite |
| `pop_provided` | `u32` | 0 | Gelieferter Wohnraum (nur fertige Gebäude des Spielers; Gesamtgrenze `Rules::max_population`) |
| `trains` | `Vec<String>` | leer | IDs ausbildbarer Einheiten |
| `dropoff` | `bool` | `false` | Annahmestelle für Ressourcen |
| `food_per_sec`, `gold_per_sec` | `f32` | 0 | Passives Einkommen |
| `attack` | `Option<TowerAttack>` | `None` | Turmangriff |
| `requires` | `Vec<String>` | leer | Voraussetzung: fertige eigene Gebäude dieser IDs |
| `buildable` | `bool` | `true` | `false` = nicht vom Spieler baubar (Bergfried, Gegnergebäude) |
| `hotkey` | `Option<char>` | `None` | Bautaste |

`TowerAttack { damage, damage_type, range, cooldown, projectile }` – alle Felder erforderlich.

| ID | Name | Größe | LP | Kosten | Bauzeit | Besonderheit |
|---|---|---|---|---|---|---|
| `keep` | Bergfried | 3×3 | 2 200 | – (nicht baubar) | 120 s | `pop_provided` 10, trainiert `worker`, `dropoff`, `gold_per_sec` 0,6 |
| `farmstead` | Gehöft | 2×2 | 700 | 60 Gold, 80 Holz | 30 s | `pop_provided` 8, `food_per_sec` 0,45 |
| `barracks` | Kaserne | 3×2 | 1 300 | 160 Gold, 120 Holz, 40 Stein | 45 s | trainiert `swordsman`, `spearman`, `archer`, `knight` |
| `watchtower` | Wachturm | 1×1 | 600 | 70 Gold, 60 Holz, 60 Stein | 30 s | Angriff 18 Pierce, Reichweite 7,5, Intervall 1,6 s, `Arrow` |
| `hollow_shrine` | Hohlschrein | 3×3 | 3 500 | – | – | Gegnerbasis, nicht baubar |
| `bone_spire` | Knochenturm | 1×1 | 900 | – | – | Angriff 16 Magic, Reichweite 7,0, Intervall 1,8 s, `SoulBolt` |

### 1.5 `AbilityDef`, `Effect`

`AbilityDef` ist **nicht** `#[serde(default)]`: alle Felder müssen angegeben werden.

| Feld | Typ | Bedeutung |
|---|---|---|
| `id`, `name`, `description` | `String` | |
| `mana` | `f32` | Manakosten |
| `cooldown` | `f32` | Abklingzeit (s) |
| `range` | `f32` | Zielreichweite (bei `SelfCentered` unbenutzt) |
| `targeting` | `AbilityTargeting` | `SelfCentered` oder `Point` |
| `effects` | `Vec<Effect>` | Werden der Reihe nach ausgeführt |
| `hotkey` | `Option<char>` | Anzeigetaste |

| `Effect` | Parameter | Wirkung |
|---|---|---|
| `Damage` | `radius`, `amount`, `damage_type` | Flächenschaden um den Zielpunkt gegen Feinde |
| `Shield` | `amount`, `duration` | Schild auf dem Wirker |
| `Rally` | `radius`, `damage_mult`, `armor_bonus`, `morale`, `duration` | Verbündete im Radius: Buff plus Moral; beendet Flucht |
| `Drain` | `radius`, `amount` | Lebensentzug bei Feinden im Radius, heilt den Wirker |
| `Summon` | `unit`, `count` | Beschwört Einheiten (muss existierende Einheit sein) |
| `Terror` | `radius`, `morale` | Moralschaden bei Feinden im Radius |

Wirkung skaliert mit der Stufe des Wirkers um 8 % je Stufe über Stufe 1 (`level_mult`, `systems/abilities.rs`).

| ID | Name | Mana | Abklingzeit | Effekte |
|---|---|---|---|---|
| `ember_cleave` | Glutspalter | 70 | 10 s | `Damage(radius 2.4, 95, Magic)` |
| `rallying_banner` | Sammelbanner | 90 | 30 s | `Rally(radius 7, ×1,25 Schaden, +2 Rüstung, +40 Moral, 12 s)` |
| `ashen_ward` | Aschenschild | 60 | 20 s | `Shield(220, 12 s)` |
| `soul_drain` | Seelenentzug | 60 | 9 s | `Drain(radius 3.5, 45)` |
| `summon_thralls` | Ruf der Gräber | 100 | 25 s | `Summon(hollow_thrall, 4)` |
| `dread_wail` | Schreckensklage | 80 | 18 s | `Terror(radius 6, 45)`, `Damage(radius 3, 40, Magic)` |

### 1.6 `Rules` (`#[serde(default)]`, `assets/data/rules.ron`)

| Feld | Standard / Wert | Bedeutung |
|---|---|---|
| `tick_rate` | 20 | Simulationsschritte pro Sekunde |
| `day_length` | 360,0 | Dauer eines Tag-Nacht-Zyklus in Sekunden |
| `tree_wood` | 120 | Holz je Baum |
| `gold_mine_gold` | 6 000 | Gold je Mine |
| `quarry_stone` | 2 000 | Stein je Steinbruch |
| `gather_time` | 3,5 | Dauer eines Sammelvorgangs (s) |
| `max_population` | 80 | Obergrenze der Bevölkerung |
| `hero_respawn_time` | 40,0 | Wartezeit bis zur Rückkehr des Helden (s) |
| `xp_levels` | `[0, 100, 250, 450, 700, 1000, 1400, 1900, 2500, 3200]` | Erfahrungsschwellen (Stufe 1 bis 10) |
| `night_sight_mult` | 0,7 | Sichtweitenfaktor nachts (nur Spieler) |
| `fog_sight_mult` | 0,75 | Sichtweitenfaktor bei Nebelwetter (nur Spieler) |
| `rain_ranged_mult` | 0,9 | Fernkampf-Faktor bei Regen |
| `demolish_refund` | 0,5 | Rückerstattung beim Abriss |

### 1.7 Schadens- und Rüstungsmodell

Schaden gegen Einheiten (`SimWorld::deal_damage`):

```text
armor      = def.armor + Buff-Rüstung + (Stufe − 1) · 0,5
reduction  = armor ≥ 0 ? armor·0,06 / (1 + 0,06·armor) : −(1 − 0,94^(−armor))
amount     = roh · DamageType::multiplier(armor_type) · Klassenbonus · (1 − reduction)
```

Der Schaden geht zuerst in den Schild, dann in die LP. Nicht-furchtlose Einheiten verlieren Moral
(`amount / max_hp · 38`). Gebäude: `reduction = armor·0,06 / (1 + 0,06·armor)`, unverwundbare Gebäude
(`invulnerable`, Überleben-Modus: Hohlschrein) nehmen keinen Schaden.

Schadensart gegen Rüstungsart (`DamageType::multiplier`):

| | Unarmored | Light | Medium | Heavy | Fortified | Hero |
|---|---|---|---|---|---|---|
| **Normal** | 1,0 | 1,0 | 1,5 | 1,0 | 0,7 | 1,0 |
| **Pierce** | 1,5 | 2,0 | 0,75 | 0,8 | 0,35 | 0,5 |
| **Siege** | 1,5 | 1,0 | 0,5 | 1,0 | 1,5 | 0,5 |
| **Magic** | 1,0 | 1,25 | 0,75 | 2,0 | 0,35 | 0,5 |
| **Hero** | 1,0 | 1,0 | 1,0 | 1,0 | 0,5 | 1,0 |

Moral: Fällt sie unter 15 % des Maximums (und die Einheit ist kein Held und nicht furchtlos), flieht die Einheit
5 Sekunden lang (`routing`, 15 % schneller, `MoraleBroken`) und kehrt mit 40 % Moral zurück. Außerhalb des Kampfes
regeneriert Moral mit 5 pro Sekunde; bei Hungersnot sinkt sie um 1,2 pro Sekunde.

---

## 2. Laufzeitentitäten (`entity.rs`)

Entitäten liegen in `Arena<T>` (generationelle IDs: `Id<T> { index, generation }`). Spieler-IDs: `PLAYER = 0`
(Glutkrone), `ENEMY = 1` (Die Hohlen), `NEUTRAL = 2`.

### 2.1 `Unit`

| Feld | Typ | Bedeutung |
|---|---|---|
| `kind` | `UnitTypeId` | Typ |
| `owner` | `PlayerId` | Besitzer |
| `pos`, `prev_pos`, `vel` | `Vec2` | Position (Kacheln), Position des vorigen Ticks (Interpolation), Geschwindigkeit |
| `facing_right` | `bool` | Blickrichtung auf dem Bildschirm |
| `hp`, `max_hp`, `mana`, `max_mana`, `morale` | `f32` | Zustandswerte |
| `order` | `Order` | Aktueller Befehl |
| `queue` | `Vec<Order>` | Folgebefehle (Shift-Befehle) |
| `path`, `path_goal`, `path_pending`, `repath_timer`, `stuck_timer` | – | Wegfindungszustand |
| `attack_cd` | `f32` | Verbleibende Abklingzeit |
| `windup` | `Option<(f32, EntityRef)>` | Laufender Schlag: Restzeit und Ziel |
| `target` | `Option<EntityRef>` | Aktuelles Ziel |
| `carry` | `Option<(ResourceKind, u32)>` | Getragene Ressource |
| `work_timer` | `f32` | Fortschritt des Sammelvorgangs |
| `level`, `xp` | `u32` | Stufe, Erfahrung |
| `ability_cd` | `Vec<f32>` | Abklingzeiten je Fähigkeitsplatz |
| `buffs` | `Vec<Buff>` | `Buff { damage_mult, armor_bonus, remaining }` |
| `shield`, `shield_time` | `f32` | Schild und Restzeit |
| `routing` | `f32` | Restzeit der Flucht |
| `combat_timer` | `f32` | > 0 = kürzlich im Kampf (4 s nach Treffer) |
| `activity`, `activity_time` | `Activity`, `f32` | Grobe Aktivität für Animation und Audio |
| `last_attacker` | `Option<EntityRef>` | |
| `tag` | `Option<String>` | Missions-Markierung (aus `Placement::tag`) |
| `wave` | `Option<u32>` | Zugehörigkeit zu einer Angriffswelle |
| `alive_time` | `f32` | Lebensdauer |

### 2.2 `Order`

| Variante | Felder | Bedeutung |
|---|---|---|
| `Idle` | – | Standard; automatisches Angreifen in Reichweite |
| `Move` | `target: Vec2`, `attack_move: bool` | Gehen bzw. Angriffsmarsch |
| `Attack` | `target: EntityRef` | Ziel verfolgen und angreifen |
| `Gather` | `node: NodeId` | Ressource sammeln (Schleife mit Abgabe) |
| `ReturnCargo` | `node: Option<NodeId>` | Fracht zur nächsten Annahmestelle bringen |
| `Build` | `building: BuildingId` | Bauen bzw. reparieren |
| `Hold` | – | Position halten |
| `Patrol` | `a`, `b`, `to_b` | Zwischen zwei Punkten patrouillieren |
| `Cast` | `ability: u8`, `point: Vec2` | Fähigkeit wirken |
| `Guard` | `post: Vec2`, `radius: f32` | Wache (KI): im Radius angreifen, danach zurückkehren |

### 2.3 `Activity`

`Idle`, `Walk`, `Attack`, `Work`, `Cast`, `Flee` – steuert Animationsclip und (über Ereignisse) Audio.

### 2.4 `Building`, `TrainItem`

| Feld | Typ | Bedeutung |
|---|---|---|
| `kind` | `BuildingTypeId` | Typ |
| `owner` | `PlayerId` | |
| `tile`, `size` | `IVec2` | Obere linke Kachel der Grundfläche, Größe |
| `hp`, `max_hp` | `f32` | Baustellen starten mit 10 % LP |
| `progress` | `f32` | Baufortschritt 0..1 |
| `complete` | `bool` | Fertig |
| `queue` | `Vec<TrainItem>` | `TrainItem { unit, progress }`, höchstens 5 Einträge |
| `rally` | `Option<Vec2>` | Sammelpunkt |
| `attack_cd` | `f32` | Turm-Abklingzeit |
| `income_acc` | `f32` | Einkommens-Akkumulator |
| `tag` | `Option<String>` | Missions-Markierung (z. B. `player_keep`, `enemy_shrine`) |
| `invulnerable` | `bool` | Unverwundbar (Überleben-Modus, Gegnerbasis) |
| `last_hit_time` | `f32` | Zeitpunkt des letzten Treffers (Trefferblitz) |

### 2.5 `ResourceNode`, `NodeKind`

`ResourceNode { kind, tile, amount, max_amount, variant }`. `NodeKind`: `Tree` (1×1, Holz), `GoldMine` (2×2, Gold),
`StoneQuarry` (2×2, Stein). Die Kachelbelegung wird in `TileMap::occupied` geführt. Fest platzierte Minen und
Steinbrüche: Goldminen bei (7, 47), (50, 45), (63, 30); Steinbrüche bei (22, 47), (30, 20)
(`mapgen::populate_resources`); Bäume entstehen aus Rauschen.

### 2.6 `Projectile`

| Feld | Typ | Bedeutung |
|---|---|---|
| `kind` | `ProjectileKind` | `Arrow` (Tempo 11), `BoneBolt` (8), `SoulBolt` (7) Kacheln/s |
| `owner` | `PlayerId` | |
| `origin`, `pos`, `prev_pos` | `Vec2` | Startpunkt, aktuelle und vorige Position |
| `target`, `target_pos` | `EntityRef`, `Vec2` | Ziel und zuletzt bekannte Zielposition |
| `speed`, `damage`, `damage_type`, `bonus_class_mult`, `source` | – | Trefferdaten |
| `arc` | `f32` | Flugbogenhöhe in Kacheln (nur Darstellung) |
| `progress`, `total` | `f32` | Flugfortschritt |

### 2.7 `EntityRef`

`EntityRef::Unit(UnitId)` | `EntityRef::Building(BuildingId)` – Verweis auf angreifbare Ziele.

---

## 3. `SimWorld`

| Feld | Typ | Gespeichert | Bedeutung |
|---|---|---|---|
| `tick` | `u64` | ja | Tickzähler |
| `time` | `f32` | ja | Spielzeit in Sekunden |
| `settings` | `MatchSettings` | ja | Modus, Schwierigkeit, Seed, Heldentod |
| `mission_def` | `MissionDef` | ja | Die gestartete Mission (komplett im Spielstand) |
| `mission` | `MissionState` | ja | Ziele, ausgelöste Trigger, Ergebnis, entdeckte/gefallene Tags, überlebte Wellen |
| `map` | `TileMap` | ja | Terrain, Blockaden, Belegung, Dekoration, `revision` |
| `units` | `Arena<Unit>` | ja | |
| `buildings` | `Arena<Building>` | ja | |
| `nodes` | `Arena<ResourceNode>` | ja | |
| `projectiles` | `Vec<Projectile>` | ja | |
| `players` | `Vec<PlayerState>` | ja | Drei Einträge: „Glutkrone“, „Die Hohlen“, „Neutral“ |
| `env` | `Environment` | ja | Tageszeit, Tag/Nacht-Zähler, Wetter, Blitz |
| `fog` | `FogOfWar` | ja | Zellen 0/1/2, `revision`, `disabled` |
| `director` | `Director` | ja | Wellensteuerung |
| `chronicle` | `Chronicle` | ja | Chronik |
| `rng` | `ChaCha8Rng` | ja | Zufallszustand |
| `hero_respawn` | `Option<(UnitTypeId, f32)>` | ja | Wartender Held und Restzeit |
| `events` | `Vec<SimEvent>` | nein | Ereignisse des letzten Schritts |
| `pending` | `Vec<PlayerCommand>` | nein | Befehle für den nächsten Schritt |
| `pathfinder`, `spatial`, `scratch` | – | nein | Hilfsstrukturen |

Zugehörige Typen:

* `PlayerState { name, resources: Cost, pop, pop_cap, starving, stats: PlayerStats }`;
  `PlayerStats { units_trained, units_lost, kills, buildings_built, gathered: Cost }`.
* `Director { enabled, next_wave_in, wave, warned, boss_summons_done }`.
* `MatchSettings { mode: GameMode, difficulty: Difficulty, seed: u64, hero_permadeath: bool }`, Standard:
  Kampagne, Normal, Seed 897, kein dauerhafter Heldentod.
* `GameMode`: `Campaign`, `Skirmish`, `Survival`, `Sandbox`. `Difficulty`: `Story` (Wellenfaktor 0,65), `Normal` (1,0),
  `Hard` (1,4).
* `Environment { time_of_day (0..1; 0,25 Sonnenaufgang, 0,5 Mittag, 0,75 Sonnenuntergang), day, night, weather,
  weather_timer, weather_intensity, lightning_timer, forced_weather, was_night }`.
  `DayPhase`: `Dawn` (0,21–0,30), `Day` (0,30–0,72), `Dusk` (0,72–0,80), sonst `Night`. Startzeit 0,30.
  `Weather`: `Clear`, `Overcast`, `Fog`, `Rain`, `Storm`, `Ashfall`.
* `TileMap { width, height, terrain, static_block, occupied, decor, revision }`; die Karte der Vertical Slice ist
  72×72. `Terrain`: `Grass`, `DarkGrass`, `Dirt`, `Road`, `Stone`, `Sand`, `Ash`, `Ford`, `Water`, `DeepWater`
  (begehbar: alle außer den beiden Wasserarten; Bewegungskosten: Straße 0,8, Furt 1,6, Sand 1,15, sonst 1,0).
* `Chronicle { entries: Vec<ChronicleEntry> }`, `ChronicleEntry { year, day, phase, kind, text }`,
  `ChronicleKind`: `Founding`, `Construction`, `Battle`, `Death`, `Hero`, `Omen`, `Victory`, `Defeat`.

---

## 4. Missionen (`mission.rs`)

Missionen sind RON-Dateien unter `assets/data/missions/` (und `data/missions/` in Mods). `MissionDef::load(path)`.
Das Beispiel ist `assets/data/missions/m01_last_settlement.ron`.

### 4.1 `MissionDef`

Alle Felder sind erforderlich (kein `#[serde(default)]` auf Strukturebene; `WaveRules` selbst ist aber
`#[serde(default)]`).

| Feld | Typ | Bedeutung |
|---|---|---|
| `id` | `String` | z. B. `m01_last_settlement` |
| `name`, `description` | `String` | Anzeige im Menü |
| `map_seed` | `u64` | Seed der Kartenvariation |
| `start_resources` | `Cost` | Startressourcen des Spielers |
| `waves` | `WaveRules` | Regeln der Angriffswellen |
| `placements` | `Vec<Placement>` | Startaufstellung |
| `objectives` | `Vec<ObjectiveDef>` | Ziele |
| `triggers` | `Vec<TriggerDef>` | Auslöser |
| `defeat_tag` | `String` | Tag des Gebäudes, dessen Zerstörung zur Niederlage führt (Vertical Slice: `player_keep`) |
| `enemy_base_tag` | `String` | Tag des Gegnerhauptgebäudes (Wellen-Spawn, Gefechtsziel; Vertical Slice: `enemy_shrine`) |

`WaveRules` (alle Standardwerte): `enabled = true`, `first_wave = 200.0`, `interval_day = 150.0`,
`interval_night = 95.0`, `base_budget = 3.0`, `budget_per_wave = 1.7`, `night_mult = 1.5`.
Mission 1 setzt `first_wave: 230.0`.

`Placement { id, owner: u8, x: f32, y: f32, count: u32 = 0, tag: Option<String> = None, guard: bool = false }`:
`id` verweist auf eine Gebäude- oder Einheiten-ID; Gebäude werden an der Kachel (`x as i32`, `y as i32`) fertig
platziert; Einheiten werden `max(count, 1)`-fach in einer kleinen Spirale um (x, y) erzeugt; `guard: true` gibt ihnen
`Order::Guard` mit Radius 6,5. Unbekannte IDs lassen `SimWorld::new` mit `Err` scheitern.

`ObjectiveDef { id, text, optional: bool = false }`.

### 4.2 `TriggerDef`, `Condition`, `Action`

`TriggerDef { id, condition: Condition, actions: Vec<Action>, modes: Vec<GameMode> = [] }`. `modes` leer = in allen
Modi aktiv. Jeder Auslöser feuert **höchstens einmal** (`MissionState::fired`); Auswertung alle 10 Ticks (2 Hz).

| `Condition` | Wahr, wenn … |
|---|---|
| `AtTime(f32)` | Spielzeit ≥ Wert (s) |
| `BuildingCompleted(String)` | Der Spieler hat ein fertiges Gebäude dieser ID |
| `BuildingCount { building, count }` | Mindestens `count` fertige Gebäude dieser ID |
| `UnitCount { unit, count }` | Mindestens `count` Einheiten dieser ID |
| `MilitaryCount(u32)` | Mindestens so viele Einheiten außer `Worker` und `Hero` |
| `ObjectiveCompleted(String)` | Ziel abgeschlossen |
| `NightStarted(u32)` | `env.night` ≥ n |
| `DayStarted(u32)` | `env.day` ≥ n |
| `TagDead(String)` | Ein Objekt mit diesem Tag wurde zerstört |
| `TagDiscovered(String)` | Ein Objekt mit diesem Tag (nicht des Spielers) war sichtbar |
| `HeroLevel(u32)` | Held des Spielers hat Stufe ≥ n |
| `WavesSurvived(u32)` | Mindestens n Wellen vollständig besiegt |
| `All(Vec<Condition>)` / `Any(Vec<Condition>)` | Alle / mindestens eine |

| `Action` | Wirkung |
|---|---|
| `Message { speaker, text, voice: Option<String> }` | Meldung im HUD; `voice` ist eine Event-ID des Audio-Manifests |
| `AddObjective(String)` | Ziel aktivieren (`ObjectiveAdded`) |
| `CompleteObjective(String)` | Ziel abschließen (`ObjectiveCompleted`) |
| `Victory` / `Defeat` | Mission beenden (`Victory`/`Defeat`) |
| `SpawnWave(f32)` | Welle mit Budget (mal Schwierigkeitsfaktor) auslösen |
| `Grant(Cost)` | Ressourcen an den Spieler |
| `EnableWaves(bool)` | Wellen-Direktor ein-/ausschalten (außer Sandbox) |
| `SetWeather(Weather)` | Wetter setzen (150 s) |
| `Chronicle(String)` | Chronikeintrag (`Omen`) |

Eingebaute Regeln (`systems/script.rs`, unabhängig von Triggern): Niederlage, wenn das Gebäude mit `defeat_tag`
fehlt oder der Spieler weder Gebäude noch Einheiten hat (außer Sandbox); im Modus Gefecht Sieg, wenn der
`enemy_base_tag` zerstört ist.

### 4.3 `MissionState`

`objectives: Vec<(String, ObjectiveState)>`, `fired: Vec<String>`, `outcome: Option<Outcome>`,
`discovered_tags`, `dead_tags`, `waves_survived`. `ObjectiveState`: `Active`, `Completed`. `Outcome`: `Victory`, `Defeat`.

### 4.4 Wellen-Direktor

Pro Welle: `budget = base_budget + budget_per_wave · (wave − 1)`; nachts `· night_mult`; im Modus Überleben
`· (1 + wave · 0,08)`; dann `· Difficulty::wave_mult()`. Das Budget wird in Einheiten umgesetzt: `hollow_thrall`
(1,0), `hollow_archer` (1,5), `grave_brute` (4,0, ab Welle 2). Nach der Welle: Intervall `interval_day` bzw.
`interval_night`. 20 s vorher kommt `WaveIncoming` mit `size 0` (Vorwarnung).

---

## 5. Ereignisse (`SimEvent`)

Die Simulation sammelt pro Schritt Ereignisse in `SimWorld::events`; das Spiel entnimmt sie nach jedem Schritt
(`std::mem::take`) und verteilt sie an Darstellung (`WorldView::on_event`), Audio (`audio_bridge::on_event`) und HUD.
Ereignisse sind nicht Teil des Spielstands.

| Variante | Felder |
|---|---|
| `UnitSpawned` | `id`, `kind`, `owner`, `pos` |
| `UnitDied` | `id`, `kind`, `owner`, `pos`, `facing_right` |
| `MeleeSwing` | `attacker`, `kind`, `pos` |
| `Hit` | `target: EntityRef`, `pos`, `amount`, `damage_type`, `blocked` |
| `ProjectileFired` | `kind`, `from`, `owner` |
| `ProjectileImpact` | `kind`, `pos`, `hit` |
| `BuildingPlaced` / `BuildingCompleted` / `BuildingDestroyed` | `id`, `kind`, `owner`, `pos` |
| `TrainCompleted` | `unit`, `kind`, `owner`, `pos` |
| `WorkStroke` | `unit`, `resource: Option<ResourceKind>`, `pos` (`None` = Bauen) |
| `ResourceDelivered` | `owner`, `kind`, `amount`, `pos` |
| `NodeDepleted` | `pos`, `tree` |
| `AbilityCast` | `unit`, `ability` (ID), `pos`, `target` |
| `LevelUp` | `unit`, `level` |
| `MoraleBroken` | `unit`, `pos` |
| `HeroFallen` | `kind`, `pos` |
| `HeroRespawned` | `unit`, `pos` |
| `WaveIncoming` | `wave`, `from`, `size` (0 = Vorwarnung) |
| `NightBegins` | `night` |
| `DayBegins` | `day` |
| `WeatherChanged` | `weather` |
| `Lightning` | – |
| `Message` | `speaker`, `text`, `voice: Option<String>` |
| `ObjectiveAdded` / `ObjectiveCompleted` | `id` |
| `Starvation` | `owner`, `active` |
| `Error` | `owner`, `text` |
| `Victory` / `Defeat` | – |

---

## 6. Befehle (`Command`)

`PlayerCommand { player: PlayerId, command: Command }`. Befehle werden mit `SimWorld::push_command` eingereiht und
am Anfang des nächsten Ticks angewendet (`systems::commands::apply`). Ungültige Befehle werden ignoriert oder
erzeugen ein `SimEvent::Error`. Einheitenlisten werden auf Besitz gefiltert.

| Variante | Felder | Wirkung |
|---|---|---|
| `Move` | `units`, `target: Vec2`, `attack_move: bool`, `queue: bool` | Bewegen (mit Formationsplätzen, `formation_slots`); `queue` hängt an |
| `Attack` | `units`, `target: EntityRef` | Ziel angreifen |
| `Stop` | `units` | Befehl abbrechen |
| `Hold` | `units` | Position halten |
| `Patrol` | `units`, `target` | Patrouille zwischen aktueller Position und Ziel |
| `Gather` | `units`, `node` | Ressource sammeln |
| `Build` | `units`, `kind: BuildingTypeId`, `tile: IVec2` | Baustelle anlegen und bauen (Prüfung Platz, Gelände erforscht, Voraussetzungen, Kosten) |
| `Repair` | `units`, `building` | Bauen/Reparieren |
| `Train` | `building`, `unit` | Ausbildung einreihen (Warteschlange ≤ 5, Wohnraum, Kosten) |
| `CancelTrain` | `building`, `index` | Eintrag abbrechen |
| `SetRally` | `building`, `point` | Sammelpunkt setzen |
| `UseAbility` | `unit`, `ability: u8`, `point` | Fähigkeit des Helden (Index in `UnitDef::abilities`) |
| `Demolish` | `building` | Abreißen (Rückerstattung `demolish_refund` = 50 % bei fertigen Gebäuden, 75 % bei Baustellen) |

---

## 7. Spielstandformat (`save.rs`)

Datei: `<slot>.acsave` in `<Nutzerverzeichnis>/saves/`. Endung `acsave` (`save::EXTENSION`).

```text
Offset  Größe     Inhalt
0       4         Magic "ASHC"
4       4         u32 Formatversion (little endian), derzeit FORMAT_VERSION = 1
8       4         u32 Header-Länge in Bytes (little endian, höchstens 1 MiB)
12      n         Header: JSON (SaveHeader)
12+n    4         u32 CRC32 des Payloads (crc32fast)
16+n    8         u64 Länge des Payloads
24+n    …         Payload: bincode 2.0 (serde, Standardkonfiguration)
```

Der JSON-Header kann ohne Dekodieren des Payloads gelesen werden (`read_header`, Spielstandliste).

`SaveHeader`:

| Feld | Typ | Bedeutung |
|---|---|---|
| `format_version` | `u32` | Dateiformat |
| `game_version` | `String` | `CARGO_PKG_VERSION` von `ashen_sim` |
| `id` | `Uuid` | Eindeutige Spielstand-ID (v4, bei jedem Speichern neu) |
| `name` | `String` | Name (der Slot-Name) |
| `mission_id`, `mission_name`, `mode` | `String` | Mission und Modus |
| `created_unix` | `u64` | Zeitstempel (Sekunden) |
| `play_time_secs` | `f32` | Spielzeit |
| `day` | `u32` | Spieltag |
| `type_table_hash` | `u32` | CRC32 der Typtabelle |

`SavePayload { world: SimWorld, type_table: Vec<String>, client_state: String }`:

* `type_table` = `GameData::type_table()`: Liste `u:<unit-id>`, `b:<building-id>`, `a:<ability-id>` in Ladereihenfolge.
  Da Typ-IDs Vektorpositionen sind, wird sie beim Laden mit den aktuell geladenen Daten **verglichen**; weicht sie
  ab (z. B. anderer Mod-Satz), meldet `decode` `SaveError::Incompatible` und das Spiel lädt nicht.
* `client_state` = Text (RON) des Spiels: Kamera, Zoom, Geschwindigkeit (`ClientState { camera, lines, speed }`).
  Das Spiel beschränkt beim Laden den Zoom auf 220–480 und die Geschwindigkeit auf 0–3.

Sicherheits- und Robustheitsmechanismen:

| Mechanismus | Umsetzung |
|---|---|
| Prüfsumme | CRC32 über den Payload; Abweichung → `SaveError::Checksum` |
| Versionierung | `FORMAT_VERSION`; andere Version → `SaveError::Version(v)` (kein Backup-Versuch) |
| Kompatibilität | Typtabelle (siehe oben) → `SaveError::Incompatible` (kein Backup-Versuch) |
| Abschneiden | Länge außerhalb der Datei → `SaveError::Format("Datei abgeschnitten")` |
| Atomares Schreiben | `write_atomic`: temporäre Datei `<slot>.tmp` schreiben, `sync_all`, vorhandene Datei in `<slot>.acsave.bak` umbenennen, temporäre Datei an den Zielnamen umbenennen; Verzeichnis wird bei Bedarf angelegt |
| Backup | Bei jedem Speichern bleibt die vorige Version als `.acsave.bak` erhalten. `load_from` versucht bei Format-/Prüfsummen-/E/A-Fehlern der Hauptdatei automatisch das Backup (mit Warnung) |
| Autosave | `autosave(dir, …, slots = 3)`: Slots `autosave_0` bis `autosave_2`; überschrieben wird der Slot mit dem ältesten `created_unix` (fehlender Slot = 0). Das Spiel löst alle `autosave_minutes` (Standard 5; 0 = aus) aus, nur während laufender, nicht pausierter Mission |
| Schnellspeichern | `F5` speichert in Slot `schnellspeicher`, `F9` lädt ihn |
| Slot-Namen | `sanitize`: nur ASCII-Buchstaben, Ziffern, `_`, `-`; sonst `_`; leer → `spielstand` |
| Liste | `list(dir)` liest alle Header, neueste zuerst; defekte Dateien erscheinen mit Fehlertext |

Nicht gespeichert werden Laufzeitpuffer (`events`, `pending`, Pfadfinder, Raster). Nach dem Laden werden sie neu
aufgebaut. Spielstände zwischen verschiedenen Formatversionen werden nicht migriert; eine Migrationsschicht ist nicht
vorhanden (bei Formatänderung ist `FORMAT_VERSION` zu erhöhen).

---

## 8. Audio-Manifest (`assets/audio/manifest.ron`)

Typen in `ashen_engine::audio::manifest`; Wurzelstruktur `AudioManifest(version, music, events, ambience)`.
Pfade sind relativ zu `assets/audio`, **ohne Endung**; das Suffix `_*` steht für nummerierte Varianten `_01`, `_02`, …
Ausführliche Beschreibung: [`../audio/audio_integration.md`](../audio/audio_integration.md).

| Typ | Felder |
|---|---|
| `AudioManifest` | `version: u32` (derzeit 1), `music: Vec<MusicTrack>`, `events: Vec<SoundEvent>`, `ambience: Vec<AmbienceLayer>` |
| `MusicTrack` | `id`, `intensity: Intensity`, `file` (Loop), `intro: Option<String>`, `looped: bool` |
| `SoundEvent` | `id`, `category: Category`, `files: Vec<String>` (Muster), `volume_db: f32`, `cooldown_ms: u32`, `max_instances: u32` (0 = unbegrenzt), `playback: Playback` (Standard `Random`) |
| `AmbienceLayer` | `id`, `file`, `volume_db` |
| `Intensity` | `Menu`, `Calm`, `Tension`, `Combat`, `Battle`, `Boss`, `Sting` (aufsteigend geordnet) |
| `Category` | `Voice`, `Narration`, `Dialogue`, `Sfx`, `Ui` |
| `Playback` | `Random` (zufällige Variante ohne direkte Wiederholung), `Sequential` (alle Teile nacheinander) |

Aktueller Umfang: 8 Musikspuren (6 Intensitäten plus zwei Stings), 105 Events (Stimmen, Erzähler, Dialoge,
Effekte), 6 Ambience-Schichten; zusammen 124 erwartete Dateimuster (`manifest.patterns()`: Loops, Intros, Event-Muster,
Ambience-Dateien).
