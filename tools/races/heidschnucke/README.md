# Heidschnucke grise cornue

Une brebis adulte de quatre ans en novembre, cinq mois après la tonte,
d'après [sa fiche](../../../docs/races/heidschnucke.md). Le garrot osseux
mesure 67 cm ; le haut de la toison environ 75 cm. Une unité SVG vaut un
centimètre. Le dessin regarde vers la droite.

## Régénérer

Depuis la racine du dépôt, avec Python 3 et Shapely :

```sh
python3 -m pip install shapely
python3 tools/races/heidschnucke/gen_heidschnucke.py
```

Un chemin de sortie peut être fourni en argument. La graine est fixe ; le
SVG versionné est la sortie exacte du générateur. `outils.py` reprend les
utilitaires vectoriels de Thônes et Marthod, limités à ceux utilisés ici.
Les quatre pattes sont recoupées au carpe (15,5 cm du sol) et au jarret
(23 cm) par `articulations.py`. Le cou est une pièce complète sous le corps ;
la tête, la mâchoire, la lèvre, les oreilles et les yeux sont articulés.
Trois pans noirs de la frange du ventre balancent sous la nappe grise.

La boîte réelle, sans les pivots, mesure 100,7244 cm de haut : `height =
50.36` donne 0,5 point par centimètre. Les pivots de pattes sont à 43 cm du
sol : `stride = 34.4`. Le manifeste reprend les quarante comportements du
Mannequin, avec marche et alerte très contagieuses, fuite précoce devant le
pointeur, jeux adultes rares. Les huit sons sont ceux du Mannequin (domaine
public, provenance déjà indiquée dans `CREDITS.md`), sans sonnaille.

## Références et relecture

Les photographies sont uniquement des références consultées hors dépôt :

- [Profil d'automne qtl1](https://commons.wikimedia.org/wiki/File:Graue_geh%C3%B6rnte_Heidschnucke_qtl1.jpg) : silhouette, mèches, aplombs, queue.
- [Tête de brebis](https://commons.wikimedia.org/wiki/File:Heidschnucke.jpg) : faucille, implantation des cornes, oreilles et œil.
- [Brebis du Hof Tutsberg, GEH](https://www.g-e-h.de/index.php/rassebeschreibungen/72-rassebeschreibungen-schafe/747-graue-geh%C3%B6rnte-heidschnucke) : couleurs et cape noire.
- [Profil en broutant](https://commons.wikimedia.org/wiki/File:Seitenansicht_Graue_geh%C3%B6rnte_Heidschnucke.JPG) : encolure basse et membres.
- [Fiche photographique LfL](https://www.lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_graue_gehoernte_heidschnucke.pdf) : brebis de profil gauche et standard.

Contrôle des dix détails de la fiche : cape noire fondue en diagonale dans
l'épaule ; mèches argentées raides et striées ; pattes nues noires et fines ;
faucilles ouvertes en arrière ; petites oreilles noires ; tête longue sans
toupet ni blanc ; queue noire courte ; frange sombre sous le ventre ; iris
paille à pupille horizontale ; corps profond porté haut sur les membres.
L'individu est une adulte : les agneaux noirs décrits dans la fiche ne sont
pas une seconde variante de ce dessin.

Pour revoir les articulations, écrire les aperçus hors des fichiers livrés :

```sh
cargo run --release -p mouflon-preview -- pets/heidschnucke/pet.toml apercu/heidschnucke poses
cargo run --release -p mouflon-preview -- pets/heidschnucke/pet.toml apercu/heidschnucke gestes
cargo run --release -p mouflon-preview -- pets/heidschnucke/pet.toml apercu/heidschnucke jeux
cargo run --release -p mouflon-preview -- pets/heidschnucke/pet.toml apercu/heidschnucke allures
cargo run --release -p mouflon-preview -- pets/heidschnucke/pet.toml apercu/heidschnucke toison
cargo test --release -p mouflon-render --test troupeau
```

Le mode `planche` de l'aperçu concerne les clips de sprites : pour un rig,
les poses et gestes servent de planches. La relecture porte aussi sur leur
rendu tourné à gauche, sur un dessin agrandi et sur le détail de la tête.

Ce qui reste visible : les mèches sont plus régulières que sur une vraie
toison de novembre et leurs fibres se fondent à la taille normale ; les
mèches du cou tournent avec leur pièce quand la brebis broute. Le moteur
conserve des jeux et gestes communs aux deux sexes (dont le flehmen), même
si le manifeste en réduit les transitions spontanées chez cette adulte.
