# ASHEN CROWNS – Audio-Integration

Wie Audiodateien in das Spiel gelangen, wie sie zugeordnet, geprüft und abgespielt werden und was zu tun ist, wenn die
produzierten Dateien geliefert werden. Grundlage ist der **tatsächliche Code**: `crates/ashen_engine/src/audio/`
(`manifest.rs`, `scanner.rs`, `mod.rs`, `backend.rs`), `crates/ashen_game/src/audio_bridge.rs`,
`crates/ashen_tools/src/main.rs` und das Manifest `assets/audio/manifest.ron`. Produktionsvorlagen (Prompts, Skripte) stehen in
`docs/audio/elevenlabs_*.md` und `docs/audio/suno_master_production.md`; die Dateinamenskonvention ist in
[`../canon.md`](../canon.md) verbindlich festgelegt.

**Aktueller Stand:** Im Repository liegen **keine Audiodateien**. Die Ordner enthalten nur `.gitkeep`;
`assets/audio/audio_report.md` meldet *0 Dateien gefunden, 124 erwartete Muster, 124 fehlend*. Das System ist
vollständig implementiert und getestet und läuft ohne Dateien stumm und fehlerfrei.

---

## 1. Ordnerstruktur

Alles liegt unter `assets/audio/`. Pfade im Manifest sind relativ dazu und **ohne Dateiendung**.

```text
assets/audio/
├── manifest.ron                     Zuordnung Event → Dateimuster (siehe Abschnitt 3)
├── audio_report.md                  Letzter Scanner-Bericht (wird von audio-scan geschrieben)
├── music/                           Musikspuren: <track>_loop.ogg, <track>_intro.ogg, Stings
├── ambience/                        Dauerhafte Umgebungsschichten: ambience_<id>_loop.ogg
├── sfx/
│   ├── combat/                      Kampfgeräusche
│   ├── magic/                       Fähigkeiten, Beschwörungen
│   ├── buildings/                   Bauen, Fertigstellen, Zerstören, Ausbilden
│   ├── ui/                          Oberfläche, Ziele
│   ├── units/                       (reserviert; derzeit von keinem Manifest-Eintrag genutzt)
│   └── world/                       Arbeit, Wetter, Horn
└── voices/
    ├── units/                       Einheitenstimmen (inkl. Bosse): <unit_id>_<situation>_<NN>.ogg
    ├── heroes/                      Heldenstimmen: <hero_id>_<situation>_<NN>.ogg
    ├── narrator/                    Erzähler: narrator_<line_id>_<NN>.ogg
    └── characters/                  Figuren-Dialoge: <char_id>_<line_id>_<NN>.ogg
```

Der Scanner durchsucht den gesamten Baum rekursiv; entscheidend ist der **relative Pfad** zu `assets/audio`, der zum
Muster im Manifest passen muss.

---

## 2. Namenskonvention

Verbindlich (Scanner-geprüft, `docs/canon.md`):

| Regel | Beschreibung |
|---|---|
| Zeichen | Nur Kleinbuchstaben `a–z`, Ziffern, Unterstriche (`snake_case`). Dateiendung ebenfalls klein. |
| Format | Bevorzugt **`.ogg`** (Vorbis). Abspielbar: `ogg`, `wav`, `flac`, `mp3`. Konvertierbar (per `ffmpeg`): `m4a`, `aac`, `aiff`, `aif`, `opus`, `webm`, `wma`, `caf`. |
| Varianten | Zweistellige Nummer ab `01`: `archer_move_01.ogg`, `archer_move_02.ogg`, … Im Manifest steht `_*`. |
| Pfad | `assets/audio/<ordner>/<name>[_NN].<ext>` |

Muster nach Kategorie:

```text
assets/audio/music/<track_id>[_intro|_loop].ogg                  music/build_calm_loop.ogg
assets/audio/voices/heroes/<hero_id>_<situation>_<NN>.ogg        voices/heroes/hero_isolde_select_01.ogg
assets/audio/voices/units/<unit_id>_<situation>_<NN>.ogg         voices/units/archer_move_02.ogg
assets/audio/voices/characters/<char_id>_<line_id>_<NN>.ogg      voices/characters/char_hal_venn_m01_intro_03.ogg
assets/audio/voices/narrator/narrator_<line_id>_<NN>.ogg         voices/narrator/narrator_m01_opening_01.ogg
assets/audio/sfx/<kategorie>/<sfx_id>_<NN>.ogg                   sfx/combat/sword_hit_01.ogg
assets/audio/ambience/<ambience_id>_loop.ogg                     ambience/ambience_day_loop.ogg
```

Situationen für Einheiten und Helden: `select`, `move`, `attack`, `retreat`, `ability`, `hit`, `death`, `victory`;
Helden zusätzlich `greet`, `levelup`, `lowhp`; Arbeiter zusätzlich `work`, `build_done`.
SFX-Kategorien: `combat`, `magic`, `buildings`, `ui`, `units`, `world`.

**Varianten-Regel (Scanner):** Das Muster `<basis>_*` passt auf Dateien im **selben Ordner**, deren Name genau
`<basis>_` plus **zwei Ziffern** ist. `worker_select_1.ogg` (einstellig) oder `worker_select_01_alt.ogg` passen nicht.
Einzeldateien ohne `_*` (z. B. `music/build_calm_loop`) werden über den exakten relativen Pfad zugeordnet.

Namensnormalisierung (`scanner::normalize_name`), angewandt bei ungültigen Namen: Umlaute `ä/ö/ü/ß` → `a/o/u/s`,
alle Nicht-Alphanumerika → `_` (mehrfache zusammengefasst, am Ende entfernt), Kleinschreibung, einstellige Endnummer
wird zweistellig (`_1` → `_01`), Endung klein. Beispiele: `Archer Move 1.OGG` → `archer_move_01.ogg`,
`hero-isolde_select_02.wav` → `hero_isolde_select_02.wav`.

---

## 3. Manifestformat (`assets/audio/manifest.ron`)

RON mit benannten Strukturen. Wurzel `AudioManifest(version, music, events, ambience)`.

```ron
AudioManifest(
    version: 1,
    music: [
        MusicTrack(id: "build_calm", intensity: Calm, file: "music/build_calm_loop",
                   intro: Some("music/build_calm_intro"), looped: true),
        MusicTrack(id: "victory_sting", intensity: Sting, file: "music/victory_sting", intro: None, looped: false),
    ],
    events: [
        SoundEvent(id: "voice.archer.move", category: Voice, files: ["voices/units/archer_move_*"],
                   volume_db: 0.0, cooldown_ms: 250, max_instances: 1),
        SoundEvent(id: "narrator.m01_opening", category: Narration, files: ["voices/narrator/narrator_m01_opening_*"],
                   volume_db: 2.0, cooldown_ms: 0, max_instances: 1, playback: Sequential),
    ],
    ambience: [
        AmbienceLayer(id: "ambience_day", file: "ambience/ambience_day_loop", volume_db: -6.0),
    ],
)
```

| Typ | Feld | Bedeutung |
|---|---|---|
| `AudioManifest` | `version` | Manifestversion (derzeit 1) |
| `MusicTrack` | `id` | Spur-ID (z. B. `build_calm`) |
| | `intensity` | `Menu`, `Calm`, `Tension`, `Combat`, `Battle`, `Boss`, `Sting` |
| | `file` | Muster der Hauptdatei (Loop) ohne Endung |
| | `intro` | Optionales Intro (`Some("…")`/`None`) |
| | `looped` | `true` = Endlosschleife der gesamten Datei |
| `SoundEvent` | `id` | Event-ID (`voice.<unit>.<situation>`, `sfx.<id>`, `narrator.<line>`, `char_<id>.<line>`) |
| | `category` | `Voice`, `Narration`, `Dialogue`, `Sfx`, `Ui` |
| | `files` | Liste von Mustern (mit `_*` für Varianten) |
| | `volume_db` | Pegel des Events in dB (Standard 0) |
| | `cooldown_ms` | Mindestabstand zwischen zwei Auslösungen desselben Events |
| | `max_instances` | Höchstzahl gleichzeitiger Wiedergaben (0 = unbegrenzt) |
| | `playback` | **Optional**, Standard `Random`. `Sequential` spielt alle Teile nacheinander. |
| `AmbienceLayer` | `id`, `file`, `volume_db` | Umgebungsschicht (Schleife) |

**`playback`:**

* `Random` (Standard): Zufällige Variante, nie dieselbe zweimal direkt hintereinander (bei mindestens zwei Varianten).
* `Sequential`: Alle zugeordneten Dateien (`_01`, `_02`, … in Nummernreihenfolge) werden **nacheinander** abgespielt –
  für Erzählerpassagen und mehrteilige Dialoge. Im aktuellen Manifest tragen acht Events `Sequential`:
  `narrator.m01_opening`, `narrator.m01_victory`, `narrator.m01_defeat` und fünf Hal-Venn-Dialoge
  (`m01_intro`, `m01_first_farm`, `m01_barracks`, `m01_shrine_found`, `m01_victory`). `narrator.m01_night_falls` und
  `char_hal_venn.m01_wave_warning` sind `Random` (je eine Zeile, evtl. mit Varianten).

**Kategorie und Bus:**

| Kategorie | Bus | Wiedergabe |
|---|---|---|
| `Voice` | Voice | Einzelner Sound (One-Shot) mit Cooldown und Instanzbegrenzung |
| `Narration`, `Dialogue` | Voice | Gehen immer über die **Sprachwarteschlange** (nacheinander, Musik wird abgesenkt); `Random` reiht eine Variante ein, `Sequential` alle Teile |
| `Sfx` | Sfx | One-Shot |
| `Ui` | Ui | One-Shot (siehe Hinweis) |

Die Oberflächenklänge (`sfx.ui_click`, `sfx.ui_hover`, `sfx.ui_error`, `sfx.ui_notify`, `sfx.objective_new`,
`sfx.objective_complete`) sind als `Category::Ui` deklariert und laufen über den Bus *Ui* (Regler „Oberfläche“).

Für Sequenzereignisse (Erzähler/Dialog) werden Cooldown berücksichtigt, `max_instances` jedoch nicht (die Wiedergabe
läuft über die Warteschlange, nicht über zählbare One-Shot-Handles).

**Umfang des aktuellen Manifests:**

| Bereich | Anzahl |
|---|---|
| Musikspuren | 8 (Menü, Aufbau ruhig, Aufbau angespannt, Gefecht klein, Gefecht groß, Boss Fahler Vogt, Siegfanfare, Niederlagenfanfare) |
| Events | 105: 62 `Voice` (51 Einheiten, 11 Helden), 4 `Narration`, 6 `Dialogue`, 33 `Sfx` |
| Ambience | 6 (`ambience_day`, `_night`, `_rain`, `_ash_wind`, `_village`, `_ruins`) |
| Erwartete Dateimuster | 124 (13 Musik inkl. 5 Intros, 105 Event-Muster, 6 Ambience) |

---

## 4. Der Scanner (`ashen_engine::audio::scanner`)

`scanner::scan(root, &manifest) -> ScanReport` wird beim Spielstart automatisch ausgeführt (`AudioSystem::new`) und
steht als Werkzeug zur Verfügung (Abschnitt 5). Er arbeitet rein lesend (Ausnahme: die beiden ausdrücklich
angeforderten Befehle `--apply-renames` und `--convert`).

**Ablauf:**

1. Alle Dateien unter `assets/audio` rekursiv sammeln; behalten werden nur Endungen aus `PLAYABLE` oder `CONVERTIBLE`.
2. Für jede Datei: konvertierbares Format → `needs_conversion`; ungültiger Name → `invalid_names` mit
   Umbenennungsvorschlag; gültiger, abspielbarer Name → Kopfbytes prüfen (`check_magic`: `OggS` bei ogg/opus,
   `RIFF…WAVE` bei wav, `fLaC` bei flac, `ID3` oder MPEG-Sync bei mp3) → bei Abweichung `corrupt`
   („Dateiinhalt passt nicht zur Endung“; leere Dateien: „Datei leer oder zu klein“).
3. Jedes erwartete Muster (`manifest.patterns()`) wird gegen die **gültig benannten, abspielbaren** Dateien aufgelöst:
   Muster `<basis>_*` → alle `<basis>_NN`; sonst exakter relativer Pfad. Gefundene Dateien werden nach Variantennummer
   sortiert.
4. Lücken in den Variantennummern (`variant_gaps`): vorhanden `01`, `03` → fehlt `02`.
5. Gleicher Name mit mehreren Endungen → `duplicates`; bevorzugt wird `.ogg`, sonst die erste Datei.
6. Muster ohne Datei → `missing` (mit Besitzer, z. B. `event:voice.worker.select`, `music:main_menu:intro`).
7. Datei, die zu mehr als einem Muster passt → `ambiguous` (manuell zu klären).
8. Gültig benannte, abspielbare Datei ohne Muster → `unknown`.
9. Inhaltsgleiche Dateien (gleiche Größe und CRC32) → `duplicates`.

**Automatische Zuordnung:** Es gibt keine manuelle Zuordnungstabelle. Eine Datei wird einem Event zugeordnet, **allein weil
Ordner und Name zum Muster im Manifest passen**. Wer eine Datei an den richtigen Ort mit dem richtigen Namen legt, hat sie
„angeschlossen“. Ungültig benannte Dateien werden **nicht** verwendet, bis sie umbenannt sind (sie tauchen nur in
`invalid_names` auf).

`ScanReport` (Auszug): `resolved` (Muster → Dateien), `missing`, `variant_gaps`, `invalid_names`, `ambiguous`,
`duplicates`, `unknown`, `needs_conversion`, `corrupt`, `total_files`. Methoden: `files_for(muster)`, `found_count()`,
`is_clean()` (keine ungültigen Namen, Duplikate, beschädigten Dateien, Konvertierungen, Mehrdeutigkeiten – fehlende Dateien
zählen nicht), `to_markdown()`.

Im Spiel: Beim Start protokolliert `AudioSystem::new` `Audio: <n> Dateien zugeordnet, <m> Muster ohne Datei`; die
Einstellungen zeigen unten `Audio: <backend> · Zugeordnete Audiodateien: <n> · Fehlend: <m>`.

---

## 5. Befehl `audio-scan` und Bericht

```text
cargo run -p ashen_tools -- audio-scan [--assets <dir>] [--apply-renames] [--convert] [--report <datei.md>]
```

| Option | Wirkung |
|---|---|
| (ohne Option) | Nur scannen und Bericht schreiben |
| `--assets <dir>` | Asset-Verzeichnis (sonst automatisch gesucht: `ASHEN_ASSETS` oder Projektordner) |
| `--apply-renames` | Ungültige Dateinamen laut Vorschlag **umbenennen** – nur, wenn die Zieldatei noch nicht existiert (sonst Meldung „übersprungen (Ziel existiert)“) |
| `--convert` | Dateien mit Format aus `CONVERTIBLE` mit `ffmpeg` nach `.ogg` konvertieren (`-c:a libvorbis -q:a 6 -ar 48000`), neben der Originaldatei. Existiert die `.ogg` schon, wird nicht überschrieben. `ffmpeg` muss installiert sein. |
| `--report <datei.md>` | Berichtsdatei (Standard `assets/audio/audio_report.md`) |

Ablauf des Werkzeugs: scannen → (optional) konvertieren → (optional) umbenennen → bei Änderungen erneut scannen →
Bericht als Markdown schreiben → Zusammenfassung ausgeben, z. B.:

```text
Audio: 0 Dateien, 0 zugeordnet, 124 Muster fehlen, 0 ungültige Namen, 0 Duplikate, 0 mehrdeutig, 0 beschädigt, 0 unbekannt.
Bericht: /…/assets/audio/audio_report.md
```

Exit-Code `0` bei Erfolg (auch wenn beschädigte Dateien gefunden wurden – dann erscheint zusätzlich die Meldung
„Beschädigte Audiodateien gefunden.“ auf stderr); `1` bei Fehlern (z. B. Manifest nicht lesbar); `2` bei falscher Bedienung.

**Reihenfolge innerhalb eines Laufs:** Zuerst werden ungültige Namen umbenannt (`--apply-renames`), dann wird neu
gescannt, danach konvertiert (`--convert`) und abschließend erneut gescannt; beide Optionen können also gemeinsam
verwendet werden. Die Originale konvertierter Dateien bleiben erhalten und stehen weiter unter „Konvertierung nötig“, bis sie
manuell entfernt werden.

**Bericht (`audio_report.md`)**: Kopf mit Wurzelpfad, gefundenen Dateien, zugeordneten Dateien, erwarteten und fehlenden
Mustern, dann Abschnitte (jeweils mit Anzahl, „Keine.“ bei leer):

1. Beschädigte Dateien
2. Ungültige Dateinamen (Vorschlag) – `von → nach (Grund)`
3. Mehrdeutige Zuordnung (bitte klären)
4. Duplikate
5. Konvertierung nötig
6. Lücken in Variantennummern
7. Unbekannte Dateien (keinem Event zugeordnet)
8. Fehlende Dateien – `Muster (Besitzer)`

---

## 6. Wiedergabe und Mischung (`AudioSystem`)

### 6.1 Busse und Lautstärke

Fünf Busse (kira-Subtracks): Musik, Effekte (Sfx), Stimmen (Voice), Umgebung (Ambience), Oberfläche (Ui). Einstellungen
(`Volumes`, Standard): Gesamt 0,85 · Musik 0,7 · Effekte 0,85 · Stimmen 1,0 · Umgebung 0,6 · Oberfläche 0,7. Jeder Bus
erhält `20·log10(Regler · Gesamt)` dB (Minimum −60 dB). Eventpegel (`volume_db`) und Distanzdämpfung kommen
hinzu.

Backend: `KiraBackend` (Cargo-Feature `audio`, Standard), sonst oder bei fehlendem Gerät `NullBackend` (stumm). One-Shots
werden vollständig in den Speicher geladen und zwischengespeichert; Musik und Ambience werden **gestreamt**.

### 6.2 Events (`play`, `play_at`)

`play(id)` bzw. `play_at(id, pan, attenuation)`:

1. Event im Manifest suchen (unbekannt → `false`, einmalige Debug-Meldung).
2. **Cooldown** (`cooldown_ms`) seit der letzten Auslösung prüfen.
3. **Instanzgrenze** (`max_instances`) anhand noch laufender Handles prüfen.
4. Dateien des Events auflösen (`files_for`); keine Dateien → `false` (einmalige Debug-Meldung).
5. Pegel: `volume_db + 20·log10(1 − Dämpfung)` (Dämpfung höchstens 0,95).
6. Variante wählen (zufällig ohne direkte Wiederholung) bzw. bei `Sequential` alle Teile einreihen; `Narration`/`Dialogue`
   in die Sprachwarteschlange, sonst One-Shot mit Stereo-Pan (−1…1).

### 6.3 Musiksteuerung

Die Musik wird über die **Intensität** gewählt (`set_music_intensity`). Es gibt pro Intensität höchstens eine Spur im
Manifest (die erste mit passender Intensität).

| Intensität | Spur (`music:<id>`) | Eingeführt für |
|---|---|---|
| `Menu` | `main_menu` | Hauptmenü |
| `Calm` | `build_calm` | Aufbau, keine Bedrohung |
| `Tension` | `build_tension` | Nacht, Welle naht, Feinde nahe der Basis |
| `Combat` | `battle_small` | Kleine Gefechte |
| `Battle` | `battle_large` | Große Schlachten |
| `Boss` | `boss_pale_reeve` | Kampf gegen den Fahlen Vogt |
| `Sting` | `victory_sting`, `defeat_sting` | Kurze Akzente (über `play_sting`, nicht über Intensität) |

Regeln:

* **Steigerung sofort:** Ist die neue Intensität höher als die aktuelle (oder läuft keine Musik, oder es geht von/zu
  `Menu`), wechselt die Spur sofort.
* **Absenkung mit Hysterese (8 s):** Eine niedrigere Intensität wird erst nach **8 Sekunden** ununterbrochen gewünschter
  Absenkung übernommen. Fordert das Spiel zwischenzeitlich eine andere Intensität, beginnt der Timer von vorn; kehrt es zur
  aktuellen zurück, wird die Absenkung verworfen. So flackert die Musik nicht bei kurzen Kampfpausen.
* **Crossfade 2,5 s:** Beim Spurwechsel blendet die alte Spur über 2,5 s aus (`stop` mit Fade), die neue über 2,5 s ein.
* **Intro → Loop:** Hat die Spur ein Intro (`intro`), wird zuerst das Intro **einmal** (nicht geloopt, ohne Einblendung)
  gespielt; sobald es endet, startet die Loop-Datei nahtlos (ohne Fade, geloopt über die gesamte Datei). Fehlt die
  Intro-Datei, startet direkt der Loop mit Crossfade. **Das Intro wird nur gespielt, wenn die Musik aus der Stille
  beginnt oder die Menü-Spur startet** (Missionsstart, Menü); bei Wechseln zwischen Intensitäten innerhalb einer
  Mission (z. B. Ruhe → Gefecht) wird der Loop direkt per Crossfade gestartet. Die Intros von `battle_small`,
  `battle_large` und `boss_pale_reeve` greifen daher nur, wenn diese Spur aus der Stille startet.
* **Stings (`play_sting(id)`):** Spielt die Datei einmalig auf dem Musikbus (kein Loop, kein Fade). Die laufende Musik wird
  währenddessen auf **−18 dB** abgesenkt (0,8 s) und nach Ende des Stings über 2 s auf den normalen Pegel
  zurückgeführt. Stings: `victory_sting` bei `Victory`, `defeat_sting` bei `Defeat`.
* **Ducking:** Solange ein Eintrag der Sprachwarteschlange (Erzähler/Dialog) läuft, wird die Musik auf **−8 dB**
  abgesenkt (0,6 s Übergang) und danach wieder angehoben. Während eines Stings ist das Ducking ausgesetzt. Einheitenstimmen
  (`Voice`) lösen **kein** Ducking aus.
* Beim Start einer Mission wird die Musik 1,5 s ausgeblendet (`stop_music`), beim Wechsel ins Menü 1,0 s; die Ambience
  wird beim Menüwechsel beendet.

### 6.4 Intensität aus dem Spielzustand (`audio_bridge::music_intensity`)

Die Auswertung läuft **jeden Frame** (Priorität von oben nach unten):

| Intensität | Bedingung |
|---|---|
| `Boss` | Eine gegnerische Einheit der Klasse `Boss` ist im Kampf (`combat_timer > 0`) |
| `Battle` | Mindestens 10 eigene Einheiten im Kampf **oder** mindestens 10 Gegner innerhalb von 18 Kacheln um das Hauptgebäude (`defeat_tag`) |
| `Combat` | Mindestens 1 eigene Einheit im Kampf (`combat_timer > 0`, 4 s nach dem letzten Treffer) |
| `Tension` | Wellen-Direktor aktiv und nächste Welle in weniger als 30 s, **oder** Nacht, **oder** mindestens 1 Gegner innerhalb von 18 Kacheln um das Hauptgebäude |
| `Calm` | sonst |

Im Hauptmenü wird dauerhaft `Menu` gesetzt.

### 6.5 Ambience-Schichten (`audio_bridge::ambience`)

Mehrere Schichten laufen gleichzeitig als gestreamte Schleifen auf dem Bus *Ambience*. Jede Schicht hat eine relative
Lautstärke 0..1, die zusätzlich zum `volume_db` der Schicht gilt (`volume_db + 20·log10(Pegel)`). Neue Schichten blenden
über 3 s ein, Pegeländerungen laufen über 2 s, nicht mehr gewünschte Schichten blenden über 3 s aus.

| Schicht | Bedingung | Pegel |
|---|---|---|
| `ambience_day` | Tag (nicht Nachtphase) | 1,0 |
| `ambience_night` | Nachtphase | 1,0 |
| `ambience_rain` | Wetter Regen oder Sturm | Wetterintensität (0..1) |
| `ambience_ash_wind` | Wetter Aschenfall **oder** Kamera näher als 22 Kacheln an `ENEMY_BASE` | 0,8 |
| `ambience_ruins` | Kamera näher als 22 Kacheln an `ENEMY_BASE` | 0,9 |
| `ambience_village` | Kamera näher als 16 Kacheln an `PLAYER_BASE` | 0,8 |

Die Position bezieht sich auf die **Kameramitte** (`camera.center`), nicht auf Einheiten.

---

## 7. Event-Mapping (`crates/ashen_game/src/audio_bridge.rs`)

Die Brücke übersetzt `SimEvent`s in Audio-Events. Bei räumlichen Events (`at`) werden **Pan** (aus der horizontalen
Lage relativ zur Bildschirmmitte, höchstens ±0,7) und **Dämpfung** (nach Abstand zur Bildmitte) berechnet; ab Dämpfung 0,95
wird nicht mehr abgespielt. Zufallsentscheidungen („1 aus n“) sind deterministische Hash-Funktionen von Position und Tick
(`chance`), nicht der Simulations-RNG.

| `SimEvent` | Audio-Event(s) | Bemerkung |
|---|---|---|
| `MeleeSwing` | `sfx.sword_hit` (Standard), `sfx.spear_hit` (`spearman`, `knight`), `sfx.bone_crunch` (`grave_brute`); zusätzlich mit Wahrscheinlichkeit 1/5 `voice.<unit_id>.attack` | räumlich |
| `Hit` | `sfx.shield_block` (blockiert); `sfx.building_hit` (Ziel ist Gebäude); `voice.<unit_id>.hit` (Ziel unter 30 % LP, Wahrscheinlichkeit 1/4) | räumlich |
| `ProjectileFired` | `sfx.arrow_fire` (`Arrow`, `BoneBolt`); `sfx.reeve_soul_drain` (`SoulBolt`) | räumlich |
| `ProjectileImpact` (`hit: true`) | `sfx.arrow_impact` | räumlich |
| `UnitDied` | `sfx.unit_death_hollow` (Besitzer Gegner) bzw. `sfx.unit_death_human`; `voice.<unit_id>.death` | räumlich |
| `BuildingPlaced` (Spieler) | `sfx.build_place` | räumlich |
| `BuildingCompleted` (Spieler) | `sfx.build_complete`; `voice.worker.build_done` | letzteres nicht räumlich |
| `BuildingDestroyed` | `sfx.building_destroyed` | räumlich, alle Besitzer |
| `TrainCompleted` (Spieler) | `sfx.train_complete`; `voice.<unit_id>.select` | nicht räumlich |
| `WorkStroke` | Holz `sfx.wood_chop`, Gold `sfx.mine_pick`, Stein `sfx.stone_quarry`, Nahrung `sfx.harvest_grain`, Bauen `sfx.build_progress` | räumlich |
| `AbilityCast` | `sfx.<ability_id>` (z. B. `sfx.ember_cleave`, `sfx.rallying_banner`, `sfx.ashen_ward`); `voice.<caster_unit_id>.ability`; bei `summon_thralls` zusätzlich `sfx.hollow_summon` | räumlich |
| `LevelUp` | `voice.hero_isolde.levelup` | fest auf Isolde verdrahtet |
| `WaveIncoming` (`size == 0`, Vorwarnung) | `sfx.horn_alarm` | nicht räumlich |
| `Message` mit `voice: Some(id)` | Event `<id>` (z. B. `narrator.m01_opening`, `char_hal_venn.m01_intro`) | Sprachwarteschlange |
| `ObjectiveAdded` / `ObjectiveCompleted` | `sfx.objective_new` / `sfx.objective_complete` | |
| `Error` (Spieler) | `sfx.ui_error` | |
| `Lightning` | `sfx.thunder` | |
| `Victory` | Sting `victory_sting`; `voice.hero_isolde.victory` | |
| `Defeat` | Sting `defeat_sting` | |
| `HeroFallen` | `voice.hero_isolde.death` | |

Weitere Auslöser im Spielcode (`mission.rs`, `hud.rs`):

| Auslöser | Audio-Event |
|---|---|
| Auswahl per Klick/Rahmen, Befehle | `voice.<unit_id>.select`, `.move`, `.attack` für die erste ausgewählte eigene Einheit (höchstens alle 0,6 s) |
| Bauen, Sammeln, Reparieren befohlen | `voice.worker.work` |
| Einheit oder Gebäude angeklickt, Befehlsknopf im HUD | `sfx.ui_click` |

**Zusätzliche Auslöser:** `voice.<unit>.retreat` bei Moralbruch (`MoraleBroken`), `voice.hero_isolde.greet` beim
ersten Anwählen der Heldin, `voice.hero_isolde.lowhp` bei Treffern unter 25 % Lebenspunkten, `sfx.horse_charge` beim
Angriff eines Ritters, `narrator.m01_defeat` bei Niederlage. Boss-Fähigkeiten: `soul_drain` → `sfx.reeve_soul_drain`,
`summon_thralls` → `sfx.hollow_summon`, `dread_wail` → `voice.boss_pale_reeve.taunt`; alle übrigen Fähigkeiten →
`sfx.<ability_id>`.

**Im Manifest definiert, vom Spielcode derzeit nicht ausgelöst:** `sfx.ui_hover`, `sfx.ui_notify`, `sfx.fire_crackle`,
`voice.<unit>.victory` der regulären Einheiten (nur `voice.hero_isolde.victory` wird gespielt) und
`voice.hero_isolde.retreat` (Helden fliehen nicht). Sie sind für spätere Phasen vorgesehen (Phase 15, Politur).

Fehlende Events werden einmalig auf Debug-Ebene protokolliert (`RUST_LOG=debug`).

Einheiten aus Mods erhalten Events nach demselben Muster (`voice.<mod-unit-id>.<situation>`), die im Manifest ergänzt werden
müssten; ohne Eintrag bleiben sie stumm.

---

## 8. Ablauf, wenn die Audiodateien geliefert werden (10 Schritte)

1. **Lieferung sichten.** Dateien und Herkunft (ElevenLabs/Suno-Exporte, siehe `docs/audio/*.md`) prüfen. Bevorzugtes Format
   ist `.ogg` (Vorbis); Musikschleifen müssen als **ganze Datei nahtlos loopbar** sein, Intros sind eigene Dateien.
2. **In die Ordnerstruktur kopieren** (Abschnitt 1) – nach Möglichkeit direkt mit den Zielnamen aus Abschnitt 2.
3. **Namen normalisieren:** `cargo run -p ashen_tools -- audio-scan --apply-renames`. Bei Meldungen
   „übersprungen (Ziel existiert)“ Dubletten manuell auflösen. Wiederholen, bis keine ungültigen Namen mehr gemeldet werden.
4. **Formate umwandeln:** `cargo run -p ashen_tools -- audio-scan --convert` (benötigt `ffmpeg`). Danach die
   Originaldateien (`m4a`, `aiff` …) aus `assets/audio` entfernen oder außerhalb des Baums ablegen.
5. **Bericht lesen:** `assets/audio/audio_report.md` – beschädigte Dateien, Duplikate, Lücken in Variantennummern, unbekannte
   Dateien, mehrdeutige Zuordnungen und fehlende Muster abarbeiten (Dateien nachliefern, umbenennen oder löschen).
6. **Manifest anpassen**, falls nötig: neue Events oder Spuren in `assets/audio/manifest.ron`, `volume_db`, `cooldown_ms`,
   `max_instances`, `playback: Sequential` für mehrteilige Erzähler-/Dialogzeilen. Neue Events, die der Spielcode auslösen soll,
   brauchen eine passende Zuordnung in `audio_bridge.rs` (siehe Abschnitt 7).
7. **Erneut scannen**, bis `is_clean` erreicht ist (Zusammenfassung ohne ungültige Namen, Duplikate, Mehrdeutigkeiten,
   beschädigte Dateien). Fehlende Muster für bewusst noch nicht gelieferte Events sind zulässig.
8. **Spiel starten:** `RUST_LOG=debug cargo run --release`. Kontrollieren: Logzeile `Audio: <n> Dateien zugeordnet, <m> Muster
   ohne Datei`; keine Warnungen `nicht ladbar`; in den Einstellungen `Audio: kira`. Fehlende Events erscheinen als Debug-Meldungen.
9. **Hörtest** nach Checkliste: Menümusik (Intro → Loop); Missionsstart (`build_calm`-Intro); Nachteinbruch (Tension,
   Ambience-Wechsel Tag → Nacht); Regen/Sturm (`ambience_rain`, Donner); Ruinen und Dorf (Ambience-Schichten); kleiner Kampf
   (Combat), große Schlacht (Battle, Crossfade 2,5 s, Rückkehr nach 8 s); Boss (`Boss`); Sieg und Niederlage (Stings, Absenken der
   Musik); Erzähler und Hal-Venn-Dialoge (Reihenfolge der Teile, Ducking −8 dB); Einheitenstimmen (Auswahl, Befehle, Tod);
   Kampf-, Bau- und Arbeits-SFX (Cooldowns, Instanzgrenzen); Pan und Distanzdämpfung beim Scrollen; Lautstärkeregler.
10. **Feinabstimmung und Abnahme:** Pegel angleichen (`volume_db` je Event), Schleifenübergänge und Intro-Übergänge prüfen,
    Cooldowns und `max_instances` justieren, Bericht erneut erzeugen und mit den Dateien ins Repository übernehmen
    (`assets/audio/**`, `audio_report.md`); Produktionsdokumente in `docs/audio` bei Abweichungen aktualisieren. Offene
    Auslöser aus Abschnitt 7 (nicht ausgelöste oder stumme Events) als Aufgaben festhalten.

---

## 9. Grenzen und Ausblick

* Die Intros der Kampf- und Boss-Spuren werden im laufenden Spiel nur bei Start aus der Stille abgespielt (siehe 6.3).
* Die Sprachsituation `victory` der regulären Einheiten sowie `sfx.ui_hover`, `sfx.ui_notify` und `sfx.fire_crackle`
  haben noch keine Auslöser (Phase 15).
* Eine Audio-Erweiterung durch Mods (eigene Events/Spuren) ist nicht vorgesehen; das Manifest ist global
  (siehe [`../tech/modding.md`](../tech/modding.md)).
* Mehrstufige Musik (Stems, taktsynchrone Übergänge, wie in `suno_master_production.md` für den Boss erwähnt) sind im
  Audiosystem **nicht** implementiert; derzeit gibt es nur Crossfades zwischen ganzen Spuren.
