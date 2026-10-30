# Un animal peint, sur un squelette

Un animal à squelette est fait de **morceaux d'images** (le corps, la tête,
chaque patte, chaque oreille, la queue, l'œil…) posés sur des **os**. Le
moteur les anime lui-même : il marche au pas et galope (un pied après
l'autre, les sabots posés sans glisser), se couche, dort, broute la tête au
sol, saute les pattes repliées, se balance quand on le tient, regarde le
curseur, cligne des yeux, respire, bat des oreilles et de la queue. Tu
peins une seule pose de profil, il fait le reste ; des animations à clés
peuvent s'y ajouter ou le remplacer.

C'est la façon de faire pour des animaux **peints** : à la main, dans
Blender avec Grease Pencil, dans Krita, Procreate ou Photoshop, ou à partir
d'une image générée par une IA, découpée en morceaux. Le dessin garde sa
texture, ses dégradés et ses coups de pinceau : ce sont des images. Il
reste léger : une douzaine de morceaux, réduits à la finesse de l'écran au
chargement, que le GPU pose directement.

`pets/aquarelle` est un exemple complet : une brebis peinte à l'aquarelle
(ses morceaux sont peints par `tools/mouflon-peintre`).

## Le dossier

```
aquarelle/
├── pet.toml          comportements, physique (voir format.md)
├── squelette.toml    les os, les morceaux, les animations
└── morceaux/
    ├── corps.png
    ├── tete.png
    └── …
```

Dans `pet.toml` :

```toml
[appearance]
skeleton = "squelette.toml"
height = 58.0          # hauteur affichée, en points
```

Les comportements sont ceux d'un dessin vectoriel (`motion = "walk"`,
`"sit"`, `"graze"`…) : pars de `pets/aquarelle/pet.toml` ou de
`pets/mannequin/pet.toml`.

## Peindre les morceaux

- L'animal est de **profil**, tourné vers la **droite**.
- Une image par morceau, sur fond **transparent** (PNG). Le plus simple :
  toutes les images à la taille de toute la toile, chaque morceau à sa
  place, comme un calque exporté seul. Mouflon recadre chaque image sur ce
  qu'elle montre.
- Les morceaux **se chevauchent** aux articulations : le haut des pattes
  rentre sous le corps, le cou sous la tête, la base des oreilles sous le
  toupet. Peins aussi ce qui est caché au repos (le haut d'une patte qui se
  lève, l'arrière de la tête qui se penche) : sinon les mouvements ouvrent
  des trous.
- Peins plus fin que l'écran : une toile de 1000 à 2000 pixels de large
  convient. Mouflon réduit chaque morceau (filtre de surface) à 2,5 pixels
  par point affiché ; il ne les agrandit jamais.
- Pas d'ombre au sol sous l'animal : le moteur en ajoute une, qui le suit
  en l'air.
- Les pattes du côté caché (`far`) un peu plus sombres que celles du côté
  visible (`near`) : elles passent l'une devant l'autre en marchant.

## `squelette.toml`

Toutes les positions sont en **pixels de la toile** (x vers la droite, y vers
le bas).

```toml
size = [1200, 900]     # la toile
ground = [560, 820]    # le point d'appui : entre les sabots, au sol
```

### Les os

```toml
[[bone]]
name = "racine"
at = [560, 820]

[[bone]]
name = "corps"
parent = "racine"
at = [560, 560]        # l'articulation : le point autour duquel il tourne
role = "body"
```

Chaque os a un nom, une articulation (`at`) et un parent, défini avant lui
(seul le premier os n'en a pas). Un os tourne autour de son articulation et
emporte ses enfants : la tête emporte ses oreilles et son œil.

Un `role` dit ce que le moteur fait de l'os de lui-même :

| `role` | Ce que le moteur en fait | Articulation |
|---|---|---|
| `body` | monte et descend à chaque pas, se penche en se couchant, respire | milieu du corps |
| `head` | regarde le curseur, hoche au pas, se baisse pour dormir, va au sol pour brouter | le cou |
| `ear` | bat en tombant, frémit, tombe en dormant | base de l'oreille |
| `eye` | cligne (s'écrase, ou laisse place à une paupière peinte), se ferme en dormant | milieu de l'œil |
| `pupil` | suit le curseur | — |
| `tail` | se balance, bat de temps en temps | base de la queue |
| `leg-front-near`, `leg-back-near` | pas, galop, sauts, pattes repliées couché | hanche ou épaule |
| `leg-front-far`, `leg-back-far` | idem, décalées dans le cycle | hanche ou épaule |

`near` : le côté qu'on voit ; `far` : l'autre. Une patte balaie juste ce
qu'il faut pour ne pas glisser : sa longueur est la distance de son
articulation au sol, et le pas d'un cycle est `stride` dans `[physics]`.

Les mouvements du moteur sont calculés comme si chaque os avec un rôle était
libre, puis ramenés à son parent : tu peux ranger les os comme tu veux (les
pattes sous le corps, par exemple), les sabots restent posés quand le corps
monte et descend.

### Les morceaux

```toml
[[piece]]
bone = "corps"
image = "morceaux/corps.png"

[[piece]]
bone = "oeil"
image = "morceaux/oeil.png"
eyes = "open"          # seulement les yeux ouverts

[[piece]]
bone = "oeil"
image = "morceaux/oeil-ferme.png"
eyes = "closed"        # la paupière peinte, quand l'œil se ferme
```

De l'**arrière vers l'avant** : le premier est dessiné en premier. Chaque
morceau suit son os. Une image plus petite que la toile se place avec
`at = [x, y]` (son coin en haut à gauche). Avec des paupières peintes
(`eyes = "closed"`), l'œil ne s'écrase plus pour cligner : les deux images
s'échangent.

### Les animations à clés

```toml
[[clip]]
name = "calme"
length = 7.0           # secondes, quand elle tourne sur l'horloge

[clip.bones.oreille-pres]
rotate = [[0, 0], [0.42, 0], [0.45, 14], [0.5, 0]]   # [instant, degrés]

[clip.bones.corps]
move = [[0, 0, 0], [0.5, 0, -6], [1, 0, 0]]         # [instant, x, y]
scale = [[0, 1], [0.5, 1.04, 0.97]]                  # [instant, échelle] ou [instant, largeur, hauteur]

[motions]
idle = "calme"
```

Une animation donne, pour quelques os, des angles (degrés, sens des aiguilles
d'une montre), des déplacements (pixels de la toile) et des échelles à des
instants de 0 à 1 ; entre deux clés, le mouvement part et arrive en douceur.
Elle tourne en boucle.

`[motions]` dit quand la jouer :

| clé | quand | sur quelle horloge |
|---|---|---|
| `idle` | toujours, sous les autres | l'horloge (`length` secondes) |
| `walk`, `run` | au pas, au galop | une boucle par cycle de pas |
| `sit`, `sleep`, `graze` | couché, endormi, en broutant | l'horloge |
| `air`, `dangle` | en l'air, tenu | l'horloge |

Elles se mêlent selon ce que fait l'animal (à mi-chemin entre le pas et le
galop, à moitié l'une, à moitié l'autre) et seuls les os et les mouvements
qu'elles animent changent. Ce qu'une animation joue, le moteur ne le refait
pas : avec une animation `walk`, les pattes et le corps ne bougent plus
d'eux-mêmes au pas, mais la tête regarde toujours le curseur et l'œil cligne
toujours.

## Depuis Blender (Grease Pencil)

1. Dessine l'animal de profil, tourné vers la droite (vue de face de la
   caméra, en orthographique), **un calque par morceau** (ou un objet par
   morceau). Prolonge ce qui sera caché aux articulations.
2. Rendu : *Film → Transparent* coché, PNG en RGBA, une résolution fixe (par
   exemple 1600 × 1200).
3. Rends chaque morceau seul (les autres masqués, ou un *View Layer* par
   morceau) : toutes les images ont la même taille et se superposent
   exactement.
4. Relève les articulations en pixels de l'image (par exemple en ouvrant le
   rendu complet dans un éditeur d'images, curseur sur l'épaule, la
   hanche, le cou…) et écris `squelette.toml`.

## Depuis une image (IA ou dessin d'un seul tenant)

Dans Krita, Photopea, GIMP ou Photoshop : découpe chaque morceau dans son
propre calque, repeins ce qui manque dessous (le haut des pattes sous la
toison, l'arrière de la tête sous l'oreille), puis exporte chaque calque en
PNG à la taille de la toile.

## Vérifier

```sh
cargo run --release -p mouflon-preview -- pets/aquarelle/pet.toml apercu poses
```

donne une planche de poses (debout, au pas, au galop, en l'air, tenu,
couché, endormi, en broutant). `video` à la place de `poses` donne une
scène de 18 secondes, image par image.
