# Chantier : les fiches de trente-quatre races

*Demande de l'auteur, 3 novembre 2026 : tous les moutons de Mouflon doivent
ressembler à de vraies races (seul Poe, hors du dépôt, reste un personnage),
avec au moins le niveau de détail des quatre races déjà dessinées, puis
« une bonne trentaine de moutons différents », chacun d'après une vraie
race cherchée sur Internet. Ce chantier fait les **fiches** ; les dessins
viendront ensuite (`docs/chantiers/races-dessins.md`), quand le moteur aura
son cou articulé et ses pattes en deux segments
(`docs/chantiers/moteur-sans-limites.md`, en cours dans une autre session).*

Lis d'abord `CLAUDE.md`, puis les quatre fiches modèles dans `docs/races/`
(`mouflon-corse.md`, `nez-noir.md`, `jacob.md`, `ouessant.md`, rédigées en
anglais) et le § 10 de `docs/animation-animale.md` : c'est le niveau
attendu, et c'est d'après ces fiches que les quatre races ont été dessinées.

## Ce qu'est une fiche

Un fichier `docs/races/<id>.md`, **en français**, qui permet à un
illustrateur de dessiner un individu précis de la race, de profil, tourné
vers la droite, sans rien inventer. Même plan que les modèles :

1. **L'individu à dessiner, et pourquoi** (sexe, âge, saison, toison
   tondue ou pleine ; ce qui le rend le plus reconnaissable).
2. **Tailles et proportions**, chiffrées : hauteur au garrot, longueur,
   tour de poitrine, hauteur sous le ventre, longueur de la tête, des
   oreilles, de la queue, position du genou et du jarret, avec leurs
   sources ; les rapports utiles au dessin (en fractions de la hauteur au
   garrot) ; **l'encolure** (longueur, port, angle au repos et en broutant)
   et **les articulations des pattes** (hauteur du carpe et du jarret,
   angles), puisque le moteur va savoir les plier.
3. **La toison ou le poil** : longueur de mèche, ondulation, comment elle
   tombe, ce qui reste ras (face, pattes), la mue.
4. **Les couleurs**, zone par zone, avec des valeurs hexadécimales tirées de
   photos (lumière, ombre), et les marques exactes (place, bords).
5. **La tête** : profil (busqué, droit), museau, naseaux, yeux (iris,
   pupille horizontale), oreilles (taille, port), toupet.
6. **Les cornes**, s'il y en a : nombre, section, enroulement en profil
   (angles, nombre de tours, où va la pointe), surface (annelures), couleur ;
   différences entre bélier et brebis.
7. **Les pattes et les onglons.**
8. **Le tempérament et le comportement** utiles à l'animation : grégaire ou
   non, vigilance, fuite, jeux, voix, rythme de la journée, ce qui la
   distingue (une race qui porte des sonnailles en transhumance, un
   mouflon qui se cabre…).
9. **Les dix détails** qui la font reconnaître, et les erreurs à éviter
   (ce qui en ferait une autre race).
10. **Les sources** (liens), **les photos regardées** (liens, jamais
    copiées dans le dépôt) et **ce sur quoi elles se contredisent**.

## Les races

Quatre remplacent les moutons qui n'étaient pas réalistes (l'auteur l'a
demandé ; leurs anciens noms disparaissent) :

| id | race | remplace | note |
|---|---|---|---|
| `bleu-du-maine` | Bleu du Maine | le Mannequin | le Mannequin s'en inspirait déjà (tête bleu ardoise) |
| `solognote` | Solognote | Aquarelle | elle sera peinte à l'aquarelle (squelette peint) |
| `merinos-d-arles` | Mérinos d'Arles | Pelote | les troupeaux transhumants portent des sonnailles : une race qui a vraiment une cloche |
| `southdown` | Southdown (un agneau) | Lainou | Lainou était un agneau : on garde un agneau, d'une vraie race |

Puis trente nouvelles races, choisies pour être très différentes les unes
des autres (silhouettes, cornes, toisons, couleurs, continents) :

**Lot A** (une session) : `bleu-du-maine`, `solognote`, `merinos-d-arles`,
`southdown`, `lacaune`, `manech-tete-rousse`, `thones-et-marthod`,
`landes-de-bretagne`, `suffolk`, `herdwick`, `swaledale`,
`scottish-blackface`, `hebridean`, `manx-loaghtan`, `soay`, `shetland`,
`wensleydale`.

**Lot B** (une autre session) : `badger-face-welsh`, `texel`, `zwartbles`,
`heidschnucke`, `skudde`, `gotland`, `islandais`, `romanov`, `racka`,
`karakul`, `awassi`, `dorper`, `barbados-blackbelly`, `navajo-churro`,
`mouflon-canadien` (le bighorn : lui se cabre avant le choc), `argali`,
`mouflon-de-dall`.

Si une race se révèle impossible à documenter sérieusement, remplace-la par
une autre race réelle bien différente des autres, et dis-le.

## Comment travailler

- Fais travailler des sous-agents en parallèle (une race chacun, cinq ou
  six à la fois), avec une consigne précise et le plan ci-dessus ; relis
  chaque fiche (chiffres sourcés, couleurs, cornes, les dix détails) et
  fais-la compléter si elle est mince.
- Regarde de vraies photos (Wikimedia Commons, associations de race,
  instituts) : télécharge-les dans un dossier de travail hors du dépôt,
  jamais dans le dépôt. User-Agent générique (`MouflonResearch/1.0`),
  aucune donnée personnelle dans aucune requête.
- Commite les fiches au fur et à mesure sur `fenetres-dock` (d'autres
  sessions y poussent aussi : `git pull --rebase` avant chaque `git push`),
  auteur comme le dit `CLAUDE.md`. Ne touche à rien d'autre que
  `docs/races/` (pas de code, pas de README) : une autre session travaille
  sur le moteur.
- À la fin, résume en français (on tutoie l'auteur) : les fiches écrites,
  les races remplacées et pourquoi, ce qui reste incertain.
