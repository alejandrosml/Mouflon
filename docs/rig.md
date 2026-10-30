# Dessiner un personnage pour Mouflon

Un personnage Mouflon est un **dessin SVG découpé en calques nommés**. Le
moteur anime ces calques lui-même : marche au pas (un pied après l'autre, les
sabots posés sans glisser) et galop par bonds, couché (l'avant se plie
d'abord, il se relève par l'arrière), sommeil, broutage la tête au sol,
sauts les pattes repliées, atterrissage, regard, respiration, clignements,
oreilles qui frémissent et queue qui bat de temps en temps. Tu dessines une
seule pose de profil, il fait le reste.

Le vectoriel n'a pas de résolution : dessine à la taille qui te convient, le
moteur le met à l'échelle (`height` dans `pet.toml`) et il reste net sur
n'importe quel écran Retina, même agrandi.

## Règles de base

- Le personnage regarde vers la **droite**.
- L'ordre des calques est l'ordre de dessin : ce qui est en bas de la liste
  est dessiné en premier (derrière).
- Les articulations se chevauchent : le haut des pattes rentre sous le corps,
  le cou sous la tête. Sinon, les rotations ouvrent des trous.
- Pas d'ombre portée ni de reflet « collés » sous le personnage : le moteur
  ajoute une ombre douce au sol et une lumière dynamique sur le corps.

## Calques animés

Chaque partie est un groupe (`<g>`) dont l'`id` est un des noms ci-dessous.
Toutes sont facultatives.

| `id` du groupe | Ce que le moteur en fait | Pivot attendu |
|---|---|---|
| `body` | écrasement, respiration, rebond de la marche, éclairage dynamique | bas du corps |
| `head` | regard, hochement au pas, tête basse en dormant, au sol pour brouter | cou |
| `ear-near`, `ear-far` | battent en tombant, frémissent, tombent en dormant | base de l'oreille |
| `eyes` (dans `head`) | clignements, yeux fermés en dormant | centre des yeux |
| `pupils` (dans `eyes`) | suivent le curseur | — |
| `tail` | balancement, coups de queue | base de la queue |
| `leg-front-near`, `leg-back-near` | pas, galop, sauts, pattes repliées couché | hanche ou épaule |
| `leg-front-far`, `leg-back-far` | idem, décalées dans le cycle | hanche ou épaule |

`near` = côté visible (devant le corps), `far` = côté caché (derrière le corps).

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
sommeil, couché, broute, galop.
Avec `video` à la place de `poses`, tu obtiens les images d'une petite scène
de bureau à 60 i/s (à assembler avec ffmpeg).

Pour l'utiliser dans l'app : menu de la barre des menus → « Ouvrir le dossier
des animaux », copie ton dossier dedans, puis relance Mouflon.
