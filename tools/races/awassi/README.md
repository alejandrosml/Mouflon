# Le bélier Awassi

Un bélier Awassi local de quatre ans, 75 cm au garrot, en juillet, dix
semaines après la tonte. Le choix de l'individu, les proportions, la
palette et les dix caractères viennent de `docs/races/awassi.md`.

Depuis la racine du dépôt, avec Python 3 et Shapely :

```sh
python3 tools/races/awassi/gen_awassi.py pets/awassi/awassi.svg
cargo run -p mouflon-preview --release -- pets/awassi/pet.toml apercu/awassi poses
cargo run -p mouflon-preview --release -- pets/awassi/pet.toml apercu/awassi gestes
cargo run -p mouflon-preview --release -- pets/awassi/pet.toml apercu/awassi jeux
cargo run -p mouflon-preview --release -- pets/awassi/pet.toml apercu/awassi allures
```

Le SVG livré est la sortie exacte du générateur ; les graines fixes
assurent sa reproduction octet pour octet. `outils.py`, adapté de celui de
la Lacaune, garde ce générateur indépendant des autres races. Les formes
sont calculées et dessinées ici : aucune photographie n'est incorporée.
Les découpes des quatre pattes et de la lèvre emploient le module commun
`tools/races/articulations.py`. Le cou est une pièce entière derrière
l'épaule ; leurs bords cachés se relaient pour garder le dessus rond
pendant le broutage. Le scrotum appartient au corps et le suit dans les
sauts et le coucher.

## Les références photographiques relues

Ces photographies ont été consultées et comparées au dessin et aux
planches du moteur, hors du dépôt, comme références anatomiques seulement :

- [Bélier tondu de profil, Bulgarie 02](https://commons.wikimedia.org/wiki/File:Awassi_sheep_Bulgaria_02.jpg) : dos droit, croupe plongeante, pattes, queue collée aux cuisses.
- [Bélier du Koweït](https://commons.wikimedia.org/wiki/File:Kuwaiti_sheep.jpg) : chanfrein, œil, grande volute ouverte et pointe remontant sous la gorge.
- [Tête, Bulgarie 03](https://commons.wikimedia.org/wiki/File:Awassi_sheep_Bulgaria_03.jpg) : section et rides irrégulières des cornes, poil brun et museau.
- [Profil, Bulgarie 08](https://commons.wikimedia.org/wiki/File:Awassi_sheep_Bulgaria_08.jpg) : volume de la queue, membres, poitrine et encolure.
- [Trois quarts, Bulgarie 07](https://commons.wikimedia.org/wiki/File:Awassi_sheep_Bulgaria_07.jpg) : oreilles larges pendantes, queue et arrière lourd.
- FAO, Epstein 1985, [figure 1-15](https://www.fao.org/4/aj003e/AJ003E017.jpg) et [figure 1-16](https://www.fao.org/4/aj003e/AJ003E018.jpg) : deux lobes du coussin et dessous nu.

## Les choix et les limites

La repousse courte laisse lire la croupe à 40° et le coussin de la queue :
28 cm de haut, 10 cm d'épaisseur de profil, bas vers 34 cm, au-dessus des
jarrets à 24 cm. Le petit crochet remonte ; sa houppe retombe sans prolonger
la queue en appendice de Karakul. Le dessous nu est suggéré par un étroit
retour de peau sourde. Le profil ne montre pas les deux lobes ni la
gouttière médiane comme une vue de derrière.

La tête mesure 23 cm ; ses oreilles, presque verticales, 16 × 8,5 cm. La
corne proche projette une hélice en grande volute ouverte, avec une
trentaine de rides majeures et une pointe plus lisse ; la torsion vers le
spectateur reste une approximation de profil. Les marques brunes des
genoux sont mates ; les canons restent clairs entre ces taches et les
chaussettes. La laine ne pend pas à cette saison : il n'y a pas de mèches
`sway-*` artificielles sous le ventre.

Le tempérament conserve tous les gestes du Mannequin : longues marches
lentes et contagieuses, broutage fréquent, longues ruminations, trot et
galop rares. Les sons sont exactement ceux du Mannequin, du domaine
public, déjà crédités dans `CREDITS.md`. Aucune cloche. Le moteur actuel
ne simule ni la tête à l'ombre sous un voisin, ni l'inertie propre d'un
coussin de graisse ; le balancement générique de la queue reste donc plus
ample que celui d'un Awassi réel dans certains gestes. Le grain et les
poils deviennent très fins à la taille de bureau ; la silhouette, les
oreilles, les cornes et la queue y portent l'identité de la race.

L'éclairage actuel du moteur regroupe la silhouette de l'encolure au repos
avec celle du corps, puis la fait suivre au corps. Le cou abaissé peut donc
laisser un léger halo clair au-dessus du garrot, surtout tourné à gauche.
Ce défaut a été isolé en comparant les rendus vectoriel, cuit et sans
éclairage : il disparaît sans éclairage. Sa correction appartient au moteur,
hors de cette race ; le contour anatomique de l'encolure est conservé.
