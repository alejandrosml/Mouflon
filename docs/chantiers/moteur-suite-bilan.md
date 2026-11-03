# Bilan du chantier moteur-suite

Travail fondé sur `origin/fenetres-dock`, commit
`e7132321d1dbc82def0d575533799a19e40b9047` ; la PR vise `fenetres-dock`.
Les références photographiques sont restées hors du dépôt.

## Étapes réalisées

1. **Audit** : les rebords partiellement hors écran ou répartis sur plusieurs
   écrans restent utilisables, sans faux bord sur deux écrans contigus.
   Les temps non finis, tailles invalides et animaux plus larges que leur
   support ne corrompent plus la simulation. Les contours SVG transformés
   ou à jointures aiguës ne sont plus coupés à la cuisson. Chaque correction
   a une régression ciblée ; les poses historiques restent identiques.
2. **Sexe et âge** : sexe déterministe indépendant du tirage des mouvements,
   proportion ou sexe fixé dans `pet.toml`, âge déclaré et conditions de
   comportement documentées. Le flehmen exige un bélier adulte ayant flairé
   une brebis ; charges et recul favorisent les rencontres entre béliers.
3. **Mère et agneau** : lien par identifiants stables, suivi sur le même
   support, approche en marchant, orientation inverse, arrêt des sabots,
   tête levée sous le flanc et queue frétillante. Appel grave bouche fermée.
   Gordon & Siegmann (1991) fournit les moyennes de durée et de fréquence ;
   le temps n'est pas accéléré. Le menu ajoute un jeune de deux semaines ;
   son dessin est préparé sur un fil dédié. Retrait de mère, chute, prise,
   regroupement, pauses, redimensionnement et réponses de chargement
   tardives sont couverts.
4. **Nez noir** : trois mèches d'encolure facultatives dont la racine suit
   le cou et dont la chute reste soumise à la verticale, avec vent et
   secousse. Le générateur produit le SVG ; aucune autre race n'a été
   redessinée.
5. **Sons** : alarme nasale et évocation de grondement du mouflon, plus
   appel maternel, synthétisés de façon reproductible, originaux et CC0.
   Leur provenance et la divergence des sources sur le cri de rut sont
   explicites dans `CREDITS.md` et `tools/audio/README.md`.
6. **Dos** : examen du rendu et raisons du report dans le § 11 de
   `docs/animation-animale.md`. Aucun découpage à coutures ni coût de
   déformation non mesuré n'a été introduit.
7. **Variantes facultatives** : non réalisées. Les races conservent leur
   dessin adulte, avec le sexe correspondant déclaré. Un jeune réduit ne
   constitue pas encore une anatomie juvénile validée.
8. **Mouvements naturels** : priorité au contact pendant la tétée, à ses
   appuis et aux interruptions sans déplacement instantané. Les autres
   pistes demandant des références vidéo sont consignées au § 11.

## Vérifications

- `cargo fmt --all` et `git diff --check` : propres.
- `cargo clippy --workspace --all-targets -- -D warnings` : réussi.
- `cargo clippy --workspace --all-targets --target aarch64-apple-darwin -- -D warnings` : réussi depuis Linux, sans liaison macOS.
- `cargo test --workspace --release` : **370 réussis, aucun échec**, 11 essais ignorés par défaut (fichiers desktopPet privés ou bancs manuels). `crowded` et `cout` sont exécutés séparément ci-dessous.
- Les **8 tests GPU ont réellement tourné**, avec 148 comparaisons CPU/GPU détaillées, sur Vulkan logiciel Mesa/lavapipe. Cela vérifie le rendu GPU ; cela ne mesure pas le compositeur Core Animation du Mac.
- `sans_changement` conserve les anciennes poses et cuissons ; les régressions ciblées couvrent limites physiques, déterminisme, sexes, familles, chargements asynchrones, tétée, contours SVG et mèches.
- `python3 -m unittest discover -s tools/audio -v` : **5 tests réussis**, dont la reproduction exacte des trois WAV livrés et l'absence de saturation.
- Régénération du Nez noir identique octet pour octet au SVG livré ; aucun autre dessin de race modifié.

## Coûts avant / après

Référence : `e713232` sur `fenetres-dock`. Linux, même environnement et même
profil release, **six paires avant/après sur un même cœur**, ordre alterné,
sans compilation ni production d'images en parallèle. Les deux versions
des bancs sont compilées avec les mêmes commandes par crate (`-p`), pour
garder les mêmes fonctionnalités Cargo :

```sh
cargo test --release -p mouflon-core --test charge -- --ignored --nocapture --exact crowded
cargo test --release -p mouflon-render --test cout -- --ignored --nocapture
```

Les six comparaisons relancent les exécutables de tests ainsi produits.
Le palier « 1 000 » contient effectivement 1 010 animaux dans les deux
versions. `cout` affiche ses échantillons au dixième de microseconde.
Une garde évite de préparer les pattes articulées sur un dessin qui n'en
possède pas ; `sans_changement` confirme que les poses restent identiques.

| Banc | Avant : médiane [min–max] | Après : médiane [min–max] |
|---|---:|---:|
| crowded, 20 animaux (ms/pas) | 0,005 [0,005–0,006] | 0,006 [0,006–0,019] |
| crowded, 100 animaux (ms/pas) | 0,033 [0,032–0,034] | 0,034 [0,033–0,055] |
| crowded, 500 animaux (ms/pas) | 0,365 [0,359–0,391] | 0,364 [0,362–0,379] |
| crowded, 1000 animaux (ms/pas) | 1,233 [1,202–1,481] | 1,240 [1,211–1,689] |
| cout, une pièce (µs/image) | 1,50 [1,40–1,50] | 1,50 [1,50–1,50] |
| cout, articulé (µs/image) | 3,20 [3,20–3,30] | 3,30 [3,20–3,30] |

## Limites restantes

Les dessins de jeunes et de brebis des quatre races, le dos déformable,
le vieillissement et une biologie plus complète restent des travaux
séparés. Les repères mammaires sont des conventions graphiques ; les
mesures de sept brebis Mérinos ne deviennent pas des données mesurées
pour toutes les races. Les tests Linux, même avec rendu Vulkan, ne
remplacent pas l'écoute et les essais des fenêtres sur le vrai Mac.

## Revue des images

Les cinq aperçus `poses`, `gestes`, `jeux`, `allures`, `toison` ont été
produits avant et après sur le Mannequin, le Nez noir et le mouflon de
Corse. Les planches ont été regardées, avec agrandissement du broutage
pour les mèches. Les jeux représentent 240 images par animal et par
version ; des contacts de douze instants ont servi à lire recul, choc,
bond, pivot et réception.

Les huit planches classiques du Mannequin et du Corse, ainsi que leurs
480 images de jeux, sont identiques octet pour octet avant et après.
Les différences du Nez noir viennent de ses nouvelles touffes. Les deux
joueurs se superposent parfois dans le scénario de jeux, qui force les
gestes sans collisions ; cette limite était déjà visible avant le chantier.

Le nouvel aperçu `famille` a été regardé sur le Mannequin, le Nez noir,
le Corse et Aquarelle, à deux et sept semaines et dans les deux sens.
La revue a conduit à relever la région mammaire implicite et à déployer
le cou du jeune : un simple test de contact ne suffisait pas à exclure
une tête retournée près du sol. Une dernière correction garde la cible
pendant le fondu de sortie, pour éviter une rupture de pose. Les clés
structurelles des squelettes reviennent elles aussi progressivement. Une
planche supplémentaire de cinq instants autour de la fin de tétée a été
regardée sur le Mannequin, le Nez noir et Aquarelle, dans les deux sens.

Références photographiques regardées, sans copie dans le dépôt :
[Jiel Beaumadier, Romane, 2011](https://commons.wikimedia.org/wiki/File:Jielbeaumadier_brebis_et_agneau_romane_bn_rambouillet_2011.jpeg),
[Brooke Anderson, Jacob, 2006](https://commons.wikimedia.org/wiki/File:Jacob_ewe_nursing_a_lamb.jpg),
et les vues du Nez noir citées dans `docs/animation-animale.md`.

Pour reproduire une planche, après `cargo build --release -p mouflon-preview` :

```sh
target/release/mouflon-preview pets/mannequin/pet.toml apercu/mannequin poses
target/release/mouflon-preview pets/nez-noir/pet.toml apercu/nez-noir toison
target/release/mouflon-preview pets/nez-noir/pet.toml apercu/nez-noir famille
```

Remplacer le mode ou le pack pour les autres vues. `famille.md` donne les
temps des six colonnes et précise le caractère provisoire du dessin du
jeune. Pour les rigs de béliers, cette scène force le sexe maternel pour
contrôler la mécanique, sans prétendre fournir un dessin de brebis.
