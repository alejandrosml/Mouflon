# Manech tête rousse

Un bélier cornu de trois à quatre ans, 65 cm au garrot, en pleine toison de
fin de printemps. Le choix suit la [fiche](../../../docs/races/manech-tete-rousse.md) :
les mâles peuvent être cornus, tandis que les femelles du standard sont
mottes. Ce dessin ne représente donc pas la brebis type du schéma laitier.

Le générateur écrit le SVG livré, sans retouche manuelle. Une unité vaut un
centimètre. La graine est fixe ; il n'utilise aucune image incorporée. Les
outils géométriques locaux viennent du générateur Thônes et Marthod, avec
le même contrat d'articulation que Thônes et Marthod et Lacaune.

```sh
python3 -m pip install shapely
python3 tools/races/manech-tete-rousse/gen_manech_tete_rousse.py pets/manech-tete-rousse/manech-tete-rousse.svg
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech poses
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech gestes
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech allures
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech toison
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech-jeux jeux
cargo run --release -p mouflon-preview -- pets/manech-tete-rousse/pet.toml apercu/manech planche
cargo test --release -p mouflon-render --test troupeau
```

`gen_manech_tete_rousse.py` dessine la tête, ses deux cornes, les pattes et
la queue. `toison.py` peint les longues mèches jarreuses ouvertes, le col
roussâtre et les trois paquets mobiles de la frange. La lèvre et les quatre
bas de pattes sont détachés avec l'outil commun `articulations.py`. La queue
courte reste presque entièrement sous la culotte. Les huit sons sont les
fichiers du Mannequin, identiques, déjà crédités comme domaine public dans
`CREDITS.md` ; aucune sonnaille n'est ajoutée.

Les photographies consultées, exclusivement hors dépôt :

- [Manech tête rousse de profil, Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Manech_t%C3%AAte_rousse_sheep.jpg) : silhouette longue, laine pendante, proportions, tête et onglons.
- [Troupeau et jeune bélier cornu, CDEO](https://www.cdeo64.fr/les-3-races-locales/) : port de tête et de l'encolure, implantation des cornes, aplombs.
- [Plaquette de la race, CDEO](https://www.cdeo64.fr/manech-tete-rousse/) : variations du fauve et collier roussâtre.
- [Bélier Latxa à face rousse, Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Aharia_latxa_0001.jpg) : anatomie et annelures des cornes de cette population voisine ; ce n'est pas une mesure morphométrique de la Manech française.
- [Deux Manech tête rousse, Exploreur, université de Toulouse](https://exploreur.univ-toulouse.fr/quand-la-selection-genetique-conserve-les-traditions) : orientation des nappes et sillons de la toison, transition du col.

Les dix traits de la fiche guident la relecture : mèches longues pointues,
raie dorsale, tête et pattes fauves nues, nez rose grisé, tête étroite peu
busquée, oreilles latérales un peu tombantes, col roussâtre, spirale ambrée
et corps allongé sans gigots gonflés. Le roux n'est pas un masque à bord net.

Limites : les dimensions non publiées restent celles estimées par la fiche ;
le dessin fixe un seul individu et une seule saison. Certaines mèches restent
assez régulières. La projection de la
corne en profil simplifie sa sortie vers l'extérieur. Les trois paquets de
frange bougent, mais les fibres individuelles restent peintes, et la laine
du cou tourne avec son articulation quand il broute. Il manque donc la
retombée gravitaire des mèches du col et l'aplatissement de la toison contre
le sol quand il est couché ; ces déformations demanderaient un autre rig ou
une évolution du moteur.
L'éclairage des images cuites laisse aussi une fine lisière claire sur le
haut du col abaissé vers la gauche. Les pointes qui remontaient sur le dos
ont été retirées en reprenant le recouvrement du garrot et la base cachée du
cou ; ce léger résidu d'éclairage reste visible sur le balayage cuit.
