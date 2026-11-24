# Leistung: Messwerkzeuge, Messwerte, Grafikprofile

Ziel: 60 FPS auf Apple A18 Pro (5-Kern-GPU, 8 GB). **Alle Zahlen unten stammen von einem Linux-Rechner mit 4 CPU-Kernen und
Software-Vulkan (llvmpipe)** – CPU-Zeiten sind aussagekräftig (der A18 Pro hat mindestens vergleichbare Einzelkernleistung),
**GPU-Zeiten sind es nicht** (Software-Rasterizer) und dienen nur dem Vergleich der Passes untereinander. Eine Messung auf dem
Zielgerät steht aus (siehe Abschnitt 6).

## 1. Werkzeuge

| Werkzeug | Aufruf | Ausgabe |
|---|---|---|
| Spiel-Benchmark (ohne Fenster) | `ashen_crowns --bench <szene> [--frames n] [--profile ...] [--zoom z] [--no-audio]` | CPU-Zeit je Frame-Abschnitt (Mittel/p50/p95/Max), Instanzen je Gruppe (Boden/Schatten/Welt/Overlay), Lichter, Draw-Calls, interne Auflösung, Startzeit, Spitzenspeicher, GPU-Zeit je Pass |
| Szenen | `mission`, `sandbox`, `skirmish`, `battle`, `siege` (wie `--scene`), `stress[:n]` = Schlacht mit n Einheiten je Seite (Standard 200, also 400 Einheiten) | |
| Sim-Benchmark | `cargo run --release -p ashen_sim --example sim_bench -- [einheiten_je_seite=200] [sekunden=60]` | Tickzeit (Mittel/p50/p95/p99/Max) und Anteil je Systemphase |
| Rauchtest | `cargo test -p ashen_sim --test bench_smoke` | 400 Einheiten, 30 s Schlacht: läuft ohne Panik, Kampf findet statt, Tickzeit unter der Taktgrenze (großzügig, Debug-Profil) |
| Atlas/Bake-Zeiten | Logzeile `Atlas … erzeugt (2D-Kunst, 3D-Bake, Packen)` beim Start | |
| GPU-Zeitstempel im Spiel | Umgebungsvariable `ASHEN_GPU_TIMING=1` (im Benchmark automatisch) | Zeit je Pass, falls der Adapter `TIMESTAMP_QUERY` kennt (Metal: ja) |

Abschnitte des Benchmarks: *update* = `Game::update` (feste Sim-Ticks, Kamera, Eingabe, Ansicht-Update), *ui* = egui-Lauf,
*view* = Ansicht aufbauen (`WorldView::render`: Sprites, Lichter, Partikel, Atmosphäre), *submit* = Renderer-CPU
(Sichtbarkeit, Sortierung, Puffer, Befehlsaufzeichnung, egui-Malen). Der Sim-Tick läuft mit 20 Hz (`rules.tick_rate`), im Frame
fallen also im Mittel 1/3 Tick an (`update` zeigt deshalb ein kleines p50 und ein größeres p95).

Der Benchmark verwendet feste Zeitschritte (1/60 s), 30 Warmup-Frames, standardmäßig 300 gemessene Frames; die Szenen sind
deterministisch, Schwankungen entstehen durch das Betriebssystem.

## 2. Messwerte

### 2.1 Frame-CPU (Release, `lto = thin`; ms je Frame, Mittel / p95)

Vorher = Stand vor der Optimierung (bereits mit Benchmark-Werkzeug), nachher = mit `GroundRects`-Cache (Abschnitt 3).

| Szene (1600×900, intern 536×302) | view vorher | view nachher | CPU gesamt vorher | CPU gesamt nachher |
|---|---|---|---|---|
| `stress:200` (400 Einheiten im Kampf, 3 176 Instanzen, 22 Lichter) | 0,72 / 0,91 | **0,37 / 0,50** | 1,40 / 2,00 | **1,05 / 1,68** |
| `mission` (Mission 1, 1 265 Instanzen, 12 Lichter) | 0,40 / 0,59 | **0,15 / 0,21** | 1,06 / 1,49 | **0,80 / 1,27** |
| `siege` (Belagerungsschlacht, 1 751 Instanzen, 25 Lichter) | 0,54 / 0,77 | **0,23 / 0,32** | 1,29 / 1,69 | **0,92 / 1,24** |

`update` (0,04–0,28) und `submit` (0,3–0,35) blieben unverändert. Selbst mit 800 Einheiten und maximalem Zoom (480 Zeilen,
4 868 Instanzen) liegt die CPU-Zeit bei 3,9 ms im Mittel (p95 9,8 ms, Spitzen durch Pfadsuche, siehe 2.2) – deutlich unter dem
16,7-ms-Budget.

### 2.2 Simulation (`sim_bench`, 60 s Spielzeit, 20 Hz, Budget 50 ms/Tick)

| Last | Tick Mittel | p95 | p99 | Max | größte Phasen |
|---|---|---|---|---|---|
| 400 Einheiten (200 je Seite), Start 425, Ende 214 | **0,46 ms** | 1,00 | 1,52 | 8,7 | Bewegung/Kollision 39 %, Einheiten-Entscheidungen 26 %, Pfadverarbeitung 17 % |
| 800 Einheiten (400 je Seite) | 2,31 ms | 4,95 | 6,63 | 8,4 | **Pfadverarbeitung 64 %**, Bewegung 17 %, Entscheidungen 12 % |

Die Simulation belegt bei 400 Einheiten rund 1,4 % eines 60-FPS-Frames (0,46 ms × 20 Ticks/s). Sie ist **kein Engpass**; ein
Eingriff (Spatial-Queries, Pfadfindungs-Budget) wäre derzeit Optimierung ohne Messgrundlage und unterbleibt. Das Pfad-Budget
(`NODE_BUDGET_PER_TICK = 24 000`, `MAX_REQUESTS_PER_TICK`) begrenzt Spitzen bereits; erst jenseits von ~800 gleichzeitig
marschierenden Einheiten dominiert die Pfadverarbeitung.

### 2.3 GPU (llvmpipe, **nicht repräsentativ**) – Zeit je Pass in ms, `stress:200`

| Profil | gbuffer | lights | composite | bloom | final | Draw-Calls |
|---|---|---|---|---|---|---|
| Leistung | 14,0 | 1,8 | 2,4 | – | 13,5 | 7 |
| Ausgewogen | 13,5 | 1,7 | 2,6 | 6,2 | 15,9 | 12 |
| Qualität | 14,5 | 1,7 | 2,5 | 6,6 | 17,6 | 14 |

Lesehilfe: Im Software-Raster dominieren G-Buffer (viel Überzeichnung: ~1 140 Bodeninstanzen für ~330 sichtbare Kacheln) und der
Ausgabe-Pass (volle Fensterauflösung). Bloom kostet 6 ms und entfällt im Profil *Leistung*. Auf einer echten GPU sind diese
Passes bei 162 k internen Pixeln (536×302) bzw. 3 M Ausgabepixeln (2556×1179) sehr klein; **gemessen ist das nicht**.

### 2.4 Start und Speicher

| Größe | Wert | Anmerkung |
|---|---|---|
| Atlas | 4096×2048, 4 492 Sprites, **64 MiB** (Albedo 32 + Normalen 32 MiB, RGBA8) | zusätzlich eine CPU-Kopie im `AtlasBuilder`/`ArtLibrary` |
| 2D-Kunst (Gelände, Icons, Props) | 6 ms | |
| 3D-Bake (Einheiten, Gebäude, Bäume) | **≈ 4,8 s auf llvmpipe** | läuft auf der GPU; auf Apple-GPU erheblich schneller (nicht gemessen) |
| Atlas packen, Animationssätze | 50–75 ms | |
| Gesamtstart bis Menü (Init) | ≈ 5,0–5,3 s (llvmpipe) | dominiert vom 3D-Bake; ein Festplatten-Cache für gebackene Sprites (geplant, siehe `3d_pixel_pipeline.md`) würde ihn auf wenige 100 ms senken |
| Spitzen-Arbeitsspeicher des Prozesses | ≈ 1,7 GiB | enthält die Puffer des Software-Vulkan; auf Apple Silicon (Unified Memory) erwartet kleiner, ungemessen |

## 3. Behobene Engpässe

1. **String-Bau und Hash-Suche je Kachel und Frame** (`view.rs`): Für jede sichtbare Kachel, jeden Übergang, Schaumsaum und jedes
   Dekorationsobjekt entstand pro Frame ein `format!`-String samt Atlas-Hash-Suche (bis zu mehrere Tausend Allokationen je Frame).
   Jetzt löst `GroundRects` alle Rechtecke einmalig auf (Index nach Geländepriorität/Variante/Seite bzw. Dekorationsart/Variante).
   Ergebnis: `view` −49 % bis −62 % (Tabelle 2.1).
2. **Pro-Frame-Allokationen der Übergangs-Puffer**: `edge_buckets` (10 Vektoren) und `foam_buf` werden wiederverwendet statt
   jedes Frame neu angelegt.
3. Neue Bodendetails und Dekoration (Kiesel, Glut, Pfützen, Schilf) kosten dank (1) keinen Mehraufwand je Frame.

Bewusst **nicht** geändert (Messung zeigt keinen Bedarf): Sim-Spatial-Queries, Pfadfindung, Partikel-Limits (Profile begrenzen
sie bereits, 600/1 600/3 200), Instanz-Culling (der Renderer verwirft unsichtbare Instanzen bereits vor dem Hochladen).

## 4. Grafikprofile (`render/profile.rs`)

| Einstellung | Leistung (Low) | Ausgewogen (Medium, Standard) | Qualität (High) | Wirkung |
|---|---|---|---|---|
| Bloom-Stufen | 0 | 3 | 4 | Anzahl Downsample/Upsample-Passes; 0 = Bloom-Pass entfällt komplett (−5 Draw-Calls) |
| Max. Lichter | 48 | 128 | 256 | Obergrenze sichtbarer Punktlichter (hellste zuerst); Kosten ∝ Lichtfläche × Lichtanzahl |
| Max. Partikel | 600 | 1 600 | 3 200 | Obergrenze der Partikel (Rauch, Glut, Staub, Regen) |
| Normalmapping | aus | an | an | Atlas-Normalen im Lichtpass; aus = flach beleuchtet |
| Atmosphärischer Nebel | an | an | an | Boden-/Distanznebel in der Komposition |
| Filmkorn | 0,0 | 0,006 | 0,009 | Rauschstärke im Ausgabe-Pass |
| Mindest-Bildzeilen (Zoom-Grenze) | 300 | 260 | 220 | Kleinster Zoom, begrenzt die interne Auflösung nach unten (mehr Zeilen = mehr Pixel) |

Wasser-, Schaum- und Gelände-Animation sind in allen Profilen gleich (kein Profilschalter, Kosten im G-Buffer-Fragment-Shader
vergleichbar mit der früheren Fassung). Nicht implementiert: die im Kommentar von `profile.rs` genannte halbe Lichtauflösung.

## 5. Reproduktion

```bash
cargo build --release
./target/release/ashen_crowns --bench stress:200 --frames 300 --no-audio
./target/release/ashen_crowns --bench mission --profile performance
cargo run --release -p ashen_sim --example sim_bench -- 200 60
```

## 6. Offene Punkte

- **Messung auf Apple A18 Pro** (Metal, echte GPU-Zeiten je Pass via `--bench`, Speicherbedarf des Prozesses): ausstehend.
  Erwartung (Schätzung, nicht gemessen): CPU ≈ 1–2 ms je Frame, GPU deutlich unter 8 ms in allen Profilen.
- Festplatten-Cache für die gebackenen Sprites (Startzeit), Freigabe der CPU-Atlaskopie nach dem Hochladen (−64 MiB).
- Pfadverarbeitung bei > 800 gleichzeitig marschierenden Einheiten (Round-Robin statt Idreihenfolge, Pfad-Cache je Zielgebiet).
