# ASHEN CROWNS – Einheiten-, Helden- und Boss-Stimmen (ElevenLabs)

Grundlage: `assets/audio/manifest.ron` (Events `voice.*`), Stimmprofile in `docs/audio/elevenlabs_characters.md`,
Produktionsregeln in `docs/audio/elevenlabs_production_guide.md`.

**Legende**
- **Ziel-Datei:** Pfad relativ zu `assets/audio/`. Exakt so benennen (`.ogg`, Vorbis q6, 48 kHz, mono).
- **Sprechtext (DE):** Generierungstext, **gesprochen wird Deutsch** (Entscheidung: alle Stimmen im Spiel sind deutsch). Eckige Klammern = **Eleven-v3-Audio-Tags**
  (bleiben englisch, werden nicht gesprochen; nicht in Untertitel übernehmen). Eigennamen exakt nach `docs/canon.md`; Aussprache laut Leitfaden, Abschnitt 6.
  Der Sprechtext ist **exakt so einzufügen** (Kontraktionen wie „'ne“, „hab“, „ich mein“ sind gewollte Umgangssprache). Kurze Rufe nicht zu Sätzen ausbauen.
  Nur `bat_swarm` (reine Sound-Effects-Prompts) bleibt englisch.
- **Regie:** Spielanweisung; Emotion, Lautstärke, Tempo, Besonderheiten.
- **Batch-Methode:** Alle Zeilen *einer* Tabelle (gleiche Situation) möglichst in **einer** v3-Generierung erzeugen,
  getrennt durch Absatz und „…“, danach schneiden. Kampf-/Schmerzlaute einzeln generieren, wenn die Emotion „überläuft“.
- **Variantenzahl:** select/move/attack 4–5, alle anderen Situationen mindestens 3 (gemäß Auftrag).
- **Lautheit:** −16 LUFS (Short-Term-Max) für Glutkrone/Heldin/Boss, −17 für Hohle; Peak −1 dBTP (siehe Leitfaden, Abschnitt 7).
- Modellnamen und Einstellungsoptionen zum Produktionszeitpunkt in der ElevenLabs-Oberfläche prüfen.

---

## 1. Glutkrone – `worker` (Leibeigener/Arbeiter)

**Stimme:** `AC_unit_worker` (Profil D1.1) · **Modell:** Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,98
**Haltung:** gutmütig-müder Bauer, ländliche Umgangssprache mit leichter süddeutscher Färbung, kein Soldat – Kampfzeilen klingen improvisiert und etwas ängstlich.

### worker_select (Event `voice.worker.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_select_01.ogg` | Jo, was gibt's zu tun? | freundlich-willig, mittlere Lautstärke |
| `voices/units/worker_select_02.ogg` | [sighs] Noch 'ne Arbeit, was? | kurzer Seufzer, dann gutmütig |
| `voices/units/worker_select_03.ogg` | Hab die Hände frei. Noch. | trocken, kleine Pause vor „Noch“ |
| `voices/units/worker_select_04.ogg` | Ja, Wächterin? Ich hör zu. | respektvoll, leicht unterwürfig |
| `voices/units/worker_select_05.ogg` | Rücken tut weh, aber ich bin da. | Ächzen in der Stimme, lächelnd |

### worker_move (Event `voice.worker.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_move_01.ogg` | Gut, bin unterwegs. | knapp, bodenständig |
| `voices/units/worker_move_02.ogg` | Dann mal los. | leichtes Brummen, gemütlich |
| `voices/units/worker_move_03.ogg` | Pass auf den Schlamm auf … geh ja schon. | zu sich selbst, halblaut |
| `voices/units/worker_move_04.ogg` | Jo, ich kenn den Weg. | selbstsicher |
| `voices/units/worker_move_05.ogg` | [tired] Geh ja, geh ja. | müde, leicht genervt |

### worker_attack (Event `voice.worker.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_attack_01.ogg` | [shouting] Weg von unseren Häusern! | verzweifelt-wütend, laut |
| `voices/units/worker_attack_02.ogg` | Ich hab 'ne Mistgabel und scheu mich nicht! | Mut zusammengekratzt, Stimme kippt leicht |
| `voices/units/worker_attack_03.ogg` | [shouting] Für Brennhollow! | Dorfstolz, kräftig |
| `voices/units/worker_attack_04.ogg` | Schon gut, schon gut – nimm das! | nervös, sich selbst Mut machend |

### worker_retreat (Event `voice.worker.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_retreat_01.ogg` | Dafür werd ich nicht bezahlt! Lauft! | panisch, gehetzt, mit Atem |
| `voices/units/worker_retreat_02.ogg` | [shouting] Zurück ins Dorf! | laut, Angst |
| `voices/units/worker_retreat_03.ogg` | Jeder für sich – ich mein, Rückzug! | hastige Selbstkorrektur, unfreiwillig komisch |

### worker_hit (Event `voice.worker.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_hit_01.ogg` | [groans] Au! Verflucht – | Schmerz + Ärger, abgebrochen |
| `voices/units/worker_hit_02.ogg` | [gasps] Argh, mein Arm! | erschrocken, scharf |
| `voices/units/worker_hit_03.ogg` | [grunts] Uff! | kurzer Stoßlaut, ≤ 0,6 s |

### worker_death (Event `voice.worker.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_death_01.ogg` | [gasps] Sag meiner Frau … das Dach … | schwach, verlöschend, letzte Sorge |
| `voices/units/worker_death_02.ogg` | [groans] Ahh … so kalt … | leiser werdend |
| `voices/units/worker_death_03.ogg` | Nein … nicht so – [exhales] | abgebrochen, letzter Atem |

### worker_victory (Event `voice.worker.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_victory_01.ogg` | [laughs] Ha! Steh noch! | erleichtertes Lachen |
| `voices/units/worker_victory_02.ogg` | Das lehrt sie, hier aufzukreuzen. | stolz, brummig |
| `voices/units/worker_victory_03.ogg` | Wir haben's geschafft … wirklich geschafft! | ungläubige Freude |

### worker_work (Event `voice.worker.work`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_work_01.ogg` | Zurück an die Arbeit. | ergeben, freundlich |
| `voices/units/worker_work_02.ogg` | Hacken, schleppen, stapeln. | rhythmisch, halb gesungen |
| `voices/units/worker_work_03.ogg` | Gutes Holz, das. | zufrieden, fachmännisch |
| `voices/units/worker_work_04.ogg` | Das Feld pflügt sich nicht von allein. | Volksweisheit, trocken |

### worker_build_done (Event `voice.worker.build_done`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/worker_build_done_01.ogg` | So. Gebaut für die Ewigkeit. Meistens. | stolz, dann trockene Einschränkung |
| `voices/units/worker_build_done_02.ogg` | Arbeit erledigt, Wächterin! | laut genug als Meldung |
| `voices/units/worker_build_done_03.ogg` | Fertig! Kommt, schaut's euch an! | freudig |

---

## 2. Glutkrone – `swordsman` (Schwertkämpfer, Miliz)

**Stimme:** `AC_unit_swordsman` (Profil D1.2) · **Modell:** Eleven v3, *Natural* (attack/hit/death *Creative*)
**Haltung:** junger Milizsoldat, zackig, Galgenhumor, norddeutsch-städtisch gefärbte Umgangssprache.

### swordsman_select (Event `voice.swordsman.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_select_01.ogg` | Schwert bereit. | zackig, knapp |
| `voices/units/swordsman_select_02.ogg` | Miliz, zu Befehl. | soldatisch, Haltung annehmen |
| `voices/units/swordsman_select_03.ogg` | Zeigt mir den Ärger. | entschlossen, leicht grinsend |
| `voices/units/swordsman_select_04.ogg` | Schild oben. Was jetzt? | wach, aufmerksam |
| `voices/units/swordsman_select_05.ogg` | Atme noch. Was ist der Befehl? | Galgenhumor |

### swordsman_move (Event `voice.swordsman.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_move_01.ogg` | Rücken aus. | knapp |
| `voices/units/swordsman_move_02.ogg` | Im Laufschritt! | energisch |
| `voices/units/swordsman_move_03.ogg` | Direkt hinter Euch, Wächterin. | loyal |
| `voices/units/swordsman_move_04.ogg` | Marschieren. | neutral, militärisch |
| `voices/units/swordsman_move_05.ogg` | Dann eben durch die Asche. | leicht resigniert, entschlossen |

### swordsman_attack (Event `voice.swordsman.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_attack_01.ogg` | [shouting] Haut sie nieder! | Kampfschrei, voll |
| `voices/units/swordsman_attack_02.ogg` | [shouting] Für die Glutkrone! | Schlachtruf, Stolz |
| `voices/units/swordsman_attack_03.ogg` | Stahl, Jungs! Stahl! | anfeuernd |
| `voices/units/swordsman_attack_04.ogg` | Zurück in eure Gräber! | wütend, verächtlich |
| `voices/units/swordsman_attack_05.ogg` | [grunts] Da, nimm das! | Anstrengung im Schlag |

### swordsman_retreat (Event `voice.swordsman.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_retreat_01.ogg` | Rückzug! Rückzug! | laut, dringlich |
| `voices/units/swordsman_retreat_02.ogg` | Sammeln am Bergfried! | befehlend, außer Atem |
| `voices/units/swordsman_retreat_03.ogg` | Zurück, wir werden überrannt! | Angst, gehetzt |

### swordsman_hit (Event `voice.swordsman.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_hit_01.ogg` | [grunts] Argh! | kurzer Schmerzlaut |
| `voices/units/swordsman_hit_02.ogg` | [groans] Ngh – nur ein Kratzer! | trotzig |
| `voices/units/swordsman_hit_03.ogg` | [gasps] Verdammt! | scharf, überrascht |

### swordsman_death (Event `voice.swordsman.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_death_01.ogg` | [gasps] Für … die Krone … | letzter Atem |
| `voices/units/swordsman_death_02.ogg` | Sagt dem Feldwebel … ich hab gehalten … | schwach, stolz |
| `voices/units/swordsman_death_03.ogg` | [exhales] Das Feuer … geht aus … | verlöschend |

### swordsman_victory (Event `voice.swordsman.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/swordsman_victory_01.ogg` | [shouting] Ha! Sieg! | Jubel |
| `voices/units/swordsman_victory_02.ogg` | Die stehen nicht wieder auf. | zufrieden, trocken |
| `voices/units/swordsman_victory_03.ogg` | Die Krone steht! | stolz |

---

## 3. Glutkrone – `spearman` (Speerträger, Veteran)

**Stimme:** `AC_unit_spearman` (Profil D1.3) · **Modell:** Eleven v3, *Natural/Robust* · Speed 0,95
**Haltung:** wortkarger Veteran, tief und ruhig; selbst im Angriff kontrolliert, nie hysterisch.

### spearman_select (Event `voice.spearman.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_select_01.ogg` | Speer bereit. | ruhig, tief |
| `voices/units/spearman_select_02.ogg` | Jo. Sprich. | wortkarg |
| `voices/units/spearman_select_03.ogg` | Alte Knochen, scharfe Spitze. | trockener Humor |
| `voices/units/spearman_select_04.ogg` | Linie hält. Was gibt's? | gelassen |

### spearman_move (Event `voice.spearman.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_move_01.ogg` | In Bewegung. | neutral |
| `voices/units/spearman_move_02.ogg` | Gleichmäßiger Schritt, Jungs. | an die Truppe gerichtet |
| `voices/units/spearman_move_03.ogg` | Jo, wir gehen. | gelassen |
| `voices/units/spearman_move_04.ogg` | Speere hoch. Marsch. | Kommandoton, nicht laut |

### spearman_attack (Event `voice.spearman.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_attack_01.ogg` | [shouting] Halten und zustoßen! | Kommandoruf |
| `voices/units/spearman_attack_02.ogg` | Speere nach vorn! | kräftig |
| `voices/units/spearman_attack_03.ogg` | Na komm. Komm in die Spitze! | grimmig, provozierend |
| `voices/units/spearman_attack_04.ogg` | Abstützen … jetzt – stoßt! | Timing: Pause, dann Ausbruch |

### spearman_retreat (Event `voice.spearman.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_retreat_01.ogg` | Zurück, langsam und ruhig! | kontrolliert |
| `voices/units/spearman_retreat_02.ogg` | Rückzug! Speere zum Feind! | Kommandoton |
| `voices/units/spearman_retreat_03.ogg` | Boden aufgeben, Jungs. Nicht rennen. | väterlich-bestimmt |

### spearman_hit (Event `voice.spearman.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_hit_01.ogg` | [grunts] Hmpf. | kaum beeindruckt |
| `voices/units/spearman_hit_02.ogg` | [groans] Hatte Schlimmeres. | gepresst |
| `voices/units/spearman_hit_03.ogg` | [exhales sharply] Nngh! | kurzer Stoß |

### spearman_death (Event `voice.spearman.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_death_01.ogg` | [exhales] Haltet … die Linie … | letzter Befehl |
| `voices/units/spearman_death_02.ogg` | Endlich … Ruhe … | fast erleichtert |
| `voices/units/spearman_death_03.ogg` | [groans] Nicht schlecht … für einen alten Mann … | leises Lächeln im Sterben |

### spearman_victory (Event `voice.spearman.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/spearman_victory_01.ogg` | Linie gehalten. Gut. | knapp, zufrieden |
| `voices/units/spearman_victory_02.ogg` | So macht man das, Jungs. | stolz-väterlich |
| `voices/units/spearman_victory_03.ogg` | Begrabt sie. Diesmal richtig. | düster-trocken |

---

## 4. Glutkrone – `archer` (Bogenschützin)

**Stimme:** `AC_unit_archer` (Profil D1.4) · **Modell:** Eleven v3, *Natural* · Speed 1,03
**Haltung:** junge, wache Jägerin, keck, präzise; beim Zielen ruhig, beim Treffer triumphierend.

### archer_select (Event `voice.archer.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_select_01.ogg` | Bogen gespannt. | knapp, wach |
| `voices/units/archer_select_02.ogg` | Augen aufs Feld. | konzentriert |
| `voices/units/archer_select_03.ogg` | Worauf schieß ich? | keck |
| `voices/units/archer_select_04.ogg` | Schützin bereit. | militärisch |
| `voices/units/archer_select_05.ogg` | Freie Schussbahn. Wer ist zuerst dran? | selbstbewusst, leicht spöttisch |

### archer_move (Event `voice.archer.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_move_01.ogg` | Beziehe Stellung. | sachlich |
| `voices/units/archer_move_02.ogg` | [whispers] Schnell und leise. | geflüstert, verschwörerisch |
| `voices/units/archer_move_03.ogg` | Such mir 'ne Anhöhe. | zielstrebig |
| `voices/units/archer_move_04.ogg` | Bin dran. | flink |

### archer_attack (Event `voice.archer.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_attack_01.ogg` | Los! | scharfer Ruf |
| `voices/units/archer_attack_02.ogg` | Pfeile ab! | kräftig |
| `voices/units/archer_attack_03.ogg` | [shouting] Spickt sie mit Federn! | Kampfeslust |
| `voices/units/archer_attack_04.ogg` | Hab dich! | triumphierend |
| `voices/units/archer_attack_05.ogg` | Ziel … und los! | ruhige Atempause vor dem Schuss |

### archer_retreat (Event `voice.archer.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_retreat_01.ogg` | Ziehe mich zurück! | gehetzt |
| `voices/units/archer_retreat_02.ogg` | Zu nah! Rückzug! | alarmiert |
| `voices/units/archer_retreat_03.ogg` | Kann nicht schießen, was auf mir hockt – zurück! | Panik mit Restwitz |

### archer_hit (Event `voice.archer.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_hit_01.ogg` | [gasps] Ah! | kurzer Schreck |
| `voices/units/archer_hit_02.ogg` | [grunts] Die haben mich erwischt! | empört, Schmerz |
| `voices/units/archer_hit_03.ogg` | [groans] Ngh – schieß trotzdem! | trotzig |

### archer_death (Event `voice.archer.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_death_01.ogg` | [gasps] Ich … hab verfehlt … | ungläubig, leise |
| `voices/units/archer_death_02.ogg` | Ah … es wird dunkel … | verlöschend |
| `voices/units/archer_death_03.ogg` | [exhales] Sagt ihnen … ich hab den Letzten … erwischt … | schwacher Stolz |

### archer_victory (Event `voice.archer.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/archer_victory_01.ogg` | Nicht einer kam durch! | jubelnd |
| `voices/units/archer_victory_02.ogg` | [laughs] Saubere Schüsse, jeder einzelne. | stolzes Lachen |
| `voices/units/archer_victory_03.ogg` | Zählt meine Pfeile. Dann zählt ihre Toten. | kühl, selbstbewusst |

---

## 5. Glutkrone – `knight` (Ritter, schwere Kavallerie)

**Stimme:** `AC_unit_knight` (Profil D1.5) · **Modell:** Eleven v3, *Natural* · Speed 0,97
**Haltung:** edel, ritterlich, etwas pathetisch (bewusster Kontrast zu Isoldes Understatement). Kein Helmfilter in der Basisaufnahme.

### knight_select (Event `voice.knight.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_select_01.ogg` | Euer Befehl? | edel, aufrecht |
| `voices/units/knight_select_02.ogg` | Stahl und Sattel, bereit. | stolz |
| `voices/units/knight_select_03.ogg` | Die Lanze der Krone wartet. | feierlich |
| `voices/units/knight_select_04.ogg` | Sprecht, und ich reite. | formell |
| `voices/units/knight_select_05.ogg` | Bei der Flamme, ich bin bereit. | gläubig, fest |

### knight_move (Event `voice.knight.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_move_01.ogg` | Reite aus. | knapp, sicher |
| `voices/units/knight_move_02.ogg` | Vorwärts! | kraftvoll |
| `voices/units/knight_move_03.ogg` | Sporen in die Flanke. | Reiterjargon, zackig |
| `voices/units/knight_move_04.ogg` | Wie Ihr befehlt. | höflich-formell |
| `voices/units/knight_move_05.ogg` | Die Straße gehört uns. | selbstbewusst |

### knight_attack (Event `voice.knight.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_attack_01.ogg` | [shouting] Angriff! | maximaler Schlachtruf |
| `voices/units/knight_attack_02.ogg` | [shouting] Für Valemark und die Flamme! | pathetisch, voll |
| `voices/units/knight_attack_03.ogg` | Reitet sie nieder! | befehlend |
| `voices/units/knight_attack_04.ogg` | Lanzen … jetzt! | Spannung, dann Ausbruch |

### knight_retreat (Event `voice.knight.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_retreat_01.ogg` | Kehrt! Rückzug! | Kommando, laut |
| `voices/units/knight_retreat_02.ogg` | Zurück zu den Linien! | dringlich |
| `voices/units/knight_retreat_03.ogg` | Ein taktischer Rückzug. Reitet! | Würde wahrend, leicht verlegen |

### knight_hit (Event `voice.knight.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_hit_01.ogg` | [grunts] Ha – nur gestreift! | verächtlich |
| `voices/units/knight_hit_02.ogg` | [groans] Meine Rüstung hält! | gepresst, trotzig |
| `voices/units/knight_hit_03.ogg` | [gasps] Ngh! Unverschämt! | empört |

### knight_death (Event `voice.knight.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_death_01.ogg` | [gasps] Mein Eid … erfüllt … | würdevoll sterbend |
| `voices/units/knight_death_02.ogg` | Vergib mir … Flamme … | leises Gebet |
| `voices/units/knight_death_03.ogg` | [exhales] Sagt meinem Haus … ich wich nicht … | letzte Kraft |

### knight_victory (Event `voice.knight.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/knight_victory_01.ogg` | [shouting] Ruhm der Glutkrone! | triumphal |
| `voices/units/knight_victory_02.ogg` | Das Feld gehört uns! | stolz |
| `voices/units/knight_victory_03.ogg` | Ein würdiger Ansturm. Gut gekämpft. | ritterlich, anerkennend |

---

## 6. Hohle – `hollow_thrall` (Hohlknecht)

**Stimme:** `AC_unit_hollow_thrall` (Profil D5.1) · **Modell:** Eleven v3, *Creative* · Speed 0,9
**Konzept:** einstige Dorfbewohner; murmeln **Wortfetzen ihres früheren Lebens**. Nicht verständlich-deutlich sprechen: hauchig,
röchelnd, verwaschen. Basisaufnahme trocken; danach **Hohle-Kette** (Leitfaden 8.3), Pitch −2. Manifest: −2 dB, max. 2 Instanzen.
**Alternative:** Voice Changer (eigene Röchel-Performance → `AC_unit_hollow_thrall`) oder SFX-Prompt (siehe unten) als Layer.

### hollow_thrall_attack (Event `voice.hollow_thrall.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_thrall_attack_01.ogg` | [groans] Hnnngh … warm … | gierig, kehlig |
| `voices/units/hollow_thrall_attack_02.ogg` | [rasping] Gib … es … zurück … | klagend-aggressiv |
| `voices/units/hollow_thrall_attack_03.ogg` | [hisses] Hhhaaah! | wortloser Angriffslaut, Luftstoß |
| `voices/units/hollow_thrall_attack_04.ogg` | … Heim … meins … | gemurmelt, besitzergreifend |

*SFX-Alternativprompt (Layer, 1,0 s, Influence 0,5):* „emaciated undead villager lunging, dry raspy throat hiss and weak
groan, breathy, close, no reverb“.

### hollow_thrall_hit (Event `voice.hollow_thrall.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_thrall_hit_01.ogg` | [groans] Uhhh – | dumpf, gleichgültig |
| `voices/units/hollow_thrall_hit_02.ogg` | [gasps] Hhkk – | trockenes Keuchen, abgebrochen |
| `voices/units/hollow_thrall_hit_03.ogg` | … tut … noch weh … | verloren, fast menschlich |

*SFX-Alternativprompt (0,6 s, Influence 0,5):* „undead creature hit, short dry choked grunt, rattling breath, close“.

### hollow_thrall_death (Event `voice.hollow_thrall.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_thrall_death_01.ogg` | [sighs] … endlich … still … | erlöst, verhauchend |
| `voices/units/hollow_thrall_death_02.ogg` | [exhales] … Mutter … | leises, trauriges Erinnern |
| `voices/units/hollow_thrall_death_03.ogg` | [rattling breath] Hhhhaaahhh … | langes Todesröcheln, 1,5–2 s |

*SFX-Alternativprompt (1,8 s, Influence 0,45):* „undead villager collapsing, long rattling death exhale fading into silence, dry, close“.

---

## 7. Hohle – `hollow_archer` (Knochenschütze)

**Stimme:** `AC_unit_hollow_archer` (Profil D5.2) · **Modell:** Eleven v3, *Creative* mit `[whispers]`
**Konzept:** skelettiert, fast ohne Atem; zischendes Flüstern, klickende Konsonanten. Nachbearbeitung: Hohle-Kette + Knochenklacken-Layer (−18 dB).

### hollow_archer_attack (Event `voice.hollow_archer.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_archer_attack_01.ogg` | [whispers] Ich ssseh dich. | kalt, zischend |
| `voices/units/hollow_archer_attack_02.ogg` | [whispers] Noch einer … | zählend, gierig |
| `voices/units/hollow_archer_attack_03.ogg` | [hisses] Tsssk! | wortlos, Zischlaut beim Schuss |
| `voices/units/hollow_archer_attack_04.ogg` | [whispers] Markier das Herz. | präzise, eisig |

*SFX-Alternativprompt (0,8 s, Influence 0,5):* „skeleton archer hissing whisper with bone clicking, dry, airy, close“.

### hollow_archer_hit (Event `voice.hollow_archer.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_archer_hit_01.ogg` | [hisses] Kkhh! | Knack-Zischen |
| `voices/units/hollow_archer_hit_02.ogg` | Ssss! | scharfes, kurzes Zischen |
| `voices/units/hollow_archer_hit_03.ogg` | [whispers] … gesprungen … | tonlos, fast verwundert |

*SFX-Alternativprompt (0,5 s, Influence 0,55):* „bone cracking with short hiss of air, dry skeletal creature hit“.

### hollow_archer_death (Event `voice.hollow_archer.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_archer_death_01.ogg` | [whispers] … zerfalle … | verwehend |
| `voices/units/hollow_archer_death_02.ogg` | [exhales] Hhhaaa … | letzter, trockener Hauch |
| `voices/units/hollow_archer_death_03.ogg` | [whispers] … das Zählen … hört auf … | erleichtert, tonlos |

*SFX-Alternativprompt (1,5 s, Influence 0,45):* „skeleton collapsing into a pile of bones with a fading dry whisper exhale“.

---

## 8. Hohle – `grave_brute` (Grabkoloss)

**Stimme:** `AC_unit_grave_brute` (Profil D5.3) · **Modell:** Eleven v3, *Creative* · Speed 0,85
**Konzept:** aufgedunsener untoter Riese; kehlig, nass, langsam; einzelne Wörter mit erdrückender Wucht.
Nachbearbeitung: Pitch −4 bis −6, Subbass-Anhebung +2 dB @ 80 Hz, Hohle-Kette; SFX-Growl als Unterschicht (−8 dB).

### grave_brute_attack (Event `voice.grave_brute.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/grave_brute_attack_01.ogg` | [shouting] Zerquetschen! | brüllend, ein Wort |
| `voices/units/grave_brute_attack_02.ogg` | [groans] Hhhrrraaagh! | wortloser Brüller, 1,2 s |
| `voices/units/grave_brute_attack_03.ogg` | [shouting] Runter! | kurz, wuchtig |
| `voices/units/grave_brute_attack_04.ogg` | [growls] Zerschmettern … die Kleinen … | langsam, genüsslich |

*SFX-Layer-Prompt (1,2 s, Influence 0,55):* „massive undead giant roar, wet guttural bellow, deep and heavy, close, no reverb“.

### grave_brute_hit (Event `voice.grave_brute.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/grave_brute_hit_01.ogg` | [groans] Hhhrrmph! | dumpfes Grunzen |
| `voices/units/grave_brute_hit_02.ogg` | [roars] Rrraah! | wütend, kurz |
| `voices/units/grave_brute_hit_03.ogg` | [groans] … tut weh … | träge, überrascht |

*SFX-Layer-Prompt (0,7 s, Influence 0,55):* „huge creature pained grunt, low wet growl, short“.

### grave_brute_death (Event `voice.grave_brute.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/grave_brute_death_01.ogg` | [groans] Hhhuuurrrgh … | langes, absinkendes Stöhnen, 2 s |
| `voices/units/grave_brute_death_02.ogg` | [exhales] … Erde … kalt … | müde, fast kindlich |
| `voices/units/grave_brute_death_03.ogg` | [roars] Rrrraaaaagh … | Brüllen, das in Röcheln zerfällt |

*SFX-Layer-Prompt (2,5 s, Influence 0,45):* „giant undead creature death groan fading into a heavy collapsing thud, deep“.

---

## 9. Heldin – `hero_isolde` (Isolde Raventhorn, Wächterin der Aschmark)

**Stimme:** `AC_hero_isolde` (Profil A1) · **Modell:** Eleven v3, *Natural* (attack/ability gelegentlich *Creative*) · Speed 1,0
**Haltung:** pflichtbewusst, kontrolliert, trocken-humorvoll; Humor durch Understatement, nie Klamauk. Lautheitsreferenz der Produktion.
Manifest: +1 dB, Cooldown 300 ms.

### hero_isolde_select (Event `voice.hero_isolde.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_select_01.ogg` | Raventhorn. Sprecht. | knapp, Autorität, **Referenzdatei** |
| `voices/heroes/hero_isolde_select_02.ogg` | Die Aschmark steht noch. Ich auch. | ruhig-stolz |
| `voices/heroes/hero_isolde_select_03.ogg` | Ja? Macht schnell, die Toten warten nicht. | leicht ungeduldig |
| `voices/heroes/hero_isolde_select_04.ogg` | Wächterin hier. | sachlich |
| `voices/heroes/hero_isolde_select_05.ogg` | [sarcastic] Was jetzt? Noch ein Feuer zu löschen? | trocken-ironisch, nicht genervt |

### hero_isolde_move (Event `voice.hero_isolde.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_move_01.ogg` | Unterwegs. | knapp |
| `voices/heroes/hero_isolde_move_02.ogg` | Ich kenne diese Wege. | sicher, ein Hauch Wehmut |
| `voices/heroes/hero_isolde_move_03.ogg` | Dann durch die Asche. | entschlossen |
| `voices/heroes/hero_isolde_move_04.ogg` | Bin unterwegs. Versucht, das Dorf nicht zu verlieren, solange ich weg bin. | trockener Humor |
| `voices/heroes/hero_isolde_move_05.ogg` | Wenn's brennt, geh ich. | lakonisch |

### hero_isolde_attack (Event `voice.hero_isolde.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_attack_01.ogg` | [shouting] Für die Aschmark! | voller Schlachtruf |
| `voices/heroes/hero_isolde_attack_02.ogg` | Zurück ins Dunkel mit euch! | kalte Wut |
| `voices/heroes/hero_isolde_attack_03.ogg` | Stahl und Glut! | Kampfformel, kraftvoll |
| `voices/heroes/hero_isolde_attack_04.ogg` | Mal sehen, ob ihr blutet. | gefährlich ruhig |
| `voices/heroes/hero_isolde_attack_05.ogg` | [shouting] Brennt sie nieder! | Befehl an Truppe |

### hero_isolde_retreat (Event `voice.hero_isolde.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_retreat_01.ogg` | Rückzug. Das ist keine Bitte. | scharf, kontrolliert |
| `voices/heroes/hero_isolde_retreat_02.ogg` | Wir ziehen ab. Morgen kämpfen wir wieder. | bitter-vernünftig |
| `voices/heroes/hero_isolde_retreat_03.ogg` | [exhales] Nicht hier. Nicht heute. Zurück! | außer Atem, entschlossen |

### hero_isolde_hit (Event `voice.hero_isolde.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_hit_01.ogg` | [grunts] Ngh! Ist das alles? | trotzig |
| `voices/heroes/hero_isolde_hit_02.ogg` | [groans] Hah … ich hatte schon schlimmeres Fieber. | Anspielung Aschenfieber, gepresst |
| `voices/heroes/hero_isolde_hit_03.ogg` | [gasps] Argh! | reiner Schmerzlaut, kurz |

### hero_isolde_death (Event `voice.hero_isolde.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_death_01.ogg` | [gasps] Haltet … die Feuer … am Brennen … | letzter Auftrag, schwach |
| `voices/heroes/hero_isolde_death_02.ogg` | Vater … ich hab gehalten … so lang ich konnte … | verletzlich, leise |
| `voices/heroes/hero_isolde_death_03.ogg` | [exhales] Nicht … jetzt … verdammt … | Wut gegen das Sterben |

### hero_isolde_victory (Event `voice.hero_isolde.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_victory_01.ogg` | Die Mark hält. Heute. | erschöpft, Pause vor „Heute“ |
| `voices/heroes/hero_isolde_victory_02.ogg` | [exhales] Na ja. Hätte schlimmer laufen können. | Understatement |
| `voices/heroes/hero_isolde_victory_03.ogg` | Zählt zuerst die Lebenden. Dann die Toten. | ernst, Verantwortung |

### hero_isolde_ability (Event `voice.hero_isolde.ability`)
Hinweis: Das Manifest kennt **ein** Ability-Event. Die Varianten decken die drei Fähigkeiten ab (je 2). Bei späterer Aufteilung
in `ember_cleave`/`rallying_banner`/`ashen_ward` können die Dateien entsprechend zugeordnet werden (Empfehlung an das Engine-Team).

| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_ability_01.ogg` | [shouting] Kostet die Glut! | Glutspalter (`ember_cleave`), Schlagschrei |
| `voices/heroes/hero_isolde_ability_02.ogg` | Spalten und brennen! | Glutspalter, mit Hiebanstrengung |
| `voices/heroes/hero_isolde_ability_03.ogg` | [shouting] Zu mir! Hebt das Banner! | Sammelbanner (`rallying_banner`), mitreißend |
| `voices/heroes/hero_isolde_ability_04.ogg` | Steht zu mir. Die Mark sieht zu! | Sammelbanner, Moralruf |
| `voices/heroes/hero_isolde_ability_05.ogg` | Asche, schirme mich. | Aschenschild (`ashen_ward`), konzentriert, halblaut |
| `voices/heroes/hero_isolde_ability_06.ogg` | Hinter mich. Sofort. | Aschenschild, schützend-befehlend |

### hero_isolde_greet (Event `voice.hero_isolde.greet`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_greet_01.ogg` | Isolde Raventhorn, Wächterin der Aschmark. An die Arbeit. | Vorstellung, sachlich-warm |
| `voices/heroes/hero_isolde_greet_02.ogg` | Brennhollow atmet noch. Gut. Dabei soll's bleiben. | ruhig, entschlossen |
| `voices/heroes/hero_isolde_greet_03.ogg` | [sarcastic] Ihr habt gerufen, ich bin da. Versucht diesmal, einen Plan zu haben. | trocken, augenzwinkernd |

### hero_isolde_levelup (Event `voice.hero_isolde.levelup`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_levelup_01.ogg` | Stärker. Gut. Die Ruinen werden nicht freundlicher. | nüchtern zufrieden |
| `voices/heroes/hero_isolde_levelup_02.ogg` | [exhales] Die Glut brennt heller. | spürbar, fast ehrfürchtig |
| `voices/heroes/hero_isolde_levelup_03.ogg` | Älter, klüger, mehr Narben. Das Übliche. | trockener Humor |

### hero_isolde_lowhp (Event `voice.hero_isolde.lowhp`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_isolde_lowhp_01.ogg` | [groans] Ich bin verletzt … schwer. | ehrlich, gepresst |
| `voices/heroes/hero_isolde_lowhp_02.ogg` | [gasps] Brauche einen Heiler. Oder ein Wunder. | Humor trotz Schmerz |
| `voices/heroes/hero_isolde_lowhp_03.ogg` | [exhales] Viel mehr halte ich nicht aus. | erschöpft, Warnung an Spieler |

---

## 10. Boss – `boss_pale_reeve` (Der Fahle Vogt)

**Stimme:** `AC_boss_pale_reeve` (Profil C4) · **Modell:** Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,95
**Konzept:** untoter Steuervogt Aureths; Motive **Zehnt, Register, Schuld, Zinsen, Glocke**. Ablage laut Manifest unter `voices/units/`.
Nachbearbeitung: Hohle-Kette + Vogt-Ergänzung (Chorus-Schicht, kalter Kathedralhall 1,8 s, Mix 12 %). Manifest: +1 dB, Cooldown 800 ms.

### boss_pale_reeve_attack (Event `voice.boss_pale_reeve.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/boss_pale_reeve_attack_01.ogg` | Euer Zehnt ist fällig. | kalt, beiläufig |
| `voices/units/boss_pale_reeve_attack_02.ogg` | Zahlt, was Ihr schuldet! | scharf, lauter |
| `voices/units/boss_pale_reeve_attack_03.ogg` | Jede Seele wird gezählt. | pedantisch, zischend |
| `voices/units/boss_pale_reeve_attack_04.ogg` | Ab ins Register mit Euch. | süffisant |

### boss_pale_reeve_ability (Event `voice.boss_pale_reeve.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/boss_pale_reeve_ability_01.ogg` | Ich fordere die Schuld Eures Atems ein. | Seelenentzug (`reeve_soul_drain`), langsam, genüsslich |
| `voices/units/boss_pale_reeve_ability_02.ogg` | [whispers] Erhebt euch, Säumige. Erhebt euch und dient. | Beschwörung (`hollow_summon`), geflüstert-befehlend |
| `voices/units/boss_pale_reeve_ability_03.ogg` | Eure Seele ist verwirkt. | Urteil, eisig |

### boss_pale_reeve_hit (Event `voice.boss_pale_reeve.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/boss_pale_reeve_hit_01.ogg` | [hisses] Unverschämtheit! | empörtes Zischen |
| `voices/units/boss_pale_reeve_hit_02.ogg` | [shrieks] Ihr wagt es, den Vogt zu schlagen? | schrill, kippende Stimme |
| `voices/units/boss_pale_reeve_hit_03.ogg` | [groans] Das wird … vermerkt. | gekränkt-pedantisch |

### boss_pale_reeve_death (Event `voice.boss_pale_reeve.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/boss_pale_reeve_death_01.ogg` | [shrieks] Nein – das Register … nicht ausgeglichen … | Entsetzen, zerfallend, 2–3 s |
| `voices/units/boss_pale_reeve_death_02.ogg` | [rattling breath] Das Tief … schickt … einen anderen … | Drohung im Sterben |
| `voices/units/boss_pale_reeve_death_03.ogg` | [exhales] … vollständig … bezahlt … | fast erlöst, verhauchend |

### boss_pale_reeve_taunt (Event `voice.boss_pale_reeve.taunt`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/boss_pale_reeve_taunt_01.ogg` | Brennhollow. Dreißig Seelen. Neunhundert Jahre im Rückstand. | verlesend wie aus einem Register |
| `voices/units/boss_pale_reeve_taunt_02.ogg` | Hört Ihr die Glocke, Wächterin? Sie läutet für Euer Konto. | leise, genüsslich |
| `voices/units/boss_pale_reeve_taunt_03.ogg` | Das Hohle Tief ist geduldig. Ich bin es nicht. | kalt, Pause in der Mitte |
| `voices/units/boss_pale_reeve_taunt_04.ogg` | [laughs] Lauft nur. Die Toten treiben immer ein. | trockenes, hohles Lachen |

---

## 11. Ausblick – spätere Einheiten (nicht im Manifest v1)

Alle IDs in diesem Abschnitt sind **Vorschläge** und vor Produktion vom Kanon-Owner freizugeben. Dateinamen folgen der
Konvention `voices/units/<unit_id>_<situation>_<NN>.ogg`. Stimmen aus Teil D des Charakterdokuments. Pro Einheit hier je drei
Zeilen für select/move/attack; für die Produktion auf 4–5 Varianten erweitern und retreat/hit/death/victory ergänzen.

### 11.1 Glutkrone – Flammenpriesterin (Priester) · Vorschlag `ember_priest` · Stimme `AC_ember_crown_priest`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ember_priest_select_01.ogg` | Die Flamme ist mit euch. | warm, segnend |
| `voices/units/ember_priest_select_02.ogg` | Wer braucht Heilung? | fürsorglich |
| `voices/units/ember_priest_select_03.ogg` | Sprich, Kind des Herdes. | feierlich |
| `voices/units/ember_priest_move_01.ogg` | Ich gehe, wo Licht gebraucht wird. | ruhig |
| `voices/units/ember_priest_move_02.ogg` | Geht voran. Ich folge den Verwundeten. | pragmatisch |
| `voices/units/ember_priest_move_03.ogg` | Die Flamme lenkt meine Schritte. | gläubig |
| `voices/units/ember_priest_attack_01.ogg` | [shouting] Brenne, Unreines! | heiliger Zorn |
| `voices/units/ember_priest_attack_02.ogg` | Zurück ins Tief! | bannend |
| `voices/units/ember_priest_attack_03.ogg` | Das Licht hole dich! | Gebetsformel als Waffe |

### 11.2 Skarn-Clans – Ahnenruferin (Magier/Priester) · Vorschlag `skarn_skald` · Stimme `AC_skarn_clans_skald`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/skarn_skald_select_01.ogg` | Die Ahnen hören zu. | tief, raunend |
| `voices/units/skarn_skald_select_02.ogg` | Welche Saga schreiben wir heute? | rhythmisch |
| `voices/units/skarn_skald_select_03.ogg` | Ich trage die Lieder der Toten. | feierlich |
| `voices/units/skarn_skald_move_01.ogg` | Wohin die Trommeln führen. | gemessen |
| `voices/units/skarn_skald_move_02.ogg` | Meine Füße kennen die alten Pfade. | ruhig |
| `voices/units/skarn_skald_move_03.ogg` | Weiter, unter dem Auge des Berges. | sagenhaft |
| `voices/units/skarn_skald_attack_01.ogg` | [shouting] Väter, leiht uns Frost! | Beschwörungsruf |
| `voices/units/skarn_skald_attack_02.ogg` | Hört den Zorn der Ahnen! | kraftvoll |
| `voices/units/skarn_skald_attack_03.ogg` | Dein Name stirbt mit dir! | Fluch |

### 11.3 Skarn-Clans – Clan-Reiterei (Kavallerie) · Vorschlag `skarn_rider` · Stimme `AC_skarn_clans_warrior` (jüngere Variante)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/skarn_rider_select_01.ogg` | Aufgesessen und hungrig. | kernig |
| `voices/units/skarn_rider_select_02.ogg` | Der Schnee bremst uns nicht. | stolz |
| `voices/units/skarn_rider_select_03.ogg` | Sag das Wort, Fürst. | ungeduldig |
| `voices/units/skarn_rider_move_01.ogg` | Scharfer Ritt! | laut |
| `voices/units/skarn_rider_move_02.ogg` | Über den Kamm! | energisch |
| `voices/units/skarn_rider_move_03.ogg` | Hufe und Eisen, los! | Burr betonen |
| `voices/units/skarn_rider_attack_01.ogg` | [shouting] Skarnheim! | Clanruf |
| `voices/units/skarn_rider_attack_02.ogg` | Zertrampelt sie! | wild |
| `voices/units/skarn_rider_attack_03.ogg` | Für die Knochen meiner Väter! | Ahnenschwur |

### 11.4 Dornenbund – Druide (Magier) · Vorschlag `thorn_druid` · Stimme `AC_thorn_covenant_druid`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/thorn_druid_select_01.ogg` | Die Wurzeln hören zu. | leise, alt |
| `voices/units/thorn_druid_select_02.ogg` | Sprich sanft. | mahnend |
| `voices/units/thorn_druid_select_03.ogg` | Der Grüne Kreis antwortet. | würdevoll |
| `voices/units/thorn_druid_move_01.ogg` | Wo das Moos dicht wächst. | ruhig |
| `voices/units/thorn_druid_move_02.ogg` | Leise, durch die Farne. | gedämpft |
| `voices/units/thorn_druid_move_03.ogg` | Der Wald macht Platz. | wissend |
| `voices/units/thorn_druid_attack_01.ogg` | [shouting] Dornen, bindet sie! | Zauberruf |
| `voices/units/thorn_druid_attack_02.ogg` | Kehr zurück in die Erde! | streng |
| `voices/units/thorn_druid_attack_03.ogg` | Die Wildnis erinnert sich an dich! | drohend |

### 11.5 Dornenbund – Veyl-Pirscherin (Assassine) · Vorschlag `veyl_stalker` · Stimme `AC_thorn_covenant_veyl_scout`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/veyl_stalker_select_01.ogg` | [whispers] Ich war nie hier. | geflüstert |
| `voices/units/veyl_stalker_select_02.ogg` | Augen wie Mondlicht. | rätselhaft |
| `voices/units/veyl_stalker_select_03.ogg` | Wer schläft heute Nacht? | gefährlich sanft |
| `voices/units/veyl_stalker_move_01.ogg` | [whispers] Durch die Schatten. | luftig |
| `voices/units/veyl_stalker_move_02.ogg` | Wie die Motte zum Licht. | Veyl-Motiv |
| `voices/units/veyl_stalker_move_03.ogg` | Ungesehen. | tonlos |
| `voices/units/veyl_stalker_attack_01.ogg` | [whispers] Still jetzt. | intim, tödlich |
| `voices/units/veyl_stalker_attack_02.ogg` | Ein Schnitt. Genug. | kalt |
| `voices/units/veyl_stalker_attack_03.ogg` | Das Laub wird dich bedecken. | melancholisch |

### 11.6 Saelith-Pakt – Gezeitenpriesterin (Magier/Priester) · Vorschlag `saelith_tide_priestess` · Stimme `AC_saelith_compact_tide_priestess`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/saelith_tide_priestess_select_01.ogg` | Die Flut wendet sich. | ruhig fließend |
| `voices/units/saelith_tide_priestess_select_02.ogg` | Frag, und das Meer antwortet vielleicht. | geheimnisvoll |
| `voices/units/saelith_tide_priestess_select_03.ogg` | Salz und Silber, zu Diensten. | elegant |
| `voices/units/saelith_tide_priestess_move_01.ogg` | Wie Wasser gehe ich. | weich |
| `voices/units/saelith_tide_priestess_move_02.ogg` | Ich folge der Strömung. | gelassen |
| `voices/units/saelith_tide_priestess_move_03.ogg` | Das Ufer ruft. | träumerisch |
| `voices/units/saelith_tide_priestess_attack_01.ogg` | [shouting] Ertrinke! | plötzliche Härte |
| `voices/units/saelith_tide_priestess_attack_02.ogg` | Das Meer nimmt, was ihm zusteht. | kühl |
| `voices/units/saelith_tide_priestess_attack_03.ogg` | Brich wie eine Welle! | kraftvoll |

### 11.7 Saelith-Pakt – Söldner-Lanzenreiter (Kavallerie) · Vorschlag `saelith_lancer` · Stimme `AC_saelith_compact_captain` (männliche Variante neu designen)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/saelith_lancer_select_01.ogg` | Voll bezahlt? Dann reite ich. | zynisch |
| `voices/units/saelith_lancer_select_02.ogg` | Lanze und Münze, beide scharf. | geschäftsmäßig |
| `voices/units/saelith_lancer_select_03.ogg` | Laut Vertrag hör ich zu. | gelangweilt |
| `voices/units/saelith_lancer_move_01.ogg` | Unterwegs, zum vereinbarten Satz. | trocken |
| `voices/units/saelith_lancer_move_02.ogg` | Pferde laufen nicht mit Versprechen. | spöttisch |
| `voices/units/saelith_lancer_move_03.ogg` | Bin unterwegs. | neutral |
| `voices/units/saelith_lancer_attack_01.ogg` | [shouting] Verdient euren Sold! | Ruf an Kameraden |
| `voices/units/saelith_lancer_attack_02.ogg` | Angriff, für die Prämie! | grinsend |
| `voices/units/saelith_lancer_attack_03.ogg` | Nichts Persönliches! | lässig-brutal |

### 11.8 Hohle – Kultist der Leeren Liturgie (Priester) · Vorschlag `hollow_cultist` · Stimme `AC_hollowed_cultist`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_cultist_select_01.ogg` | [whispers] Das Tief hört dich. | fiebrig |
| `voices/units/hollow_cultist_select_02.ogg` | Leer und bereit. | ekstatisch-ruhig |
| `voices/units/hollow_cultist_select_03.ogg` | Sprich das stille Wort. | liturgisch |
| `voices/units/hollow_cultist_move_01.ogg` | Hinab, immer hinab. | singend |
| `voices/units/hollow_cultist_move_02.ogg` | Der Schleier ist dünn hier. | entrückt |
| `voices/units/hollow_cultist_move_03.ogg` | Ich folge der Stille. | tonlos |
| `voices/units/hollow_cultist_attack_01.ogg` | [shouting] Werde geleert! | schrille Ekstase |
| `voices/units/hollow_cultist_attack_02.ogg` | Deine Flamme erlischt! | hasserfüllt |
| `voices/units/hollow_cultist_attack_03.ogg` | Nur die Stille bleibt! | Litanei |

### 11.9 Hohle – Wiedergänger-Magierin (Magier) · Vorschlag `hollow_revenant_mage` · Stimme `AC_hollowed_revenant_mage`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_revenant_mage_select_01.ogg` | Ich erinnere mich an das Licht. | wehmütig-kalt |
| `voices/units/hollow_revenant_mage_select_02.ogg` | Was willst du auflösen? | elegant, bedrohlich |
| `voices/units/hollow_revenant_mage_select_03.ogg` | Aureths letzte Hofmagierin. | stolz |
| `voices/units/hollow_revenant_mage_move_01.ogg` | Ich treibe. | gehaucht |
| `voices/units/hollow_revenant_mage_move_02.ogg` | Wie Rauch durch Ruinen. | langsam |
| `voices/units/hollow_revenant_mage_move_03.ogg` | Die Toten machen kein Geräusch. | flüsternd |
| `voices/units/hollow_revenant_mage_attack_01.ogg` | [whispers] Aufgelöst. | eisig |
| `voices/units/hollow_revenant_mage_attack_02.ogg` | Verdorre! | Zauberwort, scharf |
| `voices/units/hollow_revenant_mage_attack_03.ogg` | Dein Licht ist geliehen. | verächtlich |

### 11.10 Aschenlegion – Aschenreiter (Kavallerie) · Vorschlag `legion_ash_rider` · Stimme `AC_ashen_legion_legionnaire` (mit Helm-Signatur)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/legion_ash_rider_select_01.ogg` | Wir reiten für die Sonne. | monoton-feierlich |
| `voices/units/legion_ash_rider_select_02.ogg` | Befehligt die Legion. | kalt |
| `voices/units/legion_ash_rider_select_03.ogg` | Unsere Rösser ermüden nicht. | mechanisch |
| `voices/units/legion_ash_rider_move_01.ogg` | Die Legion rückt vor. | gleichförmig |
| `voices/units/legion_ash_rider_move_02.ogg` | Durch Feuer und Asche. | hohl |
| `voices/units/legion_ash_rider_move_03.ogg` | Wie befohlen wurde. | archaisch |
| `voices/units/legion_ash_rider_attack_01.ogg` | [shouting] Für den Hohlen König! | dröhnend |
| `voices/units/legion_ash_rider_attack_02.ogg` | Kniet oder brennt! | gebieterisch |
| `voices/units/legion_ash_rider_attack_03.ogg` | Wir weichen nicht! | Eid-Formel |

### 11.11 Aschenlegion – Legions-Herold (Priester der Sonnenhäresie) · Vorschlag `legion_herald` · Stimme `AC_ashen_legion_herald`
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/legion_herald_select_01.ogg` | Der Sonnenkönig kehrt zurück! | fanatisch |
| `voices/units/legion_herald_select_02.ogg` | Hört die Verkündung! | ausrufend |
| `voices/units/legion_herald_select_03.ogg` | Herrlichkeit in seinem Licht. | entrückt |
| `voices/units/legion_herald_move_01.ogg` | Ich trage sein Wort. | getragen |
| `voices/units/legion_herald_move_02.ogg` | Zu jedem Tor die Botschaft. | ausrufend |
| `voices/units/legion_herald_move_03.ogg` | Die Herolde gehen voran. | feierlich |
| `voices/units/legion_herald_attack_01.ogg` | [shouting] Ketzer, brennt! | schrill |
| `voices/units/legion_herald_attack_02.ogg` | Schmelzt in seinem Licht! | ekstatisch |
| `voices/units/legion_herald_attack_03.ogg` | Die Asche wird zu Gold! | fanatisch-jubelnd |

---

## 12. Echtzeit-Militär – Glutkrone (neue Einheiten)

Ergänzung zu `docs/design/rts_military.md`. Alle Einheiten dieses Abschnitts stehen im Manifest (Events `voice.<id>.<situation>`); die Stimmprofile stehen im Figurenleitfaden, Teil G (G1–G12). **Sprechtempo und Haltung** je Einheit siehe Kopfzeile; Loudness wie Glutkrone allgemein (−16 LUFS). `priest` ersetzt den Vorschlag `ember_priest` aus Abschnitt 11.1.

### 12.1 Glutkrone – `crossbowman` (Armbrustschütze)

**Stimme:** `AC_unit_crossbowman (Profil G1)` · **Modell:** Eleven v3, *Natural* · Speed 1,0
**Haltung:** nüchterner Söldnerveteran aus den Grenzgarnisonen, kurze, trockene Sätze, spricht über Reichweite und Treffer wie über Handwerk.

### crossbowman_select (Event `voice.crossbowman.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_select_01.ogg` | Armbrust bereit. | knapp, sachlich |
| `voices/units/crossbowman_select_02.ogg` | Bolzen liegt in der Rinne. | trocken, leise |
| `voices/units/crossbowman_select_03.ogg` | Jawohl. Ruhige Hände. | ruhig-selbstsicher |
| `voices/units/crossbowman_select_04.ogg` | Sagt's, und ich schieß einen durch. | leicht grimmig |

### crossbowman_move (Event `voice.crossbowman.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_move_01.ogg` | Rücke vor. | knapp |
| `voices/units/crossbowman_move_02.ogg` | Haltet die Linie, ich komme nach. | ruhig |
| `voices/units/crossbowman_move_03.ogg` | Spanne im Gehen. | beiläufig, mit Metallklicken |
| `voices/units/crossbowman_move_04.ogg` | Bin unterwegs. | neutral |

### crossbowman_attack (Event `voice.crossbowman.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_attack_01.ogg` | Schuss! | scharf, laut |
| `voices/units/crossbowman_attack_02.ogg` | Der hier durchschlägt Panzer! | grimmig |
| `voices/units/crossbowman_attack_03.ogg` | Runter mit dir! | hart |
| `voices/units/crossbowman_attack_04.ogg` | Spannen … und Schuss! | rhythmisch, zweiteilig |

### crossbowman_retreat (Event `voice.crossbowman.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_retreat_01.ogg` | Zurück, ich lade nach! | gehetzt |
| `voices/units/crossbowman_retreat_02.ogg` | Zu nah! Deckt mich! | angespannt |
| `voices/units/crossbowman_retreat_03.ogg` | Zurück in die Linie! | laut |

### crossbowman_hit (Event `voice.crossbowman.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_hit_01.ogg` | [grunts] Mein Arm! | Schmerz, kurz |
| `voices/units/crossbowman_hit_02.ogg` | [gasps] Getroffen! | überrascht |
| `voices/units/crossbowman_hit_03.ogg` | Nicht die Sehne, verflucht! | ärgerlich |

### crossbowman_death (Event `voice.crossbowman.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_death_01.ogg` | [exhales] Kalt … so kalt … | verhauchend |
| `voices/units/crossbowman_death_02.ogg` | [gasps] Nachladen … jemand … | Pflichtgefühl bis zuletzt |
| `voices/units/crossbowman_death_03.ogg` | [groans] Ich hab … nichts verfehlt … | stolz-müde |

### crossbowman_victory (Event `voice.crossbowman.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/crossbowman_victory_01.ogg` | Saubere Arbeit, alle. | zufrieden, ruhig |
| `voices/units/crossbowman_victory_02.ogg` | Das war der letzte Bolzen für heute. | trocken |
| `voices/units/crossbowman_victory_03.ogg` | Zählt die Bolzen, nicht die Toten. | müde-trocken |

### 12.2 Glutkrone – `halberdier` (Hellebardier)

**Stimme:** `AC_unit_halberdier (Profil G2)` · **Modell:** Eleven v3, *Natural* · Speed 0,95
**Haltung:** bärbeißiger Gardist mit tiefer Stimme, ein Pferdebrecher; liebt Formationen, mag keine Reiter.

### halberdier_select (Event `voice.halberdier.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_select_01.ogg` | Hellebarde hoch. | tief, knapp |
| `voices/units/halberdier_select_02.ogg` | Zur Stelle und bereit. | steif, stolz |
| `voices/units/halberdier_select_03.ogg` | Sollen sie nur zu Pferd kommen. | grimmig-amüsiert |
| `voices/units/halberdier_select_04.ogg` | Befehle, Wächterin? | respektvoll |

### halberdier_move (Event `voice.halberdier.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_move_01.ogg` | Marschiere. | knapp |
| `voices/units/halberdier_move_02.ogg` | Gleichmäßiger Schritt, Stangen waagerecht. | kommandoartig |
| `voices/units/halberdier_move_03.ogg` | Schwere Stiefel, ehrliche Straße. | brummig |
| `voices/units/halberdier_move_04.ogg` | Rücken ab. | neutral |

### halberdier_attack (Event `voice.halberdier.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_attack_01.ogg` | Anlegen! Anlegen! | laut, kommandierend |
| `voices/units/halberdier_attack_02.ogg` | Pferdebrecher! | Schlachtruf |
| `voices/units/halberdier_attack_03.ogg` | Haken und zerren! | grimmig, Anstrengung |
| `voices/units/halberdier_attack_04.ogg` | Für die Krone! | kräftig |

### halberdier_retreat (Event `voice.halberdier.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_retreat_01.ogg` | Zurück, zurück! | laut |
| `voices/units/halberdier_retreat_02.ogg` | Linie neu bilden! | befehlend |
| `voices/units/halberdier_retreat_03.ogg` | Zurück, verflucht! | gepresst |

### halberdier_hit (Event `voice.halberdier.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_hit_01.ogg` | [grunts] Rüstung hält. | trotzig |
| `voices/units/halberdier_hit_02.ogg` | [groans] Das gibt einen Bluterguss. | trocken |
| `voices/units/halberdier_hit_03.ogg` | [hisses] Ah! | kurz |

### halberdier_death (Event `voice.halberdier.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_death_01.ogg` | [exhales] Haltet die Linie … | verhauchend |
| `voices/units/halberdier_death_02.ogg` | [groans] Vergebt mir, Hauptmann. | schwer |
| `voices/units/halberdier_death_03.ogg` | [gasps] Nicht … einen Schritt … | gebrochen-stur |

### halberdier_victory (Event `voice.halberdier.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/halberdier_victory_01.ogg` | Kein Huf kam an uns vorbei. | zufrieden |
| `voices/units/halberdier_victory_02.ogg` | Stangen runter. Gut gemacht. | ruhig |
| `voices/units/halberdier_victory_03.ogg` | Die Linie hielt. | stolz |

### 12.3 Glutkrone – `shieldbearer` (Schildträger)

**Stimme:** `AC_unit_shieldbearer (Profil G3)` · **Modell:** Eleven v3, *Natural* · Speed 0,92
**Haltung:** geduldiger, schwerer Bollwerkkämpfer; spricht wenig, sehr tief, mit stoischer Ruhe.

### shieldbearer_select (Event `voice.shieldbearer.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_select_01.ogg` | Schild hoch. | tief, ruhig |
| `voices/units/shieldbearer_select_02.ogg` | Ich bin Eure Mauer. | stoisch |
| `voices/units/shieldbearer_select_03.ogg` | Hinter mich. | ruhig-bestimmt |
| `voices/units/shieldbearer_select_04.ogg` | Bereit. | einsilbig |

### shieldbearer_move (Event `voice.shieldbearer.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_move_01.ogg` | Rücke vor, Schilde verhakt. | gleichmäßig |
| `voices/units/shieldbearer_move_02.ogg` | Langsam und stetig. | brummend |
| `voices/units/shieldbearer_move_03.ogg` | Nach vorn. | knapp |
| `voices/units/shieldbearer_move_04.ogg` | Marschiere. | neutral |

### shieldbearer_attack (Event `voice.shieldbearer.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_attack_01.ogg` | Schildwall! | laut, kommandierend |
| `voices/units/shieldbearer_attack_02.ogg` | Drücken! | Anstrengung |
| `voices/units/shieldbearer_attack_03.ogg` | Nicht an mir vorbei! | hart |
| `voices/units/shieldbearer_attack_04.ogg` | Halten und zuschlagen! | rhythmisch |

### shieldbearer_retreat (Event `voice.shieldbearer.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_retreat_01.ogg` | Schilde zurück, zurück! | laut |
| `voices/units/shieldbearer_retreat_02.ogg` | Allein halte ich nicht! | gepresst |
| `voices/units/shieldbearer_retreat_03.ogg` | Zurück hinter die Mauer! | dringlich |

### shieldbearer_hit (Event `voice.shieldbearer.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_hit_01.ogg` | [grunts] Der Schild hat's abgefangen. | trocken |
| `voices/units/shieldbearer_hit_02.ogg` | [groans] Ein schwerer. | brummig |
| `voices/units/shieldbearer_hit_03.ogg` | [grunts] Steh noch. | stur |

### shieldbearer_death (Event `voice.shieldbearer.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_death_01.ogg` | [exhales] Die Mauer … fällt … | schwer, langsam |
| `voices/units/shieldbearer_death_02.ogg` | [groans] Hinter … mir … | beschützend |
| `voices/units/shieldbearer_death_03.ogg` | [sighs] Ruhe … | erlöst |

### shieldbearer_victory (Event `voice.shieldbearer.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_victory_01.ogg` | Die Mauer steht noch. | ruhig-stolz |
| `voices/units/shieldbearer_victory_02.ogg` | Kein Kratzer. Na gut, ein paar. | trocken-humorvoll |
| `voices/units/shieldbearer_victory_03.ogg` | Schilde runter. | erleichtert |

### shieldbearer_ability (Event `voice.shieldbearer.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/shieldbearer_ability_01.ogg` | Schildwall! Aufschließen! | laut, befehlend |
| `voices/units/shieldbearer_ability_02.ogg` | Schilde verhaken! | kurz, scharf |
| `voices/units/shieldbearer_ability_03.ogg` | Bildet die Mauer! | kräftig |

### 12.4 Glutkrone – `scout_rider` (Spähreiter)

**Stimme:** `AC_unit_scout_rider (Profil G4)` · **Modell:** Eleven v3, *Creative* · Speed 1,08
**Haltung:** junger, flinker Grenzreiter, wach und etwas übermütig; spricht schnell, lacht gern.

### scout_rider_select (Event `voice.scout_rider.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_select_01.ogg` | Bereit zum Ritt! | schnell, munter |
| `voices/units/scout_rider_select_02.ogg` | Was soll ich auskundschaften? | neugierig |
| `voices/units/scout_rider_select_03.ogg` | Gutes Pferd, gute Augen. | zufrieden |
| `voices/units/scout_rider_select_04.ogg` | Zeigt mir den Weg. | eifrig |

### scout_rider_move (Event `voice.scout_rider.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_move_01.ogg` | Los wie der Wind! | freudig |
| `voices/units/scout_rider_move_02.ogg` | Dem Staub voraus! | schnell |
| `voices/units/scout_rider_move_03.ogg` | Ich bin vor dem Abendessen zurück. | leichthin |
| `voices/units/scout_rider_move_04.ogg` | Hü! | Zuruf, kurz |

### scout_rider_attack (Event `voice.scout_rider.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_attack_01.ogg` | Hü! Reitet sie nieder! | laut |
| `voices/units/scout_rider_attack_02.ogg` | Schnell und sauber! | scharf |
| `voices/units/scout_rider_attack_03.ogg` | Für Brennhollow! | Schlachtruf |
| `voices/units/scout_rider_attack_04.ogg` | Na kommt schon! | herausfordernd |

### scout_rider_retreat (Event `voice.scout_rider.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_retreat_01.ogg` | Zu viele! Reiten! | panisch |
| `voices/units/scout_rider_retreat_02.ogg` | Rückzug, Rückzug! | gehetzt |
| `voices/units/scout_rider_retreat_03.ogg` | Schneller, Mädchen, schneller! | zum Pferd |

### scout_rider_hit (Event `voice.scout_rider.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_hit_01.ogg` | [gasps] Ah, meine Seite! | scharf |
| `voices/units/scout_rider_hit_02.ogg` | [grunts] Gestreift. | überspielt |
| `voices/units/scout_rider_hit_03.ogg` | [hisses] Halt durch, Mädchen! | besorgt |

### scout_rider_death (Event `voice.scout_rider.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_death_01.ogg` | [gasps] Sagt ihnen … was ich sah … | verhauchend |
| `voices/units/scout_rider_death_02.ogg` | [exhales] Reitet weiter … | leise |
| `voices/units/scout_rider_death_03.ogg` | [sighs] Der Wind … ist kalt … | erlöst |

### scout_rider_victory (Event `voice.scout_rider.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/scout_rider_victory_01.ogg` | Ich sagte doch, ich bin schnell! | lachend |
| `voices/units/scout_rider_victory_02.ogg` | Der Weg ist frei. | zufrieden |
| `voices/units/scout_rider_victory_03.ogg` | Zurück mit Neuigkeiten, wie versprochen. | munter |

### 12.5 Glutkrone – `horse_archer` (Berittener Bogenschütze)

**Stimme:** `AC_unit_horse_archer (Profil G5)` · **Modell:** Eleven v3, *Creative* · Speed 1,05
**Haltung:** geschmeidige Steppen-Söldnerin, selbstsicher, ein wenig spöttisch; schießt, bevor andere zielen.

### horse_archer_select (Event `voice.horse_archer.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_select_01.ogg` | Bogen gespannt, Pferd gefüttert. | selbstsicher |
| `voices/units/horse_archer_select_02.ogg` | Macht es zum bewegten Ziel. | spöttisch |
| `voices/units/horse_archer_select_03.ogg` | Ihr habt meinen Bogen. | ruhig |
| `voices/units/horse_archer_select_04.ogg` | Wohin, Wächterin? | locker |

### horse_archer_move (Event `voice.horse_archer.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_move_01.ogg` | Reite weit aus. | kurz |
| `voices/units/horse_archer_move_02.ogg` | Fangt mich, wenn ihr könnt. | neckend |
| `voices/units/horse_archer_move_03.ogg` | Flanke, dann Feuer. | knapp |
| `voices/units/horse_archer_move_04.ogg` | Hü! | Zuruf |

### horse_archer_attack (Event `voice.horse_archer.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_attack_01.ogg` | Pfeile los! | laut |
| `voices/units/horse_archer_attack_02.ogg` | Schießen und reiten! | rhythmisch |
| `voices/units/horse_archer_attack_03.ogg` | Zu langsam! | spöttisch |
| `voices/units/horse_archer_attack_04.ogg` | Lasst es auf sie regnen! | Schlachtruf |

### horse_archer_retreat (Event `voice.horse_archer.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_retreat_01.ogg` | Absetzen! Raus hier! | gehetzt |
| `voices/units/horse_archer_retreat_02.ogg` | Sie sind zu nah! | angespannt |
| `voices/units/horse_archer_retreat_03.ogg` | Rückzug, schießt beim Reiten! | dringlich |

### horse_archer_hit (Event `voice.horse_archer.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_hit_01.ogg` | [gasps] Ein Pfeil im Mantel! | scharf |
| `voices/units/horse_archer_hit_02.ogg` | [grunts] Nicht schlecht. | überspielt |
| `voices/units/horse_archer_hit_03.ogg` | [hisses] Ruhig, Mädchen! | zum Pferd |

### horse_archer_death (Event `voice.horse_archer.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_death_01.ogg` | [exhales] Letzter … Pfeil … | verhauchend |
| `voices/units/horse_archer_death_02.ogg` | [gasps] Der Wind hat mich geholt … | poetisch-leise |
| `voices/units/horse_archer_death_03.ogg` | [sighs] Reitet ohne mich weiter … | ruhig |

### horse_archer_victory (Event `voice.horse_archer.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/horse_archer_victory_01.ogg` | Sie kamen nie nah heran. | zufrieden |
| `voices/units/horse_archer_victory_02.ogg` | Köcher leicht, Herz leichter. | gelöst |
| `voices/units/horse_archer_victory_03.ogg` | Wieder eine Jagd vorbei. | ruhig |

### 12.6 Glutkrone – `battle_mage` (Kampfmagier, Feuerschule)

**Stimme:** `AC_unit_battle_mage (Profil G6)` · **Modell:** Eleven v3, *Creative* · Speed 1,0
**Haltung:** disziplinierter Feuermagier der Flammenakademie von Kaldris; höflich, gespannt, mit unterdrückter Hitze in der Stimme.

### battle_mage_select (Event `voice.battle_mage.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_select_01.ogg` | Die Flamme antwortet. | ruhig, tief |
| `voices/units/battle_mage_select_02.ogg` | Glut und Wille. | knapp |
| `voices/units/battle_mage_select_03.ogg` | Nennt das Ziel. | konzentriert |
| `voices/units/battle_mage_select_04.ogg` | Ich bin bereit zu brennen. | leise-intensiv |

### battle_mage_move (Event `voice.battle_mage.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_move_01.ogg` | Ich gehe im Licht. | gelassen |
| `voices/units/battle_mage_move_02.ogg` | Haltet Abstand zu meinem Saum. | trocken-warnend |
| `voices/units/battle_mage_move_03.ogg` | Wie die Flamme will. | ruhig |
| `voices/units/battle_mage_move_04.ogg` | Bewegung. | knapp |

### battle_mage_attack (Event `voice.battle_mage.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_attack_01.ogg` | Brennt! | scharf, laut |
| `voices/units/battle_mage_attack_02.ogg` | Asche zu Asche! | grimmig |
| `voices/units/battle_mage_attack_03.ogg` | Bei der Ewigen Flamme! | hymnisch |
| `voices/units/battle_mage_attack_04.ogg` | Entfacht! | knapp, Zauberruf |

### battle_mage_retreat (Event `voice.battle_mage.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_retreat_01.ogg` | Deckt mich, ich bin erschöpft! | dringlich |
| `voices/units/battle_mage_retreat_02.ogg` | Zu nah, zu nah! | panisch |
| `voices/units/battle_mage_retreat_03.ogg` | Zurück zur Linie! | gehetzt |

### battle_mage_hit (Event `voice.battle_mage.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_hit_01.ogg` | [gasps] Meine Konzentration! | scharf |
| `voices/units/battle_mage_hit_02.ogg` | [hisses] Das brennt … in die falsche Richtung. | bissig |
| `voices/units/battle_mage_hit_03.ogg` | [grunts] Ruhig bleiben. | zu sich selbst |

### battle_mage_death (Event `voice.battle_mage.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_death_01.ogg` | [exhales] Die Flamme … erlischt … | verhauchend |
| `voices/units/battle_mage_death_02.ogg` | [gasps] Lasst es … nicht umsonst sein … | leise |
| `voices/units/battle_mage_death_03.ogg` | [sighs] Asche … endlich. | erlöst |

### battle_mage_victory (Event `voice.battle_mage.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_victory_01.ogg` | Die Glut ist still. | zufrieden-ruhig |
| `voices/units/battle_mage_victory_02.ogg` | Lasst das Feuer ruhen. | leise |
| `voices/units/battle_mage_victory_03.ogg` | Das Licht hat gesiegt. | feierlich |

### battle_mage_ability (Event `voice.battle_mage.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battle_mage_ability_01.ogg` | Feuer, antworte mir! | laut, Zauberruf |
| `voices/units/battle_mage_ability_02.ogg` | Funken, erhebt euch! | scharf |
| `voices/units/battle_mage_ability_03.ogg` | Flamme, nimm sie! | hart |

### 12.7 Glutkrone – `priest` (Priester der Flamme)

**Stimme:** `AC_unit_priest (Profil G7)` · **Modell:** Eleven v3, *Natural* · Speed 0,96
**Haltung:** sanfter, warmer Flammenpriester der Dorfkapelle; zitiert Liturgie, hat im Kampf Angst, aber hilft trotzdem.

### priest_select (Event `voice.priest.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_select_01.ogg` | Die Flamme sei mit Euch. | warm, sanft |
| `voices/units/priest_select_02.ogg` | Wie kann ich dienen? | freundlich |
| `voices/units/priest_select_03.ogg` | Ich bin hier, Kind. | tröstend |
| `voices/units/priest_select_04.ogg` | Lasst das Licht uns leiten. | ruhig |

### priest_move (Event `voice.priest.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_move_01.ogg` | Wir gehen im Licht. | gelassen |
| `voices/units/priest_move_02.ogg` | Geht voran, ich folge. | ruhig |
| `voices/units/priest_move_03.ogg` | Gebt auf die Verwundeten acht. | sorgend |
| `voices/units/priest_move_04.ogg` | Ja, ja, ich gehe. | leicht gehetzt |

### priest_attack (Event `voice.priest.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_attack_01.ogg` | Zurück, Geschöpfe der Dunkelheit! | mutig, zitternd |
| `voices/units/priest_attack_02.ogg` | Flamme, beschütze uns! | laut |
| `voices/units/priest_attack_03.ogg` | Im Namen der Ewigen Flamme! | hymnisch |
| `voices/units/priest_attack_04.ogg` | Weg von ihnen! | entschlossen |

### priest_retreat (Event `voice.priest.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_retreat_01.ogg` | Ich kann nicht! Ich kann nicht! | panisch |
| `voices/units/priest_retreat_02.ogg` | Beschützt die Heiler! | dringlich |
| `voices/units/priest_retreat_03.ogg` | Flamme, vergib mir! | verzweifelt |

### priest_hit (Event `voice.priest.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_hit_01.ogg` | [gasps] Oh! Gnade! | erschrocken |
| `voices/units/priest_hit_02.ogg` | [groans] Es ist nur Schmerz. | tapfer |
| `voices/units/priest_hit_03.ogg` | [hisses] Vergib ihnen … | gepresst |

### priest_death (Event `voice.priest.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_death_01.ogg` | [exhales] In die Flamme … gehe ich … | friedlich |
| `voices/units/priest_death_02.ogg` | [gasps] Wer … wird … sie pflegen … | besorgt |
| `voices/units/priest_death_03.ogg` | [sighs] Licht … nimm mich. | erlöst |

### priest_victory (Event `voice.priest.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_victory_01.ogg` | Lob der Flamme. Versorgt die Verwundeten. | erleichtert |
| `voices/units/priest_victory_02.ogg` | Wir leben. Dank dem Licht. | gerührt |
| `voices/units/priest_victory_03.ogg` | Friede den Gefallenen. | feierlich |

### priest_ability (Event `voice.priest.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/priest_ability_01.ogg` | Werdet geheilt! | warm, kräftig |
| `voices/units/priest_ability_02.ogg` | Flamme, richte sie auf! | hymnisch |
| `voices/units/priest_ability_03.ogg` | Erhebt euch und seid gesegnet! | strahlend |

### 12.8 Glutkrone – `battering_ram` (Rammbock, Bedienmannschaft)

**Stimme:** `AC_unit_ram_crew (Profil G8)` · **Modell:** Eleven v3, *Creative* (Mehrstimmen-Chor, 3–4 Männer) · Speed 1,0
**Haltung:** Gruppe schwitzender Pioniere; rufen im Takt, Stoßrufe, Hau-Ruck. Mehrere Sprecher überlagert, dumpfe Anstrengung.

### battering_ram_select (Event `voice.battering_ram.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battering_ram_select_01.ogg` | Ramme bereit! | Chor, Zuruf |
| `voices/units/battering_ram_select_02.ogg` | Jawohl, das Tor hält nicht. | grimmig, rau |
| `voices/units/battering_ram_select_03.ogg` | Legt Euch ins Zeug! | Anfeuerung |
| `voices/units/battering_ram_select_04.ogg` | Hau ruck, Jungs! | rhythmisch |

### battering_ram_move (Event `voice.battering_ram.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battering_ram_move_01.ogg` | Schieben, schieben! | Anstrengung, im Takt |
| `voices/units/battering_ram_move_02.ogg` | Achtet auf die Räder! | hektisch |
| `voices/units/battering_ram_move_03.ogg` | Langsam … langsam … | gepresst |
| `voices/units/battering_ram_move_04.ogg` | Rollt sie vor! | Zuruf |

### battering_ram_attack (Event `voice.battering_ram.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battering_ram_attack_01.ogg` | Schwingen! Schwingen! | laut, im Takt |
| `voices/units/battering_ram_attack_02.ogg` | Reißt es nieder! | wild |
| `voices/units/battering_ram_attack_03.ogg` | Alle zusammen – jetzt! | Chor |
| `voices/units/battering_ram_attack_04.ogg` | Zerschmettert das Tor! | Schlachtruf |

### battering_ram_death (Event `voice.battering_ram.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/battering_ram_death_01.ogg` | [shouting] Sie bricht auseinander! | panisch, Chor |
| `voices/units/battering_ram_death_02.ogg` | [groans] Der Rammbock … ist verloren … | gebrochen |
| `voices/units/battering_ram_death_03.ogg` | [shouting] Lauft, Jungs, lauft! | Chor, hektisch |

### 12.9 Glutkrone – `catapult` (Katapult, Bedienmannschaft)

**Stimme:** `AC_unit_catapult_crew (Profil G9)` · **Modell:** Eleven v3, *Creative* (2–3 Sprecher) · Speed 1,0
**Haltung:** erfahrene Geschützmeister, nüchterne Zurufe in Kommandoform, Winden und Seile hörbar.

### catapult_select (Event `voice.catapult.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/catapult_select_01.ogg` | Geschütz ist geladen. | nüchtern |
| `voices/units/catapult_select_02.ogg` | Gebt uns ein Ziel. | knapp |
| `voices/units/catapult_select_03.ogg` | Spannt sie zurück! | Zuruf |
| `voices/units/catapult_select_04.ogg` | Reichweite und Richtung, Herr? | fachlich |

### catapult_move (Event `voice.catapult.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/catapult_move_01.ogg` | Zieht sie, zieht sie! | Anstrengung |
| `voices/units/catapult_move_02.ogg` | Vorsicht mit dem Arm! | besorgt |
| `voices/units/catapult_move_03.ogg` | Achtet auf die Spurrillen! | hektisch |
| `voices/units/catapult_move_04.ogg` | Langsam, langsam! | gepresst |

### catapult_attack (Event `voice.catapult.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/catapult_attack_01.ogg` | Los! | laut, Kommando |
| `voices/units/catapult_attack_02.ogg` | Feuer! | scharf |
| `voices/units/catapult_attack_03.ogg` | Zurücktreten – los! | Warnruf |
| `voices/units/catapult_attack_04.ogg` | Fort damit! | Triumph |

### catapult_death (Event `voice.catapult.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/catapult_death_01.ogg` | [shouting] Sie ist hin – der Arm ist gebrochen! | entsetzt |
| `voices/units/catapult_death_02.ogg` | [groans] Jahre an Arbeit … | gebrochen |
| `voices/units/catapult_death_03.ogg` | [shouting] Weg hier! | panisch |


---

## 13. Echtzeit-Militär – Helden Hal Venn und Torvald

Heldenstimmen gelten für `voices/heroes/` (Manifest +1 dB, Cooldown 300 ms). Situationen wie bei Isolde (Abschnitt 9): select, move, attack, retreat, hit, death, victory, ability, greet, levelup, lowhp.

### 13.1 Held – `hero_hal_venn` (Bruder Hal Venn, Flammenpriester von Brennhollow)

**Stimme:** `AC_hero_hal_venn (nutzt Profil von `char_hal_venn`, siehe Figurenleitfaden)` · **Modell:** Eleven v3, *Natural* · Speed 0,97
**Haltung:** freundlich-väterlicher Mentor, bedächtig, mit trockenem Humor; im Kampf überraschend entschlossen. Gleiche Stimme wie die Dialogfigur `char_hal_venn`.

### hero_hal_venn_select (Event `voice.hero_hal_venn.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_select_01.ogg` | Ja, Wächterin? | warm |
| `voices/heroes/hero_hal_venn_select_02.ogg` | Die Flamme ist mit uns. | ruhig |
| `voices/heroes/hero_hal_venn_select_03.ogg` | Meine Hände gehören Euch. | sanft |
| `voices/heroes/hero_hal_venn_select_04.ogg` | Ach, alte Knochen, bereit wie immer. | selbstironisch |
| `voices/heroes/hero_hal_venn_select_05.ogg` | Was braucht Brennhollow von mir? | sorgend |

### hero_hal_venn_move (Event `voice.hero_hal_venn.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_move_01.ogg` | Geht voran, ich folge. | ruhig |
| `voices/heroes/hero_hal_venn_move_02.ogg` | Langsam, meine Knie erinnern sich an Aureth. | trocken |
| `voices/heroes/hero_hal_venn_move_03.ogg` | Dorthin, wo die Not am größten ist. | ernst |
| `voices/heroes/hero_hal_venn_move_04.ogg` | Ich gehe, ich gehe. | gemütlich |
| `voices/heroes/hero_hal_venn_move_05.ogg` | Dann vorwärts. | neutral |

### hero_hal_venn_attack (Event `voice.hero_hal_venn.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_attack_01.ogg` | Im Namen der Flamme! | kräftig |
| `voices/heroes/hero_hal_venn_attack_02.ogg` | Fort mit euch, Schatten! | laut |
| `voices/heroes/hero_hal_venn_attack_03.ogg` | Genug! Zurück! | scharf |
| `voices/heroes/hero_hal_venn_attack_04.ogg` | Licht, triff sicher! | hymnisch |
| `voices/heroes/hero_hal_venn_attack_05.ogg` | Ich dulde kein weiteres verlorenes Dorf! | entschlossen |

### hero_hal_venn_retreat (Event `voice.hero_hal_venn.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_retreat_01.ogg` | Zurück, zurück! | dringlich |
| `voices/heroes/hero_hal_venn_retreat_02.ogg` | Sammelt die Verwundeten! | sorgend-laut |
| `voices/heroes/hero_hal_venn_retreat_03.ogg` | Wir leben, um wieder zu kämpfen! | angespannt |

### hero_hal_venn_hit (Event `voice.hero_hal_venn.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_hit_01.ogg` | [grunts] Hm. Das hat gebrannt. | trocken |
| `voices/heroes/hero_hal_venn_hit_02.ogg` | [gasps] Flamme, halte mich! | erschrocken |
| `voices/heroes/hero_hal_venn_hit_03.ogg` | [groans] Alt, nicht tot! | trotzig |

### hero_hal_venn_death (Event `voice.hero_hal_venn.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_death_01.ogg` | [exhales] Wächterin … die Flamme … behüte Euch … | verhauchend, warm |
| `voices/heroes/hero_hal_venn_death_02.ogg` | [gasps] Sagt Brennhollow … ich habe es versucht … | leise |
| `voices/heroes/hero_hal_venn_death_03.ogg` | [sighs] Ins Licht … | erlöst |

### hero_hal_venn_victory (Event `voice.hero_hal_venn.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_victory_01.ogg` | Die Flamme hat gesiegt. | erleichtert |
| `voices/heroes/hero_hal_venn_victory_02.ogg` | Gelobt sei die Flamme, wir leben. | gerührt |
| `voices/heroes/hero_hal_venn_victory_03.ogg` | Lasst uns die Verwundeten versorgen. | ruhig |

### hero_hal_venn_ability (Event `voice.hero_hal_venn.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_ability_01.ogg` | Flamme, heile sie! | warm, laut |
| `voices/heroes/hero_hal_venn_ability_02.ogg` | Brennt rein! | scharf |
| `voices/heroes/hero_hal_venn_ability_03.ogg` | Glaube sei unser Feuer! | hymnisch |

### hero_hal_venn_greet (Event `voice.hero_hal_venn.greet`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_greet_01.ogg` | Wächterin. Schön, Euch aufrecht zu sehen. | warm |
| `voices/heroes/hero_hal_venn_greet_02.ogg` | Die Siedlung braucht Euch, und ich auch. | ernst-freundlich |
| `voices/heroes/hero_hal_venn_greet_03.ogg` | Ich habe die Lampe am Brennen gehalten. | sanft |

### hero_hal_venn_levelup (Event `voice.hero_hal_venn.levelup`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_levelup_01.ogg` | Die Flamme brennt heller in mir. | überrascht, froh |
| `voices/heroes/hero_hal_venn_levelup_02.ogg` | Stärker. Und das macht demütig. | bescheiden |
| `voices/heroes/hero_hal_venn_levelup_03.ogg` | Die Gnade wächst. | leise |

### hero_hal_venn_lowhp (Event `voice.hero_hal_venn.lowhp`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_hal_venn_lowhp_01.ogg` | Ich muss … mich selbst versorgen … | angestrengt |
| `voices/heroes/hero_hal_venn_lowhp_02.ogg` | Noch nicht, Flamme, noch nicht. | gepresst |
| `voices/heroes/hero_hal_venn_lowhp_03.ogg` | Ich blute. Natürlich. | trocken-schwach |

### 13.2 Held – `hero_torvald` (Torvald Eisenhand, Clanfürst von Skarnheim)

**Stimme:** `AC_hero_torvald (nutzt Profil von `char_torvald`, siehe Figurenleitfaden)` · **Modell:** Eleven v3, *Creative* · Speed 0,95
**Haltung:** raues, tiefes Nordclan-Organ, laut, grobschlächtig-herzlich; ehrt Ahnen, verachtet Feigheit, trinkt gern.

### hero_torvald_select (Event `voice.hero_torvald.select`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_select_01.ogg` | Torvald Eisenhand hört zu. | tief, kräftig |
| `voices/heroes/hero_torvald_select_02.ogg` | Sprecht, und macht's kurz. | ungeduldig |
| `voices/heroes/hero_torvald_select_03.ogg` | Die Ahnen sehen zu. | ernst |
| `voices/heroes/hero_torvald_select_04.ogg` | Ha! Ein Kampf, hoffe ich? | gut gelaunt |
| `voices/heroes/hero_torvald_select_05.ogg` | Skarnheim steht an Eurer Seite. | ehrlich |

### hero_torvald_move (Event `voice.hero_torvald.move`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_move_01.ogg` | Der Weg ist lang, die Axt ist leicht! | grob-fröhlich |
| `voices/heroes/hero_torvald_move_02.ogg` | Vorwärts, vorwärts! | laut |
| `voices/heroes/hero_torvald_move_03.ogg` | Meine Füße kennen den Berg, nicht diesen Matsch. | brummig |
| `voices/heroes/hero_torvald_move_04.ogg` | Führt an, Wächterin! | herzlich |
| `voices/heroes/hero_torvald_move_05.ogg` | Dann zur nächsten Kehle. | trocken |

### hero_torvald_attack (Event `voice.hero_torvald.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_attack_01.ogg` | Für die Clans! | Schlachtruf, laut |
| `voices/heroes/hero_torvald_attack_02.ogg` | Zerschmettert sie! | wild |
| `voices/heroes/hero_torvald_attack_03.ogg` | Ahnen, seht mir zu! | kehlig |
| `voices/heroes/hero_torvald_attack_04.ogg` | Steht und sterbt! | hart |
| `voices/heroes/hero_torvald_attack_05.ogg` | Hahh! Kommt ans Eisen! | brüllend |

### hero_torvald_retreat (Event `voice.hero_torvald.retreat`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_retreat_01.ogg` | Ein Clanmann weicht vor niemandem … heute. | knurrend |
| `voices/heroes/hero_torvald_retreat_02.ogg` | Zurück zu den Schilden! | laut |
| `voices/heroes/hero_torvald_retreat_03.ogg` | Zurück, beim Bart meines Vaters! | fluchend |

### hero_torvald_hit (Event `voice.hero_torvald.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_hit_01.ogg` | [grunts] Ist das alles? | höhnisch |
| `voices/heroes/hero_torvald_hit_02.ogg` | [roars] Der hat Blut gezogen! | wütend |
| `voices/heroes/hero_torvald_hit_03.ogg` | [groans] Hrnn! | dumpf |

### hero_torvald_death (Event `voice.hero_torvald.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_death_01.ogg` | [exhales] Skarnheim … erinnere dich … | verhauchend |
| `voices/heroes/hero_torvald_death_02.ogg` | [gasps] Vater … ich komme … | friedlich-leise |
| `voices/heroes/hero_torvald_death_03.ogg` | [roars weakly] Der … Berg … ruft … | gebrochen, rau |

### hero_torvald_victory (Event `voice.hero_torvald.victory`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_victory_01.ogg` | Ha! Darauf trinken wir! | dröhnend |
| `voices/heroes/hero_torvald_victory_02.ogg` | Die Ahnen sind zufrieden. | zufrieden |
| `voices/heroes/hero_torvald_victory_03.ogg` | Keiner kann sagen, die Clans beugten sich. | stolz |

### hero_torvald_ability (Event `voice.hero_torvald.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_ability_01.ogg` | Hört mich, Berge! | laut, Ruf |
| `voices/heroes/hero_torvald_ability_02.ogg` | Ahnen, leiht mir Eisen! | kehlig |
| `voices/heroes/hero_torvald_ability_03.ogg` | Brecht! | kurz, hart |

### hero_torvald_greet (Event `voice.hero_torvald.greet`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_greet_01.ogg` | Wächterin Isolde! Euer Bier ist dünn, Eure Sache gut. | herzlich-grob |
| `voices/heroes/hero_torvald_greet_02.ogg` | Skarnheim schickt Eisen, und mich. | stolz |
| `voices/heroes/hero_torvald_greet_03.ogg` | Richtet mich auf den Feind. | ungeduldig |

### hero_torvald_levelup (Event `voice.hero_torvald.levelup`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_levelup_01.ogg` | Die Kraft der Ahnen wächst in mir! | dröhnend |
| `voices/heroes/hero_torvald_levelup_02.ogg` | Schwerere Fäuste, leichteres Herz! | lachend |
| `voices/heroes/hero_torvald_levelup_03.ogg` | Mehr Eisen im Blut! | froh |

### hero_torvald_lowhp (Event `voice.hero_torvald.lowhp`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/heroes/hero_torvald_lowhp_01.ogg` | Ich hatte Schlimmeres – glaube ich. | keuchend-trotzig |
| `voices/heroes/hero_torvald_lowhp_02.ogg` | Noch … nicht … Ahnen! | gepresst |
| `voices/heroes/hero_torvald_lowhp_03.ogg` | Mein Blut verlässt mich … | schwach |


---

## 14. Echtzeit-Militär – Die Hohlen (neue Einheiten)

Hohle Stimmen laufen im Manifest mit −2 dB, Cooldown 400 ms, max. 2 Instanzen; Nachbearbeitung wie Abschnitt 6 (Hohle-Kette, Pitch −2), Loudness −17 LUFS.

### 14.1 Hohle – `hollow_digger` (Grabgräber)

**Stimme:** `AC_unit_hollow_digger (Profil G10)` · **Modell:** Eleven v3, *Creative* · Speed 0,9
**Haltung:** arbeitsamer Wiedergänger, der im Tod weiter schürft; murmelt Fetzen von Werkzeugen und Schichten. Wie `hollow_thrall`: Hohle-Kette, Pitch −2.

### hollow_digger_attack (Event `voice.hollow_digger.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_digger_attack_01.ogg` | [rasping] Mein … mein … | gierig, kehlig |
| `voices/units/hollow_digger_attack_02.ogg` | [groans] Nicht die … Schaufel … | klagend |
| `voices/units/hollow_digger_attack_03.ogg` | [hisses] Hhhaah! | wortlos |
| `voices/units/hollow_digger_attack_04.ogg` | … Schicht … nicht vorbei … | gemurmelt |

### hollow_digger_hit (Event `voice.hollow_digger.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_digger_hit_01.ogg` | [groans] Uhh – zurück an die Arbeit … | gleichgültig |
| `voices/units/hollow_digger_hit_02.ogg` | [gasps] Hhkk – | trockenes Keuchen |
| `voices/units/hollow_digger_hit_03.ogg` | … Staub … in den … Augen … | verloren |

### hollow_digger_death (Event `voice.hollow_digger.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/hollow_digger_death_01.ogg` | [sighs] … Schicht … vorbei … | erlöst |
| `voices/units/hollow_digger_death_02.ogg` | [exhales] … tiefer … tiefer … | verhauchend |
| `voices/units/hollow_digger_death_03.ogg` | [rattling breath] Hhhaaahh … | langes Röcheln |

### 14.2 Hohle – `ghoul` (Ghul)

**Stimme:** `AC_unit_ghoul (Profil G11)` · **Modell:** Eleven v3, *Creative* + SFX-Layer · Speed 1,1
**Haltung:** ausgehungerte Leichenfresser: hohes Kreischen, Schmatzen, Schnappen; fast keine Wörter. Basis: Voice Changer oder SFX-Prompt, danach Hohle-Kette.

### ghoul_attack (Event `voice.ghoul.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ghoul_attack_01.ogg` | [screeches] Hhheeeee! | hohes Kreischen |
| `voices/units/ghoul_attack_02.ogg` | [snarls] Hunger … Hunger … | gierig, schmatzend |
| `voices/units/ghoul_attack_03.ogg` | [hisses] Frisch … frisch … | lüstern |
| `voices/units/ghoul_attack_04.ogg` | [wet growl] Grrrhhh! | tiefes, nasses Knurren |

### ghoul_hit (Event `voice.ghoul.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ghoul_hit_01.ogg` | [yelps] Hkkh! | kurzer Aufschrei |
| `voices/units/ghoul_hit_02.ogg` | [snarls] Hrrrk! | wütend |
| `voices/units/ghoul_hit_03.ogg` | [whines] Hnnn … | kläglich |

### ghoul_death (Event `voice.ghoul.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ghoul_death_01.ogg` | [shrieks] Heeeaaah … | abfallendes Kreischen |
| `voices/units/ghoul_death_02.ogg` | [gurgles] Hunger … noch … | röchelnd |
| `voices/units/ghoul_death_03.ogg` | [rattling breath] Hhhh … | dünnes Ausatmen |

### 14.3 Hohle – `ash_witch` (Aschhexe)

**Stimme:** `AC_unit_ash_witch (Profil G12)` · **Modell:** Eleven v3, *Creative* mit `[whispers]` · Speed 0,92
**Haltung:** flüsternde Nekromantin, kalt-berechnend, spricht wie eine Lehrerin zu ihren Toten; Hohle-Kette mit mehr Hall-Anteil.

### ash_witch_attack (Event `voice.ash_witch.attack`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ash_witch_attack_01.ogg` | [whispers] Brennt für mich. | kalt, flüsternd |
| `voices/units/ash_witch_attack_02.ogg` | [hisses] Asche und Knochen. | scharf |
| `voices/units/ash_witch_attack_03.ogg` | [whispers] Du warst einmal warm. | spöttisch |
| `voices/units/ash_witch_attack_04.ogg` | [cold laugh] Kommt näher. | leise-lockend |

### ash_witch_hit (Event `voice.ash_witch.hit`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ash_witch_hit_01.ogg` | [hisses] Unverschämtes Fleisch! | gereizt |
| `voices/units/ash_witch_hit_02.ogg` | [gasps] Wie … warm. | verblüfft |
| `voices/units/ash_witch_hit_03.ogg` | [hisses] Hnh! | kurz |

### ash_witch_death (Event `voice.ash_witch.death`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ash_witch_death_01.ogg` | [exhales] Das Tief … ist geduldig … | verhauchend |
| `voices/units/ash_witch_death_02.ogg` | [whispers] Erhebt euch … ohne mich … | leise |
| `voices/units/ash_witch_death_03.ogg` | [rattling breath] Staub zu Staub … | Röcheln |

### ash_witch_ability (Event `voice.ash_witch.ability`)
| Ziel-Datei | Sprechtext (DE) | Regie |
|---|---|---|
| `voices/units/ash_witch_ability_01.ogg` | [whispers] Erhebt euch, meine Kinder. | beschwörend, tief |
| `voices/units/ash_witch_ability_02.ogg` | [commanding] Hoch aus der Asche! | befehlend |
| `voices/units/ash_witch_ability_03.ogg` | [chants] Knochen zu Knochen, Fleisch zu Fleisch. | Singsang, rituell |

### 14.4 Hohle – `bat_swarm` (Fledermausschwarm)

**Stimme:** `kein Sprecher – reiner SFX-Entwurf` · **Modell:** Sound Effects (kein TTS) · Dauer je Datei 0,6–2,0 s
**Haltung:** Wolke aus Aschfledermäusen: Zirpen, Flügelschlag, Kreischen im Ultraschallrand; Mono, trocken. Voice-Events werden aus SFX-Prompts erzeugt.

### bat_swarm_attack (Event `voice.bat_swarm.attack`)
| Ziel-Datei | EN-Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/bat_swarm_attack_01.ogg` | swarm of bats diving, dense leathery wing flutter with sharp shrieks, close, dry | Fledermausschwarm stürzt herab | Flügelflattern + Kreischen |
| `voices/units/bat_swarm_attack_02.ogg` | many small bats screeching and snapping, rapid chirps, close | kreischende, schnappende Fledermäuse | schnelle Zirplaute |
| `voices/units/bat_swarm_attack_03.ogg` | angry bat swarm passing overhead, whooshing wings and high shrieks | wütender Schwarm zieht vorbei | Vorbeiflug |

### bat_swarm_hit (Event `voice.bat_swarm.hit`)
| Ziel-Datei | EN-Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/bat_swarm_hit_01.ogg` | bat swarm scattering after a hit, startled chirps and wing burst | Schwarm stiebt nach Treffer auseinander | kurz, erschrocken |
| `voices/units/bat_swarm_hit_02.ogg` | a few bats squealing in pain, short sharp squeals | wenige Fledermäuse quieken | kurz |
| `voices/units/bat_swarm_hit_03.ogg` | leathery flap burst with a single shriek, quick | ein Kreischen im Flügelschlag | sehr kurz |

### bat_swarm_death (Event `voice.bat_swarm.death`)
| Ziel-Datei | EN-Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/bat_swarm_death_01.ogg` | bat swarm dropping to the ground, falling flutter fading into silence | Schwarm fällt zu Boden | ausklingend |
| `voices/units/bat_swarm_death_02.ogg` | many bats shrieking and thudding onto dirt, fading | Fledermäuse schreien und schlagen auf | mit Aufprall |
| `voices/units/bat_swarm_death_03.ogg` | last fluttering wings and soft chirps dying out | letztes Flattern, Zirpen erstirbt | leise |
