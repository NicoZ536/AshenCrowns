# ASHEN CROWNS – Ambience-Loops (ElevenLabs Sound Effects)

Grundlage: `assets/audio/manifest.ron` (Sektion `ambience`), Regeln in `docs/audio/elevenlabs_production_guide.md`.

## 0. Grundprinzipien

1. **Ambiences sind Schichten im Spiel.** Die Engine mischt die Manifest-Layer (`ambience_day`, `ambience_night`, `ambience_rain`,
   `ambience_ash_wind`, `ambience_village`, `ambience_ruins`) je nach Tageszeit, Wetter und Kameraposition. Deshalb enthält
   **jede Datei nur ihren eigenen Inhalt**: `ambience_day` ohne Regen, ohne Dorfgeräusche; `ambience_rain` ohne Vögel; usw.
   Doppelungen (z. B. Wind in drei Dateien) vermeiden – Wind gehört primär in `ambience_ash_wind`.
2. **Eine Datei = eine komplette Generierung.** Je Ambience wird **ein** zusammengeführter Prompt (siehe Abschnitt 1) mit der angegebenen Dauer
   (derzeit höchstens ca. 30 s, in der UI prüfen) und eingeschalteter **Loop-Option** generiert; die beste komplette Generierung wird unverändert geliefert.
   Keine Schichtung im DAW, kein Zusammensetzen mehrerer Takes, kein Schneiden.
3. **Werkzeug:** ElevenLabs *Sound Effects*, Prompt Influence 0,35–0,45 (Flächen: natürliche Variation), Loop **an**.
4. **Keine Musik, keine Stimmen:** Prompts enthalten „no music, no voices“; generierte Wortfetzen/Melodien → Take verwerfen und neu generieren.
   Ausnahme: `ambience_village` darf **unverständliches, entferntes** Stimmengemurmel enthalten.
5. **Format:** beliebig; Konvertierung nach `.ogg` Vorbis q6, 48 kHz, **stereo** übernimmt die Integration.

## 0.1 Erledigt die Integration (nicht vom Nutzer)

- **Verlängerung** der gelieferten Generierung auf die Ziel-Länge von 60–120 s (Angabe je Ambience), z. B. durch mehrfaches Auslegen mit versetzten Kopien und Equal-Power-Crossfade
  (damit sich im Endloop kein Muster im 30-s-Raster wiederholt); bei Bedarf zusätzliche Detail-Ereignisse aus weiteren Generierungen einstreuen.
- **Nahtlose Loops:** Wrap-Around-Crossfade der Überhänge (3–4 s) an den Anfang, Details mindestens 3 s von der Naht entfernt, Schnitt auf Nulldurchgang,
  **kein** Fade-in/-out an Anfang/Ende, 5 Minuten Probe-Loop im Editor und im Spiel (keine Klicks, kein Pegelsprung, keine „Atmung“ an der Nahtstelle).
- **Lautheit:** −23 LUFS integriert (±1 LU), True Peak ≤ −1 dBTP per **statischem Gain** (keine dynamische Normalisierung, die die Nahtstelle verändern könnte); Manifest wendet zusätzlich −6 dB an.
- **Format:** Stereo, 48 kHz, Vorbis q6; Hochpass/EQ je Ambience laut Eintrag.

---

## 1. Manifest-Ambiences (Vertical Slice, Aschmark/Brennhollow)

### 1.1 `ambience/ambience_day_loop.ogg` – Tag in der Aschmark (Layer `ambience_day`, −6 dB)
*Charakter:* karges, verwundetes Grenzland bei Tag; spärliche Natur, die sich zurückkämpft; ruhig, ein wenig melancholisch.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | quiet open countryside daytime ambience in a barren borderland, light breeze over dry grass and sparse fields, faint rustle of dry leaves, a few distant songbirds and a single crow far away, calm and slightly melancholic, no music, no voices, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 90 s. Keine Vogel-Dichte wie im Frühlingswald (Aschmark ist karg), Krähe max. 2× pro 90 s. Hochpass 40 Hz, leichter Tiefpass 14 kHz (weich), Stereo-Breite natürlich.

### 1.2 `ambience/ambience_night_loop.ogg` – Nacht in der Aschmark (Layer `ambience_night`, −6 dB)
*Charakter:* kalte, bedrohliche Stille; die Toten könnten jederzeit kommen; sparsam, mit unheimlichen fernen Details.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | cold quiet night ambience in open countryside, low soft wind over empty fields, deep stillness, sparse thin crickets far away, a lone owl hooting twice and a faint distant wolf howl, no music, no voices, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 90 s. Details sehr sparsam (4–6 Ereignisse in 90 s); Grillen leiser als im Sommer (kalte Jahreszeit). Hochpass 30 Hz, Tiefpass 12 kHz; leicht dunkler EQ (−2 dB @ 4 kHz) gegenüber Tag.

### 1.3 `ambience/ambience_rain_loop.ogg` – Regen (Layer `ambience_rain`, −6 dB)
*Charakter:* mittlerer, kalter Regen; wird über Tag/Nacht gelegt. Keine Vögel, kein Donner (Donner = `sfx/world/thunder_*`).
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | steady medium cold rain falling on open fields, mud and puddles, even wide stereo rain hiss with soft irregular drips from thatched roofs and a gutter trickling into a puddle, constant intensity, no thunder, no birds, no music, no voices, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 60 s. Intensität konstant halten (keine An-/Abschwellphasen, sonst pumpt der Loop). Hochpass 60 Hz (Regen ohne Rumpeln), Tiefpass 15 kHz; Stereo breit.

### 1.4 `ambience/ambience_ash_wind_loop.ogg` – Aschewind (Layer `ambience_ash_wind`, −6 dB)
*Charakter:* Signatur-Ambience der Aschmark; trockener, böiger Wind, der feine Asche über Stein und Ruinen treibt; leicht unheimlich, ein hohles Pfeifen. Über Tag und Nacht einsetzbar.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | dry gusty wind carrying fine ash and grit across barren land, soft hissing sand-like texture, hollow low whistle, an occasional stronger gust howling through ruined stone, desolate and lonely, no music, no voices, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 90 s. Böen-Dynamik behalten, aber Lautheitsspitzen ≤ +4 LU über Mittelwert; Böen-Höhepunkte nicht an die Nahtstelle legen. Hochpass 35 Hz; Notch 1–2 kHz −2 dB (hohler Klang); Stereo breit; ggf. sehr leichte Modulation (Chorus Mix 5 %) gegen Musterbildung.

### 1.5 `ambience/ambience_village_loop.ogg` – Brennhollow (Layer `ambience_village`, −6 dB)
*Charakter:* kleines Dorf (~30 Einwohner), das wieder zu leben beginnt; vorsichtig, arbeitsam, gedämpft – kein lauter Markt. Wird abhängig von Kameranähe zur Siedlung eingeblendet.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | small quiet medieval village ambience, distant indistinct murmuring of a few people, crackling hearth fires, occasional distant hammer on wood, a cart creaking slowly, a few chickens and a dog barking far away, calm and cautious, no music, no clear words, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 90 s. Stimmengemurmel **unverständlich** und leise (−10 dB); keine erkennbaren Wörter (sonst Generierung ersetzen). Hammerschläge leiser als `sfx/buildings/build_progress_*`, damit Spieleraktionen hörbar bleiben. Hochpass 50 Hz; Mitten um 300 Hz −2 dB (Platz für Dialog); Stereo mittelbreit.

### 1.6 `ambience/ambience_ruins_loop.ogg` – Ruinen von Vael Dorn (Layer `ambience_ruins`, −6 dB)
*Charakter:* Ruinen im Nordosten mit dem Hohlschrein; Schleier ist dünn: tiefe Drones, hohler Wind in Steinbögen, fernes Knochenklappern, unterschwelliges Flüstern (nicht verständlich), Aschglas-Summen.
**Länge:** 30 s.

| Eintrag | EN-Prompt (eine Generierung) | Dauer | PI | Loop |
|---|---|---|---|---|
| Gesamtprompt | eerie ruined stone temple ambience, low dark drone, hollow wind moving through broken arches, faint glassy humming of magic crystals, distant bones clattering on stone, a faint unintelligible ghostly whisper, cold and empty, no music, no clear words, stereo | 30 s | 0,4 | an |

**Integration (nicht vom Nutzer):** Zielloop 90 s. Flüstern max. 2× und sehr leise (−14 dB), Wörter dürfen nicht verständlich sein. Glocken-Detail (falls ergänzt) tonal zur Bossmusik (B-Phrygisch) passend oder atonal. Sub-Anteil kontrollieren (kein Brummen < 30 Hz). Hochpass 28 Hz; Hohle-Signatur subtil (Reverse-Reverb auf dem Flüstern, Mix 15 %); Stereo breit, Mitte etwas offener.

---

## 2. Regionen-Ambiences für spätere Phasen (nicht im Manifest v1)

IDs sind **Vorschläge** nach Konvention `ambience/<ambience_id>_loop.ogg` (Präfix = Regions-ID aus dem Kanon). Je Region ein Tag- und
ein Nacht-Bett plus Spezial-Layer. Die Schicht-Tabellen (A Bett, B, C Detail) sind **Bausteine**: für die Produktion werden sie zu **einem** Prompt je Datei zusammengeführt
(eine komplette Generierung je Datei, Loop an, Dauer wie in Abschnitt 0); Schichtung und Verlängerung übernimmt die Integration.

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

*Integration Schleiernarbe (nicht vom Nutzer):* Hohle-Signatur deutlich (Reverse-Reverb, langsames Pitch-Wobble ±10 Cent auf Schicht A, Spektral-Freeze auf C).

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

**Nutzer (Lieferung):**
- [ ] Dateiname exakt (`ambience/<id>_loop`), eine komplette Generierung je Datei, Loop-Option an.
- [ ] Inhalt sauber getrennt (kein Regen im Tag-Loop, keine Vögel im Regen-Loop, keine Dorfgeräusche in Ruinen).
- [ ] Keine verständlichen Wörter, keine Musikfragmente.

**Integration (nicht vom Nutzer):**
- [ ] Länge im Ziel (Manifest-Loops 60–90 s), Schnitt auf Nulldurchgang, keine Fades an den Enden, stereo, 48 kHz, Vorbis q6.
- [ ] 5 Minuten Loop-Test ohne Klick/Pegelsprung/erkennbare Wiederholung.
- [ ] −23 LUFS integriert (±1), −1 dBTP; im Spiel gegen Musik und Dialog geprüft (Ambience darf Dialog nie verdecken).
