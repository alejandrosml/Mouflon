#!/usr/bin/env python3
"""Vérifie que la Bleu du Maine reste un exemple complet du format natif.

À lancer depuis n'importe quel dossier avec Python 3.11 ou plus récent :
    python3 tools/races/bleu-du-maine/verifie.py

Les poids et durées suivent la race ; les gestes et les choix conditionnels
du Mannequin doivent rester présents. Ce contrôle complète les tests du
moteur et la relecture des planches : il ne juge pas le dessin.
"""

from __future__ import annotations

import argparse
from collections import Counter
import math
from pathlib import Path
import tomllib


TRANSITIONS = (
    "next", "on_land", "on_edge", "on_wall", "on_meet", "on_fall",
    "on_window_side",
)


def exiger(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def choix(value: str | list[dict] | None) -> Counter:
    """Une condition seule et une liste d'une condition sont équivalentes."""
    if value is None:
        return Counter()
    if isinstance(value, str):
        return Counter({(value, ()): 1})
    resultat = Counter()
    for item in value:
        condition = item.get("when", [])
        if isinstance(condition, str):
            condition = [condition]
        resultat[(item["to"], tuple(sorted(condition)))] += 1
        poids = item.get("weight", 1)
        exiger(
            isinstance(poids, (int, float)) and math.isfinite(poids) and poids > 0,
            f"Poids invalide pour {item['to']} : {poids}",
        )
    return resultat


def verifier(pet: Path, mannequin: Path) -> tuple[int, int, int]:
    nouveau = tomllib.loads(pet.read_text())
    modele = tomllib.loads(mannequin.read_text())
    exiger(modele.keys() <= nouveau.keys(), "Une section du modèle manque")
    exiger(nouveau["pet"]["id"] == "bleu-du-maine", "Identifiant de race incorrect")
    exiger(nouveau["pet"]["category"] == "sheep", "Famille de race incorrecte")
    exiger(len(nouveau["pet"]["description"]) < 230, "Description trop longue")
    for section, fields in (("appearance", ("height",)),
                            ("physics", ("width", "stride"))):
        for field in fields:
            exiger(nouveau[section][field] > 0, f"{section}.{field} doit être positif")

    comportements = {b["name"]: b for b in nouveau["behavior"]}
    reference = {b["name"]: b for b in modele["behavior"]}
    exiger(len(comportements) == len(nouveau["behavior"]), "Nom de comportement répété")
    exiger(reference.keys() <= comportements.keys(), "Un comportement du modèle manque")

    def transitions(value, base, contexte):
        presents = choix(value)
        exiger(not (choix(base) - presents), f"Choix du modèle manquant : {contexte}")
        for target, _ in presents:
            exiger(target in comportements, f"Cible inconnue : {contexte} → {target}")

    for name, comportement in comportements.items():
        base = reference.get(name, {})
        exiger(base.keys() <= comportement.keys(), f"Clé du modèle manquante : {name}")
        if base:
            exiger(comportement["motion"] == base["motion"], f"Mouvement changé : {name}")
        for field in TRANSITIONS:
            transitions(comportement.get(field), base.get(field), f"{name}.{field}")

    exiger(modele["reflexes"].keys() <= nouveau["reflexes"].keys(), "Un réflexe manque")
    for name, value in nouveau["reflexes"].items():
        transitions(value, modele["reflexes"].get(name), f"reflexes.{name}")

    exiger(nouveau["sounds"].keys() == modele["sounds"].keys(), "Banque de sons modifiée")
    fichiers = set()
    for name, sound in modele["sounds"].items():
        exiger(nouveau["sounds"][name] == sound, f"Déclaration du son modifiée : {name}")
        for filename in sound["files"]:
            exiger((pet.parent / filename).read_bytes() ==
                   (mannequin.parent / filename).read_bytes(),
                   f"Son différent de l'original du domaine public : {filename}")
            fichiers.add(filename)
    return len(comportements), len(nouveau["reflexes"]), len(fichiers)


def main() -> None:
    racine = Path(__file__).resolve().parents[3]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pet", type=Path,
                        default=racine / "pets/bleu-du-maine/pet.toml")
    parser.add_argument("--mannequin", type=Path,
                        default=racine / "pets/mannequin/pet.toml")
    args = parser.parse_args()
    try:
        comportements, reflexes, sons = verifier(args.pet, args.mannequin)
    except (KeyError, OSError, TypeError, ValueError) as error:
        parser.exit(1, f"Échec : {error}\n")
    print(f"Format complet : {comportements} comportements, {reflexes} réflexes, "
          f"{sons} sons identiques ; tous les choix du Mannequin sont présents.")


if __name__ == "__main__":
    main()
