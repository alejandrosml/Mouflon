# Bleu du Maine

Une brebis adulte de trois à quatre ans, en toison d'automne, d'après
[`docs/races/bleu-du-maine.md`](../../../docs/races/bleu-du-maine.md).
Le dessin est original, sous MIT ou Apache-2.0, comme le dépôt ; les
photographies ont servi à observer les formes, jamais à fournir des pixels
ou des tracés. Une unité du SVG vaut un centimètre de l'animal.

## Refaire le dessin

Avec Python 3 et Shapely (la dépendance des générateurs de races) :

```sh
python3 tools/races/bleu-du-maine/gen_bleu_du_maine.py pets/bleu-du-maine/bleu-du-maine.svg
python3 tools/races/bleu-du-maine/verifie.py
```

Retoucher le générateur, puis le relancer : le SVG livré doit être sa sortie
exacte. Les graines fixes rendent la toison et le poil reproductibles.
`outils.py` reprend les outils de la Lacaune, avec le repère de ce dessin.
Le découpage des articulations utilise `tools/races/articulations.py`.

## Voir et vérifier

```sh
cargo run --release -p mouflon-preview -- pets/bleu-du-maine/pet.toml apercu/bleu-du-maine poses
cargo run --release -p mouflon-preview -- pets/bleu-du-maine/pet.toml apercu/bleu-du-maine gestes
cargo run --release -p mouflon-preview -- pets/bleu-du-maine/pet.toml apercu/bleu-du-maine jeux
cargo run --release -p mouflon-preview -- pets/bleu-du-maine/pet.toml apercu/bleu-du-maine allures
cargo run --release -p mouflon-preview -- pets/bleu-du-maine/pet.toml apercu/bleu-du-maine toison
cargo test --release -p mouflon-render --test troupeau
```

Les tests `troupeau` découvrent le nouveau dossier sans modifier une liste :
ils contrôlent le chargement, la simulation, la cuisson, les parties,
l'appui des sabots et le museau au sol en broutant. `verifie.py` compare
les comportements, réflexes et chaque branche conditionnelle au Mannequin,
et les sons octet pour octet. Les poids restent libres pour suivre la race.
La description du fichier distingue les démonstrations du moteur (jeux,
charge, flehmen) des observations sur une brebis adulte docile.

## Références photographiques

Les photographies restent **hors du dépôt**, y compris les planches qui les
juxtaposent au dessin. Les téléchargements utilisent le User-Agent générique
`MouflonResearch/1.0`. Références regardées :

- [OS Bleu du Maine : standard et photographies](https://bleudumaine.org/race/).
- [Brebis broutant, OS](https://bleudumaine.org/wp-content/uploads/2024/07/BleuDuMaine-003.jpg) : masse de la toison, angle du cou et pattes fines.
- [Brebis en toison, OS](https://bleudumaine.org/wp-content/uploads/2025/01/IMG-20250116-WA0015-473x1024.jpg) : profondeur du corps, frange, queue entière et tête ardoise.
- [Tête et antérieurs, OS](https://bleudumaine.org/wp-content/uploads/2025/01/IMG-20250116-WA0016-473x1024.jpg) : peau veloutée, rides, oreilles sombres, limite de laine et genoux nus.
- [Brebis couchée, OS](https://bleudumaine.org/wp-content/uploads/2025/01/IMG-20250116-WA0029-1024x900.jpg) : silhouette en coussin, pattes cachées sous le corps.
- [Fiche LfL, deux profils](https://lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_blauk_fleischschaf.pdf) : proportions, limite de la laine, articulations et tête nue.
- [Antenaise de concours, Scottish Farmer](https://www.thescottishfarmer.co.uk/resources/images/8440770.jpg) : modelé de la tête et finesse des membres ; sa laine teinte en ocre n'est pas une référence de couleur.

La queue entière rejoint le voisinage du jarret : l'axe articulé mesure
environ 43 cm depuis l'attache noyée dans la croupe, dont environ 31 cm
visibles. La fiche donne à la fois une longueur de 27–31 cm et une pointe
près du jarret ; ces estimations ne permettent pas de respecter les deux
depuis le haut de la croupe. Le dessin privilégie la silhouette des photos
françaises, avec la partie haute cachée dans la toison.

Le grain de laine reste une interprétation vectorielle en petits paquets,
et se fond à la taille normale du compagnon. L'encolure tourne d'une pièce
et le dos reste rigide, comme pour les autres races avec le moteur actuel.

Une limite du moteur a été isolée en broutant vers la gauche : un reflet
pâle conserve une partie de l'emplacement du cou au repos. Il apparaît dans
l'aperçu vectoriel, disparaît sans l'éclairage dynamique, et reste plus
petit au raccord dans le rendu cuit. La silhouette d'éclairage commune
inclut l'encolure au repos (`rig.rs`), puis l'éclairage vectoriel la suit
avec le corps ; la cuisson peut aussi éclairer des pixels transparents
dans les bornes du morceau. Le SVG seul ne contient pas ce reflet. Ce
défaut d'éclairage est à corriger dans le moteur, hors de cette tâche qui
ne modifie que les fichiers de la race.

## Sons

Les huit fichiers de `pets/bleu-du-maine/sons/` sont les copies exactes de
ceux du Mannequin, déjà crédités dans `CREDITS.md` : Joseph SARDIN,
[BigSoundBank](https://bigsoundbank.com), CC0 1.0 (bêlements n° 2343–2346,
broutage n° 0990 et 0497, sauts dans l'herbe n° 1839–1840). Aucune cloche.

Cette tâche ajoute les fichiers de la race. Le remplacement du Mannequin
dans l'application et la mise à jour groupée des listes et crédits restent
au chantier d'intégration des quatre nouveaux dessins.
