#!/usr/bin/env python3
"""Prüft, dass der Sprechtext in docs/audio/elevenlabs_dialogues.md exakt dem Spieltext entspricht.

Quelle Spieltext: jedes `<feld>: "...", voice: Some("<event>")` (Message, Dialogue, Decision: text; Zeitalter: blurb/Epilog) in assets/data/missions/*.ron und
assets/data/campaign/*.ron. Quelle Sprechtext: Tabellenzeilen in `elevenlabs_dialogues.md` mit der Spalte
`Sprechtext (DE, exakt wie im Spiel)`. Eleven-v3-Audio-Tags in eckigen Klammern (`[whispers]`) werden vor dem Vergleich entfernt.

Regeln je Event (Playback aus assets/audio/manifest.ron):
  * `Sequential`: die Dateien `<event>_01`, `_02`, … ergeben, in Reihenfolge mit Leerzeichen verbunden, den Spieltext.
  * sonst (Zufallsvariante): jede Datei spricht den kompletten Spieltext.

Aufruf (Repository-Wurzel):  python3 docs/audio/verify_spoken_text.py
Rückgabewert 0 = alles stimmt, 1 = Abweichungen.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs" / "audio" / "elevenlabs_dialogues.md"
TAG = re.compile(r"\[[a-z ]+\]")
MSG = re.compile(r'[a-z_]+:\s*"((?:[^"\\]|\\.)*)",\s*(?:requires:\s*\[[^\]]*\],\s*)?voice:\s*Some\("([^"]+)"\)')


def norm(s):
    s = TAG.sub("", s)
    return re.sub(r"\s+", " ", s).strip()


def unescape(s):
    return s.replace('\\"', '"').replace("\\n", " ").replace("\\\\", "\\")


def game_texts():
    out = {}  # event -> set(texte)
    for sub in ("missions", "campaign"):
        for p in sorted((ROOT / "assets" / "data" / sub).glob("*.ron")):
            for m in MSG.finditer(p.read_text(encoding="utf-8")):
                out.setdefault(m.group(2), {}).setdefault(norm(unescape(m.group(1))), p.name)
    return out


def sequential_events():
    text = (ROOT / "assets" / "audio" / "manifest.ron").read_text(encoding="utf-8")
    seq = set()
    for m in re.finditer(r'SoundEvent\(id:\s*"([^"]+)"[^\n]*', text):
        if "playback: Sequential" in m.group(0):
            seq.add(m.group(1))
    return seq


def spoken_lines():
    """Dateistamm -> Sprechtext (aus den Tabellen mit Spalte 'Sprechtext (DE, exakt wie im Spiel)')."""
    out = {}
    head = None
    for l in DOC.read_text(encoding="utf-8").split("\n"):
        if l.startswith("|"):
            cells = [c.strip() for c in l.strip().strip("|").split("|")]
            if any(c.startswith("Sprechtext (DE") for c in cells):
                head = cells
                continue
            if head is None or re.match(r"^[-: ]+$", cells[0]):
                continue
            row = dict(zip(head, cells))
            f = row.get("Ziel-Datei", "")
            if f.startswith("`") and f.endswith(".ogg`"):
                out[f.strip("`").split("/")[-1][:-4]] = row.get("Sprechtext (DE, exakt wie im Spiel)", "")
        else:
            head = None
    return out


def main():
    texts = game_texts()
    seq = sequential_events()
    spoken = spoken_lines()
    errors = []
    checked_events = 0
    checked_lines = 0
    for event, variants in sorted(texts.items()):
        if len(variants) > 1:
            errors.append(f"{event}: mehrere verschiedene Spieltexte in {sorted(set(variants.values()))}")
            continue
        game = next(iter(variants))
        prefix = event.replace(".", "_")
        stems = sorted(s for s in spoken if re.fullmatch(re.escape(prefix) + r"_\d+", s))
        if not stems:
            errors.append(f"{event}: keine Sprechzeilen im Dokument")
            continue
        checked_events += 1
        if event in seq:
            joined = norm(" ".join(spoken[s] for s in stems))
            checked_lines += len(stems)
            if joined != game:
                errors.append(f"{event} (Sequenz): Sprechtext weicht ab\n    Spiel : {game}\n    Sprech: {joined}")
        else:
            for s in stems:
                checked_lines += 1
                if norm(spoken[s]) != game:
                    errors.append(f"{s} (Variante): Sprechtext weicht ab\n    Spiel : {game}\n    Sprech: {norm(spoken[s])}")
    # Sprechzeilen ohne Spieltext (z. B. Zusatzsegmente) nur melden
    covered = {s for e in texts for s in spoken if re.fullmatch(re.escape(e.replace('.', '_')) + r"_\d+", s)}
    extra = sorted(set(spoken) - covered)
    print(f"Events mit Spieltext: {len(texts)}, geprüft: {checked_events}, Sprechzeilen (Dateien) verglichen: {checked_lines}")
    print(f"Sprechzeilen ohne Spieltext-Event (nicht geprüft): {len(extra)}" + (f" – {', '.join(extra)}" if extra else ""))
    if errors:
        print(f"FEHLER ({len(errors)}):")
        for e in errors:
            print(" -", e)
        return 1
    print("OK: Sprechtext == Spieltext")
    return 0


if __name__ == "__main__":
    sys.exit(main())
