# Chantier : la suite du moteur

*Demande de l'auteur, 3 novembre 2026 : finir ce qui reste du moteur, et
l'améliorer encore là où c'est possible. Confié à Codex (ChatGPT). Le
chantier précédent, fait le 3 novembre, est
`docs/chantiers/moteur-sans-limites.md`.*

Lis d'abord `AGENTS.md` et `CLAUDE.md`, puis `docs/architecture.md`,
`docs/rig.md`, `docs/squelette.md`, `docs/format.md`,
`docs/animation-animale.md` (surtout le § 11, ce qui reste, et ses
sources) et `docs/chantiers/moteur-sans-limites.md`.

Règle d'or, inchangée : un dessin sans les nouvelles parties bouge et se
cuit exactement comme avant (le test `sans_changement`), et ni le banc
`crowded` (environ 1,4 ms par pas pour mille animaux) ni le banc `cout` ne
montent sensiblement. Rien de bloquant sur le fil principal ni sur celui de
l'affichage ; `mouflon-core` reste déterministe et sans `unsafe`.

## Dans l'ordre

1. **Relire le cœur avec un œil neuf** (`crates/mouflon-core`, puis
   `crates/mouflon-render`) : bogues, cas limites (bords d'écran, fenêtres
   qui bougent ou disparaissent sous un animal, mille animaux en tas, pas de
   temps irréguliers, très grands et très petits animaux), déterminisme,
   coût. Corrige ce qui est faux, avec un test pour chaque correction ; écris
   le reste dans la PR. Pas de refonte gratuite.
2. **Bélier, brebis, agneau.** Le moteur ne sait pas encore qui est bélier
   ou brebis (§ 11). Donne un sexe à chaque adulte du troupeau (tiré au
   hasard, de façon déterministe ; une clé de `pet.toml` pour le fixer ou le
   pondérer, documentée dans `docs/format.md`) et sers-t'en : le flehmen
   seulement chez le bélier, après avoir flairé une brebis plutôt que
   n'importe quel mouton ; les charges, et le recul avant de charger,
   surtout entre béliers.
3. **La mère et son agneau, la tétée** (§ 11) : un lien entre une brebis et
   son agneau (le troupeau ne connaît que des familles), l'agneau qui suit
   sa mère, la brebis qui l'appelle (un bêlement grave, bouche fermée), et
   la tétée : l'agneau se glisse sous le flanc de sa mère, à l'envers
   d'elle, la tête levée vers la mamelle, et frétille de la queue ; durées et
   fréquences d'après Gordon & Siegmann 1991 (déjà dans les sources). Une
   rencontre qui place l'agneau, un geste qui lève sa tête, aucun sabot qui
   glisse.
4. **Les mèches d'encolure du Nez noir** tournent avec l'encolure quand il
   broute, au lieu de pendre (§ 11) : traite-les comme les mèches du ventre
   (`sway-*`, voir `tools/races/README.md`), ou mieux, dans son générateur
   (`tools/races/nez-noir/`), puis régénère son SVG.
5. **Les cris du mouflon** (§ 11) : son sifflement d'alarme, bref, par les
   naseaux, souvent avec un coup de patte, et son grondement en rut. Des
   sons CC0 ou du domaine public seulement, licence vérifiée, crédités dans
   `CREDITS.md` ; si tu n'en trouves pas, fabrique-les par synthèse et dis-le
   dans `CREDITS.md`.
6. **Le dos qui plie** (§ 11 : essayé en coupant le corps en deux, pas
   propre). Si tu vois une façon propre (déformer le dessin du corps le long
   du dos, par un maillage, sans coût sensible), fais-la pour s'étirer,
   ruer, bondir, se coucher ; sinon, écris pourquoi dans le § 11.
7. **Brebis et agneaux de chaque race**, si le temps le permet : une
   variante de dessin par sexe et par âge, choisie d'après l'animal (une clé
   de `pet.toml`, documentée) ; pour les quatre races du dépôt d'abord (la
   brebis d'Ouessant sans cornes, l'agneau Jacob avec sa tête de blaireau,
   la brebis Nez noir aux cornes bien plus petites que celles du bélier),
   par leurs générateurs.
8. **Tout ce qui rendrait les mouvements plus naturels encore**, d'après
   des études ou des vidéos de vrais moutons : propose, fais ce qui est sûr,
   note le reste dans le § 11.

Chaque étape a ses tests (noms en anglais, en phrases), sa doc
(`docs/animation-animale.md` avec ses sources, `docs/format.md` pour une
clé, `docs/rig.md` ou `docs/squelette.md` pour une partie), et ses planches
de l'aperçu regardées (`apercu poses`, `gestes`, `jeux`, `allures`,
`toison`), sur le Mannequin et une ou deux races.

## Ce que tu ne touches pas

Les dessins des races (`pets/<race>/`, `tools/races/<race>/`), sauf le Nez
noir à l'étape 4 et les quatre races du dépôt à l'étape 7 : d'autres tâches
dessinent en même temps. Les moutons existants qui deviennent de vraies
races sont une autre tâche aussi (`docs/chantiers/moutons-existants.md`).

## À la fin

Une PR vers `fenetres-dock`, décrite en français : ce qui est fait, étape
par étape, les planches regardées, les bancs avant et après, ce qui reste.

Le [bilan de réalisation](moteur-suite-bilan.md) détaille les résultats,
les vérifications et les limites conservées.
