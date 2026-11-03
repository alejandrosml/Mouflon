# Dessiner un personnage pour Mouflon

Un personnage Mouflon est un **dessin SVG découpé en calques nommés**. Le
moteur anime ces calques lui-même : marche au pas (un pied après l'autre, les
sabots posés sans glisser), trot par paires diagonales et galop transverse,
couché (l'avant se plie d'abord, il se relève par l'arrière), sommeil (la
tête posée au sol, une patte qui tressaille), broutage la tête au sol,
rumination, sauts les pattes repliées, raides ou en ruant (il se ramasse
avant de bondir),
atterrissage, regard, respiration, clignements, oreilles qui frémissent et
queue qui bat de temps en temps, et une dizaine de gestes (s'ébrouer,
s'étirer, se gratter, bâiller, bêler, taper du pied, le flehmen…). Tu
dessines une seule pose de profil, il fait le reste.

Avec ses **articulations** (voir plus bas), il fait mieux encore : l'encolure
descend pour brouter et pose le museau dans l'herbe, se redresse aux aguets ;
le carpe et le jarret plient en marchant, la patte d'appui reste droite ;
couché, les antérieurs se replient sous le poitrail et les postérieurs le
long du ventre ; la lèvre se retrousse ; la toison traîne et balance.

Le vectoriel n'a pas de résolution : dessine à la taille qui te convient, le
moteur le met à l'échelle (`height` dans `pet.toml`) et il reste net sur
n'importe quel écran Retina, même agrandi.

## Règles de base

- Le personnage regarde vers la **droite**.
- L'ordre des calques est l'ordre de dessin : ce qui est en bas de la liste
  est dessiné en premier (derrière).
- Les articulations se chevauchent : le haut des pattes rentre sous le corps,
  le cou sous la tête. Sinon, les rotations ouvrent des trous.
- L'encolure se dessine dans `neck` (voir « Les articulations ») : elle porte
  la tête et descend avec elle pour brouter (voir `pets/mouflon-corse`, dont
  le collier est l'encolure).
- Pas d'ombre portée ni de reflet « collés » sous le personnage : le moteur
  ajoute une ombre douce au sol et une lumière dynamique sur le corps.

## Calques animés

Chaque partie est un groupe (`<g>`) dont l'`id` est un des noms ci-dessous.
Toutes sont facultatives.

| `id` du groupe | Ce que le moteur en fait | Pivot attendu |
|---|---|---|
| `body` | écrasement, respiration, rebond de la marche, éclairage dynamique | bas du corps |
| `head` | regard, hochement au pas, tête basse en dormant, au sol pour brouter | le cou ; la nuque s'il y a une encolure |
| `ear-near`, `ear-far` | battent en tombant, frémissent, tombent en dormant | base de l'oreille |
| `eyes` (dans `head`) | clignements, yeux fermés en dormant | centre des yeux |
| `pupils` (dans `eyes`) | suivent le regard, et restent à l'horizontale quand la tête penche (la pupille du mouton est une fente horizontale ; son œil tourne d'une cinquantaine de degrés en broutant) : mets-y l'iris et la fente, et les reflets après le groupe | centre de la pupille |
| `jaw` (dans `head`) | s'ouvre pour mâcher, ruminer, bêler, bâiller | la charnière, sous la joue, à hauteur de la bouche |
| `tail` | balancement, coups de queue | base de la queue |
| `tail-mid` (dans `tail`), `tail-tip` (dans `tail-mid`) | une longue queue en morceaux : chacun suit le précédent avec un temps de retard, elle ondule | l'attache de chaque morceau |
| `leg-front-near`, `leg-back-near` | pas, galop, sauts, pattes repliées couché | hanche ou épaule |
| `leg-front-far`, `leg-back-far` | idem, décalées dans le cycle | hanche ou épaule |
| `neck` | l'encolure, qui porte la tête (`head` dedans) : elle descend pour brouter, se redresse aux aguets, se tend quand il fonce, s'allonge vers l'avant quand il dort la tête au sol | la base de l'encolure, devant le garrot, dans l'épaule |
| `leg-front-near-lower`… (dans sa patte) | le bas de chaque patte (le canon et le sabot) : il plie au carpe (devant) ou au jarret (derrière) | le carpe ou le jarret |
| `lip` (dans `head`) | la lèvre supérieure : elle se retrousse pour le flehmen | son arrière, sous le naseau |
| `sway-1`, `sway-2`, `sway-3` (dans `body`, ou juste avant lui) | des mèches de la toison : elles traînent quand il court, balancent au rythme des pas, penchent dans le vent, suivent la secousse quand il s'ébroue | le milieu de leur attache, en haut |

Et un point qui n'est pas une partie : `pivot-muzzle`, ce qui touche l'herbe
quand il broute (le bout du museau, sous les naseaux). Sans lui, le moteur
le devine près du coin du bas et de l'avant de la tête.

`near` = côté visible (devant le corps), `far` = côté caché (derrière le corps).

La mâchoire se dessine **sous** la tête (avant elle dans le groupe `head`) :
le bas du museau, et au-dessus, cachés par la tête tant que la bouche est
fermée, l'intérieur de la bouche (langue en bas) et la joue qui s'étire. Le
tracé de la tête a l'encoche de la mâchoire, sans contour le long de
l'encoche : bouche fermée, on ne voit rien ; ouverte, l'intérieur apparaît
entre les lèvres. Voir le groupe `jaw` de `pets/mannequin/mannequin.svg`.

## Les articulations

Toutes sont facultatives : **un dessin qui ne les a pas bouge exactement
comme avant** (le test `sans_changement` le vérifie à 10⁻⁷ près, et
que ses images cuites restent les mêmes). Le Mannequin
(`pets/mannequin/mannequin.svg`) les a toutes : c'est l'exemple à suivre.

**L'encolure** (`neck`). Deux façons de la ranger, le moteur fait suivre la
tête dans les deux cas :

- `neck` porte `head` (`<g id="neck">…<g id="head">…</g></g>`), à la place
  qu'avait la tête dans le document : l'encolure est dessinée par-dessus le
  corps. C'est le plus simple quand le cou est une pièce à part, au contour
  fini (le collier du mouflon).
- ou bien `neck` avant `body`, **sous** le corps, et `head` à part, après le
  corps comme avant : le corps recouvre la base de l'encolure, comme une
  marionnette découpée. C'est ce qu'il faut quand l'encolure sort de la
  toison (le lobe de laine de l'avant du Mannequin, le Nez noir, le Jacob,
  l'Ouessant).

`pivot-neck` est la base de l'encolure : un point dans l'épaule, assez bas
pour que l'encolure et la tête, mises bout à bout, atteignent le sol devant
les antérieurs (le test `troupeau` le vérifie). `pivot-head` est alors la
**nuque** (derrière les oreilles, à la base des cornes), et non plus la base
du cou : la tête y tourne. Le moteur répartit entre les deux ce que faisait
la tête d'une pièce : pour brouter, une cinématique inverse à deux
articulations pose le museau au sol, à peu près une longueur de tête devant
les antérieurs (moins si l'encolure est courte), et c'est surtout
l'encolure qui descend. Seule
l'encolure s'étire (jusqu'à 12 %) ; la tête reste d'une pièce.

Dessine l'encolure plus longue que ce qui se voit au repos : ce qui sera
découvert quand elle bouge doit exister, caché sous le corps ou la tête
(le Mannequin a un lobe de laine entier ; les races du dépôt ont une
épaisseur de laine cachée au fond de l'encolure).

**Les bas de pattes** (`leg-front-near-lower` dans `leg-front-near`, et de
même pour les trois autres). Le haut de la patte va de l'épaule ou de la
hanche au carpe ou au jarret ; le bas, du carpe ou du jarret au sabot, avec
le sabot. Pour que rien ne s'ouvre au genou :

- le bas se dessine **en premier** dans le groupe de la patte (dessous), et
  son `pivot-…-lower` est au centre de l'articulation ;
- le haut se termine en **demi-disque** autour de ce pivot, un peu plus
  large que la patte, sans trait sur ce bord ; le bas commence par l'arc du
  même cercle (il glisse sous le haut en tournant) ;
- dessine la patte droite ou à peine pliée au repos : le carpe pointe vers
  l'avant (le canon part en arrière en pliant), le jarret vers l'arrière (le
  canon part vers l'avant).

Le moteur pose chaque sabot là où il doit être (posé, il recule exactement
de ce que le corps avance : il ne glisse pas) et plie le genou de lui-même ;
à l'appui, la patte reste droite (elle se tasse de 7 % avant de plier) ; en
vol, le carpe et le jarret plient ; couché, les antérieurs se replient sous
le poitrail (le canon à plat, vers l'arrière) et les postérieurs le long du
ventre (le canon vers l'avant) ; en l'air, les pattes se replient ; à
l'atterrissage, le corps descend et les genoux amortissent ; chaque geste
plie les bonnes articulations (gratter le sol, taper du pied, se cabrer, se
gratter d'un postérieur, s'étirer la patte tendue).

**La lèvre** (`lip`, dans `head`, dessinée par-dessus le reste de la tête) :
la lèvre supérieure, du naseau à la bouche, qui recouvre exactement la tête
au repos. Juste avant elle, dans la tête, dessine ce qu'elle découvre en se
retroussant : la gencive (le mouton n'a pas d'incisives en haut, seulement
un bourrelet rose). `pivot-lip` à son arrière.

**Les mèches** (`sway-1`, `sway-2`, `sway-3`) : des boucles ou des mèches qui
pendent. Le plus sûr est de les prendre dans la toison elle-même (des
festons du bas du ventre, posés juste avant le corps, dessous) : au repos,
rien ne change. Une mèche **penche** sans quitter son attache : le moteur la
cisaille autour de la ligne horizontale de son pivot, qui ne bouge pas, et
seul son bas part vers l'arrière ou vers l'avant. Coupe donc chaque feston
d'un creux du contour au suivant, et pose son pivot au milieu de la corde
entre les deux creux : seuls les creux touchent les voisins, et ils bougent
à peine ; un bord coupé au milieu d'un feston s'ouvrirait en encoche dès
que la mèche penche. Elles sont éclairées comme le corps.

Un exemple, réduit à l'essentiel :

```xml
<g id="leg-front-near">
  <g id="leg-front-near-lower">          <!-- dessous : le canon et le sabot -->
    <path d="…"/>
    <circle id="pivot-leg-front-near-lower" cx="101.2" cy="100.6" r="0.8"/>
  </g>
  <path d="…"/>                          <!-- le haut, fini en demi-disque -->
  <circle id="pivot-leg-front-near" cx="100.5" cy="88" r="0.8"/>
</g>
<g id="neck">                            <!-- sous le corps -->
  <path d="…"/>
  <circle id="pivot-neck" cx="92" cy="66" r="0.8"/>
</g>
<g id="body">…</g>
<g id="head">                            <!-- par-dessus, à part -->
  …
  <path d="…"/>                          <!-- la gencive -->
  <g id="lip"><path d="…"/><circle id="pivot-lip" cx="130.2" cy="62.4" r="0.8"/></g>
  <circle id="pivot-head" cx="103.5" cy="51.5" r="0.8"/>   <!-- la nuque -->
  <circle id="pivot-muzzle" cx="134" cy="71" r="0.8"/>
</g>
```

**Couper un dessin existant.** `tools/races/articulations.py` le fait pour
un SVG généré par un script (voir `tools/races/README.md`) : chaque patte
coupée au carpe et au jarret, l'encolure prise dans le corps devant une
épaule arrondie, la lèvre détachée de la tête avec sa gencive, des festons
détachés en mèches ; au repos, le dessin reste le même, à l'anticrénelage
près.

**Le dos ne plie pas.** Un arrière-train (`rump`) coupé au milieu du dos et
tournant autour de lui a été essayé sur le Mannequin : dès 0,12 rad (7°), le
ventre s'ouvre d'un côté (une boucle coupée en deux, le contour interrompu)
et se double de l'autre. Une toison est d'un seul tenant : pour la plier, il
faudrait déformer le dessin, pas faire tourner des morceaux cuits en
images. Le moteur incline le corps entier (le coucher, le galop, le
cabré) ; il ne le plie pas.

## Pivots

Un pivot est un **petit cercle** (rayon 1, n'importe quelle couleur) dont
l'`id` est `pivot-` suivi du nom de la partie : `pivot-head`,
`pivot-leg-front-near`, etc. Place-le à l'articulation. Il n'est jamais
affiché. S'il manque, le moteur en devine un raisonnable.

Le pivot **`pivot-ground`** est obligatoire en pratique : c'est le point où
le personnage touche le sol, entre ses pattes.

## Ce que le SVG peut contenir

Pris en charge : tracés et formes, remplissages et contours, dégradés
linéaires et radiaux, opacité de groupe.

Le moteur cuit le dessin en images, une par suite de tracés d'une même
partie, et ne pose plus que ces images à l'écran : un dessin de milliers de
tracés ne coûte pas plus cher à animer que le Mannequin (les dessins des
quatre races du dépôt pèsent de 280 à 540 Ko, cuits en moins de vingt
images). Un
groupe semi-transparent empêche cette cuisson (le dessin est alors redessiné
à chaque image) : mets plutôt l'opacité sur chaque tracé.

Pas encore : masques, découpes (`clip-path`), filtres (flou…), motifs,
texte (convertis-le en tracés), images bitmap intégrées.

## Depuis ton logiciel de dessin

- **Figma** : le nom d'un calque devient son `id` si tu coches « Include “id”
  attribute » à l'export SVG.
- **Illustrator** : export SVG avec « Object IDs : Layer Names ».
- **Affinity Designer** : le nom du calque devient l'`id` à l'export SVG.
- **Inkscape** : propriétés de l'objet → champ « ID ».

## Le fichier `pet.toml`

Pose ton SVG et un `pet.toml` dans un dossier, par exemple `mouton/`, en
partant de `pets/mannequin/pet.toml` qui sert de modèle complet :

```toml
[pet]
id = "mouton"
name = "Mouton"
author = "Alejandro San Martin Lamas"

[appearance]
rig = "mouton.svg"
height = 64.0   # hauteur affichée, en points
```

Les comportements (marcher, s'asseoir, dormir, sauter…) se règlent dans le
même fichier, avec leurs durées et leurs enchaînements pondérés.

## Vérifier sans lancer l'app

```sh
cargo run -p mouflon-preview --release -- mouton/pet.toml apercu poses
```

produit `apercu/poses.png` : debout, marche, atterrissage, saut, tenu,
sommeil, couché, broute, galop. Avec `gestes`, les gestes (le flehmen
compris) ; avec `allures`, le pas, le trot et le galop phase par phase, le
coucher et le saut : c'est là qu'on voit plier le carpe et le jarret.
Avec `video` à la place de `poses`, tu obtiens les images d'une petite scène
de bureau à 60 i/s (à assembler avec ffmpeg).

Pour l'utiliser dans l'app : menu de la barre des menus → « Ouvrir le dossier
des animaux », copie ton dossier dedans, puis relance Mouflon.
