# ASHEN CROWNS – ElevenLabs Produktionsleitfaden (Audio)

**Rolle:** Audio Director / Voice Casting Lead · **Gültig für:** Vertical Slice (Manifest v1) und spätere Phasen
**Verbindliche Quellen:** `docs/canon.md` (Namen, IDs, Dateinamenskonvention) und `assets/audio/manifest.ron` (erwartete Dateien)

Dieser Leitfaden beschreibt, wie der Auftraggeber Stimmen, Soundeffekte und Ambiences erzeugt und **unverändert liefert**. Alles Weitere
(Nachbearbeitung, Schnitt, Lautheit, Loop-Nähte, Formatkonvertierung, Mono/Stereo) erledigt die Integration (**Teil II**, nicht vom Nutzer).
Es werden hier **keine** Audiodateien erzeugt; alle Texte, Prompts und Einstellungen sind Produktionsvorlagen.

> **Wichtiger Hinweis zu Werkzeugständen:** ElevenLabs entwickelt Modelle, Oberflächen und Limits laufend weiter.
> Alle Modellnamen (Eleven v3, Sound-Effects-Modell), Zeichen-/Längenlimits, Reglerbezeichnungen und Exportformate in diesen Dokumenten sind
> **Empfehlungen nach aktuellem Kenntnisstand**. Vor Produktionsbeginn in der ElevenLabs-Oberfläche bzw. der aktuellen Dokumentation prüfen.

**Lieferregeln (Kurzfassung)**
- Alles in **einen Ordner**, Dateinamen **exakt** wie in der Liste, **beliebiges Audioformat**.
- **Keine Nachbearbeitung** und **keine Teilstücke**: jede Datei ist eine komplette Generierung (Stimmen: eine Zeile = eine Datei mit dem vollständigen Text; Musik: die komplette Suno-Ausgabe).
- Abweichungen zwischen gesprochenem Ton und Spieltext werden **nach der Lieferung von uns behoben**, indem die Spieltexte an den Ton angepasst werden.
- Jeder Sprechtext enthält die Regieanweisungen als **Audio-Tags in eckigen Klammern** direkt im Text (z. B. „[solemn] Jahr 897 nach dem Fall. [pause] Am Rand der Aschmark …“).

---

## 0. Dokumentenübersicht

| Dokument | Inhalt |
|---|---|
| `docs/audio/elevenlabs_production_guide.md` | dieses Dokument: Werkzeuge, Einstellungen, Schreibregeln, Aussprache; Teil II: Integration (Klangziele, Nachbearbeitung, Export), Ablage, Lieferung |
| `docs/audio/elevenlabs_characters.md` | Stimmprofile & Voice-Design-Prompts (Heldin, Kernfiguren, Erzähler, Bosse, Fraktions-Archetypen, Einheiten-Zuordnung), Beispieltexte mit Audio-Tags |
| `docs/audio/elevenlabs_unit_voices.md` | alle Einheiten-, Helden- und Boss-Zeilen des Manifests (Ziel-Datei, deutscher Sprechtext mit Audio-Tags, Regie) + Ausblick |
| `docs/audio/elevenlabs_dialogues.md` | Erzähler- und Dialogzeilen Mission 1 und Kampagne (je Zeile eine Datei, Sprechtext mit Audio-Tags) + Kampagnenszenen Zeitalter I–VII |
| `docs/audio/elevenlabs_sound_effects.md` | alle SFX des Manifests je Variante + erweiterter Katalog |
| `docs/audio/elevenlabs_ambient_audio.md` | alle Ambience-Loops des Manifests (eine Generierung je Datei) + Regionen-Ambiences |
| `docs/audio/suno_master_production.md` | Musik: Suno-Prompts für alle Tracks (eine Datei je Track) |
| `docs/audio/PRODUCTION_CHECKLIST.md` | Master-Produktionsliste: alle erwarteten Dateien (aus `ashen_tools audio-list`) mit vollständigen Prompts und einfügefertigen Sprechtexten, abhakbar |

---

## 1. Grundsatzentscheidungen

1. **Aufnahmesprache Deutsch.** Alle Stimmen im Spiel werden auf Deutsch gesprochen: Erzähler, Dialoge, Helden-, Boss- und
   Einheitenrufe (Entscheidung des Auftraggebers). In allen Dokumenten stehen die Sprechtexte deutsch. Wo das Spiel denselben Text
   als Untertitel oder Dialog zeigt (`assets/data/missions/*.ron`, `assets/data/campaign/*.ron`), ist der Sprechtext **ohne Audio-Tags wortgleich**
   mit dem Spieltext; das prüft `python3 docs/audio/verify_spoken_text.py`. Nur Voice-Design-Prompts, Sound-Effects-Prompts,
   Musik-Prompts und die v3-Audio-Tags in eckigen Klammern bleiben englisch (das Werkzeug versteht sie am besten).
2. **Eine Zeile = eine Datei.** Jede Erzähler-, Dialog- und Figurenzeile ist **genau eine** Datei `<stem>_01.ogg` mit dem vollständigen Text in einem Take.
   Mehrere Dateien eines Events (Barks) sind **echte Textvarianten**, jede mit eigenem vollständigem Text.
3. **Alle Stimmen laufen auf Eleven v3**, auch der Erzähler (Multilingual v2 versteht keine Audio-Tags).
4. **Dokumentationssprache Deutsch.** Eine englische Referenzfassung der Sprechzeilen gibt es nicht; sie kann bei einer späteren englischen Lokalisierung neu erstellt werden.
5. **Kanon ist bindend.** Eigennamen, IDs und Dateinamen exakt nach `docs/canon.md`. Eigennamen werden im Deutschen gesprochen
   wie im Kanon geschrieben; die Aussprache regelt das Aussprache-Wörterbuch (Abschnitt 6).
6. **Eine Stimme = ein gespeichertes Voice-Design.** Jede Figur/Einheit erhält genau eine gespeicherte Stimme in der
   ElevenLabs-Voice-Library (Benennung `AC_<id>`, z. B. `AC_hero_isolde`, `AC_unit_swordsman`). Keine Ad-hoc-Stimmen.
7. **Keine Stimmen realer Personen.** Keine Prompts, die eine reale Person, einen bekannten Schauspieler oder eine
   bekannte Figur imitieren („sounds like …“ ist verboten). Nur beschreibende Merkmale.
8. **Musik** entsteht mit Suno (`suno_master_production.md`): pro Track genau eine Datei, die komplette Suno-Ausgabe.

---

## 2. Werkzeugübersicht und Werkzeugwahl

| Asset-Typ | Primärwerkzeug | Modell (Empfehlung) | Begründung |
|---|---|---|---|
| Neue Stimme anlegen | **Voice Design** (Text → Stimme) | aktuelles Voice-Design-Modell (v3-basiert, falls angeboten) | Stimme aus Beschreibung; 3 Vorschauen; speichern |
| Einheiten-Barks (select/move/attack …) | **Text to Speech** | **Eleven v3** (Sprache: Deutsch) | Audio-Tags für Schreie, Flüstern, Atmer; hohe Expressivität |
| Held/Boss-Barks, Kampfrufe, Tode | Text to Speech | **Eleven v3** | emotionale Spitzen, `[shouting]`, `[groans]`, `[exhales]` |
| Erzähler | Text to Speech | **Eleven v3**, Stability-Modus *Natural* | Audio-Tags (`[solemn]`, `[pause]`) steuern Gewicht und Pausen |
| Dialogszenen (Mission/Kampagne) | Text to Speech | **Eleven v3** | Emotion über Tags |
| Kreaturen-Vokale ohne Worte (Hohle) | Text to Speech (v3, *Creative*) | Eleven v3 | Laute über Tags (`[growls]`, `[hissing]`) |
| One-Shot-Soundeffekte | **Sound Effects** (Text to SFX) | aktuelles SFX-Modell | prägnante EN-Prompts, Dauer, Prompt Influence |
| Ambience-Loops | Sound Effects mit **Loop-Option** | aktuelles SFX-Modell | eine Generierung je Datei, Loop an |
| Musik | **Suno** | siehe `suno_master_production.md` | pro Track eine komplette Ausgabe |

### 2.1 Voice Design (Stimme aus Textbeschreibung)

- **Eingaben:** (a) *Voice Description* – Beschreibungs-Prompt, **englisch formuliert** (das Tool versteht ihn am besten), aber mit der
  festen Angabe **„speaks German with a natural, native accent“** (ohne diese Angabe entstehen oft englisch gefärbte Stimmen);
  (b) *Text to preview* – **deutscher** Beispieltext in der Rolle **mit Audio-Tags** (Beispieltexte in `elevenlabs_characters.md`; empfohlen deutlich über 100 Zeichen, damit die Vorschau Charakter zeigt; Mindest-/Maximallänge in der UI prüfen).
- **Gute Beschreibungen enthalten:** Alter, Geschlecht, Timbre (rau, warm, hauchig, nasal), Akzent (hier: Färbung des Deutschen, nie ein englischer Akzent), Tempo, Grundemotion,
  Sprechhaltung (befehlend, müde, spöttisch) und eine Angabe zur Aufnahmequalität („studio-quality recording, close mic,
  no background noise“). Für Kreaturen/Untote zusätzlich *gewünschte Textur* (rasselnd, hohl, kehlig).
- **Regler (sofern vorhanden):** *Loudness* neutral lassen; *Guidance Scale* mittel bis hoch (Prompt-Treue), bei zu
  „flachen“ Ergebnissen leicht senken. *Quality vs. Variety*: für Hauptfiguren Qualität bevorzugen.
- **Vorgehen:** 3 Vorschauen erzeugen → beste wählen → mit 2–3 weiteren Rollenzeilen in TTS testen → erst dann speichern.
  Bei knapp verfehlten Ergebnissen die Beschreibung präzisieren statt endlos neu würfeln (max. 3 Runden pro Figur, dann
  Beschreibung überarbeiten).
- **Grenzen:** Extreme Monsterstimmen gelingen über Voice Design selten vollständig. Ziel ist eine **gute, trockene Basisstimme**;
  Tiefe, Hall und Schichtung setzt die Integration (Abschnitt 8.3).

### 2.2 Text to Speech – Eleven v3 (alle Stimmen)

- **Audio-Tags** in eckigen Klammern steuern Ausdruck, Tempo und nonverbale Laute, z. B. `[whispers]`, `[shouting]`, `[sighs]`,
  `[laughs]`, `[chuckles]`, `[exhales]`, `[sarcastic]`, `[angry]`, `[tired]`, `[groans]`, `[solemn]`, `[pause]`, `[short pause]`, `[slowly]`.
  Jede Sprechzeile in diesem Projekt enthält **1–4 Tags** (Emotion am Anfang, Takt an sinnvollen Stellen im Satz). Tags, die nicht in der
  offiziellen Liste stehen, funktionieren oft, müssen aber getestet werden. Die Tags stehen im Sprechtext der Listen **einfügefertig**: Text 1:1 kopieren.
- **Pausen** zusätzlich über Interpunktion: Komma, Gedankenstrich (–), Auslassungspunkte (…). SSML-`<break>`-Tags werden von v3 nach
  aktuellem Stand nicht unterstützt – in der UI/Doku prüfen.
- **Stabilitätsmodus** (v3 bietet derzeit drei Stufen): *Creative* = maximal expressiv, mehr Ausreißer;
  *Natural* = Standard für Dialog, Erzähler und Barks; *Robust* = sehr konsistent, weniger reaktiv auf Tags.
- **Kurze Texte** sind bei v3 instabiler: mehrere Takes erzeugen und den besten unverändert liefern. Es wird nichts geschnitten oder zusammengesetzt;
  Kurzzeilen **einzeln** generieren.
- **Zeichenlimit** pro Generierung bei v3 – aktuelles Limit in der UI prüfen; die längsten Erzählerzeilen (Kampagne) passen in der Regel hinein.
- **Sprache Deutsch:** Eleven v3 erkennt die Sprache aus dem Text; trotzdem die Sprache, wo die Oberfläche/API ein Sprachfeld
  anbietet (`language_code`), **explizit auf Deutsch (`de`) setzen**, damit kurze Rufe („Los!“, „Hü!“) nicht als englisch gelesen werden.
  Audio-Tags bleiben **englisch** in eckigen Klammern; sie werden nicht gesprochen. Deutsche Tag-Wörter („[flüstert]“) nicht verwenden.

### 2.3 Multilingual v2

Wird **nicht mehr verwendet**: Multilingual v2 versteht keine Audio-Tags (eckige Klammern könnten als Text gesprochen werden). Auch der Erzähler läuft auf Eleven v3.

### 2.4 Sound Effects (Text to SFX)

- **Prompt:** kurz, konkret, englisch, Klangquelle + Material + Aktion + Raum/Distanz + Charakter, z. B.
  „steel longsword striking chainmail, sharp metallic clang, close, dry, no reverb“.
- **Dauer:** manuell setzen (statt „automatisch“), wie in der Liste angegeben. Mindest- und Maximaldauer in der UI prüfen
  (derzeit etwa 0,5 s bis ca. 30 s). Liegt die Dauer darunter, mit der Mindestdauer generieren; Kürzen übernimmt die Integration.
- **Prompt Influence:** niedrig (≈0,3) = kreativer, mehr Variation; hoch (≈0,6–0,8) = wörtlicher. Für funktionale
  Gameplay-Sounds eher 0,5–0,7, für Atmosphärisches 0,3–0,5. Werte stehen je Eintrag.
- **Loop-Option** (sofern im aktuellen Modell verfügbar) nur für Ambience aktivieren (so im Eintrag angegeben).
- **Varianten:** pro Generierung werden meist mehrere Vorschläge geliefert; Varianten `_01…_03` sollen hörbar verschieden,
  aber klar „derselbe Sound“ sein (siehe Variationshinweise im SFX-Dokument). Pro Datei die beste komplette Generierung liefern.

### 2.5 Musik

Musik entsteht mit **Suno** (nicht mit Eleven Music): Prompts, Struktur-Tags und Arbeitsweise in `suno_master_production.md`. Je Track wird genau eine Datei geliefert, die komplette Suno-Ausgabe.

### 2.6 Ergänzende Werkzeuge

| Werkzeug | Einsatz in ASHEN CROWNS |
|---|---|
| **Pronunciation Dictionary** (Aussprache-Wörterbuch, Alias-Regeln) | Kanon-Eigennamen (Eldraeth, Aureth, Varkhan, Vyrmathax …) in **deutscher Aussprache** einheitlich; Alias-Schreibweisen (und IPA als Referenz) aus Abschnitt 6 hinterlegen. Das Wörterbuch wirkt nur auf die Aussprache, **nicht** auf den Text. |
| **Seed** (API/erweiterte Einstellungen) | gleiche Einstellungen + Seed ≈ reproduzierbarere Takes (keine Garantie); optional im Take-Log notieren |

---

## 3. Ablauf je Produktionswelle

1. **Vorbereitung:** Aussprache-Wörterbuch in ElevenLabs anlegen (Abschnitt 6).
2. **Casting:** Stimmen nach `elevenlabs_characters.md` per Voice Design erzeugen (Beispieltext mit Tags in *Text to preview*); je Stimme 2–3 Rollenzeilen testen; speichern als `AC_<id>`.
3. **Generierung:** Zeilen aus `PRODUCTION_CHECKLIST.md` (Quelle: `elevenlabs_unit_voices.md` / `elevenlabs_dialogues.md`) **1:1 kopiert** generieren; pro Zeile 2–3 Takes, den besten wählen.
4. **SFX/Ambience/Musik:** nach der Liste generieren, je Datei die beste komplette Generierung.
5. **Lieferung:** alle Dateien unverändert in einen Ordner (Abschnitt 13). Fertig – Rest erledigt die Integration.

**Reihenfolge der Produktion (Priorität Vertical Slice):**
1. Heldin `hero_isolde` + `char_hal_venn` + `narrator` (Mission-1-Erlebnis), 2. Glutkrone-Einheiten,
3. Hohle + `boss_pale_reeve`, 4. Kampf-SFX + UI-SFX, 5. Welt-SFX + Ambience, 6. Musik.

---

## 4. Einstellungs-Empfehlungen (Startwerte)

Alle Werte sind Startpunkte; Feinabstimmung pro Stimme im Charakterdokument.

### 4.1 Eleven v3 (alle Stimmen)

| Inhalt | Stabilitätsmodus | Hinweise |
|---|---|---|
| Erzähler | Natural (bei schwankender Stimme Robust) | Speed 0,92–0,97; Tags `[solemn]`, `[pause]`, `[slowly]`, `[softly]`; lange Zeilen in einer Generierung |
| Einheiten select/move | Natural | Tags sparsam, aber mindestens einer je Zeile |
| Einheiten attack/victory | Creative oder Natural | `[shouting]` bei Kampfrufen; bei Übersteuerung Natural |
| hit/death (Schmerz) | Creative | Laut-Tags (`[groans]`, `[exhales]`) + kurze Wörter; mehrere Takes |
| Heldin (alle Barks) | Natural | `[sarcastic]`, `[chuckles]` für trockenen Humor gezielt |
| Dialogszenen | Natural | eine Zeile pro Generierung, Absicht über die Tags im Text |
| Bosse | Natural/Robust | Konsistenz wichtiger; Effekt setzt die Integration |
| Hohle (wortlose Laute) | Creative | Laute über Tags (`[growls]`, `[hissing]`, `[rasping]`) |

Geschwindigkeit (Speed), falls angeboten: 0,95–1,05; Barks eher 1,0–1,05, Bosse 0,9–0,95, Erzähler 0,92–0,97.
Faustregel: *Creative* zu unruhig → *Natural*; *Natural* reagiert zu schwach auf Tags → *Creative*; Konsistenz über viele Takes → *Robust*.

### 4.2 Sound Effects

| SFX-Typ | Dauer | Prompt Influence | Loop |
|---|---|---|---|
| Treffer, Impacts | 0,5–1,2 s | 0,5–0,7 | aus |
| Magie | 1,5–4 s | 0,4–0,6 | aus |
| UI | 0,5–1,5 s | 0,6–0,8 | aus |
| Welt/Arbeit | 1–3 s | 0,5–0,6 | aus |
| Ambience | max. verfügbare Länge (derzeit ca. 30 s) | 0,35–0,45 | **an** |

### 4.3 Download-Format

In der **höchsten verfügbaren Qualität** herunterladen (bevorzugt WAV/PCM, sonst MP3 in höchster Bitrate) und **unverändert** liefern. Jedes Format ist zulässig;
die Konvertierung nach Ogg Vorbis übernimmt der Scanner bzw. die Integration.

---

## 5. Schreibregeln für TTS-Texte

1. **Deutsch, natürlich gesprochen.** Kurze Sätze; Barks 1–8 Wörter. Umgangssprachliche Verschleifungen („'ne“, „hab“, „ich mein“) nur dort,
   wo sie im Dokument stehen (Arbeiter, Milizen); Adel, Priester, Legion und Erzähler sprechen gehoben ohne Verschleifung.
2. **Spieltext ist Ausgangspunkt.** Zeilen mit Entsprechung im Spiel (Mission, Kampagne, Zeitalter-Texte) werden **ohne Tags unverändert** gesprochen,
   auch bei Ziffern („Jahr 897 nach dem Fall“) und Kürzeln. Stimmt die Aussprache nicht, wird sie über das Aussprache-Wörterbuch
   korrigiert (z. B. Alias „897“ → „achthundertsiebenundneunzig“), **nie** durch Umschreiben des Skripts. Weicht der gesprochene Ton dennoch ab,
   passen wir nach der Lieferung die Spieltexte an den Ton an.
3. **Eigennamen** wie im Kanon geschrieben; deutsche Aussprache über das Aussprache-Wörterbuch (Alias/IPA), nicht durch falsche Schreibung im Text.
4. **Interpunktion als Regie:** „…“ für Zögern, „–“ für Abbruch oder Schwebe, „!“ sparsam (v3 reagiert stark). Komma und Punkt
   setzen deutsche Sprechpausen.
5. **Audio-Tags (Eleven v3):** englisch in eckigen Klammern, **in jeder Sprechzeile**, 1–4 pro Zeile, direkt im Text. Sie werden nicht gesprochen und **nie** in den
   Untertitel übernommen. Der Sprechtext der Liste ist bereits getaggt: 1:1 kopieren, nichts ergänzen oder entfernen.
6. **Untertitel** = Sprechtext **ohne** Tags.
7. **Keine Großbuchstaben-Wörter** zur Betonung (können buchstabiert werden); Betonung über Satzbau und Wortstellung.
8. **Deutsche Prosodie:** Der Satzakzent liegt meist auf dem letzten inhaltstragenden Wort („Die Mauer **steht** noch“); Verben am Satzende
   nicht verschlucken; Verbzweitstellung und Nebensätze mit Komma gliedern, damit v3 die Satzmelodie nicht englisch hebt. Zusammengesetzte
   Wörter (Aschmark, Hohlschrein, Bergfried) nicht trennen und nicht auf der zweiten Silbe betonen. Anrede „Ihr/Euch“ (höfisch, Hal Venn/Soldaten zur Heldin)
   und „du“ (Vertraute: Hal ↔ Isolde, Torvald ↔ Isolde) strikt wie im Dokument; Großschreibung der Anrede ändert die Aussprache nicht.
9. **Zahlen und Silben:** deutsche Zahlwörter sind lang; Sätze mit Zahlen im Zweifel einzeln generieren und den Takt prüfen.

---

## 6. Aussprache-Wörterbuch (Kanon-Eigennamen, deutsche Aussprache)

Die Namen werden **deutsch** gesprochen. Das Problem für deutsche TTS-Modelle sind vor allem **V** (im Deutschen /f/, im Kanon aber meist /v/), **ae/ei/ey**
(fremdländische Schreibung) und englisch anmutende Endungen. Reihenfolge der Prüfung: (1) Standardaussprache mit Testsatz, (2) nur bei Fehlern **Alias** hinterlegen
(Regeltyp *Alias* wirkt in allen Modellen, auch Eleven v3 und Multilingual v2), (3) IPA nur als Referenz für das Casting; die IPA-/Phonem-Regeln des Wörterbuchs
werden nur von einem Teil der Modelle unterstützt (in der UI prüfen), Alias ist der verlässliche Weg. Groß-/Kleinschreibung beachten; Einträge pro Name einmal anlegen.

| Kanon | Deutsche Aussprache (IPA) | Aussprache-Alias | Hinweis |
|---|---|---|---|
| Eldraeth | /ˈɛldʁaːɛt/ | `Eldra-et` | „EL-dra-et“, Doppelvokal „ae“ getrennt |
| Aureth | /ˈaʊʁɛt/ | `Auret` | „AU-ret“, „th“ = t |
| Solmere | /zɔlˈmeːʁə/ | `Solmehre` | „Sol-MEH-re“ |
| Varkhan Aurelith | /ˈvaʁkaːn aʊʁeˈliːt/ | `Warkahn Aureliet` | V = /v/; „kh“ = k |
| Valemark | /ˈvaːləmaʁk/ | `Walemark` | V = /v/ |
| Skarnheim | /ˈskaʁnhaɪm/ | – | Standardaussprache passt |
| Saelith | /ˈzɛːlɪt/ | `Sälit` | „SÄ-lit“ |
| Saelith-Pakt / Saelith-Inseln | /ˈzɛːlɪt/ | `Sälit-Pakt` / `Sälit-Inseln` | Alias für den Namensteil genügt |
| Veyl | /vaɪl/ | `Wail` | V = /v/, „ey“ = ai |
| Kaldris | /ˈkaldʁɪs/ | – | Standardaussprache passt |
| Arnvale | /ˈaʁnvaːlə/ | `Arnwale` | V = /v/ |
| Nordmark | /ˈnɔʁtmaʁk/ | – | Standardaussprache passt |
| Dornwald / Dornenbund | – | – | deutsche Wörter, Standardaussprache |
| Brennhollow | /ˈbʁɛnhɔlo/ | `Brennhollo` | „ow“ als offenes o |
| Vael Dorn | /vɛːl dɔʁn/ | `Wäl Dorn` | V = /v/, „ae“ = ä |
| Grauwall | /ˈɡʁaʊval/ | `Grauwall` | Standardaussprache passt, nur zur Kontrolle |
| Harrowmere | /ˈhaʁomiːʁ/ | `Harromihr` | |
| Marrowfen | /ˈmaʁofɛn/ | `Marrofen` | |
| Ithren | /ˈɪtʁən/ | – | Standardaussprache passt |
| Morvaine | /mɔʁˈvɛːn/ | `Morwäne` | V = /v/ |
| Calyx | /ˈkaːlʏks/ | `Kalüx` | „y“ = ü |
| Thal | /taːl/ | – | wie das deutsche Wort „Tal“ |
| Oruun | /oˈʁuːn/ | `Orun` | Doppel-u lang |
| Vyrmathax | /ˈvʏʁmataks/ | `Würmatax` | V = /v/, „y“ = ü, „th“ = t |
| Isolde Raventhorn | /iˈzɔldə ˈʁaːvn̩tɔʁn/ | – | Standardaussprache passt; „Raventhorn“ mit /v/ |
| Edran Raventhorn | /ˈɛdʁan ˈʁaːvn̩tɔʁn/ | – | |
| Aldric | /ˈaldʁɪk/ | – | Standardaussprache; „Aldric der Dritte“ ausgesprochen („III.“ als Alias `der Dritte`) |
| Mireille Corvane | /miˈʁɛj kɔʁˈvaːnə/ | `Mirej Korwane` | französische Vornamenform, V = /v/ |
| Hal Venn | /hal vɛn/ | `Hal Wenn` | V = /v/ (sonst „Fenn“) |
| Torvald Eisenhand | /ˈtɔʁvalt ˈaɪzn̩hant/ | – | Standardaussprache passt |
| Sylaen | /zyˈlaːɛn/ | `Süla-en` | „y“ = ü |
| Ophira Saelith-Varo | /oˈfiːʁa ˈzɛːlɪt ˈvaːʁo/ | `Ofira Sälit-Waro` | V = /v/ |
| Kassian Vale | /ˈkasi̯an veːl/ | `Kassian Wehl` | Nachname als deutsches „Wehl“ |
| Ser Wendel | /zɛʁ ˈvɛndl̩/ | – | Standardaussprache passt |
| Nel | /nɛl/ | – | |
| Gharun | /ˈɡaːʁuːn/ | `Garun` | „gh“ = g |
| Isarre Mol | /iˈzaʁə mɔl/ | – | Standardaussprache passt |
| Aldous Meringer | /ˈaldʊs ˈmeːʁɪŋɐ/ | – | Standardaussprache passt |
| n.F. | – | `nach dem Fall` | im Sprechtext ausgeschrieben, falls das Kürzel vorkommt |
| 897 | – | `achthundertsiebenundneunzig` | Alias für Ziffern im Spieltext (Aussprache prüfen) |

Aussprache der deutschen Sachwörter (Aschmark, Aschenfall, Aschglas, Hohlschrein, Glutkrone, Bergfried, Ewige Flamme, Leere Liturgie, Sonnenhäresie, Schleier,
Hohles Tief) benötigt keinen Eintrag. Ausnahme: *Aschglas* wird als zwei Silben-Kompositum betont (**ASCH**-glas), nicht „Asch-**glas**“; bei Fehlern Alias `Aschglas` → `Asch-Glas`.

Alias-Einträge zuerst mit einem Testsatz pro Name prüfen (im Satz und als Einzelwort); Alias nur setzen, wenn die Standardaussprache falsch ist. Neue Namen aus künftigen Missionen ergänzen.

---

---

# Teil II – Integration (nicht vom Nutzer)

Alles in diesem Teil erledigt die Integration an den gelieferten Rohdateien. Der Nutzer liefert unverändert; er muss nichts messen, schneiden, normalisieren oder konvertieren.

## 7. Klangziele (Loudness & Peak) – Integration

Messung nach ITU-R BS.1770 / EBU R128. **True Peak überall max. −1 dBTP.**

| Kategorie | Ziel | Toleranz | Kanäle | Bemerkung |
|---|---|---|---|---|
| Dialog / Charaktere | **−16 LUFS integriert** | ±1 LU | Mono | Manifest-Volume (+1 dB) wirkt zusätzlich |
| Erzähler | **−16 LUFS integriert** | ±1 LU | Mono | Manifest +2 dB; Erzähler sitzt bewusst vor Musik |
| Einheiten-/Helden-Barks | −16 LUFS (Short-Term-Max, s. u.) | ±1,5 LU | Mono | Familie einer Einheit untereinander angleichen |
| Hohle/Kreaturen-Vokale | −17 LUFS (Short-Term-Max) | ±1,5 LU | Mono | Manifest −2 dB, max. 2 Instanzen |
| Kampf-SFX | −14 bis −18 LUFS (Short-Term-Max) | – | Mono | Positionierbar; Transienten erhalten |
| Magie-SFX | −14 bis −16 LUFS (Short-Term-Max) | – | Mono (Stereo nur wenn nicht positioniert) | Heldenfähigkeiten dürfen herausstechen |
| Gebäude-SFX | −16 bis −20 LUFS (Short-Term-Max) | – | Mono | build_progress leise (Manifest −6 dB) |
| UI-SFX | −20 bis −24 LUFS (Short-Term-Max), Peak ≤ −6 dBFS | – | Mono oder Stereo | nie aufdringlich |
| Welt-SFX | −18 bis −22 LUFS (Short-Term-Max) | – | Mono | Arbeitsgeräusche häufig → leise |
| Ambience-Loops | **−23 LUFS integriert** | ±1 LU | Stereo | Manifest −6 dB zusätzlich |
| Musik (Referenz) | **−14 LUFS integriert** | ±1 LU | Stereo | Mischreferenz; Engine-Busse regeln endgültig |

**Kurze Dateien (< 3 s):** Integrierte LUFS sind bei sehr kurzen Clips unzuverlässig. Empfehlung: maximalen Short-Term-Wert
(3-s-Fenster; ggf. mit angehängter Stille messen) bzw. Momentary-Max verwenden und alle Varianten einer Familie **gehörmäßig
gegen eine Referenzdatei** angleichen (Referenz: `hero_isolde_select_01` für Stimmen, `sword_hit_01` für Kampf-SFX).

---

---

## 8. Nachbearbeitung – Integration

Werkzeuge: beliebige DAW bzw. Batch per `ffmpeg`/`sox`. Immer auf Kopie der gelieferten Rohdateien arbeiten.

### 8.1 Standardkette Stimme

1. **Schnitt:** Kopfstille ≤ 20 ms, Schwanzstille ≤ 150 ms (Dialog bis 300 ms); Fade-in 5 ms, Fade-out 20–40 ms.
2. **Hochpass:** 80 Hz (tiefe Männerstimmen 60–70 Hz), 100 Hz (Frauenstimmen), 12–18 dB/Okt.
3. **Artefaktkontrolle:** metallisches Flattern, Zischeln, abgehackte Konsonanten → Take verwerfen statt reparieren.
4. **De-Esser:** 5–8 kHz, max. 4–6 dB Reduktion.
5. **EQ:** leichte Präsenz +1–2 dB bei 3–4 kHz für Barks (Durchsetzung im Kampf); Mulm 200–350 Hz bei Bedarf −2 dB.
6. **Kompressor:** 3:1, Attack 10 ms, Release 80 ms, 3–5 dB Gain Reduction (Barks), Dialog sanfter (2:1).
7. **Limiter:** True-Peak −1 dBTP.
8. **Loudness** gemäß Abschnitt 7.
9. **Keine Raumhall-Zugabe** für Einheiten/Helden (Engine-Raum entscheidet). Ausnahmen siehe 8.3.

### 8.2 Standardkette SFX

1. Schnitt auf Transienten (keine Vorlaufstille > 5 ms bei Impacts), Fade-out natürlich; zu lange Ausklänge und Mindestdauer-Überhänge kürzen.
2. Hochpass je nach Quelle (UI 150 Hz, Impacts 40 Hz), DC-Offset entfernen.
3. Rauschen/Artefakte des Generators (Zirpen, „Wasser“-Artefakte) prüfen; bei Bedarf Neugenerierung anfordern.
4. Leichte Transientenbetonung bei Kampf; Limiter −1 dBTP; Loudness gemäß Abschnitt 7.
5. Mono-Summierung prüfen (Phasenauslöschung!), dann Mono-Export für positionierte Sounds.

### 8.3 Spezialbehandlungen (Fraktions-Klangsignaturen)

| Ziel | Behandlung |
|---|---|
| **Hohle** (`hollowed`, Einheiten, Fahler Vogt) | Pitch −2 bis −4 Halbtöne mit Formant-Erhalt *oder* Formant −2; Parallel-Layer: geflüsterte Dopplung (−12 dB, 30 ms versetzt); leichter Reverse-Reverb-Vorlauf (100–200 ms) nur bei Boss; Bandpass-„Hohlheit“ (Notch 1–2 kHz −4 dB); dezente Granular-/Bitcrush-Körnung (Mix ≤ 15 %) |
| **Fahler Vogt** (`boss_pale_reeve`) | wie Hohle + Chorus/Doppler-Schicht −3 Halbtöne (Mix 25 %); kurzer, kalter Hall (Kathedrale, 1,8 s, Mix 12 %), da er im Hohlschrein auftritt |
| **Aschenlegion** (`ashen_legion`) | Helm-Resonanz: Kammfilter/Kurzhall 3–6 ms, Bandpass 250 Hz–5 kHz, leichte Sättigung; Glut-Knistern als leiser Layer (−24 dB) |
| **Die Stimme aus dem Tief** (`char_deep_voice`) | drei Schichten: (1) Original, (2) −12 Halbtöne (Mix 40 %), (3) Flüsterschicht, aus derselben gelieferten Datei abgeleitet (Mix 30 %); Subharmonik 40–60 Hz; langes, dunkles Hallfeld 4–6 s, Pre-Delay 80 ms; leichte Zeit-Desynchronisierung der Layer (10–40 ms) |
| **Hohler König** (`boss_hollow_king`) | menschlich-königliche Basis; einzelne Wörter mit Deep-Voice-Layer (≤ 20 %) unterlegen; Thronsaal-Hall 2,5 s |
| **Aschdrache Vyrmathax** (`boss_ashen_dragon`) | Basisstimme −5 bis −7 Halbtöne, Layer aus SFX „large reptile growl, low rumble“ (−10 dB) unter Wortanfängen (separat erzeugt, nicht vom Nutzer); Glut-/Feuer-Rauschen; großer Höhlenhall |
| **Wurzelmutter** (`boss_root_mother`) | Chor-Dopplung (3 Kopien derselben Datei, je ±10 Cent, 15–25 ms versetzt), Holz-Knarz-SFX als Untergrund (separat erzeugt, nicht vom Nutzer), warmer Waldhall |
| **Veyl** (Sylaen, Dornenbund-Veyl) | sehr dezente Luftigkeit: Hochregal +2 dB ab 10 kHz, kurzer heller Hall; optional leiser Flügelflatter-Layer (Motte) bei Ruf-Zeilen |
| **Erzähler** | kein Effekt außer minimal „Wärme“ (Sättigung ≤ 5 %); extrem saubere Grundqualität |

---

---

## 9. Export (Integration)

**Zielformat (verbindlich):** **Ogg Vorbis, Qualität q6 (≈ 192 kbit/s), 48 kHz**, Kleinbuchstaben-Dateiname.
Stimmen & positionierte SFX **mono**; Ambience & Musik **stereo**; UI wahlweise.

**Einzeldatei (Stimme, mono):**
```bash
ffmpeg -i take.wav -ac 1 -ar 48000 -c:a libvorbis -q:a 6 hero_isolde_select_01.ogg
```

**Zweistufige Loudness-Normalisierung (Dialog −16 LUFS, −1 dBTP):**
```bash
# 1. Messung
ffmpeg -i in.wav -af loudnorm=I=-16:TP=-1:LRA=7:print_format=json -f null -
# 2. Anwendung mit gemessenen Werten (measured_I, measured_TP, measured_LRA, measured_thresh, offset aus Schritt 1)
ffmpeg -i in.wav -af loudnorm=I=-16:TP=-1:LRA=7:measured_I=<..>:measured_TP=<..>:measured_LRA=<..>:measured_thresh=<..>:offset=<..>:linear=true \
       -ac 1 -ar 48000 -c:a libvorbis -q:a 6 out.ogg
```

**Ambience (stereo, −23 LUFS):**
```bash
ffmpeg -i ambience_day_master.wav -af loudnorm=I=-23:TP=-1:LRA=11:linear=true -ac 2 -ar 48000 -c:a libvorbis -q:a 6 ambience_day_loop.ogg
```
Achtung: `loudnorm` mit `linear=true` fällt bei zu großer Abweichung auf dynamischen Modus zurück – bei Loops kann das die
Nahtstelle verändern. Loops daher bevorzugt im DAW per statischem Gain normalisieren und dann nur konvertieren.

**Batch-Konvertierung bereits normalisierter Masters (Ordnerstruktur gespiegelt):**
```bash
# masters/ spiegelt die Struktur von assets/audio/, enthält fertige WAVs
cd masters
find . -name '*.wav' | while read -r f; do
  out="../AshenCrowns/assets/audio/${f%.wav}.ogg"
  mkdir -p "$(dirname "$out")"
  ch=1; case "$f" in ./ambience/*|./music/*) ch=2;; esac
  ffmpeg -n -loglevel error -i "$f" -ac "$ch" -ar 48000 -c:a libvorbis -q:a 6 "$out"
done
```

**Loop-Hinweis:** Nach dem Export jeden Loop (Ambience, Musik `_loop`) in einer Loop-Wiedergabe und **im Spiel** prüfen
(Klick an der Nahtstelle, Lautstärkesprung). Ogg-Vorbis-Loops sind bei korrekter Engine-Implementierung sampleexakt, das ist
aber im Spiel zu verifizieren.

**Rohlieferungen archivieren:** Die vom Nutzer gelieferten Originale bleiben außerhalb von `assets/audio/` erhalten. In `assets/audio/` liegen **nur** die finalen Spieldateien.

---

---

## 10. Dateibenennung und Ablage (gemäß Kanon)

Alle Dateien klein, `snake_case`. `<NN>` = zweistellige Variantennummer ab `01`, lückenlos.

```
assets/audio/music/<track_id>.ogg                          genau eine Datei je Track
assets/audio/voices/heroes/<hero_id>_<situation>_<NN>.ogg
assets/audio/voices/units/<unit_id>_<situation>_<NN>.ogg
assets/audio/voices/characters/<char_id>_<line_id>_01.ogg  eine Datei je Zeile
assets/audio/voices/narrator/narrator_<line_id>_01.ogg     eine Datei je Zeile
assets/audio/sfx/<category>/<sfx_id>_<NN>.ogg              category ∈ combat, magic, buildings, ui, units, world
assets/audio/ambience/<ambience_id>_loop.ogg
```

**Regeln:**
- Manifest-Pfade sind relativ zu `assets/audio` und ohne Endung angegeben; Muster `_*` = `_01`, `_02`, … lückenlos.
- Der Boss `boss_pale_reeve` liegt laut Manifest unter `voices/units/` (nicht `heroes/`) – exakt so ablegen.
- **Erzähler-/Dialogzeilen** haben je Event genau eine Datei (`_01`) mit dem vollständigen Text. **Barks** (`voices/units`, `voices/heroes`) haben mehrere Dateien: jede ist eine eigene, vollständige Textvariante; die Engine wählt zufällig (nie dieselbe zweimal hintereinander).
- Kampagnen-Zeilen außerhalb des Manifests nutzen die vorgeschlagenen IDs aus `elevenlabs_dialogues.md` (Präfix `aNN_` für
  Zeitalter). Für Dialogzeilen der Heldin wird `voices/characters/hero_isolde_<line_id>.ogg` **vorgeschlagen** – vor
  Produktion mit dem Kanon-Owner bestätigen, ggf. Manifest/Kanon ergänzen lassen.
- Keine Leerzeichen, Umlaute, Großbuchstaben, Bindestriche oder Versionssuffixe (`_final`, `_v2`) in Spieldateien (Groß-/Leerzeichen/Bindestriche korrigiert `audio-scan --apply-renames`).

---

## 11. Musik (Manifest-Abdeckung)

Das Manifest erwartet **34 Musikdateien**, je Track genau eine: `music/<track_id>.ogg` mit `intro: None`. Es ist die **komplette Suno-Ausgabe** (beste Version) des Tracks;
`looped: true` heißt, die Engine loopt die Datei als Ganzes, Stings (`looped: false`) spielen einmalig. Prompts, Struktur-Tags und Lyrics stehen in
`docs/audio/suno_master_production.md`; die Dateiliste in `PRODUCTION_CHECKLIST.md`, Abschnitt D. Loop-Naht und Pegel übernimmt die Integration (Abschnitt 7–9).

---

## 12. Qualitätscheckliste (Nutzer, pro Datei)

**Inhalt & Benennung**
- [ ] Dateiname exakt wie in der Liste (klein, snake_case, `_NN` zweistellig, lückenlos ab `01`).
- [ ] Sprechtext 1:1 aus der Liste (inklusive Audio-Tags) verwendet; Eigennamen korrekt ausgesprochen.
- [ ] Eine komplette Generierung je Datei: keine Teilstücke, keine Nachbearbeitung.

**Klang**
- [ ] Keine KI-Artefakte (metallisches Flattern, Phasing, verschluckte Silben, Fremdgeräusche, Atem-Glitches).
- [ ] Stimme passt zum gespeicherten Voice-Design (gleiche Figur klingt in allen Dateien gleich).
- [ ] Emotion passt zur Situation (Tags und Regie); Tags wurden nicht mitgesprochen.
- [ ] Ambience: nur der eigene Inhalt, keine verständlichen Wörter, keine Musik.

**Rechtliches**
- [ ] Generiert mit einem Plan, der kommerzielle Nutzung erlaubt; keine Imitation realer Personen.

(Lautheit, Stille, Loop-Nähte, Format und Ingame-Prüfung gehören zur Integration, Teil II.)

---

## 13. Lieferprozess

1. **Ablage:** alle Dateien in **einen Ordner** (z. B. `lieferung/`, Unterordner erlaubt), Dateinamen exakt wie in der Liste, beliebiges Format.
2. **Import durch uns:** `cargo run -p ashen_tools -- audio-scan --import lieferung --apply-renames --convert --report assets/audio/audio_report.md`
   ordnet jede Datei anhand des Namens zu und meldet fehlende Dateien, Lücken, Unbekanntes.
3. **Abweichungen Ton ↔ Spieltext** gleichen wir danach an: die Spieltexte werden an den gesprochenen Ton angepasst.
4. **Integration** (Teil II): Schnitt, Lautheit, Loop-Nähte, Format; Ingame-Abnahme mit Mission `m01_last_settlement`.
5. **Nachlieferungen** ersetzen Dateien unter **gleichem Namen**; neue Textvarianten bei Barks werden nur nach Absprache angehängt (`_04`, `_05` …).

---

## 14. Take-Log (optional)

Ein Take-Log ist **nicht verpflichtend**. Hilfreich für Nachlieferungen: Datei, Modell, Stimme, Stability-Modus, Seed (falls genutzt), Datum.

---

## 15. Mengengerüst

Die aktuellen Zahlen je Abschnitt (A SFX, B Stimmen, C Ambience, D Musik) stehen im Kopf von `PRODUCTION_CHECKLIST.md` (erzeugt aus `ashen_tools audio-list`).
