# ASHEN CROWNS – Erzähler- und Dialogzeilen (ElevenLabs)

Grundlage: `assets/audio/manifest.ron` (Events `narrator.*`, `char_hal_venn.*`), Stimmprofile in
`docs/audio/elevenlabs_characters.md`, Regeln in `docs/audio/elevenlabs_production_guide.md`.

**Legende**
- **Ziel-Datei** relativ zu `assets/audio/`, `.ogg` Vorbis q6, 48 kHz, mono, −16 LUFS integriert, −1 dBTP.
- **EN-Text** = Generierungstext; eckige Klammern = Eleven-v3-Audio-Tags (nur bei v3-Zeilen; Erzähler mit Multilingual v2 **ohne** Tags).
- **Untertitel** = EN-Text ohne Tags. **DE** = Bedeutungsübersetzung für spätere Lokalisierung.
- **Optionsmarker** wie **[Option A – …]** in Teil 2 sind Regie: vor der Generierung **entfernen** (sonst interpretiert v3 sie als Tag oder spricht sie).
- **Abspielmodus** je Event:
  - **Sequenz** – die Dateien `_01`, `_02`, … sind aufeinanderfolgende Segmente einer Szene und müssen in Reihenfolge abgespielt werden.
  - **Zufallsvariante** – jede Datei ist eine eigenständige Alternative (Event kann mehrfach auftreten).
  > **Hinweis an das Engine-Team:** Das Manifest beschreibt `_*` allgemein als „Varianten“. Für Events im Modus *Sequenz*
  > muss die Engine die Dateien in Nummernreihenfolge abspielen (nicht zufällig). Jedes Segment ist zusätzlich so geschrieben,
  > dass es grammatisch vollständig ist, damit eine fehlerhafte Zufallswiedergabe nicht sinnentstellend wirkt.
- Segmentpausen: zwischen Sequenzsegmenten plant die Engine 0,4–0,8 s Pause; Dateien selbst enden mit max. 300 ms Stille.

---

# Teil 1 – Mission 1 „Die letzte Siedlung“ (`m01_last_settlement`, Zeitalter I)

**Szenario-Kontext:** Jahr 897 n.F., Spätwinter. Isolde Raventhorn erreicht Brennhollow in der Aschmark (ca. 30 Einwohner).
Der Bergfried steht noch; Bruder Hal Venn hält die Gemeinde zusammen. Nordöstlich liegen die Ruinen mit dem Hohlschrein von
Vael Dorn, bewacht vom Fahlen Vogt. Der Spieler baut die Siedlung auf (Gehöft, Kaserne, Wachturm), übersteht Nächte und
Angriffswellen der Hohlen, findet den Schrein und zerstört ihn.

## 1.1 Erzähler (`narrator`)

**Stimme:** `AC_narrator` · **Modell:** Multilingual v2 (Stability 0,60, Similarity 0,80, Style 0,15, Speaker Boost an,
Speed 0,95) · **Nachbearbeitung:** Standardkette, keine Effekte · Manifest: +2 dB.
**Regie allgemein:** getragen, ehrfürchtig, leise Trauer; nie Trailer-Pathos. Ein Segment = eine Generierung (in Studio als Absätze).

### narrator_m01_opening (Event `narrator.m01_opening`) – Abspielmodus: **Sequenz**
*Kontext:* Missionsstart, Kamera fährt über verbrannte Felder der Aschmark auf Brennhollow zu. Musik `main_menu`→`build_calm`.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/narrator/narrator_m01_opening_01.ogg` | Eight hundred and ninety-seven years after the fall of Aureth, the world still remembers the day the sun went dark. They call it the Ashfall. And they call what followed the Age of Ash. | Achthundertsiebenundneunzig Jahre nach dem Fall Aureths erinnert sich die Welt noch an den Tag, an dem die Sonne erlosch. Man nennt ihn den Aschenfall. Und was folgte, nennt man das Zeitalter der Asche. | ruhig, gewichtig; Pause nach „went dark“ |
| `voices/narrator/narrator_m01_opening_02.ogg` | In the south of Valemark lies the Ashmarch, a borderland of burned farms and silent bells. Beyond it, the ruins whisper, and the dead do not stay buried. | Im Süden Valemarks liegt die Aschmark, ein Grenzland verbrannter Höfe und stummer Glocken. Dahinter flüstern die Ruinen, und die Toten bleiben nicht begraben. | düster werdend, letzte Phrase leiser |
| `voices/narrator/narrator_m01_opening_03.ogg` | Of all the villages that once lined the southern road, only one still keeps its fires burning. Thirty souls. One chapel without a roof. Its name is Brennhollow. | Von allen Dörfern, die einst die Südstraße säumten, hält nur noch eines seine Feuer am Brennen. Dreißig Seelen. Eine Kapelle ohne Dach. Sein Name ist Brennhollow. | Wärme, Hoffnungsschimmer bei „Brennhollow“ |
| `voices/narrator/narrator_m01_opening_04.ogg` | And to Brennhollow, at the end of a long and bitter winter, came Isolde Raventhorn, Warden of the Ashmarch. Here, in the last settlement, our chronicle begins. | Und nach Brennhollow kam, am Ende eines langen, bitteren Winters, Isolde Raventhorn, Wächterin der Aschmark. Hier, in der letzten Siedlung, beginnt unsere Chronik. | feierlich, Abschluss, langsamer Schlusssatz |

### narrator_m01_night_falls (Event `narrator.m01_night_falls`) – Abspielmodus: **Zufallsvariante**
*Kontext:* Tag-Nacht-Wechsel; Ambience wechselt `ambience_day` → `ambience_night`. Kann in der Mission mehrfach auftreten.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/narrator/narrator_m01_night_falls_01.ogg` | Night falls over the Ashmarch. Bar the doors and keep the fires lit, for the dead walk in darkness. | Die Nacht senkt sich über die Aschmark. Verriegelt die Türen und haltet die Feuer am Brennen, denn die Toten wandeln im Dunkeln. | warnend, gedämpft |
| `voices/narrator/narrator_m01_night_falls_02.ogg` | The sun sinks behind the ruins. In Brennhollow, the watchmen count the torches, and pray the count still holds at dawn. | Die Sonne sinkt hinter die Ruinen. In Brennhollow zählen die Wächter die Fackeln und beten, dass die Zahl bei Morgengrauen noch stimmt. | still, Anspannung |
| `voices/narrator/narrator_m01_night_falls_03.ogg` | Darkness comes, and with it the cold breath of the Hollow Deep. Whatever sleeps in the ruins is stirring now. | Die Dunkelheit kommt und mit ihr der kalte Atem des Hohlen Tiefs. Was immer in den Ruinen schläft, regt sich nun. | unheilvoll, langsam |

### narrator_m01_victory (Event `narrator.m01_victory`) – Abspielmodus: **Sequenz**
*Kontext:* Hohlschrein zerstört, Fahler Vogt besiegt. Musik `victory_sting`, danach ruhige Ambience. Folgt auf Hal Venns `m01_victory`.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/narrator/narrator_m01_victory_01.ogg` | The Hollow Shrine of Vael Dorn fell silent, and the Pale Reeve's bell rang no more. | Der Hohlschrein von Vael Dorn verstummte, und die Glocke des Fahlen Vogts läutete nicht mehr. | Erleichterung, ruhig |
| `voices/narrator/narrator_m01_victory_02.ogg` | In Brennhollow, thirty souls became forty, and then sixty, as word spread that a fire still burned in the Ashmarch. | In Brennhollow wurden aus dreißig Seelen vierzig, dann sechzig, als sich herumsprach, dass in der Aschmark noch ein Feuer brannte. | wachsende Wärme |
| `voices/narrator/narrator_m01_victory_03.ogg` | It was a small victory, in a small corner of a broken world. But every crown in the history of Eldraeth began with a single settlement that refused to die. | Es war ein kleiner Sieg, in einem kleinen Winkel einer zerbrochenen Welt. Doch jede Krone in der Geschichte Eldraeths begann mit einer einzigen Siedlung, die sich weigerte zu sterben. | Ausblick, würdevoll, Hoffnung |

### narrator_m01_defeat (Event `narrator.m01_defeat`) – Abspielmodus: **Sequenz**
*Kontext:* Bergfried zerstört oder Isolde gefallen. Musik `defeat_sting`.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/narrator/narrator_m01_defeat_01.ogg` | The fires of Brennhollow went out, one by one, and the dead walked in through open doors. | Die Feuer Brennhollows erloschen, eines nach dem anderen, und die Toten traten durch offene Türen ein. | schwer, langsam |
| `voices/narrator/narrator_m01_defeat_02.ogg` | The Pale Reeve wrote thirty more names into his ledger, and the Ashmarch fell silent. | Der Fahle Vogt trug dreißig weitere Namen in sein Register ein, und die Aschmark verstummte. | tonlos, Trauer |
| `voices/narrator/narrator_m01_defeat_03.ogg` | So ended the last settlement. But this chronicle is not yet carved in stone. Rise, Warden, and try again. | So endete die letzte Siedlung. Doch diese Chronik ist noch nicht in Stein gemeißelt. Erhebe dich, Wächterin, und versuche es erneut. | Wendung zur Ermutigung im letzten Satz |

## 1.2 Bruder Hal Venn (`char_hal_venn`)

**Stimme:** `AC_char_hal_venn` · **Modell:** Eleven v3, *Natural*, Speed 0,97 (Tags sparsam) · alternativ Multilingual v2 ohne Tags
(Stability 0,50, Similarity 0,78, Style 0,20) · **Nachbearbeitung:** Standardkette, Wärme · Manifest: +1 dB.
**Regie allgemein:** väterlicher Mentor, warm, geerdet; in Gefahr klar, nie panisch. Spricht Isolde mit Vornamen an (vertrautes „du“ im DE).

### char_hal_venn_m01_intro (Event `char_hal_venn.m01_intro`) – Abspielmodus: **Sequenz**
*Kontext:* direkt nach `narrator_m01_opening`. Isolde trifft am Bergfried ein; Hal kommt ihr entgegen. Erste Aufgaben: Holz, Stein.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_intro_01.ogg` | Isolde! Flame be praised, you came. I half feared the road had swallowed you, like it swallowed the last courier. | Isolde! Die Flamme sei gepriesen, du bist gekommen. Ich fürchtete schon, die Straße hätte dich verschluckt, wie den letzten Boten. | herzliche Erleichterung, etwas außer Atem |
| `voices/characters/char_hal_venn_m01_intro_02.ogg` | [sighs] Look around you. Thirty souls, a cold hearth, and a chapel with no roof. This is what's left of Brennhollow. | Sieh dich um. Dreißig Seelen, ein kalter Herd und eine Kapelle ohne Dach. Das ist, was von Brennhollow übrig ist. | Seufzer, nüchterne Trauer |
| `voices/characters/char_hal_venn_m01_intro_03.ogg` | The keep still stands, at least. Set the people to work: timber from the woods, stone from the old quarry. We need walls before we need prayers. | Wenigstens steht der Bergfried noch. Stell die Leute an die Arbeit: Holz aus dem Wald, Stein aus dem alten Bruch. Wir brauchen Mauern, bevor wir Gebete brauchen. | tatkräftig, pragmatisch; Tutorial-Hinweis klar betonen |
| `voices/characters/char_hal_venn_m01_intro_04.ogg` | And Isolde… the bells in the northeast ruins have started ringing again. No one has rung them in a hundred years. [whispers] No one living. | Und Isolde … die Glocken in den Ruinen im Nordosten läuten wieder. Seit hundert Jahren hat sie niemand geläutet. Niemand Lebendes. | gesenkte Stimme, letzter Satz geflüstert |

### char_hal_venn_m01_first_farm (Event `char_hal_venn.m01_first_farm`) – Abspielmodus: **Sequenz**
*Kontext:* erstes Gehöft (`farmstead`) fertiggestellt; Nahrung wird produziert.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_first_farm_01.ogg` | A farmstead! Now that's a sight I'd stopped hoping for. | Ein Gehöft! Ein Anblick, auf den ich nicht mehr zu hoffen wagte. | ehrliche Freude |
| `voices/characters/char_hal_venn_m01_first_farm_02.ogg` | Bread before banners, my old abbot used to say. Fed people work harder, fight harder, and complain less. [chuckles] Well… slightly less. | Brot vor Bannern, sagte mein alter Abt immer. Satte Leute arbeiten härter, kämpfen härter und klagen weniger. Nun … etwas weniger. | Schmunzeln, Pointe |
| `voices/characters/char_hal_venn_m01_first_farm_03.ogg` | Build a few more, and Brennhollow will have a harvest by autumn. The Flame keeps us warm, but it doesn't fill our bellies. | Bau noch ein paar, und Brennhollow hat bis zum Herbst eine Ernte. Die Flamme wärmt uns, aber sie füllt nicht unsere Bäuche. | Tutorial-Hinweis (mehr Gehöfte), freundlich |

### char_hal_venn_m01_barracks (Event `char_hal_venn.m01_barracks`) – Abspielmodus: **Sequenz**
*Kontext:* Kaserne (`barracks`) fertiggestellt; Ausbildung von Schwertkämpfern, Speerträgern, Bogenschützen möglich.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_barracks_01.ogg` | Barracks. [sighs] I had hoped we would never need one again. | Eine Kaserne. Ich hatte gehofft, wir bräuchten nie wieder eine. | bedrückt |
| `voices/characters/char_hal_venn_m01_barracks_02.ogg` | Our people are farmers and woodcutters, Isolde, not soldiers. But give them a sword and someone to stand beside, and they'll learn. | Unsere Leute sind Bauern und Holzfäller, Isolde, keine Soldaten. Aber gib ihnen ein Schwert und jemanden an ihrer Seite, und sie werden es lernen. | Vertrauen in die Menschen |
| `voices/characters/char_hal_venn_m01_barracks_03.ogg` | Train them well. Swords for the line, spears to hold it, and bows on the walls. The dead don't tire, so we'll need numbers. | Bilde sie gut aus. Schwerter für die Linie, Speere, um sie zu halten, und Bögen auf den Mauern. Die Toten ermüden nicht, also brauchen wir Masse. | Tutorial-Hinweis (Einheitentypen), bestimmt |

### char_hal_venn_m01_wave_warning (Event `char_hal_venn.m01_wave_warning`) – Abspielmodus: **Zufallsvariante**
*Kontext:* Angriffswelle der Hohlen nähert sich (mehrfach pro Mission). Wird von `sfx/world/horn_alarm_*` begleitet.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_wave_warning_01.ogg` | [shouting] Isolde! Movement from the ruins! The Hollowed are coming! | Isolde! Bewegung bei den Ruinen! Die Hohlen kommen! | laut, dringlich, nicht hysterisch |
| `voices/characters/char_hal_venn_m01_wave_warning_02.ogg` | Do you hear that? Bones on stone… They're coming. Man the walls! | Hörst du das? Knochen auf Stein … Sie kommen. Besetzt die Mauern! | erst lauschend leise, dann Befehl |
| `voices/characters/char_hal_venn_m01_wave_warning_03.ogg` | The bells are ringing again! Get everyone behind the palisade, now! | Die Glocken läuten wieder! Alle hinter die Palisade, sofort! | gehetzt, kräftig |
| `voices/characters/char_hal_venn_m01_wave_warning_04.ogg` | Torches up! Something is coming out of the dark, and it isn't friendly! | Fackeln hoch! Etwas kommt aus der Dunkelheit, und es ist nicht freundlich! | angespannt, trockener Rest-Humor |

### char_hal_venn_m01_shrine_found (Event `char_hal_venn.m01_shrine_found`) – Abspielmodus: **Sequenz**
*Kontext:* Einheiten entdecken den Hohlschrein (`hollow_shrine`) mit Knochentürmen (`bone_spire`). Musik wechselt Richtung `build_tension`.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_shrine_found_01.ogg` | Flame preserve us… That's the Hollow Shrine of Vael Dorn. I thought it was only a story to frighten novices. | Flamme bewahre uns … Das ist der Hohlschrein von Vael Dorn. Ich dachte, er sei nur eine Geschichte, um Novizen zu erschrecken. | Ehrfurcht, Furcht, leise |
| `voices/characters/char_hal_venn_m01_shrine_found_02.ogg` | Look at the ashglass growing from the stones. The Veil is thin there, Isolde. Thin enough for something to reach through. | Sieh dir das Aschglas an, das aus den Steinen wächst. Der Schleier ist dort dünn, Isolde. Dünn genug, dass etwas hindurchgreifen kann. | erklärend, beunruhigt |
| `voices/characters/char_hal_venn_m01_shrine_found_03.ogg` | That bell… it's the Pale Reeve's bell. Destroy the shrine, and the dead lose their anchor here. Burn it to the ground. | Diese Glocke … das ist die Glocke des Fahlen Vogts. Zerstöre den Schrein, und die Toten verlieren hier ihren Anker. Brenn ihn nieder. | Missionsziel klar, entschlossen |

### char_hal_venn_m01_victory (Event `char_hal_venn.m01_victory`) – Abspielmodus: **Sequenz**
*Kontext:* Schrein zerstört, Vogt gefallen; vor `narrator_m01_victory`.

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/characters/char_hal_venn_m01_victory_01.ogg` | [exhales] It's over. By the Flame, it's truly over. The shrine is ash. | Es ist vorbei. Bei der Flamme, es ist wirklich vorbei. Der Schrein ist Asche. | Erleichterung, fast Tränen |
| `voices/characters/char_hal_venn_m01_victory_02.ogg` | Do you know what you've done? You've given these people something they haven't had in years. A reason to stay. | Weißt du, was du getan hast? Du hast diesen Leuten etwas gegeben, das sie seit Jahren nicht hatten. Einen Grund zu bleiben. | warm, stolz |
| `voices/characters/char_hal_venn_m01_victory_03.ogg` | [chuckles] Word will spread, Isolde. Refugees, traders… and the attention of greater lords. Let's fix the chapel roof before the king's men arrive. | Das spricht sich herum, Isolde. Flüchtlinge, Händler … und die Aufmerksamkeit größerer Herren. Reparieren wir das Kapellendach, bevor die Männer des Königs eintreffen. | humorvoll, Ausblick auf Zeitalter II |

---

# Teil 2 – Kampagnen-Dialogszenen Zeitalter I–VII (spätere Phasen, nicht im Manifest v1)

**Konventionen (Vorschlag, vor Produktion vom Kanon-Owner bestätigen):**
- Zeilen-IDs: `aNN_<szene>_<NN>` (NN = Zeitalter 01–07). Beispiel: `voices/characters/char_king_aldric_a02_summons_01.ogg`.
- Erzähler: `voices/narrator/narrator_aNN_<szene>_<NN>.ogg`.
- Heldin in Dialogszenen: `voices/characters/hero_isolde_aNN_<szene>_<NN>.ogg` (Vorschlag, siehe Leitfaden Abschnitt 10).
- Bosse in Dialogszenen: `voices/characters/<boss_id>_aNN_<szene>_<NN>.ogg` (Vorschlag).
- Entscheidungs-Reaktionen: `_choice_a_<NN>` / `_choice_b_<NN>` anhängen.
- Laufende Nummern pro Sprecher und Szene; die Spalte **#** gibt die Reihenfolge der Szene an.
- Modell: Dialog mit Eleven v3 (*Natural*), Erzähler mit Multilingual v2. Jede Zeile einzeln generieren; bei Bedarf den vorherigen
  Satz des Gegenübers als Kontext voranstellen und danach wegschneiden.

## Zeitalter I – *The Last Settlement*: „Asche des Schreins“ (`a01_shrine_ash`)
*Kontext:* Nacht nach Mission 1. Isolde und Hal Venn stehen in den Trümmern des Hohlschreins. Im Zentrum pulsiert ein Aschglas-Kern.
Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie. **Entscheidung:** Kern zerschlagen oder zur Erforschung bergen.

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_a01_shrine_ash_01.ogg` | Hal Venn | The fire's out, but the stones are still warm. That's not natural, Isolde. | Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde. | beunruhigt, leise |
| 2 | `voices/characters/hero_isolde_a01_shrine_ash_01.ogg` | Isolde | Nothing about this place is natural. Look at that crystal. It's… breathing. | Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet. | angespannt, misstrauisch |
| 3 | `voices/characters/char_deep_voice_a01_shrine_ash_01.ogg` | Stimme aus dem Tief | [whispers] Warden… you broke my little bell. How rude. | Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich. | zärtlich-spöttisch, Drei-Schichten-Effekt |
| 4 | `voices/characters/hero_isolde_a01_shrine_ash_02.ogg` | Isolde | [whispers] Hal. Tell me you heard that. | Hal. Sag mir, dass du das gehört hast. | kaum hörbar, kalter Schreck |
| 5 | `voices/characters/char_hal_venn_a01_shrine_ash_02.ogg` | Hal Venn | Heard what? Isolde, your hand is shaking. Step away from it. | Was gehört? Isolde, deine Hand zittert. Geh weg davon. | besorgt, väterlich |
| 6 | `voices/characters/char_deep_voice_a01_shrine_ash_02.ogg` | Stimme aus dem Tief | He cannot hear me. Only those who have burned with fever can. Do you remember the fever, child? I remember you. | Er kann mich nicht hören. Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich. | langsam, intim, Bezug zu Isoldes Narbe |
| 7 | `voices/characters/char_hal_venn_a01_shrine_ash_03.ogg` | Hal Venn | That ashglass could tell us how the shrine was built. Or it could be the last thing we ever study. Your choice, Warden. | Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. Oder es ist das Letzte, was wir je erforschen. Deine Entscheidung, Wächterin. | ernst, Entscheidung einleiten |
| 8A | `voices/characters/hero_isolde_a01_shrine_ash_choice_a_01.ogg` | Isolde | **[Option A – zerschlagen]** No. Whatever's in there, it knows my name. Break it. | Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es. | hart, entschlossen |
| 9A | `voices/characters/char_deep_voice_a01_shrine_ash_choice_a_01.ogg` | Stimme aus dem Tief | [laughs] Break the glass, little flame. There is so much more of me below. | Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir. | leises, hohles Lachen |
| 10A | `voices/characters/char_hal_venn_a01_shrine_ash_choice_a_01.ogg` | Hal Venn | [exhales] Good. Some knowledge is better left in the ash. | Gut. Manches Wissen bleibt besser in der Asche. | erleichtert |
| 8B | `voices/characters/hero_isolde_a01_shrine_ash_choice_b_01.ogg` | Isolde | **[Option B – bergen]** We take it. If we don't understand what we're fighting, we've already lost. | Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren. | rational, aber zögerlich |
| 9B | `voices/characters/char_deep_voice_a01_shrine_ash_choice_b_01.ogg` | Stimme aus dem Tief | Yes… carry me home. Keep me close to your fire. | Ja … trag mich heim. Halte mich nah an deinem Feuer. | zufrieden, verführerisch |
| 10B | `voices/characters/char_hal_venn_a01_shrine_ash_choice_b_01.ogg` | Hal Venn | Then it sleeps under the chapel, wrapped in iron, and I pray over it every night. | Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber. | widerwillig, entschlossen |

## Zeitalter II – *The Rising Crown*: „Der Ruf des Königs“ (`a02_summons`)
*Kontext:* Brennhollow ist gewachsen. Isolde wird an den Königshof gerufen. König Aldric III. braucht ihren Erfolg für seine
Legitimität; Herzogin Mireille Corvane bietet ihr Truppen gegen Gefolgschaft. Kassian Vale beobachtet. **Entscheidung:** dem König
den Treueid schwören oder Mireilles Bündnis annehmen.

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_a02_summons_01.ogg` | Aldric | The Warden of the Ashmarch. The court has spoken of little else for a month. Some call you a hero. Some call you a problem. | Die Wächterin der Aschmark. Der Hof spricht seit einem Monat von kaum etwas anderem. Manche nennen Euch eine Heldin. Manche ein Problem. | müde Würde, Thronsaal-Hall |
| 2 | `voices/characters/hero_isolde_a02_summons_01.ogg` | Isolde | I'm usually both, Your Majesty. It depends on who's paying for the walls. | Meist beides, Majestät. Kommt darauf an, wer die Mauern bezahlt. | trocken, respektvoll |
| 3 | `voices/characters/char_mireille_a02_summons_01.ogg` | Mireille | [laughs softly] Charming. And honest. Nordmark could use both, Warden. The Crown, it seems, can afford neither. | Charmant. Und ehrlich. Nordmark könnte beides gebrauchen, Wächterin. Die Krone, so scheint es, kann sich keins von beidem leisten. | spöttisch-elegant |
| 4 | `voices/characters/char_king_aldric_a02_summons_02.ogg` | Aldric | [angry] You forget yourself, Duchess. You stand in my hall. | Ihr vergesst Euch, Herzogin. Ihr steht in meiner Halle. | aufflackernder Zorn, Stimme bricht leicht |
| 5 | `voices/characters/char_mireille_a02_summons_02.ogg` | Mireille | I stand in a hall with a leaking roof and an empty treasury, Majesty. Warden, I offer five hundred spears and grain for a year. All I ask is that you remember who sent them. | Ich stehe in einer Halle mit undichtem Dach und leerer Schatzkammer, Majestät. Wächterin, ich biete fünfhundert Speere und Korn für ein Jahr. Ich verlange nur, dass Ihr Euch erinnert, wer sie geschickt hat. | ruhig, präzise, Drohung im Lächeln |
| 6 | `voices/characters/char_kassian_a02_summons_01.ogg` | Kassian | [whispers] A word of advice, Warden. Whichever hand you take tonight, the other one will hold a knife tomorrow. | Ein Rat, Wächterin. Welche Hand Ihr heute Nacht auch nehmt – die andere hält morgen ein Messer. | beiläufig, nah, amüsiert |
| 7 | `voices/characters/char_king_aldric_a02_summons_03.ogg` | Aldric | Kneel, Isolde Raventhorn. Your father knelt to me once. Will his daughter? | Kniet nieder, Isolde Raventhorn. Euer Vater kniete einst vor mir. Wird es seine Tochter tun? | fast flehend hinter der Würde |
| 8A | `voices/characters/hero_isolde_a02_summons_choice_a_01.ogg` | Isolde | **[Option A – Treueid]** My father died holding your border. I'll hold it too. For the Ember Crown, Majesty. | Mein Vater starb, als er Eure Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät. | ernst, feierlich |
| 9A | `voices/characters/char_king_aldric_a02_summons_choice_a_01.ogg` | Aldric | [exhales] Then the Crown is not as alone as they say. Rise, Warden. | Dann ist die Krone nicht so allein, wie man sagt. Erhebt Euch, Wächterin. | gerührt, erleichtert |
| 10A | `voices/characters/char_mireille_a02_summons_choice_a_01.ogg` | Mireille | How loyal. I do hope loyalty keeps your people fed this winter. | Wie loyal. Ich hoffe sehr, dass Loyalität Eure Leute diesen Winter satt macht. | eisig höflich |
| 8B | `voices/characters/hero_isolde_a02_summons_choice_b_01.ogg` | Isolde | **[Option B – Mireilles Bündnis]** My people can't eat oaths, Majesty. Duchess, I accept your spears. | Meine Leute können keine Eide essen, Majestät. Herzogin, ich nehme Eure Speere an. | bedauernd, pragmatisch |
| 9B | `voices/characters/char_mireille_a02_summons_choice_b_01.ogg` | Mireille | A wise woman. Nordmark remembers its friends, Warden. And its debts. | Eine kluge Frau. Nordmark erinnert sich an seine Freunde, Wächterin. Und an seine Schuldner. | zufrieden, Unterton |
| 10B | `voices/characters/char_king_aldric_a02_summons_choice_b_01.ogg` | Aldric | [tired] So. Even the Ashmarch turns away. Leave me. | So. Selbst die Aschmark wendet sich ab. Lasst mich allein. | gebrochen, leise |

## Zeitalter III – *The Broken Borders*: „Pass der Ahnen“ (`a03_ancestor_pass`)
*Kontext:* Die Hohlen brechen an den Nordgrenzen durch. Isolde braucht Skarnheims Eisen und Krieger. Torvald Eisenhand empfängt sie
an einem Gebirgspass vor den Gräbern seiner Ahnen. **Entscheidung:** Blutschwur (gemeinsamer Kampf, Ehre) oder Bezahlung in Gold.

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_a03_ancestor_pass_01.ogg` | Torvald | So the southern warden climbs my mountain. You're smaller than the songs say. | Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen. | dröhnend, prüfend, Berghall |
| 2 | `voices/characters/hero_isolde_a03_ancestor_pass_01.ogg` | Isolde | Songs exaggerate. I hear yours make you taller and wiser. | Lieder übertreiben. Eure machen Euch angeblich größer und weiser. | trocken, furchtlos |
| 3 | `voices/characters/char_torvald_a03_ancestor_pass_02.ogg` | Torvald | [laughs] Ha! She bites! Good. The dead came through the high pass three nights ago. We buried forty of our own. Forty. | Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den Hochpass. Wir haben vierzig der Unseren begraben. Vierzig. | dröhnendes Lachen, das in Trauer kippt |
| 4 | `voices/characters/hero_isolde_a03_ancestor_pass_02.ogg` | Isolde | Then we want the same thing. The Hollowed gone, and our dead left in peace. | Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden. | ernst, Respekt |
| 5 | `voices/characters/char_torvald_a03_ancestor_pass_03.ogg` | Torvald | Wanting is cheap. The Ancestor Path asks for more. Southerners pay in gold. Kin pay in blood. Which are you? | Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold. Sippe zahlt in Blut. Was bist du? | schwer, rituell |
| 6 | `voices/characters/char_torvald_a03_ancestor_pass_04.ogg` | Torvald | Choose before the snow covers the graves, warden. My ancestors are impatient. | Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig. | leiser, drängend |
| 7A | `voices/characters/hero_isolde_a03_ancestor_pass_choice_a_01.ogg` | Isolde | **[Option A – Blutschwur]** Give me the knife. My blood on your stones, and my sword beside your clan. | Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite eures Clans. | feierlich, kurzer Schmerzlaut `[exhales]` danach möglich |
| 8A | `voices/characters/char_torvald_a03_ancestor_pass_choice_a_01.ogg` | Torvald | [shouting] Hear that, fathers? Skarnheim has a new sister! Bring the mead! | Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met! | jubelnd, Gruppenjubel-Layer möglich |
| 7B | `voices/characters/hero_isolde_a03_ancestor_pass_choice_b_01.ogg` | Isolde | **[Option B – Gold]** I'm not kin, Torvald. But my gold is good, and my word is better. Name your price. | Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort noch besser. Nenn deinen Preis. | sachlich, ehrlich |
| 8B | `voices/characters/char_torvald_a03_ancestor_pass_choice_b_01.ogg` | Torvald | [sighs] Gold, then. Iron you'll have, and warriors for hire. But don't expect them to die for you. | Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben. | enttäuscht, kühl |

## Zeitalter IV – *The Age of Conquest*: „Das Register der Flut“ (`a04_tide_ledger`)
*Kontext:* Die Glutkrone (oder Mireilles Bündnis) will die Aurethwüste erreichen. Der Landweg ist lang; Ophira Saelith-Varo bietet
die Flotte des Saelith-Pakts. Kassian Vale warnt vor den Bedingungen. **Entscheidung:** Flotte annehmen (Schuld) oder Landmarsch.

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_a04_tide_ledger_01.ogg` | Ophira | Welcome aboard, Warden. Mind the ropes, they cost more than your boots. | Willkommen an Bord, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel. | amüsiert, Hafenakustik |
| 2 | `voices/characters/hero_isolde_a04_tide_ledger_01.ogg` | Isolde | Everything here costs more than my boots. That's what worries me. | Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen. | trocken |
| 3 | `voices/characters/char_ophira_a04_tide_ledger_02.ogg` | Ophira | [laughs] Forty ships, three thousand sellswords, and the Aureth coast in nine days instead of ninety. The price is a share of every shard of ashglass you dig out of those ruins. | Vierzig Schiffe, dreitausend Söldner und die Küste Aureths in neun Tagen statt neunzig. Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus diesen Ruinen grabt. | kehliges Lachen, dann geschäftlich |
| 4 | `voices/characters/char_kassian_a04_tide_ledger_01.ogg` | Kassian | Read the small print, Warden. In Saelith, the small print is where the knives are kept. | Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf. | nah, ruhig |
| 5 | `voices/characters/char_ophira_a04_tide_ledger_03.ogg` | Ophira | Master Vale. Still alive? How disappointing for so many people. | Meister Vale. Noch am Leben? Wie enttäuschend für so viele Leute. | zuckersüß, Spitze |
| 6 | `voices/characters/char_kassian_a04_tide_ledger_02.ogg` | Kassian | [amused] Only for the ones who owe you money, Ophira. | Nur für die, die Euch Geld schulden, Ophira. | amüsiert |
| 7 | `voices/characters/char_ophira_a04_tide_ledger_04.ogg` | Ophira | The tide waits for no one, Warden. Sign, or walk. | Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft. | plötzlich kühl |
| 8A | `voices/characters/hero_isolde_a04_tide_ledger_choice_a_01.ogg` | Isolde | **[Option A – Flotte]** Give me the quill. Every day we march is another village lost. | Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf. | entschlossen, Bedauern |
| 9A | `voices/characters/char_ophira_a04_tide_ledger_choice_a_01.ogg` | Ophira | Splendid. The tide always comes back for what it is owed, Warden. Always. | Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer. | warm mit Drohung |
| 8B | `voices/characters/hero_isolde_a04_tide_ledger_choice_b_01.ogg` | Isolde | **[Option B – Landmarsch]** We'll walk. I'd rather owe my blisters than owe you. | Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch. | trocken, stolz |
| 9B | `voices/characters/char_ophira_a04_tide_ledger_choice_b_01.ogg` | Ophira | [laughs softly] Pride. The most expensive thing a ruler can buy. Fair winds, Warden. | Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin. | ehrlich amüsiert, Respekt |

## Zeitalter V – *The Divided Throne*: „Nacht der Messer“ (`a05_divided_throne`)
*Kontext:* König Aldric liegt im Sterben; Mireille marschiert mit Nordmarks Heer auf den Königshof. Kassian enthüllt, dass Anhänger der
Sonnenhäresie den Hof unterwandert haben und die Hohlen mit Nachrichten versorgen. **Entscheidung:** Mireilles Anspruch stützen
(starke Hand, Einheit) oder Aldrics Linie verteidigen (Legitimität, Bürgerkrieg).

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_a05_divided_throne_01.ogg` | Aldric | [tired] Isolde. Come closer. My eyes… are not what they were. | Isolde. Kommt näher. Meine Augen … sind nicht mehr, was sie waren. | sterbend, flach, Atempausen |
| 2 | `voices/characters/hero_isolde_a05_divided_throne_01.ogg` | Isolde | I'm here, Majesty. Rest. | Ich bin hier, Majestät. Ruht Euch aus. | sanft, ungewohnt weich |
| 3 | `voices/characters/char_king_aldric_a05_divided_throne_02.ogg` | Aldric | Mireille is at the gates. If she takes the throne by the sword, every duke will think he can do the same. Valemark will tear itself apart. | Mireille steht vor den Toren. Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er könne dasselbe. Valemark wird sich selbst zerreißen. | Angst, letzte Kraft |
| 4 | `voices/characters/char_kassian_a05_divided_throne_01.ogg` | Kassian | Forgive the interruption. The Duchess is not your only problem. Three of the king's own councillors pray to the Sun Heresy. They've been sending our troop movements south. To the Hollowed. | Verzeiht die Störung. Die Herzogin ist nicht Euer einziges Problem. Drei der Räte des Königs beten zur Sonnenhäresie. Sie haben unsere Truppenbewegungen nach Süden geschickt. An die Hohlen. | ruhig, Gewicht auf „To the Hollowed“ |
| 5 | `voices/characters/hero_isolde_a05_divided_throne_02.ogg` | Isolde | [angry] And you're telling me now? | Und das sagt Ihr mir jetzt? | kalte Wut |
| 6 | `voices/characters/char_kassian_a05_divided_throne_02.ogg` | Kassian | I tell everyone everything at exactly the right moment, Warden. This is it. | Ich sage jedem alles genau im richtigen Moment, Wächterin. Das ist er. | ungerührt |
| 7 | `voices/characters/char_mireille_a05_divided_throne_01.ogg` | Mireille | [shouting] Open the gates, Warden! I didn't come to kill a dying man. I came to save a dying kingdom! | Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich kam, ein sterbendes Königreich zu retten! | von draußen, Außenhall, laut aber beherrscht |
| 8A | `voices/characters/hero_isolde_a05_divided_throne_choice_a_01.ogg` | Isolde | **[Option A – Mireille stützen]** Open the gates. Valemark needs a strong hand more than it needs a clean one. | Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere. | schwer, Entscheidung mit Schuld |
| 9A | `voices/characters/char_mireille_a05_divided_throne_choice_a_01.ogg` | Mireille | Thank you, Warden. History will call this treason or wisdom. Let's make sure it's wisdom. | Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit ist. | ernsthaft, erstmals ohne Spott |
| 10A | `voices/characters/char_king_aldric_a05_divided_throne_choice_a_01.ogg` | Aldric | [exhales] So be it… Keep… the fires lit… | So sei es … Haltet … die Feuer am Brennen … | letzte Worte, verhauchend |
| 8B | `voices/characters/hero_isolde_a05_divided_throne_choice_b_01.ogg` | Isolde | **[Option B – Aldrics Linie]** The gates stay shut. The crown passes by law, not by siege. Man the walls. | Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern. | hart, Befehlston |
| 9B | `voices/characters/char_mireille_a05_divided_throne_choice_b_01.ogg` | Mireille | Then you've chosen a corpse over a country. I'll see you on the field, Warden. | Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin. | eiskalt |
| 10B | `voices/characters/char_king_aldric_a05_divided_throne_choice_b_01.ogg` | Aldric | Your father… would be proud. I am… sorry it costs so much. | Euer Vater … wäre stolz. Es tut mir … leid, dass es so viel kostet. | dankbar, schwach |

## Zeitalter VI – *The Fallen Gods*: „Der graue Traum“ (`a06_grey_dream`)
*Kontext:* Die alten Götter schweigen seit dem Aschenfall. Im Dornwald greift die Hohlung nach der Wurzelmutter. Sylaen führt Isolde zu
ihr. **Entscheidung:** die verdorbenen Wurzeln verbrennen (Wurzelmutter stirbt, Wald überlebt verwundet) oder in den Traum eintreten,
um sie zu heilen (Gefahr, dass Isolde der Stimme aus dem Tief begegnet).

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_a06_grey_dream_01.ogg` | Sylaen | Walk softly, Warden. This is where Calyx last spoke to us, before the gods fell silent. | Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. | gedämpft, ehrfürchtig, Waldhall |
| 2 | `voices/characters/hero_isolde_a06_grey_dream_01.ogg` | Isolde | The roots are grey. Like the ashglass at Vael Dorn. | Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. | Erkennen, Unbehagen |
| 3 | `voices/characters/boss_root_mother_a06_grey_dream_01.ogg` | Wurzelmutter | Children… so many children with fire in their hands… are you here to hold me, or to burn me? | Kinder … so viele Kinder mit Feuer in den Händen … seid ihr hier, um mich zu halten, oder um mich zu verbrennen? | uralt, verwirrt, Chor-Dopplung |
| 4 | `voices/characters/char_sylaen_a06_grey_dream_02.ogg` | Sylaen | [whispers] She's dreaming the Deep's dream. If she wakes like this, the Thornwild will become a second Veilscar. | Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe. | flüsternd, Angst |
| 5 | `voices/characters/char_deep_voice_a06_grey_dream_01.ogg` | Stimme aus dem Tief | Your gods were not killed, little flame. They were… digested. Calyx tasted of spring. | Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … verdaut. Calyx schmeckte nach Frühling. | genüsslich, sehr langsam |
| 6 | `voices/characters/char_sylaen_a06_grey_dream_03.ogg` | Sylaen | There are two paths, and I hate them both. Burn the grey roots, and she dies. Or enter her dream, and try to wake her yourself. | Es gibt zwei Wege, und ich hasse beide. Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken. | gequält, ehrlich |
| 7A | `voices/characters/hero_isolde_a06_grey_dream_choice_a_01.ogg` | Isolde | **[Option A – verbrennen]** I'm sorry, Sylaen. I can't gamble a whole forest on a dream. Bring the torches. | Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln. | schwer, leise |
| 8A | `voices/characters/boss_root_mother_a06_grey_dream_choice_a_01.ogg` | Wurzelmutter | Warm… so warm… sleep well, children… | Warm … so warm … schlaft gut, Kinder … | verlöschend, friedlich |
| 9A | `voices/characters/char_sylaen_a06_grey_dream_choice_a_01.ogg` | Sylaen | The Thornwild will remember this fire, Warden. And so will I. | Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch. | Trauer, eisig |
| 7B | `voices/characters/hero_isolde_a06_grey_dream_choice_b_01.ogg` | Isolde | **[Option B – Traum betreten]** No more fires. Show me the way in. | Keine Feuer mehr. Zeig mir den Weg hinein. | entschlossen, mit Angst |
| 8B | `voices/characters/char_deep_voice_a06_grey_dream_choice_b_01.ogg` | Stimme aus dem Tief | [whispers] Yes… come inside. I have waited so long to meet you properly. | Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen. | intim, triumphierend |
| 9B | `voices/characters/char_sylaen_a06_grey_dream_choice_b_01.ogg` | Sylaen | Hold my hand, and don't let go. Whatever you hear in there… it is not your friend. | Halt meine Hand und lass nicht los. Was immer du dort drin hörst … es ist nicht dein Freund. | eindringlich, nah |

### Zusatzszene Zeitalter VI: „Der Gipfel der Glut“ (`a06_cinder_peak`)
*Kontext:* Isolde sucht den Aschdrachen Vyrmathax in der Glutweite auf, der als einziges Wesen den Aschenfall bewusst erlebt hat.

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_01.ogg` | Vyrmathax | Another little crown climbs my mountain. How brave. How brief. | Wieder klettert eine kleine Krone auf meinen Berg. Wie tapfer. Wie kurz. | gelangweilt-erhaben, Höhlenhall |
| 2 | `voices/characters/hero_isolde_a06_cinder_peak_01.ogg` | Isolde | You watched Aureth fall. I need to know how Varkhan opened the Veil. | Du hast Aureth fallen sehen. Ich muss wissen, wie Varkhan den Schleier öffnete. | fest, trotz Furcht |
| 3 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_02.ogg` | Vyrmathax | [laughs] He did not open it, mortal. He knocked. Something else opened the door. | Er hat ihn nicht geöffnet, Sterbliche. Er hat angeklopft. Etwas anderes hat die Tür geöffnet. | grollendes Lachen |
| 4 | `voices/characters/hero_isolde_a06_cinder_peak_02.ogg` | Isolde | Then tell me how to close it. | Dann sag mir, wie man sie schließt. | drängend |
| 5 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_03.ogg` | Vyrmathax | A door closes from both sides, little warden. Someone must stand on the other side. Are you that brave? Or merely that brief? | Eine Tür schließt man von beiden Seiten, kleine Wächterin. Jemand muss auf der anderen Seite stehen. Bist du so tapfer? Oder nur so kurz? | langsam, prüfend |
| 6 | `voices/characters/hero_isolde_a06_cinder_peak_03.ogg` | Isolde | [exhales] I suppose we'll find out. | Ich schätze, das werden wir herausfinden. | trocken, Angst darunter |

## Zeitalter VII – *The Last Crown*: „Solmere“ (`a07_last_crown`)
*Kontext:* Finale in der Festung Solmere. Der Hohle König Varkhan Aurelith sitzt auf dem Sonnenthron; hinter ihm klafft der Schleier.
Hal Venn begleitet Isolde. **Entscheidung:** den Schleier von innen versiegeln (Isolde opfert ihre Glut/sich selbst) oder die Hohle Krone
nehmen, um die Toten zu befehligen (Macht, die Stimme aus dem Tief bleibt).

| # | Ziel-Datei | Sprecher | EN-Text | DE | Regie |
|---|---|---|---|---|---|
| 1 | `voices/characters/boss_hollow_king_a07_last_crown_01.ogg` | Hohler König | I was the sun of Aureth. Kings knelt to my shadow. And now a border warden walks into my throne room. | Ich war die Sonne Aureths. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal. | majestätisch-brüchig, Thronsaal-Hall |
| 2 | `voices/characters/hero_isolde_a07_last_crown_01.ogg` | Isolde | Your shadow killed half a world, Varkhan. I'm here to send it back. | Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken. | ruhig, kalt |
| 3 | `voices/characters/boss_hollow_king_a07_last_crown_02.ogg` | Hohler König | I wanted to end death. For everyone. Do you understand? No more graves. No more grieving mothers. [whispers] It answered me… and it lied. | Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. Es hat mir geantwortet … und es hat gelogen. | Tragik, Wahnsinn bricht durch |
| 4 | `voices/characters/char_deep_voice_a07_last_crown_01.ogg` | Stimme aus dem Tief | I never lie. I promised him no one would die. And no one in Solmere ever has. | Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist nie wieder jemand gestorben. | sanft, grausam |
| 5 | `voices/characters/char_hal_venn_a07_last_crown_01.ogg` | Hal Venn | Isolde, the Veil is open behind the throne. Whatever you're going to do, do it now. | Isolde, hinter dem Thron ist der Schleier offen. Was immer du tun willst, tu es jetzt. | dringlich, Angst um sie |
| 6 | `voices/characters/boss_hollow_king_a07_last_crown_03.ogg` | Hohler König | Take my crown, warden. Command the dead, and no village will ever burn again. Or step through the Veil and close it behind you. Either way… someone must wear the ash. | Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … jemand muss die Asche tragen. | müde, beinahe erlöst |
| 7 | `voices/characters/char_deep_voice_a07_last_crown_02.ogg` | Stimme aus dem Tief | Choose, little flame. I will be here either way. | Wähle, kleine Flamme. Ich bin so oder so hier. | geduldig, gleichgültig |
| 8A | `voices/characters/hero_isolde_a07_last_crown_choice_a_01.ogg` | Isolde | **[Option A – Schleier versiegeln]** Hal. Keep the fires lit for me. Somebody has to hold the door. | Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten. | Abschied, gefasst, kurzes `[exhales]` |
| 9A | `voices/characters/char_hal_venn_a07_last_crown_choice_a_01.ogg` | Hal Venn | [crying] No… Isolde… Flame keep you, child. Flame keep you. | Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich. | brechende Stimme, Tränen |
| 10A | `voices/characters/char_deep_voice_a07_last_crown_choice_a_01.ogg` | Stimme aus dem Tief | Brave. Brief. And now… mine. Welcome home. | Tapfer. Kurz. Und nun … mein. Willkommen daheim. | ruhig, endgültig |
| 11A | `voices/narrator/narrator_a07_last_crown_choice_a_01.ogg` | Erzähler | And so the Veil was closed. No one crowned the Warden of the Ashmarch. But in Brennhollow, a fire has burned every night since, and no one has ever let it go out. | Und so wurde der Schleier geschlossen. Niemand krönte die Wächterin der Aschmark. Doch in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen. | Epilog, ruhig, bewegt (v2) |
| 8B | `voices/characters/hero_isolde_a07_last_crown_choice_b_01.ogg` | Isolde | **[Option B – Hohle Krone]** Give me the crown. If the dead must march, they'll march for the living. | Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden. | hart, Stimme bereits leicht verändert (dezenter Deep-Voice-Layer 10 %) |
| 9B | `voices/characters/boss_hollow_king_a07_last_crown_choice_b_01.ogg` | Hohler König | [laughs] That is exactly what I said. | Genau das habe ich auch gesagt. | hohles, trauriges Lachen, zerfallend |
| 10B | `voices/characters/char_hal_venn_a07_last_crown_choice_b_01.ogg` | Hal Venn | Isolde… your eyes. Flame forgive us both. | Isolde … deine Augen. Die Flamme vergebe uns beiden. | entsetzt, leise |
| 11B | `voices/narrator/narrator_a07_last_crown_choice_b_01.ogg` | Erzähler | And so the last crown of Eldraeth was forged from ash. The dead stood guard at every border, and no village burned again. But at night, in the halls of Solmere, the queen would sometimes speak to someone no one else could hear. | Und so wurde die letzte Krone Eldraeths aus Asche geschmiedet. Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte je wieder. Doch nachts, in den Hallen Solmeres, sprach die Königin bisweilen mit jemandem, den niemand sonst hören konnte. | Epilog, ambivalent, unheimlich leise am Ende (v2) |

---

<!-- BEGIN KAMPAGNE (erzeugt von `ashen_tools forge-campaign`, nicht von Hand ändern) -->
# Teil 3 – Echtzeit-Kampagne „Das Zeitalter der Gefallenen“ (Missionsdialoge, im Manifest)

Dieser Teil wird aus den Missionsdaten erzeugt (`cargo run -p ashen_tools -- forge-campaign`): Jede Zeile steht
identisch in der Mission (`assets/data/campaign/*.ron`), als Event im Manifest und hier. **Sprechtext = exakt der deutsche Spieltext**,
ohne Anpassung zu generieren.

**Konventionen**
- Event: `<sprecher-id>.<mission>_<szene>_<buchstabe>`; Datei: `voices/narrator/narrator_<mission>_<szene>_<buchstabe>_01.ogg` bzw. `voices/characters/<sprecher-id>_<mission>_<szene>_<buchstabe>_01.ogg`.
- Jede Zeile ein eigenes Event mit genau einer Datei (`_01`); die Reihenfolge der Zeilen steht in der Spalte **#**. Das Spiel spielt die Zeilen einer Szene nacheinander; die Pausen ergeben sich aus der Textlänge.
- Stimme: Profil `AC_<sprecher-id>` aus `elevenlabs_characters.md` (Sprache **Deutsch**, Multilingual v2 für den Erzähler, Eleven v3 *Natural* für Figuren und Bosse; Tags nur sparsam und nicht im Spieltext). Akzentangaben der Profile gelten als leichte Färbung.
- Zielformat und Loudness wie in Teil 1 (Ogg Vorbis q6, 48 kHz, mono, −16 LUFS, −1 dBTP). Manifest: Erzähler +2 dB, Figuren +1 dB.
- Neue Sprecher dieses Teils: `char_wendel`, `char_nel`, `boss_stone_oath`, `boss_reef_mistress` (Profile in `elevenlabs_characters.md`, Teil F).

## Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)

### `m02_kern`
*Kontext:* Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_kern_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde. | beunruhigt, leise |
| 2 | `voices/characters/hero_isolde_m02_kern_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet. | angespannt, misstrauisch |
| 3 | `voices/characters/char_deep_voice_m02_kern_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich. | zärtlich-spöttisch, mehrschichtiger Effekt (Profil Deep Voice) |
| 4 | `voices/characters/hero_isolde_m02_kern_d_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Hal. Sag mir, dass du das gehört hast. | kaum hörbar, kalter Schreck |
| 5 | `voices/characters/char_hal_venn_m02_kern_e_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Was gehört? Isolde, deine Hand zittert. Geh weg davon. | besorgt, väterlich |
| 6 | `voices/characters/char_deep_voice_m02_kern_f_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Er kann mich nicht hören. Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich. | leise, vertraut, bedrohlich |
| 7 | `voices/characters/char_hal_venn_m02_kern_g_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. Oder es ist das Letzte, was wir je untersuchen. Deine Wahl, Wächterin. | ernst, väterlich |

### `m02_kern_a`
*Kontext:* Reaktion auf die Entscheidung: Kern zerschlagen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_kern_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es. | hart, entschlossen |
| 2 | `voices/characters/char_deep_voice_m02_kern_a_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir. | leises, hohles Lachen |
| 3 | `voices/characters/char_hal_venn_m02_kern_a_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Gut. Manches Wissen bleibt besser in der Asche. | erleichtert |

### `m02_kern_b`
*Kontext:* Reaktion auf die Entscheidung: Kern bergen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_kern_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren. | ruhig, entschieden |
| 2 | `voices/characters/char_deep_voice_m02_kern_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Ja … trag mich heim. Halte mich nah an deinem Feuer. | zufrieden, verführerisch |
| 3 | `voices/characters/char_hal_venn_m02_kern_b_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber. | beklommen, entschlossen |

### `m02_aufbruch`
*Kontext:* Beginn der Eskorte, Ascheregen setzt ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m02_aufbruch_a_01.ogg` | Erzähler (`AC_narrator`) | Drei Tage nach dem Fall des Schreins kam ein Reiter aus dem Süden. Harrowmere, das Dorf im Aschmoor, brenne. Wer noch gehen könne, warte am Brunnen – auf ein Wunder oder auf jemanden, der keines verspreche. | getragen, Trauer unter der Ruhe |
| 2 | `voices/characters/hero_isolde_m02_aufbruch_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Wir führen sie nach Brennhollow. Kein Wagen bleibt zurück, kein Kind. Bleibt auf der Straße, und wenn der Ascheregen fällt, bleibt zusammen. | bestimmt, trocken |

### `m02_furt`
*Kontext:* Der Zug erreicht die Furt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_furt_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Das Wasser reicht bis zu den Knien. Tragt die Kinder, und redet nicht – das Moor hört zu. | gedämpft, konzentriert |

### `m02_hinterhalt`
*Kontext:* Hinterhalt im Röhricht vor der Furt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_hinterhalt_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Im Röhricht, links! Schließt die Reihen, die Schwachen in die Mitte! | scharf, alarmiert |

### `m02_ankunft`
*Kontext:* Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_ankunft_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Die Tore auf! Öffnet die Tore – Wasser, Decken, Brot! Sie leben, Isolde. Sie leben. | warm, erleichtert, rufend |
| 2 | `voices/characters/char_nel_m02_ankunft_b_01.ogg` | Nel (`AC_char_nel`) | Der Mann im Moor hat gesagt, ich soll dir ausrichten: Es war nett, dich kennenzulernen. | kindlich, monoton, unheimlich |
| 3 | `voices/characters/hero_isolde_m02_ankunft_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Welcher Mann, Nel? | behutsam, aber angespannt |
| 4 | `voices/characters/char_hal_venn_m02_ankunft_d_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Das Kind hört etwas, das uns nicht hört. Wir können es bei uns behalten und wachen – oder es den Flammenschwestern in Kaldris anvertrauen. | behutsam, ernst |

### `m02_nel_a`
*Kontext:* Entscheidung: Nel bleibt in Brennhollow.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_nel_a_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Dann bleibt sie bei uns, und ich werde ihre Träume bewachen. Die Flamme verzeiht eher Mut als Furcht. | sanft, entschlossen |
| 2 | `voices/characters/char_nel_m02_nel_a_b_01.ogg` | Nel (`AC_char_nel`) | Danke. Hier ist es warm. | leise, kindlich |

### `m02_nel_b`
*Kontext:* Entscheidung: Nel geht zu den Flammenschwestern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_nel_b_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Die Flammenschwestern in Kaldris kennen Kinder wie sie. Es ist vernünftig – und es bricht mir das Herz. | bedrückt, ehrlich |
| 2 | `voices/characters/char_nel_m02_nel_b_b_01.ogg` | Nel (`AC_char_nel`) | Kommst du mich besuchen? | klein, hoffnungsvoll |

### `m02_sieg`
*Kontext:* Schlusswort der Mission.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m02_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Sechzehn Menschen mehr in Brennhollow, und kein Vorrat für sie. Doch an diesem Abend zählte niemand nach: Es brannten zwei Feuer mehr auf dem Marktplatz als am Morgen. | warm, leise Hoffnung |

## Mission 3 – Die Graue Straße (`m03`)

### `m03_aufbruch`
*Kontext:* Start am Südende der Grauen Straße.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m03_aufbruch_a_01.ogg` | Erzähler (`AC_narrator`) | Die Graue Straße führte einmal von Brennhollow bis zu den Salzmärkten Valemarks. Nun führte sie vor allem zu Gräbern am Straßenrand – und an ihrem Ende saß ein Ritter, der mit seinen Opfern Buch führte. | trocken, düster |
| 2 | `voices/characters/hero_isolde_m03_aufbruch_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Neun Leute für eine Straße von dreißig Meilen. Das ist keine Armee, das ist ein Spaziergang mit Absicht. | trocken, entschlossen |

### `m03_briefe`
*Kontext:* Isolde findet im zweiten Posten die Briefe mit dem Siegel von Nordmark.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_briefe_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Briefe. Das Wachs trägt das Siegel von Nordmark – und die Summe reicht, um eine Kompanie ein Jahr lang zu bezahlen. | nachdenklich, kühl |
| 2 | `voices/characters/hero_isolde_m03_briefe_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Jemand bezahlt die Räuber dafür, dass Brennhollow hungert. Das merke ich mir. | leise, hart |

### `m03_wendel`
*Kontext:* Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_wendel_m03_wendel_a_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | Haltet ein, Wächterin! Ich lege die Klinge nieder – ich sehe, wann ein Spiel verloren ist. Und ich hasse es, für Verlierer zu arbeiten. | heiser, spöttisch, erschöpft |
| 2 | `voices/characters/hero_isolde_m03_wendel_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Du hast Fuhrleute erschlagen, Wendel. Ich habe die Karren gesehen. | kalt |
| 3 | `voices/characters/char_wendel_m03_wendel_c_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. Aber ich kenne jeden Pfad zwischen hier und Kaldris – und meine Männer können kämpfen. | ehrlich, müde |
| 4 | `voices/narrator/narrator_m03_wendel_d_01.ogg` | Erzähler (`AC_narrator`) | Ein Wegelagerer, der um sein Leben handelt. Die Flamme verlangt Gerechtigkeit – aber Brennhollow braucht Schwerter. Die Entscheidung liegt bei der Wächterin. | ruhig, abwägend |

### `m03_wendel_a`
*Kontext:* Entscheidung: Wendel wird gehängt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_wendel_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Du hängst, Wendel. Für jeden Karren, den du verbrannt hast. Die Straße braucht keine Reue, sie braucht Ruhe. | hart, endgültig |
| 2 | `voices/characters/char_wendel_m03_wendel_a_b_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | Gerecht. Fast. Sagt den Fuhrleuten … dass ich sie gezählt habe. | gefasst, leise |

### `m03_wendel_b`
*Kontext:* Entscheidung: Wendel und seine Männer werden in Dienst genommen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_wendel_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Du dienst mir, bis die Straße sicher ist. Wenn du uns verrätst, hole ich dich persönlich. | nüchtern, ohne Drohgebärde |
| 2 | `voices/characters/char_wendel_m03_wendel_b_b_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | Die Graue Kompanie steht Euch zu Diensten, Wächterin. Es ist das erste Mal seit Jahren, dass ich bei Tag schlafen kann. | erleichtert, trocken |

### `m03_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m03_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Am Abend zogen die ersten Karren seit zwei Jahren nordwärts. Die Aschmark war immer noch ein Grenzland – aber nun eines mit einer Straße. | ruhig, Anerkennung |

## Mission 4 – Die Lange Nacht (`m04`)

### `m04_nacht`
*Kontext:* Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m04_nacht_a_01.ogg` | Erzähler (`AC_narrator`) | Die Hohlen kamen nicht mehr in Wellen. In dieser Nacht kamen sie wie eine Flut, die einen Deich gefunden hat. Auf den Zinnen von Brennhollow zählten die Wächter ihre Fackeln und sagten nichts. | schwer, ruhig |
| 2 | `voices/characters/char_hal_venn_m04_nacht_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Der Gesandte des Königs soll bei Sonnenaufgang eintreffen, Isolde. Wenn er uns lebend findet, haben wir genug erreicht. | trocken, müde, väterlich |
| 3 | `voices/characters/hero_isolde_m04_nacht_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dann sorgen wir dafür, dass er uns lebend findet. Bogenschützen auf die Türme, Schwertträger ans Tor! | bestimmt |

### `m04_mitte`
*Kontext:* Die Grabkolosse erreichen die Mauern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m04_mitte_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Sie bringen die Kolosse! Haltet die Tore, Isolde – ich singe die Mauer fest, so lange meine Stimme trägt! | rufend, eindringlich |

### `m04_morgen`
*Kontext:* Morgengrauen. Der Gesandte des Königs, Kassian Vale, trifft am Südtor ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m04_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | Der Himmel über der Aschmark wurde grau, dann fahl, dann golden an den Rändern. Die Letzten der Hohlen sanken in die Erde zurück, als hätten sie nur geträumt, lebendig zu sein. | getragen, Erleichterung |
| 2 | `voices/characters/char_kassian_m04_morgen_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | Eine Nacht, die man in Kaldris noch in Jahren erzählen wird – leider ohne Wahrheit. Ich bin Kassian Vale. Seine Majestät bittet Euch, Wächterin, um Euren Treueeid. Und bevor Ihr antwortet: Herzogin Mireille Corvane bietet Euch fünfhundert Speere und Korn für ein Jahr. | geschmeidig, höflich, leicht amüsiert |
| 3 | `voices/characters/char_kassian_m04_morgen_c_01.ogg` | Kassian Vale (`AC_char_kassian`) | Zwei Hände strecken sich Euch entgegen, Wächterin. Die Krone fordert Euren Eid, Nordmark bietet Speere. Welche Hand Ihr heute nehmt – die andere hält morgen ein Messer. | leise, ruhig, eindringlich |

### `m04_eid_a`
*Kontext:* Entscheidung: Treueeid auf die Krone.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m04_eid_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Mein Vater starb, als er Valemarks Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät. | fest, förmlich |
| 2 | `voices/characters/char_kassian_m04_eid_a_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | Wie standhaft. Der König wird es zu schätzen wissen – und die Herzogin sich Euren Namen merken. | höflich, mit Unterton |

### `m04_eid_b`
*Kontext:* Entscheidung: Bündnis mit Herzogin Mireille Corvane.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m04_eid_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Meine Leute können keine Eide essen. Richtet der Herzogin aus, dass ich ihre Speere annehme. | trocken, pragmatisch |
| 2 | `voices/characters/char_kassian_m04_eid_b_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | Eine kluge Entscheidung – und eine sehr teure. Nordmark erinnert sich an Freunde, Wächterin. Und an Schuldner. | ruhig, vieldeutig |

## Mission 6 – Das Turnier von Kaldris (`m06`)

### `m06_turnier`
*Kontext:* Die Arena von Kaldris, Turniertag.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m06_turnier_a_01.ogg` | Erzähler (`AC_narrator`) | Kaldris trug Gold und Weiß. Über der Arena hingen die Banner der zwölf Herzogtümer, und auf der Tribüne saß ein alternder König, der aussah, als warte er auf etwas, das nicht der Sieger war. | festlich, mit leisem Unterton |
| 2 | `voices/characters/char_kassian_m06_turnier_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | Eine Beobachtung, Wächterin, ganz unverbindlich: Heute Abend wird der Balkon der Krone nicht jeden verlassen, der ihn betritt. Ich überlasse es Euch, was Ihr mit diesem Wissen anfangt. | flüsternd, geschmeidig |

### `m06_runde2`
*Kontext:* Der König ruft aus der Loge zur zweiten Runde.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_m06_runde2_a_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | Die Wächterin der Aschmark! Man erzählt, sie schlage die Besten, ohne zu schwitzen. Zeigt es uns noch einmal! | laut, belebt, mit alter Stimme |

### `m06_glocke`
*Kontext:* Abendglocke: Die Attentäter am Balkon der Krone regen sich.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m06_glocke_a_01.ogg` | Erzähler (`AC_narrator`) | Es war das Läuten der Abendglocke, das ihn rettete: ein falscher Ton, eine Sekunde zu früh. Auf dem Balkon der Krone regte sich ein Schatten, dann zwei, dann drei. | gespannt, rasch |

### `m06_anschlag`
*Kontext:* Die Attentäter fliehen vom Balkon der Krone.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_anschlag_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dort oben! Die Reiter voran – holt sie ein, bevor sie die Gassen erreichen! | scharf |

### `m06_kassian_a`
*Kontext:* Entscheidung: Der König wird gewarnt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Majestät, ein Wort unter vier Augen: Es gab eine Warnung, und sie war nicht ohne Eigennutz. | ernst, leise |
| 2 | `voices/characters/char_king_aldric_m06_kassian_a_b_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | Ihr habt mir das Leben gerettet und das Spiel verdorben. Ich weiß nicht, was mich mehr bewegt. | müde, dankbar |

### `m06_kassian_b`
*Kontext:* Entscheidung: Kassian wird in der Hand behalten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Kassian. Ihr habt mehr gewusst, als Ihr gesagt habt. Dafür schuldet Ihr mir etwas. | kühl |
| 2 | `voices/characters/char_kassian_m06_kassian_b_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | So ist es. Es ist angenehm, wenn jemand begreift, dass Wissen eine Währung ist. Ich werde zahlen, wenn die Stunde kommt. | mit feinem Lächeln |

### `m06_kassian_c`
*Kontext:* Entscheidung: Schweigen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Es war ein Anschlag, mehr nicht. Ich habe nichts gesehen, was zu erzählen wäre. | neutral, abweisend |
| 2 | `voices/characters/char_kassian_m06_kassian_c_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | Wie diskret. Ich bin beinahe enttäuscht – und beeindruckt. | trocken, flüchtig |

### `m06_ende`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_kassian_m06_ende_a_01.ogg` | Kassian Vale (`AC_char_kassian`) | Nun wisst Ihr, dass ich es wusste. Was macht Ihr damit, Wächterin: Sagt Ihr es dem König, behaltet Ihr es als Pfand – oder schweigt Ihr? | leise, belustigt |

## Mission 7 – Eisen in den Hügeln (`m07`)

### `m07_lager`
*Kontext:* Belagerungslager vor Kargsted.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m07_lager_a_01.ogg` | Erzähler (`AC_narrator`) | Eisen war in der Aschmark kostbarer als Brot. Wer die Schmiede befeuern wollte, musste durch Kargsted – und Kargsted hatte Mauern, die ein Schwarm Toter nie gebraucht hätte. | trocken, sachlich |
| 2 | `voices/characters/hero_isolde_m07_lager_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Die Rammen zuerst ans Tor, die Katapulte auf die Eckschützen. Wer die Mauer mit der Brust einreißen will, lernt es heute auf die harte Weise. | knapp, entschlossen |

### `m07_tor`
*Kontext:* Das Südtor fällt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m07_tor_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Das Tor ist gebrochen! Schildträger vor, Schützen nach – keine Gefangenen, aber lasst die Bauern am Leben! | triumphierend, scharf |

### `m07_sieg`
*Kontext:* Nach dem Sieg im Hof von Kargsted.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m07_sieg_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Eisen. Wirkliches Eisen. Ich muss es dir lassen, Isolde: Du verstehst, wann man eine Mauer überredet und wann man sie einreißt. | warm, anerkennend |

## Mission 9 – Der Schwarze Winter (`m09`)

### `m09_schwur`
*Kontext:* Der Torhof von Hrimgard. Torvald Eisenhand empfängt Isolde vor den Gräbern seiner Ahnen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m09_schwur_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen. | dröhnend, amüsiert |
| 2 | `voices/characters/hero_isolde_m09_schwur_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Lieder übertreiben. Eure machen Euch angeblich größer und weiser. | trocken, furchtlos |
| 3 | `voices/characters/char_torvald_m09_schwur_c_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den hohen Pass. Wir haben vierzig der Unseren begraben. Vierzig. | grimmig, kurz auflachend, dann dunkel |
| 4 | `voices/characters/hero_isolde_m09_schwur_d_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden. | ernst, respektvoll |
| 5 | `voices/characters/char_torvald_m09_schwur_e_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold, Sippe zahlt in Blut. Was bist du? | scharf, prüfend |
| 6 | `voices/characters/char_torvald_m09_schwur_f_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig. | leiser, drängend |

### `m09_schwur_a`
*Kontext:* Entscheidung: Blutschwur. Torvald und seine Clankrieger kämpfen mit.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m09_schwur_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite deines Clans. | fest, ohne Zögern |
| 2 | `voices/characters/char_torvald_m09_schwur_a_b_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met – und die Äxte! | jubelnd, laut |

### `m09_schwur_b`
*Kontext:* Entscheidung: Bezahlung in Gold. Söldner statt Torvald.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m09_schwur_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort ist besser. Nenne deinen Preis. | ruhig, geschäftsmäßig |
| 2 | `voices/characters/char_torvald_m09_schwur_b_b_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben. | ernüchtert, abweisend |

### `m09_sturm`
*Kontext:* Der Sturm setzt ein; die erste Welle naht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m09_sturm_a_01.ogg` | Erzähler (`AC_narrator`) | Der Schnee fiel schwarz, wo die Asche ihn berührte. Auf den Zinnen von Hrimgard sangen die Skaldinnen ihre Ahnenlieder gegen den Wind, und der Wind sang zurück – mit hohlen Stimmen. | rau, getragen |

### `m09_kolosse`
*Kontext:* Grabkolosse erreichen die Mauern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m09_kolosse_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Kolosse! Die Äxte an die Mauer, ihr Brüder – bricht der Koloss die Mauer, brechen wir den Koloss! | brüllend, kampflustig |

### `m09_morgen`
*Kontext:* Der Morgen über Hrimgard.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m09_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | Als der Morgen kam, lagen die Hohlen wie umgekippte Statuen im Schnee. Auf der Mauer standen die Überlebenden und sahen nicht hinunter – sie sahen nach Osten, wo es hell wurde. | ruhig, erschöpft |

## Mission 10 – Wurzel und Asche (`m10`)

### `m10_wald`
*Kontext:* Lichtung der Veyl am Waldrand.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_wald_a_01.ogg` | Erzähler (`AC_narrator`) | Der Dornwald war älter als die Götter, und er hatte es nie vergessen. Wer ihn betrat, tat es auf Zehenspitzen – und wer ihn verließ, nahm ein wenig Rinde in den Knochen mit. | ehrfürchtig, leise |
| 2 | `voices/characters/char_sylaen_m10_wald_b_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. Was welkt, welkt von innen – nicht durch Feuer, sondern durch einen Traum. | gedämpft, ernst, melodisch |

### `m10_lager1`
*Kontext:* Das erste Lager ist geräumt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_m10_lager1_a_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Sie sind nicht böse. Sie sind leer. Der Wald hat ihre Namen vergessen, und sie suchen sie in unseren Stimmen. | traurig, gedämpft |

### `m10_lager3`
*Kontext:* Das dritte Lager ist geräumt, der Herzhain liegt vor ihnen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_lager3_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. Sylaen, was immer da im Hain wartet, es ist dasselbe Grau. | erkennend, unbehaglich |

### `m10_hain`
*Kontext:* Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_root_mother_m10_hain_a_01.ogg` | Die Wurzelmutter (`AC_boss_root_mother`) | Kinder … so viele Kinder mit Feuer in den Händen … seid ihr gekommen, um mich zu halten oder um mich zu verbrennen? | uralt, zerrissen, mehrstimmig |
| 2 | `voices/characters/char_sylaen_m10_hain_b_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe. | flüsternd, drängend |
| 3 | `voices/characters/char_deep_voice_m10_hain_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … verdaut. Calyx schmeckte nach Frühling. | intim, kalt, genüsslich |
| 4 | `voices/characters/char_sylaen_m10_hain_d_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Es gibt zwei Wege, und ich hasse beide. Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken. | gequält, nüchtern |
| 5 | `voices/characters/char_sylaen_m10_hain_e_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Entscheide, Wächterin. Der Wald wartet nicht – und sie auch nicht. | leise, drängend |

### `m10_hain_a`
*Kontext:* Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_hain_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln. | schwer, entschlossen |
| 2 | `voices/characters/boss_root_mother_m10_hain_a_b_01.ogg` | Die Wurzelmutter (`AC_boss_root_mother`) | Warm … so warm … schlaft gut, Kinder … | verlöschend, friedlich |
| 3 | `voices/characters/char_sylaen_m10_hain_a_c_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch. | Trauer, eisig |

### `m10_hain_b`
*Kontext:* Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_hain_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Keine Feuer mehr. Zeig mir den Weg hinein. | entschlossen, mit Angst |
| 2 | `voices/characters/char_deep_voice_m10_hain_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen. | intim, flüsternd |
| 3 | `voices/characters/char_sylaen_m10_hain_b_c_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Halt meine Hand und lass nicht los. Was immer du dort drin hörst … es ist nicht dein Freund. | eindringlich, leise |

### `m10_sieg_a`
*Kontext:* Schlusswort: Wurzelmutter verbrannt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_sieg_a_a_01.ogg` | Erzähler (`AC_narrator`) | Der Hain brannte drei Tage lang. Als die Asche kalt war, wuchs zwischen den Wurzeln ein einzelner heller Trieb – und niemand wagte es, ihn zu fällen. | ruhig, bittersüß |

### `m10_sieg_b`
*Kontext:* Schlusswort: Traum betreten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_sieg_b_a_01.ogg` | Erzähler (`AC_narrator`) | Isolde Raventhorn kehrte nach drei Atemzügen aus dem Traum zurück – und sprach in den folgenden Wochen nicht mehr als nötig. Die Wurzelmutter aber erwachte, und der Dornwald antwortete mit Blüten. | leise, geheimnisvoll |

## Mission 11 – Der Riss von Marrowfen (`m11`)

### `m11_moor`
*Kontext:* Der Südrand des Moores von Marrowfen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m11_moor_a_01.ogg` | Erzähler (`AC_narrator`) | In Marrowfen riecht es nach nasser Asche und nach Dingen, die zu lange unter Wasser geschlafen haben. Wo der Riss aufgegangen war, wuchs kein Schilf mehr – nur ein weißer Ring aus Knochenstaub. | leise, unheimlich |
| 2 | `voices/characters/char_hal_venn_m11_moor_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Drei Weiler, Isolde. Wenn wir schnell sind, schaffen wir alle drei vor der Dämmerung. Und wenn wir langsam sind … schaffen wir es gar nicht. | angespannt, nüchtern |

### `m11_dorf`
*Kontext:* Ein Weiler schließt sich dem Trupp an.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m11_dorf_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Kommt, kommt – langsam, haltet euch an den Händen. Wir bringen euch über den Damm, im Namen der Flamme! | warm, rufend |

### `m11_riss`
*Kontext:* Isolde erreicht den Riss; die Stimme aus dem Tief macht ihr Angebot.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_deep_voice_m11_riss_a_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Du bist weit gekommen, kleine Flamme. Sieh, was ich dir zeige: Aschglas, so rein, dass es singt. Nimm davon, und deine Heere werden nie wieder hungern. | schmeichelnd, tief, mehrschichtig |
| 2 | `voices/characters/char_hal_venn_m11_riss_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Hör nicht hin, Isolde. Was dort flüstert, hat Harrowmere verbrannt. | scharf, besorgt |
| 3 | `voices/characters/char_hal_venn_m11_riss_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Der Riss kann geschlossen werden, wenn wir den Schrein an seinem Grund zerschlagen. Oder wir nehmen, was er uns reicht – und tragen die Folgen. | ernst, beschwörend |

### `m11_riss_a`
*Kontext:* Entscheidung: Riss versiegeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m11_riss_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Du bietest mir Brot, aus dem Mund eines Hungernden. Nein. Hal, schließ ihn. | kalt, entschlossen |
| 2 | `voices/characters/char_deep_voice_m11_riss_a_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Wie schade. Ich hatte mich gefreut, dich zu verwöhnen. | leicht gekränkt, spöttisch |

### `m11_riss_b`
*Kontext:* Entscheidung: Riss anzapfen und Aschglas ernten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m11_riss_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Nur ein wenig. Nur so viel, wie die Aschmark in zehn Wintern verbraucht. | leise, mit schlechtem Gewissen |
| 2 | `voices/characters/char_deep_voice_m11_riss_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Ein wenig. Natürlich. Ich bin geduldig, Kind. Ich war es immer. | zufrieden, samtig |

### `m11_nel`
*Kontext:* Nel, die in Brennhollow aufgenommene Waise, weist den Weg (nur wenn sie bei Hal geblieben ist).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_nel_m11_nel_a_01.ogg` | Nel (`AC_char_nel`) | Hört ihr es? Es klingt wie eine Spieluhr, die jemand zu langsam aufzieht. Dort entlang geht es – das Schilf weiß den Weg. | kindlich, abwesend |

### `m11_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m11_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Der Damm trug die Geretteten ins Trockene. Hinter ihnen schloss sich das Moor – oder öffnete sich, je nachdem, wen man später fragte. | ruhig, vieldeutig |

## Mission 13 – Die Aschglasfelder (`m13`)

### `m13_wueste`
*Kontext:* Das Lager am Rand der Aurethwüste.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m13_wueste_a_01.ogg` | Erzähler (`AC_narrator`) | Es war ein Land, das einmal Reich geheißen hatte. Nun lag es als flache Asche bis zum Horizont, und in der Asche funkelten Splitter – Aschglas, wo der Schleier dünn war, und Wachfeuer, wo die Legion stand. | weit, ehrfürchtig, leise |
| 2 | `voices/characters/hero_isolde_m13_wueste_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Schwarze Rüstungen, Glut in den Fugen, und das Wappen Aureths auf der Brust. Das sind keine Wiedergänger, das ist eine Armee. Zeigt ihnen Stahl – und Feuer. | nüchtern, kalt |

### `m13_kompanie`
*Kontext:* Wendel, wenn er in Isoldes Dienst getreten ist.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_wendel_m13_kompanie_a_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | Die Graue Kompanie kennt Wüstenräuber, Wächterin – Tote in Rüstung sind neu. Aber Tote bluten wenigstens nicht auf den Sold. | trocken, kampfeslustig |

### `m13_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m13_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Aus den Fugen der erschlagenen Legionäre stieg noch lange rotes Licht. Wer es sah, sprach später nicht davon. | leise, bedrückt |

## Mission 14 – Das Schwarze Riff (`m14`)

### `m14_tintenfass`
*Kontext:* Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_m14_tintenfass_a_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Willkommen am Kai, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel. | amüsiert, Hafenakustik |
| 2 | `voices/characters/hero_isolde_m14_tintenfass_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen. | trocken |
| 3 | `voices/characters/char_ophira_m14_tintenfass_c_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Vierzig Schiffe, dreitausend Söldner und das Riff in neun Tagen statt in neunzig. Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus den Ruinen holt. | lachend, geschäftsmäßig |
| 4 | `voices/characters/char_kassian_m14_tintenfass_d_01.ogg` | Kassian Vale (`AC_char_kassian`) | Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf. | trocken, flüsternd |
| 5 | `voices/characters/char_ophira_m14_tintenfass_e_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft. | plötzlich kühl |

### `m14_flotte_a`
*Kontext:* Entscheidung: Die Flotte annehmen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m14_flotte_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf. | entschlossen, Bitterkeit |
| 2 | `voices/characters/char_ophira_m14_flotte_a_b_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer. | warm, mit schneidendem Unterton |

### `m14_flotte_b`
*Kontext:* Entscheidung: Ohne Flotte marschieren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m14_flotte_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch. | trocken, stolz |
| 2 | `voices/characters/char_ophira_m14_flotte_b_b_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin. | ehrlich amüsiert |

### `m14_isarre`
*Kontext:* Isarre Mol, auf der Riffinsel.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_reef_mistress_m14_isarre_a_01.ogg` | Isarre Mol (`AC_boss_reef_mistress`) | Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. Das Meer hat mir jedes meiner Kinder zurückgegeben – ich bin nur gekommen, es zu tränken. | tief, nass, gurgelnd, mehrstimmig |
| 2 | `voices/characters/hero_isolde_m14_isarre_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dann bin ich hier, um dich zu erlösen. Eine Flut kann man dämmen. | kalt, ruhig |

### `m14_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m14_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Das Wasser zog sich zurück, als hätte es nie anderes vorgehabt. Auf der Riffinsel blieben Muscheln, Knochen und eine Perle von der Größe einer Faust, die später in Brennhollow in eine Rüstung eingelegt wurde. | ruhig, bildhaft |

## Mission 17 – Tod eines Königs (`m17`)

### `m17_nacht`
*Kontext:* Kaldris, Nacht des Todes von König Aldric III.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m17_nacht_a_01.ogg` | Erzähler (`AC_narrator`) | Der König starb im Winter, und die Glocken von Kaldris waren die Einzigen, die es laut sagten. Im Flammendom brannten die Kerzen weiter, als hätte niemand ihnen Bescheid gegeben. | gedämpft, schwer |
| 2 | `voices/characters/hero_isolde_m17_nacht_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Drei Spuren, ein Mörder. Der Wein, die Flamme, die Gruft. Wir gehen leise vor – wer hier Zeugen hat, hat auch Feinde. | leise, konzentriert |

### `m17_keller`
*Kontext:* Im Weinkeller des Palasts.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_keller_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Schierling im Wein. Frisch angesetzt, und nur ein einziger Krug. Wer das getan hat, kannte seinen Geschmack – und wusste, dass er Gold trinkt, kein Wasser. | kühl, analytisch |

### `m17_kapelle`
*Kontext:* In der Kapelle der Ewigen Flamme.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_kapelle_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Ein Brief unter dem Altarstein, mit dem Siegel Nordmarks. Aber die Tinte ist zu frisch, das Wachs zu sauber. Das hat jemand für mich hingelegt. | misstrauisch, leise |

### `m17_krypta`
*Kontext:* In der Krypta unter dem Dom, optional.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_krypta_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Das Siegel der Sonnenhäresie, in den Stein der Gruft geritzt. Jemand hat dem König Gift gegeben – und die Hohlen haben es gewusst, bevor er starb. | ernst, erschüttert |

### `m17_hinweis`
*Kontext:* Nur wenn Kassian Isolde einen Gefallen schuldet (Entscheidung in Mission 6).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_kassian_m17_hinweis_a_01.ogg` | Kassian Vale (`AC_char_kassian`) | Ein Wort aus der Dunkelheit, Wächterin: Die Krypta. Ihr werdet dort etwas finden, das ich Euch nicht beweisen konnte. Ich sehe Euch im Dom. | flüsternd, aus dem Schatten |

### `m17_hof`
*Kontext:* Im Flammendom vor dem versammelten Hof.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m17_hof_a_01.ogg` | Erzähler (`AC_narrator`) | Im Flammendom waren sie alle versammelt: die Herzöge, die Flammenwahrer, die Räte. Und in der Mitte der Sarg des Königs, leer – der Leichnam lag noch im Nebenraum, damit niemand ihn küsste. | feierlich, kalt |
| 2 | `voices/characters/char_mireille_m17_hof_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Wächterin. Man sagt, Ihr wisst, wer es war. Sagt es – oder lasst uns dem Mörder zuprosten, so lange er bei uns sitzt. | scharf, souverän |
| 3 | `voices/characters/char_hal_venn_m17_hof_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Du darfst nur einen Namen nennen, Isolde. Es ist die Aufgabe einer Wächterin, zu entscheiden – und den Preis zu tragen. | ernst, väterlich |

### `m17_urteil_a`
*Kontext:* Beschuldigung: Kassian Vale.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Kassian Vale hat es gewusst. Er hat gewarnt, wo er hätte handeln müssen. Sucht ihn – und fragt ihn nach seiner Rolle. | gemessen, hart |
| 2 | `voices/characters/char_mireille_m17_urteil_a_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Ein Spion, der zu viel wusste. Wie vorhersehbar – und wie bequem. | zufrieden, kalt |

### `m17_urteil_b`
*Kontext:* Beschuldigung: Herzogin Mireille.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Der Brief weist nach Nordmark. Herzogin Mireille, erklärt Euch. | ruhig, schneidend |
| 2 | `voices/characters/char_mireille_m17_urteil_b_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Ihr beschuldigt eine Frau, die fünfhundert Speere in Euren Dienst gestellt hat. Ihr werdet es bereuen. | eisig, bedrohlich |

### `m17_urteil_c`
*Kontext:* Beschuldigung: die Sonnenhäretiker.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Es waren die Sonnenhäretiker. Ihre Zeichen sind im Stein der Gruft. Kaldris hat Verräter in den eigenen Mauern. | klar, laut |
| 2 | `voices/characters/char_mireille_m17_urteil_c_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Dann reinigt die Stadt. Aber lasst die Herzöge auch ruhig schlafen. | abwägend |

### `m17_urteil_d`
*Kontext:* Die ganze Wahrheit (alle drei Beweise).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_d_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Ich kenne die ganze Wahrheit: Gift im Wein, ein falscher Brief aus Nordmark, das Siegel der Häresie – und einer, der sie alle vermittelte. Seht auf die, die von Verwirrung leben. | ruhig, zwingend |
| 2 | `voices/characters/char_mireille_m17_urteil_d_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Ihr habt mehr Mut, als ich Euch zutraute. Und mehr Beweise, als mir lieb ist. | widerwillig respektvoll |

## Mission 19 – Die Furt von Kaldris (`m19`)

### `m19_thron`
*Kontext:* Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_m19_thron_a_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | Isolde. Kommt näher. Meine Augen … sind nicht mehr, was sie waren. | sterbend, flach, mit Atempausen |
| 2 | `voices/characters/hero_isolde_m19_thron_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Ich bin hier, Majestät. Ruht Euch aus. | sanft, ungewohnt weich |
| 3 | `voices/characters/char_king_aldric_m19_thron_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | Mireille steht vor den Toren. Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er dürfe es auch. Valemark wird sich zerfleischen. | ernst, leise, schwer atmend |
| 4 | `voices/characters/char_mireille_m19_thron_d_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich bin gekommen, ein sterbendes Königreich zu retten! | laut, fordernd, vom Feld |
| 5 | `voices/characters/char_hal_venn_m19_thron_e_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Mireille oder die Linie des Königs, Isolde. Beides hat seinen Preis – und beides wird Menschen töten. | dunkel, ernst |

### `m19_thron_a`
*Kontext:* Entscheidung: Mireille stützen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m19_thron_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere. | bitter, entschlossen |
| 2 | `voices/characters/char_mireille_m19_thron_a_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit wird. | ruhig, mit Würde |
| 3 | `voices/characters/char_king_aldric_m19_thron_a_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | So sei es … Haltet … die Feuer am Brennen … | letzte Worte, verhauchend |

### `m19_thron_b`
*Kontext:* Entscheidung: Aldrens Linie verteidigen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m19_thron_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern. | klar, unerbittlich |
| 2 | `voices/characters/char_mireille_m19_thron_b_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin. | kalt, enttäuscht |
| 3 | `voices/characters/char_king_aldric_m19_thron_b_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | Euer Vater … wäre stolz. Es tut mir … leid, dass es so viel kostet. | dankbar, schwach |

### `m19_schlacht`
*Kontext:* Beginn der Schlacht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m19_schlacht_a_01.ogg` | Erzähler (`AC_narrator`) | Zwei Heere an einem Fluss. Das Wasser war flach, die Furt breit, und auf beiden Ufern standen Menschen, die mit demselben Gebet in dieselbe Schlacht zogen. | ruhig, schwer |
| 2 | `voices/characters/hero_isolde_m19_schlacht_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Haltet die Furt! Die Reiter an die Flanken, Armbrüste in die Mitte – und lasst sie kommen! | laut, kommandierend |

### `m19_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m19_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Als der Rauch sich über der Furt lichtete, trug der Fluss seine Toten in die Ebene. Das Banner der Raventhorns wehte im Zentrum, und die Herzöge zählten ihre Schwüre noch einmal nach. | ruhig, gewichtig |

## Mission 21 – Der Schrein Oruuns (`m21`)

### `m21_stollen`
*Kontext:* Der Eingang des Stollens unter Skarnheims Gipfeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m21_stollen_a_01.ogg` | Erzähler (`AC_narrator`) | Der Berg hatte seit dem Aschenfall nicht mehr gesprochen. Wer an seiner Flanke stand, spürte ihn manchmal zittern – wie ein Schlafender, der etwas Schlimmes träumt. | tief, ruhig |
| 2 | `voices/characters/hero_isolde_m21_stollen_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Der Weg führt durch drei Hallen. Haltet die Reihen geschlossen, und wenn der Stein spricht: nicht antworten. | leise, kontrolliert |

### `m21_torvald`
*Kontext:* Nur wenn Torvald den Blutschwur gehalten hat.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m21_torvald_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Meine Väter haben diesen Schrein mit eigenen Händen gebaut. Dass ich ihn nun mit der Axt zurückfordern muss, ist eine Schande, die ich gern wasche. | grimmig, ehrfürchtig |

### `m21_soeldner`
*Kontext:* Nur wenn Torvald nicht mitkämpft (Gold-Variante).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m21_soeldner_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Gold kauft Schwerter, aber keine Trauer. Dieser Berg ist nicht meiner – und doch kämpfe ich um ihn. | nachdenklich, trocken |

### `m21_gharun`
*Kontext:* Isolde und Gharun im Schreinsaal.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_stone_oath_m21_gharun_a_01.ogg` | Gharun, der Steinerne Eid (`AC_boss_stone_oath`) | Wer wagt es, den Eid zu stören? Ich wurde gebunden, zu bewahren – und ich bewahre. Was ich bewahre, spricht nicht mehr mit euch. | steinern, langsam, dröhnend, hallend |
| 2 | `voices/characters/hero_isolde_m21_gharun_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Du bewahrst den Falschen, Wächter. Wir sind gekommen, den Schrein zurückzuholen – und deinen Eid zu lösen. | ruhig, gegenüber einem Berg |

### `m21_sieg`
*Kontext:* Der Schrein wird neu geweiht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m21_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Gharun zerfiel zu Staub, der in der Luft stand, als wollte er sich noch einmal verbeugen. Der Eid war nicht gebrochen, sondern erfüllt: Der Schrein war in Sicherheit. | ruhig, gedämpft |
| 2 | `voices/characters/hero_isolde_m21_sieg_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Oruun, wenn du zuhörst: Der Schrein ist dein. Wir kommen wieder, wenn du uns brauchst. | leise, ehrfürchtig |

## Mission 25 – Ithrens letztes Licht (`m25`)

### `m25_ruinen`
*Kontext:* Am Stadtrand von Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_ruinen_a_01.ogg` | Erzähler (`AC_narrator`) | Solmere war einst eine Stadt aus Licht gewesen, Terrassen aus weißem Stein bis an den Himmel. Nun waren es Rippen, in denen der Wind klagte – und zwischen den Rippen standen Männer mit Sonnen auf den Brustpanzern, die auf einen toten König warteten. | ehrfürchtig, schwermütig |
| 2 | `voices/characters/char_hal_venn_m25_ruinen_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Sei vorsichtig, Isolde. Die Sonnenhäretiker töten Fremde, aber sie töten sie mit Gebeten. Das macht es nicht besser. | trocken, besorgt |

### `m25_tagebuch`
*Kontext:* Am Schrein Ithrens: Varkhans Tagebuch.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_tagebuch_a_01.ogg` | Erzähler (`AC_narrator`) | Unter den Resten des Sonnenhofs lag das Licht Ithrens, kaum größer als eine Kerzenflamme. Daneben, in einer Nische aus Gold, ein Buch, dessen Seiten nach Rauch und Rosenwasser rochen. | leise, ehrfürchtig |
| 2 | `voices/characters/char_hal_venn_m25_tagebuch_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Varkhans Tagebuch. Er schreibt … von seiner Tochter. Sie starb an der Schwindsucht, und er wollte … er wollte nicht den Tod besiegen, Isolde. Er wollte sie. | betroffen, leise |
| 3 | `voices/characters/char_deep_voice_m25_tagebuch_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Ein Vater, der alles verbrannte, um ein Kind zu retten. So rührend. So nützlich. | samtig, süß, bedrohlich |
| 4 | `voices/characters/char_hal_venn_m25_tagebuch_d_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Es liegt bei dir, Isolde. Das Buch enthält die Wahrheit, und die Wahrheit hat Folgen. | ernst, bedächtig |

### `m25_urteil_a`
*Kontext:* Entscheidung: Die Wahrheit veröffentlichen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Die Wahrheit gehört den Menschen. Lasst sie lesen, was Varkhan getan hat – und warum. Wer ihn dann noch erlösen will, soll es wissen. | ruhig, klar |
| 2 | `voices/characters/char_hal_venn_m25_urteil_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Das wird die Häresie spalten. Aber vielleicht ist das besser, als wenn sie an einer Lüge zerbricht. | bedächtig |

### `m25_urteil_b`
*Kontext:* Entscheidung: Den Glauben der Häretiker lassen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Sie glauben an etwas, das sie am Leben hält. Ich nehme ihnen das nicht, bevor ich es ersetzen kann. | leise, widerwillig |
| 2 | `voices/characters/char_hal_venn_m25_urteil_b_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Gnade und Feigheit sind manchmal Geschwister. Ich hoffe, du weißt, welche du gerade wählst. | sanft, mahnend |

### `m25_urteil_c`
*Kontext:* Entscheidung: Die Häretiker für den eigenen Feldzug nutzen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Sie glauben, er werde zurückkehren. Gut. Sie sollen für ihn kämpfen – bis ich ihn selbst stelle. | kalt, berechnend |
| 2 | `voices/characters/char_deep_voice_m25_urteil_c_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Wie praktisch du bist, kleine Flamme. Wie ähnlich uns. | leise, amüsiert |

### `m25_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Das Licht Ithrens erlosch nicht. Es wartete, mit der Geduld der Götter, auf jemanden, der bereit war, es zu tragen. | leise, hoffnungsvoll |

## Mission 26 – Der Aschenmarsch (`m26`)

### `m26_marsch`
*Kontext:* Brennhollow, Abenddämmerung; die Aschenlegion naht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_marsch_a_01.ogg` | Erzähler (`AC_narrator`) | Sie kamen im Gleichschritt. Zum ersten Mal seit dem Aschenfall hörte man die Hohlen nicht durch die Nacht schlurfen, sondern marschieren – und der Boden der Aschmark antwortete mit einem Beben. | schwer, getragen |
| 2 | `voices/characters/char_hal_venn_m26_marsch_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Es ist die Legion, Isolde. Die Wappen Aureths auf jeder Brust. Ich habe sie als Junge in den Liedern gesehen – nicht im Fleisch. | bestürzt, leise |
| 3 | `voices/characters/hero_isolde_m26_marsch_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dies ist keine Entscheidung, die ich allein treffen will. Aber sie steht mir zu. Halten wir Brennhollow – oder retten wir die, die in ihm wohnen? | schwer, nachdenklich |

### `m26_wahl_a`
*Kontext:* Entscheidung: Brennhollow halten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_wahl_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Wir halten. Diese Mauern haben dreißig Jahre Feuer gesehen. Jeder Bürger auf die Wälle, jedes Kind in die Keller! | laut, kommandierend |
| 2 | `voices/characters/char_hal_venn_m26_wahl_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Die Flamme steht mit Brennhollow. Sie hat es immer getan. | fest, warm |

### `m26_wahl_b`
*Kontext:* Entscheidung: Brennhollow evakuieren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_wahl_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Wir räumen die Stadt. Die Legion kann Steine erobern, aber keine Menschen. Das Osttor auf – alle zum Kaldris-Pass! | kalt, schwer |
| 2 | `voices/characters/char_hal_venn_m26_wahl_b_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Ich habe diese Mauern mit eigenen Händen gesegnet. Es ist eine gute Entscheidung. Es wird nur lange wehtun. | rau, gefasst |

### `m26_flucht`
*Kontext:* Beginn der Evakuierung.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_flucht_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Haltet die Reihen, und lasst die Kinder zwischen den Wagen gehen! Die Soldaten decken den Rückzug – wer zurückbleibt, bleibt mit Ehre! | laut, drängend |

### `m26_morgen`
*Kontext:* Brennhollow hält bis zum Morgen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | Das Morgenlicht fand die Zinnen von Brennhollow zerschossen, aber nicht erstürmt. Die Legion zog sich zurück, dem Süden entgegen – ein Heer, das nicht müde wurde, aber wartete. | ruhig, erschöpft |

### `m26_pass`
*Kontext:* Die Evakuierung erreicht die Straße nach Kaldris.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_pass_a_01.ogg` | Erzähler (`AC_narrator`) | Hinter ihnen brannte Brennhollow. Niemand sah sich um. Wer zurückblickte, hörte das Läuten der Kapelle – und erst später begriff man, dass sie von selbst läutete. | leise, schmerzvoll |

## Mission 28 – Die Tore von Solmere (`m28`)

### `m28_lager`
*Kontext:* Das Lager des Bundes vor Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m28_lager_a_01.ogg` | Erzähler (`AC_narrator`) | Hinter dem Horizont wuchsen die Mauern Solmeres: schwarz, hoch, in den Fugen glimmend. Zum ersten Mal seit tausend Jahren sahen lebende Menschen die Hauptstadt Aureths aus der Nähe – und zählten die Zinnen, ohne sie zu Ende zu zählen. | weit, ehrfürchtig |
| 2 | `voices/characters/hero_isolde_m28_lager_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Rammen ans Tor, Katapulte auf die Zinnen. Und niemand betritt Solmere, ehe das Tor offen ist – ich will keinen Mann vor die Hexen schicken, der noch eine Mauer überwinden muss. | klar, knapp |

### `m28_torvald`
*Kontext:* Nur wenn Torvald den Blutschwur gehalten hat.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m28_torvald_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | Meine Ahnen sehen zu, Isolde. Sie haben nie eine Mauer gesehen, vor der Eisenhand zögerte. Wir gehen zuerst – du brichst das Tor. | grimmig, stolz |

### `m28_sylaen`
*Kontext:* Nur wenn die Wurzelmutter erwacht ist (Pakt).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_m28_sylaen_a_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | Der Wald hat mich nicht verlassen, Wächterin. Meine Pfeile fliegen mit dem Wind – und der Wind kennt Solmere. | ruhig, melodisch |

### `m28_ophira`
*Kontext:* Nur wenn der Vertrag mit Ophira geschlossen wurde.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_m28_ophira_a_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | Meine Schiffe liegen im Hafen, meine Söldner stehen bei Euch. Vergesst nicht, dass jeder Schuss in diesen Mauern ein Eintrag in meinem Buch ist. | trocken, geschäftsmäßig |

### `m28_tor`
*Kontext:* Das Haupttor fällt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m28_tor_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Das Tor ist gebrochen! Schildträger voran, die Priester folgen den Schilden – und lasst keinen zurück! | laut, triumphierend |

### `m28_haeretiker`
*Kontext:* Die Sonnenhäretiker auf den Mauern (Entscheidung in Mission 25).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m28_haeretiker_a_01.ogg` | Erzähler (`AC_narrator`) | Auf den Mauern Solmeres standen Männer mit Sonnen auf der Brust. Sie sahen die Banner des Bundes, hörten, was Isolde Raventhorn gelesen hatte, und senkten die Waffen. Man weiß nicht, ob aus Scham oder aus Hoffnung. | ruhig, bedächtig |

### `m28_sieg`
*Kontext:* Nach der Eroberung des Torturms.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m28_sieg_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Das war das Tor. Hinter ihm liegt der Thron. Ich muss dir etwas sagen, Isolde, bevor wir hineingehen: Danke. Für alles, was du getan hast – auch für das, was dich gekostet hat. | leise, bewegt |

## Mission 29 – Der Hohle Thron (`m29`)

### `m29_tor`
*Kontext:* Das Innere des Tores von Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_tor_a_01.ogg` | Erzähler (`AC_narrator`) | Hinter dem Tor lag, was von Aureth übrig war: vier Höfe, einer hinter dem anderen, wie Kammern eines Herzens. Und am Ende der Höfe der Thron, auf dem seit tausend Jahren jemand saß, der nicht schlief und nicht starb. | feierlich, tonlos |
| 2 | `voices/characters/char_hal_venn_m29_tor_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Gleich ist es so weit, Isolde. Wenn du Zweifel hast – jetzt wäre der Moment, sie auszusprechen. | leise, warm |
| 3 | `voices/characters/hero_isolde_m29_tor_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Zweifel habe ich seit Brennhollow. Aber sie halten mich nicht mehr auf. Vorwärts, für die Mark. | ruhig, fest |

### `m29_hoefe`
*Kontext:* Beim Betreten des Hofs des Schleiers.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_hoefe_a_01.ogg` | Erzähler (`AC_narrator`) | Der dritte Hof war der Hof des Schleiers. Die Luft zitterte, als wäre sie nur gemalt, und über den Mauern hing ein Riss, durch den man die Nacht sah – eine, die nicht die eigene war. | leise, gespannt |

### `m29_thron`
*Kontext:* Im Thronsaal: Varkhan und die Stimme aus dem Tief.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_hollow_king_m29_thron_a_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Ich war die Sonne von Aureth. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal. | archaisch, königlich, hohl |
| 2 | `voices/characters/hero_isolde_m29_thron_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken. | ruhig, kalt |
| 3 | `voices/characters/boss_hollow_king_m29_thron_c_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. Es hat mir geantwortet … und es hat gelogen. | plötzlich leise, gequält |
| 4 | `voices/characters/char_deep_voice_m29_thron_d_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist niemand mehr gestorben. | ruhig, entsetzlich ungerührt |

### `m29_schleier`
*Kontext:* Der König ist halb gefallen; der Schleier öffnet sich.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m29_schleier_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Isolde, der Schleier ist offen hinter dem Thron. Was immer du tun willst, tu es jetzt. | dringlich |
| 2 | `voices/characters/boss_hollow_king_m29_schleier_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … jemand muss die Asche tragen. | tonlos, erschöpft |
| 3 | `voices/characters/char_deep_voice_m29_schleier_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Wähle, kleine Flamme. Ich bin so oder so da. | geduldig, gleichgültig |
| 4 | `voices/characters/boss_hollow_king_m29_schleier_d_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Wähle, Wächterin. Es gibt keinen vierten Weg – außer denen, die du dir durch deine Wege verdient hast. | tonlos, müde |

### `m29_ende_a`
*Kontext:* Letzte Wahl: Schleier versiegeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten. | Abschied, ruhig, sehr gefasst |
| 2 | `voices/characters/char_hal_venn_m29_ende_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich. | brechende Stimme, Tränen |
| 3 | `voices/characters/char_deep_voice_m29_ende_a_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Tapfer. Kurz. Und nun … mein. Willkommen daheim. | ruhig, endgültig |

### `m29_ende_b`
*Kontext:* Letzte Wahl: Hohle Krone nehmen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden. | hart, kalt entschlossen |
| 2 | `voices/characters/boss_hollow_king_m29_ende_b_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Genau das habe ich auch gesagt. | hohles, trauriges Lachen, zerfallend |
| 3 | `voices/characters/char_hal_venn_m29_ende_b_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | Isolde … deine Augen. Die Flamme vergebe uns beiden. | entsetzt, leise |

### `m29_ende_c`
*Kontext:* Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Varkhan. Deine Tochter hieß Sereth. Sie hat dir nie gesagt, dass sie dir verzeiht, weil sie nie dachte, dass du es brauchst. Sie tut es. | weich, ruhig, ohne Kälte |
| 2 | `voices/characters/boss_hollow_king_m29_ende_c_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | Sereth … Sereth. Ich habe den Namen so lange nicht gehört. Wer – wer hat ihn dir gesagt? | zitternd, menschlich |
| 3 | `voices/characters/char_nel_m29_ende_c_c_01.ogg` | Nel (`AC_char_nel`) | Sie hat ihn mir gesagt. Im Schlaf. Sie wartet auf dich. | kindlich, klar, leise |

### `m29_ende_d`
*Kontext:* Letzte Wahl: Das Tiefe verbrennen (nur mit zerschlagenem Kern und versiegeltem Riss).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_d_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | Es gibt kein Tor, das ich schließen muss, und keinen Thron, den ich besteigen will. Es gibt nur Feuer – und das ist alles, was ich je gut konnte. | bitter, leidenschaftlich |
| 2 | `voices/characters/char_deep_voice_m29_ende_d_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | Du verbrennst dich selbst, kleine Flamme. Schade. Ich hätte dich so gern behalten. | bedauernd, samtig |

### `m29_sieg`
*Kontext:* Schlusswort der Endschlacht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | Der Hohle König war gefallen. Die Glocken von Solmere läuteten ein letztes Mal, und danach hörte man in der ganzen Welt nur noch den Wind. | ruhig, still |

## Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

### `age_age1`
*Kontext:* Kapitelseite Zeitalter I.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age1_a_01.ogg` | Erzähler (`AC_narrator`) | Jahr 897 nach dem Fall. Am Rand der Aschmark hält Brennhollow noch stand – dreißig Seelen, ein Bergfried, ein Feld. Und jede Nacht kommt der Nebel näher. | getragen, ruhig, Chronistenton |

### `age_age2`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age2_a_01.ogg` | Erzähler (`AC_narrator`) | Brennhollow wuchs, und mit ihm der Ruf seiner Wächterin. Doch wer in der Aschmark Mauern baut, wird in Kaldris gezählt: Der König prüft seine Getreuen, und jeder Hof hat sein Messer. | getragen, ruhig, Chronistenton |

### `age_age3`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age3_a_01.ogg` | Erzähler (`AC_narrator`) | Die Hohlen griffen nicht mehr in Wellen an, sondern wie ein Heer, das einen Plan verfolgt. An drei Grenzen brachen die Mauern – und die alten Feinde mussten lernen, einander zuzuhören. | getragen, ruhig, Chronistenton |

### `age_age4`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age4_a_01.ogg` | Erzähler (`AC_narrator`) | Aus der Verteidigerin wurde eine Eroberin. Die Aschglasfelder lockten Legion und Händler zugleich, und an der Küste wartete etwas, das nicht ertrunken war, sondern wartete. | getragen, ruhig, Chronistenton |

### `age_age5`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age5_a_01.ogg` | Erzähler (`AC_narrator`) | Der König war alt, und der Thron hat keine Freunde. In Kaldris wurden Zeugen leise und Glocken laut – und Isolde stand in der Mitte, mit dem Schwert in der einen und dem Gesetz in der anderen Hand. | getragen, ruhig, Chronistenton |

### `age_age6`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age6_a_01.ogg` | Erzähler (`AC_narrator`) | Die Götter schwiegen seit dem Aschenfall. Nun regten sich Altäre unter Bergen und Ruinen, und mit ihnen erwachte, was die Götter einst gebunden hatten – und was sich an ihrer Stelle genährt hatte. | getragen, ruhig, Chronistenton |

### `age_age7`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age7_a_01.ogg` | Erzähler (`AC_narrator`) | Das Hohle Tief erwachte nicht aus eigener Kraft: Es hatte immer einen Schlüssel. Brennhollow brannte noch einmal, und dann zog ein Bund aus Fahnen und Feuern nach Solmere, wo ein toter König auf seinem Thron wartete. | getragen, ruhig, Chronistenton |

### `age_ende`
*Kontext:* Enden der Kampagne, gesprochen nach dem Abspann der letzten Mission.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_ende_a_01.ogg` | Erzähler (`AC_narrator`) | Varkhan Aurelith sprach zum ersten Mal seit tausend Jahren den Namen seiner Tochter – und das Tiefe verlor seinen Schlüssel. Es wurde still in Solmere, und im Frühjahr blühte in der Aschmark der erste Apfelbaum seit dem Fall. | ruhig, getragen, langsam im letzten Satz |
| 2 | `voices/narrator/narrator_age_ende_b_01.ogg` | Erzähler (`AC_narrator`) | Das Feuer, das Isolde in Solmere entfachte, brannte neun Tage lang. Das Tiefe wich zurück, doch die Wunden der Welt blieben offen – und die Glut der Wächterin erlosch mit ihnen. | ruhig, getragen, langsam im letzten Satz |
| 3 | `voices/narrator/narrator_age_ende_c_01.ogg` | Erzähler (`AC_narrator`) | So wurde die letzte Krone Eldraeths aus Asche geschmiedet. Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte mehr. Doch nachts, in den Hallen von Solmere, sprach die neue Herrin manchmal mit jemandem, den niemand sah. | ruhig, getragen, langsam im letzten Satz |
| 4 | `voices/narrator/narrator_age_ende_d_01.ogg` | Erzähler (`AC_narrator`) | Und so wurde der Schleier geschlossen. Niemand krönte die Wächterin der Aschmark. Aber in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen. | ruhig, getragen, langsam im letzten Satz |

<!-- END KAMPAGNE -->

---


---


---


---


---


---


---


---


---


---


---


---

## Anhang – Produktionshinweise Dialog

1. **Kontext-Trick (v3):** Für natürliche Reaktionen den letzten Satz des Gegenübers als Kontext in dieselbe Generierung
   schreiben (z. B. in Klammern als Regie oder als vorangestellter Satz) und nach der Generierung wegschneiden.
2. **Konsistenz:** Eine Figur innerhalb einer Szene immer mit derselben Stimme, demselben Modell und derselben Stabilität generieren.
3. **Szenen-Akustik** (Thronsaal, Berg, Hafen, Wald) wird im Spiel bzw. in der Cutscene-Mischung ergänzt; die gelieferten Dateien
   bleiben trocken, sofern nicht im Charakterprofil anders angegeben (Deep Voice, Bosse).
4. **Untertitel** aus der EN-Spalte ohne Tags und ohne Optionsmarker **[Option …]** übernehmen; die Marker sind Regie, nicht Text.
5. **Lokalisierung:** Die DE-Spalte ist Bedeutungsreferenz; eine spätere deutsche Vertonung benötigt ein eigenes, lippen-/längenangepasstes Skript.
