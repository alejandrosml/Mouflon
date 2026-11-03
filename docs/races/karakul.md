> Fiche de la race « Karakul » (Qorakoʻl ; en Namibie « Swakara »), préparée le 3 novembre 2026 pour dessiner un futur `pets/karakul` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Karakul : fiche pour un dessin de profil

**Noms :** Qorakoʻl qoʻyi (UZ), каракульская порода, каракульская овца (RU), Karakul, Caracul, Karakulschaf (DE) ; en Ouzbékistan, les noirs sont appelés « Arabi » [S FAO-URSS] ; en Namibie et en Afrique du Sud, la population locale porte depuis 2013 le nom de **Swakara** (de *South West Africa Karakul*) [S Nam1][S FW]. Le nom vient de l'oasis et de la ville de Qorakoʻl, sur le Zeravchan, dans la région de Boukhara [S BSE-K][S WP-EN].
**Groupe :** moutons à **queue grasse** (курдючные / жирнохвостые), à laine grossière (грубошёрстные), d'orientation « smouchka » : la peau de l'agneau nouveau-né (astrakan, karakul, « Persian lamb ») [S BSE][S SK].
**Rédigée :** le 3 novembre 2026, d'après la Grande Encyclopédie soviétique, le manuel de Sokolov et Kuts, un manuel ouzbek de *smouchkovédénié* (Tachkent, 1987), trois monographies de la FAO (URSS, Afrique tropicale, Turquie) et une note de la FAO sur la Mongolie, deux études de mensurations (Afghanistan 2022, Moldavie 2023), les textes des éleveurs américains et namibiens, et une soixantaine de photographies (section 10).

**Étiquettes :**
- **[S …]** : une source écrite, nommée en section 10.1.
- **[P]** : observé ou mesuré sur une photo (section 10.2) ; pixels comptés, couleurs moyennées sur de petites zones avec PIL.
- **[E]** : mon estimation (anatomie générale du mouton, recoupée sur les photos). Aucune source ne la donne.
- **H** = hauteur au garrot (os) ; **LT** = longueur de la tête (nuque → bout du museau).

---

## 0. En bref

- **L'animal à dessiner :** une **brebis adulte de quatre ans, de la lignée noire (« Arabi »)**, au **début de septembre**, environ quatre mois après la tonte de printemps : née noire et bouclée, elle a **grisonné** comme tous les Karakuls noirs ; sa toison est un **gris brun sombre** de mèches grossières, lustrées, à pointes roussies par le soleil, mais sa **tête, ses oreilles, ses pattes et sa queue restent noires** [S BSE][S FAO-Tur]. Sans cornes. **De profil strict, tournée vers la droite** (on voit son flanc gauche), tête portée haut, au bord d'un pâturage de désert.
- **En une phrase :** un mouton **haut sur pattes, étroit et long**, au dos qui **monte vers les reins** puis retombe sur une croupe en pente, prolongée par une **grosse queue grasse** qui pend comme un coussin derrière les cuisses et finit en **petit appendice noir en S** à hauteur du jarret ; une **tête noire, longue, au chanfrein busqué**, avec de **longues oreilles pendantes** ; une toison gris sombre aux longues mèches ondulées.
- **Le troupeau :** de grands troupeaux (environ 500 têtes dans le nord de l'Afghanistan) menés au pâturage onze heures par jour [S Musavi] ; en Ouzbékistan, ils font 18 à 20 km par jour dans le désert et boivent une fois par jour ou tous les deux jours [S ZK]. Menacé, le troupeau **fait cercle autour des agneaux** [S FarmShow][S HotR].

**Palette de base** (détails et mesures en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Toison, dos et haut du flanc (pointes roussies) | `#9C8F7A` | `#CFC3A9` | `#6B614E` |
| Toison, épaule et milieu du flanc | `#7E7462` | `#A99D86` | `#4A4034` |
| Toison, bas du flanc et cuisses | `#5A4F42` | `#7E705E` | `#2E261D` |
| Fond des mèches, creux, ventre | `#3A322A` | `#5F5548` | `#1E1812` |
| Tête, oreilles, pattes, queue (poil ras noir) | `#1A191A` | `#3A3A3E` | `#0B0B0C` |
| Iris | `#6E5646` (brun ambré) | `#8C6E52` | pupille `#0B0A0A` |
| Onglons | `#1E1B19` | `#4A4440` | `#0E0C0B` |
| Agneau nouveau-né (boucles) | `#1E1D20` | reflet `#55575F` | `#0C0C0F` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 La race, en quelques lignes

- **Mouton du désert d'Asie centrale** (Ouzbékistan surtout, Turkménistan, Tadjikistan, sud du Kazakhstan), au climat sec et chaud, aux hivers jusqu'à −35 à −40 °C [S ZK][S FAO-URSS]. Une race « ancienne » : la plupart des auteurs y voient une sélection très longue de moutons locaux par les peuples d'Asie centrale [S BSE] ; d'autres une origine croisée (moutons noirs à queue longue de Boukhara, *danadar*, croisés de moutons à croupe grasse), ou une arrivée avec les Arabes au VIIIᵉ siècle, d'où le nom « Arabi » [S FAO-URSS][S SK][S WP-DE].
- **Effectifs :** 12,4 millions en URSS en 1980 [S FAO-URSS] ; environ 31 millions dans une cinquantaine de pays selon Wikipédia en russe [S WP-RU]. En Namibie, 1,4 million en 1985 [S FAO-Afr], environ 200 000 aujourd'hui [S Nam2].
- **Couleurs :** 75 à 80 % de noirs, 13 à 17 % de gris, 5 à 7 % de *sur* (brun ou gris à pointes dorées ou argentées), moins de 3 % d'autres [S ZK] ; la BSE donne 80 % de noirs [S BSE].
- **Produits :** la peau de l'agneau de un à trois jours (le « karakul » des fourreurs) [S BSE-K][S FAO-Afr], la graisse de la queue (*dumba*, *курдюк*) très prisée dans la cuisine ouzbèke [S WP-EN], le lait des brebis (25–30 kg par lactation), la laine grossière des adultes, pour les tapis et le feutre [S BSE][S SK].
- **Ailleurs :** Afghanistan (meilleure part des peaux grises), Iran, Namibie et Afrique du Sud (Swakara, depuis 1907), Moldavie (Karakul moldave), Turquie, Mongolie (désert de Gobi), Allemagne (« Deutsches Karakulschaf », race menacée de l'année 2015), États-Unis (American Karakul, population à part depuis les importations de 1908–1929) [S FAO-Afr][S FAO-Tur][S FAO-Mon][S GEH-PM][S TLC].

### 1.2 Pourquoi une brebis, et laquelle

1. **Le troupeau, ce sont des brebis.** Dans les troupeaux de Karakuls d'Afrique australe, plus de 95 % des bêtes sont des femelles [S FAO-Afr].
2. **Ce qui fait le Karakul se voit mieux sur elle.** Sans cornes [S BSE][S SK], la brebis montre en entier la **tête busquée** et les **longues oreilles pendantes**, et rien ne distrait de la **queue grasse en S**, le trait que tous les textes nomment d'abord [S BSE][S SK][S FAO-Afr][S Buzu].
3. **La lignée noire est la race même :** « la race principale, ce sont les Karakuls noirs », de constitution solide [S ZK] ; 75 à 80 % des bêtes [S ZK].
4. **Elle montre le trait le plus curieux de la couleur :** née noire et bouclée, elle **grisonne en vieillissant**, et seules la tête et les pattes restent noires [S BSE] ; en Turquie, « la toison est noire chez l'agneau et gris foncé chez l'adulte ; la tête, les pattes et la queue des adultes sont noires » [S FAO-Tur]. On montre ainsi, sur un seul animal, l'adulte et le souvenir de l'agneau.
5. **Pourquoi début septembre :** les Karakuls sont tondus **deux fois par an**, en mai et en septembre en Afghanistan [S Musavi][S SK][S BSE]. Juste avant la tonte d'automne, la toison a environ quatre mois (mèches de 8 cm environ [E], section 3.3), assez longue pour être une toison, assez courte pour que la ligne du dos, la croupe et la **queue** se lisent. C'est aussi la fin de la saison où les moutons à queue grasse ont rempli leur réserve [E, d'après S Epstein-WAR pour l'Awassi].

**L'individu :**
- brebis de **4 ans** (pleine production entre 3 et 6 ans [S ZK]) ;
- **garrot 66 cm**, **46 kg** (sections 2.1) ;
- lignée **noire** : toison grisée, brunie aux pointes ; **tête, oreilles, pattes, queue noires** ;
- **sans cornes**, sans bourgeons visibles ;
- oreilles longues et pendantes (la race connaît des oreilles courtes et même des bêtes « sans oreilles » [S SK][S AKSR-LJ] : on ne les dessine pas) ;
- debout, carrée, tête haute, la queue grasse au repos contre les fesses ; **pas de boucle d'oreille** ni de marque de peinture.

### 1.3 L'autre sexe : le bélier (pour mémoire)

| | Brebis (choisie) | Bélier |
|---|---|---|
| Garrot | 62–70 cm [S SK] ; 66,5 [S FAO-Tur][S FAO-Mon] | 72–78 cm [S SK] ; 75,2 [S FAO-Mon] |
| Poids | 45–50 kg [S BSE][S SK] ; 40–45 à la lutte [S ZK] | 55–65 [S BSE] ; 60–70 [S SK] ; 85–90 chez les meilleurs [S WP-RU] |
| Cornes | aucune (parfois des moignons) | « en général cornus » [S BSE][S SK] : 70–80 % en Turquie [S FAO-Tur], la moitié en Afghanistan [S Musavi] ; spirale ouverte, section 6.2 |
| Tête | longue, fine, busquée | plus forte, chanfrein plus nettement busqué [S FAO-Afr] |
| Encolure et poitrail | fine | **crinière** de longues mèches grises sur l'encolure et le poitrail [P : P10, P12] |
| Queue | plus petite | plus grosse, comme chez tous les moutons à queue grasse [E, d'après S Epstein-WAR] |
| Dimorphisme | | « marqué, les mâles bien plus grands » [S FAO-Afr] |

**Le bélier gris (« shirazi ») :** la couleur grise est un mélange de fibres noires et blanches dès la naissance [S Buzu] ; elle est **dominante et létale à l'état homozygote** : les agneaux gris nés de deux parents gris meurent dans leurs premiers mois [S FAO-Afr][S BRE*]. On croise donc gris × noir. Un bélier gris adulte : tête et pattes noires, toison gris bleuté à argent, plus clair qu'une brebis noire grisonnante [P : P4, P10, P12]. Il ferait un bon second animal.

### 1.4 L'agneau

- **Grand à la naissance :** 4 à 4,5 kg, environ 10 % du poids de la mère [S ZK][S SK] ; 4,7–5,0 kg en Moldavie [S Buzu] ; 4,23 kg en Angola [S FAO-Afr].
- **Haut et long :** « relativement grand et allongé, l'encolure longue, le dos allongé, la croupe longue et effilée ; la croupe 1 à 2 cm plus haute que le garrot » ; tête longue, sèche, au profil busqué ; oreilles longues et pendantes [S Buzu]. Pattes très longues pour le corps [P : P15, P16].
- **La queue de l'agneau** : « large à la base, avec un dépôt de graisse en forme de **petit курдюк**, et la pointe fine **courbée en S**, qui atteint les jarrets » [S Buzu]. Sur la photo P2, la queue d'un agneau qui tète monte d'abord, s'arrondit vers l'arrière puis retombe en crochet [P].
- **Le pelage (la « smouchka ») :** noir brillant pour la plupart ; poil serré, élastique, soyeux, de 6 à 13 mm à la naissance [S Buzu], qui forme des **boucles** de formes nommées : le **valiok** (валёк, « rouleau » : une boucle allongée en tube couché), le **bob** (« fève » : un rouleau court), la **grivka** (« petite crinière » : une crête étroite), et, moins prisés, l'anneau, le demi-anneau, le « pois », le « tire-bouchon » [S BSE-K][S ZK]. Les rouleaux font de 12 à plus de 30 mm de long, de moins de 4 à plus de 8 mm de large (5 à 10 mm chez les gris), de moins de 4 à plus de 6 mm de haut ; ils courent **en rangées parallèles** sur le dos et la croupe, plus petits et plus désordonnés sur les flancs, le ventre, les pattes et la tête [S ZK][S Buzu].
- **Face, oreilles, pattes : poil lisse et luisant**, sans boucles [S Nam1].
- **En Namibie (Swakara)** : on a sélectionné à partir de 1948 un pelage **plat, moiré, sans boucles en tube**, aux ondes courtes et serrées [S WP-DE][S FW] ; l'agneau Swakara nouveau-né paraît donc **lisse, vernis, aux ondes plates** (P15), pas bouclé comme l'agneau d'Asie centrale (P16, P9).
- **Ça ne dure pas :** au bout de quelques jours les boucles s'ouvrent ; à 10–40 jours le poil est long (3 à 5 cm au cou) et les boucles lâches ; à 2–4 mois, une toison haute, souple, frisottée en anneaux ou en tire-bouchons [S ZK]. La couleur noire vire ensuite au brun ou au gris bleuté puis grisonne [S Nam1][S TLC].
- Les agneaux naissent aussi gris, *sur*, bruns, blancs, rouans [S BSE][S TLC][P : P17, P18].
- **Si l'on dessine un agneau** : un agneau de quelques semaines, noir, au pelage frisé serré, aux pattes longues, aux oreilles pendantes, à la petite queue grasse en S [S Buzu][P]. C'est plus parlant qu'un agneau nouveau-né, dont les boucles se perdent à petite taille.

---

## 2. Tailles et proportions

### 2.1 Les chiffres

| Mesure | Brebis | Bélier | Sources |
|---|---|---|---|
| Hauteur au garrot | **66,5 cm** (n = 50, Turquie) ; 66,5 (Mongolie) ; 66,4 (Afghanistan) ; fourchette 62–70 | 75,2 (Mongolie) ; 67,3 (Afghanistan) ; fourchette 72–78 | [S FAO-Tur][S FAO-Mon][S Musavi][S SK] |
| Hauteur à la croupe | 67,3 cm (≈ 1 cm de plus qu'au garrot) | 69,3 | [S Musavi] ; agneaux : +1–2 cm [S Buzu] |
| Longueur du corps (oblique) | **66,3** (Turquie) ; 60,3 (Mongolie) | 72,5 (Mongolie) | [S FAO-Tur][S FAO-Mon] ; l'étude afghane donne 45 cm, sans doute mesurés autrement (section 10.3) |
| Profondeur de poitrine | **29,7** | | [S FAO-Tur] |
| Largeur de poitrine | 17,2 | | [S FAO-Tur] |
| Tour de poitrine | **82,8** ; 86,1 ; 80,4 | 95,2 ; 83,4 | [S FAO-Tur][S FAO-Mon][S Musavi] |
| Tour du canon | 7,5 | | [S FAO-Tur] |
| Longueur d'oreille | 12,8 (Afghanistan) ; « jusqu'à 15–18 cm » | 13,1 | [S Musavi][S SK] |
| Longueur de la tête | 16,4 (Afghanistan, voir 10.3) ; ≈ 21–22 sur photo | 16,4 | [S Musavi] ; [P] |
| Longueur de l'encolure | 32,7 (Afghanistan) | 33,8 | [S Musavi] |
| Poids adulte | 45–50 kg ; 46 (Mongolie) ; 41 (Botswana) ; 38–42 (Turquie) ; 45–68 (États-Unis) | 55–70 ; 65 (Mongolie) ; 54 (Botswana) ; 80–100 (Swakara) ; 79–102 (États-Unis) | [S BSE][S SK][S FAO-Mon][S FAO-Afr][S FAO-Tur][S TLC][S Nam2] |
| Queue grasse (poids) | 2–3 kg (Mongolie) | | [S FAO-Mon] |
| Longueur, largeur de la queue | **aucune source trouvée** pour le Karakul | | pour comparer, Awassi : brebis 18 × 15–16 cm, bélier 30 × 25 cm [S Epstein-WAR] |

### 2.2 Ratios de dessin (brebis, H = 66 cm)

Les rapports « os » viennent des mensurations ci-dessus ; les rapports « silhouette » des photos de profil P1 (brebis grise en toison, Nouvelle-Zélande, tournée vers la gauche, 300 px de large), P2 (brebis Swakara tondue, tournée vers la droite) et P7 (bélier américain tondu, tourné vers la gauche). Sur P1, je compte 8 cm de toison sur le garrot [E] : H + 8 cm = 176 px, soit 2,4 px/cm.

| Mesure | En H | En cm | Étiquette |
|---|---|---|---|
| Hauteur à la croupe (reins compris) | 1,01–1,03 | 67–68 | [S Musavi] ; la ligne du dos culmine **aux reins** [S AKSR-LJ][S HF][P : P1] |
| Longueur du corps (os), pointe de l'épaule → ischion | **1,00** | 66 | [S FAO-Tur] ; [P] P2 : 0,95, P7 : 1,0 |
| Silhouette en toison, poitrail → fesse (queue comprise) | 1,40–1,47 | 92–97 | [P] P1 (232 px) |
| Profondeur de poitrine | **0,45** | 30 | [S FAO-Tur] ; [P] P2 : 0,47, P7 : 0,46 |
| Patte sous la poitrine (os) | **0,55** | 36 | [S FAO-Tur] (66,5 − 29,7) ; [P] P2 : 0,53 |
| Patte visible sous la frange du ventre, en toison | 0,36–0,40 | 24–26 | [P] P1 (63 px) |
| Hauteur de la toison, dessus → frange du ventre | 0,60–0,65 | 40–43 | [P] P1 |
| Largeur de poitrine | 0,26 | 17 | [S FAO-Tur] |
| Tour de poitrine | 1,25 | 83 | [S FAO-Tur] |
| Tête, LT | **0,32–0,33** | 21–22 | [P] P2 (120 px pour H = 380 px), P1 |
| Oreille | **0,19–0,23** (≈ 0,6–0,7 LT) | 12,5–15 | [S Musavi][S SK] ; [P] P2 : 0,15 (oreille repliée en partie) ; P13 : l'oreille pend jusqu'aux naseaux |
| Encolure (garrot → nuque, ligne du dessus) | 0,35–0,40 | 23–26 | [P] P2 (0,33), P1 ; l'étude afghane donne 0,49 (mesurée sans doute de la nuque au garrot en suivant la courbe) |
| Canon antérieur, largeur visible (poil compris) | ≈ 0,05 | ≈ 3,5 | [E] ; tour du canon 7,5 cm [S FAO-Tur] |
| Racine de la queue au-dessus du sol | 0,85–0,88 | 56–58 | [P] P7, P20 |
| Bas du coussin de graisse au-dessus du sol | 0,50–0,58 | 33–38 | [P] P7 (0,58), P20 (0,52) |
| Pointe de l'appendice au-dessus du sol | ≈ 0,32–0,36 (au jarret) | 21–24 | [S FAO-Mon : « touche la pointe des jarrets »][S Buzu][S WP-RU : « descend sous le jarret »] |
| Saillie de la queue en arrière des cuisses | 0,08–0,12 | 5–8 | [P] P7, P20 |
| Largeur de la queue (vue de derrière) | ≈ 0,25–0,30 | 16–20 | [E] ; « plus étroite et plus longue que celle du White Karaman » [S FAO-Tur] ; P19 de dos |

**La silhouette :** un mouton **haut sur pattes** (« высоконоги » [S SK] ; la patte sous la poitrine fait plus de la moitié de H), **long et étroit** [S TLC][S LRZ], au corps à peu près aussi long que haut, une encolure fine et assez longue portée haut [S TLC][S FAO-Mon], et derrière, la masse arrondie de la queue. Ce n'est **pas** un mouton trapu de boucherie ni une boule de laine.

- **La ligne du dessus :** un léger creux derrière le garrot, la ligne qui **monte vers les reins** (point le plus haut), puis une **croupe en pente** marquée qui se fond dans la queue « attachée bas » [S AKSR-LJ][S HF][S RBT-AU] ; la FAO décrit pour l'Afrique un garrot « saillant, au niveau du sacrum » et un dos « long et légèrement creusé » [S FAO-Afr]. Les deux disent la même forme : garrot net, dos un peu ensellé, reins hauts, croupe qui tombe.
- **Le ventre :** chez la brebis, légèrement pendant, couvert de laine [S FAO-Mon].
- **La tête** paraît longue et étroite au bout de l'encolure [S TLC][S Nam1].

### 2.3 La queue grasse, vue de profil

C'est le trait à réussir. Ce que disent les textes :
- « Queue à grand dépôt de graisse, qui se termine par un **appendice maigre en forme de S** » [S BSE] ; « large, grasse, au bout **courbé en S** » [S SK] ; « très grasse, avec une portion terminale fine » [S FAO-Afr] ; « longue, descend sous le jarret » [S WP-RU].
- La graisse est « à la base de la queue et le long des 3 à 5 premières vertèbres » [S TLC] ; « confinée à la partie haute, de chaque côté et autour de l'os de la queue ; elle ne doit pas s'étendre au bas de la queue, ni à la croupe et aux cuisses ; **le bas de la queue doit être mince** et se raccorder **droit ou en S** » [S AKSR-LJ].
- « Queue large, **attachée bas**, à base en S » [S RBT-AU] ; « croupe en pente qui se fond dans une queue large attachée bas » [S AKSR-LJ].
- Mongolie : « queue grasse de taille moyenne, 2 à 3 kg, qui **touche la pointe des jarrets** » [S FAO-Mon]. Turquie : « plus étroite et plus longue que celle du White Karaman » [S FAO-Tur]. Afghanistan : « pendante, près du sol, le bout de la queue courbé comme un S » [S Musavi].
- Agneau : « large à la base, petit курдюк, pointe fine en S qui atteint les jarrets » [S Buzu].
- **Couleur :** « le bout de la queue garde la couleur de l'agneau nouveau-né » [S FAO-URSS] ; la queue des adultes est noire [S FAO-Tur].

**Ce que j'en tire pour le dessin, flanc gauche, animal tourné vers la droite** (la queue est donc à gauche de l'image) [P : P7 tondu, P20, P1 et P6 en toison, P19 et P2 pour les agneaux][E] :

1. **Attache :** au bout de la croupe en pente, **sous** le niveau du dos, à 0,85–0,88 H du sol. Pas de queue dressée, pas de « poignée » au-dessus de la croupe.
2. **Le coussin de graisse :** il prolonge la courbe de la fesse et **pend verticalement** contre l'arrière des cuisses. En profil, c'est une **goutte** ou un **oreiller suspendu** : bord arrière **convexe**, qui ressort de 5 à 8 cm (0,08–0,12 H) derrière la ligne des fesses, **plus épais dans son tiers haut**, qui s'arrondit en bas vers 0,50–0,55 H du sol, c'est-à-dire **au-dessus du jarret et sous le grasset**. Son bord avant se perd dans l'ombre entre les cuisses.
3. **L'appendice :** du bas du coussin, au milieu, sort la **queue maigre** (l'os nu de graisse), large de 2 à 3 cm [E], couverte de poil ou de laine courte, **noire**. Elle fait un **S** : elle part un peu vers l'arrière, se recourbe vers le bas, et sa pointe revient légèrement vers l'avant ou vers le haut, à **hauteur du jarret** (0,32–0,36 H). Longueur dépliée ≈ 12–15 cm (0,2 H) [E].
4. **En toison :** les mèches de la fesse et du haut de la queue retombent par-dessus ; on voit alors à l'arrière une **bosse arrondie** qui dépasse de la ligne des cuisses et descend plus bas qu'elles, puis le petit appendice noir en dessous [P : P1, P6]. Il faut garder cette bosse visible : sans elle, c'est un autre mouton.
5. **De dos** (pour mémoire, P19) : une masse large comme le bassin à la racine, deux lobes de graisse de chaque côté de l'os, qui se resserre vers le bas et d'où pend l'appendice [S TLC][P].
6. **En mouvement :** lourde, elle ne bat pas ; au pas, elle se balance un peu de droite à gauche avec le bassin ; au trot et au galop, elle rebondit avec un léger retard [E]. Elle ne se lève pas (pas de queue relevée en alerte).
7. **Ne pas dessiner** de queue coupée : certains Karakuls américains et néo-zélandais sont écourtés, et la graisse passe alors sur la croupe [S AKSR-LJ][S RB-NZ].

### 2.4 L'encolure

- **Longueur :** environ 0,35–0,40 H jusqu'à la nuque [P : P1, P2] ; fine, ronde [S FAO-Mon : « mince, longue »][S TLC : « longue, portée mi-dressée »] ; la FAO pour l'Afrique la dit « courte et épaisse » (section 10.3). Pas de fanon chez la brebis ; des **pendeloques** (*wattles*) chez quelques bêtes [S TLC][S AKSR-LJ] : on ne les dessine pas.
- **Au repos, debout :** l'axe de l'encolure monte à **25–35° au-dessus de l'horizontale** ; la nuque dépasse un peu le garrot ; le chanfrein descend à 45–55° sous l'horizontale (P2 : encolure à 29°) [P].
- **En alerte (la pose choisie) :** encolure à **45–60°**, tête haute, chanfrein à 30–40° sous l'horizontale, oreilles un peu écartées de la tête mais toujours pendantes [P : P6, P1, P5].
- **En broutant :** encolure à **45–60° sous l'horizontale**, museau au sol, antérieurs droits [P : P21, Emil Brass P18].
- **En broutant haut :** le Karakul est un **brouteur de buissons** : « il peut atteindre aussi haut qu'une vache » [S AKSR-LJ, citant les éleveurs de 1917] ; encolure tendue vers le haut, éventuellement dressé sur les postérieurs contre une branche [E].
- **Le cou en deux segments** (pour le moteur) : la base (épaule → milieu) bouge peu ; la partie haute (milieu → nuque) fait l'essentiel de l'angle [E]. La toison de l'encolure est plus courte et plus noire que celle du corps (le noir de la tête descend en fondu sur la gorge) [P : P3, P4].

### 2.5 Les articulations des pattes

| Articulation | Hauteur | Angle debout | Étiquette |
|---|---|---|---|
| Coude (antérieur) | ≈ 0,55 H, caché sous la toison | | [S FAO-Tur] (profondeur de poitrine) |
| Carpe (genou) | **0,25–0,30 H** | presque droit, ≈ 175–180° | [P] P2 (0,31, un peu de perspective), P1 (≈ 0,2, flou) ; [E] |
| Boulet antérieur | ≈ 0,07 H | paturon à **55–60°** sur l'horizontale | [P] P2 ; [E] |
| Grasset | ≈ 0,60 H, caché sous la toison | | [E] |
| Jarret (pointe) | **0,32–0,36 H** | **140–150°** | [P] P1 (≈ 0,35), P7 (≈ 0,36) ; [E] |
| Boulet postérieur | ≈ 0,08 H | paturon à ≈ 55° | [E] |

- Les pattes sont **longues, à os léger, aux pieds petits et durs** [S HF][S TLC] ; « membres de longueur moyenne, relativement bien musclés » [S FAO-Afr] ; la constitution est « sèche et solide, l'ossature forte » [S ZK].
- La laine descend jusqu'au haut de l'avant-bras et jusqu'au-dessus du jarret ; plus bas, **poil ras noir** [S Nam1][S Musavi : « tête, oreilles, face et pieds couverts de poil »][P].

---

## 3. La toison

### 3.1 Ce qu'elle est

- **Grossière, en mèches** (« косичного строения ») [S BSE-K][S BSE] ; souvent **double** : un duvet fin dessous, des jarres longs et lustrés dessus [S TLC][S Nam1][S WP-EN] ; certaines bêtes n'ont qu'une couche de longues mèches ondulées [S AKSR-LJ][S FarmShow].
- **Diamètre :** 30–35 µm (Turquie) [S FAO-Tur] ; 25–36 µm, duvet dans les vingt, jarres dans les trente ou plus [S TLC-fib] ; 35 µm [S RBT-AU].
- **Peu de suint, sans frisure** (« no crimp »), **lustrée**, légère et volumineuse [S Nam1][S TLC][S AKSR-LJ] ; feutre très facilement : on dit que l'art du feutre est né de cette laine [S Nam1][S TLC].
- **Mèches :** ouvertes, **larges à la base, effilées vers la pointe** (signe de la double toison) [S TLC-fib].

### 3.2 À quoi elle ressemble

- **Longues mèches pointues, ondulées en vagues lâches**, qui **pendent** sur le flanc ; un yearling tondu donne des mèches de 25 cm « bouclées au bout » [S FarmShow][P : P6, P4, P8].
- Ce ne sont **pas** des boucles serrées (c'est l'agneau qui a les boucles) ni un bloc dense et court comme chez le Romanov (section 9).
- Le bord de la silhouette est **effiloché**, surtout sous le ventre et sur la croupe [P : P1, P6].
- Les **pointes** brunissent ou roussissent au soleil ; la base des mèches reste sombre : vu de près, le flanc est **rayé** de clair (pointes) et de sombre (creux) [P : P3, P6].
- Après la tonte, l'animal paraît lisse, très haut sur pattes, et la queue grasse se voit nettement [P : P7].

### 3.3 Longueur et saison

- **Deux tontes par an** (printemps et automne) en Asie centrale et en Afghanistan (mai et septembre) [S Musavi][S SK][S BSE] ; deux fois au moins aux États-Unis pour éviter le feutrage [S AKSR-LJ].
- **Pousse :** mèche de 15–18 cm (Turquie) [S FAO-Tur] ; 8–18 cm (Mongolie) [S FAO-Mon] ; de 3,6 à 20 cm [S SK] ; 15–30 cm par an (américains, une tonte) [S TLC-fib][S TLC].
- **Pour notre brebis de septembre** (quatre mois de pousse) : **8 cm** sur le flanc, 6 cm sur le dos, 10 cm en frange sous le ventre et sur les cuisses [E].

### 3.4 Ce qui reste ras

- **Tête, oreilles, face et bas des pattes à poil ras** [S Musavi][S Nam1] ; chez la brebis, la face est nue jusqu'au front, parfois avec quelques mèches roussies sur le haut du crâne [P : P13, P14].
- Un type « grossier » (*ak-gul*) a la tête, les pattes et le ventre « peu couverts de laine » [S FAO-URSS] ; d'autres textes autrichiens parlent d'une tête « bewollt » ; les photos d'Asie centrale montrent une face nue (section 10.3).

---

## 4. Les couleurs

### 4.1 Le motif

**Ce que disent les sources :**
- « Avec l'âge, les moutons noirs **grisonnent** ; seule la couleur de la tête et des pattes reste inchangée. » [S BSE]
- « La toison est noire chez l'agneau et **gris foncé** chez l'adulte. La **tête, les pattes et la queue** des adultes sont noires. » [S FAO-Tur]
- « La couleur commence à virer au **brunâtre ou au gris bleuté**, et grisonne avec l'âge. » [S Nam1] ; « la plupart naissent noirs et pâlissent vers le gris » [S TLC] ; « la plupart sont plus sombres à la naissance et s'éclaircissent avec l'âge » [S AKSR-LJ].
- Afghanistan (n = 110) : tête le plus souvent noire ou grise, corps surtout gris ou mêlé, pattes noires dans un tiers des cas [S Musavi].
- Allemagne : « les pattes sont noires du genou vers le bas » [S WP-DE].

**En profil, tourné vers la droite :**
- **Tête et oreilles :** noires, à poil ras, du bout du museau à la nuque [S BSE][P : P3, P4, P6].
- **Encolure :** le noir de la tête **descend en fondu** sur le haut de l'encolure et la gorge, sur 10 à 15 cm, puis les mèches deviennent gris brun [P : P3, P4, P6]. Pas de collier net.
- **Corps :** gris brun sombre, plus clair et plus chaud sur le dos et la croupe (pointes roussies), plus sombre sur le bas du flanc et le ventre, où la base noire des mèches se voit [P : P6, P3, P1].
- **Pattes :** noires sous la laine [S BSE][S FAO-Tur][P].
- **Queue :** coussin couvert de la toison du corps, **appendice noir** [S FAO-Tur][S FAO-URSS][P].
- **Pas de taches blanches** sur la tête, sauf une petite étoile chez certaines bêtes (P4) [P] : on n'en dessine pas.
- **Variantes** (toutes réelles, à ne pas mélanger sur un même animal) : gris bleuté (lignée grise) [P : P4, P10], brun roux (*kambar*, *sur*) [P : P3, P17], blanc à tête noire en Namibie (P2) [S FW : noirs, blancs, « tachetés »].

### 4.2 Comment les valeurs ont été mesurées

Moyennes de zones de 200 à 400 000 pixels (PIL) ; ombre = moyenne des pixels entre le 5ᵉ et le 15ᵉ centile de luminance, lumière = entre le 85ᵉ et le 95ᵉ. Les photos ont des dominantes : P4 (Akron) bleue, P12 (Nouvelle-Zélande) magenta, P6 et P8 (tirages scannés) chaudes, P2 (contre-jour) violacée. P3 (Turkménistan, lumière de novembre, ciel voilé) est la plus neutre pour les adultes d'Asie centrale.

| Photo | Zone | Moyenne | Ombre | Lumière |
|---|---|---|---|---|
| P3 (troupeau, Turkménistan) | toison grise, haut du flanc | `#A49983` | `#6B614E` | `#CFC3A9` |
| P3 | toison grise, milieu du flanc | `#8C816B` | `#403725` | `#DCD0B3` |
| P3 | toison sombre (brebis de droite) | `#554735` | `#1F1406` | `#9B876D` |
| P3 | toison rousse (variante *kambar*) | `#886844` | `#33200E` | `#E0B883` |
| P3 | tête noire (deux bêtes) | `#1A1916`, `#121211` | `#060606` | `#2C2A27` |
| P3 | patte noire | `#26221A` | `#060302` | `#786B52` (poussière) |
| P6 (brebis noire grisonnante, États-Unis) | tête | `#352C28` | `#2C2220` | `#4A4139` |
| P6 | toison, épaule (pointes claires) | `#6D5342` | `#4D372B` | `#8E725C` |
| P6 | toison, croupe | `#8D735C` | `#5B4231` | `#B89F85` |
| P6 | toison, bas du flanc | `#5F4C3F` | `#3F3128` | `#816D5A` |
| P1 (brebis grise, Nouvelle-Zélande) | toison, haut du flanc | `#A89D96` | `#6D6157` | `#D9D2CC` |
| P1 | toison, bas du flanc | `#B29A84` | `#745B43` | `#E9D1BD` |
| P1 | tête ; patte | `#232A1D` ; `#0E0F0C` | `#010101` | |
| P4 (lignée grise, zoo d'Akron) | toison, lumière | `#717E8E` | `#3F4A57` | `#9EAEC1` |
| P4 | toison, ombre | `#444C55` | `#171A1F` | `#778390` |
| P4 | joue noire ; encolure noire | `#171618` ; `#28292D` | `#111011` | `#3F4045` |
| P2 (brebis noire tondue, Namibie, contre-jour) | flanc ; tête | `#261718` ; `#42302F` | `#201113` | `#543E3B` |
| P10 (tête de bélier naturalisée, Namibie) | corne, spire | `#504846` | `#1A1615` | `#7D7573` |
| P10 | corne, pointe | `#554D4C` | `#0D0C11` | `#8F827F` |
| P10 | chanfrein noir | `#14161F` | `#0E1018` | `#1F202A` |
| P10 | crinière grise du cou | `#63625C` | `#3A382F` | `#8B8D8E` |
| P12 (bélier, Nouvelle-Zélande, dominante magenta) | corne | `#CF9996` | `#AE766D` | `#EEBCBA` |
| P12 | face noire | `#43424D` | `#302F38` | `#555462` |
| P13 (tête, haras du Turkménistan) | chanfrein (reflet du ciel) | `#434D56` | `#272E34` | `#657382` |
| P13 | oreille | `#5D5655` | `#2E2622` | `#888180` |
| P13 | toupet roussi | `#796155` | `#3F332E` | `#B08F7D` |
| P13 | iris (bas de l'œil, 180 px) | `#624F44` | `#413731` | `#7B624F` |
| P15 (agneau Swakara nouveau-né, plein soleil) | flanc | `#38352A` | `#1C190F` | `#5F5D51` |
| P9 (pelage d'agneau, gros plan) | boucles | `#2C2E35` | `#101017` | `#54575F` |

Les yeux de P10 et P11 sont des yeux de verre (têtes naturalisées) : pas de mesure d'iris dessus. La corne du bélier noir P23 est surexposée : non mesurée.

### 4.3 Palette recommandée (brebis noire grisonnante, lumière neutre)

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Toison, dos et croupe (pointes roussies) | `#9C8F7A` | `#CFC3A9` | `#6B614E` ; d'après P3 et P6 |
| Toison, épaule et milieu du flanc | `#7E7462` | `#A99D86` | `#4A4034` |
| Toison, bas du flanc, cuisses, frange du ventre | `#5A4F42` | `#7E705E` | `#2E261D` |
| Creux entre les mèches, base des mèches | `#2E2822` | `#4A4038` | `#16120E` ; traits verticaux fins, plus serrés vers le bas |
| Fondu tête → encolure | `#3A3430` | `#5A524A` | `#1C1816` ; sur 10–15 cm |
| Tête, oreilles (poil ras noir) | `#1A191A` | `#3A3A3E` (lustre, reflet froid sur le chanfrein) | `#0B0B0C` |
| Dessous de l'oreille, peau | `#2A2526` | | pigmentée, jamais rose [P : P13, P5] |
| Nez (peau nue) | `#2A2628` | `#5A5458` (humide) | naseaux `#0B0A0B` ; souvent poussiéreux `#A49B92` [P : P13] |
| Pattes | `#1C1A18` | `#3E3A35` | `#0A0908` |
| Appendice de la queue | `#1C1B1C` | `#3A383A` | `#0C0B0C` |
| Onglons | `#1E1B19` | `#4A4440` (bord usé) | `#0E0C0B` |
| Iris | `#6E5646` brun ambré | `#8C6E52` | cercle sombre `#3A2E26` ; pupille horizontale `#0B0A0A` ; reflet `#F0ECE6` |
| Variante lignée grise (bélier shirazi) | `#717E8E` → neutralisé `#7A7C80` | `#A6A9AE` | `#3F4247` |
| Variante *kambar* / brun roux | `#886844` | `#C49C6A` | `#3A2612` |

---

## 5. La tête

- **Profil :** « tête légère, allongée, **à demi busquée** » (полугорбоносая) [S SK][S BSE] ; « longue et étroite, **légèrement creusée entre les yeux**, souvent busquée » [S TLC][S Nam1] ; « front proéminent » [S HF], « front bombé » [S FAO-Mon]. Chez la brebis : chanfrein droit sur la moitié haute, puis **une courbe douce et continue jusqu'au nez** ; chez le bélier, nettement plus busqué [S FAO-Afr][P : P2, P10, P12].
- **Longueur :** LT ≈ 0,32–0,33 H, soit 21–22 cm [P] ; étroite : largeur au-dessus des yeux ≈ 0,4 LT [E].
- **Museau :** fin, arrondi, la lèvre supérieure fendue ; le bout du nez arrive un peu en avant de la verticale du menton [P : P2, P10].
- **Naseaux :** fentes obliques, courtes [E].
- **Yeux :** « yeux brillants » [S Musavi][S HF : *bright-eyed*] ; **écartés** [S HF] ; placés à mi-hauteur de la tête, au tiers arrière de LT [P] ; iris brun ambré à brun [P : P13] ; **pupille horizontale** (rectangle aux coins arrondis) comme chez tous les moutons ; paupières noires.
- **Oreilles :** le second signe de la race. **Longues, minces, souples, pendantes, tournées vers le bas et un peu vers l'avant** [S Nam1][S HF][S TLC] ; 12,5 à 15 cm, jusqu'à 18 [S Musavi][S SK]. Elles partent juste derrière et un peu au-dessus de l'œil, **tombent le long de la joue** et leur pointe arrive à peu près **au niveau des naseaux ou de la commissure des lèvres** [P : P13, P14, P5]. Forme : un « U » long, plus large au milieu [S TLC]. Quelques bêtes ont de petites oreilles en « V », très courtes (« sans oreilles ») : un défaut génétique connu de la race [S AKSR-LJ][S HotR][S SK].
- **Pas de toupet** de laine sur le front chez notre brebis [P : P5, P6] ; chez certains adultes, quelques mèches roussies retombent sur le haut du crâne [P : P13].
- **Expression :** alerte, « noble » selon les éleveurs [S AKSR-LJ].

---

## 6. Les cornes

### 6.1 Ce que disent les sources

- **Brebis :** « matki komoly » : **sans cornes** [S BSE][S SK] ; parfois des moignons ou de petites cornes (30–40 % en Turquie) [S FAO-Tur][S WP-DE][S TLC][S AKSR-LJ] ; « presque toutes sans cornes » en Afghanistan [S Musavi].
- **Bélier :** « en général cornu » [S BSE][S SK] ; cornes « **légèrement annelées, en spirale vers l'arrière et vers l'extérieur, puis vers l'avant** » [S FAO-Afr] ; « des cornes courtes ou de grandes spirales tournées vers l'extérieur » [S LRZ][S HF] ; béliers sans cornes fréquents dans certaines populations [S Musavi][S AKSR-LJ].

### 6.2 Le bélier, de profil, tourné vers la droite (pour mémoire)

D'après P10 (tête naturalisée, profil strict vers la droite), P12 (bélier vivant, profil vers la droite), P11 (de face), P23 et P24 [P] :

1. **Base :** sur le haut du crâne, en arrière des yeux, les deux bases proches.
2. **Départ :** la corne monte peu, part **vers l'arrière** en arc, passe au-dessus et derrière l'oreille.
3. **Descente :** elle redescend derrière l'œil, à 3–5 cm de la joue.
4. **Retour :** elle passe **sous l'œil** et revient **vers l'avant** ; la pointe se tourne **vers l'avant et vers l'extérieur**, au niveau de la commissure des lèvres ou un peu plus haut [P : P10, P12, P11 où les pointes dépassent nettement la largeur de la tête].
5. **Tours :** **¾ à 1 tour** en profil chez un bélier adulte [P : P10, P12, P24] ; diamètre de la spire ≈ 0,7–0,8 LT [P : P10].
6. **Écart :** la spire s'écarte de la tête en s'enroulant (tire-bouchon ouvert) : de face, les pointes dépassent les joues de 8 à 12 cm [P : P11].
7. **Section** triangulaire arrondie [E] ; **annelures** transversales serrées, nettes sur la spire, lissées vers la pointe [S FAO-Afr][P : P10, P12].
8. **Couleur :** corne **brun gris clair** à **gris brun** (P10 : `#504846`, crêtes `#7D7573`) ; brun rosé clair sur P12 (dominante magenta) ; plus sombre, presque noire, chez certains béliers noirs [P : P23].

---

## 7. Les pattes et les onglons

- **Longues, à os léger**, aux pieds petits et solides [S HF] ; aplombs droits [P] ; « pattes de longueur moyenne à longue » [S TLC].
- **Noires**, à poil ras sous le genou et le jarret [S BSE][S WP-DE][P].
- **Canon fin** (tour 7,5 cm chez la brebis [S FAO-Tur]) ; en dessin, ≈ 0,05 H de large poil compris [E].
- **Onglons :** noirs, petits, pointus, les deux doigts serrés ; ergots sombres juste derrière le paturon [P : P2, P7][E].
- **Marche :** c'est un **grand marcheur** : 18–20 km par jour sur les pâturages clairsemés du désert [S ZK] ; « exceptionnelle mobilité » [S ZK]. Pas long, léger, sur pattes hautes [P : P20, P21].

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Un mouton du désert, qui marche :** 18–20 km par jour pour trouver sa nourriture, eau saumâtre une fois par jour ou tous les deux jours [S ZK] ; onze heures de pâturage par jour en Afghanistan [S Musavi] ; herbes au printemps et en été, feuilles et rameaux en automne et en hiver [S WP-DE].
- **Brouteur agressif et « brouteur de buissons » :** « il peut atteindre aussi haut qu'une vache » [S AKSR-LJ] ; « aggressive grazers and browsers » [S TLC].
- **Très grégaire :** « un fort instinct grégaire » [S TLC] ; menacé, **le troupeau forme un cercle, les agneaux au milieu** [S FarmShow][S HotR].
- **Ne travaille pas avec les chiens** : « ils ne répondent pas aux chiens de berger ; je mène les miens n'importe où avec un seau et un peu de grain » (Deanna Perera, AKSR) [S FarmShow].
- **Indépendant, toujours en alerte**, surtout quand la routine change ; « une bande turbulente, têtue, plus sauvage que les autres moutons » selon certains éleveurs ; confiant avec un berger connu [S AKSR-LJ] ; craintif ou même agressif s'il a été peu manipulé jeune [S HotR].
- **Voix :** plutôt silencieux, mais **tout le troupeau bêle à grand bruit** si le repas est en retard ; brebis et agneau s'appellent longuement avant de se retrouver [S AKSR-LJ].
- **Mères protectrices**, agneaux très viables [S TLC][S AKSR-LJ] ; tout le troupeau protège les agneaux [S HotR].
- **Béliers :** combats tête contre tête « avec un bang de tonnerre » pour la hiérarchie [S AKSR-LJ].
- **Saisons :** lutte hors saison possible, trois agnelages en deux ans [S TLC][S Nam1] ; chaleurs qui commencent le plus souvent de jour [S FAO-Afr] ; un seul agneau en général (105–110 agneaux pour 100 brebis) [S ZK][S FAO-URSS].
- **Endurance :** chaleur, froid, sécheresse ; vit jusqu'à 12 ans, 6–7 ans de carrière [S ZK].

### 8.2 Ce qui vaut pour les moutons en général (voir `docs/animation-animale.md`)

- **Journée :** broutage et rumination, sieste à l'ombre aux heures chaudes (§ 2) ; pour un mouton du désert, la sieste de midi est longue [E].
- **Oreilles :** chez une race à oreilles pendantes, les mouvements d'oreilles se voient peu ; en alerte, elles s'écartent un peu de la tête et se tournent vers l'avant sans se dresser [E] ; en peur, elles se plaquent en arrière (§ 4).
- **Agneaux :** sauts et jeux comme partout (§ 5).

### 8.3 Ce que cela donne dans Mouflon (propositions)

- **Marcheuse :** de longues traversées au pas, plus que le Mannequin ; broute en avançant.
- **Grégaire :** suit vite (`walk` contagieux élevé), se regroupe ; en cas d'alerte forte, les adultes se serrent et **font face** autour des agneaux au lieu de fuir loin.
- **Vigilante mais pas fuyarde :** regarde souvent (`look`), fuit court, se retourne.
- **Brouter haut :** un geste « tendre le cou vers le haut » au bord d'une fenêtre, comme vers une branche [E].
- **Bêler en chœur** à l'heure fixe (un appel contagieux).
- **Queue :** jamais relevée ; elle suit le bassin avec un léger retard.
- **Bélier :** chocs francs entre béliers ; ne se cabre pas (ce trait est celui du bighorn).

---

## 9. Les dix détails, et les erreurs à éviter

### 9.1 Les dix détails qui font un Karakul

1. **La queue grasse en coussin**, attachée bas, qui pend derrière les cuisses, terminée par un **appendice mince en S** à hauteur du jarret [S BSE][S SK][S FAO-Mon][S Buzu].
2. **De longues oreilles pendantes**, tournées vers le bas et un peu vers l'avant, jusqu'au niveau des naseaux [S Nam1][S SK][S TLC].
3. **Une tête longue et étroite, au chanfrein busqué**, légèrement creusée entre les yeux [S BSE][S TLC][S Nam1].
4. **Un corps long et étroit, haut sur pattes** [S SK][S TLC][S HF].
5. **Une ligne du dos qui culmine aux reins** et une **croupe en pente** qui se fond dans la queue [S AKSR-LJ][S HF].
6. **La tête, les pattes et la queue noires** sur une toison grisée [S BSE][S FAO-Tur].
7. **Une toison grossière de longues mèches lustrées**, ondulées, pendantes, aux pointes roussies [S TLC][S Nam1][P].
8. **Une brebis sans cornes** ; le bélier, s'il est cornu, a une spirale ouverte qui revient vers l'avant [S BSE][S FAO-Afr].
9. **Des pattes noires, fines, aux petits pieds** [S HF][S WP-DE].
10. **Des agneaux noirs et luisants**, au pelage en boucles serrées (astrakan), qui grisonnent ensuite [S BSE][S ZK][S Nam1].

### 9.2 Ce qui en ferait une autre race

| Erreur | Ce qu'on obtiendrait | Pourquoi |
|---|---|---|
| Corps **blanc**, tête et cou **bruns**, queue **courte et large en cœur**, à deux lobes séparés par une encoche et une houppe relevée, finissant **au-dessus des jarrets** | un **Awassi** | l'Awassi est « blanc, à tête et pattes brunes » [S WP-Awassi][S Epstein-WAR] ; sa queue est « large et relativement courte, finissant d'ordinaire au-dessus des jarrets », en deux lobes avec une encoche « en cœur » et l'os de la queue qui « remonte » en houppe [S Epstein-WAR] ; le Karakul a une queue plus longue, au **bout en S**, une toison sombre, et des oreilles comparables (15 cm chez l'Awassi) |
| Toison grise **courte, dense, en bloc**, tête noire **marquée de blanc**, **petites oreilles** portées de côté, **queue courte sans graisse** | un **Romanov** | le Romanov naît noir et devient gris comme le Karakul, mais c'est un mouton nordique **à queue courte**, à toison courte et serrée (laine grise et jarres noirs) [S WP-Rom][P : Romanov 1–2] ; on les a croisés à Askania-Nova pour un Karakul plus prolifique [S FAO-URSS] |
| Gris à cape noire, **brebis cornue** (faucille), petites oreilles, **queue courte noire** | une **Heidschnucke** | voir `docs/races/heidschnucke.md` |
| Croupe **grasse en deux fesses**, queue presque absente | un mouton **à croupe grasse** (Edilbaï, Blackhead Persian) | le Karakul est à **queue** grasse [S BSE] |
| Corps blanc, tête et cou noirs **nets**, croupe grasse | un **Blackhead Persian / Dorper** | races voisines d'Afrique australe (le Swakara en a reçu un peu de sang [S Nam1][S FW]) |
| Queue longue et mince, sans graisse | un mouton à queue fine quelconque | « hors standard » [S AKSR-LJ][S Buzu : agneau faible à « queue mince sans graisse »] |
| Queue coupée, graisse sur la croupe | un Karakul écourté (Nouvelle-Zélande, certains américains) | réel, mais pas le type [S RB-NZ][S AKSR-LJ] |
| Pattes courtes noyées dans la laine, corps en boule | un mouton de boucherie | « haut sur pattes » [S SK] |
| Boucles serrées sur un **adulte** | un agneau trop grand, ou un Gotland | les boucles disparaissent en quelques semaines [S ZK] |
| Oreilles dressées ou horizontales | presque toutes les races nordiques | oreilles pendantes [S Nam1][S SK] |
| Tête et pattes grises comme la toison | un mouton gris uni | tête et pattes restent noires [S BSE] |

**Les voisins proches :** l'**Awassi** (autre queue grasse, mouton laitier du Croissant fertile, blanc à tête brune), le **Romanov** (même trajet de couleur, mais nordique à queue courte), la **Heidschnucke** (gris et noir, mais cornue à queue courte), les races de boucherie d'Asie centrale à énorme курдюк, comme la Hissar (Гиссарская), plus massives et plus grasses [E : non vérifié en détail].

---

## 10. Sources, photos, contradictions

### 10.1 Sources écrites (ouvertes)

**Instituts, manuels et encyclopédies d'Asie centrale et de l'URSS**
- **[S BSE]** Grande Encyclopédie soviétique, 3ᵉ éd., article « Каракульская порода » (И. Я. Аверьянов) : https://booksite.ru/FullText/1/001/008/058/977.htm
- **[S BSE-K]** id., « Каракуль » (Н. С. Гигинейшвили ; types de boucles, couleurs des peaux) : https://booksite.ru/FullText/1/001/008/058/974.htm
- **[S SK]** В. В. Соколов, Г. А. Куц, *Мировое овцеводство*, entrée « Каракульская » (garrot, oreilles 15–18 cm, queue en S, deux tontes) ; via la Bibliothèque scientifique agricole centrale (ЦНСХБ) : http://www.cnshb.ru/AKDiL/0014/base/RK_2/000251.shtm
- **[S ZK]** М. Закиров, К. Каримов, *Смушковедение*, Ташкент, Мехнат, 1987 (adaptation, marche quotidienne, couleurs, boucles et leurs mesures, évolution du pelage de l'agneau) ; PDF de la bibliothèque de l'Université vétérinaire de Samarcande : https://arm.ssuv.uz/frontend/web/books/6523cb47be6c5.pdf
- **[S UB2022]** А. А. Уримбетов, Н. А. Бобокулов (Научно-исследовательский институт каракулеводства и экологии пустынь, Самарканд), « Молочность каракульских овец окраски сур каракалпакского породного типа », *Овцы, козы, шерстяное дело* 2022 n° 1 (lu pour vérifier l'institut ; pas de mensurations) : https://wool.timacad.ru/jour/article/download/171/151
- **[S BRE*]** Большая российская энциклопедия, « Каракульская порода овец » : https://bigenc.ru/c/karakul-skaia-poroda-ovets-6ae0e1 — **la page refuse les requêtes (401)** ; je n'en reprends qu'une phrase lue dans l'extrait du moteur de recherche (gène létal « ширази », homozygotes gris morts en 6–8 mois), recoupée par [S FAO-Afr].

**FAO**
- **[S FAO-URSS]** N. G. Dmitriev, L. K. Ernst (dir.), *Animal genetic resources of the USSR*, FAO Animal Production and Health Paper 65, 1989, « Karakul (Karakulskaya) », p. 233–235 : https://www.fao.org/4/ah759e/ah759e20.pdf
- **[S FAO-Afr]** R. T. Wilson, *Small ruminant production and the small ruminant genetic resource in tropical Africa*, FAO Animal Production and Health Paper 88, 1991, « Karakul », p. 157–162 : https://www.fao.org/4/t0376e/t0376e17.pdf
- **[S FAO-Tur]** FAO, *Sheep and goats in Turkey*, ch. 5, « Karakul » (mensurations de 50 brebis d'après Özcan et Batu 1966) : https://www.fao.org/4/ah224e/ah224e05.htm
- **[S FAO-Mon]** G. Batsukh, E. Zagdsuren (Institut de recherche en élevage, Oulan-Bator), « Sheep breeds of Mongolia », FAO (tableau 11, mensurations du Karakul ; queue de 2–3 kg qui touche les jarrets) : https://www.fao.org/4/u4900t/u4900T07.htm

**Études**
- **[S Musavi]** S. A. A. Musavi, A. M. Khadimiyan, A. M. Azimi, « Morphological Characterization of Karakul Sheep in North Part of Afghanistan », *Voice of the Publisher* 8 (2022), 16–25, doi:10.4236/vp.2022.81003 (110 bêtes, 10 troupeaux) : https://www.scirp.org/pdf/vp_2022031416030357.pdf
- **[S Buzu]** I. Buzu, « Particularities of body conformation of the Moldavian Karakul lambs », *Scientific Papers, Series D, Animal Science* LXVI (2), 2023 : https://www.animalsciencejournal.usamv.ro/pdf/2023/issue_2/Art1.pdf

**Éleveurs et associations**
- **[S TLC]** The Livestock Conservancy, « Karakul – American Sheep » : https://livestockconservancy.org/karakul-american-sheep/ · **[S TLC-fib]** fiche laine (D. Robson) : https://livestockconservancy.org/wp-content/uploads/2021/10/TLC-Fiber-profiles-Karakul.pdf
- **[S AKSR-LJ]** Leslie Johnson (J Kambar Farm, éleveuse inscrite à l'American Karakul Sheep Registry depuis plus de 30 ans), « American Karakul Sheep: A Unique Heritage Breed », *Thrifty Homesteader*, 1ᵉʳ juin 2023 (oreilles, cornes, ligne du dos, **queue**, tempérament) : https://thriftyhomesteader.com/american-karakul-sheep/ — ce n'est **pas** le standard officiel de l'AKSR, dont le site (karakulsheep.com) n'a pas répondu.
- **[S FarmShow]** *Farm Show*, « Curly Karakuls May Be The World's Oldest Sheep Breed » (Deanna Perera, AKSR) : https://www.farmshow.com/a_article.php?aid=34850
- **[S HF]** *Hobby Farms*, « Karakul Sheep: Bright-eyed and Broad-tailed », 2009 : https://www.hobbyfarms.com/karakul-sheep-bright-eyed-and-broad-tailed-3/
- **[S HotR]** *Homestead on the Range*, « Karakul », 2025 (cercle de défense, oreilles absentes) : https://homesteadontherange.com/2025/02/03/karakul/
- **[S LRZ]** Lee Richardson Zoo, « Karakul sheep » : https://leerichardsonzoo.org/karakulsheep
- **[S RBT-AU]** Rare Breeds Trust of Australia, « Sheep: Karakul » (d'après MLA 2002) : https://rarebreedstrust.com.au/public/pages/sheep-karakul
- **[S RB-NZ]** New Zealand Rare Breeds, « Karakul » : https://www.rarebreeds.co.nz/karakul.html
- **[S GEH-PM]** GEH, communiqué « Das Deutsche Karakulschaf ist die gefährdete Nutztierrasse des Jahres 2015 », 10 novembre 2014 : https://www.presseportal.de/pm/6600/2876291

**Namibie (Swakara)**
- **[S Nam1]** *The Namibian*, « The Swakara fat-tailed pelt breed » : https://www.namibian.com.na/the-swakara-fat-tailed-pelt-breed/ · **[S Nam2]** « Karakul, the oldest sheep breed » : https://www.namibian.com.na/karakul-the-oldest-sheep-breed · **[S Nam3]** « Swakara sheep – lean meat and quality pelts » : https://www.namibian.com.na/swakara-sheep-lean-meat-and-quality-pelts/
- **[S FW]** *Farmer's Weekly* (Afrique du Sud), « The rise of Swakara farming » : https://www.farmersweekly.co.za/animals/sheep-and-goats/the-rise-of-swakara-farming/

**Encyclopédies et races voisines**
- **[S WP-EN]** https://en.wikipedia.org/wiki/Karakul_sheep · **[S WP-RU]** https://ru.wikipedia.org/wiki/Каракульская_овца · **[S WP-DE]** https://de.wikipedia.org/wiki/Karakulschaf
- **[S Epstein-WAR]** H. Epstein, « Awassi sheep », *World Animal Review* 44 (1982), 11–27 (forme de la queue, oreilles, couleur) : https://www.fao.org/4/p8550e/p8550e00.pdf ; et la monographie FAO de 1985 : https://www.fao.org/4/aj003e/AJ003E04.htm
- **[S WP-Awassi]** https://en.wikipedia.org/wiki/Awassi · **[S WP-Rom]** https://en.wikipedia.org/wiki/Romanov_sheep · https://en.wikipedia.org/wiki/Fat-tailed_sheep

**Cherchées sans succès :** l'American Karakul Sheep Registry (karakulsheep.com, pas de réponse) ; la fiche de l'Oklahoma State University (403) ; les articles ouzbeks de *Problems of desert husbandry* sur inlibrary.uz (mensurations d'agneaux et de brebis, **accès refusé aux scripts**) ; la fiche GEH du Karakul (l'adresse renvoie à une autre race) ; aucune fiche FAO DAD-IS lisible sans JavaScript. **Aucune mesure de la queue du Karakul** (longueur, largeur) n'a été trouvée.

### 10.2 Photos regardées

Toutes téléchargées hors du dépôt (`scratchpad/photos/karakul/`), regardées, et pour la plupart mesurées.

| N° | Page | Ce qu'elle montre |
|---|---|---|
| P1 | https://www.rarebreeds.co.nz/karakul.html (image *karakul4.jpg*) | **brebis grise en toison, de profil strict** (tournée vers la gauche), Nouvelle-Zélande : tête et pattes noires, ligne du dos qui monte aux reins, croupe arrondie, patte haute ; mesurée (petite image, 300 px) |
| P2 | https://commons.wikimedia.org/wiki/File:Namibia_2017-06_(027)_Swakara-Schafe.jpg | brebis Swakara blanche tondue **de profil vers la droite** (oreilles pendantes, profil busqué, proportions) et brebis noire ; **queue en crochet d'un agneau** qui tète ; mesurée |
| P3 | https://commons.wikimedia.org/wiki/File:Turkmen_sheep.jpg | troupeau au Turkménistan (novembre 2016) : toisons grises à tête et pattes noires, une bête rousse ; référence de couleur |
| P4 | https://commons.wikimedia.org/wiki/File:Karakul_sheep_in_Akron_Zoo.jpg | Karakul gris bleuté couché, tête noire, longues mèches ondulées ; couleurs |
| P5 | https://commons.wikimedia.org/wiki/File:Karakul_sheep_in_Akron_Zoo_2.jpg | deux Karakuls (blanc, gris) couchés : oreilles pendantes, profil busqué |
| P6 | https://thriftyhomesteader.com/american-karakul-sheep/ (*Grey-Ewe.jpg*, Anakus Karakuls) | **brebis noire grisonnante**, tête noire, longues mèches à pointes roussies, croupe ; référence de la brebis choisie |
| P7 | même page (*Shorn-Unshorn.jpg*, bélier « Rex », Pine Lane Farm) | même bélier **avant et après la tonte**, de profil : corps réel, **coussin de la queue** derrière les fesses ; mesuré |
| P8 | même page (*Black-Yearling.jpg*) | antenaise noire de trois quarts, mèches longues aux pointes brunes |
| P9 | même page (*Lamb-Coat.jpg*) | gros plan du pelage bouclé d'un agneau (astrakan) ; couleur |
| P10 | https://commons.wikimedia.org/wiki/File:Namibia_2017-06_(012).jpg_Kopf_einer_Swakara-Ramme.jpg | **tête de bélier naturalisée, profil strict vers la droite** : spire d'un tour, pointe vers l'avant, annelures, chanfrein busqué, crinière grise ; mesurée (yeux de verre) |
| P11 | https://commons.wikimedia.org/wiki/File:Namibia_2017-06_(018)_Kopf_einer_Swakara-Ramme.jpg | la même tête de face : écart des pointes, tête noire, crinière |
| P12 | https://www.rarebreeds.co.nz/karakul.html (*karakul5.jpg*) | **bélier vivant, profil vers la droite** : corne en spire d'environ ¾ de tour, face noire, crinière grise |
| P13 | https://commons.wikimedia.org/wiki/File:Studfarm_in_Turkmenistan_-_Flickr_-_Kerri-Jo_(56).jpg | tête de face (Turkménistan) : **oreilles pendantes jusqu'aux naseaux**, toupet roussi, iris ; mesurée |
| P14 | https://commons.wikimedia.org/wiki/File:Studfarm_in_Turkmenistan_-_Flickr_-_Kerri-Jo_(54).jpg | têtes noires et brunes au haras, oreilles |
| P15 | https://commons.wikimedia.org/wiki/File:102_0198_Karakul3.jpg | **agneau Swakara nouveau-né**, noir vernis, pelage plat moiré, pattes très longues ; mesuré |
| P16 | https://commons.wikimedia.org/wiki/File:Karakulschafe_in_Bolivien_(1).jpg | brebis noire primée et **deux agneaux aux boucles serrées** (Bolivie, années 1950) |
| P17 | https://thriftyhomesteader.com/american-karakul-sheep/ (*Red-Ewe-Lamb*, *Brown-Ewe-Lamb*, *Red-Ram-Lamb*, *Roan-Ewe-Lamb*, *Wattles*) | agneaux et jeunes roux, bruns, rouans ; oreilles longues ; pendeloques |
| P18 | https://commons.wikimedia.org/wiki/File:Emil_Brass,_Im_Reiche_der_Pelze,_Seite_681,_In_Deutschland_gezogene_Karakulschafe_mit_L%C3%A4mmern_(Rittergut_Graeden).jpg | Karakuls élevés en Allemagne avec leurs agneaux noirs, qui broutent |
| P19 | https://commons.wikimedia.org/wiki/File:Fettschwanzschafe_Usbekistan.JPG | marché près de Chiroqchi (Ouzbékistan) : **queues grasses vues de dos**, bêtes grises tondues à pattes noires |
| P20 | https://commons.wikimedia.org/wiki/File:%D0%9C%D1%83%D0%B3%D0%B0%D0%BB,_%D0%BE%D0%B2%D1%86%D0%B0_(2).jpg | brebis à queue grasse de la région de Samarcande (race non précisée), **en marche vers la droite** : coussin de la queue en profil ; mesurée |
| P21 | https://commons.wikimedia.org/wiki/File:Fat_tailed_sheep,_Afghanistan,_1976.jpg | troupeau afghan noir, brun, gris, qui broute ; postures de broutage |
| P22 | https://commons.wikimedia.org/wiki/File:Namibia_2017-06_(024)_Swakara-Schafe.jpg et (034), (028), (029), (038) | troupeaux et agneaux Swakara, queue d'une brebis blanche de trois quarts arrière |
| P23 | https://www.farmshow.com/a_article.php?aid=34850 | bélier noir cornu américain, crinière, cornes sombres |
| P24 | https://homesteadontherange.com/2025/02/03/karakul/ (*karakul.jpg*, *karakul3.jpg*) | gravures anciennes : brebis noire en toison longue, bélier cornu gris de profil vers la droite |
| P25 | https://commons.wikimedia.org/wiki/File:Karakul_sheep_Iz1.JPG | Karakul gris d'un élevage roumain (salon bulgare), oreilles pendantes |
| P26 | https://scz.org/animal/karakul-sheep | brebis brune à longues mèches et son agneau noir (zoo de Sedgwick County) |
| P27 | https://commons.wikimedia.org/wiki/File:Karakulschafe_in_Bolivien_(2).jpg | bête noire à tête noire en très longue toison, de profil vers la droite |
| Romanov 1–2 | https://commons.wikimedia.org/wiki/Category:Romanov_sheep (*Mouton Romanov.jpg*, *VDNH 715*) | comparaison : toison courte et dense, tête noire marquée de blanc, agneaux noirs lisses, queue courte |
| Awassi 1–2 | https://commons.wikimedia.org/wiki/Category:Awassi_sheep (*Awassi sheep Bulgaria 02.jpg*, *Awassi and Assaf type sheep…*) | comparaison : corps blanc, tête brune, **queue courte et large** finissant au-dessus du jarret |
| Planche | https://commons.wikimedia.org/wiki/Category:Karakul_(sheep) | une centaine de vignettes regardées (Namibie 2017, documents anciens de fourrure, Bolivie, Leipzig 1930) |

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Contradiction |
|---|---|
| Garrot | brebis 62–70, béliers 72–78 cm [S SK] ; 66,5 cm pour 50 brebis turques [S FAO-Tur] et en Mongolie [S FAO-Mon] ; en Afghanistan, béliers 67,3 cm seulement, à peine plus que les brebis [S Musavi], sans doute parce que la moitié des « béliers » mesurés étaient jeunes. |
| Longueur du corps | 66,3 cm [S FAO-Tur], 60,3 [S FAO-Mon], **45–46 cm** [S Musavi] : ce dernier chiffre ne peut pas être une longueur oblique d'épaule à ischion pour un mouton de 66 cm ; l'étude afghane mesure autre chose (sans le dire). De même sa « longueur de tête » de 16 cm est trop courte pour la longueur nuque–museau (≈ 21 cm sur photo). Je garde ses autres mesures (garrot, croupe, tour de poitrine, oreille). |
| Poids | brebis 45–50 [S BSE], 40–45 à la lutte [S ZK], 41 [S FAO-Afr], 38–42 [S FAO-Tur], 45–70 Swakara [S Nam2], 45–68 aux États-Unis [S TLC] ; béliers de 54 (Botswana) à 100 kg (Swakara). Le Karakul américain et le Swakara ont été croisés et sélectionnés pour la taille [S AKSR-LJ][S FW]. |
| Profil de la tête | « demi-busqué » [S BSE][S SK], « souvent busqué » [S TLC][S Nam1], « droit ou très légèrement convexe chez les mâles » [S FAO-Afr], « nez droit et front plat, rarement un peu bombé » [S Musavi]. Les photos d'Asie centrale et de Namibie montrent un busqué doux chez la brebis, plus fort chez le bélier. |
| Encolure | « courte et épaisse » [S FAO-Afr] contre « longue, portée mi-dressée » [S TLC] et « mince, longue » [S FAO-Mon]. Les photos montrent une encolure fine, de longueur moyenne. |
| Ligne du dos | « garrot saillant, au niveau du sacrum, dos long et légèrement creusé » [S FAO-Afr] ; « point le plus haut aux reins » [S AKSR-LJ][S HF] ; « dos droit » [S FAO-Mon]. La croupe est 1–2 cm plus haute que le garrot [S Musavi][S Buzu]. |
| Cornes des brebis | « комолые », sans cornes [S BSE][S SK][S Musavi] ; 30–40 % de petites cornes en Turquie [S FAO-Tur] ; « sans cornes ou moignons » [S WP-DE]. |
| Cornes des béliers | « en général cornus » [S BSE] ; 70–80 % [S FAO-Tur] ; la moitié [S Musavi] ; « cornus ou non » aux États-Unis [S AKSR-LJ]. |
| Oreilles | 12,8 cm [S Musavi] ; « jusqu'à 15–18 cm », avec des bêtes à oreilles courtes ou sans oreilles [S SK] ; « de taille moyenne » en Mongolie [S FAO-Mon]. |
| Queue | « longue, sous le jarret » [S WP-RU] ; « touche les jarrets » [S FAO-Mon] ; « près du sol » [S Musavi] ; poids 2–3 kg [S FAO-Mon] ; aucune longueur ni largeur publiée. |
| Couleur des adultes noirs | « grisonnent », tête et pattes inchangées [S BSE] ; « gris foncé », tête, pattes et queue noires [S FAO-Tur] ; « brunâtre ou gris bleuté » [S Nam1] ; « adultes surtout noirs » [S WP-DE]. Les photos montrent toute la gamme, de presque noir (P8) à gris clair (P1). |
| Proportion de noirs | 80 % [S BSE], 75–80 % [S ZK], 87 % en Mongolie [S FAO-Mon], « jusqu'à 85 % » [S WP-RU]. |
| Tontes | deux par an [S SK][S Musavi][S BSE] ; mèche de 15–18 cm en Turquie [S FAO-Tur], 15–30 cm aux États-Unis [S TLC-fib]. |
| Origine | race très ancienne d'Asie centrale [S BSE] ; croisement récent de *danadar* et de moutons à croupe grasse, ou arrivée avec les Arabes au VIIIᵉ siècle [S FAO-URSS] ; « apparue en Syrie-Mésopotamie, établie à Boukhara il y a 1 200 ans » [S FAO-Afr] ; « 1400 av. J.-C. » [S WP-EN][S TLC]. Rien de cela ne change le dessin. |
| Nom | « lac noir » ou « rose noire » (*kara-gul*) [S GEH-PM][S SK] ; d'après Averianov, de *kara-kol*, « écriture négligée » [S SK] ; de la ville de Qorakoʻl [S BSE-K][S WP-EN]. |
| Mensurations de la queue, longueur de mèche par saison, hauteur des articulations | aucune source ; tout ce qui est en 2.2, 2.3 et 2.5 sur ces points est mesuré sur photo [P] ou estimé [E]. |
