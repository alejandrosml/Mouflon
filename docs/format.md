# Le format `pet.toml`

Un animal Mouflon tient dans un dossier : un `pet.toml` et son dessin.
`pets/mannequin/pet.toml` est un exemple complet.

## Identité et apparence

```toml
[pet]
id = "mouton"                 # identifiant unique
name = "Mouton"               # nom affiché dans le menu
author = "Alejandro San Martin Lamas"

[appearance]
rig = "mouton.svg"            # dessin vectoriel découpé en calques (voir rig.md)
height = 64.0                 # hauteur affichée, en points
```

Une planche de sprites façon desktopPet marche aussi :

```toml
[appearance]
sprites = "planche.png"
tiles = [16, 11]              # colonnes, lignes
key_color = "#ff00ff"         # couleur rendue transparente
```

## Physique (facultatif)

Toutes les valeurs sont en points et en secondes.

```toml
[physics]
gravity = 2200.0              # pt/s²
terminal_velocity = 1700.0
bounce = 0.22                 # rebond à l'atterrissage
hard_landing_speed = 1150.0   # au-delà : réflexe land_hard
width = 64.0                  # largeur utilisée pour les bords et les murs
```

## Comportements

Chaque `[[behavior]]` a un nom, un mouvement, une durée et des suites possibles.

```toml
[[behavior]]
name = "walk"
motion = "walk"               # voir la liste ci-dessous
speed = 46.0                  # pt/s, pour walk et run
duration = [3.0, 9.0]         # au hasard entre 3 et 9 s ; ou un nombre ; ou "forever"
next = [
    { to = "stand", weight = 4 },
    { to = "sit", weight = 1 },
    { to = "sleep", weight = 6, when = "night" },
]
on_edge = "turn"              # au bord d'une fenêtre (sinon : réflexe edge)
on_wall = "turn"              # contre le bord de l'écran (sinon : réflexe wall)
on_fall = "fall"              # si le sol disparaît (sinon : réflexe fall)
```

Mouvements : `stand`, `walk`, `run`, `turn` (demi-tour), `sit`, `sleep`,
`look` (regarde autour de lui), `jump` (avec `jump = [vers l'avant, vers le haut]`
en pt/s), `fall`, `land` (récupération après atterrissage), `held` (tenu par le curseur).

Les poids sont relatifs : `weight = 4` contre `weight = 1` donne quatre
chances sur cinq. Une transition dont la condition est fausse est ignorée.

Conditions (`when`, une seule ou une liste) : `on_window`, `on_floor`,
`night` (22 h – 6 h), `day`, `cursor_near:220`, `cursor_far:220` (rayon en points).

## Réflexes

Ce qui se passe lors des événements physiques :

```toml
[reflexes]
spawn = "fall"                # à l'apparition, en haut de l'écran
fall = "fall"                 # le sol disparaît
land = "land"                 # atterrissage
land_hard = "land_hard"       # atterrissage violent
held = "held"                 # attrapé par le curseur
thrown = "fall"               # relâché
edge = [{ to = "turn", weight = 5 }, { to = "hop_down", weight = 1 }]
wall = "turn"
poke = "hop"                  # simple clic
bump = "tumble"               # heurté par un autre animal (mode collisions) ; sinon : thrown
```

## Listes de suites nommées

Une liste qui revient souvent se nomme une fois dans `[choices]`, puis
s'emploie comme un comportement : `next = "envie"`.

```toml
[choices]
envie = [
    { to = "marche", weight = 6 },
    { to = "repos", weight = 3 },
    { to = "baille", weight = 4, when = "night" },
]
```

## Animaux en images

Un personnage dessiné image par image a une bande d'images par animation,
dans chaque sens : `planches/gauche/marche.png`, `planches/droite/marche.png`…
Chaque comportement joue la bande qui porte son nom et avance du déplacement
prévu à chaque image : les pattes ne glissent jamais. `pets/lainou` en est un
exemple complet.

```toml
[appearance]
strips_left = "planches/gauche/{name}.png"    # {name} : nom de l'animation
strips_right = "planches/droite/{name}.png"   # facultatif : sinon, miroir des images de gauche
frame = [256, 256]            # taille d'une image dans les bandes (pixels)
scale = 0.375                 # points par pixel : 256 px font 96 points
anchor = [128, 226]           # milieu des pattes, posé sur le sol (pixels)
hold = [163, 102]             # point par lequel on l'attrape (image tournée à gauche)
drawn = ["shadow", "dust", "squash"]   # effets déjà dessinés : Mouflon ne les ajoute pas

[physics]
width = 78.0                  # largeur du corps : il se retourne quand son nez touche le bord
height = 55.0                 # hauteur du corps au-dessus du point d'appui

[[behavior]]
name = "marche"
fps = 10                      # images par seconde
loops = [3, 6]                # passages : un nombre, ou [min, max] au hasard
move = [5.5, 0]               # pixels par image : vers l'avant, vers le bas
on_wall = "demi_tour"
next = "envie"

[[behavior]]
name = "demi_tour"
fps = 12
turns = true                  # les images montrent le demi-tour : il regarde de l'autre côté à la fin
next = "marche"

[[behavior]]
name = "se_cogne"
fps = 12
move = [[0, 0], [0, 0], [-5, 0], [-7, 0], [-4, 0], [-1.5, 0], [0, 0], [0, 0], [0, 0]]  # un déplacement par image
next = "demi_tour"

[[behavior]]
name = "aventure"
strip = "marche"              # rejoue une autre bande
frames = [0, 1, 2, 3]         # images choisies dans la bande (toutes par défaut)
move = [5.5, 0]
walk_off_edges = true         # au bout d'une fenêtre, il continue et saute dans le vide

[[behavior]]
name = "tombe"
motion = "fall"               # chute physique (lancer, sol qui disparaît) ; images en boucle
fps = 10
duration = "forever"
```

Sans `motion`, un comportement avance image par image ; avec `motion =
"fall"` ou `"held"`, ses images tournent en boucle pendant la chute ou
pendant qu'on le tient. La séquence dure ses images fois ses passages, puis
l'animal passe à la suite. `loop_from = 3` fait reprendre les passages
suivants à la quatrième image (une entrée, puis une boucle).

Un animal en images vérifie à chaque pas qu'il a encore un appui (sinon il
tombe, réflexe `fall`). Sans réaction prévue au bord de l'écran ou d'une
fenêtre, il s'arrête et se retourne ; avec `walk_off_edges = true`, il
continue et tombe du rebord.

Effets de `drawn` : `shadow` (l'ombre au sol est dans les images ; Mouflon
dessine la sienne seulement en l'air, sur ce qui est en dessous), `dust`
(poussière de course et d'atterrissage), `squash` (écrasement et étirement),
`roll` (les images de chute font déjà tourner l'animal sur lui-même).

Une planche unique marche aussi : `sprites = "planche.png"`, `tiles =
[colonnes, lignes]`, `facing = "right"` si les images regardent vers la
droite, et `frames = [numéros des cases]` dans chaque comportement.

## Animaux desktopPet

Mouflon lit directement les fichiers `animations.xml` de
[desktopPet](https://github.com/Adrianotiger/desktopPet) : dépose le fichier,
ou tout le dossier de l'animal, dans le dossier des animaux.

Tout le moteur d'animation d'origine est repris : nombre de pas et
répétitions (y compris tirées au hasard), déplacements interpolés,
demi-tours `flip`, réactions aux bords (`border`, avec les restrictions
`only`), gravité, points d'apparition, personnages secondaires (`child` :
la fleur que mange le mouton, le bain, le mouton noir…), variantes de
couleur (`skins`), expressions (`screenW`, `areaH`, `imageW`, `random`,
`randS`, `Convert(…, System.Int32)`…) et animations réservées (`drag`,
`fall`, `toss`, `fall soft`, `fall hard`).

Ce que Mouflon fait mieux :

- le déplacement de chaque pas est réparti sur sa durée : l'animal glisse à
  120 Hz au lieu d'avancer par à-coups ;
- un lancer suit une vraie trajectoire (élan, rebond léger), et l'animal se
  pose tourné vers où il allait ;
- sans réaction prévue au bout d'une fenêtre, il ne tombe que lorsque son
  centre a dépassé le bord, pas dès que le bord de son image le dépasse ;
- une fenêtre fermée sous un animal le fait tomber tout de suite ;
- il se promène d'un écran à l'autre.

Pas encore repris : les sons, les réactions propres aux rencontres entre
animaux (`petSide` ; le mode collisions de Mouflon les fait culbuter), les
côtés des fenêtres comme obstacles (`windowSide`), `kill` et `sync`.

### Redessiner un animal desktopPet

Pour garder les comportements d'origine avec une planche redessinée en
haute définition, place à côté du fichier XML un `pet.toml` :

```toml
[pet]
id = "mouton"
name = "Mouton"
author = "Alejandro San Martin Lamas"

[appearance]
sprites = "mouton-4k.png"     # même disposition que la planche d'origine
tiles = [16, 11]              # colonnes, lignes (16 × 11 pour eSheep 64)
scale = 0.25                  # cases de 160 px affichées à 40 points

[desktoppet]
animations = "animations.xml"
```

La planche doit garder la disposition d'origine (une case par image, dans le
même ordre) ; sa transparence est celle du PNG. La taille affichée d'une case
vaut sa taille en pixels multipliée par `scale` : à la taille d'origine
(40 points pour eSheep 64), une planche quatre fois plus fine se lisse et
reste nette sur les écrans Retina. Plus grand que l'original, l'animal se
déplace d'autant plus vite : ses pas restent en proportion.

Sans `[appearance]`, le `pet.toml` ne fait que renommer l'animal et garde
l'image du fichier XML.

Le fichier XML peut aussi venir sans son image (balise `<png>` vide) : il
faut alors dire pour quelle taille de case ses déplacements sont réglés.

```toml
[desktoppet]
animations = "animations.xml"
tile = [40, 40]               # pixels d'une case de la planche d'origine
```

C'est le cas de Pelote (`pets/pelote`, réglé pour des cases de 80 pixels) et
de Poete, la mascotte (réglé pour les cases de 40 pixels d'eSheep 64).
