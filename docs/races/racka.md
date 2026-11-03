> Fiche de la race « Racka » (le mouton hongrois de la Puszta, *magyar racka juh*, autrefois *hortobágyi racka*), préparée le 3 novembre 2026 pour dessiner un futur `pets/racka` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Racka : fiche pour un dessin de profil

**Noms :** *magyar racka juh* ou *magyar juh* (« mouton hongrois ») en hongrois, *hortobágyi racka juh* depuis les années 1950 ; *Ovis aries strepsiceros hungaricus* [S MJKSZ][S WP-HU][S WP-DE] · Ungarisches Zackelschaf, Zackelschaf (DE) [S LfL-ZAK][S BMLUK] · Racka, Hortobágy Racka (EN) [S WP-EN] · en serbe *Vitoroga žuja* (« la jaune aux cornes torses ») [S Pihler2020]. « Racka » se prononce à peu près « ratsca ».
**Groupe :** les moutons *Zackel* (*racka* en hongrois) d'Europe centrale et des Balkans : à queue longue, à toison mixte (jarre long et duvet), cornus [S LfL-ZAK][S Neubauer2015].
**Rédigée** le 3 novembre 2026, d'après la description officielle de la race dans le programme d'élevage de la Magyar Juh- és Kecsketenyésztő Szövetség (MJKSZ), une étude biométrique serbe de 150 brebis, les fiches de la LfL de Bavière et du ministère autrichien de l'agriculture, des études hongroises (génétique, généalogie, comportement des agneaux) et une soixantaine de photographies regardées, dont une trentaine de près (section 10.2).

**Étiquettes :**
- **[S …]** : une source écrite, nommée en section 10.1.
- **[P]** : observé ou mesuré sur une photo (section 10.2) ; pixels comptés, couleurs moyennées sur de petites zones avec PIL.
- **[E]** : mon estimation (anatomie générale du mouton, recoupée sur les photos). Aucune source ne la donne.
- **H** = hauteur au garrot (os) ; **LT** = longueur de la tête (nuque → bout du museau).

---

## 0. En bref

- **L'animal à dessiner :** un **bélier blanc de cinq ans**, à la **fin de mars**, environ dix mois après la tonte de mai : **toison pleine** en longues mèches cordées qui pendent sur 20 à 25 cm, **tête et pattes fauve roux** à poil ras et brillant, et surtout ses **cornes** : deux **vis droites**, longues d'environ 55 cm, **dressées en V** au-dessus de la tête, chacune vrillée sur elle-même **quatre fois et demie**. Il se tient de **profil strict, tourné vers la droite** (on voit son flanc gauche), tête haute, sur l'herbe rase de la Puszta.
- **En une phrase :** un mouton de taille moyenne, haut et sec sous une toison mixte en cordelettes qui lui tombe presque jusqu'aux genoux, à la tête étroite et fine, aux **petites oreilles horizontales**, à la **longue queue** qui descend plus bas que le jarret, et aux **cornes en tire-bouchon dressées** que n'a aucune autre race au monde [S MJKSZ][S BMLUK].
- **La race :** environ 8 500 brebis inscrites en 2025 (4 485 blanches, 3 979 noires), deux variétés de couleur tenues dans **deux livres généalogiques séparés** depuis le début et jamais croisées [S Lévai2025][S MJKSZ] ; des troupeaux gardés au Hortobágy, au Kiskunság, au Fertő-Hanság, au Körös-Maros et au Balaton-felvidék [S WP-HU][S WP-DE].

**Palette de base** (détails et mesures en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Toison blanche (« blanc sale ») | `#D2C4A8` | `#EFE6D4` | `#8C7B5E`, creux entre les mèches `#4A3E2C` |
| Tête et pattes (fauve roux) | `#B0703F` | `#E2A97A` | `#6E3A1E` |
| Bas des pattes, paturons | `#7A5A3E` | `#A37E64` | `#3A2A1E` |
| Cornes (jaune cire) | `#C2A57C` | `#F2DDB4` | `#6E5434` ; trait sombre de l'arête `#5A4030` |
| Naseaux, lèvres (rose sombre taché) | `#B8918C` | `#E9CFCB` | taches `#3A2A28` |
| Onglons (cire foncée à brun) | `#5E4632` | `#8E7258` | `#2A1E16` |
| Iris (ambre, noisette) | `#A8763E` [E] | `#C99A5C` | cercle `#4A3020`, pupille `#0E0C0A` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 La race, en quelques lignes

- C'est le **mouton de la Grande Plaine hongroise**, la seule race qu'on appelait *juh* (« mouton », mot finno-ougrien) ; les races venues de l'Ouest, puis le mérinos, s'appelaient *birka* [S MJKSZ][S Lévai2025][S munkacsy].
- Origine incertaine : sa présence dans le bassin des Carpates n'est attestée qu'au XVIᵉ–XVIIᵉ siècle par l'archéologie et les textes [S MJKSZ][S Lévai2025]. D'après l'archéozoologue János Matolcsi, les moutons de la conquête (IXᵉ siècle) avaient des cornes vrillées **autour d'un axe horizontal** ; les cornes **en V** n'apparaissent qu'aux XVIᵉ–XVIIᵉ siècles [S WP-HU][S WP-DE][S BMLUK]. Le ministère autrichien en conclut que la forme des cornes est le fruit d'une sélection humaine de plusieurs siècles [S BMLUK].
- Race dominante jusqu'au XVIIIᵉ siècle (124 129 moutons exportés en 1737), elle recule devant le mérinos à partir de Marie-Thérèse ; « en voie de disparition » dès 1903 ; après la Seconde Guerre mondiale, elle ne survit qu'au Hortobágy, d'où son nom des années 1950 [S WP-HU][S munkacsy][S Lévai2025].
- Relèvement : 1 370 bêtes inscrites en 1975, 2 100 en 1985 ; association d'éleveurs fondée en 1983 ; description de race révisée en 2008, toujours en vigueur dans le programme d'élevage de la MJKSZ [S Lévai2025]. En 2024, 4 314 blancs et 3 905 noirs vivants inscrits, chez 48 et 43 éleveurs [S Lévai2025-abs].
- Elle est avec le bœuf gris et le porc mangalica l'un des « symboles nationaux » vivants de la Hongrie [S Zsolnai2023].

### 1.2 Pourquoi la variété blanche

La race a deux variétés, qu'on considère aujourd'hui comme génétiquement distinctes (126 noirs et 128 blancs génotypés : deux groupes nets ; écarts autour du gène *MC1R*, celui de la couleur) [S Zsolnai2023][S Lévai2025].

| | **Blanche (choisie)** | Noire |
|---|---|---|
| Part du cheptel | ~53 % (4 485 brebis en 2025) | ~47 % (3 979) [S Lévai2025] ; « 60 % blancs, 40 % noirs » [S Wovember] |
| Tête et pattes | poil ras du **jaune peau** au **brun rouge brillant** | **noir brillant** |
| Toison | « blanc sale », crème ; parfois une teinte rousse ou lilas pâle sur l'arrière | noire ; **les pointes rougissent au soleil**, puis la toison **grisonne** avec l'âge (*darusodás*, « devenir couleur de grue ») ; mèches rarement aussi longues que chez la blanche |
| Cornes | **jaune cire**, parfois des bandes plus sombres, « mais pas dominantes » | **gris ardoise foncé à noir**, bandes admises |
| Onglons | cire foncée ou brun foncé, rayés admis | ardoise foncé ou noir |
| Muqueuses | lèvres, paupières, palais, naseaux **rose sombre**, souvent tachés de pigment | lèvres, palais, langue **gris sombre** uniforme |
| Agneau | tête, bas du cou, pattes, queue **bruns**, le reste blanc ; ou **entièrement brun** | **noir brillant**, en boucles serrées « presque aussi belles que l'astrakan » |

Source de tout le tableau, sauf mention : [S MJKSZ].

**On dessine la blanche :** son **contraste à trois tons** (toison crème, tête et pattes rousses, cornes jaune cire) se lit au premier coup d'œil, même petit ; les cornes claires se détachent sur un fond sombre comme sur un fond d'herbe. La noire, sur un écran, serait une masse sombre où la vrille des cornes se perd. Le choix inverse (un bélier noir grisonnant) est décrit en 1.5.

### 1.3 Pourquoi un bélier, et lequel

1. **Ses cornes sont le signe de la race.** « Son principal signe distinctif est la forme de ses cornes, unique au monde » [S MJKSZ]. Celles du bélier sont « plus longues, plus épaisses et un peu plus écartées » que celles de la brebis, et doivent mesurer « au moins deux fois la longueur de la tête » ; celles de la brebis, plus que la longueur de la tête [S MJKSZ].
2. **Le profil est fait pour lui.** Le standard décrit lui-même les cornes **vues de côté** : elles « penchent en avant de la ligne prolongée du chanfrein » [S MJKSZ]. Un V dressé de 55 cm au-dessus d'une tête de 26 cm est, de profil, une silhouette qu'aucune autre race du chantier ne produit (section 9).
3. **Une série de photos de référence** montre, de face, de trois quarts et de dessus, un bélier blanc idéal du musée de plein air de Szentendre, le 30 mars 2008 : cornes dressées en V étroit, quatre tours et demi, toison de fin d'hiver en cordelettes (P3 à P6). Les proportions se mesurent sur deux profils stricts : un jeune bélier blanc (P1) et une brebis blanche du Hortobágy (P2).

**L'individu :**
- bélier de **5 ans** : cornes formées (« au moins 4 tours après deux ans » [S MJKSZ]), encore loin de l'écartement des très vieux béliers (section 6.4) ;
- **garrot 72 cm**, **60 kg** (moyenne et milieu de fourchette du standard) [S MJKSZ] ;
- **fin mars**, environ dix mois après la tonte : mèches de **20 à 25 cm** sur le flanc, en cordelettes, qui laissent voir les pattes à partir du genou [S MJKSZ : « jusqu'à 30 cm et plus »][P : P3, P14, P17] ;
- variété **blanche**, tête **fauve roux** franc (le milieu de la gamme admise), cornes **jaune cire** sans bandes sombres marquées ;
- debout, carré, **tête haute** (« port fier », « encolure de cerf » [S MJKSZ]), oreilles horizontales, queue pendante ;
- **ni boucle d'oreille ni cloche** (beaucoup de bêtes réelles en portent ; on ne les dessine pas, voir 8.3 pour la cloche).

### 1.4 L'autre sexe : la brebis

| | Bélier (choisi) | Brebis |
|---|---|---|
| Garrot, poids | 72 cm en moyenne ; 50–70 kg | 66 cm en moyenne ; 35–45 kg [S MJKSZ] |
| Cornes | ≥ 2 LT, ~50 cm ; V de 90 à 110° entre les axes [S WP-HU] | > 1 LT, ~30 cm ; V de 50 à 60° [S WP-HU] ; plus fines, même vrille, 3 à 4 tours [P : P12, P8] |
| Tête | « couverte », plus forte, **chanfrein moyennement busqué** | « fine, sèche », chanfrein droit [S MJKSZ] |
| Corps | ventre cylindrique | ventre plus ample ; mamelle bien développée et nue [S MJKSZ] |
| Toison | 4–5 kg [S BMLUK] ; 3–4 kg [S LfL-ZAK] | 2–3 kg [S BMLUK] ; 1–2 kg [S LfL-ZAK] |

Les brebis sans cornes sont rares (« selten ») [S LfL-ZAK] et **éliminées du livre** en Hongrie (« l'absence de cornes » est un motif d'exclusion) [S MJKSZ]. La brebis de profil (P2) : cornes dressées presque verticales, à peine plus hautes que la tête est longue, fines comme des vis à bois.

### 1.5 La variété noire (en variante)

- **Tête, oreilles, pattes noir brillant**, avec un reflet bleuté sous le ciel [S MJKSZ][P : P9, P10, P11].
- **Toison :** noire chez le jeune ; chez l'adulte, les pointes **roussissent** au soleil d'été et la toison **grisonne** dès la deuxième année (« darusodás ») ; beaucoup de vieilles bêtes deviennent si grises qu'« on peut à peine les dire noires » [S MJKSZ][S WP-HU][S WP-DE]. Sur les photos d'adultes, la **tête, le cou et la bavette restent noirs** et le **corps devient gris acier** à gris brun, plus clair en haut (P10, P11, P13) : un « capuchon » noir qui fond dans le gris. Mèches un peu plus courtes et plus frisées que chez la blanche [S MJKSZ][S BMLUK].
- **Cornes gris ardoise** à brun gris, **onglons noirs** [S MJKSZ][P].
- Un peu de poil blanc « autour de la bouche et sur les pattes » est admis après un an [S MJKSZ].
- Génétique : d'après Wikipédia (hu, de), « le blanc est récessif face au noir » ; les troupeaux noirs donnent souvent des agneaux blancs, jamais l'inverse [S WP-HU][S WP-DE] (non vérifié dans une étude ; voir 10.3).

### 1.6 L'agneau

- **Blanc :** naît la tête, le bas du cou, les pattes et la queue **bruns**, le corps blanc ; ou **tout brun** (« très fréquent ») [S MJKSZ] ; le brun pâlit en fauve et la toison devient crème [S BMLUK][P : P20, roux uni ; P21].
- **Noir :** naît **noir brillant, en boucles courtes très serrées**, « comme un agneau karakul » [S MJKSZ][S LfL-ZAK] (P19, P21).
- Les agneaux sont « forts, extrêmement vivaces, capables dès le jour de leur naissance de faire plusieurs kilomètres avec leur mère » [S MJKSZ]. Bourgeons de cornes visibles dès les premières semaines [S PostAuto : « on les sent déjà chez les petits »].
- En dessin : pattes très longues pour le corps, oreilles horizontales déjà, queue longue et fine, toison bouclée courte (2 à 4 cm) [P : P19, P20, P21][E].

---

## 2. Tailles et proportions

### 2.1 Les chiffres

| Mesure | Brebis | Bélier | Sources |
|---|---|---|---|
| Hauteur au garrot | **66 cm** (moyenne) | **72 cm** (moyenne) | [S MJKSZ][S WP-HU] ; bélier « environ 70 cm » [S WP-DE] ; 60–65 / 65–70 cm [S BMLUK] ; 55 / 60 cm [S LfL-ZAK] (voir 10.3) |
| Hauteur au garrot (Serbie, 150 brebis) | 64,3 cm | | [S Pihler2020, cité par Ružić-Muslić2021] |
| Longueur du corps (pointe de l'épaule → ischion) | 69,6 cm (1,08 H) | | idem |
| Tour de poitrine | 85,3 cm (1,33 H) | | idem |
| Largeur de poitrine | 18,9 cm (0,29 H) | | idem |
| Profondeur de poitrine | ≈ 30,3 cm (0,47 H) | | idem, recalculée d'après les indices (le tableau imprime 37,97 : coquille, voir 10.3) |
| Tour du canon | 8,0 cm | | idem |
| Indices | format 108,5 ; poitrine 62,8 ; compacité 122,7 ; massivité 132,9 ; longueur de patte 52,9 | | idem |
| Poids adulte | **35–45 kg** | **50–70 kg** | [S MJKSZ] ; 40 / 60 kg minimum [S WP-EN] ; 40–50 / 60–80 kg [S BMLUK] ; 40–45 / 55–65 kg [S LfL-ZAK] |
| Cornes (longueur) | ~30 cm ; > 1 LT | ~50 cm ; ≥ 2 LT | [S WP-HU][S MJKSZ] ; minimum 30–38 / 51 cm [S WP-EN] ; « jusqu'à 1 m » chez le bélier [S LfL-ZAK][S BMLUK][S PostAuto] |
| Écartement des cornes (angle entre les axes) | 50–60° | 90–110° | [S WP-HU][S WP-DE] ; « 60 à 90° pour le type hongrois » [S BMLUK] |
| Queue | « longue, attachée bas, dépasse toujours le jarret » | idem | [S MJKSZ] |
| Toison (longueur de mèche) | 25–30 cm | | [S WP-HU][S BMLUK] ; « jusqu'à 30 cm et plus » [S MJKSZ] ; 20,5–30,5 cm [S Wovember] |
| Tête, oreille (longueurs) | **aucune source** | | voir 2.2, mesurées sur photo |

Aucune mesure de bélier n'a été trouvée (recherches en hongrois : « testméret », « marmagasság », « törzshossz », « mellkörméret » ; en anglais et en allemand). La thèse de L. Ledwith (Budapest, 2014) sur la géométrie du crâne et des cornes de la race existe, mais son texte n'est pas en accès libre (section 10.1). Les mesures de bélier ci-dessous sont donc **celles de la brebis serbe, mises à l'échelle**, et recoupées sur photo.

### 2.2 Ratios de dessin (bélier, H = 72 cm)

Méthode : sur P1 (jeune bélier blanc, profil strict, toison courte de quelques centimètres), le dessus de la toison au garrot est à 410 px du sol ; H (os) ≈ 395 px. Sur P2 (brebis du Hortobágy, profil vers la droite, en marche, toison d'octobre), dessus de la toison à 363 px du sol. Les rapports de l'étude serbe donnent la charpente.

| Mesure | En H | En cm | Étiquette |
|---|---|---|---|
| Dessus de la toison au garrot (mars) | 1,12–1,15 | 81–83 | [E] (10 cm de toison sur le garrot, les mèches s'ouvrant en raie le long du dos [P : P3, P14]) |
| Longueur du corps (os), épaule → ischion | 1,08 | 78 | [S Pihler2020] |
| Silhouette en toison, poitrail → fesse | 1,40–1,50 | 101–108 | [P] P1 : 578 px = 1,41 (toison courte) ; P2 : 1,56 (en marche, toison de 5 mois) ; [E] 1,45 en mars |
| Profondeur de poitrine (os) | 0,47 | 34 | [S Pihler2020, indice de longueur de patte] |
| Tour de poitrine | 1,33 | 96 | [S Pihler2020] |
| Hauteur sous le sternum (os) | 0,53 | 38 | [S Pihler2020] |
| Frange du ventre au-dessus du sol (mars) | 0,26–0,30 | 19–22 | [P] P1 : 0,32 (toison courte) ; P2 : 0,33 ; P3, P17 (toison de fin d'hiver) : la frange arrive au-dessous du coude, vers le genou |
| Frange du poitrail et du cou (bavette de mèches) | 0,20–0,24 | 14–17 | [P] P3, P4 : les mèches de l'encolure tombent jusqu'à mi-canon |
| Carpe (genou) au-dessus du sol | 0,20–0,23 | 14–17 | [P] P1, P2 |
| Jarret (pointe) au-dessus du sol | 0,32–0,38 | 23–27 | [P] P1 : 0,38 ; P2 : 0,31 (en marche) |
| Encolure (garrot → nuque, ligne du dessus) | 0,32–0,36 | 23–26 | [P] P1 (0,27, tête baissée), P17 [E] |
| Tête, LT (nuque → bout du museau) | 0,35–0,38 | 25–27 | [P] P1 : 142 px = 0,36 ; P2 : 0,38 |
| Oreille | ≈ 0,10 (≈ 0,28 LT) | 7–8 | [P] P1 ; « plus petites que moyennes » [S MJKSZ] |
| Queue (avec ses mèches) | 0,60–0,75 | 43–54 | [P] P1 : de l'attache (≈ 0,93 H) à la pointe (≈ 0,15 H) ; [S MJKSZ] « dépasse toujours le jarret » |
| Corne, longueur le long de la vis | 0,75–0,85 (≈ 2,1–2,3 LT) | 54–60 | [S MJKSZ : ≥ 2 LT][S WP-HU : ~50 cm][P : P3, P4] |
| Corne, hauteur apparente de profil | 0,55–0,62 (≈ 1,6 LT) | 40–45 | [E] : 0,77 × la longueur pour un V de 80° (voir 6.2) |
| Corne, épaisseur à la base | ≈ 0,08 (≈ 0,23 LT) | ≈ 6 | [P] P4 (70 px pour une tête de 300 px) |

**La silhouette :** un **corps en tonneau étroit**, plus long que haut (1,45 fois en toison), posé sur des **pattes fines et assez hautes**, sous une toison qui ne fait pas un bloc mais un **rideau de cordelettes verticales** ; au-dessus, une **encolure dressée** et une **petite tête étroite** couronnée d'un **V de deux vis** presque aussi haut que l'encolure est longue. La ligne du dessus est droite, le dos « plus tranchant et plus étroit que chez les autres races », la croupe légèrement tombante [S MJKSZ].
- Le corps sous la toison est maigre : « musculature sèche mais élastique », « tronc de race primitive » [S MJKSZ] ; le jeune bélier de P1, à peine laineux, montre un corps profond et des pattes un peu plus longues que la poitrine n'est haute (0,53 H sous le sternum contre 0,47 H de poitrine [S Pihler2020]).
- La toison de mars **double l'épaisseur** du corps vers le bas : de profil, la frange du ventre descend jusqu'à environ un quart de H du sol [P : P3, P14, P17].

### 2.3 L'encolure

- **Longueur :** environ 0,33 H, « moyennement musclée, sans fanon ni plis, de port fier » : le standard l'appelle **encolure de cerf** (*szarvasnyak*) [S MJKSZ]. Sous la toison, on ne voit d'elle que la ligne du dessus et la cascade de mèches qui en tombe [P : P3, P17].
- **En alerte (la pose choisie) :** l'axe de l'encolure monte à **55–65° au-dessus de l'horizontale**, la nuque nettement au-dessus du dos (≈ 0,35 H) ; le chanfrein descend à **40–50° sous l'horizontale** [P : P3, P17, P2 (chanfrein à 49°)]. C'est le « port fier » du standard.
- **Au repos, debout :** encolure à 25–35° au-dessus de l'horizontale, nuque à peu près au niveau du garrot [P : P1 (20°, tête contre un poteau)][E].
- **En broutant :** encolure à **45–55° sous l'horizontale**, chanfrein presque vertical ; les cornes **pointent alors en avant**, presque horizontales, comme une fourche (P14, P15) : c'est l'image la plus étrange de la race et une pose à ne pas oublier.
- **Le cou en deux segments** (pour le moteur) : la base (épaule → milieu) bouge peu ; la partie haute (milieu → nuque) fait l'essentiel de l'angle. Les mèches qui pendent de l'encolure suivent la gravité, pas l'axe du cou : elles restent verticales quand le cou se lève, et se rabattent vers l'avant quand il descend [P : P14][E].

### 2.4 Les articulations des pattes

| Articulation | Hauteur | Angle debout | Étiquette |
|---|---|---|---|
| Coude | ≈ 0,50 H, caché sous la toison | | [E] |
| Carpe (genou) | 0,20–0,23 H | presque droit, 175–180° | [P] P1, P2 |
| Boulet antérieur | ≈ 0,07 H | paturon à 55–60° sur l'horizontale | [P] P1, P3 [E] |
| Grasset | ≈ 0,55 H, caché | | [E] |
| Jarret | 0,32–0,38 H | **145–150°** | [P] P1 : tibia oblique vers l'arrière, canon presque vertical |
| Boulet postérieur | ≈ 0,08 H | paturon à ≈ 55° | [P] P1 [E] |

- **Antérieurs :** couverts de **poil court et brillant** dès le coude ; la laine sur les antérieurs exclut du livre [S MJKSZ]. Le genou et le canon se voient nettement sous la frange [P].
- **Postérieurs :** « très souvent laineux », ce que l'élevage cherche à corriger [S MJKSZ] ; sur P1 et P3, la laine descend jusqu'au jarret, parfois en culotte jusqu'à mi-canon ; plus bas, poil ras fauve. On dessine la laine **jusqu'au jarret** [P][E].
- « Pattes **fines** mais d'ossature solide, sèches, aux tendons d'acier » [S MJKSZ] : canon antérieur large d'environ 0,05 H [E] (tour du canon 8 cm [S Pihler2020]).

---

## 3. La toison

### 3.1 Ce qu'elle est

- **Mixte, en deux couches :** un **jarre long et grossier** (*felszőr*, 60–80 µm) et un **duvet** (*pehelyszál*, 20–40 µm) ; laine de classe E, grossière [S MJKSZ]. Le *Fleece & Fiber Sourcebook* donne 16–30 µm pour le duvet, 40–60 µm pour le jarre, deux tiers de jarre pour un tiers de duvet [S Wovember].
- **Structure :** « en mèches, presque **en cordelettes** (*majdnem zsinóros*), à **ondulation longue** » [S MJKSZ] ; « la laine se feutre près du corps, si bien qu'elle **paraît pendre en boucles** » [S BMLUK][S LfL-ZAK].
- **Ce qu'elle couvre :** « seulement l'encolure et le tronc » ; elle peut tomber « presque jusqu'au sol » ; le ventre est rarement laineux, surtout chez les vieilles bêtes [S MJKSZ].
- Usage : la *suba* (grand manteau de berger en peau retournée), la *kucsma* (bonnet), le feutre [S MJKSZ][S magyarmuseum][S WP-HU].

### 3.2 À quoi elle ressemble

- **Des cordelettes verticales**, épaisses comme un doigt, qui partent de la ligne du dos et tombent droit sur le flanc, chacune ondulée sur toute sa longueur (une onde tous les 3 à 5 cm) et terminée par une **pointe tortillée**, souvent en tire-bouchon [P : P3, P8, P14, P17].
- **Une raie le long du dos :** les mèches s'ouvrent de part et d'autre de l'échine, si bien que la ligne du dos est un sillon et non un dôme [P : P14, P16].
- **Près de la peau,** un feutre plus dense ; les cordelettes se séparent vers le bas : la silhouette est **frangée** et irrégulière en bas, pas lisse [S BMLUK][P].
- **Sur l'encolure et le poitrail,** les mèches sont les plus longues : une **cascade** qui part de sous la gorge et descend en bavette jusqu'au genou (P3, P4, P16). Sur le dessus de la tête, entre les cornes, une **petite touffe de laine** bouclée (P7, P23 ; [S BMLUK : « front laineux, avec un toupet »]) ; la face, elle, reste rase (3.4).
- **Rien de commun** avec la toison serrée en blocs d'un mérinos ni avec les boucles rondes d'un Gotland : la Racka est hirsute, en rideau.

### 3.3 Longueur et saison

- **Une tonte par an** [S BMLUK], au printemps (mai), avant la montée à l'herbe [E : les photos de mars et d'avril montrent des toisons pleines, celles de juin–août des toisons courtes (P15)].
- Longueur annuelle : **25–30 cm** [S WP-HU][S BMLUK], « 30 cm et plus par endroits » [S MJKSZ] ; plus courte chez la noire [S MJKSZ][S BMLUK].
- **Pour notre bélier de fin mars :** 20–25 cm sur le flanc et l'encolure, 10 cm sur le dos (la raie), 25 cm sur la bavette, laine jusqu'au jarret sur les cuisses [E, d'après P3, P14, P17].
- Après la tonte, la Racka paraît maigre et haute sur pattes, avec une laine bouclée courte (P1 en donne l'idée, avec quelques centimètres de repousse).

### 3.4 Ce qui reste ras

- **La tête** : « couverte d'un poil court et brillant ; la face doit toujours être nette » [S MJKSZ] (sauf la touffe de la nuque, ci-dessus).
- **Les antérieurs**, du coude aux onglons : poil court brillant [S MJKSZ].
- **Les postérieurs** sous le jarret ; **la queue** porte de la laine en mèches sur sa face supérieure, puis du poil ras roux vers la pointe [P : P1].
- **Les oreilles** : poil ras, roux dehors, intérieur presque nu, brun rosé [P : P8].

---

## 4. Les couleurs

### 4.1 Le motif (variété blanche)

**Le standard :** « Le poil court de la face et des pattes va du **jaune peau** au **brun rouge brillant**. La toison est, comme on dit, **blanc sale**. On voit aussi des bêtes à teinte **rousse lilas pâle** sur l'arrière du tronc. Les cornes sont **jaune cire**, avec parfois des bandes plus sombres, mais pas dominantes. Les onglons sont **cire foncée ou brun foncé** ; les rayures sont admises. Lèvres, paupières, palais et naseaux **rose sombre**, souvent tachés de pigment. » [S MJKSZ]

**En profil, tourné vers la droite :**
- **Tête :** fauve roux uni du bout du museau à la nuque, oreilles comprises ; un peu plus clair (beige rosé) autour des naseaux et sur les lèvres ; souvent **plus foncé sur les joues et sous l'œil** [P : P3, P4, P8].
- **Limite tête/toison :** la laine crème commence derrière les joues et sous la gorge ; la limite est **fondue**, les premières mèches étant roussies [P : P3, P4].
- **Toison :** crème chaud, plus claire et plus blanche sur le dessus au soleil, **jaunie et roussie** vers le bas et vers les pointes (salissures, suint), avec des creux ombrés brun gris entre les cordelettes [P : P3, P12, P14]. Pas de taches.
- **Pattes :** fauve roux comme la tête, souvent **plus foncé** (brun) vers le paturon [P : P1, P12].
- **Queue :** laine crème dessus, poil roux vers la pointe [P : P1].
- **Variation :** de la face presque crème (brebis de Bernburg, P12) au brun rouge très soutenu (bélier de Szentendre, P3) ; on dessine le **fauve roux franc** du milieu.

### 4.2 Comment les valeurs ont été mesurées

Moyennes de zones de 300 à 45 000 pixels (PIL), avec la moyenne des 10 % les plus sombres (ombre) et des 10 % les plus clairs (lumière). P3 à P6 : soleil de mars, assez chaud ; P12 : ciel couvert, neutre ; P2 : ciel couvert, dominante verte (herbe) ; P8, P9 : soleil d'avril ; P1 a été écartée (bête à l'ombre des arbres, valeurs trop sombres).

| Photo | Zone | Moyenne | Ombre (p10) | Lumière (p90) |
|---|---|---|---|---|
| P3 (bélier, Szentendre, soleil) | face au soleil | `#C6885E` | `#93542F` | `#ECB188` |
| P3 | dos de la toison au soleil | `#EBE2D3` | `#BEAE99` | `#FEFCF8` |
| P3 | flanc (mèches et creux) | `#B39871` | `#5C452B` | `#F3DFBC` |
| P3 | mèches du poitrail à l'ombre | `#3B2F21` | `#16150F` | `#655137` |
| P3 | corne, milieu | `#9F8567` | `#64492C` | `#F6DEB5` |
| P3 | corne, base | `#BEA489` | `#71543C` | `#FDF4DF` |
| P3 | corne, pointe | `#9E8566` | `#604A37` | `#EFD7AC` |
| P3 | patte postérieure | `#594334` | `#211B17` | `#A37E64` |
| P3 | patte antérieure (ombre) | `#2F221B` | `#1E1713` | `#4B332A` |
| P4 (même bête, tête) | face | `#AB6F4A` | `#7B3F21` | `#D69C79` |
| P4 | corne, milieu | `#C3A47A` | `#74532F` | `#FAE3BE` |
| P4 | corne, haut | `#C5A77E` | `#745B3D` | `#FAE1BB` |
| P4 | oreille | `#634233` | `#36190F` | `#9C7D6A` |
| P8 (brebis ?, Túrkeve, soleil) | face | `#DCAE7F` | `#A36E3A` | `#FDDFBC` |
| P8 | joue | `#A7672A` | `#6E3603` | `#DEA26A` |
| P8 | naseaux | `#B19B92` | `#654A3F` | `#F1DED7` |
| P8 | corne, face plate | `#AB8A63` | `#5A3409` | `#FDEACF` |
| P8 | intérieur de l'oreille | `#603721` | `#401602` | `#866957` |
| P7 (Commons, soleil bas) | face | `#A4796B` | `#3E221A` | `#F7CEC0` |
| P7 | naseaux (roses, tachés) | `#AB888D` | `#281116` | `#FEECF4` |
| P7 | corne, bas | `#6E5E5C` | `#2B1A1C` | `#E2C0B8` |
| P7 | touffe de laine de la nuque | `#7A554D` | `#35140E` | `#D3B2AE` |
| P7 | toison du poitrail | `#B5988C` | `#5C3D30` | `#FCE5DB` |
| P12 (brebis, Bernburg, couvert) | face (claire) | `#D3C6B3` | `#8F7B6D` | `#F4EEE1` |
| P12 | face (plus rousse) | `#C2AA8B` | `#7D6243` | `#EBDCC4` |
| P12 | flanc | `#AC9C81` | `#6C5B42` | `#E4D6BC` |
| P12 | flanc (autre bête) | `#988A6E` | `#5E5035` | `#CBBEA4` |
| P12 | antérieur | `#51432D` | `#1E1A0C` | `#816F53` |
| P2 (brebis, Hortobágy, couvert) | flanc | `#747156` | `#3A3A2B` | `#B9B792` |
| **Variété noire** | | | | |
| P9 (brebis noire, Túrkeve) | face | `#282D37` | `#0A0C11` | `#717A8A` |
| P9 | museau | `#252A34` | `#05060A` | `#5E6876` |
| P9 | oreille | `#191918` | `#050403` | `#444246` |
| P9 | corne | `#A69F98` | `#4D463E` | `#E7E1DC` |
| P9 | toison (grisonnante) | `#635E5A` | `#2F2B27` | `#99948D` |
| P10 (bélier noir, LfL) | flanc | `#655C55` | `#2F2A24` | `#A79C94` |
| P10 | encolure | `#4D4A46` | `#1F1E1D` | `#958C84` |
| P10 | bas du flanc | `#322E28` | `#14140F` | `#695E54` |
| P10 | patte | `#292925` | `#111110` | `#676651` |
| P10 | corne | `#6D645A` | `#1D1912` | `#C0B8AF` |
| P11 (bélier noir, couvert, dominante bleue) | face | `#2C3456` | `#151D34` | `#525A86` |
| P11 | flanc gris | `#8286A5` | `#494E6B` | `#B6B9D8` |
| P11 | patte | `#28292E` | `#0C0F11` | `#98937F` |
| P7 | toison d'un jeune noir | `#1A1920` | `#020205` | `#48454D` |

### 4.3 Palette recommandée (bélier blanc, lumière neutre de fin d'hiver)

Valeurs choisies d'après le tableau, en retirant la dominante chaude du soleil bas (P3, P7) et la dominante verte de P2.

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Toison, dessus et haut du flanc | `#DCCFB6` | `#F3ECDF` | `#A8977A` |
| Toison, bas du flanc et pointes (roussies) | `#C2AE8C` | `#E2D3B6` | `#7E6A4E` |
| Creux entre les cordelettes | `#5A4B36` | | `#3A3024` |
| Face, oreilles | `#B0703F` | `#E2A97A` | `#6E3A1E` ; joue et dessous de l'œil `#94542C` |
| Tour des naseaux et lèvres | `#C79C88` | `#E8C9B8` | `#8A6656` |
| Naseaux (muqueuse) | `#B8918C` | `#E9CFCB` | taches de pigment `#3A2A28` |
| Intérieur de l'oreille | `#7A4A34` | `#A07058` | `#4A2414` |
| Touffe de laine de la nuque | `#9A6A4E` | `#C9A486` | `#5A3424` |
| Pattes (haut) | `#A06A42` | `#C99068` | `#5E3A22` |
| Pattes (bas, paturon) | `#7A5A3E` | `#A37E64` | `#3A2A1E` |
| Cornes, faces | `#C2A57C` | `#F2DDB4` | `#6E5434` |
| Cornes, arête et sillons des annelures | `#8C7052` | | `#5A4030` |
| Onglons | `#5E4632` | `#8E7258` | `#2A1E16` |
| Iris | `#A8763E` [E] | `#C99A5C` | cercle `#4A3020` |
| Pupille | `#0E0C0A` | | horizontale |

**Variante noire (bélier adulte grisonnant) :**

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Face, oreilles, pattes | `#26272B` | reflet froid `#5E6676` | `#0B0C10` |
| Encolure et bavette (restent sombres) | `#3C3936` | `#6E6862` | `#1A1918` |
| Flanc grisonnant | `#6A635C` | `#A39A92` | `#2E2A26` |
| Pointes roussies (été) | `#6B4A35` [E] | | |
| Cornes | `#6E665E` | `#A69F98` | `#2E2925` |
| Onglons | `#1E1C1A` | `#4A4440` | `#0C0B0A` |
| Iris | ambre `#B07A3A` [E, d'après P9 vu à l'œil] | | |

---

## 5. La tête

- **Forme :** « crâne allongé, qui s'amincit vers le museau » ; « la plus belle région du corps, c'est sa tête » [S MJKSZ] ; « tête remarquablement **étroite** » [S BMLUK][S LfL-ZAK]. De profil : un **coin long et fin**, le front bas, le museau effilé ; LT ≈ 0,36 H, la hauteur de la tête à l'œil ≈ 0,4 LT [P : P1, P2].
- **Chanfrein :** « moyennement **busqué** » chez le bélier ; droit chez la brebis [S MJKSZ]. Sur P3 et P4, la bosse du chanfrein culmine entre l'œil et le naseau [P].
- **Museau :** fin, lèvres serrées (« les paupières et les lèvres sont tendues ») [S MJKSZ] ; naseaux en fente oblique, rose sombre taché de brun noir [S MJKSZ][P : P7, P8].
- **Yeux :** « grands, brillants » [S LfL-ZAK][S BMLUK], « regard vif, attentif » [S MJKSZ]. Placés haut et sur le côté, à environ 0,4 LT de la nuque [P]. **Iris ambre à noisette**, **pupille horizontale** rectangulaire [P : P4, P9 (ambre net chez la noire)]. Paupières rose sombre chez la blanche [S MJKSZ].
- **Oreilles :** « plus petites que moyennes, plus fines, **portées horizontalement**, mais extrêmement mobiles » [S MJKSZ] ; « dressées en cas de danger » [S BMLUK]. Elles sortent **juste sous la base des cornes**, en arrière de l'œil, et pointent sur le côté, légèrement vers l'avant ; lancéolées, longues d'environ 0,28 LT [P : P4, P8, P12, P23].
- **Toupet :** une petite touffe de laine bouclée sur la nuque, entre les cornes (P7, P23) ; jamais sur le front ni sur la face [S MJKSZ][P].
- **Bélier vs brebis :** tête du bélier « plus couverte, mais pas grossière » ; « dimorphisme sexuel marqué » [S MJKSZ].

---

## 6. Les cornes

### 6.1 Ce que disent les sources

- **Les deux sexes** portent des cornes ; l'absence de cornes exclut du livre en Hongrie [S MJKSZ] ; des brebis sans cornes existent, rares [S LfL-ZAK].
- **Le standard (MJKSZ) :** « La forme la plus belle est, **vue de face**, une corne **à tige droite**, dressée **en V régulier**, **vrillée uniformément sur son axe long**, avec **au moins quatre tours** au-delà de deux ans. **Vue de côté, elle penche en avant de la ligne prolongée du chanfrein.** Les cornes du bélier sont plus longues, plus épaisses et un peu plus écartées. Pour garder la diversité de la race, des formes moins droites (par exemple **en tulipe**) sont admises. L'élevage cherche des cornes plus longues : chez la brebis, plus longues que la tête ; chez le bélier, au moins deux fois la longueur de la tête. » [S MJKSZ]
- **Angles et longueurs :** bélier ~0,5 m, axes à **90–110°** ; brebis 30 cm, **50–60°** [S WP-HU][S WP-DE] ; « en V, 60 à 90° » pour le type hongrois, contre 180° (horizontales) pour le type bosnien [S BMLUK] ; « tenues en **V bas** » [S Neubauer2015] ; « presque droit vers le haut et un peu en dehors » [S WP-EN] ; « à environ 45° » [S WP-EN].
- **Croissance :** toute la vie [S PostAuto] ; jusqu'à 1 m chez de vieux béliers [S LfL-ZAK][S BMLUK][S PostAuto].
- **Couleur :** blanche : jaune cire, bandes sombres admises « mais pas dominantes » ; noire : ardoise foncé à noir [S MJKSZ] ; « une ou deux bandes sombres » chez la blanche [S BMLUK].
- **Défauts :** « graves défauts de cornes » s'écartant de la description = exclusion [S MJKSZ].

### 6.2 Le bélier, de profil, tourné vers la droite (on voit sa corne gauche)

D'après P1 (profil strict, jeune bélier), P2 (profil, brebis), P3 à P6 (bélier idéal de face, trois quarts et dessus), P17 (bélier de trois quarts, vidéo de la MJKSZ) et le standard :

1. **Base :** sur le sommet du crâne, **au-dessus et un peu en arrière de l'œil**, juste devant la nuque ; les deux bases sont **presque jointives** (séparées par la touffe de laine), larges d'environ 6 cm [P : P4, P7].
2. **Axe :** la corne est une **vis à axe droit** (« tige droite ») [S MJKSZ] : pas un enroulement autour de l'oreille comme chez le mérinos ou le mouflon, mais une **tige** qui part du crâne et monte, vrillée sur elle-même. Son axe ne s'incurve qu'à peine : léger cintre vers l'extérieur dans le dernier tiers (P3, P5), pointe parfois un peu ramenée vers l'arrière [P].
3. **Angle avec le chanfrein :** de profil, l'axe de la corne fait **40 à 50° avec la ligne prolongée du chanfrein**, du côté du museau (« elle penche en avant de la ligne prolongée du chanfrein ») [S MJKSZ][P : P1 = 40° ; P2 = 47°]. Avec la tête en alerte (chanfrein à 45° sous l'horizontale), la corne est donc **presque verticale, penchée de 0 à 10° vers l'avant** [P : P1, P17, P2].
4. **Écartement :** vues de face, les deux cornes font un **V de 80°** (40° de chaque côté de la verticale) [choix dans la fourchette 60–110° des sources ; P3 et P4 : 63–71° ; P12 (brebis) : 60–80°]. De profil, ce V se voit à peine : les deux cornes se superposent ; avec une tête très légèrement tournée, la corne droite paraît juste derrière la gauche, un peu plus courte [P : P1, P2].
5. **Longueur :** 55–60 cm le long de la vis (≈ 2,2 LT) [S MJKSZ][S WP-HU] ; vue de profil, le V de 80° raccourcit la corne de cos 40° ≈ 0,77 : **hauteur apparente ≈ 42–46 cm ≈ 1,6–1,7 LT ≈ 0,6 H** [E] ; la pointe monte à environ **1,35 H** au-dessus du sol quand la tête est haute [E].
6. **Section :** **aplatie, en lame**, ovale à presque triangulaire, avec **une arête vive** (la carène) qui suit la vrille ; la face large fait environ deux fois l'épaisseur [P : P3, P5, P7, P8]. C'est cette lame tordue qui fait les « ailettes » qu'on voit à chaque demi-tour.
7. **Vrille :** **4 à 5 tours** chez notre bélier [S MJKSZ : ≥ 4][P : P3, P4 : 4 à 5 passages de l'arête sur la face avant] ; le **pas** est régulier, environ **12 cm par tour** (≈ 0,45 LT), un peu plus serré près de la base, plus lâche vers la pointe [P][E]. À chaque **demi-tour**, la lame se présente alternativement **de face** (large, claire, plate) et **de chant** (étroite, sombre, l'arête en avant) : de profil, la corne paraît donc **renflée puis étranglée** tous les 6 cm environ, comme une flamme torsadée [P : P1, P3, P8].
8. **Sens de la vrille :** les deux cornes sont **en miroir** l'une de l'autre [P : P3, P4, P7]. Le sens exact (dextre ou senestre) n'a pas pu être établi de façon sûre sur les photos, et aucune source ne le donne (10.3). **Pour dessiner sans se tromper :** la corne gauche vue de la gauche est l'image **en miroir horizontal** de la corne droite vue de la droite : reprendre P1 (bélier tourné vers la gauche, corne droite au premier plan) **retournée horizontalement** donne exactement la corne de notre bélier, sens de vrille compris.
9. **Pointe :** fine, lisse, **vers le haut et un peu en dehors** ; chez certains, ramenée légèrement en arrière [P : P3, P5].
10. **Écart à la tête :** la corne ne touche ni l'oreille ni le cou ; de profil, l'oreille sort horizontalement **sous** la base de la corne, en arrière [P : P1, P4].

**Surface :** des **annelures fines et serrées**, presque perpendiculaires à l'axe, **ondulées** (elles suivent les faces de la lame), environ une par centimètre, plus profondes vers la base ; la pointe lisse [P : P3, P5, P7]. De loin, la corne paraît **cannelée en travers** et **torsadée en long** à la fois.

**Couleur :** jaune cire, plus claire et presque blanche sur les faces au soleil, brun miel dans les sillons ; souvent **un trait sombre** (brun gris) qui suit l'arête et tourne avec elle [S MJKSZ][S BMLUK][P : P5, P8] (palette en 4.3).

### 6.3 La brebis

- Même vis à tige droite, **plus fine** (≈ 3,5 cm à la base [E]), **plus courte** (~30 cm, > 1 LT), **3 à 4 tours** [S WP-HU][S MJKSZ][P : P8, P12].
- **V plus étroit**, 50–60° [S WP-HU] ; de profil, presque verticale (P2) [P].
- Variations : certaines brebis ont un V très ouvert (P8 : ~145° !), d'autres des cornes en « tulipe » (pointes qui se rapprochent) [S MJKSZ][P : P8, P12].

### 6.4 Variations chez les béliers (à ne pas dessiner, mais à reconnaître)

- **Les vieux béliers** écartent leurs cornes avec l'âge et la longueur : V de 110 à 140°, cornes presque horizontales vers l'extérieur puis relevées, vrille plus lâche qui devient, vers la pointe, un **tire-bouchon ouvert** (un ressort avec un vide au milieu) [P : P11, P13, P16]. C'est spectaculaire de face, mais de profil ces cornes se raccourcissent beaucoup (on les voit presque en bout) : elles perdent ce qui fait la race en silhouette.
- Les cornes **horizontales**, à peine vrillées, en spirale ouverte large (P22) sont celles du type « bosnien » [S BMLUK] ou des Zackel valaques (section 9) ; sur une Racka hongroise, c'est un défaut.

---

## 7. Les pattes et les onglons

- « Pattes **fines**, mais d'ossature solide, sèches, tendons d'acier. Onglons **durs**, corne élastique, de forme régulière ; en élevage extensif, ils n'ont besoin d'**aucun soin**. » [S MJKSZ] ; « onglons petits et très résistants » [S LfL-ZAK].
- **En dessin :** pattes fauve roux, à poil ras brillant, **minces** et longues sous la frange ; aplombs droits : antérieur vertical sous l'épaule, canon postérieur presque vertical sous la fesse [P : P1, P12].
- **Onglons :** petits, pointus, serrés ; **cire foncée à brun foncé**, parfois rayés de clair chez la blanche ; noirs chez la noire [S MJKSZ][P : P1, P12]. Hauteur ≈ 0,04 H [E].
- **Allure :** « mouvement **rapide, adroit et harmonieux** » [S MJKSZ] ; au trot (P18), les pattes se lèvent haut et la toison ballotte en rideau.

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Craintive et vigilante :** « très sensibles au stress et au bruit, **nettement farouches** ; portées à une **fuite nerveuse**, si bien qu'on les attrape difficilement dehors ; elles **observent leur environnement de très près et restent constamment sur le qui-vive** » [S BMLUK]. La Racka Sheep Society britannique la dit « méfiante envers les menaces » (bruits forts, machines), attentive aux prédateurs, « un peu nerveuse, facile à effrayer, difficile à enfermer ou à attraper » [S WP-EN].
- **Paisible avec l'homme :** « elles n'attaquent pas ; mais attention quand elles **secouent la tête** » (les cornes) [S PostAuto].
- **Vivacité :** « bonne aptitude au pâturage, vivacité » ; « rien qui trahisse un tempérament mou n'est toléré » [S MJKSZ].
- **Troupeau et berger :** élevée « pendant mille ans en plein air, sans bergerie pour l'essentiel » [S MJKSZ] ; en Autriche, pâturage d'avril–mai à novembre, bergerie ouverte l'hiver [S BMLUK]. Au Hortobágy, la saison de pâture s'ouvre au printemps par une fête où les bergers en costume marchent **devant** leur troupeau de Racka (26 avril 2014) [S NBC].
- **Reproduction strictement saisonnière :** lutte de mi-juillet à mi-janvier [S MJKSZ] ; agnelage à la fin de l'hiver (janvier–février), presque toujours **un seul agneau** (jumeaux 5–15 % [S BMLUK] ; 5–40 % selon le milieu [S MJKSZ]) ; agneaux nés dehors, très robustes [S BMLUK].
- **Les agneaux** (cinq agneaux filmés 48 h après la naissance) : couchés 57 % du temps ; tétées courtes et fréquentes (4,8 % du temps) ; debout deux à trois fois plus longtemps qu'en mouvement ; **aucun jeu** le premier jour, presque aucun le deuxième (0,05 %) [S Pajor2016]. Ils suivent leur mère sur des kilomètres dès le premier jour [S MJKSZ].
- **Bonne mère, bonne laitière :** mamelle bien attachée, ~60 L de lait après sevrage (autrefois traite) [S MJKSZ][S BMLUK].

### 8.2 Ce qui vaut pour les moutons en général (voir `docs/animation-animale.md`)

- Rythme de la journée, rumination couchée, sommeil, allures (pas, trot, galop, bonds), jeux d'agneaux, rencontres et coups de tête des béliers : sections 1 à 6 de `docs/animation-animale.md`. Rien dans les sources ne dit que la Racka s'en écarte, sauf sa **vigilance** plus grande et sa **fuite** plus prompte.
- Combats de béliers : aucune source propre à la race. Avec des cornes **dressées** et non enroulées, le choc frontal se ferait cornes croisées en V ; je n'ai trouvé ni photo ni vidéo de combat pour le décrire [E : ne pas animer de choc frontal à la mouflon ; plutôt des poussées tête contre tête et des **secousses de tête** latérales, que signale [S PostAuto]].

### 8.3 Ce que cela donne dans Mouflon (propositions)

- **Vigilance :** des arrêts fréquents, tête levée haut (encolure à 60°), oreilles qui passent de l'horizontale à **dressées vers l'avant** [S BMLUK] ; puis reprise du pâturage.
- **Fuite :** démarrage brusque au trot puis au galop, la toison qui ballotte ; regroupement serré ; regard en arrière après quelques mètres [S BMLUK][E].
- **Brouter :** l'encolure descend, et les cornes basculent **vers l'avant**, presque horizontales (P14, P15) : une pose propre à la race.
- **Secouer la tête** (geste d'humeur, ou pour chasser une mouche) : les cornes balaient l'air ; c'est le geste qui fait reculer les autres [S PostAuto][E].
- **Cloche :** des Racka portent une cloche au cou dans les musées et troupeaux de démonstration (P3, P4) ; une cloche sur le meneur serait juste pour l'imagerie de la Puszta, mais je n'ai trouvé aucune source écrite sur l'usage des cloches dans les troupeaux de Racka (10.3) : la garder en option, et ne pas la dessiner sur l'individu de référence.
- **Agneaux :** très peu joueurs les deux premiers jours (le moteur peut retarder leurs jeux) [S Pajor2016].

---

## 9. Les dix détails, et les erreurs à éviter

### 9.1 Les dix détails qui font une Racka

1. **Deux cornes en vis droite**, chez le bélier comme chez la brebis, **dressées en V** au-dessus de la tête, et non enroulées autour de l'oreille.
2. **Vrille serrée et régulière** (≥ 4 tours chez l'adulte), lame aplatie à arête vive, annelures fines en travers.
3. **De profil, la corne penche en avant** de la ligne du chanfrein (40–50°) : presque verticale quand la tête est haute.
4. **Cornes jaune cire** (blanche) ou **ardoise** (noire), plus longues que la tête chez la brebis, deux fois plus chez le bélier.
5. **Toison en cordelettes verticales** ondulées, longues (20–30 cm), qui pendent en rideau et laissent une raie sur le dos ; ventre peu laineux.
6. **Tête et pattes à poil ras** : fauve roux chez la blanche, noir brillant chez la noire ; **face nette**, une petite touffe de laine seulement entre les cornes.
7. **Tête étroite et longue**, chanfrein légèrement busqué chez le bélier, grands yeux ambre.
8. **Petites oreilles horizontales**, sorties sous la base des cornes.
9. **Longue queue** qui descend plus bas que le jarret.
10. **Encolure dressée** (« de cerf »), port fier, pattes fines et assez hautes.

### 9.2 Ce qui en ferait une autre race

| Si l'on dessine… | on obtient plutôt… | La Racka |
|---|---|---|
| des cornes **enroulées en spirale** autour de l'oreille, épaisses, triangulaires, 1 à 2 tours | un **mérinos** (béliers à « longues cornes en spirale qui poussent près de la tête », brebis sans cornes [S WP-Merino]) ou un mouflon | une **vis droite** qui s'éloigne de la tête, vers le haut |
| des cornes vrillées **horizontales**, en spirale ouverte, tendues sur les côtés ; front laineux, taches noires autour des yeux et du nez | un **Zackel valaque / Walachenschaf** (cornes « en tire-bouchon, écartées sur le côté », jusqu'à 50 cm, blanches à trait noir ; brebis pas toujours cornues ; front laineux ; taches de pigment, parfois des « lunettes » [S LfL-WAL][S WP-DE-WAL]) | V **dressé** ; tête nette, fauve uni, sans taches |
| un bélier à cornes en spirale ouverte large, des **brebis sans cornes** ou à petites cornes en demi-cercle vers l'avant, des taches sombres autour des yeux, des pattes et des lèvres | une **Țurcană** de Roumanie [S LSMA2024] ou un Zackel de Transylvanie (« béliers à cornes en spirale ouverte régulière ; brebis sans cornes ou à cornes vestigiales » [S Neubauer2015]) | brebis **cornues** comme le bélier, même vis dressée |
| **quatre cornes**, ou un mélange d'animaux cornus et sans cornes, toisons de toutes couleurs, taches | un **Navajo-Churro** (« cornus ou sans cornes chez les deux sexes ; cornes multiples courantes », « blanc souvent à face et pattes tachetées » [S NCSA]) ou un Jacob | **deux** cornes, deux couleurs unies |
| une face **laineuse**, toison dense et courte en blocs, sans cornes | n'importe quelle race à viande ou un mérinos moderne | face rase, toison en cordelettes |
| une toison grise et une **tête noire** à petites cornes recourbées en arrière | une **Heidschnucke** (`docs/races/heidschnucke.md`) | variété noire : tête noire, mais cornes en V dressé et queue longue |
| une **queue courte** | un mouton nordique à queue courte (Heidschnucke, Soay, Ouessant…) | queue longue, au-delà du jarret |
| des cornes en vis **sans écartement** (parallèles), très longues, sur une chèvre ou un mouton à poil ras | un markhor, une chèvre de Girgentana | un mouton à toison, V de 60 à 110° |

**Le Navajo-Churro, le mérinos, le Zackel valaque, la Țurcană, en résumé :**

| | Racka | Mérinos (cornu) | Navajo-Churro | Walachenschaf | Țurcană |
|---|---|---|---|---|---|
| Cornes | 2, **vis droite dressée en V**, les deux sexes | 2, **spirale enroulée** près de la tête, béliers seulement | 0, 2 ou 4, béliers surtout, formes variées | 2, **tire-bouchon latéral**, bélier ; brebis fines ou nues | 2, spirale ouverte, bélier ; brebis nues ou petites cornes |
| Toison | jarre long en cordelettes | laine fine, dense, en blocs | jarre et duvet, ouverte, sans frisure | mixte grossière, blanche | mixte, longue, blanche, noire, grise |
| Tête | rase, fauve ou noire | laineuse au front | rase, souvent tachetée | front laineux, taches | taches autour des yeux |
| Sources | [S MJKSZ] | [S WP-Merino] | [S NCSA][S WP-NC] | [S LfL-WAL][S WP-DE-WAL] | [S LSMA2024] |

---

## 10. Sources, photos, contradictions

### 10.1 Sources écrites (ouvertes)

**Standard et associations de race**
- **[S MJKSZ]** Magyar Juh- és Kecsketenyésztő Szövetség, page *Hortobágyi racka (fehér, fekete) juh* : objectif d'élevage, caractères, **description extérieure** (tête, cornes, encolure, toison, couleurs des deux variétés, agneaux), motifs d'exclusion, poids et garrots moyens : https://www.mjksz.hu/tenyesztes/fajtak/hortobagyi-racka-feher-fekete-juh . Le texte est celui de la description révisée en 2008 et intégrée au programme d'élevage [S Lévai2025]. Le programme complet (*Juh tenyésztési program 2024*, 108 pages, scanné sans texte) a été ouvert mais pas lu en entier : https://www.mjksz.hu/sites/default/files/media/files/juh-tenyesztesi-program-2024.pdf
- **[S NCSA]** Navajo-Churro Sheep Association, *Breed Standard* : https://www.navajo-churrosheep.com/?p=20689
- La **Racka Sheep Society** (Royaume-Uni), https://rackasheep.com/the-breed , ne répondait pas (erreur de certificat, puis 503) ; je la cite seulement à travers [S WP-EN]. Le site de l'association hongroise d'éleveurs fondée en 1983 (Magyar Rackajuh-tenyésztő Egyesület) n'a pas été trouvé en ligne ; le livre généalogique est aujourd'hui tenu par la MJKSZ [S Lévai2025].

**Instituts et administrations**
- **[S LfL-ZAK]** Bayerische Landesanstalt für Landwirtschaft, *Ungarisches Zackelschaf* (VDL 2005) : https://lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_ungarisches_zackelschaf.pdf
- **[S LfL-WAL]** LfL, *Walachenschaf* : https://lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_walachenschaf.pdf
- **[S BMLUK]** ministère autrichien de l'agriculture, *Zackelschaf* (registre des aliments traditionnels : description, cornes, toison, tempérament, élevage) : https://www.bmluk.gv.at/themen/lebensmittel/trad-lebensmittel/Fleisch/Schaf-Ziege/zackelschaf.html
- **[S HNP]** Hortobágyi Nemzeti Park, *Nature trails in the Hortobágy National Park* (la Racka dans la gestion des prairies) : https://www.hnp.hu/uploads/documents/nature-trails-in-the-hortobagy-national-park1.pdf

**Études**
- **[S Pihler2020]** I. Pihler et al., « The Phenotype Variability of the Racka Sheep in the Republic of Serbia », *Journal of the Hellenic Veterinary Medical Society* 70 (4), 2020, 1789–1796, doi:10.12681/jhvms.22221 (150 brebis). Le PDF de la revue est derrière une protection Cloudflare ; résumé lu via Crossref : https://api.crossref.org/works/10.12681/jhvms.22221 ; **mesures et indices lus dans leur reprise** par :
- **[S Ružić-Muslić2021]** D. Ružić-Muslić et al., « Morphometric characterization of Pirot Pramenka », *Biotechnology in Animal Husbandry* 37 (2), 2021, 75–85, tableaux 2 et 4 : https://www.istocar.bg.ac.rs/wp-content/uploads/2023/02/01.BAH-21-2-11-1.6.2021.-Ruzic-Muslic-et-al..pdf
- **[S Lévai2025]** Lévai et Gáspárdy, « Pedigree based analysis of population structure in Hungarian sheep (Racka sheep) », *Danubian Animal Genetic Resources* 2025-2 (histoire, effectifs, deux livres, révision de 2008) : https://ojs3.mtak.hu/index.php/dagr/article/download/20854/17276 ; **[S Lévai2025-abs]** résumé (effectifs 2024, éleveurs) : https://real.mtak.hu/id/eprint/230019
- **[S Zsolnai2023]** génotypage de 126 Racka noirs et 128 blancs (« deux races génétiquement distinctes », *MC1R*), *Állattenyésztés és Takarmányozás* 2023 : https://real.mtak.hu/229859/
- **[S Neubauer2015]** V. Neubauer et al., « Genetic diversity and population structure of Zackel sheep and other Hungarian sheep breeds », *Archives Animal Breeding* 58 (2015), 343–350 (« cornes en vis tenues en V bas » ; Zackel de Transylvanie) : https://aab.copernicus.org/articles/58/343/2015/aab-58-343-2015.pdf
- **[S Pajor2016]** comportement de cinq agneaux Racka pendant les 48 premières heures (NAIK Herceghalom) : https://real.mtak.hu/79511/
- **[S Nagy2003]** L. Nagy, Á. Domanovszky et P. Székely, engraissement et abattage des deux variétés (Atkár, *Agrártudományi Közlemények*) : https://ojs.lib.unideb.hu/actaagrar/article/download/3379/3343 ; **[S Nagy2005]** composition du lait de 36 brebis : https://ojs.lib.unideb.hu/actaagrar/article/download/3284/3248/5876 (lues pour vérifier qu'elles ne donnent pas de mensurations : elles n'en donnent pas)
- **[S Schütz2023]** intervalle de génération chez la Gyimesi racka : https://ojs.mtak.hu/index.php/dagr/article/download/12256/9920
- **[S LSMA2024]** article roumain sur la Țurcană (« béliers à cornes bien développées ; brebis sans cornes ou à petites cornes en demi-cercle vers l'avant ; taches sombres autour des yeux, des membres et des lèvres ») : https://www.lsma.ro/index.php/lsma/article/download/2963/pdf
- **Cherchée sans succès :** L. Ledwith, *Study of the skull and horn geometry of the Hungarian racka sheep breed*, mémoire, Budapest, 2014 : https://huveta.hu/handle/10832/1232 (notice ouverte, PDF réservé : « Authentication is required »). C'est sans doute la seule étude des angles des cornes ; elle reste à lire.

**Presse, vulgarisation, encyclopédies**
- **[S munkacsy]** fiche *A magyar racka juh* (V « droit, plusieurs fois vrillé », histoire) : https://munkacsy.hu/wp-content/uploads/2025/04/Magyar-racka-juh.pdf
- **[S magyarmuseum]** M. Kútvölgyi, extrait sur la Racka et la *suba* : https://magyarmuseum.org/racka-en/racka/
- **[S Wovember]** Wovember 2016, avec un extrait du *Fleece & Fiber Sourcebook* (Robson et Ekarius, 2011, p. 320) : https://www.wovember.com/2016/11/13/the-hungarian-racka-sheep-of-hungary-magyar-racka-juh-magyarorszag/
- **[S PostAuto]** CarPostal, « Un agneau de Pâques pas comme les autres » (11 avril 2025 ; éleveur suisse : cornes toute la vie, tempérament) : https://www.postauto.ch/en/about-us-and-news/news/2025/an-easter-lamb-with-a-difference
- **[S NBC]** NBC News / Reuters, « Hungarian Shepherds Ring in Grazing Season » (Hortobágy, 26 avril 2014) : https://www.nbcnews.com/news/europe/theyre-baaack-hungarian-shepherds-ring-grazing-season-n90486
- **[S WP-HU]** https://hu.wikipedia.org/wiki/Magyar_racka_juh · **[S WP-DE]** https://de.wikipedia.org/wiki/Zackelschaf · **[S WP-EN]** https://en.wikipedia.org/wiki/Racka
- **[S WP-Merino]** https://en.wikipedia.org/wiki/Merino · **[S WP-NC]** https://en.wikipedia.org/wiki/Navajo-Churro · **[S WP-DE-WAL]** https://de.wikipedia.org/wiki/Walachenschaf
- La fiche de l'Oklahoma State University (https://breeds.okstate.edu/sheep/racka-sheep) répond 403 ; elle est la source de [S WP-EN] pour les longueurs minimales des cornes, que je ne cite qu'à travers Wikipédia.

### 10.2 Photos regardées

Toutes téléchargées hors du dépôt (`scratchpad/photos/racka/`), regardées, et pour plusieurs mesurées.

| N° | Page | Ce qu'elle montre |
|---|---|---|
| P1 | https://commons.wikimedia.org/wiki/File:B%C3%A9lier_Racka.JPG | **jeune bélier blanc, profil strict, tourné vers la gauche**, toison courte : la photo des proportions (garrot, longueur, carpe, jarret, tête, oreille, **queue jusqu'au paturon**) et de l'**angle corne / chanfrein** (40°) ; cornes de 1,5 à 2 tours à annelures serrées ; à l'ombre (couleurs non retenues) |
| P2 | https://www.flickr.com/photos/69239666@N06/22124566480/ | **brebis blanche de profil, tournée vers la droite**, en marche, **au Hortobágy**, 10 octobre 2015 : cornes presque verticales (47° avec le chanfrein), toison de cinq mois en cordelettes, pattes rousses ; mesurée |
| P3 | https://www.flickr.com/photos/damka/2377480669/ | **bélier blanc idéal**, musée de plein air de Szentendre, 30 mars 2008, de trois quarts : V étroit (~63°), **4 à 5 tours**, tête **brun rouge**, toison de fin d'hiver en cordelettes jusqu'au genou, cloche ; couleurs mesurées |
| P4 | https://www.flickr.com/photos/damka/2382894215/ | même bélier, tête de près : section en lame, arête, annelures ondulées, oreille horizontale sous la corne, iris, chanfrein busqué ; couleurs |
| P5 | https://www.flickr.com/photos/damka/2382893629/ | même bélier vu de dessus : V, **trait sombre** le long de l'arête, cintre des pointes |
| P6 | https://www.flickr.com/photos/kerekkrisztian/2375402014/ | même bélier, autre photographe, même jour : cornes de profil oblique, bavette de mèches |
| P7 | https://commons.wikimedia.org/wiki/File:Rackaschapen.jpg | tête blanche de face (lumière rasante) et une noire : **lame vrillée**, annelures, **touffe de laine** de la nuque, naseaux roses tachés ; couleurs |
| P8 | https://www.flickr.com/photos/francesco_veronesi/4575306712/ | Racka blanche, Túrkeve, avril 2010 : face fauve, joues plus rousses, V très ouvert (~145°), arête sombre, oreilles ; couleurs |
| P9 | https://www.flickr.com/photos/francesco_veronesi/4575307656/ | brebis **noire**, même troupeau : face noir bleuté, **iris ambre**, cornes ardoise, toison grisonnante à pointes brunes ; couleurs |
| P10 | photo de gauche de [S LfL-ZAK] | **bélier noir** de trois quarts, toison pleine en cordelettes gris brun, tête et pattes noires, cornes en V ; couleurs |
| P11 | https://www.flickr.com/photos/ixchelbunny/29090036413/ | bélier noir âgé, de face : **capuchon noir** sur corps gris, cornes très écartées en tire-bouchon ouvert ; couleurs (dominante bleue) |
| P12 | https://commons.wikimedia.org/wiki/File:Zackelschafe_Tiergarten_Bernburg_06-03-2008.jpg | cinq brebis blanches de face, Bernburg, mars 2008, ciel couvert : **V de 60–80°**, faces crème à fauve, oreilles horizontales, pattes fauves, longueur de la toison ; couleurs |
| P13 | https://commons.wikimedia.org/wiki/File:Ungarisches_Zackelschaf_cropped.jpg | bélier noir couché : cornes longues très écartées, avant noir et corps grisonnant |
| P14 | https://www.flickr.com/photos/csabx/4537599690/ | bélier blanc qui **broute**, Kő-hegy, avril 2010 : toison pleine de 25 cm, raie du dos, **cornes pointées en avant** à l'horizontale |
| P15 | https://www.flickr.com/photos/hattyu/4949500272/ | bélier qui broute, Rádpuszta, août 2010 : toison d'été courte, cornes en avant, jarret laineux |
| P16 | https://www.flickr.com/photos/cordylus/7159438623/ | bélier blanc « Tibor », mai 2012 : cornes très ouvertes, toison pleine, cascade de mèches du cou |
| P17 | vignette de la vidéo de [S MJKSZ] | bélier en bergerie, de trois quarts vers la droite : **cornes dressées** très hautes, toison en cordes jusqu'au genou |
| P18 | https://www.flickr.com/photos/pbraun/22059203903/ | groupe au trot de profil, Szentendre, octobre 2015 : allure, pattes, cornes dressées |
| P19 | https://www.flickr.com/photos/32621846@N06/33372668615/ | brebis blanches avec un **agneau noir** bouclé et un agneau blanc |
| P20 | https://commons.wikimedia.org/wiki/File:Fertoe-Hansag-Nationalpark_18.JPG | **agneau brun uni** au Fertő-Hanság |
| P21 | https://commons.wikimedia.org/wiki/File:Racka_sheep_-_City_Park_in_Lu%C4%8Denec.jpg et (1) à (4) | bélier noir couché, agneaux, brebis noires, parc de Lučenec |
| P22 | https://commons.wikimedia.org/wiki/File:Racka_Ram_01.JPG | bélier à cornes **horizontales** peu vrillées, face pâle : forme atypique (type bosnien ou croisement ?), contre-exemple |
| P23 | https://commons.wikimedia.org/wiki/File:Animals_in_Parc_de_Thoiry_37.jpg | tête blanche de face : oreilles horizontales, touffe de laine, face fauve |
| P24 | https://commons.wikimedia.org/wiki/File:Fertoe-Hansag-Nationalpark_19.JPG et _17, _21, _23 | troupeau mêlé blanc et noir qui broute sur la steppe |
| P25 | https://www.flickr.com/photos/barithelamb/5108500964/ | troupeau dense, forêt de cornes en V |
| P26 | https://www.molon.de/galleries/Hungary/Hortobagy/img.php?pic=12 | Racka couchées dans une bergerie du Hortobágy |
| P27 | https://www.flickr.com/photos/majorzoli/3163088319/ | trois Racka de face, Skanzen de Szentendre : V, touffes de laine |
| P28 | https://www.flickr.com/photos/maret1983/6889931516/ | bélier noir grisonnant couché |
| Planche | https://commons.wikimedia.org/wiki/Category:Racka_sheep et https://commons.wikimedia.org/wiki/Category:Racka_sheep_in_Fert%C5%91%E2%80%93Hans%C3%A1g_National_Park ; recherche Flickr « racka » (75 vignettes) | une centaine de vignettes regardées en planche (Erlenbach, Thoiry, Lučenec, Fertő-Hanság, Hortobágy, Szentendre) |
| Walachenschaf | photos de [S LfL-WAL] | comparaison : cornes latérales en tire-bouchon, front laineux, taches noires de la face |

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Contradiction |
|---|---|
| Hauteur au garrot | béliers 72, brebis 66 cm [S MJKSZ][S WP-HU] ; 65–70 / 60–65 cm [S BMLUK] ; **60 / 55 cm** [S LfL-ZAK] ; brebis serbes 64,3 cm [S Pihler2020]. Le standard hongrois l'emporte ; les chiffres bas de la LfL viennent peut-être de petits troupeaux d'Allemagne. |
| Poids | béliers 50–70, brebis 35–45 kg [S MJKSZ] ; 55–75 kg béliers [S WP-HU] ; 60–80 / 40–50 kg [S BMLUK] ; 55–65 / 40–45 kg [S LfL-ZAK] ; 60 / 40 kg « minimum » [S WP-EN]. |
| Longueur des cornes du bélier | ≥ 2 LT [S MJKSZ] ; ~0,5 m [S WP-HU] ; minimum 51 cm [S WP-EN] ; « jusqu'à 1 m » [S LfL-ZAK][S BMLUK][S PostAuto] ; « jusqu'à 61 cm » [S WP-EN]. Le mètre vaut pour de très vieux béliers. |
| Écartement (V) | 90–110° (béliers), 50–60° (brebis) [S WP-HU] ; 60–90° [S BMLUK] ; « V bas » [S Neubauer2015] ; « presque droit vers le haut » [S WP-EN]. Photos : 60 à 145°, l'écart croissant avec l'âge. |
| Profondeur de poitrine (Serbie) | le tableau de Ružić-Muslić imprime 37,97 cm, mais les indices du même article (poitrine 62,75 ; longueur de patte 52,89) donnent tous deux ≈ 30,2 cm : coquille probable ; j'utilise 30,3 cm. |
| Finesse de la laine | duvet 20–40 µm, jarre 60–80 µm [S MJKSZ] ; 16–30 / 40–60 µm [S Wovember] ; 12–40 µm [S WP-EN] ; **« 120–200 microns »** [S WP-HU], sans doute une erreur. |
| Poids de toison | béliers 4–5, brebis 2–3 kg [S BMLUK] ; 3–4 / 1–2 kg [S LfL-ZAK] ; 2–5 kg [S Wovember] ; « au moins 3 kg » pour un bélier [S WP-EN]. |
| Front laineux | « front laineux avec un toupet » [S BMLUK] contre « tête couverte de poil court brillant ; face toujours nette » [S MJKSZ]. Les photos s'accordent : touffe sur la nuque entre les cornes, face rase. |
| Couleur dominante | « le blanc est récessif face au noir » [S WP-HU][S WP-DE] : c'est l'inverse de la règle habituelle chez le mouton (le blanc d'agouti est dominant) ; aucune étude ouverte ne le confirme. |
| Effectifs | « environ 2 500 animaux » [S WP-EN, ancien] ; ~10 000 [S Pajor2016] ; 8 464 brebis inscrites en 2025 [S Lévai2025] ; 6 700 en 2015 [S Neubauer2015]. |
| Ancêtre sauvage | l'urial [S BMLUK][S WP-DE] ; le mouflon d'Asie (*Ovis gmelini*) [S WP-HU]. La génétique ne tranche pas pour la race ; ne rien en tirer pour le dessin. |
| Origine | arrivée avec les Hongrois au IXᵉ siècle [S BMLUK][S WP-DE][S PostAuto] contre « aucune preuve ; premières traces au XVIᵉ–XVIIᵉ siècle » [S MJKSZ][S Lévai2025]. |
| Brebis sans cornes | « rares » [S LfL-ZAK] ; « femelles pas toujours cornues » (fiche scolaire autrichienne, non retenue) ; exclues du livre en Hongrie [S MJKSZ]. |
| Sens de la vrille | **aucune source** ; sur les photos, les deux cornes sont en miroir, mais je n'ai pas pu établir avec certitude si la corne gauche est une vis dextre ou senestre (section 6.2, point 8 : copier P1 retournée). |
| Nombre de tours | ≥ 4 après deux ans [S MJKSZ] ; « sokszorosan csavart » (« vrillée maintes fois ») [S munkacsy] ; photos : 1,5–2 tours chez un jeune (P1), 4–5 chez l'adulte (P3). |
| Cloches | photos de Racka portant une cloche (P3, P4, Szentendre) ; aucune source écrite trouvée sur l'usage des cloches dans les troupeaux de Racka. |
| Mesures de bélier, de tête, d'oreille, de queue | **aucune source** ; tout ce qui est en section 2.2 pour le bélier est mis à l'échelle depuis la brebis serbe [S] ou mesuré sur photo [P]. |
