> Fiche de la race « Texel », préparée le 3 novembre 2026 pour dessiner un futur `pets/texel` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Texel : fiche pour un dessin de profil

**Noms :** Texel (FR, EN) · Texelaar, Texels schaap (NL) · Texelschaf (DE). Variantes à ne pas confondre : Texel français, British Texel, Texel belge (et le Beltex qui en descend), Texel bleu (*Blauwe Texelaar*), Texel Dassenkop (tête de blaireau).
**Préparée le** 3 novembre 2026, d'après les descriptions des trois livres généalogiques (Pays-Bas, Royaume-Uni, France par la voie du programme wallon), des mesures publiées, une étude brésilienne de biométrie, les études génétiques sur la musculature, et une trentaine de photos regardées de près (section 10.2).

**Comment lire les étiquettes :**
- **[S]** : source écrite (le code renvoie à la section 10.1).
- **[P]** : vu ou mesuré par moi sur une photo de la section 10.2 (pixels comptés, couleurs moyennées sur de petites zones avec Python/PIL).
- **[E]** : mon estimation, tirée des photos et de l'anatomie générale du mouton. Aucune source ne la donne.

H désigne partout la **hauteur au garrot, os compris, laine non comptée**. Pour l'individu choisi, **H = 72 cm**.

---

## 0. En bref

- **Le sujet :** un **bélier néerlandais de trois ans, né sur l'île de Texel**, vu fin d'été (début septembre), environ cinq mois après la tonte de printemps. Sa toison est dense, courte (4 à 6 cm) et craquelée en petites plaques. Il se tient debout, carré, de profil strict, **tourné vers la droite**, tête portée à hauteur du dos.
- **L'allure en une phrase :** un mouton blanc **massif, presque cubique**, sans cornes. Il a :
  - une **tête blanche, large et courte, sans laine**, au poil ras et fin, avec un **nez noir** (truffe et bord des lèvres) ;
  - de **petites oreilles épaisses** portées à l'horizontale ou un peu relevées ;
  - un **gros œil ambré** dans une face nue ;
  - une encolure si courte et si épaisse que **la tête semble sortir directement du corps** ;
  - une toison crème qui commence **juste derrière les oreilles** comme une capuche, et s'arrête à mi-avant-bras et au-dessus du jarret ;
  - des **gigots ronds et descendus**, un dos large et plat comme une table ;
  - des **pattes blanches à poil ras** et des **onglons noirs** ;
  - une queue fine et courte, attachée haut.

**Palette de base** (détails en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Toison (été, au soleil) | `#DFCDB2` | `#EEE2CC` | `#BCA683`, creux `#9C8462`, fond des fentes `#6B5638` |
| Poil de la face | `#ECE4D8` | `#F4EEE6` | `#CFC4B2`, sous la mâchoire `#8E8574` |
| Truffe (peau nue) | `#1E1E21` | reflet humide `#4A4B50` | narine `#070707` |
| Lèvres | `#3A352E` | `#5E574C` | `#211E1A` |
| Iris | `#AE9068` (ambre) | `#C6A87C` | anneau `#5C4A45`, pupille `#1A1612` |
| Intérieur de l'oreille | `#C9A9A4` | contre-jour `#9A5A55` | `#8E7470` |
| Pattes (poil ras) | `#DDD2C3` | `#EDE4D8` | `#A89A84` |
| Onglons | `#2C2A28` | usure poussiéreuse `#6E665E` | `#141312` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 Un bélier

1. **La race existe pour ses béliers.** Le Texel est d'abord une race à viande dont les béliers servent de pères en croisement terminal :
   - au Royaume-Uni, « Texel rams have become the UK producer's preferred choice of Terminal Sire » (les béliers Texel sont devenus le père terminal préféré des éleveurs britanniques) [TSS-UK] ;
   - en Belgique, le programme wallon le présente comme « un bélier idéal en croisement sur des femelles de race plus prolifique » [AWE] ;
   - en France, « ses béliers sont recherchés pour le croisement industriel » [SVS].
2. **C'est le bélier qui est célèbre.** Les records de vente qu'on cite sont des béliers : 231 000 £ en 2009 et près de 368 000 £ en 2020 à Lanark [WP-EN]. La photo la plus nette de la catégorie Commons, prise sur l'île même, est un bélier [P : Bélier texel].
3. **La musculature, trait de la race, se voit mieux chez lui.** Les Texel sont « renowned for their exceptional meatiness » (réputés pour leur viande exceptionnelle) [Clop 2006]. Le standard belge demande que la tête du bélier « dénote le caractère mâle (présence de rides sur le nez) » [AWE]. Chez le bélier, les gigots, le dos large et l'encolure épaisse sont les plus marqués : la silhouette dit « Texel » même en petit.
4. **Sans cornes, il ne risque pas d'être pris pour une autre race à cornes.** Sa tête nue, blanche et large reste entièrement visible de profil. C'est elle, avec le nez noir, qui distingue la race (section 9).

### 1.2 L'individu exact

- **Type néerlandais** (le berceau de la race), plutôt que français ou britannique : le texte du livre généalogique néerlandais est le plus court et le plus clair sur la tête (« recht neusbeen », chanfrein droit), et la photo de référence en vient [NTS][P : Bélier texel].
- **Âge :** trois ans, adulte fini. La photo de référence a été prise le 7 septembre 2015 et sa légende dit : « Bélier Texel, né sur l'île de Texel, âgé de trois ans » [P].
- **Taille :** **H = 72 cm**, entre la moyenne du livre néerlandais (70 cm pour les béliers [NTS], 70 cm pour le « bélier moyen » du tableau des mensurations [NTS-mat]) et le bas de la fourchette française (75 à 82 cm [TX-FR]).
- **Poids :** environ **100 kg** [E] (95 kg en moyenne pour un bélier néerlandais [NTS]).
- **Toison :** environ cinq mois de repousse, 4 à 6 cm de mèche [E d'après P]. Elle est dense, crème, d'aspect craquelé.
- **Pose :** debout, carré, tête à hauteur du dos, oreilles à l'horizontale, queue basse.
- **À ne pas recopier de la photo :**
  - les boucles d'identification vertes aux oreilles ;
  - la peau rouge et plissée sous l'œil (irritation ou larmoiement de cet individu) ;
  - une **petite excroissance cornée brune sur la nuque** (voir 6.2) ;
  - la queue relevée (il urinait ou chassait une mouche).
- **Sens :** la photo de référence montre le bélier tourné vers la gauche ; **la retourner en miroir**. On voit alors son flanc gauche, comme demandé.

### 1.3 Pour l'autre sexe et pour l'agneau

| Trait | Bélier (choisi) | Brebis | Agneau (6 à 10 semaines) |
|---|---|---|---|
| Hauteur au garrot | 70 cm en moyenne aux Pays-Bas [NTS] ; 75 à 82 cm en France [TX-FR] | 68 cm [NTS] (66 cm pour la « brebis moyenne » du tableau [NTS-mat]) ; 68 à 72 cm en France [TX-FR] | environ 0,55 à 0,65 H adulte [E] |
| Poids | 95 kg [NTS] ; 115 à 130 kg [TX-FR] ; 120 à 140 kg [SVS] | 75 kg [NTS] ; 75 à 90 kg [SVS] ; 79,8 kg à 18 mois au Royaume-Uni [TSS-UK] | 4,9 kg à la naissance, 25 kg à 8 semaines [TSS-UK] |
| Tête | large, front plat, plis sur le nez, joues pleines [AWE][P] | « plus fine », typiquement féminine [AWE] ; museau plus étroit [P] | ronde, courte ; oreilles en proportion plus grandes, doublées de rose [P] |
| Encolure | très courte, épaisse, se fond dans les épaules | un peu plus longue et plus fine [P] | fine, nettement visible [P] |
| Corps | gigots très ronds, dos large | même plan, moins de masse ; pis couvert de poils soyeux [AWE] | longues pattes, corps court et rond, laine bouclée serrée [P] |
| Ventre | scrotum garni de poils soyeux [AWE], visible entre les cuisses | pis entre les cuisses | — |

L'agneau a déjà le nez noir et les pattes blanches. Sur deux photos d'agneaux, on voit **une tache grise sur le devant de chaque genou** : c'est la peau calleuse et salie où il s'agenouille pour téter, pas une marque de robe [P : A well-developed Texel lamb ; Schaap50%st]. Elle est facultative.

---

## 2. Tailles et proportions

### 2.1 Chiffres publiés

| Mesure | Bélier | Brebis | Sources |
|---|---|---|---|
| Hauteur au garrot, type néerlandais | **environ 70 cm** | **environ 68 cm** | [NTS][TX-NL][WP-EN, d'après FAO DAD-IS] |
| Hauteur au garrot, type français | **75 à 82 cm** | **68 à 72 cm** | [TX-FR] (repris par [Tireur]) ; « environ 80 / 70 cm » [WP-FR][TS] |
| Hauteur au garrot mesurée, 25 jeunes béliers de concours (14 mois), Brésil | **71,15 ± 6,17 cm** | — | [Unipar] |
| Hauteur à la croupe, même étude | **72,05 ± 5,74 cm** (un peu plus haute que le garrot à cet âge) | — | [Unipar] |
| Longueur du corps (pointe de l'épaule à la pointe de la fesse), même étude | **87,68 ± 9,42 cm** | — | [Unipar] |
| Tour de poitrine, même étude | **104,10 ± 9,99 cm** | — | [Unipar] |
| Poids, même étude | 74,5 ± 19,5 kg (à 14 mois) | — | [Unipar] |
| Longueur du « bélier moyen » et de la « brebis moyenne » du livre néerlandais | **83 cm pour 70 cm de haut (114 %)** | **77 cm pour 66 cm de haut (116 %)** | [NTS-mat], case marquée en rouge « de gemiddelde ram / ooi » |
| Poids adulte | 95 kg (NL) ; 115 à 130 kg (FR) ; 120 à 140 kg (OS français) ; environ 120 kg (GB) | 75 kg (NL) ; 75 à 80 kg (FR) ; 75 à 90 kg (OS) ; environ 85 kg (GB) | [NTS][TX-FR][SVS][WP-EN] |
| Longueur totale de la colonne (scanner) | 530,5 mm, moyenne de race des Texel contrôlés au Royaume-Uni (âge et sexe non précisés sur la page) | | [TSS-UK] |
| Épaisseur de muscle (échographie) | 28,2 mm de moyenne (même population) | | [TSS-UK] |

**Le tableau néerlandais des mensurations** [NTS-mat] sert à noter le « développement » des animaux présentés au livre. Il croise la hauteur (64 à 77 cm pour les béliers adultes, 59 à 72 cm pour les brebis) et la longueur exprimée en pour-cent de la hauteur (101 à 125 %). Les cases vertes, celles qui donnent assez de points pour la note A, sont **les plus longues pour leur hauteur** : la sélection néerlandaise pousse vers des animaux **plus longs que hauts**. Le texte le confirme : « Het resultaat is grotere, langere dieren » (le résultat, ce sont des animaux plus grands et plus longs) [NTS-fok].

**Ce qui manque :** aucune source ne donne la longueur de la tête, des oreilles ou de la queue, ni la hauteur du ventre, du genou ou du jarret. Ces valeurs sont prises sur photo [P] ou estimées [E] ci-dessous.

### 2.2 Ce que disent les standards de la silhouette

- **Pays-Bas** [NTS] : « Het lichaam van de Texelaar is ruim gebouwd, balkvormig en doet massaal aan » (le corps est ample, en forme de poutre, et paraît massif). La poitrine est « breed en diep » (large et profonde), la partie moyenne « lang, breed », les lombes « breed, sterk », la croupe « lang en licht hellend » (longue et un peu inclinée).
- **France** (Texel français) [AWE][TX-FR] :
  - « Le tronc est conique, bien fondu avec les épaules, sans gorge ni fanon, sans pli ni cravate. »
  - « La hauteur de la poitrine dépasse la moitié de la hauteur du garrot et donne une impression d'animal près de terre. »
  - « L'encolure est courte. Le corps est trapu. » La croupe est « horizontale et carrée, la queue attachée haut ». Les gigots sont « aussi arrondis et descendus que possible, entre des postérieurs bien écartés ».
- **Belgique** (Texel belge et Texel bleu) [AWE] : « Le dos est horizontal ou légèrement bombé jusqu'à la croupe. » Le garrot peut être « légèrement supérieur à la hauteur du bassin (upstanding) ».
- **Royaume-Uni** [TSS-UK] : « a medium sized sheep with a long rectangular body, with a level back and medium bone structure » (un mouton moyen au corps long et rectangulaire, au dos plat, à l'ossature moyenne) ; « a solid square stance and round well-developed gigots » (aplomb carré et solide, gigots ronds et bien développés).

**Pour le dessin :** une boîte rectangulaire aux angles très arrondis, un dos plat, la croupe aussi haute que le garrot (ou un poil plus haute), et des gigots qui forment derrière une grosse boule descendant jusqu'au jarret.

### 2.3 Rapports de dessin (bélier, H = 72 cm)

Mesures prises sur trois profils :
- « Bélier texel » (1280 px), le sujet. Il est très légèrement de trois quarts arrière : l'arrière est plus loin de l'objectif, réduit d'environ 0,86. Les sabots sont cachés dans l'herbe.
- « Texel sheep », animal de concours flamand sur sol dur, tourné vers la droite.
- « Texel ram near Friths Farm », bélier britannique tourné vers la droite.

| Mesure | En fraction de H | En cm (H = 72) | Étiquette |
|---|---|---|---|
| Hauteur du dessus de la toison au garrot | 1,07 | 77 | [E] (4 à 6 cm de laine en septembre) |
| Hauteur de la croupe (os) | 1,00 à 1,02 | 72 à 73 | [S : Unipar, croupe ≥ garrot à 14 mois][P] |
| Profondeur de poitrine (garrot au sternum, os) | **0,52** | 37 | [S : TX-FR « dépasse la moitié »][E] |
| Sol au sternum, à l'os | 0,48 | 35 | [E] |
| Bas de la toison du ventre au-dessus du sol (patte visible) | **0,38 à 0,40** | 27 à 29 | [P : 0,35 du dessus de la toison sur « Bélier texel », 0,36 sur « Texel sheep »] |
| Profondeur du corps en toison (dessus au bas du ventre) | 0,68 à 0,70 | 49 à 50 | [P : 0,65 du dessus de toison] |
| Longueur du corps, pointe de l'épaule à la pointe de la fesse (os) | **1,19 à 1,23** | 86 à 89 | [S : NTS-mat 114 à 116 % ; Unipar 123 %] |
| Longueur de la silhouette en toison, poitrail à fesse | 1,35 à 1,45 | 97 à 104 | [P : 1,32 du dessus de toison sur « Bélier texel », sous-estimé par la perspective] |
| Tour de poitrine | 1,46 | 105 | [S : Unipar, 104,1 / 71,15] |
| Longueur de la tête, nuque au bout du nez | **0,37** | 27 | [E ; P : 25 cm mesurés, tête un peu tournée vers l'objectif] |
| Hauteur de la tête de profil (front à la ganache) | 0,25 | 18 | [S : TX-FR, longueur ≈ 1,5 fois cette « largeur prise de profil »][P : rapport 1,33 mesuré, raccourci] |
| Oreille, longueur | 0,13 à 0,15 | 9 à 11 | [E ; P : 6,7 cm visibles, raccourcie] |
| Queue, de l'attache à la pointe | 0,28 à 0,30 | 20 à 22 | [P : environ 21 cm sur « Bélier texel », queue relevée] |

**Masses relatives** [P] :
- vu de profil, le corps en toison fait environ **3,7 longueurs de tête** ;
- les pattes visibles sous le ventre ne font qu'**un peu plus de la moitié de la profondeur du corps en toison**. C'est un animal « près de terre » [TX-FR] ;
- les gigots descendent jusqu'à **0,45 H environ** au-dessus du sol, c'est-à-dire plus bas que le bas du ventre : vue de côté, la cuisse fait une **bosse arrondie qui dépasse la ligne du ventre** [P : Bélier texel, Texel sheep, Texel tup at Pen-y-bryn].

### 2.4 L'encolure

**Les sources :**
- France : « L'encolure est courte » ; « cou court, trapu sans gorge » [AWE][Tireur].
- Belgique (Texel belge, Texel bleu) : « Le cou est musclé, bien soudé aux épaules et horizontal dans le prolongement du corps » [AWE].
- Pays-Bas : « De hals is voldoende lang en correct geplaatst » (le cou est assez long et correctement attaché) [NTS]. C'est le seul texte qui ne parle pas d'une encolure courte.
- British Wool : « a short neck » [BW].

**Sur les photos :**
- **Longueur :** de la nuque au garrot, le long du dessus, environ **0,5 H** (37 cm) [P : Bélier texel]. Mais la laine commence juste derrière les oreilles et recouvre tout le cou. La partie qui se lit comme « cou », entre la limite nette de la face et l'épaule, ne fait que **0,2 à 0,25 H** [P]. Le cou est **aussi épais que la tête est haute**, voire plus : il n'y a pas d'étranglement derrière la tête [P].
- **Au repos :** le dessus du cou part du garrot **à peu près à l'horizontale, ou en descendant de 10 à 15°**. La nuque est au niveau du dos ou un peu plus bas : 0 à 0,1 H sous le garrot [P : Bélier texel, Texel ram near Friths Farm]. L'axe de la face (nuque vers bout du nez) est incliné d'environ **45 à 50° sous l'horizontale** [P : 51° mesurés sur « Bélier texel »].
- **En alerte :** la nuque monte jusqu'à environ 0,1 H au-dessus du garrot, et la face se redresse à 35 ou 40° sous l'horizontale [P : Friths Farm ; Texelschafe 2003 ; Is that a camera I see?]. L'encolure étant courte, **la tête ne monte jamais haut** : pas de cou de cygne.
- **En broutant :** l'encolure descend d'environ **40 à 50° sous l'horizontale** depuis le garrot ; la face est presque verticale ; les antérieurs restent droits [P : Texel tup at Pen-y-bryn ; Texel 2009 03 074 ; Schaap50%st]. Comme le cou est court, le bélier **écarte un peu les antérieurs ou les avance** pour que le bout du nez touche l'herbe [E, général chez les moutons courts de cou].

**Pour le moteur à cou articulé :** un premier segment épais du garrot à mi-cou, un second de mi-cou à la nuque, à peu près de même longueur (0,25 H chacun) [E]. La laine du cou se plisse en bourrelets sur le dessous quand la tête monte [P : plis de laine sous la gorge, Bélier texel].

### 2.5 Les articulations des pattes

| Repère | Au-dessus du sol (fraction de H) | Étiquette |
|---|---|---|
| Coude (pointe, sous la toison) | 0,45 à 0,48 | [E] |
| Carpe (« genou » de l'antérieur), centre | **0,25 à 0,28** | [P : 0,27 du dessus de toison sur « Texel sheep », sol dur ; E] |
| Boulet antérieur | 0,08 à 0,09 | [E] |
| Grasset (sous la toison du gigot) | 0,55 à 0,6 | [E] |
| Jarret, pointe (calcanéum) | **0,30 à 0,33** | [P : environ 0,31 du dessus de toison sur « Texel sheep » ; E] |
| Boulet postérieur | 0,08 à 0,09 | [E] |
| Bas de la laine sur l'antérieur | 0,35 à 0,40 (« jusqu'au milieu de l'avant-bras ») | [S : AWE][P] |
| Bas de la laine sur le postérieur | 0,35 à 0,40 (« jusqu'au-dessus du jarret ») | [S : AWE][P] |

**Angles :**
- **Jarret :** les standards wallon et français disent « Vus de côté, les jarrets sont légèrement coudés » [AWE]. Angle du jarret, mesuré entre la jambe et le canon : **environ 145 à 155°** [E d'après P : Texel sheep, Bélier texel]. Le canon postérieur est presque vertical.
- **Paturons :** courts et assez droits, environ **55 à 60°** sur l'horizontale [E]. Le livre néerlandais parle d'un « beenwerk fijn van structuur, droog en krachtig » (pattes à ossature fine, sèches et fortes) et correctes « in gang en stand » (au pas et à l'arrêt) [NTS].
- **Antérieurs :** verticaux, d'aplomb sous l'épaule, bien écartés : la poitrine large « s'extériorise par des membres antérieurs bien écartés » [AWE].

### 2.6 La queue

- « La queue doit être implantée suffisamment haut. Elle est fine et courte » [AWE] ; « la queue attachée haut » [TX-FR][AWE] ; « De staart is fijn en heeft een passende lengte » (la queue est fine et de longueur convenable) [NTS] ; « queue assez courte et attachée haut » [WP-FR].
- Sur la photo de référence, la queue sort horizontalement de l'arrière de la croupe, sur environ 21 cm. Elle est mince : environ 4 cm de large au départ, 2 cm au bout [P]. Couverte de laine courte, elle ne se fond pas dans les gigots.
- Aucune source ne dit si elle est coupée chez ce bélier. **À dessiner :** une queue fine, attachée au ras du dos, **pendante au repos contre les fesses**, longue d'environ **0,28 H**, son bout arrivant à peu près au tiers de la hauteur des gigots [E].

---

## 3. La toison et le poil

### 3.1 Type de laine

- Pays-Bas : « De wol is fijn, gesloten en vast gestapeld en bevat geen overtollig wolvet » (laine fine, fermée, en mèches serrées, sans excès de suint) [NTS].
- Royaume-Uni : « The fleece has a high loft with a staple of medium length and is highly crinkled. Fibres are classed as medium, 34 microns and less » (toison gonflante, mèche moyenne, très frisée, 34 µm au plus) [TSS-UK]. British Wool : mèche de **7 à 14 cm**, **31 à 34,5 µm**, toison de 2,75 à 3,5 kg, « dense, medium length white wool » [BW].
- France : « toison blanche, semi-fermée, volumineuse, à mèches longues » [TX-FR, d'après l'extrait cité en 10.1]. « Une toison particulièrement épaisse, pesant en moyenne 5 kg pour les femelles et 7 kg pour les mâles » [TS].
- Wikipédia anglais : mèche de 8 à 15 cm, 32 µm en moyenne, toison de 3,5 à 5,5 kg [WP-EN].
- Étude ancienne sur le Texel français [Rougeot 1956] :
  - la toison « recouvre entièrement le corps, ne laissant dégagées que la tête et la partie inférieure des membres » ;
  - mèches très longues, « s'individualisant en petites pointes à l'extérieur » ;
  - fibres de 29,5 à 32,5 µm ;
  - beaucoup de fibres hétérotypes, **aucun jarre** ;
  - « la pluie et la neige glissent sur les extrémités pointues des mèches ».
  - La longueur exacte des mèches est mal lisible dans la numérisation (de l'ordre de 14 à 18 cm sur un an de pousse).
- Belgique : « une fine couche continue (la perte de laine n'est pas souhaitable) depuis l'arrière des oreilles jusqu'à mi-hauteur des membres » [AWE].

### 3.2 À quoi elle ressemble (et comment la dessiner)

**En fin d'été, quatre à six mois après la tonte, l'état choisi** [P : Bélier texel, Texel ram 6832386, Määäähhh, Bealgro 2021] :
- la surface est **dense et fermée**, d'une teinte crème uniforme ;
- elle est **découpée en petites plaques irrégulières de 3 à 6 cm**, séparées par de fines fentes sombres : un aspect de chou-fleur ou de croûte de pain craquelée ;
- sur les flancs, les fentes dessinent des **vagues verticales**, plus marquées sur l'épaule et la cuisse où la peau bouge ;
- le contour est **lisse et arrondi** à distance (pas de franges), finement grumeleux de près ;
- sous la gorge et devant le poitrail, la laine fait quelques **bourrelets** quand la tête est relevée.

**En pleine toison d'hiver, à près d'un an de pousse** [P : Yorkshire, England ; Texel sheep and gulls on Flat Holm ; A shig or a peep?] :
- les mèches s'allongent, se séparent et pendent en pointes : la toison devient hirsute, gonflée, plus grise en surface ;
- **sur la nuque, entre les oreilles**, une calotte de laine plus longue peut se former, comme un toupet.

**Ce qui reste ras :**
- **la tête entière**, face, front, joues et oreilles, couverte d'un « beau pelage » de **poils blancs, fins et courts** [AWE][TSS-UK][NTS : « kop en benen zijn onbewold maar bedekt met wit haar »] ;
- **les pattes**, sous le milieu de l'avant-bras et sous le jarret [AWE].

**La limite entre la laine et la face est nette** [P] :
- elle passe **juste derrière la base des oreilles** ;
- elle descend derrière la ganache jusque sous la gorge ;
- la face nue sort de la toison comme d'une **capuche**. Sur les photos de face, la tête blanche est encadrée par un bourrelet de laine en fer à cheval [P : Yorkshire, England ; A Texel ram 6832386].

**Mue :** le Texel ne mue pas ; il est tondu une fois par an [E, général pour les races à laine ; British Wool le classe parmi les laines « medium » tondues].

**Juste après la tonte**, le corps paraît plus musclé encore, les gigots et l'épaule se détachent, la peau rose transparaît [P : Texel sheep shaved, mais voir la réserve en 10.3].

---

## 4. Les couleurs, zone par zone

### 4.1 Mesures sur photos

Moyennes de petites zones (5×5 à 9×9 px) :

| Photo (lumière) | Zone | Valeur mesurée |
|---|---|---|
| Bélier texel, 1280 et 1920 px (soleil de septembre, herbe verte) | dessus du dos au soleil | `#E4D2B6`, `#E0CEB4` |
| | flanc, plaques | `#BBA37E`, `#BEAB8A` ; fentes `#A28862` ; croupe dans l'ombre `#A48A65` |
| | bas du flanc dans l'ombre propre | `#67553B` à `#3C381B` (fortement teinté de vert par l'herbe) |
| | laine du cou au soleil / creux | `#ECD3B6` / `#725433` |
| | front, face au soleil | `#EFE6D9`, `#E5D8CC`, `#E3D8CB` |
| | joue à l'ombre / sous la mâchoire | `#C8BFA3` / `#6A6847` (reflet vert) |
| | museau éclairé | `#C3B8A1` |
| | truffe | `#040300` (cœur), `#44413A` (reflet) |
| | lèvre | `#473C2E` |
| | iris | `#A78869`, `#B2916C`, `#B6986D` ; bord `#5C4A45` |
| | peau rose autour de l'œil irrité | `#E0A98A` |
| | patte postérieure au soleil / antérieure à l'ombre | `#E9DCCC`, `#D3C6B7` / `#A29376` |
| | queue | `#C7B6A4` |
| Mouton texel, Salon de l'agriculture 2022 (éclairage intérieur neutre ; Texel de type français, sexe non visible) | face / face à l'ombre | `#EDE9EC` / `#DDD7D3` |
| | truffe | `#25252A` |
| | toison au dessus / milieu / creux / cou à l'ombre | `#DBCBAF` / `#CEBAA3` / `#C0B0A1` / `#B19D84` |
| | patte | `#D1C6B8` |
| | onglon (bout usé, poussiéreux) | `#8C8178` |
| A Texel ram 6832386 (ciel couvert, Angleterre) | toison au dessus / milieu / croupe | `#D6D4CD` / `#9B998C` / `#B4B2AD` |
| | truffe | `#3E404B` |
| | onglon | `#71716B` |
| Texel Ram by Clapper Lane (soleil d'hiver, gros plan) | iris | `#A78F71` |
| | intérieur de l'oreille en contre-jour | `#80403E`, `#5F2827` |
| | truffe | `#26262B` |
| Yorkshire, England (gros plan de face, couvert) | face | `#CED1D6` |
| | truffe | `#17191C` |
| | extérieur de l'oreille, à l'ombre | `#28221F` |
| Texel sheep (concours flamand, éclairage orangé) | toison | `#CB9464`, `#8E5E36` (dominante orange de la salle : à ne pas reprendre) |
| | onglons | `#86644B`, `#735239` (même dominante) |

### 4.2 Palette pour le dessin

Valeurs ramenées à une lumière du jour neutre : retrait du vert de l'herbe et de l'orange des salles.

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Toison, été | `#DFCDB2` (crème chaude) | `#EEE2CC` sur le dessus des plaques | `#BCA683` ombre douce ; `#9C8462` fentes ; `#6B5638` fond des fentes et creux profonds ; bas du flanc dans l'ombre propre `#8F7A5C` |
| Toison, hiver (variante) | `#D2CCC0` (plus grise, pointes salies) | `#E4DFD5` | `#9B998C` ; pointes brunies `#B9A88E` |
| Collerette de laine derrière la tête | `#E2CDAE` | `#ECD3B6` | `#8A6E4C` dans les plis |
| Poil de la face, du front, des oreilles (dessus) | `#ECE4D8` | `#F4EEE6` | `#CFC4B2` joue ; `#8E8574` sous la ganache ; la peau rosée transparaît autour des yeux, du nez et des lèvres `#E6CFC2` |
| Truffe (peau nue) | `#1E1E21` | `#4A4B50` (reflet humide sur le dessus) | `#070707` narines |
| Lèvres | `#3A352E` (bord noir, gris-brun) | `#5E574C` | `#211E1A` commissure ; lèvre inférieure, face interne rosée `#8A7468` |
| Iris | `#AE9068` ambre-noisette | `#C6A87C` | anneau `#5C4A45` ; pupille `#1A1612` ; reflet `#F2EEE6` |
| Paupières, bord de l'œil | `#3B302C` | | cils clairs (blancs) |
| Intérieur de l'oreille | `#C9A9A4` (rose-gris) | contre-jour `#9A5A55` | `#8E7470` au fond |
| Petites taches noires facultatives (oreilles, paupières) | `#2A2624` | | 2 à 6 mm, voir 4.3 |
| Pattes (poil ras) | `#DDD2C3` | `#EDE4D8` | `#A89A84` |
| Callosité du genou (facultative) | `#A69886` (grisâtre) | | bord fondu |
| Onglons | `#2C2A28` | usure `#6E665E` ; boue `#7A6A58` | `#141312` |
| Peau visible (scrotum, aine) | `#D8B8A8` (rose) | | `#A88272` ; poils soyeux blancs par-dessus |

### 4.3 Marques

- **Aucune marque sur le Texel blanc.** Les Texel belges et français sont « entièrement blancs. Leur tête est couverte de poils blancs, ainsi que les membres » [AWE].
- **Tolérées :**
  - au Royaume-Uni, « the occasional black spots on ears or eyelids » (quelques taches noires sur les oreilles ou les paupières) [TSS-UK] ;
  - en Belgique et en France, « quelques petites taches noires sur les oreilles et sur les paupières sont admises » [AWE].
  - Sur une photo, de **petits points noirs** se voient sur l'oreille droite [P : Yorkshire, England].
  - Pour un dessin propre : au plus deux ou trois points de 3 à 5 mm sur le bord de l'oreille, ou rien.
- **Éliminatoires pour le Texel français :** des « taches noires dans la laine ou sur le poil, supérieures en surface à une pièce de 2 euros » ; des « poils roux sur les membres » [AWE]. La couleur « blanche » n'est accordée qu'aux animaux sans « grande tache noire sur les membres ou dans la laine (diamètre de moins de 30 cm) » [AWE].
- **Nez :** noir, mais « preferably black » (de préférence noir) [TSS-UK] et « zwarte neusspiegel … aansprekend » (une truffe noire est attrayante) [NTS]. Un nez rose ou marbré est un défaut de type, pas d'élimination. **Dessiner une truffe entièrement noire**, qui déborde un peu sur le bord des lèvres.

---

## 5. La tête

### 5.1 Ce que disent les standards

- **Pays-Bas** [NTS] : « De kop is sprekend, heeft goede verhoudingen en is voorzien van een recht neusbeen en een brede bek » (la tête est expressive, bien proportionnée, avec un **chanfrein droit** et une **bouche large**).
- **Belgique, règle commune à tous les Texel** [AWE] : « la tête, sans cornes et sans laine, est bien formée avec une bonne proportion. Le crâne doit être plat et le chanfrein **concave**. Son mufle est large et son nez noir, son œil vif et expressif. »
- **Texel français** [AWE][TX-FR] :
  - « Tête forte, large au niveau du crâne » ;
  - « Chanfrein large et court » ;
  - « la longueur, du sommet à la pointe du nez, doit être environ **une fois et demie sa largeur**, celle-ci prise de profil dans sa grande dimension ».
  - La fiche d'un éleveur, qui reprend le standard, dit « chanfrein droit ou légèrement concave » [Tireur] ; une autre source, « chanfrein plutôt large plat, le sommet du crâne plat » [TS].
- **Royaume-Uni** [TSS-UK] : « head should be covered with fine white hair, the nose preferably black ».
- **Texel belge et Texel bleu** [AWE] : bélier à « caractère mâle (présence de rides sur le nez) », brebis « plus fine ».

### 5.2 Le profil, tourné vers la droite

D'après « Bélier texel » en 1920 px, « Texel sheep », « Määäähhh », « Texel und Merino » [P] :

- **Forme générale :** un **bloc court et haut**. La longueur (nuque au bout du nez) fait environ **1,5 fois la hauteur** de la tête prise au niveau des yeux [S : TX-FR]. Le crâne est plat, le front large et plat, sans bosse.
- **Ligne du dessus :**
  - de la nuque (cachée sous la capuche de laine) au front, **presque droite** ;
  - au niveau des yeux, un **très léger creux** (le « stop ») ;
  - puis un chanfrein **droit**, court et large, jusqu'à la truffe.
  - Ni busqué, ni nettement creux : une ligne droite, avec tout au plus un soupçon de concavité devant les yeux.
- **Museau :** **large et carré, haut**, peu effilé. Vu de profil, le bout du museau est presque vertical, comme un nez de bouledogue en plus long. La truffe noire occupe le haut de ce bout.
  - **Narines :** fentes en virgule, obliques, ouvertes vers le haut et l'arrière.
  - **Lèvre supérieure** fendue (philtrum), bordée de noir.
  - **Lèvre inférieure** épaisse, un peu tombante chez le bélier, la fente de la bouche longue (« brede bek ») [P].
- **Joues et ganache :** pleines et rondes. La mâchoire inférieure fait un **bord bas, épais et arrondi**, presque parallèle au chanfrein. Chez le bélier, la peau du chanfrein porte **quelques plis transversaux** [S : AWE ; P : Bélier texel, Texel Ram by Clapper Lane].
- **Œil :**
  - **grand et saillant**, placé haut sur le côté, à environ 40 % de la longueur de la tête en partant de la nuque [P] ;
  - forme d'amande, paupière supérieure lourde ;
  - **iris ambre-noisette** clair, bien visible dans la face blanche (c'est ce qui donne au Texel son regard fixe) ;
  - **pupille horizontale**, rectangle aux coins arrondis, à garder horizontale quand la tête descend pour brouter [P : Bélier texel, Texel Ram by Clapper Lane].
  - Autour de l'œil, la peau rosée transparaît sous le poil fin.
  - Le standard veut un « œil vif et expressif » [AWE]. La microphtalmie (« petit œil ») est éliminatoire en Texel belge et bleu [AWE] : c'est une maladie héréditaire connue de la race, due à une mutation du gène *PITX3* [Becker 2010]. **Dessiner un œil franc et grand.**
- **Oreilles :**
  - *Taille :* « relativement courtes » [AWE] ; « petite taille » [TS] ; « relativement minces, poilues » [TX-FR] ; mais « épaisses » [TS-agri]. Environ **0,35 à 0,4 fois la longueur de la tête** [E], soit 9 à 11 cm.
  - *Port :* « redressées » [AWE] ; « légèrement dressées » [TX-FR][TS] ; « un peu dressées » [Tireur][TS-agri]. Sur les photos [P] : **à l'horizontale ou relevées de 10 à 30°**, pointées vers l'extérieur et un peu vers l'arrière, jamais tombantes. Elles bougent beaucoup.
  - *Forme :* une feuille de saule ou une lancette, étroite à la base, la pointe assez fine ; le bord épais [P].
  - *Couleur :* poil blanc dehors ; dedans, peau rose-gris à poils clairsemés, qui rougeoie en contre-jour.
  - *De profil, tête tournée vers la droite :* l'oreille gauche (proche) part **derrière et un peu au-dessus de l'œil, juste devant la limite de la laine**. Elle se dirige vers l'arrière et l'extérieur, vers nous, donc raccourcie. L'oreille droite (lointaine) dépasse au-dessus du front, pointe en l'air et en arrière [P : Bélier texel].
- **Toupet :** aucun en toison courte. La laine s'arrête derrière les oreilles. En toison longue d'hiver, une calotte laineuse peut couvrir la nuque entre les oreilles [P : Yorkshire, England].

---

## 6. Les cornes

### 6.1 Aucune

- Le Texel est **sans cornes dans les deux sexes** : « la tête, sans cornes et sans laine » [AWE] ; « It is polled » [WP-EN] ; « sans cornes » [TS][Tireur]. Le standard britannique ne mentionne aucune corne [TSS-UK].
- **Dessiner :** un crâne plat et nu entre les oreilles, sans bourgeon ni bosse.

### 6.2 Une réserve : les petites « scurs »

- Deux béliers photographiés portent sur la nuque, entre les oreilles, **une petite excroissance cornée brune, courbe, de 3 à 5 cm**, non attachée solidement au crâne : c'est un *scur*, un rudiment de corne.
  - Couleur mesurée `#C18E6A` sur « Bélier texel » (1920 px) ; `#745955` à l'ombre sur « Texel Ram by Clapper Lane » [P].
- Aucune source sur la race n'en parle. C'est une particularité individuelle, connue chez les races sans cornes, et non un trait de race.
- **Ne pas la dessiner.** Un Texel dessiné avec un bout de corne serait lu comme un croisé ou une autre race.

---

## 7. Les pattes et les onglons

- **Ossature :**
  - France : « Il est fortement charpenté avec des attaches puissantes et courtes » ; membres « forts » [TX-FR][Tireur].
  - Belgique, règle générale : « L'ossature doit être légère tout en restant proportionnelle au développement de l'animal » [AWE].
  - Pays-Bas : « beenwerk fijn van structuur, droog en krachtig » [NTS].
  - Royaume-Uni : « medium bone structure » [TSS-UK].
  - **Pour le dessin :** des canons **fins et secs** sous un corps énorme. C'est le contraste typique : la masse est dans les muscles (avant-bras, gigots), pas dans l'os [P].
- **Laine :** les antérieurs portent de la laine « jusqu'au milieu de l'avant-bras », les postérieurs « jusqu'au-dessus du jarret » ; le reste est « couvert de poils fins » [AWE]. Le passage entre la laine et le poil est net, en biseau [P].
- **Muscles visibles :**
  - **avant-bras épais et galbé**, plus large que le canon de moitié ;
  - **gigot qui descend en boule** jusque près du jarret, avec un pli marqué entre la cuisse et le ventre [P : Bélier texel, Texel tup at Pen-y-bryn].
  - Les postérieurs sont « bien écartés » [AWE] ; vus de profil, ils sont un peu en arrière de la verticale de la fesse.
- **Couleur :** poil blanc ; chez le Texel français, des « poils roux sur les membres » sont éliminatoires [AWE]. Sur le terrain, le bas des pattes est souvent sali (vert-gris, boue) [P].
- **Onglons :** « Les onglons sont noirs » [AWE] ; « onglons noirs » [TX-FR][Tireur][TS]. Sur les photos, ils sont noirs ou gris ardoise foncé, souvent poussiéreux au bout [P]. Petits par rapport au corps : environ 6 à 7 cm de long pour un bélier [E]. Deux doigts serrés, la pince pointue ; ergots discrets à l'arrière du paturon.
- **Aplombs :** « Les aplombs du Texel doivent être corrects » [AWE] ; « aplombs très réguliers » [Tireur] ; « solid square stance » [TSS-UK]. Le bélier se tient **carré, les quatre pattes aux coins de la boîte**.

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Calme et docile :**
  - « The Texel is hardy, tough and docile » (rustique, dur et docile) [TSS-UK] ;
  - « De Texelaars zijn alom bekend om hun zeer goede karakter » (les Texel sont partout connus pour leur très bon caractère) [NTS] ;
  - une « race très douce facile à mener » [Réussir-2023].
- **Peu grégaire.** C'est le trait de comportement le plus propre à la race, et il est noté en France :
  - « Contrairement à la plupart des moutons, le texel est peu grégaire. Il aime bien pâturer seul, et est souvent élevé au pâturage avec d'autres animaux, notamment des bovins » [WP-FR] ;
  - son « faible instinct grégaire » est cité comme un atout pour le pâturage avec les bovins [SVS] ;
  - « une race … qui n'a pas l'instinct grégaire » [Réussir-2023].
  - Aucune étude contrôlée ne le mesure (je n'en ai pas trouvé) ; c'est un savoir d'éleveurs constant.
  - **Pour l'animation :** un Texel **s'écarte plus volontiers du groupe**, broute à plusieurs mètres des autres, suit moins vite quand le troupeau part, et rejoint sans hâte.
- **Rustique, au grand air :** « une race d'herbage, souvent conduite en plein air intégral » [Réussir-2023]. La toison « protège efficacement l'animal contre les intempéries et permet l'élevage des Texel en plein air, même par les grands froids et la neige » [Rougeot 1956]. Photos en hiver sur l'île, sous la neige [P : Westerweg, Winter of 2010].
- **Agneaux vigoureux :** « Lambs are famously vigorous at birth with a great will to survive » [TSS-UK].
- **Reproduction :** les brebis sont en chaleur pendant près de cinq mois de l'année, la première fois vers 7 mois [NTS]. En automne, les béliers portent un **harnais marqueur** à crayon de couleur sur le poitrail, qui laisse une trace de couleur sur la croupe des brebis saillies [P : A Texel ram in a harness ; marques de couleur sur la croupe dans Texel tup 5963597 et Mouton texel].

### 8.2 Ce qui vaut pour les moutons en général (non propre à la race)

- **Pas :** à quatre temps, en séquence latérale. La tête hoche un peu à chaque foulée d'antérieur. Le Texel, lourd et court de pattes, a un **pas court et posé** ; le corps bouge peu, les gigots roulent [E].
- **Trot et galop :** rares chez un bélier lourd. Il trotte quelques mètres pour rejoindre, ne galope que s'il est effrayé, et s'arrête vite [E].
- **Mouvement secondaire :** la toison courte et dense **ne flotte pas**. Ce sont les masses (gigots, ventre, joues) qui tressautent un peu à chaque pas. Les oreilles bougent séparément [E].
- **Poses de repos :**
  - debout, carré, tête au niveau du dos ;
  - broutant, encolure basse (2.4) ;
  - couché sur le sternum, antérieurs repliés ou **un antérieur allongé devant** [P : Mouton texel], tête posée ou haute, ruminant ;
  - à plat sur le flanc par grosse chaleur [P : Texel tups enjoying the sunshine].
- **Voix :** aucune source ne décrit une voix propre au Texel. Bêlement ordinaire, grave chez le bélier.
- **Combats de béliers :** comme tous les béliers sans cornes, ils reculent puis se heurtent front contre front. Un Texel lourd charge sur une courte distance [E].
- **Vigilance :** la tête se lève, les oreilles pointent vers le bruit, le regard ambré fixe l'intrus [P : Is that a camera I see?, où tout le troupeau regarde l'objectif]. Mais l'alerte se propage moins, puisque les animaux sont plus dispersés [E, d'après 8.1].

---

## 9. Les dix détails qui font un Texel, et les erreurs à éviter

### 9.1 Les dix détails

1. **Tête blanche, nue, large et courte** : poil ras, aucune laine sur la face ni sur le front ; longueur ≈ 1,5 fois la hauteur de profil.
2. **Nez noir** : une truffe noire au bout d'un museau large et carré, des lèvres bordées de noir.
3. **Chanfrein droit**, front plat, crâne plat entre les oreilles. Ni busqué (Suffolk, Bleu du Maine), ni bombé.
4. **Petites oreilles épaisses, à l'horizontale ou un peu relevées**, poil blanc dehors, rose dedans. Jamais longues ni tombantes.
5. **Gros œil ambré** bien visible dans la face blanche.
6. **Sans cornes**, dans les deux sexes.
7. **Encolure courte et épaisse** : la tête semble sortir directement de la toison, qui commence en capuche **juste derrière les oreilles**.
8. **Corps en poutre** : long, rectangulaire, dos large et plat, poitrine profonde, croupe carrée ; « près de terre ».
9. **Gigots ronds et descendus**, qui forment derrière une grosse boule plus basse que le ventre ; avant-bras galbés. C'est l'effet du gène de la myostatine [Clop 2006] et d'un autre gène de musculature lombaire [Macfarlane 2009].
10. **Pattes blanches à poil fin, laine jusqu'à mi-avant-bras et au-dessus du jarret, onglons noirs** ; toison crème dense et courte, craquelée en petites plaques ; queue fine et courte attachée haut.

### 9.2 Les erreurs qui en font une autre race

- **Suffolk :** tête et pattes **noires**, sans laine ; **longues oreilles noires** « bell-shaped », souvent tombantes ; nez **légèrement busqué** [Suffolk-sources]. Un Texel avec une tête sombre, des oreilles longues ou un nez busqué devient un Suffolk. Les deux sont grands (Suffolk : 80 cm et 125 kg pour un bélier [WP-Suffolk]) et sans cornes, à toison blanche : **seule la tête les sépare**.
- **Bleu du Maine :**
  - tête **bleu foncé** (gris-bleu à noir) ;
  - « des oreilles **longues et fines** de même couleur que la tête » ;
  - « un chanfrein **légèrement busqué** » ;
  - extrémité des pattes gris-bleu à noir ;
  - toison blanche à longues mèches qui ne couvre pas la tête, les genoux et les jarrets [WP-BdM].
  - Un Texel au nez noir mais à la face grise devient un Bleu du Maine. La face du Texel doit rester **franchement blanche**.
- **Texel bleu (Blauwe Texelaar) :** même corps, mais toison et tête gris-bleu à presque noir, avec un « licol » blanc, des points blancs sous les yeux et un bord blanc aux oreilles [AWE][Het Schaap]. Pas de gris dans un Texel blanc.
- **Texel Dassenkop :** corps foncé, ventre clair, tête sombre à deux bandes claires le long des yeux, menton blanc [AWE]. Le pelage foncé de la photo « A Texel (sheep) » sur Commons est d'un autre type : à ne pas suivre (10.3).
- **Beltex et Texel belge extrême :** une musculature plus outrancière encore, une croupe plus ronde que haute, des fesses en double boule [AWE : « les classes Texel Belge visent une conformation musculaire plus importante »]. Garder le Texel **massif mais pas caricatural** : un dos plat, un corps plus long que haut.
- **Moutons blancs à nez rose et longues oreilles** (types Mérinos, Île-de-France…) : la photo « Texel und Merino mit Lamm » met les deux côte à côte. À gauche, le Texel : nez noir, oreilles courtes, face large. À droite, l'autre : nez rose, longues oreilles horizontales, face longue et étroite [P].
- **À ne pas dessiner :**
  - de la laine sur le front ou les joues (en toison courte) ;
  - des cornes ou un bout de corne ;
  - un nez rose ;
  - des oreilles longues ou pendantes ;
  - un cou long et dressé ;
  - des pattes laineuses jusqu'aux sabots ;
  - des onglons clairs ;
  - des taches noires sur le corps ;
  - des poils roux aux pattes ;
  - une longue queue pendante jusqu'aux jarrets.

---

## 10. Sources, photos, contradictions

### 10.1 Sources (ouvertes ou lues pour cette fiche)

**Livres généalogiques et organismes de race :**
- [NTS] Nederlands Texelaar Schapenstamboek et Texelaar stamboek in Noord-Holland, *Texelaar* et *Rasbeschrijving* : https://www.texelsheep.nl/texelaar/ et https://www.texelsheep.nl/texelaar/rasbeschrijving/
- [NTS-mat] Idem, tableaux des mensurations pour l'admission (« matentabel rammen def. en herhaald 2023-09-18 », « matentabel ooien def. en herhaald 2023-09-18 », « ramlammeren ») : https://www.texelsheep.nl/matentabellen-voor-opname/ ; PDF : https://www.texelsheep.nl/wp-content/uploads/2023/12/matentabel-rammen-def.-en-herhaald-2023-09-18.pdf et https://www.texelsheep.nl/wp-content/uploads/2023/12/matentabel-ooien-def.-en-herhaald-2023-09-18.pdf
- [NTS-fok] Idem, *Fokdoel* : https://www.texelsheep.nl/fokdoel/ ; *Historie* : https://www.texelsheep.nl/historie/
- [TX-NL] Fokkers op Texel, *Het Texelse schaap* (historique, description reprise du livre) : http://www.texels-schaap.nl/ras.html
- [TSS-UK] The British Texel Sheep Society, *Texel Breed Description* (type, tête, toison, moyennes 2014-2024 dont le scanner) : https://texel.uk/texel-breed-description/
- [AWE] Elevéo / Association wallonne de l'élevage, programme de sélection Texel, chapitre 9 « Caractéristiques détaillées de la race » : règle commune, puis Texel français, Texel belge, Texel bleu, Texel Dassenkop ; critères éliminatoires. https://www.awenet.be/awe/commun/asbl/communication/pds.php?espece=ovin&race=Texel
- [TX-FR] *Standard de la race ovine Texel Français*, 2022, PDF hébergé par l'AWEOC : http://www.aweoc.be/aweoc.site/wp-content/uploads/2023/06/Texel-Francais-Standard-race-2022.pdf. **Le serveur m'a refusé l'accès (403).** Je ne cite que les extraits renvoyés par le moteur de recherche (tête, chanfrein, oreilles, encolure, onglons, toison, hauteurs 75-82 / 68-72 cm, poids 115-130 / 75-80 kg), qui concordent avec [AWE] et [Tireur].
- [SVS] AgroParisTech et Races de France, *Race ovine Texel* (OS Texel Génétique France, poids, effectifs, « faible instinct grégaire ») : https://svs.agroparistech.fr/especes/ovins/texel.htm
- [BW] British Wool, fiche *Texel* (mèche, micron, toison) : https://shop.britishwool.org.uk/wp-content/uploads/2025/03/Texel-Medium.pdf

**Études :**
- [Unipar] Cunha Filho L. F. C. et al. 2010, « Predição do peso corporal a partir de mensurações corporais em ovinos Texel », *Arq. Ciênc. Vet. Zool. UNIPAR* 13(1) : 5-7 (25 béliers Texel de 14 mois, salon de Londrina, avril 2008) : https://revistas.unipar.br/index.php/veterinaria/article/view/3369
- [Rougeot 1956] Rougeot J., « Les fibres médullées dans la toison de la race ovine de Texel », *Annales de zootechnie* 5(1) : https://animres.edpsciences.org/articles/animres/pdf/1956/01/Ann.Zootech._0003-424X_1956_5_1_ART0002.pdf
- [Clop 2006] Clop A. et al. 2006, « A mutation creating a potential illegitimate microRNA target site in the myostatin gene affects muscularity in sheep », *Nature Genetics* 38 : 813-818 (recherche PubMed), https://doi.org/10.1038/ng1810
- [Macfarlane 2009] Macfarlane J. M. et al. 2009, « Effects of the Texel muscling quantitative trait locus on carcass traits in crossbred lambs », *Animal* 3(2) : 189-199 (scanner, muscle lombaire ; recherche PubMed), https://doi.org/10.1017/S175173110800356X
- [Becker 2010] Becker D. et al. 2010, « Microphthalmia in Texel sheep is associated with a missense mutation in the paired-like homeodomain 3 (PITX3) gene », *PLoS One* 5(1) : e8689 (recherche PubMed), https://doi.org/10.1371/journal.pone.0008689

**Encyclopédies et presse :**
- [WP-EN] https://en.wikipedia.org/wiki/Texel_sheep (d'après FAO DAD-IS et *Mason's World Encyclopedia*)
- [WP-FR] https://fr.wikipedia.org/wiki/Texel_(race_ovine)
- [TS] Techno-Science, *Texel (race ovine)* (reprise de Wikipédia) : https://www.techno-science.net/glossaire-definition/Texel-race-ovine.html
- [TS-agri] Toutagri, *Le Texel* : https://www.toutagri.fr/article-le-texel/
- [Tireur] R. Tireur, *La passion du mouton texel*, page « La race » (blog d'éleveur reprenant le standard français) : http://tireurtexel.over-blog.com/pages/La_race-5285234.html
- [Réussir-2023] Réussir, visite d'un élevage Texel au Sommet de l'élevage 2023 : https://www.reussir.fr/mouton-texel-visite-virtuelle-dun-elevage-loccasion-du-sommet-de-lelevage-2023
- [Het Schaap] « NTS en TsNH stellen eisen Blauwe en Dassenkop Texelaars bij » : https://www.hetschaap.nl/nts-en-tsnh-stellen-eisen-blauwe-en-dassenkop-texelaars-bij/

**Races voisines :**
- [WP-BdM] https://fr.wikipedia.org/wiki/Bleu_du_Maine
- [WP-Suffolk] https://en.wikipedia.org/wiki/Suffolk_sheep
- [Suffolk-sources] Suffolk Sheep Society, *Breed information* : https://suffolks.org/breed-information.html ; guide de classement du Kentucky : https://www.kyagr.com/marketing/documents/SF_Sheep_Lamb_Classification_Guidelines.pdf

### 10.2 Photos regardées

Toutes sur Wikimedia Commons (catégorie https://commons.wikimedia.org/wiki/Category:Texel_(sheep)), téléchargées hors du dépôt.

**Profils et sujet de référence :**
- **Bélier texel** (référence) : bélier de trois ans né sur l'île de Texel, 7 septembre 2015, profil strict vers la gauche, toison de fin d'été craquelée, tête nue, œil ambré, nez noir, scur sur la nuque. Proportions, encolure et couleurs mesurées. https://commons.wikimedia.org/wiki/File:B%C3%A9lier_texel.jpg
- **Texel sheep** : animal de concours flamand, profil vers la droite sur sol dur, ruban belge ; hauteur du genou et du jarret, gigots, onglons. https://commons.wikimedia.org/wiki/File:Texel_sheep.jpg
- **Texel ram near Friths Farm** : bélier britannique de profil vers la droite au pré ; port de tête au repos, encolure courte. https://commons.wikimedia.org/wiki/File:Texel_ram_near_Friths_Farm_-_geograph.org.uk_-_2031009.jpg
- **Texel tup at Pen-y-bryn** : bélier gallois broutant, de profil ; encolure basse, gigot en boule, scrotum. https://commons.wikimedia.org/wiki/File:Texel_tup_at_Pen-y-bryn_-_geograph.org.uk_-_1572015.jpg
- **Määäähhh** : deux jeunes Texel, l'un de profil vers la droite ; tête courte, oreilles, toison serrée. https://commons.wikimedia.org/wiki/File:M%C3%A4%C3%A4%C3%A4%C3%A4hhh_(166066477).jpeg

**Têtes de près :**
- **Bélier Texel** (deux ans, de trois quarts) : face large, plis du nez, nez noir. https://commons.wikimedia.org/wiki/File:B%C3%A9lier_Texel.JPG
- **Texel Ram by Clapper Lane** : gros plan, iris ambre, pupille horizontale, scur, oreille rose en contre-jour. https://commons.wikimedia.org/wiki/File:Texel_Ram_by_Clapper_Lane_-_geograph.org.uk_-_4358250.jpg
- **Yorkshire, England** : gros plan de face en pleine toison ; capuche de laine, calotte sur la nuque, petites taches noires sur l'oreille. https://commons.wikimedia.org/wiki/File:Yorkshire,_England_(29780748142).jpg
- **Texel und Merino mit Lamm** : comparaison directe avec un mouton à nez rose et longues oreilles. https://commons.wikimedia.org/wiki/File:Texel_und_Merino_mit_Lamm.JPG

**Brebis, agneaux, troupeaux :**
- **Mouton texel** : Texel de type français couché (sexe non visible), Salon de l'agriculture 2022 ; couleurs neutres, antérieur allongé. https://commons.wikimedia.org/wiki/File:Mouton_texel.jpg
- **20160722 113 libramont** : enclos « Texel Français » à la foire de Libramont, tête et oreilles du type français. https://commons.wikimedia.org/wiki/File:20160722_113_libramont.jpg
- **A Texel ram 6832386** : bélier anglais de trois quarts, ciel couvert ; couleurs neutres. https://commons.wikimedia.org/wiki/File:A_Texel_ram_-_geograph.org.uk_-_6832386.jpg
- **Texel ram 2703220** : bélier de face sous un arbre, brouillard. https://commons.wikimedia.org/wiki/File:Texel_ram_-_geograph.org.uk_-_2703220.jpg
- **Texel tup 5963597** : bélier gallois de trois quarts, marque de couleur sur l'épaule. https://commons.wikimedia.org/wiki/File:Texel_tup_-_geograph.org.uk_-_5963597.jpg
- **A Texel ram in a harness** : béliers au harnais marqueur, brebis broutant de profil. https://commons.wikimedia.org/wiki/File:A_Texel_ram_in_a_harness_-_geograph.org.uk_-_7655024.jpg
- **Texel tups enjoying the sunshine** : béliers couchés au soleil, dont un à plat. https://commons.wikimedia.org/wiki/File:Texel_tups_enjoying_the_sunshine_-_geograph.org.uk_-_6858502.jpg
- **Texel rams in field east of Nethy Bridge** : béliers en Écosse, l'un de profil. https://commons.wikimedia.org/wiki/File:Texel_rams_in_field_east_of_Nethy_Bridge_-_geograph.org.uk_-_7315155.jpg
- **Is that a camera I see?** : troupeau britannique qui regarde l'objectif ; vigilance. https://commons.wikimedia.org/wiki/File:Is_that_a_camera_I_see%5E_-_geograph.org.uk_-_1092339.jpg
- **A shig or a peep?** : Texel en toison longue et sale, de face. https://commons.wikimedia.org/wiki/File:A_shig_or_a_peep%5E_-_geograph.org.uk_-_1566879.jpg
- **Texel ewe and twin lambs** : brebis et agneaux de face ; oreilles roses des agneaux. https://commons.wikimedia.org/wiki/File:Texel_ewe_and_twin_lambs_-_geograph.org.uk_-_767717.jpg
- **A well-developed Texel lamb** : agneau de face ; nez noir, callosités grises aux genoux. https://commons.wikimedia.org/wiki/File:A_well-developed_Texel_lamb_-_geograph.org.uk_-_767722.jpg
- **Texel avec ses agneaux** (identique à **Texel ewe and three lambs**) : brebis et trois agneaux. https://commons.wikimedia.org/wiki/File:Texel_avec_ses_agneaux.jpg
- **Texel 2009 03 074** : brebis et agneau broutant côte à côte ; port de tête au pâturage. https://commons.wikimedia.org/wiki/File:Texel_2009_03_074.jpg
- **Schaap50%st** : jeune Texelaar broutant, callosité grise au genou. https://commons.wikimedia.org/wiki/File:Schaap50%25st.jpg
- **Texelschaf**, **Texelschafe**, **Texelschafe 2003** : brebis et agneaux sur l'île et sur les dunes ; ports de tête en alerte. https://commons.wikimedia.org/wiki/File:Texelschaf.JPG · https://commons.wikimedia.org/wiki/File:Texelschafe.jpg · https://commons.wikimedia.org/wiki/File:Texelschafe_2003.jpg
- **TexelSheep** : silhouettes broutant sur une digue. https://commons.wikimedia.org/wiki/File:TexelSheep.jpg
- **Texel – Westerweg – Winter of 2010 – Texel Sheep** : troupeau dispersé sous la neige sur l'île. https://commons.wikimedia.org/wiki/File:Texel_-_Westerweg_-_Winter_of_2010_-_Texel_Sheep.jpg
- **Texel sheep and gulls on Flat Holm** : Texel en longue toison d'hiver, de profil, l'air hirsute. https://commons.wikimedia.org/wiki/File:Texel_sheep_and_gulls_on_Flat_Holm.jpg
- **Texel sheeps (Bealgro-2021) 1 à 3** : jeunes couchés et vus de dos (queues fines, pattes). https://commons.wikimedia.org/wiki/File:Texel_sheeps_(Bealgro-2021)_1.jpg (et _2, _3)
- **Netherlands Open Air Museum, Arnhem – Texel Sheep** : petit groupe broutant au musée de plein air. https://commons.wikimedia.org/wiki/File:Netherlands_Open_Air_Museum,_Arnhem_-_2020-10-31_-_Texel_Sheep_%26_Kasteelboerderij,_Oud-Beijerland.jpg

**Regardées mais écartées :**
- **A Texel (sheep)** : un mouton brun à tête noire, à toison bouclée. Il est classé dans la catégorie mais ne correspond ni au Texel blanc ni au Dassenkop décrit par [AWE]. https://commons.wikimedia.org/wiki/File:A_Texel_(sheep).JPG
- **Gruppe Schafe** : groupe mêlé, nez roses et longues oreilles : pas des Texel purs. https://commons.wikimedia.org/wiki/File:Gruppe_Schafe.JPG
- **Texel sheep shaved** : mouton tondu à tête et pattes fauves. Les poils roux sont éliminatoires en Texel français ; probablement un croisé. Retenu seulement pour l'idée de la silhouette tondue. https://commons.wikimedia.org/wiki/File:Texel_sheep_shaved.jpg
- Photos d'agnelage à De Waal (2011) : trop sombres pour mesurer.

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Désaccord |
|---|---|
| **Profil du chanfrein** | « recht neusbeen » (droit) [NTS] ; « chanfrein concave », règle wallonne commune à tous les Texel [AWE] ; « droit ou légèrement concave » [Tireur] ; « plutôt large plat » [TS]. Sur les photos, il est droit avec un léger creux devant les yeux. **Choix :** droit, soupçon de creux, jamais busqué. |
| **Hauteur au garrot** | Bélier : 70 cm [NTS][WP-EN] ; 71 cm mesurés à 14 mois [Unipar] ; 75 à 82 cm [TX-FR] ; « environ 80 » [WP-FR][TS] ; 85 à 87 cm sur un site polonais peu sûr (écarté). Brebis : 66 cm (tableau [NTS-mat]) ; 68 cm [NTS] ; 68 à 72 cm [TX-FR] ; « environ 70 » [WP-FR]. Le Texel français est plus grand que le néerlandais. **Choix :** 72 cm pour un bélier néerlandais. |
| **Poids** | Bélier : 95 kg [NTS] ; 115 à 130 kg [TX-FR] ; 120 à 140 kg [SVS] ; environ 120 kg en Grande-Bretagne [WP-EN]. Brebis : 75 kg [NTS] ; 75 à 80 [TX-FR] ; 75 à 90 [SVS] ; 79,8 kg à 18 mois [TSS-UK] ; environ 85 kg en Grande-Bretagne [WP-EN]. |
| **Longueur de l'encolure** | « courte » [AWE pour le Texel français][BW][Tireur] ; « voldoende lang » (assez longue) [NTS] ; « horizontale dans le prolongement du corps » [AWE pour le Texel belge]. |
| **Ossature** | « légère » [AWE] et « fijn » [NTS] ; mais « fortement charpenté avec des attaches puissantes » [TX-FR] et « medium » [TSS-UK]. |
| **Port des oreilles** | « redressées » [AWE] ; « légèrement dressées » [TX-FR][TS] ; « un peu dressées » [Tireur] ; « nearly horizontal » selon des sites secondaires anglais. Les photos montrent tout cela, de l'horizontale à 30° au-dessus. |
| **Épaisseur des oreilles** | « relativement minces » [TX-FR] contre « épaisses » [TS-agri]. |
| **Laine : longueur et finesse** | Mèche 7 à 14 cm [BW] ; 8 à 15 cm [WP-EN] ; « mèches longues » [TX-FR] ; très longues en 1956 [Rougeot]. Finesse ≤ 34 µm [TSS-UK] ; 31 à 34,5 µm [BW] ; 32 µm [WP-EN] ; 29,5 à 32,5 µm [Rougeot]. Toison 2,75 à 3,5 kg [BW] ; 3,5 à 5,5 kg [WP-EN] ; 5 kg (brebis) et 7 kg (bélier) [TS] ; 4 à 6 kg [Rougeot]. |
| **Nature de la toison** | « gesloten » (fermée) [NTS] contre « semi-fermée, volumineuse » [TX-FR] et « high loft, highly crinkled » [TSS-UK]. |
| **Couleur du nez** | « noir » sans réserve [AWE][TS] contre « preferably black » [TSS-UK] et « aansprekend » [NTS]. |
| **Queue** | « fine et courte » [AWE] contre « passende lengte » (longueur convenable) [NTS]. Aucune source ne dit si elle est coupée. |
| **Comportement grégaire** | « peu grégaire, aime pâturer seul » [WP-FR][SVS][Réussir-2023] contre des sites d'éleveurs américains qui le disent sociable. Aucune étude contrôlée trouvée. |
| **Dos** | « droit, horizontal » [NTS][TSS-UK : « level back »] contre « horizontal ou légèrement bombé » pour le type belge [AWE]. |
| **Origine des croisements** | Leicester, Lincoln et Wensleydale « in the late 19th century » [TSS-UK] ; Leicester puis Lincoln vers 1860 [TX-NL] ; Leicester, Lincoln et Kent au XVIII<sup>e</sup> siècle [WP-FR] ; « early 1800s » [WP-EN]. |
| **Arrivée en Grande-Bretagne** | « First importations came from France in the early seventies, followed by direct imports from Holland at the end of that decade » [TSS-UK] ; importation de stock français par l'ABRO en 1970, livre ouvert en 1972 [WP-EN] ; société fondée en 1974 [BW]. |
| **Longueur du corps** | Les rapports publiés (114 à 116 % [NTS-mat], 123 % [Unipar]) ne disent pas exactement entre quels points ils mesurent : [Unipar] mesure de la pointe de l'épaule à la pointe de la fesse ; [NTS-mat] ne le précise pas. |
| **Mesures de la tête, des oreilles, de la queue, des articulations** | **Aucune source.** Toutes les valeurs de 2.3 à 2.6 qui ne portent pas [S] sont des mesures sur photo [P] ou des estimations [E]. |
