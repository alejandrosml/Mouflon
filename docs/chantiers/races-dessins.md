# Chantier : dessiner trente races

*Suite de `docs/chantiers/races-fiches.md`. À lancer quand le moteur sait
plier le cou et les pattes (`docs/chantiers/moteur-sans-limites.md`) : les
nouvelles races profitent de l'encolure, des bas de pattes, de la lèvre et
de la toison qui bouge dès leur premier dessin.*

Lis d'abord `CLAUDE.md`, `docs/rig.md` (le contrat des parties, avec les
nouvelles : `neck`, `leg-*-lower`, `lip`, `sway-*`…), `tools/races/README.md`
(comment un générateur écrit un SVG), le § 10 de
`docs/animation-animale.md`, et la fiche de chaque race de ton lot dans
`docs/races/<id>.md`.

## État au 3 novembre 2026, au soir : la suite passe à Codex

- **Faites** (dans `pets/`) : `thones-et-marthod`, `lacaune`.
- **Commencées** (seulement dans `tools/races/<id>/`, pas encore dans
  `pets/`) : `suffolk` (le dessin est complet ; reste la dernière passe et la
  relecture contre les photos) et `swaledale` (reste une passe de
  corrections : la toison, les pattes, les poils qui se fondent dans le
  front). Le détail est dans le message du commit 7d13df4. Reprends ces
  générateurs, ne recommence pas.
- **À dessiner, avec leur fiche** : lot 1 `scottish-blackface`,
  `manech-tete-rousse`, `landes-de-bretagne`, `herdwick` ; lot 2
  `badger-face-welsh`, `texel`, `zwartbles`, `heidschnucke`, `skudde`,
  `gotland`, `islandais` ; lot 3 `romanov`, `racka`, `karakul`, `awassi`,
  `dorper`.
- **Plus tard**, quand leur fiche sera écrite
  (`docs/chantiers/races-fiches-consigne.md`) : `hebridean`,
  `manx-loaghtan`, `soay`, `shetland`, `wensleydale`, `barbados-blackbelly`,
  `navajo-churro`, `mouflon-canadien`, `argali`, `mouflon-de-dall`.

Avec Codex : une tâche par race, et une PR par race vers `fenetres-dock`,
qui ne touche que les fichiers de sa race (voir plus bas). Personne ne
relira tes planches à ta place : avant la PR, relis chacune comme un
relecteur qui ne l'a pas dessinée, à côté de trois à cinq photos de la race
(silhouette, proportions, tête, oreilles, cornes, couleurs et marques,
toison, pattes, sabots), corrige, recommence tant que ce n'est pas juste, et
dis dans la PR ce qui reste imparfait.

## Ce qu'on attend pour chaque race

Le niveau des quatre premières races au moins (regarde leurs planches dans
l'aperçu : `pets/mouflon-corse`, `pets/nez-noir`, `pets/jacob`,
`pets/ouessant`) :

- **Un générateur** `tools/races/<id>/gen_<id>.py` (Python 3, commentaires en
  français, graine fixe) qui écrit `pets/<id>/<id>.svg` : profil droit, une
  unité = un centimètre de l'animal, toutes les parties du contrat (corps,
  encolure, tête, mâchoire, lèvre, yeux, pupilles en fente horizontale,
  oreilles, queue, en chaîne si elle est longue, les quatre pattes en deux
  segments, et des mèches qui bougent si la toison est longue), sans groupe
  semi-transparent, masque, découpe ni filtre. Le SVG du dépôt est la sortie
  exacte du script.
- **Un `pets/<id>/pet.toml`** en français : nom (le vrai nom de la race),
  description courte (moins de 230 caractères), `category = "sheep"`,
  hauteur à 0,5 pt par centimètre, `width` et `stride` (1,6 × la longueur
  des pattes), les comportements du Mannequin réglés sur le tempérament de la
  race (fiche, § comportement) ; les sons du Mannequin (copiés dans
  `pets/<id>/sons/`, domaine public). Une cloche (sonnaille) seulement si la
  race en porte vraiment.
- **Regarder ce qu'on dessine** : une planche de contrôle (le dessin en
  grand, la tête de près, les poses du moteur), puis les planches `poses`,
  `gestes` et `jeux` de `mouflon-preview`, tourné à gauche compris.
  Compare avec de vraies photos de la race et corrige jusqu'à ce que les dix
  détails de la fiche soient justes.
- **Les tests** : `cargo test --release -p mouflon-render --test troupeau`
  (chaque mouton a ses parties, se cuit, pose ses sabots au sol) et les
  tests du cœur ; fmt, clippy (Linux et `aarch64-apple-darwin`) comme le dit
  `CLAUDE.md`.

## Les lots

- **Lot 1** : `lacaune`, `manech-tete-rousse`, `thones-et-marthod`,
  `landes-de-bretagne`, `suffolk`, `herdwick`, `swaledale`,
  `scottish-blackface`, `hebridean`, `manx-loaghtan`.
- **Lot 2** : `soay`, `shetland`, `wensleydale`, `badger-face-welsh`,
  `texel`, `zwartbles`, `heidschnucke`, `skudde`, `gotland`, `islandais`.
- **Lot 3** : `romanov`, `racka`, `karakul`, `awassi`, `dorper`,
  `barbados-blackbelly`, `navajo-churro`, `mouflon-canadien`, `argali`,
  `mouflon-de-dall`.

(Les quatre races qui remplacent le Mannequin, Aquarelle, Pelote et Lainou
ont leur propre chantier : `docs/chantiers/moutons-existants.md`.)

## Comment travailler

- Un sous-agent par race, quatre ou cinq à la fois, avec une consigne
  précise (la fiche, le contrat, ce qu'on attend ci-dessus). Relis chaque
  planche toi-même avant d'accepter une race ; renvoie le sous-agent
  corriger tant que ce n'est pas juste.
- Photos de référence dans un dossier de travail hors du dépôt, jamais
  copiées ; User-Agent générique (`MouflonResearch/1.0`), aucune donnée
  personnelle dans aucune requête.
- Ne touche qu'aux fichiers de tes races (`pets/<id>/`, `tools/races/<id>/`,
  et leur fiche si tu la corriges) : d'autres sessions dessinent en même
  temps. Le README, `docs/animation-animale.md`, `CREDITS.md` et les listes
  des tests seront mis à jour à la fin, en une fois.
- Commits par race sur `fenetres-dock` (`git pull --rebase` avant chaque
  `git push`), auteur comme le dit `CLAUDE.md`.
- À la fin, résume en français (on tutoie l'auteur) : les races dessinées,
  ce qui t'a demandé le plus de corrections, ce qui reste imparfait.
