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
├── README.md                    Überblick, Stand, Galerie, Steuerung, Dokumentationsliste
├── .gitignore                   target/, .DS_Store, *.log, /saves, screenshots/*.png (außer vertical_slice_*.png)
│
├── crates/
│   ├── ashen_core/              Mathe, Iso-Projektion, Rauschen, Farben (ohne Grafik/Plattform)
│   │   └── src/                 color.rs · iso.rs · lib.rs · noise.rs
│   ├── ashen_sim/               Deterministische RTS-Simulation (ohne Grafik)
│   │   ├── src/
│   │   │   ├── lib.rs           Modulliste, Re-Exporte
│   │   │   ├── world.rs         SimWorld, fester Schritt, Schaden, state_hash
│   │   │   ├── data.rs          GameData: UnitDef, BuildingDef, AbilityDef, Rules, Overlay-Logik
│   │   │   ├── rpg.rs           Gegenstände, Talente, Schmiede-Upgrades, Formationen
│   │   │   ├── entity.rs · arena.rs · command.rs · events.rs · spatial.rs · fog.rs · env.rs · chronicle.rs
│   │   │   ├── mission.rs       MissionDef, Conditions, Actions, MatchSettings
│   │   │   ├── scenario.rs      Szenario-Erweiterungen des Missionsformats (Karte, Bereiche, Startpositionen)
│   │   │   ├── challenge.rs     Herausforderungen, Wertung, Bestenliste
│   │   │   ├── campaign.rs      Echtzeit-Kampagne: Zeitalter, Missionen, Entscheidungen, Profil-Fortschritt
│   │   │   ├── map.rs · mapgen.rs (Aschmark) · genmap.rs (parametrische Karten für Gefecht und Editor)
│   │   │   ├── path.rs          A*-Wegfindung
│   │   │   ├── balance.rs       Testspieler und Konter-Matrix (Balancemessung)
│   │   │   ├── bench.rs         Reproduzierbare Last für Leistungsmessungen
│   │   │   ├── save.rs          Spielstandformat .acsave
│   │   │   └── systems/         abilities · ai · ai_player · casters · city · combat · commands · economy · environment · fog · formation · heroes · movement · script · tactics · units
│   │   ├── examples/sim_bench.rs   Sim-Benchmark
│   │   └── tests/               simulation · economy · military · tactics · balance · campaign · scenarios · bench_smoke
│   ├── ashen_strategy/          Deterministische Strategieebene der Großen Kampagne (ohne Grafik), siehe strategy_layer.md
│   │   ├── src/                 types · goods · catalog · mapgen · model · world · scenario · character · politics · diplomacy · ai · economy · population ·
│   │   │                        religion · magic · military · neutral · events · data · endgame · chronicle · command · observe · save · lib
│   │   └── tests/world.rs
│   ├── ashen_engine/            Spielunabhängige 2D-Engine
│   │   └── src/
│   │       ├── lib.rs · app.rs · gpu.rs · time.rs · input.rs · anim.rs
│   │       ├── render/          mod.rs · camera.rs · particles.rs · profile.rs · shaders.rs · types.rs
│   │       │   └── shaders/     common.wgsl · sprite.wgsl · light.wgsl · composite.wgsl · bloom.wgsl · final.wgsl
│   │       ├── bake/            3D→Pixel-Bake (mesh.rs · skeleton.rs · bake.wgsl)
│   │       ├── assets/          mod.rs (Pfade, Overrides) · pixel.rs (Canvas) · atlas.rs
│   │       ├── audio/           mod.rs · manifest.rs · scanner.rs · backend.rs
│   │       └── ui/              mod.rs (egui-Integration) · theme.rs
│   ├── ashen_game/              Das Spiel (Binary ashen_crowns)
│   │   └── src/
│   │       ├── main.rs          Einstieg, Kommandozeile
│   │       ├── game.rs          Szenenverwaltung (Menu, Mission, Editor, WorldMap), AutoRun
│   │       ├── content.rs       Daten, Mods, Missionen laden
│   │       ├── settings.rs      Einstellungen, Nutzerverzeichnisse
│   │       ├── menu.rs · ui_common.rs · ui_art.rs
│   │       ├── hud.rs · hud_city.rs · hud_hero.rs     Ingame-Oberfläche, Wirtschaft/Stadt, Heldenfenster
│   │       ├── mission.rs       Missionsszene (Takt, Kamera, Eingabe, Speichern)
│   │       ├── campaign_ui.rs   Menüseiten der Echtzeit-Kampagne
│   │       ├── play_modes.rs    Szenarien, Herausforderungen, Gefecht-Einrichtung
│   │       ├── showcase.rs      Vorführstadt und Stresslast für Screenshots/Benchmarks
│   │       ├── view.rs          Weltdarstellung und Atmosphäre
│   │       ├── audio_bridge.rs  SimEvent → Audio
│   │       ├── editor/          Karteneditor: mod · doc · panels · triggers · validate
│   │       ├── worldmap/        Große Kampagne: mod · mapview · panels · windows · widgets · battle · battle_ui · store
│   │       ├── art/             Prozedurale 2D-Pixel-Art: mod · palette · terrain · buildings · props · fx · icons …
│   │       └── art3d/           3D-Modelle für den Bake: rig · units3d · military · structures · city
│   └── ashen_tools/             CLI-Werkzeuge
│       └── src/                 main.rs · audio_list · forge · campaign_forge · campaign_missions(_b/_c) · rts_balance · strategy_stats
│
├── assets/
│   ├── data/                    Spielinhalte (RON)
│   │   ├── units.ron · buildings.ron · abilities.ron · rules.ron · goods.ron · items.ron · talents.ron · upgrades.ron
│   │   ├── missions/            m01_last_settlement.ron
│   │   ├── campaign/            campaign.ron und 17 Kampagnenmissionen
│   │   ├── scenarios/           4 historische Szenarien
│   │   ├── challenges/          8 Herausforderungen
│   │   └── world/events/        Ereignisse der Strategieebene (6 Dateien)
│   ├── fonts/                   Cinzel, Cinzel Decorative, Alegreya (OFL)
│   ├── audio/
│   │   ├── manifest.ron         Audio-Manifest (Events, Musik, Ambience)
│   │   ├── audio_report.md      Letzter Scanner-Bericht (wird von audio-scan überschrieben)
│   │   ├── music/ · ambience/ · sfx/ · voices/     (Audiodateien stehen noch aus)
│   └── sprites/                 Optionale PNG-Overrides (<name>.png, <name>_n.png) – Ordner entsteht bei Bedarf
│
├── mods/
│   └── example_mod/             Beispiel-Mod (mod.ron, data/units.ron, data/buildings.ron)
│
├── docs/
│   ├── canon.md                 Verbindliche Kanon-Referenz (Namen, IDs, Dateinamenskonvention)
│   ├── design/                  game_design_document · campaign · rts_economy · rts_military · editor_and_scenarios · grand_campaign_ui
│   ├── lore/                    world_bible.md · glossary.md
│   ├── audio/                   PRODUCTION_CHECKLIST.md · audio_integration.md · elevenlabs_*.md · suno_master_production.md · Skripte gen_production_checklist.py, verify_spoken_text.py
│   ├── art/                     graphics_and_shaders · 3d_pixel_pipeline · asset_production_plan
│   └── tech/                    architecture · modules · data_model · project_structure · roadmap · modding · strategy_layer · performance · development_log
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
| Tests einer Crate | `cargo test -p ashen_sim` (Simulation inkl. `tests/`), `cargo test -p ashen_strategy`, `cargo test -p ashen_engine`, `cargo test -p ashen_core`, `cargo test -p ashen_game` |
| Spieldaten prüfen | `cargo run -p ashen_tools -- validate-data` |
| Mod prüfen | `cargo run -p ashen_tools -- validate-data --mod mods/example_mod` |
| Audio prüfen | `cargo run -p ashen_tools -- audio-scan` |
| Benchmark | `cargo run --release -- --bench stress:200 --no-audio`, `cargo run --release -p ashen_sim --example sim_bench` (siehe [`performance.md`](performance.md)) |
| Balancemessung | `cargo run --release -p ashen_tools -- rts-balance`, `... -- strategy-stats` |

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
| `--bench <szene>` | Leistungsmessung ohne Fenster (`mission`, `sandbox`, `skirmish`, `battle`, `siege`, `stress[:n]`; `--frames n`, Standard 300), siehe [`performance.md`](performance.md) |

### 2.2 Screenshot-Optionen (Headless-Rendering)

Mit `--screenshot <datei.png>` rendert das Spiel **ohne Fenster** (`run_headless`) und beendet sich danach. Alle
weiteren Optionen steuern Szene und Bildinhalt. Ohne `--scenario`/`--challenge` starten Szenen Mission Nr. 1 (die erste geladene Mission).

| Option | Wirkung | Standard |
|---|---|---|
| `--screenshot <datei.png>` | Ohne Fenster rendern und Bild speichern | – |
| `--size <BxH>` | Bildgröße, z. B. `1920x1080` | `1600x900` |
| `--frames <n>` | Gerenderte Frames vor dem Screenshot (feste Zeit 1/60 s je Frame) | `90` |
| `--scene <name>` | `menu`, `mission` (= Kampagne), `skirmish`, `sandbox`, `editor`, `scenarios`, `challenges`, `skirmish-setup`, `campaign`, `campaign-slots`, `campaign-briefing`, `campaign-ending`, `worldmap` (Große Kampagne), `battle` und `siege` (aus einer Weltkarten-Schlacht erzeugte Echtzeitschlacht); ohne Angabe Hauptmenü | Menü |
| `--sim-seconds <s>` | Simulation vor dem Screenshot vorlaufen lassen (Spielsekunden) | 0 |
| `--time <0..1>` | Tageszeit setzen (0,25 Morgen, 0,5 Mittag, 0,76 Dämmerung, 0,9 Nacht) | Missionsstart (0,30) |
| `--weather <clear\|fog\|rain\|storm\|ash\|overcast>` | Wetter erzwingen (volle Intensität; verhindert Wetterwechsel) | zufällig |
| `--battle` | Gegnerwelle (7 Hohlknechte, 1 Grabkoloss) vor die Basis setzen | aus |
| `--select-army` | Alle Nicht-Arbeiter auswählen (zeigt HUD-Auswahl) | aus |
| `--camera <x,y>` | Kamera auf Kachel zentrieren | Bergfried |
| `--zoom <zeilen>` | Sichtbare Bildzeilen (220–480; durch das Grafikprofil nach unten begrenzt) | 330 |
| `--no-ui` | Oberfläche ausblenden | aus |
| `--ui <liste>` | Fenster öffnen: `storage`, `city`, `build[:rubrik]`, `select:<gebäude-id>`, `hero`; Weltkarte: `province`, `realm`, `family`, `court`, `diplomacy`, `military`, `economy`, `religion`, `magic`, `chronicle`, `victory` | – |
| `--demo <army\|siege\|hero\|reveal>` | Militär-Vorführung (Armee, Belagerungsschlacht, Heldenfenster, Kriegsnebel aus) | – |
| `--demo-city` | Sandbox mit Vorführstadt (Produktion, Wohnhäuser, Markt, Mauern) | aus |
| `--difficulty <easy\|normal\|hard\|nightmare>` | Schwierigkeit (im Gefecht: Gegner-KI) | normal |
| `--scenario <id>` / `--challenge <id>` | Szenario bzw. Herausforderung statt Mission 1 starten; `--outcome <victory\|defeat>` erzwingt das Ergebnis (Wertung, Bestenliste) | – |
| `--map-mode <modus>`, `--map-zoom <z>`, `--days <n>`, `--world-select <provinz>`, `--focus <provinz\|x,y>`, `--world-seed <n>` | Weltkarte: Kartenmodus (`realms`, `terrain`, `religion`, `hollow`, `economy`, `unrest`, `diplomacy`), Zoom (0,55–4,5), Tage vorlaufen, Provinz wählen, Kamera, Seed (Standard 897) | – |
| `--event-popup`, `--battle-dialog`, `--world-war` | Weltkarte: Ereignis-Popup, Schlachtdialog, Heere beider Seiten in der Hauptstadt | aus |
| `--editor-tool`, `--editor-dialog`, `--editor-tab`, `--editor-demo` | Editor: Werkzeug, Dialog (`data`, `new`, `load`), Reiter, Beispielobjekte setzen | – |
| `--select <n>`, `--campaign-mission <id>`, `--campaign-done <n>`, `--flags <a,b,…>` | Szenarien-/Herausforderungsliste vorwählen; Kampagne: Mission fürs Briefing, erste n Missionen als abgeschlossen anzeigen, Entscheidungs-Flags setzen | – |

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

# Galerie-Bilder der README
cargo run --release -- --screenshot screenshots/vertical_slice_worldmap.png --size 1600x900 --scene worldmap --map-mode realms
cargo run --release -- --screenshot screenshots/vertical_slice_campaign.png --size 1600x900 --scene campaign
cargo run --release -- --screenshot screenshots/vertical_slice_siege.png --size 1600x900 --scene siege --sim-seconds 20
cargo run --release -- --screenshot screenshots/vertical_slice_city.png --size 1600x900 --scene sandbox --demo-city
cargo run --release -- --screenshot screenshots/vertical_slice_hero.png --size 1600x900 --scene mission --demo hero --ui hero --sim-seconds 2
cargo run --release -- --screenshot screenshots/vertical_slice_editor.png --size 1600x900 --scene editor --editor-demo
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
| | `--import <dir>` – Lieferordner (beliebig strukturiert) anhand der Dateinamen in `assets/audio` einsortieren (kopiert, überschreibt nie) | |
| | `--apply-renames` – ungültige Dateinamen umbenennen (nur wenn das Ziel nicht existiert) | |
| | `--convert` – m4a/aac/aiff/aif/opus/webm/wma/caf per `ffmpeg` nach `.ogg` (Vorbis q6, 48 kHz) konvertieren | |
| | `--report <datei.md>` – Berichtsdatei (Standard `assets/audio/audio_report.md`) | |
| `audio-list` | `--assets <dir>`, `--format md\|csv` (Standard md), `--out <datei>` | Listet alle vom Manifest erwarteten Audiodateien mit exakten Namen auf (Grundlage der [Produktionsliste](../audio/PRODUCTION_CHECKLIST.md)) |
| `validate-data` | `--assets <dir>` | Lädt Basisdaten, wendet Mods an, lädt Missionen, prüft Konsistenz (u. a. Kürzel-Konflikte) |
| | `--mod <dir>` – Mod-Verzeichnis (das `data/` enthält), mehrfach möglich | |
| `forge-scenarios` | `--assets <dir>` | Erzeugt die mitgelieferten historischen Szenarien neu (`assets/data/scenarios/`) |
| `forge-campaign` | `--assets <dir>` | Erzeugt die Echtzeit-Kampagne neu (`assets/data/campaign/`, Audio-Manifest, Sprechskript) |
| `rts-balance` | `--seeds <n>`, `--minutes <n>` (Standard 12 Seeds, 30 Minuten) | Gefecht-KI gegen skriptgesteuerten Testspieler und Konter-Matrix, siehe [`../design/rts_military.md`](../design/rts_military.md) §10.5 |
| `strategy-stats` | `--seeds <n>`, `--years <n>`, `--first-seed <n>` (Standard 12 Seeds, 200 Jahre, ab Seed 1) | KI-Beobachterläufe der Strategieebene: Siegquote und -zeitpunkt, siehe [`strategy_layer.md`](strategy_layer.md) §19 |

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

Maßgeblich ist `ashen_game::mission::MissionScene::controls`; die vollständige Tabelle steht in der [`README.md`](../../README.md).

| Eingabe | Wirkung |
|---|---|
| Pfeiltasten, Bildschirmrand (abschaltbar), mittlere Maustaste ziehen | Kamera bewegen |
| Mausrad | Zoom (220–480 Bildzeilen) |
| Linke Maustaste / Ziehrahmen | Auswählen (Shift erweitert, Doppelklick wählt gleiche Einheiten im Bild) |
| Rechte Maustaste | Kontextbefehl: Bewegen, Angreifen, Sammeln, Reparieren, Sammelpunkt |
| `A` / `S` / `H` / `P` | Angriffsmarsch / Stopp / Position halten (bei nur Helden gewählt: Heldenfenster) / Patrouille |
| `F` | Formation wechseln |
| `B` (mit Arbeitern) | Baumenü mit Rubriken; Mauern: Maustaste halten, ziehen, loslassen |
| `L` / `C` | Lagerfenster / Stadtübersicht |
| `Q` `W` `E` `R` `T` `Y` | Fähigkeiten (Held, Priester, Magier, Schildträger …) |
| Kürzel der Einheit bei ausgewähltem Gebäude | Ausbilden (siehe Tooltip), `R`: Sammelpunkt |
| `Strg`/`Cmd` + `0`–`9` | Gruppe speichern; Ziffer allein: Gruppe wählen (zweimal: Kamera darauf) |
| `H` (ohne Auswahl) oder `Pos1` | Helden wählen (H öffnet zusätzlich das Heldenfenster) und Kamera darauf |
| `Leertaste` | Kamera auf erste ausgewählte Einheit |
| `+`/`-`, `Pause` oder `^` | Spielgeschwindigkeit (0–3), Pause |
| `F5` / `F9` | Schnellspeichern / Schnellladen (`schnellspeicher.acsave`) |
| `Esc` / `F10` | Esc: Befehlsmodus abbrechen, Heldenfenster schließen oder Pausenmenü; F10: Pausenmenü |
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
| `saves/<slot>.acstrat` | Spielstände der Großen Kampagne (Strategieebene) |
| `saves/battle_return/rueckkehr_*.acstrat` | Rückkehrpunkte laufender Weltkarten-Schlachten (nicht in der Ladeliste) |
| `scenarios/<id>.ron` | Im Karteneditor gespeicherte eigene Szenarien |
| `campaign/` | Profile der Echtzeit-Kampagne (ein Profil je Datei) |
| `leaderboard.ron` | Lokale Bestenliste der Herausforderungen |
| `saves/autosave_0.acsave` … `autosave_2.acsave` | Rotierende Autosaves |
| `saves/schnellspeicher.acsave` | Schnellspeicher (`F5`/`F9`) |
| `mods/<ordner>/` | Mods des Nutzers (zusätzlich zum Ordner `mods/` neben `assets/`) |

Standardwerte der Einstellungen: Profil Ausgewogen, Fenstermodus, VSync an, Lautstärken (Gesamt 0,85 · Musik 0,7 ·
Effekte 0,85 · Stimmen 1,0 · Umgebung 0,6 · Oberfläche 0,7), Bildlauf am Rand an, Tempo 1,0, Schwierigkeit Normal,
kein dauerhafter Heldentod, FPS-Anzeige aus, keine Mods, Autosave alle 5 Minuten.
