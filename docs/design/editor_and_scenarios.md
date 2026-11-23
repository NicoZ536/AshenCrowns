# ASHEN CROWNS – Karteneditor, Szenarien und Herausforderungen

Dieses Dokument beschreibt die Wiederspielwert-Werkzeuge der Vertical Slice (GDD Kapitel 18): den **Karteneditor**,
**Szenarien** (mitgelieferte, historische und eigene), den **Herausforderungsmodus** und die **Gefecht-Einrichtung**.
Alle vier arbeiten auf demselben Datenformat: Ein Szenario ist eine gewöhnliche Mission (`MissionDef`, siehe
[`../tech/data_model.md`](../tech/data_model.md)) mit zusätzlicher, expliziter Karte. Jede im Editor gespeicherte Karte
ist deshalb ohne Umweg spielbar – als Szenario, als Gefechtskarte oder als Basis einer Herausforderung.

---

## 1. Karteneditor

Aufruf: Hauptmenü → **Karteneditor** (oder in der Szenarienliste **Im Editor öffnen**). Die Szene hat eine Kopfleiste,
links die Werkzeugleiste, rechts die Eigenschaften der Auswahl und unten die Statuszeile mit Kachelposition und Gelände.

### 1.1 Kopfleiste

| Schaltfläche | Wirkung |
|---|---|
| **Neu** | Neue Karte: Breite/Höhe (32–128), Seed, Generator-Vorlage (Aschmark, Grasland, Flusstal, Hochland, Aschland, Seenplatte) oder leere Grasfläche. Eine generierte Karte enthält Bergfried und Trupp für die Spielerseite, einen bewachten Hohlschrein für den Gegner, Ziele, Siegauslöser und Wellen – sie ist sofort spielbar. |
| **Laden** | Eigene und mitgelieferte Szenarien sowie die Kampagnenmissionen als Vorlage. Mitgelieferte Szenarien und Missionen werden als Kopie (`…_kopie`) geöffnet; gespeichert wird immer im Benutzerordner. |
| **Speichern** (Strg+S) | Schreibt `<Benutzerordner>/scenarios/<id>.ron` (Pfad siehe 4.). Vorher läuft die Prüfung; bei Fehlern wird trotzdem gespeichert, die Statuszeile warnt. |
| **Zurück / Vor** (Strg+Z / Strg+Y, Strg+Umschalt+Z) | Rückgängig/Wiederholen über Schnappschüsse des gesamten Dokuments (100 Schritte). Ein Pinselstrich oder eine Texteingabe zählt als ein Schritt. |
| **Missionsdaten** | Dialog mit den Reitern Allgemein, Ziele, Auslöser, Regeln, Bereiche, Prüfung (siehe 1.4). |
| **Prüfen** | Öffnet die Prüfung (siehe 1.5). |
| **Testspiel** | Startet die aktuelle Karte als Mission. **Esc → Hauptmenü** im Testspiel kehrt zum Editor zurück; nichts geht verloren. |
| **Beenden** | Zurück ins Hauptmenü; bei Änderungen fragt der Editor nach (Speichern / Verwerfen / Abbrechen). |

Kamera: Pfeiltasten, mittlere Maustaste ziehen, Mausrad zoomt (weiter als im Spiel, bis zur Gesamtübersicht).
**G** blendet ein Raster ein. **Esc** schließt Dialoge, hebt die Auswahl auf, wechselt dann zum Auswahlwerkzeug.
Rechtsklick hebt die Auswahl auf und wählt das Auswahlwerkzeug.

### 1.2 Werkzeuge

| Werkzeug | Bedienung |
|---|---|
| **Auswahl** | Klick wählt Gebäude, Einheitengruppen, Startpunkte, Quellen, Dekoration, Bereiche; Ziehen verschiebt (Gebäude rasten auf Kacheln ein, Einheiten auf halbe Kacheln); **Entf** löscht. |
| **Gelände** | Pinsel mit allen zehn Geländearten (Gras, Dunkelgras, Erde, Straße, Fels, Sand, Asche, Furt, Wasser, Tiefes Wasser), Größe 1–12, Form Rund/Quadrat/Raute. Wasser entfernt Bäume, Quellen und Dekoration darunter; Straßen und Furten entfernen Bäume. |
| **Füllen** | Flutfüllung zusammenhängenden gleichen Geländes. |
| **Wald** | Pflanzt Bäume im Pinselbereich (Dichte einstellbar); mit Umschalt entfernt der Pinsel Bäume. |
| **Deko** | Alle 15 Dekorationsarten (Steine, Felsbrocken, Büsche, Ruinen, Säulen, Gräber, Kohlenbecken …). Blockierende Arten (Felsbrocken, Ruine, Säule, toter Baum) sperren ihre Kachel. |
| **Quellen** | Goldminen (2×2), Steinbrüche (2×2) und einzelne Bäume; der Vorrat ist je Quelle in den Eigenschaften einstellbar (0 = Standardwert aus `rules.ron`). |
| **Gebäude** / **Einheiten** | Setzt fertige Gebäude bzw. Einheitengruppen beider Seiten (Glutkrone / Die Hohlen); die Paletten zeigen die Gebäude und Einheiten der gewählten Seite samt Mod-Inhalten. Gebäude werden gegen Gelände, Quellen, Dekoration und andere Gebäude geprüft (rotes Vorschaugebäude). In den Eigenschaften: Seite, **Tag**, Anzahl, Wächter-Haltung. |
| **Start** | Setzt die Startposition der gewählten Seite (je eine pro Seite). |
| **Bereiche** | Zieht einen benannten rechteckigen Bereich auf. Bereiche sind Ziele für Auslöser („Einheiten im Bereich“) und Orte für „Einheiten erscheinen“. |
| **Radierer** | Entfernt Einheiten, Gebäude, Quellen und Dekoration unter dem Pinsel. |

Höhenstufen kennt das Datenmodell nicht (die Karte ist eine flache Kachelkarte mit Gelände, Blockaden und Dekoration);
der Editor bietet deshalb keine Höhenwerkzeuge an.

**Tags** verbinden Platzierungen mit dem Skript: `TagDead`, `TagDiscovered` und `TagDamaged` prüfen Objekte mit
diesem Tag. Das Tag `player_keep` (Niederlage-Tag) und das Tag der gegnerischen Basis (`enemy_shrine`) sind
Voreinstellungen der Regeln (Reiter Regeln).

### 1.3 Reiter der Missionsdaten

* **Allgemein**: Name, ID (wird zum Dateinamen), Autor, Jahr n.F., Kurzbeschreibung, Einsatzbesprechung.
* **Ziele**: Liste der Ziele (ID, Text, optional). Auslöser fügen Ziele hinzu und schließen sie ab.
* **Auslöser** (Trigger-Editor): Links die Liste, rechts WENN/DANN. Die Bedingung ist ein frei verschachtelbarer Baum
  aus allen Bedingungen von `mission.rs` – *Zeit erreicht, Gebäude fertig, Gebäudeanzahl, Einheitenzahl, Soldatenzahl,
  Ziel erledigt, Nacht/Tag beginnt, Tag-Ziel zerstört/entdeckt/beschädigt, Heldenstufe, Wellen überstanden,
  Ausbaustufe, Bewohnerzahl, Vorrat, Einheiten im Bereich, Gegner höchstens, Nicht, Alle, Eine von*. Darunter stehen
  beliebig viele Aktionen: *Nachricht, Ziel hinzufügen/abschließen, Sieg, Niederlage, Welle auslösen, Ressourcen
  schenken, Wellen ein/aus, Wetter setzen, Chronik-Eintrag, Einheiten erscheinen*. Auswahllisten bieten nur
  vorhandene IDs an (Bereiche, Tags, Ziele, Einheiten, Gebäude, Waren). Je Auslöser lässt sich der Modus einschränken
  (Kampagne, Gefecht, Überleben, Sandbox; leer = alle). Auslöser lassen sich kopieren, sortieren und löschen.
* **Regeln**: Startressourcen und Startwaren, Angriffswellen (aktiv, erste Welle, Abstände Tag/Nacht, Budget),
  Niederlage-Tag, Tag der gegnerischen Basis, Truppenobergrenze, Tageszeit bei Spielbeginn.
* **Bereiche**: Namen, Position, Größe; „Hin“ springt zum Bereich.

### 1.4 Prüfung

Die Prüfung liefert **Fehler** (unspielbar oder unfair), **Hinweise** und **Infos**; jede Meldung mit Ort hat einen
„Hin“-Knopf. Sie erkennt unter anderem:

* keine Startposition der Spielerseite, Startposition auf Wasser oder eingeschlossen,
* **unerreichbare Ziele**: gegnerische Starts, die gegnerische Basis, Tag-Ziele und Bereiche, auf die Auslöser
  verweisen, müssen von der Startposition des Spielers aus über begehbares Gelände erreichbar sein (eigene offene Tore
  zählen; Wasser, Bäume, Mauern und Felsbrocken blockieren),
* **fehlende Ziele**: keine Ziele definiert, kein Auslöser mit Aktion „Sieg“, Ziele, die kein Auslöser hinzufügt oder abschließt,
* ungültige Aufstellung (Gebäude im Wasser, überlappend, unbekannte IDs), Verweise auf unbekannte Bereiche, Tags,
  Einheiten, Gebäude, Waren oder Ziele, doppelte Auslöser-IDs, Niederlage-Tag ohne Gebäude (Partie wäre sofort verloren),
  Wellen ohne gegnerische Basis, fehlende Goldmine oder Bäume in der Nähe des Spielerstarts.

---

## 2. Dateiformat

Szenarien sind RON-Dateien im **Missionsformat** (`assets/data/missions/m01_last_settlement.ron` ist das Vorbild).
Alle zusätzlichen Felder sind optional; Missionen ohne sie bleiben gültig.

| Feld | Bedeutung |
|---|---|
| `map` | Explizite Karte: `width`, `height`, `terrain` (eine Textzeile je Kachelreihe), `decor` (Art, x, y, Variante), `nodes` (Baum, Goldmine, Steinbruch mit Kachel, optional `amount`, `variant`). Fehlt `map`, erzeugt der Generator die Karte aus `map_seed`. |
| `areas` | Benannte Bereiche `(id, name, x, y, w, h)` in Kacheln. |
| `starts` | Startpositionen `(owner, x, y)` (Editor, Prüfung, Kamerastart). |
| `briefing`, `author`, `year` | Einsatzbesprechung, Autor, Jahr n.F. für die Szenarienliste. |
| `unit_cap` | Obergrenze für Streitkräfte (ohne Arbeiter und Heldin); wirkt beim Ausbilden. |
| `start_time` | Tageszeit (0–1) bei Spielbeginn. |

**Kachelcodes** in `terrain`: `G` Gras, `g` Dunkelgras, `d` Erde, `R` Straße, `S` Fels, `s` Sand, `A` Asche,
`F` Furt, `w` Wasser, `W` Tiefes Wasser. Jede Zeile hat genau `width` Zeichen, es gibt `height` Zeilen.

**Neue Bedingungen und Aktionen** (additiv in `mission.rs`):

```ron
UnitsInArea(area: "vael_dorn", count: 12)     // mindestens 12 eigene Einheiten im Bereich
EnemyUnitsAtMost(0)                           // höchstens so viele Gegner leben noch
TagDamaged("player_keep")                     // das getaggte Gebäude hat Schaden genommen
Not(AtTime(300.0))                            // Verneinung
SpawnUnits(unit: "hollow_thrall", owner: 1, area: "flut_west", count: 8, attack: true)
```

`SpawnUnits` erzeugt Einheiten in der Mitte des Bereichs; mit `attack: true` greifen sie die Siedlung des Spielers an.
Ein leeres `defeat_tag` schaltet die Regel „Hauptgebäude verloren = Niederlage“ ab (z. B. für Fluchtszenarien; es
bleibt die Regel „keine Einheiten und Gebäude mehr = Niederlage“).

Wichtig: Spielstände (`.sav`, bincode) enthalten die Mission. Spielstände, die vor Einführung dieser Felder geschrieben
wurden, lassen sich nicht mehr laden.

---

## 3. Szenarien

Hauptmenü → **Szenarien** listet mitgelieferte (`assets/data/scenarios/`), Mod- und eigene Szenarien (Benutzerordner)
mit Jahr, Herkunft, Beschreibung, Einsatzbesprechung und Zielen. Schwierigkeit wählen, **Starten**; **Im Editor öffnen**
bearbeitet das Szenario (mitgelieferte als Kopie).

### 3.1 Mitgelieferte historische Szenarien

Sie folgen dem Kanon ([`../canon.md`](../canon.md), [`../lore/world_bible.md`](../lore/world_bible.md) Kap. 2.10 und 9)
und sind von Hand entworfen. Die Dateien erzeugt `cargo run -p ashen_tools -- forge-scenarios` reproduzierbar
(`crates/ashen_tools/src/forge.rs`); danach sind sie normale Szenarien und im Editor weiter bearbeitbar.

| ID | Szenario | Jahr | Spielprinzip |
|---|---|---|---|
| `sc_fall_of_grauwall` | **Der Fall von Grauwall** | 893 n.F. | Verteidigung der ummauerten Stammfeste von Haus Raventhorn gegen sechs Wellen der Hohlen in einer einzigen Nacht; Sieg beim Morgengrauen, der Bergfried darf nicht fallen. |
| `sc_flight_to_vael_dorn` | **Die Flucht nach Vael Dorn** | 894 n.F. | Geleit: 16 Flüchtlinge und eine kleine Eskorte (Isolde) durch die Aschmark über zwei Furten zu den Klostermauern; Wachposten, Hinterhalte, nachsetzende Wellen. Sieg: 12 Seelen im Bereich; Niederlage: mehr als 8 Flüchtlinge verloren. |
| `sc_siege_of_kaltherd` | **Die Belagerung von Kaltherd** | 896 n.F. | Aufbau und Verteidigung: ein Tag zum Errichten von Palisade, Tor und Türmen, dann zwei Nächte mit Angriffen aus vier Richtungen. |
| `sc_banner_of_grauwall` | **Das Banner von Grauwall** | 898 n.F. | Angriff: Lager ausbauen, Truppen ausbilden, den Hohlschrein von Grauwall zerstören und den Grabkoloss in Edrans Rüstung (Tag `edran`) bezwingen. |

**Balancenachweis:** Der Integrationstest `crates/ashen_sim/tests/scenarios.rs` spielt jedes Szenario mit einem
einfachen Testspieler (bildet Soldaten aus, verteidigt bzw. rückt vor) bis zum Sieg und prüft, dass untätiges Warten
nicht sofort verliert, dass Wellen Wirkung zeigen und dass jedes Szenario einen Siegauslöser, gültige Bereiche und
Tags sowie verbundene Startpositionen hat. Ein Mensch, der klüger spielt als der Testspieler, gewinnt mit weniger
Verlusten.

---

## 4. Herausforderungsmodus

Hauptmenü → **Herausforderungen**. Herausforderungen sind datengetrieben (`assets/data/challenges/*.ron`, auch aus
Mods unter `data/challenges/`). Eine Herausforderung verweist auf eine Mission oder ein Szenario (`base`), setzt Modus
und Schwierigkeit, wendet **Modifikatoren** an und definiert die **Wertung**.

```ron
(
    id: "ch_untouched_keep",
    name: "Unversehrter Bergfried",
    description: "…",
    base: "m01_last_settlement",          // Mission oder Szenario (ID)
    mode: Campaign,                        // Campaign | Skirmish | Survival | Sandbox
    difficulty: Normal,                    // Story | Normal | Hard
    modifiers: [KeepIntact, UnitCap(10)],
    victory: Some(DayStarted(21)),         // optional: zusätzliche Siegbedingung
    goal_text: "…",
    scoring: (victory_bonus: 2000.0, per_kill: 6.0, per_unit_lost: -15.0, par_time: 1800.0,
              time_bonus_per_sec: 0.5, rank_by: Score, counts_when_defeated: false),
)
```

### 4.1 Modifikatoren

| Modifikator | Wirkung |
|---|---|
| `UnitCap(n)` | Höchstens `n` Soldaten (ohne Arbeiter und Heldin); weiteres Ausbilden wird abgelehnt. |
| `WaveStrength(f)` | Faktor auf Wellenbudget (Basis und je Welle). |
| `WaveInterval(f)` | Faktor auf erste Welle und Wellenabstände (kleiner als 1 = häufiger). |
| `StartResources(f)` | Faktor auf Startressourcen und Startwaren. |
| `TimeLimit(s)` | Niederlage nach `s` Sekunden Spielzeit. |
| `KeepIntact` | Niederlage, sobald das Hauptgebäude (Niederlage-Tag) Schaden nimmt. |
| `HeroPermadeath` | Die Heldin kehrt nicht zurück. |
| `IronMan` | Kein Speichern, Schnellspeichern, Autosave und Laden während der Partie. |
| `NoHero` | Start ohne Heldin. |

Modifikatoren werden vor dem Start in die Mission eingearbeitet (Triggerziele, Wellenwerte, `unit_cap`), sodass die
Simulation keine Sonderfälle kennt.

### 4.2 Wertung und Bestenliste

Punkte = Siegprämie (nur bei Sieg) + Feinde × `per_kill` + Verluste × `per_unit_lost` + errichtete Gebäude ×
`per_building` + Bewohner × `per_resident` + Wellen × `per_wave` + Nächte × `per_night` + Zeitbonus
(`par_time` − Spielzeit) × `time_bonus_per_sec` (nie negativ, nur bei Sieg). `rank_by: Time` reiht nach kürzester
Siegzeit (Speedrun). `counts_when_defeated: true` lässt auch Niederlagen in die Liste (Überlebensmodi).

Die Bestenliste liegt lokal in `<Benutzerordner>/leaderboard.ron`, je Herausforderung die besten zehn Einträge
(Name, Punkte, Zeit, Datum). Nach der Partie zeigt ein Fenster am linken Rand Wertung, Platz, „Neuer Bestwert“ und die
besten fünf Einträge; die Herausforderungsliste zeigt Bestwert bzw. Bestzeit.

### 4.3 Mitgelieferte Herausforderungen

| ID | Name | Kern |
|---|---|---|
| `ch_untouched_keep` | Unversehrter Bergfried | `KeepIntact`, `UnitCap(10)`, Mission 1 |
| `ch_twenty_nights` | Zwanzig Nächte | Überleben, Sieg bei `DayStarted(21)` |
| `ch_shrine_breaker` | Schreinbrecher (Speedrun) | `StartResources(1.6)`, `WaveStrength(0.8)`, `TimeLimit(1500)`, Rang nach Zeit |
| `ch_iron_man` | Eiserner Mann | Schwer, `IronMan`, `HeroPermadeath` |
| `ch_hunger_winter` | Hungerwinter | `StartResources(0.4)`, `WaveInterval(0.8)`, fünf Nächte oder Schrein |
| `ch_few_blades` | Wenige Klingen | Überleben, `UnitCap(6)`, sieben Nächte |
| `ch_endless_waves` | Endlose Wellen | Überleben, Schwer, `WaveStrength(1.2)`, Wertung nach Wellen |
| `ch_grauwall_stand` | Der letzte Wall | Szenario Grauwall, Schwer, `UnitCap(14)`, `KeepIntact` |

---

## 5. Gefecht-Einrichtung

Hauptmenü → **Gefecht**. Einstellungen:

* **Karte**: *Generiert* (Vorlage, Größe 48/72/96/128, Seed mit Würfeln; Vorschaubild mit Startpositionen, Minen
  und Wäldern) oder *Szenario* (jede Karte aus 3.).
* **Schwierigkeit** (Erzählung/Normal/Schwer), **Startressourcen** (Karg ×0,5, Normal, Reich ×2, Üppig ×4),
  **Siegbedingung** bei generierten Karten: *Basis zerstören* (Modus Gefecht), *Überleben* (1–20 Nächte, Modus
  Überleben, Schrein unzerstörbar) oder *Vernichtung* (Schrein und alle Hohlen), dazu der dauerhafte Heldentod.

Die Schwierigkeit wird unverändert als Datum in `MatchSettings::difficulty` durchgereicht; die Gegner-KI des
Gefechtsmodus liest sie von dort (heute skaliert sie die Wellenstärke über `Difficulty::wave_mult`). Der Einrichtungsdialog
enthält keine KI-Logik. Generierte Gefechtskarten bestehen vor dem Start dieselbe Prüfung wie Editorkarten
(Test `skirmish_builds_for_all_presets_sizes_and_wins`).

---

## 6. Befehlszeile

```text
--scene editor | scenarios | challenges | skirmish-setup     Szene öffnen (Screenshots)
--scenario <id> | --challenge <id> [--outcome victory|defeat]  Szenario/Herausforderung starten
--editor-tool <name> --editor-dialog <data|new|load> --editor-tab <reiter> --editor-demo
--select <n>                                                   Eintrag der Listen vorwählen
cargo run -p ashen_tools -- validate-data                      prüft Missionen, Szenarien, Herausforderungen
cargo run -p ashen_tools -- forge-scenarios                    erzeugt die historischen Szenarien neu
```

Benutzerordner: `ASHEN_USER_DIR`, sonst plattformüblich (Linux `~/.local/share/ashen_crowns`, macOS
`~/Library/Application Support/AshenCrowns`, Windows `%APPDATA%\AshenCrowns`).

## 7. Grenzen

* Nur eine Spielerseite wird gespielt (Spieler 0); Startposition und Aufstellung für Spieler 1 dienen der Gegnerseite.
* Der Editor kennt keine Höhenstufen (nicht im Datenmodell) und keine Weltkarten-Szenarien (Grand Campaign ist geplant).
* Die Bestenliste ist rein lokal; ein Spielername wird noch nicht abgefragt („Wächterin“).
* Der Editor setzt fertige Gebäude. Baustellen und Ausbauzustände einzelner Gebäude lassen sich nicht vorgeben.
