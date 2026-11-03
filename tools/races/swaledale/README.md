# Le Swaledale

Le générateur commencé dans le commit `7d13df4` est conservé et terminé.
Il suit [la fiche de la race](../../../docs/races/swaledale.md) : bélier de
deux à trois ans en octobre, toison d'automne de 5 à 8 cm, profil droit.
Une unité du SVG vaut un centimètre ; les tailles restent des estimations
signalées dans la fiche.

Avec Python 3 et `shapely` :

```sh
python3 tools/races/swaledale/gen_swaledale.py pets/swaledale/swaledale.svg
```

Les graines sont fixes. Le SVG livré est la sortie exacte de cette commande ;
pour le retoucher, modifier le générateur puis le relancer.

Pour revoir le dessin dans le moteur, depuis la racine du dépôt :

```sh
cargo run --release -p mouflon-preview -- pets/swaledale/pet.toml apercu/swaledale poses
cargo run --release -p mouflon-preview -- pets/swaledale/pet.toml apercu/swaledale gestes
cargo run --release -p mouflon-preview -- pets/swaledale/pet.toml apercu/swaledale allures
cargo run --release -p mouflon-preview -- pets/swaledale/pet.toml apercu/swaledale toison
cargo run --release -p mouflon-preview -- pets/swaledale/pet.toml apercu/swaledale-jeux jeux
cargo test --release -p mouflon-render --test troupeau
```

Les images de contrôle et les photos restent hors du dépôt. Les poses
incluent le profil gauche ; la scène de jeux montre les deux sens.

La dernière passe reprend la toison : paquets irréguliers qui se chevauchent,
modelé local, sillons interrompus et petites boucles, quelques fentes plus
longues sur l'épaule et le col. Les brins restent fins ; les marques du bas
des pattes restent mouchetées, sans chaussettes. Le front porte un poil ras,
avec un raccord court à la racine de la corne, sans toupet. La petite pointe
de la corne éloignée reparaît devant le chanfrein.

Références photographiques de la SSBA, consultées hors du dépôt :

- [p1, profil et proportions](https://swaledale-sheep.com/wp-content/themes/swaledale/img/breed-information-base.jpg) ;
- [p3, pattes et onglons](https://swaledale-sheep.com/wp-content/uploads/2019/11/hutch998.jpg) ;
- [p4, profil d'un jeune bélier](https://swaledale-sheep.com/wp-content/uploads/2022/01/A-District-Champion.jpg) ;
- [p5, tête, laine et cornes](https://swaledale-sheep.com/wp-content/uploads/2022/05/Champion.jpg) ;
- [p7, oreille dans la spirale](https://swaledale-sheep.com/wp-content/uploads/2022/01/E-District-Champ.jpg).

Les quatre pattes sont coupées au carpe ou au jarret par l'outil commun ;
encolure, mâchoire, lèvre et queue en trois segments sont articulées. Cette
toison courte n'a pas de longues mèches pendantes sous le ventre : elle ne
reçoit pas de parties `sway-*`.

La laine garde une interprétation graphique : les paquets et les sillons
sont moins irréguliers qu'une vraie toison. Le profil seul simplifie la
profondeur des cornes, des oreilles et des deux doigts de chaque pied.
En position couchée, le moteur abaisse la longue queue avec le corps sans
la poser sur le sol : son bout peut le traverser. On conserve sa longueur
naturelle, qui fait partie des dix traits de la race.

Le manifeste reprend les quarante comportements du Mannequin, avec
davantage de vigilance, de pâture et de rumination. Les rencontres du bélier
passent par le recul et la charge ; le cabré reste défini pour l'aperçu des
gestes, sans transition entrante. Le moteur ne simule pas l'attachement au
quartier de lande (« hefting »). Les huit sons sont ceux du Mannequin, dans
le domaine public, déjà documentés dans `CREDITS.md` ; pas de sonnaille.
