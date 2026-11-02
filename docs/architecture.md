# Architecture de Mouflon : où on en est, où on va

*Réflexion du 1er novembre 2026, après les deux essais de trois minutes sur
le vrai Mac (MacBook Pro M1 Max, écran ProMotion à 120 Hz, taille 2,
collisions et physique réaliste).*

Le but : une fluidité parfaite du lancement à la fermeture, sans aucun
ralentissement quel que soit le nombre de moutons, la mémoire étant la
seule limite. On reste en Rust.

## 0. Où on en est (2 novembre 2026)

Les étapes 1 à 4 sont faites, la 5 en bonne partie, la 6 commencée (voir le
tableau de la partie 4). Ce que mesurent les essais de la CI (machine
virtuelle à trois cœurs, écran à 60 Hz, collisions et physique réaliste) :

| | une fenêtre par animal (31 oct.) | calques, fil principal (`41db34e`) | calques, fil de l'affichage (`8308f87`) |
|---|---|---|---|
| essai de trois minutes : images en retard | — | 0,93 % | 0,44 % |
| horloge de l'écran, de la synchronisation à l'appel (médiane) | — | 0,75 ms | 0,24 ms |
| essai de charge : images en retard | — | 1,40 % | 0,08 % |
| … avec 100, 500 et 1 000 animaux | — | 1,52 %, 1,24 %, 2,50 % | 0, 0, 0 |
| préparation d'une image avec 1 000 animaux (médiane, 99 %) | — | 8,3 et 20,6 ms | 8,4 et 11,8 ms |

Les retards qui restent viennent de l'écran (rafraîchissements sautés par
l'horloge de la machine virtuelle), d'un fil qui attend un cœur, et des
instantanés de la scène que prennent les essais. Plus aucun ne vient du fil
principal : il ne fait plus d'images.

Ce qui a été fait, en bref :
- **processus auxiliaire** (`helper.rs`, `survey.rs`, `watch.rs`) : le
  relevé des fenêtres, le Dock, l'Accessibilité et l'app au premier plan ;
- **une fenêtre par écran** (`screens.rs`, fil principal) et une **scène
  de calques** (`scene.rs`) : chaque animal en morceaux d'image partagés
  (`mouflon_render::pieces`), une seule transaction par image. Les calques
  sont dans le repère de la vue de la fenêtre, qu'AppKit retourne (winit la
  déclare `isFlipped`) : origine en haut à gauche, y vers le bas. Les
  instantanés appliquent le même retournement, et l'analyse de chaque essai
  les compare aux captures de l'écran ;
- **le fil de l'affichage** (`display.rs`) : troupeau, monde, scène et
  rythme, réveillé par un `CADisplayLink` inscrit sur sa propre boucle. Le
  fil principal lui envoie des demandes numérotées et reçoit ce qu'il doit
  savoir (le troupeau pour les menus, les fenêtres qui prennent la souris,
  l'animal dont ouvrir le menu, une nouvelle horloge, la fin d'un essai).
  Avec le GPU (`MOUFLON_GPU=1`), l'affichage reste sur le fil principal
  (`MOUFLON_AFFICHAGE=principal` l'y force aussi, pour comparer) ;
- **la foule** : les paires proches par balayage, en deux sortes ajustées à
  ce qui peut arriver (des dessins qui se touchent à la même hauteur, dont
  l'un va de côté ; des corps, dont l'un est en l'air), les surfaces rangées
  par colonnes, et un essai de charge (20, 100, 500, 1 000 animaux).

Ce qui reste : alléger la simulation des moutons calmes (une foule serrée de
mille moutons coûte encore 5 ms de simulation par image sur la machine de
la CI) ; retirer Vello du programme ; des tests pour chaque partie de l'app.

## 1. Ce que disent les essais

Les deux essais ont tourné l'un après l'autre sur le même Mac, avec la même
façon de jouer (des moutons ajoutés par dizaines, lancés, attrapés) :

| | une fenêtre par animal | une fenêtre par écran |
|---|---|---|
| animaux à la fois, au plus | 18 | 13 |
| images en retard en mouvement | 17,7 % | 10,1 % |
| écart normal en mouvement | 11,9 ms (84 i/s) | 8,5 ms (118 i/s) |
| écart moyen avec 12 à 14 animaux (99 %) | 17,1 ms (73 ms) | 10,4 ms (23,5 ms) |
| gels d'une demi-seconde | 9 | 0 |
| Mouflon (processeur, médiane) | 19 % d'un cœur | 12 % d'un cœur |
| WindowServer pendant l'essai (avant le lancement) | 62 % (56 %) | 71 % (72 %) |
| WindowManager | 26 % | — |
| empreinte mémoire, médiane et maximum | 196 et 468 Mo | 303 et 521 Mo |
| surfaces d'affichage | 47 Mo | 129 Mo |

Avec une référence prise avant le lancement, la conclusion de l'essai du
31 octobre s'inverse : WindowServer était déjà très occupé sans Mouflon, et
la fenêtre unique ne lui coûte presque rien de plus. Une fenêtre par écran
est nettement plus fluide qu'une fenêtre par animal.

### Les causes, une à une

1. **Gels d'une demi-seconde** (une fenêtre par animal). Le relevé des
   fenêtres des autres apps (`CGWindowListCopyWindowInfo`, sur son propre
   fil) commence par synchroniser les transactions Core Animation de *tout
   le processus* avec WindowServer
   (`SLSConnectionSynchronizeSLSCATransaction`) et garde le verrou de la
   connexion pendant ce temps. Le fil principal, qui veut valider la sienne
   (les fenêtres déplacées), attend ce verrou. Les deux s'attendent jusqu'à
   ce que l'attente se dénoue, au bout d'une demi-seconde. Un autre fil ne
   protège de rien : le verrou est celui du processus.
2. **Chaque fenêtre déplacée coûte sur le fil principal.** AppKit traite
   l'événement « fenêtre déplacée » et interroge WindowServer de façon
   synchrone (`-[NSWindow _windowMoved:]`, configuration des écrans), et
   WindowManager (le gestionnaire de fenêtres de macOS) suit chaque
   déplacement. Avec 15 à 20 animaux, cela fait 15 à 20 fois par image, et
   le coût grandit avec le nombre de moutons.
3. **La fenêtre unique attend une image libre.** Son image fait tout l'écran
   (4112 × 2658 pixels) : Mouflon attend en moyenne 2 à 5,5 ms (jusqu'à
   20 ms et plus) qu'une des surfaces se libère, et ces surfaces occupent
   129 Mo.
4. **Des appels synchrones sur le fil principal** au gré des actions :
   l'état de l'ouverture à l'ouverture de session (`SMAppService`, une
   cinquantaine de millisecondes) à chaque action des réglages, la première
   ouverture d'un menu (chargement de code par le système).
5. **Des moutons en l'air.** Poete s'est posé sur le dos de Lainou endormi,
   à la hauteur d'un Lainou debout, et y est resté une demi-minute en l'air.
   *Corrigé* (`036e948`). Les gels font aussi s'arrêter net un mouton qui
   tombe. Reste à vérifier une troisième piste : des fenêtres que macOS
   annonce mais qu'on ne voit pas (Stage Manager, fenêtres transparentes
   d'autres apps). Le prochain journal dira sur quelle fenêtre chaque
   mouton se pose.

Ce qui ne dépend pas de nous : WindowServer tournait déjà entre 56 et 72 %
d'un cœur avant le lancement, et une part des images manquées l'est par
l'écran lui-même. L'objectif est que Mouflon n'y ajoute rien et ne soit
jamais la cause d'un retard.

## 2. L'architecture d'aujourd'hui

- **`mouflon-core`** (14 000 lignes, 128 tests) : la simulation à pas fixe
  (120 pas par seconde), les comportements, la physique, les formats. Pur,
  déterministe, sans écran. *À garder.*
- **`mouflon-render`** (7 800 lignes) : des listes d'affichage, les planches
  (BC7, cache sur disque), les dessins vectoriels cuits en images, les
  squelettes. *À garder*, en simplifiant (voir 3.4).
- **`mouflon-platform`** (5 400 lignes) : le seul endroit avec du `unsafe`
  (AppKit, Core Graphics, Accessibilité, diagnostics). *À garder.*
- **`mouflon-app`** (9 200 lignes) : un objet `App` de 3 300 lignes et 73
  méthodes qui fait tout, sur le fil principal, à chaque image : horloge de
  l'écran, entrées, simulation, une fenêtre par animal (dessin GPU dans
  chacune, puis déplacement), menus, réglages, surprises, essais. *C'est là
  que tout se joue.*

Les points faibles, par ordre d'effet :
1. **une fenêtre du système par animal**, qu'on déplace à chaque image ;
2. **tout sur le fil principal** : un menu ouvert, une fenêtre des réglages,
   une requête d'Accessibilité d'une autre app ou un premier chargement de
   code retardent les images ;
3. **des appels système synchrones dans le processus** (relevé des fenêtres,
   ouverture de session) ;
4. **des boucles sur toutes les paires d'animaux** à chaque pas de
   simulation : 1 000 moutons, c'est un demi-million de paires, 120 fois
   par seconde ;
5. **plusieurs chemins de dessin** (images, Vello, cuisson, accessoires) qui
   compliquent tout le reste.

## 3. L'architecture cible

### 3.1 Le système compose, Mouflon décrit

Sur chaque écran, **une seule fenêtre transparente**, créée une fois et
jamais déplacée. Dedans, **un arbre de calques Core Animation** : un calque
par mouton (et par morceau pour un squelette, par particule, pour l'ombre),
dont le contenu est une **image toute prête, partagée** par tous les
moutons de la même espèce et de la même taille. À chaque image, Mouflon ne
change que les propriétés des calques qui bougent (position, rotation,
échelle, opacité, image affichée), en **une seule transaction**. Le
compositeur du système ne recompose que les régions qui ont changé.

- Plus de fenêtre par animal : la cause 2 disparaît, la cause 1 aussi
  (plus de transaction concurrente à chaque image).
- Plus de grande surface à attendre ni à composer en entier : la cause 3
  disparaît, et les 129 Mo avec.
- Le coût par mouton devient quelques propriétés de calque par image.

Les images des planches restent compressées (BC7) en mémoire et sur le
disque. Core Animation n'affiche pas directement nos images BC7 : les
images *vivantes*
(celles des animations en cours) sont décompressées d'avance sur un fil de
travail, dans un cache limité (les moins récentes oubliées). Un mouton qui
marche n'a besoin que de sa marche.

Les clics : la fenêtre laisse passer la souris, sauf au-dessus d'un mouton.
À chaque image, on sait où est le curseur et ce qu'il survole (les masques
des images le disent au pixel près) : la fenêtre se met à capter la souris
juste à ce moment-là. Prise, glisser, menu d'un animal et dépôt d'un
fichier depuis le Finder passent par là.

### 3.2 Le fil principal ne cadence plus rien

Un **fil d'affichage**, réveillé par l'horloge de l'écran (un
`CADisplayLink` attaché à son propre fil, macOS 14), fait chaque image : il
lit les entrées reçues, avance la simulation, met à jour les calques et
valide la transaction. Le fil principal ne garde que ce qu'il est seul à
pouvoir faire : recevoir les événements, menus, réglages, galerie.

Un menu ouvert ou une fenêtre des réglages n'arrêtent plus les moutons. Une
requête d'Accessibilité d'une autre app, un premier chargement de code, une
réponse lente du système ne retardent plus une image.

### 3.3 Aucun appel système bloquant dans le processus

Le relevé des fenêtres, le cadre du Dock, le suivi par l'Accessibilité et
l'état de l'ouverture à l'ouverture de session passent par un **processus
auxiliaire** (le même programme, lancé avec un argument), qui envoie ses
relevés par un tube. Ses appels ne touchent ni à notre connexion avec
WindowServer ni à nos fils. Le journal des essais garde, en plus, la liste
des fenêtres à chaque changement (application, cadre, niveau) et la fenêtre
sur laquelle chaque mouton se pose.

### 3.4 Pour le dessin, un seul modèle : des morceaux d'image

Une planche (une image par pose), un dessin vectoriel (cuit en morceaux),
un squelette (des morceaux peints), l'ombre, la poussière, les bulles, les
feuilles, la soucoupe : tout devient **un morceau d'image avec une
transformation et une opacité**, rangé dans un petit arbre par animal. La
liste d'affichage d'aujourd'hui n'a déjà presque plus que des images. Les
tracés, les ombres floutées et les disques ne servent plus qu'à *préparer*
ces images au chargement.

Le même arbre se dessine de deux façons :
- **calques Core Animation** sur Mac (3.1) ;
- **quadrilatères GPU** (le rendu direct d'aujourd'hui, avec wgpu) pour
  l'aperçu hors écran, la CI, et plus tard Windows et Linux là où le
  système n'offre pas de compositeur à calques.

Vello disparaît du programme.

### 3.5 Des milliers de moutons

- **Plus de boucle sur toutes les paires** : une grille (des cases d'environ
  128 points) ne compare que les voisins. Les rebords des fenêtres sont
  rangés par position.
- **Les moutons calmes coûtent moins** : un mouton qui dort ou broute
  avance sa simulation moins souvent (son dessin reste fluide).
- **Mémoire par mouton constante et petite** : un calque et son état ; les
  images sont par espèce et par taille.
- **Un essai de charge** ajoute des moutons par paliers (20, 100, 500,
  1 000) et mesure chaque palier, en CI et sur le Mac.

### 3.6 Une app découpée

L'objet `App` se sépare en parties qui se parlent par messages :

| partie | fil | rôle |
|---|---|---|
| `scene` | affichage | fenêtres d'écran, calques, test de survol, glisser |
| `display` | affichage | horloge, pas de simulation, mise à jour de la scène |
| `world` (cœur) | affichage | troupeau, physique, surfaces |
| `system` | processus auxiliaire | fenêtres, Dock, Accessibilité, ouverture de session |
| `ui` | principal | barre des menus, menu d'un animal, réglages, galerie |
| `library` | travail | catalogue des espèces, images, cache |
| `trial` | journal | mesures et journal des essais, hors du chemin des images |

Chaque partie se teste seule. Pour une centaine d'espèces, le catalogue ne
lit au lancement que les fiches (nom, auteur, description), et les images à
la première apparition ou dans la galerie.

### 3.7 Mesurer, pour ne jamais revenir en arrière

Des budgets vérifiés par l'analyse de chaque essai :
- **zéro image en retard causée par Mouflon** (celles qui restent doivent
  venir du système ou de l'écran) ;
- fil d'affichage : moins de 2 ms par image avec 100 moutons ;
- fil principal : jamais sur le chemin d'une image ;
- WindowServer : moins de 5 % d'un cœur au-dessus de sa référence d'avant
  le lancement ;
- mémoire : moins de 100 Mo plus les images vivantes, sans surface
  d'affichage par mouton.

La CI échouera si un essai montre un retard causé par Mouflon.

## 4. Le chemin, par étapes

Chaque étape donne une version à essayer sur le vrai Mac.

| étape | contenu | durée estimée | état |
|---|---|---|---|
| 0 | sauts à l'atterrissage, moutons posés en l'air sur un dos | — | fait |
| 1 | processus auxiliaire pour les appels système, journal des fenêtres sous les moutons | une demi-journée | fait (`ac180e5`) |
| 2 | scène : une fenêtre par écran et des calques pour les animaux en images (images partagées, cache des images vivantes, survol, prise, menus, dépôt) | deux jours | fait (`de49997`, à l'endroit depuis `8308f87`) |
| 3 | tout en morceaux : dessins vectoriels, squelettes, ombres, particules, surprises ; fin des fenêtres par animal et de Vello | un à deux jours | fait, sauf Vello |
| 4 | fil d'affichage : simulation et calques hors du fil principal | un jour | fait (`8308f87`) |
| 5 | des milliers de moutons : grille, simulation allégée au calme, essai de charge | un jour | balayage, surfaces par colonnes, essai de charge ; reste le calme |
| 6 | découpage de l'app, tests de chaque partie, documentation | un à deux jours | commencé (affichage, fenêtres des écrans, relevé à part) |

Soit une semaine et demie de sessions, avec un essai réel entre les étapes.

## 5. Risques et façons de les lever

- **Laisser passer la souris** au bon moment : le survol est décidé à chaque
  image. Si un clic rapide tombait entre deux images, une petite fenêtre
  invisible pourrait suivre le seul mouton sous le curseur. À mesurer dès
  l'étape 2.
- **Core Animation hors du fil principal** : permis pour des calques qui
  nous appartiennent, avec des transactions explicites (Firefox fait de même
  depuis son fil de composition). Le risque qui reste : qu'AppKit ait à
  redessiner ou à disposer un de ses propres calques de la fenêtre au moment
  où notre fil valide une transaction. La vue de chaque fenêtre d'écran ne
  dessine donc jamais rien elle-même (`NSViewLayerContentsRedrawNever`), et
  les fenêtres ne bougent ni ne changent de taille. En cas de doute,
  `MOUFLON_AFFICHAGE=principal` remet tout sur le fil principal.
- **Mémoire des images décompressées** pour les grandes planches : cache des
  images vivantes, décompression en avance et sur un autre fil.
- **Ce que macOS fait des fenêtres d'écran** (Spaces, apps en plein écran,
  Mission Control, Stage Manager, enregistrement d'écran, Dock) : à vérifier
  une à une à l'étape 2, comme pour les fenêtres d'animaux aujourd'hui.

## 6. Ce qu'on ne change pas

Rust ; le cœur déterministe et ses tests ; les formats (`pet.toml`,
desktopPet, squelettes) ; la compression BC7 et le cache sur disque ; les
outils (aperçu, peintre, images intermédiaires) ; les essais et leur
analyse, qui ont trouvé chacun des problèmes ci-dessus.
