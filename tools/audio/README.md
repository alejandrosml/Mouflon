# Trois appels synthétiques

Ces WAV sont des créations originales par synthèse : aucun enregistrement
animal, bruitage tiers ou échantillon externe n'est utilisé. Les fichiers audio
sont dédiés au domaine public sous **CC0 1.0 Universal** :
<https://creativecommons.org/publicdomain/zero/1.0/>. Le script et ses tests
suivent les licences du dépôt, `MIT OR Apache-2.0`.

```sh
python3 tools/audio/synthese.py
python3 -m unittest discover -s tools/audio -p 'test_*.py' -v
```

Sans argument, le générateur réécrit les trois fichiers livrés à leur place.
Avec un dossier en argument, il y écrit une copie d'essai. Il ne demande que
Python 3.11 ou plus récent, sans paquet supplémentaire.

- `pets/mouflon-corse/sons/mouflon-alarme.wav` : chuintement de 0,34 s,
  souffle filtré et filet de sifflement descendant. Le cri d'alerte bref,
  répété et parfois accompagné d'une frappe d'antérieur est décrit par
  Valette (2010, p. 54). Il accompagne `stamp`, sans ouvrir la bouche.
- `pets/mouflon-corse/sons/mouflon-grondement.wav` : évocation rugueuse grave
  de 0,76 s. Ce timbre n'est **pas une restitution bioacoustique** : les
  descriptions de rut ne concordent pas. Cap et al. (2008, fig. 2 et § 6)
  indiquent l'absence d'un appel propre au rut chez le mouflon. Son association
  au flehmen d'un bélier qui vient de flairer une brebis est un choix de
  présentation ; ne pas présenter ce son comme un cri mesuré ou enregistré.
- `crates/mouflon-app/assets/appel-maternel.wav` : appel sourd de 0,92 s,
  aux harmoniques graves, pour une mère s'adressant à son agneau au début
  d'une rencontre de tétée. L'app le garde en mémoire ; `PetEvent::MaternalCall`
  le déclenche sans provoquer l'ouverture de la bouche. Les bêlements graves
  maternels et la reconnaissance de la voix sont documentés par Sèbe et al.
  (2007). Fréquence fondamentale, durée et grain du WAV sont des choix
  sonores, pas des valeurs tirées de cette étude.

Les sons sont mono, PCM 16 bits à 24 kHz, crête limitée à 0,62. Ils entrent
et sortent par une enveloppe douce, sans clic. Le générateur de bruit a une
graine fixe et ne dépend jamais de celui du troupeau. La synthèse se fait
hors de l'app ; les sons sont ensuite décodés et joués sur le fil des sons,
avec les mêmes limites de débit et le même réglage de silence que les
autres voix. Le pack du mouflon emporte ses deux WAV : aucun chemin vers
un dossier voisin n'est nécessaire lorsqu'on l'importe seul.

Sources de comportement, sans reproduction sonore :

- Valette 2010, *Le Mouflon Corse et Méditerranéen*, ANCGG :
  <https://www.ancgg.org/wp-content/uploads/2020/10/Le-mouflon-Corse-et-medtterraneen.pdf>.
- Cap et al. 2008, *Male vocal behavior and phylogeny in deer*,
  DOI <https://doi.org/10.1111/j.1096-0031.2008.00223.x>.
- Sèbe et al. 2007, *Establishment of vocal communication and discrimination
  between ewes and their lamb in the first two days after parturition*,
  DOI <https://doi.org/10.1002/dev.20218>.
