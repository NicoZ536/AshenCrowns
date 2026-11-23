# ASHEN CROWNS – Einheiten-, Helden- und Boss-Stimmen (ElevenLabs)

Grundlage: `assets/audio/manifest.ron` (Events `voice.*`), Stimmprofile in `docs/audio/elevenlabs_characters.md`,
Produktionsregeln in `docs/audio/elevenlabs_production_guide.md`.

**Legende**
- **Ziel-Datei:** Pfad relativ zu `assets/audio/`. Exakt so benennen (`.ogg`, Vorbis q6, 48 kHz, mono).
- **EN-Text:** Generierungstext. Eckige Klammern = **Eleven-v3-Audio-Tags** (werden nicht gesprochen; nicht in Untertitel übernehmen).
- **DE:** kurze Bedeutungsübersetzung für spätere Lokalisierung.
- **Regie:** Spielanweisung; Emotion, Lautstärke, Tempo, Besonderheiten.
- **Batch-Methode:** Alle Zeilen *einer* Tabelle (gleiche Situation) möglichst in **einer** v3-Generierung erzeugen,
  getrennt durch Absatz und „…“, danach schneiden. Kampf-/Schmerzlaute einzeln generieren, wenn die Emotion „überläuft“.
- **Variantenzahl:** select/move/attack 4–5, alle anderen Situationen mindestens 3 (gemäß Auftrag).
- **Lautheit:** −16 LUFS (Short-Term-Max) für Glutkrone/Heldin/Boss, −17 für Hohle; Peak −1 dBTP (siehe Leitfaden, Abschnitt 7).
- Modellnamen und Einstellungsoptionen zum Produktionszeitpunkt in der ElevenLabs-Oberfläche prüfen.

---

## 1. Glutkrone – `worker` (Leibeigener/Arbeiter)

**Stimme:** `AC_unit_worker` (Profil D1.1) · **Modell:** Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,98
**Haltung:** gutmütig-müder Bauer, West-Country-Landakzent, kein Soldat – Kampfzeilen klingen improvisiert und etwas ängstlich.

### worker_select (Event `voice.worker.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_select_01.ogg` | Aye, what needs doing? | Jo, was gibt's zu tun? | freundlich-willig, mittlere Lautstärke |
| `voices/units/worker_select_02.ogg` | [sighs] Another job, is it? | Noch 'ne Arbeit, was? | kurzer Seufzer, dann gutmütig |
| `voices/units/worker_select_03.ogg` | Got my hands free. For now. | Hab die Hände frei. Noch. | trocken, kleine Pause vor „For now“ |
| `voices/units/worker_select_04.ogg` | Yes, Warden? I'm listening. | Ja, Wächterin? Ich hör zu. | respektvoll, leicht unterwürfig |
| `voices/units/worker_select_05.ogg` | Back's sore, but I'm here. | Rücken tut weh, aber ich bin da. | Ächzen in der Stimme, lächelnd |

### worker_move (Event `voice.worker.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_move_01.ogg` | Right, on my way. | Gut, bin unterwegs. | knapp, bodenständig |
| `voices/units/worker_move_02.ogg` | Off I go, then. | Dann mal los. | leichtes Brummen, gemütlich |
| `voices/units/worker_move_03.ogg` | Mind the mud… going. | Pass auf den Schlamm auf … geh ja schon. | zu sich selbst, halblaut |
| `voices/units/worker_move_04.ogg` | Aye, I know the path. | Jo, ich kenn den Weg. | selbstsicher |
| `voices/units/worker_move_05.ogg` | [tired] Walking, walking. | Geh ja, geh ja. | müde, leicht genervt |

### worker_attack (Event `voice.worker.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_attack_01.ogg` | [shouting] Get away from our homes! | Weg von unseren Häusern! | verzweifelt-wütend, laut |
| `voices/units/worker_attack_02.ogg` | I've a pitchfork, and I'm not afraid to use it! | Ich hab 'ne Mistgabel und scheu mich nicht! | Mut zusammengekratzt, Stimme kippt leicht |
| `voices/units/worker_attack_03.ogg` | [shouting] For Brennhollow! | Für Brennhollow! | Dorfstolz, kräftig |
| `voices/units/worker_attack_04.ogg` | Alright, alright — have at you! | Schon gut, schon gut – nimm das! | nervös, sich selbst Mut machend |

### worker_retreat (Event `voice.worker.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_retreat_01.ogg` | Not paid enough for this! Run! | Dafür werd ich nicht bezahlt! Lauft! | panisch, gehetzt, mit Atem |
| `voices/units/worker_retreat_02.ogg` | [shouting] Back to the village! | Zurück ins Dorf! | laut, Angst |
| `voices/units/worker_retreat_03.ogg` | Every man for himself — I mean, fall back! | Jeder für sich – ich mein, Rückzug! | hastige Selbstkorrektur, unfreiwillig komisch |

### worker_hit (Event `voice.worker.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_hit_01.ogg` | [groans] Ow! Blast it — | Au! Verflucht – | Schmerz + Ärger, abgebrochen |
| `voices/units/worker_hit_02.ogg` | [gasps] Argh, my arm! | Argh, mein Arm! | erschrocken, scharf |
| `voices/units/worker_hit_03.ogg` | [grunts] Oof! | Uff! | kurzer Stoßlaut, ≤ 0,6 s |

### worker_death (Event `voice.worker.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_death_01.ogg` | [gasps] Tell my wife… the roof… | Sag meiner Frau … das Dach … | schwach, verlöschend, letzte Sorge |
| `voices/units/worker_death_02.ogg` | [groans] Ahh… so cold… | Ahh … so kalt … | leiser werdend |
| `voices/units/worker_death_03.ogg` | No… not like this — [exhales] | Nein … nicht so – | abgebrochen, letzter Atem |

### worker_victory (Event `voice.worker.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_victory_01.ogg` | [laughs] Ha! Still standing! | Ha! Steh noch! | erleichtertes Lachen |
| `voices/units/worker_victory_02.ogg` | That'll teach 'em to come round here. | Das lehrt sie, hier aufzukreuzen. | stolz, brummig |
| `voices/units/worker_victory_03.ogg` | We did it… we actually did it! | Wir haben's geschafft … wirklich geschafft! | ungläubige Freude |

### worker_work (Event `voice.worker.work`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_work_01.ogg` | Back to the grindstone. | Zurück an die Arbeit. | ergeben, freundlich |
| `voices/units/worker_work_02.ogg` | Chop, haul, stack. Chop, haul, stack. | Hacken, schleppen, stapeln. | rhythmisch, halb gesungen |
| `voices/units/worker_work_03.ogg` | Good timber, this. | Gutes Holz, das. | zufrieden, fachmännisch |
| `voices/units/worker_work_04.ogg` | Field won't plough itself. | Das Feld pflügt sich nicht von allein. | Volksweisheit, trocken |

### worker_build_done (Event `voice.worker.build_done`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/worker_build_done_01.ogg` | There. Built to last. Mostly. | So. Gebaut für die Ewigkeit. Meistens. | stolz, dann trockene Einschränkung |
| `voices/units/worker_build_done_02.ogg` | Job's done, Warden! | Arbeit erledigt, Wächterin! | laut genug als Meldung |
| `voices/units/worker_build_done_03.ogg` | Finished! Come and have a look! | Fertig! Kommt, schaut's euch an! | freudig |

---

## 2. Glutkrone – `swordsman` (Schwertkämpfer, Miliz)

**Stimme:** `AC_unit_swordsman` (Profil D1.2) · **Modell:** Eleven v3, *Natural* (attack/hit/death *Creative*)
**Haltung:** junger Milizsoldat, zackig, Galgenhumor, nordenglischer Stadtakzent.

### swordsman_select (Event `voice.swordsman.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_select_01.ogg` | Sword ready. | Schwert bereit. | zackig, knapp |
| `voices/units/swordsman_select_02.ogg` | Militia, at your command. | Miliz, zu Befehl. | soldatisch, Haltung annehmen |
| `voices/units/swordsman_select_03.ogg` | Point me at the trouble. | Zeigt mir den Ärger. | entschlossen, leicht grinsend |
| `voices/units/swordsman_select_04.ogg` | Shield's up. What now? | Schild oben. Was jetzt? | wach, aufmerksam |
| `voices/units/swordsman_select_05.ogg` | Still breathing. What's the order? | Atme noch. Was ist der Befehl? | Galgenhumor |

### swordsman_move (Event `voice.swordsman.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_move_01.ogg` | Moving out. | Rücken aus. | knapp |
| `voices/units/swordsman_move_02.ogg` | On the double! | Im Laufschritt! | energisch |
| `voices/units/swordsman_move_03.ogg` | Right behind you, Warden. | Direkt hinter Euch, Wächterin. | loyal |
| `voices/units/swordsman_move_04.ogg` | Marching. | Marschieren. | neutral, militärisch |
| `voices/units/swordsman_move_05.ogg` | Through the ash, then. | Dann eben durch die Asche. | leicht resigniert, entschlossen |

### swordsman_attack (Event `voice.swordsman.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_attack_01.ogg` | [shouting] Cut them down! | Haut sie nieder! | Kampfschrei, voll |
| `voices/units/swordsman_attack_02.ogg` | [shouting] For the Ember Crown! | Für die Glutkrone! | Schlachtruf, Stolz |
| `voices/units/swordsman_attack_03.ogg` | Steel, lads! Steel! | Stahl, Jungs! Stahl! | anfeuernd |
| `voices/units/swordsman_attack_04.ogg` | Back to your graves! | Zurück in eure Gräber! | wütend, verächtlich |
| `voices/units/swordsman_attack_05.ogg` | [grunts] Have some of this! | Da, nimm das! | Anstrengung im Schlag |

### swordsman_retreat (Event `voice.swordsman.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_retreat_01.ogg` | Fall back! Fall back! | Rückzug! Rückzug! | laut, dringlich |
| `voices/units/swordsman_retreat_02.ogg` | Regroup at the keep! | Sammeln am Bergfried! | befehlend, außer Atem |
| `voices/units/swordsman_retreat_03.ogg` | Pull back, we're overrun! | Zurück, wir werden überrannt! | Angst, gehetzt |

### swordsman_hit (Event `voice.swordsman.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_hit_01.ogg` | [grunts] Argh! | Argh! | kurzer Schmerzlaut |
| `voices/units/swordsman_hit_02.ogg` | [groans] Ngh — just a scratch! | Ngh – nur ein Kratzer! | trotzig |
| `voices/units/swordsman_hit_03.ogg` | [gasps] Damn it! | Verdammt! | scharf, überrascht |

### swordsman_death (Event `voice.swordsman.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_death_01.ogg` | [gasps] For… the Crown… | Für … die Krone … | letzter Atem |
| `voices/units/swordsman_death_02.ogg` | Tell the sergeant… I held… | Sagt dem Feldwebel … ich hab gehalten … | schwach, stolz |
| `voices/units/swordsman_death_03.ogg` | [exhales] The fire's… going out… | Das Feuer … geht aus … | verlöschend |

### swordsman_victory (Event `voice.swordsman.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/swordsman_victory_01.ogg` | [shouting] Ha! Victory! | Ha! Sieg! | Jubel |
| `voices/units/swordsman_victory_02.ogg` | They won't be getting up again. | Die stehen nicht wieder auf. | zufrieden, trocken |
| `voices/units/swordsman_victory_03.ogg` | The Crown stands! | Die Krone steht! | stolz |

---

## 3. Glutkrone – `spearman` (Speerträger, Veteran)

**Stimme:** `AC_unit_spearman` (Profil D1.3) · **Modell:** Eleven v3, *Natural/Robust* · Speed 0,95
**Haltung:** wortkarger Veteran, tief und ruhig; selbst im Angriff kontrolliert, nie hysterisch.

### spearman_select (Event `voice.spearman.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_select_01.ogg` | Spear's ready. | Speer bereit. | ruhig, tief |
| `voices/units/spearman_select_02.ogg` | Aye. Speak. | Jo. Sprich. | wortkarg |
| `voices/units/spearman_select_03.ogg` | Old bones, sharp point. | Alte Knochen, scharfe Spitze. | trockener Humor |
| `voices/units/spearman_select_04.ogg` | Line's holding. What is it? | Linie hält. Was gibt's? | gelassen |

### spearman_move (Event `voice.spearman.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_move_01.ogg` | On the move. | In Bewegung. | neutral |
| `voices/units/spearman_move_02.ogg` | Steady pace, lads. | Gleichmäßiger Schritt, Jungs. | an die Truppe gerichtet |
| `voices/units/spearman_move_03.ogg` | Aye, we'll go. | Jo, wir gehen. | gelassen |
| `voices/units/spearman_move_04.ogg` | Spears up. Marching. | Speere hoch. Marsch. | Kommandoton, nicht laut |

### spearman_attack (Event `voice.spearman.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_attack_01.ogg` | [shouting] Hold and strike! | Halten und zustoßen! | Kommandoruf |
| `voices/units/spearman_attack_02.ogg` | Spears forward! | Speere nach vorn! | kräftig |
| `voices/units/spearman_attack_03.ogg` | Come on, then. Come onto the point! | Na komm. Komm in die Spitze! | grimmig, provozierend |
| `voices/units/spearman_attack_04.ogg` | Brace… now — thrust! | Abstützen … jetzt – stoßt! | Timing: Pause, dann Ausbruch |

### spearman_retreat (Event `voice.spearman.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_retreat_01.ogg` | Back, slow and steady! | Zurück, langsam und ruhig! | kontrolliert |
| `voices/units/spearman_retreat_02.ogg` | Withdraw! Keep your spears facing! | Rückzug! Speere zum Feind! | Kommandoton |
| `voices/units/spearman_retreat_03.ogg` | Give ground, lads. Don't run. | Boden aufgeben, Jungs. Nicht rennen. | väterlich-bestimmt |

### spearman_hit (Event `voice.spearman.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_hit_01.ogg` | [grunts] Hmph. | Hmpf. | kaum beeindruckt |
| `voices/units/spearman_hit_02.ogg` | [groans] Had worse. | Hatte Schlimmeres. | gepresst |
| `voices/units/spearman_hit_03.ogg` | [exhales sharply] Nngh! | Nngh! | kurzer Stoß |

### spearman_death (Event `voice.spearman.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_death_01.ogg` | [exhales] Hold… the line… | Haltet … die Linie … | letzter Befehl |
| `voices/units/spearman_death_02.ogg` | Finally… a rest… | Endlich … Ruhe … | fast erleichtert |
| `voices/units/spearman_death_03.ogg` | [groans] Not bad… for an old man… | Nicht schlecht … für einen alten Mann … | leises Lächeln im Sterben |

### spearman_victory (Event `voice.spearman.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/spearman_victory_01.ogg` | Line held. Good. | Linie gehalten. Gut. | knapp, zufrieden |
| `voices/units/spearman_victory_02.ogg` | That's how it's done, lads. | So macht man das, Jungs. | stolz-väterlich |
| `voices/units/spearman_victory_03.ogg` | Bury them. Properly, this time. | Begrabt sie. Diesmal richtig. | düster-trocken |

---

## 4. Glutkrone – `archer` (Bogenschützin)

**Stimme:** `AC_unit_archer` (Profil D1.4) · **Modell:** Eleven v3, *Natural* · Speed 1,03
**Haltung:** junge, wache Jägerin, keck, präzise; beim Zielen ruhig, beim Treffer triumphierend.

### archer_select (Event `voice.archer.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_select_01.ogg` | Bow's strung. | Bogen gespannt. | knapp, wach |
| `voices/units/archer_select_02.ogg` | Eyes on the field. | Augen aufs Feld. | konzentriert |
| `voices/units/archer_select_03.ogg` | What am I shooting? | Worauf schieß ich? | keck |
| `voices/units/archer_select_04.ogg` | Archer, ready. | Schützin bereit. | militärisch |
| `voices/units/archer_select_05.ogg` | Got a clear line. Who's first? | Freie Schussbahn. Wer ist zuerst dran? | selbstbewusst, leicht spöttisch |

### archer_move (Event `voice.archer.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_move_01.ogg` | Moving to position. | Beziehe Stellung. | sachlich |
| `voices/units/archer_move_02.ogg` | [whispers] Quick and quiet. | Schnell und leise. | geflüstert, verschwörerisch |
| `voices/units/archer_move_03.ogg` | Finding high ground. | Such mir 'ne Anhöhe. | zielstrebig |
| `voices/units/archer_move_04.ogg` | On it. | Bin dran. | flink |

### archer_attack (Event `voice.archer.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_attack_01.ogg` | Loose! | Los! | scharfer Ruf |
| `voices/units/archer_attack_02.ogg` | Arrows away! | Pfeile ab! | kräftig |
| `voices/units/archer_attack_03.ogg` | [shouting] Fill them with feathers! | Spickt sie mit Federn! | Kampfeslust |
| `voices/units/archer_attack_04.ogg` | Got you! | Hab dich! | triumphierend |
| `voices/units/archer_attack_05.ogg` | Mark… and loose! | Ziel … und los! | ruhige Atempause vor dem Schuss |

### archer_retreat (Event `voice.archer.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_retreat_01.ogg` | Pulling back! | Ziehe mich zurück! | gehetzt |
| `voices/units/archer_retreat_02.ogg` | Too close! Falling back! | Zu nah! Rückzug! | alarmiert |
| `voices/units/archer_retreat_03.ogg` | Can't shoot what's on top of me — back! | Kann nicht schießen, was auf mir hockt – zurück! | Panik mit Restwitz |

### archer_hit (Event `voice.archer.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_hit_01.ogg` | [gasps] Ah! | Ah! | kurzer Schreck |
| `voices/units/archer_hit_02.ogg` | [grunts] They hit me! | Die haben mich erwischt! | empört, Schmerz |
| `voices/units/archer_hit_03.ogg` | [groans] Ngh — still shooting! | Ngh – schieß trotzdem! | trotzig |

### archer_death (Event `voice.archer.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_death_01.ogg` | [gasps] I… missed… | Ich … hab verfehlt … | ungläubig, leise |
| `voices/units/archer_death_02.ogg` | Ah… it's getting dark… | Ah … es wird dunkel … | verlöschend |
| `voices/units/archer_death_03.ogg` | [exhales] Tell them… I got… the last one… | Sagt ihnen … ich hab den Letzten … erwischt … | schwacher Stolz |

### archer_victory (Event `voice.archer.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/archer_victory_01.ogg` | Not a single one got through! | Nicht einer kam durch! | jubelnd |
| `voices/units/archer_victory_02.ogg` | [laughs] Clean shots, every one. | Saubere Schüsse, jeder einzelne. | stolzes Lachen |
| `voices/units/archer_victory_03.ogg` | Count my arrows. Then count their dead. | Zählt meine Pfeile. Dann zählt ihre Toten. | kühl, selbstbewusst |

---

## 5. Glutkrone – `knight` (Ritter, schwere Kavallerie)

**Stimme:** `AC_unit_knight` (Profil D1.5) · **Modell:** Eleven v3, *Natural* · Speed 0,97
**Haltung:** edel, ritterlich, etwas pathetisch (bewusster Kontrast zu Isoldes Understatement). Kein Helmfilter in der Basisaufnahme.

### knight_select (Event `voice.knight.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_select_01.ogg` | Your command? | Euer Befehl? | edel, aufrecht |
| `voices/units/knight_select_02.ogg` | Steel and saddle, ready. | Stahl und Sattel, bereit. | stolz |
| `voices/units/knight_select_03.ogg` | The Crown's lance awaits. | Die Lanze der Krone wartet. | feierlich |
| `voices/units/knight_select_04.ogg` | Speak, and I shall ride. | Sprecht, und ich reite. | formell |
| `voices/units/knight_select_05.ogg` | By the Flame, I am ready. | Bei der Flamme, ich bin bereit. | gläubig, fest |

### knight_move (Event `voice.knight.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_move_01.ogg` | Riding out. | Reite aus. | knapp, sicher |
| `voices/units/knight_move_02.ogg` | Onward! | Vorwärts! | kraftvoll |
| `voices/units/knight_move_03.ogg` | Spurs to flank. | Sporen in die Flanke. | Reiterjargon, zackig |
| `voices/units/knight_move_04.ogg` | As you command. | Wie Ihr befehlt. | höflich-formell |
| `voices/units/knight_move_05.ogg` | The road is ours. | Die Straße gehört uns. | selbstbewusst |

### knight_attack (Event `voice.knight.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_attack_01.ogg` | [shouting] Charge! | Angriff! | maximaler Schlachtruf |
| `voices/units/knight_attack_02.ogg` | [shouting] For Valemark and the Flame! | Für Valemark und die Flamme! | pathetisch, voll |
| `voices/units/knight_attack_03.ogg` | Ride them down! | Reitet sie nieder! | befehlend |
| `voices/units/knight_attack_04.ogg` | Lances… now! | Lanzen … jetzt! | Spannung, dann Ausbruch |

### knight_retreat (Event `voice.knight.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_retreat_01.ogg` | Wheel about! Withdraw! | Kehrt! Rückzug! | Kommando, laut |
| `voices/units/knight_retreat_02.ogg` | Fall back to the lines! | Zurück zu den Linien! | dringlich |
| `voices/units/knight_retreat_03.ogg` | A tactical withdrawal. Ride! | Ein taktischer Rückzug. Reitet! | Würde wahrend, leicht verlegen |

### knight_hit (Event `voice.knight.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_hit_01.ogg` | [grunts] Ha — a glancing blow! | Ha – nur gestreift! | verächtlich |
| `voices/units/knight_hit_02.ogg` | [groans] My armour holds! | Meine Rüstung hält! | gepresst, trotzig |
| `voices/units/knight_hit_03.ogg` | [gasps] Ngh! Insolent! | Ngh! Unverschämt! | empört |

### knight_death (Event `voice.knight.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_death_01.ogg` | [gasps] My oath… fulfilled… | Mein Eid … erfüllt … | würdevoll sterbend |
| `voices/units/knight_death_02.ogg` | Forgive me… Flame… | Vergib mir … Flamme … | leises Gebet |
| `voices/units/knight_death_03.ogg` | [exhales] Tell my house… I did not yield… | Sagt meinem Haus … ich wich nicht … | letzte Kraft |

### knight_victory (Event `voice.knight.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/knight_victory_01.ogg` | [shouting] Glory to the Ember Crown! | Ruhm der Glutkrone! | triumphal |
| `voices/units/knight_victory_02.ogg` | The field is ours! | Das Feld gehört uns! | stolz |
| `voices/units/knight_victory_03.ogg` | A worthy charge. Well fought. | Ein würdiger Ansturm. Gut gekämpft. | ritterlich, anerkennend |

---

## 6. Hohle – `hollow_thrall` (Hohlknecht)

**Stimme:** `AC_unit_hollow_thrall` (Profil D5.1) · **Modell:** Eleven v3, *Creative* · Speed 0,9
**Konzept:** einstige Dorfbewohner; murmeln **Wortfetzen ihres früheren Lebens**. Nicht verständlich-deutlich sprechen: hauchig,
röchelnd, verwaschen. Basisaufnahme trocken; danach **Hohle-Kette** (Leitfaden 8.3), Pitch −2. Manifest: −2 dB, max. 2 Instanzen.
**Alternative:** Voice Changer (eigene Röchel-Performance → `AC_unit_hollow_thrall`) oder SFX-Prompt (siehe unten) als Layer.

### hollow_thrall_attack (Event `voice.hollow_thrall.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_thrall_attack_01.ogg` | [groans] Hnnngh… warm… | Hnnngh … warm … | gierig, kehlig |
| `voices/units/hollow_thrall_attack_02.ogg` | [rasping] Give… it… back… | Gib … es … zurück … | klagend-aggressiv |
| `voices/units/hollow_thrall_attack_03.ogg` | [hisses] Hhhaaah! | Hhhaaah! | wortloser Angriffslaut, Luftstoß |
| `voices/units/hollow_thrall_attack_04.ogg` | …home… mine… | … Heim … meins … | gemurmelt, besitzergreifend |

*SFX-Alternativprompt (Layer, 1,0 s, Influence 0,5):* „emaciated undead villager lunging, dry raspy throat hiss and weak
groan, breathy, close, no reverb“.

### hollow_thrall_hit (Event `voice.hollow_thrall.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_thrall_hit_01.ogg` | [groans] Uhhh — | Uhhh – | dumpf, gleichgültig |
| `voices/units/hollow_thrall_hit_02.ogg` | [gasps] Hhkk — | Hhkk – | trockenes Keuchen, abgebrochen |
| `voices/units/hollow_thrall_hit_03.ogg` | …still… hurts… | … tut … noch weh … | verloren, fast menschlich |

*SFX-Alternativprompt (0,6 s, Influence 0,5):* „undead creature hit, short dry choked grunt, rattling breath, close“.

### hollow_thrall_death (Event `voice.hollow_thrall.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_thrall_death_01.ogg` | [sighs] …finally… quiet… | … endlich … still … | erlöst, verhauchend |
| `voices/units/hollow_thrall_death_02.ogg` | [exhales] …mother… | … Mutter … | leises, trauriges Erinnern |
| `voices/units/hollow_thrall_death_03.ogg` | [rattling breath] Hhhhaaahhh… | Hhhhaaahhh … | langes Todesröcheln, 1,5–2 s |

*SFX-Alternativprompt (1,8 s, Influence 0,45):* „undead villager collapsing, long rattling death exhale fading into silence, dry, close“.

---

## 7. Hohle – `hollow_archer` (Knochenschütze)

**Stimme:** `AC_unit_hollow_archer` (Profil D5.2) · **Modell:** Eleven v3, *Creative* mit `[whispers]`
**Konzept:** skelettiert, fast ohne Atem; zischendes Flüstern, klickende Konsonanten. Nachbearbeitung: Hohle-Kette + Knochenklacken-Layer (−18 dB).

### hollow_archer_attack (Event `voice.hollow_archer.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_archer_attack_01.ogg` | [whispers] Sssee you. | Ssseh dich. | kalt, zischend |
| `voices/units/hollow_archer_attack_02.ogg` | [whispers] One more… | Noch einer … | zählend, gierig |
| `voices/units/hollow_archer_attack_03.ogg` | [hisses] Tsssk! | Tsssk! | wortlos, Zischlaut beim Schuss |
| `voices/units/hollow_archer_attack_04.ogg` | [whispers] Mark the heart. | Markier das Herz. | präzise, eisig |

*SFX-Alternativprompt (0,8 s, Influence 0,5):* „skeleton archer hissing whisper with bone clicking, dry, airy, close“.

### hollow_archer_hit (Event `voice.hollow_archer.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_archer_hit_01.ogg` | [hisses] Kkhh! | Kkhh! | Knack-Zischen |
| `voices/units/hollow_archer_hit_02.ogg` | Ssss! | Ssss! | scharfes, kurzes Zischen |
| `voices/units/hollow_archer_hit_03.ogg` | [whispers] …cracked… | … gesprungen … | tonlos, fast verwundert |

*SFX-Alternativprompt (0,5 s, Influence 0,55):* „bone cracking with short hiss of air, dry skeletal creature hit“.

### hollow_archer_death (Event `voice.hollow_archer.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_archer_death_01.ogg` | [whispers] …falling… apart… | … zerfalle … | verwehend |
| `voices/units/hollow_archer_death_02.ogg` | [exhales] Hhhaaa… | Hhhaaa … | letzter, trockener Hauch |
| `voices/units/hollow_archer_death_03.ogg` | [whispers] …the counting… stops… | … das Zählen … hört auf … | erleichtert, tonlos |

*SFX-Alternativprompt (1,5 s, Influence 0,45):* „skeleton collapsing into a pile of bones with a fading dry whisper exhale“.

---

## 8. Hohle – `grave_brute` (Grabkoloss)

**Stimme:** `AC_unit_grave_brute` (Profil D5.3) · **Modell:** Eleven v3, *Creative* · Speed 0,85
**Konzept:** aufgedunsener untoter Riese; kehlig, nass, langsam; einzelne Wörter mit erdrückender Wucht.
Nachbearbeitung: Pitch −4 bis −6, Subbass-Anhebung +2 dB @ 80 Hz, Hohle-Kette; SFX-Growl als Unterschicht (−8 dB).

### grave_brute_attack (Event `voice.grave_brute.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/grave_brute_attack_01.ogg` | [shouting] Crush! | Zerquetschen! | brüllend, ein Wort |
| `voices/units/grave_brute_attack_02.ogg` | [groans] Hhhrrraaagh! | Hhhrrraaagh! | wortloser Brüller, 1,2 s |
| `voices/units/grave_brute_attack_03.ogg` | [shouting] Down! | Runter! | kurz, wuchtig |
| `voices/units/grave_brute_attack_04.ogg` | [growls] Smash… small ones… | Zerschmettern … die Kleinen … | langsam, genüsslich |

*SFX-Layer-Prompt (1,2 s, Influence 0,55):* „massive undead giant roar, wet guttural bellow, deep and heavy, close, no reverb“.

### grave_brute_hit (Event `voice.grave_brute.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/grave_brute_hit_01.ogg` | [groans] Hhhrrmph! | Hhhrrmph! | dumpfes Grunzen |
| `voices/units/grave_brute_hit_02.ogg` | [roars] Rrraah! | Rrraah! | wütend, kurz |
| `voices/units/grave_brute_hit_03.ogg` | [groans] …hurts… | … tut weh … | träge, überrascht |

*SFX-Layer-Prompt (0,7 s, Influence 0,55):* „huge creature pained grunt, low wet growl, short“.

### grave_brute_death (Event `voice.grave_brute.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/grave_brute_death_01.ogg` | [groans] Hhhuuurrrgh… | Hhhuuurrrgh … | langes, absinkendes Stöhnen, 2 s |
| `voices/units/grave_brute_death_02.ogg` | [exhales] …ground… cold… | … Erde … kalt … | müde, fast kindlich |
| `voices/units/grave_brute_death_03.ogg` | [roars] Rrrraaaaagh… | Rrrraaaaagh … | Brüllen, das in Röcheln zerfällt |

*SFX-Layer-Prompt (2,5 s, Influence 0,45):* „giant undead creature death groan fading into a heavy collapsing thud, deep“.

---

## 9. Heldin – `hero_isolde` (Isolde Raventhorn, Wächterin der Aschmark)

**Stimme:** `AC_hero_isolde` (Profil A1) · **Modell:** Eleven v3, *Natural* (attack/ability gelegentlich *Creative*) · Speed 1,0
**Haltung:** pflichtbewusst, kontrolliert, trocken-humorvoll; Humor durch Understatement, nie Klamauk. Lautheitsreferenz der Produktion.
Manifest: +1 dB, Cooldown 300 ms.

### hero_isolde_select (Event `voice.hero_isolde.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_select_01.ogg` | Raventhorn. Speak. | Raventhorn. Sprecht. | knapp, Autorität, **Referenzdatei** |
| `voices/heroes/hero_isolde_select_02.ogg` | The Ashmarch still stands. So do I. | Die Aschmark steht noch. Ich auch. | ruhig-stolz |
| `voices/heroes/hero_isolde_select_03.ogg` | Yes? Make it quick, the dead don't wait. | Ja? Macht schnell, die Toten warten nicht. | leicht ungeduldig |
| `voices/heroes/hero_isolde_select_04.ogg` | Warden here. | Wächterin hier. | sachlich |
| `voices/heroes/hero_isolde_select_05.ogg` | [sarcastic] What now? Another fire to put out? | Was jetzt? Noch ein Feuer zu löschen? | trocken-ironisch, nicht genervt |

### hero_isolde_move (Event `voice.hero_isolde.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_move_01.ogg` | Moving. | Unterwegs. | knapp |
| `voices/heroes/hero_isolde_move_02.ogg` | I know these roads. | Ich kenne diese Wege. | sicher, ein Hauch Wehmut |
| `voices/heroes/hero_isolde_move_03.ogg` | Through the ash, then. | Dann durch die Asche. | entschlossen |
| `voices/heroes/hero_isolde_move_04.ogg` | On my way. Try not to lose the village while I'm gone. | Bin unterwegs. Versucht, das Dorf nicht zu verlieren, solange ich weg bin. | trockener Humor |
| `voices/heroes/hero_isolde_move_05.ogg` | If it's burning, I'm going. | Wenn's brennt, geh ich. | lakonisch |

### hero_isolde_attack (Event `voice.hero_isolde.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_attack_01.ogg` | [shouting] For the Ashmarch! | Für die Aschmark! | voller Schlachtruf |
| `voices/heroes/hero_isolde_attack_02.ogg` | Back into the dark with you! | Zurück ins Dunkel mit euch! | kalte Wut |
| `voices/heroes/hero_isolde_attack_03.ogg` | Steel and ember! | Stahl und Glut! | Kampfformel, kraftvoll |
| `voices/heroes/hero_isolde_attack_04.ogg` | Let's see if you bleed. | Mal sehen, ob ihr blutet. | gefährlich ruhig |
| `voices/heroes/hero_isolde_attack_05.ogg` | [shouting] Burn them down! | Brennt sie nieder! | Befehl an Truppe |

### hero_isolde_retreat (Event `voice.hero_isolde.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_retreat_01.ogg` | Fall back. That's not a request. | Rückzug. Das ist keine Bitte. | scharf, kontrolliert |
| `voices/heroes/hero_isolde_retreat_02.ogg` | Pulling out. We fight again tomorrow. | Wir ziehen ab. Morgen kämpfen wir wieder. | bitter-vernünftig |
| `voices/heroes/hero_isolde_retreat_03.ogg` | [exhales] Not here. Not today. Back! | Nicht hier. Nicht heute. Zurück! | außer Atem, entschlossen |

### hero_isolde_hit (Event `voice.hero_isolde.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_hit_01.ogg` | [grunts] Ngh! Is that all? | Ngh! Ist das alles? | trotzig |
| `voices/heroes/hero_isolde_hit_02.ogg` | [groans] Hah… I've had worse fevers. | Hah … ich hatte schon schlimmeres Fieber. | Anspielung Aschenfieber, gepresst |
| `voices/heroes/hero_isolde_hit_03.ogg` | [gasps] Argh! | Argh! | reiner Schmerzlaut, kurz |

### hero_isolde_death (Event `voice.hero_isolde.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_death_01.ogg` | [gasps] Keep… the fires… lit… | Haltet … die Feuer … am Brennen … | letzter Auftrag, schwach |
| `voices/heroes/hero_isolde_death_02.ogg` | Father… I held… as long as I could… | Vater … ich hab gehalten … so lang ich konnte … | verletzlich, leise |
| `voices/heroes/hero_isolde_death_03.ogg` | [exhales] Not… yet… damn it… | Nicht … jetzt … verdammt … | Wut gegen das Sterben |

### hero_isolde_victory (Event `voice.hero_isolde.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_victory_01.ogg` | The March holds. Today. | Die Mark hält. Heute. | erschöpft, Pause vor „Today“ |
| `voices/heroes/hero_isolde_victory_02.ogg` | [exhales] Well. That could have gone worse. | Na ja. Hätte schlimmer laufen können. | Understatement |
| `voices/heroes/hero_isolde_victory_03.ogg` | Count the living first. Then the dead. | Zählt zuerst die Lebenden. Dann die Toten. | ernst, Verantwortung |

### hero_isolde_ability (Event `voice.hero_isolde.ability`)
Hinweis: Das Manifest kennt **ein** Ability-Event. Die Varianten decken die drei Fähigkeiten ab (je 2). Bei späterer Aufteilung
in `ember_cleave`/`rallying_banner`/`ashen_ward` können die Dateien entsprechend zugeordnet werden (Empfehlung an das Engine-Team).

| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_ability_01.ogg` | [shouting] Taste the ember! | Kostet die Glut! | Glutspalter (`ember_cleave`), Schlagschrei |
| `voices/heroes/hero_isolde_ability_02.ogg` | Cleave and burn! | Spalten und brennen! | Glutspalter, mit Hiebanstrengung |
| `voices/heroes/hero_isolde_ability_03.ogg` | [shouting] To me! Raise the banner! | Zu mir! Hebt das Banner! | Sammelbanner (`rallying_banner`), mitreißend |
| `voices/heroes/hero_isolde_ability_04.ogg` | Stand with me. The March is watching! | Steht zu mir. Die Mark sieht zu! | Sammelbanner, Moralruf |
| `voices/heroes/hero_isolde_ability_05.ogg` | Ash, shield me. | Asche, schirme mich. | Aschenschild (`ashen_ward`), konzentriert, halblaut |
| `voices/heroes/hero_isolde_ability_06.ogg` | Behind me. Now. | Hinter mich. Sofort. | Aschenschild, schützend-befehlend |

### hero_isolde_greet (Event `voice.hero_isolde.greet`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_greet_01.ogg` | Isolde Raventhorn, Warden of the Ashmarch. Let's get to work. | Isolde Raventhorn, Wächterin der Aschmark. An die Arbeit. | Vorstellung, sachlich-warm |
| `voices/heroes/hero_isolde_greet_02.ogg` | Brennhollow's still breathing. Good. Let's keep it that way. | Brennhollow atmet noch. Gut. Dabei soll's bleiben. | ruhig, entschlossen |
| `voices/heroes/hero_isolde_greet_03.ogg` | [sarcastic] You called, I came. Try to have a plan this time. | Ihr habt gerufen, ich bin da. Versucht diesmal, einen Plan zu haben. | trocken, augenzwinkernd |

### hero_isolde_levelup (Event `voice.hero_isolde.levelup`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_levelup_01.ogg` | Stronger. Good. The ruins won't get any kinder. | Stärker. Gut. Die Ruinen werden nicht freundlicher. | nüchtern zufrieden |
| `voices/heroes/hero_isolde_levelup_02.ogg` | [exhales] The ember burns brighter. | Die Glut brennt heller. | spürbar, fast ehrfürchtig |
| `voices/heroes/hero_isolde_levelup_03.ogg` | Older, wiser, more scars. The usual. | Älter, klüger, mehr Narben. Das Übliche. | trockener Humor |

### hero_isolde_lowhp (Event `voice.hero_isolde.lowhp`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_isolde_lowhp_01.ogg` | [groans] I'm hurt… badly. | Ich bin verletzt … schwer. | ehrlich, gepresst |
| `voices/heroes/hero_isolde_lowhp_02.ogg` | [gasps] Need a healer. Or a miracle. | Brauche einen Heiler. Oder ein Wunder. | Humor trotz Schmerz |
| `voices/heroes/hero_isolde_lowhp_03.ogg` | [exhales] Can't take much more of this. | Viel mehr halte ich nicht aus. | erschöpft, Warnung an Spieler |

---

## 10. Boss – `boss_pale_reeve` (Der Fahle Vogt)

**Stimme:** `AC_boss_pale_reeve` (Profil C4) · **Modell:** Eleven v3, *Natural* (hit/death *Creative*) · Speed 0,95
**Konzept:** untoter Steuervogt Aureths; Motive **Zehnt, Register, Schuld, Zinsen, Glocke**. Ablage laut Manifest unter `voices/units/`.
Nachbearbeitung: Hohle-Kette + Vogt-Ergänzung (Chorus-Schicht, kalter Kathedralhall 1,8 s, Mix 12 %). Manifest: +1 dB, Cooldown 800 ms.

### boss_pale_reeve_attack (Event `voice.boss_pale_reeve.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/boss_pale_reeve_attack_01.ogg` | Your tithe is due. | Euer Zehnt ist fällig. | kalt, beiläufig |
| `voices/units/boss_pale_reeve_attack_02.ogg` | Pay what you owe! | Zahlt, was Ihr schuldet! | scharf, lauter |
| `voices/units/boss_pale_reeve_attack_03.ogg` | Every soul is counted. | Jede Seele wird gezählt. | pedantisch, zischend |
| `voices/units/boss_pale_reeve_attack_04.ogg` | Into the ledger with you. | Ab ins Register mit Euch. | süffisant |

### boss_pale_reeve_ability (Event `voice.boss_pale_reeve.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/boss_pale_reeve_ability_01.ogg` | I claim the debt of your breath. | Ich fordere die Schuld Eures Atems ein. | Seelenentzug (`reeve_soul_drain`), langsam, genüsslich |
| `voices/units/boss_pale_reeve_ability_02.ogg` | [whispers] Rise, defaulters. Rise and serve. | Erhebt euch, Säumige. Erhebt euch und dient. | Beschwörung (`hollow_summon`), geflüstert-befehlend |
| `voices/units/boss_pale_reeve_ability_03.ogg` | Your soul is forfeit. | Eure Seele ist verwirkt. | Urteil, eisig |

### boss_pale_reeve_hit (Event `voice.boss_pale_reeve.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/boss_pale_reeve_hit_01.ogg` | [hisses] Insolence! | Unverschämtheit! | empörtes Zischen |
| `voices/units/boss_pale_reeve_hit_02.ogg` | [shrieks] You dare strike the Reeve? | Ihr wagt es, den Vogt zu schlagen? | schrill, kippende Stimme |
| `voices/units/boss_pale_reeve_hit_03.ogg` | [groans] This will be… recorded. | Das wird … vermerkt. | gekränkt-pedantisch |

### boss_pale_reeve_death (Event `voice.boss_pale_reeve.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/boss_pale_reeve_death_01.ogg` | [shrieks] No — the ledger… unbalanced… | Nein – das Register … nicht ausgeglichen … | Entsetzen, zerfallend, 2–3 s |
| `voices/units/boss_pale_reeve_death_02.ogg` | [rattling breath] The Deep… will send… another… | Das Tief … schickt … einen anderen … | Drohung im Sterben |
| `voices/units/boss_pale_reeve_death_03.ogg` | [exhales] …paid… in full… | … vollständig … bezahlt … | fast erlöst, verhauchend |

### boss_pale_reeve_taunt (Event `voice.boss_pale_reeve.taunt`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/boss_pale_reeve_taunt_01.ogg` | Brennhollow. Thirty souls. Nine hundred years in arrears. | Brennhollow. Dreißig Seelen. Neunhundert Jahre im Rückstand. | verlesend wie aus einem Register |
| `voices/units/boss_pale_reeve_taunt_02.ogg` | Do you hear the bell, Warden? It tolls for your account. | Hört Ihr die Glocke, Wächterin? Sie läutet für Euer Konto. | leise, genüsslich |
| `voices/units/boss_pale_reeve_taunt_03.ogg` | The Hollow Deep is patient. I am not. | Das Hohle Tief ist geduldig. Ich bin es nicht. | kalt, Pause in der Mitte |
| `voices/units/boss_pale_reeve_taunt_04.ogg` | [laughs] Run, if you like. The dead always collect. | Lauft nur. Die Toten treiben immer ein. | trockenes, hohles Lachen |

---

## 11. Ausblick – spätere Einheiten (nicht im Manifest v1)

Alle IDs in diesem Abschnitt sind **Vorschläge** und vor Produktion vom Kanon-Owner freizugeben. Dateinamen folgen der
Konvention `voices/units/<unit_id>_<situation>_<NN>.ogg`. Stimmen aus Teil D des Charakterdokuments. Pro Einheit hier je drei
Zeilen für select/move/attack; für die Produktion auf 4–5 Varianten erweitern und retreat/hit/death/victory ergänzen.

### 11.1 Glutkrone – Flammenpriesterin (Priester) · Vorschlag `ember_priest` · Stimme `AC_ember_crown_priest`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/ember_priest_select_01.ogg` | The Flame is with you. | Die Flamme ist mit euch. | warm, segnend |
| `voices/units/ember_priest_select_02.ogg` | Who needs healing? | Wer braucht Heilung? | fürsorglich |
| `voices/units/ember_priest_select_03.ogg` | Speak, child of the hearth. | Sprich, Kind des Herdes. | feierlich |
| `voices/units/ember_priest_move_01.ogg` | I walk where the light is needed. | Ich gehe, wo Licht gebraucht wird. | ruhig |
| `voices/units/ember_priest_move_02.ogg` | Lead on. I'll follow the wounded. | Geht voran. Ich folge den Verwundeten. | pragmatisch |
| `voices/units/ember_priest_move_03.ogg` | The Flame guides my steps. | Die Flamme lenkt meine Schritte. | gläubig |
| `voices/units/ember_priest_attack_01.ogg` | [shouting] Burn, unclean thing! | Brenne, Unreines! | heiliger Zorn |
| `voices/units/ember_priest_attack_02.ogg` | Back to the Deep! | Zurück ins Tief! | bannend |
| `voices/units/ember_priest_attack_03.ogg` | Light take you! | Das Licht hole dich! | Gebetsformel als Waffe |

### 11.2 Skarn-Clans – Ahnenruferin (Magier/Priester) · Vorschlag `skarn_skald` · Stimme `AC_skarn_clans_skald`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/skarn_skald_select_01.ogg` | The ancestors listen. | Die Ahnen hören zu. | tief, raunend |
| `voices/units/skarn_skald_select_02.ogg` | Which saga shall we write today? | Welche Saga schreiben wir heute? | rhythmisch |
| `voices/units/skarn_skald_select_03.ogg` | I carry the songs of the dead. | Ich trage die Lieder der Toten. | feierlich |
| `voices/units/skarn_skald_move_01.ogg` | Where the drums lead. | Wohin die Trommeln führen. | gemessen |
| `voices/units/skarn_skald_move_02.ogg` | My feet know the old paths. | Meine Füße kennen die alten Pfade. | ruhig |
| `voices/units/skarn_skald_move_03.ogg` | Onward, under the mountain's eye. | Weiter, unter dem Auge des Berges. | sagenhaft |
| `voices/units/skarn_skald_attack_01.ogg` | [shouting] Fathers, lend us frost! | Väter, leiht uns Frost! | Beschwörungsruf |
| `voices/units/skarn_skald_attack_02.ogg` | Hear the ancestors' wrath! | Hört den Zorn der Ahnen! | kraftvoll |
| `voices/units/skarn_skald_attack_03.ogg` | Your name dies with you! | Dein Name stirbt mit dir! | Fluch |

### 11.3 Skarn-Clans – Clan-Reiterei (Kavallerie) · Vorschlag `skarn_rider` · Stimme `AC_skarn_clans_warrior` (jüngere Variante)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/skarn_rider_select_01.ogg` | Mounted and hungry. | Aufgesessen und hungrig. | kernig |
| `voices/units/skarn_rider_select_02.ogg` | The snow can't slow us. | Der Schnee bremst uns nicht. | stolz |
| `voices/units/skarn_rider_select_03.ogg` | Say the word, chief. | Sag das Wort, Fürst. | ungeduldig |
| `voices/units/skarn_rider_move_01.ogg` | Riding hard! | Scharfer Ritt! | laut |
| `voices/units/skarn_rider_move_02.ogg` | Over the ridge! | Über den Kamm! | energisch |
| `voices/units/skarn_rider_move_03.ogg` | Hooves and iron, go! | Hufe und Eisen, los! | Burr betonen |
| `voices/units/skarn_rider_attack_01.ogg` | [shouting] Skarnheim! | Skarnheim! | Clanruf |
| `voices/units/skarn_rider_attack_02.ogg` | Trample them! | Zertrampelt sie! | wild |
| `voices/units/skarn_rider_attack_03.ogg` | For my fathers' bones! | Für die Knochen meiner Väter! | Ahnenschwur |

### 11.4 Dornenbund – Druide (Magier) · Vorschlag `thorn_druid` · Stimme `AC_thorn_covenant_druid`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/thorn_druid_select_01.ogg` | The roots are listening. | Die Wurzeln hören zu. | leise, alt |
| `voices/units/thorn_druid_select_02.ogg` | Speak gently. | Sprich sanft. | mahnend |
| `voices/units/thorn_druid_select_03.ogg` | The Green Circle answers. | Der Grüne Kreis antwortet. | würdevoll |
| `voices/units/thorn_druid_move_01.ogg` | Where the moss grows thick. | Wo das Moos dicht wächst. | ruhig |
| `voices/units/thorn_druid_move_02.ogg` | Softly, through the ferns. | Leise, durch die Farne. | gedämpft |
| `voices/units/thorn_druid_move_03.ogg` | The forest makes way. | Der Wald macht Platz. | wissend |
| `voices/units/thorn_druid_attack_01.ogg` | [shouting] Thorns, bind them! | Dornen, bindet sie! | Zauberruf |
| `voices/units/thorn_druid_attack_02.ogg` | Return to the soil! | Kehr zurück in die Erde! | streng |
| `voices/units/thorn_druid_attack_03.ogg` | The wild remembers you! | Die Wildnis erinnert sich an dich! | drohend |

### 11.5 Dornenbund – Veyl-Pirscherin (Assassine) · Vorschlag `veyl_stalker` · Stimme `AC_thorn_covenant_veyl_scout`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/veyl_stalker_select_01.ogg` | [whispers] I was never here. | Ich war nie hier. | geflüstert |
| `voices/units/veyl_stalker_select_02.ogg` | Eyes like moonlight. | Augen wie Mondlicht. | rätselhaft |
| `voices/units/veyl_stalker_select_03.ogg` | Who sleeps tonight? | Wer schläft heute Nacht? | gefährlich sanft |
| `voices/units/veyl_stalker_move_01.ogg` | [whispers] Through the shadows. | Durch die Schatten. | luftig |
| `voices/units/veyl_stalker_move_02.ogg` | Like a moth to flame. | Wie die Motte zum Licht. | Veyl-Motiv |
| `voices/units/veyl_stalker_move_03.ogg` | Unseen. | Ungesehen. | tonlos |
| `voices/units/veyl_stalker_attack_01.ogg` | [whispers] Hush now. | Still jetzt. | intim, tödlich |
| `voices/units/veyl_stalker_attack_02.ogg` | One cut. Enough. | Ein Schnitt. Genug. | kalt |
| `voices/units/veyl_stalker_attack_03.ogg` | The leaves will cover you. | Das Laub wird dich bedecken. | melancholisch |

### 11.6 Saelith-Pakt – Gezeitenpriesterin (Magier/Priester) · Vorschlag `saelith_tide_priestess` · Stimme `AC_saelith_compact_tide_priestess`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/saelith_tide_priestess_select_01.ogg` | The tide is turning. | Die Flut wendet sich. | ruhig fließend |
| `voices/units/saelith_tide_priestess_select_02.ogg` | Ask, and the sea may answer. | Frag, und das Meer antwortet vielleicht. | geheimnisvoll |
| `voices/units/saelith_tide_priestess_select_03.ogg` | Salt and silver, at your service. | Salz und Silber, zu Diensten. | elegant |
| `voices/units/saelith_tide_priestess_move_01.ogg` | Like water, I go. | Wie Wasser gehe ich. | weich |
| `voices/units/saelith_tide_priestess_move_02.ogg` | Following the current. | Ich folge der Strömung. | gelassen |
| `voices/units/saelith_tide_priestess_move_03.ogg` | The shore is calling. | Das Ufer ruft. | träumerisch |
| `voices/units/saelith_tide_priestess_attack_01.ogg` | [shouting] Drown! | Ertrinke! | plötzliche Härte |
| `voices/units/saelith_tide_priestess_attack_02.ogg` | The sea takes its due. | Das Meer nimmt, was ihm zusteht. | kühl |
| `voices/units/saelith_tide_priestess_attack_03.ogg` | Break like a wave! | Brich wie eine Welle! | kraftvoll |

### 11.7 Saelith-Pakt – Söldner-Lanzenreiter (Kavallerie) · Vorschlag `saelith_lancer` · Stimme `AC_saelith_compact_captain` (männliche Variante neu designen)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/saelith_lancer_select_01.ogg` | Paid in full? Then I ride. | Voll bezahlt? Dann reite ich. | zynisch |
| `voices/units/saelith_lancer_select_02.ogg` | Lance and coin, both sharp. | Lanze und Münze, beide scharf. | geschäftsmäßig |
| `voices/units/saelith_lancer_select_03.ogg` | Contract says I listen. | Laut Vertrag hör ich zu. | gelangweilt |
| `voices/units/saelith_lancer_move_01.ogg` | Moving, at the agreed rate. | Unterwegs, zum vereinbarten Satz. | trocken |
| `voices/units/saelith_lancer_move_02.ogg` | Horses don't run on promises. | Pferde laufen nicht mit Versprechen. | spöttisch |
| `voices/units/saelith_lancer_move_03.ogg` | On my way. | Bin unterwegs. | neutral |
| `voices/units/saelith_lancer_attack_01.ogg` | [shouting] Earn your pay! | Verdient euren Sold! | Ruf an Kameraden |
| `voices/units/saelith_lancer_attack_02.ogg` | Charge, for the bonus! | Angriff, für die Prämie! | grinsend |
| `voices/units/saelith_lancer_attack_03.ogg` | Nothing personal! | Nichts Persönliches! | lässig-brutal |

### 11.8 Hohle – Kultist der Leeren Liturgie (Priester) · Vorschlag `hollow_cultist` · Stimme `AC_hollowed_cultist`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_cultist_select_01.ogg` | [whispers] The Deep hears you. | Das Tief hört dich. | fiebrig |
| `voices/units/hollow_cultist_select_02.ogg` | Empty, and ready. | Leer und bereit. | ekstatisch-ruhig |
| `voices/units/hollow_cultist_select_03.ogg` | Speak the quiet word. | Sprich das stille Wort. | liturgisch |
| `voices/units/hollow_cultist_move_01.ogg` | Down, always down. | Hinab, immer hinab. | singend |
| `voices/units/hollow_cultist_move_02.ogg` | The Veil is thin here. | Der Schleier ist dünn hier. | entrückt |
| `voices/units/hollow_cultist_move_03.ogg` | I follow the silence. | Ich folge der Stille. | tonlos |
| `voices/units/hollow_cultist_attack_01.ogg` | [shouting] Be emptied! | Werde geleert! | schrille Ekstase |
| `voices/units/hollow_cultist_attack_02.ogg` | Your flame goes out! | Deine Flamme erlischt! | hasserfüllt |
| `voices/units/hollow_cultist_attack_03.ogg` | Only the quiet remains! | Nur die Stille bleibt! | Litanei |

### 11.9 Hohle – Wiedergänger-Magierin (Magier) · Vorschlag `hollow_revenant_mage` · Stimme `AC_hollowed_revenant_mage`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_revenant_mage_select_01.ogg` | I remember the light. | Ich erinnere mich an das Licht. | wehmütig-kalt |
| `voices/units/hollow_revenant_mage_select_02.ogg` | What would you unmake? | Was willst du auflösen? | elegant, bedrohlich |
| `voices/units/hollow_revenant_mage_select_03.ogg` | Aureth's last court mage. | Aureths letzte Hofmagierin. | stolz |
| `voices/units/hollow_revenant_mage_move_01.ogg` | Drifting. | Ich treibe. | gehaucht |
| `voices/units/hollow_revenant_mage_move_02.ogg` | Like smoke through ruins. | Wie Rauch durch Ruinen. | langsam |
| `voices/units/hollow_revenant_mage_move_03.ogg` | The dead make no sound. | Die Toten machen kein Geräusch. | flüsternd |
| `voices/units/hollow_revenant_mage_attack_01.ogg` | [whispers] Unmade. | Aufgelöst. | eisig |
| `voices/units/hollow_revenant_mage_attack_02.ogg` | Wither! | Verdorre! | Zauberwort, scharf |
| `voices/units/hollow_revenant_mage_attack_03.ogg` | Your light is borrowed. | Dein Licht ist geliehen. | verächtlich |

### 11.10 Aschenlegion – Aschenreiter (Kavallerie) · Vorschlag `legion_ash_rider` · Stimme `AC_ashen_legion_legionnaire` (mit Helm-Signatur)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/legion_ash_rider_select_01.ogg` | We ride for the Sun. | Wir reiten für die Sonne. | monoton-feierlich |
| `voices/units/legion_ash_rider_select_02.ogg` | Command the Legion. | Befehligt die Legion. | kalt |
| `voices/units/legion_ash_rider_select_03.ogg` | Our steeds do not tire. | Unsere Rösser ermüden nicht. | mechanisch |
| `voices/units/legion_ash_rider_move_01.ogg` | The Legion advances. | Die Legion rückt vor. | gleichförmig |
| `voices/units/legion_ash_rider_move_02.ogg` | Through fire and ash. | Durch Feuer und Asche. | hohl |
| `voices/units/legion_ash_rider_move_03.ogg` | As it was ordered. | Wie befohlen wurde. | archaisch |
| `voices/units/legion_ash_rider_attack_01.ogg` | [shouting] For the Hollow King! | Für den Hohlen König! | dröhnend |
| `voices/units/legion_ash_rider_attack_02.ogg` | Kneel or burn! | Kniet oder brennt! | gebieterisch |
| `voices/units/legion_ash_rider_attack_03.ogg` | We do not yield! | Wir weichen nicht! | Eid-Formel |

### 11.11 Aschenlegion – Legions-Herold (Priester der Sonnenhäresie) · Vorschlag `legion_herald` · Stimme `AC_ashen_legion_herald`
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/legion_herald_select_01.ogg` | The Sun King returns! | Der Sonnenkönig kehrt zurück! | fanatisch |
| `voices/units/legion_herald_select_02.ogg` | Hear the proclamation! | Hört die Verkündung! | ausrufend |
| `voices/units/legion_herald_select_03.ogg` | Glory in his light. | Herrlichkeit in seinem Licht. | entrückt |
| `voices/units/legion_herald_move_01.ogg` | I bear his word. | Ich trage sein Wort. | getragen |
| `voices/units/legion_herald_move_02.ogg` | To every gate, the message. | Zu jedem Tor die Botschaft. | ausrufend |
| `voices/units/legion_herald_move_03.ogg` | The heralds go before. | Die Herolde gehen voran. | feierlich |
| `voices/units/legion_herald_attack_01.ogg` | [shouting] Heretics, burn! | Ketzer, brennt! | schrill |
| `voices/units/legion_herald_attack_02.ogg` | Melt in his light! | Schmelzt in seinem Licht! | ekstatisch |
| `voices/units/legion_herald_attack_03.ogg` | The ash shall turn to gold! | Die Asche wird zu Gold! | fanatisch-jubelnd |

---

## 12. Echtzeit-Militär – Glutkrone (neue Einheiten)

Ergänzung zu `docs/design/rts_military.md`. Alle Einheiten dieses Abschnitts stehen im Manifest (Events `voice.<id>.<situation>`); die Stimmprofile `D1.6`–`D1.14` sind im Figurenleitfaden neu anzulegen (gleiche Pflege wie D1.1–D1.5). **Sprechtempo und Haltung** je Einheit siehe Kopfzeile; Loudness wie Glutkrone allgemein (−16 LUFS). `priest` ersetzt den Vorschlag `ember_priest` aus Abschnitt 11.1.

### 12.1 Glutkrone – `crossbowman` (Armbrustschütze)

**Stimme:** `AC_unit_crossbowman (Profil D1.6)` · **Modell:** Eleven v3, *Natural* · Speed 1,0
**Haltung:** nüchterner Söldnerveteran aus den Grenzgarnisonen, kurze, trockene Sätze, spricht über Reichweite und Treffer wie über Handwerk.

### crossbowman_select (Event `voice.crossbowman.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_select_01.ogg` | Crossbow ready. | Armbrust bereit. | knapp, sachlich |
| `voices/units/crossbowman_select_02.ogg` | Bolt's in the groove. | Bolzen liegt in der Rinne. | trocken, leise |
| `voices/units/crossbowman_select_03.ogg` | Aye, sir. Steady hands. | Jawohl. Ruhige Hände. | ruhig-selbstsicher |
| `voices/units/crossbowman_select_04.ogg` | Say the word and I'll put one through it. | Sagt's, und ich schieß einen durch. | leicht grimmig |

### crossbowman_move (Event `voice.crossbowman.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_move_01.ogg` | Moving up. | Rücke vor. | knapp |
| `voices/units/crossbowman_move_02.ogg` | Keep the line, I'll follow. | Haltet die Linie, ich komme nach. | ruhig |
| `voices/units/crossbowman_move_03.ogg` | Winding as I walk. | Spanne im Gehen. | beiläufig, mit Metallklicken |
| `voices/units/crossbowman_move_04.ogg` | On my way. | Bin unterwegs. | neutral |

### crossbowman_attack (Event `voice.crossbowman.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_attack_01.ogg` | Loose! | Schuss! | scharf, laut |
| `voices/units/crossbowman_attack_02.ogg` | Armor-piercing, this one! | Der hier durchschlägt Panzer! | grimmig |
| `voices/units/crossbowman_attack_03.ogg` | Down you go! | Runter mit dir! | hart |
| `voices/units/crossbowman_attack_04.ogg` | Winding… and loose! | Spannen … und Schuss! | rhythmisch, zweiteilig |

### crossbowman_retreat (Event `voice.crossbowman.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_retreat_01.ogg` | Fall back, I'm reloading! | Zurück, ich lade nach! | gehetzt |
| `voices/units/crossbowman_retreat_02.ogg` | Too close! Cover me! | Zu nah! Deckt mich! | angespannt |
| `voices/units/crossbowman_retreat_03.ogg` | Back to the line! | Zurück in die Linie! | laut |

### crossbowman_hit (Event `voice.crossbowman.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_hit_01.ogg` | [grunts] Ah, my arm! | Mein Arm! | Schmerz, kurz |
| `voices/units/crossbowman_hit_02.ogg` | [gasps] Hit! | Getroffen! | überrascht |
| `voices/units/crossbowman_hit_03.ogg` | Not the string, damn it! | Nicht die Sehne, verflucht! | ärgerlich |

### crossbowman_death (Event `voice.crossbowman.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_death_01.ogg` | [exhales] Cold… so cold… | Kalt … so kalt … | verhauchend |
| `voices/units/crossbowman_death_02.ogg` | [gasps] Reload… someone… | Nachladen … jemand … | Pflichtgefühl bis zuletzt |
| `voices/units/crossbowman_death_03.ogg` | [groans] I missed… nothing… | Ich hab … nichts verfehlt … | stolz-müde |

### crossbowman_victory (Event `voice.crossbowman.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/crossbowman_victory_01.ogg` | Clean shooting, everyone. | Saubere Arbeit, alle. | zufrieden, ruhig |
| `voices/units/crossbowman_victory_02.ogg` | That's the last bolt for today. | Das war der letzte Bolzen für heute. | trocken |
| `voices/units/crossbowman_victory_03.ogg` | Count the bolts, not the dead. | Zählt die Bolzen, nicht die Toten. | müde-trocken |

### 12.2 Glutkrone – `halberdier` (Hellebardier)

**Stimme:** `AC_unit_halberdier (Profil D1.7)` · **Modell:** Eleven v3, *Natural* · Speed 0,95
**Haltung:** bärbeißiger Gardist mit tiefer Stimme, ein Pferdebrecher; liebt Formationen, mag keine Reiter.

### halberdier_select (Event `voice.halberdier.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_select_01.ogg` | Halberd up. | Hellebarde hoch. | tief, knapp |
| `voices/units/halberdier_select_02.ogg` | Present and braced. | Zur Stelle und bereit. | steif, stolz |
| `voices/units/halberdier_select_03.ogg` | Let them come on horseback. | Sollen sie nur zu Pferd kommen. | grimmig-amüsiert |
| `voices/units/halberdier_select_04.ogg` | Orders, Warden? | Befehle, Wächterin? | respektvoll |

### halberdier_move (Event `voice.halberdier.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_move_01.ogg` | Marching. | Marschiere. | knapp |
| `voices/units/halberdier_move_02.ogg` | Steady pace, keep the shafts level. | Gleichmäßiger Schritt, Stangen waagerecht. | kommandoartig |
| `voices/units/halberdier_move_03.ogg` | Heavy boots, honest road. | Schwere Stiefel, ehrliche Straße. | brummig |
| `voices/units/halberdier_move_04.ogg` | Moving out. | Rücken ab. | neutral |

### halberdier_attack (Event `voice.halberdier.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_attack_01.ogg` | Brace! Brace! | Anlegen! Anlegen! | laut, kommandierend |
| `voices/units/halberdier_attack_02.ogg` | Horse-breaker! | Pferdebrecher! | Schlachtruf |
| `voices/units/halberdier_attack_03.ogg` | Hook and drag! | Haken und zerren! | grimmig, Anstrengung |
| `voices/units/halberdier_attack_04.ogg` | For the Crown! | Für die Krone! | kräftig |

### halberdier_retreat (Event `voice.halberdier.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_retreat_01.ogg` | Pull back, pull back! | Zurück, zurück! | laut |
| `voices/units/halberdier_retreat_02.ogg` | Reform the line! | Linie neu bilden! | befehlend |
| `voices/units/halberdier_retreat_03.ogg` | Back, damn you, back! | Zurück, verflucht! | gepresst |

### halberdier_hit (Event `voice.halberdier.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_hit_01.ogg` | [grunts] Armor held. | Rüstung hält. | trotzig |
| `voices/units/halberdier_hit_02.ogg` | [groans] That'll bruise. | Das gibt einen Bluterguss. | trocken |
| `voices/units/halberdier_hit_03.ogg` | [hisses] Ah! | Ah! | kurz |

### halberdier_death (Event `voice.halberdier.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_death_01.ogg` | [exhales] Hold the line… | Haltet die Linie … | verhauchend |
| `voices/units/halberdier_death_02.ogg` | [groans] Forgive me, Captain. | Vergebt mir, Hauptmann. | schwer |
| `voices/units/halberdier_death_03.ogg` | [gasps] Not… one step… | Nicht … einen Schritt … | gebrochen-stur |

### halberdier_victory (Event `voice.halberdier.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/halberdier_victory_01.ogg` | Not a hoof got past us. | Kein Huf kam an uns vorbei. | zufrieden |
| `voices/units/halberdier_victory_02.ogg` | Shafts down. Well done. | Stangen runter. Gut gemacht. | ruhig |
| `voices/units/halberdier_victory_03.ogg` | The line held. | Die Linie hielt. | stolz |

### 12.3 Glutkrone – `shieldbearer` (Schildträger)

**Stimme:** `AC_unit_shieldbearer (Profil D1.8)` · **Modell:** Eleven v3, *Natural* · Speed 0,92
**Haltung:** geduldiger, schwerer Bollwerkkämpfer; spricht wenig, sehr tief, mit stoischer Ruhe.

### shieldbearer_select (Event `voice.shieldbearer.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_select_01.ogg` | Shield up. | Schild hoch. | tief, ruhig |
| `voices/units/shieldbearer_select_02.ogg` | I'm your wall. | Ich bin Eure Mauer. | stoisch |
| `voices/units/shieldbearer_select_03.ogg` | Behind me. | Hinter mich. | ruhig-bestimmt |
| `voices/units/shieldbearer_select_04.ogg` | Ready. | Bereit. | einsilbig |

### shieldbearer_move (Event `voice.shieldbearer.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_move_01.ogg` | Advancing, shields locked. | Rücke vor, Schilde verhakt. | gleichmäßig |
| `voices/units/shieldbearer_move_02.ogg` | Slow and steady. | Langsam und stetig. | brummend |
| `voices/units/shieldbearer_move_03.ogg` | To the front. | Nach vorn. | knapp |
| `voices/units/shieldbearer_move_04.ogg` | Moving. | Marschiere. | neutral |

### shieldbearer_attack (Event `voice.shieldbearer.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_attack_01.ogg` | Shield wall! | Schildwall! | laut, kommandierend |
| `voices/units/shieldbearer_attack_02.ogg` | Push! | Drücken! | Anstrengung |
| `voices/units/shieldbearer_attack_03.ogg` | Not past me! | Nicht an mir vorbei! | hart |
| `voices/units/shieldbearer_attack_04.ogg` | Hold and strike! | Halten und zuschlagen! | rhythmisch |

### shieldbearer_retreat (Event `voice.shieldbearer.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_retreat_01.ogg` | Shields back, back! | Schilde zurück, zurück! | laut |
| `voices/units/shieldbearer_retreat_02.ogg` | I can't hold alone! | Allein halte ich nicht! | gepresst |
| `voices/units/shieldbearer_retreat_03.ogg` | Fall back behind the wall! | Zurück hinter die Mauer! | dringlich |

### shieldbearer_hit (Event `voice.shieldbearer.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_hit_01.ogg` | [grunts] Shield took it. | Der Schild hat's abgefangen. | trocken |
| `voices/units/shieldbearer_hit_02.ogg` | [groans] Heavy one. | Ein schwerer. | brummig |
| `voices/units/shieldbearer_hit_03.ogg` | [grunts] Still standing. | Steh noch. | stur |

### shieldbearer_death (Event `voice.shieldbearer.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_death_01.ogg` | [exhales] The wall… falls… | Die Mauer … fällt … | schwer, langsam |
| `voices/units/shieldbearer_death_02.ogg` | [groans] Behind… me… | Hinter … mir … | beschützend |
| `voices/units/shieldbearer_death_03.ogg` | [sighs] Rest… | Ruhe … | erlöst |

### shieldbearer_victory (Event `voice.shieldbearer.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_victory_01.ogg` | Wall still stands. | Die Mauer steht noch. | ruhig-stolz |
| `voices/units/shieldbearer_victory_02.ogg` | Not a scratch. Well, some. | Kein Kratzer. Na gut, ein paar. | trocken-humorvoll |
| `voices/units/shieldbearer_victory_03.ogg` | Shields down. | Schilde runter. | erleichtert |

### shieldbearer_ability (Event `voice.shieldbearer.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/shieldbearer_ability_01.ogg` | Shield wall! Close up! | Schildwall! Aufschließen! | laut, befehlend |
| `voices/units/shieldbearer_ability_02.ogg` | Lock shields! | Schilde verhaken! | kurz, scharf |
| `voices/units/shieldbearer_ability_03.ogg` | Form the wall! | Bildet die Mauer! | kräftig |

### 12.4 Glutkrone – `scout_rider` (Spähreiter)

**Stimme:** `AC_unit_scout_rider (Profil D1.9)` · **Modell:** Eleven v3, *Creative* · Speed 1,08
**Haltung:** junger, flinker Grenzreiter, wach und etwas übermütig; spricht schnell, lacht gern.

### scout_rider_select (Event `voice.scout_rider.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_select_01.ogg` | Ready to ride! | Bereit zum Ritt! | schnell, munter |
| `voices/units/scout_rider_select_02.ogg` | What do you need seen? | Was soll ich auskundschaften? | neugierig |
| `voices/units/scout_rider_select_03.ogg` | Good horse, good eyes. | Gutes Pferd, gute Augen. | zufrieden |
| `voices/units/scout_rider_select_04.ogg` | Point the way. | Zeigt mir den Weg. | eifrig |

### scout_rider_move (Event `voice.scout_rider.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_move_01.ogg` | Off like the wind! | Los wie der Wind! | freudig |
| `voices/units/scout_rider_move_02.ogg` | Ahead of the dust! | Dem Staub voraus! | schnell |
| `voices/units/scout_rider_move_03.ogg` | I'll be back before supper. | Ich bin vor dem Abendessen zurück. | leichthin |
| `voices/units/scout_rider_move_04.ogg` | Hyah! | Hü! | Zuruf, kurz |

### scout_rider_attack (Event `voice.scout_rider.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_attack_01.ogg` | Hyah! Ride them down! | Hü! Reitet sie nieder! | laut |
| `voices/units/scout_rider_attack_02.ogg` | Quick and clean! | Schnell und sauber! | scharf |
| `voices/units/scout_rider_attack_03.ogg` | For Brennhollow! | Für Brennhollow! | Schlachtruf |
| `voices/units/scout_rider_attack_04.ogg` | Come on, then! | Na kommt schon! | herausfordernd |

### scout_rider_retreat (Event `voice.scout_rider.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_retreat_01.ogg` | Too many! Ride! | Zu viele! Reiten! | panisch |
| `voices/units/scout_rider_retreat_02.ogg` | Fall back, fall back! | Rückzug, Rückzug! | gehetzt |
| `voices/units/scout_rider_retreat_03.ogg` | Faster, girl, faster! | Schneller, Mädchen, schneller! | zum Pferd |

### scout_rider_hit (Event `voice.scout_rider.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_hit_01.ogg` | [gasps] Ah, my side! | Ah, meine Seite! | scharf |
| `voices/units/scout_rider_hit_02.ogg` | [grunts] Scratched. | Gestreift. | überspielt |
| `voices/units/scout_rider_hit_03.ogg` | [hisses] Hold on, girl! | Halt durch, Mädchen! | besorgt |

### scout_rider_death (Event `voice.scout_rider.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_death_01.ogg` | [gasps] Tell them… what I saw… | Sagt ihnen … was ich sah … | verhauchend |
| `voices/units/scout_rider_death_02.ogg` | [exhales] Ride on… | Reitet weiter … | leise |
| `voices/units/scout_rider_death_03.ogg` | [sighs] Wind's… cold… | Der Wind … ist kalt … | erlöst |

### scout_rider_victory (Event `voice.scout_rider.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/scout_rider_victory_01.ogg` | Told you I was fast! | Ich sagte doch, ich bin schnell! | lachend |
| `voices/units/scout_rider_victory_02.ogg` | The road is clear. | Der Weg ist frei. | zufrieden |
| `voices/units/scout_rider_victory_03.ogg` | Back with news, as promised. | Zurück mit Neuigkeiten, wie versprochen. | munter |

### 12.5 Glutkrone – `horse_archer` (Berittener Bogenschütze)

**Stimme:** `AC_unit_horse_archer (Profil D1.10)` · **Modell:** Eleven v3, *Creative* · Speed 1,05
**Haltung:** geschmeidige Steppen-Söldnerin, selbstsicher, ein wenig spöttisch; schießt, bevor andere zielen.

### horse_archer_select (Event `voice.horse_archer.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_select_01.ogg` | Bow strung, horse fed. | Bogen gespannt, Pferd gefüttert. | selbstsicher |
| `voices/units/horse_archer_select_02.ogg` | Make it a moving target. | Macht es zum bewegten Ziel. | spöttisch |
| `voices/units/horse_archer_select_03.ogg` | You have my bow. | Ihr habt meinen Bogen. | ruhig |
| `voices/units/horse_archer_select_04.ogg` | Where to, Warden? | Wohin, Wächterin? | locker |

### horse_archer_move (Event `voice.horse_archer.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_move_01.ogg` | Riding wide. | Reite weit aus. | kurz |
| `voices/units/horse_archer_move_02.ogg` | Catch me if you can. | Fangt mich, wenn ihr könnt. | neckend |
| `voices/units/horse_archer_move_03.ogg` | Flank, then fire. | Flanke, dann Feuer. | knapp |
| `voices/units/horse_archer_move_04.ogg` | Hyah! | Hü! | Zuruf |

### horse_archer_attack (Event `voice.horse_archer.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_attack_01.ogg` | Arrows away! | Pfeile los! | laut |
| `voices/units/horse_archer_attack_02.ogg` | Shoot and ride! | Schießen und reiten! | rhythmisch |
| `voices/units/horse_archer_attack_03.ogg` | Too slow! | Zu langsam! | spöttisch |
| `voices/units/horse_archer_attack_04.ogg` | Rain on them! | Lasst es auf sie regnen! | Schlachtruf |

### horse_archer_retreat (Event `voice.horse_archer.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_retreat_01.ogg` | Disengage! Ride out! | Absetzen! Raus hier! | gehetzt |
| `voices/units/horse_archer_retreat_02.ogg` | They're too close! | Sie sind zu nah! | angespannt |
| `voices/units/horse_archer_retreat_03.ogg` | Falling back, shoot as you go! | Rückzug, schießt beim Reiten! | dringlich |

### horse_archer_hit (Event `voice.horse_archer.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_hit_01.ogg` | [gasps] Arrow in my coat! | Ein Pfeil im Mantel! | scharf |
| `voices/units/horse_archer_hit_02.ogg` | [grunts] Not bad. | Nicht schlecht. | überspielt |
| `voices/units/horse_archer_hit_03.ogg` | [hisses] Easy, girl! | Ruhig, Mädchen! | zum Pferd |

### horse_archer_death (Event `voice.horse_archer.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_death_01.ogg` | [exhales] Last… arrow… | Letzter … Pfeil … | verhauchend |
| `voices/units/horse_archer_death_02.ogg` | [gasps] The wind took me… | Der Wind hat mich geholt … | poetisch-leise |
| `voices/units/horse_archer_death_03.ogg` | [sighs] Ride on without me… | Reitet ohne mich weiter … | ruhig |

### horse_archer_victory (Event `voice.horse_archer.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/horse_archer_victory_01.ogg` | They never got close. | Sie kamen nie nah heran. | zufrieden |
| `voices/units/horse_archer_victory_02.ogg` | Quiver's light, heart's lighter. | Köcher leicht, Herz leichter. | gelöst |
| `voices/units/horse_archer_victory_03.ogg` | Another hunt done. | Wieder eine Jagd vorbei. | ruhig |

### 12.6 Glutkrone – `battle_mage` (Kampfmagier, Feuerschule)

**Stimme:** `AC_unit_battle_mage (Profil D1.11)` · **Modell:** Eleven v3, *Creative* · Speed 1,0
**Haltung:** disziplinierter Feuermagier der Flammenakademie von Kaldris; höflich, gespannt, mit unterdrückter Hitze in der Stimme.

### battle_mage_select (Event `voice.battle_mage.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_select_01.ogg` | The flame answers. | Die Flamme antwortet. | ruhig, tief |
| `voices/units/battle_mage_select_02.ogg` | Ember and will. | Glut und Wille. | knapp |
| `voices/units/battle_mage_select_03.ogg` | Name the target. | Nennt das Ziel. | konzentriert |
| `voices/units/battle_mage_select_04.ogg` | I am ready to burn. | Ich bin bereit zu brennen. | leise-intensiv |

### battle_mage_move (Event `voice.battle_mage.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_move_01.ogg` | Walking in the light. | Ich gehe im Licht. | gelassen |
| `voices/units/battle_mage_move_02.ogg` | Keep your distance from my hem. | Haltet Abstand zu meinem Saum. | trocken-warnend |
| `voices/units/battle_mage_move_03.ogg` | As the flame wills. | Wie die Flamme will. | ruhig |
| `voices/units/battle_mage_move_04.ogg` | Moving. | Bewegung. | knapp |

### battle_mage_attack (Event `voice.battle_mage.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_attack_01.ogg` | Burn! | Brennt! | scharf, laut |
| `voices/units/battle_mage_attack_02.ogg` | Ash to ash! | Asche zu Asche! | grimmig |
| `voices/units/battle_mage_attack_03.ogg` | By the Eternal Flame! | Bei der Ewigen Flamme! | hymnisch |
| `voices/units/battle_mage_attack_04.ogg` | Kindle! | Entfacht! | knapp, Zauberruf |

### battle_mage_retreat (Event `voice.battle_mage.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_retreat_01.ogg` | Shield me, I'm drained! | Deckt mich, ich bin erschöpft! | dringlich |
| `voices/units/battle_mage_retreat_02.ogg` | Too close, too close! | Zu nah, zu nah! | panisch |
| `voices/units/battle_mage_retreat_03.ogg` | Fall back to the line! | Zurück zur Linie! | gehetzt |

### battle_mage_hit (Event `voice.battle_mage.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_hit_01.ogg` | [gasps] My concentration! | Meine Konzentration! | scharf |
| `voices/units/battle_mage_hit_02.ogg` | [hisses] Burns… the wrong way. | Brennt … falsch herum. | bissig |
| `voices/units/battle_mage_hit_03.ogg` | [grunts] Hold steady. | Ruhig bleiben. | zu sich selbst |

### battle_mage_death (Event `voice.battle_mage.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_death_01.ogg` | [exhales] The flame… goes out… | Die Flamme … erlischt … | verhauchend |
| `voices/units/battle_mage_death_02.ogg` | [gasps] Let it… not be… in vain… | Lasst es … nicht umsonst sein … | leise |
| `voices/units/battle_mage_death_03.ogg` | [sighs] Ash… at last. | Asche … endlich. | erlöst |

### battle_mage_victory (Event `voice.battle_mage.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_victory_01.ogg` | The embers are quiet. | Die Glut ist still. | zufrieden-ruhig |
| `voices/units/battle_mage_victory_02.ogg` | Let the fire rest. | Lasst das Feuer ruhen. | leise |
| `voices/units/battle_mage_victory_03.ogg` | Light prevailed. | Das Licht hat gesiegt. | feierlich |

### battle_mage_ability (Event `voice.battle_mage.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battle_mage_ability_01.ogg` | Fire, answer me! | Feuer, antworte mir! | laut, Zauberruf |
| `voices/units/battle_mage_ability_02.ogg` | Cinders, rise! | Funken, erhebt euch! | scharf |
| `voices/units/battle_mage_ability_03.ogg` | Flame, take them! | Flamme, nimm sie! | hart |

### 12.7 Glutkrone – `priest` (Priester der Flamme)

**Stimme:** `AC_unit_priest (Profil D1.12)` · **Modell:** Eleven v3, *Natural* · Speed 0,96
**Haltung:** sanfter, warmer Flammenpriester der Dorfkapelle; zitiert Liturgie, hat im Kampf Angst, aber hilft trotzdem.

### priest_select (Event `voice.priest.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_select_01.ogg` | The Flame be with you. | Die Flamme sei mit Euch. | warm, sanft |
| `voices/units/priest_select_02.ogg` | How may I serve? | Wie kann ich dienen? | freundlich |
| `voices/units/priest_select_03.ogg` | I am here, child. | Ich bin hier, Kind. | tröstend |
| `voices/units/priest_select_04.ogg` | Let the light guide us. | Lasst das Licht uns leiten. | ruhig |

### priest_move (Event `voice.priest.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_move_01.ogg` | We walk in the light. | Wir gehen im Licht. | gelassen |
| `voices/units/priest_move_02.ogg` | Lead on, I follow. | Geht voran, ich folge. | ruhig |
| `voices/units/priest_move_03.ogg` | Mind the wounded. | Gebt auf die Verwundeten acht. | sorgend |
| `voices/units/priest_move_04.ogg` | Yes, yes, going. | Ja, ja, ich gehe. | leicht gehetzt |

### priest_attack (Event `voice.priest.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_attack_01.ogg` | Back, creatures of the dark! | Zurück, Geschöpfe der Dunkelheit! | mutig, zitternd |
| `voices/units/priest_attack_02.ogg` | Flame, protect us! | Flamme, beschütze uns! | laut |
| `voices/units/priest_attack_03.ogg` | In the Eternal Flame's name! | Im Namen der Ewigen Flamme! | hymnisch |
| `voices/units/priest_attack_04.ogg` | Stand away from them! | Weg von ihnen! | entschlossen |

### priest_retreat (Event `voice.priest.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_retreat_01.ogg` | I can't! I can't! | Ich kann nicht! Ich kann nicht! | panisch |
| `voices/units/priest_retreat_02.ogg` | Protect the healers! | Beschützt die Heiler! | dringlich |
| `voices/units/priest_retreat_03.ogg` | Flame, forgive me! | Flamme, vergib mir! | verzweifelt |

### priest_hit (Event `voice.priest.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_hit_01.ogg` | [gasps] Oh! Mercy! | Oh! Gnade! | erschrocken |
| `voices/units/priest_hit_02.ogg` | [groans] It's only pain. | Es ist nur Schmerz. | tapfer |
| `voices/units/priest_hit_03.ogg` | [hisses] Forgive them… | Vergib ihnen … | gepresst |

### priest_death (Event `voice.priest.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_death_01.ogg` | [exhales] Into the Flame… I go… | In die Flamme … gehe ich … | friedlich |
| `voices/units/priest_death_02.ogg` | [gasps] Who… will tend… them… | Wer … wird … sie pflegen … | besorgt |
| `voices/units/priest_death_03.ogg` | [sighs] Light… take me. | Licht … nimm mich. | erlöst |

### priest_victory (Event `voice.priest.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_victory_01.ogg` | Praise the Flame. Tend the wounded. | Lob der Flamme. Versorgt die Verwundeten. | erleichtert |
| `voices/units/priest_victory_02.ogg` | We lived. Thank the light. | Wir leben. Dank dem Licht. | gerührt |
| `voices/units/priest_victory_03.ogg` | Peace to the fallen. | Friede den Gefallenen. | feierlich |

### priest_ability (Event `voice.priest.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/priest_ability_01.ogg` | Be healed! | Werdet geheilt! | warm, kräftig |
| `voices/units/priest_ability_02.ogg` | Flame, restore them! | Flamme, richte sie auf! | hymnisch |
| `voices/units/priest_ability_03.ogg` | Rise and be blessed! | Erhebt euch und seid gesegnet! | strahlend |

### 12.8 Glutkrone – `battering_ram` (Rammbock, Bedienmannschaft)

**Stimme:** `AC_unit_ram_crew (Profil D1.13)` · **Modell:** Eleven v3, *Creative* (Mehrstimmen-Chor, 3–4 Männer) · Speed 1,0
**Haltung:** Gruppe schwitzender Pioniere; rufen im Takt, Stoßrufe, Hau-Ruck. Mehrere Sprecher überlagert, dumpfe Anstrengung.

### battering_ram_select (Event `voice.battering_ram.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battering_ram_select_01.ogg` | Ram's ready! | Ramme bereit! | Chor, Zuruf |
| `voices/units/battering_ram_select_02.ogg` | Aye, the gate won't hold. | Jawohl, das Tor hält nicht. | grimmig, rau |
| `voices/units/battering_ram_select_03.ogg` | Put your backs into it! | Legt Euch ins Zeug! | Anfeuerung |
| `voices/units/battering_ram_select_04.ogg` | Heave, lads! | Hau ruck, Jungs! | rhythmisch |

### battering_ram_move (Event `voice.battering_ram.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battering_ram_move_01.ogg` | Push, push! | Schieben, schieben! | Anstrengung, im Takt |
| `voices/units/battering_ram_move_02.ogg` | Mind the wheels! | Achtet auf die Räder! | hektisch |
| `voices/units/battering_ram_move_03.ogg` | Easy… easy… | Langsam … langsam … | gepresst |
| `voices/units/battering_ram_move_04.ogg` | Roll her up! | Rollt sie vor! | Zuruf |

### battering_ram_attack (Event `voice.battering_ram.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battering_ram_attack_01.ogg` | Swing! Swing! | Schwingen! Schwingen! | laut, im Takt |
| `voices/units/battering_ram_attack_02.ogg` | Break it down! | Reißt es nieder! | wild |
| `voices/units/battering_ram_attack_03.ogg` | All together — now! | Alle zusammen – jetzt! | Chor |
| `voices/units/battering_ram_attack_04.ogg` | Smash the gate! | Zerschmettert das Tor! | Schlachtruf |

### battering_ram_death (Event `voice.battering_ram.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/battering_ram_death_01.ogg` | [shouting] She's breaking up! | Sie bricht auseinander! | panisch, Chor |
| `voices/units/battering_ram_death_02.ogg` | [groans] The ram… is lost… | Der Rammbock … ist verloren … | gebrochen |
| `voices/units/battering_ram_death_03.ogg` | [shouting] Run, lads, run! | Lauft, Jungs, lauft! | Chor, hektisch |

### 12.9 Glutkrone – `catapult` (Katapult, Bedienmannschaft)

**Stimme:** `AC_unit_catapult_crew (Profil D1.14)` · **Modell:** Eleven v3, *Creative* (2–3 Sprecher) · Speed 1,0
**Haltung:** erfahrene Geschützmeister, nüchterne Zurufe in Kommandoform, Winden und Seile hörbar.

### catapult_select (Event `voice.catapult.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/catapult_select_01.ogg` | Engine's loaded. | Geschütz ist geladen. | nüchtern |
| `voices/units/catapult_select_02.ogg` | Give us a target. | Gebt uns ein Ziel. | knapp |
| `voices/units/catapult_select_03.ogg` | Wind her back! | Spannt sie zurück! | Zuruf |
| `voices/units/catapult_select_04.ogg` | Range and bearing, sir? | Reichweite und Richtung, Herr? | fachlich |

### catapult_move (Event `voice.catapult.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/catapult_move_01.ogg` | Haul her, haul her! | Zieht sie, zieht sie! | Anstrengung |
| `voices/units/catapult_move_02.ogg` | Careful with the arm! | Vorsicht mit dem Arm! | besorgt |
| `voices/units/catapult_move_03.ogg` | Mind the ruts! | Achtet auf die Spurrillen! | hektisch |
| `voices/units/catapult_move_04.ogg` | Easy, easy! | Langsam, langsam! | gepresst |

### catapult_attack (Event `voice.catapult.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/catapult_attack_01.ogg` | Loose! | Los! | laut, Kommando |
| `voices/units/catapult_attack_02.ogg` | Fire! | Feuer! | scharf |
| `voices/units/catapult_attack_03.ogg` | Stand clear — loose! | Zurücktreten – los! | Warnruf |
| `voices/units/catapult_attack_04.ogg` | Away, away! | Fort damit! | Triumph |

### catapult_death (Event `voice.catapult.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/units/catapult_death_01.ogg` | [shouting] She's gone — the arm snapped! | Sie ist hin – der Arm ist gebrochen! | entsetzt |
| `voices/units/catapult_death_02.ogg` | [groans] Years of work… | Jahre an Arbeit … | gebrochen |
| `voices/units/catapult_death_03.ogg` | [shouting] Clear out! | Weg hier! | panisch |


---

## 13. Echtzeit-Militär – Helden Hal Venn und Torvald

Heldenstimmen gelten für `voices/heroes/` (Manifest +1 dB, Cooldown 300 ms). Situationen wie bei Isolde (Abschnitt 9): select, move, attack, retreat, hit, death, victory, ability, greet, levelup, lowhp.

### 13.1 Held – `hero_hal_venn` (Bruder Hal Venn, Flammenpriester von Brennhollow)

**Stimme:** `AC_hero_hal_venn (nutzt Profil von `char_hal_venn`, siehe Figurenleitfaden)` · **Modell:** Eleven v3, *Natural* · Speed 0,97
**Haltung:** freundlich-väterlicher Mentor, bedächtig, mit trockenem Humor; im Kampf überraschend entschlossen. Gleiche Stimme wie die Dialogfigur `char_hal_venn`.

### hero_hal_venn_select (Event `voice.hero_hal_venn.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_select_01.ogg` | Yes, Warden? | Ja, Wächterin? | warm |
| `voices/heroes/hero_hal_venn_select_02.ogg` | The Flame is with us. | Die Flamme ist mit uns. | ruhig |
| `voices/heroes/hero_hal_venn_select_03.ogg` | My hands are yours. | Meine Hände gehören Euch. | sanft |
| `voices/heroes/hero_hal_venn_select_04.ogg` | Ah, old bones, ready as ever. | Ach, alte Knochen, bereit wie immer. | selbstironisch |
| `voices/heroes/hero_hal_venn_select_05.ogg` | What does Brennhollow need of me? | Was braucht Brennhollow von mir? | sorgend |

### hero_hal_venn_move (Event `voice.hero_hal_venn.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_move_01.ogg` | Lead on, I'll follow. | Geht voran, ich folge. | ruhig |
| `voices/heroes/hero_hal_venn_move_02.ogg` | Slowly, my knees remember Aureth. | Langsam, meine Knie erinnern sich an Aureth. | trocken |
| `voices/heroes/hero_hal_venn_move_03.ogg` | To where the need is greatest. | Dorthin, wo die Not am größten ist. | ernst |
| `voices/heroes/hero_hal_venn_move_04.ogg` | Walking, walking. | Ich gehe, ich gehe. | gemütlich |
| `voices/heroes/hero_hal_venn_move_05.ogg` | Onward, then. | Dann vorwärts. | neutral |

### hero_hal_venn_attack (Event `voice.hero_hal_venn.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_attack_01.ogg` | In the Flame's name! | Im Namen der Flamme! | kräftig |
| `voices/heroes/hero_hal_venn_attack_02.ogg` | Be gone, shadows! | Fort mit euch, Schatten! | laut |
| `voices/heroes/hero_hal_venn_attack_03.ogg` | Enough! Back! | Genug! Zurück! | scharf |
| `voices/heroes/hero_hal_venn_attack_04.ogg` | Light, strike true! | Licht, triff sicher! | hymnisch |
| `voices/heroes/hero_hal_venn_attack_05.ogg` | I'll not have another village lost! | Ich dulde kein weiteres verlorenes Dorf! | entschlossen |

### hero_hal_venn_retreat (Event `voice.hero_hal_venn.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_retreat_01.ogg` | Fall back, fall back! | Zurück, zurück! | dringlich |
| `voices/heroes/hero_hal_venn_retreat_02.ogg` | Gather the wounded! | Sammelt die Verwundeten! | sorgend-laut |
| `voices/heroes/hero_hal_venn_retreat_03.ogg` | We live to fight again! | Wir leben, um wieder zu kämpfen! | angespannt |

### hero_hal_venn_hit (Event `voice.hero_hal_venn.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_hit_01.ogg` | [grunts] Hm. That stung. | Hm. Das hat gebrannt. | trocken |
| `voices/heroes/hero_hal_venn_hit_02.ogg` | [gasps] Flame, hold me! | Flamme, halte mich! | erschrocken |
| `voices/heroes/hero_hal_venn_hit_03.ogg` | [groans] Old, not dead! | Alt, nicht tot! | trotzig |

### hero_hal_venn_death (Event `voice.hero_hal_venn.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_death_01.ogg` | [exhales] Warden… the Flame… keeps you… | Wächterin … die Flamme … behüte Euch … | verhauchend, warm |
| `voices/heroes/hero_hal_venn_death_02.ogg` | [gasps] Tell Brennhollow… I tried… | Sagt Brennhollow … ich habe es versucht … | leise |
| `voices/heroes/hero_hal_venn_death_03.ogg` | [sighs] Into the light… | Ins Licht … | erlöst |

### hero_hal_venn_victory (Event `voice.hero_hal_venn.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_victory_01.ogg` | The Flame prevailed. | Die Flamme hat gesiegt. | erleichtert |
| `voices/heroes/hero_hal_venn_victory_02.ogg` | Praise be, we're alive. | Gelobt sei die Flamme, wir leben. | gerührt |
| `voices/heroes/hero_hal_venn_victory_03.ogg` | Let us tend the wounded. | Lasst uns die Verwundeten versorgen. | ruhig |

### hero_hal_venn_ability (Event `voice.hero_hal_venn.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_ability_01.ogg` | Flame, mend them! | Flamme, heile sie! | warm, laut |
| `voices/heroes/hero_hal_venn_ability_02.ogg` | Burn clean! | Brennt rein! | scharf |
| `voices/heroes/hero_hal_venn_ability_03.ogg` | Faith be our fire! | Glaube sei unser Feuer! | hymnisch |

### hero_hal_venn_greet (Event `voice.hero_hal_venn.greet`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_greet_01.ogg` | Warden. Good to see you standing. | Wächterin. Schön, Euch aufrecht zu sehen. | warm |
| `voices/heroes/hero_hal_venn_greet_02.ogg` | The settlement needs you, and so do I. | Die Siedlung braucht Euch, und ich auch. | ernst-freundlich |
| `voices/heroes/hero_hal_venn_greet_03.ogg` | I've kept the lamp lit. | Ich habe die Lampe am Brennen gehalten. | sanft |

### hero_hal_venn_levelup (Event `voice.hero_hal_venn.levelup`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_levelup_01.ogg` | The Flame burns brighter in me. | Die Flamme brennt heller in mir. | überrascht, froh |
| `voices/heroes/hero_hal_venn_levelup_02.ogg` | Stronger. Humbling. | Stärker. Demütigend. | bescheiden |
| `voices/heroes/hero_hal_venn_levelup_03.ogg` | Grace, grown. | Gnade, gewachsen. | leise |

### hero_hal_venn_lowhp (Event `voice.hero_hal_venn.lowhp`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_hal_venn_lowhp_01.ogg` | I must… tend myself… | Ich muss … mich selbst versorgen … | angestrengt |
| `voices/heroes/hero_hal_venn_lowhp_02.ogg` | Not yet, Flame, not yet. | Noch nicht, Flamme, noch nicht. | gepresst |
| `voices/heroes/hero_hal_venn_lowhp_03.ogg` | Bleeding. Naturally. | Ich blute. Natürlich. | trocken-schwach |

### 13.2 Held – `hero_torvald` (Torvald Eisenhand, Clanfürst von Skarnheim)

**Stimme:** `AC_hero_torvald (nutzt Profil von `char_torvald`, siehe Figurenleitfaden)` · **Modell:** Eleven v3, *Creative* · Speed 0,95
**Haltung:** raues, tiefes Nordclan-Organ, laut, grobschlächtig-herzlich; ehrt Ahnen, verachtet Feigheit, trinkt gern.

### hero_torvald_select (Event `voice.hero_torvald.select`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_select_01.ogg` | Torvald Ironhand listens. | Torvald Eisenhand hört zu. | tief, kräftig |
| `voices/heroes/hero_torvald_select_02.ogg` | Speak, and make it quick. | Sprecht, und macht's kurz. | ungeduldig |
| `voices/heroes/hero_torvald_select_03.ogg` | The ancestors are watching. | Die Ahnen sehen zu. | ernst |
| `voices/heroes/hero_torvald_select_04.ogg` | Ha! A fight, I hope? | Ha! Ein Kampf, hoffe ich? | gut gelaunt |
| `voices/heroes/hero_torvald_select_05.ogg` | Skarnheim stands with you. | Skarnheim steht an Eurer Seite. | ehrlich |

### hero_torvald_move (Event `voice.hero_torvald.move`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_move_01.ogg` | The road is long, the axe is light! | Der Weg ist lang, die Axt ist leicht! | grob-fröhlich |
| `voices/heroes/hero_torvald_move_02.ogg` | Forward, forward! | Vorwärts, vorwärts! | laut |
| `voices/heroes/hero_torvald_move_03.ogg` | My feet know the mountain, not this mud. | Meine Füße kennen den Berg, nicht diesen Matsch. | brummig |
| `voices/heroes/hero_torvald_move_04.ogg` | Lead on, Warden! | Führt an, Wächterin! | herzlich |
| `voices/heroes/hero_torvald_move_05.ogg` | To the next throat, then. | Dann zur nächsten Kehle. | trocken |

### hero_torvald_attack (Event `voice.hero_torvald.attack`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_attack_01.ogg` | For the clans! | Für die Clans! | Schlachtruf, laut |
| `voices/heroes/hero_torvald_attack_02.ogg` | Crush them! | Zerschmettert sie! | wild |
| `voices/heroes/hero_torvald_attack_03.ogg` | Ancestors, watch me! | Ahnen, seht mir zu! | kehlig |
| `voices/heroes/hero_torvald_attack_04.ogg` | Stand and die! | Steht und sterbt! | hart |
| `voices/heroes/hero_torvald_attack_05.ogg` | Hahh! Come to the iron! | Hahh! Kommt ans Eisen! | brüllend |

### hero_torvald_retreat (Event `voice.hero_torvald.retreat`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_retreat_01.ogg` | A clansman retreats for no one… today. | Ein Clanmann weicht vor niemandem … heute. | knurrend |
| `voices/heroes/hero_torvald_retreat_02.ogg` | Fall back to the shields! | Zurück zu den Schilden! | laut |
| `voices/heroes/hero_torvald_retreat_03.ogg` | Back, by the beard of my father! | Zurück, beim Bart meines Vaters! | fluchend |

### hero_torvald_hit (Event `voice.hero_torvald.hit`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_hit_01.ogg` | [grunts] Is that all? | Ist das alles? | höhnisch |
| `voices/heroes/hero_torvald_hit_02.ogg` | [roars] That one drew blood! | Der hat Blut gezogen! | wütend |
| `voices/heroes/hero_torvald_hit_03.ogg` | [groans] Hrnn! | Hrnn! | dumpf |

### hero_torvald_death (Event `voice.hero_torvald.death`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_death_01.ogg` | [exhales] Skarnheim… remember… | Skarnheim … erinnere dich … | verhauchend |
| `voices/heroes/hero_torvald_death_02.ogg` | [gasps] Father… I come… | Vater … ich komme … | friedlich-leise |
| `voices/heroes/hero_torvald_death_03.ogg` | [roars weakly] The… mountain… calls… | Der … Berg … ruft … | gebrochen, rau |

### hero_torvald_victory (Event `voice.hero_torvald.victory`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_victory_01.ogg` | Ha! Drink to that! | Ha! Darauf trinken wir! | dröhnend |
| `voices/heroes/hero_torvald_victory_02.ogg` | The ancestors are pleased. | Die Ahnen sind zufrieden. | zufrieden |
| `voices/heroes/hero_torvald_victory_03.ogg` | None can say the clans bow. | Keiner kann sagen, die Clans beugten sich. | stolz |

### hero_torvald_ability (Event `voice.hero_torvald.ability`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_ability_01.ogg` | Hear me, mountain! | Hört mich, Berge! | laut, Ruf |
| `voices/heroes/hero_torvald_ability_02.ogg` | Ancestors, lend me iron! | Ahnen, leiht mir Eisen! | kehlig |
| `voices/heroes/hero_torvald_ability_03.ogg` | Break! | Brecht! | kurz, hart |

### hero_torvald_greet (Event `voice.hero_torvald.greet`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_greet_01.ogg` | Warden Isolde! Your ale is thin, but your cause is good. | Wächterin Isolde! Euer Bier ist dünn, Eure Sache gut. | herzlich-grob |
| `voices/heroes/hero_torvald_greet_02.ogg` | Skarnheim sends iron, and me. | Skarnheim schickt Eisen, und mich. | stolz |
| `voices/heroes/hero_torvald_greet_03.ogg` | Point me at the enemy. | Richtet mich auf den Feind. | ungeduldig |

### hero_torvald_levelup (Event `voice.hero_torvald.levelup`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_levelup_01.ogg` | The ancestors' strength grows in me! | Die Kraft der Ahnen wächst in mir! | dröhnend |
| `voices/heroes/hero_torvald_levelup_02.ogg` | Heavier fists, lighter heart! | Schwerere Fäuste, leichteres Herz! | lachend |
| `voices/heroes/hero_torvald_levelup_03.ogg` | More iron in the blood! | Mehr Eisen im Blut! | froh |

### hero_torvald_lowhp (Event `voice.hero_torvald.lowhp`)
| Ziel-Datei | EN-Text | DE | Regie |
|---|---|---|---|
| `voices/heroes/hero_torvald_lowhp_01.ogg` | I've had worse — I think. | Ich hatte Schlimmeres – glaube ich. | keuchend-trotzig |
| `voices/heroes/hero_torvald_lowhp_02.ogg` | Not… yet… ancestors! | Noch … nicht … Ahnen! | gepresst |
| `voices/heroes/hero_torvald_lowhp_03.ogg` | My blood is leaving… | Mein Blut verlässt mich … | schwach |


---

## 14. Echtzeit-Militär – Die Hohlen (neue Einheiten)

Hohle Stimmen laufen im Manifest mit −2 dB, Cooldown 400 ms, max. 2 Instanzen; Nachbearbeitung wie Abschnitt 6 (Hohle-Kette, Pitch −2), Loudness −17 LUFS.

### 14.1 Hohle – `hollow_digger` (Grabgräber)

**Stimme:** `AC_unit_hollow_digger (Profil D5.3)` · **Modell:** Eleven v3, *Creative* · Speed 0,9
**Haltung:** arbeitsamer Wiedergänger, der im Tod weiter schürft; murmelt Fetzen von Werkzeugen und Schichten. Wie `hollow_thrall`: Hohle-Kette, Pitch −2.

### hollow_digger_attack (Event `voice.hollow_digger.attack`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_digger_attack_01.ogg` | [rasping] Mine… mine… | Mein … mein … | gierig, kehlig |
| `voices/units/hollow_digger_attack_02.ogg` | [groans] Not the… shovel… | Nicht die … Schaufel … | klagend |
| `voices/units/hollow_digger_attack_03.ogg` | [hisses] Hhhaah! | Hhhaah! | wortlos |
| `voices/units/hollow_digger_attack_04.ogg` | …shift's… not over… | … Schicht … nicht vorbei … | gemurmelt |

### hollow_digger_hit (Event `voice.hollow_digger.hit`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_digger_hit_01.ogg` | [groans] Uhh — back to work… | Uhh – zurück an die Arbeit … | gleichgültig |
| `voices/units/hollow_digger_hit_02.ogg` | [gasps] Hhkk — | Hhkk – | trockenes Keuchen |
| `voices/units/hollow_digger_hit_03.ogg` | …dust… in my… eyes… | … Staub … in den … Augen … | verloren |

### hollow_digger_death (Event `voice.hollow_digger.death`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/hollow_digger_death_01.ogg` | [sighs] …shift's… done… | … Schicht … vorbei … | erlöst |
| `voices/units/hollow_digger_death_02.ogg` | [exhales] …deeper… deeper… | … tiefer … tiefer … | verhauchend |
| `voices/units/hollow_digger_death_03.ogg` | [rattling breath] Hhhaaahh… | Hhhaaahh … | langes Röcheln |

### 14.2 Hohle – `ghoul` (Ghul)

**Stimme:** `AC_unit_ghoul (Profil D5.4)` · **Modell:** Eleven v3, *Creative* + SFX-Layer · Speed 1,1
**Haltung:** ausgehungerte Leichenfresser: hohes Kreischen, Schmatzen, Schnappen; fast keine Wörter. Basis: Voice Changer oder SFX-Prompt, danach Hohle-Kette.

### ghoul_attack (Event `voice.ghoul.attack`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ghoul_attack_01.ogg` | [screeches] Hhheeeeee! | Hhheeeee! | hohes Kreischen |
| `voices/units/ghoul_attack_02.ogg` | [snarls] Hungry… hungry… | Hunger … Hunger … | gierig, schmatzend |
| `voices/units/ghoul_attack_03.ogg` | [hisses] Fresh… fresh… | Frisch … frisch … | lüstern |
| `voices/units/ghoul_attack_04.ogg` | [wet growl] Grrrhhh! | Grrrhhh! | tiefes, nasses Knurren |

### ghoul_hit (Event `voice.ghoul.hit`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ghoul_hit_01.ogg` | [yelps] Hkkh! | Hkkh! | kurzer Aufschrei |
| `voices/units/ghoul_hit_02.ogg` | [snarls] Hrrrk! | Hrrrk! | wütend |
| `voices/units/ghoul_hit_03.ogg` | [whines] Hnnn… | Hnnn … | kläglich |

### ghoul_death (Event `voice.ghoul.death`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ghoul_death_01.ogg` | [shrieks] Heeeeaaah… | Heeeaaah … | abfallendes Kreischen |
| `voices/units/ghoul_death_02.ogg` | [gurgles] Hungry… still… | Hunger … noch … | röchelnd |
| `voices/units/ghoul_death_03.ogg` | [rattling breath] Hhhh… | Hhhh … | dünnes Ausatmen |

### 14.3 Hohle – `ash_witch` (Aschhexe)

**Stimme:** `AC_unit_ash_witch (Profil D5.5)` · **Modell:** Eleven v3, *Creative* mit `[whispers]` · Speed 0,92
**Haltung:** flüsternde Nekromantin, kalt-berechnend, spricht wie eine Lehrerin zu ihren Toten; Hohle-Kette mit mehr Hall-Anteil.

### ash_witch_attack (Event `voice.ash_witch.attack`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ash_witch_attack_01.ogg` | [whispers] Burn for me. | Brennt für mich. | kalt, flüsternd |
| `voices/units/ash_witch_attack_02.ogg` | [hisses] Ash and bone. | Asche und Knochen. | scharf |
| `voices/units/ash_witch_attack_03.ogg` | [whispers] You were warm once. | Du warst einmal warm. | spöttisch |
| `voices/units/ash_witch_attack_04.ogg` | [cold laugh] Come closer. | Kommt näher. | leise-lockend |

### ash_witch_hit (Event `voice.ash_witch.hit`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ash_witch_hit_01.ogg` | [hisses] Insolent flesh! | Unverschämtes Fleisch! | gereizt |
| `voices/units/ash_witch_hit_02.ogg` | [gasps] How… warm. | Wie … warm. | verblüfft |
| `voices/units/ash_witch_hit_03.ogg` | [hisses] Hnh! | Hnh! | kurz |

### ash_witch_death (Event `voice.ash_witch.death`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ash_witch_death_01.ogg` | [exhales] The Deep… is patient… | Das Tief … ist geduldig … | verhauchend |
| `voices/units/ash_witch_death_02.ogg` | [whispers] Rise… without me… | Erhebt euch … ohne mich … | leise |
| `voices/units/ash_witch_death_03.ogg` | [rattling breath] Dust to dust… | Staub zu Staub … | Röcheln |

### ash_witch_ability (Event `voice.ash_witch.ability`)
| Ziel-Datei | EN-Text/Prompt | DE | Regie |
|---|---|---|---|
| `voices/units/ash_witch_ability_01.ogg` | [whispers] Rise, my children. | Erhebt euch, meine Kinder. | beschwörend, tief |
| `voices/units/ash_witch_ability_02.ogg` | [commanding] Up from the ash! | Hoch aus der Asche! | befehlend |
| `voices/units/ash_witch_ability_03.ogg` | [chants] Bone to bone, flesh to flesh. | Knochen zu Knochen, Fleisch zu Fleisch. | Singsang, rituell |

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
