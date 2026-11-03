# Mérinos d'Arles : un floucat de transhumance

Le dessin suit [la fiche de la race](../../../docs/races/merinos-d-arles.md) :
un mâle castré blanc de cinq à six ans, au départ de juin, tondu en velours
court. Quatre flocs restent à la nuque, au garrot, au milieu du dos et à la
croupe. Les rubans rouges et la marque de peinture sont des choix du dessin,
distincts de la pigmentation de l'animal. Il porte des cornes ouvertes, un
redon de 18 cm et un collier-lyre de bois à clavette sculptée.

Une unité du SVG vaut un centimètre. Les sons sont les huit fichiers du
Mannequin, copiés sans modification ; ils sont dans le domaine public,
déjà documentés dans [CREDITS.md](../../../CREDITS.md#sons). Le dessin et ses
sources Python sont originaux, sous MIT ou Apache-2.0, comme le dépôt.
Aucune photo, texture ni autre image de tiers n'est incorporée.

L'épaule est à 46 cm du sol : `stride = 36.8` vaut 0,5 × 1,6 × 46.
Les vitesses du Mannequin sont multipliées par 36,8 / 29 (58,4 points/s au
pas, 101,5 au trot, 190,3 au galop). La hauteur affichée est 51,2 points
pour environ 102,4 cm jusqu'au sommet du dessin ; la largeur physique de
68,3 points correspond au museau jusqu'à la fesse, hors queue.

## Régénérer et vérifier

Depuis la racine du dépôt, avec Python 3 et `shapely` (comme les autres
générateurs) :

```sh
python3 tools/races/merinos-d-arles/gen_merinos_d_arles.py pets/merinos-d-arles/merinos-d-arles.svg
```

Le SVG livré est la sortie exacte du générateur, à graine fixe. Retoucher
le générateur et le relancer ; ne pas retoucher le SVG à la main.

```sh
cargo run --release -p mouflon-preview -- pets/merinos-d-arles/pet.toml apercu/merinos-d-arles poses
cargo run --release -p mouflon-preview -- pets/merinos-d-arles/pet.toml apercu/merinos-d-arles gestes
cargo run --release -p mouflon-preview -- pets/merinos-d-arles/pet.toml apercu/merinos-d-arles allures
cargo run --release -p mouflon-preview -- pets/merinos-d-arles/pet.toml apercu/merinos-d-arles toison
cargo run --release -p mouflon-preview -- pets/merinos-d-arles/pet.toml apercu/merinos-d-arles/jeux jeux
cargo test --release -p mouflon-render --test troupeau
```

`jeux` produit 240 images : y vérifier les deux sens, les sauts et les
demi-tours. Les noms `play_back` et `back_up` reculent puis mènent à
`sniff` ou `stand` : le floucat n'enchaîne pas de duel. Sa marche est
contagieuse, ses arrêts de broutage courts et ses alarmes rares. Les jeux
sont peu fréquents, réservés au tempérament actif.

Le test `troupeau` découvre automatiquement ce dossier : chargement,
simulation, parties articulées, cuisson, sabots au sol et museau dans
l'herbe. Les vérifications générales restent celles de `CLAUDE.md`.

## Références de relecture

Photos consultées sur le web et conservées uniquement hors du dépôt :

- [Floucat tondu de profil, P10](https://www.sonnailles.net/wp-content/uploads/2013/04/04-12f.jpg) :
  proportions, flocs irréguliers, corne ouverte, place du collier et du redon.
  L'animal photographié est sombre ; le dessin représente un blanc.
- [Tête de bélier Mérinos d'Arles, P5](https://commons.wikimedia.org/wiki/File:M%C3%A9rinos_d_arles02_SDA2011.JPG) :
  chanfrein, laine des joues, sillons et matière de la corne. Celle du
  floucat est plus fine et moins enroulée que celle du bélier entier.
- [Troupeau tondu à Saint-Rémy, P11](https://www.francetraveltips.com/wp-content/uploads/2017/06/St-Remy-Transhumance2.jpg) :
  velours serré, couleur crème, petites oreilles et peinture rouge.
- [Redon et collier-lyre, P25](https://www.sonnailles.net/wp-content/uploads/2013/04/02-9db.jpg) :
  bois, aile évasée, clavette, cloche ventrue à bouche resserrée.
- [Brebis d'un an, P8](https://commons.wikimedia.org/wiki/File:Brebis_m%C3%A9rinos_d%27Arles_%C3%A2g%C3%A9e_d%27un_an.jpg) :
  canons blancs, laine des genoux et jarrets, petits onglons clairs.

La fiche détaille les sources du standard et les choix estimés (taille du
floucat, couleur des rubans, date de tonte). Les photographies de pastrage
en toison longue servent au collier et aux cornes, pas au volume du corps
tondu de juin.

## Limites du rig actuel

Le collier et le redon suivent l'encolure. Le moteur n'a pas de rôle
pendulaire pour la sonnaille : celle-ci s'incline avec le cou au broutage
et au sommeil, alors qu'une vraie cloche reste suspendue sous la gravité.
Le battant est dessiné, sans mouvement indépendant. Les huit sons copiés
ne comprennent pas le son grave d'un redon ; aucune clochette aiguë n'est
substituée. Cette tâche ne change pas le moteur.

Le rendu cuit laisse aussi un léger halo clair près du raccord de
l'encolure lorsqu'elle descend brouter : son éclairage utilise la
silhouette globale de la toison. Les essais de recouvrement qui le
réduisent rendent le cou anguleux ou trop mince ; le profil et ses
articulations sont donc conservés. Ce halo appartient au rendu du moteur,
pas au SVG vectoriel.

Les plis, la frisure, les stries de corne et les gravures se fondent à la
taille habituelle du compagnon. Les mèches sont des morceaux rigides
animés par le moteur ; leur mouvement ne simule pas chaque fibre. Le
profil gauche est obtenu par symétrie, comme les autres races. Le rendu
Retina, les sons et l'usage sur les fenêtres restent à revoir sur un vrai
Mac.

Le dossier ajoute la nouvelle race ; le remplacement de Pelote dans
l'app, ses préférences et son icône appartient à un autre chantier.
