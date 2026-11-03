> Fiche de la race « Solognote », préparée le 3 novembre 2026 pour
> dessiner `pets/solognote` dans Mouflon. Le dessin viendra ensuite
> (`docs/chantiers/races-dessins.md`).

# Solognote : fiche pour un dessin de profil, à l'aquarelle

**Noms :** Solognote, mouton solognot, « le Solognot » (FR ; GEODE écrit
indifféremment « race Solognote » et « race SOLOGNOT », S2, S3) ·
Solognote sheep (EN). Pas d'article Wikipédia en anglais ni en allemand ;
seulement en français et en vietnamien (S6).

**Elle remplace Aquarelle.** Aquarelle était la brebis du dépôt peinte à
l'aquarelle, sur un squelette (`pets/aquarelle`, `tools/mouflon-peintre`).
La Solognote sera peinte de la même façon. Le § 4.5 donne donc, en plus
des valeurs hexadécimales, une palette pensée pour le lavis : teintes de
fond, glacis, réserves de blanc, bords fondus et bords nets, et leur
traduction dans les champs du `Wash` du peintre.

**Comment lire les balises :**
- **[STD]** : le standard de la race, publié par l'organisme de sélection
  GEODE (S1), et ses reprises (S3, S5).
- **[SRC]** : une autre source écrite (article scientifique, document de
  conservatoire, encyclopédie).
- **[PHOTO]** : vu sur une des photos du § 10.2 (P1, P2…).
- **[EST]** : mon estimation, mesurée en pixels sur les photos. Aucune
  source ne donne ce chiffre.
- Les numéros S1, S2… renvoient aux sources du § 10.1.

---

## 0. En bref

| | |
|---|---|
| **Individu** | Une **brebis adulte** de 4 à 6 ans, en toison de fin d'été (environ quatre mois après la tonte de printemps, mèches de 4 à 6 cm). Debout, de profil, tournée vers la droite, tête mi-haute, curieuse. |
| **Vue** | Profil strict, côté gauche visible. Les quatre pattes visibles, les deux de devant presque l'une derrière l'autre. |
| **Toison** | **Bise** : un gris-beige chaud, assez court, en mèches tassées. Elle part de derrière les oreilles, laisse **nus la gorge, le dessous du cou et le ventre**, et s'arrête au coude et au grasset. Sur l'encolure et l'épaule, la laine garde souvent une teinte **rousse** qui se fond dans le gris-beige du corps. |
| **Signature** | **Tête, oreilles et pattes nues, d'un roux-brun foncé uniforme** (« châtain roux », S1), sur un corps gris-beige. Pas de cornes, pas de toupet. Tête longue et étroite, profil presque droit. Oreilles moyennes à l'horizontale. **Œil très clair** (gris bleuté à paille) cerclé de brun sombre. Une **bande de peau châtain nue sous le ventre**, visible de profil. Longues pattes fines. Longue queue laineuse qui descend sous le jarret. |
| **Palette principale** | voir le tableau ci-dessous, le § 4 et la palette d'aquarelle du § 4.5 |

**Palette principale** (valeurs tirées des photos, puis ramenées à une
lumière de jour couverte ; le détail, les variantes et la palette
d'aquarelle sont au § 4) :

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Tête et oreilles (poil ras) | `#7A3E2C` | `#A8644A` (soleil `#C98F6E`) | `#4A2218` |
| Pattes (poil ras) | `#5E3A2A` | `#87604C` | `#33200F` |
| Toison bise du corps | `#CFC4B6` | `#EDE7DD` | `#958571` (creux `#6E604F`) |
| Laine rousse de l'encolure | `#7A5233` | `#A88A6C` | `#4A2C14` |
| Peau nue du ventre | `#6A4630` | `#87644A` | `#3E2915` |
| Truffe, lèvres | `#4A3A35` | `#7A6A68` (humide) | `#2A211E` |
| Onglons | `#3E3A36` | `#6D6964` | `#22201E` |
| Iris | `#B8B9B0` (gris pâle) | `#D8D9D0` | cercle `#7D7A6E` ; pupille `#141110` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 La race en quelques lignes

- Race rustique et robuste, « productrice de viande et de laine » [STD]
  (S1). Berceau : la **Sologne**, au sud de la Loire (Loir-et-Cher, Loiret,
  Cher), pays de landes, de bois, d'étangs et de sols « pauvres et
  humides » (S4, S5).
- Très ancienne. Elle s'est développée avec le commerce de la laine à la
  Renaissance, quand la Cour était dans le Val de Loire. Les draperies de
  Romorantin, fournisseurs de l'armée, en achetaient la laine (S4, S5).
- Elle vient d'une population régionale **bigarrée** (« marron et blanc »,
  S3). Dès 1800, les éleveurs préfèrent les moutons **roux** : la couleur
  garantit qu'aucun bélier mérinos ou anglais n'a été introduit (S3, S4,
  S5). Sur le tableau d'Esbrat (1852, musée d'Orléans), les moutons sont
  déjà « uniformément roux » (S5, P24).
- Apogée vers 1850 : environ **300 000 têtes**, jusque dans le Val de
  Loire, la Beauce et le Gâtinais (S4, S5). Puis le marnage, les routes,
  le drainage, le reboisement et la chasse la font presque disparaître
  (50 000 têtes en 1910) (S5, S6).
- Syndicat ovin solognot en 1934, livre généalogique (« flock-book ») en
  1942, fusion en 1948, **réserve génétique en 1968**, plan d'accouplement
  par familles depuis 1976, centre d'élevage de jeunes béliers depuis 1979
  (S4, S5). Organisme de sélection : GEODE depuis 2007-2008 (S2, S4).
- Aujourd'hui environ **3 000 brebis** et une cinquantaine d'éleveurs
  (S2, S4). Elle est très demandée pour l'**éco-pâturage** : sous-bois,
  landes, marais, bords de Loire et du Cher, réserves naturelles (S2, S3,
  S4).

### 1.2 Le choix : une brebis adulte

1. **Les deux sexes sont sans cornes.** Leur présence est un défaut
   éliminatoire (S1). Le bélier n'apporte donc aucune marque de plus : il
   est plus lourd, a l'encolure plus épaisse et le chanfrein un peu plus
   bombé (§ 1.4).
2. **Les brebis font le troupeau.** Les photos d'éco-pâturage, de salon
   et de neige montrent presque toujours des brebis (P3, P8, P14, P17,
   P18, P22).
3. **Aquarelle était une brebis.** On garde le même rôle dans le dépôt.
4. **Une brebis montre mieux la bande de ventre nue.** Chez le bélier, le
   fourreau et le scrotum la cachent en partie.

### 1.3 L'individu exact

- Brebis de **4 à 6 ans** : la tête est faite, l'œil a pâli, la toison a
  « grisé » (§ 3.3). La race vit vieux : des brebis de 10 ans agnellent
  encore, et on cite des cas à 13 ans (S4, S5).
- Hauteur au garrot **65 cm**, poids **60 kg** (milieu des fourchettes du
  § 2.1).
- **Fin d'été ou début d'automne** : environ quatre mois après la tonte de
  printemps. Mèches de 4 à 6 cm [EST] (P4, P5, P17). À cette saison, la
  bande de ventre nue, les avant-bras nus et la limite nette de la toison
  se lisent bien.
- **Laine de l'encolure et de l'épaule teintée de roux**, comme chez la
  plupart des brebis photographiées (P14, P17, P9, P2). Le reste du corps
  est bise.
- Sans boucle d'oreille. Toutes les brebis photographiées en portent deux,
  jaunes : c'est l'identification obligatoire, pas une marque de race. **Ne
  pas les dessiner.**
- Pose : debout, d'aplomb, la tête un peu au-dessus du dos, les oreilles à
  l'horizontale, le regard sur le spectateur. C'est la pose de presque
  toutes les photos de troupeau (P14, P17, P22).

### 1.4 Ce qui change pour le bélier (si on en veut un)

| Trait | Brebis | Bélier |
|---|---|---|
| Poids | 55-65 kg [STD] S1 ; « 60 kg » S2 | 80-90 kg [STD] S1 ; « 90 kg » S2, S3 |
| Hauteur | 60-70 cm (les sources ne distinguent pas les sexes, S5, S6) | [EST] 66-72 cm |
| Profil | front et chanfrein en ligne régulière, très légèrement bombée | même ligne, **plus bombée**, « particulièrement chez le bélier » [STD] S1 |
| Encolure | moyenne, fine | plus épaisse, plus musclée [EST] P23 |
| Cornes | aucune | **aucune** (éliminatoire, S1) |
| Toison | 1,5 kg | 2,5 kg (S5, S6) |
| Caractère | bonne mère, protège ses agneaux (S5) | « très ardent », actif toute l'année (S2, S5) |

---

## 2. Tailles et proportions

### 2.1 Les chiffres publiés

| Mesure | Brebis | Bélier | Sources |
|---|---|---|---|
| Hauteur au garrot | 60-70 cm | 60-70 cm | [SRC] S5 (« 0,60 à 0,70 », sans distinguer les sexes) ; S6 |
| Poids adulte | 55-65 kg | 80-90 kg | [STD] S1 ; S5, S6 |
| Poids adulte (GEODE) | 60 kg | 90 kg | S2, S3 |
| Poids de la toison | 1,5 kg | 2,5 kg | S5, S6 |
| Prolificité | 1,5-1,6 en plein air (1,8 en conditions riches) ; 1,438 d'après GEODE | | S5 ; S2 |

- **Aucune source ne donne** la longueur du corps, le tour de poitrine, la
  longueur de la tête, des oreilles ou de la queue. L'Institut de
  l'élevage (IDELE) bloque la lecture automatique de sa fiche, et je n'ai
  trouvé aucune étude morphométrique de la race (§ 10.3). Tout ce qui suit
  en fractions de H est donc [EST], sauf mention contraire.
- Pour l'échelle : une brebis de 65 cm arrive à mi-cuisse d'un adulte.
  Un berger nantais la décrit comme « plus grande et plus lourde » que le
  mouton d'Ouessant (S21).

### 2.2 Le corps [STD]

- « Selle droite et longue » (S1) : la ligne du dos est droite, du garrot à
  la queue, et le corps est long. Sur P4, la ligne du dessus est plate du
  garrot à la croupe [PHOTO].
- « Gigots assez développés » (S1) : la cuisse est pleine mais pas
  culottée comme chez une race bouchère anglaise. L'agneau a « un gigot
  plus long que rond » (S5).
- « Encolure bien fondue avec les épaules » (S1) : pas de cassure entre le
  cou et l'épaule ; la laine passe de l'un à l'autre sans pli.
- « Membres nus, assez longs » (S1) : la Solognote paraît **haute sur
  pattes** à côté des races à viande laineuses (P4, P5, P17).
- Un corps de **rustique** : côtes et hanches devinées sous une toison
  courte, ventre de brebis parfois pendant (P4 : brebis probablement
  pleine), aucune rondeur de concours.

### 2.3 Le canon du dessin (brebis, H = hauteur au garrot = 100)

H = 65 cm. Les rapports sont mesurés en pixels sur le profil P4 (brebis
qui marche, l'antérieur d'appui vertical, H = 445 px à la peau), contrôlés
sur P5, P17 et P16.

| Mesure | H = 100 | cm | Balise et source |
|---|---|---|---|
| Hauteur au garrot (peau) | 100 | 65 | [SRC] S5, S6 (60-70) |
| Haut de la toison au-dessus du garrot (mèches de 4-6 cm) | 103-106 | 67-69 | [EST] P4 : 1,03 ; mèches plus longues en hiver |
| Profondeur de poitrine (garrot-sternum, peau) | 54-57 | 35-37 | [EST] P4 : 0,56 |
| Hauteur sous le sternum (peau) | 43-46 | 28-30 | [EST] P4 : 0,44 |
| Hauteur sous le ventre (point le plus bas, brebis) | 37-40 | 24-26 | [EST] P4 : 0,38 (brebis sans doute pleine) |
| Bord bas de la toison sur le flanc (au-dessus de la bande nue) | 46-50 | 30-33 | [EST] P4 : 0,48 ; P5 |
| Bande de ventre nue, vue de profil | 6-10 | 4-6 | [EST] P4, P5 |
| Longueur pointe de l'épaule - pointe de la fesse | 125-133 | 81-86 | [EST] P4 : 1,33 ; P5 : rapport tête/corps 0,32 |
| Longueur de la silhouette en toison (poitrail - fesse) | 132-140 | 86-91 | [EST] P4 |
| Tour de poitrine | 140-150 | 91-98 | [EST] calculé sur une poitrine ovale de 0,56 H × ≈ 0,30 H ; non mesuré |
| Tête, nuque - bout du nez | 42-47 | 27-31 | [EST] P4 : 0,47 ; P5 et P16 : 0,32-0,36 de la longueur du corps |
| Hauteur de la tête à l'angle de la mâchoire | 19-21 | 12-14 | [EST] P4 |
| Oreille, base - pointe | 17-20 | 11-13 | [EST] 0,38-0,45 de la tête (P16, P19, P15) |
| Largeur de l'oreille | 7-8 | 5 | [EST] P16, P15 |
| Encolure, garrot - nuque (ligne du dessus) | 50-60 | 33-39 | [EST] P4, P5 (cou tendu) ; voir § 2.4 |
| Carpe (genou) au-dessus du sol | 24-26 | 16-17 | [EST] P4 : 0,25 |
| Pointe du jarret au-dessus du sol | 32-36 | 21-23 | [EST] P4 : 0,36 (patte en arrière) ; P5 : 0,33 |
| Boulet au-dessus du sol | 6-7 | 4-5 | [EST] P4 |
| Bord bas de la laine sur l'antérieur | ≈ 46 (au coude) | ≈ 30 | [PHOTO] P4, P5, P17 |
| Bord bas de la laine sur le postérieur | ≈ 50 (au grasset) | ≈ 32 | [PHOTO] P4, P5 |
| Queue naturelle | 55-60 | 36-39 | [EST] P4 : 0,59 ; [STD] « au-dessous de la pointe du jarret » |

**Ce qu'il faut retenir :**
- Les pattes sont **nues depuis le coude et le grasset**, pas depuis le
  genou. On voit donc presque toute la patte sous le corps : environ 0,45
  H, autant que la poitrine est profonde. La Solognote se lit comme un
  corps gris-beige assez court de laine, posé **haut** sur quatre longues
  pattes rousses.
- La tête est **longue** : près de la moitié de H, un tiers de la longueur
  du corps. Ne pas la raccourcir comme une tête d'agneau.
- La queue, longue et laineuse, descend jusqu'au jarret ou plus bas.

### 2.4 L'encolure

- [STD] « Bien fondue avec les épaules » (S1). C'est le seul mot du
  standard sur le cou.
- [EST] **Longue et fine** pour un mouton : 0,50 à 0,60 H du garrot à la
  nuque sur la ligne du dessus, cou tendu (P4 : la ligne du dessus va du
  garrot à la nuque sur ≈ 0,65 H, cou étiré vers la mangeoire ; P5 ;
  P16). Par-dessous, de la pointe du poitrail à la gorge : ≈ 0,40 H.
- **La laine couvre le dessus et les côtés du cou, pas le dessous.** La
  gorge et le bas du cou restent en poil ras roux [STD] (S1, « ne revêt
  complètement ni la gorge… ») [PHOTO] (P16 : tout le dessous du cou est
  nu jusqu'au poitrail). De profil, le cou est donc une colonne laineuse
  gris-beige ou rousse posée sur une bande de poil roux.
- [EST] **Port au repos**, debout et attentive : la ligne du dessus du cou
  monte à **35-50° au-dessus de l'horizontale** (P17 et P14, vues de trois
  quarts : 40-50° corrigés ; P22). La nuque est alors à 1,15-1,25 H du sol,
  un peu au-dessus du dos. Le cou est porté moins droit que celui d'une
  chèvre, plus haut que celui d'une Bleu du Maine.
- [EST] **Chanfrein au repos** : la ligne front-nez descend vers l'avant à
  **35-45° sous l'horizontale** (P19, P15 de face ; P2 de trois quarts).
- [EST] **En broutant l'herbe** : le cou descend à **40-50° sous
  l'horizontale**, le chanfrein devient presque vertical (75-85°) et le
  museau touche l'herbe un peu en avant des pieds de devant (P5, P6, P7).
  À la mangeoire, cou tendu presque horizontal (≈ 25-30° sous
  l'horizontale, P4).
- [PHOTO] **En broutant les ligneux** (la spécialité de la race, § 8) : le
  cou se dresse presque à la verticale et la tête bascule vers l'arrière,
  chanfrein **20-30° au-dessus de l'horizontale**, pour attraper une tête
  de chardon ou une pousse (P16). Prévoir cette pose : peu de races la
  montrent aussi souvent.
- Pas de fanon, pas de pendeloques [PHOTO].
- **Pour le moteur (cou en deux segments)** : le pli bas est au garrot et
  à la base du cou ; il porte tout l'abaissement en broutant et le
  redressement vers les ligneux (de −50° à +70° par rapport au repos). Le
  pli haut est à la nuque : il fait passer le chanfrein de vertical
  (broutage) à 25° au-dessus de l'horizontale (ligneux).

### 2.5 Les articulations des pattes

Aucune source ne chiffre les angles : tout est [EST], mesuré sur P4 (qui
marche) et P5 (qui broute), contrôlé sur P17 et P22.

- **Carpe (genou)** à 0,24-0,26 H du sol (16-17 cm). Il est **nu**, roux
  comme le reste de la patte. Au repos, l'antérieur est droit et vertical
  sous l'épaule.
- **Pointe du jarret** à 0,32-0,36 H du sol (21-23 cm) : plus haut que chez
  une race à viande trapue, parce que le canon postérieur est long.
- **Angle du jarret au repos** : 140-150° (ouvert vers l'avant). Les
  pattes sont « assez longues » (S1) et fines, sans être grêles comme
  celles d'un mouflon.
- **Paturons** courts et assez droits : antérieur 55-60° sur
  l'horizontale ; postérieur 55-65°.
- **Longueurs relatives, antérieur** : avant-bras (du coude au carpe)
  ≈ 0,20-0,22 H ; canon (du carpe au boulet) ≈ 0,18 H ; paturon et
  onglon ≈ 0,06-0,07 H. Rapport avant-bras / canon ≈ 1,1-1,2.
- **Longueurs relatives, postérieur** : jambe (du grasset au jarret)
  ≈ 0,22 H ; canon (du jarret au boulet) ≈ 0,24-0,26 H. Rapport ≈ 0,9 :
  le canon arrière est **aussi long ou plus long** que la jambe (P4).
- **Bras / cuisse** : bras (pointe de l'épaule au coude) ≈ 0,20 H, cuisse
  (hanche au grasset) ≈ 0,28 H. Les deux sont **dans la toison**. Tout ce
  qui est sous le coude et sous le grasset se voit, nu.
- **Pour le moteur (pattes en deux segments)** : devant, segment haut du
  coude au carpe, bas du carpe au sabot ; derrière, haut du grasset au
  jarret, bas du jarret au sabot. Le genou plie vers l'arrière, le jarret
  vers l'avant. Comme le segment haut est nu ici, la flexion se voit
  entière : l'avant-bras et la jambe doivent être peints, pas cachés.

---

## 3. La toison

### 3.1 Ce que disent les sources

- [STD] « De qualité ordinaire, assez courte, pourvue d'un suint abondant
  et jaunâtre » (S1).
- [SRC] « Finesse moyenne et couleur bise » (S5, S6). « Sa laine à la
  couleur caractéristique » ; « la magnifique couleur bise, unique et
  inimitable » (S4, S5).
- [STD] Défaut éliminatoire : « **jarre abondant** dans la toison » (S1).
  Il y a donc un peu de jarre (fibres grossières, raides, médullées) : il
  ne doit pas dominer.
- Aucune source ne donne le diamètre des fibres ni la longueur des mèches.

### 3.2 À quoi elle ressemble

- [PHOTO] En fin d'été (P4, P5, P17) : une toison **courte et tassée**, en
  petites mèches serrées qui dessinent à la surface un réseau de **crêtes
  ondulées et de fentes sombres**, comme une écorce ou un sol craquelé
  (P4, P5, P6). Mèches de 4 à 6 cm [EST].
- [PHOTO] En fin d'hiver, avant la tonte (P2, P3, P21, au Salon de
  l'agriculture en février) : mèches de 8 à 10 cm, plus ouvertes, pointes
  qui tombent ; la surface devient bosselée, les mèches s'écartent en
  « mottes » sur le dos.
- [PHOTO] Une brebis non tondue au printemps (P8) montre des mèches de
  10-15 cm qui pendent en cordes sur les flancs : c'est l'aspect d'une
  toison négligée, pas à dessiner.
- **Le contour** est doux et un peu bosselé sur le dos, plus frangé au bas
  des flancs, où les pointes retombent sur la bande nue du ventre.
- **Le suint jaunâtre** (S1) colle la poussière : les pointes paraissent
  plus sombres et plus brunes que la base des mèches (P2, P3, P21).

### 3.3 Pourquoi elle est « bise » : un roux qui grise

- [SRC] « Le Solognot naît entièrement rouge. » Avec l'âge, « une canitie
  progressive de la toison laineuse » se produit, « cependant que les
  extrémités couvertes de poils gardent leur couleur originale et que
  quelques poils colorés se maintiennent dans la toison » (Lauvergne et
  Raffier 1975, S7). La race est homozygote pour le « rouge » au locus
  Agouti (S7, S8).
- Donc : **l'agneau est roux de la tête aux pieds** (P1, P3, P19), le
  jeune passe par un fauve doré (P6), l'adulte est bise. La tête, les
  oreilles et les pattes, en poil ras, **restent roux toute la vie**.
- [PHOTO] Chez beaucoup d'adultes, la laine de l'**encolure, du poitrail
  et de l'épaule** reste plus rousse que le corps (P14, P17, P9, P2). Le
  roux se fond dans le gris-beige vers l'arrière, sans bord : c'est l'effet
  le plus « aquarelle » de la race.
- [PHOTO] Quelques fibres rousses et brunes, plus grossières (le jarre et
  les « poils colorés » de S7), restent dispersées dans la toison,
  surtout sur le cou, la culotte et la queue.

### 3.4 Où la laine s'arrête

- [STD] « Part de la pointe des oreilles, ne couvre jamais la tête,
  s'arrête à quelques centimètres au-dessous du genou et du jarret et ne
  revêt complètement ni la gorge ni le ventre » (S1). Défaut
  éliminatoire : « toison envahissant la face ou les membres inférieurs et
  postérieurs, ou couvrant totalement la gorge » (S1). Le texte de S5 dit
  « de la base des oreilles ».
- **Tête** : aucune laine. **Pas de toupet.** La toison commence
  **derrière les oreilles**, par une limite oblique qui va de la nuque
  (juste derrière la base de l'oreille, là où arrive la pointe de
  l'oreille couchée en arrière) vers le bas et l'arrière, jusqu'au milieu
  du cou [PHOTO] (P4, P16).
- **Gorge et dessous du cou** : poil ras roux, jusqu'au poitrail [PHOTO]
  (P16, P19).
- **Ventre** : une bande de **peau châtain à poil très court** entre les
  coudes et les grassets, visible de profil sous la frange de la toison
  (P4, P5, P17 : de 4 à 6 cm de haut).
- **Pattes** : sur les photos, la laine s'arrête **au coude** devant et
  **au grasset** derrière ; l'avant-bras, le genou, la jambe et le jarret
  sont nus (P4, P5, P6, P17, P22). Le mot « au-dessous du genou et du
  jarret » du standard ne correspond pas à ce qu'on voit (§ 10.3) : suivre
  les photos et le reste du standard (« membres nus », S1).
- **La limite laine / poil** est franche (on voit où la laine s'arrête)
  mais **pas tracée à la règle** : les pointes de mèches débordent en
  frange sur le poil roux.

### 3.5 La mue et la tonte

- Tonte une fois par an, au printemps [EST] (P4, P5 en été à toison
  courte ; P2, P3 en février à toison longue). Aucune source n'en parle.
- Juste après la tonte [EST] : le corps devient d'un brun-gris roux plus
  foncé (la laine courte montre la base des fibres et la peau châtain), et
  la brebis paraît encore plus haute sur pattes.

---

## 4. Les couleurs, zone par zone

### 4.1 Comment les valeurs ont été prises

- Moyenne de carrés et de zones choisis sur les photos, avec `python3 -I`
  et PIL ; pour chaque zone, trois bandes de luminance : ombre (5-20 %),
  base (40-60 %), lumière (85-97 %). Les zones ont été vérifiées sur une
  image de contrôle (cadres dessinés sur la photo).
- Photos utilisées : P4 et P5 (profils, lumière d'abri et soleil voilé),
  P16 (tête, lumière douce), P14 (troupeau, temps couvert), P15 (neige),
  P17 (plein soleil), P1 et P6 (agneau, jeune).
- Les valeurs du tableau sont **ramenées à une lumière de jour couverte**
  et arrondies. Les mesures brutes sont dans la colonne « Notes ».
- Attention : au plein soleil (P17), le poil roux de la face monte jusqu'à
  `#E7C3A2` sur les méplats éclairés, et la toison sature presque en blanc.
  Sous un abri (P4), tout descend d'un cran (face `#472C1A`). Le tableau
  vise le milieu.

### 4.2 Le tableau

| Zone | Base | Lumière | Ombre | Notes et mesures |
|---|---|---|---|---|
| Face, front, chanfrein | `#7A3E2C` | `#A8644A` ; méplat au soleil `#C98F6E` | `#4A2218` | P16 : base `#74392C`, lumière `#B17D6D`, ombre `#4C1F1A` ; P15 (neige) : `#815B4C` ; P14 : `#83583F` |
| Joue, dessous de la mâchoire, gorge | `#6B3326` | `#8A5442` | `#451F18` | P16 gorge : `#653128` / `#845E4A` / `#471F18` ; un peu plus rouge et plus sombre que le front |
| Oreille, dos (face externe) | `#7A3E2C` | `#AB7B6E` | `#4A2218` | P16 : `#AB7B6E` sur l'oreille éloignée éclairée |
| Oreille, intérieur | `#8A4E39` | à contre-jour `#DD9F78` | `#451E1B` | P16 : `#8A4E39`, lumière `#DD9F78`. Peau fine, poil très court : **translucide orangé** à contre-jour, jamais rose |
| Tour de l'œil (paupières) | `#3A1E1A` | | | P16 [PHOTO] : un cercle brun-violet sombre autour de l'œil |
| Iris | `#B8B9B0` gris pâle | `#D8D9D0` | cercle externe `#7D7A6E` | [PHOTO] P15, P13 : gris bleuté ; P19 : paille verdâtre (variante `#C2B98A`) ; P4 : argenté. Valeurs approchées : l'œil fait quelques pixels |
| Pupille | `#141110` | | | Horizontale, rectangle aux bouts arrondis |
| Truffe, naseaux, lèvres | `#4A3A35` | `#7A6A68` (humide) | `#2A211E` | P15 : `#675149` ; P19 : `#463D2E`. Brun-gris sombre, un peu violacé ; pas noir |
| Pattes (avant-bras, canon) | `#5E3A2A` | `#87604C` | `#33200F` | P5 : `#674233` / `#876759` / `#351E11` ; P4 (abri) : `#55362A`. Plus brunes et moins rouges que la face |
| Pattes éloignées | `#4C2E21` | `#6E4D3D` | `#28180C` | [EST] un cran plus sombres |
| Onglons | `#3E3A36` | `#6D6964` (usure, poussière) | `#22201E` | P5 : `#50524D` ; P4 : `#6D6964`. Gris ardoise foncé, pas noir de jais |
| Toison bise, corps | `#CFC4B6` | `#EDE7DD` | `#958571` | P14 : `#C3B7B2` et `#D2C9C6` / `#E3DEDE` / `#8E7A65` ; P5 : `#D7CFBF` / `#F7F5EF` / `#9A8971` |
| Toison, creux entre les mèches | `#6E604F` | | | P4, P5 : les fentes sombres du « sol craquelé » |
| Toison, pointes (suint, poussière) | `#B9A58A` | | | P2, P3, P21 : pointes plus brunes que la base |
| Toison mouillée ou d'hiver | `#A1948C` | `#B2AAA4` | `#928278` | P15 (neige) : plus grise, plus froide |
| Laine rousse de l'encolure et de l'épaule | `#7A5233` | `#A88A6C` | `#4A2C14` | P14 : `#542E11` et `#7A522D` / `#A8947F` ; P9 : `#6E523F` ; P4 : `#776B59` (presque grise) |
| Peau nue du ventre | `#6A4630` | `#87644A` | `#3E2915` | P4 : `#664936` ; P5 : `#51371C`. Châtain, mat |
| Queue (laine) | `#9A8A72` | `#BDAC98` | `#5E5544` | P5 : `#8F7F67` / `#BDAC98` ; P4 : `#7F6C57`. Un peu plus sombre que le corps |
| Agneau (laine rousse) | `#7A4636` | `#9E6E5E` | `#4E2C20` | P1 : `#714E41` / `#926D64` / `#573526`, sous ombre ; à l'œil, un roux-chocolat |
| Jeune, toison fauve (P6) | `#BB9963` | `#ECCE9E` | `#7A5A32` | P6 : stade entre l'agneau roux et l'adulte bise |

### 4.3 Le roux-brun de la tête et des pattes

- [STD] « Couleur châtain roux **uniforme** » pour la tête, « châtain
  roux » pour les membres (S1). [SRC] « La peau châtain » (S4). Défaut
  éliminatoire : « **taches blanches ou jaunâtres** à la face et aux
  membres » (S1).
- C'est un **roux sombre, chaud, tirant sur le brun-rouge** : terre de
  Sienne brûlée rabattue d'ombre, pas un orangé. Au soleil, il s'éclaircit
  en cuivre (P17) ; à l'ombre, il tourne au chocolat (P4).
- **La tête est plus rouge que les pattes.** Les pattes, plus près du sol
  et de la boue, sont un brun plus éteint (comparer P5, P17, P22).
- **Uniforme** veut dire : pas de liste, pas de tache, pas de museau plus
  clair, pas de lunettes. Seuls changent le modelé et la lumière.
- Le poil est **ras et lisse**, d'aspect velouté ; il prend un reflet
  mat, cuivré, sur les méplats (front, chanfrein, joue).

### 4.4 Les marques

- **Aucune marque blanche.** Le blanc à la tête ou aux pattes est
  éliminatoire (S1). La population ancienne était « bigarrée » (S3, S5) :
  les taches ont été éliminées par la sélection depuis deux siècles.
- **Pas de noir non plus.** Une brebis Solognote noire peut naître dans
  un troupeau roux (S8) ; elle n'est pas le type de la race.
- Les seules « marques » sont des **transitions de couleur** : la limite
  laine / poil (§ 3.4), et le dégradé roux → bise de l'encolure (§ 3.3).

### 4.5 La palette d'aquarelle

La Solognote sera peinte comme l'était Aquarelle : chaque morceau est un
lavis (`Wash` dans `tools/mouflon-peintre`), avec une teinte de lumière,
de base et d'ombre, un pigment qui s'accumule sur les bords en séchant,
une marbrure, un grain de papier et un trait de crayon léger. La lumière
vient d'en haut à gauche. Voici comment peindre la race dans ce cadre.

**Les teintes de fond (premier lavis, très dilué, mouillé sur mouillé)**

| Morceau | Fond | Pigment d'aquarelle (repère) |
|---|---|---|
| Toison | `#E9E1D4` | ocre jaune très dilué + une pointe de terre d'ombre naturelle |
| Encolure et épaule | `#D7B596` posé **dans le fond de la toison encore humide** | terre de Sienne brûlée diluée |
| Tête, oreilles | `#B9765A` | terre de Sienne brûlée + un peu de rouge indien |
| Pattes | `#8E6250` | terre de Sienne brûlée + terre d'ombre brûlée |
| Ventre nu | `#9C7258` | même mélange que les pattes, plus dilué |

**Les glacis (couches posées sur un lavis sec)**

| Où | Glacis | Effet |
|---|---|---|
| Bas des flancs, dessous du corps, côté ombre de la cuisse | gris violacé `#9A8E95`, léger | l'ombre de la toison n'est pas brune mais froide (comme `shadow` d'Aquarelle, `#CFC6C9`) |
| Fentes entre les mèches | terre d'ombre `#6E604F`, au pinceau fin, sur sec | le réseau de crêtes et de fentes (§ 3.2) ; trois ou quatre rangées irrégulières, plus serrées sur l'épaule et la cuisse |
| Pointes des mèches, bas des flancs | `#B9A58A`, brossé sec | le suint et la poussière |
| Encolure rousse | deuxième glacis `#A87A55` sur le roux du fond, vers la tête seulement | le roux se renforce près des oreilles et s'éteint vers le dos |
| Tête | ombre `#5A2A1E` sous la mâchoire, sous l'œil et derrière la joue | le modelé de la tête longue |
| Pattes éloignées | glacis entier `#4C2E21` | elles reculent |
| Tour de l'œil | `#3A1E1A`, petit, sur sec | le cerne sombre qui fait ressortir l'œil clair |

**Les réserves de blanc (le papier laissé nu)**

- Le **haut du dos et de la croupe** : une bande étroite de papier blanc,
  coupée par les crêtes des mèches, là où la lumière frappe la toison.
- Le **reflet de l'œil** : un point de papier nu dans l'iris clair.
- Le **reflet de la truffe** humide : une petite virgule.
- Un liseré sur le **bord haut de l'oreille** quand elle est à contre-jour
  (sinon, rien de blanc sur la tête : elle est uniforme).
- **Rien d'autre.** Aucune réserve sur les pattes ou la tête : une tache
  claire y passerait pour une marque blanche, éliminatoire (§ 4.4).

**Où la couleur se fond (bord perdu, mouillé sur mouillé)**

- Le passage **roux de l'encolure → bise du corps** : aucune limite,
  seulement un dégradé sur la longueur d'une tête.
- Le **contour de la toison** sur le dos et la croupe : bord brisé,
  pinceau presque sec, pour que les mèches mordent sur le fond.
- La **queue** dans la culotte : elle se perd dans la toison de la fesse,
  seule la pointe se détache.
- Les **ombres portées** sous le corps et entre les pattes.

**Où le bord est net (sur papier sec, `edge` fort, crayon visible)**

- Le **contour de la tête** (chanfrein, mâchoire) et des **oreilles** : la
  tête est nue, elle se découpe nettement.
- L'**œil**, la paupière, les naseaux, la fente de la bouche.
- La **limite laine / poil** derrière l'oreille et le long du cou : nette
  mais frangée (quelques pointes de mèches débordent sur le roux).
- Le **bord bas de la toison** sur la bande de ventre nue : frange nette.
- Les **pattes** et les **onglons** : silhouettes fines et précises ; c'est
  ce qui fait la race « haute sur pattes ».

**Dans les champs du `Wash` du peintre** (points de départ, à régler en
regardant l'aperçu) :

| Lavis | `light` | `base` | `shadow` | `pigment` | `edge` / `edge_width` | `mottling` | `grain` | `pencil` |
|---|---|---|---|---|---|---|---|---|
| Toison | `#F6F1E8` | `#DDD3C5` | `#ADA0A0` | `#7E6F66` | 0,6 / 6 | 0,07 | 0,07 | `#5C4E45`, 0,7, 4 |
| Encolure rousse (lavis par-dessus la toison, masque dégradé) | `#D9B595` | `#B98E68` | `#8A6446` | `#6B4A30` | 0,3 / 4 | 0,10 | 0,07 | aucun |
| Tête | `#C08066` | `#9A5640` | `#5E2E22` | `#4A2218` | 0,55 / 5 | 0,08 | 0,08 | `#3A1E16`, 0,8, 3 |
| Patte proche | `#946A55` | `#6E4634` | `#3E2618` | `#33200F` | 0,5 / 4 | 0,07 | 0,08 | `#2A190E`, 0,75, 3 |
| Patte éloignée | `#7A5543` | `#5A3828` | `#33200F` | `#28180C` | idem | | | idem |
| Onglon (couleur de `hoof`) | | `#3E3A36` | | | | | | |

---

## 5. La tête

- **Forme** [STD] : « fine », « chauve », « face étroite et longue »
  (S1). De profil, un long coin régulier, assez haut à la nuque, qui
  s'amincit vers un museau étroit. Longueur 0,42-0,47 H [EST].
- **Profil** [STD] : « front et chanfrein formant une ligne régulière, très
  légèrement bombée, particulièrement chez le bélier » (S1). Wikipédia dit
  « front légèrement bombé » (S6). Pour la brebis : **une seule courbe très
  douce**, sans cassure au front, sans creux sous l'œil. Ni busqué comme un
  Rouge du Roussillon ou un Bizet, ni concave.
- **Pas de toupet, pas de laine sur le front.** La nuque est en poil ras
  (P16, P19, P15). C'est la grande différence avec les races dont la laine
  coiffe le front.
- **Œil** [PHOTO] :
  - placé haut sur le côté de la tête, à 0,35-0,40 de la longueur de la
    tête depuis la nuque (P4, P16) ;
  - en amande, la paupière supérieure lourde, **cerclé d'un anneau brun
    sombre** (P16) ;
  - **iris très clair** : gris bleuté, argenté ou paille, selon l'animal et
    la lumière (P4, P13, P15, P16, P19). Sur la tête rousse, cet œil pâle
    attire le regard ; il donne la mine « éveillée » des photos ;
  - **pupille horizontale**, comme chez tous les ruminants brouteurs ;
    quand la tête descend pour brouter, l'œil tourne pour garder la
    pupille parallèle au sol (S19). Pour l'animation : garder la pupille
    horizontale quelle que soit la pente de la tête.
- **Museau et naseaux** [PHOTO] : museau étroit, assez long, couvert du même
  poil roux jusqu'au bord des lèvres. Truffe petite, **brun-gris sombre**
  (pas noire, pas rose). Naseaux en virgules obliques, remontant vers
  l'arrière. Lèvre supérieure fendue (philtrum). Lèvres fines, sombres
  (P16, P19).
- **Oreilles** [STD] : « moyennes, horizontales, légèrement droites,
  jamais tombantes, couvertes de poils fins et courts » (S1).
  - [EST] 0,38-0,45 de la longueur de la tête, larges d'environ 0,4 de leur
    longueur ; feuille ovale, pointe arrondie.
  - **Portées à l'horizontale, sur le côté**, juste derrière et un peu
    au-dessus de l'œil, à la limite de la laine (P15, P13, P17, P22).
    « Jamais tombantes » : ne pas les dessiner pendantes comme chez un
    Rouge du Roussillon ou une race laitière.
  - Même roux que la tête dehors ; dedans, peau fine orangée, translucide à
    contre-jour (§ 4.2).
  - Mobiles : elles pivotent vers l'avant quand la brebis regarde, se
    couchent en arrière quand elle broute vers le haut (P16).
- **Mâchoire** : ni prognathe ni rétrognathe ; bouche fermée et nette
  [EST].

---

## 6. Les cornes : il n'y en a pas

- [STD] « Sans cornes » ; « présence de cornes » = défaut éliminatoire
  (S1). [SRC] « Sans corne » (S6) ; « moutons roux et blancs, sans
  cornes » dès 1770 (S5).
- **Les deux sexes** sont sans cornes. Le bélier aussi.
- **Ce qu'il ne faut pas dessiner :**
  - aucune corne, ni ébauche (« bouton »), ni corne branlante (« scur ») ;
  - pas de bosses osseuses marquées sur le crâne : la nuque est arrondie,
    lisse, en poil ras ;
  - pas de toupet de laine à la place des cornes.
- Si un rig générique de Mouflon a des os de cornes, **les laisser vides**.

---

## 7. Les pattes et les onglons

- [STD] « Membres nus, assez longs, de couleur châtain roux » (S1).
- **Nus** du coude et du grasset jusqu'aux onglons (§ 3.4) : poil ras,
  lisse, qui laisse voir les tendons du canon et la saillie du genou et du
  jarret (P4, P5).
- **Fins mais solides.** Le canon est mince ; l'avant-bras est un peu plus
  épais et musclé en haut. Rien de grêle ni de « pattes de cerf ».
- **Aplombs** : antérieurs verticaux sous l'épaule ; postérieurs un peu
  sous le corps, jarret marqué (§ 2.5).
- **Onglons** [PHOTO] : petits, en pointe, les deux doigts serrés, **gris
  ardoise foncé**, souvent poussiéreux ou usés en clair au bord (P4, P5,
  P1). Ergots discrets derrière le paturon.
- **Une race des sols mouillés** : elle « résiste au piétin » (S1, S2,
  S5), et « en l'absence de piétin, elle est pratiquement la seule race
  française à pouvoir pâturer les pieds dans l'eau » (S5). Pour un décor :
  les pieds dans une prairie humide ou au bord d'un étang de Sologne
  (P10).

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Le caractère

- [SRC] « Un tempérament intelligent, vif, curieux, indépendant, **plus
  proche du caractère des chèvres que des autres moutons**. Sa démarche est
  très fière » (S5). Wikipédia reprend : « vive et très curieuse » (S6).
- [PHOTO] Sur presque toutes les photos de troupeau, **toutes les têtes
  sont levées vers le photographe**, oreilles à l'horizontale (P14, P17,
  P22, P19). La curiosité est le premier trait à animer : s'arrêter,
  lever la tête, fixer, puis s'approcher d'un pas.
- [PHOTO] Une brebis et son agneau regardent depuis derrière un tronc
  (P19) : la curiosité prudente, la tête qui dépasse.
- « Très attachante par son caractère et son esthétique » (S5).

### 8.2 La vie au pré

- **Grégaire** : troupeaux serrés, qui se groupent autour du foin en
  hiver (P18) et se tiennent en ligne face au visiteur (P22). Toujours au
  moins quelques congénères.
- **Brouteuse de ligneux.** « Son goût prononcé pour les ligneux empêche
  la fermeture du milieu par les ronces et les arbustes » (S4, S5). Elle
  valorise « les pâturages pauvres, les sous-bois ligneux » et supporte
  « des variations brusques de régime alimentaire, régime dit en
  accordéon » (S3, S5). Animation propre à la race : **tête levée, cou
  dressé, la brebis arrache une pousse ou une tête de chardon** (P16).
- **Très bonne marcheuse** (S3, S5) : elle parcourt de grands espaces
  (landes, sous-bois, bords de rivière).
- **Rustique** : dehors toute l'année, y compris dans la neige (S6 ; P13,
  P15, P18) ; tolère les parasites et le piétin (S1, S2, S5).
- **Bonne mère** : « elles protègent bien leurs agneaux contre les
  agressions » (S5). Agnelages faciles : « les éleveurs de Solognote
  dorment la nuit en période d'agnelage » (S5).
- **Rumination** couchée sur le sternum, pattes repliées sous le corps,
  tête haute (P9) ; elle se lève par l'arrière, comme tous les ruminants.

### 8.3 Les gestes et les postures

- **Allure** : pas long et souple sur de longues pattes, port de tête
  assez haut ; « démarche très fière » (S5). Trot léger. Le corps reste
  horizontal ; la queue longue balance un peu au pas.
- **Arrêt, regard** : cou à 45°, chanfrein à 40° sous l'horizontale,
  oreilles tournées en avant, immobile (P17, P22).
- **Brouter l'herbe** : cou bas, museau près des pieds de devant ; un pas,
  quelques bouchées, un pas (P5, P6).
- **Brouter vers le haut** : cou dressé, tête basculée, oreilles couchées
  en arrière (P16). Une Solognote qui se dresse contre un arbuste sur ses
  pattes de derrière n'est **pas documentée** : ne pas l'inventer comme
  trait de race.
- **Voix** : aucune voix propre à la race n'est décrite. Répertoire
  ordinaire du mouton : bêlement fort, bouche ouverte, quand elle est
  séparée du troupeau ; bêlement bas, bouche fermée, entre la brebis et
  son agneau (S20). Bêlement : tête levée, cou tendu, mâchoire ouverte
  (P9, une brebis couchée qui bêle).
- **Agneaux** : tout roux, ils jouent en groupe au milieu des brebis (P3,
  P8, P20).

---

## 9. Les dix détails, et les erreurs à éviter

### 9.1 Les dix détails qui font une Solognote

1. **Tête, oreilles et pattes nues, d'un roux-brun foncé uniforme**, sans
   aucune tache (S1).
2. **Corps couvert d'une laine bise** (gris-beige chaud), courte et
   tassée, en réseau de crêtes et de fentes (S1, S5 ; P4, P5).
3. **Pas de cornes**, ni chez la brebis ni chez le bélier (S1).
4. **Pas de laine sur la tête** : la toison commence derrière les
   oreilles ; pas de toupet (S1).
5. **Gorge, dessous du cou et ventre nus** : une bande de poil roux sous
   le cou et une bande de peau châtain sous le ventre, visibles de profil
   (S1 ; P4, P16).
6. **Longues pattes nues dès le coude et le grasset**, fines (S1 ; P4,
   P17).
7. **Tête longue et étroite**, profil en une seule courbe très douce,
   presque droit chez la brebis (S1).
8. **Oreilles moyennes portées à l'horizontale**, jamais tombantes (S1).
9. **Œil très clair** dans un cerne sombre, sur la tête rousse (P15, P16).
10. **Encolure souvent plus rousse que le corps**, en dégradé ; **longue
    queue laineuse** qui descend sous le jarret (S1 ; P14, P4).

### 9.2 Erreurs à éviter

- Une **tache blanche** au front, au museau ou aux pattes (éliminatoire,
  S1), et toute réserve de papier blanc sur la tête ou les pattes.
- Des **cornes** ou des bosses de cornes.
- Un **toupet** ou une laine qui descend sur le front ou les joues.
- Une toison qui **couvre la gorge, le ventre ou les pattes** (éliminatoire,
  S1).
- Une toison **blanche pure** : elle est bise. Ou au contraire **brune
  uniforme** : seuls l'agneau et le jeune sont roux ou fauves.
- Une tête **noire**, **rose** ou **orangé vif** : elle est châtain roux,
  sombre.
- Des oreilles **tombantes** ou dressées en l'air.
- Un profil **busqué** marqué (brebis) ou un museau court et carré.
- Une **queue coupée** courte : la queue est entière et descend sous le
  jarret (S1).
- Des pattes **courtes** ou cachées dans la laine : c'est une race haute
  sur pattes.
- Un œil **noir** ou brun foncé uniforme : l'iris est clair.
- Une **toison longue en cordes** (P8) : c'est une toison négligée.

### 9.3 Les races voisines, nommément

| Race | Ce qui la rapproche | Ce qui l'en distingue |
|---|---|---|
| **Rouge du Roussillon** | La plus proche d'aspect : tête, pattes et ventre nus et **rouges**, pas de cornes, agneau roux qui « mue vers 3 mois pour adopter une toison bise très tassée » (S12, S13, S18) | Tête et pattes souvent **pie** (rouge-brun et blanc) ou avec une **liste blanche en tête** (S12, S13) : éliminatoire chez la Solognote. Oreilles « qui pendent un peu vers l'avant » (S13), au lieu d'horizontales. Plus grande : béliers 75-90 cm (S13, S18). Une partie de la population a la base de la queue élargie en entonnoir sur 6-10 cm (héritage de la Barbarine à queue grasse, S12, S13) ; la Solognote a une queue fine. Race du Languedoc, pas de Sologne. |
| **Bizet** | Race rustique française à toison qui grise avec l'âge, tête et pattes nues et colorées (S7, S14) | Tête et pattes **noires**, avec une **liste blanche** en tête, le bout des pattes et de la queue blancs (S7, S9, S14). Le bélier porte des **cornes** en spirale (S14). Chanfrein busqué, oreilles noires courtes et dressées (S14). L'agneau naît noir, pas roux (S7). |
| **Lacaune** | Pas de cornes, tête, cou, ventre et pattes dégarnis de laine, oreilles moyennes à l'horizontale (S17) | Tout est **blanc** : toison blanche, tête et pattes couvertes de **fins poils blancs** (S17). Profil légèrement busqué. Grande brebis laitière du Rouergue, pis développé. |
| **Landes de Bretagne** | Petite race rustique de l'Ouest à tête dégarnie, employée elle aussi en éco-pâturage (S15) | Plus petite (50-60 cm, brebis 20-50 kg, S15). Couleurs variables (noire, brune, grise, blanche ; extrémités blanches ou tachées de roux) et toison plus longue, lisse, mêlée de jarre (S15). Béliers souvent **cornus** (S15). Queue de longueur très variable. Jamais l'ensemble « tête rousse uniforme + toison bise ». |
| **Berrichon** (du Cher, de l'Indre) | Voisin géographique (Berry), pas de cornes (S16) | Tout **blanc**, peau rose autour des yeux et du mufle (S16). Plus grand et plus lourd (brebis ≈ 68 cm et 70-90 kg ; béliers ≈ 73 cm et 110-140 kg, S16). Race à viande à corps compact. Croisé à la Solognote, il donne des agneaux pie (S7). |
| Rouge de l'Ouest (pour mémoire) | Tête et pattes nues colorées, sans cornes | Race à viande de l'Ouest, toison **blanche**, tête et pattes **rouge rosé**, béliers de 110-130 kg (d'après `docs/races/bleu-du-maine.md`, § 9.3) : plus lourde, plus rose, sans toison bise. |

---

## 10. Sources, photos, désaccords

### 10.1 Sources écrites

- **S1.** GEODE (organisme de sélection), « Solognote, standard de la
  race » (fiche d'une page, avec photos). Hébergée par le CRAPAL :
  https://crapal.fr/wp-content/uploads/2022/04/Standard-de-race-FBS.pdf ,
  et par GEODE :
  https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Standard%20de%20race%20FBS.pdf
- **S2.** GEODE, page de la race « Solognot » (effectifs, poids,
  prolificité, origines, aptitudes, galerie) :
  https://www.geodesheep.com/fr/solognot-426.html
- **S3.** GEODE, fiche de la race (PDF, français et anglais) :
  https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/SOLOGNOT%20FR%20ANG-1.pdf
- **S4.** CRAPAL (Conservatoire des races animales en Pays de la Loire),
  « Le mouton Solognot » (brochure des 20 ans, 4 pages : historique,
  aptitudes, effectifs, valorisation) :
  https://crapal.fr/wp-content/uploads/2022/04/20ans-LeMoutonSolognot.pdf
- **S5.** ProNaturA-France, « La race ovine solognote » (texte du
  Flock-Book solognot vers 1995 : historique, standard, caractère,
  aptitudes, avec cinq images) :
  http://pronaturafrance.free.fr/Solognote.html
- **S6.** Wikipédia (FR), « Solognote » :
  https://fr.wikipedia.org/wiki/Solognote
- **S7.** J.-J. Lauvergne et P. Raffier (1975), « Génétique de la couleur
  de la toison des races ovines françaises Solognote, Bizet et
  Berrichonne », *Annales de génétique et de sélection animale* 7(3),
  263-276 :
  https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-7-3-263.pdf
- **S8.** J.-J. Lauvergne, B. Denis, M. Théret (1977), « Hybridation entre
  un Mouflon de Corse et des brebis de divers génotypes : gènes pour la
  coloration pigmentaire », *Ann. Génét. Sél. anim.* 9(2), 151 (brebis
  Solognotes noires « apparues dans des troupeaux de couleur rouge,
  habituelle en cette race ») :
  https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-9-2-151.pdf
- **S9.** J.-J. Lauvergne et P. Raffier (1976), « Fertilité comparée des
  brebis… au locus d'Extension » (description du Bizet : noir à points
  blancs) :
  https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-8-1-47.pdf
- **S10.** Réussir Pâtre (2 mars 2020), « Au salon de l'agriculture,
  l'agrobiodiversité est ovine » (marque « Agneau de Sologne », « la race à
  tête marron ») :
  https://www.reussir.fr/patre/au-salon-de-lagriculture-lagrobiodiversite-est-ovine
- **S11.** *Études rurales* n° 191 (2013), compte rendu de B. Heude, *La
  Sologne. Des moutons, des landes et des hommes, du XVIIIᵉ siècle au
  Second Empire* (PUR, 2012) : « l'une des plus petites races ovines de
  France », supporte très bien l'humidité :
  https://journals.openedition.org/etudesrurales/9869
- **S12.** Races de Massif (CORAM), « Rouge du Roussillon » :
  https://www.races-montagnes.com/fr/races/raiole/rouge-du-roussillon.php
- **S13.** FAO, Brooke et Ryder (1978), *Declining breeds of Mediterranean
  sheep*, § « Roussillon Red » :
  https://www.fao.org/4/X6508E/X6508E03.htm
- **S14.** Wikipédia (FR), « Bizet (race ovine) » :
  https://fr.wikipedia.org/wiki/Bizet_(race_ovine)
- **S15.** Wikipédia (FR), « Landes de Bretagne » :
  https://fr.wikipedia.org/wiki/Landes_de_Bretagne
- **S16.** Wikipédia (FR), « Berrichon du Cher » :
  https://fr.wikipedia.org/wiki/Berrichon_du_Cher
- **S17.** Techno-science, « Lacaune (race) » (reprise de Wikipédia) :
  https://www.techno-science.net/fr/definitions/lacaune-race
- **S18.** Wikipédia (EN), « Rouge du Roussillon » :
  https://en.wikipedia.org/wiki/Rouge_du_Roussillon
- **S19.** EurekAlert, communiqué sur Banks et al. 2015 (*Science
  Advances*) : forme de la pupille et rotation de l'œil chez les
  brouteurs : https://www.eurekalert.org/news-releases/632799
- **S20.** *Frontiers for Young Minds* (2022), la communication vocale du
  mouton (bêlements hauts et bas) :
  https://kids.frontiersin.org/articles/10.3389/frym.2022.703514
- **S21.** Unidivers, berger urbain à Nantes (Solognote « plus grande et
  plus lourde » qu'Ouessant ; lu seulement dans un extrait de recherche) :
  https://unidivers.fr/nantes-samuel-un-berger-dans-la-ville-reseaux-sociaux/
- Consultées sans résultat utilisable :
  - IDELE (fiche « Solognote ») : la page exige JavaScript contre les
    robots, illisible.
  - Races de France (`races-de-france.fr`) : connexion refusée.
  - Pas de site actif trouvé pour une association d'éleveurs ; l'adresse
    historique du Flock-Book solognot est dans S5. La section raciale est
    aujourd'hui au sein de GEODE (S2, P23).
  - FAO DAD-IS : non consulté (application JavaScript).
  - Oklahoma State University, *Breeds of Livestock* : pas de page
    Solognote (403).

### 10.2 Photos regardées

Téléchargées hors du dépôt (dossier de travail), ouvertes et mesurées.
Wikimedia Commons a refusé trois fichiers de la catégorie (erreur 429,
« trop de requêtes ») malgré plusieurs essais espacés : `Solognote.jpg`,
`Mouton solognote agneaux.jpg`, `CampagneSolognote-f (40280678181).jpg`.

| N° | Photo | Ce qu'elle m'a appris |
|---|---|---|
| P1 | Commons, « Agneau solognot » : https://commons.wikimedia.org/wiki/File:Agneau_solognot.JPG | Agneau **entièrement roux-chocolat**, laine frisée courte ; oreille orangée translucide ; couleurs d'agneau |
| P2 | Commons, « Brebis solognote » (Salon) : https://commons.wikimedia.org/wiki/File:Brebis_solognote.jpg | Tête de trois quarts, yeux mi-clos : roux uniforme, pas de toupet, oreille relâchée qui retombe un peu quand elle somnole ; laine de l'encolure rousse à pointes plus foncées ; agneaux roux |
| P3 | Commons, « Brebis solognotes » (Salon) : https://commons.wikimedia.org/wiki/File:Brebis_solognotes.jpg | Toison longue de fin d'hiver, pointes brunies ; pattes nues ; une portée d'agneaux tous roux |
| P4 | Commons, « Mouton solognot (3) » : https://commons.wikimedia.org/wiki/File:Mouton_solognot_(3).JPG | **Le profil de référence** (tournée vers la droite) : ligne du dos droite, pattes nues dès le coude et le grasset, bande de ventre nue, queue longue, tête longue, œil argenté ; toutes les proportions du § 2.3 |
| P5 | Commons, « Mouton solognot (6) » : https://commons.wikimedia.org/wiki/File:Mouton_solognot_(6).JPG | Profil en broutant (tournée vers la gauche) : angle du cou et du chanfrein, toison « craquelée », bande nue, couleurs au soleil voilé |
| P6 | Commons, « Mouton solognot 101 » : https://commons.wikimedia.org/wiki/File:Mouton_solognot_101.JPG | Jeune à toison **fauve dorée** : le stade entre l'agneau roux et l'adulte bise |
| P7 | Commons, « Puy du Fou - Parc nature - 13 » : https://commons.wikimedia.org/wiki/File:Puy_du_Fou_-_Parc_nature_-_13.jpg | Deux brebis qui broutent ; toison claire, tête et pattes sombres, vues de loin |
| P8 | Commons, « Solognotes » : https://commons.wikimedia.org/wiki/File:Solognotes.jpeg | Troupeau au printemps ; agneau roux de profil ; brebis non tondue à mèches en cordes (à ne pas copier) ; toutes les têtes levées |
| P9 | Commons, « Mouton solognot brebis » (Salon) : https://commons.wikimedia.org/wiki/File:Mouton_solognot_brebis.jpg | Brebis couchée qui rumine et une qui bêle ; encolure rousse, oreille intérieure orangée |
| P10 | Commons, « Campagne Solognote » : https://commons.wikimedia.org/wiki/File:Campagne_Solognote_(32982882208).jpg | Le paysage du berceau : étang, arbres nus, prairie humide (décor) |
| P11 | Commons, « Moutons solognots nés à Madagascar… » (BnF, vers 1900) : https://commons.wikimedia.org/wiki/File:Moutons_solognots_n%C3%A9s_%C3%A0_Madagascar_issus_de_parents_achet%C3%A9s_en_Sologne,_Tananarive,_Madagascar_-_Etat_major_de_Madagascar_-_btv1b10119936p_(1_of_2).jpg | Le type ancien : tête sombre nue, toison claire courte, pattes fines ; même silhouette qu'aujourd'hui |
| P12 | Les trois photos du standard GEODE (S1) | Tête de face (roux uniforme, oreilles horizontales, toison bise) ; troupeau dans la neige de profil ; agnelle dans la neige |
| P13 | GEODE, image de rubrique : https://www.geodesheep.com/files/site/img/geode/RACE/Presentation/Solognot%20(photo%20rubrique).jpg | Recadrage de P15 |
| P14 | GEODE, « Solognot » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot.jpg | Troupeau sous des chênes, temps couvert : **encolure et épaule rousses** en dégradé vers un corps blanc crème ; port de tête au repos ; couleurs de toison |
| P15 | GEODE, « Solognot1 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot1.jpg | Tête de face dans la neige : **œil gris bleuté**, face étroite, oreilles horizontales, truffe brun-gris ; toison mouillée grise |
| P16 | GEODE, « Solognot3 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot3.JPG | **La tête de référence**, de profil, cou dressé vers un chardon : poil ras, cerne sombre de l'œil, naseaux, oreilles, gorge nue, limite de la laine ; couleurs de la tête |
| P17 | GEODE, « Solognot4 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot4.JPG | Cinq brebis au plein soleil, une presque de profil : pattes nues dès le coude, bande de ventre nue, encolure rousse ; couleurs au soleil |
| P18 | GEODE, « Solognot5 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot5.JPG | Troupeau serré au foin dans la neige (avec des ânes) : grégarité, rusticité |
| P19 | GEODE, « Solognot6 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot6.JPG | Brebis et agneau derrière un arbre : tête longue de face, œil paille, truffe ; agneau roux ; curiosité |
| P20 | GEODE, « Solognot7 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot7.JPG | Troupeau mené sur un chemin, agneaux roux et jeunes fauves mêlés aux adultes ; un mouton blanc à sonnaille d'une autre race en tête (ne pas l'attribuer à la Solognote) |
| P21 | GEODE, « Solognot8 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot8.JPG | Brebis en toison pleine au râtelier : mèches longues ouvertes, têtes rousses basses |
| P22 | GEODE, « Solognot9 » : https://www.geodesheep.com/files/site/img/geode/RACE/Solognot/Solognot9.JPG | Huit brebis de face devant un mur : pattes longues, toisons de bise à fauve, oreilles horizontales, toutes curieuses |
| P23 | GEODE, lettre « La Sologn'News » : https://www.geodesheep.com/files/site/img/geode/ACTUS/VENTES/VENTES%202024/newlettersolognot.JPG | Jeunes béliers du centre d'élevage : têtes plus massives, roux plus sombre |
| P24 | ProNaturA (S5) : tableau d'Esbrat (1852) et trois photos de troupeau | Moutons déjà « uniformément roux » en 1852 ; troupeaux de face et de profil |

Écartée : l'affiche « Invitation » de la vente de béliers GEODE (2026),
https://www.geodesheep.com/files/site/img/geode/ACTUS/VENTES/VENTE%20FBS%202021/Invitation_GEODE_Solognote_001.jpg :
c'est une illustration retouchée (laine stylisée), pas une photo ; je ne
m'en suis servi pour aucune mesure ni couleur.

### 10.3 Ce sur quoi les sources se contredisent

- **La queue.** Le standard GEODE dit « Queue : non coupée, couverte de
  laine, descendant au-dessous de la pointe du jarret », puis range
  « Queue non coupée » parmi les défauts éliminatoires (S1). La seconde
  phrase est sans doute une coquille pour « queue coupée » : S5 dit « la
  queue est non coupée et couverte de laine ». Dessiner une **queue
  entière**.
- **Où s'arrête la toison sur les pattes.** S1 : « à quelques centimètres
  au-dessous du genou et du jarret » ; S5 : « à quelques centimètres en
  dessous du jarret ». Mais le même standard dit « membres nus » et
  élimine la toison qui envahit les membres, et **toutes** les photos
  montrent des pattes nues dès le coude et le grasset (P4, P5, P17, P22).
  J'ai suivi les photos.
- **D'où part la toison.** « De la pointe des oreilles » (S1) ou « de la
  base des oreilles » (S5) : dans les deux cas, derrière l'oreille. Les
  photos placent la limite juste derrière la base de l'oreille.
- **La taille.** S5 et S6 donnent 60-70 cm sans distinguer les sexes ; S11
  (d'après Heude) la dit « l'une des plus petites races ovines de France »,
  ce qui ne cadre ni avec 60-70 cm ni avec 55-65 kg : c'est peut-être vrai
  de la population du XVIIIᵉ siècle, que décrit le livre.
- **Le poids du bélier.** 80-90 kg (S1, S5, S6) ou 90 kg (S2, S3).
- **Les effectifs.** « Plus de 3 000 brebis » et une cinquantaine
  d'éleveurs (S4) ; « 3 000 brebis » (S2) ; « environ 1 000 brebis » dans
  le plan d'accouplement de 1995 sur « près de 2 000 individus » (S5).
- **Le nombre de familles.** Dix familles (S2, S3, S4) ; S5 écrit
  « 40 familles » dans le texte mais en dessine onze sur son schéma : la
  gestion a changé avec le temps.
- **La couleur de l'œil.** Aucune source écrite n'en parle. Les photos
  montrent un iris clair, mais gris bleuté sur certaines (P15), paille ou
  verdâtre sur d'autres (P19), argenté sur P4. Ce n'est pas un caractère
  de standard.
- **Le nom.** GEODE écrit « Solognot » pour la race et « Solognote » pour
  la brebis et le standard (S2, S3) ; Wikipédia et le CRAPAL disent
  « Solognote » (S4, S6).
- **Aucune mesure publiée** pour la longueur du corps, la tête, les
  oreilles, la queue, le tour de poitrine ni les angles : tout le § 2.3,
  sauf la hauteur et le poids, est [EST], mesuré surtout sur un seul
  profil de bonne qualité (P4), avec les limites que cela suppose
  (animal qui marche, légère perspective).
