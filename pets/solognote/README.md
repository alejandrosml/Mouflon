# Solognote

Une brebis adulte de 65 cm au garrot, en toison de fin d'été : tête et
pattes châtain roux, laine bise, gorge et bande de ventre nues, longue queue
laineuse. Le dessin suit la [fiche de race](../../docs/races/solognote.md).
Ses morceaux PNG transparents sont peints par
[`tools/mouflon-peintre`](../../tools/mouflon-peintre) et animés par
`squelette.toml`. Les 27 PNG sont des créations originales pour Mouflon,
créditées à Alejandro San Martin Lamas et dédiées au domaine public sous
[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/), conformément
aux consignes sur les images ajoutées. Le code et la configuration restent
sous MIT ou Apache-2.0, au choix.

Depuis la racine du dépôt :

```sh
cargo run --release -p mouflon-peintre -- --solognote pets/solognote/morceaux
cargo run --release -p mouflon-peintre -- --solognote pets/solognote/morceaux --apercu apercu/solognote/profil.png
cargo run --release -p mouflon-preview -- pets/solognote/pet.toml apercu/solognote poses
cargo run --release -p mouflon-preview -- pets/solognote/pet.toml apercu/solognote gestes
cargo run --release -p mouflon-preview -- pets/solognote/pet.toml apercu/solognote allures
cargo run --release -p mouflon-preview -- pets/solognote/pet.toml apercu/solognote jeux
cargo run --release -p mouflon-preview -- pets/solognote/pet.toml apercu/solognote toison
cargo test --release -p mouflon-peintre --test solognote
sh tools/mouflon-peintre/verifie-solognote.sh
```

Le premier test repeint les deux animaux hors du dépôt et compare chaque
PNG livré octet pour octet : il protège aussi le rendu d'Aquarelle. Le
second contrôle le squelette avec le vrai moteur (articulations, appuis,
broutage, pupille et limites du dessin dans les deux sens). Il construit
ses tests dans un dossier temporaire pour garder le périmètre de ce
chantier et le fichier de dépendances partagé intacts.

Les planches de l'aperçu sont des documents de travail ; elles ne sont pas
livrées avec l'animal. Les photos sont consultées comme références, jamais
copiées dans le dépôt :

- [GEODE, la race et sa galerie](https://www.geodesheep.com/fr/solognot-426.html).
- [GEODE, standard de la race](https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Standard%20de%20race%20FBS.pdf).
- [Wikimedia Commons, profil « Mouton solognot (3) »](https://commons.wikimedia.org/wiki/File:Mouton_solognot_(3).JPG).
- [GEODE, tête et encolure au broutage haut](https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot3.JPG).

`pet.toml` favorise les longues marches, les arrêts pour regarder et le
reniflement. Les jeux sont proposés surtout aux actifs, les charges et le
flehmen restent rares. Le cabré est gardé pour vérifier le squelette dans
l'aperçu ; aucune transition spontanée n'y mène, car il n'est pas documenté
comme geste de cette race. Le broutage dans les ligneux, tête haute, décrit
dans la fiche, ne dispose pas encore d'un mouvement propre dans le moteur.
Les fréquences et les durées sont des choix d'animation, pas des mesures
éthologiques de la Solognote.

Les huit sons sont des copies exactes de ceux du Mannequin : enregistrements
de Joseph Sardin, BigSoundBank, publiés sous CC0 1.0 (moutons n° 2343 à
2346, broutage et mastication n° 0990 et 0497, sauts dans l'herbe n° 1839
et 1840). Le détail est dans [CREDITS.md](../../CREDITS.md). Aucune cloche.

Ce dossier prépare le remplacement d'Aquarelle. Le renommage et le choix
des animaux dans l'app feront l'objet d'une autre tâche.
