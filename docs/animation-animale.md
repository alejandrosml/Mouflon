# L'animation des animaux : ce que disent les études, ce qu'en fait Mouflon

Mouflon anime lui-même ses moutons dessinés en vecteurs (le Mannequin et
quatre races réelles, § 10) ou peints sur un squelette (Aquarelle) : à
partir d'une seule pose de profil, le moteur calcule à chaque image où vont
les pattes, le corps, la tête, les oreilles, la queue, les paupières et la
mâchoire. Pour que ce mouvement soit juste, il s'appuie sur ce que les
études disent des vrais moutons — et des autres quadrupèdes quand il n'y a
rien sur le mouton — et sur ce que font de mieux les jeux et les films.

Ce document rassemble ces connaissances (recherche de novembre 2026), dit
pour chacune ce que Mouflon en fait, avec ses valeurs, et ce qui reste à
faire (§ 11). Les longueurs du moteur sont en **u**, le centième de la hauteur du
dessin de l'animal ; les instants d'une foulée, en fractions de la foulée
comptées depuis le poser du postérieur qu'on voit.

Le code : `crates/mouflon-render/src/animate.rs` (les allures),
`crates/mouflon-render/src/joints.rs` (l'encolure et les pattes en deux
segments), `crates/mouflon-render/src/gesture.rs` (les gestes et la posture),
`crates/mouflon-core/src/pet.rs` (les signaux : vitesses, gestes, mâchoire,
oreilles, respiration, clignements, imitation entre voisins).

## 1. Les allures

### L'ordre des pattes

| Allure | Ordre et décalages (fraction de foulée) | Appui | Sources |
|---|---|---|---|
| Pas | séquence latérale : postérieur gauche 0, antérieur gauche **0,27**, postérieur droit 0,5, antérieur droit 0,77 | 0,58 à 0,69 | mouton 0,28 à 0,68 d'appui (Usherwood & Self Davies 2017) ; « 25 % » typique des mammifères |
| Trot | paires diagonales, l'arrière posé un rien avant l'avant (0 à 0,03) | 0,56 (trot lent) à 0,4 | cheval +0,04 (Warmblood), −0,05 (islandais) |
| Galop | **transverse** chez le mouton : arrière traînant 0, arrière meneur 0,16, avant traînant 0,26, avant meneur 0,48, puis une suspension groupée (≈ 0,23) | ≈ 0,29 | Biancardi & Minetti 2012 (152 foulées, 12 espèces) ; les cerfs, gazelles et bovins galopent « en rotation » |
| Pronk | les quatre pieds ensemble, pattes raides | — | Hildebrand 1977 (films d'*Ovis* : galop transverse, demi-bond, bond, pronk) |

Aucun bovidé ne va l'amble (Dagg 1973). Chez le mouton, le passage du pas
au trot est moins net que chez le chien (Jayes & Alexander 1978).

**Dans Mouflon** : exactement ces décalages, et ces appuis (0,64 au pas,
0,48 au trot, 0,32 au galop). Le moteur ne passe jamais d'une allure à
l'autre d'un coup : il mêle les trois selon des poids qui suivent la vitesse,
décalages et appuis compris (comme les réseaux MANN, qui mêlent les allures
sans machine à états). Les valeurs sont choisies pour qu'aucune patte ne
fasse un tour de trop pendant le passage.

### Quand changer d'allure

La vitesse qui compte est la vitesse rapportée à la taille : le nombre de
Froude, Fr = v² / (g·h), h étant la hauteur de la hanche. Les animaux de
toutes tailles changent d'allure aux mêmes nombres de Froude (Alexander
1976) : le mouton se met au trot vers Fr ≈ 0,30 et au galop vers Fr ≈ 1,1
(Jayes & Alexander 1978) ; le cheval trotte dès Fr ≈ 0,35 ; la longueur de
foulée suit λ/h ≈ 2,3·Fr^0,3.

**Dans Mouflon** : l'allure se choisit d'après la vitesse en longueurs de
patte par seconde (la longueur des pattes vaut `stride / 1,6`) : le pas
jusqu'à 2,6, le trot de 2,6 à 3,4, le galop de 4,6 à 6,0. Agrandi, un
animal garde ses allures. Le pas de nos moutons (46 pt/s, 2,5 longueurs de
patte par seconde) est un pas vif ; leur trot (80 à 86 pt/s) est plein ;
leur course (150 pt/s) un vrai galop. Un pas en broutant, très lent, garde
des pattes qui marchent (pleinement dès 0,6 longueur par seconde).

### La cadence et la foulée

- Pas : 1,2 à 1,6 foulée par seconde (Mérinos 1,13 m/s à 1,6 Hz ; 40 kg
  à 1,2 m/s : foulée 0,83 m à 1,34 Hz ; mouton de montagne autrichien :
  1,16 m/s, foulée 0,99 m, 1,17 Hz).
- Trot : la cadence monte avec la vitesse, f = 1,24 + 0,50·v Hz (mouton de
  23 kg, Heglund & Taylor 1988) : de petits pas rapides, à peine plus longs
  qu'au pas (mouton autrichien : 1,82 m/s, foulée 1,07 m, 1,7 Hz).
- Galop : la cadence ne bouge presque plus (f = 2,34 + 0,08·v Hz) ; ce sont
  les bonds qui s'allongent.

**Dans Mouflon** : la phase des pattes avance avec la **distance**
parcourue, pas avec le temps (la « roue d'arpenteur » de Rosen) : les sabots
ne glissent jamais, à n'importe quelle vitesse. La foulée vaut `stride` au
pas, 1,15 fois plus au trot (≈ 2,4 foulées par seconde au trot de nos
moutons), 1,9 fois plus au galop (≈ 2,4 à 2,7 par seconde).

### Le corps qui monte et descend

- Au **pas**, le corps passe par-dessus chaque patte d'appui comme un
  pendule renversé : le garrot est au plus haut à mi-appui des antérieurs,
  la croupe à mi-appui des postérieurs, chacun deux fois par foulée, en
  décalage d'un quart de foulée. La croupe monte plus que le garrot
  (cheval : 4,5 % contre 2 % de la hauteur au garrot, crête à crête). La
  tête est au plus bas à mi-appui des antérieurs : elle les charge (Rhodin
  2022 ; le hochement est calé pour coûter le moins d'effort).
- Au **trot**, c'est un ressort : tête, garrot et croupe descendent ensemble
  au milieu de l'appui de chaque paire diagonale (vers 40 %), remontent
  pendant le bref envol ; 3 à 6 % de la hauteur au garrot.
- Au **galop**, le centre du corps monte moins qu'au trot, mais le corps
  bascule bien plus : le nez plonge quand les antérieurs reçoivent le
  poids, se relève sous la poussée des postérieurs (≈ ±4 à 6°).

**Dans Mouflon** : au pas, garrot 1,0 u et croupe 2,2 u crête à crête, au
plus haut à 0,59 et 0,32 de la foulée (le corps se balance donc d'avant en
arrière), la tête 2,4 u en plus, au plus bas quand le garrot est au plus
haut, et elle penche en descendant ; au trot, 2,4 u, au plus bas à 0,2 et
0,7 de la foulée ; au galop, 1,4 u, au plus haut pendant la suspension
groupée, et ±0,08 radian de tangage.

### Les sabots

- Un sabot de mouton monte de 47 à 49 mm au pas, de 54 à 60 mm au trot :
  6 à 7 % de la hauteur au garrot.
- Un antérieur monte au plus haut tôt dans son vol (vers 0,3 à 0,4), un
  postérieur à mi-vol.
- De la hanche au sabot, une patte balaie 20 à 25° de chaque côté au pas et
  au trot, ≈ 18° à l'appui du galop (chien : 18 à 27°).

**Dans Mouflon** : levée de 3,4 u au pas, 4 u au trot, 7 u au galop ;
l'antérieur au plus haut à 38 % de son vol ; le balayage vient de la foulée
et de la longueur des pattes (≈ 22° pour le Mannequin au pas), et la patte
d'appui s'allonge juste ce qu'il faut pour rester au sol en s'inclinant.

### Le carpe et le jarret

Les mesures sur le mouton au pas (environ 1,2 m/s) :

- **Le carpe** a la plus grande amplitude de toute la patte : de 176° (presque
  droit, à l'appui) à 105–117° au plus plié, en vol (60 à 70° de flexion ;
  Faria 2014, brebis Santa Inês). Le coude bouge de 42 à 44°, l'épaule de
  9 à 12° seulement. Les auteurs disent que le mouton marche « raide » des
  postérieurs et « souple » des antérieurs.
- **Le jarret** plie de 30 à 35° (de 155° à 119–126°) ; le grasset de 40 à
  50°, au plus plié en vol (86 à 94°) ; le boulet reste presque droit à
  l'appui (164 à 178°) et plie en vol (144 à 156° ; Diogo 2021, brebis
  Churra sur tapis roulant). Chez le Mérinos, le genou et la cheville sont
  au plus pliés **au milieu du vol** (80,6° et 80,7° de flexion depuis la
  station debout) ; l'appui dure 60 % de la foulée (Silva 2024).
- Le cheval au trot plie davantage en vol qu'au pas (coude, carpe, grasset,
  jarret) et tend le carpe et le boulet à l'appui.

**Dans Mouflon** (avec des pattes en deux segments, voir
[rig.md](rig.md#les-articulations)) : le haut de la patte va de l'épaule ou
de la hanche au carpe ou au jarret, le bas porte le canon et le sabot. Le
moteur ne fait plus tourner une patte raide : il pose le sabot là où il doit
être et plie le genou par une cinématique inverse à deux os. **À l'appui**,
le sabot recule exactement de ce que le corps avance (il ne glisse pas :
moins de 5 % de la vitesse mesurés au pas, au trot et au galop, contre 15 %
admis avant) et la patte reste droite : elle se tasse de 7 % avant de plier,
comme le carpe et le boulet tendus d'un vrai mouton ; au bout de l'appui,
elle s'étire jusqu'à 25 % (au galop). **En vol**, la levée mesurée du sabot
(3,4 u au pas) suffit à plier le carpe d'une cinquantaine de degrés au plus
haut (au milieu du vol, comme chez le Mérinos), un quart de radian de plus
pour l'antérieur, dont le canon part vers l'arrière ; le jarret plie dans
l'autre sens, le canon vers l'avant. Le sabot posé reste sur le sol de
l'écran même quand l'élan incline l'animal. En l'air, les pattes se
replient en montant (le carpe et le jarret pliés à fond) et se tendent vers
le sol en descendant ; à l'atterrissage, le corps descend de 6 % de sa
hauteur et les genoux amortissent, les sabots glissant un peu vers
l'extérieur.

### Le saut d'agneau

Le « stotting » ou « pronking » : des bonds droits en l'air, les quatre
pattes raides et tendues, qui partent et retombent ensemble (Caro 1986) ;
la gazelle de Thomson garde la tête haute. Chez les moutons et les chèvres
domestiques, surtout les jeunes.

**Dans Mouflon** : `motion = "pronk"`, pattes raides, la tête qui secoue ; les
animaux actifs (tempérament) en font volontiers, et « Tous ensemble » fait
sauter tout le troupeau.

### Les jeux d'agneaux

Les agneaux jouent surtout entre 7 et 12 jours, puis entre 37 et 54 jours
(Augustsson 2018) : le « gambol », un saut rapide où le corps et la tête se
tordent ; le « pivot », un tour raide dans un petit saut ; la course, à trois
ou plus, aller et retour, trois fois plus le soir ; les coups de tête, où
chacun recule puis s'élance front contre front. L'invitation au jeu :
reculer, baisser la tête, fixer l'autre.

**Dans Mouflon** (`pets/mannequin/pet.toml`, surtout chez les actifs) :
`gambol` (`twist = true` : en l'air, le corps se tord, la tête part de
l'autre côté, les pattes de derrière ruent ; il en enchaîne souvent
plusieurs), `pivot` (`spin = true` : un petit bond sur place, retourné en
l'air), `race` (une course contagieuse jusqu'au bout, demi-tour, retour ;
plus le soir), et les coups de front : deux actifs qui se croisent reculent
ensemble (`back`, 1,1 s), s'élancent (`charge`) et se cognent ; le choc les
tasse tous les deux. Lainou, en images, a ses courses (`joue`).

Avant chaque saut, un animal dessiné se ramasse 0,12 s, le corps écrasé :
l'anticipation des animateurs, que l'œil attend.

L'invitation au jeu (essai 19) : de temps en temps, un agneau qui en a envie
(surtout chez les actifs) fixe un voisin de sa famille, à une ou trois
longueurs, la tête et l'avant baissés, en sautillant ; l'autre accepte le
plus souvent s'il est actif, et ils s'élancent l'un vers l'autre jusqu'au
coup de front.

### Se coucher, se relever

Le bovin se couche l'avant d'abord (il tombe sur les carpes, puis
l'arrière) et se relève l'arrière d'abord ; le mouton aussi (WikiVet). Chez
le veau, 3,4 à 4,4 s pour se coucher, 2,4 s pour se relever.

**Dans Mouflon** : c'est l'ordre du moteur depuis le début ; en se relevant,
nos moutons s'étirent souvent (voir plus bas). Avec des pattes en deux
segments, un mouton couché les replie comme un vrai mouton couché sur le
sternum : les antérieurs sous le poitrail (le carpe posé au sol, plié à
fond, le canon à plat vers l'arrière), les postérieurs le long du ventre (le
jarret derrière, le canon vers l'avant) ; il plie les antérieurs d'abord et
se relève par l'arrière. Aucune mesure publiée de ces angles chez le mouton
couché : ce sont les postures décrites et photographiées.

## 2. Une journée de mouton

- Au pré, une brebis passe 9 à 10 heures par jour à brouter, 5 à 6 à
  ruminer, 12 à 13 couchée ; elle marche très peu (2 % du temps).
- **Brouter** : une bouchée toutes les 0,93 à 0,96 s, quelle que soit
  l'herbe ; 1,3 à 1,7 mastication par bouchée. Elle reste 7 à 11 s à une
  « station », puis fait un ou deux pas tête basse vers la suivante.
- **Ruminer**, surtout couchée : chaque bol est mâché une minute environ
  (45 à 78 coups, 58 à 99 par minute), avalé, puis une pause de 10 à 15 s,
  et le bol suivant remonte (Gordon 1958). Jamais en dormant.

**Dans Mouflon** : en broutant, une bouchée par seconde (1,05 Hz, un peu
irrégulière) et la mâchoire qui mâche à 1,6 Hz ; toutes les 8,5 s environ,
un pas d'une seconde vers l'herbe suivante, tête au sol, sans jamais passer
le bord d'une fenêtre. Couché, `chew = true` : il rumine bol après bol
(9 à 16 s de mastication, 1,6 à 3 s de pause : raccourcis pour un bureau).

### L'encolure qui broute

Rien de mesuré sur l'angle de l'encolure d'un mouton qui broute. Chez le
cheval, l'angle entre la tête et l'encolure s'**ouvre** quand il mange au
sol (153°, de 146 à 161°) : c'est l'encolure qui descend, la tête se tend
dans son prolongement au lieu de se replier (113° l'encolure 15° sous le
garrot, 98° 15° au-dessus ; étude de morphométrie géométrique, *Animals*
2021). Les photographies de moutons qui broutent montrent la même chose :
l'encolure part vers l'avant et le bas, le museau dans l'herbe à peu près
une longueur de tête devant les antérieurs. Aux aguets, la tête est levée
au-dessus du dos (§ 4).

**Dans Mouflon**, avec une encolure : la tête ne descend plus d'un bloc. Une
cinématique inverse à deux articulations (la base de l'encolure dans
l'épaule, la nuque) pose le museau au sol, 0,85 longueur de tête devant les
sabots des antérieurs (plus près si l'encolure ne va pas si loin) ; c'est
surtout l'encolure qui descend (la nuque descend de plus de la moitié de ce
que descend le museau), et seule elle s'étire, jusqu'à 12 %. Aux aguets,
l'encolure se redresse (0,24 rad) ; quand il fonce, elle se tend vers le bas
(0,3 rad) ; à moitié endormi, elle s'affaisse ; endormi la tête au sol, elle
s'allonge vers l'avant (8 %) et la tête se pose presque à plat. Le regard
(le curseur) reste à la tête. Le test `troupeau` vérifie que chaque mouton
dessiné du dépôt broute le museau dans l'herbe.

## 3. Les gestes

| Geste | Ce qu'en disent les études | Dans Mouflon |
|---|---|---|
| S'ébrouer | un mouton de 55 kg secoue sa toison à 6,5 Hz (la loi des mammifères, f ∝ M^−0,22, donnerait 4,9 Hz) ; le corps tourne de 60 à 110° ; 3 ou 4 allers-retours (Dickerson 2012) | `shake` : 6 Hz ; la secousse part de la tête et file jusqu'à la queue, la tête paraît plus étroite à chaque aller et retour (elle tourne), la toison ondule, les oreilles battent, les yeux se ferment ; 0,9 s |
| S'étirer | surtout en se relevant : cou tendu, dos droit, une patte arrière tendue loin derrière, parfois plusieurs fois ; absent chez un animal malade (Fraser 1989) | `stretch` : le cou et l'avant d'abord, puis une patte arrière ; 2,4 s ; après le sommeil et en se relevant |
| Bâiller | 1 à 2 fois par jour ; contagieux : 11 % dans la minute qui suit le bâillement d'un congénère | `yawn` : la tête se lève, la bouche s'ouvre grand, les yeux se ferment ; contagieux (`contagious = 0.15`) ; avant de s'endormir |
| Se gratter | d'un sabot arrière, ou contre un support : 8 % des soins chez l'agneau (le toilettage à la bouche : 39 %) | `scratch` : un postérieur remonte gratter le ventre à petits coups rapides (4 Hz), la tête tournée vers lui, les yeux mi-clos |
| Taper du pied | face à une menace (un chien), un pied levé puis posé avec force ; rare, et le chien s'arrête souvent | `stamp` : tête haute, oreilles dressées, une patte de devant monte lentement et s'abat ; quand le pointeur file tout près |
| Gratter le sol | menace (bélier), et avant l'agnelage ; **pas** avant de se coucher | `paw` : un antérieur racle le sol, tête basse ; avant qu'un énervé ne charge |
| Se cabrer | le mouflon canadien (bighorn) se dresse sur ses pattes de derrière avant le choc ; le mouflon de Corse, lui, recule de quelques mètres puis fonce tête baissée, sans se cabrer (OFB) | `rear` : tout l'animal tourne autour de ses hanches, l'avant dressé d'une trentaine de degrés, les pattes de devant repliées, le menton rentré, les oreilles couchées, puis il retombe d'un coup et charge ; le Mannequin et Aquarelle le font souvent après avoir reculé, le mouflon de Corse jamais |
| Charger | le bélier recule, baisse la tête, fonce (le mouflon canadien se cabre, puis ≈ 40 km/h) | `paw`, puis le plus souvent `back` (il recule de quelques pas sans se retourner, tête basse), puis `charge` : tête basse, oreilles couchées, au galop ; s'il atteint un autre animal, le choc les tasse tous les deux |
| Bêler | aigu bouche ouverte : isolement, stress ; grave bouche fermée : brebis et agneau proches ; sinon rare | `bleat` et les sons `mouth = true` : la bouche s'ouvre en 0,1 s, un léger vibrato |
| Renifler | le museau tendu vers l'autre | `sniff` : quand deux moutons se croisent |
| Sonné | — | `daze` : la tête décrit de petits cercles, les yeux mi-clos, les oreilles pendantes ; après un atterrissage rude, puis il s'ébroue |
| Flehmen | le bélier flaire l'urine d'une brebis ou sa vulve, puis relève la tête (d'environ 30°), les naseaux plissés, la lèvre supérieure retroussée sur le bourrelet sans dents, et hume (Blissitt 1989) ; il suit surtout l'urine (Stevens 1982), qu'une brebis émet quand un bélier l'approche, sauf si elle est en chaleur : devant une brebis en chaleur, le flehmen est le plus rare ; celui qui suit le flairage de la vulve, bref, est le plus fréquent la veille des chaleurs (Bland & Jubilan 1987, résumés par Blissitt 1989 ; Mirto 2015). Tenu d'un instant à deux minutes (Banks 1964, cité par Blissitt 1989) ; en moyenne 7 à 19 s selon les groupes chez de jeunes béliers Najdi, 12 s pour un bélier seul avec une brebis (Bahadi 2023, sur peu de flehmen : les écarts ne sont pas significatifs) | `flehmen` : l'encolure tendue vers l'avant et le haut, la tête relevée d'une quarantaine de degrés (le chanfrein à peu près à l'horizontale, le menton en avant, comme sur les photos), puis la lèvre retroussée (la gencive rose apparaît, le mouton n'a pas d'incisives en haut), la bouche entrouverte, les yeux mi-clos, la tête qui balance à peine ; 7 à 12 s, le bas des moyennes mesurées (pour un bureau) ; après avoir reniflé un autre mouton, plus souvent chez un énervé et chez les béliers des races, rarement chez une brebis |

## 4. Les émotions : oreilles, tête, queue

Les oreilles d'un mouton disent ce qu'il ressent (Boissy 2011, Reefmann
2009) :

- **horizontales** : neutre ;
- **en arrière** : la peur, une situation nouvelle ou qu'il ne contrôle pas ;
- **dressées** : la colère, une situation déplaisante qu'il contrôle ;
- **asymétriques** (l'une en avant, l'autre en arrière) : la surprise ;
- **passives**, pendantes : le calme ; en mangeant du foin frais ;
- changeant souvent de position : un mouton isolé, inquiet.

Le vent et le bruit couchent aussi les oreilles. La vigilance : immobile, la
tête levée au-dessus du dos, 20 à 60 s ; après une menace, la tête reste
haute de longues minutes.

**Dans Mouflon** :

| Situation | Oreilles | Tête |
|---|---|---|
| énervé (tempérament) | dressées | un peu basse |
| il fonce | couchées en arrière | basse |
| tenu en l'air par le pointeur | en arrière (la peur) | — |
| dans le vent | en arrière | basse (`brace`) |
| calme (tempérament) | pendantes | un peu basse |
| il broute ou rumine | pendantes | — |
| actif (tempérament) | en avant | — |
| sursaut | asymétriques, puis droites | — |

Un énervé remue aussi les oreilles et fouette de la queue plus souvent ; un
calme, moins.

## 5. La vie du troupeau

- **Suivre** : 95 % des moutons partent dans les 12 s qui suivent le
  départ de celui qui les précède ; les départs s'accélèrent quand la moitié
  du groupe est partie. Un mouton part de lui-même environ toutes les
  3 minutes.
- **L'alerte se propage** par imitation : quand l'un lève la tête, les
  autres la lèvent.
- **Le bâillement** est contagieux (voir plus haut).
- **Les regroupements** (Ginelli 2015, 100 brebis) : le troupeau se disperse
  lentement en broutant (74 % immobiles) ; de temps en temps, un mouton du
  bord court vers le centre, les autres suivent en vague pendant ≈ 30 s,
  puis tous s'arrêtent presque ensemble ; les plus grands regroupements
  reviennent toutes les 15 minutes environ, en silence.
- Au pré, deux moutons sont en moyenne à 7 à 9 m l'un de l'autre ; un petit
  groupe broute à ≈ 1 m.

**Dans Mouflon** : un comportement `contagious` se propage aux voisins de la
même famille, à moins de quatre largeurs et à la même hauteur, avec la
chance indiquée, dans un délai de 0,4 à 5 s s'ils sont libres (debout, à
marcher, à brouter, à regarder autour d'eux), sinon à la fin de ce qu'ils
font (un animal en images ne coupe pas sa séquence). Un mouton qui part entraîne
ses voisins du même côté (`walk`, une chance sur quatre) ; quand l'un
détale, les autres détalent (`run`, 0,45) ; l'alerte (`look`, 0,25), le
bâillement (`yawn`, 0,15) et les sauts d'agneau (`pronk`, 0,3) se
propagent. Celui qui vient d'imiter attend 25 s avant d'imiter de nouveau,
et ce qu'il imite ne se propage pas plus loin : pas de va-et-vient sans fin.

Toutes les cinq à douze minutes, le troupeau se regroupe : sur un même
rebord, au moins trois animaux d'une même famille, libres et bien
dispersés, se rapprochent de leur milieu, à 0,8 largeur l'un de l'autre,
sans changer d'ordre. Le plus loin de sa place part le premier ; les autres
suivent en vague, d'autant plus tard (jusqu'à six secondes) qu'ils sont
loin de lui ; chacun marche, ou trotte s'il est loin, puis s'arrête. En
chemin, il n'imite personne ; il peut s'arrêter un peu avant sa place pour
renifler un voisin.

## 6. Dormir

- Le mouton dort couché sur le sternum, les pattes repliées ; la tête en
  avant, tournée sur le côté, ou levée ; il somnole « en sphinx », parfois
  le cou tendu, la tête au sol. Couché sur le flanc, les pattes tendues : un
  agneau qui souffre.
- 3,8 h de sommeil par jour (5,9 avec la somnolence), dont 0,6 h de sommeil
  paradoxal : les yeux bougent, **les oreilles et les pattes tressaillent**,
  le cou se relâche. La vache pose alors la tête sur son flanc.
- Il respire 16 à 34 fois par minute au repos (20 à 30 le plus souvent),
  plus lentement en dormant.

**Dans Mouflon** : 20 respirations par minute éveillé, 14 endormi ; un
animal dessiné s'endort une fois sur deux la tête posée au sol, le cou
tendu (elle descend lentement, en quelques secondes) ; toutes les 5 à 14 s,
une oreille tressaille, ou, une fois sur trois, une patte.

## 7. Les yeux

- Un clignement : la paupière se ferme en 75 à 100 ms, reste close 10 à
  50 ms, se rouvre deux fois plus lentement ; parfois deux de suite.
- Le cheval détendu cligne 8 à 9 fois par minute (5 quand il est stressé) ;
  rien de publié sur le mouton.
- Un clignement accompagne presque tous les grands mouvements du regard, et
  tout tour de tête de plus de 17°.

- La pupille du mouton (de la chèvre, du cheval) est une fente horizontale,
  dans un iris doré ou ambré : elle élargit le champ de vision à
  l'horizontale et voit net le sol devant les pattes. Quand la tête descend
  brouter, chaque œil tourne dans son orbite (cyclovergence), d'une
  cinquantaine de degrés, peut-être plus : la pupille reste parallèle au sol
  (Banks et al. 2015, *Science Advances*).

**Dans Mouflon** : 0,27 s par clignement (fermé en 0,08 s, clos 0,03 s,
rouvert en 0,16 s), toutes les 2,5 à 8 s, doublé une fois sur cinq ; il
cligne aussi en faisant demi-tour. Les gestes ferment les yeux à leur façon
(s'ébrouer, bâiller, sonné…). Depuis l'essai 19, les pupilles sont des
fentes horizontales dans un iris ambré, et le moteur les garde à
l'horizontale quand la tête penche (en broutant, en courant, tenu en l'air),
jusqu'à 0,9 radian (52°) : au-delà (une culbute), elles penchent avec la
tête.

## 8. Le vent et la météo

Un mouton qui a froid se serre contre les autres, **tourne le dos au vent**
et ne se couche pas ; à l'aise, il se couche, rumine et regarde de tous les
côtés. Il cherche un abri au-delà de ≈ 38 km/h de vent.

**Dans Mouflon** : au coup de vent (une surprise du bureau), un mouton qui a
le vent dans le museau (`when = "into_wind"`) fait demi-tour, puis se
protège : tête basse, oreilles couchées, serré sur ses pattes (`brace`), et
s'ébroue quand c'est fini. Lainou et Pelote tournent le dos au vent eux
aussi, avec leurs propres images.

## 9. Les techniques d'animation

Ce que font de mieux les jeux et la recherche, et ce que Mouflon en garde
(tout doit tenir dans quelques microsecondes par animal : mille moutons à
l'écran restent fluides).

| Technique | Ce que c'est | Dans Mouflon |
|---|---|---|
| Horloge d'allure (Spore, Hecker) | une phase par patte, φᵢ = φ + oᵢ ; la patte est à l'appui tant que φᵢ < appui | oui : décalages et appuis par allure, mêlés |
| Roue d'arpenteur (Rosen, GDC) | la phase avance avec la distance parcourue, pas avec le temps | oui : les sabots ne glissent jamais |
| Ajuster la foulée (« stride warping », Unreal) | la foulée suit la vitesse | oui : par allure (×1,15 au trot, ×1,9 au galop) |
| Pied planté | le point de pose reste fixe pendant l'appui (Trico, Coros 2011) | oui, à sa façon : la patte rigide balaie exactement le chemin du corps et s'allonge pour rester au sol |
| IK à deux os, FABRIK | genoux et cous articulés | oui (depuis novembre 2026) : une IK analytique à deux os pour l'encolure (le museau au sol) et chaque patte (le sabot posé, le genou plié du bon côté) ; la patte d'appui se tasse avant de plier, l'encolure s'étire seule (la tête reste rigide) |
| Le corps d'après les pieds | hauteur = min √(L² − Δx²) des pattes d'appui ; tangage d'après l'avant et l'arrière | approché : garrot et croupe calés sur les mi-appuis (le même résultat, sans calcul) |
| Dynamique du second ordre (t3ssel8r) | un ressort réglé par une fréquence, un amortissement et une « réponse » (dépassement, anticipation) | oui : oreilles, queue, inclinaison, écrasement, balancement sont des ressorts amortis |
| Ressorts à amortissement critique, inertialisation (Holden) | passer d'une pose à l'autre sans à-coup, en amortissant l'écart | oui : tous les poids (allures, poses, gestes, posture) s'approchent de leur cible en douceur |
| Mouvement secondaire (la toison) | les parties molles suivent le corps avec retard | des mèches (`sway`) en ressorts amortis (2 et 1,6 Hz, celles du bout en retard sur les premières) : elles traînent avec la vitesse (0,0016 rad par pt/s), penchent avec l'élan quand le corps freine ou démarre (en physique réaliste) et dans le vent (0,3 rad au plus fort d'une rafale), balancent au rythme des pas (deux fois par cycle), suivent la secousse quand il s'ébroue ; une mèche penche sans quitter son attache (cisaillée autour de la ligne de son pivot) ; aucune étude n'en parle, réglé à l'œil |
| Chaînes de Verlet (Jakobsen) | queues et oreilles en points reliés | à notre façon (essai 19) : une queue en deux ou trois morceaux est une chaîne de ressorts amortis, chacun suivant le précédent avec un temps de retard (2,2 puis 1,8 Hz) : un coup de queue descend jusqu'au bout, elle ondule |
| Bruit de Perlin au repos (Improv) | de petits mouvements qui ne se répètent pas | en partie : regards, oreilles, queue et clignements tirés au hasard, respiration |
| Clignements et regard (Eyes Alive) | clignements asymétriques, liés aux tours de tête | oui (voir plus haut) |
| Principes de Disney (Lasseter) | écrasement à volume constant, anticipation, le timing fait le poids, les parties lâches traînent | écrasement à l'atterrissage, il se ramasse avant de sauter, secousse qui file de la tête à la queue, oreilles et queue en retard sur le corps ; gratter le sol et reculer avant de charger |
| MANN, DeepPhase, Peng 2020 | réseaux de neurones qui mêlent les allures, phases par membre, imitation d'animaux filmés | l'idée retenue : mêler les allures en continu, sans machine à états ; les réseaux, trop lourds ici |
| RDR2 (chevaux), Ubisoft IK Rig, Rain World | vitesses min/idéale/max par allure, mouvements en longueurs de patte, créatures en points + images | nos allures sont rapportées à la longueur des pattes ; nos dessins sont posés sur des signaux |

## 10. Quatre races d'après nature

Le Mannequin et Aquarelle sont des moutons « en général ». Quatre animaux du
dépôt suivent au contraire une race réelle, au plus près. Pour chacune, une
fiche d'abord (le standard de la race, les études, les mesures, une quinzaine
de photographies regardées de près, et les points où les sources se
contredisent), puis un dessin vectoriel tiré de la fiche, à l'échelle (une
unité du dessin pour un centimètre de l'animal, affiché à 0,5 pt par cm) et
écrit par un script qu'on relance pour le retoucher (`tools/races`) :
on les reconnaît à leurs dix traits les plus sûrs, et on évite ce qui en
ferait une autre race. Leur `pet.toml` reprend tous les comportements du
Mannequin, réglés sur le tempérament de la race.

| Race | L'animal dessiné | Ce qui la fait reconnaître | Sa vie |
|---|---|---|---|
| **Mouflon de Corse** (`pets/mouflon-corse`) | un bélier du Cinto d'environ huit ans (cornes finies de pousser), en pelage d'hiver ; 75 cm au garrot | poil ras, pas de toison ; l'avant presque noir avec son jabot, l'arrière châtain ; la **selle** blanche sur le flanc, derrière l'épaule (pas sur le dos) ; une bande noire au bas du flanc, au-dessus du ventre blanc ; balzanes blanches ; museau et tour de l'œil blancs, chanfrein busqué ; queue courte et sombre sur un miroir blanc ; cornes lourdes, annelées, de section triangulaire, sur 270 à 320° autour de l'oreille, la pointe vers la gorge, jamais écartée | sauvage : il guette souvent (et l'alerte se propage), tape du pied quand le pointeur approche, fuit au galop à la première alarme puis se retourne pour regarder, saute volontiers d'un rebord ; il bêle peu. Il **ne se cabre pas** : aucune source ne le décrit chez lui (c'est le mouflon canadien qui se dresse) ; il recule, puis charge tête basse |
| **Nez noir du Valais** (`pets/nez-noir`) | une brebis de deux à quatre ans en pleine toison ; 75 cm au garrot | longues mèches blanches ondulées partout, jusqu'aux pattes ; un **masque** noir du museau au milieu du front, sans blanc entre le nez et les yeux, sous un toupet blanc ; oreilles noires jusqu'à la base, à l'horizontale sous des cornes pâles en spirale ; taches noires devant les genoux et derrière les jarrets, séparées des **bottes** noires par du blanc ; aucun noir sur le corps ; longue queue en trois morceaux | calme, sociable, curieuse : elle broute, se couche et renifle plus volontiers, s'alarme et charge moins |
| **Jacob** (`pets/jacob`) | un bélier de trois à cinq ans à **quatre cornes**, du type américain (le plus souvent à quatre cornes, ossature fine) ; 75 cm au garrot | les cornes du haut dressées en V, celles du bas en boucle autour de l'oreille, de la laine blanche entre les deux paires ; toison blanche à **taches noires** nettes (environ 40 %), surtout sur le cou et les épaules, pointes brûlées de soleil ; tête de blaireau : joues noires, chanfrein blanc de la nuque au nez, museau sombre ; pattes blanches sans laine, taches aux genoux et aux jarrets ; petites oreilles noires dans la boucle des cornes ; queue naturelle jusqu'au jarret | vif, aux aguets, agile : il regarde souvent (contagieux), tape du pied pour l'alarme, fuit plus vite et le troupeau le suit, saute volontiers d'un rebord ; il suit moins les autres en marchant (troupeau lâche), bêle peu, les béliers se cognent plus souvent |
| **Ouessant** (`pets/ouessant`) | un bélier noir de trois à cinq ans ; 47 cm au garrot, l'un des plus petits moutons du monde | petit mais proportionné, pas un nain ; une seule paire de cornes, chacune une large spirale autour de l'oreille, détachée de la face ; tout noir, sans une tache blanche ; toupet laineux, face nue noir de jais sous les yeux ; vraie toison de mèches ondulées roussies au soleil, jusqu'aux genoux et aux jarrets ; la **cravate**, de longs poils noirs sous la gorge ; queue courte et fine, au-dessus des jarrets ; petites oreilles rondes à l'horizontale, yeux dorés | vif et très grégaire : il broute davantage (en groupe), rumine couché plus longtemps, détale en bande par courtes pointes puis fait **volte-face** pour regarder ; rustique, il tourne le dos au vent et se blottit peu |

Leurs allures restent justes à leur taille : les vitesses du Mannequin sont
mises à l'échelle de leurs pattes (`stride` vaut 1,6 fois la longueur des
pattes), si bien que le moteur choisit le pas, le trot ou le galop au même
nombre de longueurs de patte par seconde (§ 1) ; les sauts de l'Ouessant sont
réduits comme la racine de sa taille, pour garder la même hauteur rapportée
au corps.

Depuis le chantier des articulations (novembre 2026), chaque race a une
encolure qui porte la tête (le collier du mouflon ; ailleurs l'avant du haut
de la toison, posé sous le corps, avec une épaisseur de laine cachée qui
comble l'espace quand la tête descend) et des pattes en deux segments,
coupées au carpe et au jarret : les quatre broutent le museau dans l'herbe
(le mouflon s'arrêtait à 5 cm), se couchent les pattes repliées, plient le
carpe et le jarret en marchant. Leurs générateurs font la coupe eux-mêmes
(`tools/races/articulations.py`). Ce qui se voit encore : chez le Nez noir,
les longues mèches de l'encolure tournent avec elle (une mèche se met à
l'horizontale quand il broute) ; à leur taille habituelle, la laine et les
annelures des cornes se fondent (on lit la silhouette, les taches et les
cornes ; le détail paraît quand on les agrandit) ; les races n'ont pas
encore de lèvre ni de mèches qui bougent.

Les sources de ces fiches, par race :

- Mouflon de Corse : le Plan national d'actions 2024-2033 (DREAL Corse, OFB)
  <https://arpi.unipi.it/retrieve/ebe90e8e-8bdf-4b75-b175-69ca1f412c4c/PNA_muflone%20Corsica%202024-2033.pdf> ;
  OFB 2024, *E Mufre di Corsica, 50 ans de travaux*
  <https://ofb.gouv.fr/doc/e-mufre-di-corsica-les-mouflons-de-corse-ovis-gmelini-musimon-var-corsicana-50-ans-de-travaux> ;
  la fiche de l'INPN <https://inpn.mnhn.fr/docs-web/docs/download/440950> ;
  Valette 2010, *Grande Faune* (ANCGG)
  <https://www.ancgg.org/wp-content/uploads/2020/10/Le-mouflon-Corse-et-medtterraneen.pdf> ;
  Garel, Cugnasse et Maillard 2005, l'âge par les cornes
  <https://agris.fao.org/search/en/records/65dfb4c263b8185d9cae210b>.
- Nez noir du Valais : le standard suisse, dans ses traductions de la Valais
  Blacknose Society <https://valaisblacknosesociety.co.uk/the-breed/> et de
  l'association nord-américaine <https://vbsana.com/breed/breed-standard/> ;
  la fiche de la LfL de Bavière
  <https://www.lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_walliser_schwarznasenschaf.pdf> ;
  la fiche de l'ASSONAPA
  <https://www.assonapa.it/public/schede-ovini/conservazione-e-salvaguardia/autoctone/SCHWARZNASENSCHAF.pdf>.
- Jacob : les standards de la Jacob Sheep Society
  <https://jacobsheepsociety.co.uk/breed-standard/> et de la JSBA
  <https://jsba.org/breed-standard/>, sa fiche sur les cornes
  <https://jsba.org/wp-content/uploads/2024/05/Horns.pdf> ; la Livestock
  Conservancy
  <https://livestockconservancy.org/heritage-breeds/heritage-breeds-list/jacob-american-sheep/> ;
  Anjola et McEwan 2018, la couleur et le nombre de cornes
  <https://rgu-repository.worktribe.com/output/249253/inheritance-patterns-of-coat-colouration-and-horn-number-in-jacob-sheep>.
- Ouessant : le standard du GEMO <http://www.mouton-ouessant.com/les-standards/>
  et ses caractères généraux
  <http://www.mouton-ouessant.com/generalites/caracteres-generaux> ; les notes
  de l'Ouessant Sheep Society sur le standard
  <https://ouessantsheep.org.uk/wp-content/uploads/2024/05/Guide-to-breed-standard.pdf> ;
  la fiche de l'ASSONAPA
  <https://www.assonapa.it/public/schede-ovini/conservazione-e-salvaguardia/estere/PECORA%20D'OUESSANT.pdf>.

## 11. Ce qui reste à faire

Fait à l'essai 18 : les regroupements, les jeux d'agneaux (sauts tordus,
demi-tours en l'air, courses, coups de front), le bélier qui recule avant de
charger, le sommeil la tête au sol et les pattes qui tressaillent,
l'anticipation avant un saut. À l'essai 19 : le bélier qui se cabre, l'agneau
qui invite au jeu, les queues qui ondulent, les pupilles horizontales, et
quatre races dessinées d'après nature (§ 10). Au chantier des articulations
(novembre 2026) : l'encolure qui porte la tête et broute le museau dans
l'herbe, les pattes en deux segments (le carpe et le jarret qui plient, les
pattes repliées sous l'animal couché), la lèvre et le flehmen, la toison qui
bouge. Reste :

- **L'agneau qui tète**, la brebis qui l'appelle (un bêlement grave, bouche
  fermée). Les tétées durent 41 ± 10 s à une ou deux semaines, 12 ± 1 s à six
  ou sept semaines, 36 puis 14 par jour (Gordon & Siegmann 1991, Mérinos) ;
  l'agneau se glisse sous le flanc de sa mère, la tête levée, et frétille de
  la queue. Il faudrait un lien entre une mère et son agneau (le troupeau ne
  connaît que des familles, pas des mères) et une rencontre qui place
  l'agneau sous le ventre de l'autre, à l'envers de lui : pas fait.
- **Le dos qui plie** (s'étirer, ruer, bondir, se coucher) : un arrière-train
  coupé au milieu du dos a été essayé sur le Mannequin ; dès 7°, le ventre
  s'ouvre ou se double à la coupe. Une toison est d'un seul tenant : il
  faudrait déformer le dessin (un maillage), pas faire tourner des morceaux
  cuits. Pas fait ; le corps s'incline d'un bloc.
- **Les races** : une lèvre et des mèches qui bougent (le Nez noir, aux
  longues mèches, en profiterait le plus), des mèches d'encolure qui pendent
  au lieu de tourner avec elle.
- Le sommeil la tête tournée sur le flanc (plutôt la vache ; le mouton pose
  la tête au sol, ce que font nos moutons).
- Pour les races : le cri d'alarme du mouflon (un sifflement bref par les
  naseaux, souvent avec un coup de patte) et son grondement en rut ; les
  brebis et les agneaux de chaque race (la brebis d'Ouessant n'a pas de
  cornes, l'agneau Jacob naît avec sa tête de blaireau, le bélier Nez noir
  porte des cornes bien plus fortes que la brebis) ; le flehmen devant une
  brebis plutôt qu'après n'importe quel mouton (le moteur ne sait pas encore
  qui est bélier ou brebis).

## Sources

Allures et biomécanique :

- Usherwood & Self Davies 2017, la diagonalité du pas : <https://elifesciences.org/articles/29495>
- Biancardi & Minetti 2012, galops transverse et rotatoire : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/10/Biancardi-and-Minetti_2012_Biomechanical-determinants-of-transverse-and-rotary-gallop-in-cursorial-mammals.pdf>
- Hildebrand 1977, les allures asymétriques : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/04/Hildebrand_1977_Analysis-of-Asymmetrical-Gaits.pdf>
- Dagg 1973, les allures des mammifères : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/03/Dagg_1973_Gaits-in-mammals.pdf>
- Heglund & Taylor 1988, vitesse et cadence : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/10/Heglund-and-Taylor_1988_Speed-and-stride-frequency-and-energy-cost-per-stride-how-do-they-change-with-body-size-and-gait.pdf>
- Jayes & Alexander 1978 (tableau des transitions) : <https://arxiv.org/pdf/2111.01674>
- Les pupilles horizontales et la rotation de l'œil en broutant, Banks et al. 2015 : <https://theconversation.com/revealed-why-animals-pupils-come-in-different-shapes-and-sizes-45796>
- Le pas du mouton : Agostinho 2012 <https://link.springer.com/article/10.1186/1746-6148-8-87> ; Mérinos <https://www.frontiersin.org/journals/bioengineering-and-biotechnology/articles/10.3389/fbioe.2024.1370101/full> ; Diogo 2021 <https://pmc.ncbi.nlm.nih.gov/articles/PMC8415828> ; mouton de montagne autrichien <https://doi.org/10.3920/CEP150008>
- Lee 2011, la chèvre : <https://pmc.ncbi.nlm.nih.gov/articles/PMC3163420>
- Rhodin 2022, les mouvements de la tête, du garrot et de la croupe : <https://doi.org/10.3390/ani12213053>
- Le hochement de tête : <https://doi.org/10.1098/rspb.2016.1908>
- Usherwood 2007, l'angle des pattes : <https://cob.silverchair.com/jeb/article-pdf/210/3/533/1538679/533.pdf>
- Le stotting : <https://en.wikipedia.org/wiki/Stotting>
- Le carpe, le coude, le jarret au pas, Faria et al. 2014, *BMC Veterinary Research* 10:294 : <https://doi.org/10.1186/s12917-014-0294-4>
- Le grasset, le jarret, le boulet au pas (2D et 3D), Diogo et al. 2021, *Frontiers in Veterinary Science* 8:545708 : <https://doi.org/10.3389/fvets.2021.545708>
- Le Mérinos au pas, flexion au milieu du vol, Silva et al. 2024, *Frontiers in Bioengineering and Biotechnology* 12:1370101 : <https://doi.org/10.3389/fbioe.2024.1370101>
- L'angle de la tête et de l'encolure du cheval qui mange au sol (morphométrie géométrique), *Animals* 2021, 11:763 : <https://doi.org/10.3390/ani11030763>

Comportement du mouton :

- Bouchées : <https://link.springer.com/article/10.1038/s41598-020-68827-0> ; Gibb 1997 (Rothamsted) ; pas entre les stations : <https://animres.edpsciences.org/articles/animres/pdf/1995/05/Ann.Zootech._0003-424X_1995_44_Suppl1_ART0076.pdf>
- Rumination, Gordon 1958 : <https://www.cambridge.org/core/journals/journal-of-agricultural-science/article/act-of-rumination/E4CF60617D8D20B3C9A27E29836DC697>
- Le mouton chez MSD : <https://www.msdvetmanual.com/behavior/normal-social-behavior-and-behavioral-problems-of-domestic-animals/social-behavior-of-sheep>
- S'ébrouer, Dickerson 2012 : <https://pmc.ncbi.nlm.nih.gov/articles/PMC3481573>
- S'étirer, Fraser 1989 : <https://baillement.com/stretching-fraser.html> ; bâiller, Yonezawa : <https://baillement.com/replication/sheep_yonezawa.html>
- Se gratter : <https://pmc.ncbi.nlm.nih.gov/articles/PMC9817768>
- Les oreilles, Boissy 2011 : <https://www.cambridge.org/core/journals/animal-welfare/article/cognitive-sciences-to-relate-ear-postures-to-emotions-in-sheep/92D5B366151975AC4249A263562C2421> ; Reefmann 2009 : <https://czaw.org/keyword/ear-postures/> ; seuils : <https://pmc.ncbi.nlm.nih.gov/articles/PMC4672170>
- Vigilance : <https://pmc.ncbi.nlm.nih.gov/articles/PMC3078120/> ; <https://pmc.ncbi.nlm.nih.gov/articles/PMC9899428>
- Taper du pied : <https://pmc.ncbi.nlm.nih.gov/articles/PMC7070376>
- Suivre : <https://pmc.ncbi.nlm.nih.gov/articles/PMC3016320/> ; <https://ar5iv.arxiv.org/html/1712.05774> ; regroupements, Ginelli 2015 : <https://pmc.ncbi.nlm.nih.gov/articles/PMC4611628>
- Jeux d'agneaux : <https://stud.epsilon.slu.se/13350/1/Augustsson_S_180314.pdf>
- Le flehmen du bélier : durées (jeunes béliers Najdi, une brebis entravée, quatre séances de 15 min), Bahadi et al. 2023, *Animal Reproduction* 20:e20220051 : <https://doi.org/10.1590/1984-3143-AR2022-0051> ; la posture, ce qui le déclenche et sa durée (d'après Banks 1964 et Bland & Jubilan 1987), Blissitt 1989, thèse, Édimbourg : <http://hdl.handle.net/1842/34797> ; Bland & Jubilan 1987, *Animal Behaviour* 35:735 : <https://doi.org/10.1016/S0003-3472(87)80109-4> ; il suit l'urine de la brebis, Stevens, Perry & Long 1982, *Journal of Chemical Ecology* 8:23 : <https://doi.org/10.1007/BF00984002> ; Mirto et al. 2015, *Applied Animal Behaviour Science* 186:22 : <https://doi.org/10.1016/j.applanim.2015.09.001>
- La tétée (durée et nombre par jour), Gordon & Siegmann 1991, *Physiology & Behavior* 50:1079 : <https://doi.org/10.1016/0031-9384(91)90442-q>
- Sommeil : <https://www.pharma.uzh.ch/apps/sleepcd/demo/animals/chap8/is165.htm> ; postures couchées : <https://pmc.ncbi.nlm.nih.gov/articles/PMC12411910>
- Respiration : <https://www.msdvetmanual.com/multimedia/table/resting-respiratory-rates>
- Voix : <https://www.biorxiv.org/content/10.1101/2025.07.29.667358.full.pdf>
- Le vent et le froid : <https://smallfarms.cornell.edu/2024/10/keeping-sheep-comfortable>

Techniques d'animation :

- Hecker, Spore : <https://www.cse.chalmers.se/edu/year/2011/course/TDA361/Advanced%20Computer%20Graphics/027-hecker%20copy.pdf>
- Rosen, la roue d'arpenteur : <https://www.gdcvault.com/play/1020583/>
- Coros 2011, quadrupèdes simulés : <https://www.cs.ubc.ca/~van/papers/2011-TOG-quadruped/paper.pdf>
- Johansen, locomotion semi-procédurale : <https://runevision.com/thesis/rune_skovbo_johansen_thesis.pdf>
- t3ssel8r, dynamique du second ordre : <https://www.youtube.com/watch?v=KPoeNZZ6H4s>
- Holden, « Spring-It-On » : <https://theorangeduck.com/page/spring-roll-call>
- Jakobsen, Verlet : <https://gamedeveloper.com/programming/advanced-character-physics>
- Lasseter, les principes de l'animation : <https://www.ics.uci.edu/~fowlkes/class/ics162/exercises/lasseter.pdf>
- Perlin, Improv : <https://mrl.cs.nyu.edu/~perlin/improv-paper>
- Eyes Alive : <https://www.cis.upenn.edu/~badler/papers/EyesAlive.pdf> ; dynamique des clignements : <https://frontiersin.org/journals/systems-neuroscience/articles/10.3389/fnsys.2023.1242654/full>
- Les chevaux de Red Dead Redemption 2 : <https://www.gamedeveloper.com/design/rockstar-s-quest-for-the-ultimate-video-game-horse>
- Peng 2020, imiter les animaux : <https://bair.berkeley.edu/blog/2020/04/03/laikago/>
