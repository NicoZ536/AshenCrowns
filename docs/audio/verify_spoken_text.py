#!/usr/bin/env python3
"""Prüft, dass der Sprechtext in docs/audio/elevenlabs_dialogues.md ohne Audio-Tags exakt dem Spieltext entspricht.

Quelle Spieltext: jedes `<feld>: "...", voice: Some("<event>")` (Message, Dialogue, Decision: text; Zeitalter: blurb/Epilog) in assets/data/missions/*.ron und
assets/data/campaign/*.ron. Quelle Sprechtext: Tabellenzeilen in `elevenlabs_dialogues.md` mit der Spalte `Sprechtext (DE, exakt wie im Spiel)`.
Eleven-v3-Audio-Tags in eckigen Klammern (`[whispers]`, `[short pause]`) werden vor dem Vergleich entfernt.

Regeln (eine Zeile = eine Datei):
  * Jedes Event mit Spieltext hat genau eine Sprechzeile `<event>_01` (ganze Zeile, keine Teilstücke).
  * Der Sprechtext ohne Tags ist gleich dem Spieltext.
  * Jede Sprechzeile enthält mindestens einen Audio-Tag; eckige Klammern sind wohlgeformt (nur `[kleinbuchstaben]`).
  * Erklärter Zusatz: Spricht die Datei mehr als der Spieltext (`ZUSAETZE`), muss der Sprechtext genau Spieltext + deklarierter Zusatz sein.
    Zusätze werden nach der Lieferung in den Spieltext übernommen (Spieltexte werden an den Ton angepasst).

Aufruf (Repository-Wurzel):  python3 docs/audio/verify_spoken_text.py
Rückgabewert 0 = alles stimmt, 1 = Abweichungen.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs" / "audio" / "elevenlabs_dialogues.md"
TAG = re.compile(r"\[[a-z][a-z ,'-]*\]")
MSG = re.compile(r'[a-z_]+:\s*"((?:[^"\\]|\\.)*)",\s*(?:requires:\s*\[[^\]]*\],\s*)?voice:\s*Some\("([^"]+)"\)')

# Sprechzeilen, die über den Spieltext hinaus sprechen: Event -> (Quelle des Basistextes, deklarierter Zusatz).
# Quelle: ("mission", Datei, Feld) = Spieltext aus einem Feld der Mission statt aus `voice: Some(..)`.
ZUSAETZE = {
    "narrator.m01_defeat": (
        ("mission", "m01_last_settlement.ron", "defeat_text"),
        "Doch die Chronik ist noch nicht zu Ende. Erhebt Euch, Wächterin, und versucht es erneut.",
    ),
}


def norm(s):
    s = TAG.sub(" ", s)
    return re.sub(r"\s+", " ", s).strip()


def unescape(s):
    return s.replace('\\"', '"').replace("\\n", " ").replace("\\\\", "\\")


def game_texts():
    out = {}  # event -> {text: datei}
    for sub in ("missions", "campaign"):
        for p in sorted((ROOT / "assets" / "data" / sub).glob("*.ron")):
            for m in MSG.finditer(p.read_text(encoding="utf-8")):
                out.setdefault(m.group(2), {}).setdefault(norm(unescape(m.group(1))), p.name)
    return out


def mission_field(fname, field):
    text = (ROOT / "assets" / "data" / "missions" / fname).read_text(encoding="utf-8")
    m = re.search(field + r':\s*"((?:[^"\\]|\\.)*)"', text)
    return norm(unescape(m.group(1))) if m else None


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
                key = next(h for h in head if h.startswith("Sprechtext (DE"))
                out[f.strip("`").split("/")[-1][:-4]] = row.get(key, "")
        else:
            head = None
    return out


def bracket_problem(s):
    """Wohlgeformtheit der Klammern: nach dem Entfernen gültiger Tags darf keine eckige Klammer übrig sein."""
    rest = TAG.sub("", s)
    return "[" in rest or "]" in rest


def main():
    texts = game_texts()
    spoken = spoken_lines()
    errors = []
    checked = 0
    for event, variants in sorted(texts.items()):
        if len(variants) > 1:
            errors.append(f"{event}: mehrere verschiedene Spieltexte in {sorted(set(variants.values()))}")
            continue
        game = next(iter(variants))
        stem = event.replace(".", "_")
        files = sorted(s for s in spoken if re.fullmatch(re.escape(stem) + r"_\d+", s))
        if not files:
            errors.append(f"{event}: keine Sprechzeile im Dokument")
            continue
        if files != [stem + "_01"]:
            errors.append(f"{event}: genau eine Datei `{stem}_01` erwartet, gefunden: {files}")
            continue
        checked += 1
        line = spoken[files[0]]
        if norm(line) != game:
            errors.append(f"{files[0]}: Sprechtext weicht ab\n    Spiel : {game}\n    Sprech: {norm(line)}")
    # Erklärte Zusätze (z. B. Niederlage-Erzähler: defeat_text + Zusatz)
    for event, (src, extra) in ZUSAETZE.items():
        stem = event.replace(".", "_") + "_01"
        line = spoken.get(stem)
        if line is None:
            errors.append(f"{event}: keine Sprechzeile im Dokument")
            continue
        base = mission_field(src[1], src[2]) if src[0] == "mission" else None
        if base is None:
            errors.append(f"{event}: Basistext {src} nicht gefunden")
            continue
        expect = f"{base} {extra}"
        checked += 1
        if norm(line) != expect:
            errors.append(f"{stem}: Sprechtext != {src[2]} + Zusatz\n    Soll  : {expect}\n    Sprech: {norm(line)}")
    # Tags: jede Sprechzeile hat mindestens einen, Klammern sind wohlgeformt
    for stem, line in sorted(spoken.items()):
        if not TAG.search(line):
            errors.append(f"{stem}: kein Audio-Tag im Sprechtext")
        if bracket_problem(line):
            errors.append(f"{stem}: ungültige eckige Klammer im Sprechtext: {line}")
    covered = {f"{e.replace('.', '_')}_01" for e in texts} | {f"{e.replace('.', '_')}_01" for e in ZUSAETZE}
    extra_lines = sorted(set(spoken) - covered)
    print(f"Events mit Spieltext: {len(texts)}, geprüft (ganze Zeilen, ohne Tags gleich Spieltext): {checked}, Sprechzeilen im Dokument: {len(spoken)}")
    print(f"Sprechzeilen ohne Spieltext-Event (nur Tag-Prüfung): {len(extra_lines)}")
    if errors:
        print(f"FEHLER ({len(errors)}):")
        for e in errors:
            print(" -", e)
        return 1
    print("OK: Sprechtext == Spieltext (ohne Tags), jede Sprechzeile hat Audio-Tags")
    return 0


if __name__ == "__main__":
    sys.exit(main())
