# Southdown : une agnelle de huit semaines

Le générateur suit [la fiche](../../../docs/races/southdown.md) : type
anglais moderne, au pré début mai, garrot osseux estimé à 45 cm. Il écrit
un rig original de profil droit ; une unité SVG vaut un centimètre.

```sh
python3 -m venv /tmp/mouflon-dessin
/tmp/mouflon-dessin/bin/pip install shapely
/tmp/mouflon-dessin/bin/python tools/races/southdown/gen_southdown.py pets/southdown/southdown.svg
```

La graine est fixe. Le SVG livré doit rester la sortie exacte du script.
`outils.py` reprend les outils de la Lacaune, avec le repère de cette
agnelle ; la découpe des quatre pattes et de la lèvre utilise
`../articulations.py`. L'encolure est entière, sous le corps, avec sa
partie cachée. La mâchoire découvre une bouche rose sombre ; la lèvre
couvre un bourrelet dentaire. La pupille est horizontale, la queue
naturelle s'articule en trois morceaux.

La laine est courte et serrée : aucune frange ni mèche `sway-*` pendante.
Les petites boucles et leurs creux sont répartis sans grille, avec une
lumière commune au corps, au cou, aux pattes et à la queue. Il n'y a ni
masque, ni découpe SVG, ni filtre, ni groupe semi-transparent.

## Références visuelles

Photos consultées et gardées seulement dans un dossier temporaire **hors
du dépôt**, avec un User-Agent générique `MouflonResearch/1.0` :

- [Agneaux en mouvement auprès de leur mère](https://www.heritagesheepreproduction.com/blogs/uk-sheep-breeds/southdown),
  photo fournie par la société de race, crédit Crundwell : petit thorax,
  membres fins, port de tête, jeux.
- [Agneau du Weald & Downland Living Museum](https://www.wealddown.co.uk/museum-news/lambs-springs-new-faces-at-the-museum/),
  identifié comme Southdown de pedigree : front laineux, œil dégagé,
  chanfrein court, truffe brun rosé et oreilles.
- [Deux agneaux au pré](https://www.lifeandtimesofayorkshirelad.com/landscapes/p/southdown-lambs) :
  texture serrée, variations gris-faon de la face, queue naturelle.
- [Agnelle de janvier au concours de juin](https://www.southdownsheepsociety.co.uk/shows/2016/1/5/silverware-heads-south-from-southdown-national-show) :
  profil et aplombs ; sujet plus âgé et plus massif que notre agnelle.
- [Agnelles de Mudchute](https://www.mudchute.org/blog/meet-the-southdowns) :
  laine du front et des joues ; sujets plus âgés, pas le canon du corps.

Les dix repères de la fiche guident la relecture : longues pattes et
thorax peu profond ; tête ronde ; profil droit et bouche fine ; museau
souris clair ; truffe gris-rosé mouchetée ; oreilles horizontales laineuses ;
absence de cornes ; boucles courtes ; chaussettes faon et onglons foncés ;
queue entière jusqu'au jarret. La première passe a surtout été corrigée
sur la joue trop effilée, le bord du museau trop net, l'oreille trop plate,
l'œil trop brillant et les boucles trop régulières.

## Vérifier dans le moteur

```sh
cargo run --release -p mouflon-preview -- pets/southdown/pet.toml apercu/southdown poses
cargo run --release -p mouflon-preview -- pets/southdown/pet.toml apercu/southdown gestes
cargo run --release -p mouflon-preview -- pets/southdown/pet.toml apercu/southdown allures
cargo run --release -p mouflon-preview -- pets/southdown/pet.toml apercu/southdown jeux
cargo test --release -p mouflon-render --test troupeau
```

`troupeau` découvre automatiquement `pets/southdown/` : chargement,
simulation, dessin, parties, cuisson, sabots au sol et museau dans l'herbe.
Les planches et images de travail ne sont pas versionnées.

Les remplissages et les spires de même teinte sont regroupés en tracés
composés : environ 440 tracés, au lieu de plus de 5 000 lors de la première
passe, pour les mêmes petites boucles. Le dessin reste cuit en 23 morceaux.

## Limites

Les centimètres de l'agnelle, la longueur de queue et la couleur de l'iris
restent les estimations de la fiche ; les photos montrent de vraies
variations de pigmentation. La frisure est une interprétation vectorielle
et ses détails se fondent à la petite taille du bureau. La voix est une
seule prise d'agneau existante, dont la race et l'âge ne sont pas connus
(voir `pets/southdown/sons/README.md`). La queue reste trop verticale
couchée : le moteur la descend avec le bassin sans la replier, et son bout
passe environ 8 cm sous le sol (4 pt à l'écran). La raccourcir d'autant
changerait la queue naturelle demandée par la fiche ; corriger cette pose
demande une évolution du moteur, hors de cette tâche.

Le moteur ne gère encore ni la
tétée, ni le lien individuel mère-agneau, ni la flexion du dos. L'intégration
en remplacement de Lainou et la vérification dans l'app sur un vrai Mac
relèvent de la tâche suivante.
