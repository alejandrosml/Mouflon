# L'animation des animaux : ce que disent les études, ce qu'en fait Mouflon

Mouflon anime lui-même ses moutons dessinés en vecteurs (le Mannequin) ou
peints sur un squelette (Aquarelle) : à partir d'une seule pose de profil, le
moteur calcule à chaque image où vont les pattes, le corps, la tête, les
oreilles, la queue, les paupières et la mâchoire. Pour que ce mouvement soit
juste, il s'appuie sur ce que les études disent des vrais moutons — et des
autres quadrupèdes quand il n'y a rien sur le mouton — et sur ce que font de
mieux les jeux et les films.

Ce document rassemble ces connaissances (recherche de novembre 2026), dit
pour chacune ce que Mouflon en fait, avec ses valeurs, et ce qui reste à
faire. Les longueurs du moteur sont en **u**, le centième de la hauteur du
dessin de l'animal ; les instants d'une foulée, en fractions de la foulée
comptées depuis le poser du postérieur qu'on voit.

Le code : `crates/mouflon-render/src/animate.rs` (les allures),
`crates/mouflon-render/src/gesture.rs` (les gestes et la posture),
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

### Le saut d'agneau

Le « stotting » ou « pronking » : des bonds droits en l'air, les quatre
pattes raides et tendues, qui partent et retombent ensemble (Caro 1986) ;
la gazelle de Thomson garde la tête haute. Chez les moutons et les chèvres
domestiques, surtout les jeunes.

**Dans Mouflon** : `motion = "pronk"`, pattes raides, la tête qui secoue ; les
animaux actifs (tempérament) en font volontiers, et « Tous ensemble » fait
sauter tout le troupeau.

### Se coucher, se relever

Le bovin se couche l'avant d'abord (il tombe sur les carpes, puis
l'arrière) et se relève l'arrière d'abord ; le mouton aussi (WikiVet). Chez
le veau, 3,4 à 4,4 s pour se coucher, 2,4 s pour se relever.

**Dans Mouflon** : c'est l'ordre du moteur depuis le début ; en se relevant,
nos moutons s'étirent souvent (voir plus bas).

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

## 3. Les gestes

| Geste | Ce qu'en disent les études | Dans Mouflon |
|---|---|---|
| S'ébrouer | un mouton de 55 kg secoue sa toison à 6,5 Hz (la loi des mammifères, f ∝ M^−0,22, donnerait 4,9 Hz) ; le corps tourne de 60 à 110° ; 3 ou 4 allers-retours (Dickerson 2012) | `shake` : 6 Hz ; la secousse part de la tête et file jusqu'à la queue, la tête paraît plus étroite à chaque aller et retour (elle tourne), la toison ondule, les oreilles battent, les yeux se ferment ; 0,9 s |
| S'étirer | surtout en se relevant : cou tendu, dos droit, une patte arrière tendue loin derrière, parfois plusieurs fois ; absent chez un animal malade (Fraser 1989) | `stretch` : le cou et l'avant d'abord, puis une patte arrière ; 2,4 s ; après le sommeil et en se relevant |
| Bâiller | 1 à 2 fois par jour ; contagieux : 11 % dans la minute qui suit le bâillement d'un congénère | `yawn` : la tête se lève, la bouche s'ouvre grand, les yeux se ferment ; contagieux (`contagious = 0.15`) ; avant de s'endormir |
| Se gratter | d'un sabot arrière, ou contre un support : 8 % des soins chez l'agneau (le toilettage à la bouche : 39 %) | `scratch` : un postérieur remonte gratter le ventre à petits coups rapides (4 Hz), la tête tournée vers lui, les yeux mi-clos |
| Taper du pied | face à une menace (un chien), un pied levé puis posé avec force ; rare, et le chien s'arrête souvent | `stamp` : tête haute, oreilles dressées, une patte de devant monte lentement et s'abat ; quand le pointeur file tout près |
| Gratter le sol | menace (bélier), et avant l'agnelage ; **pas** avant de se coucher | `paw` : un antérieur racle le sol, tête basse ; avant qu'un énervé ne charge |
| Charger | le bélier recule, baisse la tête, fonce (le mouflon canadien se cabre, puis ≈ 40 km/h) | `charge` : tête basse, oreilles couchées, au galop |
| Bêler | aigu bouche ouverte : isolement, stress ; grave bouche fermée : brebis et agneau proches ; sinon rare | `bleat` et les sons `mouth = true` : la bouche s'ouvre en 0,1 s, un léger vibrato |
| Renifler | le museau tendu vers l'autre | `sniff` : quand deux moutons se croisent |
| Sonné | — | `daze` : la tête décrit de petits cercles, les yeux mi-clos, les oreilles pendantes ; après un atterrissage rude, puis il s'ébroue |

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
chance indiquée, dans un délai de 0,4 à 5 s, s'ils sont libres (debout, à
marcher, à brouter, à regarder autour d'eux). Un mouton qui part entraîne
ses voisins du même côté (`walk`, une chance sur quatre) ; quand l'un
détale, les autres détalent (`run`, 0,45) ; l'alerte (`look`, 0,25), le
bâillement (`yawn`, 0,15) et les sauts d'agneau (`pronk`, 0,3) se
propagent. Celui qui vient d'imiter attend 25 s avant d'imiter de nouveau,
et ce qu'il imite ne se propage pas plus loin : pas de va-et-vient sans fin.

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

**Dans Mouflon** : 20 respirations par minute éveillé, 14 endormi ; endormi,
une oreille tressaille toutes les 5 à 14 s.

## 7. Les yeux

- Un clignement : la paupière se ferme en 75 à 100 ms, reste close 10 à
  50 ms, se rouvre deux fois plus lentement ; parfois deux de suite.
- Le cheval détendu cligne 8 à 9 fois par minute (5 quand il est stressé) ;
  rien de publié sur le mouton.
- Un clignement accompagne presque tous les grands mouvements du regard, et
  tout tour de tête de plus de 17°.

**Dans Mouflon** : 0,27 s par clignement (fermé en 0,08 s, clos 0,03 s,
rouvert en 0,16 s), toutes les 2,5 à 8 s, doublé une fois sur cinq ; il
cligne aussi en faisant demi-tour. Les gestes ferment les yeux à leur façon
(s'ébrouer, bâiller, sonné…).

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
| IK à deux os, FABRIK | genoux et cous articulés | non : nos pattes sont d'une pièce, et le profil n'en a pas besoin |
| Le corps d'après les pieds | hauteur = min √(L² − Δx²) des pattes d'appui ; tangage d'après l'avant et l'arrière | approché : garrot et croupe calés sur les mi-appuis (le même résultat, sans calcul) |
| Dynamique du second ordre (t3ssel8r) | un ressort réglé par une fréquence, un amortissement et une « réponse » (dépassement, anticipation) | oui : oreilles, queue, inclinaison, écrasement, balancement sont des ressorts amortis |
| Ressorts à amortissement critique, inertialisation (Holden) | passer d'une pose à l'autre sans à-coup, en amortissant l'écart | oui : tous les poids (allures, poses, gestes, posture) s'approchent de leur cible en douceur |
| Chaînes de Verlet (Jakobsen) | queues et oreilles en points reliés | pas encore (nos queues sont d'une pièce) |
| Bruit de Perlin au repos (Improv) | de petits mouvements qui ne se répètent pas | en partie : regards, oreilles, queue et clignements tirés au hasard, respiration |
| Clignements et regard (Eyes Alive) | clignements asymétriques, liés aux tours de tête | oui (voir plus haut) |
| Principes de Disney (Lasseter) | écrasement à volume constant, anticipation, le timing fait le poids, les parties lâches traînent | écrasement à l'atterrissage, secousse qui file de la tête à la queue, oreilles et queue en retard sur le corps ; gratter le sol avant de charger |
| MANN, DeepPhase, Peng 2020 | réseaux de neurones qui mêlent les allures, phases par membre, imitation d'animaux filmés | l'idée retenue : mêler les allures en continu, sans machine à états ; les réseaux, trop lourds ici |
| RDR2 (chevaux), Ubisoft IK Rig, Rain World | vitesses min/idéale/max par allure, mouvements en longueurs de patte, créatures en points + images | nos allures sont rapportées à la longueur des pattes ; nos dessins sont posés sur des signaux |

## 10. Ce qui reste à faire

- Les **regroupements** : de temps en temps, un mouton du bord rejoint le
  centre en courant, et tout le troupeau suit en vague.
- Les **jeux d'agneaux** : sauts avec torsion du corps (« gambol »), petit
  tour sur place (« pivot »), courses à plusieurs, coups de front après un
  recul ; plus le soir.
- Le **bélier qui recule avant de charger** (ou se cabre).
- Le sommeil : la tête posée sur le flanc, le cou relâché, les pattes qui
  tressaillent.
- Une queue en chaîne de Verlet ; l'anticipation (s'écraser un instant
  avant un saut).
- Les pupilles du mouton, horizontales, qui restent à l'horizontale quand
  la tête descend brouter.

## Sources

Allures et biomécanique :

- Usherwood & Self Davies 2017, la diagonalité du pas : <https://elifesciences.org/articles/29495>
- Biancardi & Minetti 2012, galops transverse et rotatoire : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/10/Biancardi-and-Minetti_2012_Biomechanical-determinants-of-transverse-and-rotary-gallop-in-cursorial-mammals.pdf>
- Hildebrand 1977, les allures asymétriques : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/04/Hildebrand_1977_Analysis-of-Asymmetrical-Gaits.pdf>
- Dagg 1973, les allures des mammifères : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/03/Dagg_1973_Gaits-in-mammals.pdf>
- Heglund & Taylor 1988, vitesse et cadence : <https://www.originalwisdom.com/wp-content/uploads/bsk-pdf-manager/2019/10/Heglund-and-Taylor_1988_Speed-and-stride-frequency-and-energy-cost-per-stride-how-do-they-change-with-body-size-and-gait.pdf>
- Jayes & Alexander 1978 (tableau des transitions) : <https://arxiv.org/pdf/2111.01674>
- Le pas du mouton : Agostinho 2012 <https://link.springer.com/article/10.1186/1746-6148-8-87> ; Mérinos <https://www.frontiersin.org/journals/bioengineering-and-biotechnology/articles/10.3389/fbioe.2024.1370101/full> ; Diogo 2021 <https://pmc.ncbi.nlm.nih.gov/articles/PMC8415828> ; mouton de montagne autrichien <https://doi.org/10.3920/CEP150008>
- Lee 2011, la chèvre : <https://pmc.ncbi.nlm.nih.gov/articles/PMC3163420>
- Rhodin 2022, les mouvements de la tête, du garrot et de la croupe : <https://doi.org/10.3390/ani12213053>
- Le hochement de tête : <https://doi.org/10.1098/rspb.2016.1908>
- Usherwood 2007, l'angle des pattes : <https://cob.silverchair.com/jeb/article-pdf/210/3/533/1538679/533.pdf>
- Le stotting : <https://en.wikipedia.org/wiki/Stotting>

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
