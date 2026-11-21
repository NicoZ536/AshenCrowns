# ASHEN CROWNS – Ambience-Loops (ElevenLabs Sound Effects)

Grundlage: `assets/audio/manifest.ron` (Sektion `ambience`), Regeln in `docs/audio/elevenlabs_production_guide.md`.

## 0. Grundprinzipien

1. **Ambiences sind Schichten.** Die Engine mischt die Manifest-Layer (`ambience_day`, `ambience_night`, `ambience_rain`,
   `ambience_ash_wind`, `ambience_village`, `ambience_ruins`) je nach Tageszeit, Wetter und Kameraposition. Deshalb enthält
   **jede Datei nur ihren eigenen Inhalt**: `ambience_day` ohne Regen, ohne Dorfgeräusche; `ambience_rain` ohne Vögel; usw.
   Doppelungen (z. B. Wind in drei Dateien) vermeiden – Wind gehört primär in `ambience_ash_wind`.
2. **Format:** `.ogg` Vorbis q6, 48 kHz, **stereo**, Ziellautheit **−23 LUFS integriert** (±1 LU), True Peak −1 dBTP. Manifest wendet
   zusätzlich −6 dB an.
3. **Ziel-Länge der fertigen Loops:** 60–120 s (Angabe je Loop). Da Sound-Effects-Generierungen derzeit auf ca. 30 s begrenzt sind
   (in der UI prüfen), entsteht jeder Loop aus **mehreren Schichten**, die im DAW zusammengesetzt werden.
4. **Werkzeug:** ElevenLabs *Sound Effects* mit **Loop-Option** (sofern verfügbar) für Flächen-Schichten; Detail-One-Shots ohne Loop.
   Prompt Influence für Flächen 0,3–0,45 (natürliche Variation), für Details 0,5–0,6.
5. **Keine Musik, keine Stimmen:** Prompts enthalten „no music, no voices“; generierte Wortfetzen/Melodien → verwerfen.
   Ausnahme: `ambience_village` darf **unverständliches, entferntes** Stimmengemurmel enthalten.

## 0.1 Schichtenaufbau (Standard)

| Schicht | Inhalt | Generierung | Anteil im Mix |
|---|---|---|---|
| **A – Bett** | durchgehende Fläche (Wind, Raumton, Regenrauschen) | SFX, max. Länge, Loop an, PI 0,3–0,4 | trägt die Lautheit (0 dB Referenz) |
| **B – Mittelgrund** | belebte Textur (Laub, Insekten, entfernte Arbeit) | SFX, 20–30 s, Loop an, PI 0,4 | −4 bis −8 dB |
| **C – Details** | Einzelereignisse (Vogelruf, Hundegebell, Knochenrasseln) | SFX, 1–5 s, Loop aus, PI 0,5–0,6 | −6 bis −12 dB, unregelmäßig platziert |

## 0.2 Schnitt für nahtlose Loops (Verfahren)

1. **Schichten mit unterschiedlicher Periodenlänge** anlegen (z. B. Bett 30 s, Mittelgrund 22 s, versetzt), damit sich innerhalb
   des Endloops kein erkennbares Muster im 30-s-Raster wiederholt. Bett-Schicht ggf. aus zwei verschiedenen Generierungen
   hintereinander (A1 + A2) bauen, Übergang per 2–4 s Equal-Power-Crossfade.
2. **Endlänge festlegen** (z. B. 90,000 s). Alle Schichten über die Endlänge hinaus auslegen.
3. **Wrap-Around-Crossfade:** Den Teil jeder Schicht, der über das Loop-Ende hinausragt (3–4 s), auf den Anfang der Spur
   zurücklegen und mit dem Anfang per Equal-Power-Crossfade überblenden. So ist Ende→Anfang nahtlos.
4. **Details (Schicht C)** mindestens 3 s von der Nahtstelle entfernt halten oder vollständig mitwrappen (nie abschneiden).
5. **Bounce** exakt auf Endlänge, Schnitt auf Nulldurchgang, **kein** Fade-in/-out an Anfang/Ende.
6. **Prüfen:** 5 Minuten Loop-Wiedergabe im Editor (keine Klicks, kein Pegelsprung, keine „Atmung“ an der Nahtstelle), danach im Spiel.
7. Loudness per **statischem Gain** auf −23 LUFS bringen (keine dynamische Normalisierung, die die Nahtstelle verändern könnte).

---

## 1. Manifest-Ambiences (Vertical Slice, Aschmark/Brennhollow)

### 1.1 `ambience/ambience_day_loop.ogg` – Tag in der Aschmark (Layer `ambience_day`, −6 dB)
*Charakter:* karges, verwundetes Grenzland bei Tag; spärliche Natur, die sich zurückkämpft; ruhig, ein wenig melancholisch.
**Länge:** 90 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A1 Bett | quiet open countryside daytime ambience, light breeze over dry grass and sparse fields, distant soft wind, calm, no music, no voices, stereo | 30 s | 0,35 | an |
| A2 Bett (zweite Generierung) | sparse rural landscape in late winter daytime, gentle breeze, faint rustle of dry leaves, very calm, no music, no voices | 30 s | 0,35 | an |
| B Mittelgrund | few distant songbirds and crows across a barren valley, sparse and far away, light insect hum, no music | 22 s | 0,4 | an |
| C1 Detail | single crow cawing in the distance, outdoors | 3 s | 0,55 | aus |
| C2 Detail | small flock of sparrows chirping briefly nearby, outdoors | 3 s | 0,55 | aus |
| C3 Detail | dry branch creaking in the wind, single creak | 2 s | 0,55 | aus |

**Mix/Schnitt:** A1+A2 hintereinander (60 s) plus erneut A1 versetzt bis 90 s; B als zweite Spur (22-s-Periode); Details 6–9 Ereignisse
unregelmäßig (Abstände 7–15 s), Krähe max. 2×. Keine Vogel-Dichte wie im Frühlingswald – Aschmark ist karg.
**Nachbearbeitung:** Hochpass 40 Hz, leichter Tiefpass 14 kHz (weich), Stereo-Breite natürlich.

### 1.2 `ambience/ambience_night_loop.ogg` – Nacht in der Aschmark (Layer `ambience_night`, −6 dB)
*Charakter:* kalte, bedrohliche Stille; die Toten könnten jederzeit kommen; sparsam, mit unheimlichen fernen Details.
**Länge:** 90 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A1 Bett | cold quiet night ambience in open countryside, low soft wind, deep stillness, faint distant air movement, no music, no voices, stereo | 30 s | 0,35 | an |
| A2 Bett | still winter night, very quiet low wind over empty fields, subtle low rumble of distance, no music, no voices | 30 s | 0,35 | an |
| B Mittelgrund | sparse night crickets far away, slow and thin, cold night, no music | 22 s | 0,4 | an |
| C1 Detail | lone owl hooting twice in the distance, night | 3 s | 0,55 | aus |
| C2 Detail | distant wolf howl echoing over hills, faint | 5 s | 0,5 | aus |
| C3 Detail | faint distant bell toll, single strike, eerie, far away | 5 s | 0,5 | aus |
| C4 Detail | dry twig snapping somewhere in the dark, single snap, close-mid distance | 1,5 s | 0,55 | aus |

**Mix/Schnitt:** Details sehr sparsam (4–6 Ereignisse in 90 s); die ferne Glocke (Anspielung auf die Glocke des Fahlen Vogts) max. 1×,
sehr leise (−14 dB). Grillen leiser als im Sommer (kalte Jahreszeit).
**Nachbearbeitung:** Hochpass 30 Hz, Tiefpass 12 kHz; leicht dunkler EQ (−2 dB @ 4 kHz) gegenüber Tag.

### 1.3 `ambience/ambience_rain_loop.ogg` – Regen (Layer `ambience_rain`, −6 dB)
*Charakter:* mittlerer, kalter Regen; wird über Tag/Nacht gelegt. Keine Vögel, kein Donner (Donner = `sfx/world/thunder_*`).
**Länge:** 60 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A1 Bett | steady medium rain falling on open fields and mud, even wide stereo rain hiss, no thunder, no music, no voices | 30 s | 0,35 | an |
| A2 Bett | constant cold rain on grass and puddles, consistent intensity, no thunder, no birds | 30 s | 0,35 | an |
| B Mittelgrund | rain dripping from thatched roofs and wooden eaves, irregular heavy drops, no thunder | 22 s | 0,4 | an |
| C1 Detail | water trickling from a gutter into a puddle, short | 4 s | 0,55 | aus |

**Mix/Schnitt:** Intensität konstant halten (keine An-/Abschwellphasen, sonst pumpt der Loop). B leise (−8 dB), da Dächer nicht überall sind.
**Nachbearbeitung:** Hochpass 60 Hz (Regen ohne Rumpeln), Tiefpass 15 kHz; Stereo breit.

### 1.4 `ambience/ambience_ash_wind_loop.ogg` – Aschewind (Layer `ambience_ash_wind`, −6 dB)
*Charakter:* Signatur-Ambience der Aschmark; trockener, böiger Wind, der feine Asche über Stein und Ruinen treibt; leicht unheimlich,
ein hohles Pfeifen. Über Tag und Nacht einsetzbar.
**Länge:** 90 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A1 Bett | dry gusty wind carrying fine ash and grit across barren land, soft hissing sand-like texture, hollow low whistle, no music, no voices, stereo | 30 s | 0,35 | an |
| A2 Bett | desolate wasteland wind, slow gusts rising and falling, dry grit hissing on stone, hollow and lonely | 30 s | 0,35 | an |
| B Mittelgrund | fine ash and dust skittering over rocks and dry ground, soft granular hiss, faint whistling through cracks | 22 s | 0,4 | an |
| C1 Detail | single strong wind gust howling through a ruined stone archway | 5 s | 0,5 | aus |
| C2 Detail | loose cloth or old banner flapping briefly in the wind | 3 s | 0,55 | aus |

**Mix/Schnitt:** Böen-Dynamik behalten, aber Lautheitsspitzen ≤ +4 LU über Mittelwert; Böen-Höhepunkte nicht an die Nahtstelle legen.
**Nachbearbeitung:** Hochpass 35 Hz; Notch 1–2 kHz −2 dB (hohler Klang); Stereo breit; ggf. sehr leichte Modulation (Chorus Mix 5 %) gegen Musterbildung.

### 1.5 `ambience/ambience_village_loop.ogg` – Brennhollow (Layer `ambience_village`, −6 dB)
*Charakter:* kleines Dorf (~30 Einwohner), das wieder zu leben beginnt; vorsichtig, arbeitsam, gedämpft – kein lauter Markt.
Wird abhängig von Kameranähe zur Siedlung eingeblendet.
**Länge:** 90 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A Bett | small quiet medieval village ambience, distant indistinct murmuring of a few people, crackling hearth fires, calm, no music, no clear words | 30 s | 0,4 | an |
| B1 Mittelgrund | distant medieval village work, occasional hammer on wood, wood being stacked, a cart creaking slowly, no music | 25 s | 0,4 | an |
| B2 Mittelgrund | farmyard sounds in the distance, a few chickens, a goat bleating once, no music | 22 s | 0,45 | an |
| C1 Detail | dog barking twice in the distance, village | 2 s | 0,55 | aus |
| C2 Detail | wooden bucket splashing into a well, rope creak | 3 s | 0,55 | aus |
| C3 Detail | blacksmith hammer striking an anvil three times, distant | 3 s | 0,55 | aus |
| C4 Detail | wooden door creaking open and closing, distant | 2,5 s | 0,55 | aus |

**Mix/Schnitt:** Stimmengemurmel **unverständlich** und leise (−10 dB); keine erkennbaren Wörter (sonst verwerfen). Details 8–10 Ereignisse.
Hammerschläge im Mix leiser als `sfx/buildings/build_progress_*`, damit Spieleraktionen hörbar bleiben.
**Nachbearbeitung:** Hochpass 50 Hz; Mitten um 300 Hz −2 dB (Platz für Dialog); Stereo mittelbreit.

### 1.6 `ambience/ambience_ruins_loop.ogg` – Ruinen von Vael Dorn (Layer `ambience_ruins`, −6 dB)
*Charakter:* Ruinen im Nordosten mit dem Hohlschrein; Schleier ist dünn: tiefe Drones, hohler Wind in Steinbögen, fernes Knochenklappern,
unterschwelliges Flüstern (nicht verständlich), Aschglas-Summen.
**Länge:** 90 s.

| Schicht | EN-Prompt | Dauer | PI | Loop |
|---|---|---|---|---|
| A1 Bett | eerie ruined stone temple ambience, low dark drone, hollow wind moving through broken arches, cold and empty, no music, no voices, stereo | 30 s | 0,35 | an |
| A2 Bett | ominous underground resonance, deep sub rumble, slow breathing-like air movement in stone corridors, no music | 30 s | 0,35 | an |
| B Mittelgrund | faint glassy humming of magic crystals with subtle shimmering overtones, unsettling, no melody | 22 s | 0,4 | an |
| C1 Detail | distant bones rattling and clattering on stone, brief | 3 s | 0,55 | aus |
| C2 Detail | faint unintelligible ghostly whisper drifting past, airy, no clear words | 4 s | 0,5 | aus |
| C3 Detail | small stones and grit falling from a crumbling wall | 2,5 s | 0,55 | aus |
| C4 Detail | cracked bell resonating faintly once, long dissonant decay, far | 5 s | 0,5 | aus |

**Mix/Schnitt:** Flüstern max. 2× und sehr leise (−14 dB), Wörter dürfen nicht verständlich sein. Glocken-Detail tonal zur Bossmusik
(B-Phrygisch) passend oder atonal. Sub-Anteil kontrollieren (kein Brummen < 30 Hz).
**Nachbearbeitung:** Hochpass 28 Hz; Hohle-Signatur subtil (Reverse-Reverb auf C2, Mix 15 %); Stereo breit, Mitte etwas offener.

---

## 2. Regionen-Ambiences für spätere Phasen (nicht im Manifest v1)

IDs sind **Vorschläge** nach Konvention `ambience/<ambience_id>_loop.ogg` (Präfix = Regions-ID aus dem Kanon). Je Region ein Tag- und
ein Nacht-Bett plus Spezial-Layer; Aufbau und Schnitt wie in Abschnitt 0. Länge 90–120 s.

### 2.1 Valemark (`valemark`) – fruchtbare Täler, Flüsse, Herzogtümer
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_valemark_day_loop.ogg` | A Bett | lush green river valley daytime, gentle breeze through tall grass and trees, many songbirds, distant flowing river, no music, no voices | 30 s | 0,35 |
| | B | wide slow river flowing over stones, soft and continuous | 25 s | 0,4 |
| | C | cuckoo call in the distance / distant church bell of a market town / cow lowing far away | 3–5 s | 0,55 |
| `ambience/ambience_valemark_night_loop.ogg` | A Bett | warm summer night in a fertile valley, dense crickets, frogs by a river, soft breeze | 30 s | 0,35 |
| `ambience/ambience_valemark_castle_loop.ogg` | A Bett | inside a large medieval castle courtyard, banners flapping, distant guards' footsteps, horses in stables, no clear words | 30 s | 0,4 |

### 2.2 Skarnheim (`skarnheim`) – nördliches Gebirge, Schnee, Eisen, Clans
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_skarnheim_day_loop.ogg` | A Bett | high mountain pass in winter, cold howling wind, snow hissing over rocks, vast open space, no music, no voices | 30 s | 0,35 |
| | B | distant creaking ice and occasional snow sliding off rocks | 22 s | 0,4 |
| | C | eagle cry echoing between peaks / distant war horn of a mountain hall / ice cracking | 3–5 s | 0,55 |
| `ambience/ambience_skarnheim_night_loop.ogg` | A Bett | freezing mountain night, low steady wind, deep silence, faint snow hiss | 30 s | 0,35 |
| `ambience/ambience_skarnheim_hall_loop.ogg` | A Bett | inside a stone mountain fortress hall, large fire roaring in a hearth, distant forge hammering, muffled wind outside, no clear words | 30 s | 0,4 |

### 2.3 Der Dornwald (`thornwild`) – uralter Wald, Druidenzirkel, Wurzelmutter
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_thornwild_day_loop.ogg` | A Bett | ancient dense forest daytime, deep canopy rustle, creaking old trees, rich birdsong, humid and alive, no music, no voices | 30 s | 0,35 |
| | B | insects buzzing in undergrowth, dripping water from leaves | 22 s | 0,4 |
| | C | large old tree groaning in the wind / woodpecker drumming / moth wings fluttering close | 2–5 s | 0,55 |
| `ambience/ambience_thornwild_night_loop.ogg` | A Bett | mysterious ancient forest at night, soft wind in high branches, strange distant animal calls, owls, faint luminous hum | 30 s | 0,35 |
| `ambience/ambience_thornwild_grey_loop.ogg` | A Bett | corrupted dying forest, dry brittle leaves, slow creaking, hollow wind, no birds, faint low drone, unsettling | 30 s | 0,35 |

### 2.4 Die Saelith-Inseln (`saelith`) – Handelsrepublik, Seefahrt
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_saelith_harbor_loop.ogg` | A Bett | busy medieval harbor ambience, waves lapping against wooden piers, creaking ships and ropes, seagulls, distant indistinct crowd, no music, no clear words | 30 s | 0,4 |
| | B | ship rigging creaking and ropes straining in the wind | 22 s | 0,4 |
| | C | ship bell ringing twice / crates being dropped on a dock / gull calling close | 2–4 s | 0,55 |
| `ambience/ambience_saelith_coast_day_loop.ogg` | A Bett | warm island coast, gentle surf rolling on sand, soft sea breeze, distant gulls | 30 s | 0,35 |
| `ambience/ambience_saelith_night_loop.ogg` | A Bett | calm night sea, slow waves on rocks, soft wind, distant creaking anchored ships | 30 s | 0,35 |

### 2.5 Die Aurethwüste (`aureth_wastes`) – Ruinen des alten Reiches, Solmere
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_aureth_wastes_day_loop.ogg` | A Bett | vast ash desert, hot dry wind, sand and ash hissing over ancient stone ruins, empty and desolate, no music, no voices | 30 s | 0,35 |
| | B | sand trickling down broken marble columns, faint whistling through ruins | 22 s | 0,4 |
| | C | distant collapsing stone / metal fragment clinking in the wind / dry gust through a doorway | 2–5 s | 0,55 |
| `ambience/ambience_aureth_wastes_night_loop.ogg` | A Bett | cold desert night among ruins, low wind, deep silence, faint metallic creaks | 30 s | 0,35 |
| `ambience/ambience_solmere_throne_loop.ogg` | A Bett | enormous ruined throne hall, low resonant drone, faint echoing drips, cold air moving, distant metallic armor creaks, ominous, no music | 30 s | 0,35 |

### 2.6 Die Schleiernarbe (`veilscar`) – vom Hohlen Tief beherrscht, Realität instabil
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_veilscar_loop.ogg` | A Bett | unstable otherworldly wasteland, warped low drone, wind that seems to blow backwards, distorted distant echoes, deeply unsettling, no music | 30 s | 0,3 |
| | B | glassy crystal resonance and shimmering tones bending in pitch, no melody | 22 s | 0,35 |
| | C | reversed whisper swell / deep sub pulse like a heartbeat underground / sudden silence gap then air rush | 3–5 s | 0,5 |
| `ambience/ambience_hollow_deep_loop.ogg` | A Bett | bottomless cavern beneath the world, immense dark space, slow breathing-like rumble, faint distant choir-like drone, no words | 30 s | 0,3 |

*Nachbearbeitung Schleiernarbe:* Hohle-Signatur deutlich (Reverse-Reverb, langsames Pitch-Wobble ±10 Cent auf Schicht A, Spektral-Freeze auf C).

### 2.7 Glutweite (`cinderreach`) – Vulkanregion des Aschdrachen
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_cinderreach_loop.ogg` | A Bett | volcanic landscape, deep rumbling earth, bubbling lava, hot hissing vents, ash wind, no music, no voices | 30 s | 0,35 |
| | B | lava flowing and crackling as it cools, steam hissing | 22 s | 0,4 |
| | C | distant volcanic eruption boom / rocks cracking from heat / huge creature breathing far away inside a mountain | 3–5 s | 0,5 |
| `ambience/ambience_cinderreach_lair_loop.ogg` | A Bett | inside a vast volcanic cave lair, deep slow rumble, heat shimmer hiss, piles of coins shifting faintly, slow enormous breathing echoing | 30 s | 0,35 |

### 2.8 Aschmark-Ergänzungen (`ashmarch`)
| Vorgeschlagene Datei | Schicht | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| `ambience/ambience_ashmarch_battlefield_loop.ogg` | A Bett | aftermath of a medieval battle, smoldering fires crackling, crows cawing, wind over a quiet field, distant moans, no music | 30 s | 0,4 |
| `ambience/ambience_ashmarch_fog_loop.ogg` | A Bett | thick cold fog over marshland, muffled dripping, soft low wind, distant indistinct sounds, oppressive | 30 s | 0,35 |
| `ambience/ambience_ashmarch_chapel_loop.ogg` | A Bett | interior of a small damaged stone chapel, gentle candle flame flicker, wind through a broken roof, soft drips, peaceful | 30 s | 0,4 |

---

## 3. Abnahme-Checkliste Ambience

- [ ] Dateiname exakt (`ambience/<id>_loop.ogg`), stereo, 48 kHz, Vorbis q6.
- [ ] Länge im Ziel (Manifest-Loops 60–90 s), Schnitt auf Nulldurchgang, keine Fades an den Enden.
- [ ] 5 Minuten Loop-Test ohne Klick/Pegelsprung/erkennbare Wiederholung.
- [ ] Inhalt sauber getrennt (kein Regen im Tag-Loop, keine Vögel im Regen-Loop, keine Dorfgeräusche in Ruinen).
- [ ] Keine verständlichen Wörter, keine Musikfragmente.
- [ ] −23 LUFS integriert (±1), −1 dBTP; im Spiel gegen Musik und Dialog geprüft (Ambience darf Dialog nie verdecken).
