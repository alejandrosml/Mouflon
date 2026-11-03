# Skudde

Bélier blanc adulte de quatre à six ans, 58 cm au garrot, en toison
d'automne : longues mèches pointues de poils légèrement ondulés sur un
duvet, crinière au cou et à l'épaule, tête rase mouchetée, cornes ambrées
en colimaçon, petites oreilles horizontales, queue courte en triangle.
Le choix de l'individu et les mesures viennent de
[`docs/races/skudde.md`](../../../docs/races/skudde.md).

Le générateur est déterministe (graine 1941). Une unité du SVG vaut un
centimètre. La bobine proche mesure 20,63 × 26,07 cm hors tout ; sa
section de départ reste à 6,7 cm (fiche § 2.3 et § 6). Les pattes sont
coupées au carpe (15,5 cm) et au jarret
(18,5 cm) ; l'encolure est dessinée sous le corps, la tête tourne à la
nuque ; la lèvre se soulève sur un bourrelet dentaire rose et trois paquets
du bas de la toison balancent. Le SVG est la sortie exacte du script :
pour le retoucher, modifier le générateur puis le relancer.

```sh
python3 -m pip install shapely
python3 tools/races/skudde/gen_skudde.py pets/skudde/skudde.svg
cargo run --release -p mouflon-preview -- pets/skudde/pet.toml apercu/skudde poses
cargo run --release -p mouflon-preview -- pets/skudde/pet.toml apercu/skudde gestes
cargo run --release -p mouflon-preview -- pets/skudde/pet.toml apercu/skudde jeux
cargo run --release -p mouflon-preview -- pets/skudde/pet.toml apercu/skudde allures
cargo run --release -p mouflon-preview -- pets/skudde/pet.toml apercu/skudde toison
cargo test --release -p mouflon-render --test troupeau
```

`outils.py` reprend les outils de courbes, régions et écriture SVG de la
Lacaune, avec le repère adapté à cette race. Il n'ajoute aucune dépendance
à l'application.

Les comportements reprennent ceux du Mannequin, adaptés à la fiche (§ 8) :
vif, farouche, grégaire, alarme puis fuite courte, sauts fréquents et
broutage abondant. Ces fréquences sont une interprétation pour le bureau,
pas des valeurs mesurées pour la race. Les huit sons sont des copies de
ceux du Mannequin (domaine public, déjà crédités dans `CREDITS.md`). Pas de
sonnaille.

## Références pour la relecture

Les photographies sont consultées et gardées uniquement hors du dépôt ;
aucune n'est intégrée au dessin ni livrée avec lui.

- [Bélier blanc de Dortmund, profil droit](https://commons.wikimedia.org/wiki/File:Dortmund_-_Zoo_-_Schaf_03_ies.jpg) :
  cornes, tête et crinière ; sa toison d'hiver est plus longue que celle
  du sujet d'automne retenu.
- [Bélier brun de Detmold, profil gauche](https://commons.wikimedia.org/wiki/File:Detmold_-_2025-09-06_-_Skudde_(DSC06190).jpg) :
  proportions, queue triangulaire, aplombs.
- [Fleckie, ZVSP](https://www.schafzuchtverband.de/wp-content/uploads/2023/08/BA_006__Fleckie.jpg) :
  pigmentation, section et écartement des cornes.
- [Bélier tondu, tête](https://commons.wikimedia.org/wiki/File:Skudde_Bock.jpg) :
  face rase, museau pigmenté, petites oreilles.
- [ProSpecieRara, portrait de race](https://www.prospecierara.ch/tiere/rassenportr%C3%A4ts/schafe/skudde) :
  texture de la toison, cou et épaule, oreilles et cornes.

Le dessin reste une interprétation vectorielle d'un individu : la densité
et les pointes de la toison, le dessin des mouchetures et l'enroulement des
cornes varient entre animaux. La crinière tourne avec le cou rigide du
moteur ; ses poils ne sont pas simulés un par un.
