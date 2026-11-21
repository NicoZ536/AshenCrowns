# ASHEN CROWNS – Entwicklungsroadmap (15 Phasen)

Stand: **Phase 1 (Technisches Fundament) und der Vertical Slice sind umgesetzt.** Der Vertical Slice ist die
spielbare Mission 1 „Die letzte Siedlung“ (`m01_last_settlement`, Zeitalter I) in den Modi Kampagne, Gefecht,
Überleben und Sandbox.

Diese Roadmap beschreibt für jede Phase Ziel, Status, Aufgaben, Akzeptanzkriterien, Tests, Dokumentation und die
Integration mit vorherigen Phasen. Beim Status steht jeweils konkret, was **im Vertical Slice bereits existiert** und
was **offen** ist. Alles, was als „offen“ oder „geplant“ steht, ist **nicht implementiert**.

Zur Zuordnung der Phasen: Verbindlich festgelegt sind Phase 1 (Technisches Fundament), Phase 15
(Balancing/Polishing/Performance) und die im Spiel (`crates/ashen_game/src/menu.rs`) genannten Verweise:
*Grand Campaign – Weltkarte, Dynastien, Diplomatie = Phasen 3, 8 und 10*; *Karteneditor = Phase 14*. Die übrigen
Phasenzuschnitte (2, 4–7, 9, 11–13) sind aus den Kapiteln des Game Design Documents
([`../design/game_design_document.md`](../design/game_design_document.md)) und der Kampagnenplanung
([`../design/campaign.md`](../design/campaign.md)) abgeleitet; die Kapitelverweise stehen bei den Phasen.

Verwandte Dokumente: [`architecture.md`](architecture.md), [`modules.md`](modules.md), [`data_model.md`](data_model.md).

---

## Übersicht

| Phase | Titel | Status | Im Vertical Slice vorhanden (Kurzform) |
|---|---|---|---|
| 1 | Technisches Fundament | **umgesetzt** | Engine, Renderer, Simulation, Spielstände, Audio-System, Werkzeuge, Mod-Grundlage |
| 2 | Echtzeitwelt (RTS-Kern) | teilweise (Vertical Slice) | Eine Karte, 4 Ressourcen, 3 baubare Gebäude, 5 Einheitentypen plus Held, Gegnerwellen, 4 Modi |
| 3 | Weltkarte (strategische Ebene) | offen | – (Menüeintrag „Grand Campaign“ deaktiviert) |
| 4 | Wirtschaft und Bevölkerung | teilweise (Kern) | 4 Ressourcen, Unterhalt, Hungersnot, Wohnraum |
| 5 | Militär und Kampfsystem | teilweise (Kern) | Schadens-/Rüstungstabelle, Moral, Fernkampf, Kavallerie, Türme |
| 6 | Helden und Rollenspielsystem | teilweise (1 Held) | Isolde Raventhorn, 3 Fähigkeiten, 10 Stufen, Wiederkehr |
| 7 | Magie und die Hohlung | teilweise (Effektsystem) | 6 Fähigkeitseffekte, Mana, visuelle Verderbnis am Hohlschrein |
| 8 | Dynastien und Herrscher | offen | – |
| 9 | Politik, Adelshäuser und Intrigen | offen | – |
| 10 | Diplomatie | offen | – |
| 11 | Religion, Monster, neutrale Fraktionen und Bosse | teilweise (1 Fraktion, 1 Boss) | Die Hohlen, Fahler Vogt |
| 12 | Dynamische Ereignisse und Chroniksystem | teilweise (Chronik-Kern) | Chronik mit 8 Kategorien, Wetter-/Tageszeitereignisse, Missions-Auslöser |
| 13 | Kampagne und Endgame | teilweise (Mission 1) | Mission 1 mit Zielen, Dialogen und Skript |
| 14 | Editor, Modding-Werkzeuge und Mehrspieler | teilweise (Datenmods, CLI) | Daten-Mods, `validate-data`, `audio-scan`, Headless-Screenshots, Shader-Hot-Reload |
| 15 | Balancing, Polishing, Performance | teilweise (Grundlagen) | 3 Grafikprofile, Budgets, Autosave; Plattformen außer macOS ungetestet |

Abhängigkeiten zwischen den Phasen (vereinfacht): 1 → 2 → {4, 5, 6, 7} → 3 → {8, 9, 10} → {11, 12} → 13 → 14 → 15.
Phase 14 (Werkzeuge) und Phase 15 können projektbegleitend vorgezogen werden, wo es Nutzen bringt (z. B.
Profiling-Werkzeuge).

---

## Phase 1 – Technisches Fundament

**Ziel:** Tragfähige, testbare technische Basis: eigene Engine, deterministische Simulation, Datenformate, Werkzeuge,
Dokumentation. Alles, was spätere Phasen als Grundlage brauchen.

**Status: umgesetzt.** Im Vertical Slice vorhanden:

* Workspace mit fünf Crates (`ashen_core`, `ashen_sim`, `ashen_engine`, `ashen_game`, `ashen_tools`).
* Engine: Fenster und Eingabe (`winit`), Hauptschleife, Headless-Rendering, egui-Integration, `wgpu`-Renderer
  (G-Buffer → Licht → Komposition → Bloom → Ausgabe), Atlas, Animationssystem, Partikel, Shader-Hot-Reload, drei Grafikprofile.
* Simulation: fester 20-Hz-Takt, Arena mit generationellen IDs, Befehle als Daten, `ChaCha8Rng`, `state_hash`, A\* mit
  Budget, Kriegsnebel, Umwelt, Missionsskript, Chronik.
* Datengetrieben: `units.ron`, `buildings.ron`, `abilities.ron`, `rules.ron`, Missionen als RON; Mod-Überlagerung
  (`GameData::apply_overlay`).
* Spielstände: Format `.acsave` mit Header, CRC32, Backup, Autosave, Kompatibilitätsprüfung.
* Audio: Manifest, Scanner, Event-Registry, Musiksteuerung (Intensitäten, Hysterese, Crossfade), Ambience,
  Dialogwarteschlange mit Ducking, `kira`-Backend.
* Werkzeuge: `ashen_tools audio-scan`, `ashen_tools validate-data`.
* Dokumentation: `docs/tech`, `docs/art`, `docs/audio`.

**Aufgaben (erledigt):** siehe Liste oben. **Restpunkte (bewusst verschoben):** automatisierte CI-Pipeline, getestete
Windows-/Linux-Builds und Paketierung → Phase 15.

**Akzeptanzkriterien (erfüllt):** `cargo test --workspace` läuft; `cargo run --release` startet Menü und Mission;
Determinismustest bestanden; Speichern/Laden-Roundtrip erhält den Zustand; `validate-data` meldet für Basisdaten und
Beispiel-Mod „alles konsistent“; Headless-Screenshot erzeugt ein PNG.

**Tests:** Simulations-Integrationstests (`crates/ashen_sim/tests/simulation.rs`), Modultests (Arena, A\*, Umwelt,
Kartengenerator, Kamera, FixedStep, Partikel, Atlas, Animation, Audio-Scanner, Audio-System, Canvas, Iso-Projektion).

**Dokumentation:** `docs/tech/*`, `docs/art/*`, `docs/audio/audio_integration.md`, `docs/canon.md`.

**Integration:** Grundlage aller Phasen; keine Vorgängerphase.

---

## Phase 2 – Echtzeitwelt (RTS-Kern)

**Ziel:** Ein vollständiger, vielseitiger Echtzeit-Spielkern auf Karten aller Regionen (GDD Kapitel 4, 19, 21):
mehrere Karten und Biome, Skirmish gegen eine KI, die selbst wirtschaftet und baut.

**Status: teilweise (Vertical Slice).**

*Vorhanden:* Eine handentworfene 72×72-Karte der Aschmark mit seed-basierter Variation (`mapgen`); 10 Terrainarten mit
Übergangskacheln; Brücke/Furt; Befehle `Move`, Angriffsmarsch, `Attack`, `Stop`, `Hold`, `Patrol`, `Gather`, `Build`,
`Repair`, `Train`, `SetRally`, `UseAbility`, `Demolish`; Formationsplätze; Gruppen (`Strg`/`Cmd` + Ziffer); Minimap;
Kriegsnebel mit „erforscht/sichtbar“; vier Spielmodi (`Campaign`, `Skirmish`, `Survival`, `Sandbox`); Wellen-Direktor
mit Punktebudget; Missionsskript (Bedingungen, Aktionen); drei Spielerslots (Spieler, Hohle, neutral).

*Offen:*

* Weitere Karten und Biome (andere Regionen aus `canon.md`), Kartengenerator über die feste Aschmark hinaus
  (Flussverlauf, Straße, Basen und Minenpositionen sind in `mapgen.rs` fest verdrahtet).
* Gegner-KI, die wirtschaftet, baut und angreift (aktuell nur Wellen-Direktor und Wache; keine Wirtschafts-KI).
* Mehr als zwei kämpfende Parteien und konfigurierbare Spieleranzahl (aktuell fest 3 Slots in `SimWorld::new`).
* Höhenstufen, Wegpunkt-Verwaltung, erweiterte Auswahl- und Steuerungskomfort-Funktionen (z. B. Formationsbefehle).
* Tastenbelegung frei konfigurierbar (aktuell fest im Code).

**Aufgaben:**

1. Kartensystem verallgemeinern: Karten als Daten (RON) oder parametrisierbarer Generator pro Region; Basis-, Minen-
   und Flusspositionen aus `MissionDef` statt Konstanten.
2. Spieleranzahl und Fraktionen pro Mission konfigurierbar machen (`PlayerState`-Liste aus der Mission).
3. Skirmish-KI: Wirtschafts-, Bau- und Angriffslogik auf den bestehenden Befehlen (`Command`) aufsetzen, damit KI und
   Spieler dieselbe Schnittstelle nutzen.
4. Steuerung: Warteschlangenbefehle, Formationen, Bewegungsstile, konfigurierbare Tasten.
5. Kartenvarianten in Missionsdateien (z. B. Startpositionen als Platzierungen) und Mod-Unterstützung.

**Akzeptanzkriterien:** Skirmish auf mindestens drei verschiedenen Karten gegen eine KI, die ohne Skript
wirtschaftet und angreift; Spieleranzahl pro Mission frei konfigurierbar; alle Befehle sind über `Command` darstellbar;
keine Konstanten für Karten-/Basispositionen im Code außer Standardwerten.

**Tests:** Determinismustest mit KI (zwei Läufe, gleicher Hash); Kartentests (Erreichbarkeit zwischen Basen,
Ressourcen in Reichweite, Seed-Variation); Pfad-Lasttests mit Gruppenbefehlen (Budgetgrenzen aus
`systems/movement.rs`); Befehlsvalidierung.

**Dokumentation:** `data_model.md` (Kartenformat, Spielerkonfiguration), `architecture.md`, Kartenautoren-Hinweise in `modding.md`.

**Integration mit Phase 1:** Nutzt Simulation, Befehle, Pfadbudget, Kriegsnebel, Renderer und Atlas unverändert;
neue Karten benötigen ggf. zusätzliche Terrain-/Dekor-Sprites über die vorhandene prozedurale Pipeline
(siehe `asset_production_plan.md`).

---

## Phase 3 – Weltkarte (strategische Ebene)

**Ziel:** Die zweite Spielebene der „Grand Campaign“ (GDD Kapitel 4): eine Weltkarte Eldraeths mit Provinzen, Armeen
und Missionsknoten, die RTS-Missionen startet und deren Ergebnis übernimmt.

**Status: offen.** Im Vertical Slice vorhanden sind nur Vorarbeiten: serialisierbarer Zustand und Spielstandformat
(erweiterbar), die Chronik (`Chronicle` mit Jahr/Tag), `MatchSettings` für den Missionsstart. Der Menüeintrag
„Grand Campaign“ ist deaktiviert und verweist auf die Phasen 3, 8 und 10. Eine Weltkarten-Simulation, Provinzen oder
Armeen auf der Weltkarte existieren **nicht**.

**Aufgaben:**

1. Datenmodell: Provinzen (Graph), Besitzverhältnisse, Armeen, Garnisonen, Wirtschaft je Provinz, Missionsknoten.
2. Zeitmodell der Weltkarte (Zugschritte in Spieljahren) und deterministische Weltsimulation in `ashen_sim`
   (eigenes Modul, gleicher Takt-/Befehlsansatz wie die Echtzeitwelt).
3. Neue Szene `Scene::World` in `ashen_game::game`; Weltkarten-Darstellung (Karte, Provinzgrenzen, Marker) mit dem
   vorhandenen Renderer.
4. Übergabe: Weltkarten-Konflikt oder Missionsknoten → `MissionDef`/`MatchSettings` für die Echtzeitwelt; Ergebnis
   (Verluste, Beute, Sieg) zurück in die Weltkarte.
5. Spielstand: `SavePayload` um den Weltkartenzustand erweitern (neue `FORMAT_VERSION`, Migrationspfad aus Version 1).
6. Erste Region: Aschmark (5 Provinzen laut `campaign.md`, Zeitalter I).

**Akzeptanzkriterien:** Von der Weltkarte lässt sich Mission 1 starten und beenden; das Ergebnis ändert den
Weltkartenzustand; Speichern/Laden funktioniert über beide Ebenen; Weltsimulation ist deterministisch.

**Tests:** Determinismus der Weltsimulation; Roundtrip-Spielstand mit Weltkarte; Migrationstest Format 1 → 2;
Übergabetests Weltkarte ↔ Mission.

**Dokumentation:** `data_model.md` (Weltkartenmodell, Spielstand v2), `architecture.md` (zwei Ebenen), GDD-Abgleich.

**Integration mit Phasen 1–2:** Wiederverwendung von `SimWorld::new` für Missionen, Missionsdefinitionen (`MissionDef`),
Chronik, Speichersystem; Echtzeitwelt (Phase 2) muss Karten und Spieler konfigurierbar machen, damit Weltkarten-Konflikte
passende Schlachtfelder erzeugen können.

---

## Phase 4 – Wirtschaft und Bevölkerung

**Ziel:** Die tiefe Wirtschafts- und Bevölkerungssimulation (GDD Kapitel 8 und 9) für Echtzeitwelt und Weltkarte.

**Status: teilweise (Kern im Vertical Slice).**

*Vorhanden:* Vier Ressourcen (Gold, Holz, Stein, Nahrung); Sammeln und Abliefern (Tragekapazität, `gather_time`);
passives Einkommen (`gold_per_sec`, `food_per_sec`); Nahrungsunterhalt je Einheit (`upkeep`); Hungersnot (Ausbildung
stoppt, Moral sinkt); Wohnraum durch Gebäude (`pop_provided`) mit Obergrenze `max_population` (80); Abriss mit
Rückerstattung; erschöpfliche Ressourcenquellen; Statistiken (`PlayerStats`).

*Offen:* Bevölkerungsklassen und Zufriedenheit, Steuern, Handel, Produktionsketten, Gebäudevielfalt und Techtree,
Jahreszeiten und Ernten, Wirtschaft je Weltkarten-Provinz (setzt Phase 3 voraus), Aschglas als Ressource
(`canon.md`, Phase 7).

**Aufgaben:** Weitere Ressourcen und Gebäude als Daten; Bevölkerungsmodell (Klassen, Bedürfnisse); Steuer- und
Handelsmodell; Ausbau von `economy.rs`; HUD-Anzeigen; Balancing-Tabellen.

**Akzeptanzkriterien:** Mehrstufige Produktionskette spielbar; Bevölkerungszufriedenheit beeinflusst Wirtschaft und
Moral; Wirtschaftswerte sind vollständig in RON konfigurierbar und per Mod änderbar.

**Tests:** Wirtschaftssimulation über lange Läufe (Nahrungsbilanz, keine negativen Bestände); Spielstand-Roundtrip;
Determinismus; `validate-data`-Regeln für neue Querverweise.

**Dokumentation:** `data_model.md` (neue Definitionen), Balancing-Tabellen, `modding.md`.

**Integration mit Phasen 1–3:** Erweitert `GameData`/`Rules` und `economy.rs`; HUD-Ressourcenleiste; Provinzwirtschaft
nutzt das Weltkartenmodell aus Phase 3.

---

## Phase 5 – Militär und Kampfsystem

**Ziel:** Vielfältiges Militär (GDD Kapitel 10): mehr Einheitenrollen, Belagerung, Formationen, Veteranentum.

**Status: teilweise (Kern im Vertical Slice).**

*Vorhanden:* Fünf Schadensarten gegen sechs Rüstungsarten (`DamageType::multiplier`); Rüstungsformel;
Klassenboni (`bonus_vs`); Nahkampf mit Ausholzeit (`windup`); Fernkampf mit Projektilen (`Arrow`, `BoneBolt`,
`SoulBolt`, Flugbogen nur zur Darstellung); Türme; Moral mit Flucht; Kavallerie; Wache/Aggro; Stufensystem für
Einheiten (`grant_xp`, Held genutzt); Wellen mit drei Gegnertypen.

*Offen:* Belagerungsgerät und Mauern, Formationen und Haltungen, Befehlshaber, Nachschub, Einheitenfähigkeiten
außerhalb von Helden, größere Einheitenvielfalt (mehr als 5 Spielereinheiten), Seekrieg (falls vorgesehen).

**Aufgaben:** Neue Einheiten, Gebäude und Fähigkeiten als Daten; Belagerungsmechanik (Mauern/Tore im Pfadmodell);
Formationen im Befehlssystem; Moral-/Befehlshaber-Regeln; Kampf-Balancing-Werkzeuge (Simulationsläufe ohne Grafik).

**Akzeptanzkriterien:** Belagerung einer ummauerten Basis ist spielbar; Konter-Beziehungen (Speer gegen Reiterei,
Fernkampf gegen leicht Gepanzerte, Siege gegen Gebäude) sind in Simulationsläufen messbar; Einheitenwerte sind
vollständig datengetrieben.

**Tests:** Kampfsimulation headless (Tabellen-Konter-Tests); Pfadtests mit Mauern/Toren; Determinismus mit großen
Schlachten; Performance-Test (Einheitenzahl gegen Tick-Zeit).

**Dokumentation:** `data_model.md` (Schadenstabellen), Balancing-Tabellen.

**Integration mit Phasen 1–4:** Nutzt `deal_damage`, Pfadbudget (`movement.rs`), Moral- und Wirtschaftsregeln
(Unterhalt, Ausbildung); Weltkarten-Armeen (Phase 3) übergeben ihre Zusammensetzung an Echtzeitschlachten.

---

## Phase 6 – Helden und Rollenspielsystem

**Ziel:** Das Heldensystem der Kampagne (GDD Kapitel 11): mehrere Helden mit Fähigkeitsbäumen, Ausrüstung und
Eigenschaften.

**Status: teilweise (1 Held im Vertical Slice).**

*Vorhanden:* Held Isolde Raventhorn (`hero_isolde`); drei Fähigkeiten (`ember_cleave`, `rallying_banner`,
`ashen_ward`) auf `Q`, `W`, `E`; Mana und Regeneration; 10 Erfahrungsstufen (`Rules::xp_levels`) mit
LP-Zuwachs je Stufe und 8 % Fähigkeitsbonus je Stufe; Wiederkehr im Bergfried nach 40 s mit 60 % LP;
optionaler dauerhafter Heldentod; Heldenaura (Licht); Chronikeinträge bei Stufenaufstieg; Stimmen-Events
im Manifest.

*Offen:* Fähigkeitsbäume und Talentwahl, Ausrüstung/Gegenstände, Charaktereigenschaften, mehrere Helden,
Heldenbindung an Weltkarte und Dynastie (Phase 8), Heldenporträts über die Einheitenfigur hinaus,
zielgerichtete Fähigkeiten (`AbilityTargeting::Point` ist im Datenmodell vorhanden, wird noch von keiner Fähigkeit genutzt).

**Aufgaben:** Talent-/Ausrüstungsmodell als Daten; weitere Helden; Heldenzustand in Spielstand und Weltkarte;
HUD (Charakterbogen); Zielauswahl für gezielte Fähigkeiten.

**Akzeptanzkriterien:** Mindestens zwei Helden mit eigenen Fähigkeitsbäumen; Ausrüstung ändert Werte nachvollziehbar;
Heldenzustand überlebt Missionswechsel und Speichern.

**Tests:** Level-/XP-Tests; Fähigkeitseffekt-Tests (wie `hero_ability_damages_enemies`); Spielstand-Roundtrip mit Held.

**Dokumentation:** `data_model.md` (Heldenmodell), `modding.md` (Helden per Mod).

**Integration mit Phasen 1–5:** Baut auf `AbilityDef`/`Effect`, `grant_xp`, `hero_respawn` auf; benötigt
Kampfmodell (Phase 5); Übergabe zwischen Weltkarte (Phase 3) und Mission.

---

## Phase 7 – Magie und die Hohlung

**Ziel:** Magiesystem und die Korruptionsmechanik „Die Hohlung“ (GDD Kapitel 12, `canon.md`).

**Status: teilweise (Effektsystem im Vertical Slice).**

*Vorhanden:* Fähigkeiten als Kombination elementarer Effekte (`Damage`, `Shield`, `Rally`, `Drain`, `Summon`,
`Terror`); Mana; Magieschaden mit Rüstungstabelle; Beschwörungen (Fahler Vogt); visuelle Verderbnis um den
Hohlschrein (Entsättigung, kühl-grüne Tönung, Kontrast in `WorldView::atmosphere`); grünes Hohllicht an
Gebäuden und Partikeln; Furcht-Moralschaden.

*Offen:* Hohlungs-Mechanik als Spielwert (Schleierdruck, Verderbnis von Einheiten und Land), Magieschulen,
Aschglas, Zauberbücher/Forschung, Flächeneffekte mit Dauer, zielgerichtete Zauber.

**Aufgaben:** Neue Effekttypen (Zonen, Verwandlungen, Dauerzauber); Hohlungs-Zustand in `SimWorld`; Auswirkungen auf
Terrain und Einheiten; visuelle Verderbnis datengetrieben (nicht mehr an die Konstante `ENEMY_BASE` gebunden);
Aschglas-Ressource.

**Akzeptanzkriterien:** Hohlung hat messbare Spielwirkung und visuelle Entsprechung; neue Zauber lassen sich rein per
Daten definieren; Verderbnis ist speicherbar.

**Tests:** Effekt-Unit-Tests; Verderbnisausbreitung deterministisch; Spielstand-Roundtrip; visuelle Referenzbilder per
Headless-Screenshots.

**Dokumentation:** `data_model.md`, `graphics_and_shaders.md` (Verderbnisdarstellung), `canon.md`-Abgleich.

**Integration mit Phasen 1–6:** Erweitert `Effect`/`abilities.rs`, Atmosphäre in `view.rs`, Helden- und Kampfsystem;
Wirtschaft (Aschglas, Phase 4).

---

## Phase 8 – Dynastien und Herrscher

**Ziel:** Dynastien, Herrscher und Erbfolge (GDD Kapitel 5) – die generationsübergreifende Ebene der Kampagne
(Isolde und ihre Erben).

**Status: offen.** Im Vertical Slice existiert nur die Heldin Isolde als Einzeleinheit; es gibt kein Dynastie-,
Herrscher- oder Erbfolgemodell. Der Menüeintrag „Grand Campaign“ verweist auf diese Phase.

**Aufgaben:** Datenmodell für Haus, Herrscher, Familie, Erbfolge, Eigenschaften; Alterung und Generationswechsel auf
der Weltkarte (Phase 3); Einfluss auf Wirtschaft, Militär, Diplomatie; Dynastie-Oberfläche; Chronikeinträge
(Phase 12).

**Akzeptanzkriterien:** Herrscherwechsel über Generationen funktioniert in einem Weltkartenlauf; Eigenschaften
wirken auf Spielwerte; Zustand ist speicherbar.

**Tests:** Generationssimulation über viele Jahre (Determinismus, keine inkonsistenten Linien); Spielstand-Roundtrip.

**Dokumentation:** `data_model.md`, GDD-Abgleich, `campaign.md` (Erben).

**Integration mit Phasen 1–7:** Benötigt Weltkarte (3), Helden (6) und Wirtschaft (4); liefert Grundlage für
Politik (9) und Diplomatie (10).

---

## Phase 9 – Politik, Adelshäuser, Ämter und Intrigen

**Ziel:** Innenpolitik (GDD Kapitel 6): Adelshäuser, Ämter, Loyalitäten, Intrigen.

**Status: offen.** Nichts davon im Vertical Slice.

**Aufgaben:** Modell für Häuser, Ämter, Loyalität und Intrigen; Ereignisse und Entscheidungen; Einfluss auf
Armeen und Wirtschaft; Oberfläche (Hofansicht); KI-Akteure.

**Akzeptanzkriterien:** Mehrere Häuser mit Loyalitätswerten; Intrigen lösen Ereignisse aus; Auswirkungen auf
Weltkarte und Schlachten nachvollziehbar.

**Tests:** Politiksimulation deterministisch; Szenarientests (Putsch, Verrat); Spielstand-Roundtrip.

**Dokumentation:** `data_model.md`, GDD-Abgleich.

**Integration mit Phasen 1–8:** Setzt Dynastien (8) und Weltkarte (3) voraus; nutzt das Ereignissystem
(Phase 12) für Intrigen.

---

## Phase 10 – Diplomatie

**Ziel:** Diplomatie zwischen Fraktionen (GDD Kapitel 7): Verträge, Bündnisse, Handel, Kriegserklärungen.

**Status: offen.** Nichts davon im Vertical Slice (Feindschaft ist fest: `SimWorld::hostile`). Der Menüeintrag
„Grand Campaign“ verweist auf diese Phase.

**Aufgaben:** Beziehungs- und Vertragsmodell; Diplomatie-Oberfläche; KI-Verhalten; Einfluss auf Weltkarte und
Echtzeitwelt (Bündnispartner in Schlachten); Fraktionen aus `canon.md`.

**Akzeptanzkriterien:** Bündnisse und Kriegserklärungen verändern Weltkarte und Schlacht-Feindschaften; KI reagiert
nachvollziehbar; Zustand speicherbar.

**Tests:** Diplomatiesimulation deterministisch; Regressionstests für Vertragslogik; Feindschaftsmatrix statt
fester Spielerrollen.

**Dokumentation:** `data_model.md`, GDD-Abgleich.

**Integration mit Phasen 1–9:** Ersetzt die feste Feindschaftslogik der Echtzeitwelt (`SimWorld::hostile`) durch eine
Beziehungsmatrix (setzt Phase 2: konfigurierbare Spieler voraus); nutzt Politik (9) und Dynastien (8).

---

## Phase 11 – Religion, Monster, neutrale Fraktionen und Bosse

**Ziel:** Religionen, neutrale Fraktionen, Monster und Bosse (GDD Kapitel 13 und 14).

**Status: teilweise (1 Fraktion, 1 Boss im Vertical Slice).**

*Vorhanden:* Fraktion „Die Hohlen“ (`Faction::Hollowed`): `hollow_thrall`, `hollow_archer`, `grave_brute`,
`hollow_shrine`, `bone_spire`; Boss `boss_pale_reeve` mit drei Fähigkeiten (`soul_drain`, `summon_thralls`,
`dread_wail`), Beschwörungsschwellen bei 75 % und 40 % LP, Rücksichtslosigkeit unter 50 % LP; Spieler-ID `NEUTRAL`
(ohne Verhalten); Bosskampfmusik-Intensität `Boss`; Boss-Stimmen-Events im Manifest.

*Offen:* Religionen und ihre Spielwirkung, weitere Fraktionen (Skarnheim, Dornenbund, Saelith-Pakt, Aschenlegion),
neutrale Fraktionen und Monster, weitere Bosse (Hohler König, Aschdrache Vyrmathax, Wurzelmutter, …),
mehrphasige Bosskämpfe.

**Aufgaben:** Fraktionsdefinitionen (Einheiten, Gebäude, Fähigkeiten) als Daten; neutrale Lager und Monsterverhalten;
Bossphasen als Datenmodell (statt fest verdrahtetem Verhalten in `ai.rs`); Religionssystem; Kunst und Audio je Fraktion.

**Akzeptanzkriterien:** Mindestens zwei weitere Fraktionen spielbar bzw. als Gegner; Bosse mit definierten Phasen;
neutrale Lager reagieren auf Spieler.

**Tests:** Fraktionsdaten-Validierung (`validate-data`); Bosskampf-Simulation (Phasenwechsel);
Determinismus.

**Dokumentation:** `data_model.md`, `asset_production_plan.md` (neue Sprites), Audio-Manifest-Erweiterung.

**Integration mit Phasen 1–10:** Nutzt Daten- und Missionssystem; Diplomatie (10) bestimmt Beziehungen; Kunst-
und Audio-Pipeline (`art/`, `audio/`) wird um Fraktionsinhalte erweitert. Hinweis: Wellen-Direktor und Bosslogik
referenzieren derzeit feste IDs (`hollow_thrall`, `hollow_archer`, `grave_brute`) und müssen datengetrieben werden.

---

## Phase 12 – Dynamische Ereignisse und Chroniksystem

**Ziel:** Kausalketten und dynamische Ereignisse (GDD Kapitel 15) sowie das ausgebaute Chroniksystem (Kapitel 17)
mit „Ahnenbuch“.

**Status: teilweise (Chronik-Kern im Vertical Slice).**

*Vorhanden:* `Chronicle` mit acht Kategorien (`Founding`, `Construction`, `Battle`, `Death`, `Hero`, `Omen`,
`Victory`, `Defeat`), Einträge mit Jahr, Tag, Tageszeit; automatische Einträge (Gründung, erste Nacht, Wellen,
Hunger, Heldenstufen, Sieg/Niederlage, Mission-`Chronicle`-Aktionen); Fließtext (`narrative`); Anzeige im Pausenmenü
und Abschlussbildschirm; Missions-Auslöser als lokales Ereignissystem; Wetter- und Tageszeitereignisse; Chronik im
Spielstand.

*Offen:* Kausalketten und Ereigniskatalog auf Weltkartenebene, Ereignisdichte-Einstellungen, Ahnenbuch über
mehrere Partien, Chronik der Welt (nicht nur der laufenden Mission), Ereignisauswirkungen auf Spielwerte.

**Aufgaben:** Ereignis-Datenmodell (Bedingungen, Wirkungen, Ketten) als RON; Weltereignisse auf der Weltkarte;
Chronik-Oberfläche; persistentes Ahnenbuch (eigene Datei im Nutzerverzeichnis); Verknüpfung mit Politik/Diplomatie.

**Akzeptanzkriterien:** Ereignisketten laufen über mehrere Spieljahre und erzeugen konsistente Chroniken; Ahnenbuch
bleibt über Partien erhalten; Ereignisse sind per Mod erweiterbar.

**Tests:** Kettenlogik-Tests (keine Endlosschleifen, deterministische Auswahl); Chronik-Roundtrip; Szenarien.

**Dokumentation:** `data_model.md` (Ereignisformat), `modding.md`.

**Integration mit Phasen 1–11:** Erweitert `mission.rs` (Conditions/Actions) und `chronicle.rs`; nutzt die Systeme
aus Phasen 3 und 8–10 als Ereignisquellen.

---

## Phase 13 – Kampagne und Endgame

**Ziel:** Die Grand Campaign „Das Zeitalter der Gefallenen“ in sieben Zeitaltern (`campaign.md`, GDD Kapitel 16):
Story-Missionen, Entscheidungspunkte, Enden, Endgame.

**Status: teilweise (Mission 1 im Vertical Slice).**

*Vorhanden:* Mission 1 „Die letzte Siedlung“ (Zeitalter I) als vollständige RON-Mission mit 11 Zielen, ihren
Auslösern (Einführung, Gehöft/Kaserne/Armee/Wachturm-Ziele, erste Nacht, Wellenwarnung, Überleben, Schreinfund,
Fahler Vogt, Sieg), Dialogen mit Sprecher und Sprach-Event-IDs, Boss und Gegnerbasis; Missionsstart aus dem Menü
(Kampagne); Mission wirkt im Modus Gefecht/Überleben/Sandbox mit angepassten Auslösern.

*Offen:* Alle weiteren Missionen (laut `campaign.md` bis m29), Weltkartenphasen zwischen den Missionen,
Entscheidungspunkte und Verzweigungen, Enden, Erbenvarianten, Endgame, Zeitalterwechsel, Zwischensequenzen.

**Aufgaben:** Missionen m02 ff. als Daten und Karten; Kampagnenzustand (Entscheidungen, Variablen) im Spielstand;
Zeitalterfortschritt; Abschluss-/Endbildschirme; Zwischensequenzen; Kampagnen-Audio und -Kunst.

**Akzeptanzkriterien:** Eine komplette Spielstrecke eines Zeitalters (Missionen plus Weltkartenphase) ist
durchspielbar; Entscheidungen verändern Folgemissionen; Kampagnenstand ist speicher- und ladbar.

**Tests:** Missionsskript-Tests (jede Mission headless durchspielbar, Auslöser feuern wie erwartet);
Kampagnenzustands-Roundtrip; Pfade durch Entscheidungsbäume.

**Dokumentation:** `campaign.md` (Abgleich), `data_model.md` (Kampagnenzustand), Mission-Autorenleitfaden in `modding.md`.

**Integration mit Phasen 1–12:** Nutzt Missionssystem (`mission.rs`), Weltkarte (3), Helden (6), Politik/Diplomatie
(8–10), Ereignisse (12) und die komplette Audio-/Kunstpipeline.

---

## Phase 14 – Editor, Modding-Werkzeuge und Mehrspieler

**Ziel:** Werkzeuge für Autoren und Community sowie Mehrspieler (GDD Kapitel 19; deterministische Simulation und
Befehle als Daten sind dafür vorbereitet).

**Status: teilweise (Datenmods und CLI-Werkzeuge im Vertical Slice).**

*Vorhanden:* Daten-Mods (`mods/<ordner>/mod.ron`, `data/*.ron`, `data/missions/*.ron`; Überschreiben/Ergänzen per
`id`; Aktivierung im Menü „Modifikationen“, Neuladen ohne Neustart); `ashen_tools validate-data --mod`;
`ashen_tools audio-scan`; Headless-Screenshots; Shader-Hot-Reload; Beispiel-Mod `example_mod`.

*Offen:*

* Karteneditor (Menüeintrag „Karteneditor“ ist deaktiviert, verweist auf diese Phase) und Szenario-/Missionseditor.
* Mod-Manager (Ladereihenfolge-Oberfläche, Abhängigkeiten, Konfliktanzeige; aktuell nur Ein-/Ausschalten, Reihenfolge
  per `load_order`).
* Mehrspieler: Lockstep-Netzcode auf Basis von `PlayerCommand`, Lobby, Desync-Erkennung mit `state_hash`.
* Wiederholungen (Replays): Befehlsaufzeichnung plus Startzustand.
* Mod-Validierung für Missionen aus Mods und für Trigger-Verweise.

**Aufgaben:** Editor-Szenen (Gelände malen, Platzierungen, Auslöser); Kartenformat und Speicherung (setzt Phase 2 voraus);
Mod-Manager; Netzwerkschicht (deterministische Befehlsübertragung); Replay-Rekorder und -Abspieler; Erweiterung von
`validate-data` (Missionen aus Mods, Tag-/Objective-Verweise).

**Akzeptanzkriterien:** Eine im Editor erstellte Karte ist spielbar und teilbar; zwei Clients spielen ein
Gefecht ohne Desync (gleicher `state_hash` über 30 Minuten); ein Replay reproduziert den Spielverlauf exakt;
Mod-Manager erkennt Konflikte.

**Tests:** Netzwerk-Simulation mit zwei Instanzen (Hash-Vergleich je Tick); Replay-Roundtrip; Editor-Speichern/Laden;
`validate-data`-Regressionstests.

**Dokumentation:** `modding.md` (Editor, Mod-Manager), Netzwerk-Architektur in `architecture.md`.

**Integration mit Phasen 1–13:** Nutzt `Command`/`PlayerCommand`, `state_hash`, `MissionDef`, `GameData::apply_overlay`,
Karten- und Spielerkonfiguration aus Phase 2; Cross-Plattform-Determinismus der `f32`-Simulation muss in dieser
Phase gesondert abgesichert werden.

---

## Phase 15 – Balancing, Polishing, Performance

**Ziel:** Spielgefühl, Stabilität und Leistung auf der Zielhardware (Apple A18 Pro, 8 GB) und weiteren Plattformen;
Release-Reife (GDD Kapitel 20–24: UI/UX, Tastenkürzel, Schwierigkeitsgrade, Barrierefreiheit, Balancing-Grundsätze).

**Status: teilweise (Grundlagen im Vertical Slice).**

*Vorhanden:* Drei Grafikprofile (Leistung/Ausgewogen/Qualität) mit Lichter-, Partikel-, Bloom- und Zoom-Budgets;
Low-Res-Rendering mit Ganzzahl-Skalierung; Instancing, ein Atlas, Culling; budgetierte Wegsuche; FPS-/Statistikanzeige;
Autosave mit Rotation; drei Schwierigkeitsgrade (Wellenfaktor 0,65/1,0/1,4); Fehlerbehandlung (siehe
`architecture.md`); Headless-Screenshots für Regressionsvergleiche.

*Offen:* Systematisches Balancing der Werte in `assets/data`; Profiling-Werkzeuge (GPU-Zeitstempel, Tick-Zeit-
Messung je System); Speicher-/Leistungsmessungen auf der Zielhardware; Barrierefreiheit (Farbenblindheit, Schriftgrößen,
Tastenbelegung); Lokalisierung; Tutorial; Windows-/Linux-Builds und CI; macOS-Paketierung (App-Bundle, Signierung);
Halbauflösungs-Lichtpuffer (im Profilkommentar genannt, nicht implementiert); ausgearbeitete Nebelqualitätsstufen
(`fog` ist derzeit in allen Profilen an).

**Aufgaben:** Balancing-Durchläufe mit headless Simulationsläufen; Leistungsbudgets je Szene definieren und messen;
Profiling-Overlay erweitern; Accessibility-Optionen; Paketierung und CI für macOS, Windows, Linux; Fehler-/Absturzberichte;
Politur (Animationen, Effekte, UI-Feinschliff); Abschluss der Asset-Produktion (handgezeichnete Sprites, Audio).

**Akzeptanzkriterien:** Zielwerte der Bildrate auf der Zielhardware in allen drei Profilen erreicht (konkrete Zielwerte
vorab festzulegen und zu messen); keine Abstürze in Langzeitläufen; Spielstände aller Versionen laden oder werden
migriert; Installationspakete für macOS verfügbar; Balancing-Berichte dokumentiert.

**Tests:** Langzeit- und Lasttests (große Einheitenzahlen, viele Lichter/Partikel); Speicherlecks-Prüfungen; visuelle
Regressionstests per Headless-Screenshot; Cross-Plattform-Builds in CI.

**Dokumentation:** Aktualisierte `architecture.md` (Plattformen, Performance), Release-Notizen, Balancing-Berichte.

**Integration mit Phasen 1–14:** Querschnittsphase; arbeitet auf allen Systemen. Grundlage sind die Profile
(`render/profile.rs`), Budgets (`movement.rs`, Partikel, Lichter) und Werkzeuge aus Phase 1.
