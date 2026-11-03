# Le bélier Herdwick

Le générateur écrit un animal original, de profil droit, d'après la
[fiche de la race](../../../docs/races/herdwick.md) : un bélier de trois ans
et demi, fin octobre, trois mois après la tonte de juillet. La laine est
grise naturellement, sans rouge de concours ni marque d'élevage.

```sh
python3 -m pip install shapely
python3 tools/races/herdwick/gen_herdwick.py pets/herdwick/herdwick.svg
cargo run -p mouflon-preview --release -- pets/herdwick/pet.toml apercu poses
cargo run -p mouflon-preview --release -- pets/herdwick/pet.toml apercu gestes
cargo run -p mouflon-preview --release -- pets/herdwick/pet.toml apercu jeux
```

Python 3 et Shapely suffisent au générateur. `outils.py` reprend les outils
de courbes, de régions et d'écriture SVG de la Lacaune, avec le repère du
Herdwick ; les routines de poil ras, d'œil et de bouche en dérivent aussi.
Le canon de tête, les membres, les cornes, la toison, la crinière et la
queue sont propres à ce dessin. Les graines aléatoires sont fixes : la
sortie doit être identique octet pour octet. On retouche le script et on le
relance, jamais le SVG à la main.

`pet.toml` conserve les quarante comportements du Mannequin, avec davantage
de vigilance, des départs brefs suivis d'un arrêt pour regarder et une
marche moins contagieuse (petits groupes dispersés). Les huit sons sont des
copies exactes de ceux du Mannequin, déjà crédités CC0 dans `CREDITS.md` ;
pas de sonnaille. Le retour au pâturage appris (heafing) demanderait une
mémoire de lieu que les transitions actuelles ne savent pas exprimer.

## Mesures et articulation

Une unité vaut un centimètre ; dans le repère de travail, `x` va vers le
museau et `h` vers le haut, avec le sol à zéro. `outils.S` passe au SVG.
Les limites du moteur mesurent 90,64 cm de haut : `height = 45.32` donne
0,5 point par centimètre. `width = 62.7` correspond aux 125,4 cm de la fesse
au museau ; `stride = 35.2` vaut 1,6 fois la patte antérieure de 44 cm,
à cette même échelle.

- 70 cm au garrot à la peau, 76,6 cm avec la crinière ; corps laineux
  d'environ 94 cm de long, ventre à 28–30 cm du sol ; tête de 29 cm.
- La base du cou est à `(84, 52)`, la nuque à `(104, 86)` ; le raccord
  caché du cou reprend la construction tangente de la Lacaune, pour que le
  garrot et l'encolure abaissée se rejoignent sans bosse en broutant.
- Les carpes sont à 16,5 cm, les jarrets à 22,5 cm. Les quatre pattes sont
  coupées en deux segments par `articulations.split_leg`, avec des
  chevauchements arrondis ; leur blanc, y compris derrière le coude, suit
  la patte dans les sauts et le coucher.
- La mâchoire, la lèvre supérieure et son bourrelet dentaire sont séparés.
  Les pupilles sont des fentes horizontales dans des iris ambre brun.
- La queue naturelle a trois parties qui se recouvrent ; trois petites
  mèches du bas du ventre suivent le mouvement de la toison.

## Relecture d'après nature

Les photographies restent hors du dépôt. Les références vues comprennent :

- [HSBA, tête adulte et cornes](https://www.herdwick-sheep.com/images/aboutthebreed02.jpg) :
  implantation arrière, section ronde, pointe claire, poil blanc du crâne.
- [HSBA, tête de jeune bélier](https://www.herdwick-sheep.com/images/summary-herdwicksforsale.jpg) :
  œil, cils blancs, naseau, museau et poil ras ; sa demi-corne ne sert pas
  de modèle à l'adulte.
- [HSBA, béliers de Broughton](https://www.herdwick-sheep.com/images/breed-standard-broughton.jpg) :
  proportions et aplombs ; leur rouge de concours est ignoré.
- [Geograph, tête de profil à Eskdale](https://www.geograph.org.uk/photo/5562272) :
  joue profonde, dessous de mandibule, bouche courte et enroulement de la
  corne ; là encore, aucune teinture n'est reprise.
- [Herdwick ram, Geograph](https://commons.wikimedia.org/wiki/File:Herdwick_ram_-_geograph.org.uk_-_2354185.jpg) :
  col sombre et jarre blanc dans la laine naturelle, en tenant compte de
  sa toison plus longue que les trois mois de pousse de notre individu.
- [Solidwool, portrait de bélier](https://www.saxen.com/product-showcase-solidwool-furniture) :
  mandibule, contraste du poil blanc sur le col noir, texture mêlée de
  laine et de jarre ; ses longs amas ne sont pas copiés sur la laine courte.

La relecture a notamment corrigé un gris trop uniforme, des faisceaux qui
ressemblaient à des écailles, une mandibule trop fine, une oreille trop
haute, des stries de corne trop régulières, le raccord du cou baissé et une
zone blanche du coude qui restait attachée au corps pendant les sauts.
Les fibres sont des traits courbes regroupés par style, sans motif,
masque, filtre, découpe SVG ni groupe semi-transparent.

## Ce qui reste interprété

Les dimensions de tête, d'oreille, de queue et de corne sont les estimations
de la fiche, pas des mesures sur l'animal. Le profil ne représente pas
l'écartement des cornes en profondeur ; la corne opposée est cachée.
La laine courte est rendue par des fibres et des creux mats : elle reste
plus régulière que les amas et les jarres d'un vrai bélier. Le col tourne
avec l'encolure ; ses longs poils ne se tassent pas individuellement.
La bouche reste un peu plus oblique et géométrique que sur les portraits
de profil ; la ganache et le menton gardent une continuité lisse au repos
et pendant l'ouverture de la mâchoire.
À la taille de bureau, le détail du poil et des stries se fond : on lit
surtout le contraste tête et pattes blanches / tronc gris / col sombre.
Cet animal adulte ne représente ni la brebis sans cornes ni l'agneau noir.

© 2026 Alejandro San Martin Lamas — MIT OR Apache-2.0
