# Entwicklungsprotokoll (kompakt)

Ziel: alle Roadmap-Phasen außer Sounddesign umsetzen; am Ende vollständige ElevenLabs- und Suno-Prompts liefern,
danach die gelieferten Audiodateien integrieren.

## Stand vor dem Lauf (Commit 52bfa69)
Vertical Slice: eigene Engine, deterministische RTS-Simulation, Mission 1, 3D→Pixel-Bake, neue UI.

## Verlauf des Fertigstellungslaufs

Reihenfolge der Commits (Kurzform; Merge-Commits ausgelassen):

| Schritt | Commit | Inhalt |
|---|---|---|
| Strategieebene | `7817804` | Crate `ashen_strategy`: Welt, Reiche, Dynastien, Politik, Diplomatie, Wirtschaft, Bevölkerung, Religion, Magie/Hohlung, Ereignisse, Endgame, Chronik |
| Echtzeit-Wirtschaft | `8cdcb86` | Waren, Produktionsketten, Wohnstufen, Markt, Mauern und Tore, Belagerung |
| Editor, Szenarien, Herausforderungen | `3bb19b4` | Editor-Szene (`crates/ashen_game/src/editor/`), Szenarioformat im Missionsformat (`ashen_sim::scenario`, `genmap`), Herausforderungen (`ashen_sim::challenge`), 4 historische Szenarien (`ashen_tools forge-scenarios`), Gefecht-Einrichtung |
| Echtzeit-Militär | `d0d2de7` | neue Einheiten inkl. Belagerung und Magie, Helden mit Talenten und Ausrüstung, Formationen, aufbauende Gegner-KI |
| Weltkarten-Integration | `3e1b8b4` | Szene `worldmap` mit sieben Kartenmodi und 11 Fenstern, Schlachtübergang (`BattleRequest` → Echtzeitmission → `BattleResult`), Rückkehrpunkte, Spielstände `.acstrat` |
| Echtzeit-Kampagne | `8f356b3` | „Das Zeitalter der Gefallenen“: 7 Zeitalter, 18 Missionen, Entscheidungen, Kampagnenmenü |
| Weltkarten-Polish | `59320b4` | Beschriftungen mit Kollisionsvermeidung, gebündelte Ereignismarker, Kartenbild (Grenzen, Schaumsaum, Kompassrose, Legende), `MissionDef::free_economy` für Schlachten ohne Wirtschaftsstrafen, Schlachtleiste im HUD |
| Audio-Produktionsliste | `66be55a` | `audio-list`, Lieferordner-Import (`audio-scan --import`), Master-Produktionsliste |
| Deutsch-Umstellung Audio | `f8d5a54`, `f094b50` | alle Stimmen (Sprechtexte, Voice-Design, Aussprache) auf Deutsch; Hinweis zu mehrteiligen Zeilen |
| Fehler, Balancing | `fb5bc81` | Kampagnen-Spielstände (Platz und Basismission auch in Autosaves), Hotkeys A–Z mit Konflikterkennung in `validate-data`, adaptives Hauptmenü, taktische Schlacht-KI, `rts-balance`, `strategy-stats` |
| Grafik, Performance | `d0176fa` | Wasser-Shader, Gelände-Mikrodetails, Gewand-/Fledermaus-Modelle, Weltkarten-Marker, `--bench`, `sim_bench`, `GroundRects`-Cache |
| Abschlussdokumentation | dieser Lauf | README, Roadmap, Architektur-/Modul-/Strukturdokumente, Galerie-Screenshots |

## Kernergebnisse

### Strategieebene und Balance ([`strategy_layer.md`](strategy_layer.md) §19)

Messung mit `strategy-stats --seeds 12 --years 200` (reine KI-Läufe, Schwierigkeit Lord):

| Stand | Läufe mit KI-Sieg | Früheste Entscheidung | Siege nach Art |
|---|---|---|---|
| Vorher | 9 von 12 (75 %) | Jahr 21 | Krone 7, Handelsherrschaft 2 |
| Nachher | 2 von 12 (17 %) | Jahr 80 | Krone 1, Glaubenssieg 1 |

Stellschrauben (`endgame.rs`): Krone 70 % der Provinzen plus Hochkönigstitel, 144 Monate gehalten; Handel 60 % Anteil, 250 000 Gold, 180 Monate; KI-Siege frühestens nach 80 Jahren. Laufzeit: 200 Spieljahre in ca. 2,3–3,2 s (Release).

### Echtzeit-Militär ([`rts_military.md`](../design/rts_military.md) §10.5, §10.6)

Messung mit `rts-balance --seeds 12 --minutes 30` (skriptgesteuerter Testspieler gegen die Gegner-KI, Punktquote = (Siege + ½ · Zeitlimit) / Läufe):

| Stufe | Vorher (6 Seeds) | Nachher (12 Seeds) |
|---|---|---|
| Leicht | 67 % | 96 % |
| Normal | 0 % | 46 % |
| Schwer | 17 % | 8 % |
| Albtraum | 0 % | 0 % |

Angepasst wurden nur Leicht und Normal in `rules.ron → ai_levels`. Konter-Matrix bei gleichem Wert: keine extremen Ausreißer (Differenz der Restanteile < 85 %), keine Wertänderungen an Einheiten. Taktische Führung in Weltkarten-Schlachten (`systems/tactics.rs`): Fokusfeuer, Fernkämpfer hinter dem Nahkampf, Rückzug, Belagerungsziele, Turmbesatzung; die geführte Seite gewinnt mindestens 5 von 6 Duellen gleicher Heere gegen reine Angriffsbewegung (Test `tests/tactics.rs`).

### Grafik und Performance ([`performance.md`](performance.md))

Alle Werte von Linux (4 CPU-Kerne, Software-Vulkan); GPU-Zeiten sind nicht repräsentativ.

| Messung | Wert |
|---|---|
| Frame-CPU `stress:200` (400 Einheiten) | 1,40 → 1,05 ms im Mittel (view 0,72 → 0,37 ms) |
| Frame-CPU `mission` / `siege` | 0,80 ms / 0,92 ms im Mittel nach der Optimierung |
| 800 Einheiten, maximaler Zoom | 3,9 ms im Mittel (p95 9,8 ms) |
| Simulationstick, 400 Einheiten | 0,46 ms im Mittel (p99 1,52 ms), Budget 50 ms |
| Simulationstick, 800 Einheiten | 2,31 ms im Mittel; Pfadverarbeitung 64 % |
| Atlas | 4 492 Sprites, 64 MiB |
| Start bis Menü | ≈ 5,0–5,3 s (3D-Bake ≈ 4,8 s) |

Behoben: String-Bau und Hash-Suche je Kachel und Frame (`GroundRects`-Cache, `view` −49 % bis −62 %).

### Audio

`audio-list` erzeugt aus dem Manifest die exakte Liste der erwarteten Dateien; die [Produktionsliste](../audio/PRODUCTION_CHECKLIST.md) umfasst 1087 Dateien (219 Soundeffekte, 805 Stimmen, 6 Ambience-Loops, 57 Musikstücke), alle Stimmen auf Deutsch. `audio-scan --import` sortiert einen beliebig strukturierten Lieferordner anhand der Dateinamen ein.

## Nächste Schritte

1. **Audio-Integration nach Lieferung:** Dateien mit `ashen_tools audio-scan --import <lieferordner>` einsortieren, Bericht prüfen (`assets/audio/audio_report.md`), Lautstärken und Musikübergänge im Spiel abhören ([`audio_integration.md`](../audio/audio_integration.md)).
2. **Hardware-Messung:** `--bench` auf Apple A18 Pro (Metal): GPU-Zeiten je Pass, Speicher, Startzeit ([`performance.md`](performance.md) §6).
3. **Balance mit Menschen:** lange Testsitzungen, danach Werte in `rules.ron` bzw. `endgame.rs` nachziehen.
4. **Weitere Punkte** (siehe [`roadmap.md`](roadmap.md), „Offene Punkte“): restliche 11 Kampagnenmissionen, Mehrspieler und Wiederholungen, Mod-Manager, glTF-Import, Bake-Cache, Tutorial, Barrierefreiheit, Paketierung.
