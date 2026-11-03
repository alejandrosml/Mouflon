# Les générateurs des quatre races

Les dessins des quatre races du dépôt (`pets/mouflon-corse`, `pets/nez-noir`,
`pets/jacob`, `pets/ouessant`) ne sont pas faits à la main : chacun est écrit
par un script Python, à partir de la fiche de sa race (mesures, couleurs,
marques ; voir [docs/animation-animale.md](../../docs/animation-animale.md#10-quatre-races-daprès-nature)).
Le SVG du dépôt est ce que le script écrit, octet pour octet : pour retoucher
un dessin (une corne plus épaisse, une tache déplacée), on change le script et
on le relance, plutôt que de modifier le SVG.

```sh
python3 tools/races/mouflon-corse/gen_mouflon.py pets/mouflon-corse/mouflon-corse.svg
python3 tools/races/nez-noir/gen_nez_noir.py pets/nez-noir/nez-noir.svg
python3 tools/races/jacob/gen_jacob.py pets/jacob/jacob.svg      # il faut shapely (pip install shapely)
python3 tools/races/ouessant/gen_ouessant.py pets/ouessant/ouessant.svg
```

Ensuite, regarder le résultat dans le moteur (voir le README) :

```sh
cargo run -p mouflon-preview --release -- pets/jacob/pet.toml apercu poses
cargo run -p mouflon-preview --release -- pets/jacob/pet.toml apercu gestes
cargo test --release -p mouflon-render --test troupeau
```

Le test `troupeau` vérifie que chaque mouton dessiné a toutes ses parties, se
cuit en images et pose ses sabots sur le sol.

Ce qu'il faut savoir :

- Une unité du dessin vaut un centimètre de l'animal ; `height` dans
  `pet.toml` affiche le dessin à 0,5 pt par centimètre (voir le commentaire
  de chaque `pet.toml`).
- Les tirages au hasard (mèches, boucles, poils) ont une graine fixe : le même
  script donne toujours le même dessin.
- Le mouflon met son encolure dans la tête (`pivot-head` à la base du cou),
  pour qu'elle la suive quand il broute ; les autres la gardent dans le corps.
- Seuls des tracés, des dégradés et l'opacité de chaque tracé : pas de
  groupe semi-transparent, de masque, de découpe ni de filtre (voir
  [docs/rig.md](../../docs/rig.md)).
