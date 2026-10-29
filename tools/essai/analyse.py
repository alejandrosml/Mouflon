#!/usr/bin/env python3
"""Lecture d'un journal d'essai de Mouflon (`MOUFLON_ESSAI`).

    python3 tools/essai/analyse.py mouflon-essai.log

Le journal note chaque image (écart avec la précédente, temps passé dans
chaque partie, état de chaque animal) et chaque événement. Ce script en tire
ce qui cloche : les à-coups (une image qui arrive trop tard pendant qu'un
animal bouge) avec ce qui les accompagne, les images longues à préparer, les
animations trop lentes (peu d'images par seconde), les sauts de position,
le déroulé de chaque collision, la mémoire et le processeur.
"""

import re
import statistics
import sys
from collections import defaultdict

FRAME = re.compile(r"^F (\S+) ecart (\S+) prep (\S+) (clk|tmr)(?: cad (\S+))? (.*?)((?: \| .*)?)$")
PHASES = re.compile(r"(rel|sim|son|fen|dep|des|att|env|r) (\S+)")
# Le nom d'un comportement peut contenir des espaces (« fall soft ») : les
# champs sont lus depuis la fin.
PET = re.compile(r"(\d+) (.+) (-?\d+) (\S+),(\S+) (\S+),(\S+) ([sat]) (\S+)$")
# Parties d'une image, pour dire ce qui a retardé un à-coup.
CAUSES = {"des": "dessin", "att": "attente de l'écran", "env": "envoi", "rel": "relevés",
          "sim": "simulation", "son": "sons", "fen": "fenêtres", "dep": "déplacements"}
EVENT = re.compile(r"^E (\S+) (.*)$")
SUMMARY = re.compile(r"^S (\S+) (.*)$")


def parse(path):
    frames, events, summaries, header = [], [], [], []
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("F "):
                m = FRAME.match(line)
                if not m:
                    continue
                t, gap, prep, source, cadence, phases, pets = m.groups()
                frame = {
                    "t": float(t), "gap": float(gap), "prep": float(prep), "clock": source == "clk",
                    "cadence": float(cadence) if cadence else None,
                    "phases": {k: float(v) for k, v in PHASES.findall(phases)}, "pets": {},
                }
                for chunk in pets.split(" | ")[1:]:
                    p = PET.match(chunk.strip())
                    if p:
                        pid, behavior, image, x, y, vx, vy, support, roll = p.groups()
                        frame["pets"][int(pid)] = {
                            "behavior": behavior, "image": int(image), "x": float(x), "y": float(y),
                            "vx": float(vx), "vy": float(vy), "support": support, "roll": float(roll),
                        }
                frames.append(frame)
            elif line.startswith("E "):
                m = EVENT.match(line)
                if m:
                    events.append((float(m.group(1)), m.group(2)))
            elif line.startswith("S "):
                m = SUMMARY.match(line)
                if m:
                    summaries.append((float(m.group(1)), m.group(2)))
            elif line.startswith("BILAN"):
                header.append(line)
            else:
                header.append(line)
    return frames, events, summaries, header


def moving(frame):
    return any(abs(p["vx"]) > 1 or abs(p["vy"]) > 1 or p["support"] != "s" for p in frame["pets"].values())


def near(events, t, before=0.15, after=0.05):
    return [f"{e_t:.2f} {what}" for e_t, what in events if t - before <= e_t <= t + after]


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__.split("\n\n")[1])
    frames, events, summaries, header = parse(sys.argv[1])
    for line in header:
        if not line.startswith("BILAN"):
            print(line)
    if not frames:
        sys.exit("aucune image dans le journal")
    duration = frames[-1]["t"] - frames[0]["t"]
    clocked = sum(f["clock"] for f in frames)
    print(f"\n{len(frames)} images en {duration:.0f} s ({len(frames) / max(duration, 1e-6):.0f} par seconde), "
          f"{clocked} sur l'horloge de l'écran")

    # Cadences voulues (images par seconde) et temps passé à chacune.
    cadences = defaultdict(float)
    for f in frames[1:]:
        if f["cadence"] is not None:
            cadences[f["cadence"]] += f["gap"] / 1000
    if cadences:
        parts = ", ".join(f"{c:.0f} i/s : {t:.0f} s" for c, t in sorted(cadences.items(), reverse=True))
        print(f"cadences voulues : {parts}")

    # Rythme attendu pendant le mouvement : la cadence voulue pour l'image
    # (notée par les journaux récents), sinon l'écart médian quand un animal bouge.
    moving_gaps = [f["gap"] for f in frames[1:] if moving(f) and f["gap"] > 0]
    if moving_gaps:
        normal = statistics.median(moving_gaps)
        print(f"écart normal en mouvement : {normal:.1f} ms ({1000 / normal:.0f} images par seconde)")

        def expected(f):
            return 1000 / f["cadence"] if f["cadence"] else normal

        hitches = [(i, f) for i, f in enumerate(frames)
                   if i > 0 and moving(f) and f["gap"] > expected(f) * 1.6]
        print(f"\nÀ-COUPS en mouvement (écart > 1,6 × l'écart voulu) : {len(hitches)}"
              f" ({100 * len(hitches) / max(len(moving_gaps), 1):.2f} % des images)")
        # Ce qui a pris le plus de temps dans l'image d'avant (celle qui a tardé).
        blame = defaultdict(int)
        for i, f in hitches:
            before = frames[i - 1]["phases"]
            longest = max(((v, k) for k, v in before.items() if k != "r"), default=(0, None))
            if longest[0] >= expected(f) * 0.5:
                blame[CAUSES.get(longest[1], longest[1])] += 1
            else:
                blame["hors de l'app (système, écran)"] += 1
        if blame:
            print("  causes : " + ", ".join(f"{k} {v}" for k, v in sorted(blame.items(), key=lambda kv: -kv[1])))
        worst = sorted(hitches, key=lambda h: -h[1]["gap"])[:25]
        for i, f in sorted(worst, key=lambda h: h[1]["t"]):
            before = frames[i - 1]
            ph = " ".join(f"{k} {v:.1f}" for k, v in before["phases"].items() if k != "r" and v >= 0.5)
            what = "; ".join(near(events, f["t"])) or "—"
            print(f"  {f['t']:8.2f} s : écart {f['gap']:6.1f} ms ; image d'avant préparée en {before['prep']:.1f} ms"
                  f" [{ph or 'rien de long'}] ; {what}")

    slow = sorted(frames, key=lambda f: -f["prep"])[:15]
    print("\nIMAGES LES PLUS LONGUES À PRÉPARER :")
    for f in sorted(slow, key=lambda f: f["t"]):
        ph = " ".join(f"{k} {v:.1f}" for k, v in f["phases"].items() if v >= 0.3)
        print(f"  {f['t']:8.2f} s : {f['prep']:6.1f} ms [{ph}] ; {'; '.join(near(events, f['t'], 0.05, 0.02)) or '—'}")

    # Animations : images de la séquence par seconde, comportement par comportement.
    time_in = defaultdict(float)
    changes = defaultdict(int)
    speed = defaultdict(list)
    previous = {}
    for f in frames:
        for pid, p in f["pets"].items():
            key = p["behavior"]
            time_in[key] += f["gap"] / 1000
            speed[key].append(abs(p["vx"]) + abs(p["vy"]))
            last = previous.get(pid)
            if last and last[0] == key and last[1] != p["image"]:
                changes[key] += 1
            previous[pid] = (key, p["image"])
    print("\nANIMATIONS (temps, images par seconde, vitesse moyenne) — les plus lentes d'abord :")
    rows = []
    for key, t in time_in.items():
        if t < 0.3:
            continue
        rate = changes[key] / t
        rows.append((rate, key, t, statistics.mean(speed[key])))
    for rate, key, t, v in sorted(rows):
        flag = "  ← peu d'images pour un mouvement rapide" if rate < 14 and v > 150 else ""
        print(f"  {key:<16} {t:7.1f} s  {rate:5.1f} images/s  {v:6.0f} pt/s{flag}")

    # Sauts : un animal qui se déplace bien plus que sa vitesse ne le dit.
    jumps = []
    for a, b in zip(frames, frames[1:]):
        dt = b["gap"] / 1000
        for pid, p in b["pets"].items():
            q = a["pets"].get(pid)
            if not q or dt <= 0 or q["behavior"] != p["behavior"] or p["support"] == "t":
                continue
            dx, dy = p["x"] - q["x"], p["y"] - q["y"]
            dist = (dx * dx + dy * dy) ** 0.5
            expected = (abs(p["vx"]) + abs(p["vy"]) + abs(q["vx"]) + abs(q["vy"])) * 0.5 * dt
            if dist > max(6.0, 3 * expected):
                jumps.append((b["t"], pid, p["behavior"], dist, expected))
    print(f"\nSAUTS DE POSITION (plus de 3 fois la vitesse) : {len(jumps)}")
    for t, pid, behavior, dist, expected in jumps[:20]:
        print(f"  {t:8.2f} s : n° {pid} ({behavior}) {dist:.1f} pt au lieu de {expected:.1f}")

    bumps = [(t, what) for t, what in events if "heurté" in what]
    print(f"\nCOLLISIONS : {len(bumps)}")
    for t, what in bumps[:12]:
        pid = int(what.split()[1])
        after = [f for f in frames if t <= f["t"] <= t + 1.5 and pid in f["pets"]]
        seq = []
        for f in after:
            p = f["pets"][pid]
            label = f"{p['behavior']}:{p['image']}"
            if not seq or seq[-1][0] != label:
                seq.append([label, f["t"]])
        steps = " → ".join(f"{label}@{(start - t) * 1000:.0f}" for label, start in seq[:14])
        gaps = [f["gap"] for f in after]
        print(f"  {t:8.2f} s n° {pid} : {steps}")
        if gaps:
            print(f"            écarts pendant 1,5 s : max {max(gaps):.1f} ms, médian {statistics.median(gaps):.1f} ms")

    print("\nRÉSUMÉS (toutes les dix secondes) :")
    for t, text in summaries:
        print(f"  {t:8.1f} s : {text}")
    for line in header:
        if line.startswith("BILAN"):
            print("\n" + line)


if __name__ == "__main__":
    main()
