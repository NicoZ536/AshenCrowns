# ASHEN CROWNS – Audio-Produktionsliste (Master-Checkliste)

Alles, was für das Spiel noch produziert werden muss: **1041 Dateien** in vier Abschnitten. Jeder Eintrag enthält Zieldatei (Ordner + exakter
Name), vollständigen Prompt bzw. einfügefertigen Sprechtext (mit Audio-Tags) und die Einstellungen, damit direkt am Werkzeug gearbeitet werden kann.
Die Dateinamen sind aus dem Manifest (`assets/audio/manifest.ron`) per `ashen_tools audio-list` erzeugt; die Prompts stammen aus den Fach-Dokumenten
(`docs/audio/elevenlabs_*.md`, `suno_master_production.md`). Bei Abweichungen ist das **Manifest maßgeblich**.

> Diese Datei ist erzeugt (`python3 docs/audio/gen_production_checklist.py`). Kästchen darf man abhaken, solange man nicht neu erzeugt.

## Lieferanleitung (kurz)

1. **Alles in einen Ordner** legen (z. B. `lieferung/`; Unterordner sind erlaubt).
2. **Dateinamen exakt** wie in dieser Liste (der Name vor der Endung; Kleinbuchstaben, Ziffern, Unterstriche).
3. **Beliebiges Format** (WAV, MP3, FLAC, M4A, OGG …) – am besten die höchste Qualität, die das Werkzeug anbietet.
4. **Keine Nachbearbeitung:** nichts schneiden, kürzen, normalisieren, konvertieren, loopen oder mischen. Jede Datei ist eine **komplette Generierung** (die beste).
5. **Keine Teilstücke:** Jede Sprechzeile ist **eine** Datei mit dem **vollständigen** Text; jeder Musiktrack ist **eine** Datei, die komplette Suno-Ausgabe.
6. **Sprechtexte 1:1 kopieren:** Der Codeblock (```text) unter jeder Stimmen-Datei enthält den fertigen Text inklusive Audio-Tags in eckigen Klammern
   (z. B. „[solemn] Jahr 897 nach dem Fall. [pause] Am Rand der Aschmark …“). Nichts ergänzen, nichts entfernen.
7. **Abweichungen zwischen gesprochenem Ton und Spieltext** behebt das Team **nach der Lieferung**, indem die Spieltexte an den Ton angepasst werden.
8. Alles andere (Stille, Lautheit, Loop-Nähte, Format, Mono/Stereo, Effekte) erledigt die Integration – siehe „Erledigt die Integration (nicht vom Nutzer)“.

## Statistik

| Abschnitt | Werkzeug | Dateien |
|---|---|---|
| A – Soundeffekte | ElevenLabs Sound Effects | 219 |
| B – Stimmen (Erzähler, Figuren, Bosse, Einheiten, Helden) | ElevenLabs Text to Speech (Eleven v3) / Voice Design | 782 |
| C – Ambience-Loops | ElevenLabs Sound Effects (eine Generierung je Datei, Loop an) | 6 |
| D – Musik (je Track eine Datei) | Suno | 34 |
| **Gesamt** | | **1041** |

Aufteilung B: Erzähler 52, Dialogzeilen 187, Einheiten-/Helden-/Boss-Barks 543. Aufteilung D: 34 Tracks, je eine Datei (davon 31 als Ganzes geloopt, 3 Stings/einmalige Stücke).

## Ablagestruktur (`assets/audio/`)

```text
assets/audio/
├── music/               Musik: <track>.ogg (genau eine Datei je Track, auch Stings und Stücke)
├── ambience/            Umgebungs-Loops: ambience_<id>_loop.ogg
├── sfx/
│   ├── combat/ magic/ buildings/ ui/ world/     Soundeffekte: <sfx_id>_NN.ogg
└── voices/
    ├── units/           Einheiten- und Boss-Barks: <einheit>_<situation>_NN.ogg   (inkl. Fledermausschwarm aus Abschnitt A)
    ├── heroes/          Helden-Barks: hero_<id>_<situation>_NN.ogg
    ├── narrator/        Erzähler: narrator_<mission>_<szene>_01.ogg   (eine Datei je Zeile)
    └── characters/      Dialogzeilen aller Sprecher: <sprecher>_<mission>_<szene>_01.ogg   (eine Datei je Zeile)
```

## Benennung, Zeilen, Varianten

- **Dateinamen:** nur Kleinbuchstaben, Ziffern, Unterstriche. Exakt wie in dieser Liste (Name ohne Endung). Groß-/Leerzeichen/Bindestriche
  werden mit `--apply-renames` korrigiert, aber **Wortlaut und Nummer müssen stimmen**.
- **Eine Zeile = eine Datei:** Erzähler- und Dialogzeilen heißen immer `…_01` und enthalten den vollständigen Text. Frühere Mehrteiler sind zusammengeführt.
- **Barks mit mehreren Dateien** (`worker_select_01`, `_02`, …) sind **echte Textvarianten**: jede Datei hat ihren eigenen, vollständigen Text. Zweistellig ab `01`, lückenlos.
  Zusätzliche Takes **nicht** als weitere Varianten nummerieren; die beste Aufnahme pro Eintrag liefern.
- **SFX-Varianten** (`_01`–`_03`) haben je einen eigenen Prompt; Ambience und Musik je eine Datei.

## Erledigt die Integration (nicht vom Nutzer)

Alles in diesem Abschnitt übernimmt die Integration an den gelieferten Rohdateien. Der Nutzer liefert unverändert und muss nichts davon tun.

**Allgemein**

- **Stille trimmen:** Kopfstille Stimmen ≤ 20 ms (SFX-Impacts ≤ 5 ms), Schwanzstille ≤ 150 ms (Dialog bis 300 ms); Fade-in 5 ms, Fade-out 20–40 ms (Ausnahme Loops: kein Fade); zu lange SFX-Ausklänge kürzen.
- **Lautheit normalisieren** (ITU-R BS.1770, True Peak überall ≤ −1 dBTP):

| Kategorie | Ziel | Kanäle |
|---|---|---|
| Dialog / Charaktere / Erzähler | −16 LUFS integriert (±1 LU) | Mono |
| Einheiten-/Helden-/Boss-Barks | −16 LUFS Short-Term-Max (±1,5 LU); Hohle/Kreaturen −17 LUFS | Mono |
| Kampf-SFX | −14 bis −18 LUFS Short-Term-Max | Mono |
| Magie-SFX | −14 bis −16 LUFS Short-Term-Max | Mono (Stereo, wenn nicht positioniert) |
| Gebäude-SFX | −16 bis −20 LUFS Short-Term-Max | Mono |
| UI-SFX | −20 bis −24 LUFS Short-Term-Max, Peak ≤ −6 dBFS | Mono oder Stereo |
| Welt-SFX (Arbeit, Wetter) | −18 bis −22 LUFS Short-Term-Max (`thunder` stereo) | Mono |
| Ambience-Loops (Abschnitt C) | −23 LUFS integriert (±1 LU) | Stereo |
| Musik (Abschnitt D) | −14 LUFS integriert (±1 LU); Stings Short-Term-Max ca. −12 LUFS | Stereo |

  Kurze Dateien (< 3 s): Varianten einer Familie gehörmäßig gleich laut machen (Referenz Stimmen `hero_isolde_select_01`, Kampf-SFX `sword_hit_01`).
- **Loop-Nähte:** Ambience-Loops auf Nulldurchgang schneiden und per Wrap-Around-Crossfade nahtlos machen (kein Fade an Anfang/Ende, 5 Durchläufe probehören), auf die Ziellänge verlängern.
  Musik: die gelieferte komplette Suno-Ausgabe wird **als Ganzes** geloopt; Loop-Länge = ganze Takte, Tail-Wrap, 20–50 ms Naht-Crossfade, Lautheit per statischem Gain. Stings werden nicht geloopt.
- **Formatkonvertierung:** alle Formate nach Ogg Vorbis q6, 48 kHz (`audio-scan --convert`).
- **Mono/Stereo:** wie in der Lautheitstabelle; Mono-Summe auf Phasenauslöschung prüfen.
- **Effekte und Klangsignaturen** (Hohle-Kette, Deep-Voice-Schichten, Hall der Bosse, Helm-Resonanz der Legion): Leitfaden Abschnitt 8.3; keine Hallfahnen bei Einheiten-/Helden-Barks und positionierten SFX (die Engine räumt).
- **Artefaktprüfung** (metallisches Flattern, Zirpen, Pseudo-Musik, Wortfetzen im SFX, Gesang in Instrumentals): bei Auffälligkeiten fordert die Integration eine Neugenerierung an.
- **Ingame-Abnahme:** `audio-scan`, Hörtest in Mission 1, Pegelbalance gegen Musik und Dialog.

**Eintragsspezifisch** (aus den Fach-Dokumenten; Dateiname/Event bzw. Sprecher → Maßnahme der Integration)

*A – Soundeffekte*

- arrow_fire (`sfx.arrow_fire`): Hochpass 120 Hz; Pfeil-Whoosh darf leicht ausklingen, Gesamtlänge ≤ 0,7 s.
- arrow_impact (`sfx.arrow_impact`): Hochpass 80 Hz; sehr kurzer Ausklang.
- bone_crunch (`sfx.bone_crunch`): Hochpass 100 Hz; Höhen-Crispness erhalten; Einsatz bei Treffern auf Hohle.
- boulder_impact (`sfx.boulder_impact`): Hochpass 40 Hz; Transiente −2 ms; Schuttfall als eigenes Layer nach 0,3 s.
- building_hit (`sfx.building_hit`): Hochpass 40 Hz; Tiefton-Gewicht betonen.
- catapult_launch (`sfx.catapult_launch`): Tieffrequenten Anteil (60–120 Hz) betonen; Ausklang 0,4 s kürzen.
- crossbow_fire (`sfx.crossbow_fire`): Hochpass 80 Hz; tiefer und kürzer als `arrow_fire`, damit Armbrust und Bogen unterscheidbar bleiben.
- horse_charge (`sfx.horse_charge`): Hochpass 40 Hz, Bassbereich der Hufe betonen (+2 dB @ 90 Hz); Ende ausfaden (0,4 s).
- ram_hit (`sfx.ram_hit`): Referenz für Gebäudeschäden; −1 dB gegenüber `wall_hit`.
- shield_block (`sfx.shield_block`): Hochpass 50 Hz; Low-Mid-Punch 120–200 Hz erhalten.
- spear_hit (`sfx.spear_hit`): Hochpass 60 Hz; tiefere Grundfrequenz als `sword_hit` belassen (Unterscheidbarkeit im Gefecht).
- sword_hit (`sfx.sword_hit`): Hochpass 60 Hz, Transientenbetonung +2 dB, Ausklang auf max. 0,5 s kürzen; Referenzdatei für Kampf-SFX: `sword_hit_01`.
- unit_death_hollow (`sfx.unit_death_hollow`): Hochpass 80 Hz; Geisterhauch-Anteil ggf. mit leichtem Reverse-Reverb (Mix 10 %) – Hohle-Signatur.
- unit_death_human (`sfx.unit_death_human`): Stimmenartefakte entfernen/verwerfen; Hochpass 50 Hz.
- wall_collapse (`sfx.wall_collapse`): Hochpass 30 Hz; Tiefbass erhalten; langer natürlicher Ausklang; Limiter −1 dBTP beachten (hoher Pegel).
- wall_hit (`sfx.wall_hit`): Hochpass 50 Hz; kürzer und trockener als `building_hit`; Referenz Kampf-SFX `sword_hit_01`.
- ashen_ward (`sfx.ashen_ward`): Hochpass 40 Hz; Ende sauber ausfaden (Schild-Dauerton kommt ggf. später als eigener Loop).
- bless_spell (`sfx.bless_spell`): Gleiche Lautheit wie `rallying_banner`.
- ember_cleave (`sfx.ember_cleave`): Hochpass 40 Hz; Impact bei ca. 0,3–0,4 s ausrichten (Animations-Sync, mit Gameplay abstimmen); Ausklang 1 s.
- fire_spell (`sfx.fire_spell`): Referenz für alle Feuer-Fähigkeiten; Lautheit wie `ember_cleave`.
- fireball_impact (`sfx.fireball_impact`): kurz halten; sonst überlagert sich der Klang bei Salven.
- ground_slam (`sfx.ground_slam`): Hochpass 35 Hz; Tiefbass nur im Mono-Kern.
- heal_spell (`sfx.heal_spell`): Hochpass 120 Hz; kein hörbares Pad nach 1,5 s (Ausklang kurz).
- hero_revive (`sfx.hero_revive`): Ausklang weich; nicht lauter als `level up`-Stimmen.
- hollow_summon (`sfx.hollow_summon`): Hohle-Signatur (leichter Reverse-Reverb-Vorlauf 150 ms, Notch 1–2 kHz −3 dB); Hochpass 30 Hz.
- rallying_banner (`sfx.rallying_banner`): Hochpass 50 Hz; aufsteigende Energie (Swell) nicht limitieren; Stimmen-/Wortfragmente im Chorschimmer verwerfen.
- reeve_soul_drain (`sfx.reeve_soul_drain`): Hohle-Signatur; Glocken-Variante mit Bossmusik-Tonart (B) abgleichen, falls hörbar verstimmt → Pitch korrigieren.
- shield_spell (`sfx.shield_spell`): kurz; gut hörbar in lauter Schlacht (Mitten betonen).
- war_horn (`sfx.war_horn`): Kein Hall; Hochpass 50 Hz; Ausklang ca. 0,5 s.
- build_complete (`sfx.build_complete`): Hochpass 50 Hz; Glocken-/Hornton tonal mit Musik (D-Moll/D-Dur) verträglich halten.
- build_place (`sfx.build_place`): Hochpass 50 Hz; kurz und satt (UI-nahe Rückmeldung).
- build_progress (`sfx.build_progress`): Hochpass 100 Hz; leise und unaufdringlich, wiederholt sich oft → keine markanten Ausreißer.
- building_destroyed (`sfx.building_destroyed`): Hochpass 30 Hz; Tiefbass erhalten; langer natürlicher Ausklang.
- chapel_bell (`sfx.chapel_bell`): Hochpass 120 Hz; natürlicher Ausklang bis −40 dB, dann ausblenden; ohne Hall (Distanz kommt aus dem Prompt).
- gate_close (`sfx.gate_close`): Hochpass 50 Hz; Transiente betonen (+2 dB), Ausklang max. 0,8 s.
- gate_open (`sfx.gate_open`): Hochpass 50 Hz; Transiente am Ende (Anschlag) erhalten; ohne Hall.
- market_trade (`sfx.market_trade`): Hochpass 150 Hz; hell und kurz; keine Stimmen (Markt-Gemurmel gehört in die Ambience).
- mill_grind (`sfx.mill_grind`): Hochpass 60 Hz; sehr leise mischen (−8 dB im Manifest); Start und Ende ohne Fade-in-Klick schneiden.
- sawmill_saw (`sfx.sawmill_saw`): Hochpass 100 Hz; Mitten glätten, damit sich Wiederholungen nicht ermüdend anhören; keine Stimmen.
- smithy_hammer (`sfx.smithy_hammer`): Hochpass 80 Hz; Ausklang des Amboss-Klingens auf max. 0,6 s; Zyklus-Sound, daher unaufdringlich (Peak ≤ −6 dBFS).
- train_complete (`sfx.train_complete`): Hochpass 60 Hz; deutlich, aber kurz.
- upgrade_complete (`sfx.upgrade_complete`): Hochpass 60 Hz; kein melodisches Motiv (Generator-Musik verwerfen); Lautheit wie `build_complete`.
- item_pickup (`sfx.item_pickup`): Mono; Länge 0,4–0,6 s; Lautheit wie `ui_notify`.
- map_select (`sfx.map_select`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- objective_complete (`sfx.objective_complete`): Tonart möglichst D-Dur (kompatibel mit `victory_sting`); bei tonalen Konflikten Pitch anpassen. Alternative: mit
- objective_new (`sfx.objective_new`): Hochpass 50 Hz; Stereo erlaubt (nicht positioniert).
- potion_drink (`sfx.potion_drink`): Mono; trocken; ohne Stimme (Ausatmen sehr leise).
- talent_learn (`sfx.talent_learn`): Stereo erlaubt; Lautheit wie `objective_complete` minus 3 dB.
- ui_battle_alert (`sfx.ui_battle_alert`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_chronicle_page (`sfx.ui_chronicle_page`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_click (`sfx.ui_click`): auf Transiente schneiden, Fade-out 10–20 ms, Hochpass 200 Hz. Endlängen: `ui_click_01` 0,10 s, `ui_click_02` 0,12 s, `ui_click_03` 0,08 s.
- ui_error (`sfx.ui_error`): Hochpass 120 Hz; klar „nein“, aber nicht nervig. Endlängen: `ui_error_01` 0,3 s, `ui_error_02` 0,3 s, `ui_error_03` 0,35 s.
- ui_event_popup (`sfx.ui_event_popup`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_hover (`sfx.ui_hover`): Hochpass 300 Hz, Tiefpass 10 kHz (angenehm, nicht zischelnd); sehr leise. Endlängen: `ui_hover_01` 0,10 s, `ui_hover_02` 0,08 s, `ui_hover_03` 0,10 s.
- ui_notify (`sfx.ui_notify`): Hochpass 200 Hz; Ausklang ≤ 0,8 s; tonal unauffällig (kein Konflikt mit Musik).
- ui_peace_made (`sfx.ui_peace_made`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_realm_fell (`sfx.ui_realm_fell`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_ruler_changed (`sfx.ui_ruler_changed`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_treaty_signed (`sfx.ui_treaty_signed`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_war_declared (`sfx.ui_war_declared`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- ui_window_open (`sfx.ui_window_open`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- fire_crackle (`sfx.fire_crackle`): Loop-Option nutzen; Nahtstelle prüfen (Crossfade 50–100 ms, falls nötig); keine markanten Einzel-Knacker nahe
- harvest_grain (`sfx.harvest_grain`): Hochpass 150 Hz; weich, leise.
- horn_alarm (`sfx.horn_alarm`): Natürlicher Außenhall erlaubt (Teil des Sounds); mono; Hochpass 60 Hz. Begleitet `char_hal_venn_m01_wave_warning`.
- map_army_move (`sfx.map_army_move`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- map_recruit (`sfx.map_recruit`): Hochpass 150 Hz, Fade-out 20 ms, Peak ≤ −6 dBFS.
- mine_pick (`sfx.mine_pick`): Hochpass 100 Hz.
- stone_quarry (`sfx.stone_quarry`): Hochpass 60 Hz; unterscheidbar von `mine_pick` (tiefer, schwerer).
- thunder (`sfx.thunder`): Hochpass 25 Hz; Stereo-Breite erhalten; Regen kommt aus `ambience_rain`, daher „no rain“.
- wood_chop (`sfx.wood_chop`): Hochpass 80 Hz; Einzelschlag sicherstellen (Mehrfachschläge schneiden).

*B – Stimmen*

- Erzähler (Chronist): nur Standardkette und minimale Wärme; kein Hall.
- Isolde Raventhorn (Heldin): Standardkette Stimme; Hochpass 100 Hz; Präsenz +1,5 dB @ 3,5 kHz.
- Isolde Raventhorn (Heldin): Referenzdatei für alle Stimm-Lautheiten: `voices/heroes/hero_isolde_select_01.ogg`.
- Bruder Hal Venn (Figur und Held `hero_hal_venn`): Standardkette, Hochpass 70 Hz, Wärme (Sättigung ≤ 5 %).
- Torvald Eisenhand (Figur und Held `hero_torvald`): Hochpass 60 Hz, leichte Tiefmitten-Fülle +1 dB @ 150 Hz, im Freien kurzer Berghall szenisch.
- König Aldric III.: Standardkette, Hochpass 70 Hz, im Dialog leichter Thronsaal-Hall (1,6 s, Mix 8 %) nur szenisch.
- Herzogin Mireille Corvane: Standardkette, Hochpass 100 Hz, leichte Luft +1 dB @ 10 kHz.
- Sylaen der Veyl: Veyl-Signatur (Leitfaden 8.3): Luft +2 dB ab 10 kHz, kurzer heller Hall.
- Ophira Saelith-Varo: Standardkette, Hochpass 90 Hz, Wärme leicht.
- Kassian Vale: Standardkette, Nahbesprechungseffekt erhalten (kein starker Hochpass, 70 Hz), sehr leichter Kompressor.
- Die Stimme aus dem Tief: Drei-Schichten-Verfahren (Leitfaden 8.3). Niemals ohne Effekte verwenden.
- Der Fahle Vogt: Hohle-Signatur + Vogt-Ergänzung (Leitfaden 8.3); Ablage unter `voices/units/`.
- Der Hohle König: Leitfaden 8.3 (Deep-Voice-Layer auf Schlüsselwörtern, Thronsaal-Hall).
- Die Wurzelmutter: Chor-Dopplung, Holz-Knarz-Layer, warmer Waldhall (Leitfaden 8.3).
- Ritter: dezente Helm-Nähe optional (Bandpass-Andeutung 120 Hz–9 kHz).

*C – Ambience*

- `ambience/ambience_day_loop.ogg`: Zielloop 90 s. Keine Vogel-Dichte wie im Frühlingswald (Aschmark ist karg), Krähe max. 2× pro 90 s. Hochpass 40 Hz, leichter Tiefpass 14 kHz (weich), Stereo-Breite natürlich.
- `ambience/ambience_night_loop.ogg`: Zielloop 90 s. Details sehr sparsam (4–6 Ereignisse in 90 s); Grillen leiser als im Sommer (kalte Jahreszeit). Hochpass 30 Hz, Tiefpass 12 kHz; leicht dunkler EQ (−2 dB @ 4 kHz) gegenüber Tag.
- `ambience/ambience_rain_loop.ogg`: Zielloop 60 s. Intensität konstant halten (keine An-/Abschwellphasen, sonst pumpt der Loop). Hochpass 60 Hz (Regen ohne Rumpeln), Tiefpass 15 kHz; Stereo breit.
- `ambience/ambience_ash_wind_loop.ogg`: Zielloop 90 s. Böen-Dynamik behalten, aber Lautheitsspitzen ≤ +4 LU über Mittelwert; Böen-Höhepunkte nicht an die Nahtstelle legen. Hochpass 35 Hz; Notch 1–2 kHz −2 dB (hohler Klang); Stereo breit; ggf. sehr leichte Modulation (Chorus Mix 5 %) gegen Musterbildung.
- `ambience/ambience_village_loop.ogg`: Zielloop 90 s. Stimmengemurmel unverständlich und leise (−10 dB); keine erkennbaren Wörter (sonst Generierung ersetzen). Hammerschläge leiser als `sfx/buildings/build_progress_*`, damit Spieleraktionen hörbar bleiben. Hochpass 50 Hz; Mitten um 300 Hz −2 dB (Platz für Dialog); Stereo mittelbreit.
- `ambience/ambience_ruins_loop.ogg`: Zielloop 90 s. Flüstern max. 2× und sehr leise (−14 dB), Wörter dürfen nicht verständlich sein. Glocken-Detail (falls ergänzt) tonal zur Bossmusik (B-Phrygisch) passend oder atonal. Sub-Anteil kontrollieren (kein Brummen < 30 Hz). Hochpass 28 Hz; Hohle-Signatur subtil (Reverse-Reverb auf dem Flüstern, Mix 15 %); Stereo breit, Mitte etwas offener.

*D – Musik:* je Track nur Loop-Naht (Loop-Eignung, Taktzahlen und Loop-Punkte stehen als „Hinweis für die Integration“ im Eintrag), Lautheit und Konvertierung laut „Allgemein“.

---

## A. ElevenLabs Sound Effects (219 Dateien)

**Werkzeug:** ElevenLabs → Sound Effects. Prompt (englisch) einfügen, **Dauer manuell** auf den angegebenen Wert setzen (liegt sie unter der Mindestdauer des Werkzeugs, mit der Mindestdauer generieren), **Prompt Influence** wie angegeben (Startwert; höher = wörtlicher), Loop aus. Je Eintrag mehrere Generierungen erzeugen und die beste **komplette** Generierung unverändert liefern (kein Schneiden, kein Kürzen). Zusatz „no music“ anhängen, falls der Generator Melodien einstreut.

### A.1 `sfx/combat/` – 48 Dateien

#### arrow_fire – Event `sfx.arrow_fire` (3 Dateien)
*−3 dB, 6 Instanzen, Cooldown 40 ms*

- [ ] `sfx/combat/arrow_fire_01.ogg` · Dauer 0,7 s · Prompt Influence 0,6 · Variante: Standard-Langbogen
  > longbow arrow release, taut bowstring twang and fast whoosh, close, dry
- [ ] `sfx/combat/arrow_fire_02.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: kürzer, trockener
  > wooden bow loosing an arrow, short string snap and feathered swish, close, dry
- [ ] `sfx/combat/arrow_fire_03.ogg` · Dauer 0,8 s · Prompt Influence 0,55 · Variante: tieferer Sehnenklang, Pfeifen
  > bow shot, deep bowstring thrum and arrow whistle quickly receding, close, dry

#### arrow_impact – Event `sfx.arrow_impact` (3 Dateien)
*−3 dB, 6 Instanzen*

- [ ] `sfx/combat/arrow_impact_01.ogg` · Dauer 0,5 s · Prompt Influence 0,65 · Variante: Holz, Nachvibrieren
  > arrow hitting a wooden shield, sharp thunk with a short quivering vibration, close, dry
- [ ] `sfx/combat/arrow_impact_02.ogg` · Dauer 0,5 s · Prompt Influence 0,65 · Variante: weich, dumpf
  > arrow piercing flesh through cloth, short dull thud, close, dry
- [ ] `sfx/combat/arrow_impact_03.ogg` · Dauer 0,5 s · Prompt Influence 0,6 · Variante: Boden (Fehlschuss)
  > arrow striking packed earth and small stones, dull thud with a tiny rattle, close, dry

#### bone_crunch – Event `sfx.bone_crunch` (3 Dateien)
*−1 dB, 4 Instanzen*

- [ ] `sfx/combat/bone_crunch_01.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: spröde
  > dry bones crunching and snapping under a heavy blow, brittle crack, close, dry
- [ ] `sfx/combat/bone_crunch_02.ogg` · Dauer 0,7 s · Prompt Influence 0,55 · Variante: Splittern + Streuen
  > skeleton struck by an iron mace, bones shattering and scattering, close, dry
- [ ] `sfx/combat/bone_crunch_03.ogg` · Dauer 0,5 s · Prompt Influence 0,6 · Variante: kurz, hoch
  > skull cracking with dry bone splinters, short and crisp, close, dry

#### boulder_impact – Event `sfx.boulder_impact` (3 Dateien)
*Felsbrocken schlägt ein (Flächenschaden) · 0 dB, 4 Instanzen, Cooldown 150 ms*

- [ ] `sfx/combat/boulder_impact_01.ogg` · Dauer 1,4 s · Prompt Influence 0,6 · Variante: Erdaufschlag, Gesteinssplitter
  > heavy boulder crashing into packed earth, deep thud with scattering debris, mid distance, dry
- [ ] `sfx/combat/boulder_impact_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Holzsplitter + Rumpeln
  > large stone smashing wooden planks and armor, splintering crack with a rolling rumble
- [ ] `sfx/combat/boulder_impact_03.ogg` · Dauer 1,6 s · Prompt Influence 0,6 · Variante: Mauer, Schutt
  > catapult stone impact on stone wall, huge crunch and falling rubble

#### building_hit – Event `sfx.building_hit` (3 Dateien)
*−2 dB, 3 Instanzen*

- [ ] `sfx/combat/building_hit_01.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Holzpalisade
  > heavy blow against a wooden palisade, deep timber thud with splintering, outdoors
- [ ] `sfx/combat/building_hit_02.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Stein, Geröll
  > stone wall struck by a heavy club, dull thump with falling grit and pebbles
- [ ] `sfx/combat/building_hit_03.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Tür, Knarzen
  > wooden door hit hard, sharp crack and creak, dust trickling

#### catapult_launch – Event `sfx.catapult_launch` (3 Dateien)
*Katapult feuert (Wurfarm schnellt los) · -1 dB, 3 Instanzen, Cooldown 300 ms*

- [ ] `sfx/combat/catapult_launch_01.ogg` · Dauer 1,6 s · Prompt Influence 0,6 · Variante: Holzarm, Seilzwirbeln
  > wooden catapult arm snapping forward, deep creaking release, rope twang and a heavy whoosh, close, dry
- [ ] `sfx/combat/catapult_launch_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Balkenschlag, Steinflug
  > siege engine firing, huge timber thump with a rope snap and a rising boulder whoosh, dry
- [ ] `sfx/combat/catapult_launch_03.ogg` · Dauer 1,8 s · Prompt Influence 0,55 · Variante: Ächzen, dann Knall
  > trebuchet-style arm release, groaning wood then sudden crack and whoosh, close

#### crossbow_fire – Event `sfx.crossbow_fire` (3 Dateien)
*Armbrustschuss der Armbrustschützen · -3 dB, 6 Instanzen, Cooldown 40 ms*

- [ ] `sfx/combat/crossbow_fire_01.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Sehnenschlag, Holzstoß
  > heavy crossbow release, sharp string snap with a wooden thump and a short bolt hiss, close, dry
- [ ] `sfx/combat/crossbow_fire_02.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Stahlbogen, Spannklick
  > steel prod crossbow firing, deep twang and ratchet click, close, dry
- [ ] `sfx/combat/crossbow_fire_03.ogg` · Dauer 0,7 s · Prompt Influence 0,55 · Variante: Winde, dickes Schnalzen
  > windlass crossbow discharging, thick string slap and bolt whoosh, close, dry

#### horse_charge – Event `sfx.horse_charge` (3 Dateien)
*0 dB, 2 Instanzen, Cooldown 500 ms*

- [ ] `sfx/combat/horse_charge_01.ogg` · Dauer 3,0 s · Prompt Influence 0,5 · Variante: Einzelpferd, Annäherung
  > heavy armored warhorse charging, thundering hooves on dirt approaching, tack and armor jingling, horse snort, no music
- [ ] `sfx/combat/horse_charge_02.ogg` · Dauer 3,0 s · Prompt Influence 0,5 · Variante: zwei Pferde, Wiehern
  > two armored warhorses galloping hard on packed ground, chainmail rattle, a short whinny, close
- [ ] `sfx/combat/horse_charge_03.ogg` · Dauer 3,0 s · Prompt Influence 0,5 · Variante: Start mit Wiehern
  > cavalry charge start, horse neighs then breaks into a pounding gallop, harness clinking, close

#### ram_hit – Event `sfx.ram_hit` (3 Dateien)
*Rammbock trifft Tor oder Mauer · -1 dB, 3 Instanzen, Cooldown 200 ms*

- [ ] `sfx/combat/ram_hit_01.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Tor, Eichenholz
  > iron-capped battering ram hitting a heavy wooden gate, massive boom with splintering oak, close, dry
- [ ] `sfx/combat/ram_hit_02.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Stein, Mahlen
  > battering ram slamming stone wall, deep concussive thud and grinding crack, dry
- [ ] `sfx/combat/ram_hit_03.ogg` · Dauer 1,3 s · Prompt Influence 0,55 · Variante: Knarren, dann Aufprall
  > large log ram swinging and hitting timber barricade, creak then heavy impact

#### shield_block – Event `sfx.shield_block` (3 Dateien)
*−2 dB, 4 Instanzen*

- [ ] `sfx/combat/shield_block_01.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Holz + Eisenrand
  > sword blocked by a wooden round shield with an iron rim, heavy wooden thud and metal clank, close, dry
- [ ] `sfx/combat/shield_block_02.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: tiefer, hohler
  > heavy blow against a large wooden kite shield, deep hollow wooden impact, close, dry
- [ ] `sfx/combat/shield_block_03.ogg` · Dauer 0,7 s · Prompt Influence 0,55 · Variante: metallischer
  > mace striking an iron shield boss, loud metallic bang with wooden body resonance, close, dry

#### spear_hit – Event `sfx.spear_hit` (3 Dateien)
*0 dB, 6 Instanzen*

- [ ] `sfx/combat/spear_hit_01.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Holzschaft hörbar
  > spear thrust piercing leather armor, wooden shaft thud with a sharp stab, close, dry
- [ ] `sfx/combat/spear_hit_02.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Ketten + Stich
  > iron spearhead stabbing through chainmail into a body, puncture with metal rattle, close, dry
- [ ] `sfx/combat/spear_hit_03.ogg` · Dauer 0,7 s · Prompt Influence 0,55 · Variante: zweiteilig: Kratzer + Stoß
  > spear point glancing off a shield rim then piercing, quick scrape and heavy thump, close, dry

#### sword_hit – Event `sfx.sword_hit` (3 Dateien)
*Manifest 0 dB, max. 6 Instanzen, Cooldown 60 ms*

- [ ] `sfx/combat/sword_hit_01.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: hell, Kettenrasseln
  > steel longsword striking chainmail, sharp metallic clang with a short ring, close, dry, no reverb
- [ ] `sfx/combat/sword_hit_02.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: dumpfer, fleischiger
  > steel sword slashing into leather armor and flesh, meaty thud with a metallic edge, close, dry
- [ ] `sfx/combat/sword_hit_03.ogg` · Dauer 0,6 s · Prompt Influence 0,6 · Variante: Plattenklang mit Kratzer
  > sword blade hitting plate armor, bright metallic impact with a short scrape, close, dry

#### unit_death_hollow – Event `sfx.unit_death_hollow` (3 Dateien)
*−1 dB, 3 Instanzen*

- [ ] `sfx/combat/unit_death_hollow_01.ogg` · Dauer 1,8 s · Prompt Influence 0,5 · Variante: Staub + Zischen
  > undead corpse collapsing into dust, dry bones clattering, ashy hiss of escaping air
- [ ] `sfx/combat/unit_death_hollow_02.ogg` · Dauer 1,8 s · Prompt Influence 0,45 · Variante: Geisterhauch
  > skeleton falling apart, bones rattling onto stone, faint ghostly whisper dissipating
- [ ] `sfx/combat/unit_death_hollow_03.ogg` · Dauer 1,8 s · Prompt Influence 0,5 · Variante: Aschewolke
  > rotten undead body crumbling, dry crunch, puff of ash, eerie breath fading out

#### unit_death_human – Event `sfx.unit_death_human` (3 Dateien)
*−1 dB, 3 Instanzen*

- [ ] `sfx/combat/unit_death_human_01.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: Kettenhemd + Schwert
  > armored soldier collapsing to the ground, chainmail rattle, body thud on dirt, sword clattering, no voice
- [ ] `sfx/combat/unit_death_human_02.ogg` · Dauer 1,4 s · Prompt Influence 0,55 · Variante: weich, Schlamm
  > body falling onto muddy ground, heavy thud and cloth rustle, wooden spear dropping, no voice
- [ ] `sfx/combat/unit_death_human_03.ogg` · Dauer 1,6 s · Prompt Influence 0,55 · Variante: schwer, metallisch
  > knight in plate armor falling, heavy metallic crash on stone, no voice
- *Hinweis:* ohne Stimme – die Todesstimme kommt aus den `voice.*.death`-Events. Nur Körper/Ausrüstung.

#### wall_collapse – Event `sfx.wall_collapse` (3 Dateien)
*Mauer, Tor oder Mauerturm zerstört · 0 dB, 2 Instanzen, Cooldown 300 ms*

- [ ] `sfx/combat/wall_collapse_01.ogg` · Dauer 5,0 s · Prompt Influence 0,5 · Variante: Mauerstück bricht
  > section of castle wall collapsing, thunderous stone avalanche, long rumble
- [ ] `sfx/combat/wall_collapse_02.ogg` · Dauer 5,0 s · Prompt Influence 0,5 · Variante: Turm stürzt ein
  > stone tower crumbling, huge crashing blocks and rolling rubble with a drawn-out dust-settling rumble
- [ ] `sfx/combat/wall_collapse_03.ogg` · Dauer 4,5 s · Prompt Influence 0,5 · Variante: Torhaus bricht
  > wooden gate and gatehouse breaking apart, massive timber crack then collapsing stones and debris

#### wall_hit – Event `sfx.wall_hit` (3 Dateien)
*Treffer auf Mauer, Tor oder Mauerturm · −2 dB, 4 Instanzen, Cooldown 100 ms*

- [ ] `sfx/combat/wall_hit_01.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Stein, dumpf
  > weapon striking a thick stone wall, dull heavy thud with grit, close, dry
- [ ] `sfx/combat/wall_hit_02.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Holztor, Splittern
  > axe chopping into a heavy wooden gate, deep woody thunk with splintering, close, dry
- [ ] `sfx/combat/wall_hit_03.ogg` · Dauer 0,7 s · Prompt Influence 0,6 · Variante: Eisen auf Stein
  > iron sword clanging against a stone tower wall, short metallic strike and falling grit, close, dry

### A.2 `sfx/magic/` – 39 Dateien

#### ashen_ward – Event `sfx.ashen_ward` (3 Dateien)
*Aschenschild · 0 dB, 2 Instanzen*

- [ ] `sfx/magic/ashen_ward_01.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: Kristallschimmer
  > magical shield forming, swirling ash and wind, low resonant hum, crystalline shimmer
- [ ] `sfx/magic/ashen_ward_02.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: wärmer, tiefer
  > protective barrier of ash and embers, whooshing swirl and deep warm drone
- [ ] `sfx/magic/ashen_ward_03.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: glasig, ruhiger Ausklang
  > grey magic shield activating, soft crackle of embers and glassy resonance settling

#### bless_spell – Event `sfx.bless_spell` (3 Dateien)
*Segen und Stärkungszauber (Segen der Flamme, Glaubensfeuer, Schlachtruf) · -1 dB, 2 Instanzen*

- [ ] `sfx/magic/bless_spell_01.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: goldener Glockenton
  > blessing spell, bright rising shimmer with a soft golden bell tone, magical aura, dry
- [ ] `sfx/magic/bless_spell_02.ogg` · Dauer 1,6 s · Prompt Influence 0,55 · Variante: Pulswelle
  > rallying magic pulse, warm swelling whoosh with a distant horn-like swell, no melody
- [ ] `sfx/magic/bless_spell_03.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: warme Glut, Chime
  > fiery blessing, crackling warmth rising into a shining chime

#### ember_cleave – Event `sfx.ember_cleave` (3 Dateien)
*Isoldes Glutspalter (Flächenschaden) · 0 dB, 2 Instanzen*

- [ ] `sfx/magic/ember_cleave_01.ogg` · Dauer 1,5 s · Prompt Influence 0,5 · Variante: ausgewogen
  > fiery sword swing, whoosh of flame trailing the blade, burst of embers exploding on impact, crackling sparks
- [ ] `sfx/magic/ember_cleave_02.ogg` · Dauer 1,6 s · Prompt Influence 0,5 · Variante: tiefer, wuchtiger
  > heavy flaming blade cleave, roaring arc of fire, deep ground impact and spray of embers
- [ ] `sfx/magic/ember_cleave_03.ogg` · Dauer 1,4 s · Prompt Influence 0,5 · Variante: heller, funkelnder
  > magical ember slash, sharp fire whoosh, crackling explosion of glowing sparks fading out

#### fire_spell – Event `sfx.fire_spell` (3 Dateien)
*Feuerzauber (Feuerball, Flammenwoge, Heilige Flamme, Phönixhieb, Feuersäule) · 0 dB, 3 Instanzen*

- [ ] `sfx/magic/fire_spell_01.ogg` · Dauer 1,4 s · Prompt Influence 0,6 · Variante: Aufsteigende Flamme
  > fireball spell cast, rising whoosh of flame igniting with crackling embers and a hot burst, close, dry
- [ ] `sfx/magic/fire_spell_02.ogg` · Dauer 1,6 s · Prompt Influence 0,6 · Variante: Feuersäule, Bass
  > column of fire erupting from the ground, roaring blaze with a deep bass thump
- [ ] `sfx/magic/fire_spell_03.ogg` · Dauer 1,4 s · Prompt Influence 0,55 · Variante: Flammenring, Funkeln
  > ring of flame expanding outward, sharp ignite and sizzling crackle, magical shimmer

#### fireball_impact – Event `sfx.fireball_impact` (3 Dateien)
*Feuerball oder Magiegeschoss schlägt ein · -1 dB, 4 Instanzen, Cooldown 100 ms*

- [ ] `sfx/magic/fireball_impact_01.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Verpuffung, Funken
  > fireball exploding on impact, dull whump and crackling fire burst with scattering sparks, mid distance
- [ ] `sfx/magic/fireball_impact_02.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: kurzes Auflodern
  > magical fire burst on ground, quick roar and ember pops, dry
- [ ] `sfx/magic/fireball_impact_03.ogg` · Dauer 0,9 s · Prompt Influence 0,6 · Variante: Zischen + Knall
  > flaming bolt impact, hot hiss and a sharp pop, close

#### ground_slam – Event `sfx.ground_slam` (3 Dateien)
*Erdspalter und Zorn der Ahnen · 0 dB, 2 Instanzen, Cooldown 100 ms*

- [ ] `sfx/magic/ground_slam_01.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Hammerschlag, Schockwelle
  > massive hammer striking the ground, deep earth-shaking boom with rubble and a shock wave, close
- [ ] `sfx/magic/ground_slam_02.ogg` · Dauer 1,6 s · Prompt Influence 0,55 · Variante: Donner + Grollen
  > ancestral fury burst, thunderous impact with a layered guttural roar
- [ ] `sfx/magic/ground_slam_03.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Boden bricht auf
  > ground cracking open under a heavy blow, loud crunch and rolling rumble

#### heal_spell – Event `sfx.heal_spell` (3 Dateien)
*Heilzauber (Heilendes Licht, Flammenheilung, Neuerwachen) · -2 dB, 3 Instanzen*

- [ ] `sfx/magic/heal_spell_01.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: warmer Glockenklang
  > warm healing chime swelling into a soft glowing shimmer, gentle airy choir pad, no music, close
- [ ] `sfx/magic/heal_spell_02.ogg` · Dauer 1,6 s · Prompt Influence 0,55 · Variante: Lichtschimmer
  > holy light spell, soft bright sparkle sweep with a low humming glow, magical
- [ ] `sfx/magic/heal_spell_03.ogg` · Dauer 1,5 s · Prompt Influence 0,5 · Variante: sanfte Welle
  > restorative magic wave, gentle rising tone with shimmering particles, dry

#### hero_revive – Event `sfx.hero_revive` (3 Dateien)
*Wiederbelebung eines Helden · -1 dB, 1 Instanz*

- [ ] `sfx/magic/hero_revive_01.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Herzschlag + Glut
  > hero revived by flame, sudden rush of air into a glowing resurgence swell with a deep heartbeat thump, dry
- [ ] `sfx/magic/hero_revive_02.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Feuer entfacht erneut
  > holy fire reigniting, rising crackle then a bright ring, close
- [ ] `sfx/magic/hero_revive_03.ogg` · Dauer 2,2 s · Prompt Influence 0,5 · Variante: Licht erblüht
  > resurrection light, soft low drone blossoming into a bright shimmer, no music

#### hollow_summon – Event `sfx.hollow_summon` (3 Dateien)
*Beschwörung der Hohlen · −1 dB, 2 Instanzen*

- [ ] `sfx/magic/hollow_summon_01.ogg` · Dauer 3,0 s · Prompt Influence 0,5 · Variante: Erde + Knochen
  > undead summoning, earth cracking open, bones scraping out of the soil, eerie whispering wind
- [ ] `sfx/magic/hollow_summon_02.ogg` · Dauer 3,0 s · Prompt Influence 0,45 · Variante: Drone + Stöhnen
  > necromantic ritual, low reversed drone, ghostly moans rising, dirt and gravel shifting
- [ ] `sfx/magic/hollow_summon_03.ogg` · Dauer 3,0 s · Prompt Influence 0,5 · Variante: Sub-Boom
  > dark magic summon, deep sub boom, hissing ash, skeletal hands clawing out of the ground

#### rallying_banner – Event `sfx.rallying_banner` (3 Dateien)
*Sammelbanner · 0 dB, 1 Instanz*

- [ ] `sfx/magic/rallying_banner_01.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: mit fernem Horn
  > war banner planted into the ground with a heavy thud, cloth snapping in the wind, warm magical shimmer rising, distant war horn
- [ ] `sfx/magic/rallying_banner_02.ogg` · Dauer 2,5 s · Prompt Influence 0,45 · Variante: Chorschimmer
  > heavy flag unfurling with a loud flap, bright metallic chime and rising warm choir-like glow, no words
- [ ] `sfx/magic/rallying_banner_03.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: Feuer-Akzent
  > banner pole slammed into earth, fabric whipping, soft fire whoosh and bright magical swell

#### reeve_soul_drain – Event `sfx.reeve_soul_drain` (3 Dateien)
*Seelenentzug des Fahlen Vogts · 0 dB, 2 Instanzen*

- [ ] `sfx/magic/reeve_soul_drain_01.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: Sog + Klagen
  > soul being drained, ghostly wailing pulled away, deep vacuum suction and cold wind
- [ ] `sfx/magic/reeve_soul_drain_02.ogg` · Dauer 2,5 s · Prompt Influence 0,45 · Variante: Glocke (Vogt-Motiv)
  > life force siphon, reversed whispers, rising eerie hiss, a single faint funeral bell toll
- [ ] `sfx/magic/reeve_soul_drain_03.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: eisig
  > spectral energy drain, breathy howl sucked inward, icy crackle, low drone

#### shield_spell – Event `sfx.shield_spell` (3 Dateien)
*Schutzzauber und Schildwall (Schildwall, Eiserner Schwur, Steinhaut, Flammenaegis) · -2 dB, 2 Instanzen*

- [ ] `sfx/magic/shield_spell_01.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: metallisches Klingen
  > magical ward forming, metallic ringing shimmer with a low solid thud, dry
- [ ] `sfx/magic/shield_spell_02.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Stein knirscht
  > stone skin spell, grinding crunch with a low resonant hum, short
- [ ] `sfx/magic/shield_spell_03.ogg` · Dauer 1,3 s · Prompt Influence 0,55 · Variante: Feuerhaut
  > flaming aegis igniting around a warrior, whoosh and soft crackle with a clang

#### war_horn – Event `sfx.war_horn` (3 Dateien)
*Kriegshörner und Ahnenruf (Torvald) · 0 dB, 2 Instanzen*

- [ ] `sfx/magic/war_horn_01.ogg` · Dauer 2,2 s · Prompt Influence 0,6 · Variante: tiefes Horn
  > deep nordic war horn blast, long low brass-like bellow with rough breathy overtones, outdoors
- [ ] `sfx/magic/war_horn_02.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: zwei Hörner
  > two war horns sounding together, powerful droning call with slight dissonance, dry
- [ ] `sfx/magic/war_horn_03.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Ruf + Horn
  > ancestral roar mixed with a horn call, guttural shout over low drone

### A.3 `sfx/buildings/` – 39 Dateien

#### build_complete – Event `sfx.build_complete` (3 Dateien)
*0 dB*

- [ ] `sfx/buildings/build_complete_01.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: Glöckchen
  > medieval construction finished, final hammer blow, wooden creak settling, small bronze bell chime
- [ ] `sfx/buildings/build_complete_02.ogg` · Dauer 1,5 s · Prompt Influence 0,5 · Variante: Hornton
  > timber building completed, beams settling with a satisfying thud, short warm horn note
- [ ] `sfx/buildings/build_complete_03.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: Steinvariante
  > stone structure completed, last block thud, dust settling, bright clear chime

#### build_place – Event `sfx.build_place` (3 Dateien)
*−2 dB*

- [ ] `sfx/buildings/build_place_01.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Holz
  > wooden building foundation placed on the ground, heavy timber thud and dirt crunch
- [ ] `sfx/buildings/build_place_02.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Stein
  > large stone block set down on earth, solid thump with gravel
- [ ] `sfx/buildings/build_place_03.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Balkenbündel
  > bundle of wooden beams dropped onto a pile, hollow knock and rustle

#### build_progress – Event `sfx.build_progress` (3 Dateien)
*−6 dB, 3 Instanzen, Cooldown 300 ms*

- [ ] `sfx/buildings/build_progress_01.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Hämmern
  > hammer driving iron nails into wood, three quick knocks, medieval carpentry, outdoors
- [ ] `sfx/buildings/build_progress_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Sägen
  > hand saw cutting a wooden plank, two short strokes, outdoors
- [ ] `sfx/buildings/build_progress_03.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Steinmetz
  > chisel tapping stone, rhythmic clinks, mason at work, outdoors

#### building_destroyed – Event `sfx.building_destroyed` (3 Dateien)
*0 dB*

- [ ] `sfx/buildings/building_destroyed_01.ogg` · Dauer 3,5 s · Prompt Influence 0,5 · Variante: Holz + Feuer
  > wooden building collapsing, timbers cracking and crashing down, fire roaring, debris falling
- [ ] `sfx/buildings/building_destroyed_02.ogg` · Dauer 3,5 s · Prompt Influence 0,5 · Variante: Stein
  > stone tower crumbling, heavy rocks tumbling, rubble and dust
- [ ] `sfx/buildings/building_destroyed_03.ogg` · Dauer 3,5 s · Prompt Influence 0,5 · Variante: brennendes Haus
  > burning house caving in, splintering beams, flames whooshing, embers crackling

#### chapel_bell – Event `sfx.chapel_bell` (3 Dateien)
*Unruhe in der Bevölkerung · −3 dB, 1 Instanz, Cooldown 3000 ms*

- [ ] `sfx/buildings/chapel_bell_01.ogg` · Dauer 5,0 s · Prompt Influence 0,55 · Variante: drei Schläge, fern
  > small village chapel bell tolling three times, distant
- [ ] `sfx/buildings/chapel_bell_02.ogg` · Dauer 5,0 s · Prompt Influence 0,55 · Variante: gesprungene Glocke
  > single cracked chapel bell ringing slowly with a long metallic decay, eerie, outdoors
- [ ] `sfx/buildings/chapel_bell_03.ogg` · Dauer 4,0 s · Prompt Influence 0,5 · Variante: unruhiges Läuten
  > hand-rung small bell clanging unevenly in alarm, outdoors, slightly distant

#### gate_close – Event `sfx.gate_close` (3 Dateien)
*Tor geschlossen · −2 dB, 2 Instanzen, Cooldown 200 ms*

- [ ] `sfx/buildings/gate_close_01.ogg` · Dauer 2,5 s · Prompt Influence 0,55 · Variante: Riegel fällt
  > heavy wooden gate slamming shut, iron bar dropped into place
- [ ] `sfx/buildings/gate_close_02.ogg` · Dauer 2,5 s · Prompt Influence 0,55 · Variante: lauter Knall
  > large oak gate swinging shut with a loud boom and a short rattle of chains, close, dry
- [ ] `sfx/buildings/gate_close_03.ogg` · Dauer 2,2 s · Prompt Influence 0,5 · Variante: Fallgitter stürzt
  > portcullis dropping fast, chain rattling then a heavy iron slam into the stone floor, close, dry

#### gate_open – Event `sfx.gate_open` (3 Dateien)
*Tor geöffnet · −2 dB, 2 Instanzen, Cooldown 200 ms*

- [ ] `sfx/buildings/gate_open_01.ogg` · Dauer 3,5 s · Prompt Influence 0,55 · Variante: Kette und Winde
  > heavy wooden gate opening, chains and winch cranking, creak
- [ ] `sfx/buildings/gate_open_02.ogg` · Dauer 3,5 s · Prompt Influence 0,55 · Variante: Angeln, dumpfer Anschlag
  > large oak castle gate swinging open on iron hinges, long groaning creak and a dull wooden thud at the end
- [ ] `sfx/buildings/gate_open_03.ogg` · Dauer 3,5 s · Prompt Influence 0,5 · Variante: Fallgitter
  > portcullis raised by a rattling chain, metal grinding upward then a clank as it locks, close, dry

#### market_trade – Event `sfx.market_trade` (3 Dateien)
*Handel am Markt · −3 dB, 2 Instanzen, Cooldown 120 ms*

- [ ] `sfx/buildings/market_trade_01.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Münzen zählen
  > coins counted into a leather purse, soft metallic jingle, wooden counter, close, dry
- [ ] `sfx/buildings/market_trade_02.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Münzen fallen
  > handful of coins dropped on a wooden table, bright clinking scatter and a soft thud, close, dry
- [ ] `sfx/buildings/market_trade_03.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Beutel auf Tresen
  > merchant slapping a coin purse onto a counter, heavy chink then a quick coin slide, close, dry

#### mill_grind – Event `sfx.mill_grind` (3 Dateien)
*Zyklus Mühle, Bäckerei, Brauerei · −8 dB, 2 Instanzen, Cooldown 400 ms*

- [ ] `sfx/buildings/mill_grind_01.ogg` · Dauer 2,5 s · Prompt Influence 0,55 · Variante: Mahlstein, Knarren
  > millstone grinding grain, wooden creak, soft rumble, close, dry
- [ ] `sfx/buildings/mill_grind_02.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: tiefes Rumpeln, Rieseln
  > heavy stone wheel turning slowly with a deep rumble and fine grain trickling, close, dry
- [ ] `sfx/buildings/mill_grind_03.ogg` · Dauer 2,5 s · Prompt Influence 0,5 · Variante: Holzgetriebe
  > wooden gears and axle creaking under load with a faint stone scrape, rhythmic, close, dry

#### sawmill_saw – Event `sfx.sawmill_saw` (3 Dateien)
*Zyklus Sägewerk, Holzfällerlager · −6 dB, 3 Instanzen, Cooldown 300 ms*

- [ ] `sfx/buildings/sawmill_saw_01.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: gleichmäßiger Zug
  > hand saw cutting through a log, rhythmic rasp, wood chips falling, close, dry
- [ ] `sfx/buildings/sawmill_saw_02.ogg` · Dauer 2,2 s · Prompt Influence 0,55 · Variante: langsamer, tiefer
  > two-man crosscut saw biting into thick timber, slow steady rasping strokes, sawdust hissing, close, dry
- [ ] `sfx/buildings/sawmill_saw_03.ogg` · Dauer 1,8 s · Prompt Influence 0,5 · Variante: kurz, Brett fällt
  > wood saw working through a plank, quick short strokes with a final snap as the board drops, close, dry

#### smithy_hammer – Event `sfx.smithy_hammer` (3 Dateien)
*Produktionszyklus Schmiede, Rüstkammer, Schmelzhütte · −4 dB, 3 Instanzen, Cooldown 250 ms*

- [ ] `sfx/buildings/smithy_hammer_01.ogg` · Dauer 1,5 s · Prompt Influence 0,55 · Variante: zwei Schläge, Abschrecken
  > blacksmith hammering on anvil, two ringing strikes, hiss of quenching, close, dry
- [ ] `sfx/buildings/smithy_hammer_02.ogg` · Dauer 1,2 s · Prompt Influence 0,55 · Variante: ein schwerer Schlag
  > heavy hammer striking hot iron on an anvil, single deep clang with a short ring and sparks, close, dry
- [ ] `sfx/buildings/smithy_hammer_03.ogg` · Dauer 1,6 s · Prompt Influence 0,55 · Variante: Klopfen, dann Schlag
  > smith tapping and shaping a blade on an anvil, three quick light taps then one strong blow, close, dry

#### train_complete – Event `sfx.train_complete` (3 Dateien)
*−2 dB*

- [ ] `sfx/buildings/train_complete_01.ogg` · Dauer 1,2 s · Prompt Influence 0,55 · Variante: Schwert + Trommel
  > soldier ready for duty, armor clank and sword drawn from scabbard, single drum hit
- [ ] `sfx/buildings/train_complete_02.ogg` · Dauer 1,2 s · Prompt Influence 0,55 · Variante: Antreten
  > boots stepping to attention on wood, chainmail rattle, metallic ring
- [ ] `sfx/buildings/train_complete_03.ogg` · Dauer 1,2 s · Prompt Influence 0,5 · Variante: Tür + Horn
  > barracks door opening, boots on wooden floor, short horn blast

#### upgrade_complete – Event `sfx.upgrade_complete` (3 Dateien)
*Haus aufgestiegen, Burg-Ausbau fertig · −2 dB, 2 Instanzen, Cooldown 0 ms*

- [ ] `sfx/buildings/upgrade_complete_01.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: Hammer + Horn
  > building upgraded, sturdy hammer blows, metal fittings clank, short horn
- [ ] `sfx/buildings/upgrade_complete_02.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: drei Schläge, Amboss
  > construction finished, three solid hammer knocks on timber then a bright anvil ring, no music
- [ ] `sfx/buildings/upgrade_complete_03.ogg` · Dauer 2,2 s · Prompt Influence 0,5 · Variante: Stein setzen, Blech-Schwell
  > stonework completed, heavy stone block set in place with a satisfying thud and a short rising brass swell

### A.4 `sfx/ui/` – 57 Dateien

#### item_pickup – Event `sfx.item_pickup` (3 Dateien)
*Gegenstand aufgenommen · -4 dB, 2 Instanzen, Cooldown 100 ms*

- [ ] `sfx/ui/item_pickup_01.ogg` · Dauer 0,5 s · Prompt Influence 0,6 · Variante: Beutel, Münzen
  > small leather pouch clink, coins and metal jingle, quick pickup, close, dry
- [ ] `sfx/ui/item_pickup_02.ogg` · Dauer 0,5 s · Prompt Influence 0,6 · Variante: Metallkratzer
  > pick up a sword from the ground, short metal scrape and clink
- [ ] `sfx/ui/item_pickup_03.ogg` · Dauer 0,6 s · Prompt Influence 0,55 · Variante: kurzer Chime
  > magical item collected, brief bright chime with a soft whoosh

#### map_select – Event `sfx.map_select` (3 Dateien)
*−9 dB, Cooldown 60 ms, 2 Instanzen*

- [ ] `sfx/ui/map_select_01.ogg` · Dauer 0,2 s · Prompt Influence 0,6 · Variante: Figur
  > a wooden game token set down on a parchment map, soft tap, very short
- [ ] `sfx/ui/map_select_02.ogg` · Dauer 0,2 s · Prompt Influence 0,6 · Variante: Kompass
  > a small brass compass tick on a map table, very short
- [ ] `sfx/ui/map_select_03.ogg` · Dauer 0,2 s · Prompt Influence 0,6 · Variante: Pergament
  > finger tapping on rough parchment with a faint ember crackle, very short

#### objective_complete – Event `sfx.objective_complete` (3 Dateien)
*0 dB*

- [ ] `sfx/ui/objective_complete_01.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: Fanfare
  > short triumphant medieval brass flourish with a bell chime, two seconds
- [ ] `sfx/ui/objective_complete_02.ogg` · Dauer 2,0 s · Prompt Influence 0,45 · Variante: Chor
  > warm wordless choir swell with a bright chime, uplifting, no lyrics
- [ ] `sfx/ui/objective_complete_03.ogg` · Dauer 2,0 s · Prompt Influence 0,5 · Variante: Schreiben + Glocke
  > quill stroke and seal stamp followed by an uplifting bell ring

#### objective_new – Event `sfx.objective_new` (3 Dateien)
*−1 dB*

- [ ] `sfx/ui/objective_new_01.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Pergament + Trommel
  > parchment scroll unrolling with a quill scratch and a single low drum hit
- [ ] `sfx/ui/objective_new_02.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Trommel + Horn
  > deep war drum hit followed by one resonant low horn note, no melody
- [ ] `sfx/ui/objective_new_03.ogg` · Dauer 2,0 s · Prompt Influence 0,55 · Variante: Siegel
  > red wax seal breaking, paper rustle, soft bell tone

#### potion_drink – Event `sfx.potion_drink` (3 Dateien)
*Trank getrunken · -3 dB, 1 Instanz, Cooldown 300 ms*

- [ ] `sfx/ui/potion_drink_01.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Korken, Schlucken
  > drinking a potion, cork pop then gulping swallow and a soft glassy clink, close, dry
- [ ] `sfx/ui/potion_drink_02.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Schluck, Ausatmen
  > potion vial uncorked and quaffed, short liquid gulp and exhale, close
- [ ] `sfx/ui/potion_drink_03.ogg` · Dauer 1,1 s · Prompt Influence 0,55 · Variante: Blubbern, Schimmer
  > alchemical elixir drunk, bubbling then a quick warm shimmer

#### talent_learn – Event `sfx.talent_learn` (3 Dateien)
*Talentpunkt eingesetzt und Stufenaufstieg · -3 dB, 1 Instanz, Cooldown 200 ms*

- [ ] `sfx/ui/talent_learn_01.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Glocke, aufsteigend
  > level up ding, warm rising bell with a soft golden sparkle, short, no music
- [ ] `sfx/ui/talent_learn_02.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Chime obenauf
  > skill learned, quick glowing swell with a crisp chime on top, dry
- [ ] `sfx/ui/talent_learn_03.ogg` · Dauer 1,1 s · Prompt Influence 0,5 · Variante: Hum + Funkeln
  > hero empowered, deep warm hum rising into a bright flicker, brief

#### ui_battle_alert – Event `sfx.ui_battle_alert` (3 Dateien)
*0 dB, Cooldown 1500 ms*

- [ ] `sfx/ui/ui_battle_alert_01.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Alarmhorn
  > urgent war horn two short blasts and a distant clash of swords, medieval alarm
- [ ] `sfx/ui/ui_battle_alert_02.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Trommelwirbel
  > fast war drum roll building up to a single heavy hit, tense
- [ ] `sfx/ui/ui_battle_alert_03.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Sturmglocke
  > rapid bell alarm with metallic armour rattle, tense, short

#### ui_chronicle_page – Event `sfx.ui_chronicle_page` (3 Dateien)
*−4 dB, Cooldown 200 ms*

- [ ] `sfx/ui/ui_chronicle_page_01.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: schwere Seite
  > a heavy old book page being turned slowly, thick parchment, quiet room
- [ ] `sfx/ui/ui_chronicle_page_02.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Leder
  > thin parchment page flip with a light leather bookbinding creak
- [ ] `sfx/ui/ui_chronicle_page_03.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Schreibfeder
  > quill scratching a few strokes on parchment, close, dry

#### ui_click – Event `sfx.ui_click` (3 Dateien)
*−6 dB, Cooldown 30 ms*

- [ ] `sfx/ui/ui_click_01.ogg` · Dauer 0,5 s · Prompt Influence 0,75 · Variante: Holz
  > soft wooden UI click, small knock on oak, crisp and very short, no reverb
- [ ] `sfx/ui/ui_click_02.ogg` · Dauer 0,5 s · Prompt Influence 0,75 · Variante: Metall + Papier
  > subtle metallic button click with a faint parchment rustle, very short, no reverb
- [ ] `sfx/ui/ui_click_03.ogg` · Dauer 0,5 s · Prompt Influence 0,75 · Variante: gedämpft
  > short muted leather and wood tick, tactile, very short, no reverb

#### ui_error – Event `sfx.ui_error` (3 Dateien)
*−4 dB, Cooldown 200 ms*

- [ ] `sfx/ui/ui_error_01.ogg` · Dauer 0,6 s · Prompt Influence 0,7 · Variante: Holz dumpf
  > dull low wooden thunk, muted negative feedback sound, short
- [ ] `sfx/ui/ui_error_02.ogg` · Dauer 0,6 s · Prompt Influence 0,7 · Variante: Metall dissonant
  > short low dissonant metallic clank, dry, short
- [ ] `sfx/ui/ui_error_03.ogg` · Dauer 0,6 s · Prompt Influence 0,7 · Variante: Doppelklopfen
  > soft double knock on wood, low pitch, denied feeling, short

#### ui_event_popup – Event `sfx.ui_event_popup` (3 Dateien)
*−3 dB, Cooldown 300 ms*

- [ ] `sfx/ui/ui_event_popup_01.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Pergament + Glocke
  > parchment scroll unrolling with a soft wax seal crack and a single low bell tone, medieval, no melody
- [ ] `sfx/ui/ui_event_popup_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Tisch + Feder
  > heavy parchment slap on a wooden table followed by a quiet quill scratch and a distant bell, short
- [ ] `sfx/ui/ui_event_popup_03.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Bote
  > old courier horn note far away then a rustle of paper, subtle, atmospheric

#### ui_hover – Event `sfx.ui_hover` (3 Dateien)
*−12 dB*

- [ ] `sfx/ui/ui_hover_01.ogg` · Dauer 0,5 s · Prompt Influence 0,7 · Variante: Pergament
  > very soft parchment swish, airy and delicate, extremely short
- [ ] `sfx/ui/ui_hover_02.ogg` · Dauer 0,5 s · Prompt Influence 0,7 · Variante: Glutfunke
  > tiny quiet ember crackle tick, single spark, extremely short
- [ ] `sfx/ui/ui_hover_03.ogg` · Dauer 0,5 s · Prompt Influence 0,7 · Variante: Stoff
  > light cloth brush, extremely short soft whoosh

#### ui_notify – Event `sfx.ui_notify` (3 Dateien)
*−3 dB, Cooldown 200 ms*

- [ ] `sfx/ui/ui_notify_01.ogg` · Dauer 1,0 s · Prompt Influence 0,65 · Variante: Bronzeglocke
  > small bronze hand bell ringing once, clear and short, medieval
- [ ] `sfx/ui/ui_notify_02.ogg` · Dauer 1,0 s · Prompt Influence 0,65 · Variante: weicher Gong
  > soft warm metallic chime with gentle decay
- [ ] `sfx/ui/ui_notify_03.ogg` · Dauer 1,0 s · Prompt Influence 0,65 · Variante: Doppelschlag
  > light bell, two quick strikes, gentle

#### ui_peace_made – Event `sfx.ui_peace_made` (3 Dateien)
*−1 dB, Cooldown 1000 ms*

- [ ] `sfx/ui/ui_peace_made_01.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Schwerter in Scheiden
  > two sword blades sheathed one after another, a soft warm bell chime and a quiet sigh of wind, relief
- [ ] `sfx/ui/ui_peace_made_02.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Fanfare
  > gentle brass fanfare of three notes with a warm bell, medieval peace treaty, short
- [ ] `sfx/ui/ui_peace_made_03.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Siegel + Harfe
  > wax seal pressed on parchment, quill stroke, soft harp arpeggio, calm

#### ui_realm_fell – Event `sfx.ui_realm_fell` (3 Dateien)
*−1 dB, Cooldown 1500 ms*

- [ ] `sfx/ui/ui_realm_fell_01.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Krone fällt
  > a crown falling and rolling on a stone floor, then a low mournful horn, dark fantasy
- [ ] `sfx/ui/ui_realm_fell_02.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Banner
  > a heavy banner being torn and falling, low drone and distant bell, grim
- [ ] `sfx/ui/ui_realm_fell_03.ogg` · Dauer 3,5 s · Prompt Influence 0,6 · Variante: Trauermarsch
  > slow funeral drum with a deep brass note fading out, solemn

#### ui_ruler_changed – Event `sfx.ui_ruler_changed` (3 Dateien)
*−1 dB, Cooldown 1500 ms*

- [ ] `sfx/ui/ui_ruler_changed_01.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Krönungsfanfare
  > a short ceremonial trumpet fanfare with a heavy bell toll, coronation, medieval
- [ ] `sfx/ui/ui_ruler_changed_02.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Krone
  > a heavy iron crown placed on a stone, bell toll and a choir hum, solemn
- [ ] `sfx/ui/ui_ruler_changed_03.ogg` · Dauer 3,5 s · Prompt Influence 0,6 · Variante: Glocke
  > funeral bell tolling three times followed by a soft brass chord

#### ui_treaty_signed – Event `sfx.ui_treaty_signed` (3 Dateien)
*−2 dB, Cooldown 400 ms*

- [ ] `sfx/ui/ui_treaty_signed_01.ogg` · Dauer 2,0 s · Prompt Influence 0,6 · Variante: Siegeln
  > quill signing parchment, wax seal stamped firmly, short warm chime, medieval
- [ ] `sfx/ui/ui_treaty_signed_02.ogg` · Dauer 2,0 s · Prompt Influence 0,6 · Variante: Becher
  > two goblets clinking with a soft string pluck, celebratory but quiet
- [ ] `sfx/ui/ui_treaty_signed_03.ogg` · Dauer 2,0 s · Prompt Influence 0,6 · Variante: Glocke
  > small bronze bell rung twice with a rustle of parchment

#### ui_war_declared – Event `sfx.ui_war_declared` (3 Dateien)
*0 dB, Cooldown 1000 ms*

- [ ] `sfx/ui/ui_war_declared_01.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Horn + Trommel
  > deep war drum strike followed by a long ominous war horn blast, medieval, cinematic, no melody
- [ ] `sfx/ui/ui_war_declared_02.ogg` · Dauer 2,5 s · Prompt Influence 0,6 · Variante: Fehdehandschuh
  > iron gauntlet slammed onto a stone table and a low brass swell, threatening, short
- [ ] `sfx/ui/ui_war_declared_03.ogg` · Dauer 3,0 s · Prompt Influence 0,6 · Variante: Trommelfolge
  > three slow war drum hits with a rising low choir hum, dark fantasy, short

#### ui_window_open – Event `sfx.ui_window_open` (3 Dateien)
*−8 dB, Cooldown 80 ms, 2 Instanzen*

- [ ] `sfx/ui/ui_window_open_01.ogg` · Dauer 0,3 s · Prompt Influence 0,6 · Variante: Holztafel
  > soft wooden panel slide with a faint metal latch click, very short, no reverb
- [ ] `sfx/ui/ui_window_open_02.ogg` · Dauer 0,3 s · Prompt Influence 0,6 · Variante: Mappe
  > light leather folder opening with a parchment rustle, very short
- [ ] `sfx/ui/ui_window_open_03.ogg` · Dauer 0,3 s · Prompt Influence 0,6 · Variante: Schließe
  > small brass clasp unlatching and a tiny paper swish, very short

### A.5 `sfx/world/` – 27 Dateien

#### fire_crackle – Event `sfx.fire_crackle` (3 Dateien)
*−8 dB, 3 Instanzen – Loop-fähig*

- [ ] `sfx/world/fire_crackle_01.ogg` · Dauer 5,0 s (Loop an) · Prompt Influence 0,5 · Variante: Lagerfeuer
  > small campfire crackling, popping wood, close, steady
- [ ] `sfx/world/fire_crackle_02.ogg` · Dauer 5,0 s (Loop an) · Prompt Influence 0,5 · Variante: Fackel
  > burning torch flame, steady flutter and soft crackle
- [ ] `sfx/world/fire_crackle_03.ogg` · Dauer 5,0 s (Loop an) · Prompt Influence 0,45 · Variante: Gebäudebrand
  > burning building fire, crackling embers and a low roar

#### harvest_grain – Event `sfx.harvest_grain` (3 Dateien)
*−8 dB, 2 Instanzen*

- [ ] `sfx/world/harvest_grain_01.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Sichel
  > sickle cutting wheat stalks, crisp swish and dry rustle
- [ ] `sfx/world/harvest_grain_02.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Garbe binden
  > grain bundle gathered and tied, dry straw rustle
- [ ] `sfx/world/harvest_grain_03.ogg` · Dauer 1,2 s · Prompt Influence 0,6 · Variante: Sense
  > scythe sweeping through dry grain, soft long swoosh

#### horn_alarm – Event `sfx.horn_alarm` (3 Dateien)
*0 dB, 1 Instanz, Cooldown 3000 ms*

- [ ] `sfx/world/horn_alarm_01.ogg` · Dauer 3,5 s · Prompt Influence 0,6 · Variante: ein langer Stoß
  > medieval war horn alarm, one long low blast echoing over a village valley
- [ ] `sfx/world/horn_alarm_02.ogg` · Dauer 3,5 s · Prompt Influence 0,6 · Variante: zwei Stöße
  > two urgent blasts of a large animal horn, distant echo
- [ ] `sfx/world/horn_alarm_03.ogg` · Dauer 3,5 s · Prompt Influence 0,55 · Variante: ansteigend
  > rising alarm horn call from a watchtower, reverberating over hills

#### map_army_move – Event `sfx.map_army_move` (3 Dateien)
*−5 dB, Cooldown 400 ms, 2 Instanzen*

- [ ] `sfx/world/map_army_move_01.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Marschkolonne
  > distant column of marching soldiers, rhythmic boots and armour rattle, short, medieval
- [ ] `sfx/world/map_army_move_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Tross
  > horses and a wagon creaking on a dirt road with a brief horn call far away
- [ ] `sfx/world/map_army_move_03.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Trommel
  > war drum marching beat of four hits with cloth banners flapping

#### map_recruit – Event `sfx.map_recruit` (3 Dateien)
*−4 dB, Cooldown 300 ms, 2 Instanzen*

- [ ] `sfx/world/map_recruit_01.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Aushebung
  > a drill sergeant shout far away and a crowd of boots gathering, medieval camp, short
- [ ] `sfx/world/map_recruit_02.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Waffen
  > weapons being handed out, metal clanks and leather straps, short
- [ ] `sfx/world/map_recruit_03.ogg` · Dauer 1,5 s · Prompt Influence 0,6 · Variante: Werbetrommel
  > a recruitment drum roll and short horn call in a village square

#### mine_pick – Event `sfx.mine_pick` (3 Dateien)
*−6 dB, 4 Instanzen*

- [ ] `sfx/world/mine_pick_01.ogg` · Dauer 0,7 s · Prompt Influence 0,6 · Variante: Standard
  > iron pickaxe striking rock, metallic clink and stone chips falling, single hit
- [ ] `sfx/world/mine_pick_02.ogg` · Dauer 0,7 s · Prompt Influence 0,6 · Variante: heller (Gold)
  > pickaxe hitting a gold ore vein, bright ringing clink, single hit
- [ ] `sfx/world/mine_pick_03.ogg` · Dauer 0,7 s · Prompt Influence 0,6 · Variante: kratzig
  > pickaxe impact on hard rock with a gritty scrape, single hit

#### stone_quarry – Event `sfx.stone_quarry` (3 Dateien)
*−6 dB, 4 Instanzen*

- [ ] `sfx/world/stone_quarry_01.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Meißel
  > hammer and chisel splitting a stone block, sharp crack
- [ ] `sfx/world/stone_quarry_02.ogg` · Dauer 1,0 s · Prompt Influence 0,55 · Variante: Hebeln
  > large stone pried loose with an iron bar, grinding scrape and heavy thud
- [ ] `sfx/world/stone_quarry_03.ogg` · Dauer 1,0 s · Prompt Influence 0,6 · Variante: Vorschlaghammer
  > sledgehammer hitting a boulder, deep crack and falling gravel

#### thunder – Event `sfx.thunder` (3 Dateien)
*−2 dB, Cooldown 2000 ms – stereo*

- [ ] `sfx/world/thunder_01.ogg` · Dauer 6,0 s · Prompt Influence 0,5 · Variante: fern
  > distant rolling thunder, deep rumble over hills, no rain
- [ ] `sfx/world/thunder_02.ogg` · Dauer 6,0 s · Prompt Influence 0,5 · Variante: nah, Knall
  > close thunderclap, sharp crack followed by a long rolling rumble, no rain
- [ ] `sfx/world/thunder_03.ogg` · Dauer 6,0 s · Prompt Influence 0,45 · Variante: bedrohlich, fern
  > low ominous thunder, slow rolling, very far away, no rain

#### wood_chop – Event `sfx.wood_chop` (3 Dateien)
*−6 dB, 4 Instanzen, Cooldown 150 ms*

- [ ] `sfx/world/wood_chop_01.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Stamm
  > axe chopping into a tree trunk, sharp wooden thock, outdoors, single hit
- [ ] `sfx/world/wood_chop_02.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: Splittern
  > woodcutter's axe striking a log, wood splintering, single hit
- [ ] `sfx/world/wood_chop_03.ogg` · Dauer 0,8 s · Prompt Influence 0,6 · Variante: dumpf
  > axe hitting hard oak, dull thunk with a small crack, single hit

### A.6 Fledermausschwarm – Voice-Events `voices/units/bat_swarm_*`, aber mit dem Sound-Effects-Generator erzeugen – 9 Dateien

#### bat_swarm_attack – Event `voice.bat_swarm.attack` (3 Dateien)

- [ ] `voices/units/bat_swarm_attack_01.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: Flügelflattern + Kreischen
  > swarm of bats diving, dense leathery wing flutter with sharp shrieks, close, dry
- [ ] `voices/units/bat_swarm_attack_02.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: schnelle Zirplaute
  > many small bats screeching and snapping, rapid chirps, close
- [ ] `voices/units/bat_swarm_attack_03.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: Vorbeiflug
  > angry bat swarm passing overhead, whooshing wings and high shrieks

#### bat_swarm_death – Event `voice.bat_swarm.death` (3 Dateien)

- [ ] `voices/units/bat_swarm_death_01.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: ausklingend
  > bat swarm dropping to the ground, falling flutter fading into silence
- [ ] `voices/units/bat_swarm_death_02.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: mit Aufprall
  > many bats shrieking and thudding onto dirt, fading
- [ ] `voices/units/bat_swarm_death_03.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: leise
  > last fluttering wings and soft chirps dying out

#### bat_swarm_hit – Event `voice.bat_swarm.hit` (3 Dateien)

- [ ] `voices/units/bat_swarm_hit_01.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: kurz, erschrocken
  > bat swarm scattering after a hit, startled chirps and wing burst
- [ ] `voices/units/bat_swarm_hit_02.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: kurz
  > a few bats squealing in pain, short sharp squeals
- [ ] `voices/units/bat_swarm_hit_03.ogg` · Dauer 0,6–2,0 s · Prompt Influence 0,5 (Startwert) · Variante: sehr kurz
  > leathery flap burst with a single shriek, quick

---

## B. ElevenLabs Stimmen (782 Dateien)

**Vorgehen:** Pro Sprecher einmal die Stimme anlegen (Voice Design: Prompt aus dem Stimmprofil in *Voice Description*, **Beispieltext mit Audio-Tags** in *Text to preview*, speichern als `AC_<id>`), dann alle Zeilen des Sprechers mit dieser Stimme erzeugen. **Jede Zeile ist genau eine Datei mit dem vollständigen Text in einem Take**: den Text aus dem ```text-Block des Eintrags **1:1 einfügen** (inklusive der Tags in eckigen Klammern: `[solemn]`, `[pause]`, `[shouting]` …; sie sind Eleven-v3-Audio-Tags und werden nicht gesprochen). Alle Stimmen, auch der Erzähler, laufen auf **Eleven v3** (Multilingual v2 versteht keine Tags). Alle Stimmen werden auf **Deutsch** gesprochen: in der Oberfläche/API die Sprache explizit auf Deutsch (`de`) stellen. Die Voice-Design-Prompts sind englisch formuliert und enthalten „speaks German with a natural, native accent“; die Audio-Tags bleiben englisch. Ohne die Tags ist der Text wortgleich der Spieltext im Spiel (Prüfung: `python3 docs/audio/verify_spoken_text.py`); Abweichungen im gesprochenen Ton gleichen wir nach der Lieferung an, indem wir die Spieltexte anpassen. Kanon-Eigennamen in deutscher Aussprache über das Aussprache-Wörterbuch (Leitfaden, Abschnitt 6). Barks mit mehreren Dateien sind echte Textvarianten: jede Datei bekommt ihren eigenen Text.

**Einstellungs-Startwerte** (Feinabstimmung je Sprecher im Profil; alle Stimmen Eleven v3, v3 kennt kein Similarity/Style): Stability-Modus *Natural* für Erzähler, Barks, Helden und Dialog (Erzähler bei schwankender Stimme *Robust*), *Creative* für Kampf-/Schmerzlaute und Hohle, *Robust* für konsistenzkritische Bosse; Speed 0,92–1,08 laut Profil (Erzähler 0,95). Nachbearbeitung, Lautheit und Effektketten übernimmt die Integration.

### B1 Erzähler

#### Erzähler (Chronist) – Stimme `AC_narrator` (52 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Chronist von Eldraeth; erzählt aus späterer Rückschau |
> | Alter | Mitte 60 |
> | Timbre | tiefer, warmer, satter Bariton, sehr sauber, Erzähl-Resonanz ohne Theatralik |
> | Akzent | Deutsch: neutrales, gebildetes Hochdeutsch (Hörbuch-Diktion) |
> | Tempo | gemessen, getragen, klare Satzbögen; nie langsam-schleppend |
> | Emotion | ernst, ehrfürchtig, leise Trauer, Würde; Hoffnung nur angedeutet |
> | Do | Gewicht auf Namen und Zahlen, Pausen nach Kernsätzen |
> | Don't | Trailer-Stimme, Märchenton, Dramatisierungsspitzen |
>
> **Voice-Design-Prompt:**
> > A distinguished male narrator in his mid-sixties with a deep, warm and resonant baritone voice, immaculately clear.
> > Speaks German with a natural, native accent: neutral, educated standard German (Hochdeutsch) with audiobook diction.
> > Measured, solemn storytelling pace with natural pauses, the voice of an old chronicler recounting history with reverence
> > and quiet sorrow, never theatrical. Studio-quality audiobook recording, close microphone, no background noise, no
> > reverb.
>
> **Beispieltext:**
> > „[solemn] Im achthundertsiebenundneunzigsten Jahr nach dem Fall war die Aschmark ein Land leerer Höfe und stummer
> > Glocken. [pause] Von all den Dörfern, die einst die Südstraße säumten, hielt nur noch eines seine Feuer am Brennen.
> > [slowly] Sein Name war Brennhollow.“
>
> **Modell & Einstellungen:** **Eleven v3**, Stability-Modus *Natural* (bei zu starker Schwankung zwischen Takes *Robust*), Speed 0,95; Tags `[solemn]`, `[pause]`, `[slowly]`, `[softly]`. Multilingual v2 wird nicht mehr genutzt (versteht keine Audio-Tags). Aussprache-Wörterbuch (Leitfaden, Abschnitt 6) anwenden.

- Stimme: `AC_narrator` · Modell: Eleven v3, Stability-Modus *Natural* (Robust, falls die Stimme zwischen Generierungen zu stark schwankt), Speed 0,95 · Manifest: +2 dB.
- Regie allgemein: getragen, ehrfürchtig, leise Trauer; nie Trailer-Pathos. Eine Zeile = eine Generierung.

##### narrator_m01_opening (Event `narrator.m01_opening`)
Kontext: Missionsstart, Kamera fährt über verbrannte Felder der Aschmark auf Brennhollow zu. Musik `main_menu`→`build_calm`.

- [ ] `voices/narrator/narrator_m01_opening_01.ogg` · Regie: ruhig, gewichtig; Pause nach dem ersten Satz; Gedankenstrich als Schwebe, Aufzählung mit Wärme und Hoffnungsschimmer; Schlusssatz düster, langsam, letzte Phrase leiser
  ```text
  [solemn] Jahr 897 nach dem Fall. [pause] Am Rand der Aschmark hält Brennhollow noch stand – dreißig Seelen, ein Bergfried, ein Feld. [slowly] Und jede Nacht kommt der Nebel näher.
  ```

##### narrator_m01_night_falls (Event `narrator.m01_night_falls`)
Kontext: Tag-Nacht-Wechsel; Ambience wechselt `ambience_day` → `ambience_night`. Kann in der Mission mehrfach auftreten.

- [ ] `voices/narrator/narrator_m01_night_falls_01.ogg` · Regie: still, angespannt, unheilvoll; lange Pause nach „Aschhügel“
  ```text
  [ominous] Die Sonne sank hinter die Aschhügel. [long pause] Aus dem Nordosten wehte ein Geruch nach kaltem Stein und alten Gräbern.
  ```

##### narrator_m01_victory (Event `narrator.m01_victory`)
Kontext: Hohlschrein zerstört, Fahler Vogt besiegt. Musik `victory_sting`, danach ruhige Ambience. Folgt auf Hal Venns `m01_victory`.

- [ ] `voices/narrator/narrator_m01_victory_01.ogg` · Regie: ruhig, Erleichterung; Doppelpunkt als kurze Pause, Satz bleibt offen; wachsende Wärme, würdevoll, Gewicht auf „weigerte zu sterben“, Ausklang
  ```text
  [softly] So begann es: [pause] nicht mit einer Krone, [warmly] sondern mit einem Dorf, das sich weigerte zu sterben.
  ```

##### narrator_m01_defeat (Event `narrator.m01_defeat`)
Kontext: Bergfried zerstört oder Isolde gefallen. Musik `defeat_sting`.

- [ ] `voices/narrator/narrator_m01_defeat_01.ogg` · Regie: schwer, langsam, tonlos, leiser werdend; nach der Pause Wendung zur Ermutigung
  ```text
  [somber] Brennhollow ist gefallen. [slowly] Asche bedeckt die Mark. [pause] [encouraging] Doch die Chronik ist noch nicht zu Ende. Erhebt Euch, Wächterin, und versucht es erneut.
  ```

##### `m02_aufbruch` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Beginn der Eskorte, Ascheregen setzt ein.

- [ ] `voices/narrator/narrator_m02_aufbruch_a_01.ogg` · Regie: getragen, Trauer unter der Ruhe · Zeile 1 der Szene
  ```text
  [sad] Drei Tage nach dem Fall des Schreins kam ein Reiter aus dem Süden. Harrowmere, das Dorf im Aschmoor, brenne. Wer noch gehen könne, warte am Brunnen – [pause] auf ein Wunder oder auf jemanden, der keines verspreche.
  ```

##### `m02_sieg` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Schlusswort der Mission.

- [ ] `voices/narrator/narrator_m02_sieg_a_01.ogg` · Regie: warm, leise Hoffnung · Zeile 1 der Szene
  ```text
  [warmly] [quietly] Sechzehn Menschen mehr in Brennhollow, und kein Vorrat für sie. [short pause] Doch an diesem Abend zählte niemand nach: Es brannten zwei Feuer mehr auf dem Marktplatz als am Morgen.
  ```

##### `m03_aufbruch` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Start am Südende der Grauen Straße.

- [ ] `voices/narrator/narrator_m03_aufbruch_a_01.ogg` · Regie: trocken, düster · Zeile 1 der Szene
  ```text
  [menacingly] Die Graue Straße führte einmal von Brennhollow bis zu den Salzmärkten Valemarks. Nun führte sie vor allem zu Gräbern am Straßenrand – [pause] und an ihrem Ende saß ein Ritter, der mit seinen Opfern Buch führte.
  ```

##### `m03_wendel` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.

- [ ] `voices/narrator/narrator_m03_wendel_d_01.ogg` · Regie: ruhig, abwägend · Zeile 4 der Szene
  ```text
  [thoughtfully] Ein Wegelagerer, der um sein Leben handelt. Die Flamme verlangt Gerechtigkeit – [pause] aber Brennhollow braucht Schwerter. Die Entscheidung liegt bei der Wächterin.
  ```

##### `m03_sieg` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m03_sieg_a_01.ogg` · Regie: ruhig, Anerkennung · Zeile 1 der Szene
  ```text
  [calmly] Am Abend zogen die ersten Karren seit zwei Jahren nordwärts. Die Aschmark war immer noch ein Grenzland – [pause] aber nun eines mit einer Straße.
  ```

##### `m04_nacht` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.

- [ ] `voices/narrator/narrator_m04_nacht_a_01.ogg` · Regie: schwer, ruhig · Zeile 1 der Szene
  ```text
  [seriously] Die Hohlen kamen nicht mehr in Wellen. [short pause] In dieser Nacht kamen sie wie eine Flut, die einen Deich gefunden hat. Auf den Zinnen von Brennhollow zählten die Wächter ihre Fackeln und sagten nichts.
  ```

##### `m04_morgen` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Morgengrauen. Der Gesandte des Königs, Kassian Vale, trifft am Südtor ein.

- [ ] `voices/narrator/narrator_m04_morgen_a_01.ogg` · Regie: getragen, Erleichterung · Zeile 1 der Szene
  ```text
  [solemn] Der Himmel über der Aschmark wurde grau, dann fahl, dann golden an den Rändern. [short pause] Die Letzten der Hohlen sanken in die Erde zurück, als hätten sie nur geträumt, lebendig zu sein.
  ```

##### `m06_turnier` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Die Arena von Kaldris, Turniertag.

- [ ] `voices/narrator/narrator_m06_turnier_a_01.ogg` · Regie: festlich, mit leisem Unterton · Zeile 1 der Szene
  ```text
  [solemn] [quietly] Kaldris trug Gold und Weiß. [short pause] Über der Arena hingen die Banner der zwölf Herzogtümer, und auf der Tribüne saß ein alternder König, der aussah, als warte er auf etwas, das nicht der Sieger war.
  ```

##### `m06_glocke` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Abendglocke: Die Attentäter am Balkon der Krone regen sich.

- [ ] `voices/narrator/narrator_m06_glocke_a_01.ogg` · Regie: gespannt, rasch · Zeile 1 der Szene
  ```text
  [worried] [quickly] Es war das Läuten der Abendglocke, das ihn rettete: ein falscher Ton, eine Sekunde zu früh. [short pause] Auf dem Balkon der Krone regte sich ein Schatten, dann zwei, dann drei.
  ```

##### `m07_lager` · Mission 7 – Eisen in den Hügeln (`m07`)
Kontext: Belagerungslager vor Kargsted.

- [ ] `voices/narrator/narrator_m07_lager_a_01.ogg` · Regie: trocken, sachlich · Zeile 1 der Szene
  ```text
  [seriously] Eisen war in der Aschmark kostbarer als Brot. Wer die Schmiede befeuern wollte, musste durch Kargsted – [pause] und Kargsted hatte Mauern, die ein Schwarm Toter nie gebraucht hätte.
  ```

##### `m09_sturm` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Der Sturm setzt ein; die erste Welle naht.

- [ ] `voices/narrator/narrator_m09_sturm_a_01.ogg` · Regie: rau, getragen · Zeile 1 der Szene
  ```text
  [solemn] Der Schnee fiel schwarz, wo die Asche ihn berührte. Auf den Zinnen von Hrimgard sangen die Skaldinnen ihre Ahnenlieder gegen den Wind, und der Wind sang zurück – [pause] mit hohlen Stimmen.
  ```

##### `m09_morgen` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Der Morgen über Hrimgard.

- [ ] `voices/narrator/narrator_m09_morgen_a_01.ogg` · Regie: ruhig, erschöpft · Zeile 1 der Szene
  ```text
  [tired] Als der Morgen kam, lagen die Hohlen wie umgekippte Statuen im Schnee. Auf der Mauer standen die Überlebenden und sahen nicht hinunter – [pause] sie sahen nach Osten, wo es hell wurde.
  ```

##### `m10_wald` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Lichtung der Veyl am Waldrand.

- [ ] `voices/narrator/narrator_m10_wald_a_01.ogg` · Regie: ehrfürchtig, leise · Zeile 1 der Szene
  ```text
  [solemn] [quietly] Der Dornwald war älter als die Götter, und er hatte es nie vergessen. Wer ihn betrat, tat es auf Zehenspitzen – [pause] und wer ihn verließ, nahm ein wenig Rinde in den Knochen mit.
  ```

##### `m10_sieg_a` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Schlusswort: Wurzelmutter verbrannt.

- [ ] `voices/narrator/narrator_m10_sieg_a_a_01.ogg` · Regie: ruhig, bittersüß · Zeile 1 der Szene
  ```text
  [bitterly] Der Hain brannte drei Tage lang. Als die Asche kalt war, wuchs zwischen den Wurzeln ein einzelner heller Trieb – [pause] und niemand wagte es, ihn zu fällen.
  ```

##### `m10_sieg_b` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Schlusswort: Traum betreten.

- [ ] `voices/narrator/narrator_m10_sieg_b_a_01.ogg` · Regie: leise, geheimnisvoll · Zeile 1 der Szene
  ```text
  [eerily] [quietly] Isolde Raventhorn kehrte nach drei Atemzügen aus dem Traum zurück – [pause] und sprach in den folgenden Wochen nicht mehr als nötig. Die Wurzelmutter aber erwachte, und der Dornwald antwortete mit Blüten.
  ```

##### `m11_moor` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Der Südrand des Moores von Marrowfen.

- [ ] `voices/narrator/narrator_m11_moor_a_01.ogg` · Regie: leise, unheimlich · Zeile 1 der Szene
  ```text
  [menacingly] [quietly] In Marrowfen riecht es nach nasser Asche und nach Dingen, die zu lange unter Wasser geschlafen haben. Wo der Riss aufgegangen war, wuchs kein Schilf mehr – [pause] nur ein weißer Ring aus Knochenstaub.
  ```

##### `m11_sieg` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m11_sieg_a_01.ogg` · Regie: ruhig, vieldeutig · Zeile 1 der Szene
  ```text
  [thoughtfully] Der Damm trug die Geretteten ins Trockene. Hinter ihnen schloss sich das Moor – [pause] oder öffnete sich, je nachdem, wen man später fragte.
  ```

##### `m13_wueste` · Mission 13 – Die Aschglasfelder (`m13`)
Kontext: Das Lager am Rand der Aurethwüste.

- [ ] `voices/narrator/narrator_m13_wueste_a_01.ogg` · Regie: weit, ehrfürchtig, leise · Zeile 1 der Szene
  ```text
  [solemn] [quietly] Es war ein Land, das einmal Reich geheißen hatte. Nun lag es als flache Asche bis zum Horizont, und in der Asche funkelten Splitter – [pause] Aschglas, wo der Schleier dünn war, und Wachfeuer, wo die Legion stand.
  ```

##### `m13_sieg` · Mission 13 – Die Aschglasfelder (`m13`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m13_sieg_a_01.ogg` · Regie: leise, bedrückt · Zeile 1 der Szene
  ```text
  [sad] [quietly] Aus den Fugen der erschlagenen Legionäre stieg noch lange rotes Licht. Wer es sah, sprach später nicht davon.
  ```

##### `m14_sieg` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m14_sieg_a_01.ogg` · Regie: ruhig, bildhaft · Zeile 1 der Szene
  ```text
  [calmly] Das Wasser zog sich zurück, als hätte es nie anderes vorgehabt. [short pause] Auf der Riffinsel blieben Muscheln, Knochen und eine Perle von der Größe einer Faust, die später in Brennhollow in eine Rüstung eingelegt wurde.
  ```

##### `m17_nacht` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Kaldris, Nacht des Todes von König Aldric III.

- [ ] `voices/narrator/narrator_m17_nacht_a_01.ogg` · Regie: gedämpft, schwer · Zeile 1 der Szene
  ```text
  [seriously] [quietly] Der König starb im Winter, und die Glocken von Kaldris waren die Einzigen, die es laut sagten. [short pause] Im Flammendom brannten die Kerzen weiter, als hätte niemand ihnen Bescheid gegeben.
  ```

##### `m17_hof` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Im Flammendom vor dem versammelten Hof.

- [ ] `voices/narrator/narrator_m17_hof_a_01.ogg` · Regie: feierlich, kalt · Zeile 1 der Szene
  ```text
  [solemn] Im Flammendom waren sie alle versammelt: die Herzöge, die Flammenwahrer, die Räte. Und in der Mitte der Sarg des Königs, leer – [pause] der Leichnam lag noch im Nebenraum, damit niemand ihn küsste.
  ```

##### `m19_schlacht` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Beginn der Schlacht.

- [ ] `voices/narrator/narrator_m19_schlacht_a_01.ogg` · Regie: ruhig, schwer · Zeile 1 der Szene
  ```text
  [seriously] Zwei Heere an einem Fluss. [short pause] Das Wasser war flach, die Furt breit, und auf beiden Ufern standen Menschen, die mit demselben Gebet in dieselbe Schlacht zogen.
  ```

##### `m19_sieg` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m19_sieg_a_01.ogg` · Regie: ruhig, gewichtig · Zeile 1 der Szene
  ```text
  [calmly] Als der Rauch sich über der Furt lichtete, trug der Fluss seine Toten in die Ebene. [short pause] Das Banner der Raventhorns wehte im Zentrum, und die Herzöge zählten ihre Schwüre noch einmal nach.
  ```

##### `m21_stollen` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Der Eingang des Stollens unter Skarnheims Gipfeln.

- [ ] `voices/narrator/narrator_m21_stollen_a_01.ogg` · Regie: tief, ruhig · Zeile 1 der Szene
  ```text
  [calmly] Der Berg hatte seit dem Aschenfall nicht mehr gesprochen. Wer an seiner Flanke stand, spürte ihn manchmal zittern – [pause] wie ein Schlafender, der etwas Schlimmes träumt.
  ```

##### `m21_sieg` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Der Schrein wird neu geweiht.

- [ ] `voices/narrator/narrator_m21_sieg_a_01.ogg` · Regie: ruhig, gedämpft · Zeile 1 der Szene
  ```text
  [calmly] [quietly] Gharun zerfiel zu Staub, der in der Luft stand, als wollte er sich noch einmal verbeugen. [short pause] Der Eid war nicht gebrochen, sondern erfüllt: Der Schrein war in Sicherheit.
  ```

##### `m25_ruinen` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Am Stadtrand von Solmere.

- [ ] `voices/narrator/narrator_m25_ruinen_a_01.ogg` · Regie: ehrfürchtig, schwermütig · Zeile 1 der Szene
  ```text
  [solemn] Solmere war einst eine Stadt aus Licht gewesen, Terrassen aus weißem Stein bis an den Himmel. Nun waren es Rippen, in denen der Wind klagte – [pause] und zwischen den Rippen standen Männer mit Sonnen auf den Brustpanzern, die auf einen toten König warteten.
  ```

##### `m25_tagebuch` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Am Schrein Ithrens: Varkhans Tagebuch.

- [ ] `voices/narrator/narrator_m25_tagebuch_a_01.ogg` · Regie: leise, ehrfürchtig · Zeile 1 der Szene
  ```text
  [solemn] [quietly] Unter den Resten des Sonnenhofs lag das Licht Ithrens, kaum größer als eine Kerzenflamme. [short pause] Daneben, in einer Nische aus Gold, ein Buch, dessen Seiten nach Rauch und Rosenwasser rochen.
  ```

##### `m25_sieg` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Schlusswort.

- [ ] `voices/narrator/narrator_m25_sieg_a_01.ogg` · Regie: leise, hoffnungsvoll · Zeile 1 der Szene
  ```text
  [hopefully] [quietly] Das Licht Ithrens erlosch nicht. Es wartete, mit der Geduld der Götter, auf jemanden, der bereit war, es zu tragen.
  ```

##### `m26_marsch` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Brennhollow, Abenddämmerung; die Aschenlegion naht.

- [ ] `voices/narrator/narrator_m26_marsch_a_01.ogg` · Regie: schwer, getragen · Zeile 1 der Szene
  ```text
  [solemn] Sie kamen im Gleichschritt. Zum ersten Mal seit dem Aschenfall hörte man die Hohlen nicht durch die Nacht schlurfen, sondern marschieren – [pause] und der Boden der Aschmark antwortete mit einem Beben.
  ```

##### `m26_morgen` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Brennhollow hält bis zum Morgen.

- [ ] `voices/narrator/narrator_m26_morgen_a_01.ogg` · Regie: ruhig, erschöpft · Zeile 1 der Szene
  ```text
  [tired] Das Morgenlicht fand die Zinnen von Brennhollow zerschossen, aber nicht erstürmt. Die Legion zog sich zurück, dem Süden entgegen – [pause] ein Heer, das nicht müde wurde, aber wartete.
  ```

##### `m26_pass` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Die Evakuierung erreicht die Straße nach Kaldris.

- [ ] `voices/narrator/narrator_m26_pass_a_01.ogg` · Regie: leise, schmerzvoll · Zeile 1 der Szene
  ```text
  [sorrowfully] [quietly] Hinter ihnen brannte Brennhollow. Niemand sah sich um. Wer zurückblickte, hörte das Läuten der Kapelle – [pause] und erst später begriff man, dass sie von selbst läutete.
  ```

##### `m28_lager` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Das Lager des Bundes vor Solmere.

- [ ] `voices/narrator/narrator_m28_lager_a_01.ogg` · Regie: weit, ehrfürchtig · Zeile 1 der Szene
  ```text
  [solemn] Hinter dem Horizont wuchsen die Mauern Solmeres: schwarz, hoch, in den Fugen glimmend. Zum ersten Mal seit tausend Jahren sahen lebende Menschen die Hauptstadt Aureths aus der Nähe – [pause] und zählten die Zinnen, ohne sie zu Ende zu zählen.
  ```

##### `m28_haeretiker` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Die Sonnenhäretiker auf den Mauern (Entscheidung in Mission 25).

- [ ] `voices/narrator/narrator_m28_haeretiker_a_01.ogg` · Regie: ruhig, bedächtig · Zeile 1 der Szene
  ```text
  [calmly] Auf den Mauern Solmeres standen Männer mit Sonnen auf der Brust. [short pause] Sie sahen die Banner des Bundes, hörten, was Isolde Raventhorn gelesen hatte, und senkten die Waffen. Man weiß nicht, ob aus Scham oder aus Hoffnung.
  ```

##### `m29_tor` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Das Innere des Tores von Solmere.

- [ ] `voices/narrator/narrator_m29_tor_a_01.ogg` · Regie: feierlich, tonlos · Zeile 1 der Szene
  ```text
  [solemn] Hinter dem Tor lag, was von Aureth übrig war: vier Höfe, einer hinter dem anderen, wie Kammern eines Herzens. [short pause] Und am Ende der Höfe der Thron, auf dem seit tausend Jahren jemand saß, der nicht schlief und nicht starb.
  ```

##### `m29_hoefe` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Beim Betreten des Hofs des Schleiers.

- [ ] `voices/narrator/narrator_m29_hoefe_a_01.ogg` · Regie: leise, gespannt · Zeile 1 der Szene
  ```text
  [worried] [quietly] Der dritte Hof war der Hof des Schleiers. Die Luft zitterte, als wäre sie nur gemalt, und über den Mauern hing ein Riss, durch den man die Nacht sah – [pause] eine, die nicht die eigene war.
  ```

##### `m29_sieg` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Schlusswort der Endschlacht.

- [ ] `voices/narrator/narrator_m29_sieg_a_01.ogg` · Regie: ruhig, still · Zeile 1 der Szene
  ```text
  [calmly] Der Hohle König war gefallen. [short pause] Die Glocken von Solmere läuteten ein letztes Mal, und danach hörte man in der ganzen Welt nur noch den Wind.
  ```

##### `age_age1` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)
Kontext: Kapitelseite Zeitalter I.

- [ ] `voices/narrator/narrator_age_age1_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Jahr 897 nach dem Fall. Am Rand der Aschmark hält Brennhollow noch stand – [pause] dreißig Seelen, ein Bergfried, ein Feld. Und jede Nacht kommt der Nebel näher.
  ```

##### `age_age2` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age2_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Brennhollow wuchs, und mit ihm der Ruf seiner Wächterin. [short pause] Doch wer in der Aschmark Mauern baut, wird in Kaldris gezählt: Der König prüft seine Getreuen, und jeder Hof hat sein Messer.
  ```

##### `age_age3` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age3_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Die Hohlen griffen nicht mehr in Wellen an, sondern wie ein Heer, das einen Plan verfolgt. An drei Grenzen brachen die Mauern – [pause] und die alten Feinde mussten lernen, einander zuzuhören.
  ```

##### `age_age4` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age4_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Aus der Verteidigerin wurde eine Eroberin. [short pause] Die Aschglasfelder lockten Legion und Händler zugleich, und an der Küste wartete etwas, das nicht ertrunken war, sondern wartete.
  ```

##### `age_age5` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age5_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Der König war alt, und der Thron hat keine Freunde. In Kaldris wurden Zeugen leise und Glocken laut – [pause] und Isolde stand in der Mitte, mit dem Schwert in der einen und dem Gesetz in der anderen Hand.
  ```

##### `age_age6` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age6_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Die Götter schwiegen seit dem Aschenfall. Nun regten sich Altäre unter Bergen und Ruinen, und mit ihnen erwachte, was die Götter einst gebunden hatten – [pause] und was sich an ihrer Stelle genährt hatte.
  ```

##### `age_age7` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

- [ ] `voices/narrator/narrator_age_age7_a_01.ogg` · Regie: getragen, ruhig, Chronistenton · Zeile 1 der Szene
  ```text
  [solemn] Das Hohle Tief erwachte nicht aus eigener Kraft: Es hatte immer einen Schlüssel. [short pause] Brennhollow brannte noch einmal, und dann zog ein Bund aus Fahnen und Feuern nach Solmere, wo ein toter König auf seinem Thron wartete.
  ```

##### `age_ende` · Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)
Kontext: Enden der Kampagne, gesprochen nach dem Abspann der letzten Mission.

- [ ] `voices/narrator/narrator_age_ende_a_01.ogg` · Regie: ruhig, getragen, langsam im letzten Satz · Zeile 1 der Szene
  ```text
  [solemn] [slowly] Varkhan Aurelith sprach zum ersten Mal seit tausend Jahren den Namen seiner Tochter – [pause] und das Tiefe verlor seinen Schlüssel. Es wurde still in Solmere, und im Frühjahr blühte in der Aschmark der erste Apfelbaum seit dem Fall.
  ```

- [ ] `voices/narrator/narrator_age_ende_b_01.ogg` · Regie: ruhig, getragen, langsam im letzten Satz · Zeile 2 der Szene
  ```text
  [solemn] [slowly] Das Feuer, das Isolde in Solmere entfachte, brannte neun Tage lang. Das Tiefe wich zurück, doch die Wunden der Welt blieben offen – [pause] und die Glut der Wächterin erlosch mit ihnen.
  ```

- [ ] `voices/narrator/narrator_age_ende_c_01.ogg` · Regie: ruhig, getragen, langsam im letzten Satz · Zeile 3 der Szene
  ```text
  [solemn] [slowly] So wurde die letzte Krone Eldraeths aus Asche geschmiedet. [short pause] Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte mehr. Doch nachts, in den Hallen von Solmere, sprach die neue Herrin manchmal mit jemandem, den niemand sah.
  ```

- [ ] `voices/narrator/narrator_age_ende_d_01.ogg` · Regie: ruhig, getragen, langsam im letzten Satz · Zeile 4 der Szene
  ```text
  [solemn] [slowly] Und so wurde der Schleier geschlossen. [short pause] Niemand krönte die Wächterin der Aschmark. Aber in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen.
  ```

### B2 Helden

#### Isolde Raventhorn (Heldin) – Stimme `AC_hero_isolde` (110 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Spielbare Heldin, Ritterin und Glutklingen-Adeptin; Tochter eines gefallenen Grenzlords |
> | Alter | 34 (klingt Anfang/Mitte 30, mit gelebter Härte) |
> | Timbre | mittlere bis tiefe Frauenstimme, leicht rau/heiser (Folge des Aschenfiebers), fester Kern, kaum Vibrato |
> | Akzent | Deutsch: neutrales Hochdeutsch mit leicht norddeutscher Färbung, geerdet, nicht adlig-geziert; Konsonanten klar |
> | Tempo | knapp, ökonomisch; im Humor kurze Verzögerung vor der Pointe |
> | Emotion | pflichtbewusst, kontrolliert, trocken-humorvoll; Wärme nur durchscheinend; Wut kalt, nicht schrill |
> | Do | Understatement, Müdigkeit hörbar machen, Befehle ruhig und sicher |
> | Don't | Pathos-Heldin, Kreischen, Flirt-Ton, übertriebene Ironie |
>
> **Voice-Design-Prompt:**
> > A woman in her mid-thirties with a low, slightly husky alto voice, a faint roughness at the edges as if her throat was
> > scarred by illness long ago. Speaks German with a natural, native accent: grounded standard German with a faint northern
> > coloring, plain and unpretentious. She speaks in short, controlled, economical sentences, calm authority of a veteran
> > knight, with a dry, understated sense of humour. Steady breath, no vibrato, never shrill. Close-mic, studio-quality
> > recording, no background noise.
>
> **Beispieltext:**
> > „[calmly] Die Aschmark verzeiht keine Fehler, und ich auch nicht. Baut die Palisade bis zum Einbruch der Nacht wieder
> > auf, haltet die Feuer am Brennen, und wenn irgendetwas aus diesen Ruinen kriecht … [pause] nun. Überlasst es mir.
> > Dafür bezahlt man mich. [dryly] Theoretisch.“
>
> **Modell & Einstellungen:** Eleven v3, Stabilität *Natural* (Kampfrufe ggf. *Creative*), Speed 1.0. Tags sparsam:
> `[sarcastic]`, `[chuckles]`, `[exhales]`, `[shouting]`, `[groans]`.

- Stimme: `AC_hero_isolde` (Profil A1) · Modell: Eleven v3, *Natural* (attack/ability gelegentlich *Creative*) · Speed 1,0
- Haltung: pflichtbewusst, kontrolliert, trocken-humorvoll; Humor durch Understatement, nie Klamauk. Lautheitsreferenz der Produktion. Manifest: +1 dB, Cooldown 300 ms.

##### hero_isolde_select (Event `voice.hero_isolde.select`)

- [ ] `voices/heroes/hero_isolde_select_01.ogg` · Regie: knapp, Autorität, **Referenzdatei**
  ```text
  [dryly] Raventhorn. Sprecht.
  ```

- [ ] `voices/heroes/hero_isolde_select_02.ogg` · Regie: ruhig-stolz
  ```text
  [proudly] Die Aschmark steht noch. Ich auch.
  ```

- [ ] `voices/heroes/hero_isolde_select_03.ogg` · Regie: leicht ungeduldig
  ```text
  [impatiently] Ja? Macht schnell, die Toten warten nicht.
  ```

- [ ] `voices/heroes/hero_isolde_select_04.ogg` · Regie: sachlich
  ```text
  [seriously] Wächterin hier.
  ```

- [ ] `voices/heroes/hero_isolde_select_05.ogg` · Regie: trocken-ironisch, nicht genervt
  ```text
  [impatiently] [sarcastic] Was jetzt? Noch ein Feuer zu löschen?
  ```

##### hero_isolde_move (Event `voice.hero_isolde.move`)

- [ ] `voices/heroes/hero_isolde_move_01.ogg` · Regie: knapp
  ```text
  [dryly] Unterwegs.
  ```

- [ ] `voices/heroes/hero_isolde_move_02.ogg` · Regie: sicher, ein Hauch Wehmut
  ```text
  [calmly] Ich kenne diese Wege.
  ```

- [ ] `voices/heroes/hero_isolde_move_03.ogg` · Regie: entschlossen
  ```text
  [firmly] Dann durch die Asche.
  ```

- [ ] `voices/heroes/hero_isolde_move_04.ogg` · Regie: trockener Humor
  ```text
  [amused] Bin unterwegs. Versucht, das Dorf nicht zu verlieren, solange ich weg bin.
  ```

- [ ] `voices/heroes/hero_isolde_move_05.ogg` · Regie: lakonisch
  ```text
  [dryly] Wenn's brennt, geh ich.
  ```

##### hero_isolde_attack (Event `voice.hero_isolde.attack`)

- [ ] `voices/heroes/hero_isolde_attack_01.ogg` · Regie: voller Schlachtruf
  ```text
  [shouting] Für die Aschmark!
  ```

- [ ] `voices/heroes/hero_isolde_attack_02.ogg` · Regie: kalte Wut
  ```text
  [angry] Zurück ins Dunkel mit euch!
  ```

- [ ] `voices/heroes/hero_isolde_attack_03.ogg` · Regie: Kampfformel, kraftvoll
  ```text
  [firmly] Stahl und Glut!
  ```

- [ ] `voices/heroes/hero_isolde_attack_04.ogg` · Regie: gefährlich ruhig
  ```text
  [menacingly] Mal sehen, ob ihr blutet.
  ```

- [ ] `voices/heroes/hero_isolde_attack_05.ogg` · Regie: Befehl an Truppe
  ```text
  [commanding] [shouting] Brennt sie nieder!
  ```

##### hero_isolde_retreat (Event `voice.hero_isolde.retreat`)

- [ ] `voices/heroes/hero_isolde_retreat_01.ogg` · Regie: scharf, kontrolliert
  ```text
  [calmly] Rückzug. Das ist keine Bitte.
  ```

- [ ] `voices/heroes/hero_isolde_retreat_02.ogg` · Regie: bitter-vernünftig
  ```text
  [bitterly] Wir ziehen ab. Morgen kämpfen wir wieder.
  ```

- [ ] `voices/heroes/hero_isolde_retreat_03.ogg` · Regie: außer Atem, entschlossen
  ```text
  [firmly] [exhales] Nicht hier. Nicht heute. Zurück!
  ```

##### hero_isolde_hit (Event `voice.hero_isolde.hit`)

- [ ] `voices/heroes/hero_isolde_hit_01.ogg` · Regie: trotzig
  ```text
  [firmly] [grunts] Ngh! Ist das alles?
  ```

- [ ] `voices/heroes/hero_isolde_hit_02.ogg` · Regie: Anspielung Aschenfieber, gepresst
  ```text
  [groans] Hah … ich hatte schon schlimmeres Fieber.
  ```

- [ ] `voices/heroes/hero_isolde_hit_03.ogg` · Regie: reiner Schmerzlaut, kurz
  ```text
  [sorrowfully] [gasps] Argh!
  ```

##### hero_isolde_death (Event `voice.hero_isolde.death`)

- [ ] `voices/heroes/hero_isolde_death_01.ogg` · Regie: letzter Auftrag, schwach
  ```text
  [quietly] [gasps] Haltet … die Feuer … am Brennen …
  ```

- [ ] `voices/heroes/hero_isolde_death_02.ogg` · Regie: verletzlich, leise
  ```text
  [quietly] Vater … ich hab gehalten … [pause] so lang ich konnte …
  ```

- [ ] `voices/heroes/hero_isolde_death_03.ogg` · Regie: Wut gegen das Sterben
  ```text
  [angry] [exhales] Nicht … jetzt … verdammt …
  ```

##### hero_isolde_victory (Event `voice.hero_isolde.victory`)

- [ ] `voices/heroes/hero_isolde_victory_01.ogg` · Regie: erschöpft, Pause vor „Heute“
  ```text
  [tired] Die Mark hält. Heute.
  ```

- [ ] `voices/heroes/hero_isolde_victory_02.ogg` · Regie: Understatement
  ```text
  [exhales] Na ja. Hätte schlimmer laufen können.
  ```

- [ ] `voices/heroes/hero_isolde_victory_03.ogg` · Regie: ernst, Verantwortung
  ```text
  [seriously] Zählt zuerst die Lebenden. Dann die Toten.
  ```

##### hero_isolde_ability (Event `voice.hero_isolde.ability`)

- [ ] `voices/heroes/hero_isolde_ability_01.ogg` · Regie: Glutspalter (`ember_cleave`), Schlagschrei
  ```text
  [shouting] Kostet die Glut!
  ```

- [ ] `voices/heroes/hero_isolde_ability_02.ogg` · Regie: Glutspalter, mit Hiebanstrengung
  ```text
  [commanding] Spalten und brennen!
  ```

- [ ] `voices/heroes/hero_isolde_ability_03.ogg` · Regie: Sammelbanner (`rallying_banner`), mitreißend
  ```text
  [shouting] Zu mir! Hebt das Banner!
  ```

- [ ] `voices/heroes/hero_isolde_ability_04.ogg` · Regie: Sammelbanner, Moralruf
  ```text
  [commanding] Steht zu mir. Die Mark sieht zu!
  ```

- [ ] `voices/heroes/hero_isolde_ability_05.ogg` · Regie: Aschenschild (`ashen_ward`), konzentriert, halblaut
  ```text
  [quietly] Asche, schirme mich.
  ```

- [ ] `voices/heroes/hero_isolde_ability_06.ogg` · Regie: Aschenschild, schützend-befehlend
  ```text
  [commanding] Hinter mich. Sofort.
  ```

##### hero_isolde_greet (Event `voice.hero_isolde.greet`)

- [ ] `voices/heroes/hero_isolde_greet_01.ogg` · Regie: Vorstellung, sachlich-warm
  ```text
  [warmly] Isolde Raventhorn, Wächterin der Aschmark. An die Arbeit.
  ```

- [ ] `voices/heroes/hero_isolde_greet_02.ogg` · Regie: ruhig, entschlossen
  ```text
  [firmly] Brennhollow atmet noch. Gut. Dabei soll's bleiben.
  ```

- [ ] `voices/heroes/hero_isolde_greet_03.ogg` · Regie: trocken, augenzwinkernd
  ```text
  [amused] [sarcastic] Ihr habt gerufen, ich bin da. Versucht diesmal, einen Plan zu haben.
  ```

##### hero_isolde_levelup (Event `voice.hero_isolde.levelup`)

- [ ] `voices/heroes/hero_isolde_levelup_01.ogg` · Regie: nüchtern zufrieden
  ```text
  [seriously] Stärker. Gut. Die Ruinen werden nicht freundlicher.
  ```

- [ ] `voices/heroes/hero_isolde_levelup_02.ogg` · Regie: spürbar, fast ehrfürchtig
  ```text
  [solemn] [exhales] Die Glut brennt heller.
  ```

- [ ] `voices/heroes/hero_isolde_levelup_03.ogg` · Regie: trockener Humor
  ```text
  [amused] Älter, klüger, mehr Narben. Das Übliche.
  ```

##### hero_isolde_lowhp (Event `voice.hero_isolde.lowhp`)

- [ ] `voices/heroes/hero_isolde_lowhp_01.ogg` · Regie: ehrlich, gepresst
  ```text
  [groans] Ich bin verletzt … schwer.
  ```

- [ ] `voices/heroes/hero_isolde_lowhp_02.ogg` · Regie: Humor trotz Schmerz
  ```text
  [sorrowfully] [gasps] Brauche einen Heiler. Oder ein Wunder.
  ```

- [ ] `voices/heroes/hero_isolde_lowhp_03.ogg` · Regie: erschöpft, Warnung an Spieler
  ```text
  [tired] [exhales] Viel mehr halte ich nicht aus.
  ```

##### `m02_kern` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.

- [ ] `voices/characters/hero_isolde_m02_kern_b_01.ogg` · Regie: angespannt, misstrauisch · Zeile 2 der Szene
  ```text
  [worried] Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet.
  ```

- [ ] `voices/characters/hero_isolde_m02_kern_d_01.ogg` · Regie: kaum hörbar, kalter Schreck · Zeile 4 der Szene
  ```text
  [fearful] [whispers] Hal. Sag mir, dass du das gehört hast.
  ```

##### `m02_kern_a` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern zerschlagen.

- [ ] `voices/characters/hero_isolde_m02_kern_a_a_01.ogg` · Regie: hart, entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es.
  ```

##### `m02_kern_b` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern bergen.

- [ ] `voices/characters/hero_isolde_m02_kern_b_a_01.ogg` · Regie: ruhig, entschieden · Zeile 1 der Szene
  ```text
  [calmly] Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren.
  ```

##### `m02_aufbruch` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Beginn der Eskorte, Ascheregen setzt ein.

- [ ] `voices/characters/hero_isolde_m02_aufbruch_b_01.ogg` · Regie: bestimmt, trocken · Zeile 2 der Szene
  ```text
  [firmly] Wir führen sie nach Brennhollow. [short pause] Kein Wagen bleibt zurück, kein Kind. Bleibt auf der Straße, und wenn der Ascheregen fällt, bleibt zusammen.
  ```

##### `m02_furt` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Der Zug erreicht die Furt.

- [ ] `voices/characters/hero_isolde_m02_furt_a_01.ogg` · Regie: gedämpft, konzentriert · Zeile 1 der Szene
  ```text
  [quietly] Das Wasser reicht bis zu den Knien. Tragt die Kinder, und redet nicht – [pause] das Moor hört zu.
  ```

##### `m02_hinterhalt` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Hinterhalt im Röhricht vor der Furt.

- [ ] `voices/characters/hero_isolde_m02_hinterhalt_a_01.ogg` · Regie: scharf, alarmiert · Zeile 1 der Szene
  ```text
  [urgently] Im Röhricht, links! Schließt die Reihen, die Schwachen in die Mitte!
  ```

##### `m02_ankunft` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.

- [ ] `voices/characters/hero_isolde_m02_ankunft_c_01.ogg` · Regie: behutsam, aber angespannt · Zeile 3 der Szene
  ```text
  [worried] Welcher Mann, Nel?
  ```

##### `m03_aufbruch` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Start am Südende der Grauen Straße.

- [ ] `voices/characters/hero_isolde_m03_aufbruch_b_01.ogg` · Regie: trocken, entschlossen · Zeile 2 der Szene
  ```text
  [firmly] Neun Leute für eine Straße von dreißig Meilen. Das ist keine Armee, das ist ein Spaziergang mit Absicht.
  ```

##### `m03_briefe` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Isolde findet im zweiten Posten die Briefe mit dem Siegel von Nordmark.

- [ ] `voices/characters/hero_isolde_m03_briefe_a_01.ogg` · Regie: nachdenklich, kühl · Zeile 1 der Szene
  ```text
  [coldly] Briefe. Das Wachs trägt das Siegel von Nordmark – [pause] und die Summe reicht, um eine Kompanie ein Jahr lang zu bezahlen.
  ```

- [ ] `voices/characters/hero_isolde_m03_briefe_b_01.ogg` · Regie: leise, hart · Zeile 2 der Szene
  ```text
  [firmly] [quietly] Jemand bezahlt die Räuber dafür, dass Brennhollow hungert. Das merke ich mir.
  ```

##### `m03_wendel` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.

- [ ] `voices/characters/hero_isolde_m03_wendel_b_01.ogg` · Regie: kalt · Zeile 2 der Szene
  ```text
  [coldly] Du hast Fuhrleute erschlagen, Wendel. Ich habe die Karren gesehen.
  ```

##### `m03_wendel_a` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Entscheidung: Wendel wird gehängt.

- [ ] `voices/characters/hero_isolde_m03_wendel_a_a_01.ogg` · Regie: hart, endgültig · Zeile 1 der Szene
  ```text
  [firmly] Du hängst, Wendel. Für jeden Karren, den du verbrannt hast. Die Straße braucht keine Reue, sie braucht Ruhe.
  ```

##### `m03_wendel_b` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Entscheidung: Wendel und seine Männer werden in Dienst genommen.

- [ ] `voices/characters/hero_isolde_m03_wendel_b_a_01.ogg` · Regie: nüchtern, ohne Drohgebärde · Zeile 1 der Szene
  ```text
  [seriously] Du dienst mir, bis die Straße sicher ist. Wenn du uns verrätst, hole ich dich persönlich.
  ```

##### `m04_nacht` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.

- [ ] `voices/characters/hero_isolde_m04_nacht_c_01.ogg` · Regie: bestimmt · Zeile 3 der Szene
  ```text
  [firmly] Dann sorgen wir dafür, dass er uns lebend findet. Bogenschützen auf die Türme, Schwertträger ans Tor!
  ```

##### `m04_eid_a` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Entscheidung: Treueeid auf die Krone.

- [ ] `voices/characters/hero_isolde_m04_eid_a_a_01.ogg` · Regie: fest, förmlich · Zeile 1 der Szene
  ```text
  [firmly] Mein Vater starb, als er Valemarks Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät.
  ```

##### `m04_eid_b` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Entscheidung: Bündnis mit Herzogin Mireille Corvane.

- [ ] `voices/characters/hero_isolde_m04_eid_b_a_01.ogg` · Regie: trocken, pragmatisch · Zeile 1 der Szene
  ```text
  [dryly] Meine Leute können keine Eide essen. Richtet der Herzogin aus, dass ich ihre Speere annehme.
  ```

##### `m06_anschlag` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Die Attentäter fliehen vom Balkon der Krone.

- [ ] `voices/characters/hero_isolde_m06_anschlag_a_01.ogg` · Regie: scharf · Zeile 1 der Szene
  ```text
  [sharply] Dort oben! Die Reiter voran – [pause] holt sie ein, bevor sie die Gassen erreichen!
  ```

##### `m06_kassian_a` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Der König wird gewarnt.

- [ ] `voices/characters/hero_isolde_m06_kassian_a_a_01.ogg` · Regie: ernst, leise · Zeile 1 der Szene
  ```text
  [seriously] [quietly] Majestät, ein Wort unter vier Augen: Es gab eine Warnung, und sie war nicht ohne Eigennutz.
  ```

##### `m06_kassian_b` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Kassian wird in der Hand behalten.

- [ ] `voices/characters/hero_isolde_m06_kassian_b_a_01.ogg` · Regie: kühl · Zeile 1 der Szene
  ```text
  [coldly] Kassian. Ihr habt mehr gewusst, als Ihr gesagt habt. Dafür schuldet Ihr mir etwas.
  ```

##### `m06_kassian_c` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Schweigen.

- [ ] `voices/characters/hero_isolde_m06_kassian_c_a_01.ogg` · Regie: neutral, abweisend · Zeile 1 der Szene
  ```text
  [calmly] Es war ein Anschlag, mehr nicht. Ich habe nichts gesehen, was zu erzählen wäre.
  ```

##### `m07_lager` · Mission 7 – Eisen in den Hügeln (`m07`)
Kontext: Belagerungslager vor Kargsted.

- [ ] `voices/characters/hero_isolde_m07_lager_b_01.ogg` · Regie: knapp, entschlossen · Zeile 2 der Szene
  ```text
  [firmly] Die Rammen zuerst ans Tor, die Katapulte auf die Eckschützen. [short pause] Wer die Mauer mit der Brust einreißen will, lernt es heute auf die harte Weise.
  ```

##### `m07_tor` · Mission 7 – Eisen in den Hügeln (`m07`)
Kontext: Das Südtor fällt.

- [ ] `voices/characters/hero_isolde_m07_tor_a_01.ogg` · Regie: triumphierend, scharf · Zeile 1 der Szene
  ```text
  [triumphantly] Das Tor ist gebrochen! Schildträger vor, Schützen nach – [pause] keine Gefangenen, aber lasst die Bauern am Leben!
  ```

##### `m09_schwur` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Der Torhof von Hrimgard. Torvald Eisenhand empfängt Isolde vor den Gräbern seiner Ahnen.

- [ ] `voices/characters/hero_isolde_m09_schwur_b_01.ogg` · Regie: trocken, furchtlos · Zeile 2 der Szene
  ```text
  [firmly] Lieder übertreiben. Eure machen Euch angeblich größer und weiser.
  ```

- [ ] `voices/characters/hero_isolde_m09_schwur_d_01.ogg` · Regie: ernst, respektvoll · Zeile 4 der Szene
  ```text
  [seriously] Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden.
  ```

##### `m09_schwur_a` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Entscheidung: Blutschwur. Torvald und seine Clankrieger kämpfen mit.

- [ ] `voices/characters/hero_isolde_m09_schwur_a_a_01.ogg` · Regie: fest, ohne Zögern · Zeile 1 der Szene
  ```text
  [firmly] Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite deines Clans.
  ```

##### `m09_schwur_b` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Entscheidung: Bezahlung in Gold. Söldner statt Torvald.

- [ ] `voices/characters/hero_isolde_m09_schwur_b_a_01.ogg` · Regie: ruhig, geschäftsmäßig · Zeile 1 der Szene
  ```text
  [seriously] Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort ist besser. Nenne deinen Preis.
  ```

##### `m10_lager3` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Das dritte Lager ist geräumt, der Herzhain liegt vor ihnen.

- [ ] `voices/characters/hero_isolde_m10_lager3_a_01.ogg` · Regie: erkennend, unbehaglich · Zeile 1 der Szene
  ```text
  [calmly] Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. Sylaen, was immer da im Hain wartet, es ist dasselbe Grau.
  ```

##### `m10_hain_a` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.

- [ ] `voices/characters/hero_isolde_m10_hain_a_a_01.ogg` · Regie: schwer, entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln.
  ```

##### `m10_hain_b` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.

- [ ] `voices/characters/hero_isolde_m10_hain_b_a_01.ogg` · Regie: entschlossen, mit Angst · Zeile 1 der Szene
  ```text
  [fearful] Keine Feuer mehr. Zeig mir den Weg hinein.
  ```

##### `m11_riss_a` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Entscheidung: Riss versiegeln.

- [ ] `voices/characters/hero_isolde_m11_riss_a_a_01.ogg` · Regie: kalt, entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Du bietest mir Brot, aus dem Mund eines Hungernden. Nein. Hal, schließ ihn.
  ```

##### `m11_riss_b` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Entscheidung: Riss anzapfen und Aschglas ernten.

- [ ] `voices/characters/hero_isolde_m11_riss_b_a_01.ogg` · Regie: leise, mit schlechtem Gewissen · Zeile 1 der Szene
  ```text
  [quietly] Nur ein wenig. Nur so viel, wie die Aschmark in zehn Wintern verbraucht.
  ```

##### `m13_wueste` · Mission 13 – Die Aschglasfelder (`m13`)
Kontext: Das Lager am Rand der Aurethwüste.

- [ ] `voices/characters/hero_isolde_m13_wueste_b_01.ogg` · Regie: nüchtern, kalt · Zeile 2 der Szene
  ```text
  [coldly] Schwarze Rüstungen, Glut in den Fugen, und das Wappen Aureths auf der Brust. [short pause] Das sind keine Wiedergänger, das ist eine Armee. Zeigt ihnen Stahl – und Feuer.
  ```

##### `m14_tintenfass` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.

- [ ] `voices/characters/hero_isolde_m14_tintenfass_b_01.ogg` · Regie: trocken · Zeile 2 der Szene
  ```text
  [dryly] Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen.
  ```

##### `m14_flotte_a` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Entscheidung: Die Flotte annehmen.

- [ ] `voices/characters/hero_isolde_m14_flotte_a_a_01.ogg` · Regie: entschlossen, Bitterkeit · Zeile 1 der Szene
  ```text
  [firmly] Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf.
  ```

##### `m14_flotte_b` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Entscheidung: Ohne Flotte marschieren.

- [ ] `voices/characters/hero_isolde_m14_flotte_b_a_01.ogg` · Regie: trocken, stolz · Zeile 1 der Szene
  ```text
  [proudly] Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch.
  ```

##### `m14_isarre` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Isarre Mol, auf der Riffinsel.

- [ ] `voices/characters/hero_isolde_m14_isarre_b_01.ogg` · Regie: kalt, ruhig · Zeile 2 der Szene
  ```text
  [coldly] Dann bin ich hier, um dich zu erlösen. Eine Flut kann man dämmen.
  ```

##### `m17_nacht` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Kaldris, Nacht des Todes von König Aldric III.

- [ ] `voices/characters/hero_isolde_m17_nacht_b_01.ogg` · Regie: leise, konzentriert · Zeile 2 der Szene
  ```text
  [quietly] Drei Spuren, ein Mörder. Der Wein, die Flamme, die Gruft. Wir gehen leise vor – [pause] wer hier Zeugen hat, hat auch Feinde.
  ```

##### `m17_keller` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Im Weinkeller des Palasts.

- [ ] `voices/characters/hero_isolde_m17_keller_a_01.ogg` · Regie: kühl, analytisch · Zeile 1 der Szene
  ```text
  [coldly] Schierling im Wein. Frisch angesetzt, und nur ein einziger Krug. Wer das getan hat, kannte seinen Geschmack – [pause] und wusste, dass er Gold trinkt, kein Wasser.
  ```

##### `m17_kapelle` · Mission 17 – Tod eines Königs (`m17`)
Kontext: In der Kapelle der Ewigen Flamme.

- [ ] `voices/characters/hero_isolde_m17_kapelle_a_01.ogg` · Regie: misstrauisch, leise · Zeile 1 der Szene
  ```text
  [worried] [quietly] Ein Brief unter dem Altarstein, mit dem Siegel Nordmarks. [short pause] Aber die Tinte ist zu frisch, das Wachs zu sauber. Das hat jemand für mich hingelegt.
  ```

##### `m17_krypta` · Mission 17 – Tod eines Königs (`m17`)
Kontext: In der Krypta unter dem Dom, optional.

- [ ] `voices/characters/hero_isolde_m17_krypta_a_01.ogg` · Regie: ernst, erschüttert · Zeile 1 der Szene
  ```text
  [seriously] Das Siegel der Sonnenhäresie, in den Stein der Gruft geritzt. Jemand hat dem König Gift gegeben – [pause] und die Hohlen haben es gewusst, bevor er starb.
  ```

##### `m17_urteil_a` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: Kassian Vale.

- [ ] `voices/characters/hero_isolde_m17_urteil_a_a_01.ogg` · Regie: gemessen, hart · Zeile 1 der Szene
  ```text
  [firmly] Kassian Vale hat es gewusst. Er hat gewarnt, wo er hätte handeln müssen. Sucht ihn – [pause] und fragt ihn nach seiner Rolle.
  ```

##### `m17_urteil_b` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: Herzogin Mireille.

- [ ] `voices/characters/hero_isolde_m17_urteil_b_a_01.ogg` · Regie: ruhig, schneidend · Zeile 1 der Szene
  ```text
  [calmly] Der Brief weist nach Nordmark. Herzogin Mireille, erklärt Euch.
  ```

##### `m17_urteil_c` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: die Sonnenhäretiker.

- [ ] `voices/characters/hero_isolde_m17_urteil_c_a_01.ogg` · Regie: klar, laut · Zeile 1 der Szene
  ```text
  [shouting] Es waren die Sonnenhäretiker. Ihre Zeichen sind im Stein der Gruft. Kaldris hat Verräter in den eigenen Mauern.
  ```

##### `m17_urteil_d` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Die ganze Wahrheit (alle drei Beweise).

- [ ] `voices/characters/hero_isolde_m17_urteil_d_a_01.ogg` · Regie: ruhig, zwingend · Zeile 1 der Szene
  ```text
  [calmly] Ich kenne die ganze Wahrheit: Gift im Wein, ein falscher Brief aus Nordmark, das Siegel der Häresie – [pause] und einer, der sie alle vermittelte. Seht auf die, die von Verwirrung leben.
  ```

##### `m19_thron` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

- [ ] `voices/characters/hero_isolde_m19_thron_b_01.ogg` · Regie: sanft, ungewohnt weich · Zeile 2 der Szene
  ```text
  [gently] Ich bin hier, Majestät. Ruht Euch aus.
  ```

##### `m19_thron_a` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Mireille stützen.

- [ ] `voices/characters/hero_isolde_m19_thron_a_a_01.ogg` · Regie: bitter, entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere.
  ```

##### `m19_thron_b` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Aldrens Linie verteidigen.

- [ ] `voices/characters/hero_isolde_m19_thron_b_a_01.ogg` · Regie: klar, unerbittlich · Zeile 1 der Szene
  ```text
  [calmly] Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern.
  ```

##### `m19_schlacht` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Beginn der Schlacht.

- [ ] `voices/characters/hero_isolde_m19_schlacht_b_01.ogg` · Regie: laut, kommandierend · Zeile 2 der Szene
  ```text
  [commanding] [shouting] Haltet die Furt! Die Reiter an die Flanken, Armbrüste in die Mitte – [pause] und lasst sie kommen!
  ```

##### `m21_stollen` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Der Eingang des Stollens unter Skarnheims Gipfeln.

- [ ] `voices/characters/hero_isolde_m21_stollen_b_01.ogg` · Regie: leise, kontrolliert · Zeile 2 der Szene
  ```text
  [calmly] [quietly] Der Weg führt durch drei Hallen. Haltet die Reihen geschlossen, und wenn der Stein spricht: nicht antworten.
  ```

##### `m21_soeldner` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Nur wenn Torvald nicht mitkämpft (Gold-Variante).

- [ ] `voices/characters/hero_isolde_m21_soeldner_a_01.ogg` · Regie: nachdenklich, trocken · Zeile 1 der Szene
  ```text
  [thoughtfully] Gold kauft Schwerter, aber keine Trauer. Dieser Berg ist nicht meiner – [pause] und doch kämpfe ich um ihn.
  ```

##### `m21_gharun` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Isolde und Gharun im Schreinsaal.

- [ ] `voices/characters/hero_isolde_m21_gharun_b_01.ogg` · Regie: ruhig, gegenüber einem Berg · Zeile 2 der Szene
  ```text
  [calmly] Du bewahrst den Falschen, Wächter. Wir sind gekommen, den Schrein zurückzuholen – [pause] und deinen Eid zu lösen.
  ```

##### `m21_sieg` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Der Schrein wird neu geweiht.

- [ ] `voices/characters/hero_isolde_m21_sieg_b_01.ogg` · Regie: leise, ehrfürchtig · Zeile 2 der Szene
  ```text
  [solemn] [quietly] Oruun, wenn du zuhörst: Der Schrein ist dein. Wir kommen wieder, wenn du uns brauchst.
  ```

##### `m25_urteil_a` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Die Wahrheit veröffentlichen.

- [ ] `voices/characters/hero_isolde_m25_urteil_a_a_01.ogg` · Regie: ruhig, klar · Zeile 1 der Szene
  ```text
  [calmly] Die Wahrheit gehört den Menschen. Lasst sie lesen, was Varkhan getan hat – [pause] und warum. Wer ihn dann noch erlösen will, soll es wissen.
  ```

##### `m25_urteil_b` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Den Glauben der Häretiker lassen.

- [ ] `voices/characters/hero_isolde_m25_urteil_b_a_01.ogg` · Regie: leise, widerwillig · Zeile 1 der Szene
  ```text
  [quietly] Sie glauben an etwas, das sie am Leben hält. Ich nehme ihnen das nicht, bevor ich es ersetzen kann.
  ```

##### `m25_urteil_c` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Die Häretiker für den eigenen Feldzug nutzen.

- [ ] `voices/characters/hero_isolde_m25_urteil_c_a_01.ogg` · Regie: kalt, berechnend · Zeile 1 der Szene
  ```text
  [coldly] Sie glauben, er werde zurückkehren. Gut. Sie sollen für ihn kämpfen – [pause] bis ich ihn selbst stelle.
  ```

##### `m26_marsch` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Brennhollow, Abenddämmerung; die Aschenlegion naht.

- [ ] `voices/characters/hero_isolde_m26_marsch_c_01.ogg` · Regie: schwer, nachdenklich · Zeile 3 der Szene
  ```text
  [thoughtfully] Dies ist keine Entscheidung, die ich allein treffen will. Aber sie steht mir zu. Halten wir Brennhollow – [pause] oder retten wir die, die in ihm wohnen?
  ```

##### `m26_wahl_a` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Entscheidung: Brennhollow halten.

- [ ] `voices/characters/hero_isolde_m26_wahl_a_a_01.ogg` · Regie: laut, kommandierend · Zeile 1 der Szene
  ```text
  [commanding] [shouting] Wir halten. Diese Mauern haben dreißig Jahre Feuer gesehen. Jeder Bürger auf die Wälle, jedes Kind in die Keller!
  ```

##### `m26_wahl_b` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Entscheidung: Brennhollow evakuieren.

- [ ] `voices/characters/hero_isolde_m26_wahl_b_a_01.ogg` · Regie: kalt, schwer · Zeile 1 der Szene
  ```text
  [coldly] Wir räumen die Stadt. Die Legion kann Steine erobern, aber keine Menschen. Das Osttor auf – [pause] alle zum Kaldris-Pass!
  ```

##### `m26_flucht` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Beginn der Evakuierung.

- [ ] `voices/characters/hero_isolde_m26_flucht_a_01.ogg` · Regie: laut, drängend · Zeile 1 der Szene
  ```text
  [urgently] [shouting] Haltet die Reihen, und lasst die Kinder zwischen den Wagen gehen! Die Soldaten decken den Rückzug – [pause] wer zurückbleibt, bleibt mit Ehre!
  ```

##### `m28_lager` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Das Lager des Bundes vor Solmere.

- [ ] `voices/characters/hero_isolde_m28_lager_b_01.ogg` · Regie: klar, knapp · Zeile 2 der Szene
  ```text
  [dryly] Rammen ans Tor, Katapulte auf die Zinnen. Und niemand betritt Solmere, ehe das Tor offen ist – [pause] ich will keinen Mann vor die Hexen schicken, der noch eine Mauer überwinden muss.
  ```

##### `m28_tor` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Das Haupttor fällt.

- [ ] `voices/characters/hero_isolde_m28_tor_a_01.ogg` · Regie: laut, triumphierend · Zeile 1 der Szene
  ```text
  [triumphantly] [shouting] Das Tor ist gebrochen! Schildträger voran, die Priester folgen den Schilden – [pause] und lasst keinen zurück!
  ```

##### `m29_tor` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Das Innere des Tores von Solmere.

- [ ] `voices/characters/hero_isolde_m29_tor_c_01.ogg` · Regie: ruhig, fest · Zeile 3 der Szene
  ```text
  [firmly] Zweifel habe ich seit Brennhollow. Aber sie halten mich nicht mehr auf. Vorwärts, für die Mark.
  ```

##### `m29_thron` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Im Thronsaal: Varkhan und die Stimme aus dem Tief.

- [ ] `voices/characters/hero_isolde_m29_thron_b_01.ogg` · Regie: ruhig, kalt · Zeile 2 der Szene
  ```text
  [coldly] Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken.
  ```

##### `m29_ende_a` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Schleier versiegeln.

- [ ] `voices/characters/hero_isolde_m29_ende_a_a_01.ogg` · Regie: Abschied, ruhig, sehr gefasst · Zeile 1 der Szene
  ```text
  [calmly] Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten.
  ```

##### `m29_ende_b` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Hohle Krone nehmen.

- [ ] `voices/characters/hero_isolde_m29_ende_b_a_01.ogg` · Regie: hart, kalt entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden.
  ```

##### `m29_ende_c` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).

- [ ] `voices/characters/hero_isolde_m29_ende_c_a_01.ogg` · Regie: weich, ruhig, ohne Kälte · Zeile 1 der Szene
  ```text
  [gently] Varkhan. Deine Tochter hieß Sereth. Sie hat dir nie gesagt, dass sie dir verzeiht, weil sie nie dachte, dass du es brauchst. Sie tut es.
  ```

##### `m29_ende_d` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Das Tiefe verbrennen (nur mit zerschlagenem Kern und versiegeltem Riss).

- [ ] `voices/characters/hero_isolde_m29_ende_d_a_01.ogg` · Regie: bitter, leidenschaftlich · Zeile 1 der Szene
  ```text
  [bitterly] Es gibt kein Tor, das ich schließen muss, und keinen Thron, den ich besteigen will. Es gibt nur Feuer – [pause] und das ist alles, was ich je gut konnte.
  ```

#### Bruder Hal Venn (Figur und Held `hero_hal_venn`) – Stimme `AC_char_hal_venn` (76 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Mentor, Priester der Ewigen Flamme, Stimme der Mission 1 |
> | Alter | Anfang 60 |
> | Timbre | warmer, kiesiger Bariton, leicht rauchig (Jahre am Altarfeuer), freundlicher Brustklang |
> | Akzent | Deutsch: weiches, ländlich gefärbtes Hochdeutsch (leichte norddeutsche Wärme), ländlicher als Isolde |
> | Tempo | ruhig, erzählend; bei Gefahr klar und schnell, aber nie panisch |
> | Emotion | Fürsorge, Glaube ohne Fanatismus, gutmütiger Humor, Trauer um verlorene Gemeinden |
> | Do | väterlich, gelegentlich sanfter Spott über Isoldes Sturheit |
> | Don't | Predigerpathos, Frömmelei, Märchenonkel-Ton |
>
> **Voice-Design-Prompt:**
> > A kind priest in his early sixties with a warm, gravelly baritone, slightly smoky from years beside altar fires. Speaks
> > German with a natural, native accent: soft, down-to-earth standard German with a faint rural northern warmth. Calm,
> > patient, storytelling pace, gentle humour and quiet faith, fatherly but not preachy. When urgent, he becomes clear and
> > firm without panic. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[warmly] Dreißig Seelen, Isolde. Dreißig Seelen, ein kalter Herd und eine Kapelle ohne Dach. Kein großes Königreich.
> > [softly] Aber die Flamme hat mit weniger begonnen, und du auch. Entzünde das erste Feuer, und dann sehen wir, wer zu
> > ihm heimkehrt.“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural*, Speed 0,97 (`[chuckles]`, `[sighs]` gezielt; ruhige Passagen mit `[softly]`, `[pause]`).

- Stimme: `AC_char_hal_venn` · Modell: Eleven v3, *Natural*, Speed 0,97 · Manifest: +1 dB.
- Regie allgemein: väterlicher Mentor, warm, geerdet; in Gefahr klar, nie panisch. Spricht Isolde mit Vornamen an (vertrautes „du“ im DE).
- Stimme: `AC_hero_hal_venn (nutzt Profil von `char_hal_venn`, siehe Figurenleitfaden)` · Modell: Eleven v3, *Natural* · Speed 0,97
- Haltung: freundlich-väterlicher Mentor, bedächtig, mit trockenem Humor; im Kampf überraschend entschlossen. Gleiche Stimme wie die Dialogfigur `char_hal_venn`.

##### hero_hal_venn_select (Event `voice.hero_hal_venn.select`)

- [ ] `voices/heroes/hero_hal_venn_select_01.ogg` · Regie: warm
  ```text
  [warmly] Ja, Wächterin?
  ```

- [ ] `voices/heroes/hero_hal_venn_select_02.ogg` · Regie: ruhig
  ```text
  [calmly] Die Flamme ist mit uns.
  ```

- [ ] `voices/heroes/hero_hal_venn_select_03.ogg` · Regie: sanft
  ```text
  [gently] Meine Hände gehören Euch.
  ```

- [ ] `voices/heroes/hero_hal_venn_select_04.ogg` · Regie: selbstironisch
  ```text
  [cynically] Ach, alte Knochen, bereit wie immer.
  ```

- [ ] `voices/heroes/hero_hal_venn_select_05.ogg` · Regie: sorgend
  ```text
  [calmly] Was braucht Brennhollow von mir?
  ```

##### hero_hal_venn_move (Event `voice.hero_hal_venn.move`)

- [ ] `voices/heroes/hero_hal_venn_move_01.ogg` · Regie: ruhig
  ```text
  [calmly] Geht voran, ich folge.
  ```

- [ ] `voices/heroes/hero_hal_venn_move_02.ogg` · Regie: trocken
  ```text
  [dryly] Langsam, meine Knie erinnern sich an Aureth.
  ```

- [ ] `voices/heroes/hero_hal_venn_move_03.ogg` · Regie: ernst
  ```text
  [seriously] Dorthin, wo die Not am größten ist.
  ```

- [ ] `voices/heroes/hero_hal_venn_move_04.ogg` · Regie: gemütlich
  ```text
  [calmly] Ich gehe, ich gehe.
  ```

- [ ] `voices/heroes/hero_hal_venn_move_05.ogg` · Regie: neutral
  ```text
  [calmly] Dann vorwärts.
  ```

##### hero_hal_venn_attack (Event `voice.hero_hal_venn.attack`)

- [ ] `voices/heroes/hero_hal_venn_attack_01.ogg` · Regie: kräftig
  ```text
  [firmly] Im Namen der Flamme!
  ```

- [ ] `voices/heroes/hero_hal_venn_attack_02.ogg` · Regie: laut
  ```text
  [shouting] Fort mit euch, Schatten!
  ```

- [ ] `voices/heroes/hero_hal_venn_attack_03.ogg` · Regie: scharf
  ```text
  [sharply] Genug! Zurück!
  ```

- [ ] `voices/heroes/hero_hal_venn_attack_04.ogg` · Regie: hymnisch
  ```text
  [solemn] Licht, triff sicher!
  ```

- [ ] `voices/heroes/hero_hal_venn_attack_05.ogg` · Regie: entschlossen
  ```text
  [firmly] Ich dulde kein weiteres verlorenes Dorf!
  ```

##### hero_hal_venn_retreat (Event `voice.hero_hal_venn.retreat`)

- [ ] `voices/heroes/hero_hal_venn_retreat_01.ogg` · Regie: dringlich
  ```text
  [urgently] Zurück, zurück!
  ```

- [ ] `voices/heroes/hero_hal_venn_retreat_02.ogg` · Regie: sorgend-laut
  ```text
  [shouting] Sammelt die Verwundeten!
  ```

- [ ] `voices/heroes/hero_hal_venn_retreat_03.ogg` · Regie: angespannt
  ```text
  [worried] Wir leben, um wieder zu kämpfen!
  ```

##### hero_hal_venn_hit (Event `voice.hero_hal_venn.hit`)

- [ ] `voices/heroes/hero_hal_venn_hit_01.ogg` · Regie: trocken
  ```text
  [dryly] [grunts] Hm. Das hat gebrannt.
  ```

- [ ] `voices/heroes/hero_hal_venn_hit_02.ogg` · Regie: erschrocken
  ```text
  [fearful] [gasps] Flamme, halte mich!
  ```

- [ ] `voices/heroes/hero_hal_venn_hit_03.ogg` · Regie: trotzig
  ```text
  [firmly] [groans] Alt, nicht tot!
  ```

##### hero_hal_venn_death (Event `voice.hero_hal_venn.death`)

- [ ] `voices/heroes/hero_hal_venn_death_01.ogg` · Regie: verhauchend, warm
  ```text
  [warmly] [exhales] Wächterin … die Flamme … behüte Euch …
  ```

- [ ] `voices/heroes/hero_hal_venn_death_02.ogg` · Regie: leise
  ```text
  [quietly] [gasps] Sagt Brennhollow … ich habe es versucht …
  ```

- [ ] `voices/heroes/hero_hal_venn_death_03.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] Ins Licht …
  ```

##### hero_hal_venn_victory (Event `voice.hero_hal_venn.victory`)

- [ ] `voices/heroes/hero_hal_venn_victory_01.ogg` · Regie: erleichtert
  ```text
  [relieved] Die Flamme hat gesiegt.
  ```

- [ ] `voices/heroes/hero_hal_venn_victory_02.ogg` · Regie: gerührt
  ```text
  [moved] Gelobt sei die Flamme, wir leben.
  ```

- [ ] `voices/heroes/hero_hal_venn_victory_03.ogg` · Regie: ruhig
  ```text
  [calmly] Lasst uns die Verwundeten versorgen.
  ```

##### hero_hal_venn_ability (Event `voice.hero_hal_venn.ability`)

- [ ] `voices/heroes/hero_hal_venn_ability_01.ogg` · Regie: warm, laut
  ```text
  [warmly] [shouting] Flamme, heile sie!
  ```

- [ ] `voices/heroes/hero_hal_venn_ability_02.ogg` · Regie: scharf
  ```text
  [sharply] Brennt rein!
  ```

- [ ] `voices/heroes/hero_hal_venn_ability_03.ogg` · Regie: hymnisch
  ```text
  [solemn] Glaube sei unser Feuer!
  ```

##### hero_hal_venn_greet (Event `voice.hero_hal_venn.greet`)

- [ ] `voices/heroes/hero_hal_venn_greet_01.ogg` · Regie: warm
  ```text
  [warmly] Wächterin. Schön, Euch aufrecht zu sehen.
  ```

- [ ] `voices/heroes/hero_hal_venn_greet_02.ogg` · Regie: ernst-freundlich
  ```text
  [warmly] Die Siedlung braucht Euch, und ich auch.
  ```

- [ ] `voices/heroes/hero_hal_venn_greet_03.ogg` · Regie: sanft
  ```text
  [gently] Ich habe die Lampe am Brennen gehalten.
  ```

##### hero_hal_venn_levelup (Event `voice.hero_hal_venn.levelup`)

- [ ] `voices/heroes/hero_hal_venn_levelup_01.ogg` · Regie: überrascht, froh
  ```text
  [surprised] [quickly] Die Flamme brennt heller in mir.
  ```

- [ ] `voices/heroes/hero_hal_venn_levelup_02.ogg` · Regie: bescheiden
  ```text
  [calmly] Stärker. Und das macht demütig.
  ```

- [ ] `voices/heroes/hero_hal_venn_levelup_03.ogg` · Regie: leise
  ```text
  [quietly] Die Gnade wächst.
  ```

##### hero_hal_venn_lowhp (Event `voice.hero_hal_venn.lowhp`)

- [ ] `voices/heroes/hero_hal_venn_lowhp_01.ogg` · Regie: angestrengt
  ```text
  [calmly] Ich muss … mich selbst versorgen …
  ```

- [ ] `voices/heroes/hero_hal_venn_lowhp_02.ogg` · Regie: gepresst
  ```text
  [calmly] Noch nicht, Flamme, noch nicht.
  ```

- [ ] `voices/heroes/hero_hal_venn_lowhp_03.ogg` · Regie: trocken-schwach
  ```text
  [dryly] [quietly] Ich blute. Natürlich.
  ```

##### char_hal_venn_m01_intro (Event `char_hal_venn.m01_intro`)
Kontext: direkt nach `narrator_m01_opening`. Isolde trifft am Bergfried ein; Hal kommt ihr entgegen. Erste Aufgabe: zweites Gehöft und Holz (`obj_farm`).

- [ ] `voices/characters/char_hal_venn_m01_intro_01.ogg` · Regie: sorgenvoll, nüchtern, direkt zur Heldin; Tutorial-Hinweis tatkräftig; Satz offen bis zum Schluss, eindringlich, leiser werdend, ein Hauch Angst
  ```text
  [concerned] Wächterin, die Speicher reichen nicht über den Winter. [firmly] Wir brauchen ein zweites Gehöft und Holz aus dem Westwald – [urgently] bevor es dunkel wird.
  ```

##### char_hal_venn_m01_first_farm (Event `char_hal_venn.m01_first_farm`)
Kontext: zweites Gehöft (`farmstead` ×2) fertiggestellt; nächstes Ziel: Kaserne.

- [ ] `voices/characters/char_hal_venn_m01_first_farm_01.ogg` · Regie: ehrliche Erleichterung, Ton kippt von Wärme zu Ernst, Tutorial-Hinweis (Kaserne) bestimmt und klar
  ```text
  [warmly] Die Felder werden uns ernähren. [seriously] Doch Brot allein hält die Toten nicht auf. [firmly] Errichtet eine Kaserne.
  ```

##### char_hal_venn_m01_barracks (Event `char_hal_venn.m01_barracks`)
Kontext: Kaserne (`barracks`) fertiggestellt; Ausbildung von Schwertkämpfern, Speerträgern, Bogenschützen möglich.

- [ ] `voices/characters/char_hal_venn_m01_barracks_01.ogg` · Regie: bedrückt, kurzer Seufzer vor dem Satz; Tutorial-Hinweis bestimmt; grimmige Zuversicht, Gewicht auf „Stahl“
  ```text
  [sighs] Die Kaserne steht. [firmly] Bildet Speerträger und Bogenschützen aus – [grimly] die Hohlen kennen keine Furcht, aber sie kennen Stahl.
  ```

##### char_hal_venn_m01_wave_warning (Event `char_hal_venn.m01_wave_warning`)
Kontext: erste Angriffswelle der Hohlen über die Brücke (Auslöser `wave_warning`, 215 s). Wird von `sfx/world/horn_alarm_*` begleitet.

- [ ] `voices/characters/char_hal_venn_m01_wave_warning_01.ogg` · Regie: erst lauschend, dringlich („Horn?“), dann laut und gehetzt; nicht hysterisch, Rest-Autorität, Schlusssatz Befehlston
  ```text
  [urgently] Hört Ihr das Horn? [shouting] Sie kommen über die Brücke! Alle Mann an den Bergfried!
  ```

##### char_hal_venn_m01_shrine_found (Event `char_hal_venn.m01_shrine_found`)
Kontext: Einheiten entdecken den Hohlschrein (`hollow_shrine`) mit Knochentürmen (`bone_spire`). Musik wechselt Richtung `build_tension`.

- [ ] `voices/characters/char_hal_venn_m01_shrine_found_01.ogg` · Regie: Ehrfurcht, Furcht, leise; „…“ als Atempause; Missionsziel klar, entschlossen; Schluss warnend, gedämpft, letzte Worte schwer
  ```text
  [whispers] Bei der Flamme … [pause] eine Aureth-Kapelle, und der Schleier darin ist offen. [firmly] Zerstört den Schrein, Wächterin. [quietly] Und hütet Euch vor dem, der ihn bewacht.
  ```

##### char_hal_venn_m01_victory (Event `char_hal_venn.m01_victory`)
Kontext: Schrein zerstört, Vogt gefallen; vor `narrator_m01_victory`.

- [ ] `voices/characters/char_hal_venn_m01_victory_01.ogg` · Regie: Erleichterung, fast Tränen; warm, stolz; am Ende ehrliche Dankbarkeit, ruhig, Ausblick auf Zeitalter II
  ```text
  [exhales] Der Riss ist verschlossen. [warmly] Brennhollow wird diesen Winter überstehen – [softly] dank Euch, Wächterin.
  ```

##### `m02_kern` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.

- [ ] `voices/characters/char_hal_venn_m02_kern_a_01.ogg` · Regie: beunruhigt, leise · Zeile 1 der Szene
  ```text
  [worried] [quietly] Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde.
  ```

- [ ] `voices/characters/char_hal_venn_m02_kern_e_01.ogg` · Regie: besorgt, väterlich · Zeile 5 der Szene
  ```text
  [worried] Was gehört? Isolde, deine Hand zittert. Geh weg davon.
  ```

- [ ] `voices/characters/char_hal_venn_m02_kern_g_01.ogg` · Regie: ernst, väterlich · Zeile 7 der Szene
  ```text
  [warmly] Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. [short pause] Oder es ist das Letzte, was wir je untersuchen. Deine Wahl, Wächterin.
  ```

##### `m02_kern_a` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern zerschlagen.

- [ ] `voices/characters/char_hal_venn_m02_kern_a_c_01.ogg` · Regie: erleichtert · Zeile 3 der Szene
  ```text
  [relieved] Gut. Manches Wissen bleibt besser in der Asche.
  ```

##### `m02_kern_b` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern bergen.

- [ ] `voices/characters/char_hal_venn_m02_kern_b_c_01.ogg` · Regie: beklommen, entschlossen · Zeile 3 der Szene
  ```text
  [firmly] Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber.
  ```

##### `m02_ankunft` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.

- [ ] `voices/characters/char_hal_venn_m02_ankunft_a_01.ogg` · Regie: warm, erleichtert, rufend · Zeile 1 der Szene
  ```text
  [relieved] [shouting] Die Tore auf! Öffnet die Tore – [pause] Wasser, Decken, Brot! Sie leben, Isolde. Sie leben.
  ```

- [ ] `voices/characters/char_hal_venn_m02_ankunft_d_01.ogg` · Regie: behutsam, ernst · Zeile 4 der Szene
  ```text
  [gently] Das Kind hört etwas, das uns nicht hört. Wir können es bei uns behalten und wachen – [pause] oder es den Flammenschwestern in Kaldris anvertrauen.
  ```

##### `m02_nel_a` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Entscheidung: Nel bleibt in Brennhollow.

- [ ] `voices/characters/char_hal_venn_m02_nel_a_a_01.ogg` · Regie: sanft, entschlossen · Zeile 1 der Szene
  ```text
  [firmly] Dann bleibt sie bei uns, und ich werde ihre Träume bewachen. Die Flamme verzeiht eher Mut als Furcht.
  ```

##### `m02_nel_b` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Entscheidung: Nel geht zu den Flammenschwestern.

- [ ] `voices/characters/char_hal_venn_m02_nel_b_a_01.ogg` · Regie: bedrückt, ehrlich · Zeile 1 der Szene
  ```text
  [sad] Die Flammenschwestern in Kaldris kennen Kinder wie sie. Es ist vernünftig – [pause] und es bricht mir das Herz.
  ```

##### `m04_nacht` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.

- [ ] `voices/characters/char_hal_venn_m04_nacht_b_01.ogg` · Regie: trocken, müde, väterlich · Zeile 2 der Szene
  ```text
  [tired] Der Gesandte des Königs soll bei Sonnenaufgang eintreffen, Isolde. Wenn er uns lebend findet, haben wir genug erreicht.
  ```

##### `m04_mitte` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Die Grabkolosse erreichen die Mauern.

- [ ] `voices/characters/char_hal_venn_m04_mitte_a_01.ogg` · Regie: rufend, eindringlich · Zeile 1 der Szene
  ```text
  [urgently] [shouting] Sie bringen die Kolosse! Haltet die Tore, Isolde – [pause] ich singe die Mauer fest, so lange meine Stimme trägt!
  ```

##### `m07_sieg` · Mission 7 – Eisen in den Hügeln (`m07`)
Kontext: Nach dem Sieg im Hof von Kargsted.

- [ ] `voices/characters/char_hal_venn_m07_sieg_a_01.ogg` · Regie: warm, anerkennend · Zeile 1 der Szene
  ```text
  [warmly] Eisen. Wirkliches Eisen. Ich muss es dir lassen, Isolde: Du verstehst, wann man eine Mauer überredet und wann man sie einreißt.
  ```

##### `m11_moor` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Der Südrand des Moores von Marrowfen.

- [ ] `voices/characters/char_hal_venn_m11_moor_b_01.ogg` · Regie: angespannt, nüchtern · Zeile 2 der Szene
  ```text
  [worried] Drei Weiler, Isolde. Wenn wir schnell sind, schaffen wir alle drei vor der Dämmerung. Und wenn wir langsam sind … [pause] schaffen wir es gar nicht.
  ```

##### `m11_dorf` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Ein Weiler schließt sich dem Trupp an.

- [ ] `voices/characters/char_hal_venn_m11_dorf_a_01.ogg` · Regie: warm, rufend · Zeile 1 der Szene
  ```text
  [warmly] [shouting] Kommt, kommt – langsam, haltet euch an den Händen. Wir bringen euch über den Damm, im Namen der Flamme!
  ```

##### `m11_riss` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Isolde erreicht den Riss; die Stimme aus dem Tief macht ihr Angebot.

- [ ] `voices/characters/char_hal_venn_m11_riss_b_01.ogg` · Regie: scharf, besorgt · Zeile 2 der Szene
  ```text
  [worried] Hör nicht hin, Isolde. Was dort flüstert, hat Harrowmere verbrannt.
  ```

- [ ] `voices/characters/char_hal_venn_m11_riss_c_01.ogg` · Regie: ernst, beschwörend · Zeile 3 der Szene
  ```text
  [seriously] Der Riss kann geschlossen werden, wenn wir den Schrein an seinem Grund zerschlagen. Oder wir nehmen, was er uns reicht – [pause] und tragen die Folgen.
  ```

##### `m17_hof` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Im Flammendom vor dem versammelten Hof.

- [ ] `voices/characters/char_hal_venn_m17_hof_c_01.ogg` · Regie: ernst, väterlich · Zeile 3 der Szene
  ```text
  [warmly] Du darfst nur einen Namen nennen, Isolde. Es ist die Aufgabe einer Wächterin, zu entscheiden – [pause] und den Preis zu tragen.
  ```

##### `m19_thron` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

- [ ] `voices/characters/char_hal_venn_m19_thron_e_01.ogg` · Regie: dunkel, ernst · Zeile 5 der Szene
  ```text
  [seriously] Mireille oder die Linie des Königs, Isolde. Beides hat seinen Preis – [pause] und beides wird Menschen töten.
  ```

##### `m25_ruinen` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Am Stadtrand von Solmere.

- [ ] `voices/characters/char_hal_venn_m25_ruinen_b_01.ogg` · Regie: trocken, besorgt · Zeile 2 der Szene
  ```text
  [worried] Sei vorsichtig, Isolde. Die Sonnenhäretiker töten Fremde, aber sie töten sie mit Gebeten. Das macht es nicht besser.
  ```

##### `m25_tagebuch` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Am Schrein Ithrens: Varkhans Tagebuch.

- [ ] `voices/characters/char_hal_venn_m25_tagebuch_b_01.ogg` · Regie: betroffen, leise · Zeile 2 der Szene
  ```text
  [quietly] Varkhans Tagebuch. Er schreibt … [pause] von seiner Tochter. Sie starb an der Schwindsucht, und er wollte … er wollte nicht den Tod besiegen, Isolde. Er wollte sie.
  ```

- [ ] `voices/characters/char_hal_venn_m25_tagebuch_d_01.ogg` · Regie: ernst, bedächtig · Zeile 4 der Szene
  ```text
  [seriously] Es liegt bei dir, Isolde. Das Buch enthält die Wahrheit, und die Wahrheit hat Folgen.
  ```

##### `m25_urteil_a` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Die Wahrheit veröffentlichen.

- [ ] `voices/characters/char_hal_venn_m25_urteil_a_b_01.ogg` · Regie: bedächtig · Zeile 2 der Szene
  ```text
  [calmly] Das wird die Häresie spalten. Aber vielleicht ist das besser, als wenn sie an einer Lüge zerbricht.
  ```

##### `m25_urteil_b` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Den Glauben der Häretiker lassen.

- [ ] `voices/characters/char_hal_venn_m25_urteil_b_b_01.ogg` · Regie: sanft, mahnend · Zeile 2 der Szene
  ```text
  [gently] Gnade und Feigheit sind manchmal Geschwister. Ich hoffe, du weißt, welche du gerade wählst.
  ```

##### `m26_marsch` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Brennhollow, Abenddämmerung; die Aschenlegion naht.

- [ ] `voices/characters/char_hal_venn_m26_marsch_b_01.ogg` · Regie: bestürzt, leise · Zeile 2 der Szene
  ```text
  [quietly] Es ist die Legion, Isolde. Die Wappen Aureths auf jeder Brust. Ich habe sie als Junge in den Liedern gesehen – [pause] nicht im Fleisch.
  ```

##### `m26_wahl_a` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Entscheidung: Brennhollow halten.

- [ ] `voices/characters/char_hal_venn_m26_wahl_a_b_01.ogg` · Regie: fest, warm · Zeile 2 der Szene
  ```text
  [firmly] Die Flamme steht mit Brennhollow. Sie hat es immer getan.
  ```

##### `m26_wahl_b` · Mission 26 – Der Aschenmarsch (`m26`)
Kontext: Entscheidung: Brennhollow evakuieren.

- [ ] `voices/characters/char_hal_venn_m26_wahl_b_b_01.ogg` · Regie: rau, gefasst · Zeile 2 der Szene
  ```text
  [gruffly] Ich habe diese Mauern mit eigenen Händen gesegnet. Es ist eine gute Entscheidung. Es wird nur lange wehtun.
  ```

##### `m28_sieg` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Nach der Eroberung des Torturms.

- [ ] `voices/characters/char_hal_venn_m28_sieg_a_01.ogg` · Regie: leise, bewegt · Zeile 1 der Szene
  ```text
  [moved] [quietly] Das war das Tor. Hinter ihm liegt der Thron. Ich muss dir etwas sagen, Isolde, bevor wir hineingehen: Danke. Für alles, was du getan hast – [pause] auch für das, was dich gekostet hat.
  ```

##### `m29_tor` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Das Innere des Tores von Solmere.

- [ ] `voices/characters/char_hal_venn_m29_tor_b_01.ogg` · Regie: leise, warm · Zeile 2 der Szene
  ```text
  [warmly] [quietly] Gleich ist es so weit, Isolde. Wenn du Zweifel hast – [pause] jetzt wäre der Moment, sie auszusprechen.
  ```

##### `m29_schleier` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Der König ist halb gefallen; der Schleier öffnet sich.

- [ ] `voices/characters/char_hal_venn_m29_schleier_a_01.ogg` · Regie: dringlich · Zeile 1 der Szene
  ```text
  [urgently] Isolde, der Schleier ist offen hinter dem Thron. Was immer du tun willst, tu es jetzt.
  ```

##### `m29_ende_a` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Schleier versiegeln.

- [ ] `voices/characters/char_hal_venn_m29_ende_a_b_01.ogg` · Regie: brechende Stimme, Tränen · Zeile 2 der Szene
  ```text
  [sad] Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich.
  ```

##### `m29_ende_b` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Hohle Krone nehmen.

- [ ] `voices/characters/char_hal_venn_m29_ende_b_c_01.ogg` · Regie: entsetzt, leise · Zeile 3 der Szene
  ```text
  [fearful] [quietly] Isolde … deine Augen. Die Flamme vergebe uns beiden.
  ```

#### Torvald Eisenhand (Figur und Held `hero_torvald`) – Stimme `AC_char_torvald` (48 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Clanfürst, potenzieller Verbündeter/Rivale |
> | Alter | Mitte 50 |
> | Timbre | sehr tiefer, voluminöser Bassbariton, Brustresonanz, rau vom Kälte- und Schmiederauch |
> | Akzent | Deutsch: nordisch gefärbt, rollendes Zungenspitzen-R, harte Konsonanten |
> | Tempo | gemessen und schwer; Lachen dröhnend, Zorn explosiv |
> | Emotion | Stolz, Ahnenehre, misstrauisch gegenüber Südländern, großzügig gegenüber Bewährten |
> | Do | Pausen wie Gewicht, Ahnen beschwören |
> | Don't | Wikinger-Parodie, Grunzen statt Worte |
>
> **Voice-Design-Prompt:**
> > A massive clan chieftain in his mid-fifties with a very deep, booming bass-baritone voice, rich chest resonance and a
> > rough edge from cold air and forge smoke. Speaks German with a natural, native accent: northern-sounding standard German
> > with a rolling r and hard consonants. Heavy, measured pacing, a proud and weathered authority, capable of thunderous
> > laughter and sudden anger. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[proudly] Meine Ahnen haben diesen Pass mit bloßen Händen gehauen und ihre Söhne unter seinen Steinen begraben. Ihr
> > kommt mit euren südlichen Bannern herauf und wollt Eisen? [firmly] Dann trinkt ihr mit uns, blutet mit uns und schwört
> > beim Ahnenpfad. [short pause] Oder geht heim.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural* (`[laughs]`, `[shouting]`); Speed 0,95.

- Stimme: `AC_hero_torvald (nutzt Profil von `char_torvald`, siehe Figurenleitfaden)` · Modell: Eleven v3, *Creative* · Speed 0,95
- Haltung: raues, tiefes Nordclan-Organ, laut, grobschlächtig-herzlich; ehrt Ahnen, verachtet Feigheit, trinkt gern.

##### hero_torvald_select (Event `voice.hero_torvald.select`)

- [ ] `voices/heroes/hero_torvald_select_01.ogg` · Regie: tief, kräftig
  ```text
  [firmly] Torvald Eisenhand hört zu.
  ```

- [ ] `voices/heroes/hero_torvald_select_02.ogg` · Regie: ungeduldig
  ```text
  [impatiently] Sprecht, und macht's kurz.
  ```

- [ ] `voices/heroes/hero_torvald_select_03.ogg` · Regie: ernst
  ```text
  [seriously] Die Ahnen sehen zu.
  ```

- [ ] `voices/heroes/hero_torvald_select_04.ogg` · Regie: gut gelaunt
  ```text
  [calmly] Ha! Ein Kampf, hoffe ich?
  ```

- [ ] `voices/heroes/hero_torvald_select_05.ogg` · Regie: ehrlich
  ```text
  [calmly] Skarnheim steht an Eurer Seite.
  ```

##### hero_torvald_move (Event `voice.hero_torvald.move`)

- [ ] `voices/heroes/hero_torvald_move_01.ogg` · Regie: grob-fröhlich
  ```text
  [calmly] Der Weg ist lang, die Axt ist leicht!
  ```

- [ ] `voices/heroes/hero_torvald_move_02.ogg` · Regie: laut
  ```text
  [shouting] Vorwärts, vorwärts!
  ```

- [ ] `voices/heroes/hero_torvald_move_03.ogg` · Regie: brummig
  ```text
  [gruffly] Meine Füße kennen den Berg, nicht diesen Matsch.
  ```

- [ ] `voices/heroes/hero_torvald_move_04.ogg` · Regie: herzlich
  ```text
  [warmly] Führt an, Wächterin!
  ```

- [ ] `voices/heroes/hero_torvald_move_05.ogg` · Regie: trocken
  ```text
  [dryly] Dann zur nächsten Kehle.
  ```

##### hero_torvald_attack (Event `voice.hero_torvald.attack`)

- [ ] `voices/heroes/hero_torvald_attack_01.ogg` · Regie: Schlachtruf, laut
  ```text
  [shouting] Für die Clans!
  ```

- [ ] `voices/heroes/hero_torvald_attack_02.ogg` · Regie: wild
  ```text
  [shouting] Zerschmettert sie!
  ```

- [ ] `voices/heroes/hero_torvald_attack_03.ogg` · Regie: kehlig
  ```text
  [gruffly] Ahnen, seht mir zu!
  ```

- [ ] `voices/heroes/hero_torvald_attack_04.ogg` · Regie: hart
  ```text
  [firmly] Steht und sterbt!
  ```

- [ ] `voices/heroes/hero_torvald_attack_05.ogg` · Regie: brüllend
  ```text
  [shouting] Hahh! Kommt ans Eisen!
  ```

##### hero_torvald_retreat (Event `voice.hero_torvald.retreat`)

- [ ] `voices/heroes/hero_torvald_retreat_01.ogg` · Regie: knurrend
  ```text
  [growls] Ein Clanmann weicht vor niemandem … heute.
  ```

- [ ] `voices/heroes/hero_torvald_retreat_02.ogg` · Regie: laut
  ```text
  [shouting] Zurück zu den Schilden!
  ```

- [ ] `voices/heroes/hero_torvald_retreat_03.ogg` · Regie: fluchend
  ```text
  [urgently] Zurück, beim Bart meines Vaters!
  ```

##### hero_torvald_hit (Event `voice.hero_torvald.hit`)

- [ ] `voices/heroes/hero_torvald_hit_01.ogg` · Regie: höhnisch
  ```text
  [mockingly] [grunts] Ist das alles?
  ```

- [ ] `voices/heroes/hero_torvald_hit_02.ogg` · Regie: wütend
  ```text
  [angry] [roars] Der hat Blut gezogen!
  ```

- [ ] `voices/heroes/hero_torvald_hit_03.ogg` · Regie: dumpf
  ```text
  [groans] Hrnn!
  ```

##### hero_torvald_death (Event `voice.hero_torvald.death`)

- [ ] `voices/heroes/hero_torvald_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Skarnheim … erinnere dich …
  ```

- [ ] `voices/heroes/hero_torvald_death_02.ogg` · Regie: friedlich-leise
  ```text
  [gently] [gasps] Vater … ich komme …
  ```

- [ ] `voices/heroes/hero_torvald_death_03.ogg` · Regie: gebrochen, rau
  ```text
  [sad] [roars weakly] Der … Berg … ruft …
  ```

##### hero_torvald_victory (Event `voice.hero_torvald.victory`)

- [ ] `voices/heroes/hero_torvald_victory_01.ogg` · Regie: dröhnend
  ```text
  [shouting] Ha! Darauf trinken wir!
  ```

- [ ] `voices/heroes/hero_torvald_victory_02.ogg` · Regie: zufrieden
  ```text
  [cheerfully] Die Ahnen sind zufrieden.
  ```

- [ ] `voices/heroes/hero_torvald_victory_03.ogg` · Regie: stolz
  ```text
  [proudly] Keiner kann sagen, die Clans beugten sich.
  ```

##### hero_torvald_ability (Event `voice.hero_torvald.ability`)

- [ ] `voices/heroes/hero_torvald_ability_01.ogg` · Regie: laut, Ruf
  ```text
  [shouting] Hört mich, Berge!
  ```

- [ ] `voices/heroes/hero_torvald_ability_02.ogg` · Regie: kehlig
  ```text
  [gruffly] Ahnen, leiht mir Eisen!
  ```

- [ ] `voices/heroes/hero_torvald_ability_03.ogg` · Regie: kurz, hart
  ```text
  [firmly] Brecht!
  ```

##### hero_torvald_greet (Event `voice.hero_torvald.greet`)

- [ ] `voices/heroes/hero_torvald_greet_01.ogg` · Regie: herzlich-grob
  ```text
  [warmly] Wächterin Isolde! Euer Bier ist dünn, Eure Sache gut.
  ```

- [ ] `voices/heroes/hero_torvald_greet_02.ogg` · Regie: stolz
  ```text
  [proudly] Skarnheim schickt Eisen, und mich.
  ```

- [ ] `voices/heroes/hero_torvald_greet_03.ogg` · Regie: ungeduldig
  ```text
  [impatiently] Richtet mich auf den Feind.
  ```

##### hero_torvald_levelup (Event `voice.hero_torvald.levelup`)

- [ ] `voices/heroes/hero_torvald_levelup_01.ogg` · Regie: dröhnend
  ```text
  [shouting] Die Kraft der Ahnen wächst in mir!
  ```

- [ ] `voices/heroes/hero_torvald_levelup_02.ogg` · Regie: lachend
  ```text
  [laughs] Schwerere Fäuste, leichteres Herz!
  ```

- [ ] `voices/heroes/hero_torvald_levelup_03.ogg` · Regie: froh
  ```text
  [cheerfully] Mehr Eisen im Blut!
  ```

##### hero_torvald_lowhp (Event `voice.hero_torvald.lowhp`)

- [ ] `voices/heroes/hero_torvald_lowhp_01.ogg` · Regie: keuchend-trotzig
  ```text
  [firmly] [panting] Ich hatte Schlimmeres – glaube ich.
  ```

- [ ] `voices/heroes/hero_torvald_lowhp_02.ogg` · Regie: gepresst
  ```text
  [calmly] Noch … nicht … Ahnen!
  ```

- [ ] `voices/heroes/hero_torvald_lowhp_03.ogg` · Regie: schwach
  ```text
  [quietly] Mein Blut verlässt mich …
  ```

##### `m09_schwur` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Der Torhof von Hrimgard. Torvald Eisenhand empfängt Isolde vor den Gräbern seiner Ahnen.

- [ ] `voices/characters/char_torvald_m09_schwur_a_01.ogg` · Regie: dröhnend, amüsiert · Zeile 1 der Szene
  ```text
  [amused] [shouting] Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen.
  ```

- [ ] `voices/characters/char_torvald_m09_schwur_c_01.ogg` · Regie: grimmig, kurz auflachend, dann dunkel · Zeile 3 der Szene
  ```text
  [coldly] [laughs] Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den hohen Pass. Wir haben vierzig der Unseren begraben. Vierzig.
  ```

- [ ] `voices/characters/char_torvald_m09_schwur_e_01.ogg` · Regie: scharf, prüfend · Zeile 5 der Szene
  ```text
  [thoughtfully] Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold, Sippe zahlt in Blut. Was bist du?
  ```

- [ ] `voices/characters/char_torvald_m09_schwur_f_01.ogg` · Regie: leiser, drängend · Zeile 6 der Szene
  ```text
  [urgently] [quietly] Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig.
  ```

##### `m09_schwur_a` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Entscheidung: Blutschwur. Torvald und seine Clankrieger kämpfen mit.

- [ ] `voices/characters/char_torvald_m09_schwur_a_b_01.ogg` · Regie: jubelnd, laut · Zeile 2 der Szene
  ```text
  [triumphantly] [shouting] Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met – [pause] und die Äxte!
  ```

##### `m09_schwur_b` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Entscheidung: Bezahlung in Gold. Söldner statt Torvald.

- [ ] `voices/characters/char_torvald_m09_schwur_b_b_01.ogg` · Regie: ernüchtert, abweisend · Zeile 2 der Szene
  ```text
  [calmly] Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben.
  ```

##### `m09_kolosse` · Mission 9 – Der Schwarze Winter (`m09`)
Kontext: Grabkolosse erreichen die Mauern.

- [ ] `voices/characters/char_torvald_m09_kolosse_a_01.ogg` · Regie: brüllend, kampflustig · Zeile 1 der Szene
  ```text
  [shouting] Kolosse! Die Äxte an die Mauer, ihr Brüder – [pause] bricht der Koloss die Mauer, brechen wir den Koloss!
  ```

##### `m21_torvald` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Nur wenn Torvald den Blutschwur gehalten hat.

- [ ] `voices/characters/char_torvald_m21_torvald_a_01.ogg` · Regie: grimmig, ehrfürchtig · Zeile 1 der Szene
  ```text
  [solemn] Meine Väter haben diesen Schrein mit eigenen Händen gebaut. [short pause] Dass ich ihn nun mit der Axt zurückfordern muss, ist eine Schande, die ich gern wasche.
  ```

##### `m28_torvald` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Nur wenn Torvald den Blutschwur gehalten hat.

- [ ] `voices/characters/char_torvald_m28_torvald_a_01.ogg` · Regie: grimmig, stolz · Zeile 1 der Szene
  ```text
  [proudly] Meine Ahnen sehen zu, Isolde. Sie haben nie eine Mauer gesehen, vor der Eisenhand zögerte. Wir gehen zuerst – [pause] du brichst das Tor.
  ```

### B3 Figuren

#### König Aldric III. – Stimme `AC_char_king_aldric` (6 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | alternder König mit schwankender Legitimität |
> | Alter | Ende 60 |
> | Timbre | einst kräftiger Bariton, jetzt dünner, belegter Klang; leichte Altersbrüchigkeit, gelegentliches Zittern bei langen Sätzen |
> | Akzent | Deutsch: gepflegtes, getragenes Hochdeutsch (Bühnenaussprache), altmodische Formulierungen |
> | Tempo | langsam, mit Pausen zum Atemholen; beschleunigt bei Zorn |
> | Emotion | Würde, die er festhält; darunter Angst vor Bedeutungslosigkeit; aufflackernder Stolz |
> | Do | Pluralis Majestatis gelegentlich; Erschöpfung im Atem |
> | Don't | senile Karikatur, Brüllen |
>
> **Voice-Design-Prompt:**
> > An elderly king in his late sixties with a fading baritone voice that was once powerful, now thinner and slightly
> > hoarse, with a subtle tremor on long phrases. Speaks German with a natural, native accent: refined, aristocratic
> > stage-standard German, old-fashioned and formal. Slow, deliberate pacing with audible breaths, dignity holding back fear
> > and weariness. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[wearily] Mein Großvater hielt diese Krone gegen Hunger und Feuer. Mein Vater gegen die Hohlung. Und ich … [pause]
> > mir wird gesagt, ich müsse sie gegen meine eigenen Herzöge halten. Sagt mir, Wächterin: [solemn] Kniet Ihr noch vor
> > der Glutkrone – oder nur vor dem Mann, der sie trägt?“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (Speed 0,93); Tags `[tired]`, `[angry]`, `[coughs]` sparsam; Monologe `[slowly]`, `[pause]`.

##### `m06_runde2` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Der König ruft aus der Loge zur zweiten Runde.

- [ ] `voices/characters/char_king_aldric_m06_runde2_a_01.ogg` · Regie: laut, belebt, mit alter Stimme · Zeile 1 der Szene
  ```text
  [shouting] Die Wächterin der Aschmark! Man erzählt, sie schlage die Besten, ohne zu schwitzen. Zeigt es uns noch einmal!
  ```

##### `m06_kassian_a` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Der König wird gewarnt.

- [ ] `voices/characters/char_king_aldric_m06_kassian_a_b_01.ogg` · Regie: müde, dankbar · Zeile 2 der Szene
  ```text
  [tired] Ihr habt mir das Leben gerettet und das Spiel verdorben. Ich weiß nicht, was mich mehr bewegt.
  ```

##### `m19_thron` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

- [ ] `voices/characters/char_king_aldric_m19_thron_a_01.ogg` · Regie: sterbend, flach, mit Atempausen · Zeile 1 der Szene
  ```text
  [coldly] Isolde. Kommt näher. Meine Augen … [pause] sind nicht mehr, was sie waren.
  ```

- [ ] `voices/characters/char_king_aldric_m19_thron_c_01.ogg` · Regie: ernst, leise, schwer atmend · Zeile 3 der Szene
  ```text
  [seriously] [quietly] Mireille steht vor den Toren. [short pause] Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er dürfe es auch. Valemark wird sich zerfleischen.
  ```

##### `m19_thron_a` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Mireille stützen.

- [ ] `voices/characters/char_king_aldric_m19_thron_a_c_01.ogg` · Regie: letzte Worte, verhauchend · Zeile 3 der Szene
  ```text
  [quietly] So sei es … [pause] Haltet … die Feuer am Brennen …
  ```

##### `m19_thron_b` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Aldrens Linie verteidigen.

- [ ] `voices/characters/char_king_aldric_m19_thron_b_c_01.ogg` · Regie: dankbar, schwach · Zeile 3 der Szene
  ```text
  [moved] [quietly] Euer Vater … wäre stolz. Es tut mir … [pause] leid, dass es so viel kostet.
  ```

#### Herzogin Mireille Corvane – Stimme `AC_char_mireille` (8 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | ehrgeizige Herzogin von Nordmark, Rivalin der Krone |
> | Alter | Anfang 40 |
> | Timbre | klarer, kühler Mezzosopran, sehr präzise Artikulation, samtig in den Tiefen |
> | Akzent | Deutsch: scharfes, elegantes Hochdeutsch mit schneidenden Konsonanten |
> | Tempo | kontrolliert, gezielt eingesetzte Pausen; nie gehetzt |
> | Emotion | berechnend, charmant-gefährlich, verächtlich gegenüber Schwäche; echte Überzeugung, dass sie das Reich retten muss |
> | Do | Lächeln hörbar, Drohungen höflich verpacken |
> | Don't | Klischee-Schurkin, Lachen à la Bösewicht |
>
> **Voice-Design-Prompt:**
> > A poised woman in her early forties with a cool, clear mezzo-soprano voice, velvety in the lower register. Speaks German
> > with a natural, native accent: crisp, elegant standard German with precise, cutting consonants. Measured and confident
> > delivery, every pause deliberate, a polite smile that carries a quiet threat. Intelligent, ambitious, never rushed.
> > Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[coldly] Loyalität ist ein hübsches Wort, Wächterin. Es kostet nichts, es auszusprechen, und alles, es zu halten. Der
> > König bietet Euch Dankbarkeit. Ich biete Euch Soldaten, Korn und eine Zukunft für Eure kleine Siedlung. Wählt rasch.
> > [commanding] Die Hohlen warten nicht auf Höflichkeit.“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (`[sarcastic]`, `[laughs softly]` selten; Monologe mit `[pause]`).

##### `m17_hof` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Im Flammendom vor dem versammelten Hof.

- [ ] `voices/characters/char_mireille_m17_hof_b_01.ogg` · Regie: scharf, souverän · Zeile 2 der Szene
  ```text
  [sharply] Wächterin. Man sagt, Ihr wisst, wer es war. Sagt es – [pause] oder lasst uns dem Mörder zuprosten, so lange er bei uns sitzt.
  ```

##### `m17_urteil_a` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: Kassian Vale.

- [ ] `voices/characters/char_mireille_m17_urteil_a_b_01.ogg` · Regie: zufrieden, kalt · Zeile 2 der Szene
  ```text
  [coldly] Ein Spion, der zu viel wusste. Wie vorhersehbar – [pause] und wie bequem.
  ```

##### `m17_urteil_b` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: Herzogin Mireille.

- [ ] `voices/characters/char_mireille_m17_urteil_b_b_01.ogg` · Regie: eisig, bedrohlich · Zeile 2 der Szene
  ```text
  [menacingly] Ihr beschuldigt eine Frau, die fünfhundert Speere in Euren Dienst gestellt hat. Ihr werdet es bereuen.
  ```

##### `m17_urteil_c` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Beschuldigung: die Sonnenhäretiker.

- [ ] `voices/characters/char_mireille_m17_urteil_c_b_01.ogg` · Regie: abwägend · Zeile 2 der Szene
  ```text
  [thoughtfully] Dann reinigt die Stadt. Aber lasst die Herzöge auch ruhig schlafen.
  ```

##### `m17_urteil_d` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Die ganze Wahrheit (alle drei Beweise).

- [ ] `voices/characters/char_mireille_m17_urteil_d_b_01.ogg` · Regie: widerwillig respektvoll · Zeile 2 der Szene
  ```text
  [respectfully] Ihr habt mehr Mut, als ich Euch zutraute. Und mehr Beweise, als mir lieb ist.
  ```

##### `m19_thron` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

- [ ] `voices/characters/char_mireille_m19_thron_d_01.ogg` · Regie: laut, fordernd, vom Feld · Zeile 4 der Szene
  ```text
  [shouting] Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich bin gekommen, ein sterbendes Königreich zu retten!
  ```

##### `m19_thron_a` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Mireille stützen.

- [ ] `voices/characters/char_mireille_m19_thron_a_b_01.ogg` · Regie: ruhig, mit Würde · Zeile 2 der Szene
  ```text
  [calmly] Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit wird.
  ```

##### `m19_thron_b` · Mission 19 – Die Furt von Kaldris (`m19`)
Kontext: Entscheidung: Aldrens Linie verteidigen.

- [ ] `voices/characters/char_mireille_m19_thron_b_b_01.ogg` · Regie: kalt, enttäuscht · Zeile 2 der Szene
  ```text
  [coldly] Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin.
  ```

#### Sylaen der Veyl – Stimme `AC_char_sylaen` (8 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Veyl-Druidin, Brücke zwischen Wald und Menschen |
> | Alter | alterslos (klingt Ende 20 bis Mitte 30, spricht aber mit uralter Ruhe) |
> | Timbre | weiche, hauchige, helle Stimme mit leichter Brüchigkeit wie trockenes Laub; sehr nahe |
> | Akzent | Deutsch: weiche, singende Satzmelodie, ungewöhnliche Betonungen; nicht menschlich-urban |
> | Tempo | langsam, fließend, mit langen Atembögen; Stille ist Teil der Rede |
> | Emotion | ruhig, wissend, melancholisch; Zorn leise und eisig |
> | Do | Naturbilder sanft betonen, Fragen statt Behauptungen |
> | Don't | Esoterik-Flüstern, Kindlichkeit, Elfen-Klischee |
>
> **Voice-Design-Prompt:**
> > An ageless androgynous-leaning female voice that sounds around thirty yet carries centuries of calm. Soft, airy and
> > light, with a delicate dry texture like rustling leaves, very intimate and close. Speaks German with a natural, native
> > accent, in a gentle lilting cadence with unusual, musical stress patterns, unhurried, long flowing phrases and
> > meaningful silences. Wise, melancholic, quietly otherworldly. Studio-quality recording, close microphone, no background
> > noise.
>
> **Beispieltext:**
> > „[softly] Der Dornwald erinnert sich an jede Axt, Wächterin. Jede Wurzel, die du schneidest, ist ein Wort, das du nie
> > zurücknehmen kannst. [pause] Geh leise hier … [whispers] die Wurzelmutter träumt, und ihre Träume sind grau geworden.
> > Wenn sie so erwacht, bleibt keinem von uns ein Wald.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural* (`[whispers]` nur bei Geheimnissen); Speed 0,92.

##### `m10_wald` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Lichtung der Veyl am Waldrand.

- [ ] `voices/characters/char_sylaen_m10_wald_b_01.ogg` · Regie: gedämpft, ernst, melodisch · Zeile 2 der Szene
  ```text
  [seriously] [quietly] Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. Was welkt, welkt von innen – [pause] nicht durch Feuer, sondern durch einen Traum.
  ```

##### `m10_lager1` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Das erste Lager ist geräumt.

- [ ] `voices/characters/char_sylaen_m10_lager1_a_01.ogg` · Regie: traurig, gedämpft · Zeile 1 der Szene
  ```text
  [quietly] Sie sind nicht böse. Sie sind leer. Der Wald hat ihre Namen vergessen, und sie suchen sie in unseren Stimmen.
  ```

##### `m10_hain` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.

- [ ] `voices/characters/char_sylaen_m10_hain_b_01.ogg` · Regie: flüsternd, drängend · Zeile 2 der Szene
  ```text
  [urgently] [whispers] Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe.
  ```

- [ ] `voices/characters/char_sylaen_m10_hain_d_01.ogg` · Regie: gequält, nüchtern · Zeile 4 der Szene
  ```text
  [sad] Es gibt zwei Wege, und ich hasse beide. [short pause] Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken.
  ```

- [ ] `voices/characters/char_sylaen_m10_hain_e_01.ogg` · Regie: leise, drängend · Zeile 5 der Szene
  ```text
  [urgently] [quietly] Entscheide, Wächterin. Der Wald wartet nicht – [pause] und sie auch nicht.
  ```

##### `m10_hain_a` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.

- [ ] `voices/characters/char_sylaen_m10_hain_a_c_01.ogg` · Regie: Trauer, eisig · Zeile 3 der Szene
  ```text
  [sad] Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch.
  ```

##### `m10_hain_b` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.

- [ ] `voices/characters/char_sylaen_m10_hain_b_c_01.ogg` · Regie: eindringlich, leise · Zeile 3 der Szene
  ```text
  [urgently] [quietly] Halt meine Hand und lass nicht los. Was immer du dort drin hörst … [pause] es ist nicht dein Freund.
  ```

##### `m28_sylaen` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Nur wenn die Wurzelmutter erwacht ist (Pakt).

- [ ] `voices/characters/char_sylaen_m28_sylaen_a_01.ogg` · Regie: ruhig, melodisch · Zeile 1 der Szene
  ```text
  [calmly] Der Wald hat mich nicht verlassen, Wächterin. Meine Pfeile fliegen mit dem Wind – [pause] und der Wind kennt Solmere.
  ```

#### Ophira Saelith-Varo – Stimme `AC_char_ophira` (6 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Händlerfürstin des Saelith-Pakts |
> | Alter | Mitte 40 |
> | Timbre | warmer, samtiger Alt, voll und lächelnd, leicht kehliges Lachen |
> | Akzent | Deutsch mit leichtem, melodischem Mittelmeer-Anklang, weltgewandt, mehrsprachig wirkend |
> | Tempo | entspannt, verführerisch langsam bei Angeboten; schnell und präzise beim Feilschen |
> | Emotion | amüsiert, kalkulierend, gastfreundlich mit Hintergedanken; ehrliche Verachtung für Fanatismus |
> | Do | Zahlen und Preise genießen; Charme |
> | Don't | Gier-Karikatur, Akzent übertreiben |
>
> **Voice-Design-Prompt:**
> > A worldly merchant princess in her mid-forties with a warm, velvety contralto voice, full and smiling, with a low
> > throaty laugh. Speaks fluent German with a natural accent and a light, melodic Mediterranean lilt, cosmopolitan and
> > refined. Relaxed, unhurried and charming when making offers, quick and precise when bargaining. Amused, calculating,
> > hospitable with an edge. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[amused] Jeder in Eldraeth betet zu irgendetwas, Wächterin. Eure Leute zu einer Flamme, die Clans zu ihren Toten. In
> > Saelith beten wir zur Flut, und die Flut ist ehrlich: Sie kommt immer zurück, um zu holen, was man ihr schuldet. Nun.
> > [pause] Sprechen wir über Eure Schulden?“
>
> **Modell & Einstellungen:** Eleven v3 *Natural* (`[laughs softly]`, `[amused]`); Speed 0,97.

##### `m14_tintenfass` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.

- [ ] `voices/characters/char_ophira_m14_tintenfass_a_01.ogg` · Regie: amüsiert, Hafenakustik · Zeile 1 der Szene
  ```text
  [amused] Willkommen am Kai, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel.
  ```

- [ ] `voices/characters/char_ophira_m14_tintenfass_c_01.ogg` · Regie: lachend, geschäftsmäßig · Zeile 3 der Szene
  ```text
  [seriously] [laughs] Vierzig Schiffe, dreitausend Söldner und das Riff in neun Tagen statt in neunzig. [short pause] Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus den Ruinen holt.
  ```

- [ ] `voices/characters/char_ophira_m14_tintenfass_e_01.ogg` · Regie: plötzlich kühl · Zeile 5 der Szene
  ```text
  [coldly] Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft.
  ```

##### `m14_flotte_a` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Entscheidung: Die Flotte annehmen.

- [ ] `voices/characters/char_ophira_m14_flotte_a_b_01.ogg` · Regie: warm, mit schneidendem Unterton · Zeile 2 der Szene
  ```text
  [warmly] Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer.
  ```

##### `m14_flotte_b` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Entscheidung: Ohne Flotte marschieren.

- [ ] `voices/characters/char_ophira_m14_flotte_b_b_01.ogg` · Regie: ehrlich amüsiert · Zeile 2 der Szene
  ```text
  [amused] Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin.
  ```

##### `m28_ophira` · Mission 28 – Die Tore von Solmere (`m28`)
Kontext: Nur wenn der Vertrag mit Ophira geschlossen wurde.

- [ ] `voices/characters/char_ophira_m28_ophira_a_01.ogg` · Regie: trocken, geschäftsmäßig · Zeile 1 der Szene
  ```text
  [seriously] Meine Schiffe liegen im Hafen, meine Söldner stehen bei Euch. [short pause] Vergesst nicht, dass jeder Schuss in diesen Mauern ein Eintrag in meinem Buch ist.
  ```

#### Kassian Vale – Stimme `AC_char_kassian` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Spionagemeister, undurchsichtig, wechselnde Loyalitäten |
> | Alter | Ende 30 bis Mitte 40 (bewusst schwer einzuschätzen) |
> | Timbre | glatter, leiser Tenor-Bariton, sehr nah am Mikrofon, fast vertraulich; wenig Brustklang |
> | Akzent | Deutsch, schwer verortbar: gepflegtes, neutrales Hochdeutsch mit verschliffenen Kanten |
> | Tempo | gleichmäßig ruhig, keine Hast, Sätze enden oft offen |
> | Emotion | höflich, amüsiert, nie überrascht; Bedrohung durch Ruhe |
> | Do | flüsternahe Intimität, Halbsätze, Ironie |
> | Don't | Zischeln, Schurkenlachen |
>
> **Voice-Design-Prompt:**
> > A soft-spoken man in his early forties with a smooth, quiet tenor-baritone voice, very close to the microphone, intimate
> > and confidential. Speaks German with a natural, native accent: refined, neutral standard German with softened edges,
> > hard to place regionally. Even, unhurried delivery, never surprised, politely amused, sentences that trail off as if he
> > knows more than he says. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[calmly] Ihr müsst nicht nach dem Messer suchen, Wächterin. Wollte ich Euch tot sehen, wärt Ihr vor drei Wochen
> > bequem im Schlaf gestorben. [short pause] Nein … [quietly] ich bin hier, weil jemand am Königshof die Hohlen füttert.
> > Und ich wüsste gern, wer.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural/Robust* (`[whispers]`, `[amused]`); Speed 0,96.

##### `m04_morgen` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Morgengrauen. Der Gesandte des Königs, Kassian Vale, trifft am Südtor ein.

- [ ] `voices/characters/char_kassian_m04_morgen_b_01.ogg` · Regie: geschmeidig, höflich, leicht amüsiert · Zeile 2 der Szene
  ```text
  [amused] Eine Nacht, die man in Kaldris noch in Jahren erzählen wird – [pause] leider ohne Wahrheit. Ich bin Kassian Vale. Seine Majestät bittet Euch, Wächterin, um Euren Treueeid. Und bevor Ihr antwortet: Herzogin Mireille Corvane bietet Euch fünfhundert Speere und Korn für ein Jahr.
  ```

- [ ] `voices/characters/char_kassian_m04_morgen_c_01.ogg` · Regie: leise, ruhig, eindringlich · Zeile 3 der Szene
  ```text
  [urgently] [quietly] Zwei Hände strecken sich Euch entgegen, Wächterin. Die Krone fordert Euren Eid, Nordmark bietet Speere. Welche Hand Ihr heute nehmt – [pause] die andere hält morgen ein Messer.
  ```

##### `m04_eid_a` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Entscheidung: Treueeid auf die Krone.

- [ ] `voices/characters/char_kassian_m04_eid_a_b_01.ogg` · Regie: höflich, mit Unterton · Zeile 2 der Szene
  ```text
  [calmly] Wie standhaft. Der König wird es zu schätzen wissen – [pause] und die Herzogin sich Euren Namen merken.
  ```

##### `m04_eid_b` · Mission 4 – Die Lange Nacht (`m04`)
Kontext: Entscheidung: Bündnis mit Herzogin Mireille Corvane.

- [ ] `voices/characters/char_kassian_m04_eid_b_b_01.ogg` · Regie: ruhig, vieldeutig · Zeile 2 der Szene
  ```text
  [thoughtfully] Eine kluge Entscheidung – [pause] und eine sehr teure. Nordmark erinnert sich an Freunde, Wächterin. Und an Schuldner.
  ```

##### `m06_turnier` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Die Arena von Kaldris, Turniertag.

- [ ] `voices/characters/char_kassian_m06_turnier_b_01.ogg` · Regie: flüsternd, geschmeidig · Zeile 2 der Szene
  ```text
  [whispers] Eine Beobachtung, Wächterin, ganz unverbindlich: Heute Abend wird der Balkon der Krone nicht jeden verlassen, der ihn betritt. [short pause] Ich überlasse es Euch, was Ihr mit diesem Wissen anfangt.
  ```

##### `m06_kassian_b` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Kassian wird in der Hand behalten.

- [ ] `voices/characters/char_kassian_m06_kassian_b_b_01.ogg` · Regie: mit feinem Lächeln · Zeile 2 der Szene
  ```text
  [calmly] So ist es. Es ist angenehm, wenn jemand begreift, dass Wissen eine Währung ist. Ich werde zahlen, wenn die Stunde kommt.
  ```

##### `m06_kassian_c` · Mission 6 – Das Turnier von Kaldris (`m06`)
Kontext: Entscheidung: Schweigen.

- [ ] `voices/characters/char_kassian_m06_kassian_c_b_01.ogg` · Regie: trocken, flüchtig · Zeile 2 der Szene
  ```text
  [dryly] Wie diskret. Ich bin beinahe enttäuscht – und beeindruckt.
  ```

##### `m06_ende` · Mission 6 – Das Turnier von Kaldris (`m06`)

- [ ] `voices/characters/char_kassian_m06_ende_a_01.ogg` · Regie: leise, belustigt · Zeile 1 der Szene
  ```text
  [quietly] Nun wisst Ihr, dass ich es wusste. Was macht Ihr damit, Wächterin: Sagt Ihr es dem König, behaltet Ihr es als Pfand – [pause] oder schweigt Ihr?
  ```

##### `m14_tintenfass` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.

- [ ] `voices/characters/char_kassian_m14_tintenfass_d_01.ogg` · Regie: trocken, flüsternd · Zeile 4 der Szene
  ```text
  [dryly] [whispers] Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf.
  ```

##### `m17_hinweis` · Mission 17 – Tod eines Königs (`m17`)
Kontext: Nur wenn Kassian Isolde einen Gefallen schuldet (Entscheidung in Mission 6).

- [ ] `voices/characters/char_kassian_m17_hinweis_a_01.ogg` · Regie: flüsternd, aus dem Schatten · Zeile 1 der Szene
  ```text
  [whispers] Ein Wort aus der Dunkelheit, Wächterin: Die Krypta. [short pause] Ihr werdet dort etwas finden, das ich Euch nicht beweisen konnte. Ich sehe Euch im Dom.
  ```

#### Die Stimme aus dem Tief – Stimme `AC_char_deep_voice` (15 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Manifestation des Hohlen Tiefs, älter als die Götter |
> | Alter | zeitlos; weder männlich noch weiblich festgelegt |
> | Timbre | tief, ruhig, hohl, mit flüsterndem Luftanteil; wirkt wie viele Stimmen in einer (entsteht in der Integration) |
> | Akzent | Deutsch, kein Regionalakzent; extrem deutliche, langsame Artikulation, als lerne es menschliche Sprache gerade |
> | Tempo | sehr langsam, lange Pausen, Silben gedehnt |
> | Emotion | geduldig, fast zärtlich, vollkommen gleichgültig gegenüber Leid – Versuchung statt Drohung |
> | Do | Intimität, Pausen, Fragen |
> | Don't | Monster-Brüllen, Dämonen-Klischee |
>
> **Voice-Design-Prompt:**
> > An ancient, genderless voice, deep and calm, hollow and breathy, as if speaking from the bottom of a vast empty cavern.
> > Speaks German with a natural, native pronunciation and no regional accent, extremely slow and precise articulation, each
> > syllable placed carefully as though the speaker is relearning human words. Patient, intimate, almost tender, utterly
> > indifferent to suffering. Seductive rather than threatening. Studio-quality recording, close microphone, no background
> > noise.
>
> **Beispieltext:**
> > „[whispers] Du hast ein Feuer entzündet … und es Hoffnung genannt. [slowly] Ich erinnere mich, als eure Götter Kinder
> > waren, Kleine. Ich erinnere mich, als sie verstummten. Jede Flamme erlischt. Jede Krone wird zu Asche. Komm herab …
> > [softly] und ich zeige dir, was unter dem Schleier liegt.“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Robust*, Speed 0,85; Flüsterpassagen mit `[whispers]`. Geliefert wird **eine** trockene Datei je Zeile; Schichtung und Effekte setzt die Integration.

##### `m02_kern` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.

- [ ] `voices/characters/char_deep_voice_m02_kern_c_01.ogg` · Regie: zärtlich-spöttisch, mehrschichtiger Effekt (Profil Deep Voice) · Zeile 3 der Szene
  ```text
  [mockingly] Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich.
  ```

- [ ] `voices/characters/char_deep_voice_m02_kern_f_01.ogg` · Regie: leise, vertraut, bedrohlich · Zeile 6 der Szene
  ```text
  [menacingly] [quietly] Er kann mich nicht hören. [short pause] Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich.
  ```

##### `m02_kern_a` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern zerschlagen.

- [ ] `voices/characters/char_deep_voice_m02_kern_a_b_01.ogg` · Regie: leises, hohles Lachen · Zeile 2 der Szene
  ```text
  [eerily] [laughs] Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir.
  ```

##### `m02_kern_b` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Reaktion auf die Entscheidung: Kern bergen.

- [ ] `voices/characters/char_deep_voice_m02_kern_b_b_01.ogg` · Regie: zufrieden, verführerisch · Zeile 2 der Szene
  ```text
  [seductively] Ja … trag mich heim. Halte mich nah an deinem Feuer.
  ```

##### `m10_hain` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.

- [ ] `voices/characters/char_deep_voice_m10_hain_c_01.ogg` · Regie: intim, kalt, genüsslich · Zeile 3 der Szene
  ```text
  [greedily] Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … [pause] verdaut. Calyx schmeckte nach Frühling.
  ```

##### `m10_hain_b` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.

- [ ] `voices/characters/char_deep_voice_m10_hain_b_b_01.ogg` · Regie: intim, flüsternd · Zeile 2 der Szene
  ```text
  [seductively] [whispers] Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen.
  ```

##### `m11_riss` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Isolde erreicht den Riss; die Stimme aus dem Tief macht ihr Angebot.

- [ ] `voices/characters/char_deep_voice_m11_riss_a_01.ogg` · Regie: schmeichelnd, tief, mehrschichtig · Zeile 1 der Szene
  ```text
  [calmly] Du bist weit gekommen, kleine Flamme. [short pause] Sieh, was ich dir zeige: Aschglas, so rein, dass es singt. Nimm davon, und deine Heere werden nie wieder hungern.
  ```

##### `m11_riss_a` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Entscheidung: Riss versiegeln.

- [ ] `voices/characters/char_deep_voice_m11_riss_a_b_01.ogg` · Regie: leicht gekränkt, spöttisch · Zeile 2 der Szene
  ```text
  [mockingly] Wie schade. Ich hatte mich gefreut, dich zu verwöhnen.
  ```

##### `m11_riss_b` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Entscheidung: Riss anzapfen und Aschglas ernten.

- [ ] `voices/characters/char_deep_voice_m11_riss_b_b_01.ogg` · Regie: zufrieden, samtig · Zeile 2 der Szene
  ```text
  [seductively] Ein wenig. Natürlich. Ich bin geduldig, Kind. Ich war es immer.
  ```

##### `m25_tagebuch` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Am Schrein Ithrens: Varkhans Tagebuch.

- [ ] `voices/characters/char_deep_voice_m25_tagebuch_c_01.ogg` · Regie: samtig, süß, bedrohlich · Zeile 3 der Szene
  ```text
  [menacingly] Ein Vater, der alles verbrannte, um ein Kind zu retten. So rührend. So nützlich.
  ```

##### `m25_urteil_c` · Mission 25 – Ithrens letztes Licht (`m25`)
Kontext: Entscheidung: Die Häretiker für den eigenen Feldzug nutzen.

- [ ] `voices/characters/char_deep_voice_m25_urteil_c_b_01.ogg` · Regie: leise, amüsiert · Zeile 2 der Szene
  ```text
  [amused] [quietly] Wie praktisch du bist, kleine Flamme. Wie ähnlich uns.
  ```

##### `m29_thron` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Im Thronsaal: Varkhan und die Stimme aus dem Tief.

- [ ] `voices/characters/char_deep_voice_m29_thron_d_01.ogg` · Regie: ruhig, entsetzlich ungerührt · Zeile 4 der Szene
  ```text
  [moved] Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist niemand mehr gestorben.
  ```

##### `m29_schleier` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Der König ist halb gefallen; der Schleier öffnet sich.

- [ ] `voices/characters/char_deep_voice_m29_schleier_c_01.ogg` · Regie: geduldig, gleichgültig · Zeile 3 der Szene
  ```text
  [coldly] Wähle, kleine Flamme. Ich bin so oder so da.
  ```

##### `m29_ende_a` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Schleier versiegeln.

- [ ] `voices/characters/char_deep_voice_m29_ende_a_c_01.ogg` · Regie: ruhig, endgültig · Zeile 3 der Szene
  ```text
  [calmly] Tapfer. Kurz. Und nun … [pause] mein. Willkommen daheim.
  ```

##### `m29_ende_d` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Das Tiefe verbrennen (nur mit zerschlagenem Kern und versiegeltem Riss).

- [ ] `voices/characters/char_deep_voice_m29_ende_d_b_01.ogg` · Regie: bedauernd, samtig · Zeile 2 der Szene
  ```text
  [seductively] Du verbrennst dich selbst, kleine Flamme. Schade. Ich hätte dich so gern behalten.
  ```

#### Ser Wendel der Graue – Stimme `AC_char_wendel` (5 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Banditenhauptmann der Grauen Straße (Mission 3); je nach Entscheidung gehängt oder Verbündeter |
> | Alter | Mitte 50 |
> | Timbre | heiser, rauchig, tiefer Bariton mit Kratzen; klingt, als habe er zu lange im Freien geschlafen |
> | Akzent | Deutsch: rau, bodenständig, leicht norddeutsch gefärbt |
> | Tempo | gemächlich, trockene Pausen vor Pointen |
> | Emotion | müde, spöttisch, ehrlich bis zur Selbstverachtung |
> | Do | trockener Galgenhumor, kurze Seufzer |
> | Don't | Schurkengelächter, Pathos |
>
> **Voice-Design-Prompt:**
> > A weathered man in his mid fifties with a hoarse, smoky baritone, gravel in the throat, a dry and tired tone with
> > sarcastic edges. Slow, unhurried pacing with pauses before the punchline. Speaks German with a natural, native accent:
> > rough, down-to-earth, northern-leaning colloquial German. Studio-quality recording, close microphone, no background
> > noise.
>
> **Beispieltext:** „[tired] Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. [pause] Aber ich kenne jeden
>   Pfad bis Kaldris.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural*; Speed 0,95. **Integration (nicht vom Nutzer):** Standardkette, leichte Wärme (200 Hz), kein Effekt.

##### `m03_wendel` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.

- [ ] `voices/characters/char_wendel_m03_wendel_a_01.ogg` · Regie: heiser, spöttisch, erschöpft · Zeile 1 der Szene
  ```text
  [tired] Haltet ein, Wächterin! Ich lege die Klinge nieder – [pause] ich sehe, wann ein Spiel verloren ist. Und ich hasse es, für Verlierer zu arbeiten.
  ```

- [ ] `voices/characters/char_wendel_m03_wendel_c_01.ogg` · Regie: ehrlich, müde · Zeile 3 der Szene
  ```text
  [tired] Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. Aber ich kenne jeden Pfad zwischen hier und Kaldris – [pause] und meine Männer können kämpfen.
  ```

##### `m03_wendel_a` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Entscheidung: Wendel wird gehängt.

- [ ] `voices/characters/char_wendel_m03_wendel_a_b_01.ogg` · Regie: gefasst, leise · Zeile 2 der Szene
  ```text
  [calmly] [quietly] Gerecht. Fast. Sagt den Fuhrleuten … [pause] dass ich sie gezählt habe.
  ```

##### `m03_wendel_b` · Mission 3 – Die Graue Straße (`m03`)
Kontext: Entscheidung: Wendel und seine Männer werden in Dienst genommen.

- [ ] `voices/characters/char_wendel_m03_wendel_b_b_01.ogg` · Regie: erleichtert, trocken · Zeile 2 der Szene
  ```text
  [relieved] Die Graue Kompanie steht Euch zu Diensten, Wächterin. Es ist das erste Mal seit Jahren, dass ich bei Tag schlafen kann.
  ```

##### `m13_kompanie` · Mission 13 – Die Aschglasfelder (`m13`)
Kontext: Wendel, wenn er in Isoldes Dienst getreten ist.

- [ ] `voices/characters/char_wendel_m13_kompanie_a_01.ogg` · Regie: trocken, kampfeslustig · Zeile 1 der Szene
  ```text
  [dryly] Die Graue Kompanie kennt Wüstenräuber, Wächterin – [pause] Tote in Rüstung sind neu. Aber Tote bluten wenigstens nicht auf den Sold.
  ```

#### Nel – Stimme `AC_char_nel` (5 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Mädchen, das im Schlaf mit der Stimme aus dem Tief spricht; später „Hörerin“ am Riss und beim Hohlen König |
> | Alter | etwa 8 Jahre (in späteren Missionen derselbe Charakter, Stimme unverändert, da im Spiel nur Kurzzeilen) |
> | Timbre | helle, ruhige Kinderstimme, fast ohne Betonung; leicht abwesend |
> | Akzent | Deutsch: neutral, leise, schlichte Kindersprache |
> | Tempo | langsam, gleichmäßig; Sätze enden ohne Frage-Intonation |
> | Emotion | still, nicht ängstlich; unheimlich durch Gelassenheit |
> | Do | monotone Gelassenheit, flüsternahe Nähe |
> | Don't | niedliches Kichern, Weinen |
>
> **Voice-Design-Prompt:**
> > A quiet eight-year-old girl with a clear, calm child's voice, almost flat intonation, slightly distant and dreamlike,
> > speaking slowly and evenly without rising question tones. Speaks German with a natural, native accent, plain child's
> > diction. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:** „[softly] Der Mann im Moor hat gesagt, ich soll dir ausrichten: [pause] Es war nett, dich kennenzulernen.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural*; Speed 0,92; Stability hoch. **Integration (nicht vom Nutzer):** Standardkette, bei Szenen mit der Stimme aus dem Tief ein leichter Hall-Schweif (0,6 s).

##### `m02_ankunft` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.

- [ ] `voices/characters/char_nel_m02_ankunft_b_01.ogg` · Regie: kindlich, monoton, unheimlich · Zeile 2 der Szene
  ```text
  [menacingly] Der Mann im Moor hat gesagt, ich soll dir ausrichten: Es war nett, dich kennenzulernen.
  ```

##### `m02_nel_a` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Entscheidung: Nel bleibt in Brennhollow.

- [ ] `voices/characters/char_nel_m02_nel_a_b_01.ogg` · Regie: leise, kindlich · Zeile 2 der Szene
  ```text
  [quietly] Danke. Hier ist es warm.
  ```

##### `m02_nel_b` · Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)
Kontext: Entscheidung: Nel geht zu den Flammenschwestern.

- [ ] `voices/characters/char_nel_m02_nel_b_b_01.ogg` · Regie: klein, hoffnungsvoll · Zeile 2 der Szene
  ```text
  [hopefully] Kommst du mich besuchen?
  ```

##### `m11_nel` · Mission 11 – Der Riss von Marrowfen (`m11`)
Kontext: Nel, die in Brennhollow aufgenommene Waise, weist den Weg (nur wenn sie bei Hal geblieben ist).

- [ ] `voices/characters/char_nel_m11_nel_a_01.ogg` · Regie: kindlich, abwesend · Zeile 1 der Szene
  ```text
  [calmly] Hört ihr es? Es klingt wie eine Spieluhr, die jemand zu langsam aufzieht. Dort entlang geht es – [pause] das Schilf weiß den Weg.
  ```

##### `m29_ende_c` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).

- [ ] `voices/characters/char_nel_m29_ende_c_c_01.ogg` · Regie: kindlich, klar, leise · Zeile 3 der Szene
  ```text
  [quietly] Sie hat ihn mir gesagt. Im Schlaf. Sie wartet auf dich.
  ```

### B4 Bosse

#### Der Fahle Vogt – Stimme `AC_boss_pale_reeve` (17 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Hüter des Hohlschreins von Vael Dorn; sammelt Seelen als „Zehnt“ für das Hohle Tief |
> | Alter | einst ein Steuervogt Aureths, Mitte 50, seit Jahrhunderten hohl |
> | Timbre | dünne, trockene, hohe Männerstimme, kalt und pedantisch, rasselnder Atem, Flüsteranteil |
> | Akzent | Deutsch: altmodisch-bürokratische Kanzleisprache, überpräzise Konsonanten |
> | Tempo | gleichmäßig, aufzählend, wie beim Verlesen einer Liste; bei Schmerz plötzlich kreischend |
> | Emotion | gefühllose Pflichterfüllung, morbider Ordnungssinn, Gier nach „ausstehenden“ Seelen |
> | Motiv | Zehnt, Schuld, Register, Siegel, Zählen |
> | Don't | brüllender Untoter; Humor nur unfreiwillig-makaber |
>
> **Voice-Design-Prompt:**
> > A thin, dry, high-pitched male voice of an ancient undead tax collector, cold and pedantic, with a rattling, breathy
> > texture as if his lungs are full of dust. Speaks German with a natural, native accent, in old-fashioned bureaucratic
> > diction with over-precise consonants. Even, listing cadence like reading names from a ledger; eerie calm that can snap
> > into a shriek. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[coldly] Brennhollow. Dreißig Seelen. Neunhundert Jahre im Rückstand. Das Hohle Tief ist geduldig, doch das Register
> > muss ausgeglichen werden. [menacingly] Jeder Atemzug, den Ihr tut, ist geliehen, Wächterin. Ich bin gekommen, die
> > Zinsen einzutreiben.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural* (Taunts/Ability), *Creative* (hit/death mit `[shrieks]`, `[groans]`); Speed 0,95.

- Stimme: `AC_boss_pale_reeve` (Profil C4) · Modell: Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,95

##### boss_pale_reeve_attack (Event `voice.boss_pale_reeve.attack`)

- [ ] `voices/units/boss_pale_reeve_attack_01.ogg` · Regie: kalt, beiläufig
  ```text
  [coldly] Euer Zehnt ist fällig.
  ```

- [ ] `voices/units/boss_pale_reeve_attack_02.ogg` · Regie: scharf, lauter
  ```text
  [sharply] Zahlt, was Ihr schuldet!
  ```

- [ ] `voices/units/boss_pale_reeve_attack_03.ogg` · Regie: pedantisch, zischend
  ```text
  [hissing] Jede Seele wird gezählt.
  ```

- [ ] `voices/units/boss_pale_reeve_attack_04.ogg` · Regie: süffisant
  ```text
  [shouting] Ab ins Register mit Euch.
  ```

##### boss_pale_reeve_ability (Event `voice.boss_pale_reeve.ability`)

- [ ] `voices/units/boss_pale_reeve_ability_01.ogg` · Regie: Seelenentzug (`reeve_soul_drain`), langsam, genüsslich
  ```text
  [greedily] [slowly] Ich fordere die Schuld Eures Atems ein.
  ```

- [ ] `voices/units/boss_pale_reeve_ability_02.ogg` · Regie: Beschwörung (`hollow_summon`), geflüstert-befehlend
  ```text
  [commanding] [whispers] Erhebt euch, Säumige. Erhebt euch und dient.
  ```

- [ ] `voices/units/boss_pale_reeve_ability_03.ogg` · Regie: Urteil, eisig
  ```text
  [coldly] Eure Seele ist verwirkt.
  ```

##### boss_pale_reeve_hit (Event `voice.boss_pale_reeve.hit`)

- [ ] `voices/units/boss_pale_reeve_hit_01.ogg` · Regie: empörtes Zischen
  ```text
  [angry] [hisses] Unverschämtheit!
  ```

- [ ] `voices/units/boss_pale_reeve_hit_02.ogg` · Regie: schrill, kippende Stimme
  ```text
  [shrieks] Ihr wagt es, den Vogt zu schlagen?
  ```

- [ ] `voices/units/boss_pale_reeve_hit_03.ogg` · Regie: gekränkt-pedantisch
  ```text
  [groans] Das wird … vermerkt.
  ```

##### boss_pale_reeve_death (Event `voice.boss_pale_reeve.death`)

- [ ] `voices/units/boss_pale_reeve_death_01.ogg` · Regie: Entsetzen, zerfallend, 2–3 s
  ```text
  [shrieks] Nein – das Register … nicht ausgeglichen …
  ```

- [ ] `voices/units/boss_pale_reeve_death_02.ogg` · Regie: Drohung im Sterben
  ```text
  [menacingly] [rattling breath] Das Tief … schickt … einen anderen …
  ```

- [ ] `voices/units/boss_pale_reeve_death_03.ogg` · Regie: fast erlöst, verhauchend
  ```text
  [relieved] [exhales] … vollständig … bezahlt …
  ```

##### boss_pale_reeve_taunt (Event `voice.boss_pale_reeve.taunt`)

- [ ] `voices/units/boss_pale_reeve_taunt_01.ogg` · Regie: verlesend wie aus einem Register
  ```text
  [mockingly] Brennhollow. Dreißig Seelen. Neunhundert Jahre im Rückstand.
  ```

- [ ] `voices/units/boss_pale_reeve_taunt_02.ogg` · Regie: leise, genüsslich
  ```text
  [greedily] [quietly] Hört Ihr die Glocke, Wächterin? Sie läutet für Euer Konto.
  ```

- [ ] `voices/units/boss_pale_reeve_taunt_03.ogg` · Regie: kalt, Pause in der Mitte
  ```text
  [coldly] Das Hohle Tief ist geduldig. Ich bin es nicht.
  ```

- [ ] `voices/units/boss_pale_reeve_taunt_04.ogg` · Regie: trockenes, hohles Lachen
  ```text
  [eerily] [laughs] Lauft nur. Die Toten treiben immer ein.
  ```

#### Der Hohle König – Stimme `AC_boss_hollow_king` (6 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Endgegner; Varkhan selbst, gefangen in Solmere |
> | Alter | sterblich etwa 50 gestorben, seit fast 900 Jahren untot |
> | Timbre | majestätischer, einst goldener Bariton, jetzt brüchig, rissig wie verbranntes Pergament; trockener Atem |
> | Akzent | Deutsch: archaisches, hochformelles königliches Register |
> | Tempo | langsam, souverän, gelegentlich zu lange Pausen (Erinnerungslücken) |
> | Emotion | Größenwahn, verletzter Stolz, tragische Sehnsucht nach dem Sonnenlicht; Wahnsinn bricht kurz durch |
> | Do | Tragik und Würde; Wut als plötzlicher Riss |
> | Don't | Zombie-Gegrunze, reines Monster |
>
> **Voice-Design-Prompt:**
> > A regal male voice of a fallen sun king, a once golden and commanding baritone now cracked and dry like burnt parchment,
> > with a rasping, breathy texture. Speaks German with a natural, native accent, in an archaic, highly formal royal
> > register. Slow, sovereign pacing with unsettling pauses, as if he forgets and then remembers. Proud, tragic and longing,
> > with flashes of barely contained madness. Studio-quality recording, close microphone, no background noise.
>
> **Beispieltext:**
> > „[proudly] Ich war die Sonne Aureths. Könige knieten vor meinem Schatten. Ich öffnete den Schleier, um mein Volk vor
> > dem Tod selbst zu retten … und der Tod antwortete. [bitterly] Blick auf Solmere, kleine Wächterin, und sag mir noch
> > einmal, deine Flamme könne überstrahlen, was ich verloren habe.“
>
> **Modell & Einstellungen:** Eleven v3 *Natural/Robust* (`[laughs]` – ein hohles, kurzes Lachen; `[whispers]`); Speed 0,9.

##### `m29_thron` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Im Thronsaal: Varkhan und die Stimme aus dem Tief.

- [ ] `voices/characters/boss_hollow_king_m29_thron_a_01.ogg` · Regie: archaisch, königlich, hohl · Zeile 1 der Szene
  ```text
  [solemn] Ich war die Sonne von Aureth. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal.
  ```

- [ ] `voices/characters/boss_hollow_king_m29_thron_c_01.ogg` · Regie: plötzlich leise, gequält · Zeile 3 der Szene
  ```text
  [sad] [quietly] Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. Es hat mir geantwortet … [pause] und es hat gelogen.
  ```

##### `m29_schleier` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Der König ist halb gefallen; der Schleier öffnet sich.

- [ ] `voices/characters/boss_hollow_king_m29_schleier_b_01.ogg` · Regie: tonlos, erschöpft · Zeile 2 der Szene
  ```text
  [tired] Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … [pause] jemand muss die Asche tragen.
  ```

- [ ] `voices/characters/boss_hollow_king_m29_schleier_d_01.ogg` · Regie: tonlos, müde · Zeile 4 der Szene
  ```text
  [tired] Wähle, Wächterin. Es gibt keinen vierten Weg – [pause] außer denen, die du dir durch deine Wege verdient hast.
  ```

##### `m29_ende_b` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Hohle Krone nehmen.

- [ ] `voices/characters/boss_hollow_king_m29_ende_b_b_01.ogg` · Regie: hohles, trauriges Lachen, zerfallend · Zeile 2 der Szene
  ```text
  [sad] [laughs] Genau das habe ich auch gesagt.
  ```

##### `m29_ende_c` · Mission 29 – Der Hohle Thron (`m29`)
Kontext: Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).

- [ ] `voices/characters/boss_hollow_king_m29_ende_c_b_01.ogg` · Regie: zitternd, menschlich · Zeile 2 der Szene
  ```text
  [calmly] Sereth … Sereth. Ich habe den Namen so lange nicht gehört. Wer – [pause] wer hat ihn dir gesagt?
  ```

#### Die Wurzelmutter – Stimme `AC_boss_root_mother` (2 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Naturentität des Dornwalds, von der Hohlung angegriffen |
> | Alter | uralt |
> | Timbre | tiefe, warme Frauenstimme, knarzend wie altes Holz, mehrstimmig (Chor-Dopplung durch die Integration) |
> | Akzent | Deutsch, kein Regionalakzent; Wörter wie langsam wachsend, sehr langgezogene Vokale |
> | Tempo | extrem langsam, lange Atembögen, Wortwiederholungen |
> | Emotion | mütterlich und verwundet; im korrumpierten Zustand fiebrig, verwirrt, besitzergreifend |
> | Do | Zärtlichkeit, die in Schmerz umschlägt |
> | Don't | Hexen-Kreischen |
>
> **Voice-Design-Prompt:**
> > An ancient motherly female voice, deep, warm and earthy, with a creaking texture like old wood bending in the wind.
> > Speaks German with a natural, native pronunciation and no regional accent, words stretched slowly as if growing, long
> > sustained vowels and deep breaths. Tender and wounded, drifting into feverish confusion. Studio-quality recording, close
> > microphone, no background noise.
>
> **Beispieltext:**
> > „[sorrowfully] Kinder … meine Kinder mit Äxten und Feuer. Ich nährte eure Väter. Ich barg eure Toten. Nun trinken
> > graue Wurzeln von mir, und ich kann nicht sagen, welche Hände die euren sind. [pleading] Bleibt … bleibt bei mir.
> > Lasst mich euch halten. Lasst die Erde euch wärmen.“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (verderbte Zeilen *Creative*), Speed 0,85.

##### `m10_hain` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.

- [ ] `voices/characters/boss_root_mother_m10_hain_a_01.ogg` · Regie: uralt, zerrissen, mehrstimmig · Zeile 1 der Szene
  ```text
  [eerily] Kinder … so viele Kinder mit Feuer in den Händen … [pause] seid ihr gekommen, um mich zu halten oder um mich zu verbrennen?
  ```

##### `m10_hain_a` · Mission 10 – Wurzel und Asche (`m10`)
Kontext: Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.

- [ ] `voices/characters/boss_root_mother_m10_hain_a_b_01.ogg` · Regie: verlöschend, friedlich · Zeile 2 der Szene
  ```text
  [gently] [quietly] Warm … so warm … [pause] schlaft gut, Kinder …
  ```

#### Gharun, der Steinerne Eid – Stimme `AC_boss_stone_oath` (1 Datei)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Boss der Mission 21; ein in Runenstein gebundener Eid, vom Tiefen verdreht |
> | Alter | zeitlos |
> | Timbre | extrem tiefer, langsamer Bass, mahlend wie Mühlstein, mit natürlichem Höhlenhall |
> | Akzent | Deutsch, kein Regionalakzent; archaisch, wenige Silben je Atemzug |
> | Tempo | sehr langsam, gewichtige Pausen |
> | Emotion | unerbittlich, pflichttreu bis zur Selbstvergessenheit; kein Hass |
> | Do | langgezogene Vokale, Stein-Resonanz (Integration) |
> | Don't | Schreien, Geflüster |
>
> **Voice-Design-Prompt:**
> > An immensely deep, ancient bass voice, slow and grinding like a millstone, with long vowels and heavy pauses, stern and
> > dutiful without hatred. Sounds as if spoken inside a stone hall. Speaks German with a natural, native pronunciation,
> > archaic and heavy. Studio-quality recording, no background noise.
>
> **Beispieltext:** „[commanding] Wer wagt es, den Eid zu stören? [slowly] Ich wurde gebunden, zu bewahren – und ich bewahre.“
>
> **Modell & Einstellungen:** Eleven v3, Stability-Modus *Robust*, Speed 0,85. **Integration (nicht vom Nutzer):** Pitch −3 Halbtöne, Hall 2,8 s (Steinhalle), Tiefpass 6 kHz, leichter Kompressor.

##### `m21_gharun` · Mission 21 – Der Schrein Oruuns (`m21`)
Kontext: Isolde und Gharun im Schreinsaal.

- [ ] `voices/characters/boss_stone_oath_m21_gharun_a_01.ogg` · Regie: steinern, langsam, dröhnend, hallend · Zeile 1 der Szene
  ```text
  [shouting] Wer wagt es, den Eid zu stören? Ich wurde gebunden, zu bewahren – [pause] und ich bewahre. Was ich bewahre, spricht nicht mehr mit euch.
  ```

#### Isarre Mol, Herrin des Schwarzen Riffs – Stimme `AC_boss_reef_mistress` (1 Datei)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> | Merkmal | Festlegung |
> |---|---|
> | Rolle | Boss der Mission 14; ertrunkene Gezeitenpriesterin im Bund mit dem Hohlen Tief |
> | Alter | zeitlos (zu Lebzeiten Mitte 40) |
> | Timbre | tiefe Altstimme, nass und gurgelnd, mehrstimmig überlagert wie Wellen, die einander überlaufen |
> | Akzent | Deutsch, leicht mediterran gefärbt (Saelith), unter dem Wasser verschliffen |
> | Tempo | gleitend, Wellenrhythmus; Sätze enden mit auslaufendem Atem |
> | Emotion | mütterlich-bedrohlich, belustigt; Fürsorge als Drohung |
> | Do | Unterwasser-Textur, Flüstern neben Gesang |
> | Don't | Zischeln, Monsterknurren |
>
> **Voice-Design-Prompt:**
> > A low contralto woman's voice, wet and gurgling, as if speaking from under shallow water, layered with a faint second
> > voice that follows a beat behind like waves washing back. Speaks German with a natural accent and a trace of a
> > Mediterranean lilt. Flowing, motherly and menacing. Studio-quality recording, no background noise.
>
> **Beispieltext:** „[mockingly] Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. [coldly] Das Meer hat mir jedes meiner Kinder
>   zurückgegeben.“
>
> **Modell & Einstellungen:** Eleven v3 *Creative* (`[whispers]`). Geliefert wird nur die trockene Hauptstimme; die tiefere Zweitschicht erzeugt die Integration aus derselben Datei. **Integration (nicht vom Nutzer):** Chorus 12 %, Wasser-Hall (1,8 s), Tiefpass 8 kHz.

##### `m14_isarre` · Mission 14 – Das Schwarze Riff (`m14`)
Kontext: Isarre Mol, auf der Riffinsel.

- [ ] `voices/characters/boss_reef_mistress_m14_isarre_a_01.ogg` · Regie: tief, nass, gurgelnd, mehrstimmig · Zeile 1 der Szene
  ```text
  [eerily] Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. Das Meer hat mir jedes meiner Kinder zurückgegeben – [pause] ich bin nur gekommen, es zu tränken.
  ```

### B5 Einheiten der Glutkrone

#### Arbeiter – Stimme `AC_unit_worker` (33 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Mitte 40, rau-gutmütiger Bariton, breite ländliche Umgangssprache (süddeutsch gefärbt), bodenständig, etwas langsam, müde aber willig, trockene Bauernwitze.
> - **Prompt:** "A weathered peasant labourer in his mid-forties with a rough, good-natured baritone voice. Speaks German
>   with a natural, native accent and a broad rural, southern-leaning dialect coloring. Slow, plain, slightly tired but
>   willing delivery, humble and practical, with a hint of grumbling humour. Close-mic studio-quality recording, no background
>   noise."
> - **Beispieltext:** „[wearily] Jo, ich hol das Holz. Wie gestern, wie morgen. [sighs] Aber wenn die Toten heut Nacht wiederkommen,
>   versteck ich mich hinter dem Ritter, nicht vor ihm.“
> - **Einstellungen:** v3 *Natural*, Speed 0,98.

- Stimme: `AC_unit_worker` (Profil D1.1) · Modell: Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,98
- Haltung: gutmütig-müder Bauer, ländliche Umgangssprache mit leichter süddeutscher Färbung, kein Soldat – Kampfzeilen klingen improvisiert und etwas ängstlich.

##### worker_select (Event `voice.worker.select`)

- [ ] `voices/units/worker_select_01.ogg` · Regie: freundlich-willig, mittlere Lautstärke
  ```text
  [warmly] Jo, was gibt's zu tun?
  ```

- [ ] `voices/units/worker_select_02.ogg` · Regie: kurzer Seufzer, dann gutmütig
  ```text
  [sighs] Noch 'ne Arbeit, was?
  ```

- [ ] `voices/units/worker_select_03.ogg` · Regie: trocken, kleine Pause vor „Noch“
  ```text
  [dryly] Hab die Hände frei. Noch.
  ```

- [ ] `voices/units/worker_select_04.ogg` · Regie: respektvoll, leicht unterwürfig
  ```text
  [respectfully] Ja, Wächterin? Ich hör zu.
  ```

- [ ] `voices/units/worker_select_05.ogg` · Regie: Ächzen in der Stimme, lächelnd
  ```text
  [groans] Rücken tut weh, aber ich bin da.
  ```

##### worker_move (Event `voice.worker.move`)

- [ ] `voices/units/worker_move_01.ogg` · Regie: knapp, bodenständig
  ```text
  [dryly] Gut, bin unterwegs.
  ```

- [ ] `voices/units/worker_move_02.ogg` · Regie: leichtes Brummen, gemütlich
  ```text
  [calmly] Dann mal los.
  ```

- [ ] `voices/units/worker_move_03.ogg` · Regie: zu sich selbst, halblaut
  ```text
  [quietly] Pass auf den Schlamm auf … [pause] geh ja schon.
  ```

- [ ] `voices/units/worker_move_04.ogg` · Regie: selbstsicher
  ```text
  [calmly] Jo, ich kenn den Weg.
  ```

- [ ] `voices/units/worker_move_05.ogg` · Regie: müde, leicht genervt
  ```text
  [tired] Geh ja, geh ja.
  ```

##### worker_attack (Event `voice.worker.attack`)

- [ ] `voices/units/worker_attack_01.ogg` · Regie: verzweifelt-wütend, laut
  ```text
  [angry] [shouting] Weg von unseren Häusern!
  ```

- [ ] `voices/units/worker_attack_02.ogg` · Regie: Mut zusammengekratzt, Stimme kippt leicht
  ```text
  [nervously] Ich hab 'ne Mistgabel und scheu mich nicht!
  ```

- [ ] `voices/units/worker_attack_03.ogg` · Regie: Dorfstolz, kräftig
  ```text
  [proudly] [shouting] Für Brennhollow!
  ```

- [ ] `voices/units/worker_attack_04.ogg` · Regie: nervös, sich selbst Mut machend
  ```text
  [nervously] Schon gut, schon gut – nimm das!
  ```

##### worker_retreat (Event `voice.worker.retreat`)

- [ ] `voices/units/worker_retreat_01.ogg` · Regie: panisch, gehetzt, mit Atem
  ```text
  [panicked] Dafür werd ich nicht bezahlt! Lauft!
  ```

- [ ] `voices/units/worker_retreat_02.ogg` · Regie: laut, Angst
  ```text
  [fearful] [shouting] Zurück ins Dorf!
  ```

- [ ] `voices/units/worker_retreat_03.ogg` · Regie: hastige Selbstkorrektur, unfreiwillig komisch
  ```text
  [quickly] Jeder für sich – [pause] ich mein, Rückzug!
  ```

##### worker_hit (Event `voice.worker.hit`)

- [ ] `voices/units/worker_hit_01.ogg` · Regie: Schmerz + Ärger, abgebrochen
  ```text
  [sorrowfully] [groans] Au! Verflucht –
  ```

- [ ] `voices/units/worker_hit_02.ogg` · Regie: erschrocken, scharf
  ```text
  [fearful] [gasps] Argh, mein Arm!
  ```

- [ ] `voices/units/worker_hit_03.ogg` · Regie: kurzer Stoßlaut, ≤ 0,6 s
  ```text
  [grunts] Uff!
  ```

##### worker_death (Event `voice.worker.death`)

- [ ] `voices/units/worker_death_01.ogg` · Regie: schwach, verlöschend, letzte Sorge
  ```text
  [quietly] [gasps] Sag meiner Frau … das Dach …
  ```

- [ ] `voices/units/worker_death_02.ogg` · Regie: leiser werdend
  ```text
  [quietly] [groans] Ahh … so kalt …
  ```

- [ ] `voices/units/worker_death_03.ogg` · Regie: abgebrochen, letzter Atem
  ```text
  Nein … nicht so – [exhales]
  ```

##### worker_victory (Event `voice.worker.victory`)

- [ ] `voices/units/worker_victory_01.ogg` · Regie: erleichtertes Lachen
  ```text
  [relieved] [laughs] Ha! Steh noch!
  ```

- [ ] `voices/units/worker_victory_02.ogg` · Regie: stolz, brummig
  ```text
  [proudly] Das lehrt sie, hier aufzukreuzen.
  ```

- [ ] `voices/units/worker_victory_03.ogg` · Regie: ungläubige Freude
  ```text
  [proudly] Wir haben's geschafft … wirklich geschafft!
  ```

##### worker_work (Event `voice.worker.work`)

- [ ] `voices/units/worker_work_01.ogg` · Regie: ergeben, freundlich
  ```text
  [warmly] Zurück an die Arbeit.
  ```

- [ ] `voices/units/worker_work_02.ogg` · Regie: rhythmisch, halb gesungen
  ```text
  [casually] Hacken, schleppen, stapeln.
  ```

- [ ] `voices/units/worker_work_03.ogg` · Regie: zufrieden, fachmännisch
  ```text
  [cheerfully] Gutes Holz, das.
  ```

- [ ] `voices/units/worker_work_04.ogg` · Regie: Volksweisheit, trocken
  ```text
  [dryly] Das Feld pflügt sich nicht von allein.
  ```

##### worker_build_done (Event `voice.worker.build_done`)

- [ ] `voices/units/worker_build_done_01.ogg` · Regie: stolz, dann trockene Einschränkung
  ```text
  [proudly] So. Gebaut für die Ewigkeit. Meistens.
  ```

- [ ] `voices/units/worker_build_done_02.ogg` · Regie: laut genug als Meldung
  ```text
  [shouting] Arbeit erledigt, Wächterin!
  ```

- [ ] `voices/units/worker_build_done_03.ogg` · Regie: freudig
  ```text
  [cheerfully] Fertig! Kommt, schaut's euch an!
  ```

#### Schwertkämpfer – Stimme `AC_unit_swordsman` (27 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Ende 20, kerniger, leicht heiserer Bariton, norddeutsch-städtische Färbung, zackig, entschlossen, mit Galgenhumor.
> - **Prompt:** "A young militia swordsman in his late twenties with a firm, slightly hoarse baritone voice. Speaks German
>   with a natural, native accent and a working-class northern urban coloring. Brisk, determined, soldierly delivery with a
>   touch of gallows humour, confident but not arrogant. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[dryly] Schwert scharf, Schild verbeult, und der Feldwebel brüllt immer noch. Muss ein normaler Tag in der Aschmark
>   sein. [confidently] Zeig mir was Totes, und ich mach's noch toter.“
> - **Einstellungen:** v3 *Natural*, Kampfrufe *Creative*.

- Stimme: `AC_unit_swordsman` (Profil D1.2) · Modell: Eleven v3, *Natural* (attack/hit/death *Creative*)
- Haltung: junger Milizsoldat, zackig, Galgenhumor, norddeutsch-städtisch gefärbte Umgangssprache.

##### swordsman_select (Event `voice.swordsman.select`)

- [ ] `voices/units/swordsman_select_01.ogg` · Regie: zackig, knapp
  ```text
  [commanding] Schwert bereit.
  ```

- [ ] `voices/units/swordsman_select_02.ogg` · Regie: soldatisch, Haltung annehmen
  ```text
  [calmly] Miliz, zu Befehl.
  ```

- [ ] `voices/units/swordsman_select_03.ogg` · Regie: entschlossen, leicht grinsend
  ```text
  [amused] Zeigt mir den Ärger.
  ```

- [ ] `voices/units/swordsman_select_04.ogg` · Regie: wach, aufmerksam
  ```text
  [calmly] Schild oben. Was jetzt?
  ```

- [ ] `voices/units/swordsman_select_05.ogg` · Regie: Galgenhumor
  ```text
  [amused] Atme noch. Was ist der Befehl?
  ```

##### swordsman_move (Event `voice.swordsman.move`)

- [ ] `voices/units/swordsman_move_01.ogg` · Regie: knapp
  ```text
  [dryly] Rücken aus.
  ```

- [ ] `voices/units/swordsman_move_02.ogg` · Regie: energisch
  ```text
  [firmly] Im Laufschritt!
  ```

- [ ] `voices/units/swordsman_move_03.ogg` · Regie: loyal
  ```text
  [calmly] Direkt hinter Euch, Wächterin.
  ```

- [ ] `voices/units/swordsman_move_04.ogg` · Regie: neutral, militärisch
  ```text
  [commanding] Marschieren.
  ```

- [ ] `voices/units/swordsman_move_05.ogg` · Regie: leicht resigniert, entschlossen
  ```text
  [firmly] Dann eben durch die Asche.
  ```

##### swordsman_attack (Event `voice.swordsman.attack`)

- [ ] `voices/units/swordsman_attack_01.ogg` · Regie: Kampfschrei, voll
  ```text
  [shouting] Haut sie nieder!
  ```

- [ ] `voices/units/swordsman_attack_02.ogg` · Regie: Schlachtruf, Stolz
  ```text
  [proudly] [shouting] Für die Glutkrone!
  ```

- [ ] `voices/units/swordsman_attack_03.ogg` · Regie: anfeuernd
  ```text
  [shouting] Stahl, Jungs! Stahl!
  ```

- [ ] `voices/units/swordsman_attack_04.ogg` · Regie: wütend, verächtlich
  ```text
  [angry] Zurück in eure Gräber!
  ```

- [ ] `voices/units/swordsman_attack_05.ogg` · Regie: Anstrengung im Schlag
  ```text
  [grunts] Da, nimm das!
  ```

##### swordsman_retreat (Event `voice.swordsman.retreat`)

- [ ] `voices/units/swordsman_retreat_01.ogg` · Regie: laut, dringlich
  ```text
  [urgently] [shouting] Rückzug! Rückzug!
  ```

- [ ] `voices/units/swordsman_retreat_02.ogg` · Regie: befehlend, außer Atem
  ```text
  [commanding] [panting] Sammeln am Bergfried!
  ```

- [ ] `voices/units/swordsman_retreat_03.ogg` · Regie: Angst, gehetzt
  ```text
  [fearful] Zurück, wir werden überrannt!
  ```

##### swordsman_hit (Event `voice.swordsman.hit`)

- [ ] `voices/units/swordsman_hit_01.ogg` · Regie: kurzer Schmerzlaut
  ```text
  [sorrowfully] [grunts] Argh!
  ```

- [ ] `voices/units/swordsman_hit_02.ogg` · Regie: trotzig
  ```text
  [firmly] [groans] Ngh – nur ein Kratzer!
  ```

- [ ] `voices/units/swordsman_hit_03.ogg` · Regie: scharf, überrascht
  ```text
  [surprised] [gasps] Verdammt!
  ```

##### swordsman_death (Event `voice.swordsman.death`)

- [ ] `voices/units/swordsman_death_01.ogg` · Regie: letzter Atem
  ```text
  [gasps] Für … die Krone …
  ```

- [ ] `voices/units/swordsman_death_02.ogg` · Regie: schwach, stolz
  ```text
  [proudly] [quietly] Sagt dem Feldwebel … [pause] ich hab gehalten …
  ```

- [ ] `voices/units/swordsman_death_03.ogg` · Regie: verlöschend
  ```text
  [quietly] [exhales] Das Feuer … geht aus …
  ```

##### swordsman_victory (Event `voice.swordsman.victory`)

- [ ] `voices/units/swordsman_victory_01.ogg` · Regie: Jubel
  ```text
  [triumphantly] [shouting] Ha! Sieg!
  ```

- [ ] `voices/units/swordsman_victory_02.ogg` · Regie: zufrieden, trocken
  ```text
  [dryly] Die stehen nicht wieder auf.
  ```

- [ ] `voices/units/swordsman_victory_03.ogg` · Regie: stolz
  ```text
  [proudly] Die Krone steht!
  ```

#### Speerträger – Stimme `AC_unit_spearman` (24 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Mitte 50, tiefer, rauer, ruhiger Bass-Bariton, ländlich-bodenständige Färbung, wortkarg, unerschütterlich, väterlich gegenüber Jüngeren.
> - **Prompt:** "A grizzled veteran spearman in his mid-fifties with a deep, gravelly, calm bass-baritone voice. Speaks
>   German with a natural, native accent and a rural, down-to-earth coloring. Laconic, steady and unshakeable, few words, each
>   one weighty, a protective old soldier. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[commanding] Haltet die Linie, Jungs. Füße fest, Speer nach vorn, lasst sie kommen. [calmly] Ich hab bessere Männer
>   begraben als diese Dinger, und ich überlebe die auch.“
> - **Einstellungen:** v3 *Natural/Robust*, Speed 0,95.

- Stimme: `AC_unit_spearman` (Profil D1.3) · Modell: Eleven v3, *Natural/Robust* · Speed 0,95
- Haltung: wortkarger Veteran, tief und ruhig; selbst im Angriff kontrolliert, nie hysterisch.

##### spearman_select (Event `voice.spearman.select`)

- [ ] `voices/units/spearman_select_01.ogg` · Regie: ruhig, tief
  ```text
  [calmly] Speer bereit.
  ```

- [ ] `voices/units/spearman_select_02.ogg` · Regie: wortkarg
  ```text
  [calmly] Jo. Sprich.
  ```

- [ ] `voices/units/spearman_select_03.ogg` · Regie: trockener Humor
  ```text
  [amused] Alte Knochen, scharfe Spitze.
  ```

- [ ] `voices/units/spearman_select_04.ogg` · Regie: gelassen
  ```text
  [calmly] Linie hält. Was gibt's?
  ```

##### spearman_move (Event `voice.spearman.move`)

- [ ] `voices/units/spearman_move_01.ogg` · Regie: neutral
  ```text
  [calmly] In Bewegung.
  ```

- [ ] `voices/units/spearman_move_02.ogg` · Regie: an die Truppe gerichtet
  ```text
  [calmly] Gleichmäßiger Schritt, Jungs.
  ```

- [ ] `voices/units/spearman_move_03.ogg` · Regie: gelassen
  ```text
  [calmly] Jo, wir gehen.
  ```

- [ ] `voices/units/spearman_move_04.ogg` · Regie: Kommandoton, nicht laut
  ```text
  [commanding] [shouting] Speere hoch. Marsch.
  ```

##### spearman_attack (Event `voice.spearman.attack`)

- [ ] `voices/units/spearman_attack_01.ogg` · Regie: Kommandoruf
  ```text
  [commanding] [shouting] Halten und zustoßen!
  ```

- [ ] `voices/units/spearman_attack_02.ogg` · Regie: kräftig
  ```text
  [firmly] Speere nach vorn!
  ```

- [ ] `voices/units/spearman_attack_03.ogg` · Regie: grimmig, provozierend
  ```text
  [shouting] Na komm. Komm in die Spitze!
  ```

- [ ] `voices/units/spearman_attack_04.ogg` · Regie: Timing: Pause, dann Ausbruch
  ```text
  [angry] Abstützen … jetzt – stoßt!
  ```

##### spearman_retreat (Event `voice.spearman.retreat`)

- [ ] `voices/units/spearman_retreat_01.ogg` · Regie: kontrolliert
  ```text
  [calmly] Zurück, langsam und ruhig!
  ```

- [ ] `voices/units/spearman_retreat_02.ogg` · Regie: Kommandoton
  ```text
  [commanding] Rückzug! Speere zum Feind!
  ```

- [ ] `voices/units/spearman_retreat_03.ogg` · Regie: väterlich-bestimmt
  ```text
  [firmly] Boden aufgeben, Jungs. Nicht rennen.
  ```

##### spearman_hit (Event `voice.spearman.hit`)

- [ ] `voices/units/spearman_hit_01.ogg` · Regie: kaum beeindruckt
  ```text
  [grunts] Hmpf.
  ```

- [ ] `voices/units/spearman_hit_02.ogg` · Regie: gepresst
  ```text
  [groans] Hatte Schlimmeres.
  ```

- [ ] `voices/units/spearman_hit_03.ogg` · Regie: kurzer Stoß
  ```text
  [exhales sharply] Nngh!
  ```

##### spearman_death (Event `voice.spearman.death`)

- [ ] `voices/units/spearman_death_01.ogg` · Regie: letzter Befehl
  ```text
  [commanding] [exhales] Haltet … die Linie …
  ```

- [ ] `voices/units/spearman_death_02.ogg` · Regie: fast erleichtert
  ```text
  [relieved] Endlich … Ruhe …
  ```

- [ ] `voices/units/spearman_death_03.ogg` · Regie: leises Lächeln im Sterben
  ```text
  [quietly] [groans] Nicht schlecht … für einen alten Mann …
  ```

##### spearman_victory (Event `voice.spearman.victory`)

- [ ] `voices/units/spearman_victory_01.ogg` · Regie: knapp, zufrieden
  ```text
  [dryly] Linie gehalten. Gut.
  ```

- [ ] `voices/units/spearman_victory_02.ogg` · Regie: stolz-väterlich
  ```text
  [proudly] So macht man das, Jungs.
  ```

- [ ] `voices/units/spearman_victory_03.ogg` · Regie: düster-trocken
  ```text
  [menacingly] Begrabt sie. Diesmal richtig.
  ```

#### Bogenschützin – Stimme `AC_unit_archer` (26 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Frau, Anfang 20, helle, scharfe, klare Stimme, leichte ländliche Färbung, wach, schnell, keck, präzise.
> - **Prompt:** "A sharp-eyed young woman archer in her early twenties with a bright, clear and focused voice. Speaks
>   German with a natural, native accent and a light rural coloring. Quick, alert and cheeky delivery, precise and confident,
>   a hunter's calm when aiming. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[confidently] Wind von Osten, zwei Finger links. Ich treff eine Krähe auf hundert Schritt, bevor du fertig gezeigt
>   hast. [teasingly] Na los, gib mir was, das sich lohnt.“
> - **Einstellungen:** v3 *Natural*, Speed 1,03.

- Stimme: `AC_unit_archer` (Profil D1.4) · Modell: Eleven v3, *Natural* · Speed 1,03
- Haltung: junge, wache Jägerin, keck, präzise; beim Zielen ruhig, beim Treffer triumphierend.

##### archer_select (Event `voice.archer.select`)

- [ ] `voices/units/archer_select_01.ogg` · Regie: knapp, wach
  ```text
  [dryly] Bogen gespannt.
  ```

- [ ] `voices/units/archer_select_02.ogg` · Regie: konzentriert
  ```text
  [calmly] Augen aufs Feld.
  ```

- [ ] `voices/units/archer_select_03.ogg` · Regie: keck
  ```text
  [calmly] Worauf schieß ich?
  ```

- [ ] `voices/units/archer_select_04.ogg` · Regie: militärisch
  ```text
  [commanding] Schützin bereit.
  ```

- [ ] `voices/units/archer_select_05.ogg` · Regie: selbstbewusst, leicht spöttisch
  ```text
  [mockingly] Freie Schussbahn. Wer ist zuerst dran?
  ```

##### archer_move (Event `voice.archer.move`)

- [ ] `voices/units/archer_move_01.ogg` · Regie: sachlich
  ```text
  [seriously] Beziehe Stellung.
  ```

- [ ] `voices/units/archer_move_02.ogg` · Regie: geflüstert, verschwörerisch
  ```text
  [softly] [whispers] Schnell und leise.
  ```

- [ ] `voices/units/archer_move_03.ogg` · Regie: zielstrebig
  ```text
  [calmly] Such mir 'ne Anhöhe.
  ```

- [ ] `voices/units/archer_move_04.ogg` · Regie: flink
  ```text
  [calmly] Bin dran.
  ```

##### archer_attack (Event `voice.archer.attack`)

- [ ] `voices/units/archer_attack_01.ogg` · Regie: scharfer Ruf
  ```text
  [shouting] Los!
  ```

- [ ] `voices/units/archer_attack_02.ogg` · Regie: kräftig
  ```text
  [firmly] Pfeile ab!
  ```

- [ ] `voices/units/archer_attack_03.ogg` · Regie: Kampfeslust
  ```text
  [shouting] Spickt sie mit Federn!
  ```

- [ ] `voices/units/archer_attack_04.ogg` · Regie: triumphierend
  ```text
  [triumphantly] Hab dich!
  ```

- [ ] `voices/units/archer_attack_05.ogg` · Regie: ruhige Atempause vor dem Schuss
  ```text
  [calmly] Ziel … und los!
  ```

##### archer_retreat (Event `voice.archer.retreat`)

- [ ] `voices/units/archer_retreat_01.ogg` · Regie: gehetzt
  ```text
  [urgently] Ziehe mich zurück!
  ```

- [ ] `voices/units/archer_retreat_02.ogg` · Regie: alarmiert
  ```text
  [urgently] Zu nah! Rückzug!
  ```

- [ ] `voices/units/archer_retreat_03.ogg` · Regie: Panik mit Restwitz
  ```text
  [panicked] Kann nicht schießen, was auf mir hockt – zurück!
  ```

##### archer_hit (Event `voice.archer.hit`)

- [ ] `voices/units/archer_hit_01.ogg` · Regie: kurzer Schreck
  ```text
  [fearful] [gasps] Ah!
  ```

- [ ] `voices/units/archer_hit_02.ogg` · Regie: empört, Schmerz
  ```text
  [angry] [grunts] Die haben mich erwischt!
  ```

- [ ] `voices/units/archer_hit_03.ogg` · Regie: trotzig
  ```text
  [firmly] [groans] Ngh – schieß trotzdem!
  ```

##### archer_death (Event `voice.archer.death`)

- [ ] `voices/units/archer_death_01.ogg` · Regie: ungläubig, leise
  ```text
  [quietly] [gasps] Ich … hab verfehlt …
  ```

- [ ] `voices/units/archer_death_02.ogg` · Regie: verlöschend
  ```text
  [quietly] Ah … es wird dunkel …
  ```

- [ ] `voices/units/archer_death_03.ogg` · Regie: schwacher Stolz
  ```text
  [proudly] [exhales] Sagt ihnen … ich hab den Letzten … erwischt …
  ```

##### archer_victory (Event `voice.archer.victory`)

- [ ] `voices/units/archer_victory_01.ogg` · Regie: jubelnd
  ```text
  [triumphantly] Nicht einer kam durch!
  ```

- [ ] `voices/units/archer_victory_02.ogg` · Regie: stolzes Lachen
  ```text
  [proudly] [laughs] Saubere Schüsse, jeder einzelne.
  ```

- [ ] `voices/units/archer_victory_03.ogg` · Regie: kühl, selbstbewusst
  ```text
  [coldly] Zählt meine Pfeile. Dann zählt ihre Toten.
  ```

#### Ritter – Stimme `AC_unit_knight` (26 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Anfang 40, voller, edler Bariton mit Brustklang, gehobenes Hochdeutsch (Bühnenaussprache), stolz, ritterlich-formell, etwas pathetisch (bewusst kontrastiert zu Isolde).
> - **Prompt:** "A noble armoured knight in his early forties with a full, resonant and commanding baritone voice. Speaks
>   German with a natural, native accent in refined, noble stage-standard pronunciation. Proud, chivalrous and formal delivery
>   with a touch of grandeur, unwavering and honourable. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[solemn] Bei der Ewigen Flamme und dem Eid meines Hauses: Kein Schatten überquert dieses Feld, solange ich im Sattel
>   sitze. [commanding] Blast das Horn, und lasst die Toten lernen, was ein Ansturm Valemarks bedeutet.“
> - **Einstellungen:** v3 *Natural*, Speed 0,97.

- Stimme: `AC_unit_knight` (Profil D1.5) · Modell: Eleven v3, *Natural* · Speed 0,97
- Haltung: edel, ritterlich, etwas pathetisch (bewusster Kontrast zu Isoldes Understatement). Kein Helmfilter in der Basisaufnahme.

##### knight_select (Event `voice.knight.select`)

- [ ] `voices/units/knight_select_01.ogg` · Regie: edel, aufrecht
  ```text
  [calmly] Euer Befehl?
  ```

- [ ] `voices/units/knight_select_02.ogg` · Regie: stolz
  ```text
  [proudly] Stahl und Sattel, bereit.
  ```

- [ ] `voices/units/knight_select_03.ogg` · Regie: feierlich
  ```text
  [solemn] Die Lanze der Krone wartet.
  ```

- [ ] `voices/units/knight_select_04.ogg` · Regie: formell
  ```text
  [calmly] Sprecht, und ich reite.
  ```

- [ ] `voices/units/knight_select_05.ogg` · Regie: gläubig, fest
  ```text
  [firmly] Bei der Flamme, ich bin bereit.
  ```

##### knight_move (Event `voice.knight.move`)

- [ ] `voices/units/knight_move_01.ogg` · Regie: knapp, sicher
  ```text
  [dryly] Reite aus.
  ```

- [ ] `voices/units/knight_move_02.ogg` · Regie: kraftvoll
  ```text
  [firmly] Vorwärts!
  ```

- [ ] `voices/units/knight_move_03.ogg` · Regie: Reiterjargon, zackig
  ```text
  [commanding] Sporen in die Flanke.
  ```

- [ ] `voices/units/knight_move_04.ogg` · Regie: höflich-formell
  ```text
  [calmly] Wie Ihr befehlt.
  ```

- [ ] `voices/units/knight_move_05.ogg` · Regie: selbstbewusst
  ```text
  [calmly] Die Straße gehört uns.
  ```

##### knight_attack (Event `voice.knight.attack`)

- [ ] `voices/units/knight_attack_01.ogg` · Regie: maximaler Schlachtruf
  ```text
  [shouting] Angriff!
  ```

- [ ] `voices/units/knight_attack_02.ogg` · Regie: pathetisch, voll
  ```text
  [dramatically] [shouting] Für Valemark und die Flamme!
  ```

- [ ] `voices/units/knight_attack_03.ogg` · Regie: befehlend
  ```text
  [commanding] Reitet sie nieder!
  ```

- [ ] `voices/units/knight_attack_04.ogg` · Regie: Spannung, dann Ausbruch
  ```text
  [angry] Lanzen … jetzt!
  ```

##### knight_retreat (Event `voice.knight.retreat`)

- [ ] `voices/units/knight_retreat_01.ogg` · Regie: Kommando, laut
  ```text
  [commanding] [shouting] Kehrt! Rückzug!
  ```

- [ ] `voices/units/knight_retreat_02.ogg` · Regie: dringlich
  ```text
  [urgently] Zurück zu den Linien!
  ```

- [ ] `voices/units/knight_retreat_03.ogg` · Regie: Würde wahrend, leicht verlegen
  ```text
  [nervously] Ein taktischer Rückzug. Reitet!
  ```

##### knight_hit (Event `voice.knight.hit`)

- [ ] `voices/units/knight_hit_01.ogg` · Regie: verächtlich
  ```text
  [mockingly] [grunts] Ha – nur gestreift!
  ```

- [ ] `voices/units/knight_hit_02.ogg` · Regie: gepresst, trotzig
  ```text
  [firmly] [groans] Meine Rüstung hält!
  ```

- [ ] `voices/units/knight_hit_03.ogg` · Regie: empört
  ```text
  [angry] [gasps] Ngh! Unverschämt!
  ```

##### knight_death (Event `voice.knight.death`)

- [ ] `voices/units/knight_death_01.ogg` · Regie: würdevoll sterbend
  ```text
  [solemn] [gasps] Mein Eid … erfüllt …
  ```

- [ ] `voices/units/knight_death_02.ogg` · Regie: leises Gebet
  ```text
  [quietly] Vergib mir … Flamme …
  ```

- [ ] `voices/units/knight_death_03.ogg` · Regie: letzte Kraft
  ```text
  [exhales] Sagt meinem Haus … [pause] ich wich nicht …
  ```

##### knight_victory (Event `voice.knight.victory`)

- [ ] `voices/units/knight_victory_01.ogg` · Regie: triumphal
  ```text
  [triumphantly] [shouting] Ruhm der Glutkrone!
  ```

- [ ] `voices/units/knight_victory_02.ogg` · Regie: stolz
  ```text
  [proudly] Das Feld gehört uns!
  ```

- [ ] `voices/units/knight_victory_03.ogg` · Regie: ritterlich, anerkennend
  ```text
  [proudly] Ein würdiger Ansturm. Gut gekämpft.
  ```

#### Armbrustschütze – Stimme `AC_unit_crossbowman` (24 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Anfang 40, trockener, ruhiger Bariton, nüchterner Söldnerveteran aus den Grenzgarnisonen, knappe Sätze, spricht über Reichweite wie über Handwerk.
> - **Prompt:** "A seasoned crossbow mercenary in his early forties with a dry, level baritone. Speaks German with a natural, native accent and a plain northern coloring. Short, matter-of-fact sentences, sober and unhurried, speaking of range and aim like a craftsman talks about tools, with a faint grim humour. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[calmly] Atem anhalten, warten bis der Wind nachlässt, dann abdrücken. [short pause] Schnell schießen kann jeder. Ich
>   schieße einmal.“
> - **Einstellungen:** v3 *Natural*, Speed 1,0.

- Stimme: `AC_unit_crossbowman (Profil G1)` · Modell: Eleven v3, *Natural* · Speed 1,0
- Haltung: nüchterner Söldnerveteran aus den Grenzgarnisonen, kurze, trockene Sätze, spricht über Reichweite und Treffer wie über Handwerk.

##### crossbowman_select (Event `voice.crossbowman.select`)

- [ ] `voices/units/crossbowman_select_01.ogg` · Regie: knapp, sachlich
  ```text
  [seriously] Armbrust bereit.
  ```

- [ ] `voices/units/crossbowman_select_02.ogg` · Regie: trocken, leise
  ```text
  [dryly] [quietly] Bolzen liegt in der Rinne.
  ```

- [ ] `voices/units/crossbowman_select_03.ogg` · Regie: ruhig-selbstsicher
  ```text
  [calmly] Jawohl. Ruhige Hände.
  ```

- [ ] `voices/units/crossbowman_select_04.ogg` · Regie: leicht grimmig
  ```text
  [calmly] Sagt's, und ich schieß einen durch.
  ```

##### crossbowman_move (Event `voice.crossbowman.move`)

- [ ] `voices/units/crossbowman_move_01.ogg` · Regie: knapp
  ```text
  [dryly] Rücke vor.
  ```

- [ ] `voices/units/crossbowman_move_02.ogg` · Regie: ruhig
  ```text
  [calmly] Haltet die Linie, ich komme nach.
  ```

- [ ] `voices/units/crossbowman_move_03.ogg` · Regie: beiläufig, mit Metallklicken
  ```text
  [calmly] Spanne im Gehen.
  ```

- [ ] `voices/units/crossbowman_move_04.ogg` · Regie: neutral
  ```text
  [calmly] Bin unterwegs.
  ```

##### crossbowman_attack (Event `voice.crossbowman.attack`)

- [ ] `voices/units/crossbowman_attack_01.ogg` · Regie: scharf, laut
  ```text
  [sharply] [shouting] Schuss!
  ```

- [ ] `voices/units/crossbowman_attack_02.ogg` · Regie: grimmig
  ```text
  [shouting] Der hier durchschlägt Panzer!
  ```

- [ ] `voices/units/crossbowman_attack_03.ogg` · Regie: hart
  ```text
  [firmly] Runter mit dir!
  ```

- [ ] `voices/units/crossbowman_attack_04.ogg` · Regie: rhythmisch, zweiteilig
  ```text
  [shouting] Spannen … und Schuss!
  ```

##### crossbowman_retreat (Event `voice.crossbowman.retreat`)

- [ ] `voices/units/crossbowman_retreat_01.ogg` · Regie: gehetzt
  ```text
  [urgently] Zurück, ich lade nach!
  ```

- [ ] `voices/units/crossbowman_retreat_02.ogg` · Regie: angespannt
  ```text
  [worried] Zu nah! Deckt mich!
  ```

- [ ] `voices/units/crossbowman_retreat_03.ogg` · Regie: laut
  ```text
  [shouting] Zurück in die Linie!
  ```

##### crossbowman_hit (Event `voice.crossbowman.hit`)

- [ ] `voices/units/crossbowman_hit_01.ogg` · Regie: Schmerz, kurz
  ```text
  [sorrowfully] [grunts] Mein Arm!
  ```

- [ ] `voices/units/crossbowman_hit_02.ogg` · Regie: überrascht
  ```text
  [surprised] [gasps] Getroffen!
  ```

- [ ] `voices/units/crossbowman_hit_03.ogg` · Regie: ärgerlich
  ```text
  [grunts] Nicht die Sehne, verflucht!
  ```

##### crossbowman_death (Event `voice.crossbowman.death`)

- [ ] `voices/units/crossbowman_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Kalt … so kalt …
  ```

- [ ] `voices/units/crossbowman_death_02.ogg` · Regie: Pflichtgefühl bis zuletzt
  ```text
  [gasps] Nachladen … jemand …
  ```

- [ ] `voices/units/crossbowman_death_03.ogg` · Regie: stolz-müde
  ```text
  [tired] [groans] Ich hab … nichts verfehlt …
  ```

##### crossbowman_victory (Event `voice.crossbowman.victory`)

- [ ] `voices/units/crossbowman_victory_01.ogg` · Regie: zufrieden, ruhig
  ```text
  [calmly] Saubere Arbeit, alle.
  ```

- [ ] `voices/units/crossbowman_victory_02.ogg` · Regie: trocken
  ```text
  [dryly] Das war der letzte Bolzen für heute.
  ```

- [ ] `voices/units/crossbowman_victory_03.ogg` · Regie: müde-trocken
  ```text
  [tired] Zählt die Bolzen, nicht die Toten.
  ```

#### Hellebardier – Stimme `AC_unit_halberdier` (24 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Mitte 40, tiefer, bärbeißiger Bass-Bariton, Gardist und Pferdebrecher; liebt Formationen, mag keine Reiter.
> - **Prompt:** "A gruff palace guard halberdier in his mid-forties with a deep, grumbling bass-baritone. Speaks German with a natural, native accent, clipped and disciplined, with a northern coloring. Slow, heavy and authoritative, barking short orders, with the stubborn pride of a man who has braced against many cavalry charges. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[firmly] Schäfte auf den Boden, Spitzen raus, Schulter an Schulter. [calmly] Lasst die Pferde kommen. Irgendwann
>   bleiben sie immer stehen.“
> - **Einstellungen:** v3 *Natural*, Speed 0,95.

- Stimme: `AC_unit_halberdier (Profil G2)` · Modell: Eleven v3, *Natural* · Speed 0,95
- Haltung: bärbeißiger Gardist mit tiefer Stimme, ein Pferdebrecher; liebt Formationen, mag keine Reiter.

##### halberdier_select (Event `voice.halberdier.select`)

- [ ] `voices/units/halberdier_select_01.ogg` · Regie: tief, knapp
  ```text
  [dryly] Hellebarde hoch.
  ```

- [ ] `voices/units/halberdier_select_02.ogg` · Regie: steif, stolz
  ```text
  [proudly] Zur Stelle und bereit.
  ```

- [ ] `voices/units/halberdier_select_03.ogg` · Regie: grimmig-amüsiert
  ```text
  [amused] Sollen sie nur zu Pferd kommen.
  ```

- [ ] `voices/units/halberdier_select_04.ogg` · Regie: respektvoll
  ```text
  [respectfully] Befehle, Wächterin?
  ```

##### halberdier_move (Event `voice.halberdier.move`)

- [ ] `voices/units/halberdier_move_01.ogg` · Regie: knapp
  ```text
  [dryly] Marschiere.
  ```

- [ ] `voices/units/halberdier_move_02.ogg` · Regie: kommandoartig
  ```text
  [commanding] Gleichmäßiger Schritt, Stangen waagerecht.
  ```

- [ ] `voices/units/halberdier_move_03.ogg` · Regie: brummig
  ```text
  [gruffly] Schwere Stiefel, ehrliche Straße.
  ```

- [ ] `voices/units/halberdier_move_04.ogg` · Regie: neutral
  ```text
  [calmly] Rücken ab.
  ```

##### halberdier_attack (Event `voice.halberdier.attack`)

- [ ] `voices/units/halberdier_attack_01.ogg` · Regie: laut, kommandierend
  ```text
  [commanding] [shouting] Anlegen! Anlegen!
  ```

- [ ] `voices/units/halberdier_attack_02.ogg` · Regie: Schlachtruf
  ```text
  [shouting] Pferdebrecher!
  ```

- [ ] `voices/units/halberdier_attack_03.ogg` · Regie: grimmig, Anstrengung
  ```text
  [shouting] Haken und zerren!
  ```

- [ ] `voices/units/halberdier_attack_04.ogg` · Regie: kräftig
  ```text
  [firmly] Für die Krone!
  ```

##### halberdier_retreat (Event `voice.halberdier.retreat`)

- [ ] `voices/units/halberdier_retreat_01.ogg` · Regie: laut
  ```text
  [shouting] Zurück, zurück!
  ```

- [ ] `voices/units/halberdier_retreat_02.ogg` · Regie: befehlend
  ```text
  [commanding] Linie neu bilden!
  ```

- [ ] `voices/units/halberdier_retreat_03.ogg` · Regie: gepresst
  ```text
  [urgently] Zurück, verflucht!
  ```

##### halberdier_hit (Event `voice.halberdier.hit`)

- [ ] `voices/units/halberdier_hit_01.ogg` · Regie: trotzig
  ```text
  [firmly] [grunts] Rüstung hält.
  ```

- [ ] `voices/units/halberdier_hit_02.ogg` · Regie: trocken
  ```text
  [dryly] [groans] Das gibt einen Bluterguss.
  ```

- [ ] `voices/units/halberdier_hit_03.ogg` · Regie: kurz
  ```text
  [hisses] Ah!
  ```

##### halberdier_death (Event `voice.halberdier.death`)

- [ ] `voices/units/halberdier_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Haltet die Linie …
  ```

- [ ] `voices/units/halberdier_death_02.ogg` · Regie: schwer
  ```text
  [seriously] [groans] Vergebt mir, Hauptmann.
  ```

- [ ] `voices/units/halberdier_death_03.ogg` · Regie: gebrochen-stur
  ```text
  [sad] [gasps] Nicht … einen Schritt …
  ```

##### halberdier_victory (Event `voice.halberdier.victory`)

- [ ] `voices/units/halberdier_victory_01.ogg` · Regie: zufrieden
  ```text
  [cheerfully] Kein Huf kam an uns vorbei.
  ```

- [ ] `voices/units/halberdier_victory_02.ogg` · Regie: ruhig
  ```text
  [calmly] Stangen runter. Gut gemacht.
  ```

- [ ] `voices/units/halberdier_victory_03.ogg` · Regie: stolz
  ```text
  [proudly] Die Linie hielt.
  ```

#### Schildträger – Stimme `AC_unit_shieldbearer` (27 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, ca. 38, sehr tiefer, ruhiger Bass; geduldiger, schwerer Bollwerkkämpfer, spricht wenig, stoisch.
> - **Prompt:** "A heavy shieldbearer in his late thirties with a very deep, calm bass voice, slow and stoic, speaking little and never raising his voice above a firm rumble. Speaks German with a natural, native accent, plain, steady, patient, with a northern coloring, the quiet confidence of a living wall. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[firmly] Steht hinter mir. Der Schild ist schwer, aber ich bin schwerer. [calmly] Hier kommt nichts durch.“
> - **Einstellungen:** v3 *Natural*, Speed 0,92.

- Stimme: `AC_unit_shieldbearer (Profil G3)` · Modell: Eleven v3, *Natural* · Speed 0,92
- Haltung: geduldiger, schwerer Bollwerkkämpfer; spricht wenig, sehr tief, mit stoischer Ruhe.

##### shieldbearer_select (Event `voice.shieldbearer.select`)

- [ ] `voices/units/shieldbearer_select_01.ogg` · Regie: tief, ruhig
  ```text
  [calmly] Schild hoch.
  ```

- [ ] `voices/units/shieldbearer_select_02.ogg` · Regie: stoisch
  ```text
  [calmly] Ich bin Eure Mauer.
  ```

- [ ] `voices/units/shieldbearer_select_03.ogg` · Regie: ruhig-bestimmt
  ```text
  [firmly] Hinter mich.
  ```

- [ ] `voices/units/shieldbearer_select_04.ogg` · Regie: einsilbig
  ```text
  [calmly] Bereit.
  ```

##### shieldbearer_move (Event `voice.shieldbearer.move`)

- [ ] `voices/units/shieldbearer_move_01.ogg` · Regie: gleichmäßig
  ```text
  [calmly] Rücke vor, Schilde verhakt.
  ```

- [ ] `voices/units/shieldbearer_move_02.ogg` · Regie: brummend
  ```text
  [calmly] Langsam und stetig.
  ```

- [ ] `voices/units/shieldbearer_move_03.ogg` · Regie: knapp
  ```text
  [dryly] Nach vorn.
  ```

- [ ] `voices/units/shieldbearer_move_04.ogg` · Regie: neutral
  ```text
  [calmly] Marschiere.
  ```

##### shieldbearer_attack (Event `voice.shieldbearer.attack`)

- [ ] `voices/units/shieldbearer_attack_01.ogg` · Regie: laut, kommandierend
  ```text
  [commanding] [shouting] Schildwall!
  ```

- [ ] `voices/units/shieldbearer_attack_02.ogg` · Regie: Anstrengung
  ```text
  [shouting] Drücken!
  ```

- [ ] `voices/units/shieldbearer_attack_03.ogg` · Regie: hart
  ```text
  [firmly] Nicht an mir vorbei!
  ```

- [ ] `voices/units/shieldbearer_attack_04.ogg` · Regie: rhythmisch
  ```text
  [shouting] Halten und zuschlagen!
  ```

##### shieldbearer_retreat (Event `voice.shieldbearer.retreat`)

- [ ] `voices/units/shieldbearer_retreat_01.ogg` · Regie: laut
  ```text
  [shouting] Schilde zurück, zurück!
  ```

- [ ] `voices/units/shieldbearer_retreat_02.ogg` · Regie: gepresst
  ```text
  [urgently] Allein halte ich nicht!
  ```

- [ ] `voices/units/shieldbearer_retreat_03.ogg` · Regie: dringlich
  ```text
  [urgently] Zurück hinter die Mauer!
  ```

##### shieldbearer_hit (Event `voice.shieldbearer.hit`)

- [ ] `voices/units/shieldbearer_hit_01.ogg` · Regie: trocken
  ```text
  [dryly] [grunts] Der Schild hat's abgefangen.
  ```

- [ ] `voices/units/shieldbearer_hit_02.ogg` · Regie: brummig
  ```text
  [gruffly] [groans] Ein schwerer.
  ```

- [ ] `voices/units/shieldbearer_hit_03.ogg` · Regie: stur
  ```text
  [grunts] Steh noch.
  ```

##### shieldbearer_death (Event `voice.shieldbearer.death`)

- [ ] `voices/units/shieldbearer_death_01.ogg` · Regie: schwer, langsam
  ```text
  [seriously] [exhales] Die Mauer … fällt …
  ```

- [ ] `voices/units/shieldbearer_death_02.ogg` · Regie: beschützend
  ```text
  [groans] Hinter … mir …
  ```

- [ ] `voices/units/shieldbearer_death_03.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] Ruhe …
  ```

##### shieldbearer_victory (Event `voice.shieldbearer.victory`)

- [ ] `voices/units/shieldbearer_victory_01.ogg` · Regie: ruhig-stolz
  ```text
  [proudly] Die Mauer steht noch.
  ```

- [ ] `voices/units/shieldbearer_victory_02.ogg` · Regie: trocken-humorvoll
  ```text
  [amused] Kein Kratzer. Na gut, ein paar.
  ```

- [ ] `voices/units/shieldbearer_victory_03.ogg` · Regie: erleichtert
  ```text
  [relieved] Schilde runter.
  ```

##### shieldbearer_ability (Event `voice.shieldbearer.ability`)

- [ ] `voices/units/shieldbearer_ability_01.ogg` · Regie: laut, befehlend
  ```text
  [commanding] [shouting] Schildwall! Aufschließen!
  ```

- [ ] `voices/units/shieldbearer_ability_02.ogg` · Regie: kurz, scharf
  ```text
  [sharply] Schilde verhaken!
  ```

- [ ] `voices/units/shieldbearer_ability_03.ogg` · Regie: kräftig
  ```text
  [firmly] Bildet die Mauer!
  ```

#### Spähreiter – Stimme `AC_unit_scout_rider` (24 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Anfang 20, heller, flinker Tenor, übermütig, lacht gern, spricht schnell.
> - **Prompt:** "A young border scout rider in his early twenties with a bright, quick tenor voice. Speaks German with a natural, native accent and a lively regional coloring. Fast, breathless, cheeky and cheerful, laughing easily, full of restless energy as if speaking from horseback. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[cheerfully] Der Kamm ist frei, die Straße ruhig, und mein Pferd langweilt sich. [confidently] Sagt es, und ich bin
>   da, bevor Ihr zu Ende gefragt habt.“
> - **Einstellungen:** v3 *Creative*, Speed 1,08.

- Stimme: `AC_unit_scout_rider (Profil G4)` · Modell: Eleven v3, *Creative* · Speed 1,08
- Haltung: junger, flinker Grenzreiter, wach und etwas übermütig; spricht schnell, lacht gern.

##### scout_rider_select (Event `voice.scout_rider.select`)

- [ ] `voices/units/scout_rider_select_01.ogg` · Regie: schnell, munter
  ```text
  [cheerfully] [quickly] Bereit zum Ritt!
  ```

- [ ] `voices/units/scout_rider_select_02.ogg` · Regie: neugierig
  ```text
  [greedily] Was soll ich auskundschaften?
  ```

- [ ] `voices/units/scout_rider_select_03.ogg` · Regie: zufrieden
  ```text
  [cheerfully] Gutes Pferd, gute Augen.
  ```

- [ ] `voices/units/scout_rider_select_04.ogg` · Regie: eifrig
  ```text
  [eagerly] Zeigt mir den Weg.
  ```

##### scout_rider_move (Event `voice.scout_rider.move`)

- [ ] `voices/units/scout_rider_move_01.ogg` · Regie: freudig
  ```text
  [cheerfully] Los wie der Wind!
  ```

- [ ] `voices/units/scout_rider_move_02.ogg` · Regie: schnell
  ```text
  [quickly] Dem Staub voraus!
  ```

- [ ] `voices/units/scout_rider_move_03.ogg` · Regie: leichthin
  ```text
  [calmly] Ich bin vor dem Abendessen zurück.
  ```

- [ ] `voices/units/scout_rider_move_04.ogg` · Regie: Zuruf, kurz
  ```text
  [commanding] Hü!
  ```

##### scout_rider_attack (Event `voice.scout_rider.attack`)

- [ ] `voices/units/scout_rider_attack_01.ogg` · Regie: laut
  ```text
  [shouting] Hü! Reitet sie nieder!
  ```

- [ ] `voices/units/scout_rider_attack_02.ogg` · Regie: scharf
  ```text
  [sharply] Schnell und sauber!
  ```

- [ ] `voices/units/scout_rider_attack_03.ogg` · Regie: Schlachtruf
  ```text
  [shouting] Für Brennhollow!
  ```

- [ ] `voices/units/scout_rider_attack_04.ogg` · Regie: herausfordernd
  ```text
  [shouting] Na kommt schon!
  ```

##### scout_rider_retreat (Event `voice.scout_rider.retreat`)

- [ ] `voices/units/scout_rider_retreat_01.ogg` · Regie: panisch
  ```text
  [panicked] Zu viele! Reiten!
  ```

- [ ] `voices/units/scout_rider_retreat_02.ogg` · Regie: gehetzt
  ```text
  [urgently] Rückzug, Rückzug!
  ```

- [ ] `voices/units/scout_rider_retreat_03.ogg` · Regie: zum Pferd
  ```text
  [urgently] Schneller, Mädchen, schneller!
  ```

##### scout_rider_hit (Event `voice.scout_rider.hit`)

- [ ] `voices/units/scout_rider_hit_01.ogg` · Regie: scharf
  ```text
  [sharply] [gasps] Ah, meine Seite!
  ```

- [ ] `voices/units/scout_rider_hit_02.ogg` · Regie: überspielt
  ```text
  [grunts] Gestreift.
  ```

- [ ] `voices/units/scout_rider_hit_03.ogg` · Regie: besorgt
  ```text
  [worried] [hisses] Halt durch, Mädchen!
  ```

##### scout_rider_death (Event `voice.scout_rider.death`)

- [ ] `voices/units/scout_rider_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [gasps] Sagt ihnen … was ich sah …
  ```

- [ ] `voices/units/scout_rider_death_02.ogg` · Regie: leise
  ```text
  [quietly] [exhales] Reitet weiter …
  ```

- [ ] `voices/units/scout_rider_death_03.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] Der Wind … ist kalt …
  ```

##### scout_rider_victory (Event `voice.scout_rider.victory`)

- [ ] `voices/units/scout_rider_victory_01.ogg` · Regie: lachend
  ```text
  [laughs] Ich sagte doch, ich bin schnell!
  ```

- [ ] `voices/units/scout_rider_victory_02.ogg` · Regie: zufrieden
  ```text
  [cheerfully] Der Weg ist frei.
  ```

- [ ] `voices/units/scout_rider_victory_03.ogg` · Regie: munter
  ```text
  [cheerfully] Zurück mit Neuigkeiten, wie versprochen.
  ```

#### Berittene Bogenschützin – Stimme `AC_unit_horse_archer` (24 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Frau, Ende 20, geschmeidiger heller Alt, selbstsichere, leicht spöttische Steppen-Söldnerin.
> - **Prompt:** "A confident mounted archer, a woman in her late twenties with a smooth, light alto voice. Speaks German with a natural accent and a faintly foreign, lilting coloring. Playful, teasing and self-assured, speaking quickly and lightly as if loosing arrows at a gallop. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[confidently] Ihr geht, ich reite, und trotzdem schieße ich zuerst. [teasingly] Versucht mitzuhalten.“
> - **Einstellungen:** v3 *Creative*, Speed 1,05.

- Stimme: `AC_unit_horse_archer (Profil G5)` · Modell: Eleven v3, *Creative* · Speed 1,05
- Haltung: geschmeidige Steppen-Söldnerin, selbstsicher, ein wenig spöttisch; schießt, bevor andere zielen.

##### horse_archer_select (Event `voice.horse_archer.select`)

- [ ] `voices/units/horse_archer_select_01.ogg` · Regie: selbstsicher
  ```text
  [calmly] Bogen gespannt, Pferd gefüttert.
  ```

- [ ] `voices/units/horse_archer_select_02.ogg` · Regie: spöttisch
  ```text
  [mockingly] Macht es zum bewegten Ziel.
  ```

- [ ] `voices/units/horse_archer_select_03.ogg` · Regie: ruhig
  ```text
  [calmly] Ihr habt meinen Bogen.
  ```

- [ ] `voices/units/horse_archer_select_04.ogg` · Regie: locker
  ```text
  [calmly] Wohin, Wächterin?
  ```

##### horse_archer_move (Event `voice.horse_archer.move`)

- [ ] `voices/units/horse_archer_move_01.ogg` · Regie: kurz
  ```text
  [calmly] Reite weit aus.
  ```

- [ ] `voices/units/horse_archer_move_02.ogg` · Regie: neckend
  ```text
  [calmly] Fangt mich, wenn ihr könnt.
  ```

- [ ] `voices/units/horse_archer_move_03.ogg` · Regie: knapp
  ```text
  [dryly] Flanke, dann Feuer.
  ```

- [ ] `voices/units/horse_archer_move_04.ogg` · Regie: Zuruf
  ```text
  [commanding] Hü!
  ```

##### horse_archer_attack (Event `voice.horse_archer.attack`)

- [ ] `voices/units/horse_archer_attack_01.ogg` · Regie: laut
  ```text
  [shouting] Pfeile los!
  ```

- [ ] `voices/units/horse_archer_attack_02.ogg` · Regie: rhythmisch
  ```text
  [shouting] Schießen und reiten!
  ```

- [ ] `voices/units/horse_archer_attack_03.ogg` · Regie: spöttisch
  ```text
  [mockingly] Zu langsam!
  ```

- [ ] `voices/units/horse_archer_attack_04.ogg` · Regie: Schlachtruf
  ```text
  [shouting] Lasst es auf sie regnen!
  ```

##### horse_archer_retreat (Event `voice.horse_archer.retreat`)

- [ ] `voices/units/horse_archer_retreat_01.ogg` · Regie: gehetzt
  ```text
  [urgently] Absetzen! Raus hier!
  ```

- [ ] `voices/units/horse_archer_retreat_02.ogg` · Regie: angespannt
  ```text
  [worried] Sie sind zu nah!
  ```

- [ ] `voices/units/horse_archer_retreat_03.ogg` · Regie: dringlich
  ```text
  [urgently] Rückzug, schießt beim Reiten!
  ```

##### horse_archer_hit (Event `voice.horse_archer.hit`)

- [ ] `voices/units/horse_archer_hit_01.ogg` · Regie: scharf
  ```text
  [sharply] [gasps] Ein Pfeil im Mantel!
  ```

- [ ] `voices/units/horse_archer_hit_02.ogg` · Regie: überspielt
  ```text
  [grunts] Nicht schlecht.
  ```

- [ ] `voices/units/horse_archer_hit_03.ogg` · Regie: zum Pferd
  ```text
  [hisses] Ruhig, Mädchen!
  ```

##### horse_archer_death (Event `voice.horse_archer.death`)

- [ ] `voices/units/horse_archer_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Letzter … Pfeil …
  ```

- [ ] `voices/units/horse_archer_death_02.ogg` · Regie: poetisch-leise
  ```text
  [quietly] [gasps] Der Wind hat mich geholt …
  ```

- [ ] `voices/units/horse_archer_death_03.ogg` · Regie: ruhig
  ```text
  [calmly] [sighs] Reitet ohne mich weiter …
  ```

##### horse_archer_victory (Event `voice.horse_archer.victory`)

- [ ] `voices/units/horse_archer_victory_01.ogg` · Regie: zufrieden
  ```text
  [cheerfully] Sie kamen nie nah heran.
  ```

- [ ] `voices/units/horse_archer_victory_02.ogg` · Regie: gelöst
  ```text
  [relieved] Köcher leicht, Herz leichter.
  ```

- [ ] `voices/units/horse_archer_victory_03.ogg` · Regie: ruhig
  ```text
  [calmly] Wieder eine Jagd vorbei.
  ```

#### Kampfmagier – Stimme `AC_unit_battle_mage` (27 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Anfang 30, höflicher, gespannter Tenor-Bariton, diszipliniert, unterdrückte Hitze in der Stimme (Flammenakademie von Kaldris).
> - **Prompt:** "A disciplined fire mage trained at an academy, a man in his early thirties with a polite, precise and slightly tense baritone-tenor. Speaks German with a natural, native accent, educated standard German. Controlled and formal, but with suppressed heat under every sentence, as if each word could ignite. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[calmly] Geht bitte beiseite. Ich beherrsche die Flamme. [dryly] Ich möchte nur nicht gefragt werden, wie sehr.“
> - **Einstellungen:** v3 *Creative*, Speed 1,0.

- Stimme: `AC_unit_battle_mage (Profil G6)` · Modell: Eleven v3, *Creative* · Speed 1,0
- Haltung: disziplinierter Feuermagier der Flammenakademie von Kaldris; höflich, gespannt, mit unterdrückter Hitze in der Stimme.

##### battle_mage_select (Event `voice.battle_mage.select`)

- [ ] `voices/units/battle_mage_select_01.ogg` · Regie: ruhig, tief
  ```text
  [calmly] Die Flamme antwortet.
  ```

- [ ] `voices/units/battle_mage_select_02.ogg` · Regie: knapp
  ```text
  [dryly] Glut und Wille.
  ```

- [ ] `voices/units/battle_mage_select_03.ogg` · Regie: konzentriert
  ```text
  [calmly] Nennt das Ziel.
  ```

- [ ] `voices/units/battle_mage_select_04.ogg` · Regie: leise-intensiv
  ```text
  [quietly] Ich bin bereit zu brennen.
  ```

##### battle_mage_move (Event `voice.battle_mage.move`)

- [ ] `voices/units/battle_mage_move_01.ogg` · Regie: gelassen
  ```text
  [calmly] Ich gehe im Licht.
  ```

- [ ] `voices/units/battle_mage_move_02.ogg` · Regie: trocken-warnend
  ```text
  [dryly] Haltet Abstand zu meinem Saum.
  ```

- [ ] `voices/units/battle_mage_move_03.ogg` · Regie: ruhig
  ```text
  [calmly] Wie die Flamme will.
  ```

- [ ] `voices/units/battle_mage_move_04.ogg` · Regie: knapp
  ```text
  [dryly] Bewegung.
  ```

##### battle_mage_attack (Event `voice.battle_mage.attack`)

- [ ] `voices/units/battle_mage_attack_01.ogg` · Regie: scharf, laut
  ```text
  [sharply] [shouting] Brennt!
  ```

- [ ] `voices/units/battle_mage_attack_02.ogg` · Regie: grimmig
  ```text
  [shouting] Asche zu Asche!
  ```

- [ ] `voices/units/battle_mage_attack_03.ogg` · Regie: hymnisch
  ```text
  [solemn] Bei der Ewigen Flamme!
  ```

- [ ] `voices/units/battle_mage_attack_04.ogg` · Regie: knapp, Zauberruf
  ```text
  [dryly] Entfacht!
  ```

##### battle_mage_retreat (Event `voice.battle_mage.retreat`)

- [ ] `voices/units/battle_mage_retreat_01.ogg` · Regie: dringlich
  ```text
  [urgently] Deckt mich, ich bin erschöpft!
  ```

- [ ] `voices/units/battle_mage_retreat_02.ogg` · Regie: panisch
  ```text
  [panicked] Zu nah, zu nah!
  ```

- [ ] `voices/units/battle_mage_retreat_03.ogg` · Regie: gehetzt
  ```text
  [urgently] Zurück zur Linie!
  ```

##### battle_mage_hit (Event `voice.battle_mage.hit`)

- [ ] `voices/units/battle_mage_hit_01.ogg` · Regie: scharf
  ```text
  [sharply] [gasps] Meine Konzentration!
  ```

- [ ] `voices/units/battle_mage_hit_02.ogg` · Regie: bissig
  ```text
  [hisses] Das brennt … in die falsche Richtung.
  ```

- [ ] `voices/units/battle_mage_hit_03.ogg` · Regie: zu sich selbst
  ```text
  [grunts] Ruhig bleiben.
  ```

##### battle_mage_death (Event `voice.battle_mage.death`)

- [ ] `voices/units/battle_mage_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Die Flamme … erlischt …
  ```

- [ ] `voices/units/battle_mage_death_02.ogg` · Regie: leise
  ```text
  [quietly] [gasps] Lasst es … nicht umsonst sein …
  ```

- [ ] `voices/units/battle_mage_death_03.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] Asche … endlich.
  ```

##### battle_mage_victory (Event `voice.battle_mage.victory`)

- [ ] `voices/units/battle_mage_victory_01.ogg` · Regie: zufrieden-ruhig
  ```text
  [calmly] Die Glut ist still.
  ```

- [ ] `voices/units/battle_mage_victory_02.ogg` · Regie: leise
  ```text
  [quietly] Lasst das Feuer ruhen.
  ```

- [ ] `voices/units/battle_mage_victory_03.ogg` · Regie: feierlich
  ```text
  [solemn] Das Licht hat gesiegt.
  ```

##### battle_mage_ability (Event `voice.battle_mage.ability`)

- [ ] `voices/units/battle_mage_ability_01.ogg` · Regie: laut, Zauberruf
  ```text
  [shouting] Feuer, antworte mir!
  ```

- [ ] `voices/units/battle_mage_ability_02.ogg` · Regie: scharf
  ```text
  [sharply] Funken, erhebt euch!
  ```

- [ ] `voices/units/battle_mage_ability_03.ogg` · Regie: hart
  ```text
  [firmly] Flamme, nimm sie!
  ```

#### Priester der Flamme – Stimme `AC_unit_priest` (27 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Mann, Ende 50, sanfter, warmer Bariton, Dorfkapellenpriester, zitiert Liturgie, ängstlich, hilft trotzdem.
> - **Prompt:** "A gentle village chapel priest in his late fifties with a soft, warm baritone. Speaks German with a natural, native accent and a kind, rural coloring. Calm and reassuring, quoting liturgy in a half-sung murmur, with a tremor of fear underneath that he overcomes out of duty. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[gently] Die Flamme verlangt nicht, dass wir keine Angst haben. Nur, dass wir sie am Brennen halten. [softly] Kommt,
>   lasst mich die Wunde sehen.“
> - **Einstellungen:** v3 *Natural*, Speed 0,96.

- Stimme: `AC_unit_priest (Profil G7)` · Modell: Eleven v3, *Natural* · Speed 0,96
- Haltung: sanfter, warmer Flammenpriester der Dorfkapelle; zitiert Liturgie, hat im Kampf Angst, aber hilft trotzdem.

##### priest_select (Event `voice.priest.select`)

- [ ] `voices/units/priest_select_01.ogg` · Regie: warm, sanft
  ```text
  [gently] Die Flamme sei mit Euch.
  ```

- [ ] `voices/units/priest_select_02.ogg` · Regie: freundlich
  ```text
  [warmly] Wie kann ich dienen?
  ```

- [ ] `voices/units/priest_select_03.ogg` · Regie: tröstend
  ```text
  [calmly] Ich bin hier, Kind.
  ```

- [ ] `voices/units/priest_select_04.ogg` · Regie: ruhig
  ```text
  [calmly] Lasst das Licht uns leiten.
  ```

##### priest_move (Event `voice.priest.move`)

- [ ] `voices/units/priest_move_01.ogg` · Regie: gelassen
  ```text
  [calmly] Wir gehen im Licht.
  ```

- [ ] `voices/units/priest_move_02.ogg` · Regie: ruhig
  ```text
  [calmly] Geht voran, ich folge.
  ```

- [ ] `voices/units/priest_move_03.ogg` · Regie: sorgend
  ```text
  [calmly] Gebt auf die Verwundeten acht.
  ```

- [ ] `voices/units/priest_move_04.ogg` · Regie: leicht gehetzt
  ```text
  [urgently] Ja, ja, ich gehe.
  ```

##### priest_attack (Event `voice.priest.attack`)

- [ ] `voices/units/priest_attack_01.ogg` · Regie: mutig, zitternd
  ```text
  [shouting] Zurück, Geschöpfe der Dunkelheit!
  ```

- [ ] `voices/units/priest_attack_02.ogg` · Regie: laut
  ```text
  [shouting] Flamme, beschütze uns!
  ```

- [ ] `voices/units/priest_attack_03.ogg` · Regie: hymnisch
  ```text
  [solemn] Im Namen der Ewigen Flamme!
  ```

- [ ] `voices/units/priest_attack_04.ogg` · Regie: entschlossen
  ```text
  [firmly] Weg von ihnen!
  ```

##### priest_retreat (Event `voice.priest.retreat`)

- [ ] `voices/units/priest_retreat_01.ogg` · Regie: panisch
  ```text
  [panicked] Ich kann nicht! Ich kann nicht!
  ```

- [ ] `voices/units/priest_retreat_02.ogg` · Regie: dringlich
  ```text
  [urgently] Beschützt die Heiler!
  ```

- [ ] `voices/units/priest_retreat_03.ogg` · Regie: verzweifelt
  ```text
  [sad] Flamme, vergib mir!
  ```

##### priest_hit (Event `voice.priest.hit`)

- [ ] `voices/units/priest_hit_01.ogg` · Regie: erschrocken
  ```text
  [fearful] [gasps] Oh! Gnade!
  ```

- [ ] `voices/units/priest_hit_02.ogg` · Regie: tapfer
  ```text
  [groans] Es ist nur Schmerz.
  ```

- [ ] `voices/units/priest_hit_03.ogg` · Regie: gepresst
  ```text
  [hisses] Vergib ihnen …
  ```

##### priest_death (Event `voice.priest.death`)

- [ ] `voices/units/priest_death_01.ogg` · Regie: friedlich
  ```text
  [gently] [exhales] In die Flamme … gehe ich …
  ```

- [ ] `voices/units/priest_death_02.ogg` · Regie: besorgt
  ```text
  [worried] [gasps] Wer … wird … sie pflegen …
  ```

- [ ] `voices/units/priest_death_03.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] Licht … nimm mich.
  ```

##### priest_victory (Event `voice.priest.victory`)

- [ ] `voices/units/priest_victory_01.ogg` · Regie: erleichtert
  ```text
  [relieved] Lob der Flamme. Versorgt die Verwundeten.
  ```

- [ ] `voices/units/priest_victory_02.ogg` · Regie: gerührt
  ```text
  [moved] Wir leben. Dank dem Licht.
  ```

- [ ] `voices/units/priest_victory_03.ogg` · Regie: feierlich
  ```text
  [solemn] Friede den Gefallenen.
  ```

##### priest_ability (Event `voice.priest.ability`)

- [ ] `voices/units/priest_ability_01.ogg` · Regie: warm, kräftig
  ```text
  [firmly] Werdet geheilt!
  ```

- [ ] `voices/units/priest_ability_02.ogg` · Regie: hymnisch
  ```text
  [solemn] Flamme, richte sie auf!
  ```

- [ ] `voices/units/priest_ability_03.ogg` · Regie: strahlend
  ```text
  [commanding] Erhebt euch und seid gesegnet!
  ```

#### Rammbock-Mannschaft – Stimme `AC_unit_ram_crew` (15 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Gruppe von 3–4 Männern, schwitzende Pioniere, Hau-Ruck-Rufe im Takt, dumpfe Anstrengung; mehrere Takes überlagern.
> - **Prompt:** "A crew of four sweating siege engineers shouting in rhythm, rough male voices, heaving and grunting as they swing a heavy ram, shouted work chants and effort sounds, tight and dry. They shout in German with a natural, native accent. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[shouting] Zieht! Und zieht! Und zieht! Nochmal, [excited] legt euch ins Zeug!“
> - **Einstellungen:** v3 *Creative*, Speed 1,0; je Zeile 3–4 Takes mit leicht verschobenem Einsatz überlagern (Chor), Mono-Summe.

- Stimme: `AC_unit_ram_crew (Profil G8)` · Modell: Eleven v3, *Creative* (Mehrstimmen-Chor, 3–4 Männer) · Speed 1,0
- Haltung: Gruppe schwitzender Pioniere; rufen im Takt, Stoßrufe, Hau-Ruck. Mehrere Sprecher überlagert, dumpfe Anstrengung.

##### battering_ram_select (Event `voice.battering_ram.select`)

- [ ] `voices/units/battering_ram_select_01.ogg` · Regie: Chor, Zuruf
  ```text
  [commanding] Ramme bereit!
  ```

- [ ] `voices/units/battering_ram_select_02.ogg` · Regie: grimmig, rau
  ```text
  [gruffly] Jawohl, das Tor hält nicht.
  ```

- [ ] `voices/units/battering_ram_select_03.ogg` · Regie: Anfeuerung
  ```text
  [calmly] Legt Euch ins Zeug!
  ```

- [ ] `voices/units/battering_ram_select_04.ogg` · Regie: rhythmisch
  ```text
  [calmly] Hau ruck, Jungs!
  ```

##### battering_ram_move (Event `voice.battering_ram.move`)

- [ ] `voices/units/battering_ram_move_01.ogg` · Regie: Anstrengung, im Takt
  ```text
  [calmly] Schieben, schieben!
  ```

- [ ] `voices/units/battering_ram_move_02.ogg` · Regie: hektisch
  ```text
  [urgently] Achtet auf die Räder!
  ```

- [ ] `voices/units/battering_ram_move_03.ogg` · Regie: gepresst
  ```text
  [calmly] Langsam … langsam …
  ```

- [ ] `voices/units/battering_ram_move_04.ogg` · Regie: Zuruf
  ```text
  [commanding] Rollt sie vor!
  ```

##### battering_ram_attack (Event `voice.battering_ram.attack`)

- [ ] `voices/units/battering_ram_attack_01.ogg` · Regie: laut, im Takt
  ```text
  [shouting] Schwingen! Schwingen!
  ```

- [ ] `voices/units/battering_ram_attack_02.ogg` · Regie: wild
  ```text
  [shouting] Reißt es nieder!
  ```

- [ ] `voices/units/battering_ram_attack_03.ogg` · Regie: Chor
  ```text
  [shouting] Alle zusammen – jetzt!
  ```

- [ ] `voices/units/battering_ram_attack_04.ogg` · Regie: Schlachtruf
  ```text
  [shouting] Zerschmettert das Tor!
  ```

##### battering_ram_death (Event `voice.battering_ram.death`)

- [ ] `voices/units/battering_ram_death_01.ogg` · Regie: panisch, Chor
  ```text
  [panicked] [shouting] Sie bricht auseinander!
  ```

- [ ] `voices/units/battering_ram_death_02.ogg` · Regie: gebrochen
  ```text
  [sad] [groans] Der Rammbock … ist verloren …
  ```

- [ ] `voices/units/battering_ram_death_03.ogg` · Regie: Chor, hektisch
  ```text
  [urgently] [shouting] Lauft, Jungs, lauft!
  ```

#### Katapult-Mannschaft – Stimme `AC_unit_catapult_crew` (15 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - 2–3 erfahrene Geschützmeister, nüchterne Zurufe in Kommandoform, Winden und Seile hörbar.
> - **Prompt:** "Two or three experienced siege gunners, rough and businesslike male voices calling short, clear commands over creaking ropes and a winch, calm professionals, never panicked. They call out in German with a natural, native accent. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[commanding] Spannt sie zurück! Laden! Ruhig … [short pause] und los!“
> - **Einstellungen:** v3 *Creative*, Speed 1,0; 2–3 Sprecher getrennt generieren und abwechselnd verwenden.

- Stimme: `AC_unit_catapult_crew (Profil G9)` · Modell: Eleven v3, *Creative* (2–3 Sprecher) · Speed 1,0
- Haltung: erfahrene Geschützmeister, nüchterne Zurufe in Kommandoform, Winden und Seile hörbar.

##### catapult_select (Event `voice.catapult.select`)

- [ ] `voices/units/catapult_select_01.ogg` · Regie: nüchtern
  ```text
  [seriously] Geschütz ist geladen.
  ```

- [ ] `voices/units/catapult_select_02.ogg` · Regie: knapp
  ```text
  [dryly] Gebt uns ein Ziel.
  ```

- [ ] `voices/units/catapult_select_03.ogg` · Regie: Zuruf
  ```text
  [commanding] Spannt sie zurück!
  ```

- [ ] `voices/units/catapult_select_04.ogg` · Regie: fachlich
  ```text
  [calmly] Reichweite und Richtung, Herr?
  ```

##### catapult_move (Event `voice.catapult.move`)

- [ ] `voices/units/catapult_move_01.ogg` · Regie: Anstrengung
  ```text
  [calmly] Zieht sie, zieht sie!
  ```

- [ ] `voices/units/catapult_move_02.ogg` · Regie: besorgt
  ```text
  [worried] Vorsicht mit dem Arm!
  ```

- [ ] `voices/units/catapult_move_03.ogg` · Regie: hektisch
  ```text
  [urgently] Achtet auf die Spurrillen!
  ```

- [ ] `voices/units/catapult_move_04.ogg` · Regie: gepresst
  ```text
  [calmly] Langsam, langsam!
  ```

##### catapult_attack (Event `voice.catapult.attack`)

- [ ] `voices/units/catapult_attack_01.ogg` · Regie: laut, Kommando
  ```text
  [commanding] [shouting] Los!
  ```

- [ ] `voices/units/catapult_attack_02.ogg` · Regie: scharf
  ```text
  [sharply] Feuer!
  ```

- [ ] `voices/units/catapult_attack_03.ogg` · Regie: Warnruf
  ```text
  [shouting] Zurücktreten – los!
  ```

- [ ] `voices/units/catapult_attack_04.ogg` · Regie: Triumph
  ```text
  [triumphantly] Fort damit!
  ```

##### catapult_death (Event `voice.catapult.death`)

- [ ] `voices/units/catapult_death_01.ogg` · Regie: entsetzt
  ```text
  [fearful] [shouting] Sie ist hin – [pause] der Arm ist gebrochen!
  ```

- [ ] `voices/units/catapult_death_02.ogg` · Regie: gebrochen
  ```text
  [sad] [groans] Jahre an Arbeit …
  ```

- [ ] `voices/units/catapult_death_03.ogg` · Regie: panisch
  ```text
  [panicked] [shouting] Weg hier!
  ```

### B6 Die Hohlen

#### Hohlknecht – Stimme `AC_unit_hollow_thrall` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - **Prompt:** "A hollow, emaciated male voice of a reanimated villager, dry and breathy, with a weak rattling rasp in the
>   throat, murmuring fragments of words as if half-remembering a past life. Slow, confused, mournful, unsettling. Murmurs
>   German word fragments with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[whispers] [sorrowfully] Heim … die Felder sind kalt … ich finde die Tür nicht … [rasping] Mutter, das Feuer ist aus
>   … so hungrig … so kalt …“
> - **Einstellungen:** v3 *Creative*; Pitch −2 und Hohle-Kette setzt die Integration.

- Stimme: `AC_unit_hollow_thrall` (Profil D5.1) · Modell: Eleven v3, *Creative* · Speed 0,9

##### hollow_thrall_attack (Event `voice.hollow_thrall.attack`)

- [ ] `voices/units/hollow_thrall_attack_01.ogg` · Regie: gierig, kehlig
  ```text
  [greedily] [groans] Hnnngh … warm …
  ```

- [ ] `voices/units/hollow_thrall_attack_02.ogg` · Regie: klagend-aggressiv
  ```text
  [rasping] Gib … es … zurück …
  ```

- [ ] `voices/units/hollow_thrall_attack_03.ogg` · Regie: wortloser Angriffslaut, Luftstoß
  ```text
  [hisses] Hhhaaah!
  ```

- [ ] `voices/units/hollow_thrall_attack_04.ogg` · Regie: gemurmelt, besitzergreifend
  ```text
  [shouting] … Heim … meins …
  ```

##### hollow_thrall_hit (Event `voice.hollow_thrall.hit`)

- [ ] `voices/units/hollow_thrall_hit_01.ogg` · Regie: dumpf, gleichgültig
  ```text
  [coldly] [groans] Uhhh –
  ```

- [ ] `voices/units/hollow_thrall_hit_02.ogg` · Regie: trockenes Keuchen, abgebrochen
  ```text
  [dryly] [gasps] Hhkk –
  ```

- [ ] `voices/units/hollow_thrall_hit_03.ogg` · Regie: verloren, fast menschlich
  ```text
  [grunts] … tut … noch weh …
  ```

##### hollow_thrall_death (Event `voice.hollow_thrall.death`)

- [ ] `voices/units/hollow_thrall_death_01.ogg` · Regie: erlöst, verhauchend
  ```text
  [relieved] [sighs] … endlich … still …
  ```

- [ ] `voices/units/hollow_thrall_death_02.ogg` · Regie: leises, trauriges Erinnern
  ```text
  [sad] [exhales] … Mutter …
  ```

- [ ] `voices/units/hollow_thrall_death_03.ogg` · Regie: langes Todesröcheln, 1,5–2 s
  ```text
  [rattling breath] Hhhhaaahhh …
  ```

#### Knochenschütze – Stimme `AC_unit_hollow_archer` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - **Prompt:** "A dry, whispering, skeletal voice with clicking, rattling consonants and almost no breath, thin and airy,
>   hissing fragments of words, cold and precise. Hisses German word fragments with a natural, native pronunciation. Close-mic
>   studio-quality recording, no background noise."
> - **Beispieltext:** „[hissing] Ssseh dich … ssseh die Warmen … [whispers] noch einer … markier das Herz … es hört nie auf zu zählen … noch
>   einer …“
> - **Einstellungen:** v3 *Creative* mit `[whispers]`; Integration: Hohle-Kette + hochfrequentes Knochenklacken (SFX-Layer, −18 dB).

- Stimme: `AC_unit_hollow_archer` (Profil D5.2) · Modell: Eleven v3, *Creative* mit `[whispers]`

##### hollow_archer_attack (Event `voice.hollow_archer.attack`)

- [ ] `voices/units/hollow_archer_attack_01.ogg` · Regie: kalt, zischend
  ```text
  [coldly] [whispers] Ich ssseh dich.
  ```

- [ ] `voices/units/hollow_archer_attack_02.ogg` · Regie: zählend, gierig
  ```text
  [greedily] [whispers] Noch einer …
  ```

- [ ] `voices/units/hollow_archer_attack_03.ogg` · Regie: wortlos, Zischlaut beim Schuss
  ```text
  [hisses] Tsssk!
  ```

- [ ] `voices/units/hollow_archer_attack_04.ogg` · Regie: präzise, eisig
  ```text
  [coldly] [whispers] Markier das Herz.
  ```

##### hollow_archer_hit (Event `voice.hollow_archer.hit`)

- [ ] `voices/units/hollow_archer_hit_01.ogg` · Regie: Knack-Zischen
  ```text
  [hisses] Kkhh!
  ```

- [ ] `voices/units/hollow_archer_hit_02.ogg` · Regie: scharfes, kurzes Zischen
  ```text
  [hissing] Ssss!
  ```

- [ ] `voices/units/hollow_archer_hit_03.ogg` · Regie: tonlos, fast verwundert
  ```text
  [coldly] [whispers] … gesprungen …
  ```

##### hollow_archer_death (Event `voice.hollow_archer.death`)

- [ ] `voices/units/hollow_archer_death_01.ogg` · Regie: verwehend
  ```text
  [whispers] … zerfalle …
  ```

- [ ] `voices/units/hollow_archer_death_02.ogg` · Regie: letzter, trockener Hauch
  ```text
  [dryly] [exhales] Hhhaaa …
  ```

- [ ] `voices/units/hollow_archer_death_03.ogg` · Regie: erleichtert, tonlos
  ```text
  [relieved] [whispers] … das Zählen … hört auf …
  ```

#### Grabkoloss – Stimme `AC_unit_grave_brute` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - **Prompt:** "A huge, deep, guttural creature voice of a bloated undead giant, slow heavy breathing, wet low rumbling
>   groans, able to bellow single words with crushing force. Bellows single German words with a natural, native pronunciation.
>   Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[growls] Hhhrrrgh … zerquetschen … zerquetschen … runter … bleib unten … [groans] hhhuuurrgh … ihr alle … in die Erde
>   …“
> - **Einstellungen:** v3 *Creative*; SFX-Layer „massive undead giant groan“ und Pitch −4 bis −6 setzt die Integration (nicht zu liefern).

- Stimme: `AC_unit_grave_brute` (Profil D5.3) · Modell: Eleven v3, *Creative* · Speed 0,85

##### grave_brute_attack (Event `voice.grave_brute.attack`)

- [ ] `voices/units/grave_brute_attack_01.ogg` · Regie: brüllend, ein Wort
  ```text
  [shouting] Zerquetschen!
  ```

- [ ] `voices/units/grave_brute_attack_02.ogg` · Regie: wortloser Brüller, 1,2 s
  ```text
  [groans] Hhhrrraaagh!
  ```

- [ ] `voices/units/grave_brute_attack_03.ogg` · Regie: kurz, wuchtig
  ```text
  [shouting] Runter!
  ```

- [ ] `voices/units/grave_brute_attack_04.ogg` · Regie: langsam, genüsslich
  ```text
  [greedily] [growls] Zerschmettern … die Kleinen …
  ```

##### grave_brute_hit (Event `voice.grave_brute.hit`)

- [ ] `voices/units/grave_brute_hit_01.ogg` · Regie: dumpfes Grunzen
  ```text
  [groans] Hhhrrmph!
  ```

- [ ] `voices/units/grave_brute_hit_02.ogg` · Regie: wütend, kurz
  ```text
  [angry] [roars] Rrraah!
  ```

- [ ] `voices/units/grave_brute_hit_03.ogg` · Regie: träge, überrascht
  ```text
  [surprised] [groans] … tut weh …
  ```

##### grave_brute_death (Event `voice.grave_brute.death`)

- [ ] `voices/units/grave_brute_death_01.ogg` · Regie: langes, absinkendes Stöhnen, 2 s
  ```text
  [groans] Hhhuuurrrgh …
  ```

- [ ] `voices/units/grave_brute_death_02.ogg` · Regie: müde, fast kindlich
  ```text
  [tired] [exhales] … Erde … kalt …
  ```

- [ ] `voices/units/grave_brute_death_03.ogg` · Regie: Brüllen, das in Röcheln zerfällt
  ```text
  [roars] Rrrraaaaagh …
  ```

#### Grabgräber – Stimme `AC_unit_hollow_digger` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Hohler Wiedergänger-Arbeiter, flach, heiser, murmelt Fetzen von Werkzeugen und Schichten; Hohle-Kette, Pitch −2.
> - **Prompt:** "A hollow, reanimated grave-digger with a flat, dusty, hoarse male voice, muttering fragments of tools and soil layers as if still at work, slow and monotonous, breathy and dry, mournful and unsettling. Mutters German word fragments with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[whispers] Tiefer … noch eine Handbreit … [sorrowfully] die Erde ist kalt … immer tiefer.“
> - **Einstellungen:** v3 *Creative*, Speed 0,9; Hohle-Kette (Pitch −2) setzt die Integration.

- Stimme: `AC_unit_hollow_digger (Profil G10)` · Modell: Eleven v3, *Creative* · Speed 0,9
- Haltung: arbeitsamer Wiedergänger, der im Tod weiter schürft; murmelt Fetzen von Werkzeugen und Schichten. Wie `hollow_thrall`: Hohle-Kette und Pitch −2 setzt die Integration.

##### hollow_digger_attack (Event `voice.hollow_digger.attack`)

- [ ] `voices/units/hollow_digger_attack_01.ogg` · Regie: gierig, kehlig
  ```text
  [greedily] [rasping] Mein … mein …
  ```

- [ ] `voices/units/hollow_digger_attack_02.ogg` · Regie: klagend
  ```text
  [groans] Nicht die … Schaufel …
  ```

- [ ] `voices/units/hollow_digger_attack_03.ogg` · Regie: wortlos
  ```text
  [hisses] Hhhaah!
  ```

- [ ] `voices/units/hollow_digger_attack_04.ogg` · Regie: gemurmelt
  ```text
  [shouting] … Schicht … nicht vorbei …
  ```

##### hollow_digger_hit (Event `voice.hollow_digger.hit`)

- [ ] `voices/units/hollow_digger_hit_01.ogg` · Regie: gleichgültig
  ```text
  [coldly] [groans] Uhh – zurück an die Arbeit …
  ```

- [ ] `voices/units/hollow_digger_hit_02.ogg` · Regie: trockenes Keuchen
  ```text
  [dryly] [gasps] Hhkk –
  ```

- [ ] `voices/units/hollow_digger_hit_03.ogg` · Regie: verloren
  ```text
  [grunts] … Staub … in den … Augen …
  ```

##### hollow_digger_death (Event `voice.hollow_digger.death`)

- [ ] `voices/units/hollow_digger_death_01.ogg` · Regie: erlöst
  ```text
  [relieved] [sighs] … Schicht … vorbei …
  ```

- [ ] `voices/units/hollow_digger_death_02.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] … tiefer … tiefer …
  ```

- [ ] `voices/units/hollow_digger_death_03.ogg` · Regie: langes Röcheln
  ```text
  [rattling breath] Hhhaaahh …
  ```

#### Ghul – Stimme `AC_unit_ghoul` (10 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Ausgehungerte Leichenfresser: hohes Kreischen, Schmatzen, Schnappen; fast keine Wörter.
> - **Prompt:** "A starving ghoul, a high, cracking, shrieking creature voice with wet smacking and snapping jaws, hoarse hissing and guttural gurgles, almost no words, feral and desperate. Rasps the few German words it has with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[growls] Hunger … so ein Hunger … [hissing] Fleisch … warmes Fleisch …“
> - **Einstellungen:** v3 *Creative*, Speed 1,1; Zeilen sind überwiegend Laute. Hohle-Kette setzt die Integration.

- Stimme: `AC_unit_ghoul (Profil G11)` · Modell: Eleven v3, *Creative* · Speed 1,1
- Haltung: ausgehungerte Leichenfresser: hohes Kreischen, Schmatzen, Schnappen; fast keine Wörter. Basis: v3-Take mit Lauten; Hohle-Kette setzt die Integration.

##### ghoul_attack (Event `voice.ghoul.attack`)

- [ ] `voices/units/ghoul_attack_01.ogg` · Regie: hohes Kreischen
  ```text
  [screeches] Hhheeeee!
  ```

- [ ] `voices/units/ghoul_attack_02.ogg` · Regie: gierig, schmatzend
  ```text
  [greedily] [snarls] Hunger … Hunger …
  ```

- [ ] `voices/units/ghoul_attack_03.ogg` · Regie: lüstern
  ```text
  [hisses] Frisch … frisch …
  ```

- [ ] `voices/units/ghoul_attack_04.ogg` · Regie: tiefes, nasses Knurren
  ```text
  [wet growl] Grrrhhh!
  ```

##### ghoul_hit (Event `voice.ghoul.hit`)

- [ ] `voices/units/ghoul_hit_01.ogg` · Regie: kurzer Aufschrei
  ```text
  [shouting] [yelps] Hkkh!
  ```

- [ ] `voices/units/ghoul_hit_02.ogg` · Regie: wütend
  ```text
  [angry] [snarls] Hrrrk!
  ```

- [ ] `voices/units/ghoul_hit_03.ogg` · Regie: kläglich
  ```text
  [whines] Hnnn …
  ```

##### ghoul_death (Event `voice.ghoul.death`)

- [ ] `voices/units/ghoul_death_01.ogg` · Regie: abfallendes Kreischen
  ```text
  [shrieks] Heeeaaah …
  ```

- [ ] `voices/units/ghoul_death_02.ogg` · Regie: röchelnd
  ```text
  [gurgles] Hunger … noch …
  ```

- [ ] `voices/units/ghoul_death_03.ogg` · Regie: dünnes Ausatmen
  ```text
  [rattling breath] Hhhh …
  ```

#### Aschhexe – Stimme `AC_unit_ash_witch` (13 Dateien)

**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**

> - Frau, undefinierbares Alter, flüsternde, kalt-berechnende Nekromantin; spricht wie eine Lehrerin zu ihren Toten; Hohle-Kette mit mehr Hall.
> - **Prompt:** "A whispering necromancer, a woman of indeterminate age with a cold, soft, breathy alto voice, speaking calmly and precisely like a teacher to her pupils, with a chill of dry ash in every syllable and an ancient, hollow undertone. Speaks German with a natural, native accent. Close-mic studio-quality recording, no background noise."
> - **Beispieltext:** „[softly] Erhebt euch, meine Kleinen. [menacingly] Die Asche erinnert sich, was die Lebenden vergessen.“
> - **Einstellungen:** v3 *Creative* mit `[whispers]`, Speed 0,92; Hohle-Kette mit mehr Hall-Anteil setzt die Integration.

- Stimme: `AC_unit_ash_witch (Profil G12)` · Modell: Eleven v3, *Creative* mit `[whispers]` · Speed 0,92
- Haltung: flüsternde Nekromantin, kalt-berechnend, spricht wie eine Lehrerin zu ihren Toten; Hohle-Kette mit mehr Hall-Anteil (Integration).

##### ash_witch_attack (Event `voice.ash_witch.attack`)

- [ ] `voices/units/ash_witch_attack_01.ogg` · Regie: kalt, flüsternd
  ```text
  [coldly] [whispers] Brennt für mich.
  ```

- [ ] `voices/units/ash_witch_attack_02.ogg` · Regie: scharf
  ```text
  [sharply] [hisses] Asche und Knochen.
  ```

- [ ] `voices/units/ash_witch_attack_03.ogg` · Regie: spöttisch
  ```text
  [mockingly] [whispers] Du warst einmal warm.
  ```

- [ ] `voices/units/ash_witch_attack_04.ogg` · Regie: leise-lockend
  ```text
  [quietly] [cold laugh] Kommt näher.
  ```

##### ash_witch_hit (Event `voice.ash_witch.hit`)

- [ ] `voices/units/ash_witch_hit_01.ogg` · Regie: gereizt
  ```text
  [hisses] Unverschämtes Fleisch!
  ```

- [ ] `voices/units/ash_witch_hit_02.ogg` · Regie: verblüfft
  ```text
  [gasps] Wie … warm.
  ```

- [ ] `voices/units/ash_witch_hit_03.ogg` · Regie: kurz
  ```text
  [hisses] Hnh!
  ```

##### ash_witch_death (Event `voice.ash_witch.death`)

- [ ] `voices/units/ash_witch_death_01.ogg` · Regie: verhauchend
  ```text
  [quietly] [exhales] Das Tief … ist geduldig …
  ```

- [ ] `voices/units/ash_witch_death_02.ogg` · Regie: leise
  ```text
  [quietly] [whispers] Erhebt euch … ohne mich …
  ```

- [ ] `voices/units/ash_witch_death_03.ogg` · Regie: Röcheln
  ```text
  [rattling breath] Staub zu Staub …
  ```

##### ash_witch_ability (Event `voice.ash_witch.ability`)

- [ ] `voices/units/ash_witch_ability_01.ogg` · Regie: beschwörend, tief
  ```text
  [whispers] Erhebt euch, meine Kinder.
  ```

- [ ] `voices/units/ash_witch_ability_02.ogg` · Regie: befehlend
  ```text
  [commanding] Hoch aus der Asche!
  ```

- [ ] `voices/units/ash_witch_ability_03.ogg` · Regie: Singsang, rituell
  ```text
  [solemn] [chants] Knochen zu Knochen, Fleisch zu Fleisch.
  ```

---

## C. ElevenLabs Ambience (6 Dateien)

**Werkzeug:** ElevenLabs → Sound Effects mit eingeschalteter **Loop-Option**. Je Ambience **eine** Generierung mit dem angegebenen zusammengeführten Prompt (Dauer wie angegeben, derzeit höchstens ca. 30 s); die beste komplette Generierung unverändert liefern – keine Schichtung, kein Zusammensetzen, kein Schneiden. Prompts enthalten „no music, no voices“; Ausnahme `ambience_village` (unverständliches, entferntes Murmeln). Prompt Influence 0,4 (Startwert). Verlängerung auf die Ziel-Länge, Loop-Naht, Lautheit (−23 LUFS) und Stereo-Konvertierung übernimmt die Integration.

### 1.1 `ambience/ambience_day_loop.ogg` – Tag in der Aschmark (Layer `ambience_day`, −6 dB)

- [ ] `ambience/ambience_day_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* karges, verwundetes Grenzland bei Tag; spärliche Natur, die sich zurückkämpft; ruhig, ein wenig melancholisch.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | quiet open countryside daytime ambience in a barren borderland, light breeze over dry grass and sparse fields, faint rustle of dry leaves, a few distant songbirds and a single crow far away, calm and slightly melancholic, no music, no voices, stereo | 30 s | 0,4 | an |

### 1.2 `ambience/ambience_night_loop.ogg` – Nacht in der Aschmark (Layer `ambience_night`, −6 dB)

- [ ] `ambience/ambience_night_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* kalte, bedrohliche Stille; die Toten könnten jederzeit kommen; sparsam, mit unheimlichen fernen Details.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | cold quiet night ambience in open countryside, low soft wind over empty fields, deep stillness, sparse thin crickets far away, a lone owl hooting twice and a faint distant wolf howl, no music, no voices, stereo | 30 s | 0,4 | an |

### 1.3 `ambience/ambience_rain_loop.ogg` – Regen (Layer `ambience_rain`, −6 dB)

- [ ] `ambience/ambience_rain_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* mittlerer, kalter Regen; wird über Tag/Nacht gelegt. Keine Vögel, kein Donner (Donner = `sfx/world/thunder_*`).
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | steady medium cold rain falling on open fields, mud and puddles, even wide stereo rain hiss with soft irregular drips from thatched roofs and a gutter trickling into a puddle, constant intensity, no thunder, no birds, no music, no voices, stereo | 30 s | 0,4 | an |

### 1.4 `ambience/ambience_ash_wind_loop.ogg` – Aschewind (Layer `ambience_ash_wind`, −6 dB)

- [ ] `ambience/ambience_ash_wind_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* Signatur-Ambience der Aschmark; trockener, böiger Wind, der feine Asche über Stein und Ruinen treibt; leicht unheimlich, ein hohles Pfeifen. Über Tag und Nacht einsetzbar.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | dry gusty wind carrying fine ash and grit across barren land, soft hissing sand-like texture, hollow low whistle, an occasional stronger gust howling through ruined stone, desolate and lonely, no music, no voices, stereo | 30 s | 0,4 | an |

### 1.5 `ambience/ambience_village_loop.ogg` – Brennhollow (Layer `ambience_village`, −6 dB)

- [ ] `ambience/ambience_village_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* kleines Dorf (~30 Einwohner), das wieder zu leben beginnt; vorsichtig, arbeitsam, gedämpft – kein lauter Markt. Wird abhängig von Kameranähe zur Siedlung eingeblendet.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | small quiet medieval village ambience, distant indistinct murmuring of a few people, crackling hearth fires, occasional distant hammer on wood, a cart creaking slowly, a few chickens and a dog barking far away, calm and cautious, no music, no clear words, stereo | 30 s | 0,4 | an |

### 1.6 `ambience/ambience_ruins_loop.ogg` – Ruinen von Vael Dorn (Layer `ambience_ruins`, −6 dB)

- [ ] `ambience/ambience_ruins_loop.ogg` · eine komplette Generierung, Loop-Option an; Länge 30 s

*Charakter:* Ruinen im Nordosten mit dem Hohlschrein; Schleier ist dünn: tiefe Drones, hohler Wind in Steinbögen, fernes Knochenklappern, unterschwelliges Flüstern (nicht verständlich), Aschglas-Summen.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | eerie ruined stone temple ambience, low dark drone, hollow wind moving through broken arches, faint glassy humming of magic crystals, distant bones clattering on stone, a faint unintelligible ghostly whisper, cold and empty, no music, no clear words, stereo | 30 s | 0,4 | an |

---

## D. Suno Musik (34 Dateien)

**Werkzeug:** Suno, **Custom-Modus** (Style und Lyrics getrennt). Style-Prompt (englisch, ≤ ca. 200 Zeichen) in das Style-Feld, Lyrics-/Struktur-Feld wie angegeben (Struktur-Tags; bei Instrumentals Weg A: Instrumental-Schalter an, oder Weg B: aus mit reinen Struktur-Tags ohne singbaren Text; Chor-Tracks: Schalter aus, Old-Aurethi-Text). Ausschluss-Feld, falls vorhanden: „pop vocals, rap, edm, electric guitar, autotune, trap drums“. Keine Künstlernamen in Prompts. **Pro Track genau eine Datei `music/<id>`: Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden, kein Normalisieren; beliebiges Format, höchste Qualität (WAV bevorzugt).

### D.1 `main_menu` – „Krone aus Asche“

- [ ] `music/main_menu.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `main_menu` → `assets/audio/music/main_menu.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/main_menu`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Menu · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Hauptmenü und Pause-Menü (−6 dB). Erster Eindruck des Spiels; etabliert das Leitmotiv in der Asche-Form und deutet am Loop-Ende die Glut-Form an. |
| Genre | Dark-Fantasy-Orchesterelegie, kinematisch |
| Instrumentierung | Solo-Cello, tiefe Streicher, Harfe, Altflöte, wortlose Frauenstimme, gemischter Chor (pp, Old Aurethi), ferne Kirchenglocke, gedämpfte Pauke (Kronenschlag als Herzschlag), leises Ascheregen-Rauschen, Hörner (nur T. 25–32) |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Leitmotiv Asche-Form (Cello), Glut-Form (Hörner, T. 25–32) |
| Stimmung | Melancholisch, würdevoll, wehmütig, ein Glimmen unter der Asche |
| Dynamikverlauf | pp (Intro) → mp (T. 9) → mf (T. 25, Chor + Hörner) → p (T. 33–40), damit der Loop leise zurückkehrt |
| Soundcharakter / Mix | Breites Stereobild, Steinhallen-Hall ca. 2,8 s; Cello vorne mittig, Chor weit hinten, Glocke sehr fern links. Kein Subbass, keine harten Transienten. |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–40 (32 Takte). Naht: T. 40 endet auf a-Moll (v) und führt zurück auf d-Moll in T. 9. Glocke nicht in T. 40 oder T. 9 platzieren. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Wind und Ascherauschen, Glockenschlag in T. 1, Harfen-Arpeggien auf d-Moll; ab T. 5 spielt das Solo-Cello das Fragment „Die Krone“ (1 – 5 – ♭6 – 5).
- T. 9–16: Cello spielt das vollständige Leitmotiv (Asche-Form) zweimal; Streicherteppich d – B – C – d.
- T. 17–24: Wortlose Frauenstimme übernimmt die Melodie, Altflöte im Kontrapunkt, Pauke im halben Kronenschlag.
- T. 25–32: Chor setzt pp ein und wächst auf mf; Hörner spielen die Glut-Form (große Sexte); Harmonik d – G – C – d (dorische Aufhellung).
- T. 33–40: Rücknahme auf Harfe und Cello-Fragment, Schluss auf a-Moll als Rückleitung zu T. 9.

**Suno-Style-Prompt**
```text
dark fantasy orchestral elegy, D minor, 72 bpm, solo cello motif rising fifth, harp, low strings, wordless female voice, soft distant choir, far bell, melancholic, noble
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: ash wind, distant bell, harp arpeggios]
[Instrumental: solo cello, rising fifth, half-step sigh]

[Verse]
[Instrumental: solo cello melody, low strings]

[Verse]
[Wordless female voice]
Aaah... ooh... aaah...

[Build]
[Soft choir enters]
Au-reh cin-dra, au-reh dhum
Sol-vae es-keth, el-dra se-reth

[Climax]
[Warm horns, hopeful]
Kael rha-na, nos veth
Ith-ra ar, kael ar

[Interlude]
[Instrumental: harp and cello, very quiet]

[Verse]
[Instrumental: solo cello melody, low strings]

[Outro]
[Instrumental: harp, unresolved]
```

**Varianten**
- *Ohne Chor:* gleicher Prompt mit „instrumental, no vocals“ statt Stimm-Angaben – Ersatz-Take, falls Chorartefakte auftreten.
- *Klavier-Fassung:* Cover des Haupt-Takes mit „solo piano and cello“ für ein späteres Options-/Galeriemenü.

### D.2 `build_calm` – „Brennhollow erwacht“

- [ ] `music/build_calm.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `build_calm` → `assets/audio/music/build_calm.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/build_calm`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Calm · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Friedlicher Städtebau am Tag in der Aschmark (Brennhollow, M01 „Die letzte Siedlung“, Zeitalter I). Längste Hörzeit im Spiel – muss unaufdringlich und langlebig sein. |
| Genre | Pastorales Folk-Orchester mit dunklem Unterton, Ambient-Anteil |
| Instrumentierung | Gezupfte Laute (Ostinato), Holzflöte, Fidel, Bratschen, Harfe, Klarinette, leise Rahmentrommel, sehr seltene ferne Glocke |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Glut-Form nur als Fragment (Flöte), vollständig erst T. 29–36 |
| Stimmung | Hoffnungsvoll, arbeitsam, zart; leise Wehmut über das Verlorene |
| Dynamikverlauf | p bis mp, bewusst flach (max. ca. 6 dB Schwankung), keine Crescendi, keine Akzentschläge |
| Soundcharakter / Mix | Warm, nah, kleiner Holzraum (Hall 1,4 s); Laute leicht links, Fidel rechts, Flöte mittig. Tiefmitten schlank halten, damit Arbeits-SFX (Holzhacken, Steinbruch) Platz haben. |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C-Dur (♭VII) → d-Moll in T. 5. Laute-Ostinato läuft über die Naht durch. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30–4:00 |

**Arrangement**
- T. 1–4 (Einleitung): Laute-Ostinato allein, d-Moll mit dorischer Sexte (H).
- T. 5–20: Thema A – Holzflöte mit dem Glut-Fragment „Die Krone“, Bratschen als Liegeklänge.
- T. 21–28: Thema B – Fidel mit tänzerischer Figur, Rahmentrommel setzt leise ein.
- T. 29–36: Vollständige Glut-Form in Klarinette und Bratschen unisono, Harfe umspielt.
- T. 37–44: Interlude – nur Laute und Harfe, Raum zum Atmen.
- T. 45–52: Rückleitung; Flöte kehrt zurück, Harmonik G – C – C, Laute bleibt.

**Suno-Style-Prompt**
```text
pastoral dark fantasy folk orchestra, D dorian, 72 bpm, plucked lute ostinato, wooden flute, warm violas, fiddle, soft frame drum, hopeful yet wistful, gentle, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: plucked lute ostinato alone]

[Verse]
[Instrumental: wooden flute melody, warm violas]

[Verse]
[Instrumental: fiddle dance figure, soft frame drum]

[Build]
[Instrumental: clarinet and violas in unison, gentle]

[Interlude]
[Instrumental: lute and harp only]

[Verse]
[Instrumental: wooden flute returns, lute ostinato]

[Verse]
[Instrumental: fiddle and flute together, calm]

[Outro]
[Instrumental: lute ostinato continues, no ending]
```

**Varianten**
- *Basis für `build_night`:* Cover dieses Takes mit dem Nacht-Prompt (siehe 4.3.1) – sichert melodische Verwandtschaft.
- *Stem-Reduktion:* ohne Rahmentrommel für sehr lange Aufbauphasen (Empfehlung für spätere Zeitalter-Varianten).

### D.3 `build_tension` – „Rauch am Horizont“

- [ ] `music/build_tension.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `build_tension` → `assets/audio/music/build_tension.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/build_tension`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Tension · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Bedrohung angekündigt: Wellenwarnung (Bruder Hal Venn), Feind in Sichtweite, Einbruch der Nacht in M01. Wird per R1 aus `build_calm` eingeblendet. |
| Genre | Dunkle Orchester-Suspense |
| Instrumentierung | Celli-Achtelostinato, Streicher-Tremolo, gestopfte Hörner, Bassklarinette, Pauke im Kronenschlag, Rahmentrommeln, dezenter Hohlatem-Cluster (geflüsterter Chor) |
| Tempo / Takt | 72 BPM, 4/4 (Achtel- und Sechzehntelpuls) |
| Tonart / Motivform | D-äolisch, phrygische Färbung (E♭) in T. 17–24; Fragment „Die Krone“ in gestopften Hörnern, einmal Hohl-Andeutung (A♭) in T. 23 |
| Stimmung | Lauernd, nervös, kalt; das Dorf hält den Atem an |
| Dynamikverlauf | mp (T. 1) → mf (T. 17–24) → mp (T. 25–32); Anfang und Ende auf gleicher Dichte für die Naht |
| Soundcharakter / Mix | Dunkel, tiefmittenbetont, Hall mittel (2,0 s); Tremolo breit, Ostinato mittig, Flüstern weit hinten und gefiltert |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte). T. 1 muss als weicher Einstieg funktionieren: kein Akzent, nur Ostinato und Tremolo. Naht: T. 32 auf A (V) → d in T. 1. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Arrangement**
- T. 1–8: Celli-Ostinato, Streicher-Tremolo pp–mp, Bassklarinette tiefe Liegetöne.
- T. 9–16: Gestopfte Hörner mit „Die Krone“, Pauke im halben Kronenschlag.
- T. 17–24: Kronenschlag voll (Rahmentrommeln), phrygisches E♭, Hohl-Andeutung A♭ in T. 23, Flüstercluster.
- T. 25–32: Rücknahme auf Ostinato und Tremolo, Dichte wie T. 1.

**Suno-Style-Prompt**
```text
dark fantasy suspense orchestral, D minor, 72 bpm, pulsing low cello ostinato, string tremolo, muted horns, soft war drums, faint whispers, ominous, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Verse]
[Instrumental: low cello ostinato, string tremolo, no intro hit]

[Verse]
[Instrumental: muted horns, soft timpani heartbeat]

[Build]
[Instrumental: war drums 3-3-2, faint whispers, darker]

[Verse]
[Instrumental: cello ostinato and tremolo only]

[Build]
[Instrumental: muted horns, war drums, tense]

[Verse]
[Instrumental: cello ostinato and tremolo only, steady]
```

**Varianten**
- *Nacht-Variante (Stem):* ohne Rahmentrommeln, nur Ostinato und Flüstern – für die Phase vor der ersten Welle in M01.
- *Ersatz-Take:* bei zu starkem Crescendo im Take Abschnitt T. 1–16 doppeln (Loop 32 Takte aus zwei gleichen 16er-Blöcken mit Varianz in den Hörnern).

### D.4 `battle_small` – „Klingen im Nebel“

- [ ] `music/battle_small.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `battle_small` → `assets/audio/music/battle_small.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/battle_small`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Combat · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Kleine Gefechte: Späher, erste Hohlknechte, Scharmützel bis ca. 15 Einheiten. Eingang per R2, Übergang zu `battle_large` per R1. |
| Genre | Dark-Fantasy-Kampforchester |
| Instrumentierung | Streicher-Spiccato-Ostinato, Kriegstrommeln im Kronenschlag, Hörner, Posaunen, Rührtrommel, Becken sparsam; kein Chor (reserviert für `battle_large`) |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Glutruf im Intro, Glut-Form augmentiert in Posaunen (T. 21–36) |
| Stimmung | Dringlich, rau, entschlossen |
| Dynamikverlauf | f (Intro) → mf (T. 5–20) → f (T. 21–36) → mp (Breakdown T. 37–44) → mf (T. 45–52) |
| Soundcharakter / Mix | Trocken-kraftvoll, Hall kurz (1,6 s); Trommeln breit, Ostinato eng; Platz bei 2–4 kHz für Schwert-SFX lassen (Blech nicht zu hell) |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C (♭VII) → d in T. 5; Ostinato durchgehend. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Arrangement**
- T. 1–4 (Einleitung): Paukenwirbel, Glutruf der Hörner (1 – 5 – 8), Schlag auf T. 4 Zählzeit 4 als Auftakt.
- T. 5–20: Spiccato-Ostinato und Kronenschlag-Trommeln, Hörner mit „Die Krone“.
- T. 21–36: Glut-Form augmentiert in Posaunen, Streicherläufe aufwärts, Rührtrommel.
- T. 37–44: Breakdown – nur Trommeln und tiefe Streicher.
- T. 45–52: Wiederaufbau, Hörner kehren zurück, Dichte wie T. 5.

**Suno-Style-Prompt**
```text
dark fantasy battle orchestral, D minor, 144 bpm, driving string ostinato, war drums 3-3-2 rhythm, horns and trombones, snare, urgent and gritty, instrumental, no choir
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: timpani roll, horn call rising]

[Verse]
[Instrumental: string ostinato, war drums 3-3-2]

[Build]
[Instrumental: trombones melody, rising string runs, snare]

[Breakdown]
[Instrumental: drums and low strings only]

[Verse]
[Instrumental: horns return, string ostinato]

[Build]
[Instrumental: trombones melody, snare]

[Verse]
[Instrumental: string ostinato, war drums, steady]
```

**Varianten**
- *Cover-Basis:* Für `faction_ember_crown_battle` dient dieser Take als Cover-Ausgangsmaterial (gleiche Tonika, gleiche BPM).
- *Leichte Variante (Stem):* ohne Posaunen für sehr kurze Scharmützel.

### D.5 `battle_large` – „Die Aschmark brennt“

- [ ] `music/battle_large.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `battle_large` → `assets/audio/music/battle_large.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/battle_large`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Battle · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Große Schlachten: Wellenangriffe auf Brennhollow, Sturm auf den Hohlschrein, Isolde im Nahkampf, mehr als ca. 15 Einheiten. Aus `battle_small` per R1. |
| Genre | Episches Dark-Fantasy-Orchester mit Chor |
| Instrumentierung | Volles Orchester, gemischter Chor (Old Aurethi, Staccato-Rufe und lange Linien), Taiko-artige Großtrommeln, Ambosse, tiefes Blech, Glocken, dezente Subbass-Impulse |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Glut-Form im Chor (T. 25–40), Hohl-Form in Posaunen (Bridge T. 41–48), Glut-Form tutti (T. 49–64) |
| Stimmung | Überwältigend, verzweifelt-heroisch, brennend |
| Dynamikverlauf | ff-Schlag (Intro-Ende) → f (T. 9–24) → f (T. 25–40) → mf, dunkel (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Groß, druckvoll, Hall 2,4 s; Chor breit, Trommeln tief zentriert, Subbass nur auf Zählzeit 1 einzelner Takte; Limiter-Reduktion moderat halten |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf A (V) → d in T. 9; Chor-Phrasen dürfen nicht über die Naht klingen. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Chor-Ruf „Kael rhana!“, Großtrommeln im Crescendo, Blechcluster in T. 8.
- T. 9–24: Kronenschlag voll, Streicherostinato, Blech mit „Die Krone“.
- T. 25–40: Chor singt die Glut-Form in Augmentation, Ambosse auf Zählzeit 4.
- T. 41–48: Bridge – Hohl-Form in Posaunen, Cluster, der Feind schlägt zurück.
- T. 49–64: Höhepunkt – Chor und Blech mit voller Glut-Form, Glocken.
- T. 65–72: Rückführung auf die Dichte von T. 9.

**Suno-Style-Prompt**
```text
epic dark fantasy battle, full orchestra and choir, D minor, 144 bpm, pounding taiko 3-3-2, heavy brass, chanting choir in invented language, anvils, bells, relentless
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Choir shout]
Kael rha-na!
[Huge drums crescendo]

[Verse]
[Instrumental: string ostinato, pounding drums, brass]

[Chorus]
[Choir, long notes]
Au-reh cin-dra, kael rha-na
Nos veth, nos veth, ith-ra ar

[Bridge]
[Low trombones, dark]
Um-brath rhan, vae-loth dhum

[Climax]
[Full choir and brass, bells]
Kael ar! Kael ar!
Au-reh rha-na, el-dra veth

[Verse]
[Instrumental: string ostinato, pounding drums]

[Chorus]
[Choir, long notes]
Au-reh cin-dra, kael rha-na
```

**Varianten**
- *Instrumental-Take:* ohne Chor als Sicherheits-Take; Chor-Stem aus Haupt-Take ggf. darüberlegen.
- *Belagerungs-Kontakt:* Abschnitt T. 41–48 eignet sich als Material für `siege_defense`-Cover (Tonart dort nach C transponieren lassen).

### D.6 `boss_pale_reeve` – „Der Fahle Vogt“

- [ ] `music/boss_pale_reeve.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `boss_pale_reeve` → `assets/audio/music/boss_pale_reeve.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/boss_pale_reeve`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Boss · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Mini-Boss der ersten Mission: der Fahle Vogt, Hüter des Hohlschreins von Vael Dorn. Erster Kontakt des Spielers mit der Hohl-Form des Leitmotivs. Eingang per R4. |
| Genre | Dark-Fantasy-Bosskampf, Horror-Orchester |
| Instrumentierung | Tiefe Posaunen und Tuba, Kontrabässe col legno, Glasharmonika-Klang, Flüsterchor, tiefer Männerchor (Old Aurethi), Großtrommeln, Ketten als Perkussion, Streicher sul ponticello, tiefer Orgelpedalton D, Subbass |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch (E♭); Hohl-Form des Leitmotivs (1 – ♭5 – 5 – ♭5), Streicherglissandi aufwärts als Klang des „Seelenzugs“ |
| Stimmung | Bedrohlich, unheilig, kalt, unausweichlich |
| Dynamikverlauf | pp, Stille (T. 1–4) → ff-Schlag (T. 8) → f (T. 9–40) → p, unheimlich (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Dunkel, eng in den Höhen, langer unnatürlicher Hall (3,5 s) auf Chor und Glasklang, Trommeln trocken; Subbass-Pedal konstant, aber unter −18 dBFS RMS |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–72 (64 Takte). Naht: Orgelpedal D läuft durch; T. 72 endet auf E♭ (♭II) → D in T. 9. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Flüstern, Orgelpedal D, ab T. 5 Hohl-Form in Posaunen, Tutti-Schlag auf T. 8.
- T. 9–24: Phrygisches Ostinato (D – E♭) in Kontrabässen col legno, Kronenschlag in Ketten und Trommeln.
- T. 25–40: Tiefer Männerchor „Varkh dhum“ auf der Hohl-Form.
- T. 41–48: Breakdown – Glasharmonika, Flüstern, nur Puls (Raum für die Fähigkeit Seelenzug).
- T. 49–64: Höhepunkt – Chor tutti, Blechcluster, Streicherglissandi aufwärts.
- T. 65–72: Rückführung, Ostinato wie T. 9.

**Suno-Style-Prompt**
```text
dark fantasy boss battle, D phrygian, 144 bpm, menacing low brass, whispering choir, deep male chant, chains as percussion, eerie glass harmonica, sul ponticello strings
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Whispers, deep organ drone]
dhum... dhum... um-brath...
[Low trombones, tritone]
[Massive hit]

[Verse]
[Instrumental: col legno basses, chains, drums 3-3-2]

[Chorus]
[Deep male chant]
Varkh dhum, tor um-brath
Mor-ra ka, mor-ra ka

[Breakdown]
[Glass harmonica, whispers, pulse only]
vae-loth... dhum...

[Climax]
[Full choir, brass clusters, rising string glissandi]
Um-brath ka mor-ra!
Au-reh dhum, au-reh dhum!

[Verse]
[Instrumental: col legno basses, chains, drums]
```

**Varianten**
- *Phase 2 (Stem):* Ab 50 % Lebenspunkten des Vogts kann das Engine auf den Abschnitt T. 49–64 springen (taktsynchron); dafür Loop-Markierung im Prompt-Log vermerken.
- *Ersatz-Take:* ohne Gesang, Chor durch Orgel-Cluster ersetzt, falls die Silben unsauber klingen.

### D.7 `victory_sting` – „Glut über Asche“

- [ ] `music/victory_sting.ogg` · komplette Suno-Ausgabe, einmalig, kein Loop
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `victory_sting` → `assets/audio/music/victory_sting.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/victory_sting`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Sting · intro: None · looped: false · Set A |
| Zweck / Spielsituation | Missionssieg (z. B. Hohlschrein von Vael Dorn zerstört). Erstes Erklingen der Krone-Form (Dur) im Spiel. Danach `victory_theme` nach R5. |
| Genre | Orchestrale Siegesfanfare |
| Instrumentierung | Hörner, Trompeten, Posaunen, Paukenwirbel, Glocken, Chor-Akkord (Old Aurethi), Streicher |
| Tempo / Takt | 72 BPM, 4/4 (3 Takte + Ausklang) |
| Tonart / Motivform | D-Dur; Leitmotiv Krone-Form (1 – 5 – 6 – 5 · 4 – 3 – 2 – 7 – 1), verkürzt |
| Stimmung | Erlöst, edel, warm – kein überdrehter Triumph |
| Dynamikverlauf | mf (Wirbel) → ff (T. 2) → f gehalten → Ausklang |
| Soundcharakter / Mix | Hell und breit, Hall 3,0 s mit natürlichem Ausklang; Ausklang nicht abschneiden, sanfter Fade nur in den letzten 500 ms |
| Loop-Eignung (Hinweis für die Integration) | Keine (linear). |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 8–12 s inkl. Ausklang |

**Arrangement**
- T. 1: Paukenwirbel, Hörner spielen „Die Krone“ in Dur (D – A – H – A).
- T. 2: Tutti spielt den Fall in Dur (G – Fis – E – Cis), Chor setzt ein.
- T. 3: D-Dur-Akkord mit Glocken, Chor auf „Ith-ra ar“, gehalten.
- Danach: 3–4 s natürlicher Hallausklang.

**Suno-Style-Prompt**
```text
triumphant dark fantasy orchestral fanfare, D major, 72 bpm, short victory sting, horns and trumpets, choir chord, timpani roll, bells, warm and noble, clear ending
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Timpani roll, horns rising fifth]

[Climax]
[Full orchestra and choir, major key]
Ith-ra ar!

[Outro]
[Bells ringing, held chord]

[End]
```

**Varianten**
- *Ersatz:* Gelingt kein kurzer Take, einen kurzen, geschlossenen Take mit demselben Prompt erneut erzeugen (Schneiden aus längeren Takes übernimmt bei Bedarf die Integration).
- *Ohne Chor:* reine Blech-/Glocken-Fassung für Skirmish-Siege.

### D.8 `defeat_sting` – „Die Krone fällt“

- [ ] `music/defeat_sting.ogg` · komplette Suno-Ausgabe, einmalig, kein Loop
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `defeat_sting` → `assets/audio/music/defeat_sting.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/defeat_sting`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Sting · intro: None · looped: false · Set A |
| Zweck / Spielsituation | Missionsniederlage (Bergfried von Brennhollow fällt, Isolde stirbt). Danach `defeat_theme` nach R5. |
| Genre | Orchestraler Trauer-Sting |
| Instrumentierung | Tiefe Glocke, Posaunen, Solo-Cello, tiefe Streicher, gehauchter Chor |
| Tempo / Takt | 72 BPM, 4/4 (3 Takte mit Ritardando + Ausklang) |
| Tonart / Motivform | D-Moll mit Hohl-Form: Posaune 1 – ♭5, Cello-Fall 4 – ♭3 – ♭2 – ♭7 – 1 |
| Stimmung | Endgültig, kalt, still |
| Dynamikverlauf | f (Glocke) → mp → pp, verklingend |
| Soundcharakter / Mix | Dunkel, lang verhallt (4,0 s), Glocke tief und breit; letzte Klänge in die Stille ausklingen lassen |
| Loop-Eignung (Hinweis für die Integration) | Keine (linear). |
| Ziellänge | Ziel 8–12 s inkl. Ausklang |

**Arrangement**
- T. 1: Tiefer Glockenschlag, Posaunen spielen 1 – ♭5 (D – A♭).
- T. 2: Solo-Cello spielt den Fall mit ♭2 (G – F – E♭ – C), Ritardando.
- T. 3: Landung auf leerer Quinte D – A, gehauchter Chor, A♭ reibt leise nach.
- Danach: Ausklang in Stille.

**Suno-Style-Prompt**
```text
dark fantasy orchestral defeat sting, D minor, 72 bpm, deep funeral bell, mournful trombones, solo cello falling line, breathy choir, slow fade to silence, short
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Deep funeral bell, low trombones]

[Outro]
[Solo cello falling line, slowing down]
[Breathy choir]
cin-dra...

[End]
```

**Varianten**
- *Ersatz:* Gelingt kein kurzer Take, einen kurzen, geschlossenen Take mit demselben Prompt erneut erzeugen.
- *Ohne Chor:* für Skirmish-Niederlagen ohne Story-Bezug.

### D.9 `worldmap_peace` – „Eldraeth im Frieden“

- [ ] `music/worldmap_peace.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `worldmap_peace` → `assets/audio/music/worldmap_peace.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/worldmap_peace`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: WorldPeace · intro: None · looped: true (eingetragen) |
| Zweck / Spielsituation | Strategische Weltkarte ohne Krieg des Spielerreichs: Verwaltung, Handel, Dynastiepolitik. |
| Instrumentierung | Streicher legato, Holzflöte, Harfe, Laute, ferne Hörner, Rahmentrommel |
| Tempo / Takt | 78 BPM, 4/4 |
| Tonart | G-dorisch |
| Loop-Eignung (Hinweis für die Integration) | Hoch; gleichförmiger Hauptteil von 48 Takten nach der Einleitung |

**Suno-Style-Prompt**
```text
cinematic fantasy orchestral, G dorian, 78 bpm, calm kingdom at peace, legato strings, wooden flute, harp, lute, distant horns, soft frame drum, hopeful and wide, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental]

[Verse]
[Instrumental: main theme]

[Build]
[Instrumental: rising intensity]

[Verse]
[Instrumental: theme returns]

[Outro]
[Instrumental: no ending]
```

### D.10 `worldmap_war` – „Heere auf der Karte“

- [ ] `music/worldmap_war.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `worldmap_war` → `assets/audio/music/worldmap_war.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/worldmap_war`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: WorldWar · intro: None · looped: true (eingetragen) |
| Zweck / Spielsituation | Das Spielerreich ist im Krieg oder eine Schlacht steht an. |
| Instrumentierung | tiefe Streicher Ostinato, Kriegstrommeln, Blechbläser, Chor ohne Text, Becken |
| Tempo / Takt | 96 BPM, 4/4 |
| Tonart | G-dorisch (Set B, Moll-Färbung) |
| Loop-Eignung (Hinweis für die Integration) | Hoch; gleichförmiger Hauptteil von 48 Takten nach der Einleitung |

**Suno-Style-Prompt**
```text
dark cinematic orchestral war theme, G minor, 96 bpm, relentless low string ostinato, taiko and war drums, brass stabs, wordless male choir, tense and heroic, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental]

[Verse]
[Instrumental: main theme]

[Build]
[Instrumental: rising intensity]

[Verse]
[Instrumental: theme returns]

[Outro]
[Instrumental: no ending]
```

### D.11 `worldmap_hollow` – „Der Schleier reißt“

- [ ] `music/worldmap_hollow.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `worldmap_hollow` → `assets/audio/music/worldmap_hollow.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/worldmap_hollow`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: WorldHollow · intro: None · looped: true (eingetragen) |
| Zweck / Spielsituation | Hohlung des Reiches ab Pfad 2, Hohlung der Provinzen ≥ 30 oder Schleierdruck ≥ 60. |
| Instrumentierung | tiefe Drones, Glasharfe, gedämpfte Chöre, Aschglas-Perkussion, verstimmte Laute |
| Tempo / Takt | 64 BPM, 4/4 |
| Tonart | G-phrygisch |
| Loop-Eignung (Hinweis für die Integration) | Hoch; gleichförmiger Hauptteil von 48 Takten nach der Einleitung |

**Suno-Style-Prompt**
```text
dark ambient orchestral, G phrygian, 64 bpm, slow breathing drones, glass harmonica, hushed wordless choir, detuned lute, sparse cracked percussion, eerie and mournful, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental]

[Verse]
[Instrumental: main theme]

[Build]
[Instrumental: rising intensity]

[Verse]
[Instrumental: theme returns]

[Outro]
[Instrumental: no ending]
```

### D.12 `build_night` – „Wache über Brennhollow“

- [ ] `music/build_night.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `build_night` → `assets/audio/music/build_night.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/build_night`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Friedlicher Städtebau bei Nacht (Tag-Nacht-Zyklus). Wechsel Tag ↔ Nacht per R1 mit `build_calm` (gleiches Set). |
| Genre | Nächtliches Kammer-Ambient |
| Instrumentierung | Solo-Cello, Harfe, leise Bratschen, Altflöte, Laute (sparsam), weiche Glockentöne fern |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Thema von `build_calm` (Cover), Asche-Form im Cello (T. 29–36) |
| Stimmung | Still, wachsam, Lagerfeuerwärme gegen die Dunkelheit |
| Dynamikverlauf | pp → p, flach |
| Soundcharakter / Mix | Dunkel-warm, Hall 2,8 s; Höhen leicht abgesenkt, viel Raum für Nacht-Ambience (`ambience_night_loop`) |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C → d in T. 5; Harfen-Ostinato durchgehend. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30–4:00 |

**Arrangement**
- T. 1–4 (Einleitung): Harfe allein, langsames Ostinato.
- T. 5–20: Altflöte mit dem Thema aus `build_calm`, nun in Moll.
- T. 21–28: Bratschen-Liegeklänge, Laute einzelne Töne.
- T. 29–36: Solo-Cello mit Asche-Form.
- T. 37–52: Harfe und Altflöte, Glockenton fern in T. 41.

**Suno-Style-Prompt**
```text
nocturnal dark fantasy chamber music, D minor, 72 bpm, slow harp ostinato, alto flute, solo cello, soft violas, sparse lute, distant soft bells, quiet and watchful, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: slow harp ostinato alone]

[Verse]
[Instrumental: alto flute melody, harp]

[Verse]
[Instrumental: soft violas, sparse lute notes]

[Verse]
[Instrumental: solo cello melody, harp]

[Interlude]
[Instrumental: harp and alto flute, distant bell]

[Outro]
[Instrumental: harp ostinato continues, no ending]
```

**Varianten**
- *Cover aus `build_calm`* (bevorzugt) – sichert gleiche Harmonik für den Tag-Nacht-Crossfade.
- *Regen-Nacht:* Stem ohne Laute, wenn `ambience_rain_loop` aktiv ist.

### D.13 `explore_wilds` – „Jenseits der Wachfeuer“

- [ ] `music/explore_wilds.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `explore_wilds` → `assets/audio/music/explore_wilds.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/explore_wilds`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Erkundung: Späher und Held im Nebel des Krieges, Kartenrand, Entdeckung von Aschglas-Vorkommen und Ruinen. |
| Genre | Atmosphärisches Abenteuer-Orchester |
| Instrumentierung | Altflöte, Solo-Bratsche, Harfe, Streicher-Flageoletts, Holzperkussion, ferne tiefe Trommel, Hörner pp |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Fragment „Die Krone“ in Hörnern (T. 21–24), Fall-Figur in Bratsche (T. 37–40) |
| Stimmung | Neugierig, weit, vorsichtig, leise Gefahr |
| Dynamikverlauf | p → mp (T. 21–36) → p (T. 37–52) |
| Soundcharakter / Mix | Offen, sehr breit, Hall 3,0 s; Holzperkussion links/rechts verteilt (Schritte im Wald), Flöte mittig |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf G (IV) → d in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30–4:00 |

**Arrangement**
- T. 1–4 (Einleitung): Flageoletts und einzelne Harfentöne.
- T. 5–20: Altflöte mit tastender Melodie, Holzperkussion leise.
- T. 21–24: Hörner pp mit „Die Krone“.
- T. 25–36: Solo-Bratsche übernimmt, ferne Trommel im halben Kronenschlag.
- T. 37–40: Bratsche mit der Fall-Figur.
- T. 41–52: Harfe und Flageoletts, Rückleitung über G-Dur.

**Suno-Style-Prompt**
```text
atmospheric fantasy adventure orchestral, D dorian, 72 bpm, exploration, alto flute, solo viola, harp, string harmonics, wooden percussion, distant drum, curious and wary, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: string harmonics, single harp notes]

[Verse]
[Instrumental: alto flute searching melody, wooden percussion]

[Build]
[Instrumental: soft horns, distant drum]

[Verse]
[Instrumental: solo viola melody, harp]

[Interlude]
[Instrumental: harp and harmonics]

[Verse]
[Instrumental: alto flute melody, wooden percussion]

[Outro]
[Instrumental: harmonics, no ending]
```

**Varianten**
- *Dornwald-Erkundung:* Cover mit Holzflöte und Rindentrommeln (Tonart D halten).
- *Aurethwüste:* Cover mit trockenem Raum und tiefem Bordun für die Ruinenregion.

### D.14 `diplomacy_court` – „Kerzen im Thronsaal“

- [ ] `music/diplomacy_court.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `diplomacy_court` → `assets/audio/music/diplomacy_court.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/diplomacy_court`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set B |
| Zweck / Spielsituation | Diplomatie-Bildschirm: Verhandlungen mit König Aldric III., Torvald Eisenhand, Sylaen, Ophira Saelith-Varo, Bündnisse, Verträge. Aus der Weltkarte per R1 (Set B). |
| Genre | Höfische Kammermusik, dunkel gefärbt |
| Instrumentierung | Positiv-Orgel, Laute, Kammerstreicher (Quartett), Blockflöte, leise Handtrommel |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-dorisch; Glut-Form als höfisches Menuett-Zitat in der Blockflöte (T. 17–20) |
| Stimmung | Förmlich, abwägend, höflich-kühl |
| Dynamikverlauf | p → mp (T. 9–24) → p (T. 25–32) |
| Soundcharakter / Mix | Kleiner Saal, Hall 1,8 s; trocken und klar, damit Dialogtexte lesbar bleiben; Orgel weich |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte), T. 1 als weicher Einstieg. Naht: T. 32 auf D (V) → g in T. 1. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Arrangement**
- T. 1–8: Laute und Positiv-Orgel, höfische Kadenzen.
- T. 9–16: Streichquartett mit dem Weltkartenthema als Tanz.
- T. 17–20: Blockflöte zitiert die Glut-Form.
- T. 21–24: Handtrommel und Streicher, kurzer Schatten (Es-Dur).
- T. 25–32: Laute und Orgel, Rückleitung über D-Dur.

**Suno-Style-Prompt**
```text
dark fantasy court chamber music, G dorian, 80 bpm, positive organ, lute, string quartet, recorder, soft hand drum, formal and cool, diplomatic tension, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Verse]
[Instrumental: lute and soft organ, courtly cadences]

[Verse]
[Instrumental: string quartet, stately dance]

[Bridge]
[Instrumental: recorder melody, hand drum, slight shadow]

[Verse]
[Instrumental: lute and organ, calm]

[Verse]
[Instrumental: string quartet, stately dance]
```

**Varianten**
- *Saelith-Verhandlung:* Cover mit Hackbrett statt Orgel (Tonart G halten).
- *Kriegserklärung:* Übergang nach `intrigue_shadows` per R1.

### D.15 `intrigue_shadows` – „Kassians Netz“

- [ ] `music/intrigue_shadows.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `intrigue_shadows` → `assets/audio/music/intrigue_shadows.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/intrigue_shadows`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set B |
| Zweck / Spielsituation | Politische Intrigen: Spionage-Missionen, Verrat, Thronstreit (Zeitalter V *The Divided Throne*), Kassian Vale, Herzogin Mireille Corvane; zugleich Tension-Stufe auf der Weltkarte. |
| Genre | Verschwörerisches Kammer-Suspense |
| Instrumentierung | Streicher-Pizzicato, Col-legno-Celli, Bassklarinette, Solo-Kontrabass, gedämpfte Harfe, Zither-Tremolo, Flüstern sehr leise, Holzblock |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-phrygisch (A♭); verzerrtes Fragment „Die Krone“ mit chromatischem Riss (1 – 5 – ♭6 – 6 – ♭6) in Bassklarinette |
| Stimmung | Schleichend, misstrauisch, doppelbödig |
| Dynamikverlauf | p → mp (T. 9–24) → p (T. 25–32) |
| Soundcharakter / Mix | Nah, trocken, Hall 1,2 s; Pizzicato hart links/rechts, Bassklarinette mittig, Flüstern unterhalb der Wahrnehmungsschwelle |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte). Naht: T. 32 auf A♭ (♭II) → g in T. 1. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Arrangement**
- T. 1–8: Pizzicato-Muster und Holzblock, Kontrabass solo.
- T. 9–16: Bassklarinette mit verzerrtem Kronen-Fragment.
- T. 17–24: Col-legno-Celli und Zither-Tremolo, Flüstern.
- T. 25–32: Rücknahme auf Pizzicato und Kontrabass.

**Suno-Style-Prompt**
```text
conspiratorial dark fantasy chamber suspense, G phrygian, 80 bpm, string pizzicato, col legno cellos, bass clarinet, solo double bass, muted harp, zither tremolo, sneaky, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Verse]
[Instrumental: string pizzicato pattern, woodblock, solo double bass]

[Verse]
[Instrumental: bass clarinet melody, chromatic]

[Build]
[Instrumental: col legno cellos, zither tremolo, faint whispers]

[Verse]
[Instrumental: pizzicato and double bass only]
```

**Varianten**
- *Verrat enthüllt:* Stem mit lauterem Zither-Tremolo als Stinger-Ersatz in Skriptszenen.
- *Hof-Intrige:* Cover mit Positiv-Orgel, damit der Übergang von `diplomacy_court` farblich anschließt.

### D.16 `war_preparation` – „Schmieden und Schwüre“

- [ ] `music/war_preparation.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `war_preparation` → `assets/audio/music/war_preparation.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/war_preparation`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Kriegsvorbereitung: Kaserne im Dauerbetrieb, Rekrutierung, Aufmarsch vor einem Angriff, Belagerungsvorbereitung. Alternative Tension-Stufe zu `build_tension`, wenn der Spieler selbst angreift. |
| Genre | Treibendes Dark-Fantasy-Orchester |
| Instrumentierung | Amboss- und Schmiedeperkussion, Celli-Ostinato, Rührtrommel, Hörner, summender Männerchor, Rahmentrommeln, Kontrabässe |
| Tempo / Takt | 72 BPM, 4/4 (Sechzehntel-Ostinato) |
| Tonart / Motivform | D-äolisch; Glut-Form in Hörnern (T. 17–24), Kronenschlag auf dem Amboss |
| Stimmung | Entschlossen, schwer, unheilvoll-zielgerichtet |
| Dynamikverlauf | mp → mf (T. 9–24) → mp (T. 25–32) |
| Soundcharakter / Mix | Erdig, mittenbetont, Hall 1,8 s; Amboss klar, aber nicht spitz (kein Konflikt mit `sfx.build_progress`) |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte), T. 1 ohne Akzent. Naht: T. 32 auf A (V) → d in T. 1. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Arrangement**
- T. 1–8: Celli-Ostinato in Sechzehnteln, leiser Amboss im Kronenschlag.
- T. 9–16: Rührtrommel und Rahmentrommeln, summender Männerchor.
- T. 17–24: Hörner mit Glut-Form, Kontrabässe verstärken.
- T. 25–32: Rücknahme auf Ostinato und Amboss.

**Suno-Style-Prompt**
```text
driving dark fantasy orchestral, D minor, 72 bpm, war preparation, anvil and forge percussion 3-3-2, cello ostinato, snare, horns, humming male choir, determined, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Verse]
[Instrumental: cello ostinato, soft anvil rhythm, no intro hit]

[Verse]
[Snare and frame drums, humming male choir]
mmm... hmm... mmm...

[Build]
[Instrumental: horns melody, double basses]

[Verse]
[Instrumental: cello ostinato and anvil only]
```

**Varianten**
- *Skarnheim-Rüstung:* Cover mit Drehleier und tieferem Chor (Tonart D halten).
- *Vor der Belagerung:* Stem mit lauterer Rührtrommel als Vorstufe zu `siege_assault` (Set-Wechsel nach R2).

### D.17 `siege_assault` – „Sturm auf die Mauern“

- [ ] `music/siege_assault.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `siege_assault` → `assets/audio/music/siege_assault.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/siege_assault`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Battle · intro: None · looped: true · Set S |
| Zweck / Spielsituation | Der Spieler belagert eine Festung: Rammböcke, Katapulte, Sturm auf Mauern (z. B. Bergfesten Skarnheims, Festung Solmere im Vorfeld). |
| Genre | Episches Belagerungsorchester mit Chor |
| Instrumentierung | Großtrommeln mit Rammbock-Schlägen (Zählzeit 1 und 3), Kriegshörner, tiefes Blech, Streicher-Ostinato, Männerchor-Rufe (Old Aurethi), Ketten, Holzkrachen als Perkussion, Pauken |
| Tempo / Takt | 120 BPM, 4/4 |
| Tonart / Motivform | C-äolisch; Glut-Form augmentiert im Blech (T. 25–40), Kronenschlag in den Trommeln |
| Stimmung | Unaufhaltsam, wuchtig, opferbereit |
| Dynamikverlauf | f (Intro) → f (T. 9–24) → ff (T. 25–40) → mf (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Massiv, tief, Hall 2,2 s; Rammbock-Schläge mono-zentriert mit Subanteil, Blech breit |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf G (V) → c in T. 9. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Kriegshörner, erster Rammbock-Schlag in T. 5, Paukenwirbel zu T. 9.
- T. 9–24: Rammbock-Groove, Streicher-Ostinato, Ketten.
- T. 25–40: Blech mit Glut-Form, Männerchor-Rufe.
- T. 41–48: Breakdown – nur Trommeln und Holzkrachen (Katapult-Pausen).
- T. 49–64: Höhepunkt, Chor und Blech tutti.
- T. 65–72: Rückführung auf die Dichte von T. 9.

**Suno-Style-Prompt**
```text
epic dark fantasy siege assault, C minor, 120 bpm, battering ram drum hits, war horns, heavy brass, string ostinato, male choir shouts in invented language, chains, relentless
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[War horns, first battering ram hit, timpani roll]

[Verse]
[Instrumental: ram drum groove, string ostinato, chains]

[Chorus]
[Male choir shouts]
Tor! Tor! Kael rha-na!
O-run esk, o-run esk!

[Breakdown]
[Instrumental: drums and cracking wood only]

[Climax]
[Full choir and brass]
Kael ar! Nos veth!
Tor esk, kael rha-na!

[Verse]
[Instrumental: ram drum groove, string ostinato]
```

*Textbedeutung:* „Tor! Tor! Glut, erhebe dich! Der Stein fällt! Ewige Glut! Wir schwören! Das Tor fällt!“

**Varianten**
- *Ohne Chor:* für Skirmish-Belagerungen.
- *Nachtsturm:* Stem ohne Kriegshörner, mit stärkerem Streichertremolo.

### D.18 `siege_defense` – „Die Mauer hält“

- [ ] `music/siege_defense.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `siege_defense` → `assets/audio/music/siege_defense.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/siege_defense`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Battle · intro: None · looped: true · Set S |
| Zweck / Spielsituation | Die eigene Stadt wird belagert: Mauern unter Beschuss, Hohle oder Aschenlegion vor den Toren, letzte Verteidigung. |
| Genre | Verzweifelt-heroisches Orchester mit Chor |
| Instrumentierung | Sturmglocke, Pauken, tiefe Streicher, Hörner, Frauenchor (Klage) und Männerchor (Trotz), Großtrommeln, Streicher-Tremolo |
| Tempo / Takt | 120 BPM, 4/4 |
| Tonart / Motivform | C-phrygisch (D♭); Asche-Form im Frauenchor (T. 17–32), Glut-Form tutti im Höhepunkt (T. 49–64) |
| Stimmung | Verzweifelt, dann trotzig; Durchhalten gegen die Übermacht |
| Dynamikverlauf | mf (Glocke) → f (T. 9–16) → mf, klagend (T. 17–32) → f (T. 33–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Dicht, eng, Hall 2,4 s; Glocke links fern, Chöre breit, Pauken zentriert |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf D♭ (♭II) → c in T. 9; Glocke nicht über die Naht. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Sturmglocke allein, ab T. 5 Pauken und Streichertremolo.
- T. 9–16: Großtrommeln im Kronenschlag, tiefe Streicher.
- T. 17–32: Frauenchor klagt mit Asche-Form.
- T. 33–48: Männerchor antwortet trotzig, Hörner.
- T. 49–64: Glut-Form tutti, beide Chöre.
- T. 65–72: Rückführung, Trommeln und Streicher.

**Suno-Style-Prompt**
```text
desperate dark fantasy siege defense, C phrygian, 120 bpm, alarm bell, pounding timpani, low strings, horns, female choir lament, defiant male choir, invented language
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Alarm bell alone, then timpani and string tremolo]

[Verse]
[Instrumental: war drums 3-3-2, low strings]

[Verse]
[Female choir lament]
Li-ra-en sa do-mae
Cin-dra, cin-dra, se-reth

[Chorus]
[Defiant male choir, horns]
O-run veth! Ka esk!
Nos veth, do-mae ar!

[Climax]
[Both choirs, full orchestra]
Kael rha-na, o-run veth!
Do-mae ar, kael ar!

[Verse]
[Instrumental: war drums, low strings]
```

*Textbedeutung:* „Erinnerung an die Heimat. Asche, Asche, Klage. – Stein, halte den Eid! Kein Fallen! Wir schwören, ewiges Heim! – Glut, erhebe dich, Stein hält den Eid! Ewiges Heim, ewige Glut!“

**Varianten**
- *Letzte Bastion:* Stem nur Frauenchor und Pauken, wenn der Bergfried unter 25 % Struktur fällt.
- *Gegen die Aschenlegion:* Cover mit Amboss-Akzenten (Tonart C halten, passt zu Set F6).

### D.19 `boss_hollow_king` – „Varkh Dhum“

- [ ] `music/boss_hollow_king.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `boss_hollow_king` → `assets/audio/music/boss_hollow_king.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/boss_hollow_king`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Boss · intro: None · looped: true · Set E |
| Zweck / Spielsituation | Finaler Bosskampf in der Festung Solmere gegen den Hohlen König – Varkhan Aurelith, der Letzte Sonnenkönig, gefangen und vom Hohlen Tief entstellt. Musikalischer Gipfel des Spiels: Alle Formen der Aschenkrone treffen aufeinander. |
| Genre | Kolossales Dark-Fantasy-Finale, Orchester, Chor, Orgel |
| Instrumentierung | Volles Orchester, großer gemischter Chor (Old Aurethi), Pfeifenorgel, Glocken, verfremdete imperiale Fanfaren, Frauenstimme solo, Glasharmonika-Klang, Großtrommeln, Subbass |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch mit Hohl-Färbung; Krone-Form als gebrochene imperiale Fanfare (Intro), Hohl-Form dominant, Glut-Form gegen Hohl-Form im Kontrapunkt (Höhepunkt) |
| Stimmung | Tragisch, erschreckend, monumental; Mitleid mit dem gefallenen König |
| Dynamikverlauf | p (T. 1–8) → ff (T. 16) → f (T. 17–48) → p (T. 49–56) → fff (T. 57–72) → f (T. 73–80) |
| Soundcharakter / Mix | Riesige leere Halle, Hall 4,0 s; Orgel tief und breit, Chor raumfüllend, Frauenstimme nah und trocken (Erinnerung); Subbass kontrolliert |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 17–80 (64 Takte). Naht: Orgel-Bordun D läuft durch; T. 80 auf E♭ → D in T. 17. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30–4:00 |

**Arrangement**
- T. 1–8 (Einleitung): Orgel-Bordun, Frauenstimme singt „Solvae esketh“ auf der Asche-Form.
- T. 9–16 (Einleitung): Imperiale Fanfare spielt die Krone-Form in Dur und bricht in die Hohl-Form; Crescendo, Tutti-Schlag am Ende von T. 16.
- T. 17–32: Ostinato in Streichern und Trommeln, großer Chor „Varkh dhum“.
- T. 33–48: Imperialer Marsch (Legionsmarsch-Kennung), Glocken, Blech mit Hohl-Form.
- T. 49–56: Breakdown – Frauenstimme und Glasklang, die Erinnerung des Königs an Solmere.
- T. 57–72: Höhepunkt – Glut-Form (Hörner) gegen Hohl-Form (Posaunen) im Kontrapunkt, Chor tutti, Orgel voll.
- T. 73–80: Rückführung auf das Ostinato von T. 17.

**Suno-Style-Prompt**
```text
colossal dark fantasy final boss, D phrygian, 132 bpm, huge choir in invented language, pipe organ, broken imperial fanfares, war drums, bells, female solo, tragic and terrifying
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Organ drone, solo female voice, slow]
Sol-vae es-keth, sol-vae es-keth
[Imperial fanfare, breaking apart]
[Massive crescendo]

[Verse]
[Huge choir, war drums]
Varkh dhum, varkh dhum
Au-reh cin-dra, tor um-brath

[Chorus]
[Imperial march, bells, brass]
Varkh-an, varkh ar
Um-brath ka mor-ra

[Breakdown]
[Solo female voice, glass harmonica]
Li-ra-en sa do-mae
Sol-mere... sol-mere...

[Climax]
[Full choir, organ, horns against trombones]
Kael rha-na! Au-reh rha-na!
Varkh dhum, mor-ra veh vael!

[Verse]
[Huge choir, war drums]
Varkh dhum, varkh dhum
```

*Textbedeutung:* „Die Sonne fiel. – Hohler König, Krone aus Asche, Pforte zur Tiefe. – Varkhan, ewiger König, das Tief kennt keinen Tod. – Erinnerung an die Heimat, Solmere. – Glut, erhebe dich! Krone, erhebe dich! Hohler König, Tod und Ende!“

**Varianten**
- *Phase 2 (Stem):* ab 50 % Lebenspunkten taktsynchron auf T. 57–72 springen; im Prompt-Log Markierung festhalten.
- *Todesmoment:* Die Engine blendet nach dem Sieg nicht in `victory_sting`, sondern in `story_lament` (Skript) – Mitleid statt Triumph.

### D.20 `boss_root_mother` – „Die Wurzelmutter“

- [ ] `music/boss_root_mother.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `boss_root_mother` → `assets/audio/music/boss_root_mother.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/boss_root_mother`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Boss · intro: None · looped: true · Set X |
| Zweck / Spielsituation | Bosskampf gegen die Wurzelmutter, Naturentität des Dornwalds – uralt, nicht böse, aber gnadenlos; ggf. von der Hohlung berührt. |
| Genre | Urtümliches Wald-Boss-Orchester |
| Instrumentierung | Holz- und Rindentrommeln, knarrende Kontrabässe col legno, Flüsterchor und Frauenchor (Old Aurethi), verfremdete Holzflöten, dissonante Harfe, tiefe Streicher, gestrichene Metallklänge |
| Tempo / Takt | 12/8, ♩. = 84 (vier punktierte Viertel pro Takt) |
| Tonart / Motivform | A-äolisch mit chromatischen Rissen; Wurzelreigen (Dornenbund-Kennung) verdunkelt, Asche-Form im Frauenchor (T. 17–24) |
| Stimmung | Uralt, wogend, erstickend, ehrfurchtgebietend |
| Dynamikverlauf | p (Intro) → f (T. 9–16) → mf (T. 17–24) → ff (T. 25–36) → f (T. 37–40) |
| Soundcharakter / Mix | Dicht, organisch, Hall 3,0 s mit dunklem Charakter; Holz knarrt nah, Chöre umhüllen von allen Seiten |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–40 (32 Takte). Naht: T. 40 auf E (V) → a in T. 9; Wiegefigur durchgehend. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Knarren der Kontrabässe, Flüstern „calyre“, Rindentrommeln setzen ein.
- T. 9–16: 12/8-Groove der Holztrommeln, verdunkelter Wurzelreigen in Flöten.
- T. 17–24: Frauenchor mit Asche-Form, dissonante Harfe.
- T. 25–36: Höhepunkt – Chöre, alle Trommeln, tiefe Streicher.
- T. 37–40: Rückführung auf den Groove.

**Suno-Style-Prompt**
```text
primal dark fantasy forest boss, A minor, 12/8, 84 bpm, wooden and bark drums, creaking col legno basses, whispering and female choir in invented language, eerie flutes, harp
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Creaking wood, whispers]
ca-li-reh... ca-li-reh...
[Bark drums enter]

[Verse]
[Instrumental: wooden drums 12/8 groove, eerie flutes]

[Chorus]
[Female choir]
Ca-li-reh en o-run
Mor-ra veh ca-li-reh

[Climax]
[Full choirs, all drums, low strings]
Ca-li-reh ar! Ka esk!
El-dra, el-dra, ca-li-reh!

[Verse]
[Instrumental: wooden drums groove, flutes]
```

*Textbedeutung:* „Wachstum im Stein. Tod und Wachstum. Ewiges Wachstum! Kein Fallen! Welt, Welt, Wachstum!“

**Varianten**
- *Gehohlte Wurzelmutter (Stem/Cover):* mit Glasharmonika-Klang und Hohl-Färbung, falls die Kampagne die Wurzelmutter von der Hohlung befallen zeigt.
- *Versöhnung:* Nach dem Kampf statt Sieg-Sting die ruhige Wiegefigur aus `faction_thorn_covenant_theme` (Skript).

### D.21 `faction_ember_crown_battle` – „Glutklingen“

- [ ] `music/faction_ember_crown_battle.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `faction_ember_crown_battle` → `assets/audio/music/faction_ember_crown_battle.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/faction_ember_crown_battle`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Combat · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Kampfmusik, wenn gegen die Glutkrone gekämpft wird (Gefecht, Zeitalter V *The Divided Throne* – Bürgerkrieg gegen Herzogin Mireille Corvane) und als heroische Combat-Alternative bei Isoldes Großangriffen. |
| Genre | Heroisches Kampforchester mit Chor |
| Instrumentierung | Hörner, Trompeten, Streicher-Ostinato, Rührtrommeln, Großtrommeln im Kronenschlag, gemischter Chor, Glocken, Pauken |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Glutruf als Signal, Glut-Form augmentiert im Chor (T. 21–36) |
| Stimmung | Entschlossen, stolz, glühend |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–60) → f (T. 61–68) |
| Soundcharakter / Mix | Glänzend, kraftvoll, Hall 2,0 s; Blech hell, aber nicht schrill, Trommeln tief zentriert |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–68 (64 Takte). Naht: T. 68 auf A (V) → d in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–4 (Einleitung): Glutruf der Trompeten, Paukenwirbel.
- T. 5–20: Streicher-Ostinato, Rührtrommeln, Hörner mit der Prozessionsmelodie des Themas.
- T. 21–36: Chor mit Glut-Form, Glocken.
- T. 37–44: Breakdown – Trommeln und tiefe Streicher.
- T. 45–60: Höhepunkt, Chor und Blech tutti.
- T. 61–68: Rückführung.

**Suno-Style-Prompt**
```text
heroic dark fantasy battle orchestral, D dorian, 144 bpm, trumpet call, string ostinato, snare drums, war drums 3-3-2, choir in invented language, bells, proud and fierce
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Trumpet call, timpani roll]

[Verse]
[Instrumental: string ostinato, snare drums, horns melody]

[Chorus]
[Choir]
Kael rha-na! Nos veth!
Ith-ra ar, kael ar!

[Breakdown]
[Instrumental: drums and low strings]

[Climax]
[Full choir and brass]
Au-reh rha-na! Kael ar!

[Verse]
[Instrumental: string ostinato, snare drums]
```

**Varianten**
- *Cover aus dem Thema* (bevorzugt) oder aus `battle_small` (gleiches Set).
- *Bürgerkrieg:* Stem mit Hohl-Andeutung (A♭) in den Posaunen für Zeitalter V.

### D.22 `faction_skarn_clans_battle` – „Eisenhand“

- [ ] `music/faction_skarn_clans_battle.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `faction_skarn_clans_battle` → `assets/audio/music/faction_skarn_clans_battle.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/faction_skarn_clans_battle`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Combat · intro: None · looped: true · Set F2 |
| Zweck / Spielsituation | Kampf gegen die Clans von Skarnheim: schwere Infanterie, Schildwälle, Gebirgsschlachten. |
| Genre | Wuchtiges Nordclan-Kampforchester |
| Instrumentierung | Große Rahmentrommeln, Stampfen und Klatschen, Männerchor-Rufe, treibende Drehleier, tiefes Blech, Kriegshörner, Kontrabässe |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | E-äolisch; Ahnenschritt verdoppelt, Thema-Melodie in Drehleier, Asche-Form in Posaunen (T. 21–36) |
| Stimmung | Wild, unbeugsam, hart wie Eisen |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–60) → f (T. 61–68) |
| Soundcharakter / Mix | Roh, tief, Hall 2,6 s; Stampfen und Trommeln vorne, Chor breit |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–68 (64 Takte). Naht: T. 68 auf B (V) → e in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–4 (Einleitung): Kriegshörner, Stampfen, Ruf „Haa!“.
- T. 5–20: Trommeln und Drehleier treiben, Kontrabässe.
- T. 21–36: Posaunen mit Asche-Form, Chorrufe.
- T. 37–44: Breakdown – nur Stampfen, Klatschen und Bordun.
- T. 45–60: Höhepunkt, alle Kräfte.
- T. 61–68: Rückführung.

**Suno-Style-Prompt**
```text
fierce nordic dark fantasy battle, E minor, 132 bpm, huge frame drums, stomps and claps, driving hurdy gurdy, male choir shouts, low brass, war horns, unyielding, no guitars
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[War horns, stomps]
Haa!

[Verse]
[Instrumental: driving drums and hurdy gurdy, double basses]

[Chorus]
[Male choir shouts, trombones]
O-run! Veth! O-run! Veth!
Haa! Ka esk! Haa!

[Breakdown]
[Stomps, claps, drone only]

[Climax]
[Full choir, brass, drums]
O-run ar! Veth ar! Haa!

[Verse]
[Instrumental: driving drums and hurdy gurdy]
```

**Varianten**
- *Cover aus dem Thema* (bevorzugt).
- *Verbündete Clans:* Stem mit Glutruf-Zitat in den Hörnern, wenn Skarnheim an der Seite der Glutkrone kämpft.

### D.23 `faction_ashen_legion_theme` – „Die Ewige Legion“

- [ ] `music/faction_ashen_legion_theme.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `faction_ashen_legion_theme` → `assets/audio/music/faction_ashen_legion_theme.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/faction_ashen_legion_theme`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set F6 |
| Zweck / Spielsituation | Thema der Aschenlegion (Schwarz/Glutorange), verfluchte Ritter Aureths, ewige Legion des Hohlen Königs: Aufmarsch, Legionsfestungen, Aurethwüste. |
| Genre | Imperial-dunkler Marsch |
| Instrumentierung | Tiefes Blech, Amboss auf Zählzeit 4, Ketten, Männerchor (Old Aurethi), verfremdete Fanfaren, Großtrommeln, Kontrabässe |
| Tempo / Takt | 88 BPM, 4/4 |
| Tonart / Motivform | C-phrygisch (D♭); Legionsmarsch: Kronenschlag als Marsch, Hohl-Form im Blech (T. 13–20), verfremdete Krone-Form als „verlorene Herrlichkeit“ (T. 29–32) |
| Stimmung | Unerbittlich, stolz-verdammt, imperial |
| Dynamikverlauf | mf (Intro) → f (T. 5–28) → mf (T. 29–32) → f (T. 33–44) |
| Soundcharakter / Mix | Riesige leere Halle, Hall 3,2 s; Blech tief und breit, Amboss klar, Chor schwer |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–44 (40 Takte). Naht: T. 44 auf D♭ (♭II) → c in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–4 (Einleitung): Ketten, Amboss, ferne verfremdete Fanfare.
- T. 5–12: Legionsmarsch in Trommeln und Kontrabässen.
- T. 13–20: Blech mit Hohl-Form.
- T. 21–28: Männerchor „Aureh dhum, varkh ar“.
- T. 29–32: Verfremdete Krone-Form in gedämpften Trompeten (verlorene Herrlichkeit).
- T. 33–44: Marsch und Chor, Rückleitung.

**Suno-Style-Prompt**
```text
dark imperial fantasy march, C phrygian, 88 bpm, heavy low brass, anvil on beat four, chains, male choir in invented language, distorted fanfares, war drums, doomed and proud
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Chains, anvil, distant distorted fanfare]

[Verse]
[Instrumental: march drums, double basses]

[Verse]
[Instrumental: heavy brass melody, anvil]

[Chorus]
[Male choir]
Au-reh dhum, varkh ar
Veth ar, veth ar, sol-vae esk

[Bridge]
[Muted trumpets, faded glory]

[Chorus]
[Male choir, march]
Au-reh dhum, varkh ar
```

*Textbedeutung:* „Hohle Krone, ewiger König. Ewiger Eid, ewiger Eid, die Sonne fällt.“

**Varianten**
- *Aurethwüste (Stem):* ohne Chor, mit Wüstenwind-Ambience für Erkundung der Ruinen.
- *Instrumental:* Chor durch Posaunen ersetzt.

### D.24 `faction_ashen_legion_battle` – „Schwarze Glut“

- [ ] `music/faction_ashen_legion_battle.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `faction_ashen_legion_battle` → `assets/audio/music/faction_ashen_legion_battle.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/faction_ashen_legion_battle`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Combat · intro: None · looped: true · Set F6 |
| Zweck / Spielsituation | Kampf gegen die Aschenlegion: schwere verfluchte Ritter, Phalanx, Glutwaffen. |
| Genre | Brachiales Dark-Fantasy-Kampforchester |
| Instrumentierung | Tiefes Blech, Amboss in Sechzehnteln, Männerchor-Staccato, Streicher-Ostinato, Pauken, Großtrommeln, Ketten, verfremdete Fanfaren |
| Tempo / Takt | 88 BPM, 4/4 (doppelte Unterteilung, Sechzehntel-Ostinati) |
| Tonart / Motivform | C-phrygisch; Legionsmarsch beschleunigt, Hohl-Form augmentiert (T. 21–36) |
| Stimmung | Erdrückend, gnadenlos, glühend |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–52) |
| Soundcharakter / Mix | Massiv, Hall 2,6 s; Amboss und Pauken vorne, Chor breit, Blech mit leichter Sättigung |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf G (V, mit D♭ als Färbung) → c in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–4 (Einleitung): Fanfare bricht ab, Amboss-Salve, Chor „Aureh!“.
- T. 5–20: Streicher-Ostinato, Amboss in Sechzehnteln, Pauken.
- T. 21–36: Blech mit Hohl-Form augmentiert, Chor-Staccato.
- T. 37–44: Breakdown – Ketten und Großtrommeln.
- T. 45–52: Höhepunkt und Rückführung.

**Suno-Style-Prompt**
```text
brutal dark fantasy battle march, C phrygian, 88 bpm, sixteenth note anvil, heavy brass, staccato male choir in invented language, string ostinato, timpani, chains, crushing
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Broken fanfare, anvil burst]
Au-reh!

[Verse]
[Instrumental: string ostinato, sixteenth anvil, timpani]

[Chorus]
[Staccato male choir, heavy brass]
Au-reh! Varkh! Au-reh! Varkh!
Kael dhum, cin-dra ar!

[Breakdown]
[Instrumental: chains and war drums]

[Climax]
[Full choir and brass]
Varkh ar! Veth ar! Au-reh dhum!
```

**Varianten**
- *Cover aus dem Thema* (bevorzugt).
- *Legion mit Hohlem König:* Stem mit Orgel-Bordun als Brücke zu `boss_hollow_king` (Set-Wechsel nach R4).

### D.25 `story_revelation` – „Die Stimme aus dem Tief“

- [ ] `music/story_revelation.ogg` · komplette Suno-Ausgabe, einmalig, kein Loop
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `story_revelation` → `assets/audio/music/story_revelation.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/story_revelation`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: false · Set F5 |
| Zweck / Spielsituation | Enthüllungen und Wendepunkte: Die Stimme aus dem Tief spricht, Verrat (z. B. Kassian Vale), Erkenntnis über die wahre Natur des Hohlen Tiefs. |
| Genre | Psychologischer Horror-Score |
| Instrumentierung | Bassklarinette und Kontrabass (Tiefenstimme: Ton, der sich um einen Viertelton verbiegt), Streichercluster, verfremdete Stimme, Glasharmonika-Klang, Subbass, ein einzelner Glockenschlag |
| Tempo / Takt | 60 BPM, 4/4 |
| Tonart / Motivform | G♯-lokrisch; Tiefenstimme, Hohl-Form in der verfremdeten Stimme (T. 13–20) |
| Stimmung | Beklemmend, schwindelerregend, entlarvend |
| Dynamikverlauf | pp → f (T. 21–22, Schlag) → pp (T. 23–26) |
| Soundcharakter / Mix | Eng, dann plötzlich riesig; Hall 5,0 s auf der Stimme; Subbass langsam anschwellend |
| Loop-Eignung (Hinweis für die Integration) | Keine (linear). |
| Ziellänge | ca. 1:45 (26 Takte = 104 s) |

**Arrangement**
- T. 1–12: Tiefenstimme in Bassklarinette, Kontrabass antwortet, Cluster schleicht sich ein.
- T. 13–20: Verfremdete Stimme mit Hohl-Form, Subbass wächst.
- T. 21–22: Glockenschlag und Tutti-Cluster – die Enthüllung.
- T. 23–26: Nur Subbass und Glasklang, Ausklang.

**Suno-Style-Prompt**
```text
psychological dark fantasy horror score, G-sharp locrian, 60 bpm, bending bass clarinet, double bass, string clusters, eerie processed voice, glass harmonica, swelling sub bass
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: bending bass clarinet note, double bass answers]

[Verse]
[Eerie processed voice, slow]
Um-brath... nos... um-brath...
Ka mor-ra, ka vael

[Climax]
[Single bell strike, full string cluster]

[Outro]
[Sub bass and glass harmonica, fading]

[End]
```

*Textbedeutung:* „Das Tief … wir … das Tief … kein Tod, kein Ende.“

**Varianten**
- *Verrat (ohne Stimme):* instrumental für menschliche Enthüllungen ohne Bezug zum Tief.
- *Kurzfassung:* T. 13–26 (ca. 56 s).

### D.26 `mystery_ruins` – „Echos von Aureth“

- [ ] `music/mystery_ruins.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `mystery_ruins` → `assets/audio/music/mystery_ruins.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/mystery_ruins`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Ruinen Aureths: Aurethwüste, Ruinen von Solmere, auch der Ort des Hohlschreins von Vael Dorn vor dessen Entdeckung; Aschglas-Vorkommen. Passt zu `ambience_ruins_loop`. |
| Genre | Dunkles Ambient mit orchestralen Resten |
| Instrumentierung | Tiefer Bordun D, ferne Frauenstimme, verhallte Glocke, Streicher-Flageoletts, Kristallklänge (Aschglas), Wind-Textur, sehr leises Solo-Cello |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Asche-Form extrem gedehnt in Cello (T. 9–16) und Stimme (T. 21–28) |
| Stimmung | Verlassen, ehrfürchtig, rätselhaft |
| Dynamikverlauf | pp konstant, leichte Wellen |
| Soundcharakter / Mix | Sehr weit, Hall 6,0 s, viel Raum; Kristallklänge hoch und verstreut |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte). Bordun über die Naht, keine Glocke in T. 32 oder T. 1. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–8: Bordun, Wind, vereinzelte Kristallklänge.
- T. 9–16: Solo-Cello mit gedehnter Asche-Form.
- T. 17–20: Verhallte Glocke (T. 17), Flageoletts.
- T. 21–28: Ferne Frauenstimme mit dem Fall.
- T. 29–32: Bordun und Wind.

**Suno-Style-Prompt**
```text
dark ambient fantasy, D minor, slow and beatless, deep drone, ruins of a fallen empire, distant female voice, reverberant bell, string harmonics, crystal chimes, wind, no drums
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: deep drone, wind, crystal chimes]

[Verse]
[Instrumental: very soft solo cello, slow]

[Interlude]
[Reverberant bell, string harmonics]

[Verse]
[Distant female voice]
aaah... sol-vae... aaah...

[Outro]
[Instrumental: drone and wind, no ending]
```

**Varianten**
- *Aschglas-Höhle:* Stem mit lauteren Kristallklängen, ohne Stimme.
- *Solmere:* Cover mit Orgel-Bordun als Vorbote von Set E.

### D.27 `mystery_veilscar` – „Wo die Welt dünn wird“

- [ ] `music/mystery_veilscar.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `mystery_veilscar` → `assets/audio/music/mystery_veilscar.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/mystery_veilscar`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set F5 |
| Zweck / Spielsituation | Die Schleiernarbe (Veilscar) im tiefen Süden: Realität instabil, vom Hohlen Tief beherrscht; auch Orte, an denen der Schleier dünn ist. |
| Genre | Experimentelles Horror-Ambient |
| Instrumentierung | Glasharmonika-Klang, Waterphone-artige Klänge, rückwärts laufende Streicher, sich verbiegende Töne (Tiefenstimme), Cluster-Chor sehr leise, Subbass-Atem |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | Cluster um G♯; Hohlatem, Fragmente der Hohl-Form (T. 17–24) rückwärts |
| Stimmung | Desorientierend, falsch, unwirklich |
| Dynamikverlauf | pp → p (T. 17–24) → pp |
| Soundcharakter / Mix | Unnatürlich, Hall 8,0 s, Klänge wandern im Panorama; tiefe Frequenzen pulsieren langsam |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte); Atemrhythmus überbrückt die Naht. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–8: Subbass-Atem, Glasklang, wandernde Töne.
- T. 9–16: Rückwärts-Streicher, Waterphone-artige Klänge.
- T. 17–24: Cluster-Chor mit rückwärts gesetzten Fragmenten der Hohl-Form.
- T. 25–32: Nur Atem und sich verbiegende Töne.

**Suno-Style-Prompt**
```text
experimental horror ambient, G-sharp cluster, beatless, glass harmonica, waterphone textures, reversed strings, pitch bending tones, faint cluster choir, breathing sub bass, unreal
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: breathing sub bass, glass harmonica]

[Verse]
[Instrumental: reversed strings, waterphone textures]

[Interlude]
[Faint cluster choir]
dhum... vae... loth...

[Outro]
[Instrumental: breathing and bending tones, no ending]
```

**Varianten**
- *Tiefere Narbe:* Stem mit stärkerem Subbass für spätere Missionen in Zeitalter VI.
- *Ohne Chor:* reines Ambient.

### D.28 `mystery_old_gods` – „Die Schweigenden“

- [ ] `music/mystery_old_gods.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `mystery_old_gods` → `assets/audio/music/mystery_old_gods.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/mystery_old_gods`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set X |
| Zweck / Spielsituation | Schreine und Relikte der Alten Götter Ithren, Morvaine, Calyx, Thal und Oruun, die seit dem Aschenfall verstummt sind. |
| Genre | Ätherisches Sakral-Ambient |
| Instrumentierung | Je ein Klang pro Gott: hohe Streicher (Ithren, Licht), tiefe Frauenstimme (Morvaine, Übergang), Holzflöte (Calyx, Wachstum), Harfenwellen (Thal, Meer), tiefer Steinklang-Bordun (Oruun); ferner Chor |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | E-lydisch (♯4 als Zeichen des Unerreichbaren); kein Leitmotiv – nur ein einzelner aufsteigender Quintschritt (Aufschwung) als Erinnerung an die Krone (T. 25) |
| Stimmung | Erhaben, fern, still, wehmütig |
| Dynamikverlauf | pp → p (T. 17–28) → pp |
| Soundcharakter / Mix | Riesiger heller Raum, Hall 7,0 s; Klänge nacheinander, kaum gleichzeitig |
| Loop-Eignung (Hinweis für die Integration) | Sehr hoch. Ganze Datei ist der Loop (32 Takte); Bordun E über die Naht. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00 |

**Arrangement**
- T. 1–4: Steinklang-Bordun E (Oruun).
- T. 5–8: Hohe Streicher (Ithren).
- T. 9–12: Harfenwellen (Thal).
- T. 13–16: Holzflöte (Calyx).
- T. 17–24: Tiefe Frauenstimme (Morvaine), ferner Chor.
- T. 25–28: Alle fünf Farben sehr leise, aufsteigende Quinte in den Streichern.
- T. 29–32: Bordun allein.

**Suno-Style-Prompt**
```text
ethereal sacred fantasy ambient, E lydian, beatless, high strings, deep female voice, wooden flute, harp waves, stone-like low drone, distant choir, vast and silent, no drums
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: stone-like low drone]

[Verse]
[Instrumental: high strings, then harp waves]

[Verse]
[Instrumental: wooden flute, slow]

[Interlude]
[Deep female voice, distant choir]
Ith-ra... mor-ra... ca-li-reh...
tha-len... o-run...

[Outro]
[Instrumental: drone alone, no ending]
```

**Varianten**
- *Einzelschrein:* Stem mit nur einer Götterfarbe pro Schrein (z. B. nur Harfenwellen am Schrein des Thal).
- *Zeitalter VI:* Cover mit Hohl-Färbung (lydisch → lokrisch) für entweihte Schreine.

### D.29 `endgame_last_crown` – „Die Letzte Krone“

- [ ] `music/endgame_last_crown.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `endgame_last_crown` → `assets/audio/music/endgame_last_crown.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/endgame_last_crown`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set E |
| Zweck / Spielsituation | Zeitalter VII *The Last Crown*: Weltkarte und Aufbau im Endgame; Valemark, Verbündete und Feinde ringen um die Aschenkrone, Solmere ruft. Ersetzt Weltkarten- und Calm-Musik. |
| Genre | Monumentales Dark-Fantasy-Orchester |
| Instrumentierung | Volles Orchester, Chor pp–mf, Glocken, Hörner; Zitate der Fraktionskennungen (Drehleier – Skarnheim, Holzflöte – Dornenbund, Hackbrett – Saelith, Glutruf – Glutkrone) |
| Tempo / Takt | 66 BPM, 4/4 |
| Tonart / Motivform | D-äolisch → D-dorisch; Leitmotiv in Asche-, Glut- und Hohl-Form (Krone-Form bleibt dem Finale vorbehalten) |
| Stimmung | Schicksalsschwer, gesammelt, alles steht auf dem Spiel |
| Dynamikverlauf | p (Intro) → mf (T. 9–24) → f (T. 25–32) → mp (T. 33–48) |
| Soundcharakter / Mix | Groß und tief, Hall 3,5 s; Fraktionszitate klar erkennbar im Panorama verteilt |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–48 (40 Takte). Naht: T. 48 auf A (v) → d in T. 9. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30–4:00 |

**Arrangement**
- T. 1–8 (Einleitung): Glocke, Solo-Cello mit Asche-Form, Chor pp.
- T. 9–16: Streicher und Hörner mit Glut-Form; Glutruf-Zitat.
- T. 17–24: Fraktionszitate nacheinander (je 2 Takte: Drehleier, Holzflöte, Hackbrett, Hörner).
- T. 25–32: Hohl-Form in Posaunen gegen Glut-Form im Chor.
- T. 33–48: Rücknahme, Cello und Chor pp, Rückleitung über a-Moll.

**Suno-Style-Prompt**
```text
monumental dark fantasy orchestral, D minor to dorian, 66 bpm, fateful, choir in invented language, bells, horns, hurdy gurdy, wooden flute, hammered dulcimer accents, solo cello
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Bell, solo cello, soft choir]
Au-reh... au-reh...

[Verse]
[Instrumental: strings and horns, horn call]

[Interlude]
[Instrumental: hurdy gurdy, wooden flute, hammered dulcimer, horns in turn]

[Chorus]
[Choir against low trombones]
Au-reh vael, au-reh rha-na
Nos veth, el-dra veth

[Outro]
[Solo cello and soft choir, no ending]
Au-reh...
```

*Textbedeutung:* „Krone … – Ende der Krone, Krone, erhebe dich. Wir schwören, die Welt schwört.“

**Varianten**
- *Aufbau-Fassung (Stem):* ohne Chor im Höhepunkt, für lange Aufbauphasen im Zeitalter VII.
- *Verbündeten-Gewichtung:* Covers, in denen nur die Kennungen der tatsächlich verbündeten Fraktionen hervortreten.

### D.30 `endgame_final_battle` – „Vor den Toren Solmeres“

- [ ] `music/endgame_final_battle.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `endgame_final_battle` → `assets/audio/music/endgame_final_battle.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/endgame_final_battle`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest (Vorschlag) | intensity: Battle · intro: None · looped: true · Set E |
| Zweck / Spielsituation | Letzte Schlacht vor den Toren Solmeres: Vereinte Heere gegen Aschenlegion und Hohle. Mündet in `boss_hollow_king` (R4, gleiche Tonika D, Tempo 132). |
| Genre | Episches Finale, Orchester und großer Chor |
| Instrumentierung | Volles Orchester, großer Chor, alle Fraktionskennungen (Ahnenschritt-Trommeln, Wurzelreigen-Flöten, Gezeitenlauf-Hackbrett, Glutruf-Hörner) gegen Legionsmarsch-Amboss und Hohlatem-Cluster, Glocken, Orgel |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch → D-dorisch; Hohl-Form gegen Glut-Form, im Höhepunkt (T. 57–64) einmal die Krone-Form im vollen Tutti |
| Stimmung | Alles oder nichts, vereint, glühend |
| Dynamikverlauf | f (Intro) → f (T. 9–24) → ff (T. 25–48) → mf (T. 49–56) → fff (T. 57–64) → f (T. 65–72) |
| Soundcharakter / Mix | Maximal groß, Hall 3,0 s; Trommeln und Chor tragen, Fraktionskennungen kurz und klar hervorgehoben |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf E♭ (♭II) → D in T. 9. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:30 |

**Arrangement**
- T. 1–8 (Einleitung): Glutruf, Antwort der verfremdeten Legionsfanfare, Trommeln aller Fraktionen setzen ein.
- T. 9–24: Ostinato und Kronenschlag, Amboss der Legion dagegen.
- T. 25–48: Chor mit Glut-Form; Fraktionskennungen im Wechsel (Drehleier, Flöten, Hackbrett) über dem Groove.
- T. 49–56: Breakdown – Hohlatem-Cluster, Orgel, die Übermacht.
- T. 57–64: Höhepunkt – Krone-Form im vollen Tutti, Glocken.
- T. 65–72: Rückführung auf den Groove.

**Suno-Style-Prompt**
```text
epic dark fantasy final battle, D phrygian to dorian, 132 bpm, massive choir in invented language, full orchestra, war drums 3-3-2, anvils, hurdy gurdy, flutes, dulcimer, bells
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Horn call, distorted fanfare answers, all drums enter]

[Verse]
[Instrumental: ostinato, war drums 3-3-2, anvils]

[Chorus]
[Massive choir]
Kael rha-na! O-run veth!
Ca-li-reh, tha-len, nos rhan!

[Breakdown]
[Breathing cluster, organ, dark]
Um-brath rhan...

[Climax]
[Full orchestra and choir, major, bells]
Au-reh rha-na! Ith-ra ar!
Kael ar! El-dra ar!

[Verse]
[Instrumental: ostinato, war drums]
```

*Textbedeutung:* „Glut, erhebe dich! Stein, halte den Eid! Wachstum, Meer, wir steigen! – Das Tief steigt … – Krone, erhebe dich! Ewiges Licht! Ewige Glut! Ewige Welt!“

**Varianten**
- *Ohne Krone-Form (Stem):* falls die Kampagne eine dunkle Endvariante bietet – Höhepunkt mit Hohl-Form.
- *Instrumental:* Chor durch Blech ersetzt.

### D.31 `boss_stone_oath` – „Der Steinerne Eid“

- [ ] `music/boss_stone_oath.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `boss_stone_oath` → `assets/audio/music/boss_stone_oath.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/boss_stone_oath`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Boss · intro: None · looped: true · Set S |
| Zweck / Spielsituation | Bosskampf gegen Gharun, den Steinernen Eid, im Schreinsaal Oruuns (Mission 21, Zeitalter VI). Ein Wächter aus Runenstein, dessen Eid das Tiefe verdreht hat: Pflicht ohne Hass. |
| Genre | Schwerer, langsamer Boss-Marsch mit Männerchor |
| Instrumentierung | Tiefe Steintrommeln (Taiko-artig, aber aus Fels), Amboss, Kontrabässe, tiefe Posaunen, Männerchor (Skarn-Ahnenlieder in Old Aurethi), Rahmentrommeln, Steinglocken |
| Tempo / Takt | 96 BPM, 6/4 (zwei breite Dreiergruppen, „mahlender“ Mühlsteinpuls) |
| Tonart / Motivform | C-phrygisch; Aschenkrone-Motiv in den Posaunen verlangsamt, Ahnenruf (Skarnheim-Kennung) im Chor |
| Stimmung | Unerbittlich, ehrfürchtig, feierlich-traurig |
| Dynamikverlauf | p (Intro, T. 1–4) → mf (T. 5–12) → f (T. 13–20) → ff (T. 21–28) → mf (T. 29–32, Rückführung) |
| Soundcharakter / Mix | Steinhalle, Hall 3,5 s, trockene tiefe Impacts, Chor weit; Tiefen bis 40 Hz |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–32 (28 Takte = 70 s). Naht: T. 32 auf G (V) → c in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Suno-Style-Prompt**
```text
slow heavy dark fantasy boss march, C phrygian, 96 bpm, 6/4, deep stone drums, anvil, double basses, low trombones, male choir in invented language, solemn and relentless
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Single anvil strike, low stone drums enter]

[Verse]
[Instrumental: grinding 6/4 pulse, double basses, low trombones]

[Chorus]
[Male choir, slow and heavy]
Orun mah-ren, ka esk
Veth-ka orun, ka esk

[Climax]
[Full choir, anvil and stone drums, trombones with the crown theme slowed]
Orun, orun, esk ar!

[Verse]
[Instrumental: stone drums, basses]
```

*Textbedeutung:* „Stein, der bewahrt, nicht fällt. Eid des Steins, nicht fällt. Stein, Stein – kein Fall!“

**Varianten**
- *Erlösung (Cover):* nach dem Sieg ruhige Fassung nur mit Chor und Steinglocken (ca. 0:40) als Übergang zum Sieg-Sting.

### D.32 `boss_reef_mistress` – „Das Schwarze Riff“

- [ ] `music/boss_reef_mistress.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `boss_reef_mistress` → `assets/audio/music/boss_reef_mistress.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/boss_reef_mistress`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Boss · intro: None · looped: true · Set S |
| Zweck / Spielsituation | Bosskampf gegen Isarre Mol, Herrin des Schwarzen Riffs, auf der Riffinsel (Mission 14, Zeitalter IV): eine ertrunkene Gezeitenpriesterin, mütterlich und bedrohlich. |
| Genre | Maritimer Dark-Fantasy-Boss mit Frauenchor und Wellenrhythmik |
| Instrumentierung | Verstimmte Hackbrett-Arpeggien (Saelith-Kennung), tiefe Streicher mit langen Glissandi, Muschelhorn, Frauenchor, Wasser-Percussion (Taucherglocke, Kesselpauken), Harfe unter Wasser (Tiefpass) |
| Tempo / Takt | 12/8, ♩. = 88 (Wellenpuls) |
| Tonart / Motivform | E-dorisch mit tiefer zweiter Stufe (Riss); Saelith-Kennung im Hackbrett, Asche-Form im Frauenchor, verzerrt |
| Stimmung | Wogend, mütterlich-bedrohlich, nass, schwermütig |
| Dynamikverlauf | p (Intro, T. 1–8) → mf (T. 9–16) → f (T. 17–28) → ff (T. 29–36) → mf (T. 37–40) |
| Soundcharakter / Mix | Weiter, feuchter Hall (2,6 s), Chorus auf Streichern, Tiefpass-Wellen als Zwischenschicht, Brandungsrauschen leise im Hintergrund |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 9–40 (32 Takte). Naht: T. 40 auf B (V) → e in T. 9. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 3:00–3:30 |

**Suno-Style-Prompt**
```text
dark fantasy sea boss, E dorian, 12/8, 88 bpm, detuned dulcimer arpeggios, long cello glissandi, conch horn, female choir in invented language, diving bell percussion, wet and menacing
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Surf, diving bell, conch horn]

[Verse]
[Instrumental: detuned dulcimer arpeggios, 12/8 wave pulse, low strings]

[Chorus]
[Female choir, layered like waves]
Tha-lel nor mah-ren
Thal ka-lun, ar-esh

[Climax]
[Full choir, timpani, glissandi]
Tha-lel, tha-lel, esk ar!

[Verse]
[Instrumental: dulcimer and strings]
```

*Textbedeutung:* „Flut, die zurückkehrt. Thal (Gott des Meeres), tief, hoch! Flut, Flut, kein Fall!“

**Varianten**
- *Ebbe:* nach dem Sieg der Wellenpuls ohne Chor als Abklang (ca. 0:30).

### D.33 `escort_march` – „Der lange Zug“

- [ ] `music/escort_march.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `escort_march` → `assets/audio/music/escort_march.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/escort_march`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Tension · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Eskortmissionen (Mission 2, Zeitalter I): Flüchtlinge durch das Aschmoor führen; Spannung ohne Schlachtlärm. |
| Genre | Leiser Marsch mit gedämpfter Trommel und Solostreichern |
| Instrumentierung | Gedämpfte Rahmentrommel (Fußmarsch), Solo-Bratsche, Drehleier-Bordun, Holzflöte, entfernte Glocke; kein Blech |
| Tempo / Takt | 72 BPM, 4/4 (Schrittpuls) |
| Tonart / Motivform | D-äolisch; Asche-Form in Bratsche, Glut-Form andeutungsweise in der Flöte |
| Stimmung | Erschöpft, wachsam, leise Hoffnung |
| Dynamikverlauf | pp (T. 1–8) → p (T. 9–24) → mp (T. 25–32) → p (T. 33–40) |
| Soundcharakter / Mix | Trocken-nah, leichter Hall 1,6 s, Wind-Layer leise; viel Raum für Sprachausgabe (Mittenbereich 1–3 kHz zurückgenommen) |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 1–40 (40 Takte = 133 s), kein Intro. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Suno-Style-Prompt**
```text
quiet dark fantasy marching theme, D minor, 72 bpm, muffled frame drum steps, solo viola, hurdy gurdy drone, wooden flute, distant bell, weary and watchful, no brass
```

**Lyrics-/Struktur-Feld**
```text
[Instrumental: muffled frame drum steps, hurdy gurdy drone]
[Solo viola plays the ash motif, wooden flute answers]
[Distant bell every eight bars]
[Instrumental: quiet theme repeats, softer]
```

### D.34 `campaign_chronicle` – „Die Chronik der Wächterin“

- [ ] `music/campaign_chronicle.ogg` · komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)
- **Komplette Suno-Ausgabe (beste Version) liefern; Loop-Naht und Pegel übernimmt die Integration.** Keine Intro-/Loop-Trennung, kein Schneiden.

| Feld | Inhalt |
|---|---|
| ID / Ziel-Datei | `campaign_chronicle` → `assets/audio/music/campaign_chronicle.ogg` |
| Lieferung | Komplette Suno-Ausgabe (beste Version) als eine Datei `music/campaign_chronicle`, beliebiges Format. Loop-Naht und Pegel übernimmt die Integration. |
| Manifest | intensity: Menu · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Kampagnenmenü (Pergamentkarte, Briefing, Zusammenfassung): ruhige Chronikmusik, die zwischen den Missionen trägt. |
| Genre | Kammermusikalisches Dark-Fantasy-Thema mit Chor-Andeutung |
| Instrumentierung | Solo-Cello, Harfe, Laute, Holzflöte, leiser Frauenchor (Old Aurethi), Glockenspiel |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Aschenkrone-Motiv in Asche-Form (Cello), Glut-Form in der Flöte beim Auswahlwechsel des Zeitalters |
| Stimmung | Nachdenklich, würdevoll, wehmütig |
| Dynamikverlauf | p (Intro, T. 1–4) → mp (T. 5–20) → mf (T. 21–28) → p (T. 29–32) |
| Soundcharakter / Mix | Intimer Raum, Hall 2,0 s; Pergament-Knistern nicht in der Musik (kommt als UI-Effekt) |
| Loop-Eignung (Hinweis für die Integration) | Hoch. Loop T. 5–32 (28 Takte = 93 s). Naht: T. 32 auf A (V) → d in T. 5. |
| Ziellänge | Komplette Suno-Ausgabe, Ziel 2:30–3:00 |

**Suno-Style-Prompt**
```text
reflective dark fantasy chamber theme, D dorian, 72 bpm, solo cello, harp, lute, wooden flute, soft female choir in invented language, glockenspiel, dignified and wistful
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Harp arpeggio, solo cello enters]

[Verse]
[Instrumental: cello with the ash motif, lute accompaniment]

[Chorus]
[Soft female choir]
Li-ra-en sa do-mae
Kael rha-na, nos veth

[Verse]
[Instrumental: flute and harp, glockenspiel]
```

*Textbedeutung:* „Erinnerung an die Heimat. Krone, erhebe dich, wir schwören.“

---

## Kurzanleitung: Dateien ins Spiel bringen

1. Lieferung in einen Ordner legen, z. B. `lieferung/` (gemischt, auch verschiedene Formate).
2. Zuordnen, normalisieren, konvertieren und Bericht schreiben:

   ```text
   cargo run -p ashen_tools -- audio-scan --import lieferung --apply-renames --convert --report assets/audio/audio_report.md
   ```

   `--import` kopiert jede Datei anhand ihres Namens in den richtigen Ordner unter `assets/audio/` (bereits liegende Dateien werden nie überschrieben).
   Liegen die Dateien schon in den Zielordnern, genügt `audio-scan --apply-renames --convert --report`.
3. Bericht `assets/audio/audio_report.md` lesen: *Fehlende Dateien*, *Lücken in Variantennummern*, *Unbekannte Dateien*, *Beschädigte Dateien*, *Duplikate*
   abarbeiten und erneut scannen. Ziel: keine ungültigen Namen, Duplikate, Mehrdeutigkeiten, beschädigten Dateien.
4. Originale konvertierter Dateien (`.m4a` usw.) aus `assets/audio/` entfernen, sobald die `.ogg` existiert.
5. Hörtest im Spiel (`RUST_LOG=debug cargo run --release`): Logzeile `Audio: <n> Dateien zugeordnet, <m> Muster ohne Datei`, Details in
   [`audio_integration.md`](audio_integration.md), Abschnitt 8.
6. Liste jederzeit prüfen/neu ausgeben: `cargo run -p ashen_tools -- audio-list --format csv --out audio_liste.csv`.
