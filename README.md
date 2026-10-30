# Mouflon

**A light and beautiful desktop companion manager.**<br>
*Un gestionnaire de compagnons de bureau, léger et beau.*

[English](#english) · [Français](#français)

![Pelote and Lainou](docs/mascottes.png)

## English

Mouflon brings little companions to your Mac. They drop from the top of the
screen, stroll along your windows and ride with them when you move them, climb
the edges of the screen, graze, bleat, doze off — and let you catch them and
throw them across the desktop.

- **Light**: a native menu bar app, drawn by the GPU (Metal on a Mac). A
  pet made of images is a few quads drawn straight into its window (a soft
  shadow, the image, round particles), each image uploaded once, and all
  the windows of a frame go to the GPU in a single submission. A vector pet
  is baked into images when it loads, one per moving piece (head, legs,
  tail…), so it is drawn the same way; the full vector renderer (Vello)
  only starts for drawings that cannot be baked, and stops when they leave.
  At launch, each kind of pet decodes its sheets on its own threads, one
  row of tiles at a time, while the menu and the GPU get ready. Its images
  are then compressed to BC7 in the background (four times less memory,
  read as is by the GPU) and kept on disk, mapped back in from there: the
  next launches find them ready in about ten milliseconds, even after the
  app is moved or updated, and those pages do not count in the app's memory
  footprint. Images
  follow the display's own clock
  (its full rate for brisk moves, half a ProMotion display's rate for a
  stroll, 30 fps at rest, 20 fps asleep), a sprite is
  redrawn only when its image changes, and the window list is read on a
  separate thread (often only while a window carrying a pet moves), so it
  never holds up an image.
- **Beautiful**: continuous 120 Hz physics, timed to the moment each image
  reaches the display (ProMotion included), sub-pixel motion, in-between
  images for smoother animations, sharp pixel-exact images at rest, soft
  shadows, dust on landing.
- **Compatible with desktopPet**: Mouflon reads the pets of
  [desktopPet](https://github.com/Adrianotiger/desktopPet) (eSheep 64 and the
  community's pets) as they are, with all their scenes: climbing the screen
  edges, walking on the ceiling, going down windows, secondary characters,
  pets that meet and do things together, window sides that block the way.
- **And much more**: sounds (bleats, a little bell, grazing; desktopPet pets'
  own sounds too), high-definition sheets for desktopPet pets, a native format
  with images drawn for both directions, pets that greet each other or tumble
  in collision mode, *All Together*, a goodbye before leaving, four sizes, pets
  you can catch without leaving the app you are working in, pets that walk on
  the Dock (or under it) and on floating panels such as the Keyboard Viewer,
  install by drag and drop, a menu in the language of macOS (English, French,
  Spanish, German, Italian, Dutch, Portuguese, Russian, Japanese, Korean,
  Chinese).
- **Written entirely in Rust**: `unsafe` code only at the boundary with macOS,
  in a single crate, each block justified.

### Mascots

| | |
|---|---|
| **Poete** | The mascot of Mouflon, a painted sheep. It lives in its own repository and waits for the permission of its original creator before being shared. |
| **Pelote** | A soft wool sheep with a collar and a bell (`pets/pelote`): the desktopPet behaviors, drawn in high definition. |
| **Lainou** | A small wool sheep with a bell (`pets/lainou`): 26 animations drawn frame by frame, in both directions. |
| Mannequin | The built-in example: a sheep drawn as a vector rig (slate-blue head, after the Bleu du Maine breed) and animated by Mouflon (`pets/mannequin`). |

### Requirements

- A Mac with Apple Silicon (M1 or later), macOS 27 Golden Gate
  (`MIN_MACOS=26.0` to accept Tahoe as well).
- [Rust](https://rustup.rs) 1.90 or later (`rustup update`) and the Xcode
  command line tools (`xcode-select --install`).

### Run

```sh
cargo run --release
```

Launched from the repository, Mouflon also loads the pets of `pets/` and of the
`animaux/` folder: Poete drops onto your frontmost window if it is there, Pelote
otherwise. Mouflon can be handled in three places, which always show the same
state (a change made in one is seen at once in the others):

- **The menu bar icon**, for what changes often: add pets (the same one as many
  times as you like: each kind shows how many are on screen when there are
  several), say goodbye to all of them or to one of a kind, get them to play
  together, pause, mute them; *Settings…* (⌘,), *About Mouflon*, *Quit*.
- **A right click on a pet** (or Control-click): say goodbye to that one, add
  another of its kind, change its own size (that pet only), mute, pause, open
  the settings. It waits under its menu; the others go on with their lives.
- **The settings window**: size (of all the pets), sounds, realistic physics,
  collisions between pets, walking only on the frontmost app, walking under the
  Dock, opening Mouflon at login, and the pets folder, each option with a line
  saying what it does.

Pets notice what you do, without reading anything: a pointer rushing past
startles them; while you type, Lainou keeps quiet (it grazes, rests, watches
you work); after three minutes without anyone at the keyboard or the mouse,
they doze off one after the other, and wake up when you come back. Mornings
and evenings change their mood a little. Mouflon only knows *when* a key is
pressed, never *which* one (macOS tells every app, with no permission).

With *Realistic physics* (on by default), each pet is a rigid body with a mass,
an angle and a spin, in Mouflon's own 2D physics engine (`mouflon-core`,
`physics.rs`, pure Rust). A window you drag carries the pets standing on it as
long as their feet hold; jerk it and it slides from under them, pull it down
faster than they can fall and they drop, raise it and stop sharply and they are
tossed up. Held, a pet hangs from the point you took it by and swings like a
pendulum; let go, it keeps the momentum of your hand and its spin, bounces off
the edges of the screens and the sides of windows, then gets back on its feet.
With collisions on, pets hit each other in mid-air (trading momentum) and can
land and ride on each other's back. Without it, the physics of eSheep and
desktopPet: a pet sticks to its window and falls straight down. The whole
engine costs about 15 µs per 1/120 s step for 32 pets.

Pets walk on the Dock. With *Walk under the Dock*, they go down to the bottom of
the screen, pass behind the Dock and can jump onto it. To know where the Dock
really is, Mouflon needs the Accessibility permission (System Settings →
Privacy & Security → Accessibility). It also uses it to be told at once when a
window moves (only that moment: nothing in it is read), so that a pet standing
on a window you start dragging leaves with it instead of waiting for the next
poll; it uses it for nothing else. The settings window says so, and offers to
open System Settings while the permission is missing. The option is not
required. Launched with `cargo run`, the permission goes to the terminal app.

*Open Mouflon at login* registers the app with macOS (System Settings →
General → Login Items), which keeps that choice: the box shows what macOS says,
even if it was changed over there.

To try a pet without installing it, pass its file or folder: it is loaded with
the others, and it is the one that appears.

```sh
cargo run --release -- path/to/esheep64
```

To check that the animation is smooth, double-click `tools/essai/Essai de 3
minutes.command` on a Mac (with the app built in `dist/`): Mouflon runs for
three minutes while you play with the pets, then quits, the Terminal window
closes and a notification tells you it is done. Everything goes to
`essais/<date>/`:

- the journal of the trial (`journal.log`): every image (gap, time spent in
  each part, what the screen's clock said, allocations), every event
  (collisions, landings, sounds, windows, menus, notifications of the system
  such as sleep, spaces or the frontmost app), the stack of the main thread
  whenever an image is late or the event loop stays busy (sampled every 4 ms
  by a watchdog thread), and every ten seconds the resources of the process
  (CPU, performance cores, energy, instructions, time waiting for a core,
  wakeups, disk, memory, page faults, context switches, system calls), of
  each of its threads and of the system (load, memory pressure, swap,
  thermal state, Low Power Mode);
- around it, `sample` (where the CPU time goes), `top` (the other apps),
  `log stream` (messages of the system about Mouflon), `vm_stat`, `pmset`
  and `footprint`, with the time of each step (`reperes.txt`).

`tools/essai/analyse.py essais/<date>/` reads it all and lists what went
wrong: hitches and their likely cause, stalls of the main thread and where it
was, the slowest images, animations short of images, position jumps, each
collision step by step, the resources, and what the Mac was doing around the
longest hitches. The CI runs the same trial on every Mac build.

Without the script: `MOUFLON_ESSAI=180` runs a trial of 180 seconds
(`MOUFLON_JOURNAL` names the journal, `mouflon-essai.log` by default),
`MOUFLON_ESSAI_ANIMAUX=lainou,lainou,pelote` chooses its pets and
`MOUFLON_ESSAI_COLLISIONS=1` turns collisions on.

```sh
MOUFLON_ESSAI=180 MOUFLON_ESSAI_COLLISIONS=1 cargo run --release
python3 tools/essai/analyse.py mouflon-essai.log
```

`MOUFLON_VITRINE=folder` opens the settings window, saves pictures of it in
that folder (plain, then with its notes in light and dark mode) with a pet's
menu written out, then quits; `MOUFLON_LANGUE=de` picks another language than
macOS's. The CI does it in five languages on every Mac build, which shows the
window's layout without a Mac at hand.

The `essais/` and `animaux/` folders are ignored by git: keep test pets and
pets you do not publish there.

### Build the app

```sh
bash scripts/bundle-macos.sh     # dist/Mouflon.app, ad hoc signature
```

The app bundles the repository's pets (`pets/`) and those of the `animaux/`
folder, such as Poete (`animaux/poete`). At first launch, Poete greets the user
if it is bundled, Pelote otherwise. Only put pets there that you are allowed to
distribute. `APP_ICON=path/to/icon.png` replaces the app icon (1024 × 1024).

A distributable build, signed with Developer ID and notarized:

```sh
# Once: store the notarization credentials in the keychain.
xcrun notarytool store-credentials mouflon --apple-id <your Apple ID> --team-id <TEAMID>

SIGN_IDENTITY="Developer ID Application: Alejandro San Martin Lamas (TEAMID)" \
NOTARY_PROFILE=mouflon bash scripts/bundle-macos.sh
```

#### Mac App Store

```sh
APPSTORE=1 TEAM_ID=<TEAMID> \
SIGN_IDENTITY="Apple Distribution: Alejandro San Martin Lamas (TEAMID)" \
INSTALLER_IDENTITY="3rd Party Mac Developer Installer: Alejandro San Martin Lamas (TEAMID)" \
PROVISION_PROFILE=~/Downloads/Mouflon_Mac_App_Store.provisionprofile \
bash scripts/bundle-macos.sh
```

The script signs the app for the sandbox (`packaging/macos/AppStore.entitlements.in`),
embeds the provisioning profile and produces `dist/Mouflon-<version>-<build>.pkg`,
to upload with the Transporter app. The `animaux/` folder is not bundled in this
version (`APPSTORE_ANIMAUX=1` to include it).

While developing, sign your builds with your “Apple Development” certificate
rather than ad hoc: macOS ties some permissions (such as Accessibility) to the
signature, and an ad hoc signature changes with every build.

#### Releases from GitHub

The `.github/workflows/ci.yml` workflow tests every commit, builds the bundle
on macOS and, for a `v*` tag, signs, notarizes and publishes a release when
these repository secrets exist: `APPLE_CERTIFICATE` (Developer ID certificate
exported as .p12, base64-encoded), `APPLE_CERTIFICATE_PASSWORD`,
`APPLE_SIGN_IDENTITY`, `APPLE_ID`, `APPLE_TEAM_ID`, `APPLE_APP_PASSWORD`
(app-specific password).

### Add pets

Settings → *Open Folder*
(`~/Library/Application Support/com.alejandrosanmartinlamas.Mouflon/pets`).
The folder accepts:

- a folder in Mouflon's format: a `pet.toml` and its drawing;
- a desktopPet pet as it is: its `animations.xml` file, or its whole folder
  (for example `Pets/esheep64` from the desktopPet repository);
- a pet made of images: one strip of images per animation and per direction,
  described by a `pet.toml` (see [docs/format.md](docs/format.md#animaux-en-images)
  and `pets/lainou`);
- a redrawn desktopPet pet: its original behaviors with a finer sheet,
  described by a small `pet.toml` (see [docs/format.md](docs/format.md#redessiner-un-animal-desktoppet)
  and `pets/pelote`).

A pet dropped in the folder appears in the menu without restarting the app.
Even simpler: drag its file or folder from the Finder onto a pet on screen. It
is installed, and the pet you dropped it on makes way for it.

To draw your own character: [docs/rig.md](docs/rig.md); to tune its behaviors:
[docs/format.md](docs/format.md) (both in French).

### Architecture

| Crate | Role | `unsafe` |
|---|---|---|
| `mouflon-core` | physics (classic, and the realistic rigid-body engine), behaviors, pet formats, world (screens, visible window ledges) | forbidden |
| `mouflon-render` | SVG rig, procedural animation, shadow, light, particles; rigs baked into images (tiny-skia); Vello or SVG output | forbidden |
| `mouflon-platform` | other apps' windows (and their moves, with Accessibility), screens, cursor, the user's activity (when, never what), languages, the Dock (Accessibility), non-activating pet windows, settings window and a pet's menu (AppKit), opening at login | isolated and commented |
| `mouflon-app` | transparent windows (winit), GPU (wgpu: direct sprite renderer, Vello for vector pets), input, native menu, translations | forbidden |
| `tools/mouflon-preview` | off-screen preview: poses, video scene, flock, encounter, image sheet, icon | forbidden |
| `tools/mouflon-licenses` | gathers the licenses of the libraries built into the app | forbidden |
| `tools/inbetween` | computes the in-between images of a pet, once (Python script, RIFE network), not part of the app | — |
| `tools/essai` | the three-minute trial (a double-clickable script) and the reading of its journal and surroundings (Python script) | — |

All the app's code is Rust. Only the boundary with macOS contains `unsafe`, in
`mouflon-platform`, with a justification for each block. The rest of the
repository is data (pets in TOML, SVG and PNG, translations, documentation),
a build script in bash and a Python script that prepares in-between images.

### Off-screen preview

```sh
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu all
ffmpeg -framerate 60 -i apercu/frames/frame_%05d.png -c:v libx264 -pix_fmt yuv420p apercu.mp4

# Several pets at their spawn points (30 s, 4 pets):
cargo run -p mouflon-preview --release -- pets/pelote/pet.toml apercu troupeau 30 4

# Two Lainou bumping into each other, then a throw; and all its images as
# Mouflon draws them, in both directions:
cargo run -p mouflon-preview --release -- pets/lainou/pet.toml apercu rencontre
cargo run -p mouflon-preview --release -- pets/lainou/pet.toml apercu planche
```

With a desktopPet file, a long simulation without display checks that no
behavior gets stuck:

```sh
MOUFLON_DESKTOPPET=path/to/animations.xml \
  cargo test -p mouflon-core --release long_run -- --ignored --nocapture
```

### Roadmap

- Window tracking through accessibility notifications.
- A settings window.
- Windows and Linux (X11; Wayland in a reduced mode).
- A real-time 3D option.

### License

© 2026 **Alejandro San Martin Lamas**. Mouflon is free software, available
under the [MIT License](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE),
at your option: you may use, change and share it freely, including
commercially, as long as the copyright notice with the author's name stays.
The pets of this repository (Pelote, Lainou, the Mannequin) are under the same
licenses. Credits and third-party licenses: [CREDITS.md](CREDITS.md).

## Français

Mouflon fait vivre de petits compagnons sur ton Mac. Ils tombent du haut de
l'écran, se promènent sur tes fenêtres et voyagent avec elles quand tu les
déplaces, escaladent les bords de l'écran, broutent, bêlent, s'endorment — et
se laissent attraper et lancer à travers le bureau.

- **Léger** : une app native de la barre des menus, dessinée par le GPU
  (Metal sur Mac). Un animal en images, ce sont quelques quadrilatères
  dessinés droit dans sa fenêtre (une ombre douce, l'image, des particules
  rondes), chaque image envoyée une fois au GPU, et toutes les fenêtres
  d'une image partent au GPU en un seul envoi. Un animal vectoriel est cuit
  en images à son chargement, une par morceau qui bouge (tête, pattes,
  queue…), et se dessine de la même façon ; le moteur vectoriel complet
  (Vello) ne se met en route que pour un dessin qui ne se cuit pas, et
  s'arrête quand il s'en va. Au lancement, chaque espèce décode ses
  planches sur ses propres fils, une rangée de cases à la fois, pendant que
  le menu et le GPU se préparent. Ses images sont ensuite compressées en
  BC7 en arrière-plan (quatre fois moins de mémoire, lues telles quelles
  par le GPU) et gardées sur le disque, d'où elles sont projetées en
  mémoire : les lancements suivants les retrouvent prêtes en une dizaine de
  millisecondes, même après un déplacement ou une mise à jour de l'app, et
  ces pages ne comptent pas dans l'empreinte mémoire de l'app. Les images suivent l'horloge de
  l'écran lui-même (sa pleine cadence pour un mouvement vif, la moitié de
  celle d'un écran ProMotion pour une promenade, 30 i/s au repos, 20 i/s
  endormi), une image n'est redessinée que quand elle change, et la liste des
  fenêtres est relevée sur un fil à part (souvent seulement lorsqu'une
  fenêtre portant un animal bouge), sans jamais retenir une image.
- **Beau** : une physique continue à 120 Hz, calée sur l'instant où chaque
  image arrive à l'écran (ProMotion compris), des mouvements au sous-pixel
  près, des images intermédiaires pour des animations plus fluides, des
  images nettes au pixel près au repos, des ombres douces, de la poussière à
  l'atterrissage.
- **Compatible desktopPet** : Mouflon lit tels quels les animaux de
  [desktopPet](https://github.com/Adrianotiger/desktopPet) (eSheep 64 et ceux
  de la communauté), avec toutes leurs scènes : escalade des bords de l'écran,
  marche au plafond, descente des fenêtres, personnages secondaires, animaux
  qui se rencontrent et font des choses ensemble, côtés des fenêtres qui
  barrent le chemin.
- **Et bien plus** : des sons (bêlements, clochette, broutage ; ceux des
  animaux desktopPet aussi), des planches haute définition pour les animaux
  desktopPet, un format natif aux images dessinées dans les deux sens, des
  animaux qui se saluent, ou qui culbutent en mode collisions,
  « Tous ensemble », un au revoir avant de partir, quatre tailles, des animaux
  qu'on attrape sans quitter l'app où l'on travaille, qui marchent sur le Dock
  (ou dessous) et sur les panneaux flottants comme le visualiseur de clavier,
  l'installation par glisser-déposer, un menu dans la langue de macOS (anglais,
  français, espagnol, allemand, italien, néerlandais, portugais, russe,
  japonais, coréen, chinois).
- **Écrit entièrement en Rust** : du code `unsafe` seulement à la frontière
  avec macOS, dans un seul crate, chaque bloc justifié.

### Les mascottes

| | |
|---|---|
| **Poete** | La mascotte de Mouflon, un mouton peint. Il vit dans son propre dépôt et attend l'accord de son créateur d'origine avant d'être partagé. |
| **Pelote** | Un mouton tout en laine douce, avec son collier et sa clochette (`pets/pelote`) : les comportements de desktopPet, dessinés en haute définition. |
| **Lainou** | Un petit mouton de laine à clochette (`pets/lainou`) : 26 animations dessinées image par image, dans les deux sens. |
| Mannequin | L'exemple intégré : un mouton dessiné en vecteurs (tête bleu ardoise, d'après la race Bleu du Maine) et animé par Mouflon (`pets/mannequin`). |

### Prérequis

- Un Mac Apple Silicon (M1 ou plus récent), macOS 27 Golden Gate
  (`MIN_MACOS=26.0` pour accepter aussi Tahoe).
- [Rust](https://rustup.rs) 1.90 ou plus récent (`rustup update`) et les outils
  en ligne de commande Xcode (`xcode-select --install`).

### Lancer

```sh
cargo run --release
```

Lancé depuis le dépôt, Mouflon charge aussi les animaux de `pets/` et du dossier
`animaux/` : Poete tombe sur ta fenêtre au premier plan s'il y est, sinon
Pelote. Mouflon se règle à trois endroits, qui montrent toujours le même état
(un changement fait à l'un se voit aussitôt aux autres) :

- **L'icône de la barre des menus**, pour ce qu'on change souvent : ajouter des
  animaux (le même autant de fois qu'on veut : chaque espèce dit combien des
  siens sont à l'écran quand ils sont plusieurs), dire au revoir à tous ou à un
  seul d'une espèce, les faire jouer ensemble, mettre en pause, couper les
  sons ; « Réglages… » (⌘,), « À propos de Mouflon », « Quitter ».
- **Un clic droit sur un animal** (ou Contrôle-clic) : lui dire au revoir, en
  ajouter un autre de son espèce, changer sa taille à lui (cet animal
  seulement), couper les sons, mettre en pause, ouvrir les réglages. Il
  attend sous son menu ; les autres continuent leur vie.
- **La fenêtre des réglages** : taille (de tous les animaux), sons, physique
  réaliste, collisions entre animaux, marche sur la seule app au premier plan,
  marche sous le Dock, ouverture de Mouflon à l'ouverture de session et
  dossier des animaux, chaque option avec une ligne qui dit ce qu'elle fait.

Les animaux remarquent ce que tu fais, sans rien lire : un pointeur qui file
tout près les fait sursauter ; quand tu tapes, Lainou se fait discret (il
broute, se repose, te regarde travailler) ; au bout de trois minutes sans
personne au clavier ni à la souris, ils s'endorment l'un après l'autre, et se
réveillent à ton retour. Le matin et le soir changent un peu leur humeur.
Mouflon sait seulement *quand* une touche est frappée, jamais *laquelle* (macOS
le dit à toutes les apps, sans autorisation).

Les animaux marchent sur le Dock. Avec « Marcher sous le Dock », ils descendent
jusqu'en bas de l'écran, passent derrière le Dock et peuvent sauter dessus.
Pour savoir où est vraiment le Dock, Mouflon a besoin de l'autorisation
d'Accessibilité (Réglages Système → Confidentialité et sécurité →
Accessibilité). Elle lui sert aussi à être prévenu aussitôt qu'une fenêtre
bouge (seulement ce moment : rien n'y est lu), pour qu'un animal posé sur une
fenêtre qu'on commence à tirer parte avec elle au lieu d'attendre le prochain
relevé ; elle ne lui sert à rien d'autre. La fenêtre des réglages le dit, et
propose d'ouvrir Réglages Système tant que l'autorisation manque.
L'option est facultative. Lancé avec `cargo run`, c'est le Terminal qui reçoit
l'autorisation.

Avec « Physique réaliste » (cochée d'origine), chaque animal est un corps
rigide qui a une masse, un angle et une rotation, dans le moteur physique 2D de
Mouflon (`mouflon-core`, `physics.rs`, en Rust pur). Une fenêtre qu'on déplace
emporte les animaux posés dessus tant que leurs pattes tiennent ; secouée d'un
coup sec, elle file sous eux ; tirée vers le bas plus vite qu'ils ne tombent,
elle les lâche ; montée puis arrêtée net, elle les lance en l'air. Tenu, un
animal pend sous le point par lequel on l'a pris et se balance comme un
pendule ; lâché, il garde l'élan de la main et sa rotation, rebondit sur les
bords des écrans et les côtés des fenêtres, puis se remet sur ses pattes. Avec
les collisions, les animaux se heurtent en plein vol (ils échangent leur élan)
et peuvent se poser sur le dos l'un de l'autre, qui les emporte. Sans elle, la
physique d'eSheep et de desktopPet : l'animal colle à sa fenêtre et tombe tout
droit. Tout le moteur coûte environ 15 µs par pas de 1/120 s pour 32 animaux.

« Ouvrir Mouflon à l'ouverture de session » inscrit l'app auprès de macOS
(Réglages Système → Général → Ouverture), qui garde ce choix : la case montre
ce qu'en dit macOS, même s'il a été changé de ce côté-là.

Pour essayer un animal sans l'installer, passe son fichier ou son dossier en
argument : il est chargé en plus des autres, et c'est lui qui apparaît.

```sh
cargo run --release -- chemin/vers/esheep64
```

Pour vérifier que l'animation est fluide, double-clique sur `tools/essai/Essai
de 3 minutes.command` sur un Mac (avec l'app construite dans `dist/`) :
Mouflon tourne trois minutes pendant que tu joues avec les animaux, puis
quitte, la fenêtre du Terminal se ferme et une notification te prévient. Tout
va dans `essais/<date>/` :

- le journal de l'essai (`journal.log`) : chaque image (écart, temps passé
  dans chaque partie, ce qu'a dit l'horloge de l'écran, allocations), chaque
  événement (collisions, atterrissages, sons, fenêtres, menus, notifications
  du système comme la veille, les bureaux ou l'app au premier plan), la pile
  du fil principal dès qu'une image tarde ou que la boucle d'événements reste
  occupée (relevée toutes les 4 ms par un fil de surveillance), et toutes les
  dix secondes les ressources du processus (processeur, cœurs performants,
  énergie, instructions, attente d'un cœur, réveils, disque, mémoire, fautes
  de page, changements de contexte, appels système), de chacun de ses fils et
  du système (charge, pression mémoire, échange, état thermique, économie
  d'énergie) ;
- autour, `sample` (où passe le temps de calcul), `top` (les autres apps),
  `log stream` (les messages du système sur Mouflon), `vm_stat`, `pmset` et
  `footprint`, avec l'heure de chaque étape (`reperes.txt`).

`tools/essai/analyse.py essais/<date>/` lit le tout et dit ce qui cloche : les
à-coups et leur cause probable, les blocages du fil principal et où il en
était, les images les plus longues à préparer, les animations qui manquent
d'images, les sauts de position, chaque collision pas à pas, les ressources,
et ce que faisait le Mac autour des plus longs à-coups. La CI fait le même
essai à chaque construction pour Mac.

Sans le script : `MOUFLON_ESSAI=180` lance un essai de 180 secondes
(`MOUFLON_JOURNAL` nomme le journal, `mouflon-essai.log` par défaut),
`MOUFLON_ESSAI_ANIMAUX=lainou,lainou,pelote` choisit ses animaux et
`MOUFLON_ESSAI_COLLISIONS=1` met le mode collisions.

```sh
MOUFLON_ESSAI=180 MOUFLON_ESSAI_COLLISIONS=1 cargo run --release
python3 tools/essai/analyse.py mouflon-essai.log
```

`MOUFLON_VITRINE=dossier` ouvre la fenêtre des réglages, en enregistre des
images dans ce dossier (telle quelle, puis avec ses remarques, en clair et en
sombre) avec le texte du menu d'un animal, puis quitte ; `MOUFLON_LANGUE=de`
choisit une autre langue que celle de macOS. La CI le fait en cinq langues à
chaque construction pour Mac : on voit la mise en page de la fenêtre sans
avoir de Mac sous la main.

Les dossiers `essais/` et `animaux/` sont ignorés par git : on y garde les
animaux d'essai et ceux qu'on ne publie pas.

### Construire l'app

```sh
bash scripts/bundle-macos.sh     # dist/Mouflon.app, signée en ad hoc
```

L'app emporte les animaux du dépôt (`pets/`) et ceux du dossier `animaux/`,
comme Poete (`animaux/poete`). Au premier lancement, Poete accueille
l'utilisateur s'il est livré, sinon Pelote. N'y mets que des animaux que tu as
le droit de distribuer. `APP_ICON=chemin/vers/icone.png` remplace l'icône de
l'app (1024 × 1024).

Une version distribuable, signée Developer ID et notarisée :

```sh
# Une seule fois : enregistrer les identifiants de notarisation dans le trousseau.
xcrun notarytool store-credentials mouflon --apple-id <ton Apple ID> --team-id <TEAMID>

SIGN_IDENTITY="Developer ID Application: Alejandro San Martin Lamas (TEAMID)" \
NOTARY_PROFILE=mouflon bash scripts/bundle-macos.sh
```

#### Mac App Store

```sh
APPSTORE=1 TEAM_ID=<TEAMID> \
SIGN_IDENTITY="Apple Distribution: Alejandro San Martin Lamas (TEAMID)" \
INSTALLER_IDENTITY="3rd Party Mac Developer Installer: Alejandro San Martin Lamas (TEAMID)" \
PROVISION_PROFILE=~/Downloads/Mouflon_Mac_App_Store.provisionprofile \
bash scripts/bundle-macos.sh
```

Le script signe l'app pour le bac à sable (`packaging/macos/AppStore.entitlements.in`),
y place le profil d'approvisionnement et produit `dist/Mouflon-<version>-<build>.pkg`,
à envoyer avec l'app Transporter. Le dossier `animaux/` n'est pas livré dans
cette version (`APPSTORE_ANIMAUX=1` pour l'inclure).

En développement, signe tes builds avec ton certificat « Apple Development »
plutôt qu'en ad hoc : macOS rattache certaines autorisations (comme
l'Accessibilité) à la signature, et une signature ad hoc change à chaque
compilation.

#### Publier depuis GitHub

Le workflow `.github/workflows/ci.yml` teste chaque commit, construit le bundle
sur macOS et, pour un tag `v*`, signe, notarise et publie une version si ces
secrets existent dans le dépôt : `APPLE_CERTIFICATE` (certificat Developer ID
exporté en .p12, encodé en base64), `APPLE_CERTIFICATE_PASSWORD`,
`APPLE_SIGN_IDENTITY`, `APPLE_ID`, `APPLE_TEAM_ID`, `APPLE_APP_PASSWORD`
(mot de passe pour app).

### Ajouter des animaux

Réglages → « Ouvrir le dossier »
(`~/Library/Application Support/com.alejandrosanmartinlamas.Mouflon/pets`).
Le dossier accepte :

- un dossier au format Mouflon : un `pet.toml` et son dessin ;
- un animal desktopPet tel quel : son fichier `animations.xml`, ou tout son
  dossier (par exemple `Pets/esheep64` du dépôt desktopPet) ;
- un animal en images : une bande d'images par animation et par sens, décrite
  par un `pet.toml` (voir [docs/format.md](docs/format.md#animaux-en-images) et
  `pets/lainou`) ;
- un animal desktopPet redessiné : ses comportements d'origine avec une planche
  plus fine, décrite par un petit `pet.toml` (voir
  [docs/format.md](docs/format.md#redessiner-un-animal-desktoppet) et `pets/pelote`).

Un animal déposé dans le dossier apparaît dans le menu sans relancer l'app.
Plus simple encore : fais glisser son fichier ou son dossier depuis le Finder
sur un animal à l'écran. Il est installé, et l'animal visé lui laisse sa place.

Pour dessiner ton propre personnage : [docs/rig.md](docs/rig.md) ; pour régler
ses comportements : [docs/format.md](docs/format.md).

### Architecture

| Crate | Rôle | `unsafe` |
|---|---|---|
| `mouflon-core` | physique (classique, et le moteur réaliste de corps rigides), comportements, formats d'animaux, monde (écrans, rebords visibles des fenêtres) | interdit |
| `mouflon-render` | rig SVG, animation procédurale, ombre, lumière, particules ; rigs cuits en images (tiny-skia) ; sortie Vello ou SVG | interdit |
| `mouflon-platform` | fenêtres des autres apps (et leurs déplacements, avec l'Accessibilité), écrans, curseur, activité de l'utilisateur (quand, jamais quoi), langues, Dock (Accessibilité), fenêtres d'animaux non activantes, fenêtre des réglages et menu d'un animal (AppKit), ouverture à l'ouverture de session | isolé et commenté |
| `mouflon-app` | fenêtres transparentes (winit), GPU (wgpu : rendu direct des images, Vello pour les dessins vectoriels), entrées, menu natif, traductions | interdit |
| `tools/mouflon-preview` | aperçu hors écran : poses, scène vidéo, troupeau, rencontre, planche d'images, icône | interdit |
| `tools/mouflon-licenses` | réunit les licences des bibliothèques compilées dans l'app | interdit |
| `tools/inbetween` | calcule une fois pour toutes les images intermédiaires d'un animal (script Python, réseau RIFE), hors de l'app | — |
| `tools/essai` | l'essai de trois minutes (un script à double-cliquer) et la lecture de son journal et de ses alentours (script Python) | — |

Tout le code de l'app est en Rust. Seule la frontière avec macOS contient de
l'`unsafe`, dans `mouflon-platform`, avec une justification pour chaque bloc.
Le reste du dépôt est fait de données (animaux en TOML, SVG et PNG,
traductions, documentation), d'un script de construction en bash et d'un
script Python qui prépare les images intermédiaires.

### Aperçu hors écran

```sh
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu all
ffmpeg -framerate 60 -i apercu/frames/frame_%05d.png -c:v libx264 -pix_fmt yuv420p apercu.mp4

# Plusieurs animaux à leurs points d'apparition (30 s, 4 animaux) :
cargo run -p mouflon-preview --release -- pets/pelote/pet.toml apercu troupeau 30 4

# Deux Lainou qui se rentrent dedans, puis un lancer ; et toutes ses images
# telles que Mouflon les dessine, dans les deux sens :
cargo run -p mouflon-preview --release -- pets/lainou/pet.toml apercu rencontre
cargo run -p mouflon-preview --release -- pets/lainou/pet.toml apercu planche
```

Avec un fichier desktopPet, une longue simulation sans affichage vérifie
qu'aucun comportement ne bloque :

```sh
MOUFLON_DESKTOPPET=chemin/vers/animations.xml \
  cargo test -p mouflon-core --release long_run -- --ignored --nocapture
```

### Feuille de route

- Suivi des fenêtres par notifications d'accessibilité.
- Fenêtre de réglages.
- Windows et Linux (X11 ; Wayland en mode dégradé).
- Option 3D temps réel.

### Licence

© 2026 **Alejandro San Martin Lamas**. Mouflon est un logiciel libre, sous
[licence MIT](LICENSE-MIT) ou [licence Apache 2.0](LICENSE-APACHE), au choix :
on peut l'utiliser, le modifier et le partager librement, y compris
commercialement, tant que la mention de copyright avec le nom de l'auteur
reste. Les animaux de ce dépôt (Pelote, Lainou, le Mannequin) sont sous les
mêmes licences. Crédits et licences des tiers : [CREDITS.md](CREDITS.md).
