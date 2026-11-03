# Relecture du dessin de la Solognote

Relecture du 3 novembre 2026, d'après la [fiche de race](../../docs/races/solognote.md)
et le [chantier des dessins](../../docs/chantiers/races-dessins.md). Individu :
une brebis adulte de 65 cm au garrot, en toison de fin d'été. Les images
sont des peintures originales ; les photos n'ont servi que de références,
conservées hors du dépôt.

## Références regardées à côté du dessin

- [P4, profil droit](https://commons.wikimedia.org/wiki/File:Mouton_solognot_(3).JPG) :
  longueur du corps, membres nus, bande ventrale et queue entière.
- [P5, broutage de profil](https://commons.wikimedia.org/wiki/File:Mouton_solognot_(6).JPG) :
  encolure basse, chanfrein, crêtes serrées et limite de la laine.
- [P14, troupeau GEODE](https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot.jpg) :
  port des oreilles et passage du roux de l'encolure au corps bise.
- [P16, tête et gorge GEODE](https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot3.JPG) :
  poil court, œil clair, museau, lèvres et gorge découverte.
- [P17, brebis debout GEODE](https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot4.JPG) :
  hauteur des pattes, port de tête et proportions au repos.

La planche de contrôle rassemble le profil en grand, la tête de près,
l'animal à sa taille de bureau et des poses du moteur. Les planches de
poses des quatre races de référence du dépôt ont aussi été regardées pour
comparer le niveau de détail et les raccords. Les commandes de
régénération sont dans le [README](README.md).

## Les dix caractères contrôlés

| Caractère de la fiche | Choix du dessin |
|---|---|
| Tête, oreilles et pattes rousses et nues | Lavis châtain roux, pattes un peu plus brunes, sans tache ni réserve blanche |
| Toison bise courte et tassée | Fond gris-beige chaud, pigment dans les creux, crêtes courbes et brossage sec |
| Absence de cornes | Aucun morceau de corne ; l'oreille éloignée a été déplacée pour supprimer une pointe qui ressemblait à un cornillon |
| Absence de toupet | Crâne nu ; la laine s'arrête derrière les oreilles |
| Gorge et ventre découverts | Gorge en poil roux et bande ventrale continue sous la frange |
| Longues pattes nues | Avant-bras et canons visibles ; carpe à 25 % du garrot, jarret à 34 % environ |
| Tête longue et étroite | Museau raccourci après comparaison, ganache modelée ; profil presque droit |
| Oreilles moyennes horizontales | Pavillons à bout arrondi, sans pendentif ni boucle d'identification |
| Œil clair | Iris gris pâle, cerne sombre et pupille horizontale indépendante, paupière peinte |
| Encolure rousse et queue entière | Dégradé terre de Sienne vers la laine bise ; queue laineuse sous le jarret, en trois segments |

## Corrections des planches

Six passes du peintre ont notamment corrigé la tête trop longue, l'œil
trop grand, le museau trop gris, les oreilles pointues, le cou trop pâle,
le réseau de laine trop régulier et le fond visible entre les lèvres.
La dernière passe a interrompu et assombri les crêtes du cou pour éviter
l'effet de cordons clairs.

Les planches `poses`, `gestes`, `allures` et `toison` ont été relues dans
les deux sens. Pour voir les mêmes séries vers la gauche, une copie de
travail de l'outil d'aperçu, hors du dépôt, a seulement inversé le sens des
cas dessinés ; aucun changement de l'outil partagé n'est livré.

- **Poses** : tête et gorge nues lisibles, ventre dégagé ; broutage du
  museau au sol devant les antérieurs, coucher sur le sternum et pattes
  repliées ; pas de jour visible aux raccords du cou ou des pattes.
- **Gestes** : bâillement, bêlement, flehmen et paupière regardés de près ;
  l'intérieur de la bouche reste peint, la lèvre ne ressemble pas à une
  marque blanche. Le cabré sert seulement au contrôle : aucune transition
  spontanée n'y mène dans `pet.toml`.
- **Allures** : pas, trot et galop phase par phase, coucher et saut ; les
  carpes et jarrets plient, les hauts recouvrent les bas et les sabots
  restent attachés. La silhouette demeure celle d'une brebis haute sur
  pattes.
- **Toison** : les trois petites mèches du bas suivent le mouvement sans
  emporter la bande de ventre nue ; la queue reste continue dans ses
  balancements.
- **Jeux** : film de huit secondes, avec deux animaux tournés l'un vers
  l'autre, reculs, petits chocs, gambades et demi-tour en l'air ; contrôle
  des raccords et des changements de sens sur des planches de travail.

Le pivot de tête a été remonté à la vraie nuque, au sommet arrière du
crâne : le broutage demandait sinon une rotation dépassant la compensation
prévue pour l'œil. Les tests stricts passent avec ce pivot, sans changer
le moteur ni élargir la tolérance sur la pupille.

## Limites conservées

La laine et le modelé restent une interprétation à l'aquarelle : les
crêtes sont moins irrégulières et moins organiques que celles des photos,
et le tronc reste simplifié. À la taille du bureau, une partie du grain,
des fibres et du détail de l'œil se fond dans la réduction des morceaux.
Le dos s'incline d'un bloc, comme les autres squelettes du moteur.

Le broutage des ligneux, tête haute, n'a pas encore de mouvement dédié.
Les sons sont le répertoire ordinaire du mouton, sans voix spécifique de
Solognote ni cloche. Le rendu Retina, la fluidité et les interactions sur
un vrai Mac restent à vérifier. Le remplacement du nom et du choix
d'Aquarelle dans l'app est réservé à une autre tâche.
