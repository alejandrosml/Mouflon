# Romanov

Une brebis adulte de trois ans, début février, environ trois mois et demi
après la tonte : 65 cm au garrot, toison courte gris bleuté, tête noire à
liste frontale courte, oreilles en lancette, sans cornes ni crinière.
L'individu et les proportions viennent de `docs/races/romanov.md`.

Le générateur dessine le profil droit à une unité par centimètre. Sa
graine fixe est 1802 ; les boucles de 6 à 12 mm restent courtes et mates.
`outils.py` reprend les primitives de courbes et d'écriture de la Lacaune,
avec le repère propre au Romanov. Le SVG est la sortie exacte du script.

```sh
python3 tools/races/romanov/gen_romanov.py pets/romanov/romanov.svg
cargo run -p mouflon-preview --release -- pets/romanov/pet.toml apercu/romanov poses
cargo run -p mouflon-preview --release -- pets/romanov/pet.toml apercu/romanov gestes
cargo run -p mouflon-preview --release -- pets/romanov/pet.toml apercu/romanov jeux
```

Python 3 et `shapely` sont nécessaires. Comme pour les deux dernières
races, les pattes sont recoupées au carpe et au jarret par `split_leg`, la
lèvre par `lift_part`, avec son bourrelet dentaire caché dessous. La tête
et le cou ont leurs pivots propres ; la queue courte tient en une pièce.
La toison courte ne comporte aucune mèche pendante.
Le haut de l'épaule et le bord caché de l'encolure partagent un arc autour
de son pivot, pour garder le raccord arrondi quand la tête se pose au sol.
Les remplissages de la toison ont un même sens de parcours : leur
recouvrement reste éclairé sans trou dans le masque du moteur.

Photos de comparaison, consultées hors du dépôt :

- [Tête de près, Salon de l'agriculture](https://commons.wikimedia.org/wiki/File:Mouton_romanov_t%C3%AAte.jpg) : chanfrein, œil et anneau gris, poil ras, oreille ; la lèvre blanche de cette brebis n'est pas retenue pour l'individu choisi.
- [Brebis en toison courte](https://commons.wikimedia.org/wiki/File:Moutons_romanov.jpg) : grain et nappes de gris irrégulières, absence de longues mèches.
- [Queues naturelles](https://commons.wikimedia.org/wiki/File:Moutons_romanov_SIA.jpg) : petites queues laineuses sombres, courtes et effilées.
- [Brebis du zoo de Kyiv](https://zoo.kyiv.ua/wp-content/uploads/2025/04/romanivska-vivczya.jpg) : proportions des membres et port de tête ; sa toison longue crème n'est pas retenue.
- [Géode, organisme de sélection](https://www.geodesheep.com/fr/romanov-425.html) : comparaison du type français et des oreilles.

Les sons sont les huit fichiers du Mannequin, déjà dans le domaine public
(voir `CREDITS.md`), sans cloche. Ils ne constituent pas des enregistrements
propres au Romanov. L'appel se fait bouche fermée pour évoquer les voix
graves décrites par l'INRA ; sa hauteur sonore reste celle du fichier.
La fiche décrit aussi des agneaux noirs et des béliers à crinière : ce rig
représente seulement la brebis adulte. Les dimensions tête/oreilles et les
articulations restent les estimations signalées comme telles dans la fiche.
