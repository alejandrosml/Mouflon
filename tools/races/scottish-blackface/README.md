# Scottish Blackface : le bélier Lanark

Un bélier de quatre ans, en toison naturelle d'un an juste avant la tonte,
de profil droit. Le dessin suit [la fiche de race](../../../docs/races/scottish-blackface.md) :
garrot osseux de 70 cm, tronc de 84 cm, tête de 28 cm, carpe à 18 cm et
jarret à 22 cm. Une unité du SVG vaut un centimètre de l'animal.

```sh
python3 -m pip install shapely
python3 tools/races/scottish-blackface/gen_scottish_blackface.py pets/scottish-blackface/scottish-blackface.svg
cargo run -p mouflon-preview --release -- pets/scottish-blackface/pet.toml apercu poses
cargo run -p mouflon-preview --release -- pets/scottish-blackface/pet.toml apercu gestes
cargo run -p mouflon-preview --release -- pets/scottish-blackface/pet.toml apercu jeux
```

Le générateur est déterministe (graine 1901, puis graines locales fixes).
Le SVG livré est sa sortie exacte ; une retouche se fait dans le script.
La régénération a été comparée octet pour octet avec Python 3.12 et Shapely
2.2.0. Le moteur mesure 137,18 × 98,16 cm de limites peintes et cuit le rig
en 27 morceaux ; le `pet.toml` l'affiche à 0,5 point par centimètre.
`outils.py` reprend et réduit les outils géométriques de Thônes et Marthod ;
les contours, les mesures et les mèches de ce bélier sont propres à cette race.
La découpe des quatre pattes et de la lèvre utilise `../articulations.py`.
L'encolure est une pièce complète sous le corps ; la queue a trois segments,
et trois faisceaux de la frange du ventre suivent les mouvements de la toison.

Les références sont seulement regardées hors du dépôt. Parmi les photos
répertoriées dans la fiche, cinq ont servi à la relecture :

- [P23, bélier en toison naturelle](https://static.wixstatic.com/media/117303_d094ca51dc144e3697f20c3d8f06f74c~mv2.jpg) : proportions et marques franches de la tête et des canons.
- [P20, bélier de colline](https://static.wixstatic.com/media/117303_019e770e1e804a0fa26697f5aa8ee69d~mv2.jpg) : cornes miel annelées, museau et gorge blancs.
- [P2, bélier de Stornoway](https://commons.wikimedia.org/wiki/File:Blackface_ram_in_Stornoway.jpg) : corne adulte en hélice et oreille dans la boucle ; sa laine teinte n'est pas reprise.
- [P9, bélier et brebis](https://commons.wikimedia.org/wiki/File:Blackface_ram_ewe.jpg) : barbe, frange et couleur naturelle de la laine.
- [P10, longue toison de colline](https://commons.wikimedia.org/wiki/File:Road_from_Falcarragh_SE_to_R251_-_Sheep_along_road_-_geograph.org.uk_-_1185251.jpg) : mèches pendantes aux ondulations lâches, pointes effilées.

Les petites fibres et les annelures se fondent à la taille de bureau :
la silhouette, le noir ras, le museau blanc, les grandes cornes et la frange
portent alors la reconnaissance de la race. Le profil unique ne montre pas
tout l'écartement latéral des cornes. La laine du cou tourne d'une pièce
avec l'encolure, comme celle des autres races longues du dépôt ; le raccord
au dos reste un peu raide au broutage. Le tronc et les grandes mèches du haut
restent plus réguliers que sur les photos. Le heft et les déplacements
saisonniers ne sont pas simulés par les réglages actuels du moteur.

© 2026 Alejandro San Martin Lamas — MIT OR Apache-2.0
