# Les générateurs des quatre races

Les dessins des quatre races du dépôt (`pets/mouflon-corse`, `pets/nez-noir`,
`pets/jacob`, `pets/ouessant`) ne sont pas faits à la main : chacun est écrit
par un script Python, à partir de la fiche de sa race (mesures, couleurs,
marques ; voir [docs/animation-animale.md](../../docs/animation-animale.md#10-quatre-races-daprès-nature)).
Le SVG du dépôt est ce que le script écrit, octet pour octet : pour retoucher
un dessin (une corne plus épaisse, une tache déplacée), on change le script et
on le relance, plutôt que de modifier le SVG.

```sh
pip install shapely      # pour tous les générateurs (la découpe aux articulations)
python3 tools/races/mouflon-corse/gen_mouflon.py pets/mouflon-corse/mouflon-corse.svg
python3 tools/races/nez-noir/gen_nez_noir.py pets/nez-noir/nez-noir.svg
python3 tools/races/jacob/gen_jacob.py pets/jacob/jacob.svg
python3 tools/races/ouessant/gen_ouessant.py pets/ouessant/ouessant.svg
python3 tools/races/mannequin/articule.py      # le Mannequin, redécoupé (voir plus bas)
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
- Les articulations (voir [docs/rig.md](../../docs/rig.md)) : chaque
  générateur dessine ses pattes d'une pièce et son encolure dans le corps,
  puis appelle `articulations.articulate_breed` avant d'écrire le SVG. Cet
  outil commun coupe chaque patte au carpe et au jarret (à la hauteur
  donnée : la tache du genou, la pointe du jarret), et prend l'encolure dans
  le corps : ce qui est devant une épaule arrondie (`articulations.shoulder`,
  un arc du garrot au poitrail), posé sous le corps, qui la recouvre à sa
  base ; la tête, restée par-dessus, tourne désormais à la nuque
  (`poll`), et `muzzle` dit ce qui touche l'herbe. Chaque coupe suit un
  cercle autour du pivot (rien ne s'ouvre quand l'articulation tourne) ; au
  repos, le dessin ne change pas, à l'anticrénelage près. Une nouvelle race
  fait de même : `carpus_y`, `hock_y`, et la zone de l'encolure. Place la
  base de l'encolure (`pivot`) assez bas dans l'épaule pour que l'encolure
  et la tête atteignent l'herbe devant les antérieurs (le test `troupeau`
  le vérifie) ; `filler` ajoute au fond de l'encolure une épaisseur de
  laine, cachée au repos, qui comble l'espace quand la tête descend.
- La lèvre : chaque générateur donne, dans le repère de sa tête, la zone de
  la lèvre supérieure (sous la narine, du bout du nez vers la commissure,
  jusque sous la ligne de la bouche ; environ 15 % de la longueur de la
  tête, sinon, retroussée, elle monterait jusqu'au chanfrein) et la gencive
  qu'elle cache (`articulations.gum`, bien au-dessus de la ligne de la
  bouche pour rester cachée au repos). `articulations.lift_part` la détache
  de la tête, groupes sans nom compris ; elle laisse de côté ce que la
  lèvre recouvre entièrement (le fond clair du crâne sous le masque noir
  laisserait un liseré), et un dégradé en partie transparent reste entier
  dans la tête là où il est opaque (le museau blanc du mouflon reste blanc
  sous la lèvre retroussée).
- Les mèches : le Nez noir marque les mèches du bas de chaque colonne qui
  pendent sous le ventre, entre les pattes (`class="frange-1"`…), et
  `articulations.hang_strands` les coupe au bas du fond de la toison : le
  bout balance (`sway-1` à `sway-3`), le haut reste. L'Ouessant range ses
  mèches pendantes du ventre dans trois groupes `sway-*`, accrochés à leur
  racine (le pivot, la racine moyenne).
- Le mouflon a son cou à part (le collier) : il devient l'encolure (`neck`),
  qui porte la tête ; la corne et l'oreille du fond, peintes derrière le
  collier, sont rognées de ce qu'il cachait (`articulations.clip_away`).
- `tools/races/mannequin/articule.py` tire le Mannequin redécoupé
  (`pets/mannequin/mannequin.svg`) de sa version d'avant les articulations,
  gardée pour le test `sans_changement` : son lobe de laine de l'avant
  devient l'encolure, trois festons du ventre deviennent des mèches
  (`articulations.cut_under`), chacun pris d'un creux du contour au suivant
  (une mèche penche sans quitter son attache, voir docs/rig.md), la lèvre
  est détachée de la tête avec la gencive qu'elle cache
  (`articulations.lift_part`).
- Seuls des tracés, des dégradés et l'opacité de chaque tracé : pas de
  groupe semi-transparent, de masque, de découpe ni de filtre (voir
  [docs/rig.md](../../docs/rig.md)).
