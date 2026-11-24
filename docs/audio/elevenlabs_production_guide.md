# ASHEN CROWNS – ElevenLabs Produktionsleitfaden (Audio)

**Rolle:** Audio Director / Voice Casting Lead · **Gültig für:** Vertical Slice (Manifest v1) und spätere Phasen
**Verbindliche Quellen:** `docs/canon.md` (Namen, IDs, Dateinamenskonvention) und `assets/audio/manifest.ron` (erwartete Dateien)

Dieser Leitfaden beschreibt, wie der Auftraggeber sämtliche Stimmen, Soundeffekte, Ambiences und die Manifest-Musik
mit ElevenLabs erzeugt, nachbearbeitet, exportiert und im Repository abliefert. Es werden hier **keine** Audiodateien
erzeugt; alle Texte, Prompts und Einstellungen sind Produktionsvorlagen.

> **Wichtiger Hinweis zu Werkzeugständen:** ElevenLabs entwickelt Modelle, Oberflächen und Limits laufend weiter.
> Alle Modellnamen (Eleven v3, Multilingual v2, Sound-Effects-Modell, Eleven Music), Zeichen-/Längenlimits,
> Reglerbezeichnungen und Exportformate in diesen Dokumenten sind **Empfehlungen nach aktuellem Kenntnisstand**.
> Vor Produktionsbeginn und bei jeder neuen Produktionswelle in der ElevenLabs-Oberfläche bzw. der aktuellen
> Dokumentation prüfen und Abweichungen im Take-Log (Abschnitt 14) vermerken.

---

## 0. Dokumentenübersicht

| Dokument | Inhalt |
|---|---|
| `docs/audio/elevenlabs_production_guide.md` | dieses Dokument: Pipeline, Werkzeugwahl, Einstellungen, Klangziele, Nachbearbeitung, Export, Ablage, Musik, QA, Lieferung |
| `docs/audio/elevenlabs_characters.md` | Stimmprofile & Voice-Design-Prompts (Heldin, Kernfiguren, Erzähler, Bosse, Fraktions-Archetypen, Einheiten-Zuordnung) |
| `docs/audio/elevenlabs_unit_voices.md` | alle Einheiten-, Helden- und Boss-Zeilen des Manifests (Ziel-Datei, EN-Text, DE, Regie) + Ausblick |
| `docs/audio/elevenlabs_dialogues.md` | Erzähler- und Dialogzeilen Mission 1 (Manifest) + Kampagnenszenen Zeitalter I–VII |
| `docs/audio/elevenlabs_sound_effects.md` | alle SFX des Manifests je Variante + erweiterter Katalog |
| `docs/audio/elevenlabs_ambient_audio.md` | alle Ambience-Loops des Manifests + Regionen-Ambiences |
| `docs/audio/PRODUCTION_CHECKLIST.md` | Master-Produktionsliste: alle erwarteten Dateien (aus `ashen_tools audio-list`) mit vollständigen Prompts und Einstellungen, abhakbar |

---

## 1. Grundsatzentscheidungen

1. **Aufnahmesprache Englisch** (Primärsprache). Alle Sprechtexte in den Dokumenten sind englisch.
2. **Dokumentationssprache Deutsch.** Zu jeder Sprechzeile gibt es eine kurze deutsche Übersetzung als Basis für eine
   spätere Lokalisierung (die DE-Übersetzung ist **kein** fertiges Lokalisierungsskript, sondern Bedeutungsreferenz).
3. **Kanon ist bindend.** Eigennamen, IDs und Dateinamen exakt nach `docs/canon.md`. Englische Begriffe, die der Kanon nur
   deutsch führt, sind im Glossar (Abschnitt 6) als *EN-Arbeitsbegriff* markiert und vom Kanon-Owner zu bestätigen.
4. **Eine Stimme = ein gespeichertes Voice-Design.** Jede Figur/Einheit erhält genau eine gespeicherte Stimme in der
   ElevenLabs-Voice-Library (Benennung `AC_<id>`, z. B. `AC_hero_isolde`, `AC_unit_swordsman`). Keine Ad-hoc-Stimmen.
5. **Keine Stimmen realer Personen.** Keine Prompts, die eine reale Person, einen bekannten Schauspieler oder eine
   bekannte Figur imitieren („sounds like …“ ist verboten). Nur beschreibende Merkmale.
6. **Vorhandene Musik im Manifest** wird ebenfalls hier abgedeckt (Abschnitt 11), weil das Manifest diese Dateien erwartet.

---

## 2. Werkzeugübersicht und Werkzeugwahl

| Asset-Typ | Primärwerkzeug | Modell (Empfehlung) | Begründung |
|---|---|---|---|
| Neue Stimme anlegen | **Voice Design** (Text → Stimme) | aktuelles Voice-Design-Modell (v3-basiert, falls angeboten) | Stimme aus Beschreibung; 3 Vorschauen; speichern |
| Einheiten-Barks (select/move/attack …) | **Text to Speech** | **Eleven v3** | Audio-Tags für Schreie, Flüstern, Atmer; hohe Expressivität |
| Held/Boss-Barks, Kampfrufe, Tode | Text to Speech | **Eleven v3** | emotionale Spitzen, `[shouting]`, `[groans]`, `[exhales]` |
| Erzähler, lange Passagen | Text to Speech (ggf. **Studio**) | **Multilingual v2** | stabile, gleichmäßige, lange Erzählung; feinere Regler |
| Dialogszenen (Mission/Kampagne) | Text to Speech | **Eleven v3** (Dialog) / Multilingual v2 (ruhige Monologe) | v3 für Emotion; v2 wenn Konsistenz über viele Takes wichtiger ist |
| Kreaturen-Vokale ohne Worte (Hohle) | Sound Effects **oder** v3 + Nachbearbeitung **oder** Voice Changer | Sound-Effects-Modell / Eleven v3 | Grunzen, Rasseln, Brüllen klingen per SFX oft organischer |
| One-Shot-Soundeffekte | **Sound Effects** (Text to SFX) | aktuelles SFX-Modell | prägnante EN-Prompts, Dauer, Prompt Influence |
| Ambience-Loops | Sound Effects mit **Loop-Option** + DAW-Schichtung | aktuelles SFX-Modell | Loop-Option, mehrere Schichten, nahtloser Schnitt |
| Musik (Manifest) | **Eleven Music** (Text to Music), falls im Plan verfügbar | aktuelles Musikmodell | instrumental, Struktur-Prompts; alternativ Komponist |

### 2.1 Voice Design (Stimme aus Textbeschreibung)

- **Eingaben:** (a) *Voice Description* – englischer Beschreibungs-Prompt; (b) *Text to preview* – englischer Beispieltext
  in der Rolle (empfohlen deutlich über 100 Zeichen, damit die Vorschau Charakter zeigt; Mindest-/Maximallänge in der UI prüfen).
- **Gute Beschreibungen enthalten:** Alter, Geschlecht, Timbre (rau, warm, hauchig, nasal), Akzent, Tempo, Grundemotion,
  Sprechhaltung (befehlend, müde, spöttisch) und eine Angabe zur Aufnahmequalität („studio-quality recording, close mic,
  no background noise“). Für Kreaturen/Untote zusätzlich *gewünschte Textur* (rasselnd, hohl, kehlig).
- **Regler (sofern vorhanden):** *Loudness* neutral lassen; *Guidance Scale* mittel bis hoch (Prompt-Treue), bei zu
  „flachen“ Ergebnissen leicht senken. *Quality vs. Variety*: für Hauptfiguren Qualität bevorzugen.
- **Vorgehen:** 3 Vorschauen erzeugen → beste wählen → mit 2–3 weiteren Rollensätzen in TTS testen → erst dann speichern.
  Bei knapp verfehlten Ergebnissen die Beschreibung präzisieren statt endlos neu würfeln (max. 3 Runden pro Figur, dann
  Beschreibung überarbeiten).
- **Grenzen:** Extreme Monsterstimmen gelingen über Voice Design selten vollständig. Ziel ist eine **gute Basisstimme**;
  der Rest (Tiefe, Hall, Schichtung) entsteht in der Nachbearbeitung (Abschnitt 8).

### 2.2 Text to Speech – Eleven v3

- **Audio-Tags** in eckigen Klammern steuern Ausdruck und nonverbale Laute, z. B. `[whispers]`, `[shouting]`, `[sighs]`,
  `[laughs]`, `[chuckles]`, `[exhales]`, `[sarcastic]`, `[angry]`, `[tired]`, `[groans]`. **Nur wo sinnvoll** – ein Tag pro
  Satz reicht meist. Tags, die nicht in der offiziellen Liste stehen, funktionieren oft, müssen aber getestet werden.
- **Pausen** über Interpunktion: Komma, Gedankenstrich (—), Auslassungspunkte (…). SSML-`<break>`-Tags werden von v3 nach
  aktuellem Stand nicht unterstützt – in der UI/Doku prüfen.
- **Stabilitätsmodus** (v3 bietet derzeit drei Stufen): *Creative* = maximal expressiv, mehr Ausreißer;
  *Natural* = Standard für Dialog; *Robust* = sehr konsistent, weniger reaktiv auf Tags.
- **Kurze Texte** sind bei v3 instabiler. Empfehlung **Batch-Methode**: mehrere Barks einer Situation in **einer** Generierung
  erzeugen (Zeilen durch Absatz/„…“ getrennt), danach im Editor schneiden. Das hält Klangfarbe und Lautstärke konsistent.
  Wenn sich Emotion „hinüberzieht“, Zeilen einzeln generieren.
- **Zeichenlimit** pro Generierung bei v3 niedriger als bei v2 – aktuelles Limit in der UI prüfen; lange Szenen splitten.

### 2.3 Text to Speech – Multilingual v2

- Für **Erzähler** und lange, ruhige Monologe. Regler: *Stability*, *Similarity (Clarity + Similarity Enhancement)*,
  *Style Exaggeration*, *Speaker Boost*, *Speed*.
- Keine Audio-Tags (v2 interpretiert eckige Klammern ggf. als Text) – Emotion über Wortwahl, Interpunktion und Regler.
- Für längere Erzählstrecken **Studio** (ehemals Projects) nutzen: Absätze, Neu-Generierung einzelner Absätze, Aussprache-Wörterbuch.

### 2.4 Sound Effects (Text to SFX)

- **Prompt:** kurz, konkret, englisch, Klangquelle + Material + Aktion + Raum/Distanz + Charakter, z. B.
  „steel longsword striking chainmail, sharp metallic clang, close, dry, no reverb“.
- **Dauer:** manuell setzen (statt „automatisch“), wenn das Timing wichtig ist. Mindest- und Maximaldauer in der UI prüfen
  (derzeit etwa 0,5 s bis ca. 30 s). Kürzere Sounds (UI-Hover) etwas länger generieren und im Editor kürzen.
- **Prompt Influence:** niedrig (≈0,3) = kreativer, mehr Variation; hoch (≈0,6–0,8) = wörtlicher. Für funktionale
  Gameplay-Sounds eher 0,5–0,7, für Atmosphärisches 0,3–0,5.
- **Loop-Option** (sofern im aktuellen Modell verfügbar) für Ambience und Dauerschleifen aktivieren; Loop trotzdem im Editor prüfen.
- **Varianten:** pro Generierung werden meist mehrere Vorschläge geliefert; Varianten `_01…_03` sollen hörbar verschieden,
  aber klar „derselbe Sound“ sein (siehe Variationshinweise im SFX-Dokument).

### 2.5 Eleven Music (Text to Music)

- Nur nutzen, wenn im Plan enthalten und die **Lizenzbedingungen für kommerzielle Spielenutzung** passen (vor Einsatz prüfen).
- Prompts beschreiben Genre, Instrumentierung, Tempo (BPM), Tonart, Stimmung, Struktur; *instrumental only* angeben.
- Intro/Loop-Paare werden als **ein** Stück mit klarer Struktur generiert und im DAW taktgenau getrennt (Abschnitt 11).

### 2.6 Ergänzende Werkzeuge

| Werkzeug | Einsatz in ASHEN CROWNS |
|---|---|
| **Voice Changer** (Speech to Speech) | Eigene Performance (z. B. Grunzen, Röcheln der Hohlen, Atemzüge der Heldin) aufnehmen und in eine gespeicherte Stimme umwandeln – ideal für nonverbale Vokale mit exaktem Timing |
| **Pronunciation Dictionary** (Aussprache-Wörterbuch, Alias-Regeln) | Eigennamen (Eldraeth, Aureth, Varkhan, Vyrmathax …) einheitlich; Alias-Schreibweisen aus Abschnitt 6 hinterlegen |
| **Voice Isolator** | nur bei Bedarf, um Rauschen/Artefakte aus Voice-Changer-Aufnahmen zu entfernen |
| **Seed** (API/erweiterte Einstellungen) | gleiche Einstellungen + Seed ≈ reproduzierbarere Takes (keine Garantie); Seed im Take-Log notieren |

---

## 3. Produktionspipeline

```
Kanon/Manifest ─► Skript & Prompts (diese Doku) ─► Voice Design (Stimmen anlegen, speichern)
      │                                                     │
      │                                                     ▼
      │                                     TTS-Takes (v3 / v2) · SFX · Ambience · Musik
      │                                                     │
      │                                                     ▼
      │                         Auswahl (Take-Log) ─► Nachbearbeitung (DAW/Batch) ─► Loudness/Peak
      │                                                     │
      ▼                                                     ▼
Dateinamen prüfen ◄──────────── Export .ogg Vorbis q6, 48 kHz ─► Ablage assets/audio/… ─► audio-scan ─► Ingame-Test
```

**Schrittfolge je Produktionswelle:**

1. **Vorbereitung:** Glossar/Aussprache-Wörterbuch in ElevenLabs anlegen (Abschnitt 6).
2. **Casting:** Stimmen nach `elevenlabs_characters.md` per Voice Design erzeugen; je Stimme Beispieltext + 2 Rollenzeilen
   testen; speichern als `AC_<id>`. Casting-Freigabe durch Audio Director (Vergleich aller Stimmen einer Fraktion nebeneinander).
3. **Generierung:** Zeilen aus `elevenlabs_unit_voices.md` / `elevenlabs_dialogues.md` generieren; pro Zeile **mind. 2–3 Takes**.
4. **Auswahl:** besten Take wählen, im Take-Log vermerken (Datei, Modell, Stimme, Einstellungen, Seed, Datum).
5. **SFX/Ambience/Musik:** nach den jeweiligen Dokumenten generieren, je Variante beste Auswahl.
6. **Nachbearbeitung:** Ketten aus Abschnitt 8 (Batch, wo möglich).
7. **Loudness & Peak** nach Abschnitt 7 normalisieren.
8. **Export** nach Abschnitt 9 in den exakten Ziel-Dateinamen.
9. **Ablage** unter `assets/audio/…` (Abschnitt 10), danach `cargo run -p ashen_tools -- audio-scan`.
10. **Ingame-Test** (Mix, Wiederholungsermüdung, Cooldowns), QA-Checkliste (Abschnitt 12), Abnahme.

**Reihenfolge der Produktion (Priorität Vertical Slice):**
1. Heldin `hero_isolde` + `char_hal_venn` + `narrator` (Mission-1-Erlebnis), 2. Glutkrone-Einheiten,
3. Hohle + `boss_pale_reeve`, 4. Kampf-SFX + UI-SFX, 5. Welt-SFX + Ambience, 6. Musik.

---

## 4. Einstellungs-Empfehlungen (Startwerte)

Alle Werte sind Startpunkte; Feinabstimmung pro Stimme im Charakterdokument. Werte als 0–1 bzw. Prozent der UI-Regler.

### 4.1 Eleven v3

| Inhalt | Stabilitätsmodus | Hinweise |
|---|---|---|
| Einheiten select/move | Natural | Batch-Methode; Tags sparsam |
| Einheiten attack/victory | Creative oder Natural | `[shouting]` bei Kampfrufen; bei Übersteuerung Natural |
| hit/death (Schmerz) | Creative | Laut-Tags (`[groans]`, `[exhales]`) + kurze Wörter; mehrere Takes |
| Heldin (alle Barks) | Natural | `[sarcastic]`, `[chuckles]` für trockenen Humor gezielt |
| Dialogszenen | Natural | eine Zeile pro Generierung, Kontextsatz davor möglich (später wegschneiden) |
| Bosse | Natural/Robust | Konsistenz wichtiger; Effekt kommt aus Nachbearbeitung |
| Hohle (wortlose Laute) | Creative | alternativ SFX-Generator, siehe Einheitendokument |

Geschwindigkeit (Speed), falls angeboten: 0,95–1,05; Barks eher 1,0–1,05, Bosse 0,9–0,95.

### 4.2 Multilingual v2

| Inhalt | Stability | Similarity | Style | Speaker Boost | Speed |
|---|---|---|---|---|---|
| Erzähler | 0,55–0,65 | 0,75–0,85 | 0,10–0,20 | an | 0,92–0,97 |
| Ruhige Monologe (Hal Venn, König) | 0,45–0,55 | 0,75–0,80 | 0,15–0,30 | an | 0,95–1,0 |
| Bedrohliche Stimmen (Deep Voice, Hollow King) | 0,60–0,75 | 0,80–0,90 | 0,05–0,15 | an | 0,85–0,92 |

Faustregel: Stability zu niedrig → schwankende Lautstärke/Emotion; zu hoch → monoton. Style über 0,4 erhöht Artefakt-Risiko.

### 4.3 Sound Effects

| SFX-Typ | Dauer | Prompt Influence | Loop |
|---|---|---|---|
| Treffer, Impacts | 0,5–1,2 s | 0,5–0,7 | aus |
| Magie | 1,5–4 s | 0,4–0,6 | aus |
| UI | 0,5–1,5 s (danach kürzen) | 0,6–0,8 | aus |
| Welt/Arbeit | 1–3 s | 0,5–0,6 | aus |
| Ambience-Schichten | max. verfügbare Länge (derzeit ca. 30 s) | 0,3–0,5 | **an** |

### 4.4 Ausgabeformat beim Download

In der **höchsten verfügbaren Qualität** herunterladen: bevorzugt WAV/PCM (48 kHz, falls im Plan verfügbar, sonst 44,1 kHz),
alternativ MP3 in höchster Bitrate. Nie ein bereits verlustbehaftetes MP3 als einzige Master-Datei archivieren, wenn PCM möglich ist.
Resampling auf 48 kHz erfolgt im Export-Schritt.

---

## 5. Schreibregeln für TTS-Texte

1. **Englisch, natürlich gesprochen.** Kurze Sätze; Barks 1–8 Wörter.
2. **Zahlen ausschreiben** („eight hundred ninety-seven“ statt „897“), Abkürzungen auflösen („After the Fall“ statt „A.F.“).
3. **Eigennamen** gemäß Glossar; schwierige Namen per Aussprache-Wörterbuch (Alias), nicht durch falsche Schreibung im Text.
4. **Interpunktion als Regie:** „…“ für Zögern, „—“ für Abbruch, „!“ sparsam (v3 reagiert stark).
5. **Audio-Tags nur in v3** und nur, wo im Dokument angegeben. Tags werden nicht gesprochen; Text in Klammern nie ins Spiel-Untertitel übernehmen.
6. **Untertitel** = EN-Text **ohne** Tags.
7. **Keine Großbuchstaben-Wörter** zur Betonung (können buchstabiert werden); Betonung über Satzbau/Kursivverzicht.

---

## 6. EN-Glossar und Aussprache (für Aussprache-Wörterbuch)

| Kanon (DE) | Englisch im VO | Status | Aussprache-Alias (Vorschlag) |
|---|---|---|---|
| Eldraeth | Eldraeth | Kanon | „EL-dray-eth“ → Alias `Eldrayeth` |
| Aureth | Aureth | Kanon | „OR-eth“ → Alias `Oreth` |
| Solmere | Solmere | Kanon | „SOL-meer“ → Alias `Solmeer` |
| Varkhan Aurelith | Varkhan Aurelith | Kanon | „VAR-kahn or-EL-ith“ → `Varkahn Oreleth` |
| der Letzte Sonnenkönig | the Last Sun King | EN-Arbeitsbegriff | – |
| Das Hohle Tief | the Hollow Deep | Kanon (engl.) | – |
| der Schleier | the Veil | EN-Arbeitsbegriff | – |
| der Aschenfall | the Ashfall | EN-Arbeitsbegriff | – |
| Zeitalter der Asche | the Age of Ash | EN-Arbeitsbegriff | – |
| Aschglas | ashglass | EN-Arbeitsbegriff | – |
| Die Hohlung | the Hollowing | Kanon (engl.) | – |
| n.F. | After the Fall | EN-Arbeitsbegriff | ausschreiben |
| Die Aschmark | the Ashmarch | EN-Arbeitsbegriff (ID `ashmarch`) | – |
| Valemark | Valemark | Kanon | „VALE-mark“ |
| Skarnheim | Skarnheim | Kanon | „SKARN-hime“ → `Skarnhime` |
| Der Dornwald | the Thornwild | Kanon (engl.) | – |
| Saelith | Saelith | Kanon | „SAY-lith“ → `Saylith` |
| Die Aurethwüste | the Aureth Wastes | EN-Arbeitsbegriff | – |
| Die Schleiernarbe | the Veilscar | Kanon (engl.) | – |
| Glutweite | Cinderreach | Kanon (engl.) | – |
| Die Glutkrone | the Ember Crown | Kanon (engl.) | – |
| Die Clans von Skarnheim | the Skarn Clans / the Clans of Skarnheim | EN-Arbeitsbegriff | – |
| Der Dornenbund | the Thorn Covenant | Kanon (engl.) | – |
| Der Saelith-Pakt | the Saelith Compact | EN-Arbeitsbegriff (ID) | – |
| Die Hohlen | the Hollowed | Kanon (engl.) | – |
| Die Aschenlegion | the Ashen Legion | EN-Arbeitsbegriff (ID) | – |
| Veyl | Veyl | Kanon | „VAIL“ |
| Ewige Flamme | the Eternal Flame | EN-Arbeitsbegriff | – |
| Erzflammenwahrer | the Arch-Flamekeeper | EN-Arbeitsbegriff | – |
| Kaldris | Kaldris | Kanon | „KAL-driss“ |
| Ahnenpfad / Grüner Kreis / Haus der Gezeiten | the Ancestor Path / the Green Circle / the Tide House | EN-Arbeitsbegriffe | – |
| Leere Liturgie / Sonnenhäresie | the Hollow Liturgy / the Sun Heresy | EN-Arbeitsbegriffe | – |
| Ithren, Morvaine, Calyx, Thal, Oruun | dto. | Kanon | „ITH-ren“, „mor-VAIN“, „KAY-lix“, „THAHL“, „oh-ROON“ |
| Der Hohle König | the Hollow King | EN-Arbeitsbegriff | – |
| Aschdrache Vyrmathax | the Ashen Dragon Vyrmathax | EN-Arbeitsbegriff | „VEER-ma-thax“ → `Veermathax` |
| Die Wurzelmutter | the Root Mother | EN-Arbeitsbegriff | – |
| Der Fahle Vogt | the Pale Reeve | EN-Arbeitsbegriff (ID) | – |
| Brennhollow | Brennhollow | Kanon | „BREN-hollow“ |
| Hohlschrein von Vael Dorn | the Hollow Shrine of Vael Dorn | EN-Arbeitsbegriff | „VAIL DORN“ |
| Isolde Raventhorn | Isolde Raventhorn | Kanon | „ee-SOL-duh RAY-ven-thorn“ → `Eesolda` |
| Wächterin der Aschmark | Warden of the Ashmarch | EN-Arbeitsbegriff | – |
| Glutklingen-Adeptin | Emberblade adept | EN-Arbeitsbegriff | – |
| Glutspalter / Sammelbanner / Aschenschild | Ember Cleave / Rallying Banner / Ashen Ward | EN aus IDs | – |
| Aldric III. | King Aldric the Third | Kanon | – |
| Mireille Corvane | Mireille Corvane | Kanon | „meer-AY kor-VANE“ → `Meeray` |
| Nordmark | Nordmark | Kanon | „NORD-mark“ |
| Hal Venn | Brother Hal Venn | Kanon | – |
| Torvald Eisenhand | Torvald Eisenhand | Kanon (Name bleibt) | „TOR-vald EYE-zen-hand“ |
| Sylaen | Sylaen | Kanon | „sih-LAY-en“ → `Silayen` |
| Ophira Saelith-Varo | Ophira Saelith-Varo | Kanon | „oh-FEE-ra SAY-lith VAR-oh“ |
| Kassian Vale | Kassian Vale | Kanon | „KASS-ee-an“ |
| Die Stimme aus dem Tief | the Voice from the Deep | EN-Arbeitsbegriff | – |

Alias-Einträge zuerst mit einem Testsatz pro Name prüfen; Alias nur setzen, wenn die Standardaussprache falsch ist.

---

## 7. Klangziele (Loudness & Peak)

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

## 8. Nachbearbeitung

Werkzeuge: beliebige DAW (z. B. Reaper, Audacity) bzw. Batch per `ffmpeg`/`sox`. Immer auf Kopie der Master-Takes arbeiten.

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

1. Schnitt auf Transienten (keine Vorlaufstille > 5 ms bei Impacts), Fade-out natürlich.
2. Hochpass je nach Quelle (UI 150 Hz, Impacts 40 Hz), DC-Offset entfernen.
3. Rauschen/Artefakte des Generators (Zirpen, „Wasser“-Artefakte) prüfen; ggf. Take verwerfen.
4. Leichte Transientenbetonung bei Kampf; Limiter −1 dBTP; Loudness gemäß Abschnitt 7.
5. Mono-Summierung prüfen (Phasenauslöschung!), dann Mono-Export für positionierte Sounds.

### 8.3 Spezialbehandlungen (Fraktions-Klangsignaturen)

| Ziel | Behandlung |
|---|---|
| **Hohle** (`hollowed`, Einheiten, Fahler Vogt) | Pitch −2 bis −4 Halbtöne mit Formant-Erhalt *oder* Formant −2; Parallel-Layer: geflüsterte Dopplung (−12 dB, 30 ms versetzt); leichter Reverse-Reverb-Vorlauf (100–200 ms) nur bei Boss; Bandpass-„Hohlheit“ (Notch 1–2 kHz −4 dB); dezente Granular-/Bitcrush-Körnung (Mix ≤ 15 %) |
| **Fahler Vogt** (`boss_pale_reeve`) | wie Hohle + Chorus/Doppler-Schicht −3 Halbtöne (Mix 25 %); kurzer, kalter Hall (Kathedrale, 1,8 s, Mix 12 %), da er im Hohlschrein auftritt |
| **Aschenlegion** (`ashen_legion`) | Helm-Resonanz: Kammfilter/Kurzhall 3–6 ms, Bandpass 250 Hz–5 kHz, leichte Sättigung; Glut-Knistern als leiser Layer (−24 dB) |
| **Die Stimme aus dem Tief** (`char_deep_voice`) | drei Schichten: (1) Original, (2) −12 Halbtöne (Mix 40 %), (3) Flüster-Take derselben Zeile (Mix 30 %); Subharmonik 40–60 Hz; langes, dunkles Hallfeld 4–6 s, Pre-Delay 80 ms; leichte Zeit-Desynchronisierung der Layer (10–40 ms) |
| **Hohler König** (`boss_hollow_king`) | menschlich-königliche Basis; einzelne Wörter mit Deep-Voice-Layer (≤ 20 %) unterlegen; Thronsaal-Hall 2,5 s |
| **Aschdrache Vyrmathax** (`boss_ashen_dragon`) | Basisstimme −5 bis −7 Halbtöne, Layer aus SFX „large reptile growl, low rumble“ (−10 dB) unter Wortanfängen; Glut-/Feuer-Rauschen; großer Höhlenhall |
| **Wurzelmutter** (`boss_root_mother`) | Chor-Dopplung (3 Takes, je ±10 Cent, 15–25 ms versetzt), Holz-Knarz-SFX als Untergrund, warmer Waldhall |
| **Veyl** (Sylaen, Dornenbund-Veyl) | sehr dezente Luftigkeit: Hochregal +2 dB ab 10 kHz, kurzer heller Hall; optional leiser Flügelflatter-Layer (Motte) bei Ruf-Zeilen |
| **Erzähler** | kein Effekt außer minimal „Wärme“ (Sättigung ≤ 5 %); extrem saubere Grundqualität |

---

## 9. Export

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

**Masters archivieren:** WAV-Masters (48 kHz/24 bit), Rohtakes und Take-Log außerhalb von `assets/audio/` sichern
(eigener Speicherort des Auftraggebers). In `assets/audio/` liegen **nur** die finalen Spieldateien.

---

## 10. Dateibenennung und Ablage (gemäß Kanon)

Alle Dateien klein, `snake_case`, `.ogg` bevorzugt. `<NN>` = zweistellige Variantennummer ab `01`, lückenlos.

```
assets/audio/music/<track_id>[_intro|_loop].ogg
assets/audio/voices/heroes/<hero_id>_<situation>_<NN>.ogg
assets/audio/voices/units/<unit_id>_<situation>_<NN>.ogg
assets/audio/voices/characters/<char_id>_<line_id>.ogg
assets/audio/voices/narrator/narrator_<line_id>.ogg
assets/audio/sfx/<category>/<sfx_id>[_NN].ogg        category ∈ combat, magic, buildings, ui, units, world
assets/audio/ambience/<ambience_id>[_loop].ogg
```

**Regeln:**
- Manifest-Pfade sind relativ zu `assets/audio` und ohne Endung angegeben; Muster `_*` = `_01`, `_02`, … lückenlos.
- Der Boss `boss_pale_reeve` liegt laut Manifest unter `voices/units/` (nicht `heroes/`) – exakt so ablegen.
- Dialog-/Erzählerzeilen mit mehreren Teilen (`_01`, `_02` …) sind **Szenensegmente** oder **Alternativen** – siehe
  Abspielmodus-Angabe in `elevenlabs_dialogues.md`. Dateinamen sind in beiden Fällen gleich aufgebaut.
- Kampagnen-Zeilen außerhalb des Manifests nutzen die vorgeschlagenen IDs aus `elevenlabs_dialogues.md` (Präfix `aNN_` für
  Zeitalter). Für Dialogzeilen der Heldin wird `voices/characters/hero_isolde_<line_id>.ogg` **vorgeschlagen** – vor
  Produktion mit dem Kanon-Owner bestätigen, ggf. Manifest/Kanon ergänzen lassen.
- Keine Leerzeichen, Umlaute, Großbuchstaben, Bindestriche oder Versionssuffixe (`_final`, `_v2`) in Spieldateien.

---

## 11. Musik (Manifest-Abdeckung)

Das Manifest erwartet 14 Musikdateien. Erzeugung mit **Eleven Music** (falls verfügbar/lizenziert) oder durch einen Komponisten
nach denselben Vorgaben. Musikalische Identität: *Glutkrone* = Celli, tiefe Hörner, Rahmentrommeln, Drehleier, Frauenchor
(wortlos); *Hohle* = verstimmte Streicher, gestrichenes Metall, tiefer Männerchor (wortlos), Atemgeräusche.
Alle Stücke **instrumental**, keine verständlichen Gesangstexte.

**Intro/Loop-Verfahren:** Ein Stück mit klar beschriebener Struktur generieren (Intro + Hauptteil), im DAW auf Taktgrenzen
trennen. `_intro` endet exakt dort, wo `_loop` beginnt (gleiches Tempo, gleiche Tonart). `_loop` muss Ende→Anfang nahtlos
übergehen (Nachhall des Loop-Endes per Crossfade an den Loop-Anfang zurückfalten). Loudness −14 LUFS integriert (Referenz), −1 dBTP, stereo.

| Ziel-Datei | Länge (Ziel) | Tempo/Tonart | Englischer Prompt | Schnitt/Hinweis |
|---|---|---|---|---|
| `music/main_menu_intro.ogg` | 15–25 s | 70 BPM, D-Moll | „Dark fantasy orchestral intro, instrumental only. A lone hurdy-gurdy drone and distant wind, then low solo cello states a mournful noble theme, deep horn swell, soft wordless female choir rising, ends on an open suspended chord leading into the main theme. Melancholic, ancient, ash-covered kingdom, cinematic but intimate.“ | endet auf Takt-Eins des Loops |
| `music/main_menu_loop.ogg` | 90–150 s | 70 BPM, D-Moll | „Dark fantasy orchestral main theme, instrumental only, 70 BPM, D minor. Cello and viola melody over warm low strings, slow frame drum heartbeat, hurdy-gurdy drone, distant wordless female choir, occasional deep horn calls. Noble, grieving, hopeful undertone. Steady, loopable, no big ending.“ | nahtloser Loop, kein Ausklang |
| `music/build_calm_intro.ogg` | 10–20 s | 84 BPM, G-Dorisch | „Gentle medieval folk intro, instrumental only. Plucked lute and soft hand drum, morning birds feeling, a simple wooden flute melody begins, warm and fragile, a small village rebuilding after catastrophe.“ | geht direkt in Loop |
| `music/build_calm_loop.ogg` | 120–180 s | 84 BPM, G-Dorisch | „Calm medieval folk underscore for a strategy game base-building phase, instrumental only, 84 BPM, G dorian. Lute arpeggios, wooden flute, soft cello pads, light frame drum, subtle hurdy-gurdy. Hopeful but weary, unobtrusive, sits under speech, no sudden dynamics, loopable.“ | dezent, Dialog-tauglich (Mitten sparsam) |
| `music/build_tension_loop.ogg` | 90–150 s | 84 BPM, G-Moll | „Tense dark fantasy underscore, instrumental only, 84 BPM, G minor. Low string ostinato, muted pulsing frame drum, distant bowed metal and creaking textures, hurdy-gurdy drone turned dissonant, sparse cello motif. Something is coming from the ruins at night. Restrained, loopable.“ | gleiche BPM wie Calm für Crossfade |
| `music/battle_small_intro.ogg` | 6–12 s | 120 BPM, D-Moll | „Short aggressive orchestral battle intro, instrumental only. Snare roll and war drum hits, low brass stab, rising string tremolo, launches into a fast skirmish rhythm.“ | endet auf Downbeat |
| `music/battle_small_loop.ogg` | 60–120 s | 120 BPM, D-Moll | „Dark fantasy skirmish music, instrumental only, 120 BPM, D minor. Driving frame drums and taiko, spiccato strings ostinato, short horn motifs, hurdy-gurdy rhythmic drone, no choir. Gritty, urgent, small-scale fight at a village edge. Loopable, consistent energy.“ | mittlere Intensität |
| `music/battle_large_intro.ogg` | 8–15 s | 132 BPM, C-Moll | „Epic dark fantasy battle intro, instrumental only. Massive war drums, full low brass fanfare, wordless male and female choir shout-like accents, string runs, builds into a full battle theme.“ | – |
| `music/battle_large_loop.ogg` | 90–150 s | 132 BPM, C-Moll | „Epic dark fantasy battle music, instrumental only, 132 BPM, C minor. Full orchestra, heavy taiko and war drums, driving low strings, heroic horn theme, wordless choir, metallic percussion hits. Desperate, heroic, ash and fire. Loopable without a final cadence.“ | höchste Intensität (außer Boss) |
| `music/boss_pale_reeve_intro.ogg` | 10–20 s | 96 BPM, B-Moll (Phrygisch) | „Ominous undead boss intro, instrumental only. A single tolling funeral bell, breathy whispers-like textures, detuned violins sliding down, deep male choir hum, sudden heavy drum hit leading into the boss theme.“ | Glocke als Signatur des Vogts |
| `music/boss_pale_reeve_loop.ogg` | 90–150 s | 96 BPM, B-Phrygisch | „Undead boss battle music, instrumental only, 96 BPM, B phrygian. Tolling bell on every four bars, heavy war drums, dissonant low brass clusters, detuned string tremolo, bowed metal screeches, deep wordless male choir. Cold, ritualistic, like a grim tax collector of souls. Loopable.“ | Glocke im Loop taktgenau |
| `music/victory_sting.ogg` | 5–8 s | frei, D-Dur | „Short dark fantasy victory sting, instrumental only. Bright horn fanfare over warm strings and a single choir swell, resolves to D major, bittersweet relief, ends with a natural ring-out.“ | kein Loop, natürlicher Ausklang |
| `music/defeat_sting.ogg` | 5–8 s | frei, D-Moll | „Short dark fantasy defeat sting, instrumental only. Low cello falling line, muffled drum hit, fading hurdy-gurdy drone, ash and silence, ends unresolved and dark.“ | kein Loop |

---

## 12. Qualitätscheckliste (pro Datei abhaken)

**Inhalt & Benennung**
- [ ] Dateiname exakt wie im Dokument/Manifest (klein, snake_case, `_NN` zweistellig, lückenlos ab `01`).
- [ ] Richtiger Ordner (`voices/heroes`, `voices/units`, `voices/characters`, `voices/narrator`, `sfx/<category>`, `ambience`, `music`).
- [ ] Text wörtlich wie im Skript (Abweichungen im Take-Log begründet); Eigennamen korrekt ausgesprochen.
- [ ] Mindestvariantenzahl erfüllt (Sprach-Events ≥ 3, select/move/attack 4–5, SFX-Varianten 3).

**Klang**
- [ ] Keine KI-Artefakte (metallisches Flattern, Phasing, verschluckte Silben, Fremdgeräusche, Atem-Glitches).
- [ ] Stimme passt zum gespeicherten Voice-Design (gleiche Figur klingt in allen Dateien gleich).
- [ ] Emotion passt zur Situation (Regieanweisung); Tags wurden nicht mitgesprochen.
- [ ] Loudness im Zielbereich, True Peak ≤ −1 dBTP, kein Clipping, kein DC-Offset.
- [ ] Kopf-/Schwanzstille im Rahmen; Fades vorhanden; kein Klick.
- [ ] Mono-kompatibel (bei Mono-Export keine Auslöschung).
- [ ] Varianten einer Familie gleich laut und klar unterscheidbar.
- [ ] Loops: nahtlos (keine Klicks, kein Pegelsprung, keine erkennbare „Wiederholungs-Signatur“ im 30-s-Takt).

**Technik**
- [ ] Ogg Vorbis, q6, 48 kHz, korrekte Kanalzahl.
- [ ] `cargo run -p ashen_tools -- audio-scan` ohne Fehler für die Lieferung.
- [ ] Ingame gehört: Mix gegen Musik & Ambience, Wiederholungsermüdung (10× hintereinander auslösen), Cooldown-Verhalten.

**Rechtliches**
- [ ] Generiert mit einem Plan, der kommerzielle Nutzung erlaubt; keine Imitation realer Personen.

---

## 13. Lieferprozess

1. **Ablage:** alle finalen Dateien direkt unter `assets/audio/…` gemäß Abschnitt 10 (Ordner existieren bereits:
   `music/`, `voices/{heroes,units,characters,narrator}/`, `sfx/{combat,magic,buildings,ui,units,world}/`, `ambience/`).
2. **Scanner ausführen** im Repository-Wurzelverzeichnis:
   ```bash
   cargo run -p ashen_tools -- audio-scan
   ```
   Der Scanner prüft die Dateinamenskonvention und gleicht vorhandene Dateien mit dem Manifest ab (fehlende Varianten,
   unerwartete Dateien, falsche Benennung). Alle Meldungen beheben, erneut scannen. *(Falls das Werkzeug zum Lieferzeitpunkt
   noch nicht im Repository vorhanden ist, Abgleich manuell anhand der Tabellen in diesen Dokumenten durchführen.)*
3. **Ingame-Abnahme** mit Mission `m01_last_settlement`: Einheiten anwählen/bewegen/angreifen, Gebäude bauen, eine Nacht
   und eine Welle erleben, Hohlschrein finden, Fahlen Vogt bekämpfen, Sieg und Niederlage auslösen.
4. **Take-Log** (siehe 14) zusammen mit der Lieferung übergeben (außerhalb `assets/audio/`, Ablageort nach Absprache).
5. **Nachlieferungen** ersetzen Dateien unter **gleichem Namen**; neue Varianten werden lückenlos angehängt (`_04`, `_05` …).
   Variantenzahl erhöhen nur in Absprache (Engine wählt aus allen vorhandenen Varianten).
6. **Abnahmekriterium Vertical Slice:** Scanner ohne fehlende Manifest-Dateien, QA-Checkliste vollständig, Audio Director-Freigabe.

---

## 14. Take-Log (Pflichtfelder)

| Feld | Beispiel |
|---|---|
| Ziel-Datei | `voices/heroes/hero_isolde_select_01.ogg` |
| Werkzeug / Modell | TTS / Eleven v3 |
| Stimme | `AC_hero_isolde` |
| Einstellungen | Natural, Speed 1.0 |
| Seed (falls genutzt) | 418223 |
| Text (final) | „Raventhorn. Speak.“ |
| Take-Nr. / Datum | 3 / 2026-11-24 |
| Nachbearbeitung | Standardkette Stimme, −16 LUFS-S |
| Freigabe | AD ✔ |

---

## 15. Mengengerüst Vertical Slice (laut diesen Dokumenten)

| Bereich | Events | Dateien |
|---|---|---|
| Einheiten-Stimmen (Glutkrone) | 37 | 136 |
| Hohle-Einheiten | 9 | 30 |
| Heldin `hero_isolde` | 11 | 42 |
| Boss `boss_pale_reeve` | 5 | 17 |
| Erzähler Mission 1 | 4 | 13 |
| Hal Venn Mission 1 | 6 | 20 |
| SFX | 33 | 99 |
| Ambience | 6 | 6 |
| Musik | 8 Tracks | 14 |
| **Summe** | 105 Events + 6 Ambience + 8 Tracks | **377** |
