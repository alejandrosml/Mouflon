# Gotland : le dessin de la brebis

`gen_gotland.py` écrit `pets/gotland/gotland.svg`. Python 3 et `shapely`
suffisent (comme les autres générateurs) ; `outils.py` reprend les outils
vectoriels de la Lacaune, avec le repère propre à cette race.

```sh
python3 tools/races/gotland/gen_gotland.py pets/gotland/gotland.svg
cargo run -p mouflon-preview --release -- pets/gotland/pet.toml apercu/gotland poses
cargo run -p mouflon-preview --release -- pets/gotland/pet.toml apercu/gotland gestes
cargo run -p mouflon-preview --release -- pets/gotland/pet.toml apercu/gotland jeux
cargo run -p mouflon-preview --release -- pets/gotland/pet.toml apercu/gotland allures
cargo run -p mouflon-preview --release -- pets/gotland/pet.toml apercu/gotland toison
```

La fiche [de la race](../../../docs/races/gotland.md) décrit l'individu :
brebis adulte, 70 cm au garrot sous la laine, en pleine toison d'automne.
Une unité du SVG est un centimètre. Les mesures non publiées restent les
estimations de la fiche ; la silhouette n'est pas une mesure vétérinaire.
Les tirages sont à graine fixe. Le SVG livré est la sortie du script,
jamais une version retouchée à la main.

Les boucles sont des petites mèches pleines modelées, portant un ruban
enroulé et des éclats partiels dédoublés : plus petites sur le cou et le
dos, plus larges sur le flanc, avec une lumière froide sur le dessus et
des creux sombres sous le ventre. Les trois festons sous le corps bougent
avec la toison ; ils restent courts, sans frange de laine longue. Le cou,
les quatre bas de pattes, la mâchoire, les pupilles horizontales et la lèvre
suivent le contrat du rig. La queue naturelle est courte, à pointe poilue.

Les comportements partent de ceux du Mannequin : brebis vive, curieuse,
familière, peu fuyante, grégaire en marche. Les sons sont ses huit sons CC0
copiés à l'identique, déjà crédités dans `CREDITS.md`, sans sonnaille.

## Références regardées

Photos consultées uniquement hors du dépôt, téléchargées avec le
User-Agent générique `MouflonResearch/1.0`. Elles servent à relire les
proportions, les matières et les poses ; aucune n'est un élément du SVG.

- [Peter Hasselbom, portrait de 08106 de profil](https://www.flickr.com/photos/peter_hasselbom/6130438032/) :
  chanfrein presque droit, œil haut et en arrière, joue profonde, museau fin,
  poil noir satiné et petites vrilles noires contre la nuque.
- [GSBANA, brebis du standard](https://gsbana.org/breed-standards/) :
  face et oreilles noires, œil ambre, contraste avec l'argent de la toison.
- [Bittersweet Farm, Gotland de profil en hiver](https://www.bittersweetfarm.ca/gotland-sheep) :
  anneaux et vrilles serrés, volume des mèches, corps long et profond,
  extrémités noires nues et ossature fine. L'éclairage neigeux est plus bleu
  que celui retenu pour le dessin.
- [Appletree Farm, mouton en marche de profil](https://appletreefarmgotlands.com/gotland-sheep/) :
  ligne du dos, port du cou et des oreilles, proportion des membres.
- [Ashford Heights Farm, Gotland au pâturage](https://www.ashfordheightsfarm.com/our-animals) :
  boucles superposées de taille inégale, ventre profond, oreilles attentives.

Le [standard suédois](https://www.gotlandsfar.se/17/31/breed-standards/)
confirme les signes retenus : gris clair à foncé, tête et membres noirs,
sans laine, sans cornes, queue courte à pointe sans laine. Le Gotland
ne doit pas être confondu avec le Gute cornu.

## Limites du dessin et du comportement

À la taille normale du compagnon (0,5 point par centimètre), les fibres et
les petites spires se fondent ; le contraste tête noire/corps gris, les
aplombs et les oreilles portent alors l'essentiel de la lecture. Le rig
est constitué de morceaux rigides : les mèches du cou tournent avec lui et
la toison ne se déforme pas comme une peau réelle lors des poses extrêmes.
Quand le cou descend brouter, le devant du garrot garde une petite marche
arrondie, au lieu de se fondre dans une peau qui se tend. Les raccords se
recouvrent, mais cette rigidité reste visible.

Sur un fond sombre ou coloré, le broutage et le sommeil révèlent aussi un
halo pâle au-dessus de l'épaule : l'éclairage partagé garde la silhouette
de l'encolure au repos, alors que celle-ci a tourné. Il apparaît dans les
images cuites comme dans les tracés vectoriels (contrôle du scénario
`jeux`, image 239). La silhouette de lumière est assemblée dans
`crates/mouflon-render/src/rig.rs`, puis appliquée par `paint.rs` et
`bake.rs`. Corriger ce défaut demande une modification du moteur partagé,
hors du périmètre de cette race.

À très grand agrandissement, certains paquets de mèches restent trop
lisses et donnent encore un aspect un peu minéral. Les familles de
torsades, anneaux et crochets, leurs sens inversés et leurs fibres fines
évitent un motif uniforme ; elles ne reproduisent pas toute l'irrégularité
et la transparence d'une vraie toison. Les longueurs cachées du cou sont
une construction pour l'animation, et la queue est peu visible sous la
croupe dans plusieurs poses.

Le format actuel ne règle pas l'espacement du troupeau selon l'activité,
ni une approche volontaire vers le pointeur. Les poids des transitions,
durées et vitesses sont des choix d'animation, pas des mesures de la race.
Le cabré du moteur garde une gestuelle de jeu/combat : même avec du mâchage,
il ne reproduit pas exactement la brebis appuyée à un buisson pour brouter
en hauteur.
