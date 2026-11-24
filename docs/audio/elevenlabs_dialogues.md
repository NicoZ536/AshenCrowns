# ASHEN CROWNS – Erzähler- und Dialogzeilen (ElevenLabs)

Grundlage: `assets/audio/manifest.ron` (Events `narrator.*`, `char_*.*`, `boss_*.*`, `hero_*.*`), Stimmprofile in
`docs/audio/elevenlabs_characters.md`, Regeln in `docs/audio/elevenlabs_production_guide.md`.

**Legende**
- **Ziel-Datei** relativ zu `assets/audio/`, `.ogg` Vorbis q6, 48 kHz, mono. Format und Lautheit sind Sache der Integration, nicht der Lieferung.
- **Eine Zeile = eine Datei:** Jedes Event (Erzähler-, Dialog-, Figurenzeile) besteht aus **genau einer** Datei `<stem>_01.ogg` mit dem **vollständigen** Text in **einem** Take.
  Keine Teilstücke, kein Schneiden, kein Zusammensetzen. (Frühere Mehrteiler `_01…_n` sind zu einer Datei zusammengeführt.)
- **Sprechtext (DE, exakt wie im Spiel)** = einfügefertiger Generierungstext **inklusive Audio-Tags**. Alles in eckigen Klammern (`[solemn]`, `[pause]`) ist ein
  Eleven-v3-Audio-Tag (englisch, wird nicht gesprochen). Ohne die Tags ist der Text wörtlich der Spieltext aus `assets/data/missions/*.ron` bzw. `assets/data/campaign/*.ron`
  (Prüfung: `python3 docs/audio/verify_spoken_text.py`). Alle Stimmen, auch der Erzähler, laufen auf **Eleven v3** (Multilingual v2 versteht keine Tags).
- **Audio-Tags:** 1–4 pro Zeile; Emotion/Ton am Anfang (`[solemn]`, `[whispers]`, `[angry]`, `[fearful]`, `[tired]`, `[commanding]`, `[shouting]`, `[laughs softly]`, `[sighs]` …), Takt im Satz
  (`[pause]`, `[short pause]`, `[softly]`, `[slowly]`, `[rising tension]` …). Die Spalte **Regie** bleibt als Kurzhinweis zur Absicht stehen; maßgeblich ist der Text mit Tags.
- **Abweichungen zwischen Ton und Spieltext** werden nach der Lieferung von uns behoben, indem die Spieltexte an den gesprochenen Ton angepasst werden. Nachbearbeiten oder Schneiden ist nicht nötig.
- **Teil 2** (nicht im Manifest) ist ein Vorschlagstext in deutscher Sprechfassung; englische Referenztexte entfallen.
- **Optionsmarker** wie **[Option A – …]** in Teil 2 sind Regie: vor der Generierung **entfernen** (sonst interpretiert v3 sie als Tag oder spricht sie).
- **Abspielmodus:** Jede Zeile ist ein eigenes Event mit einer Datei; die Engine spielt sie einmal ab (Erzähler/Dialog über die Sprecherwarteschlange mit Musik-Ducking, Pausen zwischen aufeinanderfolgenden Zeilen ergeben sich aus der Warteschlange).
  Eine Datei wird mit höchstens 300 ms Schwanzstille erwartet; ein Kürzen übernimmt die Integration.

---

# Teil 1 – Mission 1 „Die letzte Siedlung“ (`m01_last_settlement`, Zeitalter I)

**Szenario-Kontext:** Jahr 897 n.F., Spätwinter. Isolde Raventhorn erreicht Brennhollow in der Aschmark (ca. 30 Einwohner).
Der Bergfried steht noch; Bruder Hal Venn hält die Gemeinde zusammen. Nordöstlich liegen die Ruinen mit dem Hohlschrein von
Vael Dorn, bewacht vom Fahlen Vogt. Der Spieler baut die Siedlung auf (Gehöft, Kaserne, Wachturm), übersteht Nächte und
Angriffswellen der Hohlen, findet den Schrein und zerstört ihn.

## 1.1 Erzähler (`narrator`)

**Stimme:** `AC_narrator` · **Modell:** Eleven v3, Stability-Modus *Natural* (Robust, falls die Stimme zwischen Generierungen zu stark schwankt), Speed 0,95 · Manifest: +2 dB.
**Regie allgemein:** getragen, ehrfürchtig, leise Trauer; nie Trailer-Pathos. Eine Zeile = eine Generierung.

### narrator_m01_opening (Event `narrator.m01_opening`)
*Kontext:* Missionsstart, Kamera fährt über verbrannte Felder der Aschmark auf Brennhollow zu. Musik `main_menu`→`build_calm`.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/narrator/narrator_m01_opening_01.ogg` | [solemn] Jahr 897 nach dem Fall. [pause] Am Rand der Aschmark hält Brennhollow noch stand – dreißig Seelen, ein Bergfried, ein Feld. [slowly] Und jede Nacht kommt der Nebel näher. | ruhig, gewichtig; Pause nach dem ersten Satz; Gedankenstrich als Schwebe, Aufzählung mit Wärme und Hoffnungsschimmer; Schlusssatz düster, langsam, letzte Phrase leiser |

### narrator_m01_night_falls (Event `narrator.m01_night_falls`)
*Kontext:* Tag-Nacht-Wechsel; Ambience wechselt `ambience_day` → `ambience_night`. Kann in der Mission mehrfach auftreten.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/narrator/narrator_m01_night_falls_01.ogg` | [ominous] Die Sonne sank hinter die Aschhügel. [long pause] Aus dem Nordosten wehte ein Geruch nach kaltem Stein und alten Gräbern. | still, angespannt, unheilvoll; lange Pause nach „Aschhügel“ |

Der Text ist immer derselbe; die früheren drei Takes (warnend/still/unheilvoll) sind zu **einer** Datei zusammengefallen. Keine Textvarianten erfinden.

### narrator_m01_victory (Event `narrator.m01_victory`)
*Kontext:* Hohlschrein zerstört, Fahler Vogt besiegt. Musik `victory_sting`, danach ruhige Ambience. Folgt auf Hal Venns `m01_victory`.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/narrator/narrator_m01_victory_01.ogg` | [softly] So begann es: [pause] nicht mit einer Krone, [warmly] sondern mit einem Dorf, das sich weigerte zu sterben. | ruhig, Erleichterung; Doppelpunkt als kurze Pause, Satz bleibt offen; wachsende Wärme, würdevoll, Gewicht auf „weigerte zu sterben“, Ausklang |

### narrator_m01_defeat (Event `narrator.m01_defeat`)
*Kontext:* Bergfried zerstört oder Isolde gefallen. Musik `defeat_sting`.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/narrator/narrator_m01_defeat_01.ogg` | [somber] Brennhollow ist gefallen. [slowly] Asche bedeckt die Mark. [pause] [encouraging] Doch die Chronik ist noch nicht zu Ende. Erhebt Euch, Wächterin, und versucht es erneut. | schwer, langsam, tonlos, leiser werdend; nach der Pause Wendung zur Ermutigung |

Hinweis: Die ersten beiden Sätze sind wörtlich `defeat_text` der Mission (Ergebnisbildschirm). Der Zusatz „Doch die Chronik ist noch nicht zu Ende. Erhebt Euch, Wächterin, und versucht es erneut.“
hat keine Spieltext-Entsprechung; er bleibt Teil der Datei und wird nach der Lieferung im Spieltext nachgezogen (`verify_spoken_text.py` führt ihn als erklärten Zusatz).

## 1.2 Bruder Hal Venn (`char_hal_venn`)

**Stimme:** `AC_char_hal_venn` · **Modell:** Eleven v3, *Natural*, Speed 0,97 · Manifest: +1 dB.
**Regie allgemein:** väterlicher Mentor, warm, geerdet; in Gefahr klar, nie panisch. Spricht Isolde mit Vornamen an (vertrautes „du“ im DE).

### char_hal_venn_m01_intro (Event `char_hal_venn.m01_intro`)
*Kontext:* direkt nach `narrator_m01_opening`. Isolde trifft am Bergfried ein; Hal kommt ihr entgegen. Erste Aufgabe: zweites Gehöft und Holz (`obj_farm`).

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_intro_01.ogg` | [concerned] Wächterin, die Speicher reichen nicht über den Winter. [firmly] Wir brauchen ein zweites Gehöft und Holz aus dem Westwald – [urgently] bevor es dunkel wird. | sorgenvoll, nüchtern, direkt zur Heldin; Tutorial-Hinweis tatkräftig; Satz offen bis zum Schluss, eindringlich, leiser werdend, ein Hauch Angst |

### char_hal_venn_m01_first_farm (Event `char_hal_venn.m01_first_farm`)
*Kontext:* zweites Gehöft (`farmstead` ×2) fertiggestellt; nächstes Ziel: Kaserne.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_first_farm_01.ogg` | [warmly] Die Felder werden uns ernähren. [seriously] Doch Brot allein hält die Toten nicht auf. [firmly] Errichtet eine Kaserne. | ehrliche Erleichterung, Ton kippt von Wärme zu Ernst, Tutorial-Hinweis (Kaserne) bestimmt und klar |

### char_hal_venn_m01_barracks (Event `char_hal_venn.m01_barracks`)
*Kontext:* Kaserne (`barracks`) fertiggestellt; Ausbildung von Schwertkämpfern, Speerträgern, Bogenschützen möglich.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_barracks_01.ogg` | [sighs] Die Kaserne steht. [firmly] Bildet Speerträger und Bogenschützen aus – [grimly] die Hohlen kennen keine Furcht, aber sie kennen Stahl. | bedrückt, kurzer Seufzer vor dem Satz; Tutorial-Hinweis bestimmt; grimmige Zuversicht, Gewicht auf „Stahl“ |

### char_hal_venn_m01_wave_warning (Event `char_hal_venn.m01_wave_warning`)
*Kontext:* erste Angriffswelle der Hohlen über die Brücke (Auslöser `wave_warning`, 215 s). Wird von `sfx/world/horn_alarm_*` begleitet.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_wave_warning_01.ogg` | [urgently] Hört Ihr das Horn? [shouting] Sie kommen über die Brücke! Alle Mann an den Bergfried! | erst lauschend, dringlich („Horn?“), dann laut und gehetzt; nicht hysterisch, Rest-Autorität, Schlusssatz Befehlston |

Der Text ist immer derselbe; die früheren vier Takes sind zu **einer** Datei zusammengefallen. Keine Textvarianten erfinden.

### char_hal_venn_m01_shrine_found (Event `char_hal_venn.m01_shrine_found`)
*Kontext:* Einheiten entdecken den Hohlschrein (`hollow_shrine`) mit Knochentürmen (`bone_spire`). Musik wechselt Richtung `build_tension`.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_shrine_found_01.ogg` | [whispers] Bei der Flamme … [pause] eine Aureth-Kapelle, und der Schleier darin ist offen. [firmly] Zerstört den Schrein, Wächterin. [quietly] Und hütet Euch vor dem, der ihn bewacht. | Ehrfurcht, Furcht, leise; „…“ als Atempause; Missionsziel klar, entschlossen; Schluss warnend, gedämpft, letzte Worte schwer |

### char_hal_venn_m01_victory (Event `char_hal_venn.m01_victory`)
*Kontext:* Schrein zerstört, Vogt gefallen; vor `narrator_m01_victory`.

| Ziel-Datei | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|
| `voices/characters/char_hal_venn_m01_victory_01.ogg` | [exhales] Der Riss ist verschlossen. [warmly] Brennhollow wird diesen Winter überstehen – [softly] dank Euch, Wächterin. | Erleichterung, fast Tränen; warm, stolz; am Ende ehrliche Dankbarkeit, ruhig, Ausblick auf Zeitalter II |

---

# Teil 2 – Kampagnen-Dialogszenen Zeitalter I–VII (spätere Phasen, nicht im Manifest v1)

**Konventionen (Vorschlag, vor Produktion vom Kanon-Owner bestätigen):**
- Zeilen-IDs: `aNN_<szene>_<NN>` (NN = Zeitalter 01–07). Beispiel: `voices/characters/char_king_aldric_a02_summons_01.ogg`.
- Erzähler: `voices/narrator/narrator_aNN_<szene>_<NN>.ogg`.
- Heldin in Dialogszenen: `voices/characters/hero_isolde_aNN_<szene>_<NN>.ogg` (Vorschlag, siehe Leitfaden Abschnitt 10).
- Bosse in Dialogszenen: `voices/characters/<boss_id>_aNN_<szene>_<NN>.ogg` (Vorschlag).
- Entscheidungs-Reaktionen: `_choice_a_<NN>` / `_choice_b_<NN>` anhängen.
- Laufende Nummern pro Sprecher und Szene; die Spalte **#** gibt die Reihenfolge der Szene an.
- Modell: alle Zeilen (Dialog, Bosse, Erzähler) mit Eleven v3 (*Natural*). Jede Zeile einzeln in einem Take generieren und unverändert liefern; kein Kontextsatz, der später weggeschnitten werden müsste.

## Zeitalter I – *The Last Settlement*: „Asche des Schreins“ (`a01_shrine_ash`)
*Kontext:* Nacht nach Mission 1. Isolde und Hal Venn stehen in den Trümmern des Hohlschreins. Im Zentrum pulsiert ein Aschglas-Kern.
Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie. **Entscheidung:** Kern zerschlagen oder zur Erforschung bergen.

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_a01_shrine_ash_01.ogg` | Hal Venn | [worried] [quietly] Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde. | beunruhigt, leise |
| 2 | `voices/characters/hero_isolde_a01_shrine_ash_01.ogg` | Isolde | [worried] Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet. | angespannt, misstrauisch |
| 3 | `voices/characters/char_deep_voice_a01_shrine_ash_01.ogg` | Stimme aus dem Tief | [mockingly] [whispers] Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich. | zärtlich-spöttisch, Drei-Schichten-Effekt |
| 4 | `voices/characters/hero_isolde_a01_shrine_ash_02.ogg` | Isolde | [fearful] [whispers] Hal. Sag mir, dass du das gehört hast. | kaum hörbar, kalter Schreck |
| 5 | `voices/characters/char_hal_venn_a01_shrine_ash_02.ogg` | Hal Venn | [worried] Was gehört? Isolde, deine Hand zittert. Geh weg davon. | besorgt, väterlich |
| 6 | `voices/characters/char_deep_voice_a01_shrine_ash_02.ogg` | Stimme aus dem Tief | [seductively] [slowly] Er kann mich nicht hören. [short pause] Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich. | langsam, intim, Bezug zu Isoldes Narbe |
| 7 | `voices/characters/char_hal_venn_a01_shrine_ash_03.ogg` | Hal Venn | [seriously] Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. [short pause] Oder es ist das Letzte, was wir je erforschen. Deine Entscheidung, Wächterin. | ernst, Entscheidung einleiten |
| 8A | `voices/characters/hero_isolde_a01_shrine_ash_choice_a_01.ogg` | Isolde | [firmly] Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es. | Option A – zerschlagen; hart, entschlossen |
| 9A | `voices/characters/char_deep_voice_a01_shrine_ash_choice_a_01.ogg` | Stimme aus dem Tief | [eerily] [laughs] Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir. | leises, hohles Lachen |
| 10A | `voices/characters/char_hal_venn_a01_shrine_ash_choice_a_01.ogg` | Hal Venn | [relieved] [exhales] Gut. Manches Wissen bleibt besser in der Asche. | erleichtert |
| 8B | `voices/characters/hero_isolde_a01_shrine_ash_choice_b_01.ogg` | Isolde | [nervously] Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren. | Option B – bergen; rational, aber zögerlich |
| 9B | `voices/characters/char_deep_voice_a01_shrine_ash_choice_b_01.ogg` | Stimme aus dem Tief | [seductively] Ja … trag mich heim. Halte mich nah an deinem Feuer. | zufrieden, verführerisch |
| 10B | `voices/characters/char_hal_venn_a01_shrine_ash_choice_b_01.ogg` | Hal Venn | [firmly] Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber. | widerwillig, entschlossen |

## Zeitalter II – *The Rising Crown*: „Der Ruf des Königs“ (`a02_summons`)
*Kontext:* Brennhollow ist gewachsen. Isolde wird an den Königshof gerufen. König Aldric III. braucht ihren Erfolg für seine
Legitimität; Herzogin Mireille Corvane bietet ihr Truppen gegen Gefolgschaft. Kassian Vale beobachtet. **Entscheidung:** dem König
den Treueid schwören oder Mireilles Bündnis annehmen.

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_a02_summons_01.ogg` | Aldric | [tired] Die Wächterin der Aschmark. [short pause] Der Hof spricht seit einem Monat von kaum etwas anderem. Manche nennen Euch eine Heldin. Manche ein Problem. | müde Würde, Thronsaal-Hall |
| 2 | `voices/characters/hero_isolde_a02_summons_01.ogg` | Isolde | [dryly] Meist beides, Majestät. Kommt darauf an, wer die Mauern bezahlt. | trocken, respektvoll |
| 3 | `voices/characters/char_mireille_a02_summons_01.ogg` | Mireille | [mockingly] [laughs softly] Charmant. Und ehrlich. Nordmark könnte beides gebrauchen, Wächterin. Die Krone, so scheint es, kann sich keins von beidem leisten. | spöttisch-elegant |
| 4 | `voices/characters/char_king_aldric_a02_summons_02.ogg` | Aldric | [angry] Ihr vergesst Euch, Herzogin. Ihr steht in meiner Halle. | aufflackernder Zorn, Stimme bricht leicht |
| 5 | `voices/characters/char_mireille_a02_summons_02.ogg` | Mireille | [menacingly] Ich stehe in einer Halle mit undichtem Dach und leerer Schatzkammer, Majestät. [short pause] Wächterin, ich biete fünfhundert Speere und Korn für ein Jahr. Ich verlange nur, dass Ihr Euch erinnert, wer sie geschickt hat. | ruhig, präzise, Drohung im Lächeln |
| 6 | `voices/characters/char_kassian_a02_summons_01.ogg` | Kassian | [amused] [whispers] Ein Rat, Wächterin. Welche Hand Ihr heute Nacht auch nehmt – [pause] die andere hält morgen ein Messer. | beiläufig, nah, amüsiert |
| 7 | `voices/characters/char_king_aldric_a02_summons_03.ogg` | Aldric | [calmly] Kniet nieder, Isolde Raventhorn. Euer Vater kniete einst vor mir. Wird es seine Tochter tun? | fast flehend hinter der Würde |
| 8A | `voices/characters/hero_isolde_a02_summons_choice_a_01.ogg` | Isolde | [solemn] Mein Vater starb, als er Eure Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät. | Option A – Treueid; ernst, feierlich |
| 9A | `voices/characters/char_king_aldric_a02_summons_choice_a_01.ogg` | Aldric | [relieved] [exhales] Dann ist die Krone nicht so allein, wie man sagt. Erhebt Euch, Wächterin. | gerührt, erleichtert |
| 10A | `voices/characters/char_mireille_a02_summons_choice_a_01.ogg` | Mireille | [coldly] Wie loyal. Ich hoffe sehr, dass Loyalität Eure Leute diesen Winter satt macht. | eisig höflich |
| 8B | `voices/characters/hero_isolde_a02_summons_choice_b_01.ogg` | Isolde | [calmly] Meine Leute können keine Eide essen, Majestät. Herzogin, ich nehme Eure Speere an. | Option B – Mireilles Bündnis; bedauernd, pragmatisch |
| 9B | `voices/characters/char_mireille_a02_summons_choice_b_01.ogg` | Mireille | [cheerfully] Eine kluge Frau. Nordmark erinnert sich an seine Freunde, Wächterin. Und an seine Schuldner. | zufrieden, Unterton |
| 10B | `voices/characters/char_king_aldric_a02_summons_choice_b_01.ogg` | Aldric | [sad] [tired] So. Selbst die Aschmark wendet sich ab. Lasst mich allein. | gebrochen, leise |

## Zeitalter III – *The Broken Borders*: „Pass der Ahnen“ (`a03_ancestor_pass`)
*Kontext:* Die Hohlen brechen an den Nordgrenzen durch. Isolde braucht Skarnheims Eisen und Krieger. Torvald Eisenhand empfängt sie
an einem Gebirgspass vor den Gräbern seiner Ahnen. **Entscheidung:** Blutschwur (gemeinsamer Kampf, Ehre) oder Bezahlung in Gold.

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_a03_ancestor_pass_01.ogg` | Torvald | [thoughtfully] [shouting] Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen. | dröhnend, prüfend, Berghall |
| 2 | `voices/characters/hero_isolde_a03_ancestor_pass_01.ogg` | Isolde | [firmly] Lieder übertreiben. Eure machen Euch angeblich größer und weiser. | trocken, furchtlos |
| 3 | `voices/characters/char_torvald_a03_ancestor_pass_02.ogg` | Torvald | [sad] [laughs] Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den Hochpass. Wir haben vierzig der Unseren begraben. Vierzig. | dröhnendes Lachen, das in Trauer kippt |
| 4 | `voices/characters/hero_isolde_a03_ancestor_pass_02.ogg` | Isolde | [seriously] Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden. | ernst, Respekt |
| 5 | `voices/characters/char_torvald_a03_ancestor_pass_03.ogg` | Torvald | [solemn] Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold. Sippe zahlt in Blut. Was bist du? | schwer, rituell |
| 6 | `voices/characters/char_torvald_a03_ancestor_pass_04.ogg` | Torvald | [urgently] [quietly] Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig. | leiser, drängend |
| 7A | `voices/characters/hero_isolde_a03_ancestor_pass_choice_a_01.ogg` | Isolde | [sorrowfully] [grunts] Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite eures Clans. | Option A – Blutschwur; feierlich, kurzer Schmerzlaut `[exhales]` danach möglich |
| 8A | `voices/characters/char_torvald_a03_ancestor_pass_choice_a_01.ogg` | Torvald | [triumphantly] [shouting] Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met! | jubelnd, Gruppenjubel-Layer möglich |
| 7B | `voices/characters/hero_isolde_a03_ancestor_pass_choice_b_01.ogg` | Isolde | [seriously] Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort noch besser. Nenn deinen Preis. | Option B – Gold; sachlich, ehrlich |
| 8B | `voices/characters/char_torvald_a03_ancestor_pass_choice_b_01.ogg` | Torvald | [coldly] [sighs] Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben. | enttäuscht, kühl |

## Zeitalter IV – *The Age of Conquest*: „Das Register der Flut“ (`a04_tide_ledger`)
*Kontext:* Die Glutkrone (oder Mireilles Bündnis) will die Aurethwüste erreichen. Der Landweg ist lang; Ophira Saelith-Varo bietet
die Flotte des Saelith-Pakts. Kassian Vale warnt vor den Bedingungen. **Entscheidung:** Flotte annehmen (Schuld) oder Landmarsch.

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_a04_tide_ledger_01.ogg` | Ophira | [amused] Willkommen an Bord, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel. | amüsiert, Hafenakustik |
| 2 | `voices/characters/hero_isolde_a04_tide_ledger_01.ogg` | Isolde | [dryly] Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen. | trocken |
| 3 | `voices/characters/char_ophira_a04_tide_ledger_02.ogg` | Ophira | [gruffly] [laughs] Vierzig Schiffe, dreitausend Söldner und die Küste Aureths in neun Tagen statt neunzig. [short pause] Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus diesen Ruinen grabt. | kehliges Lachen, dann geschäftlich |
| 4 | `voices/characters/char_kassian_a04_tide_ledger_01.ogg` | Kassian | [calmly] Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf. | nah, ruhig |
| 5 | `voices/characters/char_ophira_a04_tide_ledger_03.ogg` | Ophira | [seductively] Meister Vale. Noch am Leben? Wie enttäuschend für so viele Leute. | zuckersüß, Spitze |
| 6 | `voices/characters/char_kassian_a04_tide_ledger_02.ogg` | Kassian | [amused] Nur für die, die Euch Geld schulden, Ophira. | amüsiert |
| 7 | `voices/characters/char_ophira_a04_tide_ledger_04.ogg` | Ophira | [coldly] Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft. | plötzlich kühl |
| 8A | `voices/characters/hero_isolde_a04_tide_ledger_choice_a_01.ogg` | Isolde | [firmly] Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf. | Option A – Flotte; entschlossen, Bedauern |
| 9A | `voices/characters/char_ophira_a04_tide_ledger_choice_a_01.ogg` | Ophira | [menacingly] Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer. | warm mit Drohung |
| 8B | `voices/characters/hero_isolde_a04_tide_ledger_choice_b_01.ogg` | Isolde | [proudly] Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch. | Option B – Landmarsch; trocken, stolz |
| 9B | `voices/characters/char_ophira_a04_tide_ledger_choice_b_01.ogg` | Ophira | [amused] [laughs softly] Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin. | ehrlich amüsiert, Respekt |

## Zeitalter V – *The Divided Throne*: „Nacht der Messer“ (`a05_divided_throne`)
*Kontext:* König Aldric liegt im Sterben; Mireille marschiert mit Nordmarks Heer auf den Königshof. Kassian enthüllt, dass Anhänger der
Sonnenhäresie den Hof unterwandert haben und die Hohlen mit Nachrichten versorgen. **Entscheidung:** Mireilles Anspruch stützen
(starke Hand, Einheit) oder Aldrics Linie verteidigen (Legitimität, Bürgerkrieg).

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_a05_divided_throne_01.ogg` | Aldric | [coldly] [tired] Isolde. Kommt näher. Meine Augen … [pause] sind nicht mehr, was sie waren. | sterbend, flach, Atempausen |
| 2 | `voices/characters/hero_isolde_a05_divided_throne_01.ogg` | Isolde | [gently] Ich bin hier, Majestät. Ruht Euch aus. | sanft, ungewohnt weich |
| 3 | `voices/characters/char_king_aldric_a05_divided_throne_02.ogg` | Aldric | [fearful] Mireille steht vor den Toren. [short pause] Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er könne dasselbe. Valemark wird sich selbst zerreißen. | Angst, letzte Kraft |
| 4 | `voices/characters/char_kassian_a05_divided_throne_01.ogg` | Kassian | [eerily] Verzeiht die Störung. Die Herzogin ist nicht Euer einziges Problem. Drei der Räte des Königs beten zur Sonnenhäresie. Sie haben unsere Truppenbewegungen nach Süden geschickt. An die Hohlen. | ruhig, Gewicht auf „An die Hohlen“ |
| 5 | `voices/characters/hero_isolde_a05_divided_throne_02.ogg` | Isolde | [angry] Und das sagt Ihr mir jetzt? | kalte Wut |
| 6 | `voices/characters/char_kassian_a05_divided_throne_02.ogg` | Kassian | [moved] Ich sage jedem alles genau im richtigen Moment, Wächterin. Das ist er. | ungerührt |
| 7 | `voices/characters/char_mireille_a05_divided_throne_01.ogg` | Mireille | [calmly] [shouting] Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich kam, ein sterbendes Königreich zu retten! | von draußen, Außenhall, laut aber beherrscht |
| 8A | `voices/characters/hero_isolde_a05_divided_throne_choice_a_01.ogg` | Isolde | [seriously] Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere. | Option A – Mireille stützen; schwer, Entscheidung mit Schuld |
| 9A | `voices/characters/char_mireille_a05_divided_throne_choice_a_01.ogg` | Mireille | [seriously] Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit ist. | ernsthaft, erstmals ohne Spott |
| 10A | `voices/characters/char_king_aldric_a05_divided_throne_choice_a_01.ogg` | Aldric | [quietly] [exhales] So sei es … [pause] Haltet … die Feuer am Brennen … | letzte Worte, verhauchend |
| 8B | `voices/characters/hero_isolde_a05_divided_throne_choice_b_01.ogg` | Isolde | [commanding] Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern. | Option B – Aldrics Linie; hart, Befehlston |
| 9B | `voices/characters/char_mireille_a05_divided_throne_choice_b_01.ogg` | Mireille | [coldly] Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin. | eiskalt |
| 10B | `voices/characters/char_king_aldric_a05_divided_throne_choice_b_01.ogg` | Aldric | [moved] [quietly] Euer Vater … wäre stolz. Es tut mir … [pause] leid, dass es so viel kostet. | dankbar, schwach |

## Zeitalter VI – *The Fallen Gods*: „Der graue Traum“ (`a06_grey_dream`)
*Kontext:* Die alten Götter schweigen seit dem Aschenfall. Im Dornwald greift die Hohlung nach der Wurzelmutter. Sylaen führt Isolde zu
ihr. **Entscheidung:** die verdorbenen Wurzeln verbrennen (Wurzelmutter stirbt, Wald überlebt verwundet) oder in den Traum eintreten,
um sie zu heilen (Gefahr, dass Isolde der Stimme aus dem Tief begegnet).

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_a06_grey_dream_01.ogg` | Sylaen | [solemn] [quietly] Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. | gedämpft, ehrfürchtig, Waldhall |
| 2 | `voices/characters/hero_isolde_a06_grey_dream_01.ogg` | Isolde | [calmly] Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. | Erkennen, Unbehagen |
| 3 | `voices/characters/boss_root_mother_a06_grey_dream_01.ogg` | Wurzelmutter | [eerily] Kinder … so viele Kinder mit Feuer in den Händen … [pause] seid ihr hier, um mich zu halten, oder um mich zu verbrennen? | uralt, verwirrt, Chor-Dopplung |
| 4 | `voices/characters/char_sylaen_a06_grey_dream_02.ogg` | Sylaen | [fearful] [whispers] Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe. | flüsternd, Angst |
| 5 | `voices/characters/char_deep_voice_a06_grey_dream_01.ogg` | Stimme aus dem Tief | [greedily] [slowly] Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … [pause] verdaut. Calyx schmeckte nach Frühling. | genüsslich, sehr langsam |
| 6 | `voices/characters/char_sylaen_a06_grey_dream_03.ogg` | Sylaen | [sad] Es gibt zwei Wege, und ich hasse beide. [short pause] Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken. | gequält, ehrlich |
| 7A | `voices/characters/hero_isolde_a06_grey_dream_choice_a_01.ogg` | Isolde | [seriously] [quietly] Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln. | Option A – verbrennen; schwer, leise |
| 8A | `voices/characters/boss_root_mother_a06_grey_dream_choice_a_01.ogg` | Wurzelmutter | [gently] [quietly] Warm … so warm … [pause] schlaft gut, Kinder … | verlöschend, friedlich |
| 9A | `voices/characters/char_sylaen_a06_grey_dream_choice_a_01.ogg` | Sylaen | [sad] Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch. | Trauer, eisig |
| 7B | `voices/characters/hero_isolde_a06_grey_dream_choice_b_01.ogg` | Isolde | [fearful] Keine Feuer mehr. Zeig mir den Weg hinein. | Option B – Traum betreten; entschlossen, mit Angst |
| 8B | `voices/characters/char_deep_voice_a06_grey_dream_choice_b_01.ogg` | Stimme aus dem Tief | [triumphantly] [whispers] Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen. | intim, triumphierend |
| 9B | `voices/characters/char_sylaen_a06_grey_dream_choice_b_01.ogg` | Sylaen | [urgently] Halt meine Hand und lass nicht los. Was immer du dort drin hörst … [pause] es ist nicht dein Freund. | eindringlich, nah |

### Zusatzszene Zeitalter VI: „Der Gipfel der Glut“ (`a06_cinder_peak`)
*Kontext:* Isolde sucht den Aschdrachen Vyrmathax in der Glutweite auf, der als einziges Wesen den Aschenfall bewusst erlebt hat.

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_01.ogg` | Vyrmathax | [calmly] Wieder klettert eine kleine Krone auf meinen Berg. Wie tapfer. Wie kurz. | gelangweilt-erhaben, Höhlenhall |
| 2 | `voices/characters/hero_isolde_a06_cinder_peak_01.ogg` | Isolde | [fearful] Du hast Aureth fallen sehen. Ich muss wissen, wie Varkhan den Schleier öffnete. | fest, trotz Furcht |
| 3 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_02.ogg` | Vyrmathax | [laughs] Er hat ihn nicht geöffnet, Sterbliche. Er hat angeklopft. Etwas anderes hat die Tür geöffnet. | grollendes Lachen |
| 4 | `voices/characters/hero_isolde_a06_cinder_peak_02.ogg` | Isolde | [urgently] Dann sag mir, wie man sie schließt. | drängend |
| 5 | `voices/characters/boss_ashen_dragon_a06_cinder_peak_03.ogg` | Vyrmathax | [thoughtfully] [slowly] Eine Tür schließt man von beiden Seiten, kleine Wächterin. [short pause] Jemand muss auf der anderen Seite stehen. Bist du so tapfer? Oder nur so kurz? | langsam, prüfend |
| 6 | `voices/characters/hero_isolde_a06_cinder_peak_03.ogg` | Isolde | [fearful] [exhales] Ich schätze, das werden wir herausfinden. | trocken, Angst darunter |

## Zeitalter VII – *The Last Crown*: „Solmere“ (`a07_last_crown`)
*Kontext:* Finale in der Festung Solmere. Der Hohle König Varkhan Aurelith sitzt auf dem Sonnenthron; hinter ihm klafft der Schleier.
Hal Venn begleitet Isolde. **Entscheidung:** den Schleier von innen versiegeln (Isolde opfert ihre Glut/sich selbst) oder die Hohle Krone
nehmen, um die Toten zu befehligen (Macht, die Stimme aus dem Tief bleibt).

| # | Ziel-Datei | Sprecher | Sprechtext (DE) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_hollow_king_a07_last_crown_01.ogg` | Hohler König | [calmly] Ich war die Sonne Aureths. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal. | majestätisch-brüchig, Thronsaal-Hall |
| 2 | `voices/characters/hero_isolde_a07_last_crown_01.ogg` | Isolde | [coldly] Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken. | ruhig, kalt |
| 3 | `voices/characters/boss_hollow_king_a07_last_crown_02.ogg` | Hohler König | Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. [whispers] Es hat mir geantwortet … [pause] und es hat gelogen. | Tragik, Wahnsinn bricht durch |
| 4 | `voices/characters/char_deep_voice_a07_last_crown_01.ogg` | Stimme aus dem Tief | [gently] Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist nie wieder jemand gestorben. | sanft, grausam |
| 5 | `voices/characters/char_hal_venn_a07_last_crown_01.ogg` | Hal Venn | [fearful] Isolde, hinter dem Thron ist der Schleier offen. Was immer du tun willst, tu es jetzt. | dringlich, Angst um sie |
| 6 | `voices/characters/boss_hollow_king_a07_last_crown_03.ogg` | Hohler König | [tired] Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … [pause] jemand muss die Asche tragen. | müde, beinahe erlöst |
| 7 | `voices/characters/char_deep_voice_a07_last_crown_02.ogg` | Stimme aus dem Tief | [coldly] Wähle, kleine Flamme. Ich bin so oder so hier. | geduldig, gleichgültig |
| 8A | `voices/characters/hero_isolde_a07_last_crown_choice_a_01.ogg` | Isolde | [calmly] Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten. | Option A – Schleier versiegeln; Abschied, gefasst, kurzes `[exhales]` |
| 9A | `voices/characters/char_hal_venn_a07_last_crown_choice_a_01.ogg` | Hal Venn | [sad] [crying] Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich. | brechende Stimme, Tränen |
| 10A | `voices/characters/char_deep_voice_a07_last_crown_choice_a_01.ogg` | Stimme aus dem Tief | [calmly] Tapfer. Kurz. Und nun … [pause] mein. Willkommen daheim. | ruhig, endgültig |
| 11A | `voices/narrator/narrator_a07_last_crown_choice_a_01.ogg` | Erzähler | [moved] Und so wurde der Schleier geschlossen. [short pause] Niemand krönte die Wächterin der Aschmark. Doch in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen. | Epilog, ruhig, bewegt (v2) |
| 8B | `voices/characters/hero_isolde_a07_last_crown_choice_b_01.ogg` | Isolde | [firmly] Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden. | Option B – Hohle Krone; hart, Stimme bereits leicht verändert (dezenter Deep-Voice-Layer 10 %) |
| 9B | `voices/characters/boss_hollow_king_a07_last_crown_choice_b_01.ogg` | Hohler König | [sad] [laughs] Genau das habe ich auch gesagt. | hohles, trauriges Lachen, zerfallend |
| 10B | `voices/characters/char_hal_venn_a07_last_crown_choice_b_01.ogg` | Hal Venn | [fearful] [quietly] Isolde … deine Augen. Die Flamme vergebe uns beiden. | entsetzt, leise |
| 11B | `voices/narrator/narrator_a07_last_crown_choice_b_01.ogg` | Erzähler | [menacingly] [quietly] Und so wurde die letzte Krone Eldraeths aus Asche geschmiedet. [short pause] Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte je wieder. Doch nachts, in den Hallen Solmeres, sprach die Königin bisweilen mit jemandem, den niemand sonst hören konnte. | Epilog, ambivalent, unheimlich leise am Ende (v2) |

---

<!-- BEGIN KAMPAGNE (erzeugt von `ashen_tools forge-campaign`, nicht von Hand ändern) -->
# Teil 3 – Echtzeit-Kampagne „Das Zeitalter der Gefallenen“ (Missionsdialoge, im Manifest)

Dieser Teil wird aus den Missionsdaten erzeugt (`cargo run -p ashen_tools -- forge-campaign`): Jede Zeile steht
identisch in der Mission (`assets/data/campaign/*.ron`), als Event im Manifest und hier. **Sprechtext = exakt der deutsche Spieltext**,
Die Audio-Tags im Sprechtext sind von Hand ergänzt und bleiben beim Neuerzeugen erhalten, solange der Text ohne Tags dem Spieltext entspricht (sonst steht der Spieltext ohne Tags da und muss neu getaggt werden).

**Konventionen**
- Event: `<sprecher-id>.<mission>_<szene>_<buchstabe>`; Datei: `voices/narrator/narrator_<mission>_<szene>_<buchstabe>_01.ogg` bzw. `voices/characters/<sprecher-id>_<mission>_<szene>_<buchstabe>_01.ogg`.
- Jede Zeile ein eigenes Event mit genau einer Datei (`_01`); die Reihenfolge der Zeilen steht in der Spalte **#**. Das Spiel spielt die Zeilen einer Szene nacheinander; die Pausen ergeben sich aus der Textlänge.
- Stimme: Profil `AC_<sprecher-id>` aus `elevenlabs_characters.md` (Sprache **Deutsch**, alle Stimmen inklusive Erzähler mit Eleven v3, Stability-Modus *Natural*; jede Zeile enthält Eleven-v3-Audio-Tags in eckigen Klammern, die nicht gesprochen werden und nicht in den Spieltext gehören). Akzentangaben der Profile gelten als leichte Färbung.
- Zielformat und Loudness wie in Teil 1 (Ogg Vorbis q6, 48 kHz, mono, −16 LUFS, −1 dBTP). Manifest: Erzähler +2 dB, Figuren +1 dB.
- Neue Sprecher dieses Teils: `char_wendel`, `char_nel`, `boss_stone_oath`, `boss_reef_mistress` (Profile in `elevenlabs_characters.md`, Teil F).

## Mission 2 – Die Flüchtlinge von Harrowmere (`m02`)

### `m02_kern`
*Kontext:* Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_kern_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [worried] [quietly] Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde. | beunruhigt, leise |
| 2 | `voices/characters/hero_isolde_m02_kern_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [worried] Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet. | angespannt, misstrauisch |
| 3 | `voices/characters/char_deep_voice_m02_kern_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [mockingly] Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich. | zärtlich-spöttisch, mehrschichtiger Effekt (Profil Deep Voice) |
| 4 | `voices/characters/hero_isolde_m02_kern_d_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [fearful] [whispers] Hal. Sag mir, dass du das gehört hast. | kaum hörbar, kalter Schreck |
| 5 | `voices/characters/char_hal_venn_m02_kern_e_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [worried] Was gehört? Isolde, deine Hand zittert. Geh weg davon. | besorgt, väterlich |
| 6 | `voices/characters/char_deep_voice_m02_kern_f_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [menacingly] [quietly] Er kann mich nicht hören. [short pause] Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich. | leise, vertraut, bedrohlich |
| 7 | `voices/characters/char_hal_venn_m02_kern_g_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [warmly] Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. [short pause] Oder es ist das Letzte, was wir je untersuchen. Deine Wahl, Wächterin. | ernst, väterlich |

### `m02_kern_a`
*Kontext:* Reaktion auf die Entscheidung: Kern zerschlagen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_kern_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es. | hart, entschlossen |
| 2 | `voices/characters/char_deep_voice_m02_kern_a_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [eerily] [laughs] Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir. | leises, hohles Lachen |
| 3 | `voices/characters/char_hal_venn_m02_kern_a_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [relieved] Gut. Manches Wissen bleibt besser in der Asche. | erleichtert |

### `m02_kern_b`
*Kontext:* Reaktion auf die Entscheidung: Kern bergen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_kern_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren. | ruhig, entschieden |
| 2 | `voices/characters/char_deep_voice_m02_kern_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [seductively] Ja … trag mich heim. Halte mich nah an deinem Feuer. | zufrieden, verführerisch |
| 3 | `voices/characters/char_hal_venn_m02_kern_b_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [firmly] Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber. | beklommen, entschlossen |

### `m02_aufbruch`
*Kontext:* Beginn der Eskorte, Ascheregen setzt ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m02_aufbruch_a_01.ogg` | Erzähler (`AC_narrator`) | [sad] Drei Tage nach dem Fall des Schreins kam ein Reiter aus dem Süden. Harrowmere, das Dorf im Aschmoor, brenne. Wer noch gehen könne, warte am Brunnen – [pause] auf ein Wunder oder auf jemanden, der keines verspreche. | getragen, Trauer unter der Ruhe |
| 2 | `voices/characters/hero_isolde_m02_aufbruch_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Wir führen sie nach Brennhollow. [short pause] Kein Wagen bleibt zurück, kein Kind. Bleibt auf der Straße, und wenn der Ascheregen fällt, bleibt zusammen. | bestimmt, trocken |

### `m02_furt`
*Kontext:* Der Zug erreicht die Furt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_furt_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [quietly] Das Wasser reicht bis zu den Knien. Tragt die Kinder, und redet nicht – [pause] das Moor hört zu. | gedämpft, konzentriert |

### `m02_hinterhalt`
*Kontext:* Hinterhalt im Röhricht vor der Furt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m02_hinterhalt_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [urgently] Im Röhricht, links! Schließt die Reihen, die Schwachen in die Mitte! | scharf, alarmiert |

### `m02_ankunft`
*Kontext:* Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_ankunft_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [relieved] [shouting] Die Tore auf! Öffnet die Tore – [pause] Wasser, Decken, Brot! Sie leben, Isolde. Sie leben. | warm, erleichtert, rufend |
| 2 | `voices/characters/char_nel_m02_ankunft_b_01.ogg` | Nel (`AC_char_nel`) | [menacingly] Der Mann im Moor hat gesagt, ich soll dir ausrichten: Es war nett, dich kennenzulernen. | kindlich, monoton, unheimlich |
| 3 | `voices/characters/hero_isolde_m02_ankunft_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [worried] Welcher Mann, Nel? | behutsam, aber angespannt |
| 4 | `voices/characters/char_hal_venn_m02_ankunft_d_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [gently] Das Kind hört etwas, das uns nicht hört. Wir können es bei uns behalten und wachen – [pause] oder es den Flammenschwestern in Kaldris anvertrauen. | behutsam, ernst |

### `m02_nel_a`
*Kontext:* Entscheidung: Nel bleibt in Brennhollow.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_nel_a_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [firmly] Dann bleibt sie bei uns, und ich werde ihre Träume bewachen. Die Flamme verzeiht eher Mut als Furcht. | sanft, entschlossen |
| 2 | `voices/characters/char_nel_m02_nel_a_b_01.ogg` | Nel (`AC_char_nel`) | [quietly] Danke. Hier ist es warm. | leise, kindlich |

### `m02_nel_b`
*Kontext:* Entscheidung: Nel geht zu den Flammenschwestern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m02_nel_b_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [sad] Die Flammenschwestern in Kaldris kennen Kinder wie sie. Es ist vernünftig – [pause] und es bricht mir das Herz. | bedrückt, ehrlich |
| 2 | `voices/characters/char_nel_m02_nel_b_b_01.ogg` | Nel (`AC_char_nel`) | [hopefully] Kommst du mich besuchen? | klein, hoffnungsvoll |

### `m02_sieg`
*Kontext:* Schlusswort der Mission.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m02_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [warmly] [quietly] Sechzehn Menschen mehr in Brennhollow, und kein Vorrat für sie. [short pause] Doch an diesem Abend zählte niemand nach: Es brannten zwei Feuer mehr auf dem Marktplatz als am Morgen. | warm, leise Hoffnung |

## Mission 3 – Die Graue Straße (`m03`)

### `m03_aufbruch`
*Kontext:* Start am Südende der Grauen Straße.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m03_aufbruch_a_01.ogg` | Erzähler (`AC_narrator`) | [menacingly] Die Graue Straße führte einmal von Brennhollow bis zu den Salzmärkten Valemarks. Nun führte sie vor allem zu Gräbern am Straßenrand – [pause] und an ihrem Ende saß ein Ritter, der mit seinen Opfern Buch führte. | trocken, düster |
| 2 | `voices/characters/hero_isolde_m03_aufbruch_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Neun Leute für eine Straße von dreißig Meilen. Das ist keine Armee, das ist ein Spaziergang mit Absicht. | trocken, entschlossen |

### `m03_briefe`
*Kontext:* Isolde findet im zweiten Posten die Briefe mit dem Siegel von Nordmark.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_briefe_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Briefe. Das Wachs trägt das Siegel von Nordmark – [pause] und die Summe reicht, um eine Kompanie ein Jahr lang zu bezahlen. | nachdenklich, kühl |
| 2 | `voices/characters/hero_isolde_m03_briefe_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] [quietly] Jemand bezahlt die Räuber dafür, dass Brennhollow hungert. Das merke ich mir. | leise, hart |

### `m03_wendel`
*Kontext:* Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_wendel_m03_wendel_a_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | [tired] Haltet ein, Wächterin! Ich lege die Klinge nieder – [pause] ich sehe, wann ein Spiel verloren ist. Und ich hasse es, für Verlierer zu arbeiten. | heiser, spöttisch, erschöpft |
| 2 | `voices/characters/hero_isolde_m03_wendel_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Du hast Fuhrleute erschlagen, Wendel. Ich habe die Karren gesehen. | kalt |
| 3 | `voices/characters/char_wendel_m03_wendel_c_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | [tired] Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. Aber ich kenne jeden Pfad zwischen hier und Kaldris – [pause] und meine Männer können kämpfen. | ehrlich, müde |
| 4 | `voices/narrator/narrator_m03_wendel_d_01.ogg` | Erzähler (`AC_narrator`) | [thoughtfully] Ein Wegelagerer, der um sein Leben handelt. Die Flamme verlangt Gerechtigkeit – [pause] aber Brennhollow braucht Schwerter. Die Entscheidung liegt bei der Wächterin. | ruhig, abwägend |

### `m03_wendel_a`
*Kontext:* Entscheidung: Wendel wird gehängt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_wendel_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Du hängst, Wendel. Für jeden Karren, den du verbrannt hast. Die Straße braucht keine Reue, sie braucht Ruhe. | hart, endgültig |
| 2 | `voices/characters/char_wendel_m03_wendel_a_b_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | [calmly] [quietly] Gerecht. Fast. Sagt den Fuhrleuten … [pause] dass ich sie gezählt habe. | gefasst, leise |

### `m03_wendel_b`
*Kontext:* Entscheidung: Wendel und seine Männer werden in Dienst genommen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m03_wendel_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [seriously] Du dienst mir, bis die Straße sicher ist. Wenn du uns verrätst, hole ich dich persönlich. | nüchtern, ohne Drohgebärde |
| 2 | `voices/characters/char_wendel_m03_wendel_b_b_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | [relieved] Die Graue Kompanie steht Euch zu Diensten, Wächterin. Es ist das erste Mal seit Jahren, dass ich bei Tag schlafen kann. | erleichtert, trocken |

### `m03_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m03_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Am Abend zogen die ersten Karren seit zwei Jahren nordwärts. Die Aschmark war immer noch ein Grenzland – [pause] aber nun eines mit einer Straße. | ruhig, Anerkennung |

## Mission 4 – Die Lange Nacht (`m04`)

### `m04_nacht`
*Kontext:* Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m04_nacht_a_01.ogg` | Erzähler (`AC_narrator`) | [seriously] Die Hohlen kamen nicht mehr in Wellen. [short pause] In dieser Nacht kamen sie wie eine Flut, die einen Deich gefunden hat. Auf den Zinnen von Brennhollow zählten die Wächter ihre Fackeln und sagten nichts. | schwer, ruhig |
| 2 | `voices/characters/char_hal_venn_m04_nacht_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [tired] Der Gesandte des Königs soll bei Sonnenaufgang eintreffen, Isolde. Wenn er uns lebend findet, haben wir genug erreicht. | trocken, müde, väterlich |
| 3 | `voices/characters/hero_isolde_m04_nacht_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Dann sorgen wir dafür, dass er uns lebend findet. Bogenschützen auf die Türme, Schwertträger ans Tor! | bestimmt |

### `m04_mitte`
*Kontext:* Die Grabkolosse erreichen die Mauern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m04_mitte_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [urgently] [shouting] Sie bringen die Kolosse! Haltet die Tore, Isolde – [pause] ich singe die Mauer fest, so lange meine Stimme trägt! | rufend, eindringlich |

### `m04_morgen`
*Kontext:* Morgengrauen. Der Gesandte des Königs, Kassian Vale, trifft am Südtor ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m04_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Der Himmel über der Aschmark wurde grau, dann fahl, dann golden an den Rändern. [short pause] Die Letzten der Hohlen sanken in die Erde zurück, als hätten sie nur geträumt, lebendig zu sein. | getragen, Erleichterung |
| 2 | `voices/characters/char_kassian_m04_morgen_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [amused] Eine Nacht, die man in Kaldris noch in Jahren erzählen wird – [pause] leider ohne Wahrheit. Ich bin Kassian Vale. Seine Majestät bittet Euch, Wächterin, um Euren Treueeid. Und bevor Ihr antwortet: Herzogin Mireille Corvane bietet Euch fünfhundert Speere und Korn für ein Jahr. | geschmeidig, höflich, leicht amüsiert |
| 3 | `voices/characters/char_kassian_m04_morgen_c_01.ogg` | Kassian Vale (`AC_char_kassian`) | [urgently] [quietly] Zwei Hände strecken sich Euch entgegen, Wächterin. Die Krone fordert Euren Eid, Nordmark bietet Speere. Welche Hand Ihr heute nehmt – [pause] die andere hält morgen ein Messer. | leise, ruhig, eindringlich |

### `m04_eid_a`
*Kontext:* Entscheidung: Treueeid auf die Krone.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m04_eid_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Mein Vater starb, als er Valemarks Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät. | fest, förmlich |
| 2 | `voices/characters/char_kassian_m04_eid_a_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [calmly] Wie standhaft. Der König wird es zu schätzen wissen – [pause] und die Herzogin sich Euren Namen merken. | höflich, mit Unterton |

### `m04_eid_b`
*Kontext:* Entscheidung: Bündnis mit Herzogin Mireille Corvane.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m04_eid_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [dryly] Meine Leute können keine Eide essen. Richtet der Herzogin aus, dass ich ihre Speere annehme. | trocken, pragmatisch |
| 2 | `voices/characters/char_kassian_m04_eid_b_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [thoughtfully] Eine kluge Entscheidung – [pause] und eine sehr teure. Nordmark erinnert sich an Freunde, Wächterin. Und an Schuldner. | ruhig, vieldeutig |

## Mission 6 – Das Turnier von Kaldris (`m06`)

### `m06_turnier`
*Kontext:* Die Arena von Kaldris, Turniertag.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m06_turnier_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [quietly] Kaldris trug Gold und Weiß. [short pause] Über der Arena hingen die Banner der zwölf Herzogtümer, und auf der Tribüne saß ein alternder König, der aussah, als warte er auf etwas, das nicht der Sieger war. | festlich, mit leisem Unterton |
| 2 | `voices/characters/char_kassian_m06_turnier_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [whispers] Eine Beobachtung, Wächterin, ganz unverbindlich: Heute Abend wird der Balkon der Krone nicht jeden verlassen, der ihn betritt. [short pause] Ich überlasse es Euch, was Ihr mit diesem Wissen anfangt. | flüsternd, geschmeidig |

### `m06_runde2`
*Kontext:* Der König ruft aus der Loge zur zweiten Runde.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_m06_runde2_a_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [shouting] Die Wächterin der Aschmark! Man erzählt, sie schlage die Besten, ohne zu schwitzen. Zeigt es uns noch einmal! | laut, belebt, mit alter Stimme |

### `m06_glocke`
*Kontext:* Abendglocke: Die Attentäter am Balkon der Krone regen sich.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m06_glocke_a_01.ogg` | Erzähler (`AC_narrator`) | [worried] [quickly] Es war das Läuten der Abendglocke, das ihn rettete: ein falscher Ton, eine Sekunde zu früh. [short pause] Auf dem Balkon der Krone regte sich ein Schatten, dann zwei, dann drei. | gespannt, rasch |

### `m06_anschlag`
*Kontext:* Die Attentäter fliehen vom Balkon der Krone.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_anschlag_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [sharply] Dort oben! Die Reiter voran – [pause] holt sie ein, bevor sie die Gassen erreichen! | scharf |

### `m06_kassian_a`
*Kontext:* Entscheidung: Der König wird gewarnt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [seriously] [quietly] Majestät, ein Wort unter vier Augen: Es gab eine Warnung, und sie war nicht ohne Eigennutz. | ernst, leise |
| 2 | `voices/characters/char_king_aldric_m06_kassian_a_b_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [tired] Ihr habt mir das Leben gerettet und das Spiel verdorben. Ich weiß nicht, was mich mehr bewegt. | müde, dankbar |

### `m06_kassian_b`
*Kontext:* Entscheidung: Kassian wird in der Hand behalten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Kassian. Ihr habt mehr gewusst, als Ihr gesagt habt. Dafür schuldet Ihr mir etwas. | kühl |
| 2 | `voices/characters/char_kassian_m06_kassian_b_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [calmly] So ist es. Es ist angenehm, wenn jemand begreift, dass Wissen eine Währung ist. Ich werde zahlen, wenn die Stunde kommt. | mit feinem Lächeln |

### `m06_kassian_c`
*Kontext:* Entscheidung: Schweigen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m06_kassian_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Es war ein Anschlag, mehr nicht. Ich habe nichts gesehen, was zu erzählen wäre. | neutral, abweisend |
| 2 | `voices/characters/char_kassian_m06_kassian_c_b_01.ogg` | Kassian Vale (`AC_char_kassian`) | [dryly] Wie diskret. Ich bin beinahe enttäuscht – und beeindruckt. | trocken, flüchtig |

### `m06_ende`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_kassian_m06_ende_a_01.ogg` | Kassian Vale (`AC_char_kassian`) | [quietly] Nun wisst Ihr, dass ich es wusste. Was macht Ihr damit, Wächterin: Sagt Ihr es dem König, behaltet Ihr es als Pfand – [pause] oder schweigt Ihr? | leise, belustigt |

## Mission 7 – Eisen in den Hügeln (`m07`)

### `m07_lager`
*Kontext:* Belagerungslager vor Kargsted.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m07_lager_a_01.ogg` | Erzähler (`AC_narrator`) | [seriously] Eisen war in der Aschmark kostbarer als Brot. Wer die Schmiede befeuern wollte, musste durch Kargsted – [pause] und Kargsted hatte Mauern, die ein Schwarm Toter nie gebraucht hätte. | trocken, sachlich |
| 2 | `voices/characters/hero_isolde_m07_lager_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Die Rammen zuerst ans Tor, die Katapulte auf die Eckschützen. [short pause] Wer die Mauer mit der Brust einreißen will, lernt es heute auf die harte Weise. | knapp, entschlossen |

### `m07_tor`
*Kontext:* Das Südtor fällt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m07_tor_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [triumphantly] Das Tor ist gebrochen! Schildträger vor, Schützen nach – [pause] keine Gefangenen, aber lasst die Bauern am Leben! | triumphierend, scharf |

### `m07_sieg`
*Kontext:* Nach dem Sieg im Hof von Kargsted.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m07_sieg_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [warmly] Eisen. Wirkliches Eisen. Ich muss es dir lassen, Isolde: Du verstehst, wann man eine Mauer überredet und wann man sie einreißt. | warm, anerkennend |

## Mission 9 – Der Schwarze Winter (`m09`)

### `m09_schwur`
*Kontext:* Der Torhof von Hrimgard. Torvald Eisenhand empfängt Isolde vor den Gräbern seiner Ahnen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m09_schwur_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [amused] [shouting] Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen. | dröhnend, amüsiert |
| 2 | `voices/characters/hero_isolde_m09_schwur_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Lieder übertreiben. Eure machen Euch angeblich größer und weiser. | trocken, furchtlos |
| 3 | `voices/characters/char_torvald_m09_schwur_c_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [coldly] [laughs] Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den hohen Pass. Wir haben vierzig der Unseren begraben. Vierzig. | grimmig, kurz auflachend, dann dunkel |
| 4 | `voices/characters/hero_isolde_m09_schwur_d_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [seriously] Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden. | ernst, respektvoll |
| 5 | `voices/characters/char_torvald_m09_schwur_e_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [thoughtfully] Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold, Sippe zahlt in Blut. Was bist du? | scharf, prüfend |
| 6 | `voices/characters/char_torvald_m09_schwur_f_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [urgently] [quietly] Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig. | leiser, drängend |

### `m09_schwur_a`
*Kontext:* Entscheidung: Blutschwur. Torvald und seine Clankrieger kämpfen mit.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m09_schwur_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite deines Clans. | fest, ohne Zögern |
| 2 | `voices/characters/char_torvald_m09_schwur_a_b_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [triumphantly] [shouting] Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met – [pause] und die Äxte! | jubelnd, laut |

### `m09_schwur_b`
*Kontext:* Entscheidung: Bezahlung in Gold. Söldner statt Torvald.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m09_schwur_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [seriously] Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort ist besser. Nenne deinen Preis. | ruhig, geschäftsmäßig |
| 2 | `voices/characters/char_torvald_m09_schwur_b_b_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [calmly] Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben. | ernüchtert, abweisend |

### `m09_sturm`
*Kontext:* Der Sturm setzt ein; die erste Welle naht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m09_sturm_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Der Schnee fiel schwarz, wo die Asche ihn berührte. Auf den Zinnen von Hrimgard sangen die Skaldinnen ihre Ahnenlieder gegen den Wind, und der Wind sang zurück – [pause] mit hohlen Stimmen. | rau, getragen |

### `m09_kolosse`
*Kontext:* Grabkolosse erreichen die Mauern.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m09_kolosse_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [shouting] Kolosse! Die Äxte an die Mauer, ihr Brüder – [pause] bricht der Koloss die Mauer, brechen wir den Koloss! | brüllend, kampflustig |

### `m09_morgen`
*Kontext:* Der Morgen über Hrimgard.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m09_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | [tired] Als der Morgen kam, lagen die Hohlen wie umgekippte Statuen im Schnee. Auf der Mauer standen die Überlebenden und sahen nicht hinunter – [pause] sie sahen nach Osten, wo es hell wurde. | ruhig, erschöpft |

## Mission 10 – Wurzel und Asche (`m10`)

### `m10_wald`
*Kontext:* Lichtung der Veyl am Waldrand.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_wald_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [quietly] Der Dornwald war älter als die Götter, und er hatte es nie vergessen. Wer ihn betrat, tat es auf Zehenspitzen – [pause] und wer ihn verließ, nahm ein wenig Rinde in den Knochen mit. | ehrfürchtig, leise |
| 2 | `voices/characters/char_sylaen_m10_wald_b_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [seriously] [quietly] Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. Was welkt, welkt von innen – [pause] nicht durch Feuer, sondern durch einen Traum. | gedämpft, ernst, melodisch |

### `m10_lager1`
*Kontext:* Das erste Lager ist geräumt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_m10_lager1_a_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [quietly] Sie sind nicht böse. Sie sind leer. Der Wald hat ihre Namen vergessen, und sie suchen sie in unseren Stimmen. | traurig, gedämpft |

### `m10_lager3`
*Kontext:* Das dritte Lager ist geräumt, der Herzhain liegt vor ihnen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_lager3_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. Sylaen, was immer da im Hain wartet, es ist dasselbe Grau. | erkennend, unbehaglich |

### `m10_hain`
*Kontext:* Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_root_mother_m10_hain_a_01.ogg` | Die Wurzelmutter (`AC_boss_root_mother`) | [eerily] Kinder … so viele Kinder mit Feuer in den Händen … [pause] seid ihr gekommen, um mich zu halten oder um mich zu verbrennen? | uralt, zerrissen, mehrstimmig |
| 2 | `voices/characters/char_sylaen_m10_hain_b_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [urgently] [whispers] Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe. | flüsternd, drängend |
| 3 | `voices/characters/char_deep_voice_m10_hain_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [greedily] Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … [pause] verdaut. Calyx schmeckte nach Frühling. | intim, kalt, genüsslich |
| 4 | `voices/characters/char_sylaen_m10_hain_d_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [sad] Es gibt zwei Wege, und ich hasse beide. [short pause] Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken. | gequält, nüchtern |
| 5 | `voices/characters/char_sylaen_m10_hain_e_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [urgently] [quietly] Entscheide, Wächterin. Der Wald wartet nicht – [pause] und sie auch nicht. | leise, drängend |

### `m10_hain_a`
*Kontext:* Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_hain_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln. | schwer, entschlossen |
| 2 | `voices/characters/boss_root_mother_m10_hain_a_b_01.ogg` | Die Wurzelmutter (`AC_boss_root_mother`) | [gently] [quietly] Warm … so warm … [pause] schlaft gut, Kinder … | verlöschend, friedlich |
| 3 | `voices/characters/char_sylaen_m10_hain_a_c_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [sad] Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch. | Trauer, eisig |

### `m10_hain_b`
*Kontext:* Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m10_hain_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [fearful] Keine Feuer mehr. Zeig mir den Weg hinein. | entschlossen, mit Angst |
| 2 | `voices/characters/char_deep_voice_m10_hain_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [seductively] [whispers] Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen. | intim, flüsternd |
| 3 | `voices/characters/char_sylaen_m10_hain_b_c_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [urgently] [quietly] Halt meine Hand und lass nicht los. Was immer du dort drin hörst … [pause] es ist nicht dein Freund. | eindringlich, leise |

### `m10_sieg_a`
*Kontext:* Schlusswort: Wurzelmutter verbrannt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_sieg_a_a_01.ogg` | Erzähler (`AC_narrator`) | [bitterly] Der Hain brannte drei Tage lang. Als die Asche kalt war, wuchs zwischen den Wurzeln ein einzelner heller Trieb – [pause] und niemand wagte es, ihn zu fällen. | ruhig, bittersüß |

### `m10_sieg_b`
*Kontext:* Schlusswort: Traum betreten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m10_sieg_b_a_01.ogg` | Erzähler (`AC_narrator`) | [eerily] [quietly] Isolde Raventhorn kehrte nach drei Atemzügen aus dem Traum zurück – [pause] und sprach in den folgenden Wochen nicht mehr als nötig. Die Wurzelmutter aber erwachte, und der Dornwald antwortete mit Blüten. | leise, geheimnisvoll |

## Mission 11 – Der Riss von Marrowfen (`m11`)

### `m11_moor`
*Kontext:* Der Südrand des Moores von Marrowfen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m11_moor_a_01.ogg` | Erzähler (`AC_narrator`) | [menacingly] [quietly] In Marrowfen riecht es nach nasser Asche und nach Dingen, die zu lange unter Wasser geschlafen haben. Wo der Riss aufgegangen war, wuchs kein Schilf mehr – [pause] nur ein weißer Ring aus Knochenstaub. | leise, unheimlich |
| 2 | `voices/characters/char_hal_venn_m11_moor_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [worried] Drei Weiler, Isolde. Wenn wir schnell sind, schaffen wir alle drei vor der Dämmerung. Und wenn wir langsam sind … [pause] schaffen wir es gar nicht. | angespannt, nüchtern |

### `m11_dorf`
*Kontext:* Ein Weiler schließt sich dem Trupp an.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m11_dorf_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [warmly] [shouting] Kommt, kommt – langsam, haltet euch an den Händen. Wir bringen euch über den Damm, im Namen der Flamme! | warm, rufend |

### `m11_riss`
*Kontext:* Isolde erreicht den Riss; die Stimme aus dem Tief macht ihr Angebot.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_deep_voice_m11_riss_a_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [calmly] Du bist weit gekommen, kleine Flamme. [short pause] Sieh, was ich dir zeige: Aschglas, so rein, dass es singt. Nimm davon, und deine Heere werden nie wieder hungern. | schmeichelnd, tief, mehrschichtig |
| 2 | `voices/characters/char_hal_venn_m11_riss_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [worried] Hör nicht hin, Isolde. Was dort flüstert, hat Harrowmere verbrannt. | scharf, besorgt |
| 3 | `voices/characters/char_hal_venn_m11_riss_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [seriously] Der Riss kann geschlossen werden, wenn wir den Schrein an seinem Grund zerschlagen. Oder wir nehmen, was er uns reicht – [pause] und tragen die Folgen. | ernst, beschwörend |

### `m11_riss_a`
*Kontext:* Entscheidung: Riss versiegeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m11_riss_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Du bietest mir Brot, aus dem Mund eines Hungernden. Nein. Hal, schließ ihn. | kalt, entschlossen |
| 2 | `voices/characters/char_deep_voice_m11_riss_a_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [mockingly] Wie schade. Ich hatte mich gefreut, dich zu verwöhnen. | leicht gekränkt, spöttisch |

### `m11_riss_b`
*Kontext:* Entscheidung: Riss anzapfen und Aschglas ernten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m11_riss_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [quietly] Nur ein wenig. Nur so viel, wie die Aschmark in zehn Wintern verbraucht. | leise, mit schlechtem Gewissen |
| 2 | `voices/characters/char_deep_voice_m11_riss_b_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [seductively] Ein wenig. Natürlich. Ich bin geduldig, Kind. Ich war es immer. | zufrieden, samtig |

### `m11_nel`
*Kontext:* Nel, die in Brennhollow aufgenommene Waise, weist den Weg (nur wenn sie bei Hal geblieben ist).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_nel_m11_nel_a_01.ogg` | Nel (`AC_char_nel`) | [calmly] Hört ihr es? Es klingt wie eine Spieluhr, die jemand zu langsam aufzieht. Dort entlang geht es – [pause] das Schilf weiß den Weg. | kindlich, abwesend |

### `m11_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m11_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [thoughtfully] Der Damm trug die Geretteten ins Trockene. Hinter ihnen schloss sich das Moor – [pause] oder öffnete sich, je nachdem, wen man später fragte. | ruhig, vieldeutig |

## Mission 13 – Die Aschglasfelder (`m13`)

### `m13_wueste`
*Kontext:* Das Lager am Rand der Aurethwüste.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m13_wueste_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [quietly] Es war ein Land, das einmal Reich geheißen hatte. Nun lag es als flache Asche bis zum Horizont, und in der Asche funkelten Splitter – [pause] Aschglas, wo der Schleier dünn war, und Wachfeuer, wo die Legion stand. | weit, ehrfürchtig, leise |
| 2 | `voices/characters/hero_isolde_m13_wueste_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Schwarze Rüstungen, Glut in den Fugen, und das Wappen Aureths auf der Brust. [short pause] Das sind keine Wiedergänger, das ist eine Armee. Zeigt ihnen Stahl – und Feuer. | nüchtern, kalt |

### `m13_kompanie`
*Kontext:* Wendel, wenn er in Isoldes Dienst getreten ist.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_wendel_m13_kompanie_a_01.ogg` | Ser Wendel der Graue (`AC_char_wendel`) | [dryly] Die Graue Kompanie kennt Wüstenräuber, Wächterin – [pause] Tote in Rüstung sind neu. Aber Tote bluten wenigstens nicht auf den Sold. | trocken, kampfeslustig |

### `m13_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m13_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [sad] [quietly] Aus den Fugen der erschlagenen Legionäre stieg noch lange rotes Licht. Wer es sah, sprach später nicht davon. | leise, bedrückt |

## Mission 14 – Das Schwarze Riff (`m14`)

### `m14_tintenfass`
*Kontext:* Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_m14_tintenfass_a_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [amused] Willkommen am Kai, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel. | amüsiert, Hafenakustik |
| 2 | `voices/characters/hero_isolde_m14_tintenfass_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [dryly] Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen. | trocken |
| 3 | `voices/characters/char_ophira_m14_tintenfass_c_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [seriously] [laughs] Vierzig Schiffe, dreitausend Söldner und das Riff in neun Tagen statt in neunzig. [short pause] Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus den Ruinen holt. | lachend, geschäftsmäßig |
| 4 | `voices/characters/char_kassian_m14_tintenfass_d_01.ogg` | Kassian Vale (`AC_char_kassian`) | [dryly] [whispers] Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf. | trocken, flüsternd |
| 5 | `voices/characters/char_ophira_m14_tintenfass_e_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [coldly] Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft. | plötzlich kühl |

### `m14_flotte_a`
*Kontext:* Entscheidung: Die Flotte annehmen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m14_flotte_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf. | entschlossen, Bitterkeit |
| 2 | `voices/characters/char_ophira_m14_flotte_a_b_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [warmly] Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer. | warm, mit schneidendem Unterton |

### `m14_flotte_b`
*Kontext:* Entscheidung: Ohne Flotte marschieren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m14_flotte_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [proudly] Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch. | trocken, stolz |
| 2 | `voices/characters/char_ophira_m14_flotte_b_b_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [amused] Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin. | ehrlich amüsiert |

### `m14_isarre`
*Kontext:* Isarre Mol, auf der Riffinsel.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_reef_mistress_m14_isarre_a_01.ogg` | Isarre Mol (`AC_boss_reef_mistress`) | [eerily] Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. Das Meer hat mir jedes meiner Kinder zurückgegeben – [pause] ich bin nur gekommen, es zu tränken. | tief, nass, gurgelnd, mehrstimmig |
| 2 | `voices/characters/hero_isolde_m14_isarre_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Dann bin ich hier, um dich zu erlösen. Eine Flut kann man dämmen. | kalt, ruhig |

### `m14_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m14_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Das Wasser zog sich zurück, als hätte es nie anderes vorgehabt. [short pause] Auf der Riffinsel blieben Muscheln, Knochen und eine Perle von der Größe einer Faust, die später in Brennhollow in eine Rüstung eingelegt wurde. | ruhig, bildhaft |

## Mission 17 – Tod eines Königs (`m17`)

### `m17_nacht`
*Kontext:* Kaldris, Nacht des Todes von König Aldric III.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m17_nacht_a_01.ogg` | Erzähler (`AC_narrator`) | [seriously] [quietly] Der König starb im Winter, und die Glocken von Kaldris waren die Einzigen, die es laut sagten. [short pause] Im Flammendom brannten die Kerzen weiter, als hätte niemand ihnen Bescheid gegeben. | gedämpft, schwer |
| 2 | `voices/characters/hero_isolde_m17_nacht_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [quietly] Drei Spuren, ein Mörder. Der Wein, die Flamme, die Gruft. Wir gehen leise vor – [pause] wer hier Zeugen hat, hat auch Feinde. | leise, konzentriert |

### `m17_keller`
*Kontext:* Im Weinkeller des Palasts.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_keller_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Schierling im Wein. Frisch angesetzt, und nur ein einziger Krug. Wer das getan hat, kannte seinen Geschmack – [pause] und wusste, dass er Gold trinkt, kein Wasser. | kühl, analytisch |

### `m17_kapelle`
*Kontext:* In der Kapelle der Ewigen Flamme.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_kapelle_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [worried] [quietly] Ein Brief unter dem Altarstein, mit dem Siegel Nordmarks. [short pause] Aber die Tinte ist zu frisch, das Wachs zu sauber. Das hat jemand für mich hingelegt. | misstrauisch, leise |

### `m17_krypta`
*Kontext:* In der Krypta unter dem Dom, optional.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_krypta_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [seriously] Das Siegel der Sonnenhäresie, in den Stein der Gruft geritzt. Jemand hat dem König Gift gegeben – [pause] und die Hohlen haben es gewusst, bevor er starb. | ernst, erschüttert |

### `m17_hinweis`
*Kontext:* Nur wenn Kassian Isolde einen Gefallen schuldet (Entscheidung in Mission 6).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_kassian_m17_hinweis_a_01.ogg` | Kassian Vale (`AC_char_kassian`) | [whispers] Ein Wort aus der Dunkelheit, Wächterin: Die Krypta. [short pause] Ihr werdet dort etwas finden, das ich Euch nicht beweisen konnte. Ich sehe Euch im Dom. | flüsternd, aus dem Schatten |

### `m17_hof`
*Kontext:* Im Flammendom vor dem versammelten Hof.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m17_hof_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Im Flammendom waren sie alle versammelt: die Herzöge, die Flammenwahrer, die Räte. Und in der Mitte der Sarg des Königs, leer – [pause] der Leichnam lag noch im Nebenraum, damit niemand ihn küsste. | feierlich, kalt |
| 2 | `voices/characters/char_mireille_m17_hof_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [sharply] Wächterin. Man sagt, Ihr wisst, wer es war. Sagt es – [pause] oder lasst uns dem Mörder zuprosten, so lange er bei uns sitzt. | scharf, souverän |
| 3 | `voices/characters/char_hal_venn_m17_hof_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [warmly] Du darfst nur einen Namen nennen, Isolde. Es ist die Aufgabe einer Wächterin, zu entscheiden – [pause] und den Preis zu tragen. | ernst, väterlich |

### `m17_urteil_a`
*Kontext:* Beschuldigung: Kassian Vale.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Kassian Vale hat es gewusst. Er hat gewarnt, wo er hätte handeln müssen. Sucht ihn – [pause] und fragt ihn nach seiner Rolle. | gemessen, hart |
| 2 | `voices/characters/char_mireille_m17_urteil_a_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [coldly] Ein Spion, der zu viel wusste. Wie vorhersehbar – [pause] und wie bequem. | zufrieden, kalt |

### `m17_urteil_b`
*Kontext:* Beschuldigung: Herzogin Mireille.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Der Brief weist nach Nordmark. Herzogin Mireille, erklärt Euch. | ruhig, schneidend |
| 2 | `voices/characters/char_mireille_m17_urteil_b_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [menacingly] Ihr beschuldigt eine Frau, die fünfhundert Speere in Euren Dienst gestellt hat. Ihr werdet es bereuen. | eisig, bedrohlich |

### `m17_urteil_c`
*Kontext:* Beschuldigung: die Sonnenhäretiker.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [shouting] Es waren die Sonnenhäretiker. Ihre Zeichen sind im Stein der Gruft. Kaldris hat Verräter in den eigenen Mauern. | klar, laut |
| 2 | `voices/characters/char_mireille_m17_urteil_c_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [thoughtfully] Dann reinigt die Stadt. Aber lasst die Herzöge auch ruhig schlafen. | abwägend |

### `m17_urteil_d`
*Kontext:* Die ganze Wahrheit (alle drei Beweise).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m17_urteil_d_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Ich kenne die ganze Wahrheit: Gift im Wein, ein falscher Brief aus Nordmark, das Siegel der Häresie – [pause] und einer, der sie alle vermittelte. Seht auf die, die von Verwirrung leben. | ruhig, zwingend |
| 2 | `voices/characters/char_mireille_m17_urteil_d_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [respectfully] Ihr habt mehr Mut, als ich Euch zutraute. Und mehr Beweise, als mir lieb ist. | widerwillig respektvoll |

## Mission 19 – Die Furt von Kaldris (`m19`)

### `m19_thron`
*Kontext:* Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_king_aldric_m19_thron_a_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [coldly] Isolde. Kommt näher. Meine Augen … [pause] sind nicht mehr, was sie waren. | sterbend, flach, mit Atempausen |
| 2 | `voices/characters/hero_isolde_m19_thron_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [gently] Ich bin hier, Majestät. Ruht Euch aus. | sanft, ungewohnt weich |
| 3 | `voices/characters/char_king_aldric_m19_thron_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [seriously] [quietly] Mireille steht vor den Toren. [short pause] Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er dürfe es auch. Valemark wird sich zerfleischen. | ernst, leise, schwer atmend |
| 4 | `voices/characters/char_mireille_m19_thron_d_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [shouting] Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich bin gekommen, ein sterbendes Königreich zu retten! | laut, fordernd, vom Feld |
| 5 | `voices/characters/char_hal_venn_m19_thron_e_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [seriously] Mireille oder die Linie des Königs, Isolde. Beides hat seinen Preis – [pause] und beides wird Menschen töten. | dunkel, ernst |

### `m19_thron_a`
*Kontext:* Entscheidung: Mireille stützen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m19_thron_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere. | bitter, entschlossen |
| 2 | `voices/characters/char_mireille_m19_thron_a_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [calmly] Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit wird. | ruhig, mit Würde |
| 3 | `voices/characters/char_king_aldric_m19_thron_a_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [quietly] So sei es … [pause] Haltet … die Feuer am Brennen … | letzte Worte, verhauchend |

### `m19_thron_b`
*Kontext:* Entscheidung: Aldrens Linie verteidigen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m19_thron_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern. | klar, unerbittlich |
| 2 | `voices/characters/char_mireille_m19_thron_b_b_01.ogg` | Herzogin Mireille Corvane (`AC_char_mireille`) | [coldly] Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin. | kalt, enttäuscht |
| 3 | `voices/characters/char_king_aldric_m19_thron_b_c_01.ogg` | König Aldric III. (`AC_char_king_aldric`) | [moved] [quietly] Euer Vater … wäre stolz. Es tut mir … [pause] leid, dass es so viel kostet. | dankbar, schwach |

### `m19_schlacht`
*Kontext:* Beginn der Schlacht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m19_schlacht_a_01.ogg` | Erzähler (`AC_narrator`) | [seriously] Zwei Heere an einem Fluss. [short pause] Das Wasser war flach, die Furt breit, und auf beiden Ufern standen Menschen, die mit demselben Gebet in dieselbe Schlacht zogen. | ruhig, schwer |
| 2 | `voices/characters/hero_isolde_m19_schlacht_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [commanding] [shouting] Haltet die Furt! Die Reiter an die Flanken, Armbrüste in die Mitte – [pause] und lasst sie kommen! | laut, kommandierend |

### `m19_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m19_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Als der Rauch sich über der Furt lichtete, trug der Fluss seine Toten in die Ebene. [short pause] Das Banner der Raventhorns wehte im Zentrum, und die Herzöge zählten ihre Schwüre noch einmal nach. | ruhig, gewichtig |

## Mission 21 – Der Schrein Oruuns (`m21`)

### `m21_stollen`
*Kontext:* Der Eingang des Stollens unter Skarnheims Gipfeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m21_stollen_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Der Berg hatte seit dem Aschenfall nicht mehr gesprochen. Wer an seiner Flanke stand, spürte ihn manchmal zittern – [pause] wie ein Schlafender, der etwas Schlimmes träumt. | tief, ruhig |
| 2 | `voices/characters/hero_isolde_m21_stollen_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] [quietly] Der Weg führt durch drei Hallen. Haltet die Reihen geschlossen, und wenn der Stein spricht: nicht antworten. | leise, kontrolliert |

### `m21_torvald`
*Kontext:* Nur wenn Torvald den Blutschwur gehalten hat.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m21_torvald_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [solemn] Meine Väter haben diesen Schrein mit eigenen Händen gebaut. [short pause] Dass ich ihn nun mit der Axt zurückfordern muss, ist eine Schande, die ich gern wasche. | grimmig, ehrfürchtig |

### `m21_soeldner`
*Kontext:* Nur wenn Torvald nicht mitkämpft (Gold-Variante).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m21_soeldner_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [thoughtfully] Gold kauft Schwerter, aber keine Trauer. Dieser Berg ist nicht meiner – [pause] und doch kämpfe ich um ihn. | nachdenklich, trocken |

### `m21_gharun`
*Kontext:* Isolde und Gharun im Schreinsaal.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_stone_oath_m21_gharun_a_01.ogg` | Gharun, der Steinerne Eid (`AC_boss_stone_oath`) | [shouting] Wer wagt es, den Eid zu stören? Ich wurde gebunden, zu bewahren – [pause] und ich bewahre. Was ich bewahre, spricht nicht mehr mit euch. | steinern, langsam, dröhnend, hallend |
| 2 | `voices/characters/hero_isolde_m21_gharun_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Du bewahrst den Falschen, Wächter. Wir sind gekommen, den Schrein zurückzuholen – [pause] und deinen Eid zu lösen. | ruhig, gegenüber einem Berg |

### `m21_sieg`
*Kontext:* Der Schrein wird neu geweiht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m21_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] [quietly] Gharun zerfiel zu Staub, der in der Luft stand, als wollte er sich noch einmal verbeugen. [short pause] Der Eid war nicht gebrochen, sondern erfüllt: Der Schrein war in Sicherheit. | ruhig, gedämpft |
| 2 | `voices/characters/hero_isolde_m21_sieg_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [solemn] [quietly] Oruun, wenn du zuhörst: Der Schrein ist dein. Wir kommen wieder, wenn du uns brauchst. | leise, ehrfürchtig |

## Mission 25 – Ithrens letztes Licht (`m25`)

### `m25_ruinen`
*Kontext:* Am Stadtrand von Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_ruinen_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Solmere war einst eine Stadt aus Licht gewesen, Terrassen aus weißem Stein bis an den Himmel. Nun waren es Rippen, in denen der Wind klagte – [pause] und zwischen den Rippen standen Männer mit Sonnen auf den Brustpanzern, die auf einen toten König warteten. | ehrfürchtig, schwermütig |
| 2 | `voices/characters/char_hal_venn_m25_ruinen_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [worried] Sei vorsichtig, Isolde. Die Sonnenhäretiker töten Fremde, aber sie töten sie mit Gebeten. Das macht es nicht besser. | trocken, besorgt |

### `m25_tagebuch`
*Kontext:* Am Schrein Ithrens: Varkhans Tagebuch.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_tagebuch_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [quietly] Unter den Resten des Sonnenhofs lag das Licht Ithrens, kaum größer als eine Kerzenflamme. [short pause] Daneben, in einer Nische aus Gold, ein Buch, dessen Seiten nach Rauch und Rosenwasser rochen. | leise, ehrfürchtig |
| 2 | `voices/characters/char_hal_venn_m25_tagebuch_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [quietly] Varkhans Tagebuch. Er schreibt … [pause] von seiner Tochter. Sie starb an der Schwindsucht, und er wollte … er wollte nicht den Tod besiegen, Isolde. Er wollte sie. | betroffen, leise |
| 3 | `voices/characters/char_deep_voice_m25_tagebuch_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [menacingly] Ein Vater, der alles verbrannte, um ein Kind zu retten. So rührend. So nützlich. | samtig, süß, bedrohlich |
| 4 | `voices/characters/char_hal_venn_m25_tagebuch_d_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [seriously] Es liegt bei dir, Isolde. Das Buch enthält die Wahrheit, und die Wahrheit hat Folgen. | ernst, bedächtig |

### `m25_urteil_a`
*Kontext:* Entscheidung: Die Wahrheit veröffentlichen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Die Wahrheit gehört den Menschen. Lasst sie lesen, was Varkhan getan hat – [pause] und warum. Wer ihn dann noch erlösen will, soll es wissen. | ruhig, klar |
| 2 | `voices/characters/char_hal_venn_m25_urteil_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [calmly] Das wird die Häresie spalten. Aber vielleicht ist das besser, als wenn sie an einer Lüge zerbricht. | bedächtig |

### `m25_urteil_b`
*Kontext:* Entscheidung: Den Glauben der Häretiker lassen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [quietly] Sie glauben an etwas, das sie am Leben hält. Ich nehme ihnen das nicht, bevor ich es ersetzen kann. | leise, widerwillig |
| 2 | `voices/characters/char_hal_venn_m25_urteil_b_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [gently] Gnade und Feigheit sind manchmal Geschwister. Ich hoffe, du weißt, welche du gerade wählst. | sanft, mahnend |

### `m25_urteil_c`
*Kontext:* Entscheidung: Die Häretiker für den eigenen Feldzug nutzen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m25_urteil_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Sie glauben, er werde zurückkehren. Gut. Sie sollen für ihn kämpfen – [pause] bis ich ihn selbst stelle. | kalt, berechnend |
| 2 | `voices/characters/char_deep_voice_m25_urteil_c_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [amused] [quietly] Wie praktisch du bist, kleine Flamme. Wie ähnlich uns. | leise, amüsiert |

### `m25_sieg`
*Kontext:* Schlusswort.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m25_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [hopefully] [quietly] Das Licht Ithrens erlosch nicht. Es wartete, mit der Geduld der Götter, auf jemanden, der bereit war, es zu tragen. | leise, hoffnungsvoll |

## Mission 26 – Der Aschenmarsch (`m26`)

### `m26_marsch`
*Kontext:* Brennhollow, Abenddämmerung; die Aschenlegion naht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_marsch_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Sie kamen im Gleichschritt. Zum ersten Mal seit dem Aschenfall hörte man die Hohlen nicht durch die Nacht schlurfen, sondern marschieren – [pause] und der Boden der Aschmark antwortete mit einem Beben. | schwer, getragen |
| 2 | `voices/characters/char_hal_venn_m26_marsch_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [quietly] Es ist die Legion, Isolde. Die Wappen Aureths auf jeder Brust. Ich habe sie als Junge in den Liedern gesehen – [pause] nicht im Fleisch. | bestürzt, leise |
| 3 | `voices/characters/hero_isolde_m26_marsch_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [thoughtfully] Dies ist keine Entscheidung, die ich allein treffen will. Aber sie steht mir zu. Halten wir Brennhollow – [pause] oder retten wir die, die in ihm wohnen? | schwer, nachdenklich |

### `m26_wahl_a`
*Kontext:* Entscheidung: Brennhollow halten.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_wahl_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [commanding] [shouting] Wir halten. Diese Mauern haben dreißig Jahre Feuer gesehen. Jeder Bürger auf die Wälle, jedes Kind in die Keller! | laut, kommandierend |
| 2 | `voices/characters/char_hal_venn_m26_wahl_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [firmly] Die Flamme steht mit Brennhollow. Sie hat es immer getan. | fest, warm |

### `m26_wahl_b`
*Kontext:* Entscheidung: Brennhollow evakuieren.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_wahl_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Wir räumen die Stadt. Die Legion kann Steine erobern, aber keine Menschen. Das Osttor auf – [pause] alle zum Kaldris-Pass! | kalt, schwer |
| 2 | `voices/characters/char_hal_venn_m26_wahl_b_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [gruffly] Ich habe diese Mauern mit eigenen Händen gesegnet. Es ist eine gute Entscheidung. Es wird nur lange wehtun. | rau, gefasst |

### `m26_flucht`
*Kontext:* Beginn der Evakuierung.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m26_flucht_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [urgently] [shouting] Haltet die Reihen, und lasst die Kinder zwischen den Wagen gehen! Die Soldaten decken den Rückzug – [pause] wer zurückbleibt, bleibt mit Ehre! | laut, drängend |

### `m26_morgen`
*Kontext:* Brennhollow hält bis zum Morgen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_morgen_a_01.ogg` | Erzähler (`AC_narrator`) | [tired] Das Morgenlicht fand die Zinnen von Brennhollow zerschossen, aber nicht erstürmt. Die Legion zog sich zurück, dem Süden entgegen – [pause] ein Heer, das nicht müde wurde, aber wartete. | ruhig, erschöpft |

### `m26_pass`
*Kontext:* Die Evakuierung erreicht die Straße nach Kaldris.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m26_pass_a_01.ogg` | Erzähler (`AC_narrator`) | [sorrowfully] [quietly] Hinter ihnen brannte Brennhollow. Niemand sah sich um. Wer zurückblickte, hörte das Läuten der Kapelle – [pause] und erst später begriff man, dass sie von selbst läutete. | leise, schmerzvoll |

## Mission 28 – Die Tore von Solmere (`m28`)

### `m28_lager`
*Kontext:* Das Lager des Bundes vor Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m28_lager_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Hinter dem Horizont wuchsen die Mauern Solmeres: schwarz, hoch, in den Fugen glimmend. Zum ersten Mal seit tausend Jahren sahen lebende Menschen die Hauptstadt Aureths aus der Nähe – [pause] und zählten die Zinnen, ohne sie zu Ende zu zählen. | weit, ehrfürchtig |
| 2 | `voices/characters/hero_isolde_m28_lager_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [dryly] Rammen ans Tor, Katapulte auf die Zinnen. Und niemand betritt Solmere, ehe das Tor offen ist – [pause] ich will keinen Mann vor die Hexen schicken, der noch eine Mauer überwinden muss. | klar, knapp |

### `m28_torvald`
*Kontext:* Nur wenn Torvald den Blutschwur gehalten hat.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_torvald_m28_torvald_a_01.ogg` | Torvald Eisenhand (`AC_char_torvald`) | [proudly] Meine Ahnen sehen zu, Isolde. Sie haben nie eine Mauer gesehen, vor der Eisenhand zögerte. Wir gehen zuerst – [pause] du brichst das Tor. | grimmig, stolz |

### `m28_sylaen`
*Kontext:* Nur wenn die Wurzelmutter erwacht ist (Pakt).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_sylaen_m28_sylaen_a_01.ogg` | Sylaen der Veyl (`AC_char_sylaen`) | [calmly] Der Wald hat mich nicht verlassen, Wächterin. Meine Pfeile fliegen mit dem Wind – [pause] und der Wind kennt Solmere. | ruhig, melodisch |

### `m28_ophira`
*Kontext:* Nur wenn der Vertrag mit Ophira geschlossen wurde.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_ophira_m28_ophira_a_01.ogg` | Ophira Saelith-Varo (`AC_char_ophira`) | [seriously] Meine Schiffe liegen im Hafen, meine Söldner stehen bei Euch. [short pause] Vergesst nicht, dass jeder Schuss in diesen Mauern ein Eintrag in meinem Buch ist. | trocken, geschäftsmäßig |

### `m28_tor`
*Kontext:* Das Haupttor fällt.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m28_tor_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [triumphantly] [shouting] Das Tor ist gebrochen! Schildträger voran, die Priester folgen den Schilden – [pause] und lasst keinen zurück! | laut, triumphierend |

### `m28_haeretiker`
*Kontext:* Die Sonnenhäretiker auf den Mauern (Entscheidung in Mission 25).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m28_haeretiker_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Auf den Mauern Solmeres standen Männer mit Sonnen auf der Brust. [short pause] Sie sahen die Banner des Bundes, hörten, was Isolde Raventhorn gelesen hatte, und senkten die Waffen. Man weiß nicht, ob aus Scham oder aus Hoffnung. | ruhig, bedächtig |

### `m28_sieg`
*Kontext:* Nach der Eroberung des Torturms.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m28_sieg_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [moved] [quietly] Das war das Tor. Hinter ihm liegt der Thron. Ich muss dir etwas sagen, Isolde, bevor wir hineingehen: Danke. Für alles, was du getan hast – [pause] auch für das, was dich gekostet hat. | leise, bewegt |

## Mission 29 – Der Hohle Thron (`m29`)

### `m29_tor`
*Kontext:* Das Innere des Tores von Solmere.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_tor_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Hinter dem Tor lag, was von Aureth übrig war: vier Höfe, einer hinter dem anderen, wie Kammern eines Herzens. [short pause] Und am Ende der Höfe der Thron, auf dem seit tausend Jahren jemand saß, der nicht schlief und nicht starb. | feierlich, tonlos |
| 2 | `voices/characters/char_hal_venn_m29_tor_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [warmly] [quietly] Gleich ist es so weit, Isolde. Wenn du Zweifel hast – [pause] jetzt wäre der Moment, sie auszusprechen. | leise, warm |
| 3 | `voices/characters/hero_isolde_m29_tor_c_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Zweifel habe ich seit Brennhollow. Aber sie halten mich nicht mehr auf. Vorwärts, für die Mark. | ruhig, fest |

### `m29_hoefe`
*Kontext:* Beim Betreten des Hofs des Schleiers.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_hoefe_a_01.ogg` | Erzähler (`AC_narrator`) | [worried] [quietly] Der dritte Hof war der Hof des Schleiers. Die Luft zitterte, als wäre sie nur gemalt, und über den Mauern hing ein Riss, durch den man die Nacht sah – [pause] eine, die nicht die eigene war. | leise, gespannt |

### `m29_thron`
*Kontext:* Im Thronsaal: Varkhan und die Stimme aus dem Tief.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/boss_hollow_king_m29_thron_a_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [solemn] Ich war die Sonne von Aureth. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal. | archaisch, königlich, hohl |
| 2 | `voices/characters/hero_isolde_m29_thron_b_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [coldly] Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken. | ruhig, kalt |
| 3 | `voices/characters/boss_hollow_king_m29_thron_c_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [sad] [quietly] Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. Es hat mir geantwortet … [pause] und es hat gelogen. | plötzlich leise, gequält |
| 4 | `voices/characters/char_deep_voice_m29_thron_d_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [moved] Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist niemand mehr gestorben. | ruhig, entsetzlich ungerührt |

### `m29_schleier`
*Kontext:* Der König ist halb gefallen; der Schleier öffnet sich.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/char_hal_venn_m29_schleier_a_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [urgently] Isolde, der Schleier ist offen hinter dem Thron. Was immer du tun willst, tu es jetzt. | dringlich |
| 2 | `voices/characters/boss_hollow_king_m29_schleier_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [tired] Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … [pause] jemand muss die Asche tragen. | tonlos, erschöpft |
| 3 | `voices/characters/char_deep_voice_m29_schleier_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [coldly] Wähle, kleine Flamme. Ich bin so oder so da. | geduldig, gleichgültig |
| 4 | `voices/characters/boss_hollow_king_m29_schleier_d_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [tired] Wähle, Wächterin. Es gibt keinen vierten Weg – [pause] außer denen, die du dir durch deine Wege verdient hast. | tonlos, müde |

### `m29_ende_a`
*Kontext:* Letzte Wahl: Schleier versiegeln.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_a_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [calmly] Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten. | Abschied, ruhig, sehr gefasst |
| 2 | `voices/characters/char_hal_venn_m29_ende_a_b_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [sad] Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich. | brechende Stimme, Tränen |
| 3 | `voices/characters/char_deep_voice_m29_ende_a_c_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [calmly] Tapfer. Kurz. Und nun … [pause] mein. Willkommen daheim. | ruhig, endgültig |

### `m29_ende_b`
*Kontext:* Letzte Wahl: Hohle Krone nehmen.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_b_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [firmly] Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden. | hart, kalt entschlossen |
| 2 | `voices/characters/boss_hollow_king_m29_ende_b_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [sad] [laughs] Genau das habe ich auch gesagt. | hohles, trauriges Lachen, zerfallend |
| 3 | `voices/characters/char_hal_venn_m29_ende_b_c_01.ogg` | Bruder Hal Venn (`AC_char_hal_venn`) | [fearful] [quietly] Isolde … deine Augen. Die Flamme vergebe uns beiden. | entsetzt, leise |

### `m29_ende_c`
*Kontext:* Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_c_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [gently] Varkhan. Deine Tochter hieß Sereth. Sie hat dir nie gesagt, dass sie dir verzeiht, weil sie nie dachte, dass du es brauchst. Sie tut es. | weich, ruhig, ohne Kälte |
| 2 | `voices/characters/boss_hollow_king_m29_ende_c_b_01.ogg` | Der Hohle König (`AC_boss_hollow_king`) | [calmly] Sereth … Sereth. Ich habe den Namen so lange nicht gehört. Wer – [pause] wer hat ihn dir gesagt? | zitternd, menschlich |
| 3 | `voices/characters/char_nel_m29_ende_c_c_01.ogg` | Nel (`AC_char_nel`) | [quietly] Sie hat ihn mir gesagt. Im Schlaf. Sie wartet auf dich. | kindlich, klar, leise |

### `m29_ende_d`
*Kontext:* Letzte Wahl: Das Tiefe verbrennen (nur mit zerschlagenem Kern und versiegeltem Riss).

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/characters/hero_isolde_m29_ende_d_a_01.ogg` | Isolde Raventhorn (`AC_hero_isolde`) | [bitterly] Es gibt kein Tor, das ich schließen muss, und keinen Thron, den ich besteigen will. Es gibt nur Feuer – [pause] und das ist alles, was ich je gut konnte. | bitter, leidenschaftlich |
| 2 | `voices/characters/char_deep_voice_m29_ende_d_b_01.ogg` | Die Stimme aus dem Tief (`AC_char_deep_voice`) | [seductively] Du verbrennst dich selbst, kleine Flamme. Schade. Ich hätte dich so gern behalten. | bedauernd, samtig |

### `m29_sieg`
*Kontext:* Schlusswort der Endschlacht.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_m29_sieg_a_01.ogg` | Erzähler (`AC_narrator`) | [calmly] Der Hohle König war gefallen. [short pause] Die Glocken von Solmere läuteten ein letztes Mal, und danach hörte man in der ganzen Welt nur noch den Wind. | ruhig, still |

## Zeitalter der Kampagne – Kapiteleinleitungen und Enden (`age`)

### `age_age1`
*Kontext:* Kapitelseite Zeitalter I.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age1_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Jahr 897 nach dem Fall. Am Rand der Aschmark hält Brennhollow noch stand – [pause] dreißig Seelen, ein Bergfried, ein Feld. Und jede Nacht kommt der Nebel näher. | getragen, ruhig, Chronistenton |

### `age_age2`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age2_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Brennhollow wuchs, und mit ihm der Ruf seiner Wächterin. [short pause] Doch wer in der Aschmark Mauern baut, wird in Kaldris gezählt: Der König prüft seine Getreuen, und jeder Hof hat sein Messer. | getragen, ruhig, Chronistenton |

### `age_age3`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age3_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Die Hohlen griffen nicht mehr in Wellen an, sondern wie ein Heer, das einen Plan verfolgt. An drei Grenzen brachen die Mauern – [pause] und die alten Feinde mussten lernen, einander zuzuhören. | getragen, ruhig, Chronistenton |

### `age_age4`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age4_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Aus der Verteidigerin wurde eine Eroberin. [short pause] Die Aschglasfelder lockten Legion und Händler zugleich, und an der Küste wartete etwas, das nicht ertrunken war, sondern wartete. | getragen, ruhig, Chronistenton |

### `age_age5`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age5_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Der König war alt, und der Thron hat keine Freunde. In Kaldris wurden Zeugen leise und Glocken laut – [pause] und Isolde stand in der Mitte, mit dem Schwert in der einen und dem Gesetz in der anderen Hand. | getragen, ruhig, Chronistenton |

### `age_age6`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age6_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Die Götter schwiegen seit dem Aschenfall. Nun regten sich Altäre unter Bergen und Ruinen, und mit ihnen erwachte, was die Götter einst gebunden hatten – [pause] und was sich an ihrer Stelle genährt hatte. | getragen, ruhig, Chronistenton |

### `age_age7`

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_age7_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Das Hohle Tief erwachte nicht aus eigener Kraft: Es hatte immer einen Schlüssel. [short pause] Brennhollow brannte noch einmal, und dann zog ein Bund aus Fahnen und Feuern nach Solmere, wo ein toter König auf seinem Thron wartete. | getragen, ruhig, Chronistenton |

### `age_ende`
*Kontext:* Enden der Kampagne, gesprochen nach dem Abspann der letzten Mission.

| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |
|---|---|---|---|---|
| 1 | `voices/narrator/narrator_age_ende_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [slowly] Varkhan Aurelith sprach zum ersten Mal seit tausend Jahren den Namen seiner Tochter – [pause] und das Tiefe verlor seinen Schlüssel. Es wurde still in Solmere, und im Frühjahr blühte in der Aschmark der erste Apfelbaum seit dem Fall. | ruhig, getragen, langsam im letzten Satz |
| 2 | `voices/narrator/narrator_age_ende_b_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [slowly] Das Feuer, das Isolde in Solmere entfachte, brannte neun Tage lang. Das Tiefe wich zurück, doch die Wunden der Welt blieben offen – [pause] und die Glut der Wächterin erlosch mit ihnen. | ruhig, getragen, langsam im letzten Satz |
| 3 | `voices/narrator/narrator_age_ende_c_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [slowly] So wurde die letzte Krone Eldraeths aus Asche geschmiedet. [short pause] Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte mehr. Doch nachts, in den Hallen von Solmere, sprach die neue Herrin manchmal mit jemandem, den niemand sah. | ruhig, getragen, langsam im letzten Satz |
| 4 | `voices/narrator/narrator_age_ende_d_01.ogg` | Erzähler (`AC_narrator`) | [solemn] [slowly] Und so wurde der Schleier geschlossen. [short pause] Niemand krönte die Wächterin der Aschmark. Aber in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen. | ruhig, getragen, langsam im letzten Satz |

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


---

## Anhang – Produktionshinweise Dialog

1. **Kontext über Tags:** Für natürliche Reaktionen die Absicht über die Audio-Tags im Sprechtext steuern (z. B. `[interrupting]`, `[quietly]`). Kein vorangestellter Kontextsatz: es wird nichts weggeschnitten.
2. **Konsistenz:** Eine Figur innerhalb einer Szene immer mit derselben Stimme, demselben Modell und derselben Stabilität generieren.
3. **Szenen-Akustik** (Thronsaal, Berg, Hafen, Wald) sowie Effekte der Deep Voice und der Bosse setzt die Integration; geliefert werden trockene Takes.
4. **Untertitel** aus der Sprechtext-Spalte ohne Tags übernehmen; Optionsmarker stehen in der Regie-Spalte, nicht im Sprechtext.
5. **Lokalisierung:** Die DE-Spalte ist Bedeutungsreferenz; eine spätere deutsche Vertonung benötigt ein eigenes, lippen-/längenangepasstes Skript.
