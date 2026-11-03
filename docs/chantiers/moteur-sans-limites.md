# Chantier : un moteur sans limites de mouvement

*Demande de l'auteur, 3 novembre 2026 : « améliore le moteur pour qu'il n'y
ait plus de limites dans les mouvements des moutons ». Ce chantier se fait
dans une session cloud de Claude Code, sur la branche `fenetres-dock`.*

> **Fait le 3 novembre 2026** : l'encolure, les pattes en deux segments, la
> lèvre et le flehmen, la toison qui bouge ; le Mannequin, Aquarelle et les
> quatre races articulés. Le contrat des nouvelles parties est dans
> `docs/rig.md` et `docs/squelette.md` ; ce que disent les études et ce qui
> reste (la tétée, le dos qui plie : essayé, pas propre), dans
> `docs/animation-animale.md` (§ 1, 2, 3, 10, 11).

Lis d'abord `CLAUDE.md` (les consignes du dépôt : vérifications avant
chaque commit, conventions, auteur des commits), puis `docs/rig.md`,
`docs/squelette.md`, `docs/animation-animale.md` (surtout le § 10, les
quatre races, et le § 11, ce qui reste) et `docs/architecture.md`.

## Ce qui limite aujourd'hui

Vu en dessinant les quatre races réelles (`pets/mouflon-corse`,
`pets/nez-noir`, `pets/jacob`, `pets/ouessant`) :

- **Le cou ne se plie pas.** La tête tourne d'un bloc autour de
  `pivot-head` : pour brouter, elle descend toute raide et le museau
  s'arrête au-dessus du sol ; le mouflon a dû mettre son encolure dans le
  groupe `head` pour que la tête ne s'en détache pas, et son museau reste à
  5 cm du sol.
- **Les pattes sont raides** : une seule pièce de l'épaule (ou de la
  hanche) au sabot. Au pas, au trot et au galop, elles balancent comme des
  compas, sans plier le genou (le carpe) ni le jarret ; couché, l'animal les
  cache sous lui ou elles dépassent comme des bâtons ; un saut ne les replie
  pas vraiment.
- **Le corps est d'une pièce** : pas de dos qui s'arrondit (s'étirer, ruer,
  bondir, se coucher).
- **La toison ne bouge pas** (vent, course, secousse), alors que les
  oreilles et la queue ont leurs ressorts.
- **Pas de lèvre** : ni flehmen (le bélier en rut, lèvre retroussée), ni
  tétée.

## Ce qu'il faut faire

Règle d'or : **rien ne change pour un dessin qui n'a pas les nouvelles
parties** (même pose, mêmes images cuites, au pixel près : vérifie-le par
un test sur le Mannequin tel qu'il est aujourd'hui, avant de le redécouper).

1. **Le cou articulé.** Une partie `neck` (rig SVG) et un rôle d'os `neck`
   (squelette peint), entre le corps et la tête : quand il existe, `head`
   est dans `neck`. `pivot-neck` à la base de l'encolure (devant le garrot),
   `pivot-head` à la nuque. Le moteur répartit ce que fait la tête entre
   les deux articulations : pour brouter, c'est surtout l'encolure qui
   descend, et une petite cinématique inverse à deux articulations pose le
   museau au sol (au bon endroit, devant les antérieurs) ; tête haute aux
   aguets, l'encolure se redresse ; en dormant la tête au sol, l'encolure
   s'allonge vers l'avant ; en chargeant, elle se tend, tête basse ; le
   regard (le curseur) passe surtout par la tête. Donne des angles tirés de
   ce qu'on sait du mouton (mesures ou vidéos de moutons qui broutent,
   documentées dans `docs/animation-animale.md` avec leurs sources).

2. **Les pattes en deux segments.** Des parties `leg-front-near-lower`,
   `leg-front-far-lower`, `leg-back-near-lower`, `leg-back-far-lower`, chacune
   dans sa patte, avec leur pivot au carpe (antérieurs) ou au jarret
   (postérieurs) ; un rôle d'os pour le bas de patte dans les squelettes.
   Le moteur les plie comme un mouton :
   - au pas, au trot, au galop : le carpe se fléchit pendant le vol (le
     canon part vers l'arrière), le jarret aussi ; à l'appui, la patte est
     presque droite ; les sabots restent posés au sol (cinématique inverse à
     deux os, le pas reste celui de `stride`, aucun glissement : il y a déjà
     des tests de glissement, étends-les) ;
   - en sautant : pattes repliées ; à l'atterrissage, elles amortissent ;
   - **couché** : les antérieurs repliés sous le poitrail (carpes pliés à
     fond, canons sous le corps), les postérieurs repliés le long du ventre,
     visibles comme sur un vrai mouton couché ; il se couche les genoux
     d'abord et se relève par l'arrière (l'ordre existe déjà dans les
     comportements) ;
   - se cabrer, ruer, gratter le sol (l'antérieur qui se plie et gratte),
     taper du pied, se gratter d'un postérieur : chaque geste plie les bonnes
     articulations.

3. **Le dos qui plie**, seulement si c'est propre : par exemple un
   arrière-train (`rump`) dans `body`, avec son pivot au milieu du dos, pour
   s'étirer, ruer, bondir, se coucher. Si les jointures se voient (la toison
   coupée en deux), ne le fais pas et écris pourquoi dans la doc.

4. **La toison qui bouge.** Des parties secondaires (par exemple `sway-1`,
   `sway-2`… dans `body`) qui suivent le corps avec un ressort amorti : elles
   traînent quand il court, se balancent quand il s'ébroue, penchent dans le
   vent ; même principe que les oreilles et la queue en chaîne.

5. **La lèvre.** Une partie `lip` (la lèvre supérieure, dans `head`) et un
   geste `flehmen` (le bélier en rut : encolure tendue, tête relevée, lèvre
   retroussée, quelques secondes) ; un comportement qui l'utilise dans le
   Mannequin et Aquarelle. La tétée (un agneau sous sa mère) si c'est
   faisable.

6. **Les dessins du dépôt.** Le Mannequin (`pets/mannequin/mannequin.svg`)
   devient l'exemple des nouvelles parties : redécoupe son SVG (encolure,
   bas des pattes, lèvre…) sans changer son allure. Aquarelle : ajoute les
   os et les morceaux peints qu'il faut (`tools/mouflon-peintre`,
   `pets/aquarelle/squelette.toml`). Les quatre races : adapte leurs
   générateurs (`tools/races/*`, voir son README) pour qu'ils sortent
   l'encolure et les bas de pattes, régénère, et regarde le résultat dans
   l'aperçu. D'autres sessions redessineront ensuite les autres moutons
   d'après de vraies races : écris le contrat des nouvelles parties
   clairement dans `docs/rig.md` et `docs/squelette.md`, avec un exemple.

7. **Regarder ce qu'on dessine.** Les planches de l'aperçu
   (`cargo run -p mouflon-preview --release -- pets/<animal>/pet.toml <dossier> poses`,
   `gestes`, `jeux`) doivent montrer les nouvelles articulations (couché les
   pattes repliées, brouter le museau au sol, galop les jarrets pliés,
   flehmen). Regarde chaque planche avant de dire que c'est juste, et compare
   avec des photos de vrais moutons.

8. **Le coût.** Les nouvelles parties sont cuites en images comme les
   autres ; le banc `crowded` (environ 1,24 ms par pas pour mille animaux)
   ne doit pas monter sensiblement, ni le coût du dessin par image.

9. **Tests et docs.** Chaque nouveauté a ses tests (noms en anglais, en
   phrases) ; `docs/rig.md`, `docs/squelette.md`, `docs/format.md` (s'il y a
   de nouvelles clés), `docs/animation-animale.md` (ce que disent les études,
   avec les sources, et le § 11 mis à jour), le README (anglais et
   français) si ça se voit pour l'utilisateur.

## Comment travailler

- Commits sur `fenetres-dock`, auteur et vérifications comme le dit
  `CLAUDE.md` (fmt, clippy Linux et `aarch64-apple-darwin`, tests en
  release, banc `crowded`). Des commits par étape, poussés au fur et à
  mesure, pour qu'on puisse suivre.
- Pour toute recherche sur le web, ne mets aucune donnée personnelle dans
  une requête (pas d'adresse e-mail dans un en-tête : un User-Agent
  générique comme `MouflonResearch/1.0`). Les photos trouvées servent de
  référence seulement : jamais copiées dans le dépôt.
- Ne dessine ni ne reproduis jamais un personnage protégé (Poe de *Stray
  Sheep*, le mouton d'eSheep) ; `animaux/` n'est jamais dans le dépôt.
- À la fin, résume en français (on tutoie l'auteur) : ce qui est fait, les
  planches regardées, ce qui reste, et les commits.
