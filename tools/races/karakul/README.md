# Karakul

Brebis de quatre ans de la lignée noire, grisonnée, début septembre avant
la tonte d'automne. Le dessin suit [la fiche](../../../docs/races/karakul.md) :
66 cm au garrot osseux, longues mèches gris brun à pointes roussies, tête et
pattes noires, longues oreilles pendantes, absence de cornes, reins hauts,
croupe inclinée et queue grasse naturelle terminée en S.

## Régénérer

Python 3 et `shapely`, comme les autres générateurs de races :

```sh
python3 tools/races/karakul/gen_karakul.py pets/karakul/karakul.svg
```

La graine est fixe. Le SVG livré est la sortie exacte du script ; retoucher
le générateur et le relancer, jamais le SVG directement. `outils.py` reprend
les primitives géométriques de la Lacaune, localement ; la silhouette, la
tête, les mèches et la queue sont propres au Karakul. Les découpes sont
calculées à la génération : aucun masque, filtre, `clipPath`, image ou
texte incorporé au SVG.

Une unité vaut un centimètre. Les bornes visibles mesurées par le moteur
font 115,6835 × 86,2296 cm ; `height = 43.115` et `width = 57.842` donnent
0,5 point par centimètre. Le pivot antérieur est à 45 cm du sol :
`stride = 1,6 × 45 × 0,5 = 36` points. Carpes à 18 cm, jarrets à 22,5 cm.
L'encolure est une pièce sous le corps, les quatre pattes sont recoupées
par `articulations.py`, la lèvre découvre une gencive pigmentée, trois
groupes de mèches accompagnent les allures. Le coussin gras est peint dans
le corps : il suit le bassin et ne se relève pas pendant une secousse.
Seul l'appendice maigre est articulé en trois segments, dont la racine
chevauche le bas du coussin. L'attache anatomique est à 57,5 cm, le bas du
coussin à 34,7 cm, la pointe à 22,8 cm du sol.

## Références regardées

Les photographies servent uniquement à observer la race. Elles ne sont
ni copiées dans le dépôt ni incorporées au dessin.

- [Brebis de profil, Rare Breeds NZ](https://www.rarebreeds.co.nz/karakul.html)
  (`karakul4.jpg`) : proportions, ligne du dos, hauteur des pattes. Sa queue
  écourtée n'est pas un modèle pour la queue naturelle.
- [Brebis grisonnante et bélier avant/après tonte, Leslie Johnson](https://thriftyhomesteader.com/american-karakul-sheep/)
  (`Grey-Ewe.jpg`, `Shorn-Unshorn.jpg`) : mèches, fondu de l'encolure et
  volume caudal, distingué du volume de laine.
- [Troupeau au Turkménistan](https://commons.wikimedia.org/wiki/File:Turkmen_sheep.jpg) :
  couleurs gris brun, mèches ouvertes, tête noire et profil busqué.
- [Oreilles, Karakul Shepherds Alliance](https://karakulshepherds.org/gallery/gallery-of-ears/) :
  longueur, pli, direction pendante ; la photographie du jeune ne sert
  pas de référence pour la toison de l'adulte.

## Comportement et limites

Les gestes du Mannequin sont conservés, avec davantage de marche et de
broutage, une forte contagion de marche, des regards fréquents et une
fuite brève qui conduit au demi-tour puis à l'observation. L'appel est
faiblement contagieux. Les sons sont ceux du Mannequin, domaine public
(voir `CREDITS.md`), sans sonnaille. Le cabré reste disponible pour l'aperçu,
sans transition spontanée vers lui.

La formation d'un cercle protecteur, le broutage des branches et les appels
à heure de repas ne sont pas programmés par cette race : le moteur commun
ne propose pas ces comportements. Les fibres gardent une écriture
vectorielle plus régulière que sur les photographies. Les dimensions du
coussin et de sa pointe reprennent les estimations photographiques de la
fiche ; aucune mensuration publiée de queue Karakul n'y est disponible.

## Contrôler les poses

```sh
cargo run --release -p mouflon-preview -- pets/karakul/pet.toml apercu/karakul poses
cargo run --release -p mouflon-preview -- pets/karakul/pet.toml apercu/karakul gestes
cargo run --release -p mouflon-preview -- pets/karakul/pet.toml apercu/karakul allures
cargo run --release -p mouflon-preview -- pets/karakul/pet.toml apercu/karakul toison
cargo run --release -p mouflon-preview -- pets/karakul/pet.toml apercu/karakul/jeux jeux
cargo test --release -p mouflon-render --test troupeau
```

Regarder aussi les animaux tournés à gauche dans `poses` et dans les jeux.
Revoir particulièrement la pente du cou en broutant, la lèvre du flehmen,
les racines des mèches en secousse et le coussin caudal, qui doit rester
contre les cuisses. Les sorties d'aperçu ne sont pas versionnées.
