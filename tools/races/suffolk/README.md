# Le Suffolk

Le générateur commencé dans le commit `7d13df4` est repris ici. Il dessine
le bélier britannique de trois ans choisi dans
[`docs/races/suffolk.md`](../../../docs/races/suffolk.md) : 85 cm au garrot,
en octobre, avec 3 à 4 cm de laine naturelle, sans teinture ni taille de
concours. Une unité SVG vaut un centimètre.

Depuis la racine du dépôt, avec Python 3 et Shapely :

```sh
python3 tools/races/suffolk/gen_suffolk.py pets/suffolk/suffolk.svg
cargo run -p mouflon-preview --release -- pets/suffolk/pet.toml apercu/suffolk poses
cargo run -p mouflon-preview --release -- pets/suffolk/pet.toml apercu/suffolk gestes
cargo run -p mouflon-preview --release -- pets/suffolk/pet.toml apercu/suffolk allures
cargo run -p mouflon-preview --release -- pets/suffolk/pet.toml apercu/suffolk jeux
cargo test --release -p mouflon-render --test troupeau
```

`gen_suffolk.py` porte les mesures, les contours, les volumes et les
articulations ; `toison.py` peint le réseau de sillons et le grain fin ;
`outils.py` écrit les tracés. Retoucher ces sources et régénérer le SVG,
jamais le SVG seul. Les graines sont fixes. La toison courte ne porte pas
de mèches pendantes `sway-*` ; la queue coupée tient en une seule partie.

## Relecture d'après nature

Photos consultées en dehors du dépôt, à comparer aux planches sans les
copier dans le dépôt :

- [Le jeune bélier de profil dehors, P5 de la fiche](https://commons.wikimedia.org/w/index.php?curid=4257844) :
  tête busquée, oreille tombante, noir froid, col oblique et proportions.
  Sa toison plus longue n'est pas la repousse d'octobre choisie ici.
- [Tête Suffolk de face, P26](https://commons.wikimedia.org/wiki/File:T%C3%AAte_de_brebie_Suffolk.JPG) :
  poil noir court, reflet du ciel, front sans bonnet blanc, œil discret.
- [Bélier de l'association des éleveurs de Basse-Saxe](https://www.schafzucht-niedersachsen.de/Schafzucht-Verbaende-Niedersachsen/images/Inhalte/Zuchtprogramme/Weser-Ems/Suffolk_02neu.jpg) :
  corps profond, gigot plein, dos plat et limites de laine aux membres.
- [Suffolk, Races de France](https://racesdefrance.fr/race/suffolk/) :
  conformation bouchère et laine naturelle ; tête portée plus haut.
- [Troupeau Bentley, Suffolk Sheep Society](https://www.suffolksheep.org/2022/08/bentley-suffolks-focusing-on-results/) :
  finesse des oreilles et séparation nette entre laine et poil noir ;
  brebis et jeunes, à distinguer du bélier adulte dessiné.

La dernière passe a raccourci, assoupli et atténué les sillons de la laine,
densifié son grain, et varié le relief par zones. Les mêmes champs de
texture traversent le corps, le cou et les hauts de pattes : leurs
jointures restent discrètes au repos. Le cadre SVG a été élargi pour
contenir le nez entier dans les lecteurs qui découpent au `viewBox`.

À chaque relecture : tête et membres noirs nus ; col derrière les
oreilles ; oreilles longues et tombantes ; absence de cornes ; tête
longue et busquée ; tronc long et profond ; laine courte serrée ; limites
aux carpes et jarrets ; pattes fortes et onglons noirs ; iris brun et queue
courte attachée haut. Vérifier aussi les raccords quand les articulations
plient, les quatre soles au repos et le museau dans l'herbe.

## Limites du dessin

Le grain reste une interprétation vectorielle, plus régulier que la laine
photographiée. Dans les poses extrêmes de broutage et de sommeil, le cou
et sa texture tournent d'une pièce ; le dos ne se déforme pas. Les jeux
forcés de l'aperçu vérifient les articulations, mais leurs bonds ne sont
pas le comportement habituel d'un bélier de 130 kg : le manifeste les rend
rares. Les sons sont ceux du Mannequin, déjà crédités dans `CREDITS.md`,
sans prétendre enregistrer la voix grave de cet individu.
