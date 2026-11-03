# Islandais : un bélier meneur

Le dessin suit [la fiche](../../../docs/races/islandais.md) : un bélier de
trois ans, noir « à tête d'aigle », en septembre. La robe et les cornes
viennent de Jökull 22-978 ; les proportions de Strumpur 14-815, dont la
toison moins longue laisse voir les pattes. Ce n'est donc pas le type blanc
et trapu sélectionné pour la viande.

## Refaire le dessin

Depuis la racine du dépôt, avec Python 3 et Shapely (déjà utilisé par les
autres générateurs) :

```sh
python3 tools/races/islandais/gen_islandais.py pets/islandais/islandais.svg
```

Sans argument, le script écrit aussi ce fichier, quel que soit le dossier
courant. La graine est fixe (`22978`). Le SVG livré est sa sortie exacte :
les retouches se font dans le générateur. `outils.py` reprend les outils
d'écriture de la Thônes et Marthod, avec le repère de cet animal.

Une unité vaut un centimètre ; le sol est à `h = 0`. Le garrot nu est à
80 cm, la laine vers 84 cm. La frange descend vers 25 cm ; carpes à 19,3 cm,
jarrets à 25,2 cm, tête de 29 cm. Les limites mesurées par le lecteur du
moteur sont de 108,7533 cm en hauteur et 131,4820 cm en largeur. `height`
et `width` valent la moitié de ces mesures ; `stride = 39.2` correspond à
1,6 × 49 cm de patte × 0,5 point/cm.

Le cou est une pièce cachée sous le corps ; la tête tourne à la nuque.
Les quatre pattes sont recoupées par `articulations.articulate_breed` au
carpe et au jarret. La mâchoire, les oreilles, les pupilles horizontales,
la lèvre avec sa gencive et trois bouquets de mèches ont leurs pivots.
La queue naturelle de 18 cm est presque entièrement cachée par la culotte.
Aucun masque SVG, découpe, filtre ni groupe semi-transparent. Le moteur
cuit le rig en 25 morceaux.

## Références regardées

Les photos servent uniquement à comparer, elles ne sont pas dans le dépôt :

- [RML, Hrútaskrá 2025–2026](https://www.rml.is/static/files/RML_saudfjarraekt/kynbotastarf/hrutaskra/hrutaskra-2025-2026-vef.pdf),
  page imprimée 28 (page 32 du PDF) : profil et tête de Jökull, profil de
  Völundur ; masque noir, liste blanche, bavette et cornes ouvertes.
- [ISBONA, The Iceland Breed of Leadersheep](https://www.isbona.com/images/pdf/newsletterarticles/LS_Article.pdf),
  page imprimée 22 : Strumpur, proportions du tronc, pattes et balzanes.
- [Meneuse au pré](https://www.forystusetur.is/static/gallery/forystufe/md/asta_spitali-39-of-40-.jpg) :
  silhouette haute sur pattes et port du cou.
- [Tête de meneuse](https://www.forystusetur.is/static/gallery/forystufe/md/asta_spitali-4-of-40-.jpg) :
  limites du masque, oreille en feuille et museau noir. Le bleu du flash
  n'est pas la couleur de l'iris.
- [Groupe de meneurs](https://www.forystusetur.is/static/gallery/forystufe/md/er-etta-forystuf-1.jpg) :
  toisons irrégulières, pattes fines et variation des cornes.

Les corrections principales ont porté sur les mèches (racines fondues,
longueurs irrégulières, ondulation lâche et pointes roussies), la bavette,
l'insertion des cornes, le jarret, les petits onglons et la queue cachée.

## Revoir les articulations

```sh
cargo run -p mouflon-preview --release -- pets/islandais/pet.toml apercu/islandais poses
cargo run -p mouflon-preview --release -- pets/islandais/pet.toml apercu/islandais gestes
cargo run -p mouflon-preview --release -- pets/islandais/pet.toml apercu/islandais jeux
cargo run -p mouflon-preview --release -- pets/islandais/pet.toml apercu/islandais allures
cargo run -p mouflon-preview --release -- pets/islandais/pet.toml apercu/islandais toison
cargo test --release -p mouflon-render --test troupeau
```

`jeux` écrit une séquence de 240 images ; comparer plusieurs instants du
recul, de la charge, du saut tordu et du demi-tour, dans les deux sens.
`poses` inclut des vues vers la gauche. Les planches restent hors du dépôt.

Les huit sons sont les copies exactes de ceux du Mannequin, du domaine
public (voir `CREDITS.md`). Aucune sonnaille : la fiche n'en documente pas.
Les quarante comportements du Mannequin sont conservés ; les réglages
favorisent la vigilance, la marche contagieuse et une fuite brève suivie
du retour au broutage. Les durées sont des choix d'animation.

## Ce qui reste imparfait

Le dessin est une interprétation de profil des photographies : les
longueurs et la panachure varient entre individus. Les mèches restent
stylisées ; les bouquets du ventre bougent, mais la bavette et la toison
du cou tournent avec l'encolure au lieu de pendre selon la gravité. La
spirale vue de profil ne rend pas tout l'écartement latéral des cornes.
Le cou reste assez droit dans les poses extrêmes. À fort grossissement,
les cornes et le toupet gardent un aspect géométrique ; une petite forme
pâle apparaît derrière l’épaule dans certaines poses basses vers la gauche.

Le moteur ne distingue pas un chef de troupeau et ne reproduit ni sa
hiérarchie, ni sa familiarité avec une personne, ni le tölt : la contagion
de la marche en donne seulement une approximation. La météo anticipée
par les meneurs relève des récits, elle n'est pas simulée.
