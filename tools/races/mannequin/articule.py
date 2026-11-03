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
- trois festons du bas de la toison détachés en mèches (`sway-1` à
  `sway-3`), posés sous le corps : au repos rien ne change, en mouvement la
  frange du ventre balance ;
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
# Les festons du bas du ventre qui deviennent des mèches, de l'avant vers
# l'arrière : chacun pris d'un creux du contour au suivant (la corde entre
# les deux creux est son attache, son milieu le pivot). Coupée là, une mèche
# qui penche ne laisse aucun bord coupé à la vue : seuls ses creux touchent
# ses voisines, et ils bougent à peine.
LOCKS = [((90.44, 95.09), (79.48, 95.6)), ((79.48, 95.6), (62.18, 96.54)),
         ((62.18, 96.54), (52.03, 94.54))]


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
    for i, ((xa, ya), (xb, yb)) in enumerate(LOCKS):
        area = [(xa, ya), (xb, yb), (xb, 130.0), (xa, 130.0)]
        pivot = (round((xa + xb) / 2, 2), round((ya + yb) / 2, 2))
        A.cut_under(root, paints, "body", area, f"sway-{i + 1}", pivot, margin=1.5)
    ET.indent(root, space="  ")
    out = ET.tostring(root, encoding="unicode") + "\n"
    with open(OUT, "w") as f:
        f.write(out)
    print(OUT)


if __name__ == "__main__":
    main()
