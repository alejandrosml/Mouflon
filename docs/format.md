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
on_meet = "surprise"          # en rencontrant un autre animal (sinon : réflexe meet)
on_window_side = "turn"       # le côté d'une fenêtre barre le chemin (sinon : réflexe window_side)
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
meet = "greet"                # rencontre un autre animal en chemin (hors mode collisions)
window_side = "turn"          # le côté d'une fenêtre lui barre le chemin
leave = "wave"                # au revoir (« Tout retirer ») : il le joue, puis s'efface
together = "hop"              # « Tous ensemble » : tous les animaux le jouent en même temps
```

Sans `meet`, deux animaux qui se croisent passent l'un devant l'autre ;
sans `window_side`, l'animal passe devant la fenêtre ; sans `leave`, il
s'efface aussitôt.

### Rencontres

Hors mode collisions, un animal qui en atteint un autre en marchant (au
même niveau, en allant vers lui) réagit, et l'autre parfois avec lui, comme
dans desktopPet :

1. s'ils ont des réactions en commun (même nom, poids non nul), les deux
   font la même chose : deux Lainou se saluent ensemble ;
2. sinon, si l'autre joue une réaction que le premier connaît, même de poids
   nul, le premier l'imite une fois sur deux (`{ to = "dort", weight = 0 }` :
   il ne s'endort jamais de lui-même en rencontrant quelqu'un, mais peut
   rejoindre un animal endormi) ;
3. sinon, chacun réagit de son côté, l'autre seulement s'il lui fait face.

Un animal en chemin prend le réflexe `meet` ; au repos (assis, endormi…),
il ne réagit que si son comportement a un `on_meet`. Après une rencontre,
chacun attend un peu avant la suivante.

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
l'animal passe à la suite. `fps = [20, 8]` change de rythme le long de la
séquence : un atterrissage vif au début, plus posé à la fin. `loop_from = 3` fait reprendre les passages
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

## Images intermédiaires

Pour un mouvement plus fluide, un animal en images peut avoir des images
intermédiaires entre deux images qui se suivent. Vers la fin de chaque pas,
Mouflon montre celles qui mènent à l'image suivante : une pose longue reste
tenue, puis l'animal change de pose en douceur (80 ms au plus par image
intermédiaire). Elles sont décrites par un fichier à part :

```toml
[appearance]
inbetweens = "entre-deux.toml"
```

L'outil `tools/inbetween` les calcule et écrit ce fichier (avec le réseau
RIFE, qui estime le mouvement entre deux images ; il laisse sans image
intermédiaire les paires trop différentes, comme un demi-tour de profil à
face). Pour des bandes, chaque bande a la sienne, avec `count` images après
chacune de ses images (après la dernière, vers la première) :

```toml
count = 1
strips_left = "entre-deux/gauche/{name}.png"
strips_right = "entre-deux/droite/{name}.png"

[counts]
tombe = 3                     # une chute rapide : trois images après chaque image

[skip]
demi_tour = [0, 1, 2]         # pas d'image intermédiaire après ces images
```

Pour une planche unique (un animal desktopPet redessiné, par exemple), une
planche à part, de mêmes cases, et la liste des paires d'images :

```toml
sprites = "entre-deux.png"
tiles = [12, 23]
# [image de départ, image d'arrivée, première case, nombre de cases]
pairs = [
    [1, 0, 0, 3],
    [0, 1, 3, 3],
]
```

Une bande sans la sienne, ou une paire absente de la liste, reste telle
qu'elle est dessinée.

## Sons

Les sons d'un animal ont un nom ; chacun est un fichier audio (MP3, AAC, WAV),
ou plusieurs prises dont l'une est tirée au hasard à chaque fois, avec un
volume de 0 à 1. Un comportement fait entendre un son au début, ou sur une
image de sa séquence à chaque passage, selon une chance de 0 à 1 :

```toml
[sounds]
bele = { files = ["sons/agneau-1.mp3", "sons/agneau-2.mp3"], volume = 0.8 }
clochette = { files = ["sons/clochette-1.mp3", "sons/clochette-2.mp3"], volume = 0.5 }
broute = "sons/broute.mp3"

[[behavior]]
name = "bele"
sound = "bele"                # au début du comportement
fps = 10

[[behavior]]
name = "course"
sound = [{ name = "clochette", frame = 2, chance = 0.15 }]   # sur la troisième image, une fois sur sept environ
fps = 14
```

Les sons se coupent dans le menu (« Sons »).

## Animaux desktopPet

Mouflon lit directement les fichiers `animations.xml` de
[desktopPet](https://github.com/Adrianotiger/desktopPet) : dépose le fichier,
ou tout le dossier de l'animal, dans le dossier des animaux.

Tout le moteur d'animation d'origine est repris : nombre de pas et
répétitions (y compris tirées au hasard), déplacements interpolés,
demi-tours `flip`, réactions aux bords (`border`, avec les restrictions
`only`, dont `windowSide` pour le côté d'une fenêtre et `petSide` pour la
rencontre d'un autre animal), gravité, points d'apparition, personnages
secondaires (`child` : la fleur que mange le mouton, le bain, le mouton
noir…), variantes de couleur (`skins`), expressions (`screenW`, `areaH`,
`imageW`, `random`, `randS`, `Convert(…, System.Int32)`…) et animations
réservées (`drag`, `fall`, `toss`, `fall soft`, `fall hard`, `kill` pour
l'au revoir, `sync` pour « Tous ensemble »).

Ce que Mouflon fait mieux :

- le déplacement de chaque pas est réparti sur sa durée : l'animal glisse à
  120 Hz au lieu d'avancer par à-coups ;
- un lancer suit une vraie trajectoire (élan, rebond léger), et l'animal se
  pose tourné vers où il allait ;
- sans réaction prévue au bout d'une fenêtre, il ne tombe que lorsque son
  centre a dépassé le bord, pas dès que le bord de son image le dépasse ;
- une fenêtre fermée sous un animal le fait tomber tout de suite ;
- il se promène d'un écran à l'autre.

Les sons (`<sounds>`, en MP3) sont joués au début de leur animation, selon
leur probabilité ; la répétition (`<loop>`) n'est pas encore reprise. En
mode collisions, Mouflon fait culbuter les animaux qui se rentrent dedans
au lieu de jouer leurs rencontres `petSide`.

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

Le `pet.toml` peut ajouter des sons aux animations du fichier, par leur
nom, et des images intermédiaires (`inbetweens` dans `[appearance]`) :

```toml
[sounds]
bele = { files = ["sons/mouton-1.mp3", "sons/mouton-2.mp3"], volume = 0.7 }
broute = "sons/broute.mp3"

[desktoppet.sounds]
eat = "broute"
drag = { name = "bele", chance = 0.5 }
```
