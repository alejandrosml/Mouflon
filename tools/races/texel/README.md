# Texel

Un bélier néerlandais de trois ans né sur l'île de Texel, au début de
septembre : 72 cm au garrot osseux, 77 cm avec sa toison de fin d'été.
Une unité SVG vaut un centimètre. La fiche est dans
[`docs/races/texel.md`](../../../docs/races/texel.md).

Depuis la racine du dépôt, avec Python 3 et `shapely` installé :

```sh
python3 tools/races/texel/gen_texel.py pets/texel/texel.svg
cargo run --release -p mouflon-preview -- pets/texel/pet.toml apercu/texel poses
cargo run --release -p mouflon-preview -- pets/texel/pet.toml apercu/texel gestes
cargo run --release -p mouflon-preview -- pets/texel/pet.toml apercu/texel jeux
cargo run --release -p mouflon-preview -- pets/texel/pet.toml apercu/texel allures
cargo test --release -p mouflon-render --test troupeau
```

La graine `20150907` est la date de la photo du sujet. La sortie est
reproductible, sans téléchargement ni ressource externe. `outils.py`
reprend les outils géométriques de la Lacaune, dans le repère de ce dessin.
Les coupes sont faites par `tools/races/articulations.py` : quatre pattes
en deux segments, cou sous le corps, tête à la nuque, lèvre supérieure
mobile et bourrelet dentaire caché. La mâchoire et chaque oreille sont
indépendantes ; l'iris ambré porte une pupille horizontale. Aucune mèche
`sway` : cette toison courte et dense ne pend pas. Aucune sonnaille.

Les huit sons sont ceux du Mannequin, domaine public, déjà crédités dans
`CREDITS.md`. Les comportements reprennent ses quarante états, avec un
pâturage prolongé, des pas posés, peu de contagion et peu de jeux spontanés
pour ce bélier adulte calme et peu grégaire. Les paramètres de comportement
sont une interprétation de la fiche, pas des mesures propres à la race.

## Références regardées

Les photographies servent seulement à comparer silhouette, tête, toison,
pattes et sabots. Elles restent hors du dépôt, ne sont pas incorporées au
SVG et ne sont pas nécessaires au générateur.

- [Bélier texel, île de Texel, 7 septembre 2015](https://commons.wikimedia.org/wiki/File:B%C3%A9lier_texel.jpg) :
  sujet choisi, retourné en miroir pour le profil droit ; ni boucle
  d'identification, ni irritation sous l'œil, ni excroissance cornée.
- [Texel sheep, concours flamand](https://commons.wikimedia.org/wiki/File:Texel_sheep.jpg) :
  aplombs, hauteur du carpe et du jarret, gigot et onglons sur sol dur ;
  l'éclairage orangé de la salle n'est pas repris.
- [Texel ram near Friths Farm](https://commons.wikimedia.org/wiki/File:Texel_ram_near_Friths_Farm_-_geograph.org.uk_-_2031009.jpg) :
  port naturel de la tête et cou court, masse rectangulaire.
- [Texel Ram by Clapper Lane](https://commons.wikimedia.org/wiki/File:Texel_Ram_by_Clapper_Lane_-_geograph.org.uk_-_4358250.jpg) :
  œil, truffe, intérieur des oreilles et direction du poil ras.

Le dessin conserve les dix caractères de la fiche : tête blanche nue et
courte, nez noir et carré, chanfrein droit, petites oreilles, œil ambré,
absence de cornes, cou épais en capuche, dos plat et poitrine profonde,
gigots descendus, pattes blanches sèches à onglons noirs. La queue fine
retombe au repos ; sa position levée dans la photo principale n'est pas
recopiée.

## Limites du dessin

La laine est une interprétation vectorielle des creux et du grain : elle
ne reproduit pas chaque plaque du sujet photographié. Le moteur anime
des pièces rigides : le gros gigot et les plis de l'encolure ne se
déforment pas comme des tissus vivants ; le raccord de teinte entre le
cou et l'épaule reste perceptible dans certaines poses extrêmes.
Le profil opposé est un miroir,
sans reconstruction en trois dimensions. À la taille du bureau, les
fibres et les plis du museau se fondent ; la silhouette, l'œil et le nez
restent les principaux repères.
