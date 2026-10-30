#!/bin/bash
# Le même essai de trois minutes, mais avec une seule fenêtre transparente
# par écran où se dessinent tous les animaux, au lieu d'une fenêtre par
# animal qu'on déplace à chaque image : de quoi comparer les deux façons
# d'afficher. Les résultats vont dans essais/<date>-une-fenetre/.
ici="$(cd "$(dirname "$0")" && pwd)"
MOUFLON_ESSAI_SCENE=1 exec "$ici/Essai de 3 minutes.command"
