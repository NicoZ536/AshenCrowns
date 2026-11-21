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
