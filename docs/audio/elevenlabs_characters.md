# ASHEN CROWNS – Stimmprofile & Voice Design (ElevenLabs)

Grundlage: `docs/canon.md`. Produktionsregeln, Glossar und Aussprache: `docs/audio/elevenlabs_production_guide.md`.

**Aufbau jedes Profils:** ID · Rolle · Alter · Timbre · Akzent · Tempo · Emotion/Grundhaltung · **Voice-Design-Prompt** (englisch formuliert, da das Werkzeug englische Beschreibungen am besten versteht; **muss** die Angabe „speaks German with a natural, native accent“ enthalten; in das Feld *Voice Description*) · **Beispieltext** (deutsch, in das Feld *Text to preview*) · Modell &
Einstellungs-Empfehlung · Integration (nicht vom Nutzer) · Do/Don't.

**Allgemeine Regeln**
- Jede gespeicherte Stimme heißt `AC_<id>` (z. B. `AC_hero_isolde`, `AC_unit_knight`, `AC_ember_crown_priest`).
- Prompts beschreiben Merkmale, **niemals reale Personen** oder bekannte Figuren.
- Jeder Prompt endet mit einer Qualitätsangabe; bei Kreaturen wird die Textur ergänzt, Effekte setzt aber die Integration.
- Einstellungen sind Startwerte (siehe Leitfaden, Abschnitt 4); Modellnamen und Regler zum Produktionszeitpunkt in der UI prüfen.
- Casting-Freigabe: Alle Stimmen einer Fraktion nebeneinander anhören – sie müssen **unterscheidbar** sein (Alter, Tonhöhe, Akzent).

---

## Teil A – Heldin der Vertical Slice

### A1 `hero_isolde` – Isolde Raventhorn, Wächterin der Aschmark

| Merkmal | Festlegung |
|---|---|
| Rolle | Spielbare Heldin, Ritterin und Glutklingen-Adeptin; Tochter eines gefallenen Grenzlords |
| Alter | 34 (klingt Anfang/Mitte 30, mit gelebter Härte) |
| Timbre | mittlere bis tiefe Frauenstimme, leicht rau/heiser (Folge des Aschenfiebers), fester Kern, kaum Vibrato |
| Akzent | Deutsch: neutrales Hochdeutsch mit leicht norddeutscher Färbung, geerdet, nicht adlig-geziert; Konsonanten klar |
| Tempo | knapp, ökonomisch; im Humor kurze Verzögerung vor der Pointe |
| Emotion | pflichtbewusst, kontrolliert, trocken-humorvoll; Wärme nur durchscheinend; Wut kalt, nicht schrill |
| Do | Understatement, Müdigkeit hörbar machen, Befehle ruhig und sicher |
| Don't | Pathos-Heldin, Kreischen, Flirt-Ton, übertriebene Ironie |

**Voice-Design-Prompt:**
> A woman in her mid-thirties with a low, slightly husky alto voice, a faint roughness at the edges as if her throat was
> scarred by illness long ago. Speaks German with a natural, native accent: grounded standard German with a faint northern
> coloring, plain and unpretentious. She speaks in short, controlled, economical sentences, calm authority of a veteran
> knight, with a dry, understated sense of humour. Steady breath, no vibrato, never shrill. Close-mic, studio-quality
> recording, no background noise.

**Beispieltext:**
> „[calmly] Die Aschmark verzeiht keine Fehler, und ich auch nicht. Baut die Palisade bis zum Einbruch der Nacht wieder
> auf, haltet die Feuer am Brennen, und wenn irgendetwas aus diesen Ruinen kriecht … [pause] nun. Überlasst es mir.
> Dafür bezahlt man mich. [dryly] Theoretisch.“

**Modell & Einstellungen:** Eleven v3, Stabilität *Natural* (Kampfrufe ggf. *Creative*), Speed 1.0. Tags sparsam:
`[sarcastic]`, `[chuckles]`, `[exhales]`, `[shouting]`, `[groans]`.
**Integration (nicht vom Nutzer):** Standardkette Stimme; Hochpass 100 Hz; Präsenz +1,5 dB @ 3,5 kHz.
**Referenzdatei** für alle Stimm-Lautheiten: `voices/heroes/hero_isolde_select_01.ogg`.

---

## Teil B – Kernfiguren der Kampagne

### B1 `char_king_aldric` – König Aldric III. von Valemark

| Merkmal | Festlegung |
|---|---|
| Rolle | alternder König mit schwankender Legitimität |
| Alter | Ende 60 |
| Timbre | einst kräftiger Bariton, jetzt dünner, belegter Klang; leichte Altersbrüchigkeit, gelegentliches Zittern bei langen Sätzen |
| Akzent | Deutsch: gepflegtes, getragenes Hochdeutsch (Bühnenaussprache), altmodische Formulierungen |
| Tempo | langsam, mit Pausen zum Atemholen; beschleunigt bei Zorn |
| Emotion | Würde, die er festhält; darunter Angst vor Bedeutungslosigkeit; aufflackernder Stolz |
| Do | Pluralis Majestatis gelegentlich; Erschöpfung im Atem |
| Don't | senile Karikatur, Brüllen |

**Voice-Design-Prompt:**
> An elderly king in his late sixties with a fading baritone voice that was once powerful, now thinner and slightly
> hoarse, with a subtle tremor on long phrases. Speaks German with a natural, native accent: refined, aristocratic
> stage-standard German, old-fashioned and formal. Slow, deliberate pacing with audible breaths, dignity holding back fear
> and weariness. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[wearily] Mein Großvater hielt diese Krone gegen Hunger und Feuer. Mein Vater gegen die Hohlung. Und ich … [pause]
> mir wird gesagt, ich müsse sie gegen meine eigenen Herzöge halten. Sagt mir, Wächterin: [solemn] Kniet Ihr noch vor
> der Glutkrone – oder nur vor dem Mann, der sie trägt?“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (Speed 0,93); Tags `[tired]`, `[angry]`, `[coughs]` sparsam; Monologe `[slowly]`, `[pause]`.
**Integration (nicht vom Nutzer):** Standardkette, Hochpass 70 Hz, im Dialog leichter Thronsaal-Hall (1,6 s, Mix 8 %) nur szenisch.

### B2 `char_mireille` – Herzogin Mireille Corvane von Nordmark

| Merkmal | Festlegung |
|---|---|
| Rolle | ehrgeizige Herzogin von Nordmark, Rivalin der Krone |
| Alter | Anfang 40 |
| Timbre | klarer, kühler Mezzosopran, sehr präzise Artikulation, samtig in den Tiefen |
| Akzent | Deutsch: scharfes, elegantes Hochdeutsch mit schneidenden Konsonanten |
| Tempo | kontrolliert, gezielt eingesetzte Pausen; nie gehetzt |
| Emotion | berechnend, charmant-gefährlich, verächtlich gegenüber Schwäche; echte Überzeugung, dass sie das Reich retten muss |
| Do | Lächeln hörbar, Drohungen höflich verpacken |
| Don't | Klischee-Schurkin, Lachen à la Bösewicht |

**Voice-Design-Prompt:**
> A poised woman in her early forties with a cool, clear mezzo-soprano voice, velvety in the lower register. Speaks German
> with a natural, native accent: crisp, elegant standard German with precise, cutting consonants. Measured and confident
> delivery, every pause deliberate, a polite smile that carries a quiet threat. Intelligent, ambitious, never rushed.
> Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[coldly] Loyalität ist ein hübsches Wort, Wächterin. Es kostet nichts, es auszusprechen, und alles, es zu halten. Der
> König bietet Euch Dankbarkeit. Ich biete Euch Soldaten, Korn und eine Zukunft für Eure kleine Siedlung. Wählt rasch.
> [commanding] Die Hohlen warten nicht auf Höflichkeit.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (`[sarcastic]`, `[laughs softly]` selten; Monologe mit `[pause]`).
**Integration (nicht vom Nutzer):** Standardkette, Hochpass 100 Hz, leichte Luft +1 dB @ 10 kHz.

### B3 `char_hal_venn` – Bruder Hal Venn, Flammenpriester in Brennhollow (Mentor)

| Merkmal | Festlegung |
|---|---|
| Rolle | Mentor, Priester der Ewigen Flamme, Stimme der Mission 1 |
| Alter | Anfang 60 |
| Timbre | warmer, kiesiger Bariton, leicht rauchig (Jahre am Altarfeuer), freundlicher Brustklang |
| Akzent | Deutsch: weiches, ländlich gefärbtes Hochdeutsch (leichte norddeutsche Wärme), ländlicher als Isolde |
| Tempo | ruhig, erzählend; bei Gefahr klar und schnell, aber nie panisch |
| Emotion | Fürsorge, Glaube ohne Fanatismus, gutmütiger Humor, Trauer um verlorene Gemeinden |
| Do | väterlich, gelegentlich sanfter Spott über Isoldes Sturheit |
| Don't | Predigerpathos, Frömmelei, Märchenonkel-Ton |

**Voice-Design-Prompt:**
> A kind priest in his early sixties with a warm, gravelly baritone, slightly smoky from years beside altar fires. Speaks
> German with a natural, native accent: soft, down-to-earth standard German with a faint rural northern warmth. Calm,
> patient, storytelling pace, gentle humour and quiet faith, fatherly but not preachy. When urgent, he becomes clear and
> firm without panic. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[warmly] Dreißig Seelen, Isolde. Dreißig Seelen, ein kalter Herd und eine Kapelle ohne Dach. Kein großes Königreich.
> [softly] Aber die Flamme hat mit weniger begonnen, und du auch. Entzünde das erste Feuer, und dann sehen wir, wer zu
> ihm heimkehrt.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural*, Speed 0,97 (`[chuckles]`, `[sighs]` gezielt; ruhige Passagen mit `[softly]`, `[pause]`).
**Integration (nicht vom Nutzer):** Standardkette, Hochpass 70 Hz, Wärme (Sättigung ≤ 5 %).

### B4 `char_torvald` – Torvald Eisenhand, Clanfürst Skarnheims

| Merkmal | Festlegung |
|---|---|
| Rolle | Clanfürst, potenzieller Verbündeter/Rivale |
| Alter | Mitte 50 |
| Timbre | sehr tiefer, voluminöser Bassbariton, Brustresonanz, rau vom Kälte- und Schmiederauch |
| Akzent | Deutsch: nordisch gefärbt, rollendes Zungenspitzen-R, harte Konsonanten |
| Tempo | gemessen und schwer; Lachen dröhnend, Zorn explosiv |
| Emotion | Stolz, Ahnenehre, misstrauisch gegenüber Südländern, großzügig gegenüber Bewährten |
| Do | Pausen wie Gewicht, Ahnen beschwören |
| Don't | Wikinger-Parodie, Grunzen statt Worte |

**Voice-Design-Prompt:**
> A massive clan chieftain in his mid-fifties with a very deep, booming bass-baritone voice, rich chest resonance and a
> rough edge from cold air and forge smoke. Speaks German with a natural, native accent: northern-sounding standard German
> with a rolling r and hard consonants. Heavy, measured pacing, a proud and weathered authority, capable of thunderous
> laughter and sudden anger. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[proudly] Meine Ahnen haben diesen Pass mit bloßen Händen gehauen und ihre Söhne unter seinen Steinen begraben. Ihr
> kommt mit euren südlichen Bannern herauf und wollt Eisen? [firmly] Dann trinkt ihr mit uns, blutet mit uns und schwört
> beim Ahnenpfad. [short pause] Oder geht heim.“

**Modell & Einstellungen:** Eleven v3 *Natural* (`[laughs]`, `[shouting]`); Speed 0,95.
**Integration (nicht vom Nutzer):** Hochpass 60 Hz, leichte Tiefmitten-Fülle +1 dB @ 150 Hz, im Freien kurzer Berghall szenisch.

### B5 `char_sylaen` – Sylaen der Veyl, Druidin des Dornenbundes

| Merkmal | Festlegung |
|---|---|
| Rolle | Veyl-Druidin, Brücke zwischen Wald und Menschen |
| Alter | alterslos (klingt Ende 20 bis Mitte 30, spricht aber mit uralter Ruhe) |
| Timbre | weiche, hauchige, helle Stimme mit leichter Brüchigkeit wie trockenes Laub; sehr nahe |
| Akzent | Deutsch: weiche, singende Satzmelodie, ungewöhnliche Betonungen; nicht menschlich-urban |
| Tempo | langsam, fließend, mit langen Atembögen; Stille ist Teil der Rede |
| Emotion | ruhig, wissend, melancholisch; Zorn leise und eisig |
| Do | Naturbilder sanft betonen, Fragen statt Behauptungen |
| Don't | Esoterik-Flüstern, Kindlichkeit, Elfen-Klischee |

**Voice-Design-Prompt:**
> An ageless androgynous-leaning female voice that sounds around thirty yet carries centuries of calm. Soft, airy and
> light, with a delicate dry texture like rustling leaves, very intimate and close. Speaks German with a natural, native
> accent, in a gentle lilting cadence with unusual, musical stress patterns, unhurried, long flowing phrases and
> meaningful silences. Wise, melancholic, quietly otherworldly. Studio-quality recording, close microphone, no background
> noise.

**Beispieltext:**
> „[softly] Der Dornwald erinnert sich an jede Axt, Wächterin. Jede Wurzel, die du schneidest, ist ein Wort, das du nie
> zurücknehmen kannst. [pause] Geh leise hier … [whispers] die Wurzelmutter träumt, und ihre Träume sind grau geworden.
> Wenn sie so erwacht, bleibt keinem von uns ein Wald.“

**Modell & Einstellungen:** Eleven v3 *Natural* (`[whispers]` nur bei Geheimnissen); Speed 0,92.
**Integration (nicht vom Nutzer):** Veyl-Signatur (Leitfaden 8.3): Luft +2 dB ab 10 kHz, kurzer heller Hall.

### B6 `char_ophira` – Ophira Saelith-Varo, Händlerfürstin

| Merkmal | Festlegung |
|---|---|
| Rolle | Händlerfürstin des Saelith-Pakts |
| Alter | Mitte 40 |
| Timbre | warmer, samtiger Alt, voll und lächelnd, leicht kehliges Lachen |
| Akzent | Deutsch mit leichtem, melodischem Mittelmeer-Anklang, weltgewandt, mehrsprachig wirkend |
| Tempo | entspannt, verführerisch langsam bei Angeboten; schnell und präzise beim Feilschen |
| Emotion | amüsiert, kalkulierend, gastfreundlich mit Hintergedanken; ehrliche Verachtung für Fanatismus |
| Do | Zahlen und Preise genießen; Charme |
| Don't | Gier-Karikatur, Akzent übertreiben |

**Voice-Design-Prompt:**
> A worldly merchant princess in her mid-forties with a warm, velvety contralto voice, full and smiling, with a low
> throaty laugh. Speaks fluent German with a natural accent and a light, melodic Mediterranean lilt, cosmopolitan and
> refined. Relaxed, unhurried and charming when making offers, quick and precise when bargaining. Amused, calculating,
> hospitable with an edge. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[amused] Jeder in Eldraeth betet zu irgendetwas, Wächterin. Eure Leute zu einer Flamme, die Clans zu ihren Toten. In
> Saelith beten wir zur Flut, und die Flut ist ehrlich: Sie kommt immer zurück, um zu holen, was man ihr schuldet. Nun.
> [pause] Sprechen wir über Eure Schulden?“

**Modell & Einstellungen:** Eleven v3 *Natural* (`[laughs softly]`, `[amused]`); Speed 0,97.
**Integration (nicht vom Nutzer):** Standardkette, Hochpass 90 Hz, Wärme leicht.

### B7 `char_kassian` – Kassian Vale, Spionagemeister

| Merkmal | Festlegung |
|---|---|
| Rolle | Spionagemeister, undurchsichtig, wechselnde Loyalitäten |
| Alter | Ende 30 bis Mitte 40 (bewusst schwer einzuschätzen) |
| Timbre | glatter, leiser Tenor-Bariton, sehr nah am Mikrofon, fast vertraulich; wenig Brustklang |
| Akzent | Deutsch, schwer verortbar: gepflegtes, neutrales Hochdeutsch mit verschliffenen Kanten |
| Tempo | gleichmäßig ruhig, keine Hast, Sätze enden oft offen |
| Emotion | höflich, amüsiert, nie überrascht; Bedrohung durch Ruhe |
| Do | flüsternahe Intimität, Halbsätze, Ironie |
| Don't | Zischeln, Schurkenlachen |

**Voice-Design-Prompt:**
> A soft-spoken man in his early forties with a smooth, quiet tenor-baritone voice, very close to the microphone, intimate
> and confidential. Speaks German with a natural, native accent: refined, neutral standard German with softened edges,
> hard to place regionally. Even, unhurried delivery, never surprised, politely amused, sentences that trail off as if he
> knows more than he says. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[calmly] Ihr müsst nicht nach dem Messer suchen, Wächterin. Wollte ich Euch tot sehen, wärt Ihr vor drei Wochen
> bequem im Schlaf gestorben. [short pause] Nein … [quietly] ich bin hier, weil jemand am Königshof die Hohlen füttert.
> Und ich wüsste gern, wer.“

**Modell & Einstellungen:** Eleven v3 *Natural/Robust* (`[whispers]`, `[amused]`); Speed 0,96.
**Integration (nicht vom Nutzer):** Standardkette, Nahbesprechungseffekt erhalten (kein starker Hochpass, 70 Hz), sehr leichter Kompressor.

### B8 `char_deep_voice` – Die Stimme aus dem Tief

| Merkmal | Festlegung |
|---|---|
| Rolle | Manifestation des Hohlen Tiefs, älter als die Götter |
| Alter | zeitlos; weder männlich noch weiblich festgelegt |
| Timbre | tief, ruhig, hohl, mit flüsterndem Luftanteil; wirkt wie viele Stimmen in einer (entsteht in der Integration) |
| Akzent | Deutsch, kein Regionalakzent; extrem deutliche, langsame Artikulation, als lerne es menschliche Sprache gerade |
| Tempo | sehr langsam, lange Pausen, Silben gedehnt |
| Emotion | geduldig, fast zärtlich, vollkommen gleichgültig gegenüber Leid – Versuchung statt Drohung |
| Do | Intimität, Pausen, Fragen |
| Don't | Monster-Brüllen, Dämonen-Klischee |

**Voice-Design-Prompt:**
> An ancient, genderless voice, deep and calm, hollow and breathy, as if speaking from the bottom of a vast empty cavern.
> Speaks German with a natural, native pronunciation and no regional accent, extremely slow and precise articulation, each
> syllable placed carefully as though the speaker is relearning human words. Patient, intimate, almost tender, utterly
> indifferent to suffering. Seductive rather than threatening. Studio-quality recording, close microphone, no background
> noise.

**Beispieltext:**
> „[whispers] Du hast ein Feuer entzündet … und es Hoffnung genannt. [slowly] Ich erinnere mich, als eure Götter Kinder
> waren, Kleine. Ich erinnere mich, als sie verstummten. Jede Flamme erlischt. Jede Krone wird zu Asche. Komm herab …
> [softly] und ich zeige dir, was unter dem Schleier liegt.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Robust*, Speed 0,85; Flüsterpassagen mit `[whispers]`. Geliefert wird **eine** trockene Datei je Zeile; Schichtung und Effekte setzt die Integration.
**Integration (nicht vom Nutzer):** Drei-Schichten-Verfahren (Leitfaden 8.3). Niemals ohne Effekte verwenden.

### B9 `narrator` – Erzähler (Chronist)

| Merkmal | Festlegung |
|---|---|
| Rolle | Chronist von Eldraeth; erzählt aus späterer Rückschau |
| Alter | Mitte 60 |
| Timbre | tiefer, warmer, satter Bariton, sehr sauber, Erzähl-Resonanz ohne Theatralik |
| Akzent | Deutsch: neutrales, gebildetes Hochdeutsch (Hörbuch-Diktion) |
| Tempo | gemessen, getragen, klare Satzbögen; nie langsam-schleppend |
| Emotion | ernst, ehrfürchtig, leise Trauer, Würde; Hoffnung nur angedeutet |
| Do | Gewicht auf Namen und Zahlen, Pausen nach Kernsätzen |
| Don't | Trailer-Stimme, Märchenton, Dramatisierungsspitzen |

**Voice-Design-Prompt:**
> A distinguished male narrator in his mid-sixties with a deep, warm and resonant baritone voice, immaculately clear.
> Speaks German with a natural, native accent: neutral, educated standard German (Hochdeutsch) with audiobook diction.
> Measured, solemn storytelling pace with natural pauses, the voice of an old chronicler recounting history with reverence
> and quiet sorrow, never theatrical. Studio-quality audiobook recording, close microphone, no background noise, no
> reverb.

**Beispieltext:**
> „[solemn] Im achthundertsiebenundneunzigsten Jahr nach dem Fall war die Aschmark ein Land leerer Höfe und stummer
> Glocken. [pause] Von all den Dörfern, die einst die Südstraße säumten, hielt nur noch eines seine Feuer am Brennen.
> [slowly] Sein Name war Brennhollow.“

**Modell & Einstellungen:** **Eleven v3**, Stability-Modus *Natural* (bei zu starker Schwankung zwischen Takes *Robust*), Speed 0,95; Tags `[solemn]`, `[pause]`, `[slowly]`, `[softly]`. Multilingual v2 wird nicht mehr genutzt (versteht keine Audio-Tags). Aussprache-Wörterbuch (Leitfaden, Abschnitt 6) anwenden.
**Integration (nicht vom Nutzer):** nur Standardkette und minimale Wärme; kein Hall.

---

## Teil C – Bosse

### C1 `boss_hollow_king` – Der Hohle König (Varkhan Aurelith, der Letzte Sonnenkönig)

| Merkmal | Festlegung |
|---|---|
| Rolle | Endgegner; Varkhan selbst, gefangen in Solmere |
| Alter | sterblich etwa 50 gestorben, seit fast 900 Jahren untot |
| Timbre | majestätischer, einst goldener Bariton, jetzt brüchig, rissig wie verbranntes Pergament; trockener Atem |
| Akzent | Deutsch: archaisches, hochformelles königliches Register |
| Tempo | langsam, souverän, gelegentlich zu lange Pausen (Erinnerungslücken) |
| Emotion | Größenwahn, verletzter Stolz, tragische Sehnsucht nach dem Sonnenlicht; Wahnsinn bricht kurz durch |
| Do | Tragik und Würde; Wut als plötzlicher Riss |
| Don't | Zombie-Gegrunze, reines Monster |

**Voice-Design-Prompt:**
> A regal male voice of a fallen sun king, a once golden and commanding baritone now cracked and dry like burnt parchment,
> with a rasping, breathy texture. Speaks German with a natural, native accent, in an archaic, highly formal royal
> register. Slow, sovereign pacing with unsettling pauses, as if he forgets and then remembers. Proud, tragic and longing,
> with flashes of barely contained madness. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[proudly] Ich war die Sonne Aureths. Könige knieten vor meinem Schatten. Ich öffnete den Schleier, um mein Volk vor
> dem Tod selbst zu retten … und der Tod antwortete. [bitterly] Blick auf Solmere, kleine Wächterin, und sag mir noch
> einmal, deine Flamme könne überstrahlen, was ich verloren habe.“

**Modell & Einstellungen:** Eleven v3 *Natural/Robust* (`[laughs]` – ein hohles, kurzes Lachen; `[whispers]`); Speed 0,9.
**Integration (nicht vom Nutzer):** Leitfaden 8.3 (Deep-Voice-Layer auf Schlüsselwörtern, Thronsaal-Hall).

### C2 `boss_ashen_dragon` – Der Aschdrache Vyrmathax

| Merkmal | Festlegung |
|---|---|
| Rolle | Herrscher der Glutweite |
| Alter | uralt |
| Timbre | gewaltiger, langsamer, rauchiger Bass; jedes Wort wie aus einer Esse; Zischen auf S-Lauten |
| Akzent | Deutsch: altertümlich, gedehnt, fast gelangweilt-erhaben |
| Tempo | sehr langsam, mit Atemgeräuschen zwischen Sätzen |
| Emotion | sardonische Überlegenheit, Neugier auf Sterbliche, kalte Wut über die Störung |
| Do | Genuss an der eigenen Größe; leiser = bedrohlicher |
| Don't | Gebrüll statt Text, Comic-Drache |

**Voice-Design-Prompt:**
> An enormous ancient creature with a slow, smoky, cavernous bass voice, every word rumbling as if exhaled through embers,
> with a faint hiss on sibilants. Speaks German with a natural, native accent, in archaic, drawn-out, grandly bored
> diction. Very slow pacing with heavy breaths between sentences. Sardonic, superior and curious about mortals, with cold
> anger underneath. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[mockingly] Wieder klettert eine kleine Krone auf meinen Berg. Wie tapfer. Wie kurz. [pause] Ich sah Aureth von genau
> diesem Gipfel brennen und fand es schön. Sprich schnell, Wächterin. Meine Geduld ist kürzer als das Leben
> deinesgleichen – und für dich weit weniger wertvoll.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Robust*, Speed 0,85.
**Integration (nicht vom Nutzer):** Leitfaden 8.3 (Pitch −5 bis −7, Growl-Layer aus SFX, Höhlenhall).

### C3 `boss_root_mother` – Die Wurzelmutter

| Merkmal | Festlegung |
|---|---|
| Rolle | Naturentität des Dornwalds, von der Hohlung angegriffen |
| Alter | uralt |
| Timbre | tiefe, warme Frauenstimme, knarzend wie altes Holz, mehrstimmig (Chor-Dopplung durch die Integration) |
| Akzent | Deutsch, kein Regionalakzent; Wörter wie langsam wachsend, sehr langgezogene Vokale |
| Tempo | extrem langsam, lange Atembögen, Wortwiederholungen |
| Emotion | mütterlich und verwundet; im korrumpierten Zustand fiebrig, verwirrt, besitzergreifend |
| Do | Zärtlichkeit, die in Schmerz umschlägt |
| Don't | Hexen-Kreischen |

**Voice-Design-Prompt:**
> An ancient motherly female voice, deep, warm and earthy, with a creaking texture like old wood bending in the wind.
> Speaks German with a natural, native pronunciation and no regional accent, words stretched slowly as if growing, long
> sustained vowels and deep breaths. Tender and wounded, drifting into feverish confusion. Studio-quality recording, close
> microphone, no background noise.

**Beispieltext:**
> „[sorrowfully] Kinder … meine Kinder mit Äxten und Feuer. Ich nährte eure Väter. Ich barg eure Toten. Nun trinken
> graue Wurzeln von mir, und ich kann nicht sagen, welche Hände die euren sind. [pleading] Bleibt … bleibt bei mir.
> Lasst mich euch halten. Lasst die Erde euch wärmen.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Natural* (verderbte Zeilen *Creative*), Speed 0,85.
**Integration (nicht vom Nutzer):** Chor-Dopplung, Holz-Knarz-Layer, warmer Waldhall (Leitfaden 8.3).

### C4 `boss_pale_reeve` – Der Fahle Vogt (Mini-Boss Mission 1)

| Merkmal | Festlegung |
|---|---|
| Rolle | Hüter des Hohlschreins von Vael Dorn; sammelt Seelen als „Zehnt“ für das Hohle Tief |
| Alter | einst ein Steuervogt Aureths, Mitte 50, seit Jahrhunderten hohl |
| Timbre | dünne, trockene, hohe Männerstimme, kalt und pedantisch, rasselnder Atem, Flüsteranteil |
| Akzent | Deutsch: altmodisch-bürokratische Kanzleisprache, überpräzise Konsonanten |
| Tempo | gleichmäßig, aufzählend, wie beim Verlesen einer Liste; bei Schmerz plötzlich kreischend |
| Emotion | gefühllose Pflichterfüllung, morbider Ordnungssinn, Gier nach „ausstehenden“ Seelen |
| Motiv | Zehnt, Schuld, Register, Siegel, Zählen |
| Don't | brüllender Untoter; Humor nur unfreiwillig-makaber |

**Voice-Design-Prompt:**
> A thin, dry, high-pitched male voice of an ancient undead tax collector, cold and pedantic, with a rattling, breathy
> texture as if his lungs are full of dust. Speaks German with a natural, native accent, in old-fashioned bureaucratic
> diction with over-precise consonants. Even, listing cadence like reading names from a ledger; eerie calm that can snap
> into a shriek. Studio-quality recording, close microphone, no background noise.

**Beispieltext:**
> „[coldly] Brennhollow. Dreißig Seelen. Neunhundert Jahre im Rückstand. Das Hohle Tief ist geduldig, doch das Register
> muss ausgeglichen werden. [menacingly] Jeder Atemzug, den Ihr tut, ist geliehen, Wächterin. Ich bin gekommen, die
> Zinsen einzutreiben.“

**Modell & Einstellungen:** Eleven v3 *Natural* (Taunts/Ability), *Creative* (hit/death mit `[shrieks]`, `[groans]`); Speed 0,95.
**Integration (nicht vom Nutzer):** Hohle-Signatur + Vogt-Ergänzung (Leitfaden 8.3); Ablage unter `voices/units/`.

---

## Teil D – Fraktions-Sprachidentitäten und Archetypen

Jede Fraktion hat eine **Sprachfarbe** (Akzent, Wortwahl, Rhythmus) und eine **Klangsignatur** (Nachbearbeitung). Archetypen
sind wiederverwendbare Stimmen für Einheiten und Nebenfiguren. Einheiten der Vertical Slice sind den Archetypen fest zugeordnet.

### D1 `ember_crown` – Die Glutkrone von Valemark

- **Sprachfarbe:** deutsche Register nach Stand: Adel gehobenes Hochdeutsch (Bühnenaussprache), Ritter gepflegtes Hochdeutsch, Milizen/Leibeigene
  ländliche Umgangssprache mit leichter Dialektfärbung (süd- bzw. norddeutsch, nicht karikiert). Flammen-Metaphern („Haltet die Feuer am Brennen“, „Bei der Flamme“).
- **Klangsignatur:** trocken, direkt, keine Effekte; Mut und Erschöpfung.
- **Einheiten-Zuordnung (Vertical Slice):** `worker` → D1.1, `swordsman` → D1.2, `spearman` → D1.3, `archer` → D1.4, `knight` → D1.5.

#### D1.1 Arbeiter/Leibeigener – `AC_unit_worker` (Einheit `worker`)
- Mann, Mitte 40, rau-gutmütiger Bariton, breite ländliche Umgangssprache (süddeutsch gefärbt), bodenständig, etwas langsam, müde aber willig, trockene Bauernwitze.
- **Prompt:** "A weathered peasant labourer in his mid-forties with a rough, good-natured baritone voice. Speaks German
  with a natural, native accent and a broad rural, southern-leaning dialect coloring. Slow, plain, slightly tired but
  willing delivery, humble and practical, with a hint of grumbling humour. Close-mic studio-quality recording, no background
  noise."
- **Beispieltext:** „[wearily] Jo, ich hol das Holz. Wie gestern, wie morgen. [sighs] Aber wenn die Toten heut Nacht wiederkommen,
  versteck ich mich hinter dem Ritter, nicht vor ihm.“
- **Einstellungen:** v3 *Natural*, Speed 0,98.

#### D1.2 Soldat/Milizionär – `AC_unit_swordsman` (Einheit `swordsman`)
- Mann, Ende 20, kerniger, leicht heiserer Bariton, norddeutsch-städtische Färbung, zackig, entschlossen, mit Galgenhumor.
- **Prompt:** "A young militia swordsman in his late twenties with a firm, slightly hoarse baritone voice. Speaks German
  with a natural, native accent and a working-class northern urban coloring. Brisk, determined, soldierly delivery with a
  touch of gallows humour, confident but not arrogant. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[dryly] Schwert scharf, Schild verbeult, und der Feldwebel brüllt immer noch. Muss ein normaler Tag in der Aschmark
  sein. [confidently] Zeig mir was Totes, und ich mach's noch toter.“
- **Einstellungen:** v3 *Natural*, Kampfrufe *Creative*.

#### D1.3 Veteran/Speerträger – `AC_unit_spearman` (Einheit `spearman`)
- Mann, Mitte 50, tiefer, rauer, ruhiger Bass-Bariton, ländlich-bodenständige Färbung, wortkarg, unerschütterlich, väterlich gegenüber Jüngeren.
- **Prompt:** "A grizzled veteran spearman in his mid-fifties with a deep, gravelly, calm bass-baritone voice. Speaks
  German with a natural, native accent and a rural, down-to-earth coloring. Laconic, steady and unshakeable, few words, each
  one weighty, a protective old soldier. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[commanding] Haltet die Linie, Jungs. Füße fest, Speer nach vorn, lasst sie kommen. [calmly] Ich hab bessere Männer
  begraben als diese Dinger, und ich überlebe die auch.“
- **Einstellungen:** v3 *Natural/Robust*, Speed 0,95.

#### D1.4 Bogenschützin – `AC_unit_archer` (Einheit `archer`)
- Frau, Anfang 20, helle, scharfe, klare Stimme, leichte ländliche Färbung, wach, schnell, keck, präzise.
- **Prompt:** "A sharp-eyed young woman archer in her early twenties with a bright, clear and focused voice. Speaks
  German with a natural, native accent and a light rural coloring. Quick, alert and cheeky delivery, precise and confident,
  a hunter's calm when aiming. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[confidently] Wind von Osten, zwei Finger links. Ich treff eine Krähe auf hundert Schritt, bevor du fertig gezeigt
  hast. [teasingly] Na los, gib mir was, das sich lohnt.“
- **Einstellungen:** v3 *Natural*, Speed 1,03.

#### D1.5 Ritter (schwere Kavallerie) – `AC_unit_knight` (Einheit `knight`)
- Mann, Anfang 40, voller, edler Bariton mit Brustklang, gehobenes Hochdeutsch (Bühnenaussprache), stolz, ritterlich-formell, etwas pathetisch (bewusst kontrastiert zu Isolde).
- **Prompt:** "A noble armoured knight in his early forties with a full, resonant and commanding baritone voice. Speaks
  German with a natural, native accent in refined, noble stage-standard pronunciation. Proud, chivalrous and formal delivery
  with a touch of grandeur, unwavering and honourable. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[solemn] Bei der Ewigen Flamme und dem Eid meines Hauses: Kein Schatten überquert dieses Feld, solange ich im Sattel
  sitze. [commanding] Blast das Horn, und lasst die Toten lernen, was ein Ansturm Valemarks bedeutet.“
- **Einstellungen:** v3 *Natural*, Speed 0,97.
- **Integration (nicht vom Nutzer):** dezente Helm-Nähe optional (Bandpass-Andeutung 120 Hz–9 kHz).

#### D1.6 Flammenpriesterin (spätere Einheit) – `AC_ember_crown_priest`
- Frau, Ende 30, warmer, tragender Mezzosopran, gebildetes Hochdeutsch, feierlich, mitfühlend, leicht singender Gebetston.
- **Prompt:** "A devout priestess of a fire faith in her late thirties with a warm, carrying mezzo-soprano voice. Speaks
  German with a natural, native accent, educated standard German. Solemn and compassionate, with a gentle, almost chant-like
  cadence in blessings. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[gently] Steht im Licht und fürchtet euch nicht. Die Flamme hat dunklere Nächte gesehen und jede durchbrannt.
  [firmly] Hebt eure Schilde. Ich halte eure Herzen warm.“

#### D1.7 Adliger General (Nebenfigur) – `AC_ember_crown_general`
- Mann, Ende 50, schneidender, harter Bariton, militärisch-knapper Adel, ungeduldig, an Gehorsam gewöhnt.
- **Prompt:** "A stern aristocratic general in his late fifties with a hard, cutting baritone voice, clipped upper-class
  military diction, speaks German with a natural, native accent, impatient and used to instant obedience. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[angry] Ich bin nicht drei Tage durch Asche und Regen geritten, um Ausreden zu hören. [commanding] Reihen auf dem
  Kamm bilden, Kavallerie an die linke Flanke, und wer zuerst bricht, steht mir persönlich Rede und Antwort.“

### D2 `skarn_clans` – Die Clans von Skarnheim

- **Sprachfarbe:** nordische Färbung, rollende R, kurze, harte Sätze, Ahnen- und Eisenbilder („Bei den Knochen meiner Väter“, „Eisen vergisst nicht“).
- **Klangsignatur:** voller, brustbetonter Klang; Kampfrufe als Gruppenrufe gut layerbar; optional leichter Berghall in Szenen.

#### D2.1 Clan-Krieger (schwere Infanterie) – `AC_skarn_clans_warrior`
- **Prompt:** "A towering northern clan warrior in his thirties with a deep, rough baritone voice. Speaks German with a
  natural, native accent, a strong northern coloring with a rolling r. Blunt, loud, proud delivery, short hard sentences,
  battle-hungry. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[gruffly] Meine Axt ist schwer und meine Geduld leicht. Zeig mir den Feind und tritt zurück, Südländer. [proudly] Die
  Ahnen sehen zu, und ich werde sie heute nicht beschämen.“

#### D2.2 Ahnenruferin / Skaldin (Priesterin/Magierin) – `AC_skarn_clans_skald`
- **Prompt:** "An older northern woman in her fifties, a keeper of ancestral songs, with a low, resonant, slightly smoky
  voice. Speaks German with a natural, native accent, a northern coloring with a rolling r. Rhythmic, chant-like, solemn
  delivery, as if every sentence is part of an old saga. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[solemn] Hört mich, Väter unter dem Berg. Hört mich, Mütter in den gefrorenen Hallen. [pause] Leiht unseren Armen
  Eisen und unserem Blut Frost, denn die Toten sind gekommen, und die Lebenden müssen antworten.“

#### D2.3 Schildmaid (Elite-Infanterie) – `AC_skarn_clans_shieldmaiden`
- **Prompt:** "A fierce young shieldmaiden in her late twenties with a strong, husky mid-range voice and a northern
  coloring, speaks German with a natural, native accent. Confident, mocking and fearless, laughs in danger. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[shouting] Schilde hoch, Kinn hoch, und wagt es nicht, vor mir zu fallen. [laughs] Ich hab ein Fass Met gewettet,
  dass ich mehr Schädel breche als ihr alle, und ich will es einstreichen.“

#### D2.4 Schmied/Arbeiter – `AC_skarn_clans_smith`
- **Prompt:** "A broad middle-aged northern blacksmith with a booming, warm bass voice. Speaks German with a natural,
  native accent and a thick northern coloring with a rolling r, cheerful, loud and gruff. Close-mic studio-quality
  recording, no background noise."
- **Beispieltext:** „[calmly] Eisen erinnert sich an jeden Hammerschlag, Freund. [warmly] Gib mir Kohle, gib mir Erz und gib mir Ruhe, und
  ich geb dir Klingen, die deine Enkel überdauern.“

### D3 `thorn_covenant` – Der Dornenbund

- **Sprachfarbe:** Menschen mit sanftem, singendem Lilt; Veyl mit hauchiger, ungewöhnlich betonter Melodik. Naturbilder,
  Rätselhaftes, leise Befehle (Guerilla).
- **Klangsignatur:** Menschen trocken-natürlich; Veyl mit Luft- und Hallsignatur (Leitfaden 8.3).

#### D3.1 Druide (Magier/Priester) – `AC_thorn_covenant_druid`
- **Prompt:** "An elderly forest druid in his seventies with a soft, deep and earthy voice, speaks German with a natural,
  native accent and a gentle, singing lilt, slow and wise, warm but stern when the forest is threatened. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[gently] Jeder Same ist ein Versprechen und jede faule Wurzel ein gebrochenes. [pause] Der Grüne Kreis verlangt wenig
  von dir: Nimm nur, was nachwächst, und verbrenne nichts, was du nicht neu pflanzen kannst.“

#### D3.2 Veyl-Späherin (Assassine/Plänkler) – `AC_thorn_covenant_veyl_scout`
- **Prompt:** "A slender otherworldly young female forest scout with a light, airy, breathy voice, quick and whispery,
  speaks German with a natural, native accent, with a strange musical cadence and unusual stress on words, alert and
  elusive. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[whispers] Sie sehen uns nie, sie hören nur das Laub. Drei hinter der Eiche, zwei am Bach, einer schläft schon für
  immer. [mischievously] Soll ich zwei daraus machen?“

#### D3.3 Waldläufer (Bogenschütze) – `AC_thorn_covenant_ranger`
- **Prompt:** "A lean human woodsman in his thirties with a calm, low and quiet voice, speaks German with a natural, native
  accent and a soft lilt, patient and observant, speaks as if not to scare game. Close-mic studio-quality recording, no
  background noise."
- **Beispieltext:** „[whispers] Leise jetzt. Der Wind hat gedreht, und das Wild steht still. Etwas in der Senke riecht nach Grabeserde.
  [short pause] Ich nehme den ersten Schuss. Du den zweiten, falls es einen gibt.“

### D4 `saelith_compact` – Der Saelith-Pakt

- **Sprachfarbe:** weltgewandt, melodischer Mittelmeer-Anklang (leicht, nicht karikiert), Handels- und Seefahrtsbegriffe
  („ein fairer Preis“, „bei der Flut“), Söldner pragmatisch und zynisch.
- **Klangsignatur:** trocken, präsent; Hafen-/Deckszenen mit leichter Außenakustik.

#### D4.1 Händler – `AC_saelith_compact_merchant`
- **Prompt:** "A smooth, cheerful merchant in his forties with a warm, honeyed tenor voice. Speaks German with a natural
  accent and a light melodic Mediterranean lilt, fast-talking, persuasive and flattering, always smiling. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[cheerfully] Seide von den östlichen Inseln, Gewürze, die einen König zum Weinen brächten, und für dich, mein Freund,
  ein Preis so fair, dass selbst die Flut erröten würde. [slyly] Natürlich Vorauskasse.“

#### D4.2 Söldnerhauptfrau (General/Kommandantin) – `AC_saelith_compact_captain`
- **Prompt:** "A hardened mercenary captain in her late thirties with a low, smoky, confident alto voice, speaks German with
  a natural accent and a light Mediterranean lilt, cynical, dry and businesslike, commanding without raising her voice.
  Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[coldly] Loyalität ist was für Priester und Hunde. Meine Kompanie kämpft für Münze, und zwar sehr gut. [dryly] Zahlt
  pünktlich, und wir sterben für Euch. Zahlt zu spät, und wir führen dieses Gespräch mit Eurem Feind.“

#### D4.3 Gezeitenpriesterin (Magierin) – `AC_saelith_compact_tide_priestess`
- **Prompt:** "A serene sea priestess in her thirties with a cool, flowing, slightly breathy soprano-mezzo voice, speaks
  German with a natural accent and a gentle Mediterranean lilt, rhythmic like waves, calm and mysterious. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[solemn] Die Flut nimmt, und die Flut kehrt zurück. Was wir dem Meer opfern, erinnert das Meer. [quietly] Tretet vom
  Wasser zurück, Kapitän. Heute ist das Meer hungrig.“

#### D4.4 Matrose/Arbeiter – `AC_saelith_compact_sailor`
- **Prompt:** "A boisterous young sailor in his twenties with a bright, loud, cheeky tenor voice. Speaks German with a
  natural accent, casual dockside slang and a light Mediterranean lilt, energetic and joking. Close-mic studio-quality
  recording, no background noise."
- **Beispieltext:** „[cheerfully] Taue holen, Kisten festzurren, und wenn die Kapitänin fragt, hab ich die ganze Zeit gearbeitet. [laughs
  softly] Riechst du das? Das ist Salz, Geld und das Problem von jemand anderem.“

### D5 `hollowed` – Die Hohlen

- **Sprachfarbe:** Die Hohlen waren Menschen. Einfache Einheiten sprechen **keine Sätze**, sondern Laute und **Wortfetzen aus
  ihrem früheren Leben** („… Heim …“, „… kalt …“, „… Mutter …“). Kultisten der Leeren Liturgie sprechen in liturgischer Litanei.
- **Klangsignatur:** Hohle-Kette (Leitfaden 8.3). Basisaufnahmen trocken, Effekte setzt immer die Integration.
- **Einheiten-Zuordnung:** `hollow_thrall` → D5.1, `hollow_archer` → D5.2, `grave_brute` → D5.3, Boss `boss_pale_reeve` → C4.

#### D5.1 Hohlknecht – `AC_unit_hollow_thrall` (Einheit `hollow_thrall`)
- **Prompt:** "A hollow, emaciated male voice of a reanimated villager, dry and breathy, with a weak rattling rasp in the
  throat, murmuring fragments of words as if half-remembering a past life. Slow, confused, mournful, unsettling. Murmurs
  German word fragments with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[whispers] [sorrowfully] Heim … die Felder sind kalt … ich finde die Tür nicht … [rasping] Mutter, das Feuer ist aus
  … so hungrig … so kalt …“
- **Einstellungen:** v3 *Creative*; Pitch −2 und Hohle-Kette setzt die Integration.

#### D5.2 Knochenschütze – `AC_unit_hollow_archer` (Einheit `hollow_archer`)
- **Prompt:** "A dry, whispering, skeletal voice with clicking, rattling consonants and almost no breath, thin and airy,
  hissing fragments of words, cold and precise. Hisses German word fragments with a natural, native pronunciation. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[hissing] Ssseh dich … ssseh die Warmen … [whispers] noch einer … markier das Herz … es hört nie auf zu zählen … noch
  einer …“
- **Einstellungen:** v3 *Creative* mit `[whispers]`; Integration: Hohle-Kette + hochfrequentes Knochenklacken (SFX-Layer, −18 dB).

#### D5.3 Grabkoloss – `AC_unit_grave_brute` (Einheit `grave_brute`)
- **Prompt:** "A huge, deep, guttural creature voice of a bloated undead giant, slow heavy breathing, wet low rumbling
  groans, able to bellow single words with crushing force. Bellows single German words with a natural, native pronunciation.
  Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[growls] Hhhrrrgh … zerquetschen … zerquetschen … runter … bleib unten … [groans] hhhuuurrgh … ihr alle … in die Erde
  …“
- **Einstellungen:** v3 *Creative*; SFX-Layer „massive undead giant groan“ und Pitch −4 bis −6 setzt die Integration (nicht zu liefern).

#### D5.4 Kultist der Leeren Liturgie (Priester, spätere Einheit) – `AC_hollowed_cultist`
- **Prompt:** "A fanatical human cultist in his forties with a thin, intense, slightly trembling tenor voice, whispering and
  chanting in feverish liturgical rhythm, ecstatic and frightening. Speaks German with a natural, native accent. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[monotone] Leere das Gefäß. Leere den Namen. Leere das Herz, und das Tief wird es füllen. [whispers] Unter dem
  Schleier gibt es keinen Hunger, keine Trauer, keine Flamme. Nur die Stille. Nur die Stille.“

#### D5.5 Hohler Wiedergänger-Magier (spätere Einheit) – `AC_hollowed_revenant_mage`
- **Prompt:** "An ancient undead sorceress with a hollow, echoing, breathy female voice, slow and elegant, cold and
  aristocratic, each word drawn out like smoke. Speaks German with a natural, native accent. Close-mic studio-quality
  recording, no background noise."
- **Beispieltext:** „[wistfully] Einst war ich Hofmagierin Aureths. Ich kannte tausend Namen des Lichts. [coldly] Nun kenne ich nur ein
  Wort der Dunkelheit, und es genügt, dich aufzulösen.“

### D6 `ashen_legion` – Die Aschenlegion

- **Sprachfarbe:** archaisches, militärisch-formelles Deutsch der Ritter Aureths; Eid- und Pflichtformeln, „wir“ statt „ich“ (Legion
  als Kollektiv), keine Verschleifungen („Wir weichen nicht“).
- **Klangsignatur:** Helm-Resonanz + Glut-Knistern (Leitfaden 8.3); Stimmen tiefer und metallischer als die Glutkrone.

#### D6.1 Legionär (Soldat) – `AC_ashen_legion_legionnaire`
- **Prompt:** "A cursed armoured soldier with a deep, flat, emotionless baritone voice, speaking in archaic military
  formality, slow and mechanical as if repeating an oath for centuries, slightly hoarse and dry. Speaks German with a
  natural, native accent. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[monotone] Wir sind die Legion der Sonne. Wir marschieren seit achthundert Jahren. Wir ermüden nicht. Wir weichen
  nicht. [slowly] Wir erinnern uns nicht an die Gesichter unserer Kinder.“

#### D6.2 Aschenritter-Kommandant (General) – `AC_ashen_legion_commander`
- **Prompt:** "A towering cursed knight commander with a commanding, resonant, dark bass-baritone voice, archaic and noble,
  cold fury held in iron discipline, measured and imperious. Speaks German with a natural, native accent. Close-mic
  studio-quality recording, no background noise."
- **Beispieltext:** „[commanding] Kniet nieder, Erben von Usurpatoren. Der Hohle König hat nicht abgedankt. [coldly] Jeder Acker zwischen
  Solmere und dem Meer gehört ihm von Rechts wegen, und die Legion ist gekommen, ihn zurückzufordern, Feld um brennendes
  Feld.“

#### D6.3 Legions-Herold (Priester der Sonnenhäresie) – `AC_ashen_legion_herald`
- **Prompt:** "A gaunt herald with a high, piercing, fervent tenor voice, archaic proclamatory style, fanatical devotion,
  like announcing the return of a god-king. Speaks German with a natural, native accent. Close-mic studio-quality recording,
  no background noise."
- **Beispieltext:** „[triumphantly] Frohlockt, denn der Sonnenkönig kehrt zurück! Die Asche wird zu Gold, die Toten erheben sich in
  Herrlichkeit, und jede falsche Krone schmilzt in seinem Licht. [shouting] Öffnet eure Tore und werdet verschont!“

---

## Teil E – Casting-Übersicht (Kurzreferenz)

| Stimme (Library-Name) | ID/Einheit | Fraktion | Modell primär | Alter/Geschlecht | Akzent |
|---|---|---|---|---|---|
| `AC_hero_isolde` | `hero_isolde` | ember_crown | v3 | 34 / w | neutral, leicht norddeutsch |
| `AC_char_king_aldric` | `char_king_aldric` | ember_crown | v3 | ~68 / m | gehobenes Hochdeutsch, königlich |
| `AC_char_mireille` | `char_mireille` | ember_crown | v3 | ~42 / w | scharfes Hochdeutsch |
| `AC_char_hal_venn` | `char_hal_venn` | ember_crown | v3 | ~62 / m | ländlich gefärbt (nord) |
| `AC_char_torvald` | `char_torvald` | skarn_clans | v3 | ~55 / m | nordisch, rollendes R |
| `AC_char_sylaen` | `char_sylaen` | thorn_covenant | v3 | alterslos / w | sanfter, singender Lilt |
| `AC_char_ophira` | `char_ophira` | saelith_compact | v3 | ~45 / w | leichter Mittelmeer-Lilt (Deutsch) |
| `AC_char_kassian` | `char_kassian` | – | v3 | ~40 / m | unverortbar, neutral |
| `AC_char_deep_voice` | `char_deep_voice` | hollowed | v3 | zeitlos / neutral | keiner |
| `AC_narrator` | `narrator` | – | v3 | ~65 / m | neutral, gebildet (Hochdeutsch) |
| `AC_boss_hollow_king` | `boss_hollow_king` | ashen_legion/hollowed | v3 | untot / m | archaisch königlich |
| `AC_boss_ashen_dragon` | `boss_ashen_dragon` | – | v3 | uralt / m | archaisch |
| `AC_boss_root_mother` | `boss_root_mother` | thorn_covenant (korrumpiert) | v3 | uralt / w | keiner |
| `AC_boss_pale_reeve` | `boss_pale_reeve` | hollowed | v3 | untot / m | bürokratisch, Kanzleisprache |
| `AC_char_wendel` | `char_wendel` | – (Graue Kompanie) | v3 | ~55 / m | rau, leicht norddeutsch gefärbt |
| `AC_char_nel` | `char_nel` | – | v3 | ~8 / w | neutral, leise |
| `AC_boss_stone_oath` | `boss_stone_oath` | – (Skarnheim, verdreht) | v3 | zeitlos / m | archaisch |
| `AC_boss_reef_mistress` | `boss_reef_mistress` | hollowed (Saelith) | v3 | zeitlos / w | mediterran, unter Wasser |
| `AC_unit_worker` | `worker` | ember_crown | v3 | ~45 / m | ländlich, süddeutsch gefärbt |
| `AC_unit_swordsman` | `swordsman` | ember_crown | v3 | ~28 / m | norddeutsch-städtisch |
| `AC_unit_spearman` | `spearman` | ember_crown | v3 | ~55 / m | ländlich-bodenständig |
| `AC_unit_archer` | `archer` | ember_crown | v3 | ~22 / w | ländlich leicht gefärbt |
| `AC_unit_knight` | `knight` | ember_crown | v3 | ~42 / m | gehobenes Hochdeutsch |
| `AC_unit_hollow_thrall` | `hollow_thrall` | hollowed | v3 / SFX | untot / m | – |
| `AC_unit_hollow_archer` | `hollow_archer` | hollowed | v3 / SFX | untot / neutral | – |
| `AC_unit_grave_brute` | `grave_brute` | hollowed | v3 / SFX | untot / m | – |

**Unterscheidbarkeit Glutkrone prüfen:** worker (tief-gemütlich, langsam) ≠ spearman (tiefst, ruhig, wortkarg) ≠ swordsman
(mittel, zackig) ≠ knight (voll, edel) ≠ archer (hell, schnell) ≠ Isolde (rau-tief weiblich, trocken).

---

## Teil F – Zusätzliche Sprecher der Echtzeit-Kampagne

Vier Sprecher, die erst die Kampagnenmissionen (`assets/data/campaign/`) einführen. Ihre Zeilen stehen in
`docs/audio/elevenlabs_dialogues.md`, Teil 3. Alle Texte werden **auf Deutsch** generiert; die (englisch formulierten) Voice-Design-Prompts
beschreiben nur die Stimme und enthalten jeweils die Angabe zur deutschen Sprache.

### F1 `char_wendel` – Ser Wendel der Graue (Raubritter, später Hauptmann der Grauen Kompanie)

| Merkmal | Festlegung |
|---|---|
| Rolle | Banditenhauptmann der Grauen Straße (Mission 3); je nach Entscheidung gehängt oder Verbündeter |
| Alter | Mitte 50 |
| Timbre | heiser, rauchig, tiefer Bariton mit Kratzen; klingt, als habe er zu lange im Freien geschlafen |
| Akzent | Deutsch: rau, bodenständig, leicht norddeutsch gefärbt |
| Tempo | gemächlich, trockene Pausen vor Pointen |
| Emotion | müde, spöttisch, ehrlich bis zur Selbstverachtung |
| Do | trockener Galgenhumor, kurze Seufzer |
| Don't | Schurkengelächter, Pathos |

**Voice-Design-Prompt:**
> A weathered man in his mid fifties with a hoarse, smoky baritone, gravel in the throat, a dry and tired tone with
> sarcastic edges. Slow, unhurried pacing with pauses before the punchline. Speaks German with a natural, native accent:
> rough, down-to-earth, northern-leaning colloquial German. Studio-quality recording, close microphone, no background
> noise.

**Beispieltext:** „[tired] Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. [pause] Aber ich kenne jeden
  Pfad bis Kaldris.“

**Modell & Einstellungen:** Eleven v3 *Natural*; Speed 0,95. **Integration (nicht vom Nutzer):** Standardkette, leichte Wärme (200 Hz), kein Effekt.

### F2 `char_nel` – Nel (Waise aus Harrowmere, hört das Tiefe)

| Merkmal | Festlegung |
|---|---|
| Rolle | Mädchen, das im Schlaf mit der Stimme aus dem Tief spricht; später „Hörerin“ am Riss und beim Hohlen König |
| Alter | etwa 8 Jahre (in späteren Missionen derselbe Charakter, Stimme unverändert, da im Spiel nur Kurzzeilen) |
| Timbre | helle, ruhige Kinderstimme, fast ohne Betonung; leicht abwesend |
| Akzent | Deutsch: neutral, leise, schlichte Kindersprache |
| Tempo | langsam, gleichmäßig; Sätze enden ohne Frage-Intonation |
| Emotion | still, nicht ängstlich; unheimlich durch Gelassenheit |
| Do | monotone Gelassenheit, flüsternahe Nähe |
| Don't | niedliches Kichern, Weinen |

**Voice-Design-Prompt:**
> A quiet eight-year-old girl with a clear, calm child's voice, almost flat intonation, slightly distant and dreamlike,
> speaking slowly and evenly without rising question tones. Speaks German with a natural, native accent, plain child's
> diction. Studio-quality recording, close microphone, no background noise.

**Beispieltext:** „[softly] Der Mann im Moor hat gesagt, ich soll dir ausrichten: [pause] Es war nett, dich kennenzulernen.“

**Modell & Einstellungen:** Eleven v3 *Natural*; Speed 0,92; Stability hoch. **Integration (nicht vom Nutzer):** Standardkette, bei Szenen mit der Stimme aus dem Tief ein leichter Hall-Schweif (0,6 s).

### F3 `boss_stone_oath` – Gharun, der Steinerne Eid (Wächter von Oruuns Schrein)

| Merkmal | Festlegung |
|---|---|
| Rolle | Boss der Mission 21; ein in Runenstein gebundener Eid, vom Tiefen verdreht |
| Alter | zeitlos |
| Timbre | extrem tiefer, langsamer Bass, mahlend wie Mühlstein, mit natürlichem Höhlenhall |
| Akzent | Deutsch, kein Regionalakzent; archaisch, wenige Silben je Atemzug |
| Tempo | sehr langsam, gewichtige Pausen |
| Emotion | unerbittlich, pflichttreu bis zur Selbstvergessenheit; kein Hass |
| Do | langgezogene Vokale, Stein-Resonanz (Integration) |
| Don't | Schreien, Geflüster |

**Voice-Design-Prompt:**
> An immensely deep, ancient bass voice, slow and grinding like a millstone, with long vowels and heavy pauses, stern and
> dutiful without hatred. Sounds as if spoken inside a stone hall. Speaks German with a natural, native pronunciation,
> archaic and heavy. Studio-quality recording, no background noise.

**Beispieltext:** „[commanding] Wer wagt es, den Eid zu stören? [slowly] Ich wurde gebunden, zu bewahren – und ich bewahre.“

**Modell & Einstellungen:** Eleven v3, Stability-Modus *Robust*, Speed 0,85. **Integration (nicht vom Nutzer):** Pitch −3 Halbtöne, Hall 2,8 s (Steinhalle), Tiefpass 6 kHz, leichter Kompressor.

### F4 `boss_reef_mistress` – Isarre Mol, Herrin des Schwarzen Riffs

| Merkmal | Festlegung |
|---|---|
| Rolle | Boss der Mission 14; ertrunkene Gezeitenpriesterin im Bund mit dem Hohlen Tief |
| Alter | zeitlos (zu Lebzeiten Mitte 40) |
| Timbre | tiefe Altstimme, nass und gurgelnd, mehrstimmig überlagert wie Wellen, die einander überlaufen |
| Akzent | Deutsch, leicht mediterran gefärbt (Saelith), unter dem Wasser verschliffen |
| Tempo | gleitend, Wellenrhythmus; Sätze enden mit auslaufendem Atem |
| Emotion | mütterlich-bedrohlich, belustigt; Fürsorge als Drohung |
| Do | Unterwasser-Textur, Flüstern neben Gesang |
| Don't | Zischeln, Monsterknurren |

**Voice-Design-Prompt:**
> A low contralto woman's voice, wet and gurgling, as if speaking from under shallow water, layered with a faint second
> voice that follows a beat behind like waves washing back. Speaks German with a natural accent and a trace of a
> Mediterranean lilt. Flowing, motherly and menacing. Studio-quality recording, no background noise.

**Beispieltext:** „[mockingly] Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. [coldly] Das Meer hat mir jedes meiner Kinder
  zurückgegeben.“

**Modell & Einstellungen:** Eleven v3 *Creative* (`[whispers]`). Geliefert wird nur die trockene Hauptstimme; die tiefere Zweitschicht erzeugt die Integration aus derselben Datei. **Integration (nicht vom Nutzer):** Chorus 12 %, Wasser-Hall (1,8 s), Tiefpass 8 kHz.

---

## Teil G – Stimmprofile der Echtzeit-Militäreinheiten

Profile für die Einheiten aus `docs/audio/elevenlabs_unit_voices.md`, Abschnitte 12 und 14 (im Manifest als `voice.<einheit>.<situation>`).
Format wie Teil D. Jede Stimme heißt `AC_unit_<id>`. Die Helden `hero_hal_venn` und `hero_torvald` verwenden die Profile B3 (`char_hal_venn`)
und B4 (`char_torvald`); `bat_swarm` hat keine Stimme (reiner Sound-Effects-Entwurf).

### G1 `AC_unit_crossbowman` – Armbrustschütze (Einheit `crossbowman`, Glutkrone)
- Mann, Anfang 40, trockener, ruhiger Bariton, nüchterner Söldnerveteran aus den Grenzgarnisonen, knappe Sätze, spricht über Reichweite wie über Handwerk.
- **Prompt:** "A seasoned crossbow mercenary in his early forties with a dry, level baritone. Speaks German with a natural, native accent and a plain northern coloring. Short, matter-of-fact sentences, sober and unhurried, speaking of range and aim like a craftsman talks about tools, with a faint grim humour. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[calmly] Atem anhalten, warten bis der Wind nachlässt, dann abdrücken. [short pause] Schnell schießen kann jeder. Ich
  schieße einmal.“
- **Einstellungen:** v3 *Natural*, Speed 1,0.

### G2 `AC_unit_halberdier` – Hellebardier (Einheit `halberdier`, Glutkrone)
- Mann, Mitte 40, tiefer, bärbeißiger Bass-Bariton, Gardist und Pferdebrecher; liebt Formationen, mag keine Reiter.
- **Prompt:** "A gruff palace guard halberdier in his mid-forties with a deep, grumbling bass-baritone. Speaks German with a natural, native accent, clipped and disciplined, with a northern coloring. Slow, heavy and authoritative, barking short orders, with the stubborn pride of a man who has braced against many cavalry charges. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[firmly] Schäfte auf den Boden, Spitzen raus, Schulter an Schulter. [calmly] Lasst die Pferde kommen. Irgendwann
  bleiben sie immer stehen.“
- **Einstellungen:** v3 *Natural*, Speed 0,95.

### G3 `AC_unit_shieldbearer` – Schildträger (Einheit `shieldbearer`, Glutkrone)
- Mann, ca. 38, sehr tiefer, ruhiger Bass; geduldiger, schwerer Bollwerkkämpfer, spricht wenig, stoisch.
- **Prompt:** "A heavy shieldbearer in his late thirties with a very deep, calm bass voice, slow and stoic, speaking little and never raising his voice above a firm rumble. Speaks German with a natural, native accent, plain, steady, patient, with a northern coloring, the quiet confidence of a living wall. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[firmly] Steht hinter mir. Der Schild ist schwer, aber ich bin schwerer. [calmly] Hier kommt nichts durch.“
- **Einstellungen:** v3 *Natural*, Speed 0,92.

### G4 `AC_unit_scout_rider` – Spähreiter (Einheit `scout_rider`, Glutkrone)
- Mann, Anfang 20, heller, flinker Tenor, übermütig, lacht gern, spricht schnell.
- **Prompt:** "A young border scout rider in his early twenties with a bright, quick tenor voice. Speaks German with a natural, native accent and a lively regional coloring. Fast, breathless, cheeky and cheerful, laughing easily, full of restless energy as if speaking from horseback. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[cheerfully] Der Kamm ist frei, die Straße ruhig, und mein Pferd langweilt sich. [confidently] Sagt es, und ich bin
  da, bevor Ihr zu Ende gefragt habt.“
- **Einstellungen:** v3 *Creative*, Speed 1,08.

### G5 `AC_unit_horse_archer` – Berittene Bogenschützin (Einheit `horse_archer`, Glutkrone)
- Frau, Ende 20, geschmeidiger heller Alt, selbstsichere, leicht spöttische Steppen-Söldnerin.
- **Prompt:** "A confident mounted archer, a woman in her late twenties with a smooth, light alto voice. Speaks German with a natural accent and a faintly foreign, lilting coloring. Playful, teasing and self-assured, speaking quickly and lightly as if loosing arrows at a gallop. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[confidently] Ihr geht, ich reite, und trotzdem schieße ich zuerst. [teasingly] Versucht mitzuhalten.“
- **Einstellungen:** v3 *Creative*, Speed 1,05.

### G6 `AC_unit_battle_mage` – Kampfmagier (Feuerschule) (Einheit `battle_mage`, Glutkrone)
- Mann, Anfang 30, höflicher, gespannter Tenor-Bariton, diszipliniert, unterdrückte Hitze in der Stimme (Flammenakademie von Kaldris).
- **Prompt:** "A disciplined fire mage trained at an academy, a man in his early thirties with a polite, precise and slightly tense baritone-tenor. Speaks German with a natural, native accent, educated standard German. Controlled and formal, but with suppressed heat under every sentence, as if each word could ignite. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[calmly] Geht bitte beiseite. Ich beherrsche die Flamme. [dryly] Ich möchte nur nicht gefragt werden, wie sehr.“
- **Einstellungen:** v3 *Creative*, Speed 1,0.

### G7 `AC_unit_priest` – Priester der Flamme (Einheit `priest`, Glutkrone)
- Mann, Ende 50, sanfter, warmer Bariton, Dorfkapellenpriester, zitiert Liturgie, ängstlich, hilft trotzdem.
- **Prompt:** "A gentle village chapel priest in his late fifties with a soft, warm baritone. Speaks German with a natural, native accent and a kind, rural coloring. Calm and reassuring, quoting liturgy in a half-sung murmur, with a tremor of fear underneath that he overcomes out of duty. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[gently] Die Flamme verlangt nicht, dass wir keine Angst haben. Nur, dass wir sie am Brennen halten. [softly] Kommt,
  lasst mich die Wunde sehen.“
- **Einstellungen:** v3 *Natural*, Speed 0,96.

### G8 `AC_unit_ram_crew` – Rammbock-Mannschaft (Chor) (Einheit `battering_ram`, Glutkrone)
- Gruppe von 3–4 Männern, schwitzende Pioniere, Hau-Ruck-Rufe im Takt, dumpfe Anstrengung; mehrere Takes überlagern.
- **Prompt:** "A crew of four sweating siege engineers shouting in rhythm, rough male voices, heaving and grunting as they swing a heavy ram, shouted work chants and effort sounds, tight and dry. They shout in German with a natural, native accent. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[shouting] Zieht! Und zieht! Und zieht! Nochmal, [excited] legt euch ins Zeug!“
- **Einstellungen:** v3 *Creative*, Speed 1,0; je Zeile 3–4 Takes mit leicht verschobenem Einsatz überlagern (Chor), Mono-Summe.

### G9 `AC_unit_catapult_crew` – Katapult-Mannschaft (Einheit `catapult`, Glutkrone)
- 2–3 erfahrene Geschützmeister, nüchterne Zurufe in Kommandoform, Winden und Seile hörbar.
- **Prompt:** "Two or three experienced siege gunners, rough and businesslike male voices calling short, clear commands over creaking ropes and a winch, calm professionals, never panicked. They call out in German with a natural, native accent. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[commanding] Spannt sie zurück! Laden! Ruhig … [short pause] und los!“
- **Einstellungen:** v3 *Creative*, Speed 1,0; 2–3 Sprecher getrennt generieren und abwechselnd verwenden.

### G10 `AC_unit_hollow_digger` – Grabgräber (Hohle) (Einheit `hollow_digger`, Hohle)
- Hohler Wiedergänger-Arbeiter, flach, heiser, murmelt Fetzen von Werkzeugen und Schichten; Hohle-Kette, Pitch −2.
- **Prompt:** "A hollow, reanimated grave-digger with a flat, dusty, hoarse male voice, muttering fragments of tools and soil layers as if still at work, slow and monotonous, breathy and dry, mournful and unsettling. Mutters German word fragments with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[whispers] Tiefer … noch eine Handbreit … [sorrowfully] die Erde ist kalt … immer tiefer.“
- **Einstellungen:** v3 *Creative*, Speed 0,9; Hohle-Kette (Pitch −2) setzt die Integration.

### G11 `AC_unit_ghoul` – Ghul (Hohle) (Einheit `ghoul`, Hohle)
- Ausgehungerte Leichenfresser: hohes Kreischen, Schmatzen, Schnappen; fast keine Wörter.
- **Prompt:** "A starving ghoul, a high, cracking, shrieking creature voice with wet smacking and snapping jaws, hoarse hissing and guttural gurgles, almost no words, feral and desperate. Rasps the few German words it has with a natural, native pronunciation. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[growls] Hunger … so ein Hunger … [hissing] Fleisch … warmes Fleisch …“
- **Einstellungen:** v3 *Creative*, Speed 1,1; Zeilen sind überwiegend Laute. Hohle-Kette setzt die Integration.

### G12 `AC_unit_ash_witch` – Aschhexe (Hohle) (Einheit `ash_witch`, Hohle)
- Frau, undefinierbares Alter, flüsternde, kalt-berechnende Nekromantin; spricht wie eine Lehrerin zu ihren Toten; Hohle-Kette mit mehr Hall.
- **Prompt:** "A whispering necromancer, a woman of indeterminate age with a cold, soft, breathy alto voice, speaking calmly and precisely like a teacher to her pupils, with a chill of dry ash in every syllable and an ancient, hollow undertone. Speaks German with a natural, native accent. Close-mic studio-quality recording, no background noise."
- **Beispieltext:** „[softly] Erhebt euch, meine Kleinen. [menacingly] Die Asche erinnert sich, was die Lebenden vergessen.“
- **Einstellungen:** v3 *Creative* mit `[whispers]`, Speed 0,92; Hohle-Kette mit mehr Hall-Anteil setzt die Integration.
