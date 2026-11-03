> Fiche de la race « Heidschnucke » (la Graue Gehörnte Heidschnucke, de la lande de Lunebourg), préparée le 3 novembre 2026 pour dessiner un futur `pets/heidschnucke` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Heidschnucke grise cornue : fiche pour un dessin de profil

**Noms :** Graue Gehörnte Heidschnucke (DE, abréviation officielle **GGH**, code de race 30) · German Grey Heath (EN) · « Schnucke » tout court dans la lande · en français, faute de nom établi, on garde *Heidschnucke* (Wikipédia FR emploie « jaglu », qu'aucune autre source ne connaît) [S VLH][S LfL-GGH][S WP-EN][S WP-FR].
**Groupe :** moutons nordiques à queue courte (*nordische Kurzschwanzschafe*), mischwollig (toison mixte) [S LfL-GGH][S VLH].
**Rédigée :** le 3 novembre 2026, d'après le programme d'élevage du Verband Lüneburger Heidschnuckenzüchter (VLH, 2021), les fiches de la LfL de Bavière et de la GEH, une étude de zootechnie, les pages du Verein Naturschutzpark (VNP) qui garde six troupeaux de la lande, et une trentaine de photographies regardées de près (section 10).

**Étiquettes :**
- **[S …]** : une source écrite, nommée en section 10.1.
- **[P]** : observé ou mesuré sur une photo (section 10.2) ; les pixels ont été comptés, les couleurs moyennées sur de petites zones avec PIL.
- **[E]** : mon estimation (anatomie générale du mouton, recoupée sur les photos). Aucune source ne la donne.
- **H** = hauteur au garrot (os), **LT** = longueur de la tête (nuque → bout du museau).

---

## 0. En bref

- **L'animal à dessiner :** une **brebis adulte de quatre ans**, en **fin d'automne** (début novembre), environ cinq mois après la tonte de juin : toison grise déjà longue (mèches d'une dizaine de centimètres), pendante. Elle se tient carrée, **de profil strict, tournée vers la droite** (on voit son flanc gauche), tête haute, sur la bruyère passée.
- **En une phrase :** un mouton de taille moyenne, haut sur pattes fines, au **corps gris argent** fait de longues mèches raides et pendantes striées de noir, avec une **cape noire** (tête, encolure jusqu'au garrot, bavette du poitrail) qui se fond dans le gris de l'épaule, des **pattes noires sans laine**, une **courte queue noire** à poil ras, de **petites oreilles** portées de biais, et des **cornes chez les deux sexes** : en **faucille** vers l'arrière chez la brebis, en **escargot** chez le bélier.
- **Le troupeau :** la Heidschnucke vit en grand troupeau (environ 350 brebis, jusqu'à 400 agneaux et quelques béliers), mené chaque jour 8 à 12 heures sur la lande par un berger et ses chiens, rentré la nuit à la bergerie ; elle est farouche et fuit qui l'approche [S VNP].

**Palette de base** (détails et mesures en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Toison grise (flanc) | `#B4B0AA` | `#DAD6D0` | `#6E6A68`, creux `#3C3A3B` |
| Stries sombres des mèches | `#4A484C` | `#6B686B` | `#2A292C` |
| Cape noire (encolure, bavette, ventre) | `#1C1B1F` | `#3B3B3F` | `#0D0C10` |
| Face et pattes (poil ras) | `#1E1F24` | reflet froid `#4A5266` | `#111115` |
| Cornes de la brebis | `#7A7876` | anneau clair de la base `#A8A6A2` | `#4E4C4C` |
| Onglons | `#222120` | usure `#5A5650` | `#121110` |
| Iris | `#C8B78E` (paille, ambre pâle) | `#DCCBA4` | cercle `#5E5446`, pupille `#0E0E10` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 La race, en quelques lignes

- C'est le mouton de la **lande de Lunebourg** (Basse-Saxe), qui a fait la lande en broutant la bruyère et les jeunes bouleaux ; « Schnucke » viendrait du bas allemand *snickern*, grignoter [S VNP][S NP2][S GEH-GGH].
- Jusqu'au XIXᵉ siècle, gris et blancs, cornus et sans cornes formaient une seule population ; trois races en sont sorties au XXᵉ siècle (premier livre généalogique de la grise en 1930, association d'éleveurs en 1949) [S GEH-GGH][S WP-DE].
- Effectifs : 379 578 Heidschnucken dans l'ancienne principauté de Lunebourg en 1848, environ 12 000 en 2002 [S VLH-web] ; au 1ᵉʳ janvier 2021, 73 béliers et 4 393 brebis inscrits au livre du VLH, dans 24 élevages [S VLH]. La GEH en a fait la « race menacée de l'année 2026 » [S dpa].
- Sa viande est une appellation d'origine protégée de l'UE, « Lüneburger Heidschnucke g. U. » (règlement (CE) n° 134/98) [S WP-DE].

### 1.2 Pourquoi la grise cornue, et pas l'une des deux blanches

Trois races portent le nom ; la grise cornue est la plus nombreuse et c'est elle qui « fait » la lande de Lunebourg [S WP-EN][S WP-DE][S GEH-GGH].

| | **Graue Gehörnte (GGH)** | Weiße Gehörnte (WGH) | Weiße Hornlose, « Moorschnucke » (WHH) |
|---|---|---|---|
| Couleur | toison grise, tête, pattes, queue **noires**, bavette noire | **blanche, sans marques** ; tête, pattes, queue courte blanches à poil ras ; petites taches de pigment tolérées | **blanche, sans marques** ; tête parfois avec un petit toupet de laine (*Schaupe*) |
| Cornes | bélier en escargot, brebis en arrière | bélier « en escargot », brebis « en faucille, recourbées vers l'arrière » | **aucune**, chez les deux sexes |
| Garrot (LfL) | béliers 70–75, brebis 65–70 cm | béliers 65–70, brebis 60–65 cm | béliers 65–70, brebis 60–65 cm (d'autres sources : 50–55 cm) |
| Brebis (poids) | 45–55 kg | 40–50 kg | 40–50 kg |
| Milieu | lande sèche, sable | landes et tourbières | tourbières humides (Diepholz) |
| Toison voulue | « pas trop claire », gris argent | « blanc pur » | blanc pur, « dense et longuement ondulée », pas en boucles rondes |

Sources : [S LfL-GGH][S LfL-WGH][S LfL-WHH][S GEH-WGH][S GEH-WHH][S WP-WPH]. La blanche cornue serait sortie de la grise par sélection dans les années 1920 [S GEH-WGH] ; la Moorschnucke, d'un croisement avec une race sans cornes à laine lisse [S GEH-WHH][S WP-WPH].

**On dessine la grise :** c'est elle qu'on voit sur les affiches de la lande, et son dessin (gris argent sur cape noire) se lit au premier coup d'œil, même petit. Les deux blanches ressembleraient de loin à n'importe quel mouton blanc à face nue.

### 1.3 Pourquoi une brebis, et laquelle

1. **Le troupeau, ce sont des brebis.** Un troupeau du VNP : « environ 350 brebis, jusqu'à 400 agneaux selon la saison, et quelques béliers » [S VNP]. L'image de la lande, c'est elle.
2. **Ses cornes la distinguent.** Presque toutes les races cornues du chantier ont un bélier à cornes en spirale (mouflon, Ouessant, Jacob, Soay, Hébridais, Racka, argali…). La brebis Heidschnucke porte des **cornes fines en faucille, rejetées vers l'arrière** [S LfL-GGH][S VLH][S GEH-GGH] : un trait rare, qui la sépare aussi du Gotland, gris mais sans cornes (section 9).
3. **Elle montre tout le motif du standard :** cape noire jusqu'au garrot, bavette, pattes, queue noires, toison argent [S VLH].
4. **Il existe une photo de référence presque idéale :** une brebis grise de profil strict tournée vers la droite, au Wildpark Lüneburger Heide, en novembre (photo P1, section 10.2), qui sert ici aux mesures.

**L'individu :**
- brebis de **4 ans**, adulte faite (cornes formées, couleur grise définitive) ;
- **garrot 67 cm**, **50 kg** (milieu des fourchettes du standard) [S VLH][S LfL-GGH] ;
- **début novembre**, cinq mois après la tonte de juin [S VNP][S dpa] : mèches d'environ **10 cm** sur le flanc [E, d'après P1], qui pendent déjà, le dos un peu ébouriffé ;
- couleur **idéale du standard** : toison gris argent, cape noire jusqu'au garrot, bavette noire, aucun poil gris dans le noir de la tête [S VLH] ;
- debout, carrée, tête haute, oreilles de biais, queue pendante ; **pas de boucle d'oreille** (toutes les bêtes réelles en portent une jaune : on ne la dessine pas).

### 1.4 L'autre sexe : le bélier (pour mémoire)

| | Brebis (choisie) | Bélier |
|---|---|---|
| Garrot, poids | 65–70 cm ; 45–55 kg | 70–75 cm ; 75–90 kg [S VLH][S LfL-GGH][S GEH-GGH] |
| Cornes | fines, section **elliptique**, en faucille vers l'arrière [S GEH-GGH] | **massives, section triangulaire**, « en escargot », en spirale tournée vers l'extérieur, **écartées de la tête** pour ne pas y entrer [S GEH-GGH][S VLH][S LfL-GGH] ; environ 1 à 1¼ tour [P] |
| Encolure | fine, noire | épaisse, avec une **crinière** de longs poils noirs sur la gorge et l'encolure au printemps [P : P2, P11] |
| Tête | longue, fine, chanfrein droit | plus forte, chanfrein légèrement busqué [P : P3, P4] |
| Toison | 1,7–2,5 kg | 4,0 kg (LfL, *Altböcke*) ou 2,5–3,0 kg (GEH, VLH pour les antenais) |

### 1.5 L'agneau

- Il naît **noir, en laine bouclée** [S LfL-GGH][S VLH][S VNP] ; 4 kg seul, 3 kg en portée multiple [S VLH].
- Il **change de couleur** pendant sa première année : du noir au **brun** puis au gris ; la couleur grise de la race ne vient qu'après la première tonte [S LfL-GGH][S VLH] (« du noir au brun puis au gris dans les deux premières années », [S GEH-GGH]). Dans un troupeau d'été, on voit donc des agneaux noirs, des antenais roussâtres et des adultes gris [P : P19, P20].
- Pattes longues pour le corps, oreilles larges tenues de côté, bourgeons de cornes, queue noire déjà courte [P : GEH-Beuren, P20].

---

## 2. Tailles et proportions

### 2.1 Les chiffres

| Mesure | Brebis | Bélier | Sources |
|---|---|---|---|
| Hauteur au garrot | **65–70 cm** | **70–75 cm** | [S VLH][S LfL-GGH][S GEH-GGH] ; le Naturpark donne 50–60 cm pour « la » Heidschnucke (section 10.3) |
| Poids adulte | **45–55 kg** | **75–90 kg** | mêmes sources ; antenaises 35–45, antenais 60–80 kg |
| Poids de naissance | 4 kg (simple), 3 kg (multiple) | | [S VLH] |
| Toison (poids) | 1,7–2,5 kg | 4,0 kg (LfL) ; 2,5–3,0 (GEH) | [S LfL-GGH][S GEH-GGH][S VLH] |
| Longueur du corps, tour de poitrine, hauteur sous le ventre, tête, oreille, queue | **aucune source trouvée** | | voir 2.2, mesurés sur photo |

Aucune étude biométrique de la GGH n'a été trouvée (recherches en allemand et en anglais : « Körpermaße », « Rumpflänge », « Brustumfang », « body length », « chest girth »). Les mesures ci-dessous viennent des photos de profil P1 (brebis tournée vers la droite), P13 (brebis du LfL, tournée vers la gauche) et P14 (bélier du VLH, tourné vers la droite).

### 2.2 Ratios de dessin (brebis, H = 67 cm)

Méthode : sur P1, le dessus de la toison au garrot est à 485 px du sol ; avec 8 cm de laine sur le garrot [E], H (os) ≈ 433 px, soit 6,5 px/cm. P13 donne les mêmes rapports à quelques centièmes près.

| Mesure | En H | En cm | Étiquette |
|---|---|---|---|
| Dessus de la toison au garrot | 1,10–1,12 | 74–75 | [P] P1, P13 |
| Silhouette en toison, poitrail → fesse | 1,35–1,40 | 90–94 | [P] P1 : 610 px ; P13 : 1,35 |
| Longueur du corps (os), pointe de l'épaule → ischion | 1,10–1,15 | 74–77 | [E] |
| Hauteur de la toison, dessus → frange du ventre | 0,68–0,70 | 46–47 | [P] P1, P13 |
| Profondeur de poitrine (os) | ≈ 0,48 | ≈ 32 | [E] ; « tronc profond et fermé, côtes bien cintrées » [S LfL-GGH] |
| Tour de poitrine | ≈ 1,3 | 85–90 | [E] (pour 50 kg) |
| Bas de la bavette (sous le poitrail) au-dessus du sol | 0,45–0,48 | 30–32 | [P] P1 |
| Patte visible sous la frange du ventre | 0,40–0,42 | 27–28 | [P] P1 (180 px), P13 |
| Carpe (genou) au-dessus du sol | 0,22–0,24 | 15–16 | [P] P1, P13 ; 0,21 sur le bélier P14 |
| Boulet antérieur au-dessus du sol | ≈ 0,08 | ≈ 5 | [P] P1, P14 |
| Jarret (pointe) au-dessus du sol | 0,33–0,36 | 22–24 | [P] P1 (150 px), P13 |
| Boulet postérieur | ≈ 0,09 | ≈ 6 | [P] P1 |
| Encolure (garrot → nuque, ligne du dessus) | 0,33–0,35 | 22–23 | [P] P1 (141 px) |
| Tête, LT (nuque → bout du museau) | 0,33–0,36 | 22–24 | [P] P13 (≈ 21 cm, tête un peu tournée) ; [E] |
| Oreille | ≈ 0,13 (≈ 0,4 LT) | 8–10 | [E] ; « petites » [S LfL-GGH][S GEH-GGH] |
| Queue (poil noir, hors laine) | 0,20–0,22 | 13–15 | [P] P1 (95 px) ; la pointe à 0,8 H du sol |
| Corne de la brebis, corde base → pointe | 0,25–0,28 | 17–19 | [P] P1 (117 px) |
| Corne de la brebis, le long de la courbe | 0,30–0,37 | 20–25 | [E] |

**La silhouette :** un rectangle gris plus long que haut (environ 1,35 fois), posé sur **quatre pattes noires, fines et assez longues** (la patte visible fait 0,6 fois la hauteur de la toison en automne, presque autant que le corps juste après la tonte), surmonté d'une encolure noire dressée. Ce n'est **pas** une boule de laine aux pattes courtes : la Heidschnucke est « haute et fine » de pattes (« Fundament fein und trocken ») [S LfL-GGH][S GEH-GGH].
- **Le corps sous la toison :** sur la brebis tondue de P24 (un peu de biais), la profondeur du corps fait environ 0,5 H et la patte sous le ventre à peu près autant ; la toison d'automne ajoute donc surtout du volume en haut et sur les côtés, et une frange de 5 à 10 cm sous le ventre [P].
- Ligne du dos à peu près droite ; la croupe tombe un peu vers la queue [P : P1, P13].
- La toison ne fait pas un bloc net : le bord est **effiloché** (mèches pointues qui dépassent), surtout le long du dos et au bas du flanc [P].
- La tête paraît petite et longue au bout de l'encolure ; avec l'encolure noire, tête et cou forment une seule forme noire au-dessus du poitrail.

### 2.3 L'encolure

- **Longueur :** environ 0,35 H, fine, ronde, sans fanon [P : P1, P13] [E].
- **En alerte (la pose choisie) :** l'axe de l'encolure monte à **55–65° au-dessus de l'horizontale** ; la nuque dépasse le dessus du dos d'environ 0,35 H ; le chanfrein descend de 30 à 45° sous l'horizontale (P13 : 30° ; P14 : 40° ; P1 regarde l'objectif, 65°) [P].
- **Au repos, debout :** l'encolure s'abaisse à 30–40° au-dessus de l'horizontale, la nuque à peu près au niveau du garrot ou un peu au-dessus [E].
- **En broutant :** l'encolure descend à **45–55° sous l'horizontale**, les antérieurs restent droits (l'un parfois avancé d'un pas), le chanfrein presque vertical, le museau au sol [P : P6, P9].
- **Le cou en deux segments** (pour le moteur) : la base (épaule → milieu) bouge peu ; la partie haute (milieu → nuque) fait l'essentiel de l'angle. Couvrant tout le cou, la cape noire suit la courbe : son bord gris reste sur l'épaule et ne bouge pas avec la tête [E].

### 2.4 Les articulations des pattes

| Articulation | Hauteur | Angle debout | Étiquette |
|---|---|---|---|
| Coude (antérieur) | ≈ 0,50 H, caché sous la toison | | [E] |
| Carpe (genou) | 0,22–0,24 H | presque droit, ≈ 175–180° | [P] P1, P13, P14 |
| Boulet antérieur | ≈ 0,08 H | paturon à **50–60°** sur l'horizontale | [P] P13 (59°), P14 (53°) |
| Grasset | ≈ 0,55 H, caché sous la toison | | [E] |
| Jarret | 0,33–0,36 H | **145–155°** (tibia oblique vers l'arrière, canon presque vertical) | [P] P1 : tibia à 32° de la verticale, canon à 7° |
| Boulet postérieur | ≈ 0,09 H | paturon à ≈ 55° | [P] P1 |

- La laine descend jusqu'au **haut de l'avant-bras** et à **mi-cuisse** (jusqu'au-dessus du jarret) ; plus bas, **poil ras noir** : le genou et le jarret se voient nettement [S VLH : « Beine unbewollt »][P].
- Le canon antérieur est fin et droit ; le jarret, sec, bien marqué en arrière [P].

---

## 3. La toison

### 3.1 Ce qu'elle est

- **Mixte, à deux couches :** « l'extérieur de grossier poil de jarre (*Oberhaar*), l'intérieur de duvet plus fin et doux (*Unterhaar*) » [S LfL-GGH][S VLH][S GEH-GGH].
- Finesse **38–39 µm** (« DE bis E/EE ») [S LfL-GGH][S VLH][S GEH-GGH] : une laine de tapis, sans frisure, qu'on ne file guère ; elle sert au feutre, aux coussins, à l'isolation, à l'engrais [S VNP][S dpa][S VLH-web].
- Les jarres « kempy » (*kempy heterotypes*) **poussent sans arrêt** et forment le manteau extérieur ; on les trouve chez la Heidschnucke grise [S Kurt2008]. Chez une race proche (Rauhwolliges Pommersches Landschaf), ce sont ces fibres **pigmentées** mêlées à des fibres **non pigmentées** qui donnent la couleur « grise à bleue » de la toison [S Kurt2008]. Le gris de la Heidschnucke est de même un **mélange de fibres noires et blanches**, pas une fibre grise [P : les stries noires visibles dans chaque mèche] [E pour le mécanisme chez la GGH].
- La toison grise « doit être uniforme et pas trop claire » ; idéal : **gris argent** [S VLH][S LfL-GGH]. La GEH dit « gris à gris foncé » [S GEH-GGH].

### 3.2 À quoi elle ressemble

- **Longues mèches raides ou à peine ondulées, pointues, qui pendent** verticalement sur le flanc [P : P1, P2, P11, P12]. « Extrêmement longue », « strähnig » (en mèches, filandreuse) [S WP-DE].
- **Pas de boucles :** rien des boucles serrées du Gotland ni du bloc dense du Romanov (section 9). Les pointes s'enroulent tout au plus en une demi-vague [P].
- **Stries :** chaque mèche porte des poils noirs ; de près, le flanc gris paraît **rayé verticalement** de traits sombres et fins [P : P1, P13, GEH-Tutsberg].
- **Bord :** la ligne du dos est hérissée de pointes ; la frange du ventre pend, noire ou gris très sombre [P : P1, P14].
- Après la tonte de juin, l'animal paraît lisse et clair, gris bleuté, très haut sur pattes, la cape noire de l'encolure et la ligne noire du ventre nettes comme un dessin [P : P24, troupeau tondu].

### 3.3 Longueur et saison

- **Une tonte par an**, des adultes seulement, **en juin** (450 bêtes tondues près de Soltau, juin 2026 ; 3 à 4 minutes par bête) [S dpa][S VNP] ; le Naturpark dit « en mai » [S NP2].
- **Longueur de mèche :** aucune source fiable ouverte. Sur les photos de printemps (P2, P11, P12 ; environ dix mois de pousse), les mèches pendent sur 20 à 25 cm ; à cinq mois (P1, novembre), sur 8 à 12 cm [P, estimation de l'œil]. On en tire une pousse d'environ 2 cm par mois pour les jarres [E].
- **Pour notre brebis de novembre :** 10 cm sur le flanc, 6 à 8 cm sur le dos, 12 cm en frange sous le ventre [E].
- Au printemps, la toison devient très longue, se feutre par plaques, prend des pointes brunâtres (soleil, intempéries) et peut paraître presque noire à l'avant chez certains béliers [P : P2, P11].

### 3.4 Ce qui reste ras

- **Tête, pattes et région de la queue sans laine, couvertes de poil noir** [S VLH][S LfL-GGH] ; la GEH dit « poils noirs courts » [S GEH-GGH].
- La laine commence derrière la nuque et les joues, sur l'encolure ; **la face est nue** jusqu'aux cornes, sans toupet [P : P3, P4, P5].
- L'encolure porte de longs poils noirs (ce n'est pas du poil ras) : la « cape » noire est une **toison noire**, pas une peau nue [P : P1, P14].

---

## 4. Les couleurs

### 4.1 Le motif

**Le standard (VLH 2021, repris par la LfL) :** « L'idéal est une toison gris argent à **bavette noire** ; l'encolure et la nuque doivent elles aussi être noires **jusqu'au garrot**. Tête, pattes et région de la queue sans laine, à poil noir. Le **poil gris piqué** (*graues Stichelhaar*) **n'est pas souhaité**. » [S VLH][S LfL-GGH]

**En profil, tourné vers la droite :**
- **Tête :** entièrement noire, à poil ras, du bout du museau à la nuque, oreilles comprises [S VLH][P].
- **Cape :** l'encolure entière est noire, dessus comme dessous ; le noir descend sur le poitrail en **bavette** jusque entre les antérieurs [S VLH][P : P1, P13, P14].
- **Limite noir/gris :** une **diagonale fondue** qui part du garrot (en haut) et descend en biais vers l'arrière de l'épaule puis vers le coude ; en profil, le noir couvre environ **le premier quart à tiers** de la longueur de la silhouette [P : P1 (bord gris vers x = 640), P13, P14]. Le passage se fait sur **10 à 15 cm**, par des mèches noires qui s'avancent dans le gris, jamais par un bord net [P].
- **Ventre :** une **ligne sombre** continue la bavette sous le ventre (frange noire ou gris très sombre), visible en profil sous le gris du flanc [P : P1, P14]. Sur les bêtes tondues, ce noir du ventre remonte un peu sur le bas du flanc et rejoint le noir des cuisses, à l'intérieur et en bas, au-dessus des pattes [P : P24].
- **Flanc :** gris, plus clair en haut (dos, haut du flanc), plus sombre vers le bas (dégradé naturel de la toison, plus l'ombre) [P].
- **Pattes :** noires de la toison aux onglons [S VLH][P].
- **Queue :** noire, à poil ras [S VLH][S NP2][P : P1].
- **Variation :** certaines bêtes ont le noir qui s'étend sur l'épaule et le flanc (gris foncé), d'autres un gris très clair presque blanc ; les vieilles bêtes grisonnent au museau [P : P2, P5, P13]. On dessine l'idéal.
- **Ni tache blanche, ni étoile, ni liste** : la tête est noire (le blanc sur la tête est le signe du Gotland ou du Romanov, section 9).

### 4.2 Comment les valeurs ont été mesurées

Moyennes de zones de 300 à 18 000 pixels (PIL), avec les 10ᵉ et 90ᵉ percentiles de luminance pour l'ombre et la lumière. Les photos sous ciel couvert (P1) ont une **dominante bleue**, surtout sur le poil noir (reflet du ciel) ; la photo de la GEH (Tutsberg, lumière d'automne diffuse) est la plus neutre.

| Photo | Zone | Moyenne | Ombre (p10) | Lumière (p90) |
|---|---|---|---|---|
| GEH-Tutsberg (brebis, lande, automne) | haut du flanc | `#C3BFBA` | `#9D9C98` | `#DFDAD4` |
| GEH-Tutsberg | milieu du flanc | `#B2ACA4` | `#91897C` | `#CBC8C1` |
| GEH-Tutsberg | encolure noire | `#201F24` | `#0D0C11` | `#3B3B3D` |
| GEH-Tutsberg | face | `#1F2129` | `#121116` | `#33363F` |
| GEH-Tutsberg | corne de brebis | `#767476` | `#4D4B4C` | `#AEAFB4` |
| P1 (brebis, couvert, novembre) | haut du flanc | `#B0AEB6` | `#8A878E` | `#D5D2DD` |
| P1 | milieu du flanc | `#989498` | `#747277` | `#BAB4B4` |
| P1 | bas du flanc | `#69676D` | `#3A3839` | `#959094` |
| P1 | encolure noire | `#111014` | `#09090B` | `#1A191E` |
| P1 | frange noire du ventre | `#1D1D1E` | `#080808` | `#434345` |
| P1 | chanfrein (reflet du ciel) | `#4A5173` | `#292935` | `#8796C1` |
| P1 | patte postérieure | `#1F1E1F` | `#0E0E0E` | `#38383A` |
| P13 (LfL, brebis, couvert) | haut du flanc | `#B7B5B0` | `#9C9B96` | `#CECFCA` |
| P13 | milieu du flanc | `#A29C94` | `#81796E` | `#BDB8B2` |
| P13 | bas du flanc | `#857B6E` | `#5F574A` | `#A3998D` |
| P14 (VLH, bélier) | flanc | `#C7CAC7` | `#A8ACAD` | `#E0E1DB` |
| P14 | bavette | `#323537` | `#121315` | `#6E7679` |
| P14 | face | `#242629` | `#151615` | `#343C3F` |
| P14 | corne (type sombre) | `#3C4044` | `#272C2C` | `#53555C` |
| P7 (soleil bas d'automne) | flanc | `#A18A6F` | `#8A6F5A` | `#BBA38B` |
| P7 | encolure noire | `#111008` | `#060604` | `#1C240D` |
| P4 (brebis, tête) | corne, milieu | `#858B9D` | `#707287` | `#9FA3BC` |
| P4 | corne, anneau clair de la base | `#A1A4B7` | `#84869D` | `#CAD0E0` |
| P4 | corne, pointe | `#5E626C` | `#59535F` | `#6D6A75` |
| P4 | oreille | `#191819` | `#100B11` | `#262628` |
| P4 | laine du cou (grise) | `#97949A` | `#6B6265` | `#BAB9C1` |
| P3 (bélier, tête) | iris | `#988E7C` | `#62635D` | `#D2C2A0` |
| P3 | corne, dessus | `#626361` | `#434542` | `#878B8E` |
| P2 (bélier, printemps) | corne, flanc de la spire | `#4A3C35` | `#261913` | `#836C64` |
| P2 | corne, pointe | `#2A231E` | `#130E08` | `#433E42` |
| P2 | patte | `#101015` | `#070709` | `#191D28` |
| GEH-Beuren (bélier, printemps) | corne (type clair), lumière | `#8A6F5C` | `#4F3524` | `#C2A897` |
| GEH-Beuren | toison de printemps | `#675B55` | `#3E3E32` | `#8C7B73` |
| GEH-Beuren | agneau noir | `#202021` | `#0E0E0E` | `#36373C` |

### 4.3 Palette recommandée (brebis, lumière neutre d'automne)

Valeurs choisies d'après le tableau, la dominante bleue de P1 retirée ; P7 donne la variante chaude (soleil bas).

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Toison, haut du flanc et dos | `#BAB6B0` | `#DCD8D2` (pointes des mèches) | `#94918C` |
| Toison, milieu du flanc | `#ABA69F` | `#C9C5BE` | `#857F77` |
| Toison, bas du flanc | `#857F78` | `#A39D95` | `#5A5652` ; creux entre mèches `#3C3A3B` |
| Stries noires dans les mèches | `#4A484C` | `#6B686B` | `#2A292C` ; traits fins verticaux, plus serrés vers le bas et vers l'épaule |
| Transition épaule (noir fondu dans le gris) | `#5E5C60` | `#8A878A` | `#353438` |
| Cape noire (encolure, bavette) | `#1C1B1F` | `#3B3B3F` (lustre des mèches) | `#0D0C10` |
| Frange du ventre | `#232225` | `#434345` | `#0A0A0B` |
| Face, oreilles, pattes (poil ras) | `#1E1F24` | `#4A5266` (reflet froid sur le chanfrein, le front, l'avant du canon) | `#111115` |
| Intérieur de l'oreille | `#2A2627` | | peau pigmentée, jamais rose |
| Nez (peau nue) | `#2A2B30` | `#5A5D64` (humide) | naseaux `#0B0B0D` |
| Queue | `#1A1A1D` | `#38383C` | `#0C0C0E` |
| Cornes de la brebis | `#7A7876` gris ardoise | `#A8A6A2` (anneau de pousse près de la base, crêtes) | `#4E4C4C` ; pointe `#5C5A5A` |
| Onglons | `#222120` | `#5A5650` (bord usé, poussière) | `#121110` |
| Iris | `#C8B78E` | `#DCCBA4` | cercle limbique `#5E5446` ; pupille `#0E0E10` ; reflet `#F2EEE6` |
| Paupières, cils | `#0E0E10` | | |
| Variante soleil bas (toison) | `#A89279` | `#C4AE94` | `#7D6A57` |

**Bélier :** cornes de type sombre `#3C4044` / `#626361`, crêtes `#878B8E` ; ou de type brun `#6E5646`, crêtes claires `#C2A897`, pointe `#2A231E`. **Agneau :** `#202021`, reflets `#36373C` ; antenais en mue : brun roux `#5A4637` [E, d'après P19].

---

## 5. La tête

- **Forme :** « allongée » (LfL : *der längliche Kopf*), « longue et cunéiforme » (GEH : *länglich und keilförmig*) [S LfL-GGH][S GEH-GGH]. Une tête fine qui s'effile vers un museau étroit, plus longue et plus fine que celle d'un mouton à viande [P : P1, P4, P13].
- **Profil :** chanfrein **droit**, à peine convexe chez la brebis ; un peu busqué chez le bélier [P : P3, P4, P14]. Aucune source ne le précise.
- **Museau :** étroit, lèvres fines et noires, nez noir humide, naseaux en virgule obliques ; lèvre supérieure fendue comme chez tous les moutons [P : P4][E].
- **Œil :** latéral, haut, à environ 40 % de LT en avant de la nuque, juste sous et devant la base de la corne [P : P1, P4]. **Iris clair, jaune paille à ambre pâle**, très visible dans la face noire ; **pupille horizontale**, rectangle aux coins arrondis [P : P3, P4, P1]. Paupière supérieure lourde, qui couvre le haut de l'iris.
- **Oreilles :** « petites, **dressées de biais vers le haut** » (*klein, schräg aufwärts*) [S LfL-GGH][S VLH][S GEH-GGH]. En photo, elles partent **sur le côté**, de l'horizontale à 20° au-dessus, sous et derrière la base de la corne ; en feuille, pointe arrondie, intérieur noir [P : P1, P4, P13]. En profil, l'oreille du côté vu sort derrière l'œil, sous l'arc de la corne, un peu raccourcie par la perspective ; celle de l'autre côté dépasse au-dessus du front.
- **Pas de toupet :** front à poil ras noir ; quelques mèches grises peuvent pousser à la nuque, entre les cornes [P : P1, P5]. (La Moorschnucke, elle, peut avoir une *Schaupe* [S LfL-WHH].)
- **Poils gris :** les vieilles bêtes grisonnent au museau et autour des yeux [P : P5] ; le standard ne veut pas de poil gris piqué [S VLH]. Notre brebis de quatre ans a la tête noire unie.

---

## 6. Les cornes

### 6.1 Ce que disent les sources

- **Les deux sexes portent des cornes** [S VLH][S LfL-GGH][S GEH-GGH][S VNP][S WP-EN].
- **Brebis :** « des cornes recourbées vers l'arrière » (*nach hinten gebogene Hörner*) [S VLH][S LfL-GGH] ; « plus fines, de **section elliptique** » [S GEH-GGH].
- **Bélier :** « en escargot » (*schneckenartige Hörner*) [S VLH][S LfL-GGH] ; « de **section triangulaire**, en spires tournées vers l'extérieur » [S GEH-GGH] ; « les énormes cornes torsadées des béliers » servent d'armoiries dans la lande [S GEH-GGH].
- **Pour les deux :** « assez écartées pour éviter, chez les béliers, que les spires n'entrent dans la tête » [S VLH][S LfL-GGH].
- Les cornes apparaissent la première année ; l'agneau naît sans [S VNP].

### 6.2 La brebis, de profil, tournée vers la droite (on voit sa corne gauche)

D'après P1 (profil strict), P4 (tête de trois quarts), P13, GEH-Tutsberg et la photo de la brebis du VLH :

1. **Base :** sur le haut du crâne, à la nuque, juste au-dessus et un peu en arrière de l'œil ; les deux bases sont proches, séparées par quelques centimètres de poil noir [P].
2. **Départ :** la corne **monte** sur 3 à 5 cm, légèrement en arrière (à 60–70° au-dessus de l'horizontale) [P : P4, Tutsberg].
3. **Courbe :** elle s'incline aussitôt **vers l'arrière** (vers la gauche de l'image) en un arc régulier, en passant au-dessus de l'oreille [P].
4. **Pointe :** elle redescend et finit **en arrière et en bas**, à peu près au niveau de l'œil ou de la base de l'oreille, à 3–6 cm derrière l'oreille, **pointant vers l'arrière-bas et un peu en dehors** [P : P1, P4]. L'arc fait **100 à 150°** : une faucille, jamais un cercle fermé [P].
5. **Taille :** corde de 17–19 cm (0,25–0,28 H ; environ 0,8 LT), 20 à 25 cm le long de la courbe [P][E] ; épaisseur à la base ≈ 3 cm (≈ 0,13 LT), qui s'effile régulièrement vers une pointe fine [E, d'après P4].
6. **Écart :** elle ne touche jamais la nuque ni l'encolure ; en profil, on voit du noir (poil de la nuque) entre la corne et le cou [P].
7. **Corne de l'autre côté :** presque cachée ; son arc dépasse à peine au-dessus de celui de la corne vue, un peu en avant [P : P1].
8. **Variation :** certaines brebis ont des cornes presque horizontales vers l'arrière (P13), d'autres plus redressées puis tombantes, la pointe à hauteur de l'oreille (Tutsberg) ; d'autres s'écartent plus sur le côté (P5) ; chez les plus vieilles, la pointe revient un peu vers l'avant sous l'oreille et la corne dessine un « C » ouvert vers l'avant, sans jamais faire un tour (P24) [P]. On dessine la faucille simple de P1.

**Surface :** lisse à finement annelée ; un **anneau plus clair**, gris blanchâtre, près de la base (la pousse récente), et des anneaux de croissance plus sombres plus loin [P : P4]. **Section :** ovale aplatie [S GEH-GGH]. **Couleur :** gris ardoise moyen, plus clair à la base, plus sombre et plus lisse vers la pointe (palette en 4.3) [P].

### 6.3 Le bélier (pour mémoire)

- **Trajet, vu de profil, tourné vers la droite :** la corne part de la nuque, monte, part vers l'arrière, descend derrière l'oreille, passe **sous l'oreille** vers l'avant, puis la pointe remonte et **s'écarte vers l'extérieur**, au niveau de la joue ou un peu plus bas [P : P3, P14, P2, P8].
- **Nombre de tours :** aucune source n'en donne. Photos : environ **1 tour** chez un jeune bélier de vente (P14), **1 à 1¼** chez un bélier adulte (P2, P3), avec la pointe qui part loin sur le côté (P2, P8) [P].
- **Taille :** en profil, la spire fait **0,75–0,8 LT** de diamètre (P14 : 145 × 165 px pour une tête de ≈ 200 px), soit 22–25 cm ; elle encadre l'œil en arrière et l'oreille en dessous [P].
- **Écart :** la spire est **ouverte** et se détache de la tête en s'enroulant (un tire-bouchon qui s'écarte) [S VLH][P : P8, vue de dessus] ; de face, les pointes dépassent nettement la largeur de la tête [P : P2, P16].
- **Section triangulaire** [S GEH-GGH] ; **grosses annelures transversales** serrées, plus profondes vers la base ; pointe lisse [P : P2, P8].
- **Couleur :** deux types sur les photos : **gris anthracite à presque noir** (béliers de vente de Müden, P14, P16 ; tête P3) ou **brun corne** à crêtes claires (P2, GEH-Beuren) [P].

---

## 7. Les pattes et les onglons

- « Fondation correctement posée, **fine et sèche**, onglons **durs** » [S LfL-GGH][S VLH] ; « un fondement correct et des onglons durs sont une condition impérative » [S VLH], parce que les troupeaux **marchent** chaque jour des kilomètres (*Marschtauglichkeit*) [S GEH-GGH][S LfL-WGH].
- **En dessin :** pattes **noires**, à poil ras, **minces** (canon antérieur large d'environ 0,06 H [E]), longues sous la toison ; aplombs droits, vus de profil : antérieur vertical sous l'épaule, canon postérieur presque vertical sous la fesse [P : P1, P13, P14].
- **Onglons :** noirs à ardoise, petits et pointus, les deux doigts serrés ; ergots sombres juste derrière le paturon [P : P1, P14][E]. Hauteur de l'onglon ≈ 0,05 H [E].
- **Marche :** sur P7 et P14 (bélier qui marche), le pas est long et léger ; le paturon reste souple [P].

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Un animal de troupeau mené :** depuis des siècles, la Heidschnucke est **gardée par un berger** sur la lande ouverte et **rentrée la nuit** [S GEH-GGH][S VNP]. Aujourd'hui encore, les troupeaux du VNP sortent **8 à 12 heures par jour** et font **des dizaines de kilomètres** ; chaque troupeau compte environ 350 brebis, jusqu'à 400 agneaux et quelques béliers, avec un berger ou une bergère et **plusieurs chiens** [S VNP].
- **Les chiens « courent le sillon »** (*Furche laufen*) : ils patrouillent sans fin le long d'une limite (bord de champ, chemin) que le berger leur montre, pour que le troupeau ne la passe pas [S VNP]. Le troupeau broute donc **en masse serrée**, contre une limite invisible.
- **Marcheuse :** la « Marschtauglichkeit » (aptitude à marcher) est un objectif d'élevage, « parce que les bêtes vont chaque jour à pied de la bergerie aux pâtures, à plusieurs kilomètres, et reviennent le soir » [S GEH-GGH].
- **Farouche :** « La Heidschnucke est un animal **très craintif**, qui ne se laisse pas caresser. Elle **prend la fuite aussitôt** qu'un inconnu s'approche. » [S VNP]
- **Grignoteuse :** elle broute en marchant herbes, jeunes pousses de bruyère et de bouleau (« Schnucke », de *snickern*, grignoter) [S VNP][S NP2][S LfL-GGH] ; autrefois elle grattait la neige gelée pour trouver sa nourriture [S GEH-GGH].
- **Frugale et robuste :** son rumen s'adapte aux maigres pâtures de la lande (volume de liquide ruminal plus grand, rapporté au poids, que chez le Mérinos) [S Weyreter1986].
- **Bruyante à la tonte** : les bêtes s'agitent et bêlent [S dpa].
- **Saisons :** chaleurs seulement quelques semaines en automne ; agneaux à la fin de l'hiver ou au début du printemps [S GEH-GGH] (Wikipédia FR : lutte en décembre, agnelage d'avril à juin [S WP-FR]). Un, deux, rarement trois agneaux [S WP-FR][S VLH].
- **Longévité :** six à huit ans pour les brebis, un peu plus pour les béliers [S dpa] ; quinze ans selon le Naturpark [S NP1].

### 8.2 Ce qui vaut pour les moutons en général (voir `docs/animation-animale.md`)

- **Voix :** aucun cri propre à la race n'est décrit. Bêlement aigu bouche ouverte (isolement, stress), grave bouche fermée (brebis et agneau) (§ 3).
- **Journée :** 9 à 10 h de broutage, 5 à 6 h de rumination, surtout couchée (§ 2). Chez la Heidschnucke, la rumination de jour est plus courte (elle est menée) et se fait surtout la nuit à la bergerie [E, d'après S VNP].
- **Oreilles :** horizontales au calme, en arrière quand elle a peur (§ 4) ; chez elle, les oreilles « dressées de biais » partent vite en arrière.
- **Béliers :** combats tête contre tête après un recul, comme chez tous les moutons cornus ; rien de propre à la race n'est décrit.
- **Agneaux :** sauts et jeux, comme partout ; les agneaux noirs au milieu des brebis grises font un contraste fort [P : P20].

### 8.3 Ce que cela donne dans Mouflon (propositions)

- **Grégaire et menée :** suit vite (`walk` contagieux élevé), se regroupe souvent, broute serrée ; marche plus volontiers que le Mannequin (de longues traversées au pas sur un rebord).
- **Craintive :** l'alerte (`look`) et la fuite (`run`) se propagent fort ; elle détale tôt quand le pointeur approche, puis se retourne pour regarder ; elle **ne vient pas** renifler le pointeur.
- **Peu de jeux d'adultes**, peu de charges (sauf un bélier) ; les agneaux noirs, eux, jouent.
- **Robuste :** tourne le dos au vent sans se blottir.

---

## 9. Les dix détails, et les erreurs à éviter

### 9.1 Les dix détails qui font une Heidschnucke grise

1. **La cape noire :** tête, encolure entière jusqu'au garrot et bavette noires ; le noir se **fond** en diagonale dans le gris de l'épaule [S VLH][P].
2. **Une toison gris argent faite de longues mèches raides qui pendent**, rayées de poils noirs, au bord effiloché ; ni boucles, ni bloc [S LfL-GGH][S WP-DE][P].
3. **Des pattes noires, fines, longues, sans laine**, avec genou et jarret bien visibles [S VLH][S LfL-GGH][P].
4. **Des cornes chez les deux sexes :** la brebis en **faucille vers l'arrière**, fine, ovale ; le bélier en **escargot** ouvert, triangulaire, écarté de la tête [S VLH][S GEH-GGH].
5. **De petites oreilles noires portées de biais**, sur le côté, un peu relevées [S LfL-GGH][P].
6. **Une tête noire, longue et cunéiforme**, à poil ras, chanfrein droit, sans toupet ni tache blanche [S GEH-GGH][P].
7. **Une courte queue noire à poil ras** (moins d'un quart de H), qui pend contre la fesse et s'arrête haut au-dessus du jarret [S VLH][S WP-NEST][P].
8. **Une ligne sombre sous le ventre** qui prolonge la bavette [P].
9. **Des yeux clairs, jaune paille**, dans la face noire [P].
10. **Une silhouette de taille moyenne, profonde mais haute sur pattes** ; et dans le troupeau, des **agneaux noirs bouclés** [S GEH-GGH][S LfL-GGH][P].

### 9.2 Ce qui en ferait une autre race

| Erreur | Ce qu'on obtiendrait | Pourquoi |
|---|---|---|
| Brebis sans cornes, toison grise en **boucles serrées et brillantes**, tête noire parfois **marquée de blanc** (front, museau) | un **Gotland** | le Gotland est « sans cornes », « fin d'os », à toison « fine, longue, lustrée, dense » avec une **boucle régulière** (*purl*), de l'argent au charbon, 29–34 µm [S WP-Got-EN][S WP-Got-DE] ; la Heidschnucke a de longs jarres raides de 38–39 µm et des cornes |
| Toison grise **courte et dense**, tête noire avec **liste ou taches blanches**, sans cornes | un **Romanov** | Romanov : naît noir et devient gris comme elle, mais toison mixte courte et serrée (laine grise et jarres noirs), tête souvent marquée de blanc, prolificité énorme [S WP-Rom][P : Romanov 1–3] |
| Tout blanc, cornes en faucille ou en escargot | une **Weiße Gehörnte Heidschnucke** | [S LfL-WGH] |
| Tout blanc, sans cornes, petit, éventuellement un toupet | une **Moorschnucke** | [S LfL-WHH][S GEH-WHH] |
| Laine sur la face ou les pattes, toupet | une race de plaine laineuse | la tête et les pattes sont « unbewollt » [S VLH] |
| Longue queue laineuse jusqu'au jarret | n'importe quel mouton à queue longue | race nordique à **queue courte** [S LfL-GGH][S WP-NEST] |
| Bord net entre noir et gris, comme un col peint | un dessin, pas une toison | le passage est fondu sur 10–15 cm [P] |
| Corps gris uni sans aucune strie, ou blanc pur | une autre toison | le gris est un mélange de fibres noires et blanches [S Kurt2008][P] |
| Poils gris piqués sur la tête noire | un défaut du standard | « graues Stichelhaar unerwünscht » [S VLH] |
| Corps brun roux | un **antenais** en train de changer de couleur | le brun est une étape de l'agneau [S GEH-GGH] |
| Spirale du bélier collée à la joue, ou cornes qui s'enroulent vers la face | un défaut (cornes qui entrent dans la tête) | le standard veut des cornes « assez écartées » [S VLH] |
| Pattes courtes noyées dans la laine | un Nez noir du Valais ou un mouton de foire | « fondation fine et sèche », pattes hautes [S LfL-GGH][P] |

**Les voisines proches** à garder en tête : le **Gotland** (gris, nordique, mais sans cornes, en boucles), le **Romanov** (gris, nordique, mais court, dense, tête marquée de blanc), la **Wrzosówka** polonaise (mouton de lande gris très semblable : agneaux noirs qui grisonnent, toison mixte grise [S Springer-Wrz]), et le **Rauhwolliges Pommersches Landschaf** (gris bleuté, mais tête et pattes grises, sans la cape noire nette) [S Kurt2008]. Le **Skudde** du chantier est bien plus petit et le plus souvent blanc.

---

## 10. Sources, photos, contradictions

### 10.1 Sources écrites (ouvertes)

**Standard et associations de race**
- **[S VLH]** Verband Lüneburger Heidschnuckenzüchter e.V., *Zuchtprogramm Graue Gehörnte Heidschnucke* (adopté le 26 novembre 2021, en vigueur au 1ᵉʳ janvier 2022 ; tableau des performances, objectif d'élevage avec la couleur idéale, effectifs) : https://www.schafzucht-niedersachsen.de/Schafzucht-Verbaende-Niedersachsen/index.php?option=com_phocadownload&view=category&download=290:zuchtprogamm-graue-gehoernte-heidschnucke&id=5&Itemid=677&lang=de
- **[S VLH-web]** VLH, page « die Heidschnucke » (histoire, effectifs 1848–2002, usage de la laine) : https://www.schafzucht-niedersachsen.de/Schafzucht-Verbaende-Niedersachsen/index.php?option=com_content&view=article&id=22&Itemid=664&lang=de ; page d'accueil du VLH (77ᵉ Heidschnuckentag à Müden, 9 juillet 2026) : https://www.schafzucht-niedersachsen.de/Schafzucht-Verbaende-Niedersachsen/index.php?option=com_content&view=article&id=20&Itemid=648&lang=de

**Instituts et conservatoires**
- **[S LfL-GGH]** Bayerische Landesanstalt für Landwirtschaft, fiche *Graue Gehörnte Heidschnucke* (VDL 2005) : https://www.lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_graue_gehoernte_heidschnucke.pdf
- **[S LfL-WGH]** LfL, *Weiße Gehörnte Heidschnucke* : https://lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_weisse_gehoernte_heidschnucke.pdf
- **[S LfL-WHH]** LfL, *Weiße Hornlose Heidschnucke (Moorschnucke)* : https://lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_wei__e_hornlose_heidschnucke.pdf
- **[S GEH-GGH]** Gesellschaft zur Erhaltung alter und gefährdeter Haustierrassen, *Graue Gehörnte Heidschnucke* (sections des cornes, couleur des agneaux, histoire, Marschtauglichkeit) : https://www.g-e-h.de/index.php/rassebeschreibungen/72-rassebeschreibungen-schafe/747-graue-gehörnte-heidschnucke
- **[S GEH-WGH]** GEH, *Weiße Gehörnte Heidschnucke* : https://www.g-e-h.de/index.php/rassebeschreibungen/72-rassebeschreibungen-schafe/111-weisse-gehoernte-heidschnucke
- **[S GEH-WHH]** GEH, *Moorschnucke* : https://www.g-e-h.de/index.php/rassebeschreibungen/72-rassebeschreibungen-schafe/101-moorschnucke

**Gestionnaires de la lande, presse**
- **[S VNP]** Verein Naturschutzpark / VNP Stiftung, dépliant *Heidschnucken* (troupeaux, berger et chiens, 8–12 h par jour, tonte en juin, animal craintif) : https://www.verein-naturschutzpark.de/wp-content/uploads/VNP_Faltblatt_Heidschnucken_Einzelseiten_lr.pdf
- **[S NP1]** Naturpark Lüneburger Heide, *Heidschnucke (German grey heath)* (fiche : garrot 50–60 cm, 45–60 kg, 15 ans) : https://naturpark-lueneburger-heide.de/en/regional-specialities/products-from-the-heath/heidschnucke-german-grey-heath
- **[S NP2]** Naturpark Lüneburger Heide, *Heidschnucke, wolf, black grouse & co.* (tonte en mai, tête, pattes, queue noires) : https://naturpark-lueneburger-heide.de/en/nature-and-culture/plants-and-animals/heidschnucke-german-grey-heath-sheep-wolf-black-grouse-co
- **[S dpa]** dépêche dpa reprise par news.de, *450 Heidschnucken werden geschoren* (Soltau, 4 juin 2026) : https://www.news.de/panorama/859679488/450-heidschnucken-werden-geschoren-news-aus-soltau-niedersachsen-zu-agrar-dpa-ni/1/

**Études**
- **[S Kurt2008]** G. Kurt et H. R. Brandt, « Development and characterization of lambs' coats of East Prussian Skudden and Rough-coated Pomeranian Landrace sheep », *Archiv für Tierzucht* 51 (2008), 467–478 (types de fibres, jarres « kempy » de la Heidschnucke grise, couleur grise par fibres pigmentées et non pigmentées) : https://aab.copernicus.org/articles/51/467/2008/aab-51-467-2008.pdf
- **[S Weyreter1986]** H. Weyreter et W. von Engelhardt, « Adaptation of Heidschnucken and Merino sheep to pasture conditions in the heather region of Northern Germany », *Z. Tierphysiol. Tierernähr. Futtermittelkd.* 56 (1986), 117–122 (notice AGRIS) : https://agris.fao.org/search/en/records/64774ef7f2e6fe92b3641bc7

**Encyclopédies et races voisines**
- **[S WP-DE]** https://de.wikipedia.org/wiki/Heidschnucke · **[S WP-EN]** https://en.wikipedia.org/wiki/Heidschnucke · **[S WP-FR]** https://fr.wikipedia.org/wiki/Heidschnucke (« Jaglu »)
- **[S WP-WPH]** https://en.wikipedia.org/wiki/White_Polled_Heath
- **[S WP-NEST]** https://en.wikipedia.org/wiki/Northern_European_short-tailed_sheep (queue courte en fanon, 13 vertèbres)
- **[S WP-Got-EN]** https://en.wikipedia.org/wiki/Gotland_sheep · **[S WP-Got-DE]** https://de.wikipedia.org/wiki/Gotlandschaf
- **[S WP-Rom]** https://en.wikipedia.org/wiki/Romanov_sheep
- **[S wissen]** wissen.de, *Heidschnucke* : https://www.wissen.de/lexikon/heidschnucke
- **[S Springer-Wrz]** description de la Wrzosówka, « petit mouton à toison mixte grise ; les agneaux naissent noirs et grisonnent avec l'âge » (matériel supplémentaire 3 d'un article de *Scientific Reports*, 2025) : https://media.springernature.com/original/springer-static/esm/art%3A10.1038%2Fs41598-025-97931-2/MediaObjects/41598_2025_97931_MOESM4_ESM.pdf

**Cherchées sans succès :** la fiche de l'Oklahoma State University (https://breeds.okstate.edu/sheep/graue-gehoernte-heidschnucke-sheep.html) répond 403, et son archive est bloquée ; un résumé de recherche lui prête 25 cm de jarre et 6 cm de duvet, non vérifiés, donc **non utilisés**. La fiche TGRDEU (genres.de) liée par la GEH répond 404. Aucune étude biométrique de la race n'a été trouvée.

### 10.2 Photos regardées

Toutes téléchargées hors du dépôt (`scratchpad/photos/heidschnucke/`), regardées, et pour la plupart mesurées.

| N° | Page | Ce qu'elle montre |
|---|---|---|
| P1 | https://commons.wikimedia.org/wiki/File:Graue_geh%C3%B6rnte_Heidschnucke_qtl1.jpg | **brebis de profil strict, tournée vers la droite**, Wildpark Lüneburger Heide, 15 novembre 2009 ; la photo de référence : proportions, cornes en faucille, cape noire, frange noire du ventre, **queue noire** visible derrière la fesse, mèches pendantes rayées |
| P2 | https://commons.wikimedia.org/wiki/File:Graue_Heidschnucke_im_%22Wildpark_Schwarze_Berge%22.jpg | bélier adulte de trois quarts, toison de printemps très longue, presque noire à l'avant, cornes brunes en escargot (1¼ tour), pointes écartées, crinière |
| P3 | https://commons.wikimedia.org/wiki/File:HeidschnKopf.JPG | tête de bélier : cornes anthracite à annelures, iris jaune paille, pupille horizontale, chanfrein |
| P4 | https://commons.wikimedia.org/wiki/File:Heidschnucke.jpg | tête de brebis de trois quarts : cornes en faucille vers l'arrière avec anneau clair à la base, oreilles sur le côté, œil ambre, mèches grises du cou |
| P5 | https://commons.wikimedia.org/wiki/File:Kopf_Grauer_geh%C3%B6rnter_Heidschnucke.JPG | tête de face (vieille bête) : cornes écartées, museau grisonnant |
| P6 | https://commons.wikimedia.org/wiki/File:Seitenansicht_Graue_geh%C3%B6rnte_Heidschnucke.JPG | bête qui **broute**, de profil vers la gauche : encolure basse, antérieurs droits, toison passée, queue noire |
| P7 | https://commons.wikimedia.org/wiki/File:Undeloh-wilsede-einzelne-heidschnucke.JPG | brebis qui marche sur un chemin pavé, soleil bas d'automne, troupeau derrière |
| P8 | https://commons.wikimedia.org/wiki/File:Widder_Graue_geh%C3%B6rnte_Heidschnucke_MA.JPG | bélier vu d'en haut : spires écartées de la tête, annelures, pointes vers l'extérieur |
| P9 | https://commons.wikimedia.org/wiki/File:Mannheim_Graue_geh%C3%B6rnte_Heidschnucke.JPG | bête qui broute, toison hirsute |
| P10 | https://commons.wikimedia.org/wiki/File:Herzogenriedpark_Mannheim_Graue_geh%C3%B6rnte_Heidschnucke.JPG | brebis de dos, cornes, pattes noires |
| P11 | https://commons.wikimedia.org/wiki/File:Berlin,_Internationale_Gr%C3%BCne_Woche,_Heideschnucke.jpg | bélier en toison très longue (mèches pendantes de 20 cm et plus) |
| P12 | https://commons.wikimedia.org/wiki/File:Heidschnucke_March2011.jpg | brebis en toison de mars, de côté ; bélier sombre devant |
| P13 | photo de la fiche LfL (https://www.lfl.bayern.de/mam/cms07/itz/dateien/schafrassen_rassebeschr_graue_gehoernte_heidschnucke.pdf), reprise dans le programme du VLH | **brebis de profil vers la gauche**, troupeau en bergerie : cape noire jusqu'au garrot, toison argent, pattes, cornes recourbées ; mesurée |
| P14 | photo de gauche du programme d'élevage du VLH (lien [S VLH]) | **bélier de profil, tourné vers la droite**, en marche : cornes anthracite en escargot, encolure et bavette noires, ventre sombre ; mesuré |
| P15 | https://commons.wikimedia.org/wiki/File:Heidschnuckenbock-Auktion_2023_(2).JPG | béliers de vente vus de derrière : **queues noires** courtes entre les fesses grises |
| P16 | https://commons.wikimedia.org/wiki/File:Heidschnuckentag_A2005.jpg | béliers de trois quarts au Heidschnuckentag de Müden : cornes noires, cape noire |
| P17 | https://commons.wikimedia.org/wiki/File:Heidschnuckenbock-Auktion_2005_(4).jpg | deux béliers de profil qui marchent |
| P18 | https://commons.wikimedia.org/wiki/File:Heidschnuckenbock-Auktion_2023_(1).JPG | béliers de face (un plus clair) |
| P19 | https://commons.wikimedia.org/wiki/File:Undeloh-wilsede-strasse-heidschnucke-ohrmarke.JPG | troupeau sur un chemin : adultes gris, **antenais bruns** |
| P20 | https://commons.wikimedia.org/wiki/File:Neuenkirchen_(LH)_-_Sch%C3%A4ferhof_23_ies.jpg | grand troupeau sous les pins : **agneaux noirs bouclés** parmi les brebis grises |
| P21 | https://commons.wikimedia.org/wiki/File:Neuenkirchen_(LH)_-_Sch%C3%A4ferhof_24_ies.jpg | brebis de près, corne, toison |
| P22 | https://commons.wikimedia.org/wiki/File:Heidschnucke-Horst.jpg | brebis sombres dans la neige, de face : oreilles, iris |
| P23 | https://commons.wikimedia.org/wiki/File:Heidschnucke_0108_V002A5.jpg | bélier dans un chaume (noir et blanc) : silhouette, cornes écartées |
| P24 | https://commons.wikimedia.org/wiki/File:Geschorene_Heidschnucken_25.jpg | **troupeau tondu** : le corps réel (pattes longues, corps mince), cape noire, ventre et cuisses noirs, cornes de brebis en « C », un agneau brun |
| GEH-Tutsberg | https://www.g-e-h.de/images/bilder/Rassebeschreibungen/GGH_AH_HofTutsberg_Bremken_af%2063.jpg | **brebis dans la bruyère en automne**, lumière neutre : couleurs de référence, cornes, cape |
| GEH-Beuren | https://www.g-e-h.de/images/bilder/Rassebeschreibungen/Graue%20gehornte%20Heidschnucke%20Bock_Milerski%20Beuren%202011%204.jpg | bélier en toison de printemps, cornes brunes, et **agneau noir** couché |
| Planche | https://commons.wikimedia.org/wiki/Category:Heidschnucke et https://commons.wikimedia.org/wiki/Category:Heidschnucke_(L%C3%BCneburger_Heide) | une centaine de vignettes regardées en planche (troupeaux, bergers, tonte, ventes de béliers) |
| Romanov 1–3 | https://commons.wikimedia.org/wiki/Category:Romanov_sheep (fichiers *Mouton Romanov.jpg*, *Moutons romanov SIA.jpg*, *Romanov sheep.jpg*) | comparaison : toison courte et dense, tête noire marquée de blanc, sans cornes |
| Gotland 1–2 | https://commons.wikimedia.org/wiki/Category:Gotland_sheep (fichiers *Detmold - 2025-09-06 - Gotländisches Pelzschaf (DSC06191).jpg*, *Gotland pelt sheep.jpg*) | comparaison : boucles serrées grises, tête noire, sans cornes |

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Contradiction |
|---|---|
| Hauteur au garrot | brebis 65–70, béliers 70–75 cm [S VLH][S LfL-GGH][S GEH-GGH] contre 50–60 cm pour « la » Heidschnucke [S NP1]. Le standard l'emporte ; le Naturpark parle sans doute de bêtes de lande plus légères, ou se trompe. |
| Poids | brebis 45–55 kg [S VLH] ; 45–60 kg sans distinction de sexe [S NP1] ; « une quarantaine de kilogrammes en moyenne » [S WP-FR]. Les poids ont « presque doublé » au XXᵉ siècle par sélection [S WP-DE], ce qui explique peut-être les chiffres bas. |
| Taille relative | « la plus petite race » [S wissen] contre « la plus grande des trois Heidschnucken, de taille moyenne parmi les moutons » [S GEH-GGH]. |
| Toison des béliers | 4,0 kg (*Altböcke*) [S LfL-GGH] ; 2,5–3,0 kg « Böcke » [S GEH-GGH], chiffre que le VLH donne pour les antenais [S VLH]. |
| Changement de couleur | commence la première année, gris après la première tonte [S LfL-GGH][S VLH] ; « après un an » [S VNP] ; « dans la deuxième année » [S WP-DE][S WP-EN][S WP-FR][S NP2] ; « du noir au brun puis au gris dans les deux premières années » [S GEH-GGH]. |
| Mois de la tonte | juin [S VNP][S dpa] ; mai [S NP2]. |
| Longévité | 6–8 ans, béliers un peu plus [S dpa] ; 15 ans [S NP1]. |
| Couleur de la toison | « gris argent », « pas trop claire », cape noire jusqu'au garrot [S VLH] ; « gris à gris foncé, à bavette noire », sans parler de l'encolure [S GEH-GGH]. Les photos montrent toute la gamme. |
| Cornes des brebis blanches (WGH) | « en faucille, recourbées vers l'arrière » [S LfL-WGH][S GEH-WGH] contre « chez les blanches, seuls les béliers ont des cornes » [S wissen] (qui confond sans doute avec la Moorschnucke). |
| Garrot de la Moorschnucke | brebis 60–65, béliers 65–70 cm [S LfL-WHH] ; 50–55 cm [S GEH-WHH] ; brebis 45–50, béliers 55–60 cm [S WP-WPH]. |
| Saison de lutte | « quelques semaines en automne » [S GEH-GGH] ; décembre [S WP-FR]. |
| Origine | « descend du mouflon de Corse et de Sardaigne » [S VLH-web][S GEH-GGH][S VNP][S NP2]. Les moutons domestiques viennent du mouflon d'Asie ; le mouflon de Corse est lui-même un mouton domestique très ancien retourné à l'état sauvage (voir `docs/races/mouflon-corse.md`). Cette « origine » est une tradition, pas un fait à dessiner. |
| Cornes du bélier : nombre de tours | aucune source ; photos : 1 à 1¼ tour (section 6.3). |
| Longueur des mèches | aucune source fiable ouverte (section 3.3). |
| Mesures du corps (longueur, tour de poitrine, tête, oreille, queue) | aucune source ; tout ce qui est en section 2.2 est mesuré sur photo [P] ou estimé [E]. |
