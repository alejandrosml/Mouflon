#!/usr/bin/env python3
"""Redécoupe le Mannequin à ses articulations : l'exemple des nouvelles
parties du contrat (docs/rig.md).

Le Mannequin est dessiné à la main ; sa version d'avant les articulations est
gardée dans `crates/mouflon-render/tests/donnees/mannequin-avant.svg` (le
test `sans_changement` vérifie que le moteur la fait toujours bouger comme
avant). Ce script en tire `pets/mannequin/mannequin.svg` :

- les pattes en deux segments, coupées au carpe et au jarret ;
- l'encolure (`neck`) : le lobe de laine de l'avant, déjà dessiné sous le
  corps, devient l'encolure, posée juste avant `body` (le corps la recouvre
  à sa base) ; la tête reste par-dessus le corps, à part, et tourne
  désormais à la nuque (le moteur lui fait suivre l'encolure) ;
- la lèvre supérieure (`lip`), avec la gencive qu'elle découvre ;
- trois mèches de laine qui pendent sous le corps (`sway-1` à `sway-3`) ;
- `pivot-muzzle`, ce qui touche l'herbe.

    python3 tools/races/mannequin/articule.py

© 2026 Alejandro San Martin Lamas — MIT OR Apache-2.0
"""
import os
import sys
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import articulations as A  # noqa: E402

ROOT = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
SOURCE = os.path.join(ROOT, "crates/mouflon-render/tests/donnees/mannequin-avant.svg")
OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(ROOT, "pets/mannequin/mannequin.svg")

Q = A.Q

# Le carpe (devant) et le jarret (derrière) : à la bosse du dessin.
CARPUS_Y = 100.6
HOCK_Y = 100.0
# L'encolure : sa base, sous la laine de l'avant du corps ; la nuque,
# derrière les oreilles ; ce qui touche l'herbe.
NECK = (92.0, 66.0)
POLL = (103.5, 51.5)
MUZZLE = (134.0, 71.0)
# La lèvre supérieure, sous le naseau, jusqu'à la commissure ; elle se
# retrousse autour de son arrière.
LIP = [(129.4, 61.8), (133.0, 60.6), (135.5, 60.4), (137.6, 62.0), (137.6, 67.0),
       (135.9, 66.9), (134.3, 67.9), (132.2, 68.6), (130.0, 68.0), (128.8, 64.4)]
LIP_PIVOT = (130.2, 62.4)
# Les mèches sous le corps : (x de l'attache, y de l'attache, longueur, largeur).
TUFTS = [(99.5, 90.2, 5.6, 6.0), (70.0, 95.6, 5.2, 7.0), (49.5, 93.6, 4.8, 6.2)]


def gum():
    """La gencive (le bourrelet du haut : le mouton n'a pas d'incisives en
    haut), cachée sous la lèvre tant qu'elle est baissée."""
    d = ("M129.8 62.6C131.6 61.6 134.2 61.3 136.1 62.2C136.7 63.6 136.6 65.2 135.7 66.4"
         "C134.2 67.6 132.1 68.1 130.3 67.7C129.4 66.1 129.2 64.2 129.8 62.6Z")
    pad = ET.Element(Q + "path", {"d": d, "fill": "#c97f8f"})
    ridge = ET.Element(Q + "path", {
        "d": "M130.4 65.9C132.2 66.6 134.2 66.3 135.6 65.3",
        "fill": "none", "stroke": "#a85d6f", "stroke-width": "0.7", "stroke-linecap": "round"})
    dark = ET.Element(Q + "path", {
        "d": "M130.2 63.3C132.0 62.3 134.3 62.1 135.9 62.8",
        "fill": "none", "stroke": "#8e4a5b", "stroke-width": "0.6", "stroke-linecap": "round"})
    return [pad, ridge, dark]


def tuft(x, y, length, width, name):
    """Une mèche de laine, attachée en (x, y) sous la toison : sa base,
    cachée dans le corps, et son bout bouclé, cerné en bas seulement."""
    w = width / 2
    b = y + length
    fill = (f"M{x - w:.2f} {y - 2:.2f}C{x - w - 0.6:.2f} {y + length * 0.55:.2f} {x - w * 0.6:.2f} {b:.2f} {x:.2f} {b:.2f}"
            f"C{x + w * 0.6:.2f} {b:.2f} {x + w + 0.6:.2f} {y + length * 0.55:.2f} {x + w:.2f} {y - 2:.2f}Z")
    edge = (f"M{x - w - 0.25:.2f} {y + length * 0.3:.2f}C{x - w - 0.2:.2f} {y + length * 0.75:.2f} {x - w * 0.55:.2f} {b:.2f} {x:.2f} {b:.2f}"
            f"C{x + w * 0.55:.2f} {b:.2f} {x + w + 0.2:.2f} {y + length * 0.75:.2f} {x + w + 0.25:.2f} {y + length * 0.3:.2f}")
    curl = (f"M{x - w * 0.35:.2f} {b - length * 0.42:.2f}A{w * 0.42:.2f} {w * 0.42:.2f} 0 0 0 {x + w * 0.45:.2f} {b - length * 0.48:.2f}")
    g = ET.Element(Q + "g", {"id": name})
    ET.SubElement(g, Q + "path", {"d": fill, "fill": "url(#wool-body)"})
    ET.SubElement(g, Q + "path", {"d": edge, "fill": "none", "stroke": "#a9b2c4", "stroke-width": "1.2",
                                  "stroke-linecap": "round", "stroke-linejoin": "round"})
    ET.SubElement(g, Q + "path", {"d": curl, "fill": "none", "stroke": "#c9d0de", "stroke-width": "0.9",
                                  "stroke-linecap": "round"})
    ET.SubElement(g, Q + "circle", {"id": f"pivot-{name}", "cx": f"{x:.2f}", "cy": f"{y:.2f}", "r": "0.8"})
    return g


def main():
    text = open(SOURCE).read()
    legs = []
    for name, y in (("leg-back-far", HOCK_Y), ("leg-front-far", CARPUS_Y),
                    ("leg-back-near", HOCK_Y), ("leg-front-near", CARPUS_Y)):
        joint, half = A.leg_joint(text, name, y)
        legs.append((name, joint, half + 0.4, (0.0, 1.0)))
    parser = ET.XMLParser(target=ET.TreeBuilder(insert_comments=True))
    root = ET.fromstring(text, parser=parser)
    paints = A.Paints(root)
    for name, joint, r, axis in legs:
        A.split_leg(root, paints, name, joint, r, axis)
    A.lift_part(root, paints, "head", LIP, "lip", LIP_PIVOT, under=gum())
    # Le lobe de laine de l'avant : le premier tracé du corps, dessiné sous
    # les autres.
    parent, index, body = A._find(root, "body")
    lobe = [c for c in body if A._tag(c) == "path"][0]
    body.remove(lobe)
    neck = ET.Element(Q + "g", {"id": "neck"})
    neck.append(lobe)
    neck.append(A._pivot("neck", NECK))
    parent.insert(index, neck)
    _, _, head = A._find(root, "head")
    for c in list(head):
        if c.get("id") == "pivot-head":
            head.remove(c)
    head.append(A._pivot("head", POLL))
    head.append(A._pivot("muzzle", MUZZLE))
    # Le dégradé de la laine du corps, en coordonnées absolues, pour les mèches.
    defs = root.find(Q + "defs")
    wool = ET.SubElement(defs, Q + "linearGradient", {
        "id": "wool-body", "gradientUnits": "userSpaceOnUse", "x1": "0", "y1": "49", "x2": "0", "y2": "96.6"})
    for offset, color in (("0", "#ffffff"), ("0.55", "#f4f6fa"), ("1", "#dde3ee")):
        ET.SubElement(wool, Q + "stop", {"offset": offset, "stop-color": color})
    _, _, body = A._find(root, "body")
    pivot = [c for c in body if c.get("id") == "pivot-body"]
    for c in pivot:
        body.remove(c)
    for i, (x, y, length, width) in enumerate(TUFTS):
        body.append(tuft(x, y, length, width, f"sway-{i + 1}"))
    for c in pivot:
        body.append(c)
    ET.indent(root, space="  ")
    out = ET.tostring(root, encoding="unicode") + "\n"
    with open(OUT, "w") as f:
        f.write(out)
    print(OUT)


if __name__ == "__main__":
    main()
