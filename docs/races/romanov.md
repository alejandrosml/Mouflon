> Fiche de la race « Romanov », préparée le 3 novembre 2026 pour dessiner un futur `pets/romanov` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Romanov : fiche pour un dessin de profil

**Noms :** романовская порода овец, « романовская овца » (RU) · Romanov (FR, EN, IT) · Romanow-Schaf (DE) · romanovská ovce (CZ) · owca romanowska (PL). Le nom vient de l'ancien district de Romanov-Borissoglebsk (aujourd'hui Toutaïev), dans la région de Iaroslavl, sur la haute Volga, et non de la dynastie [S WP-RU][S FAO][S USAMV] (Wikipédia FR dit le contraire, section 10.3).
**Groupe :** moutons nordiques à queue courte ; en Russie, race « grossière » (грубошёрстная) à fourrure et à viande (овчинно-мясная, « шубная ») [S FAO][S WP-RU][S Furaeva2025].
**Rédigée :** le 3 novembre 2026, d'après le règlement russe de pointage de la race (arrêté du ministère de l'Agriculture n° 540 du 2 décembre 2016, qui fixe le type voulu), trois études de mensurations de Iaroslavl (163 béliers, 756 brebis, croissance des agneaux), la monographie de la FAO sur les races de l'ex-URSS, les travaux de l'INRA (introduction en France en 1963, comportement), la fiche de l'ASSONAPA, et une vingtaine de photographies regardées de près (section 10).

**Étiquettes :**
- **[S …]** : une source écrite, nommée en section 10.1.
- **[P]** : observé ou mesuré sur une photo (section 10.2) ; pixels comptés, couleurs moyennées sur des zones de 100 à 50 000 pixels avec PIL (10ᵉ et 90ᵉ percentiles de luminance pour l'ombre et la lumière).
- **[E]** : mon estimation (anatomie générale du mouton, recoupée sur les photos). Aucune source ne la donne.
- **H** = hauteur au garrot (os) ; **LT** = longueur de la tête (nuque → bout du museau).

---

## 0. En bref

- **L'animal à dessiner :** une **brebis adulte de trois ans**, début février, environ **trois mois et demi après la tonte d'octobre** (la race est tondue trois fois par an) : toison courte, serrée, **gris bleuté**, en petites boucles d'un centimètre ; de **profil strict, tournée vers la droite** (on voit son flanc gauche), tête haute, oreilles en alerte.
- **En une phrase :** un mouton de taille moyenne, rond comme un tonneau sur des **pattes noires assez hautes et fines**, couvert d'une **toison gris bleuté, mate et dense**, faite de duvet blanc qui dépasse des jarres noirs ; une **tête noire, sèche, longue, au chanfrein busqué**, marquée d'une **liste blanche** qui descend du toupet vers le front ; de **grandes oreilles noires dressées de biais**, très mobiles ; **gros yeux saillants** ; **aucune corne** ; une **queue courte** (moins de 13 cm) couverte de laine sombre.
- **Ce qui la distingue de ses voisines grises :** la Heidschnucke a des cornes, une toison de longues mèches raides et une cape noire jusqu'au garrot ; le Gotland a une toison longue, brillante, en anglaises régulières. Le Romanov a une toison **courte, mate, bouclée serré**, gris bleuté jusqu'aux oreilles, et le bélier une **crinière noire** (section 9).
- **Sa vie :** la brebis la plus prolifique de Russie : deux à quatre agneaux par portée, nés **noirs** et qui grisonnent en quelques mois ; l'animal est **plus craintif de l'homme** que les races de plaine (étude de l'INRA, section 8) [S FAO][S AFS1988][S LeNeindre1993].

**Palette de base** (détails et mesures en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Toison, haut du flanc et dos | `#A9A79F` | `#CBC9C2` | `#85827A` |
| Toison, milieu et bas du flanc | `#8A8982` | `#ADADA6` | `#5F5E58`, creux `#45443F` |
| Reflet bleuté (là où la toison s'ouvre) | `#7D8590` | `#A3ABB6` | `#545A63` |
| Tête, oreilles, pattes (poil ras noir) | `#212226` | `#3D3E43`, reflet froid `#4A5263` | `#121315` |
| Liste blanche | `#EDEEEC` | `#FAFBFB` | `#AEB2B2` |
| Queue (laine sombre) | `#45433F` | `#64625D` | `#2F2D2A` |
| Iris | `#5A3E26` (brun) | `#9A7448` | pupille `#0E0E10` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 La race, en quelques lignes

- Sortie, à la fin du XVIIᵉ ou au XVIIIᵉ siècle, des moutons nordiques à queue courte des paysans de la haute Volga, sélectionnés pour la **pelisse** (le « шуба » russe, la touloupe) ; première mention écrite en **1802** [S FAO][S WP-RU][S USAMV]. Premier standard « de type normal » écrit par P. N. Koulechov en **1908** [S WP-RU].
- **Prolificité record** : en moyenne 2,15 agneaux par agnelage sur 110 000 agnelages (Korenev 1964), jusqu'à 9 ; 230–270 % de prolificité, 2 agnelages par an ou 3 en 2 ans [S FAO][S WP-RU] ; plus de 3 agneaux par portée chez les brebis adultes en France [S AFS1988] ; 180–320 %, en moyenne 278 % en Italie [S ASSONAPA].
- **Effectifs :** 523 008 têtes en 1980 en URSS [S FAO] ; **51 100** au 1ᵉʳ janvier 2024 en Russie, dont 70 % dans le district fédéral central (surtout Iaroslavl) [S Furaeva2024].
- **En France :** importée par l'INRA en **1963** ; 15 000 brebis de race pure et 80 000 brebis croisées F1 en 1987 [S AFS1988] ; base de la souche INRA 401, devenue la **Romane** (Romanov × Berrichon du Cher) [S ProdAnim][S Patre] ; utilisée en croisement « à double étage » (brebis F1 Romanov × race locale, puis bélier de boucherie) [S WP-FR].
- Elle a servi à fixer le **Gotland** suédois, croisé avec des Romanov et des Karakul dans les années 1920–1930 [S WP-Got][S WP-FR] : d'où la ressemblance (section 9).

### 1.2 Pourquoi une brebis adulte

1. **Tout le motif de la race se voit sur elle :** toison gris bleuté, tête noire à liste blanche, pattes noires, oreilles dressées, pas de cornes. Le standard décrit d'abord la brebis : le bélier n'en diffère que par la crinière, la peau plus épaisse et l'ossature plus lourde [S Ordre540][S Furaeva2025].
2. **Un agneau serait moins lisible.** L'agneau naît **entièrement noir** (parfois une petite étoile) [S WP-RU][S USAMV][P : R8] : en petit, ce n'est plus qu'un « agneau noir » comme dans n'importe quelle race. Le gris bleuté, qui fait le Romanov, n'arrive qu'à trois à cinq mois.
3. **Un bélier se confondrait avec une race à crinière ou à cape** (Heidschnucke) : la crinière noire mange l'encolure et le poitrail (R5, R19). Il reste une bonne variante (section 1.4).
4. **La brebis, c'est la race :** ce sont ses portées de trois ou quatre agneaux noirs qui font sa réputation ; une brebis grise suivie d'agneaux noirs est l'image du Romanov [P : R7, R8].

### 1.3 L'individu exact

- **Brebis de 3 ans**, adulte faite (couleur définitive, ossature finie) ; **H = 65 cm**, **55 kg** : la moyenne des brebis mesurées à Iaroslavl (65,2 cm) [S RUDN2022] et le poids moyen des brebis des élevages russes en 2023 (57 kg) [S Furaeva2024], à la limite de la classe « élite » du standard (55 kg) [S Ordre540].
- **Saison :** début février. Les éleveurs russes tondent trois fois par an, vers mars, juin et octobre, à cause de la mue saisonnière [S WP-RU] (« 3 ou 4 tontes » [S FAO]). Le standard juge la toison **trois mois après la tonte** [S Ordre540] : c'est l'état « de référence ». Notre brebis a environ 3,5 mois de pousse : duvet de **6 à 7 cm**, jarres de **3 à 4 cm** [S Ordre540 pour 3 mois][E pour 3,5 mois].
- **Marques :** une **liste blanche étroite** (проточина) du toupet au milieu du front, qui s'arrête au-dessus de la ligne des yeux ; museau noir ; pattes et queue sans blanc (le blanc y est permis, mais on garde le dessin simple ; section 4.2) [S Ordre540][P].
- **Pose :** debout, carrée, tête haute, encolure à 50° environ, oreilles en avant et de biais, queue pendante. **Pas de boucle d'oreille** (presque toutes les bêtes photographiées en portent une jaune : on ne la dessine pas).

### 1.4 L'autre sexe : le bélier (pour mémoire)

| | Brebis (choisie) | Bélier |
|---|---|---|
| Garrot | 65 cm (moyenne de 756 brebis) [S RUDN2022] | **69,8 cm** (moyenne de 163 béliers ; 65,9 à 72,9 selon l'élevage ; certains 77–80) [S Furaeva2025] |
| Poids | 45–55 kg (standard ancien), 57 kg (moyenne 2023) | 65–75 kg, jusqu'à 100 [S WP-RU] ; 76 kg en moyenne en 2023, 83 kg dans les élevages de sélection [S Furaeva2024] |
| Crinière | aucune | **crinière de jarres noirs** sur l'encolure dès 9 mois, « nettement séparée du reste de la toison » [S Ordre540][S WP-RU] ; elle couvre la gorge, le dessous et les côtés de l'encolure et le poitrail jusqu'entre les antérieurs, parfois une tache noire derrière l'épaule [P : R5, R19] |
| Tête | sèche, chanfrein busqué | « plus large et plus courte », chanfrein **nettement** plus busqué [S ASSONAPA][S WP-RU] |
| Ossature | fine | « plus grossier, peau plus épaisse, crinière noire, ossature puissante » [S Furaeva2025] |
| Cornes | aucune | aucune dans le type voulu (section 6) |

### 1.5 L'agneau et l'antenais

- **À la naissance :** **noir** de la tête aux pieds, souvent une petite étoile ou une liste blanche, parfois du blanc aux pieds ou au bout de la queue [S USAMV][P : R8]. Laine bouclée serrée, d'un noir profond (`#0A0C0E` mesuré [P : R8]).
- **Taille :** 2,26 kg en moyenne à la naissance [S RUDN2022] ; garrot **32 cm**, longueur du tronc 22 cm, tour de poitrine 26 cm [S Kosilov2022] : un agneau **tout en pattes**, plus haut que long.
- **Couleur :** le pelage commence à pâlir **vers 2 à 4 semaines** ; à 3–5 mois, le duvet est entièrement dépigmenté [S WP-RU] (« de 1 à 3 mois » [S USAMV]). Un agneau de deux mois est donc gris foncé fumé, plus sombre que sa mère.
- **À 4 mois :** garrot 55–57 cm ; **à 12 mois** : 61–62 cm, tour de poitrine 83–85 cm [S Kosilov2022].

---

## 2. Tailles et proportions

### 2.1 Les chiffres

| Mesure | Brebis | Bélier | Sources |
|---|---|---|---|
| Hauteur au garrot | **65,2 cm** (756 brebis) ; « environ 60 » | **69,8 cm** (163) ; 65–70 | [S RUDN2022][S Furaeva2025][S Kyiv] |
| Hauteur au sacrum | 64,4 cm | 71,1 cm | idem |
| Longueur oblique du tronc (pointe de l'épaule → ischion) | **67,6 cm** | **74,7 cm** | idem |
| Tour de poitrine (derrière les épaules) | **92,9 cm** | **98,9 cm** | idem |
| Profondeur de poitrine | non publiée (≈ 30 cm [E]) | **32,6 cm** | [S Furaeva2025] |
| Largeur de poitrine | 27,4 cm (chiffre douteux, section 10.3) | 19,6 cm | idem |
| Largeur aux hanches (maklocks) | 19,9 cm | 18,2 cm | idem |
| Tour du canon | non publié (≈ 9 cm [E]) | 10,9 cm | [S Furaeva2025] |
| Poids adulte | 45–55 kg ; moyenne 2023 : 57 | 65–75 kg ; moyenne 2023 : 76 | [S WP-RU][S Furaeva2024] |
| Poids minimum du standard | élite 55, 1ʳᵉ classe 48, 2ᵉ classe 42 kg | élite 70, 1ʳᵉ classe 60 kg | [S Ordre540] |
| Queue | **8–10 cm** ; 9–10 cm ; 13 cm en moyenne ; « jusqu'à 13 cm » chez le bélier (1908) | | [S WP-RU][S Furaeva2025][S ASSONAPA] |
| Tête, oreille | **aucune mesure publiée** | | section 2.3 |

### 2.2 Les indices de forme (163 béliers) [S Furaeva2025]

Les zootechniciens russes résument la forme par des indices ; ils valent aussi pour dessiner :

| Indice | Valeur moyenne | Ce qu'il veut dire |
|---|---|---|
| Longueur des pattes (H − profondeur de poitrine) / H | **53,3 %** | le dessous de la poitrine est à **0,53 H** du sol : la race est **haute sur pattes** (« un peu haute sur pattes » [S WP-FR]) |
| Étirement (longueur oblique / H) | **107,0 %** | tronc à peine plus long que haut |
| Compacité (tour de poitrine / longueur oblique) | **132,4 %** | tronc rond, « en tonneau » |
| Poitrine (largeur / profondeur) | 60,1 % | poitrine profonde plus que large |
| Ossature (tour du canon / H) | 15,6 % | canons fins |

Chez les brebis mesurées : longueur oblique / H = 67,6 / 65,2 = **1,04** ; tour de poitrine / H = **1,42** [S RUDN2022].

### 2.3 Ratios de dessin (brebis, H = 65 cm)

| Mesure | En H | En cm | Étiquette |
|---|---|---|---|
| Dessus de la toison au garrot (6–7 cm de laine) | 1,09–1,11 | 71–72 | [E] d'après [S Ordre540] (longueurs de mèche) |
| Longueur du tronc (os), pointe de l'épaule → ischion | 1,04 | 68 | [S RUDN2022] |
| Silhouette en toison, poitrail → fesse | 1,22–1,28 | 79–83 | [E] (+ 6 cm de laine devant et derrière) ; le bélier R19 donne 1,36 fois la hauteur de sa toison (photo petite, presque de profil) [P] |
| Dessous de la poitrine (os) au-dessus du sol | 0,53 | 34–35 | [S Furaeva2025] (indice des béliers, appliqué à la brebis) [E] |
| Profondeur de poitrine (os) | 0,47 | 30,5 | [E] d'après 32,6 / 69,8 chez les béliers |
| Bas de la frange du ventre au-dessus du sol | 0,46–0,48 | 30–31 | [E] ; le ventre est « couvert de laine » [S Ordre540] |
| Patte visible sous la toison, rapportée à la hauteur de la toison | 0,42–0,46 | | [E] ; mesuré 0,46 sur la brebis de Kyiv (R18, toison longue, presque de face) ; 0,31–0,34 sur les photos prises d'en haut (R1, R17), qui raccourcissent les pattes [P] |
| Encolure (garrot → nuque, ligne du dessus) | 0,35–0,40 | 23–26 | [E] ; « longue, charnue » [S Furaeva2025], « de longueur moyenne pour la race » [S Ordre540] |
| Tête, LT | 0,35–0,37 | 23–24 | [E] ; « petite » [S ASSONAPA], « de taille moyenne » [S Ordre540] ; 0,41 fois la hauteur de la toison chez le bélier R19 [P] |
| Oreille | 0,15–0,17 (≈ 0,45 LT) | 10–11 | [E] ; ≈ 0,15 de la hauteur de toison sur R17, de trois quarts [P] |
| Queue | 0,14–0,20 | 9–13 | [S] (voir 2.1) |
| Largeur du canon antérieur, de profil | ≈ 0,05 | ≈ 3 | [E] d'après le tour du canon |

**La silhouette :** un tonneau gris presque carré (à peine plus long que haut), posé **haut** sur quatre pattes noires fines ; une encolure assez longue qui porte une petite tête noire allongée ; de grandes oreilles en V qui débordent de la tête. Le dos est **droit** [S Ordre540][S Furaeva2025], la croupe **tombe** un peu vers la queue (section 10.3). La toison courte suit le corps : le contour est net, finement grumeleux (des boucles d'un centimètre), **jamais effiloché** ni pendant.
- **Le corps sous la toison :** sur les bêtes tondues de juillet (R11–R14), le tronc paraît mince et la patte sous le ventre presque aussi longue que la profondeur du corps [P].
- **Les brebis pleines** (photos de février, R1, R2, R17) ont le ventre très bas et rond, les flancs gonflés : à éviter pour l'individu de référence.

### 2.4 L'encolure

- **Longueur :** environ 0,35–0,40 H, ronde, « musclée » [S Ordre540], sans fanon [P]. La laine grise la couvre jusqu'à la nuque ; le noir de la tête s'arrête net derrière les joues et sous la gorge [P : R1, R3, R18].
- **En alerte (la pose choisie) :** axe de l'encolure à **50–65° au-dessus de l'horizontale** ; la nuque dépasse le garrot d'environ 0,35 H ; le chanfrein descend de 25 à 40° sous l'horizontale ; sur R18 (brebis de Kyiv, tête haute), l'encolure est presque verticale, le museau en avant [P].
- **Au repos, debout :** 25–35° au-dessus de l'horizontale, la nuque à peine plus haute que le garrot [E].
- **En broutant :** l'encolure descend à **40–55° sous l'horizontale**, le chanfrein presque vertical, les antérieurs droits ; sur les photos de Ieïsk (R12, R13), le museau touche le sol à hauteur des sabots antérieurs, un peu devant eux [P].
- **En deux segments** (pour le moteur) : la base (épaule → milieu) bouge peu ; la partie haute (milieu → nuque) fait l'essentiel de l'angle [E].

### 2.5 Les articulations des pattes

| Articulation | Hauteur | Angle debout | Étiquette |
|---|---|---|---|
| Coude | ≈ 0,50 H, sous la frange du ventre | | [E] |
| Carpe (genou) | **0,21–0,25 H** | presque droit, 175–180° | [P] R18 (≈ 0,21) ; [E] |
| Boulet antérieur | ≈ 0,08 H | paturon à 55–60° sur l'horizontale | [E] |
| Grasset | ≈ 0,55 H, caché par la laine | | [E] |
| Jarret (pointe) | **0,30–0,34 H** | **145–155°** | [E], recoupé sur R11 et R19 [P, approximatif] |
| Boulet postérieur | ≈ 0,09 H | paturon à ≈ 55° | [E] |

- La laine descend jusqu'au **haut de l'avant-bras** (à peine sous le coude) et, derrière, jusqu'**au-dessus du jarret** (bas de la cuisse) ; plus bas, poil ras noir, et le genou comme le jarret se voient nettement [S Ordre540 : « pattes couvertes de poil de couverture »][P : R1, R2, R11].
- Pattes « droites, d'une longueur moyenne pour la race, **largement écartées** » [S Ordre540][S Furaeva2025][S ASSONAPA] : de profil, cela ne se voit pas ; de face ou de dos (R2), les quatre pattes sont bien séparées.
- Défaut cité par la FAO : pattes trop serrées, garrot saillant, dos ensellé [S FAO] : on dessine le type voulu, garrot large et dos droit.

### 2.6 La queue

- **Courte**, « fine » (*affusolata*), 8 à 13 cm [S WP-RU][S Furaeva2025][S ASSONAPA] : elle s'arrête **bien au-dessus du jarret**, vers mi-hauteur de la fesse [P : R2, R11].
- **Couverte de laine** courte, en général **plus sombre** que le corps (gris anthracite à noir) ; le bout est **parfois blanc**, ce que le standard admet [S Ordre540][P : R2, R11, R13].
- **Jamais coupée** sur les photos ; elle pend droit contre la fesse, se soulève un peu quand l'animal marche vite [P : R11, R12].

---

## 3. La toison

### 3.1 Ce qu'elle est

- **Deux sortes de fibres seulement**, sans fibres intermédiaires : des **jarres noirs** (ость), courts et grossiers, 60–90 µm, et un **duvet blanc** (пух), fin, 20–27 µm [S WP-RU][S FAO]. La FAO le dit ainsi : « le poil est court et noir, la laine est longue et blanche » ; les follicules se forment en deux temps seulement, d'abord ceux des jarres, puis ceux du duvet [S FAO]. Wikipédia EN donne 20,9 µm et 71,9 µm [S WP-EN].
- **Le duvet dépasse les jarres** : c'est la règle de la race. « Les fibres de duvet doivent dépasser les jarres et former une **boucle bien marquée de taille moyenne (6 à 12 mm)** sur la plus grande partie de la toison » [S Ordre540].
- **Proportion :** 1 jarre pour 4 à 10 brins de duvet (1:4 à 1:7 pour l'élite) [S Ordre540][S WP-RU] ; 2 600 à 2 800 fibres par cm² [S WP-RU].
- **Couleur :** « jarres **noirs**, duvet **gris clair** ; jarres blancs, duvet noir et fibres intermédiaires ne sont pas admis » [S Ordre540]. De l'extérieur, la toison est **grise** ; ouverte, **gris foncé et gris à reflet bleu** [S Ordre540] ; « un gris acier à reflet bleu » [S WP-RU] ; « gris avec une nuance de bleu » [S FAO].

### 3.2 À quoi elle ressemble

- **Courte, dense, mate**, en **petites boucles serrées** d'environ un centimètre, sans mèches pendantes : une moquette bouclée qui suit les formes [P : R1, R2, R3, R9]. Rien de la mèche longue et raide de la Heidschnucke, rien de l'anglaise brillante du Gotland (section 9).
- **Grain :** chaque boucle est blanchâtre en surface (le duvet) avec de l'ombre noire entre les boucles (les jarres et la peau). De loin, cela donne un **gris moyen uni** ; de près, un pointillé clair sur fond sombre [P : R3].
- **Taches plus sombres :** la toison n'est pas uniforme ; sur les photos de février, des **plages gris foncé** de la taille d'une main apparaissent sur le flanc et derrière l'épaule, là où les jarres sont plus nombreux [P : R1 (`#5E5E5A` contre `#7E7E74` autour), R2]. Le standard tolère « une bande plus sombre de jarres noirs sur l'encolure, le garrot et le dos » ; une forte coloration y fait déclasser la peau [S Ordre540].
- **Bord :** net ; sur le dos, une ligne légèrement grumeleuse ; sous le ventre, une courte frange de 1 à 2 cm de plus [P][E].

### 3.3 Longueur, tontes et mue

- **Tontes :** **trois par an**, vers mars, juin et octobre, « à cause de la mue saisonnière » [S WP-RU] ; 3 ou 4 tontes par an [S FAO]. En France et en Italie, les troupeaux sont souvent tondus moins souvent, d'où des toisons plus longues sur les photos (R17, R18) [P].
- **Longueurs de référence :** trois mois après la tonte, **jarres 2,5–3,5 cm, duvet 4–7 cm** ; « le jarre est plus court que le duvet de 1,5 à 3,5 cm » [S Ordre540]. Chez l'agneau de 8–9 mois, après la tonte de la laine d'agneau : jarres 2,4–3,0 cm, duvet 4,6 cm [S FAO].
- **Pour notre brebis de février** (3,5 mois) : duvet 6–7 cm, jarres 3–4 cm [E]. La toison ajoute donc environ 6 cm sur le dos, 4–5 cm sur le flanc, 1–2 cm de frange sous le ventre [E].
- **Toison longue (variante) :** au-delà de six mois, les pointes du duvet blanchissent et jaunissent au soleil ; la toison devient **crème à beige** en surface, ouverte en mèches grossières ; seul l'intérieur reste gris [P : R17 (`#D1B99C` au soleil), R18 (`#DCD5D4` sur le dos, `#9C8A78` sur le flanc)]. Ce n'est pas l'aspect que veut le standard : on ne la dessine pas.
- **Après la tonte :** le corps paraît gris souris, ras, mince, haut sur pattes (R11–R14, juillet) [P].

### 3.4 Ce qui reste ras

- « **Les pattes et la tête doivent être couvertes de poil de couverture** » (poil ras) ; « le ventre doit être couvert de laine » [S Ordre540].
- **Tête :** entièrement en poil ras noir, du bout du museau à la nuque, oreilles comprises ; **pas de toupet** de laine : la laine grise commence derrière la nuque et les joues [P : R3, R5, R18].
- **Pattes :** poil ras noir sous le coude et sous le jarret (voir 2.5).

---

## 4. Les couleurs

### 4.1 Le motif

**Le standard (arrêté n° 540, § 3.2–3.3) :** tête « **noire**, chez la plupart des animaux avec une **marque blanche en liste ou en étoile** » ; toison grise ; « les marques blanches sont admises sur les antérieurs **sous le carpe**, sur les postérieurs **sous le jarret**, et **au bout de la queue**. Les marques blanches **ailleurs ne sont pas admises** » [S Ordre540].

**En profil, tourné vers la droite :**
- **Tête :** noire, poil ras, du museau à la nuque, oreilles comprises ; la liste blanche se voit sur le haut du front, au-dessus de l'œil [S Ordre540][P].
- **Limite noir/gris :** **nette**, en arrière des joues et sous la gorge : la tête noire se « pose » sur une encolure grise. Le passage se fait sur 2 à 4 cm, par du gris foncé sur le haut de la nuque [P : R1, R3, R17, R18]. C'est le contraire de la Heidschnucke, dont tout le cou est noir.
- **Encolure, corps, ventre, croupe :** gris ; plus clair sur le dos (la lumière), plus sombre vers le bas et derrière l'épaule [P].
- **Queue :** gris anthracite à noire, laineuse [P : R2].
- **Pattes :** noires depuis la fin de la laine jusqu'aux onglons [S][P].

### 4.2 Les marques blanches, en détail

| Marque | Fréquence | Forme | Étiquette |
|---|---|---|---|
| Étoile ou liste sur le front | « la plupart des animaux » | de la petite étoile sur la nuque (R6, R8) à la liste étroite jusqu'au milieu du front (R5, R14), jusqu'à la large liste qui descend au museau (R4) ; **bords nets** | [S Ordre540][P] |
| Museau ou lèvre blancs | fréquent sur les photos françaises et italiennes | un « bouchon » blanc autour du nez et de la lèvre (R3, R17), parfois avec peau rose (R17) | [P] ; le standard russe ne le mentionne pas (section 10.3) |
| Pieds blancs | admis | une couronne ou une chaussette au-dessus de l'onglon, **jamais au-dessus du carpe ou du jarret** | [S Ordre540][P : R11, R14, R17] |
| Bout de la queue blanc | admis | les derniers centimètres | [S Ordre540][P : R11] |
| Taches blanches sur le corps | **refusées** | | [S Ordre540] |

**Pour notre brebis :** une liste étroite (1,5–2 cm de large, 0,1 LT) du toupet jusqu'à 2 cm au-dessus de la ligne des yeux, bords nets, un peu effilée vers le bas. Rien d'autre.

### 4.3 Comment les valeurs ont été mesurées

| Photo | Zone | Moyenne | Ombre (p10) | Lumière (p90) |
|---|---|---|---|---|
| R1 (Salon de l'agriculture, février, intérieur) | dos | `#ADA9A1` | `#858178` | `#CAC6BE` |
| R1 | milieu du flanc (zone claire) | `#7E7E74` | `#58584F` | `#A9AA9F` |
| R1 | tache sombre du flanc | `#5E5E5A` | `#3F3F39` | `#8A8A88` |
| R1 | bas du flanc | `#88887D` | `#646359` | `#AEAEA4` |
| R1 | haut de l'encolure (près de la tête) | `#57544B` | `#2B2A26` | `#878374` |
| R1 | patte antérieure | `#16140D` | `#0B0A06` | `#3D3622` |
| R1 | liste (brebis du fond, à l'ombre) | `#BBBDC4` | `#808088` | `#DCDEE4` |
| R2 (même salon) | dos | `#A8A495` | `#54534F` | `#C9C4B6` |
| R2 | croupe | `#A5A18E` | `#777366` | `#C4C0AD` |
| R2 | queue | `#42403C` | `#312F2B` | `#64625D` |
| R2 | face | `#201F1B` | `#11110D` | `#51504C` |
| R3 (tête, intérieur) | flanc | `#88877D` | `#5F5D53` | `#B5B5AB` |
| R3 | encolure | `#8D8A7E` | `#5F5C52` | `#BDBBAE` |
| R3 | face | `#2E2F33` | `#202123` | `#3D3E43` |
| R3 | liste | `#F0F1F1` | `#AEB2B2` | `#FEFFFF` |
| R3 | oreille | `#212220` | `#101210` | `#3B3C3A` |
| R3 | cercle pâle autour de l'œil | `#646262` | `#252522` | `#89888A` |
| R3 | iris (à l'ombre de la paupière) | `#21242A` | `#13151B` | `#373A40` |
| R3 | nez (peau) | `#252422` | `#141210` | `#3F3C3A` |
| R5 (bélier, couvert, automne ; dominante bleue) | flanc | `#B4BDCC` | `#727A8C` | `#E6ECF5` |
| R5 | crinière | `#23262B` | `#131518` | `#43454A` |
| R5 | noir de l'épaule | `#444D62` | `#303745` | `#576279` |
| R5 | face (reflet du ciel) | `#4A5263` | `#2D2E33` | `#647188` |
| R5 | liste | `#F0F2F6` | `#8891AB` | `#FFFFFF` |
| R5 | patte postérieure | `#2F3134` | `#1A1C1D` | `#5F605F` |
| R8 (VDNKh, Moscou) | agneau noir | `#0A0C0E` | `#050608` | `#0F1214` |
| R8 | toison de la mère (longue, à l'ombre) | `#59544D` | `#2A241E` | `#8C8780` |
| R8 | face de la mère | `#191F29` | `#0B0C0E` | `#34435B` |
| R11 (Ieïsk, juillet, tondue) | flanc | `#A99D92` | `#6B5B4C` | `#D8CFC7` |
| R11 | face | `#0E0B0C` | `#080505` | `#1D1C1E` |
| R10 (couchée au soleil) | flanc | `#5E4A44` | `#3F2B24` | `#7B6661` |
| R17 (ASSONAPA, soleil) | dos | `#E2D4C6` | `#ADA49B` | `#F3E9DE` |
| R17 | flanc (toison longue) | `#D1B99C` | `#AF9579` | `#E9D4B9` |
| R17 | face | `#232221` | `#181717` | `#473E3A` |
| R17 | museau blanc | `#DBC9C0` | `#85746D` | `#F5F0EA` |
| R17 | patte | `#2C2620` | `#1A150F` | `#534538` |
| R18 (Kyiv, couvert) | dos | `#DCD5D4` | `#C7BBB6` | `#E9E3E4` |
| R18 | flanc (toison longue) | `#9C8A78` | `#483824` | `#DDCFC7` |
| R18 | face | `#222633` | `#14161C` | `#353B4F` |
| R18 | patte | `#161918` | `#090D0E` | `#4C4E30` |

**Lecture :** sous lumière neutre (R1, R2, R3), la toison courte mesure entre `#7E7E74` et `#ADA9A1`, un gris **à peine chaud** ; le « bleu » du standard vient surtout du fond de la toison (jarres noirs sous duvet blanc) et apparaît sous ciel couvert (R5) ou quand la toison s'ouvre. Les toisons longues (R17, R18) virent au crème. Le noir de la tête est **froid** (`#2E2F33`, `#222633`), celui des pattes un peu plus chaud.

### 4.4 Palette recommandée (brebis, lumière neutre d'hiver)

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Toison, dos et haut du flanc | `#A9A79F` | `#CBC9C2` (dessus des boucles) | `#85827A` |
| Toison, milieu du flanc | `#8A8982` | `#ADADA6` | `#5F5E58` ; creux entre boucles `#45443F` |
| Toison, bas du flanc, ventre, cuisse | `#7A7972` | `#9E9D96` | `#53524D` |
| Plages sombres (derrière l'épaule, haut du dos) | `#5F5F5B` | `#8A8A88` | `#3F3F3A` ; bords fondus sur 5–10 cm |
| Reflet bleuté (là où la toison s'ouvre, plis de l'encolure) | `#7D8590` | `#A3ABB6` | `#545A63` [E, d'après R5 et le standard] |
| Haut de l'encolure, juste derrière la tête | `#5A5850` | `#878374` | `#2B2A26` |
| Tête, oreilles (poil ras) | `#212226` | `#3D3E43` ; reflet froid sur chanfrein et front `#4A5263` | `#121315` |
| Intérieur de l'oreille | `#2C2A2A` | | peau pigmentée, jamais rose [P : R3] |
| Cercle autour de l'œil | `#5E5D5C` | `#89888A` | anneau de poil plus clair, fin [P : R3] |
| Nez (peau nue), lèvres | `#252422` | `#45433F` (humide) | naseaux `#0B0B0C` |
| Liste blanche | `#EDEEEC` | `#FAFBFB` | `#AEB2B2` |
| Pattes | `#1D1C1A` | `#3D3A35` | `#0E0D0B` |
| Onglons | `#262422` | bord usé `#5A554E` | `#121110` [E] (aucun onglon net sur les photos) |
| Queue | `#45433F` | `#64625D` | `#2F2D2A` |
| Iris | `#5A3E26` | `#9A7448` | cercle `#2E2218` ; pupille `#0E0E10` ; reflet `#F2EEE6` ([E] : mesuré presque noir à l'ombre, R3 ; brun ambré au soleil, R18) |

### 4.5 Bélier, agneau, variantes

- **Bélier :** même toison ; **crinière** `#23262B`, lumière `#43454A`, ombre `#131518` ; elle couvre la gorge, le dessous et le côté de l'encolure jusqu'au poitrail, avec un bord **net** sur l'épaule ; parfois une large tache noire derrière l'épaule (R5, R19) [P].
- **Agneau nouveau-né :** `#0A0C0E`, reflets `#1F242A` [P : R8] ; étoile `#E8E9E8`. **Agneau de 2 mois :** gris fumé `#4A4844` [E].
- **Variante « toison longue » (non retenue) :** pointes `#D8C8B4`, intérieur `#8E8478` [P : R17, R18].

---

## 5. La tête

- **Forme :** « petite, sèche, allongée, avec une **bosse marquée sur le nez** » [S Furaeva2025] ; « de taille moyenne, sèche, allongée, noire… **profil busqué** » [S Ordre540] ; « petite… au nez aquilin ; crâne arrondi » [S ASSONAPA] ; « petite tête anguleuse » [S WP-FR].
- **Profil :** front un peu bombé, puis un chanfrein **convexe** sur toute sa longueur, la bosse culminant à mi-chemin entre l'œil et le nez ; museau étroit qui s'arrondit au bout [P : R3, R5, R19]. La brebis est busquée **modérément** ; le bélier, « nettement » [S ASSONAPA][S WP-RU].
- **Museau, nez :** nez noir à peau nue, naseaux en virgule obliques, lèvres fines ; lèvre supérieure fendue [P : R3][E].
- **Yeux :** « **grands, saillants** » (глаза выпуклые, большие) [S Ordre540][S Furaeva2025] ; « volumineux » [S WP-FR]. Haut sur la tête, à environ 40 % de LT en avant de la nuque [E]. Un fin **anneau de poils gris** entoure l'œil [P : R3]. Iris **brun foncé à brun ambré** [P : R3, R18] ; pupille horizontale, rectangle aux coins arrondis (comme tous les moutons). Paupière supérieure épaisse.
- **Oreilles :** « **dressées, mobiles** » (уши прямостоячие, подвижные) [S Ordre540] ; « dressées, énergiques » [S Furaeva2025] ; « droites et mobiles » [S ASSONAPA]. Sur les photos, elles sont **grandes** pour la tête (≈ 0,45 LT), en **lancette pointue**, portées **à l'horizontale ou un peu relevées** (0 à 30° au-dessus), souvent un peu en avant quand l'animal regarde [P : R3, R6, R17, R18, R20]. En profil, l'oreille gauche sort derrière l'œil, à la nuque, et pointe vers l'arrière-haut ou vers l'avant selon l'attention ; l'oreille droite dépasse au-dessus du front.
- **Pas de toupet**, pas de laine sur les joues [P].

---

## 6. Les cornes

### 6.1 Ce que disent les sources

- **Le type voulu est sans cornes, chez les deux sexes.** « Béliers et brebis doivent être **sans cornes** » [S Ordre540] ; « Béliers et brebis sont sans cornes » [S Furaeva2025] ; « mâles privés de cornes » [S ASSONAPA] ; « mâles et femelles ne portent pas de cornes » [S WP-FR] ; « la présence de cornes chez les deux sexes n'est pas admise : c'est le signe que l'animal n'est pas pur » [S USAMV].
- **Mais il en existe :** le premier standard (Koulechov, 1908) dit les animaux « sans cornes **et** cornus ; les sans-cornes sont préférables » [S WP-RU]. La FAO distingue trois types dans la race : le type **grossier** a « des cornes bien développées chez les béliers et de petites cornes chez les brebis » ; le type standard est « cornu ou sans cornes » ; le type fin est sans cornes [S FAO]. Wikipédia DE : « le plus souvent sans cornes » [S WP-DE].
- **Photos :** aucune des bêtes regardées ne porte de cornes ni de moignons visibles [P : R1–R20].

### 6.2 Ce qu'on dessine

- **Aucune corne, ni chez la brebis, ni chez le bélier.** Le crâne est arrondi entre les oreilles [S ASSONAPA], couvert de poil ras noir ; la liste blanche part de ce sommet.
- Si l'on voulait un jour un « Romanov ancien » de type grossier, le bélier porterait des cornes : aucune source ne les décrit (forme, tours), on ne les inventera pas.

---

## 7. Les pattes et les onglons

- « Fortes, **droites**, d'une longueur moyenne pour la race » [S Ordre540] ; « fines mais solides, droites, de longueur moyenne, **largement écartées** » [S Furaeva2025] ; « droites, largement écartées, à **poil noir lisse** » [S ASSONAPA].
- **En dessin :** pattes **noires**, poil ras, minces (canon d'environ 0,05 H) et **longues** sous la toison (indice 53 %) ; aplombs droits : antérieur vertical sous l'épaule, canon postérieur presque vertical sous la fesse [S][P : R11, R18].
- **Genou et jarret** bien visibles sous le poil ras [P]. Paturons assez courts, souples.
- **Onglons :** noirs à ardoise, petits, les deux doigts serrés ; ergots sombres derrière le paturon [E] ; une **couronne blanche** au-dessus d'un onglon postérieur est fréquente mais on ne la dessine pas sur notre brebis (4.2).

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Plus craintive de l'homme que les races de plaine :** une étude de l'INRA (Le Neindre et al. 1993) compare des agnelles Mérinos d'Arles, Romanov et croisées dans cinq tests. Les Romanov **évitent davantage l'homme**, mettent plus de temps à l'approcher, **regardent plus souvent l'observateur**, mangent moins dans un lieu nouveau, défèquent et urinent plus (signe de peur), et **font plus souvent demi-tour** quand on les mène au parc d'essai. Leur **activité de déplacement ne diffère pas** de celle des Mérinos : elles ne courent pas plus, elles s'inquiètent plus. Les croisées ressemblent au père Romanov : l'écart est génétique [S LeNeindre1993].
- **Voix :** les Romanov poussent **moins de bêlements aigus** et **plus de bêlements graves** que les Mérinos ; leurs bêlements graves augmentent quand elles voient des congénères. Elles flairent moins le sol et les murs [S LeNeindre1993].
- **Bonne mère :** chez les primipares, les troubles du léchage ou de l'acceptation de l'agneau sont plus rares (10 %) que chez la Préalpes du Sud (35 %) ou l'Île-de-France (50 %) ; le comportement agressif envers l'agneau aussi [S AFS1988]. « Instinct maternel très marqué, qui permet d'allaiter et de nourrir plusieurs agneaux à la fois ; agneaux très vigoureux à la naissance » [S ASSONAPA].
- **Portées :** 2–3 agneaux en moyenne, jusqu'à 7 à 9 [S FAO][S USAMV] ; répartition en Russie : 6–8 % de simples, 38–40 % de doubles, 44–46 % de triples, 8–10 % de quatre à huit [S WP-RU].
- **Sans saison :** « la sélection pour la fécondité a supprimé le caractère saisonnier des chaleurs » ; gestation un peu plus courte (140–150 jours) [S FAO] ; agnelages toute l'année. Agnelles très précoces [S AFS1988].
- **Rustique au froid :** elle supporte le froid et les écarts de température ; l'**humidité** et l'ammoniac des bergeries lui sont néfastes [S FAO] ; elle supporte mal les fortes chaleurs d'été (fertilité, ardeur des béliers) [S AFS1988].
- **Mue :** la toison mue au fil des saisons, d'où les trois tontes [S WP-RU].

### 8.2 Ce qui vaut pour les moutons en général (voir `docs/animation-animale.md`)

- Grégarité, rythme de la journée (9 à 10 h de broutage, rumination surtout couchée), oreilles en arrière quand il a peur, sauts d'agneaux : rien de propre à la race n'est décrit.
- Aucun cri propre au Romanov n'est décrit au-delà de l'étude de l'INRA (8.1).
- Béliers : aucun combat propre à la race n'est décrit ; sans cornes, ils se cognent front contre front comme les autres races sans cornes.

### 8.3 Ce que cela donne dans Mouflon (propositions)

- **Vigilante :** elle **regarde** souvent (alerte contagieuse) et **évite le pointeur** de plus loin que le Mannequin ; quand on la mène, elle fait volte-face ; elle ne vient pas renifler le pointeur.
- **Pas plus coureuse que les autres :** même vitesse et même activité que le Mannequin ; c'est la distance de fuite et le regard qui changent.
- **Voix grave :** plutôt des bêlements graves, bouche fermée, surtout quand elle voit les autres ; peu de bêlements aigus.
- **Mère :** si le moteur a des agneaux, une brebis Romanov en a **deux à quatre**, noirs, qui la suivent de près.
- **Oreilles :** très mobiles (le standard le dit) : elles pivotent souvent, en avant quand elle guette.

---

## 9. Les dix détails, et les erreurs à éviter

### 9.1 Les dix détails qui font un Romanov

1. **Une toison gris bleuté, courte, dense et mate**, en petites boucles serrées d'un centimètre, qui suit le corps ; ni mèches pendantes, ni anglaises brillantes [S Ordre540][P].
2. **Le gris s'arrête net à la tête :** la tête noire se pose sur une encolure grise ; l'encolure n'est **pas** noire chez la brebis [S Ordre540][P].
3. **Une liste ou une étoile blanche** sur le front, chez la plupart des animaux, à bords nets [S Ordre540][P].
4. **Une tête sèche, allongée, au chanfrein busqué**, sans laine ni toupet [S Ordre540][S Furaeva2025].
5. **De grandes oreilles noires dressées, de biais, très mobiles** [S Ordre540][P].
6. **De gros yeux saillants**, cerclés d'un fin anneau gris [S Ordre540][P].
7. **Aucune corne**, chez les deux sexes [S Ordre540].
8. **Des pattes noires, fines, assez hautes** (poitrine à 0,53 H du sol), écartées, avec genou et jarret visibles ; blanc permis seulement aux pieds [S Furaeva2025][S Ordre540].
9. **Une queue courte et laineuse** (moins de 13 cm), plus sombre que le corps, parfois à bout blanc [S][P].
10. **Le troupeau :** des brebis grises suivies de **trois ou quatre agneaux noirs** ; chez le bélier, une **crinière noire** sur l'encolure et le poitrail [S FAO][S Ordre540][P].

### 9.2 Ce qui en ferait une autre race

| Erreur | Ce qu'on obtiendrait | Pourquoi |
|---|---|---|
| Cornes en faucille chez la brebis, en escargot chez le bélier ; encolure entièrement noire jusqu'au garrot ; longues mèches raides pendantes ; petites oreilles | une **Heidschnucke** grise | elle a des cornes chez les deux sexes, une cape noire jusqu'au garrot, une toison de jarres de 38–39 µm qui pend ; tête noire sans liste (`docs/races/heidschnucke.md`) ; le Romanov est sans cornes, gris jusqu'aux oreilles, en boucles courtes |
| Toison **longue**, **brillante**, en **anglaises régulières** qui pendent, tête noire fine, oreilles moyennes, ossature très fine | un **Gotland** | « fleece fine, long, lustrous and dense… a clearly defined even curl », 29–34 µm, « fine-boned » [S WP-Got] ; le Gotland peut aussi avoir du blanc sur le dessus de la tête ou autour du nez [S WP-Got], la liste ne suffit donc pas : c'est la **toison courte et mate** et le **chanfrein busqué** qui font le Romanov |
| Toison **blanche**, tête blanche | un **Finnois** ou une Romane | le Finnois, l'autre race prolifique nordique, est surtout blanc [E] ; la Romane (Romanov × Berrichon du Cher) a hérité de la toison blanche du Berrichon [E] |
| Tête **blanche** avec des taches noires autour des yeux, ou une large bande blanche sur toute la face | un **croisé** (R15) | le standard veut une tête noire, la marque blanche limitée à une liste ou une étoile [S Ordre540] |
| Toison **noire** chez un adulte, ou brun chocolat | un **agneau** Romanov (ou un Ouessant) | le noir est l'état de l'agneau ; le brun, celui d'une toison longue brûlée [S WP-RU][P] |
| Corps à **taches blanches** | un croisé ou un Jacob | taches blanches « non admises » [S Ordre540] |
| Laine sur la face ou les pattes, toupet | une race de plaine laineuse | « tête et pattes couvertes de poil de couverture » [S Ordre540] |
| Pattes courtes noyées dans la toison | un mouton de foire | indice de longueur des pattes 53 % [S Furaeva2025] |
| Longue queue laineuse jusqu'au jarret | n'importe quelle race à queue longue | queue de 8–13 cm [S] |
| Cape et bavette noires sur une **brebis** | un **bélier** Romanov ou une Heidschnucke | seule la crinière du bélier est noire [S Ordre540] |
| Toison crème à mèches ouvertes | un Romanov mal tondu, pas le type voulu | « au regard, la toison doit être grise » [S Ordre540][P : R17, R18] |
| Corps noir, **liste blanche jusqu'au bout du nez**, balzanes blanches, queue longue | un **Zwartbles** | le Zwartbles est noir de partout sauf la liste, quatre balzanes et le bout d'une queue longue (`docs/races/zwartbles.md`) ; le Romanov adulte est gris, à queue courte |

**Les voisines** : le **Gotland** (sa fille en partie, gris, sans cornes, en anglaises brillantes), la **Heidschnucke** (grise, cornue, à cape noire), la **Wrzosówka** polonaise (petit mouton de lande à toison mixte grise, agneaux noirs qui grisonnent, comme le Romanov) [S heidschnucke.md], et le **Finnois** (prolifique, blanc).

---

## 10. Sources, photos, contradictions

### 10.1 Sources écrites (ouvertes)

**Standard russe et études de Iaroslavl**
- **[S Ordre540]** Arrêté du ministère de l'Agriculture de la Fédération de Russie n° 540 du 2 décembre 2016, *Порядок и условия проведения бонитировки племенных овец романовской породы* (le type voulu : tête, oreilles, yeux, encolure, pattes, toison, marques blanches, crinière, poids minimums, groupes de peaux) : https://pravo.ppt.ru/prikaz/minselkhoz/n-540-131489 (aussi https://rulaws.ru/acts/Prikaz-Minselhoza-Rossii-ot-02.12.2016-N-540/)
- **[S Furaeva2025]** N. S. Furaeva, V. I. Khrustaleva, S. I. Sokolova, E. A. Zvereva, « Сравнительная характеристика экстерьера и индексов телосложения баранов романовской породы », *Овцы, козы, шерстяное дело* 2025 n° 1, p. 40–43, DOI 10.26897/2074-0840-2025-1-40-43 (163 béliers de 5 élevages, mensurations, indices ; description du type : tête, oreilles, yeux, queue 9–10 cm, béliers et brebis sans cornes) : https://wool.timacad.ru/jour/article/download/431/413
- **[S Furaeva2024]** N. S. Furaeva et al., « Племенные ресурсы романовского овцеводства », *Овцы, козы, шерстяное дело* 2024 n° 4, p. 15–19 (effectifs 2024, poids moyens 2023) : https://yaragrovuz.ru/sveden/files/eiu/PLEMENNYE_RESURSY_ROMANOVSKOGO_OVCEVODSTVA.pdf
- **[S RUDN2022]** M. V. Abramova et al., « Оценка селекционных признаков овец романовской породы в зависимости от полиморфизма гена гормона роста », *Вестник РУДН, Агрономия и животноводство* 17(4), 2022, p. 514–526 (mensurations de 756 brebis de Iaroslavl, poids à la naissance) : https://agrojournal.rudn.ru/agronomy/article/download/19833/16333
- **[S Kosilov2022]** V. I. Kosilov et al., « Особенности линейного роста молодняка овец романовской породы в зависимости от полового диморфизма », *Овцы, козы, шерстяное дело* 2022 n° 1, p. 13–15 (mensurations des agneaux de 0 à 12 mois) : https://wool.timacad.ru/jour/article/download/162/143
- Consultés, sans chiffre retenu : Korenev et al. 2023, *Племенная работа в романовском овцеводстве* (https://wool.timacad.ru/jour/article/download/176/232) ; Furaeva et al. 2025, structure généalogique (https://yaragrovuz.ru/sveden/files/aie/GENEALOGICHESKAYA_STRUKTURA_OVEC_ROMANOVSKOI_PORODY_I_PERSPEKTIVY_EE_RAZVITIYA.pdf) ; Kostylev et al., croisements de lignées (https://yaragrovuz.ru/images/Vestnik_APK/14_5/25-29.pdf) ; Valetko et Tchepouchtanova 2024, hybrides Romanov × argali (https://min.urgau.ru/images/2024/3-2024/18-3-2024.pdf).

**Monographies, instituts, associations**
- **[S FAO]** FAO, *Sheep breeds of the former USSR* (chapitre « Coarsewooled breeds : Romanov », p. 230–232 : trois types, cornes, toison, défauts, prolificité) : https://www.fao.org/docrep/pdf/009/ah759e/ah759e20.pdf
- **[S ASSONAPA]** Associazione Nazionale della Pastorizia, *Razza ovina Romanov* (morphologie, queue de 13 cm, toison, crinière, garrot max 70 cm, poids) : https://www.assonapa.it/public/schede-ovini/conservazione-e-salvaguardia/estere/ROMANOV.pdf
- **[S USAMV]** V. D. Bulmaga et al., « The use of the Romanov breed in different crossbreeding programs », *Scientific Papers, Series D, Animal Science* LXV(1), 2022 (standard résumé, cornes non admises, agneaux) : https://www.animalsciencejournal.usamv.ro/pdf/2022/issue_1/Art43.pdf
- **[S Kyiv]** Zoo de Kyiv, *Romanov sheep* (garrot 65–70 cm béliers, ≈ 60 cm brebis ; poids) : https://zoo.kyiv.ua/?p=11030

**INRA et France**
- **[S AFS1988]** G. Ricordeau, L. Tchamitchian, J. P. Poivey, « Performance, development and use of Romanov in France », *Journal of Agricultural Science in Finland* 60 (1988), 558–565 (importation de 1963, effectifs 1987, comportement maternel, chaleur, parasitisme) : https://journal.fi/afs/article/download/72313/34109/96396
- **[S LeNeindre1993]** P. Le Neindre, P. Poindron, G. Trillat, P. Orgeur, « Influence of breed on reactivity of sheep to humans », *Genetics Selection Evolution* 25 (1993), 447–458 : https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-25-5-447.pdf
- **[S Ricordeau1976]** G. Ricordeau et al., « Amélioration de la productivité des brebis Berrichonnes du Cher par croisement. I », *Annales de Génétique et de Sélection animale* 8 (1976), 367 (brebis Romanov de 2 ans : 45 kg après la mise bas) : https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-8-3-367.pdf
- **[S ProdAnim]** G. Ricordeau et al., « La race ovine INRA 401 : un exemple de souche synthétique », *INRA Productions animales* (Romanov « très prolifique, maternelle… toison colorée ») : https://productions-animales.org/article/download/4300/12825/31089
- **[S Patre]** *Réussir Pâtre*, « Le plein d'agneaux avec la race Romane », 2 février 2015 : https://www.reussir.fr/patre/le-plein-dagneaux-avec-la-race-romane
- **[S Flamant]** J. C. Flamant et al., « Évaluation des aptitudes laitières des brebis de race pure ou croisées Romanov », *Annales de Génétique et de Sélection animale* 11 (1979), 223 (consulté, rien de visuel) : https://gsejournal.biomedcentral.com/counter/pdf/10.1186/1297-9686-11-3-223.pdf

**Encyclopédies, autres études**
- **[S WP-RU]** https://ru.wikipedia.org/wiki/Романовская_порода_овец (type, standard de Koulechov 1908, toison, fibres, dépigmentation de l'agneau, trois tontes, poids ; d'après Arsenʹev 1985 et le *Справочник* d'Aripov 1990)
- **[S WP-FR]** https://fr.wikipedia.org/wiki/Romanov_(race_ovine) · **[S WP-EN]** https://en.wikipedia.org/wiki/Romanov_sheep · **[S WP-DE]** https://de.wikipedia.org/wiki/Romanow-Schaf
- **[S WP-Got]** https://en.wikipedia.org/wiki/Gotland_sheep (et https://sv.wikipedia.org/wiki/Gotlandsfår)
- **[S WP-NEST]** https://en.wikipedia.org/wiki/Northern_European_short-tailed_sheep
- **[S Sen2021]** Şen et Uğurlu 2021, agneaux Romanov en Turquie (garrot 44,1 cm à 90 jours), notice : https://acikerisim.omu.edu.tr/entities/publication/29971448-70e7-45fb-9993-52b629904d78/full
- **[S heidschnucke.md]** la fiche voisine du dépôt, `docs/races/heidschnucke.md` (Heidschnucke, Wrzosówka).

**Cherchées sans succès :** la fiche race de France Génétique Élevage (https://www.france-genetique-elevage.org/Romanov.html) a répondu 503 puis par un certificat refusé ; le site de GEODE (la coopérative qui gérait la race en France) ne répond pas ; la fiche de l'Oklahoma State University répond 403 ; l'article de la Grande Encyclopédie russe (bigenc.ru) demande une connexion ; l'étude croate de Kožarić et al. 2009 (mensurations de brebis adultes) n'a pas pu être ouverte ; aucune fiche de la LfL de Bavière pour le Romanov. **Aucune mesure publiée de la tête, des oreilles, du carpe ou du jarret** : tout ce qui les concerne est [P] ou [E].

### 10.2 Photos regardées

Toutes téléchargées hors du dépôt (`scratchpad/photos/romanov/`), regardées, et pour la plupart mesurées.

| N° | Page | Ce qu'elle montre |
|---|---|---|
| R1 | https://commons.wikimedia.org/wiki/File:Moutons_romanov.jpg | trois brebis au Salon de l'agriculture (Paris, février) ; celle de devant, **presque de profil vers la droite**, broute : toison courte gris bleuté en petites boucles, **plages plus sombres** sur le flanc, tête noire qui se pose sur l'encolure grise, pattes noires ; liste blanche sur la brebis du fond ; mesurée |
| R2 | https://commons.wikimedia.org/wiki/File:Moutons_romanov_SIA.jpg | même salon, brebis vues de dos : **queues courtes sombres laineuses**, pattes noires écartées, croupe grise ; mesurée |
| R3 | https://commons.wikimedia.org/wiki/File:Mouton_romanov_t%C3%AAte.jpg | brebis de profil vers la droite, tête de près : **chanfrein**, liste blanche sur le haut du front, lèvre blanche, œil saillant à anneau gris, oreilles dressées, limite nette noir/gris ; mesurée |
| R4 | https://commons.wikimedia.org/wiki/File:Mouton_Romanov.jpg | quatre jeunes couchés dans la paille : de la petite étoile à la **large liste qui descend au museau**, oreilles en lancette tenues à l'horizontale |
| R5 | https://commons.wikimedia.org/wiki/File:Beranlinierohan.jpg | **bélier** de trois quarts (photo de Wikipédia EN, RU, DE), automne : **crinière noire** sur gorge, encolure et poitrail, tache noire derrière l'épaule, liste blanche, corps gris, pattes noires ; mesuré |
| R6 | https://commons.wikimedia.org/wiki/File:Romanow-Schaf_(Tierpark_Zittau).jpg | jeune de face (parc de Zittau) : petite étoile, oreilles de côté, pattes noires hautes |
| R7 | https://commons.wikimedia.org/wiki/File:Fj%C3%A4ll-K%C3%BChe,_Romanow-Schaf_(Tierpark_Zittau).jpg | petit troupeau au râtelier, adultes gris en toison longue et **agneaux noirs** |
| R8 | https://commons.wikimedia.org/wiki/File:VDNH_715_(10656586435).jpg | **brebis et quatre agneaux noirs** (VDNKh, Moscou ; photo de Wikipédia RU « remarquez le changement de couleur avec l'âge ») ; petite étoile chez la mère et les agneaux ; mesurée |
| R9 | https://commons.wikimedia.org/wiki/File:Romanov_sheep,_Male_lambs.JPG | trois jeunes béliers : toison grise courte, listes, oreilles orange de soleil (fines, translucides), pattes noires |
| R10 | https://commons.wikimedia.org/wiki/File:Romanov_sheep.jpg | brebis couchée au soleil : gris tirant sur le brun, tête noire, pattes noires ; mesurée |
| R11 | https://commons.wikimedia.org/wiki/File:2019-07-15._%D0%97%D0%BE%D0%BE%D0%BF%D0%B0%D1%80%D0%BA_%D0%BF%D1%80%D0%B8_%D1%85%D1%80%D0%B0%D0%BC%D0%B5_%D0%90%D1%80%D1%85%D0%B0%D0%BD%D0%B3%D0%B5%D0%BB%D0%B0_%D0%9C%D0%B8%D1%85%D0%B0%D0%B8%D0%BB%D0%B0_%D0%B2_%D0%95%D0%B9%D1%81%D0%BA%D0%B5_040.jpg | zoo de Ieïsk, juillet, bêtes **tondues** : bout de queue blanc, couronnes blanches au-dessus des onglons, liste, pattes longues ; mesurée |
| R12 | https://commons.wikimedia.org/wiki/File:2019-07-15._%D0%97%D0%BE%D0%BE%D0%BF%D0%B0%D1%80%D0%BA_%D0%BF%D1%80%D0%B8_%D1%85%D1%80%D0%B0%D0%BC%D0%B5_%D0%90%D1%80%D1%85%D0%B0%D0%BD%D0%B3%D0%B5%D0%BB%D0%B0_%D0%9C%D0%B8%D1%85%D0%B0%D0%B8%D0%BB%D0%B0_%D0%B2_%D0%95%D0%B9%D1%81%D0%BA%D0%B5_041.jpg | jeune gris foncé qui **broute**, de profil vers la gauche : encolure basse, queue noire |
| R13 | https://commons.wikimedia.org/wiki/File:2019-07-15._%D0%97%D0%BE%D0%BE%D0%BF%D0%B0%D1%80%D0%BA_%D0%BF%D1%80%D0%B8_%D1%85%D1%80%D0%B0%D0%BC%D0%B5_%D0%90%D1%80%D1%85%D0%B0%D0%BD%D0%B3%D0%B5%D0%BB%D0%B0_%D0%9C%D0%B8%D1%85%D0%B0%D0%B8%D0%BB%D0%B0_%D0%B2_%D0%95%D0%B9%D1%81%D0%BA%D0%B5_042.jpg | deux bêtes tondues qui broutent : liste large, pied blanc, ligne du dos |
| R14 | https://commons.wikimedia.org/wiki/File:2019-07-15._%D0%97%D0%BE%D0%BE%D0%BF%D0%B0%D1%80%D0%BA_%D0%BF%D1%80%D0%B8_%D1%85%D1%80%D0%B0%D0%BC%D0%B5_%D0%90%D1%80%D1%85%D0%B0%D0%BD%D0%B3%D0%B5%D0%BB%D0%B0_%D0%9C%D0%B8%D1%85%D0%B0%D0%B8%D0%BB%D0%B0_%D0%B2_%D0%95%D0%B9%D1%81%D0%BA%D0%B5_043.jpg | trois bêtes couchées : pattes repliées, tête sur le sol ou levée, oreilles, une encolure plus noire |
| R15 | https://commons.wikimedia.org/wiki/File:Romanov_Sheep_(3728739860).jpg | tête **blanche** à bande noire sur l'œil, iris ambré : sans doute un **croisé** ; contre-exemple |
| R16 | https://commons.wikimedia.org/wiki/File:Pure_Romanov_Ewe_-_Chimera_Modern_Farms_-_Iran.jpg | brebis dite « pure » à toison blanche et brune : **atypique**, non utilisée |
| R17 | photo de couverture de la fiche ASSONAPA (lien [S ASSONAPA]) | brebis de trois quarts tournée vers la droite, au soleil : **toison longue crème**, museau blanc à peau rose, pattes noires, autres brebis à pied blanc ; mesurée |
| R18 | https://zoo.kyiv.ua/wp-content/uploads/2025/04/romanivska-vivczya.jpg (page [S Kyiv]) | brebis debout en toison longue, tête haute, troupeau au pré : **pattes longues**, encolure dressée, œil ambré ; mesurée |
| R19 | https://raisingsheep.net/wp-content/uploads/2021/01/romanov-ram.jpg (page https://raisingsheep.net/romanov.html) | **bélier presque de profil** vers la gauche : crinière noire qui descend jusqu'aux antérieurs, tache noire derrière l'épaule, liste, chanfrein busqué ; mesuré |
| R20 | https://www.raisingsheep.net/wp-content/uploads/2020/08/romanov-sheep-1024x538.jpg (même page) | têtes de brebis de face : listes de toutes largeurs, **grandes oreilles** tenues à l'horizontale, intérieur sombre |
| Gotland 1 | https://commons.wikimedia.org/wiki/File:Gotland_pelt_sheep.jpg | comparaison : brebis Gotland, toison grise en boucles plus longues et lustrées, tête noire **sans liste**, oreilles moyennes |
| Gotland 2 | https://commons.wikimedia.org/wiki/File:Gotlandschaf_Aue_am_Gatschower_Os.jpg | rangée dans la catégorie Gotland, mais brune et cornue : sans doute un Gute ou un croisé ; non utilisée |
| Planche | https://commons.wikimedia.org/wiki/Category:Romanov_sheep et https://www.flickr.com/photos/tags/romanovsheep/ | la catégorie entière ; les étiquettes Flickr « romanov » ne montrent presque que la famille impériale (aucune photo retenue) |

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Contradiction |
|---|---|
| Poids des brebis | 45–55 kg (jusqu'à 90) [S WP-RU] ; 45–50 [S FAO][S WP-DE] ; 40–50 [S WP-EN] ; 50–60 [S ASSONAPA][S USAMV][S Kyiv] ; **60–70** [S WP-FR] ; 45 kg à 2 ans à Bourges dans les années 1960 [S Ricordeau1976] ; moyenne russe de 2023 : 57 kg [S Furaeva2024]. La race s'est alourdie ; les Romanov de France et des Pays-Bas sont « parfois nettement plus grands et lourds » [S WP-DE]. |
| Poids des béliers | 65–75 [S WP-RU] ; 55–80 [S FAO][S WP-EN] ; 70–90 [S ASSONAPA] ; 75–90 [S USAMV] ; 80–90 [S WP-FR] ; moyenne 2023 : 76 kg, 83 kg dans les élevages de sélection [S Furaeva2024]. |
| Hauteur au garrot | « jusqu'à 70 cm » (1908) [S WP-RU] ; « max 70 cm » [S ASSONAPA] ; « en moyenne 70 cm » [S USAMV] ; brebis « environ 60 » [S Kyiv] contre 65,2 mesurés [S RUDN2022] ; béliers 69,8 en moyenne, mais 77–80 cm pour certains [S Furaeva2025]. |
| Cornes | sans cornes chez les deux sexes, exigé [S Ordre540][S Furaeva2025][S USAMV] ; « sans cornes et cornus, les sans-cornes préférables » (1908) [S WP-RU] ; type grossier à cornes, type standard « cornu ou sans cornes » [S FAO] ; « le plus souvent sans cornes » [S WP-DE]. |
| Marque de la tête | « liste ou étoile chez la plupart » [S Ordre540] ; « tête noire à **large** liste blanche » [S FAO] ; « noire ou à petits signes blancs » [S ASSONAPA] ; « des marques blanches sur le museau et les oreilles, en général » [S WP-RU] ; le museau blanc, fréquent sur les photos françaises (R3, R17), n'est pas mentionné par le standard, qui refuse le blanc « ailleurs » qu'aux pieds et au bout de la queue. |
| Croupe | « крестец свислым » : le standard veut une croupe **tombante** [S Ordre540] ; « зад ровный… не свислый » : croupe droite, **non tombante** [S Furaeva2025] ; la FAO range la croupe « étroite et tombante » parmi les **défauts** [S FAO] ; « sacrum légèrement abaissé » [S USAMV]. On dessine une croupe qui tombe **un peu**. |
| Encolure | « longue, charnue » [S Furaeva2025] ; « de longueur moyenne pour la race » [S Ordre540]. |
| Tête | « petite » [S ASSONAPA][S WP-RU] ; « de taille moyenne » [S Ordre540]. |
| Queue | 8–10 cm [S WP-RU] ; 9–10 cm [S Furaeva2025] ; 13 cm en moyenne [S ASSONAPA] ; « jusqu'à 13 cm » chez le bélier (1908) [S WP-RU]. |
| Couleur de la toison | « gris acier à reflet bleu » [S WP-RU][S FAO][S Ordre540] ; « elle marie le noir et le marron avec une prédominance de blanc » [S WP-FR]. Les photos montrent du gris bleuté (toison courte) au crème (toison longue). |
| Origine du nom | district de Romanov-Borissoglebsk, ville de Romanov [S WP-RU][S USAMV][S WP-EN] ; « de la famille Romanov » [S WP-FR]. |
| Date de la race | fin du XVIIᵉ siècle [S FAO][S WP-DE] ; XVIIIᵉ siècle [S WP-RU][S WP-FR]. |
| Tontes | 3 par an (mars, juin, octobre) [S WP-RU] ; 3 ou 4 [S FAO]. |
| Poids de toison | brebis 1,4–1,8 kg par an [S WP-RU], 1,5–1,8 [S FAO], 2,2 kg en 2023 [S Furaeva2024] ; 4,5 kg en moyenne [S WP-EN]. |
| Diamètre des fibres | jarres 60–90 µm, duvet 20–27 µm [S WP-RU] ; 71,9 et 20,9 µm [S WP-EN]. |
| Dépigmentation de l'agneau | commence à 2–4 semaines, finie à 3–5 mois [S WP-RU] ; « après 1 à 3 mois » [S USAMV]. |
| Maturité sexuelle | 3–4 mois [S WP-EN] ; 4–5 mois [S Kyiv] ; 10–12 mois [S WP-RU] ; brebis mises à la reproduction à 14–18 mois [S FAO]. |
| Effectifs en France | 15 000 brebis pures et 80 000 F1 en 1987 [S AFS1988] ; 3 000 animaux dont 1 000 brebis inscrites [S WP-FR] (chiffre plus récent, non daté). |
| Largeur de poitrine des brebis | 27,4 cm [S RUDN2022] contre 19,6 cm chez les béliers [S Furaeva2025] : la valeur des brebis paraît trop forte (peut-être une profondeur) ; non utilisée. |
| Mesures de la tête, des oreilles, du carpe, du jarret | aucune source ; tout est [P] ou [E] (section 2). |
