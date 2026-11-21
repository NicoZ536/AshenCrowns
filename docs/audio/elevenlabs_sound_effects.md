# ASHEN CROWNS – Soundeffekte (ElevenLabs Sound Effects)

Grundlage: `assets/audio/manifest.ron` (Events `sfx.*`), Regeln in `docs/audio/elevenlabs_production_guide.md`.

**Legende & Grundregeln**
- **Ziel-Datei** relativ zu `assets/audio/`; `.ogg` Vorbis q6, 48 kHz. **Mono** für alle positionierten Sounds (combat, magic,
  buildings, world – außer `thunder`), UI mono oder stereo, `thunder` stereo.
- **Prompt** englisch, in das Prompt-Feld des Sound-Effects-Werkzeugs. Prompts sind bewusst konkret (Quelle, Material, Aktion,
  Nähe, Raum). Zusatz „no music“ verwenden, falls der Generator melodische Elemente einstreut.
- **Dauer** manuell setzen (Sekunden). **Prompt Influence (PI)** 0–1 (höher = wörtlicher). Werte sind Startempfehlungen.
- Mindest-/Maximaldauer, verfügbare Optionen (z. B. Loop) und Modellversion zum Produktionszeitpunkt in der UI prüfen
  (derzeit ca. 0,5 s bis ca. 30 s). Sounds, die kürzer als die Mindestdauer sein müssen (UI), länger generieren und kürzen.
- **Varianten:** Je Event 3 Dateien (`_01`–`_03`). Jede Variante hat einen eigenen, leicht abgewandelten Prompt (Material/Charakter),
  damit sie hörbar verschieden, aber eindeutig derselbe Sound-Typ sind. Pro Prompt mehrere Generierungen erzeugen und die beste wählen.
- **Nachbearbeitung allgemein:** Schnitt auf Transiente (≤ 5 ms Vorlauf), DC-Offset entfernen, Hochpass passend zur Quelle,
  Limiter −1 dBTP, Lautheit nach Leitfaden Abschnitt 7; Varianten einer Familie gleich laut. Keine Hallfahnen bei positionierten
  Sounds (Engine-Raum), außer wo angegeben.
- **Generator-Artefakte** (Zirpen, „Unterwasser“-Klang, Pseudo-Musik, Stimmenfetzen) → Take verwerfen.

---

## 1. Kampf – `sfx/combat/`

### sword_hit (Event `sfx.sword_hit`) · Manifest 0 dB, max. 6 Instanzen, Cooldown 60 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/sword_hit_01.ogg` | steel longsword striking chainmail, sharp metallic clang with a short ring, close, dry, no reverb | 0,6 s | 0,6 | hell, Kettenrasseln |
| `sfx/combat/sword_hit_02.ogg` | steel sword slashing into leather armor and flesh, meaty thud with a metallic edge, close, dry | 0,6 s | 0,6 | dumpfer, fleischiger |
| `sfx/combat/sword_hit_03.ogg` | sword blade hitting plate armor, bright metallic impact with a short scrape, close, dry | 0,6 s | 0,6 | Plattenklang mit Kratzer |

**Nachbearbeitung:** Hochpass 60 Hz, Transientenbetonung +2 dB, Ausklang auf max. 0,5 s kürzen; Referenzdatei für Kampf-SFX: `sword_hit_01`.

### spear_hit (Event `sfx.spear_hit`) · 0 dB, 6 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/spear_hit_01.ogg` | spear thrust piercing leather armor, wooden shaft thud with a sharp stab, close, dry | 0,6 s | 0,6 | Holzschaft hörbar |
| `sfx/combat/spear_hit_02.ogg` | iron spearhead stabbing through chainmail into a body, puncture with metal rattle, close, dry | 0,6 s | 0,6 | Ketten + Stich |
| `sfx/combat/spear_hit_03.ogg` | spear point glancing off a shield rim then piercing, quick scrape and heavy thump, close, dry | 0,7 s | 0,55 | zweiteilig: Kratzer + Stoß |

**Nachbearbeitung:** Hochpass 60 Hz; tiefere Grundfrequenz als `sword_hit` belassen (Unterscheidbarkeit im Gefecht).

### arrow_fire (Event `sfx.arrow_fire`) · −3 dB, 6 Instanzen, Cooldown 40 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/arrow_fire_01.ogg` | longbow arrow release, taut bowstring twang and fast whoosh, close, dry | 0,7 s | 0,6 | Standard-Langbogen |
| `sfx/combat/arrow_fire_02.ogg` | wooden bow loosing an arrow, short string snap and feathered swish, close, dry | 0,6 s | 0,6 | kürzer, trockener |
| `sfx/combat/arrow_fire_03.ogg` | bow shot, deep bowstring thrum and arrow whistle quickly receding, close, dry | 0,8 s | 0,55 | tieferer Sehnenklang, Pfeifen |

**Nachbearbeitung:** Hochpass 120 Hz; Pfeil-Whoosh darf leicht ausklingen, Gesamtlänge ≤ 0,7 s.

### arrow_impact (Event `sfx.arrow_impact`) · −3 dB, 6 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/arrow_impact_01.ogg` | arrow hitting a wooden shield, sharp thunk with a short quivering vibration, close, dry | 0,5 s | 0,65 | Holz, Nachvibrieren |
| `sfx/combat/arrow_impact_02.ogg` | arrow piercing flesh through cloth, short dull thud, close, dry | 0,5 s | 0,65 | weich, dumpf |
| `sfx/combat/arrow_impact_03.ogg` | arrow striking packed earth and small stones, dull thud with a tiny rattle, close, dry | 0,5 s | 0,6 | Boden (Fehlschuss) |

**Nachbearbeitung:** Hochpass 80 Hz; sehr kurzer Ausklang.

### shield_block (Event `sfx.shield_block`) · −2 dB, 4 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/shield_block_01.ogg` | sword blocked by a wooden round shield with an iron rim, heavy wooden thud and metal clank, close, dry | 0,6 s | 0,6 | Holz + Eisenrand |
| `sfx/combat/shield_block_02.ogg` | heavy blow against a large wooden kite shield, deep hollow wooden impact, close, dry | 0,6 s | 0,6 | tiefer, hohler |
| `sfx/combat/shield_block_03.ogg` | mace striking an iron shield boss, loud metallic bang with wooden body resonance, close, dry | 0,7 s | 0,55 | metallischer |

**Nachbearbeitung:** Hochpass 50 Hz; Low-Mid-Punch 120–200 Hz erhalten.

### horse_charge (Event `sfx.horse_charge`) · 0 dB, 2 Instanzen, Cooldown 500 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/horse_charge_01.ogg` | heavy armored warhorse charging, thundering hooves on dirt approaching, tack and armor jingling, horse snort, no music | 3,0 s | 0,5 | Einzelpferd, Annäherung |
| `sfx/combat/horse_charge_02.ogg` | two armored warhorses galloping hard on packed ground, chainmail rattle, a short whinny, close | 3,0 s | 0,5 | zwei Pferde, Wiehern |
| `sfx/combat/horse_charge_03.ogg` | cavalry charge start, horse neighs then breaks into a pounding gallop, harness clinking, close | 3,0 s | 0,5 | Start mit Wiehern |

**Nachbearbeitung:** Hochpass 40 Hz, Bassbereich der Hufe betonen (+2 dB @ 90 Hz); Ende ausfaden (0,4 s).

### bone_crunch (Event `sfx.bone_crunch`) · −1 dB, 4 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/bone_crunch_01.ogg` | dry bones crunching and snapping under a heavy blow, brittle crack, close, dry | 0,6 s | 0,6 | spröde |
| `sfx/combat/bone_crunch_02.ogg` | skeleton struck by an iron mace, bones shattering and scattering, close, dry | 0,7 s | 0,55 | Splittern + Streuen |
| `sfx/combat/bone_crunch_03.ogg` | skull cracking with dry bone splinters, short and crisp, close, dry | 0,5 s | 0,6 | kurz, hoch |

**Nachbearbeitung:** Hochpass 100 Hz; Höhen-Crispness erhalten; Einsatz bei Treffern auf Hohle.

### unit_death_human (Event `sfx.unit_death_human`) · −1 dB, 3 Instanzen
*Hinweis:* **ohne Stimme** – die Todesstimme kommt aus den `voice.*.death`-Events. Nur Körper/Ausrüstung.

| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/unit_death_human_01.ogg` | armored soldier collapsing to the ground, chainmail rattle, body thud on dirt, sword clattering, no voice | 1,5 s | 0,55 | Kettenhemd + Schwert |
| `sfx/combat/unit_death_human_02.ogg` | body falling onto muddy ground, heavy thud and cloth rustle, wooden spear dropping, no voice | 1,4 s | 0,55 | weich, Schlamm |
| `sfx/combat/unit_death_human_03.ogg` | knight in plate armor falling, heavy metallic crash on stone, no voice | 1,6 s | 0,55 | schwer, metallisch |

**Nachbearbeitung:** Stimmenartefakte entfernen/verwerfen; Hochpass 50 Hz.

### unit_death_hollow (Event `sfx.unit_death_hollow`) · −1 dB, 3 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/unit_death_hollow_01.ogg` | undead corpse collapsing into dust, dry bones clattering, ashy hiss of escaping air | 1,8 s | 0,5 | Staub + Zischen |
| `sfx/combat/unit_death_hollow_02.ogg` | skeleton falling apart, bones rattling onto stone, faint ghostly whisper dissipating | 1,8 s | 0,45 | Geisterhauch |
| `sfx/combat/unit_death_hollow_03.ogg` | rotten undead body crumbling, dry crunch, puff of ash, eerie breath fading out | 1,8 s | 0,5 | Aschewolke |

**Nachbearbeitung:** Hochpass 80 Hz; Geisterhauch-Anteil ggf. mit leichtem Reverse-Reverb (Mix 10 %) – Hohle-Signatur.

### building_hit (Event `sfx.building_hit`) · −2 dB, 3 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/combat/building_hit_01.ogg` | heavy blow against a wooden palisade, deep timber thud with splintering, outdoors | 1,0 s | 0,55 | Holzpalisade |
| `sfx/combat/building_hit_02.ogg` | stone wall struck by a heavy club, dull thump with falling grit and pebbles | 1,0 s | 0,55 | Stein, Geröll |
| `sfx/combat/building_hit_03.ogg` | wooden door hit hard, sharp crack and creak, dust trickling | 1,0 s | 0,55 | Tür, Knarzen |

**Nachbearbeitung:** Hochpass 40 Hz; Tiefton-Gewicht betonen.

---

## 2. Magie – `sfx/magic/`

### ember_cleave (Event `sfx.ember_cleave`) – Isoldes Glutspalter (Flächenschaden) · 0 dB, 2 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/magic/ember_cleave_01.ogg` | fiery sword swing, whoosh of flame trailing the blade, burst of embers exploding on impact, crackling sparks | 1,5 s | 0,5 | ausgewogen |
| `sfx/magic/ember_cleave_02.ogg` | heavy flaming blade cleave, roaring arc of fire, deep ground impact and spray of embers | 1,6 s | 0,5 | tiefer, wuchtiger |
| `sfx/magic/ember_cleave_03.ogg` | magical ember slash, sharp fire whoosh, crackling explosion of glowing sparks fading out | 1,4 s | 0,5 | heller, funkelnder |

**Nachbearbeitung:** Hochpass 40 Hz; Impact bei ca. 0,3–0,4 s ausrichten (Animations-Sync, mit Gameplay abstimmen); Ausklang 1 s.

### rallying_banner (Event `sfx.rallying_banner`) – Sammelbanner · 0 dB, 1 Instanz
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/magic/rallying_banner_01.ogg` | war banner planted into the ground with a heavy thud, cloth snapping in the wind, warm magical shimmer rising, distant war horn | 2,5 s | 0,5 | mit fernem Horn |
| `sfx/magic/rallying_banner_02.ogg` | heavy flag unfurling with a loud flap, bright metallic chime and rising warm choir-like glow, no words | 2,5 s | 0,45 | Chorschimmer |
| `sfx/magic/rallying_banner_03.ogg` | banner pole slammed into earth, fabric whipping, soft fire whoosh and bright magical swell | 2,5 s | 0,5 | Feuer-Akzent |

**Nachbearbeitung:** Hochpass 50 Hz; aufsteigende Energie (Swell) nicht limitieren; Stimmen-/Wortfragmente im Chorschimmer verwerfen.

### ashen_ward (Event `sfx.ashen_ward`) – Aschenschild · 0 dB, 2 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/magic/ashen_ward_01.ogg` | magical shield forming, swirling ash and wind, low resonant hum, crystalline shimmer | 2,0 s | 0,5 | Kristallschimmer |
| `sfx/magic/ashen_ward_02.ogg` | protective barrier of ash and embers, whooshing swirl and deep warm drone | 2,0 s | 0,5 | wärmer, tiefer |
| `sfx/magic/ashen_ward_03.ogg` | grey magic shield activating, soft crackle of embers and glassy resonance settling | 2,0 s | 0,5 | glasig, ruhiger Ausklang |

**Nachbearbeitung:** Hochpass 40 Hz; Ende sauber ausfaden (Schild-Dauerton kommt ggf. später als eigener Loop).

### hollow_summon (Event `sfx.hollow_summon`) – Beschwörung der Hohlen · −1 dB, 2 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/magic/hollow_summon_01.ogg` | undead summoning, earth cracking open, bones scraping out of the soil, eerie whispering wind | 3,0 s | 0,5 | Erde + Knochen |
| `sfx/magic/hollow_summon_02.ogg` | necromantic ritual, low reversed drone, ghostly moans rising, dirt and gravel shifting | 3,0 s | 0,45 | Drone + Stöhnen |
| `sfx/magic/hollow_summon_03.ogg` | dark magic summon, deep sub boom, hissing ash, skeletal hands clawing out of the ground | 3,0 s | 0,5 | Sub-Boom |

**Nachbearbeitung:** Hohle-Signatur (leichter Reverse-Reverb-Vorlauf 150 ms, Notch 1–2 kHz −3 dB); Hochpass 30 Hz.

### reeve_soul_drain (Event `sfx.reeve_soul_drain`) – Seelenentzug des Fahlen Vogts · 0 dB, 2 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/magic/reeve_soul_drain_01.ogg` | soul being drained, ghostly wailing pulled away, deep vacuum suction and cold wind | 2,5 s | 0,5 | Sog + Klagen |
| `sfx/magic/reeve_soul_drain_02.ogg` | life force siphon, reversed whispers, rising eerie hiss, a single faint funeral bell toll | 2,5 s | 0,45 | Glocke (Vogt-Motiv) |
| `sfx/magic/reeve_soul_drain_03.ogg` | spectral energy drain, breathy howl sucked inward, icy crackle, low drone | 2,5 s | 0,5 | eisig |

**Nachbearbeitung:** Hohle-Signatur; Glocken-Variante mit Bossmusik-Tonart (B) abgleichen, falls hörbar verstimmt → Pitch korrigieren.

---

## 3. Gebäude – `sfx/buildings/`

### build_place (Event `sfx.build_place`) · −2 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/buildings/build_place_01.ogg` | wooden building foundation placed on the ground, heavy timber thud and dirt crunch | 0,8 s | 0,6 | Holz |
| `sfx/buildings/build_place_02.ogg` | large stone block set down on earth, solid thump with gravel | 0,8 s | 0,6 | Stein |
| `sfx/buildings/build_place_03.ogg` | bundle of wooden beams dropped onto a pile, hollow knock and rustle | 0,8 s | 0,6 | Balkenbündel |

**Nachbearbeitung:** Hochpass 50 Hz; kurz und satt (UI-nahe Rückmeldung).

### build_progress (Event `sfx.build_progress`) · −6 dB, 3 Instanzen, Cooldown 300 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/buildings/build_progress_01.ogg` | hammer driving iron nails into wood, three quick knocks, medieval carpentry, outdoors | 1,5 s | 0,6 | Hämmern |
| `sfx/buildings/build_progress_02.ogg` | hand saw cutting a wooden plank, two short strokes, outdoors | 1,5 s | 0,6 | Sägen |
| `sfx/buildings/build_progress_03.ogg` | chisel tapping stone, rhythmic clinks, mason at work, outdoors | 1,5 s | 0,6 | Steinmetz |

**Nachbearbeitung:** Hochpass 100 Hz; leise und unaufdringlich, wiederholt sich oft → keine markanten Ausreißer.

### build_complete (Event `sfx.build_complete`) · 0 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/buildings/build_complete_01.ogg` | medieval construction finished, final hammer blow, wooden creak settling, small bronze bell chime | 1,5 s | 0,55 | Glöckchen |
| `sfx/buildings/build_complete_02.ogg` | timber building completed, beams settling with a satisfying thud, short warm horn note | 1,5 s | 0,5 | Hornton |
| `sfx/buildings/build_complete_03.ogg` | stone structure completed, last block thud, dust settling, bright clear chime | 1,5 s | 0,55 | Steinvariante |

**Nachbearbeitung:** Hochpass 50 Hz; Glocken-/Hornton tonal mit Musik (D-Moll/D-Dur) verträglich halten.

### building_destroyed (Event `sfx.building_destroyed`) · 0 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/buildings/building_destroyed_01.ogg` | wooden building collapsing, timbers cracking and crashing down, fire roaring, debris falling | 3,5 s | 0,5 | Holz + Feuer |
| `sfx/buildings/building_destroyed_02.ogg` | stone tower crumbling, heavy rocks tumbling, rubble and dust | 3,5 s | 0,5 | Stein |
| `sfx/buildings/building_destroyed_03.ogg` | burning house caving in, splintering beams, flames whooshing, embers crackling | 3,5 s | 0,5 | brennendes Haus |

**Nachbearbeitung:** Hochpass 30 Hz; Tiefbass erhalten; langer natürlicher Ausklang.

### train_complete (Event `sfx.train_complete`) · −2 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/buildings/train_complete_01.ogg` | soldier ready for duty, armor clank and sword drawn from scabbard, single drum hit | 1,2 s | 0,55 | Schwert + Trommel |
| `sfx/buildings/train_complete_02.ogg` | boots stepping to attention on wood, chainmail rattle, metallic ring | 1,2 s | 0,55 | Antreten |
| `sfx/buildings/train_complete_03.ogg` | barracks door opening, boots on wooden floor, short horn blast | 1,2 s | 0,5 | Tür + Horn |

**Nachbearbeitung:** Hochpass 60 Hz; deutlich, aber kurz.

---

## 4. Benutzeroberfläche – `sfx/ui/`

UI-Sounds: Material „Holz, Pergament, Bronze, Glut“ – passend zur Dark-Fantasy-Oberfläche. Peak ≤ −6 dBFS, sehr kurze Fades.

### ui_click (Event `sfx.ui_click`) · −6 dB, Cooldown 30 ms
| Ziel-Datei | EN-Prompt | Dauer (gen. → final) | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/ui_click_01.ogg` | soft wooden UI click, small knock on oak, crisp and very short, no reverb | 0,5 → 0,10 s | 0,75 | Holz |
| `sfx/ui/ui_click_02.ogg` | subtle metallic button click with a faint parchment rustle, very short, no reverb | 0,5 → 0,12 s | 0,75 | Metall + Papier |
| `sfx/ui/ui_click_03.ogg` | short muted leather and wood tick, tactile, very short, no reverb | 0,5 → 0,08 s | 0,75 | gedämpft |

**Nachbearbeitung:** auf Transiente schneiden, Fade-out 10–20 ms, Hochpass 200 Hz.

### ui_hover (Event `sfx.ui_hover`) · −12 dB
| Ziel-Datei | EN-Prompt | Dauer (gen. → final) | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/ui_hover_01.ogg` | very soft parchment swish, airy and delicate, extremely short | 0,5 → 0,10 s | 0,7 | Pergament |
| `sfx/ui/ui_hover_02.ogg` | tiny quiet ember crackle tick, single spark, extremely short | 0,5 → 0,08 s | 0,7 | Glutfunke |
| `sfx/ui/ui_hover_03.ogg` | light cloth brush, extremely short soft whoosh | 0,5 → 0,10 s | 0,7 | Stoff |

**Nachbearbeitung:** Hochpass 300 Hz, Tiefpass 10 kHz (angenehm, nicht zischelnd); sehr leise.

### ui_error (Event `sfx.ui_error`) · −4 dB, Cooldown 200 ms
| Ziel-Datei | EN-Prompt | Dauer (gen. → final) | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/ui_error_01.ogg` | dull low wooden thunk, muted negative feedback sound, short | 0,6 → 0,3 s | 0,7 | Holz dumpf |
| `sfx/ui/ui_error_02.ogg` | short low dissonant metallic clank, dry, short | 0,6 → 0,3 s | 0,7 | Metall dissonant |
| `sfx/ui/ui_error_03.ogg` | soft double knock on wood, low pitch, denied feeling, short | 0,6 → 0,35 s | 0,7 | Doppelklopfen |

**Nachbearbeitung:** Hochpass 120 Hz; klar „nein“, aber nicht nervig.

### ui_notify (Event `sfx.ui_notify`) · −3 dB, Cooldown 200 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/ui_notify_01.ogg` | small bronze hand bell ringing once, clear and short, medieval | 1,0 s | 0,65 | Bronzeglocke |
| `sfx/ui/ui_notify_02.ogg` | soft warm metallic chime with gentle decay | 1,0 s | 0,65 | weicher Gong |
| `sfx/ui/ui_notify_03.ogg` | light bell, two quick strikes, gentle | 1,0 s | 0,65 | Doppelschlag |

**Nachbearbeitung:** Hochpass 200 Hz; Ausklang ≤ 0,8 s; tonal unauffällig (kein Konflikt mit Musik).

### objective_new (Event `sfx.objective_new`) · −1 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/objective_new_01.ogg` | parchment scroll unrolling with a quill scratch and a single low drum hit | 2,0 s | 0,55 | Pergament + Trommel |
| `sfx/ui/objective_new_02.ogg` | deep war drum hit followed by one resonant low horn note, no melody | 2,0 s | 0,55 | Trommel + Horn |
| `sfx/ui/objective_new_03.ogg` | red wax seal breaking, paper rustle, soft bell tone | 2,0 s | 0,55 | Siegel |

**Nachbearbeitung:** Hochpass 50 Hz; Stereo erlaubt (nicht positioniert).

### objective_complete (Event `sfx.objective_complete`) · 0 dB
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/ui/objective_complete_01.ogg` | short triumphant medieval brass flourish with a bell chime, two seconds | 2,0 s | 0,5 | Fanfare |
| `sfx/ui/objective_complete_02.ogg` | warm wordless choir swell with a bright chime, uplifting, no lyrics | 2,0 s | 0,45 | Chor |
| `sfx/ui/objective_complete_03.ogg` | quill stroke and seal stamp followed by an uplifting bell ring | 2,0 s | 0,5 | Schreiben + Glocke |

**Nachbearbeitung:** Tonart möglichst D-Dur (kompatibel mit `victory_sting`); bei tonalen Konflikten Pitch anpassen. Alternative: mit
Eleven Music als 2-s-Sting erzeugen.

---

## 5. Welt – `sfx/world/`

### wood_chop (Event `sfx.wood_chop`) · −6 dB, 4 Instanzen, Cooldown 150 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/wood_chop_01.ogg` | axe chopping into a tree trunk, sharp wooden thock, outdoors, single hit | 0,8 s | 0,6 | Stamm |
| `sfx/world/wood_chop_02.ogg` | woodcutter's axe striking a log, wood splintering, single hit | 0,8 s | 0,6 | Splittern |
| `sfx/world/wood_chop_03.ogg` | axe hitting hard oak, dull thunk with a small crack, single hit | 0,8 s | 0,6 | dumpf |

**Nachbearbeitung:** Hochpass 80 Hz; Einzelschlag sicherstellen (Mehrfachschläge schneiden).

### mine_pick (Event `sfx.mine_pick`) · −6 dB, 4 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/mine_pick_01.ogg` | iron pickaxe striking rock, metallic clink and stone chips falling, single hit | 0,7 s | 0,6 | Standard |
| `sfx/world/mine_pick_02.ogg` | pickaxe hitting a gold ore vein, bright ringing clink, single hit | 0,7 s | 0,6 | heller (Gold) |
| `sfx/world/mine_pick_03.ogg` | pickaxe impact on hard rock with a gritty scrape, single hit | 0,7 s | 0,6 | kratzig |

**Nachbearbeitung:** Hochpass 100 Hz.

### stone_quarry (Event `sfx.stone_quarry`) · −6 dB, 4 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/stone_quarry_01.ogg` | hammer and chisel splitting a stone block, sharp crack | 1,0 s | 0,6 | Meißel |
| `sfx/world/stone_quarry_02.ogg` | large stone pried loose with an iron bar, grinding scrape and heavy thud | 1,0 s | 0,55 | Hebeln |
| `sfx/world/stone_quarry_03.ogg` | sledgehammer hitting a boulder, deep crack and falling gravel | 1,0 s | 0,6 | Vorschlaghammer |

**Nachbearbeitung:** Hochpass 60 Hz; unterscheidbar von `mine_pick` (tiefer, schwerer).

### harvest_grain (Event `sfx.harvest_grain`) · −8 dB, 2 Instanzen
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/harvest_grain_01.ogg` | sickle cutting wheat stalks, crisp swish and dry rustle | 1,2 s | 0,6 | Sichel |
| `sfx/world/harvest_grain_02.ogg` | grain bundle gathered and tied, dry straw rustle | 1,2 s | 0,6 | Garbe binden |
| `sfx/world/harvest_grain_03.ogg` | scythe sweeping through dry grain, soft long swoosh | 1,2 s | 0,6 | Sense |

**Nachbearbeitung:** Hochpass 150 Hz; weich, leise.

### horn_alarm (Event `sfx.horn_alarm`) · 0 dB, 1 Instanz, Cooldown 3000 ms
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/horn_alarm_01.ogg` | medieval war horn alarm, one long low blast echoing over a village valley | 3,5 s | 0,6 | ein langer Stoß |
| `sfx/world/horn_alarm_02.ogg` | two urgent blasts of a large animal horn, distant echo | 3,5 s | 0,6 | zwei Stöße |
| `sfx/world/horn_alarm_03.ogg` | rising alarm horn call from a watchtower, reverberating over hills | 3,5 s | 0,55 | ansteigend |

**Nachbearbeitung:** Natürlicher Außenhall erlaubt (Teil des Sounds); mono; Hochpass 60 Hz. Begleitet `char_hal_venn_m01_wave_warning`.

### thunder (Event `sfx.thunder`) · −2 dB, Cooldown 2000 ms – **stereo**
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/thunder_01.ogg` | distant rolling thunder, deep rumble over hills, no rain | 6,0 s | 0,5 | fern |
| `sfx/world/thunder_02.ogg` | close thunderclap, sharp crack followed by a long rolling rumble, no rain | 6,0 s | 0,5 | nah, Knall |
| `sfx/world/thunder_03.ogg` | low ominous thunder, slow rolling, very far away, no rain | 6,0 s | 0,45 | bedrohlich, fern |

**Nachbearbeitung:** Hochpass 25 Hz; Stereo-Breite erhalten; Regen kommt aus `ambience_rain`, daher „no rain“.

### fire_crackle (Event `sfx.fire_crackle`) · −8 dB, 3 Instanzen – **Loop-fähig**
| Ziel-Datei | EN-Prompt | Dauer | PI | Variationshinweis |
|---|---|---|---|---|
| `sfx/world/fire_crackle_01.ogg` | small campfire crackling, popping wood, close, steady | 5,0 s (Loop an) | 0,5 | Lagerfeuer |
| `sfx/world/fire_crackle_02.ogg` | burning torch flame, steady flutter and soft crackle | 5,0 s (Loop an) | 0,5 | Fackel |
| `sfx/world/fire_crackle_03.ogg` | burning building fire, crackling embers and a low roar | 5,0 s (Loop an) | 0,45 | Gebäudebrand |

**Nachbearbeitung:** Loop-Option nutzen; Nahtstelle prüfen (Crossfade 50–100 ms, falls nötig); keine markanten Einzel-Knacker nahe
der Nahtstelle; Hochpass 60 Hz.

---

## 6. Erweiterter Katalog für spätere Phasen (nicht im Manifest v1)

IDs sind **Vorschläge** (Kanon-Freigabe nötig); Kategorien gemäß Konvention (`combat`, `magic`, `buildings`, `ui`, `units`, `world`).
Je Eintrag 3 Varianten einplanen (`_01`–`_03`), Variation durch Material/Intensität wie oben.

### 6.1 Belagerung (`sfx/combat/`)
| Vorgeschlagene Datei(en) | EN-Prompt | Dauer | PI |
|---|---|---|---|
| `sfx/combat/siege_ram_hit_01…03.ogg` | iron-capped battering ram slamming a wooden castle gate, massive boom, splintering, chains rattling | 2,0 s | 0,55 |
| `sfx/combat/catapult_launch_01…03.ogg` | wooden catapult release, creaking torsion ropes, heavy arm thump, stone whooshing away | 2,5 s | 0,55 |
| `sfx/combat/trebuchet_launch_01…03.ogg` | huge trebuchet firing, counterweight crash, long creak and sling whip, projectile whoosh | 3,5 s | 0,5 |
| `sfx/combat/boulder_impact_01…03.ogg` | heavy boulder smashing into a stone wall, deep crash, rubble falling, dust | 2,5 s | 0,55 |
| `sfx/combat/gate_break_01…03.ogg` | castle gate breaking open, huge wooden crack, iron hinges snapping, debris | 3,0 s | 0,5 |
| `sfx/combat/ballista_fire_01…03.ogg` | ballista bolt fired, heavy string snap and deep whoosh | 1,5 s | 0,6 |
| `sfx/combat/ladder_place_01…03.ogg` | wooden siege ladder slammed against a stone wall, clatter and creak | 1,2 s | 0,6 |
| `sfx/combat/boiling_oil_01…03.ogg` | cauldron of boiling oil poured from battlements, splash and violent sizzle | 2,5 s | 0,5 |
| `sfx/combat/wall_collapse_01…03.ogg` | section of castle wall collapsing, thunderous stone avalanche, long rumble | 5,0 s | 0,5 |

### 6.2 Kavallerie & Einheitenbewegung (`sfx/units/`)
| Vorgeschlagene Datei(en) | EN-Prompt | Dauer | PI |
|---|---|---|---|
| `sfx/units/horse_gallop_loop_01…03.ogg` | single horse galloping steadily on dirt road, rhythmic hooves, tack jingling, loopable | 6,0 s (Loop) | 0,55 |
| `sfx/units/horse_trot_01…03.ogg` | horse trotting on cobblestones, steady clip-clop | 3,0 s | 0,55 |
| `sfx/units/horse_whinny_01…03.ogg` | warhorse whinny, agitated, close | 1,5 s | 0,6 |
| `sfx/units/horse_pain_01…03.ogg` | wounded horse screaming briefly, close, no gore | 1,5 s | 0,55 |
| `sfx/units/horse_halt_01…03.ogg` | armored horse skidding to a halt, snort, harness rattle | 1,5 s | 0,55 |
| `sfx/units/footsteps_armor_01…03.ogg` | soldier walking in chainmail on dirt, four steps, rattle | 2,0 s | 0,6 |
| `sfx/units/footsteps_hollow_01…03.ogg` | undead shuffling footsteps, dragging feet, bones clicking | 2,0 s | 0,55 |
| `sfx/units/grave_brute_stomp_01…03.ogg` | massive undead giant stomp, ground-shaking thud, loose bones rattling | 1,2 s | 0,55 |

### 6.3 Magieschulen (`sfx/magic/`)
| Schule / Fraktion | Vorgeschlagene Datei(en) | EN-Prompt | Dauer | PI |
|---|---|---|---|---|
| Ewige Flamme (Glutkrone) | `sfx/magic/flame_blessing_01…03.ogg` | holy fire blessing, warm soft whoosh, gentle bell shimmer, crackling embers | 2,5 s | 0,5 |
| Ewige Flamme | `sfx/magic/flame_heal_01…03.ogg` | healing warmth spell, soft rising tone, quiet fire breath, sparkle | 2,0 s | 0,5 |
| Ahnenpfad (Skarn) | `sfx/magic/ancestor_frost_01…03.ogg` | frost magic, ice crystals forming and cracking, cold wind howl, deep drum | 2,5 s | 0,5 |
| Ahnenpfad | `sfx/magic/ancestor_call_01…03.ogg` | spectral ancestors called, deep throat-singing-like drone, wind, distant drums, no words | 3,0 s | 0,45 |
| Grüner Kreis (Dornenbund) | `sfx/magic/thorn_bind_01…03.ogg` | thorny vines bursting from the ground, creaking wood, rustling leaves, tightening snap | 2,0 s | 0,5 |
| Grüner Kreis | `sfx/magic/root_heal_01…03.ogg` | nature healing spell, soft leaf rustle, gentle wooden creak, airy shimmer | 2,0 s | 0,5 |
| Haus der Gezeiten (Saelith) | `sfx/magic/tide_wave_01…03.ogg` | magical water wave crashing, surging splash, deep whoosh | 2,5 s | 0,5 |
| Haus der Gezeiten | `sfx/magic/tide_mist_01…03.ogg` | sea mist spell, hissing spray, soft rolling waves, eerie calm | 3,0 s | 0,45 |
| Leere Liturgie (Hohle) | `sfx/magic/void_curse_01…03.ogg` | dark void curse, reversed whisper, deep sub drop, ash hiss | 2,0 s | 0,5 |
| Asche/Glut (Aschenlegion) | `sfx/magic/legion_ash_burst_01…03.ogg` | burst of black ash and orange embers, heavy whoosh, metallic clang | 2,0 s | 0,5 |
| Aschglas | `sfx/magic/ashglass_resonance_01…03.ogg` | glowing magic crystal resonating, glassy hum, faint whisper-like overtones | 3,0 s | 0,45 |
| Aschglas | `sfx/magic/ashglass_shatter_01…03.ogg` | magic crystal shattering, bright glass burst with dark echo | 1,5 s | 0,55 |
| Schleier | `sfx/magic/veil_tear_01…03.ogg` | reality tearing open, deep wrenching rumble, distorted wind, ghostly choir drone | 4,0 s | 0,45 |

### 6.4 Wetter (`sfx/world/`)
| Vorgeschlagene Datei(en) | EN-Prompt | Dauer | PI |
|---|---|---|---|
| `sfx/world/wind_gust_01…03.ogg` | strong wind gust sweeping through trees and over open fields | 4,0 s | 0,5 |
| `sfx/world/lightning_strike_01…03.ogg` | lightning strike very close, violent electric crack and immediate thunder boom | 3,0 s | 0,5 |
| `sfx/world/hail_burst_01…03.ogg` | short burst of hail on wooden roofs and stone | 4,0 s | 0,5 |
| `sfx/world/ashfall_gust_01…03.ogg` | gust carrying fine ash, dry hissing whisper, soft grit on stone | 4,0 s | 0,45 |
| `sfx/world/snow_gust_01…03.ogg` | icy mountain blizzard gust, howling, snow hiss | 4,0 s | 0,5 |
| `sfx/world/rain_burst_01…03.ogg` | sudden heavy rain burst hitting thatch and mud | 4,0 s | 0,5 |

### 6.5 Gebäudeanimationen (`sfx/buildings/`)
| Vorgeschlagene Datei(en) | EN-Prompt | Dauer | PI |
|---|---|---|---|
| `sfx/buildings/mill_loop_01…03.ogg` | wooden windmill turning, creaking sails, grinding millstone, loopable | 8,0 s (Loop) | 0,5 |
| `sfx/buildings/smithy_hammer_01…03.ogg` | blacksmith hammering on anvil, three ringing strikes, hiss of quenching | 3,0 s | 0,55 |
| `sfx/buildings/keep_flag_loop_01…03.ogg` | large cloth flag flapping in wind on a tower, loopable | 6,0 s (Loop) | 0,5 |
| `sfx/buildings/gate_open_01…03.ogg` | heavy wooden gate opening, chains and winch cranking, creak | 3,5 s | 0,55 |
| `sfx/buildings/gate_close_01…03.ogg` | heavy wooden gate slamming shut, iron bar dropped into place | 2,5 s | 0,55 |
| `sfx/buildings/chapel_bell_01…03.ogg` | small village chapel bell tolling three times, distant | 5,0 s | 0,55 |
| `sfx/buildings/farm_animals_loop_01…03.ogg` | farmyard with chickens, a goat, distant cow, loopable, no people | 8,0 s (Loop) | 0,45 |
| `sfx/buildings/watchtower_torch_loop_01…03.ogg` | torch burning in wind on a wooden tower, flutter and crackle, loopable | 6,0 s (Loop) | 0,5 |
| `sfx/buildings/bone_spire_pulse_01…03.ogg` | eerie bone tower pulsing with dark energy, low hum, bone rattle, whisper | 3,0 s | 0,45 |
| `sfx/buildings/hollow_shrine_bell_01…03.ogg` | cracked funeral bell tolling once inside ruins, dissonant, long decay | 5,0 s | 0,55 |
| `sfx/buildings/upgrade_complete_01…03.ogg` | building upgraded, sturdy hammer blows, metal fittings clank, short horn | 2,0 s | 0,5 |
