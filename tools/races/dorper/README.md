# Dorper : un bélier après la mue

Ce dessin original suit [la fiche](../../../docs/races/dorper.md) : un
bélier sud-africain adulte, de trois à quatre ans, environ 105 kg, en fin
d'été après sa mue naturelle. Il regarde vers la droite. Une unité SVG
vaut un centimètre ; son garrot est à 70 cm. La lumière est neutre : les
reflets orangés du sable de la photographie du Kalahari sont écartés.

## Régénérer

Python 3 et `shapely` sont nécessaires, comme pour les dernières races.
Depuis la racine du dépôt :

```sh
python3 tools/races/dorper/gen_dorper.py pets/dorper/dorper.svg
cargo run -p mouflon-preview --release -- pets/dorper/pet.toml apercu/dorper poses
cargo run -p mouflon-preview --release -- pets/dorper/pet.toml apercu/dorper gestes
cargo run -p mouflon-preview --release -- pets/dorper/pet.toml apercu/dorper jeux
cargo run -p mouflon-preview --release -- pets/dorper/pet.toml apercu/dorper allures
```

Le script ne lit aucune image et n'accède pas au réseau. La graine 1950
fixe le grain du poil ; les formes, proportions et couleurs sont
construites. Le SVG livré est sa sortie exacte. `outils.py` reprend les
outils géométriques du générateur de la Lacaune ; les formes du Dorper sont
propres à ce générateur.

## Choix de dessin

- Un coffre profond et long : garrot 70 cm, sternum 31 cm, ventre 24 cm ;
  dos droit, croupe large, gigots pleins jusqu'au jarret. Les canons sont
  courts, blancs et à poil ras ; les onglons sont noirs et fendus.
- Une tête forte, longue de 24 cm, au front convexe et au large museau
  plissé. L'œil ambre porte une pupille horizontale indépendante de la
  rotation de la tête. Les paupières sont gris bleuté, sans tour roux.
- Un cou noir à limite nette, presque verticale, en avant de l'épaule ;
  le dernier tiers de la crête reste blanc, avec des plis transversaux.
  Pas de noir sur le corps ni les canons.
- Les oreilles sont larges, noires, légèrement tombantes et raccourcies
  par la perspective. Le petit moignon corné est lisse, brun gris,
  dirigé vers le haut et l'arrière. La queue est le court moignon du
  sujet choisi ; elle n'est pas grasse.
- Poil d'été court, modelé doux et stries très fines suivant les muscles ;
  ni crinière, ni franges, ni mèches animées. Le scrotum pigmenté suit
  le corps et ne descend pas au-dessous du jarret.
- L'encolure, cachée sous le corps à sa base, porte la tête dans le moteur.
  Les quatre pattes sont coupées au carpe (19,5 cm) ou au jarret (21,5 cm)
  par `articulations.split_leg` avec recouvrement circulaire. La mâchoire,
  la lèvre supérieure et le bourrelet dentaire sont dessinés séparément.
  La queue est posée sous le corps, avec une réserve étroite dans la
  croupe pour garder son attache quand le coffre pivote pendant les bonds.

## Références regardées

Les photographies servent à vérifier le dessin, elles ne sont jamais
copiées dans le dépôt ni incorporées au SVG :

- [Dorper ram in the Kalahari — South Africa](https://commons.wikimedia.org/wiki/File:Dorper_ram_in_the_Kalahari_-_South_Africa_.jpg) :
  proportions du sujet, masse musculaire, plis blancs du cou, limite du
  noir et petit moignon corné. La patte tirée en arrière et le reflet du
  sable ne sont pas repris.
- [Open Show Res Ch. Dorper Ram, 2017](https://commons.wikimedia.org/wiki/File:Open_Show_Res_Ch._Dorper_Ram_(23726476118).jpg) :
  aplombs, cou noir, poitrine, queue courte, onglons pigmentés.
- [Open Show Champion Dorper Ram, 2017](https://commons.wikimedia.org/wiki/File:Open_Show_Champion_Dorper_Ram_(36909052393).jpg) :
  museau masculin, silhouette, insertion du cou et musculature.
- [Detalhes da cabeça da Ovelha Dorper — Sítio Caramonã](https://commons.wikimedia.org/wiki/File:Detalhes_da_cabe%C3%A7a_da_Ovelha_Dorper_-_S%C3%ADtio_Caramon%C3%A3.jpg) :
  iris, pupille, paupières, grain du poil noir, épaisseur du bord d'oreille.
  La finesse de cette tête de brebis et ses reflets roux ne sont pas repris.

Les droits du dessin original sont ceux du dépôt (MIT ou Apache-2.0).
Les sons sont les mêmes découpes du domaine public que le Mannequin,
déjà décrites dans `CREDITS.md`. Le Dorper n'a pas de sonnaille.

## Limites

La hauteur exacte d'un bélier de concours de 105 kg et les angles de ses
articulations restent des estimations, explicitées dans la fiche. Le
profil unique ne représente ni la largeur du coffre vue de face, ni la
variation du port des oreilles entre individus. Les plis du cou et le
scrotum suivent des pièces rigides ; le moteur ne simule pas leur peau.
La queue reste attachée grâce à sa réserve sous la croupe ; dans les bonds
fortement inclinés, le moignon peut paraître allongé de quelques centimètres,
car le moteur translate la queue sans lui donner toute l'inclinaison du
coffre. Le poil court et les plis fins se fondent à la taille habituelle du bureau.
Ce sujet représente l'état après la mue, pas les plaques de laine du
printemps ni le White Dorper.

L'éclairage commun du moteur laisse encore une très légère forme claire
au-dessus du raccord de l'encolure quand le Dorper broute ou dort la tête
au sol. Elle se voit dans l'aperçu vectoriel et reste, plus discrète, dans
le rendu cuit. Le moteur rassemble les silhouettes du corps et du cou au
repos pour leur éclairage, puis déplace le cou séparément. La géométrie
des remplissages et l'épaisseur des plis ont été vérifiées : aucun tracé
ne dépasse de l'encolure. Corriger cet artefact demande une modification
du rendu commun, hors du périmètre de cette race.
