#!/usr/bin/env python3
"""Erzeugt docs/audio/PRODUCTION_CHECKLIST.md aus der Dateiliste von `ashen_tools audio-list` und den Prompt-Dokumenten.

Aufruf (im Repository-Wurzelverzeichnis):

    python3 docs/audio/gen_production_checklist.py            # ruft `cargo run -p ashen_tools -- audio-list --format csv` selbst auf
    python3 docs/audio/gen_production_checklist.py --list liste.csv

Die Dateinamen kommen ausschließlich aus `audio-list` (Manifest ist maßgeblich), die Prompts und Sprechtexte aus den
Fach-Dokumenten `docs/audio/elevenlabs_*.md` und `suno_master_production.md`. Nach dem Erzeugen prüft der Test
`every_expected_file_is_in_production_checklist` (cargo test -p ashen_tools), dass jede erwartete Datei im Dokument steht.
Abgehakte Kästchen gehen bei erneutem Erzeugen verloren: nach Produktionsbeginn nicht mehr neu erzeugen.
"""
import csv
import io
import re
import subprocess
import sys
from collections import OrderedDict, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "docs" / "audio"
OUT = DOCS / "PRODUCTION_CHECKLIST.md"


# ---------------------------------------------------------------- Dokumente einlesen
def md_cell(c):
    return c.strip()


def path_key(cell):
    c = cell.strip()
    if len(c) > 2 and c[0] == "`" and c[-1] == "`":
        c = c[1:-1]
        if c.endswith(".ogg") and "…" not in c:
            return c[:-4]
    return None


class Row:
    def __init__(self, doc, line, cells, heading, h2):
        self.doc, self.line, self.cells, self.heading, self.h2 = doc, line, cells, heading, h2


class Docs:
    def __init__(self):
        self.rows = {}  # rel -> Row
        self.lines = {}  # Dokumentname -> Zeilen
        self.notes = defaultdict(list)  # (doc, heading_line) -> Textzeilen des Abschnitts
        for p in sorted(DOCS.glob("elevenlabs_*.md")):
            if "production_guide" not in p.name:
                self._parse(p)

    def _parse(self, path):
        name = path.name
        lines = path.read_text(encoding="utf-8").split("\n")
        self.lines[name] = lines
        heading = (None, -1)
        h2 = ""
        i = 0
        while i < len(lines):
            l = lines[i]
            if l.startswith("#"):
                heading = (l, i)
                if l.startswith("## ") or l.startswith("# "):
                    h2 = l
            elif l.startswith("|") and i + 1 < len(lines) and re.match(r"^\|[-| :]+\|\s*$", lines[i + 1]):
                head = [h.strip() for h in l.strip().strip("|").split("|")]
                i += 2
                while i < len(lines) and lines[i].startswith("|"):
                    cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
                    d = dict(zip(head, cells))
                    for c in cells[:3]:
                        k = path_key(c)
                        if k:
                            self.rows.setdefault(k, Row(name, i, d, heading, h2))
                            break
                    i += 1
                continue
            elif l.strip() and heading[1] >= 0 and not l.startswith("---"):
                self.notes[(name, heading[1])].append(l)
            i += 1

    def note(self, row, *prefixes):
        """Zeilen des Abschnitts einer Tabellenzeile, die mit einem der Präfixe beginnen."""
        out = []
        for l in self.notes.get((row.doc, row.heading[1]), []):
            if any(l.startswith(p) for p in prefixes):
                out.append(l)
        return out


def read(path):
    return path.read_text(encoding="utf-8").split("\n")


def block_from(lines, start, level_hashes=None):
    level = len(lines[start]) - len(lines[start].lstrip("#"))
    out = []
    for j in range(start + 1, len(lines)):
        l = lines[j]
        if l.startswith("#"):
            lv = len(l) - len(l.lstrip("#"))
            if lv <= level:
                break
        if l.strip() == "---":
            break
        out.append(l)
    while out and not out[-1].strip():
        out.pop()
    while out and not out[0].strip():
        out.pop(0)
    return out


def dat(n):
    return f"{n} Datei" if n == 1 else f"{n} Dateien"


def strip_md(s):
    return s.replace("**", "").strip()


def get(row, *keys, default=""):
    for k in keys:
        if k in row.cells and row.cells[k]:
            return row.cells[k]
    return default


# ---------------------------------------------------------------- Liste (audio-list)
def load_list(argv):
    if "--list" in argv:
        text = Path(argv[argv.index("--list") + 1]).read_text(encoding="utf-8")
    else:
        text = subprocess.run(
            ["cargo", "run", "-q", "-p", "ashen_tools", "--", "audio-list", "--format", "csv"],
            cwd=ROOT, check=True, capture_output=True, text=True,
        ).stdout
    entries = list(csv.DictReader(io.StringIO(text)))
    for e in entries:
        e["rel"] = e["folder"] + "/" + e["file"][: -len(".ogg")]
        e["path"] = e["folder"] + "/" + e["file"]
    return entries


def speaker_of(e):
    i = e["id"]
    if i.startswith("voice."):
        return i.split(".")[1]
    return i.split(".")[0]


# ---------------------------------------------------------------- Stimmprofile
SPEAKERS = OrderedDict()  # Schlüssel -> (Gruppe, Anzeigename)


def add(group, entries):
    for k, name in entries:
        SPEAKERS[k] = (group, name)


add("B1 Erzähler", [("narrator", "Erzähler (Chronist)")])
add("B2 Helden", [("hero_isolde", "Isolde Raventhorn (Heldin)"), ("char_hal_venn", "Bruder Hal Venn (Figur und Held `hero_hal_venn`)"), ("char_torvald", "Torvald Eisenhand (Figur und Held `hero_torvald`)")])
add("B3 Figuren", [("char_king_aldric", "König Aldric III."), ("char_mireille", "Herzogin Mireille Corvane"), ("char_sylaen", "Sylaen der Veyl"), ("char_ophira", "Ophira Saelith-Varo"), ("char_kassian", "Kassian Vale"), ("char_deep_voice", "Die Stimme aus dem Tief"), ("char_wendel", "Ser Wendel der Graue"), ("char_nel", "Nel")])
add("B4 Bosse", [("boss_pale_reeve", "Der Fahle Vogt"), ("boss_hollow_king", "Der Hohle König"), ("boss_root_mother", "Die Wurzelmutter"), ("boss_stone_oath", "Gharun, der Steinerne Eid"), ("boss_reef_mistress", "Isarre Mol, Herrin des Schwarzen Riffs")])
add("B5 Einheiten der Glutkrone", [(k, n) for k, n in [("worker", "Arbeiter"), ("swordsman", "Schwertkämpfer"), ("spearman", "Speerträger"), ("archer", "Bogenschützin"), ("knight", "Ritter"), ("crossbowman", "Armbrustschütze"), ("halberdier", "Hellebardier"), ("shieldbearer", "Schildträger"), ("scout_rider", "Spähreiter"), ("horse_archer", "Berittene Bogenschützin"), ("battle_mage", "Kampfmagier"), ("priest", "Priester der Flamme"), ("battering_ram", "Rammbock-Mannschaft"), ("catapult", "Katapult-Mannschaft")]])
add("B6 Die Hohlen", [("hollow_thrall", "Hohlknecht"), ("hollow_archer", "Knochenschütze"), ("grave_brute", "Grabkoloss"), ("hollow_digger", "Grabgräber"), ("ghoul", "Ghul"), ("ash_witch", "Aschhexe")])

MERGE = {"hero_hal_venn": "char_hal_venn", "hero_torvald": "char_torvald"}
VOICE_NAME = {"battering_ram": "AC_unit_ram_crew", "catapult": "AC_unit_catapult_crew"}
UNITS = {k for k, (g, _) in SPEAKERS.items() if g.startswith(("B5", "B6"))}


def voice_name(k):
    if k in VOICE_NAME:
        return VOICE_NAME[k]
    return f"AC_unit_{k}" if k in UNITS else f"AC_{k}"


def profile_block(chars, key):
    cands = [f"`{key}`", f"`{voice_name(key)}`"]
    if key in VOICE_NAME:
        cands.append(f"`{VOICE_NAME[key]}`")
    for i, l in enumerate(chars):
        if l.startswith("#") and any(c in l for c in cands):
            return block_from(chars, i)
    return None


def header_lines(docs, key):
    """`**Stimme:**`-, `**Haltung:**`- und `**Regie allgemein:**`-Zeilen der Abschnitte, die den Sprecher betreffen."""
    keys = {key} | {h for h, m in MERGE.items() if m == key}
    out = []
    for name, lines in docs.lines.items():
        if name in ("elevenlabs_characters.md", "elevenlabs_production_guide.md", "elevenlabs_sound_effects.md", "elevenlabs_ambient_audio.md"):
            continue
        for i, l in enumerate(lines):
            if l.startswith("#") and any(f"`{k}`" in l for k in keys) and "Event" not in l and "AC_" not in l:
                j = i + 1
                chunk = []
                while j < len(lines) and not lines[j].startswith("#") and not lines[j].startswith("|"):
                    if lines[j].startswith(("**Stimme:**", "**Haltung:**", "**Regie allgemein:**")):
                        # Folgezeilen eines mehrzeiligen Absatzes mitnehmen
                        t = lines[j]
                        k = j + 1
                        while k < len(lines) and lines[k].strip() and not lines[k].startswith(("**", "#", "|", "*Kontext")):
                            t += " " + lines[k].strip()
                            k += 1
                        chunk.append(t)
                    j += 1
                    if len(chunk) >= 3:
                        break
                if chunk:
                    out.extend(c for c in chunk if c not in out)
    return out


# ---------------------------------------------------------------- Ausgabe
def entry_sfx(e, row, out):
    bat = e["id"].startswith("voice.bat_swarm")
    dur = get(row, "Dauer", "Dauer (gen. → final)", default=e["hint"].replace("Dauer ", "") or ("0,6–2,0 s" if bat else ""))
    pi = get(row, "PI", default="0,5 (Startwert)" if bat else "")
    var = get(row, "Variationshinweis", "Regie")
    meta = [f"Dauer {dur}" if dur else "", f"Prompt Influence {pi}" if pi else "", f"Variante: {var}" if var else ""]
    out.append(f"- [ ] `{e['path']}` · " + " · ".join(m for m in meta if m))
    out.append(f"  > {get(row, 'EN-Prompt', 'EN-Text/Prompt', 'EN-Text')}")


def event_heading(row):
    h = row.heading[0] or ""
    m = re.match(r"^#+\s*(.*?)\s*\(Event `([^`]+)`\)\s*(.*)$", h)
    if m:
        name, ev, rest = m.groups()
        rest = rest.strip().lstrip("–·").strip()
        return name, ev, rest
    return strip_md(h.lstrip("# ")), "", ""


def section_sfx(entries, docs, out):
    sfx = [e for e in entries if e["category"] in ("sfx", "ui") or e["id"].startswith("voice.bat_swarm")]
    folders = OrderedDict()
    for e in sfx:
        key = e["folder"] if not e["id"].startswith("voice.bat_swarm") else "BAT"
        folders.setdefault(key, []).append(e)
    # Reihenfolge: combat, magic, buildings, ui, world, danach Rest
    order = ["sfx/combat", "sfx/magic", "sfx/buildings", "sfx/ui", "sfx/world"]
    keys = [k for k in order if k in folders] + [k for k in folders if k not in order and k != "BAT"] + (["BAT"] if "BAT" in folders else [])
    n = 0
    for fk in keys:
        items = folders[fk]
        label = "Fledermausschwarm – Voice-Events `voices/units/bat_swarm_*`, aber mit dem Sound-Effects-Generator erzeugen" if fk == "BAT" else f"`{fk}/`"
        out.append(f"### A.{n + 1} {label} – {dat(len(items))}")
        out.append("")
        n += 1
        byev = OrderedDict()
        for e in items:
            byev.setdefault(e["id"], []).append(e)
        for ev, es in byev.items():
            row = docs.rows[es[0]["rel"]]
            name, evid, rest = event_heading(row)
            out.append(f"#### {name} – Event `{ev}` ({dat(len(es))})")
            if rest:
                out.append(f"*{strip_md(rest)}*")
            out.append("")
            for e in es:
                entry_sfx(e, docs.rows[e["rel"]], out)
            notes = docs.note(row, "*Hinweis", "**Nachbearbeitung", "*Nachbearbeitung")
            for nline in notes:
                out.append(f"- {strip_md(nline)}")
            out.append("")
    return len(sfx)


def section_voices(entries, docs, chars, out):
    voices = [e for e in entries if e["category"] in ("voice", "narration", "dialogue") and not e["id"].startswith("voice.bat_swarm")]
    by_speaker = defaultdict(list)
    for e in voices:
        k = speaker_of(e)
        k = MERGE.get(k, k)
        by_speaker[k].append(e)
    unknown = [k for k in by_speaker if k not in SPEAKERS]
    if unknown:
        sys.exit(f"Sprecher ohne Gruppe im Generator: {unknown}")
    cur_group = None
    count = 0
    for k, (group, disp) in SPEAKERS.items():
        es = by_speaker.get(k)
        if not es:
            continue
        if group != cur_group:
            out.append(f"### {group}")
            out.append("")
            cur_group = group
        count += len(es)
        out.append(f"#### {disp} – Stimme `{voice_name(k)}` ({dat(len(es))})")
        out.append("")
        out.append("**Stimmprofil / Voice Design (einmal pro Sprecher anlegen, danach alle Zeilen unten mit dieser Stimme erzeugen):**")
        out.append("")
        prof = profile_block(chars, k)
        if prof is None:
            sys.exit(f"Kein Stimmprofil für {k}")
        out.extend(("> " + p if p.strip() else ">") for p in prof)
        out.append("")
        for h in header_lines(docs, k):
            out.append(f"- {strip_md(h)}")
        out.append("")
        # Zeilen in Dokumentreihenfolge
        def keyf(e):
            r = docs.rows[e["rel"]]
            return (0 if r.doc == "elevenlabs_unit_voices.md" else 1, r.doc, r.line)
        es.sort(key=keyf)
        last_heading = None
        for e in es:
            r = docs.rows[e["rel"]]
            hkey = (r.doc, r.heading[1])
            if hkey != last_heading:
                last_heading = hkey
                name, ev, rest = event_heading(r)
                if ev:
                    title = f"{name} (Event `{ev}`)"
                else:
                    title = f"{name} · {strip_md(r.h2.lstrip('# '))}"
                out.append(f"##### {strip_md(title)}")
                for kl in docs.note(r, "*Kontext"):
                    out.append(strip_md(kl).replace("*Kontext:*", "Kontext:"))
                out.append("")
            text_de = get(r, "Sprechtext (DE, exakt wie im Spiel)", "Sprechtext (DE)")
            exact = "Sprechtext (DE, exakt wie im Spiel)" in r.cells
            regie = get(r, "Regie")
            num = get(r, "#")
            meta = ["Regie: " + regie if regie else "", f"Zeile {num} der Szene" if num else ""]
            out.append(f"- [ ] `{e['path']}` · " + " · ".join(m for m in meta if m))
            if text_de:
                out.append(f"  > {text_de}")
                out.append("  > *(Sprache: Deutsch, exakter Spieltext)*" if exact else "  > *(Sprache: Deutsch)*")
            else:
                en = get(r, "EN-Text", "EN-Text/Prompt", "EN-Prompt")
                out.append(f"  > {en}")
            out.append("")
    return count


def section_ambience(entries, docs, out):
    lines = docs.lines["elevenlabs_ambient_audio.md"]
    amb = [e for e in entries if e["category"] == "ambience"]
    amb.sort(key=lambda e: next(i for i, l in enumerate(lines) if l.startswith("#") and f"`{e['path']}`" in l))
    for e in amb:
        idx = next(i for i, l in enumerate(lines) if l.startswith("#") and f"`{e['path']}`" in l)
        heading = strip_md(lines[idx].lstrip("# "))
        out.append(f"### {heading}")
        out.append("")
        out.append(f"- [ ] `{e['path']}` · {e['hint']} (fertiger Loop, zusammengesetzt aus den Schichten unten)")
        out.append("")
        out.extend(block_from(lines, idx))
        out.append("")
    return len(amb)


def section_music(entries, out):
    mus = [e for e in entries if e["category"] in ("music", "music_intro")]
    suno = read(DOCS / "suno_master_production.md")
    by_track = OrderedDict()
    for e in mus:
        by_track.setdefault(e["id"], []).append(e)
    # Reihenfolge der Track-Bibel
    pos = {}
    for i, l in enumerate(suno):
        m = re.match(r"^####\s+\S+\s+`([a-z0-9_]+)`", l)
        if m and m.group(1) in by_track and m.group(1) not in pos:
            pos[m.group(1)] = i
    missing = [t for t in by_track if t not in pos]
    if missing:
        sys.exit(f"Track ohne Suno-Abschnitt: {missing}")
    for n, t in enumerate(sorted(by_track, key=lambda x: pos[x]), 1):
        es = sorted(by_track[t], key=lambda e: (e["category"] != "music_intro", e["path"]))
        idx = pos[t]
        title = re.sub(r"^\S+\s+", "", strip_md(suno[idx].lstrip("# ")))
        out.append(f"### D.{n} {title}")
        out.append("")
        intro = [e for e in es if e["category"] == "music_intro"]
        main = [e for e in es if e["category"] == "music"][0]
        for e in es:
            out.append(f"- [ ] `{e['path']}` · {e['hint']}")
        if intro:
            out.append("- Schnitt: Intro und Loop stammen aus **demselben** Take. Intro = Take-Anfang bis exakt zum Loop-Start (kein Fade-out, keine Stille); Loop = ganze Taktzahl, Anfang und Ende auf Taktgrenzen, Naht per 20–50 ms Crossfade, Tail-Wrap (siehe Kopf, Schnittregeln).")
        elif main["hint"].startswith("Loop"):
            out.append("- Schnitt: **kein Intro**; der Loop muss auch als Einstieg funktionieren (weicher erster Takt). Ganze Taktzahl, Anfang und Ende auf Taktgrenzen, Naht per 20–50 ms Crossfade, Tail-Wrap.")
        else:
            out.append("- Schnitt: **einmaliges Stück/Sting, kein Loop** – sauberer Anfang (5 ms Fade-in), natürlicher Schluss mit Ausklang (kein abruptes Abschneiden).")
        out.append("")
        out.extend(block_from(suno, idx))
        out.append("")
    return len(mus)


# ---------------------------------------------------------------- Kopf und Schluss
HEAD = """# ASHEN CROWNS – Audio-Produktionsliste (Master-Checkliste)

Alles, was für das Spiel noch produziert werden muss: **{total} Dateien** in vier Abschnitten. Jeder Eintrag enthält Zieldatei (Ordner + exakter
Name), vollständigen Prompt bzw. Sprechtext und die Einstellungen, damit direkt am Werkzeug gearbeitet werden kann. Die Dateinamen sind
aus dem Manifest (`assets/audio/manifest.ron`) per `ashen_tools audio-list` erzeugt; die Prompts stammen aus den Fach-Dokumenten
(`docs/audio/elevenlabs_*.md`, `suno_master_production.md`). Bei Abweichungen ist das **Manifest maßgeblich**.

> Diese Datei ist erzeugt (`python3 docs/audio/gen_production_checklist.py`). Kästchen darf man abhaken, solange man nicht neu erzeugt.

## Statistik

| Abschnitt | Werkzeug | Dateien |
|---|---|---|
| A – Soundeffekte | ElevenLabs Sound Effects | {a} |
| B – Stimmen (Erzähler, Figuren, Bosse, Einheiten, Helden) | ElevenLabs Text to Speech / Voice Design | {b} |
| C – Ambience-Loops | ElevenLabs Sound Effects (mehrschichtig) | {c} |
| D – Musik (Loops, Intros, Stings) | Suno | {d} |
| **Gesamt** | | **{total}** |

Aufteilung B: {b_detail}. Aufteilung D: {d_detail}.

## Ablagestruktur (`assets/audio/`)

```text
assets/audio/
├── music/               Musik: <track>_loop.ogg, <track>_intro.ogg, Stings/Stücke ohne Loop
├── ambience/            Umgebungs-Loops: ambience_<id>_loop.ogg
├── sfx/
│   ├── combat/ magic/ buildings/ ui/ world/     Soundeffekte: <sfx_id>_NN.ogg
└── voices/
    ├── units/           Einheiten- und Boss-Barks: <einheit>_<situation>_NN.ogg   (inkl. Fledermausschwarm aus Abschnitt A)
    ├── heroes/          Helden-Barks: hero_<id>_<situation>_NN.ogg
    ├── narrator/        Erzähler: narrator_<mission>_<szene>_NN.ogg
    └── characters/      Dialogzeilen aller Sprecher: <sprecher>_<mission>_<szene>_NN.ogg
```

## Formate, Benennung, Varianten

- **Zielformat:** `.ogg` (Vorbis q6, 48 kHz). Abspielbar sind auch `.wav`, `.flac`, `.mp3`; `.m4a`, `.aac`, `.aiff`, `.aif`, `.opus`,
  `.webm`, `.wma`, `.caf` werden bei Bedarf per `ffmpeg` konvertiert (`--convert`). Am besten in der höchsten verfügbaren Qualität
  (WAV/PCM) herunterladen und liefern; Konvertierung erledigt der Scanner.
- **Dateinamen:** nur Kleinbuchstaben, Ziffern, Unterstriche. Exakt wie in dieser Liste (Name ohne Endung). Groß-/Leerzeichen/Bindestriche
  werden mit `--apply-renames` korrigiert, aber **Wortlaut und Nummer müssen stimmen**.
- **Varianten:** zweistellig ab `01`, lückenlos (`worker_select_01`, `_02`, …). Die Liste enthält jede Variante einzeln mit eigenem Prompt
  bzw. Text. Zusätzliche Takes **nicht** als weitere Varianten nummerieren; die beste Variante pro Eintrag wählen.
- **Mehrteilige Zeilen** (Erzähler/Dialog `_01`, `_02`, …) werden vom Spiel in Nummernreihenfolge hintereinander abgespielt (Sequenz).
  Jede Datei endet mit höchstens 300 ms Stille. **Tipp für natürlichen Sprachfluss:** die komplette Zeile (alle Teilstücke
  zusammen) in einem Take erzeugen und anschließend an den Teilgrenzen schneiden, statt jedes Teilstück einzeln zu generieren.

## Lautheitsziele (ITU-R BS.1770, True Peak überall ≤ −1 dBTP)

| Kategorie | Ziel | Kanäle |
|---|---|---|
| Dialog / Charaktere / Erzähler (Abschnitt B, Teil Figuren/Erzähler) | −16 LUFS integriert (±1 LU) | Mono |
| Einheiten-/Helden-/Boss-Barks | −16 LUFS Short-Term-Max (±1,5 LU); Hohle/Kreaturen −17 LUFS | Mono |
| Kampf-SFX | −14 bis −18 LUFS Short-Term-Max | Mono |
| Magie-SFX | −14 bis −16 LUFS Short-Term-Max | Mono (Stereo, wenn nicht positioniert) |
| Gebäude-SFX | −16 bis −20 LUFS Short-Term-Max | Mono |
| UI-SFX | −20 bis −24 LUFS Short-Term-Max, Peak ≤ −6 dBFS | Mono oder Stereo |
| Welt-SFX (Arbeit, Wetter) | −18 bis −22 LUFS Short-Term-Max (`thunder` stereo) | Mono |
| Ambience-Loops (Abschnitt C) | −23 LUFS integriert (±1 LU) | Stereo |
| Musik (Abschnitt D) | −14 LUFS integriert (±1 LU); Stings Short-Term-Max ca. −12 LUFS | Stereo |

Kurze Dateien (< 3 s): Varianten einer Familie gehörmäßig gleich laut machen (Referenz Stimmen `hero_isolde_select_01`, Kampf-SFX `sword_hit_01`).

## Schnittregeln

1. **Stille trimmen:** Kopfstille Stimmen ≤ 20 ms (SFX-Impacts ≤ 5 ms), Schwanzstille ≤ 150 ms (Dialog bis 300 ms); Fade-in 5 ms, Fade-out 20–40 ms. Ausnahme Loops: kein Fade.
2. **Loops nahtlos:** Ambience-Loops und Musik-Loops auf Nulldurchgang bzw. Taktgrenze schneiden, Ende → Anfang ohne Klick, Lautheitssprung oder Hall-Loch (5 Durchläufe probehören). Ambience: Wrap-Around-Crossfade der Überhänge, kein Fade an Anfang/Ende. Musik: Loop-Länge = ganze Takte, Tail-Wrap, 20–50 ms Naht-Crossfade.
3. **Intro separat:** Das Intro ist eine eigene Datei (`*_intro.ogg`), stammt aus demselben Take wie der Loop, endet **ohne Fade** und stille exakt am Loop-Start. Tracks ohne Intro-Eintrag bekommen keins.
4. **Keine Hallfahnen** bei Einheiten-, Helden-Barks und positionierten SFX (die Engine räumt); Ausnahmen stehen am Eintrag.
5. **Artefakte** (metallisches Flattern, Zirpen, Pseudo-Musik, Wortfetzen im SFX, Gesang in Instrumentals) → Take verwerfen, neu generieren.
6. **Mono/Stereo** wie in der Lautheitstabelle; Mono-Summe auf Phasenauslöschung prüfen.

## Lieferweise

- Fertige Dateien **gesammelt in einen Ordner** legen (z. B. `lieferung/`); Unterordner sind erlaubt (rekursiv durchsucht). Die Zuordnung zum Zielordner
  erledigt `audio-scan --import <Lieferordner>` allein anhand des Dateinamens (siehe Kurzanleitung am Ende).
- Alternativ direkt in die Zielordner unter `assets/audio/` kopieren (Pfad wie bei jedem Eintrag angegeben).
- Mit der Lieferung ein Take-Log (Tool, Modell, Seed/Einstellungen, verworfene Takes) mitgeben – nicht in `assets/audio/`.
"""

TAIL = """## Kurzanleitung: Dateien ins Spiel bringen

1. Lieferung in einen Ordner legen, z. B. `lieferung/` (gemischt, auch verschiedene Formate).
2. Zuordnen, normalisieren, konvertieren und Bericht schreiben:

   ```text
   cargo run -p ashen_tools -- audio-scan --import lieferung --apply-renames --convert --report assets/audio/audio_report.md
   ```

   `--import` kopiert jede Datei anhand ihres Namens in den richtigen Ordner unter `assets/audio/` (bereits liegende Dateien werden nie überschrieben).
   Liegen die Dateien schon in den Zielordnern, genügt `audio-scan --apply-renames --convert --report`.
3. Bericht `assets/audio/audio_report.md` lesen: *Fehlende Dateien*, *Lücken in Variantennummern*, *Unbekannte Dateien*, *Beschädigte Dateien*, *Duplikate*
   abarbeiten und erneut scannen. Ziel: keine ungültigen Namen, Duplikate, Mehrdeutigkeiten, beschädigten Dateien.
4. Originale konvertierter Dateien (`.m4a` usw.) aus `assets/audio/` entfernen, sobald die `.ogg` existiert.
5. Hörtest im Spiel (`RUST_LOG=debug cargo run --release`): Logzeile `Audio: <n> Dateien zugeordnet, <m> Muster ohne Datei`, Details in
   [`audio_integration.md`](audio_integration.md), Abschnitt 8.
6. Liste jederzeit prüfen/neu ausgeben: `cargo run -p ashen_tools -- audio-list --format csv --out audio_liste.csv`.
"""


def main():
    entries = load_list(sys.argv)
    docs = Docs()
    chars = read(DOCS / "elevenlabs_characters.md")
    a_lines, b_lines, c_lines, d_lines = [], [], [], []
    na = section_sfx(entries, docs, a_lines)
    nb = section_voices(entries, docs, chars, b_lines)
    nc = section_ambience(entries, docs, c_lines)
    nd = section_music(entries, d_lines)
    total = len(entries)
    if na + nb + nc + nd != total:
        sys.exit(f"Summe {na}+{nb}+{nc}+{nd} != {total}")
    cat = defaultdict(int)
    for e in entries:
        if e["category"] in ("voice", "narration", "dialogue") and not e["id"].startswith("voice.bat_swarm"):
            cat[e["category"]] += 1
    b_detail = f"Erzähler {cat['narration']}, Dialogzeilen {cat['dialogue']}, Einheiten-/Helden-/Boss-Barks {cat['voice']}"
    mus = defaultdict(int)
    for e in entries:
        if e["category"] in ("music", "music_intro"):
            mus[e["category"]] += 1
    d_detail = f"Loops/Stücke {mus['music']}, Intros {mus['music_intro']}"
    out = [HEAD.format(total=total, a=na, b=nb, c=nc, d=nd, b_detail=b_detail, d_detail=d_detail)]
    out.append(f"---\n\n## A. ElevenLabs Sound Effects ({na} Dateien)\n")
    out.append("**Werkzeug:** ElevenLabs → Sound Effects. Prompt (englisch) einfügen, **Dauer manuell** auf den angegebenen Wert setzen, **Prompt Influence** wie angegeben (Startwert; höher = wörtlicher), Loop aus (außer ausdrücklich Loop). Je Eintrag mehrere Generierungen erzeugen und die beste wählen. Zusatz „no music“ anhängen, falls der Generator Melodien einstreut. Kurze Dateien (< Mindestdauer des Werkzeugs) länger generieren und kürzen.\n")
    out.extend(a_lines)
    out.append(f"---\n\n## B. ElevenLabs Stimmen ({nb} Dateien)\n")
    out.append("**Vorgehen:** Pro Sprecher einmal die Stimme anlegen (Voice Design: Prompt aus dem Stimmprofil in *Voice Description*, Beispieltext in *Text to preview*, speichern als `AC_<id>`), dann alle Zeilen des Sprechers mit dieser Stimme erzeugen. Sprechtext **exakt** wie angegeben einfügen; eckige Klammern (`[sighs]`, `[shouting]`) sind Eleven-v3-Audio-Tags und werden nicht gesprochen. Alle Stimmen werden auf **Deutsch** gesprochen (Erzähler, Dialoge, Helden-, Boss- und Einheitenrufe): in der Oberfläche/API die Sprache explizit auf Deutsch (`de`) stellen, Modell Eleven v3 bzw. Multilingual v2. Die Voice-Design-Prompts sind englisch formuliert und enthalten „speaks German with a natural, native accent“; die Audio-Tags in eckigen Klammern bleiben englisch. Zeilen mit dem Vermerk „exakter Spieltext“ entsprechen wortgleich dem Untertitel im Spiel (Prüfung: `python3 docs/audio/verify_spoken_text.py`); bei Sequenz-Events sind die Dateien `_01`, `_02`, … wörtliche Teilstücke der Spielzeile. Kanon-Eigennamen in deutscher Aussprache über das Aussprache-Wörterbuch (Leitfaden, Abschnitt 6).")
    out.append("")
    out.append("**Einstellungs-Startwerte** (Feinabstimmung je Sprecher im Profil): Eleven v3 – Stability-Modus *Natural* für Barks, Helden und Dialog, *Creative* für Kampf-/Schmerzlaute und Hohle, *Robust* für konsistenzkritische Bosse (v3 kennt kein Similarity/Style; Speed 0,92–1,08 laut Profil). Multilingual v2 (Erzähler): Stability 0,60, Similarity 0,80, Style 0,15, Speaker Boost an, Speed 0,95; ruhige Monologe 0,45–0,55 / 0,75–0,80 / 0,15–0,30; bedrohliche Stimmen 0,60–0,75 / 0,80–0,90 / 0,05–0,15. Style über 0,4 erhöht das Artefaktrisiko. Nachbearbeitung: Standardkette Stimme (Hochpass 80–100 Hz, De-Esser, Kompressor 3:1, Limiter −1 dBTP); Hohle/Bosse/Deep Voice zusätzlich Spezialketten laut Leitfaden Abschnitt 8.3.\n")
    out.extend(b_lines)
    out.append(f"---\n\n## C. ElevenLabs Ambience ({nc} Dateien)\n")
    out.append("**Werkzeug:** ElevenLabs → Sound Effects mit Loop-Option, mehrschichtig (Bett, Mittelgrund, Details) im DAW zusammensetzen. Fertige Loops 90 s (siehe Eintrag), **stereo**, −23 LUFS, **kein Fade an Anfang/Ende**. Prompts enthalten „no music, no voices“; Ausnahme `ambience_village` (unverständliches, entferntes Murmeln). Je Schicht Prompt Influence 0,3–0,45 (Flächen) bzw. 0,5–0,6 (Details).\n")
    out.append("**Schichtenaufbau:** A Bett (durchgehende Fläche, 0 dB Referenz) · B Mittelgrund (−4 bis −8 dB) · C Details (Einzelereignisse, −6 bis −12 dB, unregelmäßig platziert). Schichten mit unterschiedlicher Periodenlänge anlegen, Überhänge (3–4 s) per Equal-Power-Crossfade an den Anfang wrappen, Details mindestens 3 s von der Naht entfernt halten, Bounce auf Nulldurchgang, 5 Minuten Probe-Loop, Lautheit per statischem Gain.\n")
    out.extend(c_lines)
    out.append(f"---\n\n## D. Suno Musik ({nd} Dateien)\n")
    out.append("**Werkzeug:** Suno, **Custom-Modus** (Style und Lyrics getrennt). Style-Prompt (englisch, ≤ ca. 200 Zeichen) in das Style-Feld, Lyrics-/Struktur-Feld wie angegeben (Struktur-Tags; bei Instrumentals Weg A: Instrumental-Schalter an, oder Weg B: aus mit reinen Struktur-Tags ohne singbaren Text; Chor-Tracks: Schalter aus, Old-Aurethi-Text). Ausschluss-Feld, falls vorhanden: „pop vocals, rap, edm, electric guitar, autotune, trap drums“. Loop-Rohtakes 2:30–4:00 anstreben, Intro und Loop immer aus demselben Take, Tempo auf die angegebene BPM fixieren. Export als WAV (48 kHz, 24 Bit) archivieren, Spieldatei `.ogg` Vorbis q6, 48 kHz, Stereo, −14 LUFS integriert, True Peak ≤ −1 dBTP; Intro und Loop höchstens 1 LU Unterschied. Keine Künstlernamen in Prompts.\n")
    out.extend(d_lines)
    out.append("---\n")
    out.append(TAIL)
    text = "\n".join(out)
    text = re.sub(r"\n{3,}", "\n\n", text)
    OUT.write_text(text.rstrip("\n") + "\n", encoding="utf-8")
    print(f"{OUT.relative_to(ROOT)}: A {na}, B {nb}, C {nc}, D {nd}, gesamt {total}, {text.count(chr(10))} Zeilen")


if __name__ == "__main__":
    main()
