# Landes de Bretagne

Le générateur dessine la brebis choisie dans
[la fiche de race](../../../docs/races/landes-de-bretagne.md) : une adulte
blanche de trois à cinq ans, sans cornes, de 57 cm au garrot, en pleine
toison de mai. Le dessin est original et vectoriel ; les photos servent
seulement à vérifier l'anatomie et la matière. Aucune n'est dans le dépôt.

Depuis la racine du dépôt, avec Python 3 et `shapely` :

```sh
python3 tools/races/landes-de-bretagne/gen_landes_de_bretagne.py pets/landes-de-bretagne/landes-de-bretagne.svg
cargo run --release -p mouflon-preview -- pets/landes-de-bretagne/pet.toml apercu/landes poses
cargo run --release -p mouflon-preview -- pets/landes-de-bretagne/pet.toml apercu/landes gestes
cargo run --release -p mouflon-preview -- pets/landes-de-bretagne/pet.toml apercu/landes jeux
cargo run --release -p mouflon-preview -- pets/landes-de-bretagne/pet.toml apercu/landes allures
cargo run --release -p mouflon-preview -- pets/landes-de-bretagne/pet.toml apercu/landes toison
```

Une unité du SVG vaut un centimètre. Les graines sont fixes : relancer le
script produit les mêmes octets. `outils.py` reprend les fonctions locales
mises au point pour la Lacaune ; le dessin, ses mesures et ses mèches sont
propres aux Landes de Bretagne. Le script commun `articulations.py` coupe
les quatre pattes au carpe (14,5 cm) et au jarret (18,5 cm), et détache la
lèvre supérieure. L'encolure est dessinée sous le corps, avec un volume
caché à sa base. Trois nappes du ventre bougent indépendamment et la queue
est une chaîne de trois morceaux, jusqu'à la petite crosse du bout.

La tête reste nue : pas de toupet sur le chanfrein, ni de laine sur les
joues. Le front et le nez sont rectilignes ; le fond blanc domine les
mouchetures rousses. Les oreilles fines restent proches de l'horizontale,
leur intérieur est rose, les pupilles sont horizontales et l'iris ambré.
La laine crème forme des mèches distinctes, grossières, aux pointes beige,
avec des jarres ; les membres restent nus sous le coude et la mi-jambe.
La queue est laineuse et naturelle. Les petites boucles jaunes n'inventent
aucun numéro d'animal. Pas de cloche.

Les photographies étudiées (hors dépôt) :

- [Écomusée de la Bintinais, présentation et photographies d'Hervé Ronné](https://www.ecomusee-rennes-metropole.fr/le-mouton-des-landes/) : volume de la brebis blanche, transition entre cou laineux et tête nue, oreilles et membres.
- [CRAPAL, mouton des Landes de Bretagne](https://crapal.fr/races-animales/ovin/mouton-landes-de-bretagne/) et sa [photo du troupeau littoral](https://crapal.fr/wp-content/uploads/2024/11/Mouton-Lande-De-Bretagne.jpg) : nez rose marbré, iris, taches du carpe, queue et proportions.
- [R. Fresneau, les différentes robes et types de laine](https://landesdebretagne.canalblog.com/archives/2013/01/02/23143551.html), notamment [le profil noir](https://storage.canalblog.com/38/60/905772/69919496_o.jpg) : proportions du corps en pleine toison, longues mèches et pattes visibles sous la laine.
- [Wikimedia Commons, tête de profil au Salon de l'agriculture 2011](https://commons.wikimedia.org/wiki/File:Landes_de_bretagne_SDA2011.JPG) : tête fine, face blanche mouchetée, petites oreilles, poil ras.
- [Wikimedia Commons, brebis à l'Écomusée du pays de Rennes](https://commons.wikimedia.org/wiki/File:%C3%89comus%C3%A9e_du_pays_de_Rennes,_mouton_des_landes_de_Bretagne.jpg) : laine en mèches, longue queue laineuse et port du cou en broutant.

Les proportions fines restent des estimations de la fiche, pas des mesures
vétérinaires. La robe et l'implantation des petites taches représentent un
individu plausible parmi les nombreux types de la race. La laine du cou
pivote d'un seul tenant avec l'encolure ; les mèches du flanc ne sont pas
simulées une à une. Le même profil est reflété pour marcher vers la gauche,
et la crosse est rendue visible dans le plan du dessin alors qu'elle se
recourbe aussi de côté chez les animaux photographiés.

À très fort agrandissement, les grandes mèches restent plus régulières et
facettées que les nappes de laine photographiées, et la crosse conserve un
peu de géométrie. Les petites taches et les fibres se fondent à la taille
habituelle du compagnon ; elles se lisent dans les vues agrandies.

L'aperçu vectoriel peut laisser un faible reflet au-dessus du dos quand
le cou descend : la silhouette d'éclairage du moteur comprend encore le
cou au repos. Ce reflet n'apparaît pas dans les images cuites utilisées
par l'app, vérifiées aussi en broutage. Le moteur commun reste inchangé.
