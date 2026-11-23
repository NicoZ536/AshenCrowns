# ASHEN CROWNS – Modding-Handbuch

Dieses Handbuch beschreibt, was Mods im aktuellen Stand (Vertical Slice, Phase 1) können und was nicht. Es beruht auf
`crates/ashen_game/src/content.rs` (Laden, `mod.ron`), `crates/ashen_sim/src/data.rs` (`GameData::apply_overlay`),
`crates/ashen_sim/src/mission.rs` (Missionen) und `crates/ashen_tools/src/main.rs` (`validate-data`). Datenformate im
Detail: [`data_model.md`](data_model.md). Beispiel-Mod: `mods/example_mod/`.

**Kurzfassung:** Mods sind **reine Daten-Mods** (RON-Dateien). Sie können Einheiten, Gebäude und Fähigkeiten ändern oder
hinzufügen und zusätzliche Missionen liefern. Sie können **keinen Code** enthalten, **keine Engine** ändern und derzeit
weder Sprites noch Audio mitbringen. Ein Mod-Manager ist **geplant (Phase 14)**.

---

## 1. Ordnerstruktur

```text
mods/
└── mein_mod/                  Ordnername beliebig (die ID steht in mod.ron)
    ├── mod.ron                Pflicht: Metadaten
    └── data/
        ├── units.ron          optional: Einheiten (Hülle  (units: [ ... ]) )
        ├── buildings.ron      optional: Gebäude (Hülle  (buildings: [ ... ]) )
        ├── abilities.ron      optional: Fähigkeiten (Hülle  (abilities: [ ... ]) )
        ├── goods.ron          optional: Waren (Hülle  (goods: [ ... ]) ), siehe docs/design/rts_economy.md
        ├── items.ron          optional: Gegenstände der Helden (Hülle  (items: [ ... ]) ), siehe docs/design/rts_military.md
        ├── talents.ron        optional: Talentbäume (Hülle  (heroes: [ ... ]) ); gleicher `hero` ersetzt den Baum
        ├── upgrades.ron       optional: Schmiede-Upgrades (Hülle  (upgrades: [ ... ]) )
        └── missions/
            └── *.ron          optional: zusätzliche Missionen (je eine MissionDef)
```

Mods werden an zwei Orten gesucht (`content::discover_mods`):

1. `<assets>/../mods` – also `mods/` neben dem Ordner `assets/` (im Repository `mods/`).
2. `<Nutzerverzeichnis>/mods` – siehe [`project_structure.md`](project_structure.md), Abschnitt „Nutzerdaten-Verzeichnisse“
   (macOS: `~/Library/Application Support/AshenCrowns/mods`).

Ein Unterordner gilt nur dann als Mod, wenn er eine lesbare `mod.ron` enthält. Eine fehlerhafte `mod.ron` ergibt
eine Warnung im Log (`Mod <pfad> ungültig: …`) und der Mod wird übersprungen. Haben zwei Ordner dieselbe `id`, zählt
der zuerst gefundene (Projektordner vor Nutzerordner).

---

## 2. `mod.ron`

RON-Struktur `ModInfo` (`content.rs`). Kommentare mit `//` sind erlaubt.

| Feld | Typ | Pflicht | Bedeutung |
|---|---|---|---|
| `id` | `String` | ja | Eindeutige Mod-ID; wird in `settings.ron` (`enabled_mods`) gespeichert |
| `name` | `String` | ja | Anzeigename im Menü „Modifikationen“ |
| `version` | `String` | ja | Versionstext, wird im Menü angezeigt |
| `author` | `String` | ja | Autor |
| `description` | `String` | ja | Beschreibung |
| `load_order` | `i32` | nein (Standard 0) | Ladereihenfolge: kleinere Werte werden **zuerst** angewendet; bei Gleichstand entscheidet die `id` alphabetisch. Spätere Mods überschreiben frühere. |

Beispiel (`mods/example_mod/mod.ron`):

```ron
(
    id: "example_mod",
    name: "Glutmiliz (Beispiel-Mod)",
    version: "1.0.0",
    author: "Ashen Crowns Team",
    description: "Fügt die Glutmiliz hinzu und macht Speerträger etwas robuster. Dient als Vorlage für eigene Mods.",
    load_order: 100,
)
```

Aktivierung: Hauptmenü → „Modifikationen“ → Kontrollkästchen. Die Auswahl wird sofort in `settings.ron`
gespeichert und die Inhalte werden **ohne Neustart neu geladen** (`MenuAction::ReloadContent`; `Content::load`
beginnt immer wieder bei den Basisdaten). Aktive Mods erscheinen mit „aktiv“; Ladefehler erscheinen darunter in Rot.

---

## 3. Überschreib- und Ergänzungslogik (`GameData::apply_overlay`)

Für jeden aktiven Mod (in der Reihenfolge `load_order`, dann `id`) wird `apply_overlay(<mod>/data)` aufgerufen:

1. Die Dateien `units.ron`, `buildings.ron`, `abilities.ron`, `goods.ron`, `items.ron`, `talents.ron`, `upgrades.ron` werden – **falls vorhanden** – gelesen. Fehlende Dateien
   werden ohne Meldung ignoriert.
2. Für jeden Eintrag gilt: Gibt es bereits einen Eintrag mit **gleicher `id`**, wird er **vollständig ersetzt**;
   andernfalls wird der Eintrag **am Ende angehängt**.
3. Danach werden die Index-Tabellen neu aufgebaut und die **gesamten** Daten mit `validate()` geprüft
   (Querverweise, Werte).
4. Zurückgegeben wird eine Änderungsliste (`unit:<id>`, `building:<id>`, `ability:<id>`); das Spiel protokolliert die Anzahl.

**Wichtig – vollständige Ersetzung, keine Zusammenführung:**

* Ein ersetzender Eintrag muss **alle** Werte selbst angeben, die vom Standard abweichen sollen. Nicht genannte Felder
  erhalten den **Standardwert der Definition** (`UnitDef::default()` bzw. `BuildingDef::default()`, siehe
  `data_model.md`), **nicht** den Wert des ersetzten Basiseintrags. Das Beispiel `spearman` in
  `mods/example_mod/data/units.ron` wiederholt deshalb alle Basiswerte und ändert nur `hp`, `armor`, `speed`,
  `cost`, `train_time` und `morale`.
* `AbilityDef` hat **keine** Standardwerte: Fähigkeiten müssen vollständig angegeben werden (alle Felder, auch
  `hotkey`, `range`, `targeting`).
* Hat ein Mod ein Basisgebäude oder eine Basiseinheit ersetzt, übernimmt er spätere Änderungen am Basisinhalt nicht
  automatisch (z. B. wenn `barracks` im Basisspiel neue Einheiten ausbildet).
* `rules.ron` (Globale Regeln) wird **nicht** überlagert. Mods können `Rules` nicht ändern.
* Entfernen von Basisinhalten ist nicht möglich.

**Indizes und Spielstände:** Typ-IDs sind Positionen in den Vektoren. Neue Einträge hängen sich hinten an; die
Reihenfolge ergibt sich aus Basisdaten und Ladereihenfolge der Mods. Spielstände speichern die Typtabelle
(`u:<id>`, `b:<id>`, `a:<id>`) und lassen sich nur laden, wenn die aktuell geladenen Inhalte **exakt dieselbe**
Tabelle ergeben (`SaveError::Incompatible`). Wer Mods ein- oder ausschaltet, kann Spielstände unbrauchbar machen,
die mit anderem Mod-Satz erstellt wurden.

### 3.1 Vorlage: Einheit

```ron
// mods/mein_mod/data/units.ron
(
    units: [
        (
            id: "ember_militia", name: "Glutmiliz", faction: EmberCrown, class: Infantry,
            description: "Billige Bauernmiliz mit Fackeln und Mistgabeln.",
            hp: 170.0, armor: 1.0, armor_type: Medium,
            damage_min: 8.0, damage_max: 11.0, damage_type: Normal, range: 0.6, cooldown: 1.3, windup: 0.3,
            speed: 2.6, sight: 6.5, radius: 0.29,
            cost: (gold: 45.0, wood: 10.0), train_time: 9.0, pop: 1, upkeep: 0.04, morale: 70.0,
            xp_value: 8, hotkey: Some('M'),
        ),
    ],
)
```

Pflicht ist nur `id`; alle anderen Felder haben Standardwerte (`#[serde(default)]`). Gültige Aufzählungswerte stehen in
`data_model.md`, Abschnitt 1.1. RON-Schreibweisen: optionale Felder als `Some(...)`/`None`, Kosten als Tupel
`(gold: 50.0)`, Listen mit `[...]`.

### 3.2 Vorlage: Gebäude, das die Einheit ausbildet

```ron
// mods/mein_mod/data/buildings.ron – ersetzt die Kaserne vollständig
(
    buildings: [
        (
            id: "barracks", name: "Kaserne", faction: EmberCrown, size: (3, 2),
            description: "Bildet Schwertkämpfer, Speerträger, Bogenschützen, Ritter und Glutmiliz aus.",
            hp: 1300.0, armor: 4.0, armor_type: Fortified, cost: (gold: 160.0, wood: 120.0, stone: 40.0),
            build_time: 45.0, sight: 6.0,
            trains: ["swordsman", "spearman", "archer", "knight", "ember_militia"], hotkey: Some('B'),
        ),
    ],
)
```

### 3.3 Vorlage: Fähigkeit

```ron
// mods/mein_mod/data/abilities.ron – alle Felder sind Pflicht
(
    abilities: [
        (
            id: "flame_burst", name: "Flammenstoß", hotkey: None,
            description: "Flächenschaden um den Wirker.",
            mana: 50.0, cooldown: 12.0, range: 0.0, targeting: SelfCentered,
            effects: [Damage(radius: 2.0, amount: 60.0, damage_type: Magic)],
        ),
    ],
)
```

Eine Fähigkeit wirkt, wenn ihre ID in `abilities` einer Einheit steht (Reihenfolge = Taste `Q`, `W`, `E` des
ausgewählten Helden). Verfügbare Effekte: `Damage`, `Shield`, `Rally`, `Drain`, `Summon`, `Terror`
(`data_model.md`, Abschnitt 1.5).

---

## 4. Missionen aus Mods

Jede `*.ron`-Datei unter `<mod>/data/missions/` wird – **nur wenn der Mod aktiv ist** – nach den Basismissionen geladen
(alphabetisch sortiert je Ordner) und erscheint im Menü „Neues Spiel“ in der Missionsauswahl.

* Format: `MissionDef`, siehe `data_model.md`, Abschnitt 4 und das Beispiel `assets/data/missions/m01_last_settlement.ron`.
* `Placement::id` muss eine existierende Einheit oder ein existierendes Gebäude sein (Basis oder Mod).
* Voice-Verweise (`voice: Some("…")`) sind Event-IDs des **globalen** Audio-Manifests `assets/audio/manifest.ron`;
  Mods können keine eigenen Audio-Events ergänzen. Unbekannte Events werden stumm übersprungen.
* Die Mission-ID wird **nicht** auf Eindeutigkeit geprüft: Eine Mod-Mission mit der ID einer Basismission **ersetzt sie
  nicht**, sondern erscheint zusätzlich in der Liste.
* Der Menüknopf „Kampagne“ und die Modi „Gefecht“/„Sandbox“ im Hauptmenü starten immer die **erste** geladene Mission
  (Mission 1 der Basisdaten). Mod-Missionen werden über „Neues Spiel“ gewählt.

Minimalgerüst (Struktur wie `MissionDef`, Koordinaten und Tags nach Mission 1, weil die Karte fest ist):

```ron
(
    id: "mod_testmission",
    name: "Testmission",
    description: "Eine Mod-Mission.",
    map_seed: 897,
    start_resources: (gold: 300.0, wood: 200.0, stone: 100.0, food: 150.0),
    waves: (enabled: true, first_wave: 180.0),
    defeat_tag: "player_keep",
    enemy_base_tag: "enemy_shrine",
    placements: [
        (id: "keep", owner: 0, x: 12.0, y: 53.0, tag: Some("player_keep")),
        (id: "worker", owner: 0, x: 16.5, y: 52.5, count: 5),
        (id: "hero_isolde", owner: 0, x: 17.5, y: 53.5, tag: Some("hero")),
        (id: "hollow_shrine", owner: 1, x: 60.0, y: 12.0, tag: Some("enemy_shrine")),
    ],
    objectives: [(id: "obj_start", text: "Überlebt die erste Welle.")],
    triggers: [
        (id: "start", condition: AtTime(1.0), actions: [AddObjective("obj_start")]),
        (id: "done", condition: WavesSurvived(1), actions: [CompleteObjective("obj_start"), Victory]),
    ],
)
```

Auslöser feuern höchstens einmal und werden alle 10 Ticks (2 Hz) ausgewertet. Mit `modes: [Campaign]` lässt sich ein
Auslöser auf bestimmte Spielmodi beschränken (Weglassen = alle Modi).

**Fehlerverhalten:** Scheitert der Start einer Mission (z. B. unbekannte ID in `placements`), zeigt das Spiel einen
Fehlerdialog mit nur einem Knopf „Beenden“. Missionsdateien mit Parse-Fehlern erscheinen dagegen als Meldung im
Mod-Menü und werden nicht geladen. Prüfen Sie Missionen daher vor dem Spielen (Abschnitt 5).

---

## 5. Validierung

```text
cargo run -p ashen_tools -- validate-data --mod mods/example_mod
```

Optionen: `--assets <dir>` (Asset-Verzeichnis, sonst automatisch gesucht), `--mod <dir>` (mehrfach möglich; das
Verzeichnis, das `data/` enthält). Ablauf:

1. Basisdaten laden (`GameData::load_dir`) und ihre Anzahl ausgeben.
2. Jeden `--mod` in der angegebenen Reihenfolge anwenden (`apply_overlay`) und die Anzahl der Einträge ausgeben.
3. Alle Missionen in `assets/data/missions/*.ron` laden, jede Platzierungs-ID prüfen und die Mission **einmal starten**
   (`SimWorld::new`), um Karte und Aufstellung zu testen. Ausgabe je Mission: Ziele und Auslöser.
4. Abschluss `<n> Mission(en) geprüft – alles konsistent.`

Exit-Code `0` bei Erfolg, `1` bei Fehler (mit Dateipfad und Grund, z. B. `Datei … fehlerhaft: …` oder
`Ungültige Spieldaten: Gebäude barracks trainiert unbekannte Einheit …`), `2` bei falscher Bedienung.

Was geprüft wird (`GameData::validate`): `hp > 0`, `speed >= 0`, `damage_max >= damage_min`; Fähigkeiten einer Einheit
existieren; `trains` verweist auf existierende Einheiten; `requires` auf existierende Gebäude; Gebäudegröße ungleich 0;
`Summon` verweist auf existierende Einheit.

**Was nicht geprüft wird** (bekannte Lücken, siehe Phase 14 der Roadmap):

* Missionen **aus Mods** (`<mod>/data/missions`) – `validate-data` liest nur `assets/data/missions`. Behelf: Die
  Missionsdatei vorübergehend nach `assets/data/missions/` kopieren, `validate-data --mod <mod>` ausführen und die
  Kopie wieder entfernen.
* Verweise innerhalb von Auslösern (Ziel-IDs, Tags, Gebäude-/Einheiten-IDs in Bedingungen) und Audio-Event-IDs.
* Doppelte Hotkeys, fehlende Sprites oder Icons.

Zusätzlich lohnt ein Test im Spiel: `RUST_LOG=debug cargo run --release` meldet fehlende Audio-Events und Sprites.

---

## 6. Beispiel-Mod `example_mod`

`mods/example_mod/` („Glutmiliz“, Version 1.0.0, `load_order: 100`):

| Datei | Inhalt |
|---|---|
| `mod.ron` | Metadaten (siehe oben) |
| `data/units.ron` | **Ersetzt** `spearman` (LP 260 → 300, Rüstung 2 → 3, Tempo 2,6 → 2,5, Kosten Gold 80 → 85, Ausbildung 14 → 15 s, Moral 95 → 100, neue Beschreibung) und **fügt** `ember_militia` (Glutmiliz: 170 LP, Rüstung 1, Schaden 8–11, Kosten 45 Gold + 10 Holz, Ausbildung 9 s, Taste `M`) hinzu |
| `data/buildings.ron` | **Ersetzt** `barracks`: `trains` enthält zusätzlich `ember_militia` |

Ausprobieren:

```text
cargo run -p ashen_tools -- validate-data --mod mods/example_mod      # Konsistenz prüfen
cargo run --release                                                   # Menü → Modifikationen → „Glutmiliz“ aktivieren
```

Nach der Aktivierung erscheint die Glutmiliz im Ausbildungsmenü der Kaserne (Taste `M`). Da der Mod keine Sprites
mitbringt, verwendet das Spiel für `ember_militia` die Figur des Schwertkämpfers und dessen Porträt
(`ArtLibrary::unit_anims`/`portrait`); Sprachausgaben für `voice.ember_militia.*` gibt es nicht.

Der Test `mod_overlay_overrides_and_adds` (`crates/ashen_sim/tests/simulation.rs`) sichert dieses Verhalten ab.

---

## 7. Grenzen

| Grenze | Erläuterung |
|---|---|
| **Kein Code** | Mods sind RON-Dateien; es gibt keine Skriptsprache und keine Plug-in-Schnittstelle. Logik lässt sich nur über die vorhandenen `Condition`/`Action`-Varianten (Missionen) und die Effekttypen (Fähigkeiten) ausdrücken. |
| **Keine Engine-Änderungen** | Rendering, Shader, Simulationsregeln, Schadens-/Rüstungstabelle (`DamageType::multiplier`) sind im Code fest. Neue Schadensarten, Rüstungsarten, Einheitenklassen, Ressourcen, Effekt- oder Befehlsarten sind nur durch Codeänderung möglich. |
| `Rules` fest | `rules.ron` kann nicht per Mod überlagert werden (Tickrate, Tageslänge, Ressourcenmengen, Bevölkerungsgrenze, XP-Stufen). |
| Karte fest | Die Karte entsteht aus `mapgen` mit festem Layout (Fluss, Straße, Basen, Minen); Missionen steuern nur `map_seed`. |
| Spielerslots fest | Drei Spieler (Spieler, Hohle, neutral); Feindschaft ist fest verdrahtet. |
| Wellen-KI | Der Wellen-Direktor erzeugt ausschließlich `hollow_thrall`, `hollow_archer` und `grave_brute` (feste IDs). Wer diese Einheiten per Mod ersetzt, verändert die Wellen. Fehlt eine dieser IDs in den Daten, bricht die Wellenerzeugung ab, sobald diese Art gewürfelt wird (die Welle fällt kleiner aus oder entfällt). |
| Bau-Menü | Das Baumenü der Arbeiter zeigt alle Gebäude mit `buildable: true` und `faction: EmberCrown` (inklusive Mod-Gebäuden, `mission::player_buildables`); das Tastenkürzel kommt aus `hotkey`. Ohne eigenes Sprite erscheint ein Mod-Gebäude als magentafarbenes Ersatz-Sprite. |
| Tasten | Einheiten-Hotkeys werden nur für die Buchstaben A, B, C, D, E, F, G, K, M, P, Q, S, T, V, W, X, Z verarbeitet. Auf Konflikte mit bestehenden Tasten achten. |
| Keine Sprites/Audio im Mod | Mods haben keinen eigenen Sprite- oder Audioordner. Neue Einheiten verwenden eine Basisfigur (Glutkrone/Neutral: Schwertkämpfer, Hohle: Hohlknecht), neue Gebäude werden als magentafarbenes Ersatz-Sprite gezeichnet, neue Fähigkeiten ohne Icon ebenfalls. Handgezeichnete PNG-Overrides (`assets/sprites/`) gelten global für das Spiel, nicht pro Mod (siehe [`../art/asset_production_plan.md`](../art/asset_production_plan.md)). |
| Spielstände | Nur ladbar bei identischem Mod-Satz (Typtabelle). |
| Fehlerhafte Mods | `apply_overlay` arbeitet atomar: Ein Mod wird zuerst auf eine Kopie der Daten angewendet und geprüft; schlägt eine Datei fehl, bleiben die Daten unverändert und der Mod wird als fehlerhaft gemeldet (Test `faulty_mod_leaves_data_untouched`). |

---

## 8. Geplant: Mod-Manager (Phase 14)

Der Menüpunkt „Modifikationen“ bietet heute nur Ein-/Ausschalten; die Ladereihenfolge wird über `load_order` in der
`mod.ron` festgelegt. **Geplant** (Roadmap Phase 14, noch nicht implementiert) sind:

* Mod-Manager-Oberfläche mit Ladereihenfolge, Abhängigkeiten und Konfliktanzeige,
* erweiterte Validierung (Missionen aus Mods, Verweise in Auslösern),
* Karten- und Szenario-Editor, mit dem Autoren Missionen und Karten ohne Handarbeit an RON-Dateien erstellen.

Ob Mods künftig eigene Sprites und Audiodateien mitbringen dürfen, ist noch nicht entschieden und wäre eine
Erweiterung der Asset-Pipeline (siehe Produktionsplan, Stufe 0).
