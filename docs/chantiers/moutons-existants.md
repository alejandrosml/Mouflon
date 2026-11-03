# Chantier : les moutons de Mouflon deviennent de vraies races

*Demande de l'auteur, 3 novembre 2026 : tous les moutons existants doivent
être redessinés d'après de vraies races, au moins au niveau de détail des
quatre races (`pets/mouflon-corse`, `pets/nez-noir`, `pets/jacob`,
`pets/ouessant`), et porter leur vrai nom. Seul Poe (Poete), qui n'est pas
dans le dépôt, reste un personnage : n'y touche pas, ne le dessine pas. Une
cloche seulement pour une race qui en porte vraiment. À lancer après le
chantier du moteur (`docs/chantiers/moteur-sans-limites.md`).*

Lis d'abord `CLAUDE.md`, `docs/rig.md`, `docs/squelette.md`,
`tools/races/README.md`, le § 10 de `docs/animation-animale.md`, et les
fiches `docs/races/bleu-du-maine.md`, `solognote.md`, `merinos-d-arles.md`,
`southdown.md`.

## Les quatre remplacements

| aujourd'hui | devient | ce qui change |
|---|---|---|
| le Mannequin (`pets/mannequin`, rig SVG, intégré à l'app) | **Bleu du Maine** | un rig dessiné d'après la fiche, au niveau des quatre races, par un générateur dans `tools/races/bleu-du-maine/` ; il reste l'exemple complet du format (tous les gestes, tous les choix selon le tempérament) et l'animal intégré à l'app |
| Aquarelle (`pets/aquarelle`, squelette peint) | **Solognote**, peinte à l'aquarelle | ses morceaux repeints d'après la race par `tools/mouflon-peintre` (le style aquarelle reste : c'est sa raison d'être) |
| Pelote (`pets/pelote`, planche desktopPet) | **Mérinos d'Arles** | un rig dessiné ; les troupeaux transhumants portent des sonnailles : il peut garder une cloche |
| Lainou (`pets/lainou`, bandes d'images) | **agneau Southdown** | un rig dessiné, un agneau (jeux, sauts, voix d'agneau) |

Choisis les identifiants (`bleu-du-maine`, `solognote`, `merinos-d-arles`,
`southdown`) et renomme partout ce qui en dépend :

- **L'app** : l'animal intégré, le premier lancement (aujourd'hui Pelote,
  puis Lainou), l'icône de l'app (aujourd'hui la tête de Pelote :
  `packaging/macos/AppIcon.svg` et `.png` ; `mouflon-preview … icon` sait
  rendre une tête), les réglages gardés d'une version à l'autre (un
  `settings.toml` qui nomme `lainou` ou `pelote` doit encore marcher).
- **Les tests** : beaucoup nomment `lainou`, `pelote` ou `mannequin`. Les
  tests qui vérifient le format desktopPet et les bandes d'images doivent
  continuer d'exister : garde les anciennes planches de Pelote et de Lainou
  **comme données de test seulement** (hors de `pets/`, pour que l'app ne
  les montre plus), ou remplace-les par de petits exemples équivalents.
- **Les essais** : `tools/essai/Essai de 3 minutes.command` part de
  `lainou,pelote` ; la CI aussi (`.github/workflows/ci.yml`). Choisis deux
  races à la place.
- **La doc** : README (anglais et français : les mascottes, le premier
  lancement), `docs/format.md` (l'exemple du Mannequin), `docs/rig.md`,
  `docs/squelette.md`, `docs/animation-animale.md`, `CREDITS.md`,
  `CLAUDE.md`.

## Ce qu'on attend

Comme dans `docs/chantiers/races-dessins.md` : un générateur par rig, toutes
les parties du contrat (encolure, bas de pattes, lèvre, mèches), un
`pet.toml` réglé sur le tempérament de la race, une planche de contrôle et
les planches `poses`, `gestes`, `jeux` regardées et comparées à de vraies
photos, les tests, fmt et clippy, le banc `crowded`. Commits par étape sur
`fenetres-dock` (`git pull --rebase` avant chaque `git push`). Photos de
référence hors du dépôt, User-Agent générique, aucune donnée personnelle
dans aucune requête.

À la fin, résume en français (on tutoie l'auteur) : ce qui a changé de nom,
les dessins, ce qui reste à vérifier sur le vrai Mac.
