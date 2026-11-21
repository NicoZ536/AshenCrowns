# ASHEN CROWNS – Modulübersicht

Jedes Modul jeder Crate mit Zweck, wichtigsten Typen/Funktionen und Abhängigkeiten. Pfade sind relativ zum
Repository-Wurzelverzeichnis. Überblick und Abhängigkeitsdiagramm: [`architecture.md`](architecture.md). Datenstrukturen
im Detail: [`data_model.md`](data_model.md).

„Abhängigkeiten“ nennt jeweils die wichtigsten internen Module/Crates, die das Modul verwendet.

---

## 1. `ashen_core` (`crates/ashen_core`)

Gemeinsame Grundbausteine ohne Grafik- oder Plattformabhängigkeit. Externe Abhängigkeiten: `glam`, `serde`.
`lib.rs` re-exportiert `glam::{IVec2, UVec2, Vec2, Vec3, Vec4}`.

| Modul | Zweck | Wichtigste Typen/Funktionen | Abhängigkeiten |
|---|---|---|---|
| `color.rs` | Farben und Farbrampen für prozedurale Pixel-Art und UI | `Rgba(u8,u8,u8,u8)` mit `hex`, `with_alpha`, `lerp`, `scale`, `luma`, `to_f32`; `Ramp` mit `from_hex`, `at(t)`, `dither(t,x,y)` (geordnetes Dithering über `noise::bayer`), `darkest`, `lightest` | `noise`, `serde` |
| `iso.rs` | Isometrische 2:1-Projektion | `TILE_W = 32.0`, `TILE_H = 16.0`, `tile_to_world`, `world_to_tile`, `depth_of` (= `x + y`) | `glam` |
| `noise.rs` | Zustandslose Hash-Rauschfunktionen (deterministisch, ohne RNG-Zustand) | `hash_u32`, `hash2`, `rand2`, `value2`, `fbm2`, `BAYER4`, `bayer` | – |

---

## 2. `ashen_sim` (`crates/ashen_sim`)

Deterministische, render-unabhängige RTS-Simulation. Externe Abhängigkeiten: `ashen_core`, `glam`, `serde`, `ron`,
`serde_json`, `bincode`, `rand`, `rand_chacha`, `uuid`, `crc32fast`, `log`. `lib.rs` re-exportiert `Command`,
`PlayerCommand`, `GameData`, `SimEvent`, `SimWorld`.

### 2.1 Kernmodule

| Modul | Zweck | Wichtigste Typen/Funktionen | Abhängigkeiten |
|---|---|---|---|
| `world.rs` | Gesamter Simulationszustand und fester Schritt | `SimWorld` (`new`, `step`, `push_command`, `spawn_unit`, `place_building`, `spawn_node`, `deal_damage`, `can_place`, `state_hash`, Abfragen wie `hero_of`, `building_with_tag`, `nearest_dropoff`, `sight_of`), `PlayerState`, `PlayerStats`, `Director`, `START_YEAR = 897` | fast alle Module |
| `data.rs` | Datengetriebene Inhalte aus RON | `GameData` (`load_dir`, `apply_overlay`, `validate`, `unit_id`/`building_id`/`ability_id`, `type_table`), `UnitDef`, `BuildingDef`, `AbilityDef`, `Effect`, `TowerAttack`, `Rules`, `Cost`, `DamageType` (inkl. `multiplier`), `ArmorType`, `UnitClass`, `Faction`, `ResourceKind`, `ProjectileKind`, `DataError`, Typ-IDs `UnitTypeId`/`BuildingTypeId`/`AbilityId` | `serde`, `ron` |
| `entity.rs` | Laufzeitentitäten | `Unit`, `Building`, `ResourceNode`, `Projectile`, `Order`, `Activity`, `Buff`, `TrainItem`, `NodeKind`, `EntityRef`, Aliase `UnitId`/`BuildingId`/`NodeId`, Spieler-IDs `PLAYER`/`ENEMY`/`NEUTRAL` | `arena`, `data` |
| `arena.rs` | Generationelle Arena mit Freiliste, stabile IDs, deterministische Iteration | `Id<T>`, `Arena<T>` (`insert`, `remove`, `get`, `get_mut`, `iter`, `iter_mut`, `ids`) | `serde` |
| `command.rs` | Spielerbefehle als reine Daten | `Command` (13 Varianten), `PlayerCommand { player, command }` | `entity`, `data` |
| `events.rs` | Ausgabe der Simulation für Darstellung, Audio, HUD, Chronik | `SimEvent` (30 Varianten) | `entity`, `env` |
| `mission.rs` | Datengetriebene Missionen und Laufzeit-Missionszustand | `MissionDef`, `Placement`, `ObjectiveDef`, `TriggerDef`, `Condition`, `Action`, `WaveRules`, `MatchSettings`, `GameMode`, `Difficulty`, `MissionState`, `ObjectiveState`, `Outcome`; `MissionDef::load(path)` | `data`, `env` |
| `map.rs` | Kachelkarte | `TileMap` (`walkable`, `buildable`, `occupy`, `nearest_walkable`, `line_walkable`, `tile_of`, `center_of`), `Terrain` (10 Arten, `priority`, `move_cost`), `Decor`, `DecorKind` | `glam` |
| `mapgen.rs` | Kartengenerator der Aschmark (handentworfenes Layout plus seed-basierte Variation) | `generate(map_seed, variation_seed)`, `populate_resources`, `MAP_SIZE = 72`, `PLAYER_BASE`, `ENEMY_BASE`, `reserved` | `ashen_core::noise`, `map`, `world` |
| `path.rs` | A\*-Wegfindung (8 Nachbarn, Oktil-Heuristik, keine Eckenschnitte) mit Sichtlinien-Glättung | `Pathfinder::find` (Knotenbudget, Teilpfad bei Budgetende), `smooth` | `map` |
| `spatial.rs` | Gleichförmiges Raster zur Nachbarschaftssuche | `SpatialGrid` (`rebuild`, `query`), `CELL = 2.0` | `entity` |
| `fog.rs` | Kriegsnebel des Spielers | `FogOfWar` (`UNEXPLORED`/`EXPLORED`/`VISIBLE`, `reveal`, `state`, `visible`, `explored`, `revision`, `disabled`) | – |
| `env.rs` | Tageszeit und Wetter | `Environment` (`tick`, `phase`, `is_night`, `sun_elevation`), `Weather` (6 Arten), `DayPhase`, `EnvTransitions` | `rand_chacha` |
| `chronicle.rs` | Chronik bemerkenswerter Ereignisse | `Chronicle` (`record`, `narrative`), `ChronicleEntry`, `ChronicleKind` | `serde` |
| `save.rs` | Robustes Speichersystem (`.acsave`) | `encode`, `decode`, `save_to`, `load_from`, `autosave`, `list`, `read_header`, `write_atomic`, `SaveHeader`, `SavePayload`, `SaveError`, `MAGIC`, `FORMAT_VERSION`, `EXTENSION` | `world`, `data`, `bincode`, `crc32fast`, `uuid` |

### 2.2 Systeme (`systems/`)

Jedes System ist eine freie Funktion über `&mut SimWorld` (und `&GameData`).

| Modul | Zweck | Wichtigste Funktionen |
|---|---|---|
| `systems/mod.rs` | Registriert die Untermodule | – |
| `commands.rs` | Wendet `PlayerCommand`s an, prüft Besitz, Kosten, Wohnraum, Warteschlange (max. 5), Bauplatz | `apply`, `formation_slots` |
| `environment.rs` | Wandelt Umwelt-Übergänge in `SimEvent`s (`NightBegins`, `DayBegins`, `WeatherChanged`, `Lightning`) und Chronikeinträge | `tick` |
| `economy.rs` | Passives Einkommen (Nahrung, Gold), Nahrungsunterhalt, Hungersnot, Ausbildung, Bevölkerung, Rückkehr des Helden | `tick`, `recompute_population`, `exit_point`, `hero_respawn` |
| `ai.rs` | Gegner-KI der Vertical Slice: Wellen-Direktor („Hohler Ruf“), Wellenziele, Boss-/Zaubererverhalten | `tick`, `spawn_wave` |
| `units.rs` | Zustandsautomat je `Order` (Bewegen, Angreifen, Sammeln, Abliefern, Bauen, Halten, Patrouille, Zaubern, Wache), Zielsuche, Flucht, Statuswerte (Regeneration, Moral, Schild, Buffs) | `tick`, `status_tick`, `acquire`, `engage` |
| `movement.rs` | Budgetierte Wegsuche, Pfadverfolgung, Separation und Kollision | `process_paths` (48 Anfragen / 24 000 Knoten pro Tick), `integrate` |
| `combat.rs` | Angriffsauflösung (Ausholzeit, Schaden), Projektile, Türme, Entfernen gefallener Entitäten, Erfahrung | `tick`, `cleanup_dead`, `grant_xp` |
| `abilities.rs` | Führt `AbilityDef`-Effekte aus (`Damage`, `Shield`, `Rally`, `Drain`, `Summon`, `Terror`) mit Stufenbonus 8 % je Stufe | `cast` |
| `fog.rs` | Aktualisiert den Kriegsnebel aus den Sichtweiten des Spielers (Nacht- und Nebelmultiplikatoren) | `update` |
| `script.rs` | Wertet Missions-Auslöser (`Condition` → `Action`) aus, Entdeckung markierter Ziele, eingebaute Sieg-/Niederlageregeln | `tick` |

### 2.3 Tests

`crates/ashen_sim/tests/simulation.rs`: Missionsstart nach Kanon, Sammeln, Bauen und Ausbilden, Determinismus,
Wellen und Langzeitlauf, Spielstand-Roundtrip, Heldenfähigkeit, Mod-Überlagerung. Dazu Modultests in `arena.rs`,
`env.rs`, `path.rs`, `mapgen.rs` (und weiteren).

---

## 3. `ashen_engine` (`crates/ashen_engine`)

Spielunabhängige 2D-Engine. Externe Abhängigkeiten: `ashen_core`, `wgpu`, `winit`, `egui`, `egui-wgpu`, `egui-winit`,
`glam`, `bytemuck`, `image`, `serde`, `ron`, `pollster`, `log`, optional `kira`. `lib.rs` re-exportiert `run`,
`run_headless`, `AppConfig`, `Engine`, `Game`, `WindowRequest`, `egui`, `glam`, `winit::keyboard::KeyCode`.

| Modul | Zweck | Wichtigste Typen/Funktionen | Abhängigkeiten |
|---|---|---|---|
| `app.rs` | Fenster (winit), Hauptschleife, `Game`-Trait, Headless-Rendering | `AppConfig`, `Engine`, `Game`, `WindowRequest` (`Fullscreen`, `Vsync`, `Exit`), `run`, `run_headless`, `run_frame` | `gpu`, `render`, `audio`, `input`, `ui`, `time`, `assets` |
| `gpu.rs` | wgpu-Kontext: Instanz, Adapter (`LowPower`), Gerät, Queue; Fehlerprotokoll | `GpuContext` (`new`, `headless`), `GpuError` | `wgpu` |
| `time.rs` | Zeitmessung und fester Takt | `FrameTimer` (`tick` mit 100-ms-Deckel, `tick_fixed`, `fps`, `avg_ms`, `max_ms`), `FixedStep` (`new(hz)`, `advance`, `alpha`, `max_steps = 8`) | – |
| `input.rs` | Eingabezustand pro Frame | `Input` (`key_down`, `key_pressed`, `key_released`, `mouse_down/pressed/released`, `mouse`, `mouse_delta`, `wheel`, Modifikatoren, `command_mod` = Cmd auf macOS sonst Strg, `ui_wants_pointer`, `ui_wants_keyboard`), `MouseButton` | `winit` |
| `anim.rs` | Animationsclips aus Atlas-Sprites | `AnimationClip` (`frame_at`, `duration`, `finished`), `AnimationSet` (Rückfall auf `idle`) | `render::SpriteRect` |
| `ui/mod.rs` | egui-Integration (Eingabe über winit, Zeichnen über wgpu) | `EguiLayer` (`take_input`, `paint`, `handle_output`, `register_texture`, `update_texture`) | `egui_winit`, `egui_wgpu` |
| `ui/theme.rs` | Dark-Fantasy-Theme | Farben `GOLD`, `EMBER`, `BLOOD`, `PARCHMENT`, `LEATHER`, `IRON`, `HOLLOW_GREEN`, `MANA` u. a.; `apply`, `panel_frame`, `parchment_frame`, `heading` | `egui` |

### 3.1 `render/`

| Modul | Zweck | Wichtigste Typen/Funktionen |
|---|---|---|
| `render/mod.rs` | Deferred-2D-Renderer, `FrameData`, Zielpuffer, Passes | `Renderer` (`new`, `render`, `set_atlas`, `atlas_view`, `set_fog_of_war`, `clear_fog_of_war`, `set_profile`, `hot_reload`, `world_to_screen`), `FrameData` (`ground`, `shadows`, `world`, `heat`, `overlay`, `lights`, `atmosphere`), `RenderStats` |
| `render/types.rs` | GPU-Datentypen | `Material` (7), `SpriteRect`, `SpriteInstance` (80 Byte; Builder `material`, `emissive`, `flip`, `tint`, `alpha`, `shear`, `squash`, `water_depth`, `crop_top`, `solid`), `Light` (48 Byte), `Globals` (Uniform), `Atmosphere` |
| `render/camera.rs` | 2D-Kamera mit Ganzzahl-Skalierung und Subpixel-Scrolling | `Camera2D`, `ViewParams` (`view_proj`, `screen_to_world`, `world_to_screen`, `visible_world`) |
| `render/particles.rs` | CPU-Partikelsystem (Rauch, Glut, Funken, Magie, Staub, Regenspritzer) mit Obergrenze | `Particle`, `ParticleSystem` (`spawn`, `update`, `draw`) |
| `render/profile.rs` | Grafikprofile | `GraphicsProfile` (`Performance`, `Balanced`, `Quality`), `ProfileSettings` |
| `render/shaders.rs` | Eingebettete WGSL-Quellen, Hot-Reload, Kompilierung mit Fehlerabfang | `ShaderManager` (`source`, `poll_changed`), `compile`, `COMMON`, `SOURCES` |
| `render/shaders/common.wgsl` | Gemeinsames `Globals`-Uniform, Hash-/Rauschfunktionen, Bayer-Matrix, Vollbilddreieck | – |
| `render/shaders/sprite.wgsl` | G-Buffer-Pass für Sprites, Wasser, Schatten, Hitze | `vs_main`, `fs_gbuffer`, `fs_shadow`, `fs_heat` |
| `render/shaders/light.wgsl` | Punktlichter additiv in HDR | `vs_main`, `fs_main` |
| `render/shaders/composite.wgsl` | Beleuchtung, Kriegsnebel, Nebel, Wetter | `fs_main` |
| `render/shaders/bloom.wgsl` | Helligkeitsfilter, Downsample (13 Taps), Upsample (Tent) | `fs_prefilter`, `fs_down`, `fs_up` |
| `render/shaders/final.wgsl` | Ganzzahl-Skalierung, Tonemapping, Grading, Vignette, Filmkorn, Gamma | `fs_main` |

### 3.2 `assets/`

| Modul | Zweck | Wichtigste Typen/Funktionen |
|---|---|---|
| `assets/mod.rs` | Pfadauflösung und PNG-Overrides | `find_assets_dir` (`ASHEN_ASSETS`, Bundle, Arbeitsverzeichnis), `load_override(assets, name)` |
| `assets/pixel.rs` | Pixel-Leinwand mit Albedo-, Normalen- und Emissionskanal | `Canvas` (`px`, `pxn`, `glow`, `rect`, `line`, `ellipse_shaded`, `polygon`, `outline`, `pillow_normals`, `grain`, `blit`, `rotated`, `from_png`), `NormalPx`, `normals` (Standardnormalen) |
| `assets/atlas.rs` | Regal-Packer, ein Atlas für alle Sprites | `AtlasBuilder` (`add`, `build`), `Atlas` (`get`, `try_get`, `names`, `save_png`, `size`, `albedo`, `normal`) |

### 3.3 `audio/`

| Modul | Zweck | Wichtigste Typen/Funktionen |
|---|---|---|
| `audio/mod.rs` | Audiosystem: Event-Registry, Musiksteuerung, Ambience, Dialogwarteschlange, Ducking | `AudioSystem` (`new`, `with_backend`, `play`, `play_at`, `play_sting`, `set_music_intensity`, `stop_music`, `set_ambience`, `set_volumes`, `update`, `voice_busy`, `current_track`), `Volumes` |
| `audio/manifest.rs` | Manifestformat (RON) | `AudioManifest` (`load`, `event`, `track`, `patterns`), `MusicTrack`, `SoundEvent`, `AmbienceLayer`, `Intensity` (7), `Category` (5), `Playback` (`Random`, `Sequential`) |
| `audio/scanner.rs` | Dateizuordnung und Prüfung | `scan`, `ScanReport` (`files_for`, `found_count`, `is_clean`, `to_markdown`), `valid_name`, `normalize_name`, `check_magic`, `convert_to_ogg`, `PLAYABLE`, `CONVERTIBLE`, `RenameSuggestion` |
| `audio/backend.rs` | Ausgabe-Abstraktion | Trait `AudioBackend`, `Bus` (5), `NullBackend`, `KiraBackend` (Feature `audio`) |

---

## 4. `ashen_game` (`crates/ashen_game`)

Das Spiel (Binary `ashen_crowns`, `default-run`). Abhängigkeiten: `ashen_core`, `ashen_sim`, `ashen_engine`, `glam`,
`egui`, `serde`, `ron`, `log`, `env_logger`.

| Modul | Zweck | Wichtigste Typen/Funktionen | Abhängigkeiten |
|---|---|---|---|
| `main.rs` | Einstiegspunkt, Kommandozeile (`usage()`), Start im Fenster oder Headless | `main`, `usage` | `game`, `settings`, `ashen_engine` |
| `game.rs` | Verbindet Engine, Inhalte, Grafik und Szenen; `AutoRun` für Screenshots/Demos | `AshenGame` (implementiert `Game`), `Scene` (`Menu`, `Mission`), `AutoRun` | `content`, `art`, `menu`, `mission`, `hud` |
| `content.rs` | Lädt Spielinhalte inklusive Mods und Missionen | `Content` (`load`), `ModInfo`, `discover_mods` | `ashen_sim::data`, `ashen_sim::mission` |
| `settings.rs` | Benutzereinstellungen und plattformgerechte Verzeichnisse | `UserSettings` (`load`, `save`), `data_dir`, `saves_dir` | `ashen_engine::render::GraphicsProfile`, `audio::Volumes` |
| `menu.rs` | Hauptmenü mit lebendigem Hintergrund (Sandbox-Welt in der Dämmerung) | `MenuScene`, `Page` (Main, NewGame, Load, Settings, Mods, Credits), `MenuAction` | `view`, `content`, `ui_common` |
| `mission.rs` | Missionsszene: Simulationstakt, Kamera, Auswahl, Befehle aus Eingabe, Ereignisverarbeitung, Speichern/Autosave | `MissionScene` (`new`, `from_world`, `load`, `save`, `update`, `render`), `CommandMode`, `SceneAction`, `place_tile`, `letter_key` | `view`, `audio_bridge`, `ashen_sim` |
| `hud.rs` | Oberfläche der Mission: Ressourcenleiste, Minimap, Auswahlpanel, Befehlskarte, Ziele, Meldungen, Pausenmenü, Chronik, Ergebnisbildschirm | `draw`, `HudCtx`, `MenuPage` | `mission`, `ui_common`, `art` |
| `ui_common.rs` | Gemeinsame UI-Bausteine | `image_from_rect`, `atlas_image`, `save_list`, `settings_ui` | `art`, `settings` |
| `view.rs` | Weltdarstellung: Simulationszustand → Sprites, Lichter, Partikel, Atmosphäre | `WorldView` (`render`, `on_event`, `update`, `atmosphere`, `add_marker`), `Overlay`, `depth_from_world_y` | `art`, `ashen_engine::render`, `ashen_sim` |
| `audio_bridge.rs` | Simulationsereignisse → Audio-Events, Musikintensität, Ambience | `on_event`, `music_intensity`, `ambience` | `ashen_engine::audio` |

### 4.1 `art/` – prozedurale Pixel-Art

| Modul | Zweck | Inhalt |
|---|---|---|
| `art/mod.rs` | Asset-Pipeline: baut den Atlas, wendet PNG-Overrides an, erzeugt Animationssets | `ArtLibrary` (`build`, `get`, `portrait`, `unit_anims`), `UNIT_ART` (10 Einheiten) |
| `art/palette.rs` | Farbpalette der Art Direction | 29 Rampen (`grass()`, `stone()`, `steel()`, `crimson()` …), Akzentfarben `EMBER`, `EMBER_HOT`, `FIRE_DEEP`, `HOLLOW_GLOW`, `HOLLOW_DEEP`, `WINDOW_WARM`, Konstante `OUTLINE` |
| `art/terrain.rs` | Bodenkacheln 32×16, Übergänge, Brücke, Fundamente | `tile_name`, `edge_name`, `in_diamond`, `paint_foundation`, `VARIANTS = 4` |
| `art3d/` | 3D-Modelle (Rig, Einheiten, Bauwerke, Bäume, Dekoration) und 3D→Pixel-Bake in 5 Richtungen, siehe [`../art/3d_pixel_pipeline.md`](../art/3d_pixel_pipeline.md) | `<unit>_<anim>_<dir>_<n>`, `bld_*`, `tree_*`, `node_*`, `decor_*`, `<unit>_portrait` |
| `art/buildings.rs` | `BuildingMeta` (Effektpunkte) und 2D-Baugerüste | `scaffold_*` |
| `art/props.rs` | 2D-Bodendetails: Grasbüschel, Blumen, Schilf | `decor_tuft_*`, `decor_flowers_*`, `decor_reeds_*` |
| `art/fx.rs` | Weiche Partikel, Schatten, Auswahlringe, Projektile, Fahne, Befehlsmarker | `fx_*`, `proj_*`, `shadow_*`, `ring_*`, `marker_*`, `rally_flag` |
| `art/icons.rs` | 16×16-Symbole für Ressourcen, Befehle und Fähigkeiten | `icon_*` |

---

## 5. `ashen_tools` (`crates/ashen_tools`)

| Modul | Zweck | Befehle |
|---|---|---|
| `main.rs` | Entwicklerwerkzeuge | `audio-scan [--assets <dir>] [--apply-renames] [--convert] [--report <datei.md>]` und `validate-data [--assets <dir>] [--mod <dir> ...]` |

Abhängigkeiten: `ashen_sim` (`GameData`, `MissionDef`, `SimWorld`), `ashen_engine` (`audio::manifest`,
`audio::scanner`, `assets::find_assets_dir`), `log`, `env_logger`. Exit-Codes: `0` Erfolg, `1` Fehler, `2` falsche
Bedienung. Details: [`project_structure.md`](project_structure.md), [`../audio/audio_integration.md`](../audio/audio_integration.md),
[`modding.md`](modding.md).
