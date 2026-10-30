#!/usr/bin/env python3
"""Lecture d'un essai de Mouflon (`MOUFLON_ESSAI`).

    python3 tools/essai/analyse.py essais/2026-10-30_21h40/   # dossier d'un essai
    python3 tools/essai/analyse.py journal.log                # un journal seul

Le journal (format décrit dans `crates/mouflon-app/src/stats.rs`) note chaque
image, chaque événement, les retards du fil principal avec sa pile, et toutes
les dix secondes les ressources du processus, de ses fils et du système. Le
dossier d'un essai (`Essai de 3 minutes.command`) y ajoute ce qui a été relevé
autour : `top`, `log stream`, `sample`, `vm_stat`, `pmset`, et des repères.

Ce script en tire ce qui cloche : les à-coups et leur cause probable, les
blocages et où était le fil principal, les images longues à préparer, les
animations trop lentes, les sauts de position, les collisions, puis les
ressources (processeur, énergie, mémoire, allocations, fils, système) et ce
qui se passait sur le Mac au moment des à-coups.
"""

import os
import re
import statistics
import sys
from collections import Counter, defaultdict
from datetime import datetime, timedelta, timezone

# Le nom d'un comportement peut contenir des espaces (« fall soft ») : les
# champs sont lus depuis la fin.
PET = re.compile(r"(\d+) (.+) (-?\d+) (\S+),(\S+) (\S+),(\S+) ([sat]) (\S+)$")
# Parties d'une image, pour dire ce qui a retardé un à-coup.
CAUSES = {"des": "dessin", "att": "attente de l'écran", "env": "envoi", "rel": "relevés",
          "sim": "simulation", "son": "sons", "fen": "fenêtres ouvertes ou fermées",
          "dep": "déplacements de fenêtres", "cur": "zones cliquables"}
PHASE_KEYS = set(CAUSES)
NUM = re.compile(r"^-?\d+(?:\.\d+)?$")


def number(text):
    try:
        return float(text)
    except ValueError:
        return None


def pairs(tokens):
    """« clé valeur clé valeur… » → dictionnaire (valeurs numériques)."""
    out = {}
    i = 0
    while i + 1 < len(tokens):
        value = number(tokens[i + 1])
        if value is not None:
            out[tokens[i]] = value
            i += 2
        else:
            i += 1
    return out


class Trial:
    def __init__(self):
        self.header = []
        self.frames = []
        self.events = []
        self.stalls = []      # dict(t, cause, gap, late, samples, asleep, mode, stacks=[(count, text)])
        self.small_stacks = []  # (t, count, text) des petits retards
        self.diag_late = []   # (t, ms)
        self.lines = defaultdict(list)  # S, SH, SP, SS, SB, SA, SW → [(t, dict)]
        self.threads = []     # (t, {nom: pct})
        self.panics = []
        self.bilan = None
        self.start_epoch = None
        self.tz = timezone.utc


def parse_frame(line):
    head, _, pets = line.partition(" | ")
    tokens = head.split()
    if len(tokens) < 7 or tokens[2] != "ecart" or tokens[4] != "prep":
        return None
    frame = {"t": float(tokens[1]), "gap": float(tokens[3]), "prep": float(tokens[5]),
             "clock": tokens[6] == "clk", "values": pairs(tokens[7:]), "pets": {}}
    v = frame["values"]
    frame["cadence"] = v.get("cad")
    frame["refresh"] = v.get("ecr")
    frame["phases"] = {k: v[k] for k in PHASE_KEYS if k in v}
    if pets:
        for chunk in (" | " + pets).split(" | ")[1:]:
            p = PET.match(chunk.strip())
            if p:
                pid, behavior, image, x, y, vx, vy, support, roll = p.groups()
                frame["pets"][int(pid)] = {
                    "behavior": behavior, "image": int(image), "x": float(x), "y": float(y),
                    "vx": float(vx), "vy": float(vy), "support": support, "roll": float(roll),
                }
    return frame


def parse(path):
    trial = Trial()
    last_stall = None
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            kind, _, rest = line.partition(" ")
            if kind == "F":
                frame = parse_frame(line)
                if frame:
                    trial.frames.append(frame)
            elif kind == "E":
                t, _, what = rest.partition(" ")
                if number(t) is not None:
                    trial.events.append((float(t), what))
            elif kind == "B":
                tokens = rest.split()
                values = pairs(tokens[1:])
                mode = tokens[tokens.index("mode") + 1] if "mode" in tokens else ""
                cause = tokens[tokens.index("raison") + 1] if "raison" in tokens else "?"
                last_stall = {"t": float(tokens[0]), "cause": cause, "gap": values.get("ecart_ms", 0),
                              "late": values.get("retard_ms", 0), "samples": int(values.get("echantillons", 0)),
                              "asleep": int(values.get("endormi", 0)), "mode": mode, "stacks": []}
                trial.stalls.append(last_stall)
            elif kind == "BP" and last_stall is not None:
                count, _, text = rest.partition(" ")
                last_stall["stacks"].append((int(count), text))
            elif kind == "BPP":
                count, _, text = rest.partition(" ")
                t = trial.lines["SB"][-1][0] if trial.lines["SB"] else 0.0
                trial.small_stacks.append((t, int(count), text))
            elif kind == "BD":
                tokens = rest.split()
                trial.diag_late.append((float(tokens[0]), float(tokens[-1])))
            elif kind == "SF":
                tokens = rest.split()
                usage = {}
                for token in tokens[3:]:
                    name, _, value = token.partition("=")
                    pct = number(value.split("%")[0])
                    if pct is not None:
                        usage[name] = pct
                trial.threads.append((float(tokens[0]), usage))
            elif kind in ("S", "SH", "SP", "SS", "SB", "SA", "SW"):
                tokens = rest.split()
                values = pairs(tokens[1:])
                if kind == "SS" and "charge" in tokens:
                    i = tokens.index("charge")
                    values["charge"] = float(tokens[i + 1])
                    values["charge5"] = float(tokens[i + 2])
                    values["charge15"] = float(tokens[i + 3])
                if kind == "SB" and "modes" in tokens:
                    values["modes"] = tokens[tokens.index("modes") + 1]
                trial.lines[kind].append((float(tokens[0]), values))
            elif kind == "PANIQUE":
                trial.panics.append(rest)
            elif kind == "BILAN":
                trial.bilan = line
            elif trial.bilan is not None and line.startswith(" "):
                trial.bilan += "\n" + line
            else:
                trial.header.append(line)
                if kind == "DEBUT":
                    parts = rest.split()
                    trial.start_epoch = float(parts[0])
                    if len(parts) >= 4:
                        m = re.match(r"([+-])(\d\d)(\d\d)", parts[3])
                        if m:
                            sign = 1 if m.group(1) == "+" else -1
                            trial.tz = timezone(sign * timedelta(hours=int(m.group(2)), minutes=int(m.group(3))))
    trial.stalls.sort(key=lambda s: s["t"])
    trial.events.sort()
    return trial


def moving(frame):
    return any(abs(p["vx"]) > 1 or abs(p["vy"]) > 1 or p["support"] != "s" for p in frame["pets"].values())


def near(events, t, before=0.15, after=0.05):
    return [f"{e_t:.2f} {what}" for e_t, what in events if t - before <= e_t <= t + after]


def short_stack(text, keep=10):
    """Les appels les plus parlants d'une pile : le haut, puis ceux de Mouflon."""
    frames = text.split(" < ")
    shown = frames[:4]
    ours = [f for f in frames[4:] if f.startswith("mouflon`") and "std::" not in f and "core::" not in f]
    shown += ours[:keep - len(shown)]
    out = " < ".join(re.sub(r"\+\d+$", "", f) for f in shown)
    if len(frames) > len(shown):
        out += " < …"
    return out


def percentile(values, p):
    if not values:
        return 0.0
    values = sorted(values)
    return values[min(len(values) - 1, round((len(values) - 1) * p))]


def expected_interval(frame, normal):
    """L'écart voulu entre deux images : la cadence demandée, mais pas plus
    vite que l'écran (ProMotion met quelques images à monter à 120 Hz)."""
    want = 1000 / frame["cadence"] if frame["cadence"] else 0
    screen = frame["refresh"] or 0
    return max(want, screen) or normal


def classify(trial, i, normal):
    """La cause probable d'un à-coup à l'image `i`."""
    frames = trial.frames
    f, before = frames[i], frames[i - 1]
    v = f["values"]
    interval = expected_interval(f, normal)
    for stall in trial.stalls:
        if before["t"] - 0.002 <= stall["t"] <= f["t"]:
            where = "endormi" if stall["asleep"] > stall["samples"] / 2 else "occupé"
            return f"blocage du fil principal ({where})"
    if f["clock"]:
        if v.get("mv", 0) >= 1:
            return "écran : rafraîchissements sautés par l'horloge"
        if v.get("vs", 0) > interval * 0.5:
            return "horloge appelée en retard (fil principal pris, ou processus en attente d'un cœur)"
        if v.get("q", 0) > interval * 0.5:
            return "battement retenu dans la boucle d'événements"
        if v.get("co", 0) >= 1:
            return "battements fondus"
    elif v.get("tard", 0) > interval * 0.5:
        return "minuterie en retard"
    longest = max(((value, key) for key, value in before["phases"].items()), default=(0, None))
    if longest[0] >= interval * 0.5:
        return f"image d'avant longue : {CAUSES.get(longest[1], longest[1])}"
    if before["prep"] >= interval * 0.5:
        return "image d'avant longue (hors parties mesurées)"
    return "hors de l'app (système, écran)"


def wall_to_trial(trial, when):
    if trial.start_epoch is None:
        return None
    return when.timestamp() - trial.start_epoch


def read_top(trial, path):
    """`top -l 0` : un bloc toutes les deux secondes → [(t, [(cpu, nom, énergie)])]."""
    if not os.path.exists(path):
        return []
    blocks, current, stamp = [], None, None
    columns = None
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.match(r"^(\d{4})/(\d\d)/(\d\d) (\d\d):(\d\d):(\d\d)", line)
            if m:
                stamp = datetime(*map(int, m.groups()), tzinfo=trial.tz)
                current = []
                blocks.append((wall_to_trial(trial, stamp), current))
                continue
            if line.startswith("PID"):
                columns = line.split()
                continue
            if current is not None and columns and re.match(r"^\s*\d+", line):
                parts = line.split()
                try:
                    cpu = float(parts[columns.index("%CPU")])
                except (ValueError, IndexError):
                    continue
                name = " ".join(parts[1:len(parts) - (len(columns) - 2)])
                power = parts[columns.index("POWER") - len(columns)] if "POWER" in columns else "?"
                current.append((cpu, name, power))
    return [(t, procs) for t, procs in blocks if t is not None]


def read_log(trial, path):
    """`log stream --style compact` : [(t, texte)]."""
    if not os.path.exists(path):
        return []
    out = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.match(r"^(\d{4})-(\d\d)-(\d\d) (\d\d):(\d\d):(\d\d)\.(\d+)", line)
            if not m:
                continue
            *date, frac = m.groups()
            stamp = datetime(*map(int, date), int(frac[:6].ljust(6, "0")), tzinfo=trial.tz)
            t = wall_to_trial(trial, stamp)
            if t is not None:
                out.append((t, line.rstrip()[m.end():].strip()))
    return out


def read_marks(trial, path):
    """Repères du script : [(t, texte)]."""
    if not os.path.exists(path) or trial.start_epoch is None:
        return []
    out = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.match(r"^(\d+) \+\d+ s : (.*)$", line.strip())
            if m:
                out.append((float(m.group(1)) - trial.start_epoch, m.group(2)))
    return out


def sample_top(path, count=30):
    """Le haut des piles le plus vu par `sample` (« Sort by top of stack »)."""
    if not os.path.exists(path):
        return []
    lines, inside = [], False
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if line.startswith("Sort by top of stack"):
                inside = True
                continue
            if inside:
                if not line.strip() or line.startswith("Binary Images"):
                    if lines:
                        break
                    continue
                lines.append(line.rstrip())
    return lines[:count]


def section(title):
    print(f"\n{title}")


def report_frames(trial):
    frames, events = trial.frames, trial.events
    duration = frames[-1]["t"] - frames[0]["t"]
    clocked = sum(f["clock"] for f in frames)
    print(f"\n{len(frames)} images en {duration:.0f} s ({len(frames) / max(duration, 1e-6):.0f} par seconde), "
          f"{clocked} sur l'horloge de l'écran")
    cadences = defaultdict(float)
    for f in frames[1:]:
        if f["cadence"] is not None:
            cadences[f["cadence"]] += f["gap"] / 1000
    if cadences:
        parts = ", ".join(f"{c:.0f} i/s : {t:.0f} s" for c, t in sorted(cadences.items(), reverse=True))
        print(f"cadences voulues : {parts}")
    rates = defaultdict(float)
    for f in frames[1:]:
        if f["refresh"]:
            rates[round(1000 / f["refresh"])] += f["gap"] / 1000
    if rates:
        parts = ", ".join(f"{hz} Hz : {t:.0f} s" for hz, t in sorted(rates.items(), reverse=True) if t >= 0.5)
        print(f"fréquence de l'écran : {parts}")

    moving_gaps = [f["gap"] for f in frames[1:] if moving(f) and f["gap"] > 0]
    if not moving_gaps:
        return []
    normal = statistics.median(moving_gaps)
    print(f"écart normal en mouvement : {normal:.1f} ms ({1000 / normal:.0f} images par seconde)")
    hitches = [(i, f) for i, f in enumerate(frames)
               if i > 0 and moving(f) and f["gap"] > expected_interval(f, normal) * 1.6]
    section(f"À-COUPS en mouvement (écart > 1,6 × l'écart voulu, l'écran compris) : {len(hitches)}"
            f" ({100 * len(hitches) / max(len(moving_gaps), 1):.2f} % des images)")
    causes = Counter(classify(trial, i, normal) for i, _ in hitches)
    for cause, n in causes.most_common():
        lost = sum(f["gap"] - expected_interval(f, normal) for i, f in hitches if classify(trial, i, normal) == cause)
        print(f"  {n:5d}  {cause}  ({lost / 1000:.1f} s perdues)")
    worst = sorted(hitches, key=lambda h: -h[1]["gap"])[:30]
    print("  les plus longs :")
    for i, f in sorted(worst, key=lambda h: h[1]["t"]):
        before = frames[i - 1]
        v = f["values"]
        ph = " ".join(f"{k} {x:.1f}" for k, x in before["phases"].items() if x >= 0.5)
        clock = (f" vs {v.get('vs', 0):.1f} q {v.get('q', 0):.1f} mv {v.get('mv', 0):.0f} co {v.get('co', 0):.0f}"
                 if f["clock"] and "vs" in v else (f" tard {v.get('tard', 0):.1f}" if "tard" in v else ""))
        screen = f" ; écran {1000 / f['refresh']:.0f} Hz" if f["refresh"] else ""
        what = "; ".join(near(events, f["t"])) or "—"
        print(f"  {f['t']:8.2f} s : écart {f['gap']:6.1f} ms [{classify(trial, i, normal)}]{clock}{screen} ; "
              f"avant : {before['prep']:.1f} ms [{ph or 'rien de long'}] ; {what}")
    return hitches


def report_clock(trial):
    beats = [f["values"] for f in trial.frames if f["clock"] and "vs" in f["values"]]
    if not beats:
        return
    section("HORLOGE DE L'ÉCRAN (médiane, 99 %, maximum, en ms) :")
    for key, label in (("vs", "de la synchronisation à l'appel"), ("q", "de l'appel au début de l'image")):
        values = [b[key] for b in beats if key in b]
        print(f"  {label:<34} {statistics.median(values):6.2f} {percentile(values, 0.99):6.2f} {max(values):7.2f}")
    missed = sum(b.get("mv", 0) for b in beats)
    fused = sum(b.get("co", 0) for b in beats)
    print(f"  rafraîchissements sautés : {missed:.0f} ; battements fondus : {fused:.0f}")
    late = [f["values"]["tard"] for f in trial.frames if not f["clock"] and "tard" in f["values"]]
    if late:
        print(f"  minuterie : {len(late)} images, retard médian {statistics.median(late):.1f} ms, "
              f"max {max(late):.1f} ms")


def report_stalls(trial):
    section(f"BLOCAGES ET GRANDS RETARDS DU FIL PRINCIPAL (racontés : {len(trial.stalls)}) :")
    if not trial.stalls:
        print("  aucun")
    for stall in trial.stalls[:40]:
        state = f"{stall['asleep']}/{stall['samples']} échantillons endormi"
        mode = f" ; mode {stall['mode']}" if stall["mode"] else ""
        print(f"  {stall['t']:8.2f} s : {stall['cause']}, écart {stall['gap']:.0f} ms, retard {stall['late']:.0f} ms "
              f"({state}){mode} ; {'; '.join(near(trial.events, stall['t'], 0.1, 0.6)) or '—'}")
        for count, text in stall["stacks"][:2]:
            print(f"      {count:4d} × {short_stack(text)}")
    if len(trial.stalls) > 40:
        print(f"  … et {len(trial.stalls) - 40} autres")
    # Où le fil principal attendait-il, tous retards confondus ?
    tops = Counter()
    for stall in trial.stalls:
        for count, text in stall["stacks"]:
            tops[" < ".join(re.sub(r"\+\d+$", "", f) for f in text.split(" < ")[:3])] += count
    for _, count, text in trial.small_stacks:
        tops[" < ".join(re.sub(r"\+\d+$", "", f) for f in text.split(" < ")[:3])] += count
    if tops:
        section("HAUT DE LA PILE PENDANT LES RETARDS (tous, échantillons) :")
        for text, count in tops.most_common(12):
            print(f"  {count:6d}  {text}")
    if trial.diag_late:
        section(f"LE FIL DES DIAGNOSTICS LUI-MÊME EN RETARD (tout le processus attendait) : {len(trial.diag_late)}")
        for t, ms in trial.diag_late[:15]:
            print(f"  {t:8.2f} s : {ms:.0f} ms")


def report_slow(trial):
    slow = sorted(trial.frames, key=lambda f: -f["prep"])[:15]
    section("IMAGES LES PLUS LONGUES À PRÉPARER :")
    for f in sorted(slow, key=lambda f: f["t"]):
        ph = " ".join(f"{k} {v:.1f}" for k, v in f["phases"].items() if v >= 0.3)
        extra = " ".join(f"{k} {f['values'][k]:.1f}" for k in ("dmax", "amax", "al") if f["values"].get(k, 0) >= 0.5)
        print(f"  {f['t']:8.2f} s : {f['prep']:6.1f} ms [{ph}] {extra} ; "
              f"{'; '.join(near(trial.events, f['t'], 0.05, 0.02)) or '—'}")


def report_allocs(trial):
    allocs = [f["values"]["al"] for f in trial.frames if "al" in f["values"]]
    if not allocs:
        return
    kb = [f["values"].get("ko", 0) for f in trial.frames if "al" in f["values"]]
    section("ALLOCATIONS DU FIL PRINCIPAL PAR IMAGE (médiane, 99 %, maximum) :")
    print(f"  nombre : {statistics.median(allocs):.0f} {percentile(allocs, 0.99):.0f} {max(allocs):.0f} ; "
          f"Kio : {statistics.median(kb):.1f} {percentile(kb, 0.99):.1f} {max(kb):.1f} ; "
          f"images sans allocation : {100 * sum(a == 0 for a in allocs) / len(allocs):.0f} %")


def report_animations(trial):
    time_in, changes, speed, previous = defaultdict(float), defaultdict(int), defaultdict(list), {}
    for f in trial.frames:
        for pid, p in f["pets"].items():
            key = p["behavior"]
            time_in[key] += f["gap"] / 1000
            speed[key].append(abs(p["vx"]) + abs(p["vy"]))
            last = previous.get(pid)
            if last and last[0] == key and last[1] != p["image"]:
                changes[key] += 1
            previous[pid] = (key, p["image"])
    section("ANIMATIONS (temps, images par seconde, vitesse moyenne) — les plus lentes d'abord :")
    rows = []
    for key, t in time_in.items():
        if t >= 0.3:
            rows.append((changes[key] / t, key, t, statistics.mean(speed[key])))
    for rate, key, t, v in sorted(rows):
        flag = "  ← peu d'images pour un mouvement rapide" if rate < 14 and v > 150 else ""
        print(f"  {key:<16} {t:7.1f} s  {rate:5.1f} images/s  {v:6.0f} pt/s{flag}")


def report_jumps(trial):
    jumps = []
    for a, b in zip(trial.frames, trial.frames[1:]):
        dt = b["gap"] / 1000
        for pid, p in b["pets"].items():
            q = a["pets"].get(pid)
            if not q or dt <= 0 or q["behavior"] != p["behavior"] or p["support"] == "t":
                continue
            dist = ((p["x"] - q["x"]) ** 2 + (p["y"] - q["y"]) ** 2) ** 0.5
            expected = (abs(p["vx"]) + abs(p["vy"]) + abs(q["vx"]) + abs(q["vy"])) * 0.5 * dt
            if dist > max(6.0, 3 * expected):
                jumps.append((b["t"], pid, p["behavior"], dist, expected))
    section(f"SAUTS DE POSITION (plus de 3 fois la vitesse) : {len(jumps)}")
    by_behavior = Counter(behavior for _, _, behavior, _, _ in jumps)
    if by_behavior:
        print("  par comportement : " + ", ".join(f"{k} {v}" for k, v in by_behavior.most_common(8)))
    for t, pid, behavior, dist, expected in jumps[:12]:
        print(f"  {t:8.2f} s : n° {pid} ({behavior}) {dist:.1f} pt au lieu de {expected:.1f}")


def report_collisions(trial):
    bumps = [(t, what) for t, what in trial.events if "heurté" in what]
    section(f"COLLISIONS : {len(bumps)}")
    for t, what in bumps[:8]:
        pid = int(what.split()[1])
        after = [f for f in trial.frames if t <= f["t"] <= t + 1.5 and pid in f["pets"]]
        seq = []
        for f in after:
            p = f["pets"][pid]
            label = f"{p['behavior']}:{p['image']}"
            if not seq or seq[-1][0] != label:
                seq.append([label, f["t"]])
        steps = " → ".join(f"{label}@{(start - t) * 1000:.0f}" for label, start in seq[:12])
        print(f"  {t:8.2f} s n° {pid} : {steps}")


def series(trial, kind, key):
    return [values[key] for _, values in trial.lines[kind] if key in values]


def summary(values):
    return (f"{values[0]:8.1f} {statistics.median(values):8.1f} {max(values):8.1f} {values[-1]:8.1f}"
            if values else "")


def report_resources(trial):
    section("RESSOURCES (début, médiane, maximum, fin) :")
    rows = [
        ("S", "memoire_mo", "mémoire résidente (Mo)"), ("S", "empreinte_mo", "empreinte mémoire (Mo)"),
        ("S", "mo_gpu", "images sur le GPU (Mo)"), ("S", "mo_images", "images en mémoire (Mo)"),
        ("SP", "cpu_pct", "processeur (% d'un cœur)"), ("SP", "coeurs_perf_pct", "dont cœurs performants (%)"),
        ("SP", "puissance_mw", "puissance (mW)"), ("SP", "ipc", "instructions par cycle"),
        ("SP", "attente_coeur_ms", "attente d'un cœur (ms / 10 s)"), ("SP", "reveils", "réveils (/ 10 s)"),
        ("SP", "reveils_inactif", "réveils d'un cœur endormi"), ("SP", "csw", "changements de contexte"),
        ("SP", "appels_mach", "appels Mach"), ("SP", "appels_unix", "appels Unix"),
        ("SP", "msg_env", "messages Mach envoyés"), ("SP", "fautes", "fautes de page"),
        ("SP", "pic_periode_mo", "pic de mémoire (Mo)"), ("SP", "fils", "fils"), ("SP", "fd", "fichiers ouverts"),
        ("SP", "qos_ui_ms", "temps « interactif » (ms)"), ("SP", "qos_def_ms", "temps « par défaut » (ms)"),
        ("SP", "qos_util_ms", "temps « utilitaire » (ms)"),
        ("SA", "allocs", "allocations (/ 10 s)"), ("SA", "alloue_mo", "octets alloués (Mo / 10 s)"),
        ("SA", "vivant_mo", "mémoire allouée vivante (Mo)"), ("SA", "principal_allocs", "allocations, fil principal"),
        ("SB", "occupe_pct", "boucle occupée (%)"), ("SB", "plus_long_ms", "plus longue tâche (ms)"),
        ("SB", "reveils", "réveils de la boucle"), ("SB", "petits_retards", "petits retards"),
        ("SH", "fondus", "battements fondus"), ("SH", "sautes", "rafraîchissements sautés"),
        ("SH", "appel_max_ms", "appel de l'horloge, retard max (ms)"),
        ("SH", "passage_max_ms", "passage par la boucle, max (ms)"), ("SH", "dmax", "déplacement max (ms)"),
        ("SH", "amax", "attente d'écran max (ms)"), ("SH", "souris", "mouvements de souris"),
        ("SH", "lents", "gestionnaires lents"), ("SW", "moyen_ms", "relevé des fenêtres (ms)"),
        ("SW", "max_ms", "relevé des fenêtres, max (ms)"), ("SW", "fenetres", "fenêtres des autres apps"),
    ]
    for kind, key, label in rows:
        values = series(trial, kind, key)
        if values:
            print(f"  {label:<38} {summary(values)}")
    modes = sorted({values["modes"] for _, values in trial.lines["SB"] if "modes" in values})
    if modes:
        print(f"  modes de la boucle vus : {', '.join(modes)}")

    if trial.threads:
        section("FILS (processeur moyen, % d'un cœur) :")
        totals = defaultdict(float)
        for _, usage in trial.threads:
            for name, pct in usage.items():
                totals[re.sub(r"^fil\d+$", "fil sans nom", name)] += pct
        for name, total in sorted(totals.items(), key=lambda kv: -kv[1])[:15]:
            print(f"  {name:<40} {total / len(trial.threads):6.2f}")

    system = trial.lines["SS"]
    if system:
        section("SYSTÈME (début, médiane, maximum, fin) :")
        for key, label in (("cpu_machine_pct", "processeur de la machine (%)"), ("charge", "charge (1 min)"),
                           ("pression", "pression mémoire (1, 2, 4)"), ("libre_mo", "mémoire libre (Mo)"),
                           ("compressee_mo", "mémoire compressée (Mo)"), ("echange_mo", "échange (Mo)"),
                           ("pageouts", "pages écrites sur disque"), ("swapouts", "échanges vers le disque"),
                           ("thermique", "état thermique (0 à 3)"), ("economie", "économie d'énergie")):
            values = series(trial, "SS", key)
            if values:
                print(f"  {label:<38} {summary(values)}")


def report_events(trial):
    interesting = [(t, w) for t, w in trial.events
                   if w.startswith(("système", "boucle", "gestionnaire lent", "relevé lent", "menu", "horloge"))]
    section(f"ÉVÉNEMENTS DU SYSTÈME ET DE LA BOUCLE : {len(interesting)}")
    for t, what in interesting[:60]:
        print(f"  {t:8.2f} s : {what}")
    if len(interesting) > 60:
        print(f"  … et {len(interesting) - 60} autres")


def report_around(trial, hitches, folder):
    tops = read_top(trial, os.path.join(folder, "top.txt"))
    logs = read_log(trial, os.path.join(folder, "log.txt"))
    marks = read_marks(trial, os.path.join(folder, "reperes.txt"))
    if marks:
        section("REPÈRES DU SCRIPT :")
        for t, what in marks:
            print(f"  {t:8.1f} s : {what}")
    if tops:
        section("LES APPS QUI CALCULENT LE PLUS (moyenne des relevés de top, % d'un cœur) :")
        totals, energy = defaultdict(float), defaultdict(float)
        for _, procs in tops:
            for cpu, name, power in procs:
                totals[name] += cpu
                value = number(power)
                if value is not None:
                    energy[name] += value
        for name, total in sorted(totals.items(), key=lambda kv: -kv[1])[:12]:
            print(f"  {name:<28} {total / len(tops):6.1f}  énergie {energy[name] / len(tops):6.1f}")
    if hitches and (tops or logs):
        section("AUTOUR DES À-COUPS LES PLUS LONGS :")
        for i, f in sorted(hitches, key=lambda h: -h[1]["gap"])[:10]:
            t = f["t"]
            print(f"  {t:8.2f} s (écart {f['gap']:.0f} ms) :")
            block = min(tops, key=lambda b: abs(b[0] - t), default=None)
            if block and abs(block[0] - t) < 3:
                busy = ", ".join(f"{name} {cpu:.0f} %" for cpu, name, _ in sorted(block[1], reverse=True)[:5])
                print(f"      top à {block[0]:.0f} s : {busy}")
            for lt, text in [(lt, text) for lt, text in logs if abs(lt - t) <= 0.5][:6]:
                print(f"      log {lt:8.2f} s : {text[:160]}")
    if logs:
        section(f"MESSAGES DU SYSTÈME : {len(logs)} (les plus fréquents)")
        kinds = Counter(re.sub(r"[0-9a-fx]{4,}", "#", text)[:120] for _, text in logs)
        for text, count in kinds.most_common(15):
            print(f"  {count:5d}  {text}")
    sample = sample_top(os.path.join(folder, "sample.txt"))
    if sample:
        section("SAMPLE : HAUT DES PILES LE PLUS VU (tout l'essai, tous les fils) :")
        for line in sample:
            print(f"  {line}")


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__.split("\n\n")[1])
    target = sys.argv[1]
    folder = target if os.path.isdir(target) else None
    path = os.path.join(target, "journal.log") if folder else target
    trial = parse(path)
    for line in trial.header:
        print(line)
    for panic in trial.panics:
        print(f"\nPANIQUE {panic}")
    if not trial.frames:
        sys.exit("aucune image dans le journal")
    hitches = report_frames(trial)
    report_clock(trial)
    report_stalls(trial)
    report_slow(trial)
    report_allocs(trial)
    report_animations(trial)
    report_jumps(trial)
    report_collisions(trial)
    report_resources(trial)
    report_events(trial)
    if folder:
        report_around(trial, hitches, folder)
    section("RÉSUMÉS (toutes les dix secondes) :")
    for t, values in trial.lines["S"]:
        print(f"  {t:8.1f} s : " + " ".join(f"{k} {v:g}" for k, v in values.items()))
    if trial.bilan:
        print("\n" + trial.bilan)


if __name__ == "__main__":
    main()
