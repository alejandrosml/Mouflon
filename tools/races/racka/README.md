# Racka

Un bélier blanc de cinq ans à la fin de mars, en pleine toison, suivant
[la fiche de la race](../../../docs/races/racka.md). Le dessin regarde vers
la droite ; une unité SVG vaut un centimètre de l'animal.

Depuis la racine du dépôt, avec Python 3 et Shapely (comme les autres
générateurs de races) :

```sh
python3 tools/races/racka/gen_racka.py pets/racka/racka.svg
cargo run --release -p mouflon-preview -- pets/racka/pet.toml apercu/racka poses
cargo run --release -p mouflon-preview -- pets/racka/pet.toml apercu/racka gestes
cargo run --release -p mouflon-preview -- pets/racka/pet.toml apercu/racka jeux
cargo run --release -p mouflon-preview -- pets/racka/pet.toml apercu/racka allures
cargo run --release -p mouflon-preview -- pets/racka/pet.toml apercu/racka toison
cargo test --release -p mouflon-render --test troupeau
```

Le générateur a une graine fixe. Le SVG livré est sa sortie exacte ; les
retouches se font dans le script. `outils.py` reprend et adapte les outils
vectoriels de la Lacaune ; la découpe de la lèvre utilise l'outil commun
`tools/races/articulations.py`. Les autres articulations sont dessinées
directement dans leurs groupes.

## Références regardées

Les photos servent à observer l'animal ; aucune n'est copiée dans le dépôt
ni incorporée au dessin. Les quatre références principales sont celles de
la fiche :

- [P1, jeune bélier de profil](https://commons.wikimedia.org/wiki/File:B%C3%A9lier_Racka.JPG) : charpente, profil des cornes, queue et pattes ; la toison courte ne sert pas de modèle pour mars.
- [P3, adulte à Szentendre, 30 mars 2008](https://www.flickr.com/photos/damka/2377480669/) : toison pleine, mèches, robe et cornes adultes.
- [P4, tête du même bélier](https://www.flickr.com/photos/damka/2382894215/) : lames vrillées, annelures, museau et oreilles.
- [P12, troupeau à Bernburg en mars](https://commons.wikimedia.org/wiki/File:Zackelschafe_Tiergarten_Bernburg_06-03-2008.jpg) : longueurs de mèches, franges, palette et jambes.

## Choix du dessin

Le V des cornes est un V **de face** : de profil, ses deux axes se
superposent presque. Le dessin garde cette projection, avec la corne du
fond légèrement décalée. La torsion est celle d'une lame autour d'un axe
presque droit, avec une arête qui tourne et des annelures en travers.
Les cordelettes de laine tombent verticalement ; les trois nappes mobiles
sous le ventre se chevauchent à leur racine. La queue descend sous le
jarret et porte deux articulations supplémentaires. Tête et antérieurs
restent ras, les cuisses sont laineuses jusqu'au jarret.

La fiche donnait 1,35 fois le garrot pour la pointe des cornes : ce chiffre
est celui de la nuque. La hauteur totale a été corrigée dans la fiche en
additionnant la nuque haute et la projection des cornes, soit environ
1,9 à 2 fois le garrot osseux.

Les réglages partent du Mannequin : vigilance fréquente, fuite courte en
bande, demi-tour pour regarder. Les rencontres gardent une approche lente
et une secousse de tête, sans inventer un combat frontal propre à la race.
Les huit sons sont les copies exactes de ceux du Mannequin, déjà crédités
dans `CREDITS.md` ; aucune sonnaille n'est ajoutée.

## Limites à garder en tête

Le profil strict masque une grande partie de la corne du fond et ne montre
pas l'ouverture du V frontal. Le sens absolu de la vrille et certaines
mensurations restent des estimations, comme le précise la fiche. Les
mèches du cou suivent la pièce d'encolure : elles ne conservent pas chacune
leur verticale pendant le broutage. Les cordelettes et annelures fines se
fondent à la taille habituelle du compagnon ; elles sont surtout lisibles
sur les vues agrandies.
