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

## Anhang – Produktionshinweise Dialog

1. **Kontext-Trick (v3):** Für natürliche Reaktionen den letzten Satz des Gegenübers als Kontext in dieselbe Generierung
   schreiben (z. B. in Klammern als Regie oder als vorangestellter Satz) und nach der Generierung wegschneiden.
2. **Konsistenz:** Eine Figur innerhalb einer Szene immer mit derselben Stimme, demselben Modell und derselben Stabilität generieren.
3. **Szenen-Akustik** (Thronsaal, Berg, Hafen, Wald) wird im Spiel bzw. in der Cutscene-Mischung ergänzt; die gelieferten Dateien
   bleiben trocken, sofern nicht im Charakterprofil anders angegeben (Deep Voice, Bosse).
4. **Untertitel** aus der EN-Spalte ohne Tags und ohne Optionsmarker **[Option …]** übernehmen; die Marker sind Regie, nicht Text.
5. **Lokalisierung:** Die DE-Spalte ist Bedeutungsreferenz; eine spätere deutsche Vertonung benötigt ein eigenes, lippen-/längenangepasstes Skript.
