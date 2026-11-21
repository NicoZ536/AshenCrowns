# ASHEN CROWNS – Projektstruktur, Build und Betrieb

Verzeichnisbaum, Befehle, Umgebungsvariablen und Nutzerdaten. Architektur: [`architecture.md`](architecture.md);
Module: [`modules.md`](modules.md).

---

## 1. Verzeichnisbaum

```text
AshenCrowns/
├── Cargo.toml                   Workspace (Resolver 3, Edition 2024, Rust ≥ 1.88), gemeinsame Abhängigkeiten, Build-Profile
├── Cargo.lock
├── CLAUDE.md                    Arbeitsanweisungen für agentische Sitzungen im Projekt (keine Spielinhalte)
├── README.md                    (derzeit leer)
├── .gitignore                   target/, .DS_Store, *.log, /saves, screenshots/*.png (außer vertical_slice_*.png)
│
├── crates/
│   ├── ashen_core/              Mathe, Iso-Projektion, Rauschen, Farben (ohne Grafik/Plattform)
│   │   └── src/                 color.rs · iso.rs · lib.rs · noise.rs
│   ├── ashen_sim/               Deterministische Simulation (ohne Grafik)
│   │   ├── src/
│   │   │   ├── lib.rs           Modulliste, Re-Exporte
│   │   │   ├── world.rs         SimWorld, fester Schritt, Schaden, state_hash
│   │   │   ├── data.rs          GameData: UnitDef, BuildingDef, AbilityDef, Rules, Overlay-Logik
│   │   │   ├── entity.rs        Unit, Building, ResourceNode, Projectile, Order, Activity
│   │   │   ├── arena.rs         Generationelle Arena
│   │   │   ├── command.rs       Command, PlayerCommand
│   │   │   ├── events.rs        SimEvent
│   │   │   ├── mission.rs       MissionDef, Conditions, Actions, MatchSettings
│   │   │   ├── map.rs           TileMap, Terrain, Decor
│   │   │   ├── mapgen.rs        Kartengenerator der Aschmark
│   │   │   ├── path.rs          A*-Wegfindung
│   │   │   ├── spatial.rs       Nachbarschaftsraster
│   │   │   ├── fog.rs           Kriegsnebel
│   │   │   ├── env.rs           Tageszeit und Wetter
│   │   │   ├── chronicle.rs     Chronik
│   │   │   ├── save.rs          Spielstandformat .acsave
│   │   │   └── systems/         abilities · ai · combat · commands · economy · environment · fog · movement · script · units
│   │   └── tests/simulation.rs  Integrationstests der Simulation
│   ├── ashen_engine/            Spielunabhängige 2D-Engine
│   │   └── src/
│   │       ├── lib.rs · app.rs · gpu.rs · time.rs · input.rs · anim.rs
│   │       ├── render/          mod.rs · camera.rs · particles.rs · profile.rs · shaders.rs · types.rs
│   │       │   └── shaders/     common.wgsl · sprite.wgsl · light.wgsl · composite.wgsl · bloom.wgsl · final.wgsl
│   │       ├── assets/          mod.rs (Pfade, Overrides) · pixel.rs (Canvas) · atlas.rs
│   │       ├── audio/           mod.rs · manifest.rs · scanner.rs · backend.rs
│   │       └── ui/              mod.rs (egui-Integration) · theme.rs
│   ├── ashen_game/              Das Spiel (Binary ashen_crowns)
│   │   └── src/
│   │       ├── main.rs          Einstieg, Kommandozeile
│   │       ├── game.rs          Szenenverwaltung, AutoRun
│   │       ├── content.rs       Daten, Mods, Missionen laden
│   │       ├── settings.rs      Einstellungen, Nutzerverzeichnisse
│   │       ├── menu.rs · hud.rs · ui_common.rs
│   │       ├── mission.rs       Missionsszene (Takt, Kamera, Eingabe, Speichern)
│   │       ├── view.rs          Weltdarstellung und Atmosphäre
│   │       ├── audio_bridge.rs  SimEvent → Audio
│   │       └── art/             Prozedurale Pixel-Art: mod · palette · terrain · units · buildings · props · fx · icons
│   └── ashen_tools/             CLI-Werkzeuge
│       └── src/main.rs          audio-scan, validate-data
│
├── assets/
│   ├── data/                    Spielinhalte (RON)
│   │   ├── units.ron · buildings.ron · abilities.ron · rules.ron
│   │   └── missions/            m01_last_settlement.ron
│   ├── audio/
│   │   ├── manifest.ron         Audio-Manifest (Events, Musik, Ambience)
│   │   ├── audio_report.md      Letzter Scanner-Bericht (wird von audio-scan überschrieben)
│   │   ├── music/ · ambience/ · sfx/{buildings,combat,magic,ui,units,world}/
│   │   └── voices/{characters,heroes,narrator,units}/     (aktuell nur .gitkeep, keine Audiodateien)
│   └── sprites/                 Optionale PNG-Overrides (<name>.png, <name>_n.png) – Ordner entsteht bei Bedarf
│
├── mods/
│   └── example_mod/             Beispiel-Mod (mod.ron, data/units.ron, data/buildings.ron)
│
├── docs/
│   ├── canon.md                 Verbindliche Kanon-Referenz (Namen, IDs, Dateinamenskonvention)
│   ├── design/                  game_design_document.md · campaign.md
│   ├── lore/                    world_bible.md · glossary.md
│   ├── audio/                   audio_integration.md · elevenlabs_*.md · suno_master_production.md
│   ├── art/                     graphics_and_shaders.md · asset_production_plan.md
│   └── tech/                    architecture.md · modules.md · data_model.md · project_structure.md · roadmap.md · modding.md
│
└── screenshots/                 Headless-Aufnahmen (Dateien nach Muster vertical_slice_*.png werden versioniert)
```

Hinweise:

* `assets/sprites/` ist im Repository nicht vorhanden; alle Sprites werden prozedural erzeugt. Der Ordner wird nur
  benötigt, wenn handgezeichnete Sprites eingebunden werden (siehe [`../art/asset_production_plan.md`](../art/asset_production_plan.md)).
* Das Spiel sucht `assets/` selbstständig (siehe `ASHEN_ASSETS` unten); Mods in `mods/` neben `assets/`.
* `/saves` in `.gitignore` bezieht sich auf das Repository-Wurzelverzeichnis. Spielstände liegen jedoch im
  Nutzerverzeichnis (Abschnitt 5).

---

## 2. Build, Start, Test

Voraussetzung: Rust ≥ 1.88 (Edition 2024). Auf macOS (Zielplattform) ist Metal über `wgpu` ohne Zusatzinstallation
verfügbar. Für Konvertierung von Audioformaten wird optional `ffmpeg` benötigt (`audio-scan --convert`).

| Zweck | Befehl |
|---|---|
| Spiel starten (Release, empfohlen) | `cargo run --release` |
| Spiel starten (Entwicklungsprofil) | `cargo run` |
| Nur bauen | `cargo build --release` |
| Ohne Audioausgabe-Abhängigkeit bauen | `cargo run --release -p ashen_game --no-default-features` |
| Alle Tests | `cargo test --workspace` |
| Tests einer Crate | `cargo test -p ashen_sim` (Simulation inkl. `tests/simulation.rs`), `cargo test -p ashen_engine`, `cargo test -p ashen_core`, `cargo test -p ashen_game` |
| Spieldaten prüfen | `cargo run -p ashen_tools -- validate-data` |
| Mod prüfen | `cargo run -p ashen_tools -- validate-data --mod mods/example_mod` |
| Audio prüfen | `cargo run -p ashen_tools -- audio-scan` |

`default-members` des Workspace ist `ashen_game`, `default-run` ist `ashen_crowns`; `cargo run --release` startet
daher das Spiel. Argumente für das Spiel folgen nach `--`:

```text
cargo run --release -- --windowed --profile quality
```

Build-Profile (Workspace-`Cargo.toml`): `dev` mit `opt-level = 1` (eigener Code) und `opt-level = 3` für alle
Abhängigkeiten; `release` mit `lto = "thin"`, `codegen-units = 4`, `panic = "abort"`, `debug = false`.

### 2.1 Spieloptionen (`cargo run --release -- <Optionen>`)

Aus `usage()` in `crates/ashen_game/src/main.rs` (`--help` oder `-h` gibt sie aus; unbekannte Optionen beenden das
Programm mit Exit-Code 2):

| Option | Wirkung |
|---|---|
| `--windowed` / `--fullscreen` | Fenster- bzw. Vollbildmodus (überschreibt die gespeicherte Einstellung für diesen Lauf) |
| `--profile <performance\|balanced\|quality>` | Grafikprofil (wird auch in den Einstellungen gespeichert; unbekannte Werte ergeben `balanced`) |
| `--no-audio` | Audioausgabe deaktivieren |

### 2.2 Screenshot-Optionen (Headless-Rendering)

Mit `--screenshot <datei.png>` rendert das Spiel **ohne Fenster** (`run_headless`) und beendet sich danach. Alle
weiteren Optionen steuern Szene und Bildinhalt. Szenen starten stets Mission Nr. 1 (die erste geladene Mission).

| Option | Wirkung | Standard |
|---|---|---|
| `--screenshot <datei.png>` | Ohne Fenster rendern und Bild speichern | – |
| `--size <BxH>` | Bildgröße, z. B. `1920x1080` | `1600x900` |
| `--frames <n>` | Gerenderte Frames vor dem Screenshot (feste Zeit 1/60 s je Frame) | `90` |
| `--scene <menu\|mission\|skirmish\|sandbox>` | `mission` = Kampagne, sonst entsprechender Modus; `menu` oder ohne Angabe = Hauptmenü | Menü |
| `--sim-seconds <s>` | Simulation vor dem Screenshot vorlaufen lassen (Spielsekunden) | 0 |
| `--time <0..1>` | Tageszeit setzen (0,25 Morgen, 0,5 Mittag, 0,76 Dämmerung, 0,9 Nacht) | Missionsstart (0,30) |
| `--weather <clear\|fog\|rain\|storm\|ash\|overcast>` | Wetter erzwingen (volle Intensität; verhindert Wetterwechsel) | zufällig |
| `--battle` | Gegnerwelle (7 Hohlknechte, 1 Grabkoloss) vor die Basis setzen | aus |
| `--select-army` | Alle Nicht-Arbeiter auswählen (zeigt HUD-Auswahl) | aus |
| `--camera <x,y>` | Kamera auf Kachel zentrieren | Bergfried |
| `--zoom <zeilen>` | Sichtbare Bildzeilen (220–480; durch das Grafikprofil nach unten begrenzt) | 330 |
| `--no-ui` | Oberfläche ausblenden | aus |

Beispiele:

```text
# Hauptmenü
cargo run --release -- --screenshot screenshots/test_menu.png --scene menu

# Mission am Mittag, Standardansicht
cargo run --release -- --screenshot screenshots/test_day.png --scene mission --time 0.5

# Nacht mit Nebel, ohne Oberfläche, Qualitätsprofil
cargo run --release -- --screenshot screenshots/test_night.png --scene mission --time 0.9 --weather fog --no-ui --profile quality

# Kampfszene vor der Basis, 20 Sekunden vorgelaufen, Armee ausgewählt
cargo run --release -- --screenshot screenshots/test_battle.png --scene mission --battle --sim-seconds 20 --select-army

# Hohlschrein im Nordosten (Kachel 58, 16), Gewitter
cargo run --release -- --screenshot screenshots/test_shrine.png --scene mission --camera 58,16 --weather storm --sim-seconds 5
```

Hinweise: Im Headless-Betrieb gibt es keine Audioausgabe und keine Eingabe. `--time` wird **nach** der
Vorlaufsimulation gesetzt. Die Aufnahmen unter `screenshots/` werden laut `.gitignore` nicht versioniert, außer
Dateien nach dem Muster `vertical_slice_*.png`. Benötigt wird ein GPU-Adapter (Metal; auf Linux Vulkan oder ein
Software-Adapter).

### 2.3 Werkzeuge (`ashen_tools`)

```text
cargo run -p ashen_tools -- <befehl> [optionen]
```

| Befehl | Optionen | Wirkung |
|---|---|---|
| `audio-scan` | `--assets <dir>` – Asset-Verzeichnis (sonst automatisch) | Durchsucht `assets/audio`, ordnet Dateien dem Manifest zu, prüft sie und schreibt einen Markdown-Bericht |
| | `--apply-renames` – ungültige Dateinamen umbenennen (nur wenn das Ziel nicht existiert) | |
| | `--convert` – m4a/aac/aiff/aif/opus/webm/wma/caf per `ffmpeg` nach `.ogg` (Vorbis q6, 48 kHz) konvertieren | |
| | `--report <datei.md>` – Berichtsdatei (Standard `assets/audio/audio_report.md`) | |
| `validate-data` | `--assets <dir>` | Lädt Basisdaten, wendet Mods an, lädt Missionen aus `assets/data/missions`, startet jede Mission einmal |
| | `--mod <dir>` – Mod-Verzeichnis (das `data/` enthält), mehrfach möglich | |

Ausgabe und Exit-Codes: `0` Erfolg; `1` Fehler (z. B. Ladefehler; bei `audio-scan` zusätzlich Hinweis, wenn
beschädigte Dateien gefunden wurden – der Exit-Code bleibt dabei `0`); `2` falsche Bedienung. Standard-Log-Filter
der Werkzeuge: `warn`. Details: [`../audio/audio_integration.md`](../audio/audio_integration.md), [`modding.md`](modding.md).

---

## 3. Umgebungsvariablen

| Variable | Gelesen von | Wirkung |
|---|---|---|
| `ASHEN_ASSETS` | `ashen_engine::assets::find_assets_dir` (Spiel und Werkzeuge) | Pfad zum `assets`-Verzeichnis; muss einen Unterordner `data` enthalten. Ohne Variable: neben der ausführbaren Datei, `../Resources/assets` (macOS-App-Bundle), bis zu vier Ebenen aufwärts, dann Arbeitsverzeichnis und bis zu vier Ebenen aufwärts. |
| `ASHEN_SHADER_DIR` | `ashen_engine::render::shaders::ShaderManager` | Verzeichnis mit `common.wgsl`, `sprite.wgsl`, `light.wgsl`, `composite.wgsl`, `bloom.wgsl`, `final.wgsl`. Aktiviert **Shader-Hot-Reload**: Änderungen werden alle 0,5 s erkannt und neu übersetzt; bei Fehlern bleibt die letzte funktionierende Version aktiv. Beispiel: `ASHEN_SHADER_DIR=crates/ashen_engine/src/render/shaders cargo run --release`. Fehlende Dateien im Verzeichnis fallen auf die eingebetteten Quellen zurück. |
| `ASHEN_USER_DIR` | `ashen_game::settings::data_dir` | Überschreibt das Nutzerdatenverzeichnis (Einstellungen, Spielstände, Mods). Nützlich für Tests und portable Installationen. |
| `RUST_LOG` | `env_logger` | Log-Filter. Standard im Spiel: `info,wgpu_core=warn,wgpu_hal=warn,naga=warn`; in `ashen_tools`: `warn`. Beispiel: `RUST_LOG=debug cargo run --release` (zeigt u. a. fehlende Audio-Events). |

Weitere vom System gelesene Variablen (Pfadbestimmung): `HOME`, `APPDATA` (Windows), `XDG_DATA_HOME` (Linux);
außerdem die `wgpu`-eigenen Umgebungsvariablen (z. B. zur Backend-Wahl), da die Instanz mit
`..._from_env` erzeugt wird.

---

## 4. Eingabe und Steuerung (Kurzreferenz)

Maßgeblich ist `ashen_game::mission::MissionScene::controls`.

| Eingabe | Wirkung |
|---|---|
| Pfeiltasten, Bildschirmrand (abschaltbar), mittlere Maustaste ziehen | Kamera bewegen |
| Mausrad | Zoom (220–480 Bildzeilen) |
| Linke Maustaste / Ziehrahmen | Auswählen (Shift erweitert, Doppelklick wählt gleiche Einheiten im Bild) |
| Rechte Maustaste | Kontextbefehl: Bewegen, Angreifen, Sammeln, Reparieren, Sammelpunkt |
| `A` / `S` / `H` / `P` | Angriffsmarsch / Stopp / Halten / Patrouille |
| `B` (mit Arbeitern), dann `F`/`B`/`T` | Baumenü: Gehöft, Kaserne, Wachturm |
| `Q` / `W` / `E` | Fähigkeiten des ausgewählten Helden |
| Buchstabe der Einheit bei ausgewähltem Gebäude | Ausbilden (z. B. `S` Schwertkämpfer, `P` Speerträger, `A` Bogenschütze, `K` Ritter, `W` Leibeigener) |
| `R` | Sammelpunkt setzen |
| `Strg`/`Cmd` + `0`–`9` | Gruppe speichern; Ziffer allein: Gruppe wählen (zweimal: Kamera darauf) |
| `H` (ohne Auswahl) oder `Pos1` | Helden wählen und Kamera darauf |
| `Leertaste` | Kamera auf erste ausgewählte Einheit |
| `+`/`-`, `Pause` oder `^` | Spielgeschwindigkeit (0–3), Pause |
| `F5` / `F9` | Schnellspeichern / Schnellladen (`schnellspeicher.acsave`) |
| `Esc` / `F10` | Abbrechen bzw. Pausenmenü |
| `Alt` halten | Lebensbalken aller Einheiten |

---

## 5. Nutzerdaten-Verzeichnisse

Basis (`settings::data_dir()`), in dieser Reihenfolge:

1. `ASHEN_USER_DIR`, falls gesetzt.
2. macOS: `~/Library/Application Support/AshenCrowns`
3. Windows: `%APPDATA%\AshenCrowns`
4. Linux: `$XDG_DATA_HOME/ashen_crowns`, sonst `~/.local/share/ashen_crowns`

| Pfad (relativ zur Basis) | Inhalt |
|---|---|
| `settings.ron` | Benutzereinstellungen (`UserSettings`: Grafikprofil, Vollbild, VSync, Lautstärken, Bildlauf, Schwierigkeit, Heldentod, FPS-Anzeige, aktive Mods, Autosave-Intervall). Wird beim Beenden geschrieben; fehlerhafte Datei → Standardwerte. |
| `saves/<slot>.acsave` | Spielstände (Format siehe [`data_model.md`](data_model.md)) |
| `saves/<slot>.acsave.bak` | Backup der vorherigen Version |
| `saves/autosave_0.acsave` … `autosave_2.acsave` | Rotierende Autosaves |
| `saves/schnellspeicher.acsave` | Schnellspeicher (`F5`/`F9`) |
| `mods/<ordner>/` | Mods des Nutzers (zusätzlich zum Ordner `mods/` neben `assets/`) |

Standardwerte der Einstellungen: Profil Ausgewogen, Fenstermodus, VSync an, Lautstärken (Gesamt 0,85 · Musik 0,7 ·
Effekte 0,85 · Stimmen 1,0 · Umgebung 0,6 · Oberfläche 0,7), Bildlauf am Rand an, Tempo 1,0, Schwierigkeit Normal,
kein dauerhafter Heldentod, FPS-Anzeige aus, keine Mods, Autosave alle 5 Minuten.
