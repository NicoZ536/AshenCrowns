# ASHEN CROWNS – The Age of the Fallen
# Suno-Master-Produktionsdokument (Musik)

**Rolle:** Music Director / Komponist · **Produktionswerkzeug:** Suno (Produktion durch den Auftraggeber)
**Verbindliche Quellen:** `docs/canon.md` (Namen, Fraktionen, Dateinamenskonvention) und
`assets/audio/manifest.ron` (Abschnitt `music`, Track-IDs, Dateien, Feld `intensity`).
**Ablage aller Musikdateien:** `assets/audio/music/` – Dateinamen exakt nach Kanon:
`assets/audio/music/<track_id>[_intro|_loop].ogg`

Dieses Dokument ist die vollständige musikalische Bibel des Spiels: Identität, Suno-Arbeitsweise,
dynamisches Musiksystem, eine Track-Bibel mit fertigen Prompts für **48 Tracks** sowie Checkliste und
Prioritäten. Alle Suno-spezifischen Funktionshinweise sind **Empfehlungen**: Funktionsumfang, Modellversion,
Maximallängen, Exportformate und Nutzungsrechte ändern sich – **zum Produktionszeitpunkt in der genutzten
Suno-Version und im gebuchten Plan prüfen.**

---

## Inhalt

1. Musikalische Identität
2. Arbeitsweise mit Suno
3. Dynamisches Musiksystem
4. Track-Bibel (alle Tracks)
5. Produktions-Checkliste
6. Prioritätenliste

---

## 1. Musikalische Identität

### 1.1 Leitgedanke

Eldraeth lebt im **Zeitalter der Asche**: Ein Reich ist gefallen, die Götter schweigen, und unter der Erde
atmet das **Hohle Tief**. Die Musik erzählt drei Kräfte, die im ganzen Spiel gegeneinander arbeiten:

| Kraft | Bedeutung im Spiel | Musikalische Übersetzung |
|---|---|---|
| **Asche** | Verlust, Erinnerung an Aureth, Kälte der Gegenwart | Moll/Äolisch, tiefe Streicher, Solo-Cello, Stille als Gestaltungsmittel, trockene Räume |
| **Glut** | Überlebenswille, Glaube der Ewigen Flamme, Isolde, die Glutkrone | Dorisch, Hörner, warme Bratschen, Frauenstimme, Rahmentrommeln, aufsteigende Quinten |
| **Hohlung** | Das Hohle Tief, die Hohlen, die Aschenlegion, Korruption | Tritonus, Cluster, Sul-ponticello-Streicher, rückwärts laufende Klänge, flüsternder Chor, Subbass-Atem |

Grundregel: **Jeder Track trägt mindestens zwei dieser Kräfte.** Selbst friedliche Musik hat einen Schatten,
selbst die dunkelste Musik einen Funken.

### 1.2 Leitmotiv „Die Aschenkrone“

Das Leitmotiv verkörpert die Krone Aureths – einst Sonne, dann Asche, im Endgame wieder umkämpft. Es ist
**zweitaktig**, steht im 4/4-Takt und wird als Stufenfolge notiert, damit es in jeder Tonart funktioniert.

**Grundform „Asche“ (äolisch), in D notiert:**

```text
Takt 1 („Die Krone“):   D (Viertel)  –  A (Halbe)  –  B♭ (Achtel)  –  A (Achtel)
Takt 2 („Der Fall“):    G (punkt. Viertel) – F (Achtel) – E (Achtel) – C (Achtel) – D (Viertel)
Stufen:                 1 – 5 – ♭6 – 5  |  4 – ♭3 – 2 – ♭7 – 1
```

**Intervall-Dramaturgie:**

| Element | Intervall | Bedeutung |
|---|---|---|
| Aufschwung | reine Quinte aufwärts (1→5), lang gehalten | Die Krone, Würde, das alte Reich |
| Riss | kleine Sekunde aufwärts und zurück (5→♭6→5), kurz-kurz | Der Riss in der Krone, der Fluch, ein Seufzer |
| Fall | stufenweiser Abstieg über eine Quarte (4→♭3→2) | Der Aschenfall |
| Landung ohne Leitton | Ganzton von unten (♭7→1) | Keine Erlösung: Die Kadenz schließt, aber ohne Strahlen |

**Rhythmische Signatur „Kronenschlag“:** Achtelgruppierung **3 + 3 + 2** (Akzente auf Zählzeit 1, 2+ und 4),
gespielt von tiefen Trommeln, Pauken oder Col-legno-Streichern. In ruhigen Tracks erscheint der Kronenschlag
als langsamer Herzschlag (halbe Geschwindigkeit, nur gedämpfte Pauke), in Kampftracks als treibendes Ostinato.
Der Kronenschlag ist das rhythmische Erkennungszeichen, das auch dann trägt, wenn Suno die Melodie nicht
exakt trifft.

**Erscheinungsformen (immer gleiche Kontur, veränderte Stufen):**

| Form | Stufen | Einsatz | Klangfarbe |
|---|---|---|---|
| **Asche** (Grundform) | 1 – 5 – ♭6 – 5 · 4 – ♭3 – 2 – ♭7 – 1 | Menü, Städtebau, Story, Weltkarte | Solo-Cello, Bratschen, Altflöte |
| **Glut** (dorisch) | 1 – 5 – **6** – 5 · 4 – ♭3 – 2 – ♭7 – 1 | Hoffnung, Isolde, Glutkrone, Kampfbeginn | Hörner unisono, Frauenstimme |
| **Krone** (Dur, selten) | 1 – 5 – 6 – 5 · 4 – **3** – 2 – **7** – 1 | Nur Siegmomente, Credits, Finale | volles Blech, Chor, Glocken |
| **Hohl** (Tritonus) | 1 – **♭5** – 5 – ♭5 · 4 – ♭3 – **♭2** – ♭7 – 1 | Die Hohlen, Bosse, Schleiernarbe, Niederlage | tiefe Posaunen, flüsternder Chor, verfremdete Streicher |

**Verwendungsregeln**
- Das vollständige zweitaktige Motiv erklingt **höchstens zweimal pro Loop**; sonst nur Takt 1 („Die Krone“)
  als Fragment, damit es sich bei langen Sessions nicht abnutzt.
- Die **Krone-Form** (Dur) ist für Belohnungsmomente reserviert (Sieg, Credits, Finale) und erscheint vorher nie.
- Die **Hohl-Form** erscheint ausschließlich in Verbindung mit Hohlung, Hohlem Tief, Aschenlegion oder Bossen.
- Im Kampf wird das Motiv in Augmentation (doppelte Notenwerte) von tiefem Blech über das Ostinato gelegt.

**Wie das Motiv in Suno landet (Empfehlung):** Suno setzt keine Noten exakt um. Deshalb zwei Wege kombinieren:
1. **Prompt-Beschreibung:** Formulierungen wie „rising fifth motif with half-step sigh“ oder
   „solemn horn call, rising fifth, falling line“ im Style-Feld.
2. **Motiv-Seed (eigenes Material):** Der Auftraggeber spielt das Motiv selbst ein (Klavier, Cello oder
   gerenderte MIDI-Skizze, 8–16 Takte, korrektes Tempo und Tonart des Sets) und nutzt es – sofern die genutzte
   Suno-Version Audio-Upload bzw. Cover von eigenem Material erlaubt – als Ausgangspunkt. Nur eigenes,
   rechtefreies Material hochladen. Takes, in denen die Kontur (Aufschwung – Riss – Fall) hörbar ist,
   werden bevorzugt ausgewählt.

### 1.3 Nebenmotive und Fraktionskennungen

| Kennung | Zugehörigkeit | Beschreibung | Typischer Träger |
|---|---|---|---|
| **Glutruf** | Glutkrone von Valemark | Hornruf 1 – 5 – 8, punktiert, endet offen auf der 5 | Hörner, Naturtrompete |
| **Ahnenschritt** | Clans von Skarnheim | Schwerer Zweierschritt, tiefe Männerstimmen auf Quinten-Bordun, Ruf „Haa – o-run“ | Männerchor, Drehleier, große Rahmentrommeln |
| **Wurzelreigen** | Dornenbund | 6/8-Wiegefigur, pentatonisch, Bernstein-Klang | Holzflöte, Harfe, gezupfte Leier |
| **Gezeitenlauf** | Saelith-Pakt | Wellenförmige Arpeggien auf- und abwärts, mixolydische Septime | Hackbrett, Harfe, Zither, Streicher-Spiccato |
| **Hohlatem** | Die Hohlen / Hohles Tief | Langsames Anschwellen eines Clusters im Atemrhythmus (4 Schläge ein, 4 aus), darunter Subbass | Chor geflüstert, Glasharmonika-Klang, Streicher sul ponticello |
| **Legionsmarsch** | Aschenlegion | Kronenschlag als Marsch, Hohl-Form des Leitmotivs im Blech, Amboss auf Zählzeit 4 | Tiefes Blech, Amboss, Männerchor |
| **Tiefenstimme** | Die Stimme aus dem Tief | Ein einzelner tiefer Ton, der sich um einen Viertelton verbiegt | Bassklarinette, Kontrabass, verfremdete Stimme |

### 1.4 Klangpalette

**Kernorchester (in fast allen Tracks):** tiefe Streicher (Celli, Kontrabässe), Bratschen, Solo-Cello,
Hörner, gedämpfte Pauken, große Rahmentrommeln, Altflöte, Harfe, Kirchenglocke in großer Entfernung,
Frauenstimme solo (wortlos oder Old Aurethi), gemischter Chor (Old Aurethi).

**Farben je Sphäre**

| Sphäre | Zusätzliche Instrumente | Raum |
|---|---|---|
| Valemark / Glutkrone | Naturtrompeten, Laute, Fidel, Glocken, Snare-Rührtrommel | Steinhalle, mittlerer Hall |
| Skarnheim | Drehleier, Streichleier, tiefe Männerstimmen, Bordunpfeifen, Kriegshörner aus Horn | Weiter Bergraum, langer kalter Hall |
| Dornwald | Holzflöten, gezupfte Leier, Harfe, Holzperkussion, Rindentrommeln, Flüsterchor | Dichter Wald, kurzer warmer Raum |
| Saelith-Inseln | Hackbrett, gezupfte Kastenzither, Harfe, Marimba-artige Holzklänge, Handtrommeln, Streicher-Spiccato | Offener Hafen, hell, mittlerer Hall |
| Hohle / Schleiernarbe | Glasharmonika-Klang, Waterphone-artige Effekte, rückwärts laufende Streicher, Cluster-Chor, Subbass | Unnatürlich lang, dunkel, gefiltert |
| Aschenlegion / Aureth | Tiefes Blech, Amboss, Ketten, verfremdete Fanfaren, Männerchor | Riesige leere Halle, Ruinenecho |
| Glutweite (Drache) | Taiko-artige Großtrommeln, Metallperkussion, verzerrte Blechcluster, Feuerrauschen | Vulkankessel, drückend |

**Ausdrücklich ausgeschlossen (Klang):** EDM-Drops, Trap-Hi-Hats, Dubstep-Bässe, Autotune, Pop-Songstrukturen
mit gesungenem Refrain, E-Gitarren-Riffs, Synth-Leads im Vordergrund, Rap, verständliche englische oder deutsche
Liedtexte. Hybride Elemente (Subbass-Impulse, Riser, Reverse-Effekte) nur dezent in Battle-, Boss- und
Hohlungs-Tracks.

### 1.5 Tonarten- und Modus-Konzept

**Modus = Bedeutung**

| Modus | Bedeutung | Typische Tracks |
|---|---|---|
| Dorisch | Glut, Hoffnung, Gemeinschaft | build_calm, Glutkrone, Weltkarte Frühling |
| Äolisch | Asche, Ernst, Gegenwart | main_menu, build_tension, Schlachten |
| Mixolydisch | Wohlstand, Handel, Aufbruch | economy_prosperity, Saelith, Weltkarte Sommer |
| Phrygisch | Bedrohung, Fremdheit, Bosse | boss_pale_reeve, Intrige, Belagerungsverteidigung |
| Lokrisch / Tritonus-Färbung | Das Hohle Tief | Die Hohlen, Schleiernarbe, Enthüllung |
| Lydisch | Göttliches, Unerreichbares | mystery_old_gods |
| Dur (ionisch) | Nur Belohnung (Krone-Form) | Sieg-Momente, Credits-Finale |

**Tonika-Architektur:** Das Spiel beginnt und endet in **D**. D ist die Tonika der Aschmark (Startregion),
des Hauptmenüs und des Endgames in Solmere. Die Hohlen stehen auf **G♯/A♭** – dem Tritonus zu D, der
„Gegenwelt“. Die übrigen Fraktionen und Spielmodi erhalten eigene Tonika (siehe Musik-Sets, Kapitel 3.2).

### 1.6 Old Aurethi – Fantasiesprache für Chöre

Old Aurethi ist die erfundene Hof- und Ritualsprache des untergegangenen Reiches Aureth. Sie klingt
lateinisch-feierlich, ist aber **keine reale Sprache**. Alle Chortexte in diesem Dokument sind in Old Aurethi
oder als bedeutungsfreie Vokalisen verfasst.

**Aussprache-Regeln:** Betonung auf der vorletzten Silbe; „ae“ = „ä“; „th“ = stimmloses englisches th;
„kh“ = deutsches „ch“ in „Bach“; Doppelvokale getrennt sprechen. **Im Suno-Lyrics-Feld ohne Sonderzeichen
schreiben** (Spalte „Suno-Schreibung“), Silben bei Bedarf mit Bindestrich trennen.

| Old Aurethi | Suno-Schreibung | Bedeutung |
|---|---|---|
| aurē | aureh | Krone |
| cindra | cindra | Asche |
| kael | kael | Glut, Flamme |
| solvae | sol-vae | Sonne |
| vaeloth | vae-loth | Schleier |
| dhum | dhum | hohl, leer |
| umbrath | um-brath | Tiefe, das Tief |
| morra | mor-ra | Tod, Übergang |
| ithra | ith-ra | Licht |
| calyre | ca-li-reh | Wachstum, Wald |
| thalen | tha-len | Meer, Gezeit |
| orun | o-run | Stein, Beständigkeit |
| veth | veth | Eid, Treue |
| esk | esk | fallen |
| esketh | es-keth | fiel (Vergangenheit: Endung -eth) |
| rhan | rhan | steigen |
| rhana | rha-na | erhebe dich (Befehl: Endung -a) |
| liraen | li-ra-en | Erinnerung |
| sereth | se-reth | Klage |
| domae | do-mae | Heim, Haus |
| eldra | el-dra | Welt |
| varkh | varkh | König |
| nos | nos | wir, uns |
| ve | veh | und |
| en | en | in, im |
| ka | ka | nicht, kein |
| ar | ar | ewig |
| sa | sa | der, die, das |
| tor | tor | Tor, Pforte |
| vael | vael | Ende |

Plural: Endung **-ae** (aurē → auræ, Suno-Schreibung „au-rae“). Genitiv durch Nachstellung:
„aurē cindra“ = „Krone der Asche“.

**Kernsätze (wiederverwendbar):**
- *Aurē cindra, aurē dhum.* – Krone aus Asche, hohle Krone.
- *Kael rhana, nos veth.* – Glut, erhebe dich, wir schwören.
- *Solvae esketh, eldra sereth.* – Die Sonne fiel, die Welt klagt.
- *Umbrath ka morra.* – Das Tief kennt keinen Tod.
- *Ithra ar, kael ar.* – Ewiges Licht, ewige Glut.
- *Vaeloth dhum, umbrath rhan.* – Der Schleier ist leer, das Tief steigt.
- *Liraen sa domae.* – Erinnerung an die Heimat.
- *Varkh dhum, tor umbrath.* – Hohler König, Pforte zur Tiefe.

### 1.7 Verbot-Liste (verbindlich)

1. **Keine Künstlernamen in Suno-Prompts oder Lyrics** – weder Komponisten, Bands, Sänger noch Produzenten,
   auch nicht als „im Stil von“, „inspiriert von“ oder in Abkürzungen.
2. **Keine Werk-, Film-, Serien- oder Spieltitel** in Prompts (keine Referenzen auf bekannte Soundtracks).
3. **Keine Imitation bekannter Soundtracks:** keine Nachbildung bekannter Themen, Ostinati, Akkordfolgen oder
   Signaturklänge. Jeder ausgewählte Take wird gegengehört; bei auffälliger Ähnlichkeit zu bekannter Musik
   wird er verworfen.
4. **Keine realen liturgischen oder lateinischen Texte** (z. B. Messtexte) und keine bekannten
   Choral-Melodien – Chöre singen ausschließlich Old Aurethi oder Vokalisen.
5. **Keine verständlichen Liedtexte** in Englisch, Deutsch oder anderen realen Sprachen.
6. **Keine Stimmenimitation** realer Sängerinnen und Sänger; keine Uploads fremder Aufnahmen für Cover oder
   Referenz – nur eigene Motiv-Seeds und eigene Suno-Takes.
7. **Keine kulturellen Stereotype als Stil-Etikett:** Fraktionen werden über Instrumente und Stimmung
   beschrieben, nicht über reale Ethnien, Religionen oder Nationen.
8. **Keine Klänge aus der Ausschlussliste in 1.4** (EDM-Drops, Trap, Dubstep, Autotune, E-Gitarren-Riffs,
   Pop-Refrains, Rap).
9. **Keine Marken- oder Produktnamen** in Prompts oder Dateinamen.
10. **Keine Abweichung von der Dateinamenskonvention** – der Asset-Scanner prüft sie.

---

## 2. Arbeitsweise mit Suno

> **Versionshinweis:** Alle Angaben zu Funktionen (Custom-Modus, Instrumental-Schalter, Extend, Cover,
> Audio-Upload, Personas, Bearbeiten einzelner Abschnitte, Stems, Exportformate, Maximallängen) sind
> Empfehlungen auf Basis typischer Suno-Workflows. **Vor Produktionsbeginn in der aktuell genutzten
> Suno-Version prüfen**, welche Funktionen, Modellversionen, Längenlimits und Exportformate verfügbar sind und
> welche Nutzungsrechte der gebuchte Plan für kommerzielle Verwendung einräumt. Pro Track wird die genutzte
> Modellversion im Prompt-Log festgehalten.

### 2.1 Modus und Felder

- **Custom-Modus verwenden** (getrennte Eingabe von Style und Lyrics). Den einfachen Beschreibungsmodus nicht
  für Produktions-Takes nutzen.
- **Style-Feld** (englisch, ≤ ca. 200 Zeichen): Genre, Instrumente, Tempo (BPM), Tonart/Modus, Stimmung,
  Raumcharakter, „instrumental“ bzw. Chor-Angabe. Wichtigste Begriffe zuerst – Suno gewichtet den Anfang
  stärker. Keine Künstlernamen (Verbot-Liste).
- **Lyrics-Feld:** Struktur-Tags und kurze Spielanweisungen in eckigen Klammern; bei Chor-Tracks Old-Aurethi-Text.
- **Instrumental-Tracks:** Je nach Suno-Version blendet der Instrumental-Schalter das Lyrics-Feld aus.
  Empfehlung:
  - **Weg A (Standard):** Instrumental-Schalter **an**, Struktur nur über das Style-Feld steuern
    (z. B. „slow intro, steady middle, soft ending“).
  - **Weg B (mehr Strukturkontrolle):** Instrumental-Schalter **aus**, im Lyrics-Feld **nur** Struktur-Tags
    und Klammeranweisungen ohne singbaren Text, im Style-Feld „instrumental, no vocals“. Falls Suno dennoch
    Gesang erzeugt: Take verwerfen oder Weg A nutzen.
  - Die in diesem Dokument angegebenen Lyrics-/Struktur-Felder sind für Weg B bzw. Chor-Tracks geschrieben.
- **Chor-Tracks:** Instrumental-Schalter aus; im Style-Feld „wordless choir“ bzw. „choir singing in an
  invented language“; Old-Aurethi-Text im Lyrics-Feld in Suno-Schreibung.
- **Ausschluss-Feld** (falls die Version ein Feld für auszuschließende Stile anbietet): standardmäßig
  „pop vocals, rap, edm, electric guitar, autotune, trap drums“ eintragen.

### 2.2 Struktur-Tags

Tags sind **Hinweise**, keine Garantie. Suno interpretiert sie modellabhängig. Bewährt:

| Tag | Wirkung / Verwendung |
|---|---|
| `[Intro]` | Einleitung; für spätere `_intro`-Datei |
| `[Verse]` | Hauptteil mit Thema; in Instrumentals „Thema A“ |
| `[Build]` | Steigerung, Verdichtung |
| `[Climax]` | Höhepunkt, volle Besetzung |
| `[Bridge]` | Kontrastteil, neue Harmonik |
| `[Breakdown]` | Ausdünnung, nur Rhythmus oder Pads |
| `[Interlude]` | Ruhiger Zwischenteil |
| `[Instrumental]` | Instrumentaler Abschnitt ohne Gesang |
| `[Chorus]` | Nur bei Chor-Tracks: Chor-Hauptteil |
| `[Outro]` | Ausklang |
| `[End]` | Signal zum Beenden (hilft bei Stings und kurzen Tracks) |
| `[Fade Out]` | Ausblenden (nicht für Loop-Takes verwenden) |

**Beschreibende Klammern:** `[Instrumental: solo cello, rising fifth motif]`, `[Soft]`, `[Choir enters]`,
`[Drums stop]`. Kurz halten (≤ 8 Wörter je Klammer).

### 2.3 Längensteuerung

- Suno bestimmt die Länge selbst; die maximale Länge pro Generierung hängt von der Version ab (prüfen).
- **Loop-Tracks:** Rohtake von **2:30–4:00** anstreben, damit ein stabiler Mittelteil von mindestens der
  Ziel-Looplänge entsteht. Viele Abschnitte mit `[Verse]`/`[Instrumental]` verlängern, `[End]` weglassen.
- **Stings:** sehr kurze Struktur (`[Intro]` → `[Climax]` → `[Outro]` → `[End]`). Liefert Suno zu lang, wird
  der Sting aus einem starken Schluss oder Höhepunkt eines längeren Takes geschnitten (oft die bessere Wahl).
- **Lineare Tracks (Story, Credits):** Gewünschte Dauer im Style-Feld nicht angeben (wirkt unzuverlässig);
  stattdessen Abschnittsanzahl im Lyrics-Feld steuern und mit Extend verlängern.

### 2.4 Extend, Cover und weitere Funktionen (sofern verfügbar)

| Funktion | Einsatz in dieser Produktion |
|---|---|
| **Extend** | Takt-genau ab einem stabilen Punkt fortsetzen, um den Loop-Körper zu verlängern oder einen Track um Teil 2 zu ergänzen (Credits, Story). Ab einem Taktanfang extendieren, Style unverändert lassen. |
| **Cover** | Melodische Verwandtschaft herstellen: Kampfvariante aus dem Fraktionsthema, Nachtversion aus dem Tag-Track, Sieg-Thema aus dem Menü. Gleiche Tonart und BPM im neuen Style angeben. |
| **Audio-Upload eigener Seeds** | Motiv-Seed der Aschenkrone als Ausgangsmaterial (nur eigenes Material). |
| **Persona / gespeicherter Stil** | Chor- und Solostimme über viele Tracks konsistent halten (Glutstimme, Aurethi-Chor). |
| **Abschnitt ersetzen / bearbeiten** | Einzelne fehlerhafte Takte (Gesangsartefakte, Stolperer) neu generieren. |
| **Stems** | Für saubere Loop-Schnitte, reduzierte Varianten (ohne Percussion/Chor) und Mix-Korrekturen. |
| **Remaster / Qualitätsverbesserung** | Nur wenn sich Tempo und Takt dabei nicht verschieben; danach Loop-Punkte neu prüfen. |

### 2.5 Generierungs-Workflow pro Track

1. Style-Prompt und Lyrics-/Struktur-Feld aus der Track-Bibel exakt übernehmen.
2. **Mindestens 6–10 Takes** erzeugen (mehrere Durchläufe).
3. Vorauswahl nach Kriterien: (a) Tempo stabil, (b) Tonart passt zum Set, (c) Kontur der Aschenkrone bzw.
   Fraktionskennung hörbar, (d) keine Gesangsartefakte, (e) keine Ähnlichkeit zu bekannter Musik,
   (f) mindestens ein langer, gleichförmiger Abschnitt für den Loop.
4. Besten Take ggf. per Extend verlängern oder per Cover variieren.
5. Download in höchster verfügbarer Qualität (WAV bevorzugt), sonst höchste MP3-Qualität.
6. Prompt-Log führen: Track-ID, Datum, Suno-Modellversion, Style-Prompt, Lyrics-Feld, Take-Kennung/Link,
   verwendete Funktion (Extend/Cover), Auswahlbegründung.

### 2.6 Strategie für Intro- und Loop-Versionen

- **Intro und Loop stammen immer aus demselben Take** (gleiche Tonart, gleiches Tempo, gleicher Klang).
- Die **Intro-Datei** enthält alles vom Anfang des Takes bis **exakt** zum Loop-Startpunkt. Sie endet
  ohne Fade, ohne Stille – das Engine spielt danach sample-genau die Loop-Datei.
- Die **Loop-Datei** beginnt am Loop-Startpunkt (Taktanfang) und endet am Loop-Endpunkt (Taktanfang des
  Takts nach dem letzten Loop-Takt). Länge = ganze Anzahl Takte (bevorzugt 16, 24, 32, 48 oder 64 Takte).
- Harmonisch muss der letzte Loop-Takt in den ersten Loop-Takt führen (z. B. Dominante oder ♭VII → i).
- Tracks mit `intro: None` (z. B. `build_tension`) erhalten **keine** Intro-Datei; ihr Loop muss so beginnen,
  dass er auch als Einstieg funktioniert (weicher erster Takt).

### 2.7 Nahtlose Loops in der DAW schneiden (Schritt für Schritt)

1. **Projekt anlegen:** Samplerate 48 kHz, 32-Bit-Float intern. Take importieren.
2. **Tempo ermitteln und fixieren:** Tempo des Takes erkennen, Grid auf die **Ziel-BPM aus der Track-Bibel**
   setzen. Abweichung bis ca. 3 % per hochwertigem Time-Stretch auf exakte Ziel-BPM korrigieren; driftet
   das Tempo innerhalb des Takes stärker, Take verwerfen oder Warp-Marker pro Takt setzen.
3. **Taktanfang 1 bestimmen:** ersten Schwerpunkt (Downbeat) auf Takt 1 legen. Alle Loop-Punkte liegen
   **ausschließlich auf Taktgrenzen**.
4. **Loop-Bereich wählen:** gleichförmigen Abschnitt mit ganzer Taktzahl suchen, dessen Anfangsakkord und
   Klangdichte zum Endbereich passen. Kein Beckenwirbel, kein Chor-Einsatz, kein Crescendo über die Naht.
5. **Ausklang einbacken (Tail-Wrap):** das Audio direkt hinter dem Loop-Ende (Hallfahne, ausklingende Noten,
   1–2 Takte) kopieren und am Loop-Anfang unter den Beginn mischen, damit der Übergang nicht „trocken“ klingt.
6. **Naht-Crossfade:** an Loop-Start und -Ende einen **Crossfade von 20–50 ms** (equal power) setzen bzw.
   Nulldurchgänge nutzen, damit kein Klick entsteht. Kein längerer Fade – das Tempo-Raster bleibt exakt.
7. **Intro schneiden:** von Take-Beginn (ggf. Anfangsstille entfernen, 5 ms Fade-in) bis exakt Loop-Start.
8. **Test:** Loop mindestens 5× hintereinander und Intro→Loop→Loop im DAW-Loop abhören; auf Klicks,
   Lautheitssprünge, Hall-Löcher und Phasenauslöschungen achten.
9. **Längenprüfung:** Loop-Länge in Samples = Takte × Schläge pro Takt × 60 / BPM × 48 000.
   Abweichung von mehr als 1 Sample korrigieren.

### 2.8 Export und Formate

| Stufe | Format |
|---|---|
| Arbeitsmaster (Archiv, nicht im Repo) | WAV, 48 kHz, 24 Bit, Stereo, je Datei |
| Spieldatei (Repo) | **OGG Vorbis**, 48 kHz, Stereo, Qualität q6 (ca. 192 kbit/s) |
| Alternativ vom Scanner erkannt | `.wav`, `.flac`, `.mp3` – nur in Ausnahmefällen, `.ogg` bevorzugt |

- Keine Stille am Anfang oder Ende von Loop-Dateien; keine eingebetteten Metadaten-Pausen.
- Gapless-Wiedergabe der OGG-Dateien im Engine testen (Encoder-Vorlauf kann je nach Decoder Lücken erzeugen;
  bei Problemen ist `.wav` für betroffene Loops zulässig).
- Liefert Suno 44,1 kHz, wird vor dem Schnitt hochwertig auf 48 kHz konvertiert. Einheitliche Samplerate
  für alle Musikdateien – falls das Engine eine andere Rate erwartet, gilt die Engine-Rate.

### 2.9 Loudness

- **Zielwert: −14 LUFS integriert, True Peak ≤ −1 dBTP** für jede Datei.
- Messung bei Loops über die gesamte Loop-Datei; bei Intros über die Intro-Datei; Intro und Loop dürfen
  um höchstens 1 LU voneinander abweichen, damit der Übergang nicht springt.
- **Stings** (unter 15 s) sind für integrierte Messung zu kurz: Short-Term-Maximum ca. −12 LUFS, True Peak
  ≤ −1 dBTP, wahrgenommene Lautheit gegen `battle_large_loop` abgleichen.
- Dynamikunterschiede zwischen Intensitätsstufen entstehen **im Arrangement und im Engine-Mix**, nicht durch
  unterschiedliche Mastering-Pegel. Empfohlene Engine-Offsets: Menu 0 dB, Calm −3 dB, Tension −2 dB,
  Combat −1 dB, Battle 0 dB, Boss 0 dB, Sting 0 dB.
- Limiter sanft (Gain Reduction im Mittel ≤ 2 dB). Suno-Takes sind oft bereits laut – eher absenken als
  komprimieren.

### 2.10 Dateibenennung und Ablage

Konvention laut Kanon: `assets/audio/music/<track_id>[_intro|_loop].ogg` – klein, `snake_case`.

| Track-Art | Dateien |
|---|---|
| Loop mit Intro | `<track_id>_intro.ogg` und `<track_id>_loop.ogg` |
| Loop ohne Intro | nur `<track_id>_loop.ogg` |
| Linear (Sting, Story, Credits) | nur `<track_id>.ogg` |

**Pflichtdateien laut Manifest (Vertical Slice, exakt 13 Dateien):**

```text
assets/audio/music/main_menu_intro.ogg
assets/audio/music/main_menu_loop.ogg
assets/audio/music/build_calm_intro.ogg
assets/audio/music/build_calm_loop.ogg
assets/audio/music/build_tension_loop.ogg
assets/audio/music/battle_small_intro.ogg
assets/audio/music/battle_small_loop.ogg
assets/audio/music/battle_large_intro.ogg
assets/audio/music/battle_large_loop.ogg
assets/audio/music/boss_pale_reeve_intro.ogg
assets/audio/music/boss_pale_reeve_loop.ogg
assets/audio/music/victory_sting.ogg
assets/audio/music/defeat_sting.ogg
```

Hinweis: `build_tension` hat laut Manifest **kein** Intro – eine Datei `build_tension_intro.ogg` darf nicht
abgelegt werden. Zukünftige Track-IDs dürfen nicht auf `_intro` oder `_loop` enden.

### 2.11 Rechte und Dokumentation

- Vor Produktionsbeginn prüfen und dokumentieren, welche Nutzungsrechte der gebuchte Suno-Plan für
  kommerzielle Spiele einräumt (Stand zum Generierungszeitpunkt).
- Prompt-Log (siehe 2.5) und Arbeitsmaster archivieren; Zuordnung Datei ↔ Take muss jederzeit nachvollziehbar sein.

---

## 3. Dynamisches Musiksystem

### 3.1 Intensitätsstufen (Manifest-Feld `intensity`)

Priorität (höher schlägt niedriger): **Sting > Boss > Battle > Combat > Tension > Calm > Menu**

| Stufe | Bedeutung | Empfohlene Auslöser (Vorschlag für die Integration) | Manifest-Track (Vertical Slice) |
|---|---|---|---|
| **Menu** | Menüs, Ergebnisbildschirme, Credits | Hauptmenü, Pause-Menü (Musik läuft weiter, −6 dB), Sieg-/Niederlagebildschirm | `main_menu` |
| **Calm** | Aufbau, Erkundung, Weltkarte ohne Bedrohung | Keine feindlichen Einheiten im Sichtbereich, keine Warnung aktiv | `build_calm` |
| **Tension** | Bedrohung angekündigt | Wellenwarnung, Feind in Sicht, Nacht in M01, Spionage-Ereignis | `build_tension` |
| **Combat** | Kleines Gefecht | Erste Kampfhandlung; bis ca. 15 Einheiten beteiligt | `battle_small` |
| **Battle** | Große Schlacht | Mehr als ca. 15 Einheiten im Kampf, Angriff auf Bergfried, Held im Kampf mit Elite | `battle_large` |
| **Boss** | Bosskampf | Boss betritt Kampfradius oder Skript-Event | `boss_pale_reeve` |
| **Sting** | Kurzes, nicht-loopendes Ereignis | Missionssieg, Missionsniederlage, Zeitalterwechsel | `victory_sting`, `defeat_sting` |

### 3.2 Musik-Sets (Tonart- und BPM-Familien)

Innerhalb eines Sets haben alle Tracks **dieselbe Tonika** und **dieselbe BPM oder das Verhältnis 1:2**.
Dadurch sind taktsynchrone Crossfades harmonisch und rhythmisch sauber.

| Set | Tonika | BPM-Familie | Takt | Tracks |
|---|---|---|---|---|
| **A – Aschmark / Valemark (Mission)** | D | 72 / 144 | 4/4 | main_menu, build_calm, build_night, economy_prosperity, explore_wilds, build_tension, war_preparation, battle_small, battle_large, boss_pale_reeve, faction_ember_crown_theme, faction_ember_crown_battle, victory_sting, defeat_sting, victory_theme, defeat_theme, mystery_ruins, credits_theme |
| **B – Weltkarte & Hof** | G | 80 | 4/4 | worldmap_spring, worldmap_summer, worldmap_autumn, worldmap_winter, worldmap_fallen_gods, diplomacy_court, intrigue_shadows, age_transition_sting |
| **S – Belagerung** | C | 120 | 4/4 | siege_assault, siege_defense |
| **E – Endgame / Solmere** | D | 66 / 132 | 4/4 | endgame_last_crown, endgame_final_battle, boss_hollow_king |
| **F2 – Skarnheim** | E | 66 / 132 | 4/4 | faction_skarn_clans_theme, faction_skarn_clans_battle |
| **F3 – Dornwald** | A | ♩. = 72 | 6/8 | faction_thorn_covenant_theme, faction_thorn_covenant_battle |
| **F4 – Saelith** | B♭ | 104 | 4/4 | faction_saelith_compact_theme, faction_saelith_compact_battle |
| **F5 – Die Hohlen** | G♯ | 60 / 120 | 4/4 | faction_hollowed_theme, faction_hollowed_battle, mystery_veilscar, story_revelation |
| **F6 – Aschenlegion** | C | 88 | 4/4 | faction_ashen_legion_theme, faction_ashen_legion_battle |
| **X – Einzelstücke** | je Track | je Track | je Track | boss_ashen_dragon, boss_root_mother, mystery_old_gods, story_lament, story_ashfall_memory |

Die Glutkrone (F1) ist bewusst Teil von Set A, weil sie die Spielerfraktion der Kampagne ist.
Bosse und Einzelstücke werden immer mit Intro oder Set-Wechsel-Regel betreten (siehe 3.3).

### 3.3 Übergangsregeln

| Nr. | Situation | Regel |
|---|---|---|
| R1 | Gleiches Set, benachbarte Stufe (Calm ↔ Tension, Combat ↔ Battle) | Taktsynchroner Equal-Power-Crossfade, Start am nächsten Taktanfang, Dauer **2 Takte** (Set A: 6,7 s bei 72 BPM bzw. 3,3 s bei 144 BPM). Ziel-Loop startet an derselben Taktposition innerhalb seines Loops (Positions-Sync) oder an Loop-Takt 1. |
| R2 | Aufwärts in den Kampf (Calm/Tension → Combat/Battle) | Am nächsten Taktanfang (max. 1 Takt Wartezeit) laufende Musik über 1 Takt ausblenden; Ziel-**Intro** startet hart am Taktanfang, danach Loop. Ohne Intro: Loop ab Takt 1 mit 1 Takt Fade-in. Gilt auch setübergreifend. |
| R3 | Abwärts aus dem Kampf (Combat/Battle → Tension/Calm) | Erst nach **15 s ohne Kampfkontakt** (Hysterese). 4 Takte Fade-out, Ziel-Loop mit 2 Takten Fade-in. Calm-Tracks setzen an der zuletzt gespielten Position fort (Resume), um Wiederholung zu vermeiden. |
| R4 | Beliebig → Boss | Am nächsten **Schlag** laufende Musik in 250–500 ms ausblenden, Boss-Intro startet hart. Boss-Musik bleibt bis zum Tod des Bosses. Danach 2 Takte Fade-out, 2 s Ambience, dann Battle- oder Calm-Loop nach R3. |
| R5 | Sting | Laufende Musik über 500 ms ausblenden (bei gleichzeitigem Erzähler: auf −18 dB ducken), Sting spielt einmal, danach 1,5–3 s nur Ambience, dann Folgetrack (z. B. `victory_theme`, `defeat_theme` oder `main_menu`). Stings unterbrechen nie andere Stings. |
| R6 | Set-Wechsel (andere Tonika oder BPM) bei gleicher oder fallender Intensität | Kein überlappender Crossfade. 2 Takte Fade-out → mindestens 1 s Ambience → Ziel-Intro bzw. Loop mit 2 Takten Fade-in. Ausnahme: tempolose Tracks (`mystery_*`) dürfen mit 4 s Crossfade überblendet werden. |
| R7 | Menü → Spiel | `main_menu` 2 s Fade-out zum Ladebildschirm; Spielstart mit `build_calm_intro` (bzw. Intro des Kontext-Calm-Tracks). |
| R8 | Dialog, Erzähler | Musik-Ducking −6 dB, Attack 300 ms, Release 800 ms. Bei Skript-Zwischensequenzen mit Story-Track: Story-Track ersetzt den laufenden Loop nach R6. |

**Mindestverweildauer (gegen „Flattern“):** Calm 30 s, Tension 20 s, Combat 20 s, Battle 30 s.
Ein Wechsel aufwärts darf die Mindestverweildauer immer unterbrechen, ein Wechsel abwärts nie.

**Kontext-Auswahl (welcher Track pro Stufe):**

| Kontext | Menu | Calm | Tension | Combat | Battle | Boss |
|---|---|---|---|---|---|---|
| Mission Aschmark/Valemark, Tag | main_menu | build_calm | build_tension | battle_small | battle_large | boss_pale_reeve (M01) |
| Mission, Nacht | – | build_night | build_tension | battle_small | battle_large | je Mission |
| Mission, Wirtschaftsphase (hohe Bevölkerung, Markt) | – | economy_prosperity | war_preparation | battle_small | battle_large | – |
| Mission, Kartenrand/Nebel erkunden | – | explore_wilds | build_tension | battle_small | battle_large | – |
| Mission mit Gegner Hohle | – | Kontext-Calm | faction_hollowed_theme | faction_hollowed_battle | battle_large | Kontext-Boss |
| Mission gegen Skarnheim, Dornenbund, Saelith oder Aschenlegion | – | Kontext-Calm | Kontext-Tension (bei der Aschenlegion: faction_ashen_legion_theme) | Kampfvariante der Gegnerfraktion (z. B. faction_skarn_clans_battle) | battle_large | Kontext-Boss |
| Gefecht (Skirmish) | main_menu | Thema der eigenen Fraktion (z. B. faction_thorn_covenant_theme) | war_preparation | Kampfvariante der Gegnerfraktion (z. B. faction_saelith_compact_battle) | battle_large | – |
| Weltkarte | – | worldmap_<Jahreszeit> bzw. worldmap_fallen_gods (Zeitalter VI) | intrigue_shadows | – | – | – |
| Diplomatie-Bildschirm | – | diplomacy_court | intrigue_shadows | – | – | – |
| Belagerung (Skript) | – | Kontext-Calm | war_preparation | – | siege_assault / siege_defense | – |
| Zeitalter VII | – | endgame_last_crown | endgame_last_crown | battle_large | endgame_final_battle | boss_hollow_king |
| Geheimnisvoller Ort | – | mystery_ruins / mystery_veilscar / mystery_old_gods | – | Kontext-Combat | – | – |

### 3.4 Vorgeschlagene Manifest-Einträge für zukünftige Tracks

Die folgenden Einträge sind **nicht** in `manifest.ron` eingetragen; sie dienen der Audio-Integration als
Vorlage, sobald die Dateien existieren. Linear gespielte Story-Tracks sind als `Calm` mit `looped: false`
und Skript-Auslösung vorgesehen.

```ron
MusicTrack(id: "build_night", intensity: Calm, file: "music/build_night_loop", intro: Some("music/build_night_intro"), looped: true),
MusicTrack(id: "economy_prosperity", intensity: Calm, file: "music/economy_prosperity_loop", intro: Some("music/economy_prosperity_intro"), looped: true),
MusicTrack(id: "explore_wilds", intensity: Calm, file: "music/explore_wilds_loop", intro: Some("music/explore_wilds_intro"), looped: true),
MusicTrack(id: "war_preparation", intensity: Tension, file: "music/war_preparation_loop", intro: None, looped: true),
MusicTrack(id: "worldmap_spring", intensity: Calm, file: "music/worldmap_spring_loop", intro: Some("music/worldmap_spring_intro"), looped: true),
MusicTrack(id: "worldmap_summer", intensity: Calm, file: "music/worldmap_summer_loop", intro: Some("music/worldmap_summer_intro"), looped: true),
MusicTrack(id: "worldmap_autumn", intensity: Calm, file: "music/worldmap_autumn_loop", intro: Some("music/worldmap_autumn_intro"), looped: true),
MusicTrack(id: "worldmap_winter", intensity: Calm, file: "music/worldmap_winter_loop", intro: Some("music/worldmap_winter_intro"), looped: true),
MusicTrack(id: "worldmap_fallen_gods", intensity: Calm, file: "music/worldmap_fallen_gods_loop", intro: Some("music/worldmap_fallen_gods_intro"), looped: true),
MusicTrack(id: "diplomacy_court", intensity: Calm, file: "music/diplomacy_court_loop", intro: None, looped: true),
MusicTrack(id: "intrigue_shadows", intensity: Tension, file: "music/intrigue_shadows_loop", intro: None, looped: true),
MusicTrack(id: "siege_assault", intensity: Battle, file: "music/siege_assault_loop", intro: Some("music/siege_assault_intro"), looped: true),
MusicTrack(id: "siege_defense", intensity: Battle, file: "music/siege_defense_loop", intro: Some("music/siege_defense_intro"), looped: true),
MusicTrack(id: "boss_hollow_king", intensity: Boss, file: "music/boss_hollow_king_loop", intro: Some("music/boss_hollow_king_intro"), looped: true),
MusicTrack(id: "boss_ashen_dragon", intensity: Boss, file: "music/boss_ashen_dragon_loop", intro: Some("music/boss_ashen_dragon_intro"), looped: true),
MusicTrack(id: "boss_root_mother", intensity: Boss, file: "music/boss_root_mother_loop", intro: Some("music/boss_root_mother_intro"), looped: true),
MusicTrack(id: "faction_ember_crown_theme", intensity: Calm, file: "music/faction_ember_crown_theme_loop", intro: Some("music/faction_ember_crown_theme_intro"), looped: true),
MusicTrack(id: "faction_ember_crown_battle", intensity: Combat, file: "music/faction_ember_crown_battle_loop", intro: Some("music/faction_ember_crown_battle_intro"), looped: true),
MusicTrack(id: "faction_skarn_clans_theme", intensity: Calm, file: "music/faction_skarn_clans_theme_loop", intro: Some("music/faction_skarn_clans_theme_intro"), looped: true),
MusicTrack(id: "faction_skarn_clans_battle", intensity: Combat, file: "music/faction_skarn_clans_battle_loop", intro: Some("music/faction_skarn_clans_battle_intro"), looped: true),
MusicTrack(id: "faction_thorn_covenant_theme", intensity: Calm, file: "music/faction_thorn_covenant_theme_loop", intro: Some("music/faction_thorn_covenant_theme_intro"), looped: true),
MusicTrack(id: "faction_thorn_covenant_battle", intensity: Combat, file: "music/faction_thorn_covenant_battle_loop", intro: Some("music/faction_thorn_covenant_battle_intro"), looped: true),
MusicTrack(id: "faction_saelith_compact_theme", intensity: Calm, file: "music/faction_saelith_compact_theme_loop", intro: Some("music/faction_saelith_compact_theme_intro"), looped: true),
MusicTrack(id: "faction_saelith_compact_battle", intensity: Combat, file: "music/faction_saelith_compact_battle_loop", intro: Some("music/faction_saelith_compact_battle_intro"), looped: true),
MusicTrack(id: "faction_hollowed_theme", intensity: Tension, file: "music/faction_hollowed_theme_loop", intro: Some("music/faction_hollowed_theme_intro"), looped: true),
MusicTrack(id: "faction_hollowed_battle", intensity: Combat, file: "music/faction_hollowed_battle_loop", intro: Some("music/faction_hollowed_battle_intro"), looped: true),
MusicTrack(id: "faction_ashen_legion_theme", intensity: Tension, file: "music/faction_ashen_legion_theme_loop", intro: Some("music/faction_ashen_legion_theme_intro"), looped: true),
MusicTrack(id: "faction_ashen_legion_battle", intensity: Combat, file: "music/faction_ashen_legion_battle_loop", intro: Some("music/faction_ashen_legion_battle_intro"), looped: true),
MusicTrack(id: "victory_theme", intensity: Menu, file: "music/victory_theme_loop", intro: Some("music/victory_theme_intro"), looped: true),
MusicTrack(id: "defeat_theme", intensity: Menu, file: "music/defeat_theme_loop", intro: Some("music/defeat_theme_intro"), looped: true),
MusicTrack(id: "age_transition_sting", intensity: Sting, file: "music/age_transition_sting", intro: None, looped: false),
MusicTrack(id: "story_lament", intensity: Calm, file: "music/story_lament", intro: None, looped: false),
MusicTrack(id: "story_ashfall_memory", intensity: Calm, file: "music/story_ashfall_memory", intro: None, looped: false),
MusicTrack(id: "story_revelation", intensity: Tension, file: "music/story_revelation", intro: None, looped: false),
MusicTrack(id: "mystery_ruins", intensity: Calm, file: "music/mystery_ruins_loop", intro: None, looped: true),
MusicTrack(id: "mystery_veilscar", intensity: Tension, file: "music/mystery_veilscar_loop", intro: None, looped: true),
MusicTrack(id: "mystery_old_gods", intensity: Calm, file: "music/mystery_old_gods_loop", intro: None, looped: true),
MusicTrack(id: "endgame_last_crown", intensity: Tension, file: "music/endgame_last_crown_loop", intro: Some("music/endgame_last_crown_intro"), looped: true),
MusicTrack(id: "endgame_final_battle", intensity: Battle, file: "music/endgame_final_battle_loop", intro: Some("music/endgame_final_battle_intro"), looped: true),
MusicTrack(id: "credits_theme", intensity: Menu, file: "music/credits_theme", intro: None, looped: false),
```

---
## 4. Track-Bibel

### 4.0 Übersicht aller Tracks

Legende Dateien: **I+L** = `<id>_intro.ogg` + `<id>_loop.ogg` · **L** = nur `<id>_loop.ogg` ·
**Lin** = nur `<id>.ogg` (linear). Priorität siehe Kapitel 6.

| # | Track-ID | Bereich | Intensität | Set | Tonart | BPM | Dateien | Prio |
|---|---|---|---|---|---|---|---|---|
| 1 | `main_menu` | Hauptmenü | Menu | A | D-äolisch | 72 | I+L | P0 |
| 2 | `build_calm` | Städtebau Tag | Calm | A | D-dorisch | 72 | I+L | P0 |
| 3 | `build_tension` | Bedrohung | Tension | A | D-äolisch | 72 | L | P0 |
| 4 | `battle_small` | Kleine Schlacht | Combat | A | D-äolisch | 144 | I+L | P0 |
| 5 | `battle_large` | Große Schlacht | Battle | A | D-äolisch | 144 | I+L | P0 |
| 6 | `boss_pale_reeve` | Boss Fahler Vogt | Boss | A | D-phrygisch | 144 | I+L | P0 |
| 7 | `victory_sting` | Sieg | Sting | A | D-Dur | 72 | Lin | P0 |
| 8 | `defeat_sting` | Niederlage | Sting | A | D-Moll/Hohl | 72 | Lin | P0 |
| 9 | `worldmap_spring` | Weltkarte Frühling | Calm | B | G-dorisch | 80 | I+L | P1 |
| 10 | `worldmap_summer` | Weltkarte Sommer | Calm | B | G-mixolydisch | 80 | I+L | P2 |
| 11 | `worldmap_autumn` | Weltkarte Herbst | Calm | B | G-äolisch | 80 | I+L | P1 |
| 12 | `worldmap_winter` | Weltkarte Winter | Calm | B | G-äolisch | 80 | I+L | P2 |
| 13 | `worldmap_fallen_gods` | Weltkarte Zeitalter VI | Calm | B | G-phrygisch | 80 | I+L | P3 |
| 14 | `build_night` | Städtebau Nacht | Calm | A | D-äolisch | 72 | I+L | P1 |
| 15 | `economy_prosperity` | Wirtschaft | Calm | A | D-mixolydisch | 72 | I+L | P2 |
| 16 | `explore_wilds` | Erkundung | Calm | A | D-dorisch | 72 | I+L | P1 |
| 17 | `diplomacy_court` | Diplomatie | Calm | B | G-dorisch | 80 | L | P2 |
| 18 | `intrigue_shadows` | Politische Intrigen | Tension | B | G-phrygisch | 80 | L | P2 |
| 19 | `war_preparation` | Kriegsvorbereitung | Tension | A | D-äolisch | 72 | L | P1 |
| 20 | `siege_assault` | Belagerung Angriff | Battle | S | C-äolisch | 120 | I+L | P2 |
| 21 | `siege_defense` | Belagerung Verteidigung | Battle | S | C-phrygisch | 120 | I+L | P2 |
| 22 | `boss_hollow_king` | Boss Hohler König | Boss | E | D-phrygisch/Hohl | 132 | I+L | P3 |
| 23 | `boss_ashen_dragon` | Boss Aschdrache Vyrmathax | Boss | X | F-phrygisch | 152 | I+L | P3 |
| 24 | `boss_root_mother` | Boss Wurzelmutter | Boss | X | A-äolisch | ♩. = 84 (12/8) | I+L | P2 |
| 25 | `faction_ember_crown_theme` | Glutkrone Thema | Calm | A | D-mixolydisch | 72 | I+L | P1 |
| 26 | `faction_ember_crown_battle` | Glutkrone Kampf | Combat | A | D-dorisch | 144 | I+L | P1 |
| 27 | `faction_skarn_clans_theme` | Skarnheim Thema | Calm | F2 | E-äolisch | 66 | I+L | P2 |
| 28 | `faction_skarn_clans_battle` | Skarnheim Kampf | Combat | F2 | E-äolisch | 132 | I+L | P2 |
| 29 | `faction_thorn_covenant_theme` | Dornenbund Thema | Calm | F3 | A-dorisch | ♩. = 72 (6/8) | I+L | P2 |
| 30 | `faction_thorn_covenant_battle` | Dornenbund Kampf | Combat | F3 | A-äolisch | ♩. = 72 (6/8) | I+L | P2 |
| 31 | `faction_saelith_compact_theme` | Saelith Thema | Calm | F4 | B♭-mixolydisch | 104 | I+L | P2 |
| 32 | `faction_saelith_compact_battle` | Saelith Kampf | Combat | F4 | B♭-dorisch | 104 | I+L | P2 |
| 33 | `faction_hollowed_theme` | Die Hohlen Thema | Tension | F5 | G♯-lokrisch | 60 | I+L | P1 |
| 34 | `faction_hollowed_battle` | Die Hohlen Kampf | Combat | F5 | G♯-phrygisch | 120 | I+L | P1 |
| 35 | `faction_ashen_legion_theme` | Aschenlegion Thema | Tension | F6 | C-phrygisch | 88 | I+L | P2 |
| 36 | `faction_ashen_legion_battle` | Aschenlegion Kampf | Combat | F6 | C-phrygisch | 88 | I+L | P2 |
| 37 | `victory_theme` | Sieg-Bildschirm | Menu | A | D-Dur/mixolydisch | 72 | I+L | P1 |
| 38 | `defeat_theme` | Niederlage-Bildschirm | Menu | A | D-äolisch | 72 | I+L | P1 |
| 39 | `age_transition_sting` | Zeitalterwechsel | Sting | B | G-dorisch | 80 | Lin | P1 |
| 40 | `story_lament` | Trauer, Verlust | Calm | X | D-äolisch | 60 rubato | Lin | P1 |
| 41 | `story_ashfall_memory` | Rückblende Aschenfall | Calm | X | D-Dur → D-phrygisch | 66 rubato | Lin | P2 |
| 42 | `story_revelation` | Enthüllung, Stimme aus dem Tief | Tension | F5 | G♯-lokrisch | 60 | Lin | P3 |
| 43 | `mystery_ruins` | Ruinen Aureths | Calm | A | D-äolisch | 60 (tempolos) | L | P1 |
| 44 | `mystery_veilscar` | Schleiernarbe | Tension | F5 | G♯ (Cluster) | 60 (tempolos) | L | P3 |
| 45 | `mystery_old_gods` | Schreine der Alten Götter | Calm | X | E-lydisch | 60 (tempolos) | L | P2 |
| 46 | `endgame_last_crown` | Zeitalter VII, Weltkarte/Aufbau | Tension | E | D-äolisch → dorisch | 66 | I+L | P3 |
| 47 | `endgame_final_battle` | Letzte Schlacht vor Solmere | Battle | E | D-phrygisch → Dur | 132 | I+L | P3 |
| 48 | `credits_theme` | Abspann | Menu | A | D-dorisch → D-Dur | 72 | Lin | P3 |

**Taktdauern zur Orientierung (4/4):** 60 BPM = 4,000 s · 66 = 3,636 s · 72 = 3,333 s · 80 = 3,000 s ·
88 = 2,727 s · 104 = 2,308 s · 120 = 2,000 s · 132 = 1,818 s · 144 = 1,667 s · 152 = 1,579 s.
6/8 bei ♩. = 72: 1,667 s pro Takt. 12/8 bei ♩. = 84: 2,857 s pro Takt.

**Arrangement-Taktangaben** beziehen sich auf den geschnittenen Take nach Tempo-Fixierung. Der Suno-Take
trifft diese Taktzahlen nicht exakt; maßgeblich sind die Abschnittsfunktionen und die Loop-Längen.

---

### 4.1 Manifest-Tracks (Vertical Slice)

#### 4.1.1 `main_menu` – „Krone aus Asche“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `main_menu` → `assets/audio/music/main_menu_intro.ogg`, `assets/audio/music/main_menu_loop.ogg` |
| Manifest | intensity: Menu · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Hauptmenü und Pause-Menü (−6 dB). Erster Eindruck des Spiels; etabliert das Leitmotiv in der Asche-Form und deutet am Loop-Ende die Glut-Form an. |
| Genre | Dark-Fantasy-Orchesterelegie, kinematisch |
| Instrumentierung | Solo-Cello, tiefe Streicher, Harfe, Altflöte, wortlose Frauenstimme, gemischter Chor (pp, Old Aurethi), ferne Kirchenglocke, gedämpfte Pauke (Kronenschlag als Herzschlag), leises Ascheregen-Rauschen, Hörner (nur T. 25–32) |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Leitmotiv Asche-Form (Cello), Glut-Form (Hörner, T. 25–32) |
| Stimmung | Melancholisch, würdevoll, wehmütig, ein Glimmen unter der Asche |
| Dynamikverlauf | pp (Intro) → mp (T. 9) → mf (T. 25, Chor + Hörner) → p (T. 33–40), damit der Loop leise zurückkehrt |
| Soundcharakter / Mix | Breites Stereobild, Steinhallen-Hall ca. 2,8 s; Cello vorne mittig, Chor weit hinten, Glocke sehr fern links. Kein Subbass, keine harten Transienten. |
| Loop-Eignung | Hoch. Loop T. 9–40 (32 Takte). Naht: T. 40 endet auf a-Moll (v) und führt zurück auf d-Moll in T. 9. Glocke nicht in T. 40 oder T. 9 platzieren. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 26,7 s · Loop 32 Takte = 106,7 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Wind und Ascherauschen, Glockenschlag in T. 1, Harfen-Arpeggien auf d-Moll; ab T. 5 spielt das Solo-Cello das Fragment „Die Krone“ (1 – 5 – ♭6 – 5).
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

---

#### 4.1.2 `build_calm` – „Brennhollow erwacht“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `build_calm` → `assets/audio/music/build_calm_intro.ogg`, `assets/audio/music/build_calm_loop.ogg` |
| Manifest | intensity: Calm · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Friedlicher Städtebau am Tag in der Aschmark (Brennhollow, M01 „Die letzte Siedlung“, Zeitalter I). Längste Hörzeit im Spiel – muss unaufdringlich und langlebig sein. |
| Genre | Pastorales Folk-Orchester mit dunklem Unterton, Ambient-Anteil |
| Instrumentierung | Gezupfte Laute (Ostinato), Holzflöte, Fidel, Bratschen, Harfe, Klarinette, leise Rahmentrommel, sehr seltene ferne Glocke |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Glut-Form nur als Fragment (Flöte), vollständig erst T. 29–36 |
| Stimmung | Hoffnungsvoll, arbeitsam, zart; leise Wehmut über das Verlorene |
| Dynamikverlauf | p bis mp, bewusst flach (max. ca. 6 dB Schwankung), keine Crescendi, keine Akzentschläge |
| Soundcharakter / Mix | Warm, nah, kleiner Holzraum (Hall 1,4 s); Laute leicht links, Fidel rechts, Flöte mittig. Tiefmitten schlank halten, damit Arbeits-SFX (Holzhacken, Steinbruch) Platz haben. |
| Loop-Eignung | Sehr hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C-Dur (♭VII) → d-Moll in T. 5. Laute-Ostinato läuft über die Naht durch. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 13,3 s · Loop 48 Takte = 160,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Laute-Ostinato allein, d-Moll mit dorischer Sexte (H).
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

---

#### 4.1.3 `build_tension` – „Rauch am Horizont“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `build_tension` → `assets/audio/music/build_tension_loop.ogg` (**kein Intro** laut Manifest) |
| Manifest | intensity: Tension · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Bedrohung angekündigt: Wellenwarnung (Bruder Hal Venn), Feind in Sichtweite, Einbruch der Nacht in M01. Wird per R1 aus `build_calm` eingeblendet. |
| Genre | Dunkle Orchester-Suspense |
| Instrumentierung | Celli-Achtelostinato, Streicher-Tremolo, gestopfte Hörner, Bassklarinette, Pauke im Kronenschlag, Rahmentrommeln, dezenter Hohlatem-Cluster (geflüsterter Chor) |
| Tempo / Takt | 72 BPM, 4/4 (Achtel- und Sechzehntelpuls) |
| Tonart / Motivform | D-äolisch, phrygische Färbung (E♭) in T. 17–24; Fragment „Die Krone“ in gestopften Hörnern, einmal Hohl-Andeutung (A♭) in T. 23 |
| Stimmung | Lauernd, nervös, kalt; das Dorf hält den Atem an |
| Dynamikverlauf | mp (T. 1) → mf (T. 17–24) → mp (T. 25–32); Anfang und Ende auf gleicher Dichte für die Naht |
| Soundcharakter / Mix | Dunkel, tiefmittenbetont, Hall mittel (2,0 s); Tremolo breit, Ostinato mittig, Flüstern weit hinten und gefiltert |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte). T. 1 muss als weicher Einstieg funktionieren: kein Akzent, nur Ostinato und Tremolo. Naht: T. 32 auf A (V) → d in T. 1. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Loop 32 Takte = 106,7 s |

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

---

#### 4.1.4 `battle_small` – „Klingen im Nebel“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `battle_small` → `assets/audio/music/battle_small_intro.ogg`, `assets/audio/music/battle_small_loop.ogg` |
| Manifest | intensity: Combat · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Kleine Gefechte: Späher, erste Hohlknechte, Scharmützel bis ca. 15 Einheiten. Eingang per R2, Übergang zu `battle_large` per R1. |
| Genre | Dark-Fantasy-Kampforchester |
| Instrumentierung | Streicher-Spiccato-Ostinato, Kriegstrommeln im Kronenschlag, Hörner, Posaunen, Rührtrommel, Becken sparsam; kein Chor (reserviert für `battle_large`) |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Glutruf im Intro, Glut-Form augmentiert in Posaunen (T. 21–36) |
| Stimmung | Dringlich, rau, entschlossen |
| Dynamikverlauf | f (Intro) → mf (T. 5–20) → f (T. 21–36) → mp (Breakdown T. 37–44) → mf (T. 45–52) |
| Soundcharakter / Mix | Trocken-kraftvoll, Hall kurz (1,6 s); Trommeln breit, Ostinato eng; Platz bei 2–4 kHz für Schwert-SFX lassen (Blech nicht zu hell) |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C (♭VII) → d in T. 5; Ostinato durchgehend. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Intro 4 Takte = 6,7 s · Loop 48 Takte = 80,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Paukenwirbel, Glutruf der Hörner (1 – 5 – 8), Schlag auf T. 4 Zählzeit 4 als Auftakt.
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

---

#### 4.1.5 `battle_large` – „Die Aschmark brennt“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `battle_large` → `assets/audio/music/battle_large_intro.ogg`, `assets/audio/music/battle_large_loop.ogg` |
| Manifest | intensity: Battle · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Große Schlachten: Wellenangriffe auf Brennhollow, Sturm auf den Hohlschrein, Isolde im Nahkampf, mehr als ca. 15 Einheiten. Aus `battle_small` per R1. |
| Genre | Episches Dark-Fantasy-Orchester mit Chor |
| Instrumentierung | Volles Orchester, gemischter Chor (Old Aurethi, Staccato-Rufe und lange Linien), Taiko-artige Großtrommeln, Ambosse, tiefes Blech, Glocken, dezente Subbass-Impulse |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Glut-Form im Chor (T. 25–40), Hohl-Form in Posaunen (Bridge T. 41–48), Glut-Form tutti (T. 49–64) |
| Stimmung | Überwältigend, verzweifelt-heroisch, brennend |
| Dynamikverlauf | ff-Schlag (Intro-Ende) → f (T. 9–24) → f (T. 25–40) → mf, dunkel (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Groß, druckvoll, Hall 2,4 s; Chor breit, Trommeln tief zentriert, Subbass nur auf Zählzeit 1 einzelner Takte; Limiter-Reduktion moderat halten |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf A (V) → d in T. 9; Chor-Phrasen dürfen nicht über die Naht klingen. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 13,3 s · Loop 64 Takte = 106,7 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Chor-Ruf „Kael rhana!“, Großtrommeln im Crescendo, Blechcluster in T. 8.
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

---

#### 4.1.6 `boss_pale_reeve` – „Der Fahle Vogt“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `boss_pale_reeve` → `assets/audio/music/boss_pale_reeve_intro.ogg`, `assets/audio/music/boss_pale_reeve_loop.ogg` |
| Manifest | intensity: Boss · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Mini-Boss der ersten Mission: der Fahle Vogt, Hüter des Hohlschreins von Vael Dorn. Erster Kontakt des Spielers mit der Hohl-Form des Leitmotivs. Eingang per R4. |
| Genre | Dark-Fantasy-Bosskampf, Horror-Orchester |
| Instrumentierung | Tiefe Posaunen und Tuba, Kontrabässe col legno, Glasharmonika-Klang, Flüsterchor, tiefer Männerchor (Old Aurethi), Großtrommeln, Ketten als Perkussion, Streicher sul ponticello, tiefer Orgelpedalton D, Subbass |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch (E♭); Hohl-Form des Leitmotivs (1 – ♭5 – 5 – ♭5), Streicherglissandi aufwärts als Klang des „Seelenzugs“ |
| Stimmung | Bedrohlich, unheilig, kalt, unausweichlich |
| Dynamikverlauf | pp, Stille (T. 1–4) → ff-Schlag (T. 8) → f (T. 9–40) → p, unheimlich (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Dunkel, eng in den Höhen, langer unnatürlicher Hall (3,5 s) auf Chor und Glasklang, Trommeln trocken; Subbass-Pedal konstant, aber unter −18 dBFS RMS |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: Orgelpedal D läuft durch; T. 72 endet auf E♭ (♭II) → D in T. 9. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 13,3 s · Loop 64 Takte = 106,7 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Flüstern, Orgelpedal D, ab T. 5 Hohl-Form in Posaunen, Tutti-Schlag auf T. 8.
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

---

#### 4.1.7 `victory_sting` – „Glut über Asche“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `victory_sting` → `assets/audio/music/victory_sting.ogg` |
| Manifest | intensity: Sting · intro: None · looped: false · Set A |
| Zweck / Spielsituation | Missionssieg (z. B. Hohlschrein von Vael Dorn zerstört). Erstes Erklingen der Krone-Form (Dur) im Spiel. Danach `victory_theme` nach R5. |
| Genre | Orchestrale Siegesfanfare |
| Instrumentierung | Hörner, Trompeten, Posaunen, Paukenwirbel, Glocken, Chor-Akkord (Old Aurethi), Streicher |
| Tempo / Takt | 72 BPM, 4/4 (3 Takte + Ausklang) |
| Tonart / Motivform | D-Dur; Leitmotiv Krone-Form (1 – 5 – 6 – 5 · 4 – 3 – 2 – 7 – 1), verkürzt |
| Stimmung | Erlöst, edel, warm – kein überdrehter Triumph |
| Dynamikverlauf | mf (Wirbel) → ff (T. 2) → f gehalten → Ausklang |
| Soundcharakter / Mix | Hell und breit, Hall 3,0 s mit natürlichem Ausklang; Ausklang nicht abschneiden, sanfter Fade nur in den letzten 500 ms |
| Loop-Eignung | Keine (linear). |
| Ziellänge | Suno-Rohtake beliebig (Sting wird geschnitten) · Ziel 8–12 s inkl. Ausklang |

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
- *Schnitt-Alternative:* Schluss-Akkord eines `victory_theme`-Takes als Sting schneiden, falls kein kurzer Take gelingt (identische Klangfarbe sichert Anschluss).
- *Ohne Chor:* reine Blech-/Glocken-Fassung für Skirmish-Siege.

---

#### 4.1.8 `defeat_sting` – „Die Krone fällt“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `defeat_sting` → `assets/audio/music/defeat_sting.ogg` |
| Manifest | intensity: Sting · intro: None · looped: false · Set A |
| Zweck / Spielsituation | Missionsniederlage (Bergfried von Brennhollow fällt, Isolde stirbt). Danach `defeat_theme` nach R5. |
| Genre | Orchestraler Trauer-Sting |
| Instrumentierung | Tiefe Glocke, Posaunen, Solo-Cello, tiefe Streicher, gehauchter Chor |
| Tempo / Takt | 72 BPM, 4/4 (3 Takte mit Ritardando + Ausklang) |
| Tonart / Motivform | D-Moll mit Hohl-Form: Posaune 1 – ♭5, Cello-Fall 4 – ♭3 – ♭2 – ♭7 – 1 |
| Stimmung | Endgültig, kalt, still |
| Dynamikverlauf | f (Glocke) → mp → pp, verklingend |
| Soundcharakter / Mix | Dunkel, lang verhallt (4,0 s), Glocke tief und breit; letzte Klänge in die Stille ausklingen lassen |
| Loop-Eignung | Keine (linear). |
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
- *Schnitt-Alternative:* Schluss eines `defeat_theme`-Takes als Sting nutzen.
- *Ohne Chor:* für Skirmish-Niederlagen ohne Story-Bezug.

---
### 4.2 Weltkarte (Jahreszeiten und Zeitalter)

**Einsatzregel:** In den Zeitaltern I–V folgt die Weltkarten-Musik der Jahreszeit im Spiel
(`worldmap_spring` → `worldmap_summer` → `worldmap_autumn` → `worldmap_winter`). Jahreszeitenwechsel
per R1 (gleiches Set B, 2 Takte Crossfade = 6,0 s). In Zeitalter VI *The Fallen Gods* ersetzt
`worldmap_fallen_gods` alle Jahreszeiten, in Zeitalter VII *The Last Crown* übernimmt `endgame_last_crown`
(Set-Wechsel nach R6).

#### 4.2.1 `worldmap_spring` – „Tauwasser über Valemark“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `worldmap_spring` → `assets/audio/music/worldmap_spring_intro.ogg`, `assets/audio/music/worldmap_spring_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set B |
| Zweck / Spielsituation | Strategische Weltkarte im Frühling: Planung, Truppenbewegung, Ausbau der Provinzen Valemarks. |
| Genre | Kinematisches Fantasy-Orchester, weit und erzählend |
| Instrumentierung | Streicher (legato), Holzflöte, Harfe, Laute, ferne Hörner, Glockenspiel sehr leise, Rahmentrommel im halben Kronenschlag |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-dorisch; Glut-Form in fernen Hörnern (T. 21–28) |
| Stimmung | Erwachend, weit, vorsichtig zuversichtlich |
| Dynamikverlauf | p (Intro) → mp (T. 5–20) → mf (T. 21–28) → mp (T. 29–52) |
| Soundcharakter / Mix | Luftig, breit, Hall 2,6 s; Flöte vorne, Hörner weit hinten; hohe Lage hell, aber nicht scharf |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf F-Dur (♭VII) → g-Moll in T. 5. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 12,0 s · Loop 48 Takte = 144,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Harfe und Streicher-Flageoletts, ein Flötenton.
- T. 5–20: Flötenthema über Laute-Begleitung, Streicher legato.
- T. 21–28: Ferne Hörner mit der Glut-Form, Rahmentrommel im halben Kronenschlag.
- T. 29–40: Streicher übernehmen das Thema, Harfe in Achteln.
- T. 41–52: Rücknahme, Laute und Flöte, Rückleitung über F-Dur.

**Suno-Style-Prompt**
```text
cinematic fantasy orchestral, G dorian, 80 bpm, spring thaw, legato strings, wooden flute theme, harp, lute, distant horns, soft frame drum, wide and hopeful, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: harp, string harmonics]

[Verse]
[Instrumental: wooden flute theme, lute]

[Build]
[Instrumental: distant horns, soft frame drum]

[Verse]
[Instrumental: strings take the theme, harp eighths]

[Interlude]
[Instrumental: lute and flute, calm]

[Verse]
[Instrumental: wooden flute theme, strings]

[Outro]
[Instrumental: lute, no ending]
```

**Varianten**
- *Cover-Basis:* Dieser Take ist Ausgangsmaterial für `worldmap_summer`, `worldmap_autumn` und `worldmap_winter` (Cover mit deren Prompts), damit alle Jahreszeiten dasselbe Thema tragen.
- *Zeitalter II:* Stem ohne Hörner für frühe Kampagne (*The Rising Crown*).

---

#### 4.2.2 `worldmap_summer` – „Goldene Felder, schwere Luft“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `worldmap_summer` → `assets/audio/music/worldmap_summer_intro.ogg`, `assets/audio/music/worldmap_summer_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set B |
| Zweck / Spielsituation | Weltkarte im Sommer – Erntezeit und klassische Feldzugssaison; Wohlstand mit drückender Vorahnung. |
| Genre | Fantasy-Folk-Orchester, rhythmischer |
| Instrumentierung | Fidel, Hackbrett, Laute, Rahmentrommeln, Streicher-Pizzicato, ferne Naturtrompeten, Holzflöte, tiefe Streicher |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-mixolydisch; Frühlingsthema als Cover, Glutruf der Naturtrompeten (T. 29–36) |
| Stimmung | Warm, reich, geschäftig; schwüle Spannung darunter |
| Dynamikverlauf | mp → mf (T. 13–36) → mp (T. 37–52) |
| Soundcharakter / Mix | Sonnig-warm, Mitten präsent, Hall 2,0 s; Hackbrett rechts, Fidel links, Trompeten fern |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C-Dur (IV) → G in T. 5. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 12,0 s · Loop 48 Takte = 144,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Hackbrett-Figur allein, dann Rahmentrommel.
- T. 5–12: Fidel spielt das Weltkartenthema, Pizzicato-Streicher.
- T. 13–28: Volles Folk-Ensemble, Laute und Flöte im Wechsel.
- T. 29–36: Ferne Naturtrompeten mit dem Glutruf, tiefe Streicher grollen leise (Vorahnung).
- T. 37–52: Rücknahme auf Hackbrett und Fidel.

**Suno-Style-Prompt**
```text
fantasy folk orchestra, G mixolydian, 80 bpm, summer harvest, fiddle theme, hammered dulcimer, lute, frame drums, string pizzicato, distant trumpets, warm with tension, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: hammered dulcimer figure, frame drum]

[Verse]
[Instrumental: fiddle theme, string pizzicato]

[Verse]
[Instrumental: full folk ensemble, lute and flute]

[Build]
[Instrumental: distant trumpets call, low strings rumble]

[Interlude]
[Instrumental: dulcimer and fiddle only]

[Verse]
[Instrumental: fiddle theme, steady]
```

**Varianten**
- *Cover-Fassung:* aus `worldmap_spring` erzeugt (bevorzugt), freie Generierung als Ersatz.
- *Kriegssommer:* Stem mit verstärkten tiefen Streichern, wenn sich Valemark im Krieg befindet.

---

#### 4.2.3 `worldmap_autumn` – „Laubfall der Herzogtümer“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `worldmap_autumn` → `assets/audio/music/worldmap_autumn_intro.ogg`, `assets/audio/music/worldmap_autumn_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set B |
| Zweck / Spielsituation | Weltkarte im Herbst – Bilanz, Rivalität der Herzogtümer (Herzogin Mireille Corvane), Vorbereitung auf den Winter. |
| Genre | Melancholisches Kammerorchester |
| Instrumentierung | Solo-Bratsche, Klarinette, Harfe, Celli, Kontrabass, Altflöte, ferne Glocke |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-äolisch; Leitmotiv Asche-Form in Solo-Bratsche (T. 13–20) |
| Stimmung | Herb, nachdenklich, vergänglich |
| Dynamikverlauf | p → mp (T. 13–36) → p (T. 37–52) |
| Soundcharakter / Mix | Warm-dunkel, intim, Hall 2,2 s; Bratsche vorne, Harfe breit |
| Loop-Eignung | Sehr hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf D (V) → g in T. 5. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 12,0 s · Loop 48 Takte = 144,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Harfe in fallenden Arpeggien, ferner Glockenton.
- T. 5–12: Klarinette mit dem Weltkartenthema in Moll.
- T. 13–20: Solo-Bratsche mit Leitmotiv (Asche-Form), Celli tragen.
- T. 21–36: Altflöte und Bratsche im Dialog, Harfe in Vierteln.
- T. 37–52: Celli und Harfe allein, Rückleitung über D-Dur.

**Suno-Style-Prompt**
```text
melancholic fantasy chamber orchestra, G minor, 80 bpm, autumn, solo viola melody, clarinet, harp falling arpeggios, warm cellos, alto flute, distant bell, reflective, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: harp falling arpeggios, distant bell]

[Verse]
[Instrumental: clarinet theme, minor key]

[Verse]
[Instrumental: solo viola melody, warm cellos]

[Interlude]
[Instrumental: alto flute and viola dialogue]

[Verse]
[Instrumental: clarinet theme, harp]

[Outro]
[Instrumental: cellos and harp, no ending]
```

**Varianten**
- *Cover-Fassung:* aus `worldmap_spring` erzeugt (bevorzugt).
- *Trauerherbst:* Stem nur Bratsche und Harfe nach schweren Kampagnenverlusten.

---

#### 4.2.4 `worldmap_winter` – „Unter dem Aschenschnee“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `worldmap_winter` → `assets/audio/music/worldmap_winter_intro.ogg`, `assets/audio/music/worldmap_winter_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set B |
| Zweck / Spielsituation | Weltkarte im Winter – Versorgungsnot, Stillstand der Fronten, Asche mischt sich in den Schnee. |
| Genre | Minimalistisches Ambient-Orchester |
| Instrumentierung | Hohe Streicher-Flageoletts, Altflöte, gestrichener Psalter-Klang, tiefer Bordun (Kontrabass), summender Männerchor fern, einzelne Harfentöne |
| Tempo / Takt | 80 BPM, 4/4 (sehr ruhige Bewegung, halbe und ganze Noten) |
| Tonart / Motivform | G-äolisch; nur Fragment „Die Krone“ in Altflöte (T. 13–16, T. 37–40) |
| Stimmung | Kalt, still, ausharrend |
| Dynamikverlauf | pp → p (T. 13–36) → pp (T. 37–52) |
| Soundcharakter / Mix | Kalt-hell, sehr breit, langer Hall (4,0 s); keine Perkussion; tiefer Bordun sehr leise |
| Loop-Eignung | Sehr hoch. Loop T. 5–52 (48 Takte). Bordun G läuft über die Naht. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 12,0 s · Loop 48 Takte = 144,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Flageoletts aus dem Nichts, Bordun G.
- T. 5–12: Gestrichener Psalter-Klang mit langsamen Wechseln g – Es – g.
- T. 13–16: Altflöte mit „Die Krone“.
- T. 17–36: Summender Männerchor fern, Harfentöne wie Schneeflocken.
- T. 37–40: Altflöte mit „Die Krone“, eine Oktave tiefer.
- T. 41–52: Nur Flageoletts und Bordun.

**Suno-Style-Prompt**
```text
minimal fantasy ambient orchestral, G minor, 80 bpm, frozen winter, high string harmonics, alto flute, bowed psaltery, low drone, distant humming male choir, cold and still, no drums
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: high string harmonics, low drone]

[Verse]
[Instrumental: bowed psaltery, slow chords]

[Verse]
[Instrumental: alto flute, sparse harp notes]

[Interlude]
[Distant humming male choir]
mmm... ooh... mmm...

[Verse]
[Instrumental: alto flute lower, harmonics]

[Outro]
[Instrumental: harmonics and drone only, no ending]
```

**Varianten**
- *Cover-Fassung:* aus `worldmap_spring` (bevorzugt), mit stark reduzierter Besetzung.
- *Skarnheim-Winter:* Cover mit Drehleier statt Psalter, wenn die Kamera über Skarnheim steht (Tonart G beibehalten).

---

#### 4.2.5 `worldmap_fallen_gods` – „Das Schweigen der Götter“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `worldmap_fallen_gods` → `assets/audio/music/worldmap_fallen_gods_intro.ogg`, `assets/audio/music/worldmap_fallen_gods_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set B |
| Zweck / Spielsituation | Weltkarte in Zeitalter VI *The Fallen Gods*: Der Schleier reißt weiter, die alten Götter Ithren, Morvaine, Calyx, Thal und Oruun bleiben stumm; Hohlung breitet sich aus. Ersetzt die Jahreszeiten. |
| Genre | Sakral-dunkles Orchester mit Chor |
| Instrumentierung | Orgel (weich), gemischter Chor (Old Aurethi), tiefes Blech, Glasharmonika-Klang, Streicher-Cluster, Großtrommel fern im halben Kronenschlag |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-phrygisch (A♭); Asche-Form im Chor, Hohl-Form in Posaunen (T. 29–36) |
| Stimmung | Sakral, zerbrochen, ehrfürchtig, verloren |
| Dynamikverlauf | p → mf (T. 21–36) → p (T. 37–52) |
| Soundcharakter / Mix | Kathedralenraum, Hall 4,5 s; Orgel tief und breit, Chor hinten, Glasklang hoch und sehr leise |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Orgel-Bordun G über die Naht; Naht auf A♭ (♭II) → G. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 12,0 s · Loop 48 Takte = 144,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Orgel-Bordun G, Glasklang.
- T. 5–20: Chor pp mit der Asche-Form, Streicher-Cluster.
- T. 21–28: Chor wächst, Großtrommel fern.
- T. 29–36: Posaunen mit Hohl-Form, Chor bricht ab, nur Orgel bleibt.
- T. 37–52: Chor kehrt pp zurück, Rückleitung über A♭.

**Suno-Style-Prompt**
```text
sacred dark fantasy orchestral, G phrygian, 80 bpm, soft pipe organ drone, choir in invented language, low brass, glass harmonica, string clusters, distant drum, awe and loss
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: organ drone, glass harmonica]

[Verse]
[Soft choir]
Ith-ra, mor-ra, ca-li-reh
Tha-len, o-run, ka veh

[Build]
[Choir grows, distant drum]
Sol-vae es-keth, el-dra se-reth

[Bridge]
[Low trombones, choir stops, organ alone]

[Verse]
[Soft choir returns]
Ith-ra, mor-ra, ca-li-reh
Vae-loth dhum

[Outro]
[Instrumental: organ drone, no ending]
```

*Textbedeutung:* „Licht, Tod, Wachstum, Meer, Stein – nicht mehr. Die Sonne fiel, die Welt klagt. Der Schleier ist leer.“

**Varianten**
- *Instrumental:* Chor durch Orgel-Register ersetzt, falls Silben unsauber.
- *Spätphase:* Stem mit lauterer Hohl-Form, wenn die Hohlung mehr als die Hälfte der Karte erfasst hat.

---

### 4.3 Städtebau und wirtschaftliche Entwicklung

#### 4.3.1 `build_night` – „Wache über Brennhollow“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `build_night` → `assets/audio/music/build_night_intro.ogg`, `assets/audio/music/build_night_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Friedlicher Städtebau bei Nacht (Tag-Nacht-Zyklus). Wechsel Tag ↔ Nacht per R1 mit `build_calm` (gleiches Set). |
| Genre | Nächtliches Kammer-Ambient |
| Instrumentierung | Solo-Cello, Harfe, leise Bratschen, Altflöte, Laute (sparsam), weiche Glockentöne fern |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Thema von `build_calm` (Cover), Asche-Form im Cello (T. 29–36) |
| Stimmung | Still, wachsam, Lagerfeuerwärme gegen die Dunkelheit |
| Dynamikverlauf | pp → p, flach |
| Soundcharakter / Mix | Dunkel-warm, Hall 2,8 s; Höhen leicht abgesenkt, viel Raum für Nacht-Ambience (`ambience_night_loop`) |
| Loop-Eignung | Sehr hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf C → d in T. 5; Harfen-Ostinato durchgehend. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 13,3 s · Loop 48 Takte = 160,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Harfe allein, langsames Ostinato.
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

---

#### 4.3.2 `economy_prosperity` – „Markt der Asche“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `economy_prosperity` → `assets/audio/music/economy_prosperity_intro.ogg`, `assets/audio/music/economy_prosperity_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Wirtschaftliche Entwicklung: Siedlung wächst, Markt, Handel mit Aschglas, hohe Bevölkerung, Ressourcenüberschuss. |
| Genre | Lebhaftes Fantasy-Folk-Ensemble |
| Instrumentierung | Laute, Fidel, Hackbrett, Schalmei, Blockflöte, Pizzicato-Streicher, Rahmentrommeln, Tamburin |
| Tempo / Takt | 72 BPM, 4/4 (Achtel-Groove, wirkt lebhafter) |
| Tonart / Motivform | D-mixolydisch; Glut-Form in Schalmei (T. 13–20); Asche-Fragment in Celli (T. 29–36) als Schatten |
| Stimmung | Geschäftig, zuversichtlich, stolz; ein Hauch Mahnung |
| Dynamikverlauf | mp → mf (T. 13–28) → mp (T. 29–44) |
| Soundcharakter / Mix | Hell, nah, Hall 1,5 s; Instrumente klar getrennt im Panorama; Tiefmitten schlank für Bau-SFX |
| Loop-Eignung | Hoch. Loop T. 5–44 (40 Takte). Naht: T. 44 auf A (V) → D in T. 5. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 4 Takte = 13,3 s · Loop 40 Takte = 133,3 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Hackbrett und Tamburin.
- T. 5–12: Fidel-Thema über Laute, Pizzicato.
- T. 13–20: Schalmei mit Glut-Form, Rahmentrommeln.
- T. 21–28: Tutti-Tanz, Blockflöte umspielt.
- T. 29–36: Ausdünnung, Celli mit Asche-Fragment, Laute.
- T. 37–44: Fidel kehrt zurück, Rückleitung.

**Suno-Style-Prompt**
```text
lively fantasy folk ensemble, D mixolydian, 72 bpm, bustling market, fiddle, hammered dulcimer, shawm, recorder, lute, string pizzicato, frame drums, proud and busy, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: hammered dulcimer, tambourine]

[Verse]
[Instrumental: fiddle theme, lute, pizzicato]

[Verse]
[Instrumental: shawm melody, frame drums]

[Build]
[Instrumental: full dance, recorder ornaments]

[Interlude]
[Instrumental: cellos, lute, quieter]

[Verse]
[Instrumental: fiddle theme returns]
```

**Varianten**
- *Saelith-Handel:* Cover mit stärkerem Hackbrett und Zither, wenn ein Handelsabkommen mit dem Saelith-Pakt aktiv ist (Tonart D beibehalten, um im Set A zu bleiben).
- *Spätere Zeitalter:* Stem ohne Tamburin für ernstere Kampagnenphasen.

---

### 4.4 Erkundung, Diplomatie, Intrige, Kriegsvorbereitung

#### 4.4.1 `explore_wilds` – „Jenseits der Wachfeuer“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `explore_wilds` → `assets/audio/music/explore_wilds_intro.ogg`, `assets/audio/music/explore_wilds_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Erkundung: Späher und Held im Nebel des Krieges, Kartenrand, Entdeckung von Aschglas-Vorkommen und Ruinen. |
| Genre | Atmosphärisches Abenteuer-Orchester |
| Instrumentierung | Altflöte, Solo-Bratsche, Harfe, Streicher-Flageoletts, Holzperkussion, ferne tiefe Trommel, Hörner pp |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Fragment „Die Krone“ in Hörnern (T. 21–24), Fall-Figur in Bratsche (T. 37–40) |
| Stimmung | Neugierig, weit, vorsichtig, leise Gefahr |
| Dynamikverlauf | p → mp (T. 21–36) → p (T. 37–52) |
| Soundcharakter / Mix | Offen, sehr breit, Hall 3,0 s; Holzperkussion links/rechts verteilt (Schritte im Wald), Flöte mittig |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf G (IV) → d in T. 5. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 4 Takte = 13,3 s · Loop 48 Takte = 160,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Flageoletts und einzelne Harfentöne.
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

---

#### 4.4.2 `diplomacy_court` – „Kerzen im Thronsaal“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `diplomacy_court` → `assets/audio/music/diplomacy_court_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set B |
| Zweck / Spielsituation | Diplomatie-Bildschirm: Verhandlungen mit König Aldric III., Torvald Eisenhand, Sylaen, Ophira Saelith-Varo, Bündnisse, Verträge. Aus der Weltkarte per R1 (Set B). |
| Genre | Höfische Kammermusik, dunkel gefärbt |
| Instrumentierung | Positiv-Orgel, Laute, Kammerstreicher (Quartett), Blockflöte, leise Handtrommel |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-dorisch; Glut-Form als höfisches Menuett-Zitat in der Blockflöte (T. 17–20) |
| Stimmung | Förmlich, abwägend, höflich-kühl |
| Dynamikverlauf | p → mp (T. 9–24) → p (T. 25–32) |
| Soundcharakter / Mix | Kleiner Saal, Hall 1,8 s; trocken und klar, damit Dialogtexte lesbar bleiben; Orgel weich |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte), T. 1 als weicher Einstieg. Naht: T. 32 auf D (V) → g in T. 1. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Loop 32 Takte = 96,0 s |

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

---

#### 4.4.3 `intrigue_shadows` – „Kassians Netz“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `intrigue_shadows` → `assets/audio/music/intrigue_shadows_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set B |
| Zweck / Spielsituation | Politische Intrigen: Spionage-Missionen, Verrat, Thronstreit (Zeitalter V *The Divided Throne*), Kassian Vale, Herzogin Mireille Corvane; zugleich Tension-Stufe auf der Weltkarte. |
| Genre | Verschwörerisches Kammer-Suspense |
| Instrumentierung | Streicher-Pizzicato, Col-legno-Celli, Bassklarinette, Solo-Kontrabass, gedämpfte Harfe, Zither-Tremolo, Flüstern sehr leise, Holzblock |
| Tempo / Takt | 80 BPM, 4/4 |
| Tonart / Motivform | G-phrygisch (A♭); verzerrtes Fragment „Die Krone“ mit chromatischem Riss (1 – 5 – ♭6 – 6 – ♭6) in Bassklarinette |
| Stimmung | Schleichend, misstrauisch, doppelbödig |
| Dynamikverlauf | p → mp (T. 9–24) → p (T. 25–32) |
| Soundcharakter / Mix | Nah, trocken, Hall 1,2 s; Pizzicato hart links/rechts, Bassklarinette mittig, Flüstern unterhalb der Wahrnehmungsschwelle |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte). Naht: T. 32 auf A♭ (♭II) → g in T. 1. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Loop 32 Takte = 96,0 s |

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

---

#### 4.4.4 `war_preparation` – „Schmieden und Schwüre“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `war_preparation` → `assets/audio/music/war_preparation_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Kriegsvorbereitung: Kaserne im Dauerbetrieb, Rekrutierung, Aufmarsch vor einem Angriff, Belagerungsvorbereitung. Alternative Tension-Stufe zu `build_tension`, wenn der Spieler selbst angreift. |
| Genre | Treibendes Dark-Fantasy-Orchester |
| Instrumentierung | Amboss- und Schmiedeperkussion, Celli-Ostinato, Rührtrommel, Hörner, summender Männerchor, Rahmentrommeln, Kontrabässe |
| Tempo / Takt | 72 BPM, 4/4 (Sechzehntel-Ostinato) |
| Tonart / Motivform | D-äolisch; Glut-Form in Hörnern (T. 17–24), Kronenschlag auf dem Amboss |
| Stimmung | Entschlossen, schwer, unheilvoll-zielgerichtet |
| Dynamikverlauf | mp → mf (T. 9–24) → mp (T. 25–32) |
| Soundcharakter / Mix | Erdig, mittenbetont, Hall 1,8 s; Amboss klar, aber nicht spitz (kein Konflikt mit `sfx.build_progress`) |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte), T. 1 ohne Akzent. Naht: T. 32 auf A (V) → d in T. 1. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Loop 32 Takte = 106,7 s |

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

---
### 4.5 Belagerungen

Beide Belagerungs-Tracks bilden Set S (C, 120 BPM). Wechsel zwischen ihnen per R1 (2 Takte = 4,0 s),
Eintritt aus anderen Sets per R2 über das jeweilige Intro.

#### 4.5.1 `siege_assault` – „Sturm auf die Mauern“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `siege_assault` → `assets/audio/music/siege_assault_intro.ogg`, `assets/audio/music/siege_assault_loop.ogg` |
| Manifest (Vorschlag) | intensity: Battle · intro: Some · looped: true · Set S |
| Zweck / Spielsituation | Der Spieler belagert eine Festung: Rammböcke, Katapulte, Sturm auf Mauern (z. B. Bergfesten Skarnheims, Festung Solmere im Vorfeld). |
| Genre | Episches Belagerungsorchester mit Chor |
| Instrumentierung | Großtrommeln mit Rammbock-Schlägen (Zählzeit 1 und 3), Kriegshörner, tiefes Blech, Streicher-Ostinato, Männerchor-Rufe (Old Aurethi), Ketten, Holzkrachen als Perkussion, Pauken |
| Tempo / Takt | 120 BPM, 4/4 |
| Tonart / Motivform | C-äolisch; Glut-Form augmentiert im Blech (T. 25–40), Kronenschlag in den Trommeln |
| Stimmung | Unaufhaltsam, wuchtig, opferbereit |
| Dynamikverlauf | f (Intro) → f (T. 9–24) → ff (T. 25–40) → mf (T. 41–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Massiv, tief, Hall 2,2 s; Rammbock-Schläge mono-zentriert mit Subanteil, Blech breit |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf G (V) → c in T. 9. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 16,0 s · Loop 64 Takte = 128,0 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Kriegshörner, erster Rammbock-Schlag in T. 5, Paukenwirbel zu T. 9.
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

---

#### 4.5.2 `siege_defense` – „Die Mauer hält“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `siege_defense` → `assets/audio/music/siege_defense_intro.ogg`, `assets/audio/music/siege_defense_loop.ogg` |
| Manifest (Vorschlag) | intensity: Battle · intro: Some · looped: true · Set S |
| Zweck / Spielsituation | Die eigene Stadt wird belagert: Mauern unter Beschuss, Hohle oder Aschenlegion vor den Toren, letzte Verteidigung. |
| Genre | Verzweifelt-heroisches Orchester mit Chor |
| Instrumentierung | Sturmglocke, Pauken, tiefe Streicher, Hörner, Frauenchor (Klage) und Männerchor (Trotz), Großtrommeln, Streicher-Tremolo |
| Tempo / Takt | 120 BPM, 4/4 |
| Tonart / Motivform | C-phrygisch (D♭); Asche-Form im Frauenchor (T. 17–32), Glut-Form tutti im Höhepunkt (T. 49–64) |
| Stimmung | Verzweifelt, dann trotzig; Durchhalten gegen die Übermacht |
| Dynamikverlauf | mf (Glocke) → f (T. 9–16) → mf, klagend (T. 17–32) → f (T. 33–48) → ff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Dicht, eng, Hall 2,4 s; Glocke links fern, Chöre breit, Pauken zentriert |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf D♭ (♭II) → c in T. 9; Glocke nicht über die Naht. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 16,0 s · Loop 64 Takte = 128,0 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Sturmglocke allein, ab T. 5 Pauken und Streichertremolo.
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

---

### 4.6 Bosskämpfe

`boss_pale_reeve` steht in 4.1.6. Alle Bosse werden nach R4 betreten.

#### 4.6.1 `boss_hollow_king` – „Varkh Dhum“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `boss_hollow_king` → `assets/audio/music/boss_hollow_king_intro.ogg`, `assets/audio/music/boss_hollow_king_loop.ogg` |
| Manifest (Vorschlag) | intensity: Boss · intro: Some · looped: true · Set E |
| Zweck / Spielsituation | Finaler Bosskampf in der Festung Solmere gegen den Hohlen König – Varkhan Aurelith, der Letzte Sonnenkönig, gefangen und vom Hohlen Tief entstellt. Musikalischer Gipfel des Spiels: Alle Formen der Aschenkrone treffen aufeinander. |
| Genre | Kolossales Dark-Fantasy-Finale, Orchester, Chor, Orgel |
| Instrumentierung | Volles Orchester, großer gemischter Chor (Old Aurethi), Pfeifenorgel, Glocken, verfremdete imperiale Fanfaren, Frauenstimme solo, Glasharmonika-Klang, Großtrommeln, Subbass |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch mit Hohl-Färbung; Krone-Form als gebrochene imperiale Fanfare (Intro), Hohl-Form dominant, Glut-Form gegen Hohl-Form im Kontrapunkt (Höhepunkt) |
| Stimmung | Tragisch, erschreckend, monumental; Mitleid mit dem gefallenen König |
| Dynamikverlauf | p (T. 1–8) → ff (T. 16) → f (T. 17–48) → p (T. 49–56) → fff (T. 57–72) → f (T. 73–80) |
| Soundcharakter / Mix | Riesige leere Halle, Hall 4,0 s; Orgel tief und breit, Chor raumfüllend, Frauenstimme nah und trocken (Erinnerung); Subbass kontrolliert |
| Loop-Eignung | Hoch. Loop T. 17–80 (64 Takte). Naht: Orgel-Bordun D läuft durch; T. 80 auf E♭ → D in T. 17. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 16 Takte = 29,1 s · Loop 64 Takte = 116,4 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Orgel-Bordun, Frauenstimme singt „Solvae esketh“ auf der Asche-Form.
- T. 9–16 (Intro-Datei): Imperiale Fanfare spielt die Krone-Form in Dur und bricht in die Hohl-Form; Crescendo, Tutti-Schlag am Ende von T. 16.
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

---

#### 4.6.2 `boss_ashen_dragon` – „Vyrmathax“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `boss_ashen_dragon` → `assets/audio/music/boss_ashen_dragon_intro.ogg`, `assets/audio/music/boss_ashen_dragon_loop.ogg` |
| Manifest (Vorschlag) | intensity: Boss · intro: Some · looped: true · Set X |
| Zweck / Spielsituation | Bosskampf gegen den Aschdrachen Vyrmathax in der Glutweite (Cinderreach): Vulkankessel, Ascheregen, Flugangriffe. |
| Genre | Wildes Dark-Fantasy-Kampforchester, perkussiv |
| Instrumentierung | Taiko-artige Großtrommeln, Metallperkussion (Ambosse, Bleche), Blechcluster, Horn-Glissandi (Drachenbrüllen), tiefer Männerchor, Streicher-Tremolo, Feuerrauschen-Riser |
| Tempo / Takt | 152 BPM, 4/4 (Akzente 3 + 3 + 2) |
| Tonart / Motivform | F-phrygisch (G♭); Asche-Form augmentiert in tiefen Hörnern mit Clustern; eigene Kennung „Flügelschlag“ (Doppelschlag der Großtrommeln auf Zählzeit 4 und 4+) |
| Stimmung | Rasend, glühend, urgewaltig |
| Dynamikverlauf | mf (Intro-Beginn) → ff (T. 8) → f (T. 9–40) → mf (T. 41–48) → fff (T. 49–64) → f (T. 65–72) |
| Soundcharakter / Mix | Heiß, druckvoll, Hall 2,0 s; Trommeln dominant, Blech mit leichter Sättigung, Höhen gezähmt (Feuer-SFX brauchen Platz) |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf G♭ (♭II) → f in T. 9. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 12,6 s · Loop 64 Takte = 101,1 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Feuerrauschen, Horn-Glissando (Brüllen), Flügelschläge der Trommeln, Tutti-Schlag in T. 8.
- T. 9–24: Taiko-Groove mit Kronenschlag, Streicher-Tremolo, Metallperkussion.
- T. 25–40: Tiefer Männerchor „Kael dhum“, Hörner mit Asche-Form.
- T. 41–48: Breakdown – Trommeln allein und Flügelschläge (Flugphase des Drachen).
- T. 49–64: Höhepunkt – Blechcluster, Chor, alle Trommeln.
- T. 65–72: Rückführung auf den Groove von T. 9.

**Suno-Style-Prompt**
```text
ferocious dark fantasy dragon boss battle, F phrygian, 152 bpm, thunderous taiko, metal percussion, roaring brass clusters, horn glissandi, deep male choir, fiery string tremolo
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Fire roar, horn glissando, double drum wingbeats]
[Massive hit]

[Verse]
[Instrumental: taiko groove 3-3-2, string tremolo, metal percussion]

[Chorus]
[Deep male choir]
Kael dhum, kael dhum
Cin-dra rhan, el-dra esk

[Breakdown]
[Instrumental: drums alone, wingbeats]

[Climax]
[Brass clusters, full choir, all drums]
Kael ar! Cin-dra ar!
Vyr-ma-thax! Vyr-ma-thax!

[Verse]
[Instrumental: taiko groove, string tremolo]
```

*Textbedeutung:* „Hohle Glut. Asche steigt, die Welt fällt. Ewige Glut! Ewige Asche! Vyrmathax!“

**Varianten**
- *Bodenphase (Stem):* ohne Horn-Glissandi, wenn der Drache am Boden kämpft.
- *Ersatz-Take:* instrumental ohne Chor bei unsauberen Silben.

---

#### 4.6.3 `boss_root_mother` – „Die Wurzelmutter“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `boss_root_mother` → `assets/audio/music/boss_root_mother_intro.ogg`, `assets/audio/music/boss_root_mother_loop.ogg` |
| Manifest (Vorschlag) | intensity: Boss · intro: Some · looped: true · Set X |
| Zweck / Spielsituation | Bosskampf gegen die Wurzelmutter, Naturentität des Dornwalds – uralt, nicht böse, aber gnadenlos; ggf. von der Hohlung berührt. |
| Genre | Urtümliches Wald-Boss-Orchester |
| Instrumentierung | Holz- und Rindentrommeln, knarrende Kontrabässe col legno, Flüsterchor und Frauenchor (Old Aurethi), verfremdete Holzflöten, dissonante Harfe, tiefe Streicher, gestrichene Metallklänge |
| Tempo / Takt | 12/8, ♩. = 84 (vier punktierte Viertel pro Takt) |
| Tonart / Motivform | A-äolisch mit chromatischen Rissen; Wurzelreigen (Dornenbund-Kennung) verdunkelt, Asche-Form im Frauenchor (T. 17–24) |
| Stimmung | Uralt, wogend, erstickend, ehrfurchtgebietend |
| Dynamikverlauf | p (Intro) → f (T. 9–16) → mf (T. 17–24) → ff (T. 25–36) → f (T. 37–40) |
| Soundcharakter / Mix | Dicht, organisch, Hall 3,0 s mit dunklem Charakter; Holz knarrt nah, Chöre umhüllen von allen Seiten |
| Loop-Eignung | Hoch. Loop T. 9–40 (32 Takte). Naht: T. 40 auf E (V) → a in T. 9; Wiegefigur durchgehend. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 8 Takte = 22,9 s · Loop 32 Takte = 91,4 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Knarren der Kontrabässe, Flüstern „calyre“, Rindentrommeln setzen ein.
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

---
### 4.7 Fraktionsmusik (je Thema + Kampfvariante)

**Produktionsregel:** Zuerst das Thema generieren und auswählen, dann die Kampfvariante **per Cover aus dem
Thema-Take** erzeugen (gleiche Tonika, gleiche BPM-Familie laut Set). So teilen beide Tracks Melodie und
Klangfarbe, und der Wechsel Thema → Kampf (R2) klingt wie eine Eskalation desselben Stücks.
**Einsatz:** Das Thema läuft als Calm-Musik der eigenen Fraktion im Gefecht (bei den Hohlen und der
Aschenlegion als Tension-Stufe, wenn sie als Gegner auftauchen); die Kampfvariante ist die Combat-Musik,
wenn gegen diese Fraktion gekämpft wird.

#### 4.7.1 `faction_ember_crown_theme` – „Die Flamme wacht“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_ember_crown_theme` → `assets/audio/music/faction_ember_crown_theme_intro.ogg`, `assets/audio/music/faction_ember_crown_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Thema der Glutkrone von Valemark (Karmesin/Gold): Fraktionsauswahl, Gefecht als Glutkrone, Kampagnenmomente der Krone, Auftritte von König Aldric III. und Bruder Hal Venn. |
| Genre | Feierliches Ritter-Orchester mit Folk-Elementen |
| Instrumentierung | Hörner (Glutruf), Naturtrompeten, Laute, Fidel, Streicher, Frauenstimme solo (Glutstimme, Old Aurethi), Kirchenglocken, Rührtrommel, Orgel leise |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-mixolydisch; Glutruf (Intro), Glut-Form in Frauenstimme (T. 13–20) und Hörnern (T. 29–36) |
| Stimmung | Gläubig, ritterlich, warm, pflichtbewusst |
| Dynamikverlauf | mf (Glutruf) → mp (T. 5–12) → mf (T. 13–28) → f (T. 29–36) → mp (T. 37–44) |
| Soundcharakter / Mix | Steinhalle, Hall 2,6 s; Hörner und Glocken mit Glanz, Laute nah, Frauenstimme mittig vorne |
| Loop-Eignung | Hoch. Loop T. 5–44 (40 Takte). Naht: T. 44 auf C (♭VII) → D in T. 5. |
| Ziellänge | Suno-Rohtake 3:00–3:30 · Intro 4 Takte = 13,3 s · Loop 40 Takte = 133,3 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Glutruf der Hörner, Glockenschlag, Orgel-Liegeton.
- T. 5–12: Laute und Fidel mit Prozessionsmelodie, Streicher.
- T. 13–20: Frauenstimme singt die Glut-Form („Kael rhana“).
- T. 21–28: Streicher und Naturtrompeten, Rührtrommel im Kronenschlag.
- T. 29–36: Hörner unisono mit Glut-Form, Glocken.
- T. 37–44: Rücknahme auf Laute und Orgel.

**Suno-Style-Prompt**
```text
noble dark fantasy knightly orchestral, D mixolydian, 72 bpm, horn call, natural trumpets, lute, fiddle, strings, solo female voice in invented language, church bells, snare
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Horn call rising, church bell, soft organ]

[Verse]
[Instrumental: lute and fiddle procession, strings]

[Verse]
[Solo female voice]
Kael rha-na, kael rha-na
Ith-ra ar, nos veth

[Build]
[Instrumental: strings and natural trumpets, snare]

[Climax]
[Horns in unison, bells]
[Solo female voice]
Kael ar, au-reh rha-na

[Outro]
[Instrumental: lute and organ, no ending]
```

**Varianten**
- *Kirchenfassung:* Cover mit Orgel und Chor für Szenen in Kaldris (Erzflammenwahrer).
- *Instrumental:* Frauenstimme durch Solo-Horn ersetzt.

---

#### 4.7.2 `faction_ember_crown_battle` – „Glutklingen“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_ember_crown_battle` → `assets/audio/music/faction_ember_crown_battle_intro.ogg`, `assets/audio/music/faction_ember_crown_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Kampfmusik, wenn gegen die Glutkrone gekämpft wird (Gefecht, Zeitalter V *The Divided Throne* – Bürgerkrieg gegen Herzogin Mireille Corvane) und als heroische Combat-Alternative bei Isoldes Großangriffen. |
| Genre | Heroisches Kampforchester mit Chor |
| Instrumentierung | Hörner, Trompeten, Streicher-Ostinato, Rührtrommeln, Großtrommeln im Kronenschlag, gemischter Chor, Glocken, Pauken |
| Tempo / Takt | 144 BPM, 4/4 |
| Tonart / Motivform | D-dorisch; Glutruf als Signal, Glut-Form augmentiert im Chor (T. 21–36) |
| Stimmung | Entschlossen, stolz, glühend |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–60) → f (T. 61–68) |
| Soundcharakter / Mix | Glänzend, kraftvoll, Hall 2,0 s; Blech hell, aber nicht schrill, Trommeln tief zentriert |
| Loop-Eignung | Hoch. Loop T. 5–68 (64 Takte). Naht: T. 68 auf A (V) → d in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 6,7 s · Loop 64 Takte = 106,7 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Glutruf der Trompeten, Paukenwirbel.
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

---

#### 4.7.3 `faction_skarn_clans_theme` – „Ahnenfeuer im Schnee“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_skarn_clans_theme` → `assets/audio/music/faction_skarn_clans_theme_intro.ogg`, `assets/audio/music/faction_skarn_clans_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set F2 |
| Zweck / Spielsituation | Thema der Clans von Skarnheim (Eisblau/Stahl): Clanrat, Bergfesten, Torvald Eisenhand, Ahnenpfad. |
| Genre | Nordisches Folk-Orchester mit tiefem Männerchor |
| Instrumentierung | Drehleier, Streichleier, tiefer Männerchor auf Quinten-Bordun, große Rahmentrommeln, Kriegshorn aus Horn, Kontrabässe, Celli |
| Tempo / Takt | 66 BPM, 4/4 |
| Tonart / Motivform | E-äolisch; Ahnenschritt (Zweierschritt), Asche-Form in der Streichleier (T. 13–20) |
| Stimmung | Stoisch, rau, ehrwürdig, kalt |
| Dynamikverlauf | mp (Intro) → mf (T. 5–28) → mp (T. 29–36) |
| Soundcharakter / Mix | Weiter Bergraum, Hall 3,5 s; Chor tief und breit, Drehleier mittig, Trommeln schwer |
| Loop-Eignung | Hoch. Loop T. 5–36 (32 Takte). Naht: Bordun E über die Naht, T. 36 auf D (♭VII) → e in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 14,5 s · Loop 32 Takte = 116,4 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Kriegshorn, Bordun der Drehleier.
- T. 5–12: Männerchor auf Bordun mit Ruf „Haa – o-run“, Rahmentrommeln im Ahnenschritt.
- T. 13–20: Streichleier mit Asche-Form.
- T. 21–28: Chor und Drehleier gemeinsam, Kontrabässe.
- T. 29–36: Rücknahme auf Bordun und Trommeln.

**Suno-Style-Prompt**
```text
nordic dark fantasy folk orchestral, E minor, 66 bpm, hurdy gurdy drone, bowed lyre, deep male choir chanting, huge frame drums, war horn, stoic and cold, ancestral
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[War horn, hurdy gurdy drone]

[Verse]
[Deep male choir, slow steps]
Haa... o-run... haa... o-run...
Li-ra-en, li-ra-en

[Verse]
[Instrumental: bowed lyre melody, drums]

[Chorus]
[Male choir and hurdy gurdy]
O-run ar, veth ar
Haa... o-run...

[Outro]
[Instrumental: drone and drums, no ending]
```

*Textbedeutung:* „Stein … Erinnerung … Ewiger Stein, ewiger Eid.“ (Bezug auf Oruun, den Steingott.)

**Varianten**
- *Clanrat (Stem):* nur Chor und Bordun für Dialogszenen mit Torvald Eisenhand.
- *Instrumental:* Chor durch Drehleier-Melodie ersetzt.

---

#### 4.7.4 `faction_skarn_clans_battle` – „Eisenhand“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_skarn_clans_battle` → `assets/audio/music/faction_skarn_clans_battle_intro.ogg`, `assets/audio/music/faction_skarn_clans_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set F2 |
| Zweck / Spielsituation | Kampf gegen die Clans von Skarnheim: schwere Infanterie, Schildwälle, Gebirgsschlachten. |
| Genre | Wuchtiges Nordclan-Kampforchester |
| Instrumentierung | Große Rahmentrommeln, Stampfen und Klatschen, Männerchor-Rufe, treibende Drehleier, tiefes Blech, Kriegshörner, Kontrabässe |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | E-äolisch; Ahnenschritt verdoppelt, Thema-Melodie in Drehleier, Asche-Form in Posaunen (T. 21–36) |
| Stimmung | Wild, unbeugsam, hart wie Eisen |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–60) → f (T. 61–68) |
| Soundcharakter / Mix | Roh, tief, Hall 2,6 s; Stampfen und Trommeln vorne, Chor breit |
| Loop-Eignung | Hoch. Loop T. 5–68 (64 Takte). Naht: T. 68 auf B (V) → e in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 7,3 s · Loop 64 Takte = 116,4 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Kriegshörner, Stampfen, Ruf „Haa!“.
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

---

#### 4.7.5 `faction_thorn_covenant_theme` – „Der Grüne Kreis“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_thorn_covenant_theme` → `assets/audio/music/faction_thorn_covenant_theme_intro.ogg`, `assets/audio/music/faction_thorn_covenant_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set F3 |
| Zweck / Spielsituation | Thema des Dornenbundes (Moosgrün/Bernstein): Druidenzirkel, die Veyl, Sylaen, uralter Dornwald. |
| Genre | Mystisches Wald-Folk-Ensemble |
| Instrumentierung | Holzflöte, Harfe, gezupfte Leier, Rindentrommel, Frauenstimme (Old Aurethi), Flüsterchor, Streicher, Holzperkussion |
| Tempo / Takt | 6/8, ♩. = 72 |
| Tonart / Motivform | A-dorisch; Wurzelreigen (6/8-Wiegefigur, pentatonisch), Glut-Form in Frauenstimme (T. 25–40) |
| Stimmung | Verwurzelt, geheimnisvoll, lebendig, alt |
| Dynamikverlauf | p (Intro) → mp (T. 9–24) → mf (T. 25–40) → mp (T. 41–56) → p (T. 57–72) |
| Soundcharakter / Mix | Dichter Wald, kurzer warmer Raum (Hall 1,6 s) plus ferner Hall auf Flüsterchor; Holzflöte nah |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf G (♭VII) → a in T. 9; Wiegefigur der Leier durchgehend. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 8 Takte = 13,3 s · Loop 64 Takte = 106,7 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Leier-Wiegefigur allein, Flüstern „calyre“.
- T. 9–24: Holzflöte mit Wurzelreigen, Rindentrommel.
- T. 25–40: Frauenstimme mit Glut-Form, Harfe und Streicher.
- T. 41–56: Flöte und Harfe im Wechsel, Holzperkussion.
- T. 57–72: Rücknahme auf Leier und Flüstern.

**Suno-Style-Prompt**
```text
mystical dark fantasy forest folk, A dorian, lilting 6/8, 72 bpm, wooden flute, harp, plucked lyre, bark drum, female voice and whispers in invented language, ancient, alive
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Plucked lyre lilting figure, whispers]
ca-li-reh... ca-li-reh...

[Verse]
[Instrumental: wooden flute melody, bark drum]

[Chorus]
[Female voice, harp and strings]
Ca-li-reh, ca-li-reh en o-run
Li-ra-en sa el-dra

[Interlude]
[Instrumental: flute and harp, wooden percussion]

[Outro]
[Plucked lyre, whispers, no ending]
ca-li-reh...
```

*Textbedeutung:* „Wachstum, Wachstum im Stein. Erinnerung an die Welt.“

**Varianten**
- *Veyl-Ritual (Stem):* nur Flüsterchor, Leier und Rindentrommel.
- *Instrumental:* Frauenstimme durch Altflöte ersetzt.

---

#### 4.7.6 `faction_thorn_covenant_battle` – „Dornenhinterhalt“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_thorn_covenant_battle` → `assets/audio/music/faction_thorn_covenant_battle_intro.ogg`, `assets/audio/music/faction_thorn_covenant_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set F3 |
| Zweck / Spielsituation | Kampf gegen den Dornenbund: Guerilla, Hinterhalte, Naturmagie im dichten Wald. |
| Genre | Treibendes Wald-Kampfensemble |
| Instrumentierung | Holz- und Rindentrommeln (schnell), Streicher-Staccato, Flöten-Triller, Rindenhorn, rhythmische Flüsterrufe, Harfe gedämpft, tiefe Streicher |
| Tempo / Takt | 6/8, ♩. = 72 (Sechzehntel-Unterteilung, wirkt doppelt so schnell wie das Thema) |
| Tonart / Motivform | A-äolisch; Wurzelreigen in Staccato-Streichern, Asche-Form im Rindenhorn (T. 25–32) |
| Stimmung | Hinterhältig, flink, wild |
| Dynamikverlauf | mf (Intro) → f (T. 9–24) → mf (T. 25–32) → f (T. 33–56) → mf, plötzliche Pausen (T. 57–64) → f (T. 65–72) |
| Soundcharakter / Mix | Trocken, perkussiv, Hall 1,4 s; Trommeln im Panorama verteilt (Angriffe von allen Seiten) |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf E (V) → a in T. 9. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 8 Takte = 13,3 s · Loop 64 Takte = 106,7 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Rindenhorn, Flüsterruf, Trommeln setzen schlagartig ein.
- T. 9–24: Staccato-Streicher mit Wurzelreigen, schnelle Holztrommeln.
- T. 25–32: Rindenhorn mit Asche-Form.
- T. 33–56: Flöten-Triller, alle Trommeln, Flüsterrufe.
- T. 57–64: Hinterhalt-Pausen – Generalpausen von je einem halben Takt, dann Schläge.
- T. 65–72: Rückführung.

**Suno-Style-Prompt**
```text
driving dark fantasy forest battle, A minor, fast 6/8, 72 bpm, rapid wooden and bark drums, staccato strings, flute trills, bark horn, rhythmic whispered shouts, ambush, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Bark horn, whispered shout, drums burst in]

[Verse]
[Instrumental: staccato strings, rapid wooden drums]

[Build]
[Instrumental: bark horn melody, low strings]

[Verse]
[Instrumental: flute trills, all drums, whispered shouts]

[Breakdown]
[Instrumental: sudden silences, sharp hits]

[Verse]
[Instrumental: staccato strings, rapid wooden drums]
```

**Varianten**
- *Cover aus dem Thema* (bevorzugt).
- *Mit Chor:* Flüsterrufe als Old-Aurethi-Silben „Ca! Li! Reh!“ für Skriptschlachten mit Sylaen.

---

#### 4.7.7 `faction_saelith_compact_theme` – „Silber und Gezeiten“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_saelith_compact_theme` → `assets/audio/music/faction_saelith_compact_theme_intro.ogg`, `assets/audio/music/faction_saelith_compact_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: Some · looped: true · Set F4 |
| Zweck / Spielsituation | Thema des Saelith-Pakts (Türkis/Silber): Handelsrepublik, Händlerfürsten, Ophira Saelith-Varo, Häfen der Saelith-Inseln, Haus der Gezeiten. |
| Genre | Weltgewandtes Fantasy-Orchester mit Saiteninstrumenten |
| Instrumentierung | Hackbrett, gezupfte Kastenzither, Harfe, Streicher-Spiccato (Gezeitenlauf), Handtrommeln, Oboe, tiefe Streicher, Glockenspiel fern |
| Tempo / Takt | 104 BPM, 4/4 |
| Tonart / Motivform | B♭-mixolydisch; Gezeitenlauf (wellenförmige Arpeggien), Glut-Form in Oboe (T. 21–28) als Zeichen, dass auch der Pakt Aureths Erbe begehrt |
| Stimmung | Glänzend, wendig, berechnend, seeluftig |
| Dynamikverlauf | mp (Intro) → mf (T. 5–36) → mp (T. 37–52) |
| Soundcharakter / Mix | Offener Hafen, hell, Hall 2,0 s; Hackbrett und Zither im Stereo verteilt, Oboe mittig |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf A♭ (♭VII) → B♭ in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 9,2 s · Loop 48 Takte = 110,8 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Gezeitenlauf im Hackbrett, Glockenspiel.
- T. 5–20: Zither-Melodie über Spiccato-Wellen, Handtrommeln.
- T. 21–28: Oboe mit Glut-Form.
- T. 29–36: Tutti, tiefe Streicher wie Seegang.
- T. 37–52: Rücknahme auf Hackbrett und Harfe.

**Suno-Style-Prompt**
```text
elegant fantasy maritime orchestral, B-flat mixolydian, 104 bpm, hammered dulcimer waves, plucked zither, harp, spiccato strings, hand drums, oboe, bright and shrewd, instrumental
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: hammered dulcimer wave arpeggios, glockenspiel]

[Verse]
[Instrumental: plucked zither melody, spiccato strings, hand drums]

[Verse]
[Instrumental: oboe melody, harp]

[Build]
[Instrumental: full ensemble, low strings like ocean swell]

[Outro]
[Instrumental: dulcimer and harp, no ending]
```

**Varianten**
- *Handelsrat (Stem):* nur Zither, Harfe und Oboe für Diplomatieszenen mit Ophira.
- *Seefahrt:* Cover mit Männerchor-Summen und tieferem Seegang für Flottenbewegungen.

---

#### 4.7.8 `faction_saelith_compact_battle` – „Söldner und Segel“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_saelith_compact_battle` → `assets/audio/music/faction_saelith_compact_battle_intro.ogg`, `assets/audio/music/faction_saelith_compact_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set F4 |
| Zweck / Spielsituation | Kampf gegen den Saelith-Pakt: Söldnerheere, Küstenlandungen, Hafenkämpfe, Flotte. |
| Genre | Rhythmisches Seefahrts-Kampforchester |
| Instrumentierung | Handtrommeln und Großtrommeln, schnelles Hackbrett, Streicher-Spiccato, Blech (Flotte), Männerchor als Rudergesang, Zither-Tremolo |
| Tempo / Takt | 104 BPM, 4/4 (Sechzehntel-Unterteilung) |
| Tonart / Motivform | B♭-dorisch; Gezeitenlauf beschleunigt, Thema-Melodie im Blech, Rudergesang auf Old Aurethi |
| Stimmung | Kühn, geschäftsmäßig-gefährlich, stürmisch |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → f (T. 45–52) |
| Soundcharakter / Mix | Breit, lebhaft, Hall 1,8 s; Trommeln und Hackbrett vorne, Blech dahinter |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf F (V) → b in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 9,2 s · Loop 48 Takte = 110,8 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Blechsignal, Trommelwirbel.
- T. 5–20: Hackbrett und Spiccato in schnellen Wellen, Handtrommeln.
- T. 21–36: Rudergesang des Männerchors, Blech mit Thema.
- T. 37–44: Breakdown – Trommeln und Zither-Tremolo.
- T. 45–52: Rückführung, Hackbrett.

**Suno-Style-Prompt**
```text
stormy fantasy naval battle orchestral, B-flat dorian, 104 bpm, hand drums and war drums, fast hammered dulcimer, spiccato strings, brass, male rowing chant in invented language
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Brass signal, drum roll]

[Verse]
[Instrumental: fast dulcimer and spiccato strings, hand drums]

[Chorus]
[Male rowing chant, brass]
Tha-len! Heh! Tha-len! Heh!
Tha-len veh kael, nos rhan!

[Breakdown]
[Instrumental: drums and zither tremolo]

[Verse]
[Instrumental: fast dulcimer, spiccato strings]
```

*Textbedeutung:* „Meer! Meer! Meer und Glut, wir steigen!“

**Varianten**
- *Cover aus dem Thema* (bevorzugt).
- *Instrumental:* Rudergesang durch Blech-Staccato ersetzt.

---

#### 4.7.9 `faction_hollowed_theme` – „Die Leere Liturgie“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_hollowed_theme` → `assets/audio/music/faction_hollowed_theme_intro.ogg`, `assets/audio/music/faction_hollowed_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Tension · intro: Some · looped: true · Set F5 |
| Zweck / Spielsituation | Thema der Hohlen (Aschgrau/Fahlgrün), Diener des Hohlen Tiefs: Hohlschreine, Knochentürme, Ausbreitung der Hohlung, Kult der Leeren Liturgie. Tension-Musik, wenn die Hohlen in Sicht sind. |
| Genre | Horror-Ambient-Orchester mit Ritualchor |
| Instrumentierung | Flüsterchor, tiefe Männerstimmen, Glasharmonika-Klang, Streicher sul ponticello, Subbass-Atem (Hohlatem), rückwärts laufende Klänge, ferne verstimmte Glocke |
| Tempo / Takt | 60 BPM, 4/4 (Atemrhythmus: 4 Schläge ein, 4 aus) |
| Tonart / Motivform | G♯-lokrisch; Hohl-Form des Leitmotivs in tiefen Stimmen (T. 13–20), Hohlatem |
| Stimmung | Unheimlich, seelenlos, kalt, rituell |
| Dynamikverlauf | pp (Intro) → p (T. 5–12) → mp (T. 13–20) → pp (T. 21–28) |
| Soundcharakter / Mix | Unnatürlich langer Hall (6,0 s), gefiltert; Subbass im Atemrhythmus, Flüstern im ganzen Panorama, Glasklang hoch und leise |
| Loop-Eignung | Sehr hoch. Loop T. 5–28 (24 Takte). Hohlatem ist zyklisch und überbrückt die Naht. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 16,0 s · Loop 24 Takte = 96,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Rückwärts-Klang, verstimmte Glocke, erstes Einatmen des Clusters.
- T. 5–12: Hohlatem im Subbass, Flüstern „dhum… umbrath“.
- T. 13–20: Tiefe Männerstimmen mit der Hohl-Form, sul ponticello.
- T. 21–28: Nur Atem und Glasklang.

**Suno-Style-Prompt**
```text
horror dark fantasy ritual ambient, G-sharp locrian, 60 bpm, whispering choir, deep male voices, glass harmonica, sul ponticello strings, breathing sub bass, reversed sounds
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Reversed sounds, detuned distant bell, breathing cluster]

[Verse]
[Whispering choir]
dhum... um-brath... dhum...
vae-loth... dhum...

[Chorus]
[Deep male voices, slow]
Um-brath ka mor-ra
Vae-loth dhum, um-brath rhan

[Outro]
[Breathing only, glass harmonica, no ending]
```

**Varianten**
- *Hohlschrein (Stem):* lauterer Chor, wenn der Spieler einen Hohlschrein betritt.
- *Instrumental:* Stimmen durch tiefe Bassklarinetten ersetzt.

---

#### 4.7.10 `faction_hollowed_battle` – „Die Toten marschieren“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_hollowed_battle` → `assets/audio/music/faction_hollowed_battle_intro.ogg`, `assets/audio/music/faction_hollowed_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set F5 |
| Zweck / Spielsituation | Kampf gegen die Hohlen: Hohlknechte, Knochenschützen, Grabkolosse; Combat-Musik, wenn die Hohlen der Gegner sind (ab Kampagne nach M01, in M01 selbst gilt `battle_small`). |
| Genre | Untoten-Kampforchester, Horror |
| Instrumentierung | Kontrabässe und Celli col legno, Knochenklappern (Holzperkussion), Großtrommeln, tiefes Blech, rhythmischer Flüsterchor, Streicher-Glissandi, Subbass |
| Tempo / Takt | 120 BPM, 4/4 |
| Tonart / Motivform | G♯-phrygisch (A); Hohl-Form in tiefem Blech (T. 21–36), Kronenschlag als Knochenklappern |
| Stimmung | Unaufhaltsam, morbide, erstickend |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mp (T. 37–44) → ff (T. 45–56) → f (T. 57–60) |
| Soundcharakter / Mix | Dunkel, eng, Hall 3,0 s; Holzklappern trocken vorne, Blech tief, Flüstern breit |
| Loop-Eignung | Hoch. Loop T. 5–60 (56 Takte). Naht: T. 60 auf A (♭II) → G♯ in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 8,0 s · Loop 56 Takte = 112,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Rückwärts-Riser, Schlag, Flüsterchor „dhum!“.
- T. 5–20: Col-legno-Ostinato, Knochenklappern im Kronenschlag, Großtrommeln.
- T. 21–36: Tiefes Blech mit Hohl-Form, Flüsterchor rhythmisch.
- T. 37–44: Breakdown – nur Klappern und Glissandi.
- T. 45–56: Höhepunkt, Blech und Chor.
- T. 57–60: Rückführung.

**Suno-Style-Prompt**
```text
undead dark fantasy battle, G-sharp phrygian, 120 bpm, col legno strings, bone rattles, war drums 3-3-2, low brass, rhythmic whispering choir, string glissandi, sub bass, relentless
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Reversed riser, hit, whispered shout]
Dhum!

[Verse]
[Instrumental: col legno ostinato, bone rattles, war drums]

[Chorus]
[Rhythmic whispering choir, low brass]
Dhum, dhum, um-brath rhan
Mor-ra ka, mor-ra ka

[Breakdown]
[Instrumental: bone rattles, string glissandi]

[Climax]
[Full choir and brass]
Um-brath ka mor-ra!

[Verse]
[Instrumental: col legno ostinato, war drums]
```

**Varianten**
- *Cover aus dem Thema* (bevorzugt), trotz Tempo 1:2 – Thema-Take im Cover mit 120 BPM angeben.
- *Grabkoloss-Welle (Stem):* mit verstärktem Subbass, wenn Grabkolosse dabei sind.

---

#### 4.7.11 `faction_ashen_legion_theme` – „Die Ewige Legion“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_ashen_legion_theme` → `assets/audio/music/faction_ashen_legion_theme_intro.ogg`, `assets/audio/music/faction_ashen_legion_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Tension · intro: Some · looped: true · Set F6 |
| Zweck / Spielsituation | Thema der Aschenlegion (Schwarz/Glutorange), verfluchte Ritter Aureths, ewige Legion des Hohlen Königs: Aufmarsch, Legionsfestungen, Aurethwüste. |
| Genre | Imperial-dunkler Marsch |
| Instrumentierung | Tiefes Blech, Amboss auf Zählzeit 4, Ketten, Männerchor (Old Aurethi), verfremdete Fanfaren, Großtrommeln, Kontrabässe |
| Tempo / Takt | 88 BPM, 4/4 |
| Tonart / Motivform | C-phrygisch (D♭); Legionsmarsch: Kronenschlag als Marsch, Hohl-Form im Blech (T. 13–20), verfremdete Krone-Form als „verlorene Herrlichkeit“ (T. 29–32) |
| Stimmung | Unerbittlich, stolz-verdammt, imperial |
| Dynamikverlauf | mf (Intro) → f (T. 5–28) → mf (T. 29–32) → f (T. 33–44) |
| Soundcharakter / Mix | Riesige leere Halle, Hall 3,2 s; Blech tief und breit, Amboss klar, Chor schwer |
| Loop-Eignung | Hoch. Loop T. 5–44 (40 Takte). Naht: T. 44 auf D♭ (♭II) → c in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 10,9 s · Loop 40 Takte = 109,1 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Ketten, Amboss, ferne verfremdete Fanfare.
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

---

#### 4.7.12 `faction_ashen_legion_battle` – „Schwarze Glut“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `faction_ashen_legion_battle` → `assets/audio/music/faction_ashen_legion_battle_intro.ogg`, `assets/audio/music/faction_ashen_legion_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Combat · intro: Some · looped: true · Set F6 |
| Zweck / Spielsituation | Kampf gegen die Aschenlegion: schwere verfluchte Ritter, Phalanx, Glutwaffen. |
| Genre | Brachiales Dark-Fantasy-Kampforchester |
| Instrumentierung | Tiefes Blech, Amboss in Sechzehnteln, Männerchor-Staccato, Streicher-Ostinato, Pauken, Großtrommeln, Ketten, verfremdete Fanfaren |
| Tempo / Takt | 88 BPM, 4/4 (doppelte Unterteilung, Sechzehntel-Ostinati) |
| Tonart / Motivform | C-phrygisch; Legionsmarsch beschleunigt, Hohl-Form augmentiert (T. 21–36) |
| Stimmung | Erdrückend, gnadenlos, glühend |
| Dynamikverlauf | f (Intro) → f (T. 5–20) → ff (T. 21–36) → mf (T. 37–44) → ff (T. 45–52) |
| Soundcharakter / Mix | Massiv, Hall 2,6 s; Amboss und Pauken vorne, Chor breit, Blech mit leichter Sättigung |
| Loop-Eignung | Hoch. Loop T. 5–52 (48 Takte). Naht: T. 52 auf G (V, mit D♭ als Färbung) → c in T. 5. |
| Ziellänge | Suno-Rohtake 3:00 · Intro 4 Takte = 10,9 s · Loop 48 Takte = 130,9 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Fanfare bricht ab, Amboss-Salve, Chor „Aureh!“.
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

---
### 4.8 Ergebnis-Bildschirme und Zeitalterwechsel

#### 4.8.1 `victory_theme` – „Die Glut lebt“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `victory_theme` → `assets/audio/music/victory_theme_intro.ogg`, `assets/audio/music/victory_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Menu · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Sieg-Bildschirm und Missionsauswertung nach `victory_sting` (R5); läuft, solange Statistiken und Belohnungen angezeigt werden. |
| Genre | Feierliches Orchester mit Chor, getragen |
| Instrumentierung | Hörner, Streicher, gemischter Chor (Old Aurethi), Harfe, Laute, Glocken, Pauken sanft |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-Dur (Intro, Krone-Form) → D-mixolydisch (Loop, Glut-Form) |
| Stimmung | Erleichtert, dankbar, würdevoll; der Preis des Sieges schwingt mit |
| Dynamikverlauf | f (Intro-Höhepunkt T. 5–8) → mp (Loop T. 9–24) → mf (T. 25–32) |
| Soundcharakter / Mix | Hell, warm, Hall 2,8 s; Chor breit, Laute nah im Loop (unaufdringlich, da Bildschirmtexte gelesen werden) |
| Loop-Eignung | Hoch. Loop T. 9–32 (24 Takte). Naht: T. 32 auf C (♭VII) → D in T. 9. |
| Ziellänge | Suno-Rohtake 2:30–3:00 · Intro 8 Takte = 26,7 s · Loop 24 Takte = 80,0 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Hörner und Chor mit der vollständigen Krone-Form, Glocken, Höhepunkt T. 5–8, dann Abklingen.
- T. 9–16: Laute und Harfe mit dem Glut-Fragment, Streicher legato.
- T. 17–24: Chor summt pp, Hörner fern.
- T. 25–32: Streicher mit Glut-Form, Rückleitung über C-Dur.

**Suno-Style-Prompt**
```text
warm triumphant fantasy orchestral, D major to mixolydian, 72 bpm, horns, choir in invented language, harp, lute, bells, gentle timpani, relieved and noble, bittersweet
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Horns and full choir, bells]
Ith-ra ar! Kael ar!
Au-reh rha-na, el-dra veth!

[Verse]
[Instrumental: lute and harp, legato strings]

[Verse]
[Choir humming softly, distant horns]
mmm... aah... mmm...

[Verse]
[Instrumental: strings melody, warm]

[Outro]
[Instrumental: lute and harp, no ending]
```

**Varianten**
- *Kampagnenfinale:* Cover mit größerem Chor für das Ende von Zeitalter VII (vor `credits_theme`).
- *Instrumental:* Chor durch Hörner ersetzt (Gefecht).

---

#### 4.8.2 `defeat_theme` – „Asche im Wind“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `defeat_theme` → `assets/audio/music/defeat_theme_intro.ogg`, `assets/audio/music/defeat_theme_loop.ogg` |
| Manifest (Vorschlag) | intensity: Menu · intro: Some · looped: true · Set A |
| Zweck / Spielsituation | Niederlage-Bildschirm nach `defeat_sting` (R5) mit Optionen „Erneut versuchen“ / „Hauptmenü“. |
| Genre | Elegisches Kammerorchester |
| Instrumentierung | Solo-Cello, Harfe, tiefe Streicher, gehauchte Frauenstimme, Altflöte, ferne tiefe Glocke |
| Tempo / Takt | 72 BPM, 4/4 (sehr ruhig, halbe Noten) |
| Tonart / Motivform | D-äolisch; Hohl-Andeutung im Intro, Asche-Form im Cello (T. 5–12), Glut-Fragment in Altflöte am Loop-Ende (T. 25–28) als Einladung zum neuen Versuch |
| Stimmung | Trauernd, still, aber nicht hoffnungslos |
| Dynamikverlauf | p (Intro) → mp (T. 5–20) → p (T. 21–28) |
| Soundcharakter / Mix | Dunkel-warm, Hall 3,2 s; Cello nah, Stimme fern |
| Loop-Eignung | Hoch. Loop T. 5–28 (24 Takte). Naht: T. 28 auf A (v) → d in T. 5. |
| Ziellänge | Suno-Rohtake 2:30 · Intro 4 Takte = 13,3 s · Loop 24 Takte = 80,0 s |

**Arrangement**
- T. 1–4 (Intro-Datei): Tiefe Glocke, Streicher mit Hohl-Andeutung (A♭ → A).
- T. 5–12: Solo-Cello mit Asche-Form, Harfe.
- T. 13–20: Gehauchte Frauenstimme „cindra… liraen“, tiefe Streicher.
- T. 21–24: Harfe allein.
- T. 25–28: Altflöte mit Glut-Fragment „Die Krone“ (große Sexte), Rückleitung.

**Suno-Style-Prompt**
```text
elegiac dark fantasy chamber orchestral, D minor, 72 bpm, solo cello, harp, low strings, breathy distant female voice, alto flute, deep distant bell, mournful yet quietly hopeful
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Deep distant bell, low strings]

[Verse]
[Instrumental: solo cello melody, harp]

[Verse]
[Breathy distant female voice]
cin-dra... li-ra-en...
cin-dra... sa do-mae...

[Interlude]
[Instrumental: harp alone]

[Outro]
[Instrumental: alto flute, hopeful phrase, no ending]
```

**Varianten**
- *Kampagnen-Game-Over:* Stem ohne Altflöten-Schluss (endgültiger Ton) für Niederlagen, die die Kampagne beenden.
- *Gefecht:* instrumental ohne Stimme.

---

#### 4.8.3 `age_transition_sting` – „Ein neues Zeitalter“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `age_transition_sting` → `assets/audio/music/age_transition_sting.ogg` |
| Manifest (Vorschlag) | intensity: Sting · intro: None · looped: false · Set B |
| Zweck / Spielsituation | Übergang zwischen den Kampagnenzeitaltern I *The Last Settlement* bis VII *The Last Crown*, über der Weltkarte; der Erzähler (Chronist) spricht darüber – Musik lässt Platz. |
| Genre | Orchestraler Kapitel-Sting |
| Instrumentierung | Große Glocke, Chor, Streicher, Hörner, Harfe, Pauke |
| Tempo / Takt | 80 BPM, 4/4 (6 Takte + Ausklang) |
| Tonart / Motivform | G-dorisch; Leitmotiv Asche-Form (T. 1–2) wandelt sich zur Glut-Form (T. 3–4), Schluss auf leerer Quinte G – D |
| Stimmung | Erhaben, schicksalhaft, Seite einer Chronik wird umgeschlagen |
| Dynamikverlauf | mp (Glocke) → f (T. 3–4) → p (T. 5–6) → Ausklang |
| Soundcharakter / Mix | Breit, Hall 3,5 s; Mitten bei 1–3 kHz zurückhaltend für die Erzählerstimme |
| Loop-Eignung | Keine (linear). |
| Ziellänge | ca. 18 s Musik + 3–4 s Ausklang = ca. 22 s |

**Arrangement**
- T. 1–2: Glockenschlag, Streicher spielen die Asche-Form.
- T. 3–4: Hörner und Chor wandeln sie zur Glut-Form, Paukenwirbel.
- T. 5–6: Harfe und Chor pp, leere Quinte G – D.
- Danach: Hallausklang.

**Suno-Style-Prompt**
```text
majestic dark fantasy orchestral chapter sting, G dorian, 80 bpm, great bell, strings, horns, choir in invented language, harp, timpani roll, fateful, short with clear ending
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Great bell, strings melody]

[Climax]
[Horns and choir, timpani roll]
El-dra rhan, li-ra-en ar

[Outro]
[Harp and soft choir, open fifth]

[End]
```

*Textbedeutung:* „Die Welt steigt, ewige Erinnerung.“

**Varianten**
- *Dunkle Zeitalter (VI und VII):* Cover mit Hohl-Form statt Glut-Form (G-phrygisch, A♭).
- *Ohne Chor:* falls der Erzählertext sehr dicht ist.

---

### 4.9 Traurige Storymomente und Enthüllungen

Story-Tracks sind linear und werden per Skript gestartet (R6/R8). Taktangaben bei Rubato beziehen sich auf
das nominelle Tempo; kleine Tempoabweichungen sind erwünscht.

#### 4.9.1 `story_lament` – „Sereth“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `story_lament` → `assets/audio/music/story_lament.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: false · Set X |
| Zweck / Spielsituation | Trauer und Verlust: Tod wichtiger Figuren, Fall einer Siedlung, Abschied; auch nach dem Sieg über den Hohlen König (Mitleid mit Varkhan Aurelith). |
| Genre | Orchesterlamento |
| Instrumentierung | Solo-Cello, Frauenstimme solo (Old Aurethi), Streicher, Harfe, Altflöte, ferne Glocke |
| Tempo / Takt | 60 BPM, 4/4, rubato |
| Tonart / Motivform | D-äolisch; Asche-Form im Cello, gedehnt; am Ende Glut-Fragment in der Stimme |
| Stimmung | Zutiefst traurig, zart, würdevoll |
| Dynamikverlauf | pp → mf (T. 17–24) → pp (T. 25–30) |
| Soundcharakter / Mix | Intim, Hall 3,0 s; Cello und Stimme nah, Streicher als weicher Teppich |
| Loop-Eignung | Keine (linear). Bei längeren Szenen kann T. 9–16 im Engine wiederholt werden (Taktgrenzen sauber). |
| Ziellänge | ca. 2:00 (30 Takte = 120 s) inkl. Ausklang |

**Arrangement**
- T. 1–8: Solo-Cello allein mit Asche-Form, sehr langsam.
- T. 9–16: Streicher treten hinzu, Harfe.
- T. 17–24: Frauenstimme „Sereth“, Höhepunkt mf.
- T. 25–28: Altflöte und Cello, Glut-Fragment in der Stimme.
- T. 29–30: Einzelner Glockenton, Ausklang.

**Suno-Style-Prompt**
```text
heartbreaking dark fantasy orchestral lament, D minor, 60 bpm, rubato, solo cello, solo female voice in invented language, soft strings, harp, alto flute, distant bell
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: solo cello alone, very slow]

[Verse]
[Instrumental: soft strings join, harp]

[Verse]
[Solo female voice]
Se-reth, se-reth, li-ra-en
Mor-ra tor, sol-vae vael

[Climax]
[Solo female voice, strings swell]
Kael... ka esk...

[Outro]
[Alto flute and cello, single distant bell]

[End]
```

*Textbedeutung:* „Klage, Klage, Erinnerung. Pforte des Todes, Ende der Sonne. – Glut … fällt nicht.“

**Varianten**
- *Instrumental:* Stimme durch Solo-Bratsche ersetzt (für weniger gewichtige Verluste).
- *Kurzfassung:* T. 1–8 + T. 29–30 (ca. 40 s) aus demselben Take für kleine Szenen.

---

#### 4.9.2 `story_ashfall_memory` – „Der Tag, an dem die Sonne fiel“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `story_ashfall_memory` → `assets/audio/music/story_ashfall_memory.ogg` |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: false · Set X |
| Zweck / Spielsituation | Rückblende auf Aureth und den Aschenfall (Jahr 0): Varkhan Aurelith vollzieht das Ritual in Solmere, der Schleier öffnet sich, das Reich stürzt. |
| Genre | Kinematische Orchestererzählung |
| Instrumentierung | Naturtrompeten, Orgel, großer Chor, Glocken, Streicher; ab der Mitte Streichercluster, Subbass, Flüsterchor, Glasharmonika-Klang |
| Tempo / Takt | 66 BPM, 4/4, rubato in den Übergängen |
| Tonart / Motivform | D-Dur (imperiale Fassung der Glut-Form) → D-phrygisch mit Hohl-Form → Stille |
| Stimmung | Pracht, Hybris, Entsetzen, Leere |
| Dynamikverlauf | f (T. 1–12) → mf (T. 13–20) → ff-Zusammenbruch (T. 21–32) → pp (T. 33–41) |
| Soundcharakter / Mix | Erst Kathedrale (Hall 3,5 s), hell und golden; ab T. 21 gefiltert, unnatürlicher Hall, Subbass; Ende fast still |
| Loop-Eignung | Keine (linear). |
| Ziellänge | ca. 2:30 (41 Takte = 149 s) |

**Arrangement**
- T. 1–12: Imperiale Pracht – Trompeten, Orgel und Chor mit Glut-Form in Dur-Harmonisierung, Glocken.
- T. 13–20: Das Ritual – Chor wird zum Flüstern, Orgel-Bordun, Glasklang.
- T. 21–32: Der Schleier reißt – Streichercluster, Hohl-Form in Posaunen, Subbass, Chor schreit „vaeloth“.
- T. 33–41: Asche – einzelnes Solo-Cello mit dem Fall (4 – ♭3 – ♭2 – ♭7 – 1), dann Stille.

**Suno-Style-Prompt**
```text
cinematic dark fantasy orchestral story, D major turning to D phrygian, 66 bpm, imperial trumpets, organ, grand choir, bells, then collapse, string clusters, whispers, solo cello
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Imperial trumpets, organ, bells, grand choir]
Sol-vae, sol-vae, au-reh ar!
Varkh-an, varkh ar, ith-ra ar!

[Verse]
[Choir fades to whispers, organ drone, glass harmonica]
um-brath... tor... vae-loth...

[Climax]
[String clusters, low trombones, sub bass]
Vae-loth! Vae-loth!
Sol-vae es-keth!

[Outro]
[Solo cello, falling line, then silence]

[End]
```

*Textbedeutung:* „Sonne, Sonne, ewige Krone! Varkhan, ewiger König, ewiges Licht! – Tiefe … Pforte … Schleier … – Schleier! Die Sonne fiel!“

**Varianten**
- *Ohne Rückblende-Bild:* Stem nur T. 21–41 als „Schleierriss“ für spätere Hohlungs-Ereignisse.
- *Instrumental:* Chor durch Orgel ersetzt.

---

#### 4.9.3 `story_revelation` – „Die Stimme aus dem Tief“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `story_revelation` → `assets/audio/music/story_revelation.ogg` |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: false · Set F5 |
| Zweck / Spielsituation | Enthüllungen und Wendepunkte: Die Stimme aus dem Tief spricht, Verrat (z. B. Kassian Vale), Erkenntnis über die wahre Natur des Hohlen Tiefs. |
| Genre | Psychologischer Horror-Score |
| Instrumentierung | Bassklarinette und Kontrabass (Tiefenstimme: Ton, der sich um einen Viertelton verbiegt), Streichercluster, verfremdete Stimme, Glasharmonika-Klang, Subbass, ein einzelner Glockenschlag |
| Tempo / Takt | 60 BPM, 4/4 |
| Tonart / Motivform | G♯-lokrisch; Tiefenstimme, Hohl-Form in der verfremdeten Stimme (T. 13–20) |
| Stimmung | Beklemmend, schwindelerregend, entlarvend |
| Dynamikverlauf | pp → f (T. 21–22, Schlag) → pp (T. 23–26) |
| Soundcharakter / Mix | Eng, dann plötzlich riesig; Hall 5,0 s auf der Stimme; Subbass langsam anschwellend |
| Loop-Eignung | Keine (linear). |
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

---

### 4.10 Geheimnisvolle Orte

Tempolose Ambient-Loops (nominell 60 BPM für das DAW-Raster). Dürfen nach R6 mit 4 s Crossfade
überblendet werden. Loop-Länge 32 Takte = 128 s; Loop-Punkte trotzdem auf Taktgrenzen setzen, damit
die Schnittlogik einheitlich bleibt.

#### 4.10.1 `mystery_ruins` – „Echos von Aureth“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `mystery_ruins` → `assets/audio/music/mystery_ruins_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set A |
| Zweck / Spielsituation | Ruinen Aureths: Aurethwüste, Ruinen von Solmere, auch der Ort des Hohlschreins von Vael Dorn vor dessen Entdeckung; Aschglas-Vorkommen. Passt zu `ambience_ruins_loop`. |
| Genre | Dunkles Ambient mit orchestralen Resten |
| Instrumentierung | Tiefer Bordun D, ferne Frauenstimme, verhallte Glocke, Streicher-Flageoletts, Kristallklänge (Aschglas), Wind-Textur, sehr leises Solo-Cello |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | D-äolisch; Asche-Form extrem gedehnt in Cello (T. 9–16) und Stimme (T. 21–28) |
| Stimmung | Verlassen, ehrfürchtig, rätselhaft |
| Dynamikverlauf | pp konstant, leichte Wellen |
| Soundcharakter / Mix | Sehr weit, Hall 6,0 s, viel Raum; Kristallklänge hoch und verstreut |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte). Bordun über die Naht, keine Glocke in T. 32 oder T. 1. |
| Ziellänge | Suno-Rohtake 3:00 · Loop 32 Takte = 128,0 s |

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

---

#### 4.10.2 `mystery_veilscar` – „Wo die Welt dünn wird“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `mystery_veilscar` → `assets/audio/music/mystery_veilscar_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Tension · intro: None · looped: true · Set F5 |
| Zweck / Spielsituation | Die Schleiernarbe (Veilscar) im tiefen Süden: Realität instabil, vom Hohlen Tief beherrscht; auch Orte, an denen der Schleier dünn ist. |
| Genre | Experimentelles Horror-Ambient |
| Instrumentierung | Glasharmonika-Klang, Waterphone-artige Klänge, rückwärts laufende Streicher, sich verbiegende Töne (Tiefenstimme), Cluster-Chor sehr leise, Subbass-Atem |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | Cluster um G♯; Hohlatem, Fragmente der Hohl-Form (T. 17–24) rückwärts |
| Stimmung | Desorientierend, falsch, unwirklich |
| Dynamikverlauf | pp → p (T. 17–24) → pp |
| Soundcharakter / Mix | Unnatürlich, Hall 8,0 s, Klänge wandern im Panorama; tiefe Frequenzen pulsieren langsam |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte); Atemrhythmus überbrückt die Naht. |
| Ziellänge | Suno-Rohtake 3:00 · Loop 32 Takte = 128,0 s |

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

---

#### 4.10.3 `mystery_old_gods` – „Die Schweigenden“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `mystery_old_gods` → `assets/audio/music/mystery_old_gods_loop.ogg` (kein Intro) |
| Manifest (Vorschlag) | intensity: Calm · intro: None · looped: true · Set X |
| Zweck / Spielsituation | Schreine und Relikte der Alten Götter Ithren, Morvaine, Calyx, Thal und Oruun, die seit dem Aschenfall verstummt sind. |
| Genre | Ätherisches Sakral-Ambient |
| Instrumentierung | Je ein Klang pro Gott: hohe Streicher (Ithren, Licht), tiefe Frauenstimme (Morvaine, Übergang), Holzflöte (Calyx, Wachstum), Harfenwellen (Thal, Meer), tiefer Steinklang-Bordun (Oruun); ferner Chor |
| Tempo / Takt | tempolos, nominell 60 BPM, 4/4 |
| Tonart / Motivform | E-lydisch (♯4 als Zeichen des Unerreichbaren); kein Leitmotiv – nur ein einzelner aufsteigender Quintschritt (Aufschwung) als Erinnerung an die Krone (T. 25) |
| Stimmung | Erhaben, fern, still, wehmütig |
| Dynamikverlauf | pp → p (T. 17–28) → pp |
| Soundcharakter / Mix | Riesiger heller Raum, Hall 7,0 s; Klänge nacheinander, kaum gleichzeitig |
| Loop-Eignung | Sehr hoch. Ganze Datei ist der Loop (32 Takte); Bordun E über die Naht. |
| Ziellänge | Suno-Rohtake 3:00 · Loop 32 Takte = 128,0 s |

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

---

### 4.11 Endgame und Credits

#### 4.11.1 `endgame_last_crown` – „Die Letzte Krone“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `endgame_last_crown` → `assets/audio/music/endgame_last_crown_intro.ogg`, `assets/audio/music/endgame_last_crown_loop.ogg` |
| Manifest (Vorschlag) | intensity: Tension · intro: Some · looped: true · Set E |
| Zweck / Spielsituation | Zeitalter VII *The Last Crown*: Weltkarte und Aufbau im Endgame; Valemark, Verbündete und Feinde ringen um die Aschenkrone, Solmere ruft. Ersetzt Weltkarten- und Calm-Musik. |
| Genre | Monumentales Dark-Fantasy-Orchester |
| Instrumentierung | Volles Orchester, Chor pp–mf, Glocken, Hörner; Zitate der Fraktionskennungen (Drehleier – Skarnheim, Holzflöte – Dornenbund, Hackbrett – Saelith, Glutruf – Glutkrone) |
| Tempo / Takt | 66 BPM, 4/4 |
| Tonart / Motivform | D-äolisch → D-dorisch; Leitmotiv in Asche-, Glut- und Hohl-Form (Krone-Form bleibt dem Finale vorbehalten) |
| Stimmung | Schicksalsschwer, gesammelt, alles steht auf dem Spiel |
| Dynamikverlauf | p (Intro) → mf (T. 9–24) → f (T. 25–32) → mp (T. 33–48) |
| Soundcharakter / Mix | Groß und tief, Hall 3,5 s; Fraktionszitate klar erkennbar im Panorama verteilt |
| Loop-Eignung | Hoch. Loop T. 9–48 (40 Takte). Naht: T. 48 auf A (v) → d in T. 9. |
| Ziellänge | Suno-Rohtake 3:30–4:00 · Intro 8 Takte = 29,1 s · Loop 40 Takte = 145,5 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Glocke, Solo-Cello mit Asche-Form, Chor pp.
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

---

#### 4.11.2 `endgame_final_battle` – „Vor den Toren Solmeres“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `endgame_final_battle` → `assets/audio/music/endgame_final_battle_intro.ogg`, `assets/audio/music/endgame_final_battle_loop.ogg` |
| Manifest (Vorschlag) | intensity: Battle · intro: Some · looped: true · Set E |
| Zweck / Spielsituation | Letzte Schlacht vor den Toren Solmeres: Vereinte Heere gegen Aschenlegion und Hohle. Mündet in `boss_hollow_king` (R4, gleiche Tonika D, Tempo 132). |
| Genre | Episches Finale, Orchester und großer Chor |
| Instrumentierung | Volles Orchester, großer Chor, alle Fraktionskennungen (Ahnenschritt-Trommeln, Wurzelreigen-Flöten, Gezeitenlauf-Hackbrett, Glutruf-Hörner) gegen Legionsmarsch-Amboss und Hohlatem-Cluster, Glocken, Orgel |
| Tempo / Takt | 132 BPM, 4/4 |
| Tonart / Motivform | D-phrygisch → D-dorisch; Hohl-Form gegen Glut-Form, im Höhepunkt (T. 57–64) einmal die Krone-Form im vollen Tutti |
| Stimmung | Alles oder nichts, vereint, glühend |
| Dynamikverlauf | f (Intro) → f (T. 9–24) → ff (T. 25–48) → mf (T. 49–56) → fff (T. 57–64) → f (T. 65–72) |
| Soundcharakter / Mix | Maximal groß, Hall 3,0 s; Trommeln und Chor tragen, Fraktionskennungen kurz und klar hervorgehoben |
| Loop-Eignung | Hoch. Loop T. 9–72 (64 Takte). Naht: T. 72 auf E♭ (♭II) → D in T. 9. |
| Ziellänge | Suno-Rohtake 3:30 · Intro 8 Takte = 14,5 s · Loop 64 Takte = 116,4 s |

**Arrangement**
- T. 1–8 (Intro-Datei): Glutruf, Antwort der verfremdeten Legionsfanfare, Trommeln aller Fraktionen setzen ein.
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

---

#### 4.11.3 `credits_theme` – „Was aus der Asche wächst“

| Feld | Inhalt |
|---|---|
| ID / Ziel-Dateien | `credits_theme` → `assets/audio/music/credits_theme.ogg` |
| Manifest (Vorschlag) | intensity: Menu · intro: None · looped: false · Set A |
| Zweck / Spielsituation | Abspann nach dem Ende der Kampagne; musikalische Zusammenfassung des Spiels. |
| Genre | Orchestrale Suite mit Chor |
| Instrumentierung | Solo-Cello, volles Orchester, großer Chor, Frauenstimme solo, alle Fraktionsinstrumente (Drehleier, Holzflöte, Hackbrett, Laute), Glocken |
| Tempo / Takt | 72 BPM, 4/4 |
| Tonart / Motivform | D-dorisch → D-Dur; Asche-Form (Beginn) → Fraktionskennungen → Krone-Form (Höhepunkt) → Glut-Form (Schluss) |
| Stimmung | Bittersüß, versöhnlich, hoffnungsvoll |
| Dynamikverlauf | p (T. 1–16) → mf (T. 17–48) → ff (T. 49–72) → p (T. 73–96) |
| Soundcharakter / Mix | Konzertsaal, Hall 3,0 s; natürlich-dynamisch, Limiter sehr sanft |
| Loop-Eignung | Keine (linear). |
| Ziellänge | ca. 5:20 (96 Takte = 320 s); Produktion in 2–3 Teilen per Extend |

**Arrangement**
- T. 1–16: Solo-Cello mit der Asche-Form wie im Hauptmenü, Harfe.
- T. 17–48: Fraktionssuite, je 8 Takte: Laute und Glutruf (Glutkrone), Drehleier (Skarnheim), Holzflöte (Dornenbund), Hackbrett (Saelith).
- T. 49–72: Großer Chor und Orchester mit der Krone-Form, Glocken, Höhepunkt.
- T. 73–88: Frauenstimme mit der Glut-Form, Streicher.
- T. 89–96: Solo-Cello allein, Schluss auf D-Dur.

**Suno-Style-Prompt**
```text
bittersweet fantasy orchestral suite, D dorian to D major, 72 bpm, solo cello, grand choir in invented language, hurdy gurdy, wooden flute, dulcimer, lute, bells, hopeful finale
```

**Lyrics-/Struktur-Feld**
```text
[Intro]
[Instrumental: solo cello, harp, quiet]

[Verse]
[Instrumental: lute and horn call]

[Verse]
[Instrumental: hurdy gurdy, then wooden flute, then hammered dulcimer]

[Climax]
[Grand choir and full orchestra, bells, major key]
Au-reh rha-na, ith-ra ar
Kael ar, el-dra veth
Cin-dra vael, ca-li-reh rhan

[Verse]
[Solo female voice, strings]
Li-ra-en sa do-mae
Kael rha-na, nos veth

[Outro]
[Instrumental: solo cello alone, ending in major]

[End]
```

*Textbedeutung:* „Krone, erhebe dich, ewiges Licht. Ewige Glut, die Welt schwört. Ende der Asche, das Wachstum steigt. – Erinnerung an die Heimat. Glut, erhebe dich, wir schwören.“

**Varianten**
- *Produktion:* Teil 1 (T. 1–48) generieren, Teil 2 per Extend ab T. 49; Übergang in der DAW auf einer Taktgrenze verbinden.
- *Kurzfassung:* T. 1–16 + T. 49–72 + T. 89–96 (ca. 2:40) für einen kurzen Abspann im Gefecht.

---
## 5. Produktions-Checkliste

### 5.1 Vor Produktionsbeginn (einmalig)

- [ ] Aktuelle Suno-Version, verfügbare Modelle, Maximallänge, Exportformate (WAV/MP3), Extend, Cover, Audio-Upload, Personas, Stems und Abschnittsbearbeitung geprüft und im Prompt-Log notiert.
- [ ] Nutzungsrechte des gebuchten Suno-Plans für kommerzielle Spiele geprüft und dokumentiert.
- [ ] Motiv-Seed „Die Aschenkrone“ in D (72 BPM) und G (80 BPM) selbst eingespielt (8–16 Takte, Asche- und Glut-Form).
- [ ] DAW-Vorlage angelegt: 48 kHz, Tempo-Grid, Loudness-Meter (LUFS integriert, True Peak), Limiter, OGG-Export q6.
- [ ] Prompt-Log angelegt (Spalten: Track-ID, Datum, Modellversion, Style-Prompt, Lyrics-Feld, Take-Kennung, Funktion, Auswahlbegründung, Loop-Punkte in Takten und Samples).
- [ ] Old-Aurethi-Glossar (1.6) und Verbot-Liste (1.7) für alle Beteiligten verfügbar.

### 5.2 Pro Track

- [ ] Style-Prompt und Lyrics-/Struktur-Feld unverändert aus der Track-Bibel übernommen; keine Künstlernamen.
- [ ] 6–10 Takes erzeugt; Vorauswahl nach Kriterien aus 2.5.
- [ ] Gegenhören auf Ähnlichkeit zu bekannter Musik; im Zweifel verwerfen.
- [ ] Kontur des Leitmotivs bzw. Fraktionskennung hörbar; Tonart passt zum Set (3.2).
- [ ] Bei abgeleiteten Tracks: Cover aus dem vorgesehenen Ausgangs-Take erzeugt (z. B. Kampfvariante aus Thema).
- [ ] Download in höchster Qualität, ggf. Konvertierung auf 48 kHz.
- [ ] Tempo auf Ziel-BPM fixiert (Abweichung ≤ 3 % gestreckt, sonst Take verworfen).
- [ ] Loop-Punkte auf Taktgrenzen, Loop-Länge = geplante Taktzahl, Länge in Samples geprüft.
- [ ] Tail-Wrap und Naht-Crossfade 20–50 ms gesetzt; 5× Loop-Test und Intro→Loop-Test ohne Klick.
- [ ] Intro endet exakt am Loop-Start, ohne Fade und ohne Stille.
- [ ] Loudness −14 LUFS integriert (Stings: Short-Term-Maximum ca. −12 LUFS), True Peak ≤ −1 dBTP; Intro und Loop innerhalb 1 LU.
- [ ] Export als OGG Vorbis, 48 kHz, Stereo, q6; Dateiname exakt nach Kanon (klein, `snake_case`, `_intro`/`_loop` bzw. ohne Suffix bei linearen Tracks).
- [ ] Ablage in `assets/audio/music/`; keine zusätzlichen Dateien (z. B. kein `build_tension_intro.ogg`).
- [ ] Arbeitsmaster (WAV 24 Bit) und Prompt-Log-Eintrag archiviert.

### 5.3 Integrations- und Abnahmetest

- [ ] Asset-Scanner meldet keine Konventionsverstöße.
- [ ] Alle 13 Pflichtdateien des Manifests vorhanden (Liste in 2.10).
- [ ] Gapless-Test der OGG-Loops im Engine bestanden (sonst betroffene Loops als `.wav`).
- [ ] Übergänge im Spiel getestet: R1 (build_calm ↔ build_tension, battle_small ↔ battle_large), R2 (Calm → Combat mit Intro), R3 (Hysterese 15 s), R4 (Boss), R5 (Stings), R6 (Set-Wechsel), R8 (Ducking bei Dialog und Erzähler).
- [ ] Mindestverweildauern verhindern Flattern zwischen Stufen.
- [ ] Lautheitsbalance gegen SFX und Stimmen im Spiel geprüft (Engine-Offsets aus 2.9).
- [ ] Langzeittest: 60 Minuten Aufbauphase ohne störende Wiederholung (build_calm, build_night, economy_prosperity).
- [ ] Vorgeschlagene Manifest-Einträge (3.4) für jeden fertigen Zukunfts-Track an die Audio-Integration übergeben.

---

## 6. Prioritätenliste

### P0 – Vertical Slice (sofort, blockiert die Mission „Die letzte Siedlung“)

Reihenfolge innerhalb von P0 (Abhängigkeiten beachten):

1. `build_calm` – längste Hörzeit, legt Klangwelt von Set A fest.
2. `build_tension` – direkter Crossfade-Partner von `build_calm`; gleiche Tonika und BPM prüfen.
3. `battle_small` – erster Kampf in M01.
4. `battle_large` – Crossfade-Partner von `battle_small` (beide 144 BPM, D).
5. `boss_pale_reeve` – Höhepunkt von M01.
6. `victory_sting` und `defeat_sting` – Missionsende.
7. `main_menu` – zuletzt in P0, weil es von den fertigen Klangfarben der anderen Tracks profitiert.

### P1 – Kampagne Zeitalter I–II und Kernsysteme

`build_night`, `explore_wilds`, `war_preparation`, `victory_theme`, `defeat_theme`,
`faction_ember_crown_theme`, `faction_ember_crown_battle`, `faction_hollowed_theme`,
`faction_hollowed_battle`, `worldmap_spring` (Cover-Basis für alle Jahreszeiten), `worldmap_autumn`,
`age_transition_sting`, `story_lament`, `mystery_ruins`

### P2 – Kampagne Zeitalter III–V und Gefechtsmodus

`economy_prosperity`, `worldmap_summer`, `worldmap_winter`, `diplomacy_court`, `intrigue_shadows`,
`siege_assault`, `siege_defense`, `faction_skarn_clans_theme`, `faction_skarn_clans_battle`,
`faction_thorn_covenant_theme`, `faction_thorn_covenant_battle`, `faction_saelith_compact_theme`,
`faction_saelith_compact_battle`, `faction_ashen_legion_theme`, `faction_ashen_legion_battle`,
`story_ashfall_memory`, `boss_root_mother`, `mystery_old_gods`

### P3 – Zeitalter VI–VII, Finale, Abspann

`worldmap_fallen_gods`, `mystery_veilscar`, `story_revelation`, `boss_ashen_dragon`,
`endgame_last_crown`, `endgame_final_battle`, `boss_hollow_king`, `credits_theme`

**Abhängigkeiten über Prioritäten hinweg**
- `build_night` entsteht per Cover aus `build_calm` → erst nach Abnahme von `build_calm`.
- `worldmap_summer`, `worldmap_autumn`, `worldmap_winter` entstehen per Cover aus `worldmap_spring`.
- Alle Fraktions-Kampfvarianten entstehen per Cover aus dem jeweiligen Thema.
- `boss_hollow_king` und `endgame_final_battle` gemeinsam produzieren (gleiches Set E, direkter Übergang).
- `credits_theme` zuletzt: Es zitiert Leitmotiv und Fraktionskennungen in ihrer endgültigen Klangfarbe.

**Summe:** 48 Tracks – 8 Manifest-Tracks (P0) und 40 zukünftige Tracks (14 × P1, 18 × P2, 8 × P3).
