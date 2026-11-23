# Große Kampagne – Weltkarte, Fenster und Schlachtübergang

Szene `WorldMap` (`crates/ashen_game/src/worldmap/`). Die Simulation liegt vollständig in `ashen_strategy`
(siehe [`../tech/strategy_layer.md`](../tech/strategy_layer.md)); alle Spielerhandlungen laufen über `StrategyCommand`.

## Zugang

* Hauptmenü → **Große Kampagne** (oder Neues Spiel → „Große Kampagne …“): Reich (Aschmark-Marken mit Isolde, Glutkrone,
  Skarnheim, Dornenbund, Saelith …), Schwierigkeit (Pilger … Aschenfall), Regelsatz (Kampagne / Sandbox), Schlachten
  (selbst entscheiden / immer automatisch), Dauerhafter Tod, Eiserner Mann, Seed.
* Hauptmenü → Spiel laden: Spielstände der Großen Kampagne (`.acstrat`) stehen in einer eigenen Liste mit Plakette
  „GROSSE KAMPAGNE“. Autosave jährlich (3 rotierende Slots), Speichern über das Pausenmenü (Esc).

## Karte

Die Provinzen (Voronoi-Polygone der Simulation, 1000 × 700) werden auf ein 400 × 280-Raster gefüllt und mit Nearest-Filter
als Pixel-Art-Textur gezeigt: Biom-Geländetextur mit Rauschen, Küstenlinien, Meer mit Wellenkämmen und Glitzern, Grenzen
in Reichsfarben (Reichsgrenzen kräftig, Provinzgrenzen dünn). Darüber liegen Siedlungssymbole aus dem gebackenen
Atlas (Hütte → Haus → Bürgerhaus → Bergfried nach Siedlungsstufe, Wachturm/Mauerturm nach Befestigung), Reichsfahnen an
Hauptstädten, Heere als Figur mit Banner und Stärke, Marschpfeile (Seewege gestrichelt), Belagerungs-/Schlachtmarker
(Glut, Funken) und Ereignismarker als 8×8-Pixelsymbole: Schlacht, Belagerung, Seuche, Schleierriss/Hohlschrein, Monster,
Boss, Unruhen, Hungersnot. Nebel wabert über Gebieten mit Hohlung ≥ 25.

Kamera: Ziehen, Pfeiltasten/WASD, Kantenbildlauf, Mausrad oder +/− (Zoom auf den Zeiger).

### Kartenmodi (Taste Tab oder Leiste oben links)

| Modus | Färbung |
|---|---|
| Reiche | Reichsfarbe des Besitzers |
| Gelände | nur Biom |
| Religion | Mehrheitsglaube, Sättigung = Anteil |
| Hohlung | Hohlung 0–100 (grün), Schleier ≥ 4 violett |
| Wohlstand | Steuerkraft (Kopfsteuer × Zufriedenheit + Gebäude) |
| Unruhe | Unruhe/Spannung grün → gelb → rot |
| Diplomatie | relativ zum Spielerreich: eigenes/Vasallen gold, Verbündete blau, Krieg rot, Meinung grün/grau/orange |

Die Legende steht unten links.

## Bedienung

| Eingabe | Wirkung |
|---|---|
| Linksklick Provinz / Heer | wählt, öffnet Provinz- bzw. Heeresfenster |
| Rechtsklick Provinz | bewegt das gewählte eigene Heer über den Provinzgraphen (Vorschau-Pfad beim Überfahren) |
| Leertaste · 1–5 | Pause · Tempo (0,7 / 1,5 / 3 / 6 / 12 Tage pro Sekunde) |
| P R F H T M E G Z C V | Fenster Provinz, Reich, Dynastie, Hof, Diplomatie, Heere, Wirtschaft, Glaube, Magie, Chronik, Siegziele |
| Esc | oberstes Fenster schließen / Pausenmenü |

Die Zeit hält an, solange ein Ereignis-Popup, ein Friedensangebot, eine Schlacht oder das Menü auf eine Entscheidung wartet.

## Fenster

* **Oberleiste**: Datum, Tempo, Staatskasse mit Monatsbilanz, Legitimität, Hohlung (Pfad), Prestige.
* **Provinz**: Siedlung, Schichten mit Zufriedenheit, Religion, Gebäude, Steuerstufe, Gebäude bauen, Befestigung ausbauen.
* **Reich**: Herrscher, Zustand, sieben Ämter (ernennen, entlassen, Aufgabe), Gesetze und Regierung.
* **Dynastie**: Stammbaum (Eltern, Geschwister, Ehepartner, Kinder, Enkel), Eigenschaften, Attribute, Erbe, Erziehung, Ehen arrangieren.
* **Adel & Intrigen**: Häuser (Loyalität, Macht, Rechte, Lehen), Komplotte starten/auslösen.
* **Diplomatie**: Meinungen, Verträge anbieten/kündigen, Geschenke, Tribut, Embargo, Krieg erklären (Kriegsgrund aus `find_cb`, Kriegsziel), Frieden aushandeln mit Kosten in Kriegspunkten.
* **Heere**: Liste, Regimenter, Teilen, Vereinen, Befehlshaber, Sturmangriff, Auflösen, Truppen ausheben, Söldner.
* **Wirtschaft**: Kasse, Steuern, Zölle, Kredite, ertragreichste Provinzen, Markt (Preise, Knappheit).
* **Glaube**, **Magie** (Forschung, Rituale, Drachentribut), **Chronik** (Pergament, Filter nach Art, Bedeutung, Reich, Suche), **Siegziele** (acht Ziele mit Fortschritt).
* **Popups**: Ereignisse (Pergament mit Optionen), Friedensangebote, Schlachtdialog, Sieg/Untergang; Benachrichtigungen rechts oben.

## Schlachtübergang

Mit `BattleMode::PlayerBattlesRequested` hinterlegt die Strategie Schlachten mit Spielerbeteiligung. Der Dialog bietet
**Selbst kommandieren** oder **Automatisch auflösen** (`auto_resolve_pending`).

Selbst kommandieren (`worldmap/battle.rs`):

1. `plan_battle` erzeugt eine `MissionDef` im Sandbox-Modus (keine Standard-Niederlage, keine Wellen) mit Karte
   (`MissionDef::map`, 72 × 72): Gelände aus dem Biom der Provinz (Ebene, Hügel, Wald mit Bäumen, Gebirge, Sumpf, Wüste, Küste/Insel mit Meer, Vulkan,
   Tundra, Ruinen, Hohlland), Jahreszeit (kahler Boden im Herbst/Winter), Wetter und Tageszeit aus der Anfrage, Fluss mit Furten
   bei manchen Feldschlachten, Ascheflächen bei Schleierstärke ≥ 3. Seed aus Provinz, Tag und Schlacht-Id.
2. Belagerung: Der Verteidiger erhält Bergfried, Mauerring (Größe und Türme nach Befestigungsstufe, Tore, Wachtürme, Häuser nach
   Siedlungsstufe); die Breschen der Anfrage entfernen Mauerabschnitte auf der Angreiferseite.
3. Heere: Regimenter werden nach Klasse aufgestellt (Infanterie vorn, Fernkämpfer dahinter, Reiter an den Flanken). Große Heere werden
   skaliert (höchstens 48 Einheiten je Seite, 1 RTS-Einheit = `scale` Soldaten); Lebenspunkte und Moral stammen aus Regiment und Heer.
   Helden: Isolde als `hero_isolde`, weitere als Ritter. Der Spieler führt immer Besitzer 0, der Gegner Besitzer 1; Verteidiger halten ihren Posten, Angreifer des Gegners marschieren mit Angriffsbewegung vor.
4. Sieg/Niederlage entscheidet `battle_outcome` (alle Gegner tot, Bergfried gefallen, nach 30 Minuten gewinnt der Verteidiger bzw. die stärkere Seite).
   Schlachtleiste: **Rückzug** (Niederlage) und **Automatisch beenden**.
5. `build_result` zählt überlebende Einheiten je Regiment (Tag `r<Heer>:<Index>`), skaliert zurück, bestimmt Lebenspunkte, Rückzug und Heldenschicksal
   (unversehrt/verwundet/gefangen/gefallen; fehlende Heldeneinheit = gefallen bei dauerhaftem Tod, sonst verwundet) und liefert das `BattleResult`; `apply_battle_result` wendet es an.

### Zuordnungstabelle strategische Einheit → RTS-Einheit

Gibt es die strategische ID in `units.ron`, wird sie direkt verwendet; sonst gilt `UNIT_TABLE` (Test: jede ID aus
`catalog::UNITS` hat einen Eintrag, jedes Ziel existiert). Unbekannte Mod-IDs fallen nach Klasse auf eine Standardeinheit.

| Strategisch | RTS | Strategisch | RTS |
|---|---|---|---|
| worker, swordsman, spearman, archer, crossbowman, halberdier, shieldbearer, scout_rider, horse_archer, knight, priest, battle_mage, battering_ram, catapult, hero_isolde, hero_hal_venn, hero_torvald | gleich | ember_adept | swordsman |
| flame_priest | priest | ember_mage | battle_mage |
| mangonel, trebuchet | catapult | skarn_ironguard | shieldbearer |
| skarn_rune_thrower, veyl_stalker | archer | thorn_warden | spearman |
| saelith_marine | swordsman | saelith_arbalest | crossbowman |
| hollow_thrall, hollow_archer, grave_brute, hollow_digger, ghoul, ash_witch, bat_swarm | gleich | veil_wraith | ghoul |
| hollow_priest | ash_witch | ashen_legionnaire | hollow_thrall |
| cinder_knight, grave_worm, glow_salamander | grave_brute | ash_wolf | bat_swarm |
| moor_light | hollow_archer | boss_pale_reeve | gleich |
| boss_ashen_dragon, boss_root_mother, boss_hollow_king, boss_stone_oath, boss_reef_mistress | boss_pale_reeve | | |

### Speichern während einer Schlacht

Die Weltzeit ruht. Beim Start schreibt das Spiel einen **Rückkehrpunkt** (`saves/battle_return/rueckkehr_*.acstrat`) mit der noch offenen
Schlachtanfrage. Jeder Echtzeit-Spielstand der Schlacht (manuell oder Autosave) trägt im `client_state` einen `BattleLink`
(Rückkehrpunkt, Schlacht-Id, Skalierung). Beim Laden stellt das Spiel den Rückkehrpunkt wieder her; nach der Schlacht kehrt es zur
Weltkarte zurück und legt einen strategischen Autosave an. Der Rückkehrpunkt liegt in einem Unterordner und erscheint nicht in der Ladeliste.

## Audio

Intensitäten `WorldPeace` < `WorldWar` < `WorldHollow` (Musik `worldmap_peace/war/hollow`), UI-Klänge `sfx.ui_event_popup`, `ui_war_declared`,
`ui_peace_made`, `ui_chronicle_page`, `ui_window_open`, `ui_treaty_signed`, `ui_battle_alert`, `ui_realm_fell`, `ui_ruler_changed`, `map_select` und
`sfx.map_army_move`, `map_recruit`. Prompts: `docs/audio/elevenlabs_sound_effects.md` (Abschnitt 4b) und `docs/audio/suno_master_production.md` (4.2b).

## Screenshots (headless)

```
ashen_crowns --scene worldmap --screenshot k.png [--days 60] [--map-mode hollow] [--ui province,diplomacy] [--world-select kaltherd]
                              [--focus x,y] [--map-zoom 2] [--event-popup] [--battle-dialog] [--world-war] [--world-seed 897]
ashen_crowns --scene battle|siege --sim-seconds 25 --screenshot s.png   # Echtzeitschlacht aus einem BattleRequest
```
