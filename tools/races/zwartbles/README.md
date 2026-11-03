# Zwartbles

Une brebis de trois à quatre ans, au début de l'automne, trois à quatre mois
après la tonte. Le dessin suit la [fiche](../../../docs/races/zwartbles.md) :
80 cm au garrot, repousse de 4 à 6 cm, corps long en poire, tête nue sans
cornes, liste continue, quatre balzanes et queue naturelle au bout blanc.

```sh
python3 -m pip install shapely
python3 tools/races/zwartbles/gen_zwartbles.py pets/zwartbles/zwartbles.svg
cargo run -p mouflon-preview --release -- pets/zwartbles/pet.toml apercu/zwartbles poses
cargo run -p mouflon-preview --release -- pets/zwartbles/pet.toml apercu/zwartbles gestes
cargo run -p mouflon-preview --release -- pets/zwartbles/pet.toml apercu/zwartbles jeux
cargo run -p mouflon-preview --release -- pets/zwartbles/pet.toml apercu/zwartbles allures
```

Le générateur et ses outils locaux écrivent tout le SVG ; ne pas retoucher
le SVG à la main. Les graines sont fixes. Une unité vaut un centimètre ;
le sol est à `Y = 112`, la nuque à `(121.5, 16)` dans le SVG. L'encolure,
les quatre pattes en deux segments, la mâchoire, la lèvre, les oreilles et
les trois segments de queue sont articulés dès le dessin. Le corps cache
la racine des trois petits festons du ventre qui oscillent avec la toison.
Les outils de courbes et d'écriture sont adaptés de ceux de la Lacaune,
avec un repère propre à cette race ; il n'y a aucune dépendance entre les
générateurs de races.

Les aplats de toison (corps, encolure et festons) sont peints par des traits
opaques jointifs, avec leurs dégradés et leurs frisures. Le modelé appartient
ainsi au dessin : le masque d'éclairage global du moteur, construit au repos,
ne laisse ni silhouette claire derrière le cou abaissé ni rectangle hors des
images cuites. Il n'y a aucun remplissage dans ces parties, donc pas de masque
commun de lumière pour cette race ; aucun changement du moteur n'est nécessaire.
La queue possède une courte base solidaire du corps, qui conserve son
attache haute quand la croupe tourne, puis trois segments mobiles derrière
le corps, autour d'un pivot enfoui. Leur recouvrement évite un jour au
milieu du coucher ; la partie mobile peut se masquer dans une ruade.

Pour relire chaque planche, le profil agrandi et la tête de près à côté
des photos, vérifier les dix traits de la fiche :

1. Toison brun-noir chaude ; tête et membres plus noirs et plus froids.
2. Liste blanche continue du crâne au nez, au-dessus de l'œil, élargie au
   museau ; joue et gorge noires.
3. Quatre balzanes, plus hautes derrière, toujours sous carpe et jarret.
4. Dernier quart à tiers de la queue blanc.
5. Queue naturelle longue, droite et pendante, articulée en chaîne.
6. Aucune corne, aucun toupet ni laine sur la tête.
7. Corps haut et long en poire, arrière plus profond, croupe un peu plus haute.
8. Long cou porté haut ; le museau atteint l'herbe devant les antérieurs.
9. Tête longue et étroite, chanfrein presque droit, museau fin.
10. Grandes oreilles noires horizontales ; iris ambré à pupille horizontale.

Examiner aussi le miroir vers la gauche, les raccords de l'encolure, du
carpe et du jarret, la lèvre retroussée et les sabots au sol dans les poses
repliées et les jeux. Les planches restent hors des fichiers livrés.
`cargo test --release -p mouflon-render --test troupeau` vérifie aussi le
chargement, la cuisson, les articulations, les sabots et le museau au sol.

Photos utilisées pour relire le dessin, seulement comme références, sans
les incorporer au dépôt :

- [Brebis tondue Alexandrina, profil et queue naturelle](https://commons.wikimedia.org/wiki/File:Alexandrina.jpg).
- [Brebis en toison moyenne, Geograph 1557467](https://www.geograph.org.uk/photo/1557467).
- [Profil de concours, Cheshire Show](https://www.farmersguardian.com/news/4324863/zwarbles-ewe-claims-inter-breed-sheep-title).
- [Greenhill, silhouette et balzanes](https://www.greenhillzwartblessheep.co.uk/pics/).
- [Zwartbles Ireland, tête et oreilles](https://zwartblesireland.com/zwartbles/).
- [Brebis en train de brouter](https://blairdrummond.com/animals/our-animals/).

Le [standard de la Zwartbles Sheep Association](https://zwartbles.org/the-breed/)
permet deux à quatre balzanes ; cet individu en a quatre, les postérieures
plus hautes. Aucun blanc dans la laine, les oreilles, la gorge ni le ventre.
Le blanc de la queue commence à environ 42 cm du sol et reste inférieur au
tiers de sa longueur. La toison chocolat représente une repousse naturelle,
avec des pointes un peu brunies, plutôt qu'un animal taillé pour un concours.

Le profil reste une interprétation : le ventre, la profondeur de la joue et
le relief de laine changent entre individus, et un rig découpé garde une
encolure qui tourne d'un bloc. Le manifeste reprend les quarante comportements
du Mannequin : davantage de pâturage et d'examen curieux, une marche volontiers
suivie par le troupeau, moins de fuite et de combats. Les vitesses suivent la
longueur des pattes. Les huit sons sont identiques octet pour octet à ceux du
Mannequin, déjà crédités dans `CREDITS.md` (domaine public/CC0). Aucune voix
propre au Zwartbles n'est prétendue, aucune sonnaille n'est ajoutée.

© 2026 Alejandro San Martin Lamas — MIT OR Apache-2.0
