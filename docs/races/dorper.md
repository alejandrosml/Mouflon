> Fiche de la race « Dorper », préparée le 3 novembre 2026 pour dessiner un futur `pets/dorper` dans Mouflon. Le dessin viendra ensuite (`docs/chantiers/races-dessins.md`).

# Dorper : fiche pour un dessin de profil

**Noms :** Dorper (EN, FR, AF), parfois « Blackhead Dorper » pour le distinguer du **White Dorper** (tout blanc), qui appartient à la même race et au même livre généalogique ; ce dernier s'est d'abord appelé *Dorsian* (« Dorsie » en afrikaans) [SA-hist][FAO-Wilson]. Le nom vient des premières syllabes des deux races parentes : **Dor**set Horn × Blackhead **Per**sian [ADSBS-FAQ].
**Préparée le** 3 novembre 2026, d'après le standard de la société sud-africaine (Dorper Sheep Breeders' Society of South Africa), ceux des sociétés américaine, australienne et néo-zélandaise, la description de la FAO, cinq études de mensurations (Afrique du Sud, Mexique, Malaisie, Indonésie, Uruguay par la base DAGRIS de l'ILRI), une étude sur la mue, une revue sur le comportement au pâturage, et une vingtaine de photos regardées de près (section 10.2).

**Comment lire les étiquettes :**
- **[S]** : source écrite (le code renvoie à la section 10.1).
- **[P]** : vu ou mesuré par moi sur une photo de la section 10.2 (pixels comptés, couleurs moyennées sur des carrés de 5×5 à 9×9 px avec Python/PIL).
- **[E]** : mon estimation, tirée des photos et de l'anatomie générale du mouton. Aucune source ne la donne.

H désigne partout la **hauteur au garrot, os compris, poil non compté** (le poil d'été fait à peine 1 cm). Pour l'individu choisi, **H = 70 cm** [E, justifiée en 2.1].

---

## 0. En bref

- **Le sujet :** un **bélier sud-africain de concours, « type 5 », de trois à quatre ans** (six dents ou plus), vu **en fin d'été, après sa mue naturelle** : couvert d'un poil court et lisse (moins de 1,5 cm), sans toison. Il se tient debout, carré, de profil strict, **tourné vers la droite** (on voit son flanc gauche), tête haute comme au concours.
- **L'allure en une phrase :** un mouton **blanc, massif, en forme de tonneau sur des pattes courtes**, à **tête et cou noirs**. Il a :
  - une **tête noire entière**, au poil ras et mat, longue et forte, au chanfrein **légèrement busqué**, au museau large, plissé chez le bélier ;
  - un **cou noir jusqu'aux deux tiers environ**, la limite noir/blanc **nette**, en collier oblique devant l'épaule ; **rien de noir derrière** ;
  - des **oreilles moyennes, noires, portées à l'horizontale ou un peu tombantes** ;
  - pas de vraies cornes : un **petit moignon corné (« scur » ou base de corne)** de 3 à 4 cm sur la nuque, que le standard sud-africain tient pour l'idéal ;
  - un corps **long, large et profond** (la poitrine fait plus de la moitié de la hauteur), un **dos droit**, une **croupe longue et large**, des gigots descendant jusqu'au jarret ;
  - des **pattes blanches à poil ras** et des **onglons noirs** ;
  - la **queue coupée** en un court moignon (pratique d'élevage, voir 2.6).

**Palette de base** (détails en section 4)

| Zone | Base | Lumière | Ombre |
|---|---|---|---|
| Corps blanc (poil d'été) | `#EDE3D2` | `#F7F0E4` | `#C9C2B6`, sous le ventre `#A9A196` |
| Tête et cou noirs (poil mat) | `#221C19` | reflet brun `#3A302B`, reflet gris `#4A4A50` | `#100D0D` |
| Truffe, lèvres (peau nue) | `#1B1716` | reflet humide `#3C3936` | narine `#0A0808` |
| Iris | `#B97F55` (ambre orangé) | `#CE9464` | `#A17150`, pupille `#17232A` |
| Bord des paupières | `#6E727D` (gris bleuté) | | |
| Pattes | `#E8DECD` | `#F4EDE2` | `#ACA394` |
| Onglons | `#1C1817` | usure `#4E4642` | `#0A0909` |
| Scur (moignon corné) | `#7F6644` | `#9A825E` | `#574330` |

---

## 1. L'individu à dessiner, et pourquoi

### 1.1 Un bélier

1. **Le Dorper est une race à viande dont la réputation repose sur ses béliers.** Elle a été créée pour produire « fast growing and heavily muscled lambs » (des agneaux à croissance rapide et très musclés) [ADSBS-about] ; aux États-Unis, ses béliers servent de pères terminaux en croisement (« as terminal sires produce fast growing, muscular lambs ») [ADSBS-about].
2. **Le standard demande un caractère sexuel marqué**, plus visible chez le mâle : « Rams should appear masculine and carry secondary development, such as wrinkled skin, on a broad muzzle » (les béliers doivent paraître masculins et porter des caractères secondaires, comme une peau plissée sur un museau large) [ADSBS-std] ; l'Australie demande « a masculine appearance » pour la tête et le corps du bélier [AU-std]. Les plis du museau et le front ridé se voient bien sur les béliers photographiés [P : Jr. Show Champion Dorper Ram 2018 ; Domba dorper fullblood jantan].
3. **La musculature et le « tonneau »** (« long, deep wide body, ribs well sprung, loin broad and full ») [SA-std] sont les plus nets chez le bélier. Le standard américain veut d'ailleurs pour le bélier un « reverse wedge » (coin inversé : avant puissant, poitrine profonde) et pour la brebis un coin « féminin », plus léger devant [ADSBS-std].
4. **La meilleure photo de profil strict tournée vers la droite est un bélier** : « Dorper ram in the Kalahari », photographié dans les dunes rouges du Kalahari en juillet 2012 [P]. Elle donne les proportions, la limite du noir et le moignon corné.

### 1.2 L'individu exact

- **Origine et type :** sud-africain, du type que le classement de la société sud-africaine noterait **T5** (« very good », 5 points) : conformation, taille, répartition du gras, couleur et couverture toutes au plus haut (voir 4.4 pour le système de notes) [SA-std].
- **Âge :** trois à quatre ans, « 6 Teeth + » (six dents ou plus) dans le tableau de poids de la société [SA-std]. Il est adulte fini.
- **Poids :** environ **105 kg**, la moyenne sud-africaine des béliers de six dents et plus (105,5 kg) [SA-std] ; aux États-Unis, « Mature rams typically average between 230-270 pounds » (104 à 122 kg) [ADSBS-FAQ].
- **Saison et couverture :** **fin d'été austral (février-mars), après la mue de printemps**. Il porte le poil court et lâche du standard, « short, loose, light covering of hair and wool » [SA-std], moins de 1,5 cm, sans plaque de laine. C'est presque l'aspect des animaux tondus pour le concours (les Dorper « are shown fully shorn », présentés tondus [ADSBS-FAQ] ; en Nouvelle-Zélande, « shorn less than 6 weeks or have a cover less than 1.5cm » [NZ-std]), mais sans les traces de tondeuse. La variante « avant la mue » est décrite en 3.2.
- **Pose :** debout, carré, de profil strict, tête haute (nuque au-dessus du garrot), oreilles à l'horizontale.
- **Taille :** **H = 70 cm** [E]. Les mesures publiées de béliers adultes vont de 63 à 68 cm (Mexique, béliers de 3 à 4 ans, mais légers : 53 à 61 kg [Moreno 2026]) ; des béliers « fullblood » de 12 mois mesurent déjà 73 cm (Indonésie [Utomo 2026]). Un bélier de concours de 105 kg est dans le haut de cette fourchette.
- **À ne pas recopier de la photo de référence :** la boucle d'identification blanche à l'oreille ; le sable rouge reflété sous le ventre (ombre orangée, voir 4.1) ; la patte postérieure gauche tirée en arrière (pose de présentation).

### 1.3 Pour l'autre sexe et pour l'agneau

| Trait | Bélier (choisi) | Brebis | Agneau (4 à 10 semaines) |
|---|---|---|---|
| Hauteur au garrot | 63 à 68 cm (3-4 ans, Mexique) [Moreno 2026] ; H = 70 [E] | 62 à 64 cm (3-4 ans, Mexique) [Moreno 2026] ; 60,7 cm (1-2 ans, Afrique du Sud) [Madikadike 2022] ; 61 et 66 cm (Uruguay) [DAGRIS] | 0,55 à 0,65 H adulte [E] |
| Poids | 105,5 kg (6 dents +) [SA-std] ; 104 à 122 kg [ADSBS-FAQ] ; 90 à 105 kg [Kovács 2008] | 95,0 kg (6 dents +, « some of the ewes might have been pregnant ») [SA-std] ; 82 à 95 kg [ADSBS-FAQ] ; 70 à 95 kg [Kovács 2008] | 3,5 à 4,5 kg à la naissance [FAO-Wilson] ; 36 kg vers 3,5 à 4 mois [NZ-std][Kovács 2008] |
| Tête | forte, longue, museau large et plissé, « masculine » [ADSBS-std][P] | plus fine, « well defined feminine features » [ADSBS-std] ; museau plus étroit, profil plus droit [P : Grand Champion Dorper Ewe ; Ovce a jehne dorper] | ronde, courte ; oreilles grandes pour la tête, à l'horizontale [P : Ovce a jehne dorper ; Borrego dorper] |
| Silhouette | « reverse wedge » : avant puissant [ADSBS-std] | « feminine wedge » : avant affiné, arrière et ventre puissants [ADSBS-std] ; « lighter in front and getting bigger and heavier to the back » [NZ-std] | longues pattes, corps court ; poil laineux, frisé, blanc pur [P : Droperschaf1] |
| Noir | tête et cou | même règle ; le noir descend parfois moins bas sur le cou [P] | le noir est déjà là à la naissance, souvent **limité à la tête et à la gorge** [P : Droperschaf1 ; Ovce a jehne dorper] |
| Organes | scrotum qui « should not drop below the point of hock » [ADSBS-std], bien visible entre les cuisses | pis pigmenté, deux trayons [ADSBS-std] | queue entière jusqu'à la coupe (si elle a lieu) [P : Droperschaf1] |

### 1.4 Le White Dorper, en bref

- **Même race, même standard, sauf la couleur** : « Exactly the same breed standards are applied to both Blackhead Dorper and White Dorper except regarding colour and pigmentation » [SA-hist].
- **Origine :** des éleveurs ont gardé les agneaux blancs des premiers croisements Dorset Horn × Persan (« The first crosses between Persian and Dorset Horn were mostly spotted but there were also white lambs »), et des croisements Dorset Horn × Van Rooy ; on les appela *Dorsian* jusqu'à la fusion des deux sociétés en 1964 [SA-hist][AU-hist].
- **Couleur :** « A white sheep, fully pigmented around the eyes, under the tail, on the udder and the teats is the ideal. A limited number of other coloured spots is permissible on the ears and underline » [SA-std] ; en Australie, ces petites taches peuvent être « both black and ginger » (noires et rousses) [AU-std]. Aux États-Unis, « Color may appear on the hooves » [ADSBS-std].
- **Sur les photos :** tête blanche à poil ras, **paupières et narines souvent roses** chez les animaux de moindre qualité (un défaut de pigmentation) [P : Carneiro White Dorper ; Dorper white ram] ; **onglons ambrés** `#E4C69F` [P : Jr. Show Reserve Champion Dorper Ram 2018].
- Pour Mouflon, le White Dorper pourrait être une variante de couleur du même dessin (mêmes proportions) ; cette fiche décrit le Dorper à tête noire.

---

## 2. Tailles et proportions

### 2.1 Chiffres publiés

| Mesure | Bélier | Brebis | Sources |
|---|---|---|---|
| Hauteur au garrot, adultes de 3 à 4 ans (Mexique, élevage intensif, n = 33 béliers, 59 brebis) | **68,0 ± 4,7 cm** (rang social haut), **63,3 ± 3,4** (bas) | **64,0 ± 3,2**, **62,1 ± 3,9** | [Moreno 2026] |
| Tour de poitrine, même étude | **93,4 ± 6,4** / 90,4 ± 5,7 cm | 83,9 ± 7,5 / 81,3 ± 4,8 cm | [Moreno 2026] |
| Poids, même étude | 61,5 / 53,1 kg | 40,4 / 37,3 kg | [Moreno 2026] (animaux légers pour la race) |
| Cornes, même étude | **2 béliers sur 14 et 4 sur 19 portent des cornes (14 et 21 %)** | — | [Moreno 2026] |
| Béliers « fullblood » de 12 mois (Indonésie, n = 10) | garrot **73 cm**, longueur **87 cm**, tour de poitrine **100 cm** | — | [Utomo 2026] |
| Sujets de 1 à 2 ans (Afrique du Sud, n = 50) | garrot 60,7 ; croupe 66,9 ; longueur 74,9 ; sternum au sol 40,9 ; poitrine 82,7 cm ; 30,5 kg | garrot 60,7 ; croupe 62,3 ; longueur 68,5 ; sternum au sol 41,2 ; poitrine 84,0 cm ; 33,4 kg | [Madikadike 2022] |
| Sujets de 12-13 mois (Malaisie, 10 par groupe) | garrot 56,9 ; longueur (base du cou à la base de la queue) 61,7 ; poitrine 74,5 ; **tête 21,5 ; oreille 13,0 × 6,8 cm** | garrot 54,1 ; longueur 55,3 ; poitrine 68,6 ; **tête 20,1 ; oreille 12,9 × 6,6 cm** | [Nordin 2025] |
| Brebis adultes (Uruguay, station ; n = 40 pour la seconde valeur) | — | garrot 66,1 et 60,9 cm ; poitrine 82,6 et 75,8 cm | [DAGRIS] |
| Hauteur « 60 cm » | « height of 60 - 60 cm, larger size is not desired » (sic) | | [Kovács 2008] |
| Rapport corps/pattes | « a **60:40 ratio** of height to body and legs » (60 % de la hauteur pour le corps, 40 % pour les pattes) | idem | [ADSBS-std] |
| Poids adulte | 105,5 kg [SA-std] ; 104 à 122 kg [ADSBS-FAQ] ; 90 à 105 kg [Kovács 2008] | 95 kg [SA-std] ; 82 à 95 kg [ADSBS-FAQ] ; 70 à 95 kg [Kovács 2008] | |

**Pourquoi H = 70 cm.** Aucune source ne donne la hauteur d'un bélier sud-africain de concours. Les béliers adultes mesurés au Mexique (63 à 68 cm) pèsent à peine 53 à 61 kg ; les jeunes béliers « fullblood » indonésiens de 12 mois mesurent déjà 73 cm. Un bélier de 105 kg se place entre les deux : 70 cm [E].

**Ce qui manque :** aucune source sur la hauteur du carpe et du jarret, la longueur de la queue, l'angle des articulations. Ces valeurs sont prises sur photo [P] ou estimées [E] ci-dessous.

### 2.2 Ce que disent les standards de la silhouette

- **Afrique du Sud** [SA-std] :
  - encolure « of medium length, well-fleshed and broad and well-coupled to the forequarters » (de longueur moyenne, bien musclée, large, bien attachée aux épaules) ;
  - « A moderate protrusion of the brisket beyond the shoulders, moderate width and good depth are the ideal » (le poitrail dépasse modérément les épaules) ;
  - « a long straight back and not a "devil's grip". A slight dip behind the shoulders is permissible » (dos long et droit, sans pincement derrière les épaules ; un léger creux derrière l'épaule est permis) ;
  - « A long and wide rump is the ideal. The inner and outer twist to be well fleshed and deep » (croupe longue et large, culotte bien remplie et descendue).
  - « The sheep should be symmetrical and well-proportioned. A calm temperament with a vigorous appearance is the ideal. »
- **États-Unis** [ADSBS-std] : « symmetrical and balanced where the height, width, length and depth of body are proportionate » ; le **rapport 60:40** ; « muscling extends down to hocks » (le muscle descend jusqu'aux jarrets).
- **Australie** [AU-std] : des épaules « not excessively broad », « stretch » (de la longueur), « a reasonable amount of lift » (un peu de hauteur sur pattes) pour les régions pastorales, mais « excessively tall sheep without well defined muscle should be avoided ».
- **Hongrie (revue)** : « stocky with short legs » (trapu, court sur pattes) [Kovács 2008].

**Pour le dessin :** un tonneau long, profond et large, posé bas ; le dos tiré au cordeau ; l'avant plein et arrondi (le poitrail dépasse un peu devant les antérieurs) ; la croupe longue, presque horizontale, terminée par une culotte ronde qui descend jusqu'au jarret.

### 2.3 Rapports de dessin (bélier, H = 70 cm)

Mesures prises sur trois profils :
- « Dorper ram in the Kalahari » (1280 px), le sujet : profil strict vers la droite, sol de sable. Il est un peu de trois quarts arrière : les sabots postérieurs touchent le sol 45 px plus bas que les antérieurs, si bien que l'arrière est un peu agrandi. Je rapporte tout à l'avant (H = 632 px, du garrot à la pince de l'antérieur gauche).
- « Open Show Res Ch. Dorper Ram (23726476118) » : bélier tondu de concours, profil vers la droite, sol plat (H = 372 px).
- « Grand Champion Dorper Ewe (44234017935) » : brebis tondue de concours, profil vers la droite (H = 217 px).

| Mesure | En fraction de H | En cm (H = 70) | Étiquette |
|---|---|---|---|
| Hauteur de la croupe | 1,00 à 1,03 | 70 à 72 | [S : Madikadike, croupe ≥ garrot chez les jeunes][P : dos de niveau] |
| Profondeur de poitrine (garrot au sternum) | **0,56** | 39 | [P : 0,565 Kalahari ; 0,52 bélier de concours plus jeune ; 0,50 brebis][S : ADSBS « 60:40 »] |
| Sol au sternum | **0,44** | 31 | [P][S : ADSBS 40 % de pattes] |
| Bas du ventre au-dessus du sol | **0,34** | 24 | [P : 0,34 Kalahari ; 0,34 brebis ; 0,44 bélier plus jeune] (le ventre du bélier adulte descend plus bas que le sternum) |
| Longueur de la silhouette, poitrail à fesse | **1,36 à 1,42** | 95 à 99 | [P : 1,36 à 1,43 Kalahari selon la correction de perspective ; 1,35 bélier de concours ; 1,43 brebis] |
| Longueur du corps, pointe de l'épaule à la pointe de la fesse (os) | 1,19 à 1,23 | 83 à 86 | [S : Utomo 87/73 ; Madikadike béliers 74,9/60,7] |
| Tour de poitrine | 1,37 (jusqu'à 1,45 chez un bélier gras) | 96 à 101 | [S : Moreno 93,4/68 ; Utomo 100/73][E] |
| Longueur de la tête, nuque au bout du nez | **0,32 à 0,36** | 22 à 25 | [P : 0,32 Kalahari, tête levée][S : Nordin, 21,5 cm pour 56,9 cm de garrot, soit 0,38 chez un jeune] |
| Hauteur de la tête de profil (front à la ganache) | 0,22 à 0,24 | 15 à 17 | [P] |
| Oreille, longueur | 0,19 à 0,21 | 13 à 15 | [S : Nordin, 13,0 cm à 12-13 mois, la taille de l'oreille variant peu avec l'âge][E adulte] |
| Oreille, largeur | 0,10 | 7 | [S : Nordin, 6,8 cm] |
| Encolure, longueur du dessus (nuque au garrot) | 0,55 | 39 | [P : 356 px sur 632, Kalahari] |
| Moignon de queue | 0,05 à 0,08 | 4 à 6 | [P : Kalahari, Dorper white ram ; voir 2.6] |

**Masses relatives** [P] :
- le corps fait environ **4,3 longueurs de tête** de long ;
- vu de profil, la poitrine, profonde, et le ventre plein font un bloc presque rectangulaire ; les pattes visibles sous le ventre ne font qu'**un tiers de la hauteur** ;
- les gigots descendent jusqu'à **0,35 H** au-dessus du sol, au niveau du jarret : « muscling extends down to hocks » [ADSBS-std][P].

### 2.4 L'encolure

**Les sources :** « medium length, well-fleshed and broad and well-coupled » [SA-std][NZ-std][ADSBS-std]. Aucune ne parle d'un cou court ; c'est la masse musculaire et la tête noire qui la font paraître courte.

**Sur les photos :**
- **Longueur :** du garrot à la nuque le long du dessus, **0,55 H** (39 cm) [P : Kalahari]. Le dessous (du poitrail à la gorge) fait environ 0,4 H [P].
- **Épaisseur :** à sa base, le cou est **aussi épais que la tête est longue** (0,35 H) ; il se rétrécit vers la nuque, où il ne fait plus que la hauteur de la tête [P].
- **Au repos (pose de concours) :** le dessus du cou monte d'environ **30°** au-dessus de l'horizontale depuis le garrot ; la nuque est à **0,25 à 0,3 H au-dessus du garrot**. L'axe de la face (nuque vers bout du nez) descend d'environ **40°** sous l'horizontale [P : 31° et 39° mesurés sur Kalahari]. Au pré, sans meneur, la tête se porte plus bas : nuque à peu près au niveau du garrot, cou à 10-20° [P : Ovelha Dorper ; Point Buchon Trail sheep ; E].
- **Sur le dessus du cou tondu**, derrière la limite du noir, la peau fait **des plis transversaux en accordéon** quand la tête est relevée [P : Kalahari, cinq à six plis nets, `#D2B88A` dans les creux].
- **En alerte :** le cou se redresse jusqu'à 40-45°, la face à 25-30° sous l'horizontale [E d'après P : Dorper Sheep Breed Antigua].
- **En broutant :** le cou descend à 40-50° sous l'horizontale ; la face est presque verticale [E, général chez le mouton ; P : animaux broutant de Point Buchon Trail sheep]. Le Dorper **broute aussi plus haut** que les autres moutons, dans les buissons (8.1) : cou à l'horizontale ou relevé, tête tendue vers une branche.

**Pour le moteur à cou articulé :** deux segments de longueur voisine (0,27 H chacun) : un premier épais, du garrot à mi-cou, qui porte la limite du noir à son extrémité ; un second plus mince, de mi-cou à la nuque [E]. Les plis du dessus se creusent quand la tête monte.

### 2.5 Les articulations des pattes

| Repère | Au-dessus du sol (fraction de H) | Étiquette |
|---|---|---|
| Coude | 0,44 à 0,47 | [P : au niveau du sternum ; E] |
| Carpe (« genou » de l'antérieur), centre | **0,26 à 0,29** | [P : 0,29 Kalahari ; 0,24 bélier de concours] |
| Boulet antérieur | 0,08 | [E] |
| Grasset | 0,52 à 0,58, caché dans le gigot | [E] |
| Jarret, pointe (calcanéum) | **0,29 à 0,31** | [P : 0,30 Kalahari et bélier de concours] |
| Boulet postérieur | 0,08 | [E] |

**Angles :**
- **Jarret :** le standard veut des jarrets « strong without a tendency to turn in or out. Sickle, bandy or perpendicular hocks are culling faults » (jarrets coudés en faucille, cagneux ou droits sont éliminatoires) [SA-std]. Angle d'un jarret au repos, entre jambe et canon : **140 à 150°** [E]. Sur la photo de référence, la patte gauche est tirée en arrière pour la présentation et l'angle se ferme à environ 127° [P] : ne pas le recopier pour la pose debout.
- **Paturons :** « strong pasterns » ; « Weak pasterns and X legs must be discriminated against » [SA-std]. Courts, environ **60°** sur l'horizontale [E].
- **Antérieurs :** verticaux, d'aplomb sous l'épaule, « straight and well-placed » [SA-std] ; en Australie, « The shoulders should come down straight however a little inward shape is tolerable » [AU-std].

### 2.6 La queue

- **Le standard sud-africain n'en dit rien.** Aux États-Unis, le standard des agneaux de boucherie n'admet aucune « surgical alterations other than re-docking » (aucune modification chirurgicale sauf la recoupe de la queue) [ADSBS-market] : la coupe est donc courante.
- À l'inverse, le Dorper figure parmi les races qui « generally do not require tail docking », parce que les moutons à poil n'ont pas « long, wooly tails » [Sheep101].
- **Sur les photos :** les adultes ont presque tous **un court moignon** à l'attache de la queue, 4 à 6 cm, couvert de poil blanc [P : Kalahari, petite bosse en haut de la culotte ; Dorper white ram ; brebis White Dorper vues de dos dans Dorper sheep.jpg]. L'agneau de « Droperschaf1 » porte au contraire **une queue entière, fine, couverte de poil laineux, qui atteint le jarret** [P : Droperschaf1].
- Le Dorper n'a **pas la queue grasse** ni la croupe grasse de son ancêtre persan (comparaison en 9.2) : sa queue, entière, est mince.
- **À dessiner :** un moignon de 0,05 à 0,08 H, attaché au ras du dos, à peine visible au-dessus des fesses. **Variante entière** (élevages qui ne coupent pas) : une queue mince de 0,4 H environ, pendante jusqu'au jarret [E d'après P : Droperschaf1].

---

## 3. Le poil et la mue

### 3.1 Ce que disent les standards

- **Afrique du Sud** [SA-std] : « The ideal is a short, loose, light covering of hair and wool with wool predominating on fore quarter and with a natural clean kemp underline. Too much wool or hair is undesirable. Exclusively wool or hair is a fault. **Manes are a disqualifier** » (l'idéal est une couverture courte, lâche et légère de poil et de laine, la laine dominant sur l'avant, le dessous naturellement net, en jarre ; trop de laine ou de poil n'est pas souhaitable ; uniquement de la laine ou uniquement du poil est un défaut ; **une crinière est éliminatoire**). La FAO le reprend : « A mane is grounds for disqualification » [FAO-Wilson].
- **États-Unis** [ADSBS-std] : « a short, loose, light mixture of hair and wool on their body with a natural clean underline. The head should be covered in short clean hair. Sheep may naturally shed all their wool or have wool covering remain on their sides, hindquarters, and neck. Wool on underlines and belly is undesirable. Covering faults include sheep that have course hair, long and dense wool, or **a coarse hairy mane** ».
- **Australie** [AU-std] : « The breech and belly should be clean of wool » ; « A covering of fine hair or soft short wool is preferred, whereas a covering of coarse hair is to be avoided » ; « **A fully shedding animal is not necessary** ».
- **Nouvelle-Zélande** [NZ-std] : deux types de couverture coexistent, « either wool or hair. Both types are good shedders » ; des jumeaux peuvent naître l'un à poil, l'autre à laine. « Some animals will shed their entire fleece while others will not shed so well. **Under the neck, on the belly and around the breech is always hair only** ».
- **La tête** : « short, dullish black hair » (poil court, noir, mat) [SA-std][NZ-std].

**Pas de crinière.** On attribue parfois au Dorper une crinière de poil, comme en ont certains moutons à poil ; c'est justement un **défaut éliminatoire** dans le standard sud-africain et un défaut aux États-Unis. Le bélier se dessine **sans crinière** : l'encolure garde le même poil court que le reste.

### 3.2 La mue

- **Elle est voulue :** « A Dorper should shed its covering each summer. Shedding ability is a trait that should be selected for » [ADSBS-FAQ] ; « no shearing is required » [SA-char]. Le Persan à tête noire a été choisi pour cela (« coat shedding ») [AU-hist].
- **Quand :** au printemps et en été, « shed in late spring and summer » [WP-EN]. Dans un élevage expérimental chinois (Ningxia), une partie des brebis **n'avait toujours pas mué en mai** et a dû être tondue ; chez celles qui muent, les follicules secondaires sont en anagène en septembre, au repos (télogène) en janvier, en début d'anagène en mars [Shedding 2024]. La mue se fait donc à la reprise de croissance du printemps, quand le nouveau poil pousse l'ancienne laine.
- **Comment elle se voit** [P : Point Buchon Trail sheep, Californie, 29 février 2008, troupeau avant la mue] :
  - sur le dos, la croupe et le haut des flancs, une **couche de laine lâche de 3 à 5 cm**, blanc sale `#D2D0C4`, aux pointes salies, qui se **soulève en plaques** et en mèches ;
  - les plaques se décollent et laissent paraître en dessous le **poil court et lisse**, plus blanc ;
  - le **ventre, le bas des flancs, les pattes et la tête restent ras** [P][S : NZ-std] ;
  - chez certaines brebis tondues ou déjà muées, une **plaque de laine reste sur l'épaule et le garrot** [P : White Dorper ewes], ce qui s'accorde avec « wool predominating on fore quarter » [SA-std] ;
  - sur la nuque et le haut du cou noir, la laine d'hiver est **roussie, brun rouille** `#6B4A30` à `#8A6A48` : la pointe des mèches noires brûle au soleil [P : Point Buchon Trail sheep ; Detalhes da cabeça, où le cou montre des reflets brun-rouge].
- **Après la mue** (état choisi), le corps est couvert d'un poil fin et serré de **0,5 à 1,5 cm**, qui laisse lire chaque muscle, comme un velours ras [P : Dorper ram in trailer ; Kalahari, tondu].

### 3.3 À dessiner (fin d'été)

- **Contour lisse**, sans frange : le dessin suit les muscles (épaule, avant-bras, gigot), comme un cheval à poil court.
- **Texture** : un grain fin, des stries verticales discrètes sur l'épaule et la cuisse là où la peau bouge [P : Ovce a jehne dorper, poil légèrement ondulé ; Kalahari].
- **Ce qui reste ras partout** : la tête entière (« 100% hair forward of poll ») et les pattes (« 100% hair below the knees and hocks ») [ADSBS-market] ; les oreilles sans laine (« Wool on the ears » est éliminatoire chez l'agneau de boucherie) [ADSBS-market].
- **Variante « avant la mue »** (printemps) : ajouter sur le dos et la croupe une couche de laine de 3 à 5 cm, découpée en plaques irrégulières, avec quelques lambeaux qui pendent sur le flanc ; garder ventre, pattes et tête ras ; brunir les pointes sur le cou noir.

---

## 4. Les couleurs, zone par zone

### 4.1 Mesures sur photos

| Photo (lumière) | Zone | Valeur mesurée |
|---|---|---|
| Dorper ram in the Kalahari (plein soleil d'hiver austral, sable rouge) | flanc au soleil / dos / épaule / cuisse | `#E2CBA1` / `#DFC595` / `#E0CBA5` / `#E4D2AF` |
| | bas du flanc / dessous du ventre | `#B69270` / `#925726` (reflet orangé du sable : **à ne pas reprendre**) |
| | creux des plis du cou | `#D2B88A` |
| | cou noir au soleil / à l'ombre | `#302722`, `#2A211C` / `#2F241C` |
| | face, front, joue, chanfrein | `#362C28`, `#27201D`, `#27201D`, `#211B18` |
| | museau, lèvre, narine | `#281B17`, `#231915`, `#1E1610` |
| | scur, lumière / ombre | `#7F6644` / `#574330` |
| | patte antérieure au soleil / à l'ombre | `#D4B78E` / `#AE956C` |
| | intérieur du jarret, au soleil | `#FAF1DF` |
| Open Show Res Ch. Dorper Ram (salle, éclairage neutre) | corps tondu | `#E9E6DE`, `#F0EBE7` ; ombre du ventre `#CBC8C1` |
| | tête, cou noir | `#191A1C`, `#120F10` |
| | patte | `#EADFCD` |
| Jr. Show Champion Dorper Ram 2018 (salle) | corps / tête / cou / plis du front | `#F8ECE0` / `#282827` / `#171613` / `#24211E` |
| Ovce a jehne dorper (ciel couvert, brebis de face) | corps éclairé / poitrail / flanc à l'ombre | `#E3D5C7` / `#D6BEAB` / `#AFAEB4` |
| | face, front, cou noir | `#050A16`, `#111824`, `#080812` (noir bleuté sous le ciel couvert) |
| | museau | `#212129` |
| | onglon (poussiéreux) | `#6D6967` |
| Dorper ram in trailer (plein soleil, ombres de la bétaillère) | corps au soleil | `#F0E3C9`, `#F3E6CD` |
| | cou noir, oreille | `#111216`, `#12171D` |
| | onglons | `#08090A`, `#131417` |
| | poil roussi sur le canon (salissure) | `#503D2E`, `#3F322A` |
| Dorper sheep (Belagro-2021) (salle, sous-exposée) | tête, cou | `#181317`, `#181216` |
| | museau | `#09070A` |
| | onglons | `#231B1A`, `#36271F` |
| Detalhes da cabeça da Ovelha Dorper (ombre, gros plan, Brésil) | iris éclairé / à l'ombre | `#C1875A`, `#CE9464` / `#A17150`, `#784B2D` |
| | pupille | `#17232A` |
| | bord de la paupière inférieure | `#6E727D` |
| | front brun-noir / reflets gris du poil | `#2D1E19` / `#4A4A50`, `#564F51` |
| | poils roux autour de l'œil | `#5A3E2C`, `#523835` |
| | truffe / lèvre inférieure | `#202020` / `#2E2D32` |
| | blanc du cou (lumière / dessus de la nuque) | `#C4B7A9` / `#F9F8F5` |
| Point Buchon Trail sheep (couvert, Californie, avant la mue) | laine du dos / du flanc | `#D2EDEC` / `#9FAD98` (dominante verte de l'herbe) |
| Jr. Show Reserve Champion Dorper Ram 2018 (White Dorper) | corps / tête / museau | `#F9F4F1` / `#FCF7F0` / `#F2E3DC` |
| | onglons (non pigmentés) | `#E4C69F`, `#EACFA9` |

### 4.2 Palette pour le dessin

Valeurs ramenées à une lumière du jour neutre : retrait de l'orangé du sable du Kalahari et du vert de l'herbe.

| Zone | Base | Lumière | Ombre / notes |
|---|---|---|---|
| Corps (poil d'été) | `#EDE3D2` (blanc chaud) | `#F7F0E4` | `#C9C2B6` ombre douce ; `#A9A196` sous le ventre et entre les cuisses ; plis du cou `#D2B88A` |
| Laine d'hiver (variante avant la mue) | `#D8D2C4` | `#E8E3D8` | `#A8A190` creux entre les plaques ; pointes salies `#BFB39C` |
| Tête et cou noirs (poil mat) | `#221C19` (noir brun) | `#3A302B` sur le dessus du crâne et le chanfrein ; reflet gris `#4A4A50` sous le ciel | `#100D0D` sous la ganache et dans le pli de la gorge |
| Noir roussi (pointes au soleil, facultatif) | `#4A3528` | `#6B4A30` | |
| Truffe, lèvres | `#1B1716` | `#3C3936` (humide) | narines `#0A0808` |
| Iris | `#B97F55` | `#CE9464` | `#A17150` ; pupille `#17232A` ; reflet `#EDEBE6` |
| Paupières | `#6E727D` (gris bleuté, bord nu) | | cils noirs |
| Intérieur de l'oreille | `#2E2626` (peau sombre, poil noir clairsemé) | contre-jour `#5A3A33` | `#151111` |
| Scur | `#7F6644` | `#9A825E` | `#574330` ; base cachée dans le poil noir |
| Pattes | `#E8DECD` | `#F4EDE2` | `#ACA394` ; salissure facultative au paturon `#7A6A58` |
| Onglons | `#1C1817` (noir) | usure poussiéreuse `#4E4642` | `#0A0909` ; boue `#6D6967` |
| Peau visible (aine, scrotum, mamelle) | `#5E4A42` (pigmentée, voulue) | | une peau rose `#D8B8A8` tachetée de sombre est admise aux États-Unis [ADSBS-std] |

### 4.3 La limite du noir

**Ce que disent les textes :**
- « A white sheep with black confined to the head and neck is the ideal » [SA-std] ; même formule en Australie (« black colour confined to the neck and head area ») [AU-std] et aux États-Unis (« a black head and neck ») [ADSBS-std].
- Une description ancienne, hongroise, dit plus précisément : « the head and the **upper part of the neck** is black, the body and the legs are white » (la tête et le haut du cou) [Kovács 2008].
- Pour le Persan à tête noire, son ancêtre, la FAO précise : « a black head and neck, with the black not extending farther back, and **running evenly round the neck** » (le noir ne va pas plus loin et fait le tour du cou de façon égale) [FAO-Wilson].
- Pour les agneaux de boucherie américains (croisés compris), la tolérance est plus large : « black limited to head, neck, and forequarters not extending below the knee to the dewclaw or behind the heart girth » (le noir peut gagner l'avant, mais pas sous le genou ni en arrière du passage des sangles) [ADSBS-market].

**Ce que montrent les photos** [P] :
- **Une ligne nette**, sans dégradé : le passage du noir au blanc se fait sur 1 à 2 cm, comme une encolure trempée dans l'encre. Chez les animaux non tondus, le bord est seulement un peu effiloché par les mèches [P : Point Buchon Trail sheep].
- **Sur le dessus du cou**, le noir s'arrête **aux 0,6 à 0,7 de la longueur de la crête** en partant de la nuque : 0,59 sur le bélier du Kalahari, 0,70 sur la brebis championne de 2018. Le garrot et le dernier tiers de l'encolure sont blancs.
- **Sur le dessous**, le noir couvre toute la gorge et descend jusqu'à la **base du cou**, devant la pointe de l'épaule. Chez la plupart des animaux il s'arrête **au-dessus du poitrail** [P : Kalahari ; Grand Champion Dorper Ewe] ; chez certains il couvre aussi le haut du poitrail en bavette [P : Dorper ram in trailer ; Domba dorper fullblood jantan ; Ovce a jehne dorper].
- **L'angle de la ligne** varie d'un animal à l'autre. Sur le sujet, elle est **presque verticale**, inclinée d'environ 15° (le bas plus en avant que le haut), et passe à un travers de main devant l'épaule [P : Kalahari, 74° sur l'horizontale]. Sur les brebis championnes de 2017 et 2018, elle est inclinée à 45-60°, toujours le bas en avant ; sur la brebis brésilienne, le bas est au contraire plus en arrière [P : Ovelha Dorper]. **Pour le dessin : une ligne presque verticale, du dessus du cou aux 0,6 de la crête jusqu'au bas du cou, juste devant la pointe de l'épaule.**
- **Côté gauche et côté droit** se rejoignent en collier : le noir fait le tour du cou, comme chez le Persan.

### 4.4 Taches admises, défauts, et le classement « type 1 à 5 »

- **Taches :** « Black spots, to a limited extent on the body and legs are permissible, but an entirely white sheep or a sheep predominantly black is undesirable » [SA-std]. En Nouvelle-Zélande : « it is preferable that a body spot is joined to the neck colour » (une tache doit de préférence toucher le noir du cou) [NZ-std]. Chez l'agneau de boucherie américain : au plus « 4 square inches (size of softball) » de noir en arrière des sangles (environ 26 cm²) ; des petites taches noires « above the hoof line and below the dewclaw » (entre le sabot et l'ergot) sont acceptées [ADSBS-market]. **Sur la brebis championne de 2018, des taches noires couvrent le paturon des quatre pattes** [P].
- **Défauts de couleur :** « Brown hair around the eyes, white teats, white under the tail and white hoofs are undesirable » [SA-std] ; « excessive brown or white hair around eyes or on the face », « blue in one or both eyes » [ADSBS-std] ; « Solid brown, red or rust colored head » est éliminatoire chez l'agneau de boucherie [ADSBS-market]. Une bande blanche (« blaze ») ne doit pas « continue in front of the horn base on the head » [NZ-std] : sur certaines brebis, le blanc du cou remonte en effet jusque sur la nuque [P : Detalhes da cabeça ; brebis à l'arrière-plan de Dorper white ram].
- **Pigmentation voulue :** onglons noirs, peau pigmentée sous la queue, sur l'anus, les organes génitaux et les trayons [SA-std][ADSBS-std].
- **Le classement** de la société sud-africaine [SA-std] note chaque animal de **1 à 5** (« Very good = 5 », « Above average = 4 », « Average = 3 », « Poor or below average = 2 », « Very poor with cull points = 1 ») sur cinq caractères, chacun désigné par une lettre : **B** la conformation, **H** la couverture, **G** la taille, **D** la répartition du gras, **P** la couleur. Le **type T** en découle : « Emphasis is placed on conformation. Size and fat distribution when determining type, while colour and covering are of secondary importance ». D'après un éleveur australien qui l'explique à ses acheteurs [Amarula] : **T5** et **T4** sont des animaux de souche (« stud ») ; **T3** (ou « first select », S1) un bon animal commercial ; **T2** (S2) un animal inférieur ; **T1** un animal à réformer. Un animal à la conformation de souche mais qui a trop ou trop peu de noir est noté par exemple « C4P2T3 » et tombe en type 3. Le standard américain précise : un peu de couleur autour des yeux vaut au mieux P3, davantage P2, beaucoup P1 [ADSBS-std].
- **Pour un « type 5 » : aucune tache sur le corps**, tête et cou noirs, limite nette, pattes blanches, onglons noirs.

### 4.5 Histoire de la couleur

« The early Dorpers were white with black spots/patches all over the body. During 1957, this aspect received a lot of attention and discussion. The outflow of this was the even colour distribution of today's Dorpers » (les premiers Dorper étaient blancs tachés de noir partout ; en 1957, on a décidé de sélectionner la répartition régulière d'aujourd'hui) [SA-hist]. Un Dorper pie ou taché fait donc « ancien » ou croisé.

---

## 5. La tête

### 5.1 Ce que disent les standards

- **Afrique du Sud** [SA-std] : « Strong and long, with large eyes, widely spaced and protectively placed. Strong nose, strong well-shaped mouth with well-fitted deep jaws. **The forehead must not be dished.** The size of the ears must be in relation to the head. [...] The head must be covered with short, dullish black hair in the Dorper and dull, white hair in the White Dorper. The head must be dry i.e. without indications of fat localisation. »
- **États-Unis** [ADSBS-std] : les mêmes termes ; le bélier porte « wrinkled skin, on a broad muzzle » ; « Discriminate against heavy horns and **a dewlap under jaw** » (pénaliser un fanon sous la mâchoire) ; les mâchoires prognathes ou brachygnathes sont éliminatoires.
- **Nouvelle-Zélande** [NZ-std] : « Strong and long with **triangle shape**. [...] **Long ears are preferable.** »
- **Agneaux de boucherie (États-Unis)** [ADSBS-market] : « Moderate size ear with no wool covering » ; « Head should be strong, bold, with a deep jaw » ; « Long, pendulous ears » et « Long, narrow muzzle » sont des défauts.

### 5.2 Le profil, tourné vers la droite

D'après « Dorper ram in the Kalahari », « Detalhes da cabeça da Ovelha Dorper », « Jr. Show Champion Dorper Ram 2018 », « Ovelha Dorper » [P] :

- **Forme générale :** un **triangle long** : large et haut au niveau des yeux et de la ganache, qui s'amincit vers un museau pourtant large et carré. Longueur (nuque au bout du nez) environ **1,4 fois** la hauteur au niveau des yeux [P].
- **Ligne du dessus :**
  - de la nuque au front, presque droite, interrompue par le **scur** à la nuque ;
  - le front **jamais creux** (« must not be dished ») ;
  - puis un **chanfrein légèrement busqué**, en arc doux du front au bout du nez ; la courbe est plus marquée chez le bélier [P : Kalahari ; Jr. Show Champion Dorper Ram 2018]. C'est un héritage du Persan, dont la FAO décrit le « convex appearance to profile » dû à des coussinets de gras sur la nuque et le nez [FAO-Wilson] ; le Dorper doit, lui, avoir une tête « dry », sans gras [SA-std].
- **Peau du bélier :** le front, le dessus des yeux et le chanfrein portent des **plis transversaux** profonds [P : Jr. Show Champion Dorper Ram 2018 ; Domba dorper fullblood jantan] ; sur le dessin, deux ou trois rides plus sombres (`#100D0D`) sur fond `#221C19`, sans reflet.
- **Museau :** large, haut, au bout arrondi presque vertical. Truffe noire, narines en virgule obliques ; lèvre supérieure fendue ; **lèvre inférieure épaisse**, mâchoire profonde (« deep jaws »). Pas de fanon sous la mâchoire.
- **Joues et ganache :** fortes, rondes ; le bord inférieur de la mâchoire est épais et forme un angle net avec la gorge.
- **Œil :**
  - **grand**, « widely spaced and protectively placed » (bien écartés et protégés sous une arcade marquée) [SA-std] ;
  - placé haut, à environ **35 à 40 % de la longueur de la tête** en partant de la nuque [P : Detalhes da cabeça] ;
  - forme d'amande, paupière supérieure lourde ;
  - **iris ambre orangé** `#B97F55` bien visible dans la face noire : c'est le seul point clair de la tête [P : Detalhes da cabeça] ;
  - **pupille horizontale**, large, rectangle aux coins arrondis, à garder horizontale quand la tête descend ;
  - **bord des paupières nu, gris bleuté** `#6E727D` [P].
- **Oreilles :**
  - *Taille :* « in relation to the head » [SA-std] ; « long ears are preferable » [NZ-std] contre « moderate size » [ADSBS-market] ; mesurées à **13 cm de long, 6,8 cm de large** chez des jeunes de 12-13 mois [Nordin 2025]. Adulte : 13 à 15 cm, soit **0,55 à 0,6 fois la longueur de la tête** [S][E].
  - *Forme :* une **feuille large, au bout arrondi**, plus large que celle du Texel ou du Suffolk ; le bord épais ; poil noir ras dehors, peau sombre dedans [P : Ovce a jehne dorper ; Dorper ram in trailer ; Detalhes da cabeça].
  - *Port :* **à l'horizontale ou un peu tombantes** (10 à 30° sous l'horizontale), pointées vers l'extérieur et un peu en arrière [P : Dorper ram in trailer, Ovce a jehne dorper, Borrego dorper : horizontales ; Detalhes da cabeça : tombante, presque pendante chez cette brebis]. Jamais dressées.
  - *De profil, tête tournée vers la droite :* l'oreille gauche (proche) part derrière l'œil, sous le scur, et se dirige vers l'arrière et l'extérieur, donc raccourcie ; l'oreille droite (lointaine) dépasse à peine derrière la nuque [P : Kalahari, oreille couchée en arrière].
- **Toupet :** aucun ; la tête est entièrement à poil ras (« 100% hair forward of poll ») [ADSBS-market].

---

## 6. Les cornes

### 6.1 Ce que disent les sources

- **Afrique du Sud :** « **A developed horn base or small horns are the ideal.** Heavy horns are undesirable but permissible » (une base de corne développée ou de petites cornes sont l'idéal ; des cornes lourdes ne sont pas souhaitables mais permises) [SA-std]. La FAO le résume : « Small horns is the ideal but heavy horns are permissible » [FAO-Wilson].
- **États-Unis :** « Discriminate against heavy horns [...]. Scurs are acceptable » [ADSBS-std] ; « Most Dorpers are without horns but many rams have scurs, and some will have horns, but this should not disqualify a Dorper ram from being stud quality. Experience has shown that horned rams tend to be more masculine and have a stronger libido » [ADSBS-FAQ].
- **Agneaux de boucherie :** « Polled or small horns or scurs » acceptés, « Heavy horns » pénalisées [ADSBS-market].
- **Mesuré :** 14 à 21 % des béliers adultes d'un troupeau mexicain portent des cornes [Moreno 2026].
- **Origines :** le Dorset Horn est cornu dans les deux sexes ; le Persan à tête noire est sans cornes (« Horns should be absent (although scurs do occur) ») [FAO-Wilson]. Le Dorper hérite d'un mélange.

### 6.2 À dessiner

- **Le bélier choisi porte un scur** sur la nuque, comme celui de la photo de référence [P : Kalahari] :
  - un **moignon conique, court, de 3 à 4 cm** de haut pour 3 cm de base, posé sur le haut du crâne, un peu en arrière et au-dessus de l'œil ;
  - **pointe dirigée vers le haut et l'arrière**, légèrement recourbée ;
  - surface lisse, sans annelures visibles à cette taille, un peu translucide ;
  - couleur **brun-gris corne** `#7F6644`, plus sombre à la base `#574330`, où il sort du poil noir.
- **Brebis :** le plus souvent sans cornes ni scurs ; un crâne arrondi entre les oreilles [P : toutes les brebis regardées].
- **Ne pas dessiner** de vraies cornes enroulées : même si des « heavy horns » sont tolérées, elles feraient lire un Dorset Horn ou un croisé.

---

## 7. Les pattes et les onglons

- **Ossature et aplombs :** « Forelegs must be strong, straight and well-placed with strong pasterns and hoofs not too widely split » ; « The hind legs must be strong and well-placed, with sturdy feet and strong pasterns » [SA-std]. « The hooves should be square and not long. Animals with rolled back feet, splade front feet or over developed hooves should be culled » [AU-std].
- **Courtes :** les pattes ne font que 40 % de la hauteur [ADSBS-std]. Les canons sont **épais et courts**, l'avant-bras très musclé ; le gigot descend jusqu'au jarret [ADSBS-std][P].
- **Poil :** ras, blanc, sans laine sous le genou et le jarret (« 100% hair below the knees and hocks ») [ADSBS-market]. Pas de noir sous le genou : « Speckling or spots from knee to dewclaw or from hock to dewclaw » est éliminatoire chez l'agneau de boucherie [ADSBS-market].
- **Onglons :** **noirs** (« white hoofs are undesirable ») [SA-std][NZ-std] ; « complete pigmentation on [...] hooves » [ADSBS-std]. Sur les photos, noirs ou gris ardoise, souvent poussiéreux au bout [P]. Une petite tache noire au-dessus de la couronne, sur le paturon, est fréquente et admise [P : Grand Champion Dorper Ewe][ADSBS-market]. Longueur environ 6 cm pour un bélier [E] ; deux doigts serrés, carrés au bout ; ergots discrets.
- **Salissure :** au pré, le poil des canons se tache de brun-roux (`#503D2E`) et de vert [P : Dorper ram in trailer] : facultatif.
- **La marche :** l'Australie veut un animal qui « move easily and feely » (sic) et « the ability to walk long distances » [AU-std].

---

## 8. Tempérament et comportement (pour l'animation)

### 8.1 Ce qui est documenté pour la race

- **Calme :** « A calm temperament with a vigorous appearance is the ideal » [SA-std] ; « The Dorper is a smart, intelligent sheep that is easy to work with. They are generally quiet to handle, free moving and not aggressive as some breeds. They also have their own unique personalities » [NZ-std] ; « tame and docile, easy to handle » [Kovács 2008]. En Australie, « Animals that show signs of aggressive behaviour in the yards are not preferred » [AU-std].
- **Brouteur peu sélectif, qui broute aussi les buissons :**
  - « Dorpers utilised shrubs and bushes to a greater extent, but grass to a lesser extent in relation to Merino sheep » ; ils consomment plus d'espèces de plantes [Brand 2000] ;
  - « They graze in a similar manner to a goat and have the ability to graze at a higher level than traditional sheep » (ils broutent un peu comme une chèvre, plus haut que les moutons ordinaires) [NZ-std].
  - **Pour l'animation :** le Dorper alterne la tête basse dans l'herbe et la **tête tendue à l'horizontale vers un buisson** (une feuille, un bord de fenêtre).
- **Il marche peu pour choisir :** « Dorpers walked less to select food, or a suitable spot to graze, which consequently led to a shorter grazing time and less separate grazing periods » [Brand 2000][SA-char]. Pour l'animation : **des sessions de broutage moins nombreuses et plus longues**, avec peu de déplacements entre deux bouchées ; un animal qui se pose et mange.
- **Résistant à la chaleur :** « Because of their Blackhead Persian origin, Dorpers have natural tolerance to high temperatures and heavy insect populations » [ADSBS-about]. Créé pour le Karoo aride, il vit aussi au froid et sous la neige [SA-char].
- **Mères attentives :** « Excellent "fussing" mothers » [NZ-std] ; « When the ewes are left alone they will lamb easily, and will retain and look after their lambs » [SA-char]. Les agneaux sont « vigorous and have high survivability » [ADSBS-about].
- **Pas de saison :** les brebis peuvent agneler trois fois en deux ans [SA-char][ADSBS-FAQ] ; « ram lambs can begin to work at only 90 days of age » [NZ-std]. Pour Mouflon : des agneaux à toute saison.
- **Rang social et combats :** dans un troupeau mexicain, les béliers dominants gagnent 69 % des affrontements, faits de « bumps, threats, shoves, chases, escapes, and evasions » (coups de tête, menaces, poussées, poursuites, fuites, évitements) ; les brebis se disputent la mangeoire de la même façon [Moreno 2026]. Au contact des brebis, un bélier dominant **bêle beaucoup plus** (15 à 26 vocalisations relevées en deux jours, contre 2 à 4 pour un dominé), renifle, fait le flehmen et donne des coups de patte avant [Moreno 2026].
- **Ce qui manque :** la revue de Brand note « a total lack of scientific information on other behaviour aspects of this unique breed » [Brand 2000]. Je n'ai trouvé aucune étude sur la vigilance, la fuite ou les jeux propres au Dorper.

### 8.2 Ce qui vaut pour les moutons en général (non propre à la race)

- **Pas :** à quatre temps. Le Dorper, lourd et court sur pattes, a un **pas court, posé, régulier** ; le tonneau bouge peu, la tête hoche à chaque foulée [E].
- **Trot et galop :** courts et rares chez un bélier lourd ; il trotte pour rejoindre le groupe [E].
- **Mouvement secondaire :** le poil court ne flotte pas ; seuls les gigots, le ventre et le scrotum tressautent un peu ; les oreilles, horizontales, bougent séparément [E]. En variante « avant la mue », les plaques de laine qui pendent peuvent se balancer.
- **Poses de repos :** debout, carré, tête au niveau du dos ; couché sur le sternum, antérieurs repliés ou **un antérieur allongé devant**, tête haute, ruminant [P : Dorper sheep (Belagro-2021), bélier couché, un antérieur tendu, un postérieur allongé sur le côté] ; à l'ombre par forte chaleur [E].
- **Combats de béliers :** ils reculent de quelques pas puis se heurtent front contre front ; un bélier lourd charge court [E, général chez les béliers sans grandes cornes]. Aucune source ne décrit un Dorper qui se cabre.
- **Vigilance :** tête levée, oreilles tournées vers le bruit, regard fixe ; tout le groupe regarde dans la même direction [P : Dorper Sheep Breed Antigua, trois animaux tête haute vers l'objectif].
- **Voix :** aucune source ne décrit un bêlement propre au Dorper.

---

## 9. Les dix détails qui font un Dorper, et les erreurs à éviter

### 9.1 Les dix détails

1. **Tête et cou noirs, corps blanc** : le noir couvre la tête entière et les deux tiers avant du cou ; rien de noir en arrière.
2. **Une limite nette**, presque verticale, devant l'épaule ; le noir fait le tour du cou en collier ; la gorge noire jusqu'à la base du cou.
3. **Poil noir mat**, sans laine sur la tête ; **un seul point clair, l'œil ambre**.
4. **Pas de laine longue** : en fin d'été, un poil blanc ras et lisse qui montre les muscles ; au printemps, des plaques de laine qui partent, le ventre, les pattes et la tête restant ras. **Jamais de crinière.**
5. **Un tonneau sur pattes courtes** : poitrine profonde (0,56 H), pattes 0,44 H, corps 1,4 H de long ; dos droit, croupe longue et large, gigots jusqu'au jarret.
6. **Tête longue et forte, en triangle**, chanfrein légèrement busqué, museau large et plissé chez le bélier, front jamais creux.
7. **Oreilles moyennes à grandes, larges, noires**, à l'horizontale ou un peu tombantes, sans laine.
8. **Un petit scur** de 3 à 4 cm sur la nuque du bélier, ou rien ; pas de grandes cornes.
9. **Pattes blanches à poil ras, onglons noirs**, souvent une petite tache noire au paturon.
10. **Queue coupée** en court moignon (ou, entière, mince et à poil) ; ni queue grasse ni croupe grasse.

### 9.2 Les erreurs qui en font une autre race

- **Suffolk :** tête **et pattes** noires, « jet black, wool-free heads and legs that sharply contrast their clean white fleeces and pink skin » [Suffolk-US] ; une **vraie toison** de laine blanche qu'on tond chaque année, qui monte jusqu'à la nuque et recouvre le cou : chez le Suffolk le noir s'arrête **derrière les oreilles**, chez le Dorper il descend sur le cou. Des oreilles **« long, bell-shaped » qui pendent sous l'horizontale** et dont la pointe se tourne un peu vers l'avant ; la tête est « free of wool both at the poll and cheeks » [Suffolk-US]. Un Dorper aux pattes noires, ou au cou blanc et laineux jusqu'aux oreilles, devient un Suffolk.
- **Persan à tête noire (Blackhead Persian), son ancêtre** [FAO-Wilson] : même robe (tête et cou noirs, le noir « running evenly round the neck », onglons noirs, corps blanc à poil de jarre jusqu'à 4 cm), même absence de cornes (« scurs do occur »). Mais :
  - une **croupe grasse** et une **queue en trois parties** : une large masse ferme collée à la croupe, une deuxième partie relevée contre la première, terminée par une zone de peau noire nue, puis une petite queue pendante de 5 à 8 cm ;
  - un **fanon** qui pend librement sous le cou, un poitrail saillant ;
  - un **profil convexe** dû à des coussinets de gras sur la nuque et le nez ;
  - un animal **beaucoup plus petit** (bélier « up to 70 kg », brebis 50 kg ; 30 kg pour les brebis de départ selon [Kovács 2008]) et moins musclé.
  - Un Dorper avec une grosse croupe ronde au-dessus d'une queue courte et large, ou un fanon, devient un Persan.
- **Awassi** [WP-Awassi] : brebis laitière du Proche-Orient à **queue grasse** ; toison de laine blanche, mais **tête et pattes brunes** (parfois blanches, grises, noires ou tachées) ; **profil convexe et oreilles pendantes** ; béliers normalement cornus. Un Dorper à tête brune, aux oreilles pendantes et à la toison est un Awassi ; un Dorper à « solid brown, red or rust colored head » est d'ailleurs éliminé chez l'agneau [ADSBS-market].
- **Dorset Horn, l'autre parent :** tout blanc, laineux, **cornu dans les deux sexes** ; un Dorper à grandes cornes en spirale et à toison épaisse tire vers lui.
- **White Dorper :** ne pas mélanger les deux : un Dorper « entirely white » est indésirable [SA-std], et un White Dorper ne porte du noir qu'en petites taches aux oreilles et au dessous [SA-std].
- **Croisés et vieux type :** un corps taché de noir partout (« the early Dorpers were white with black spots/patches all over the body ») [SA-hist] ; un animal « predominantly black » [SA-std] ; un noir qui déborde sur l'épaule ou l'avant-bras ; une tête brune [P : Dorper Sheep Breed Antigua, trois croisés, dont un brun et un noir à ventre blanc].
- **À ne pas dessiner :**
  - de la laine sur la tête ou les oreilles ;
  - une crinière de poil sur l'encolure ;
  - un noir qui dégrade en gris ou qui passe derrière l'épaule ;
  - des pattes noires ou tachées de noir sur le canon ;
  - des onglons blancs ;
  - un cou mince et long, des pattes hautes (« tall, leggy, shallow, narrow bodied » [ADSBS-market]) ;
  - un poil roux ou brun autour des yeux (défaut) ;
  - un fanon sous la mâchoire ;
  - une queue grasse ou une croupe grasse ;
  - de grandes cornes.

---

## 10. Sources, photos, contradictions

### 10.1 Sources (ouvertes ou lues pour cette fiche)

**Sociétés de race :**
- [SA-std] Dorper Sheep Breeders' Society of South Africa, *Breed Standards* (standard d'excellence, système de notes 1 à 5, lettres B, H, G, D, P, T, couleur, poids moyens par âge) : https://dorpersa.co.za/breed-standards/
- [SA-hist] Idem, *Breed History* (d'après J. A. Nel, *History of the Dorper*, 1993) : https://dorpersa.co.za/breed-history/
- [SA-char] Idem, *Breed Characteristics* (adaptation, utilisation du veld, mères, peaux, pas de tonte) : https://dorpersa.co.za/breed-characteristics/
- [ADSBS-std] American Dorper Sheep Breeders' Society, *Breed Standards* (rapport 60:40, tête, cornes, « cut-off line » de couleur, couverture, pointage ; d'après *Dorpers Into The New Century* de Dolf Lategan) : https://dorpersheep.org/breed-standards/
- [ADSBS-about] Idem, *About Dorpers* : https://dorpersheep.org/about-dorpers/
- [ADSBS-FAQ] Idem, *FAQs* (scurs et cornes, poids des béliers et des brebis, mue, présentation tondue) : https://dorpersheep.org/faqs/
- [ADSBS-market] Idem, *Market Lamb Classification Standards* (adoptés le 15 août 2019) : https://dorpersheep.org/wp-content/uploads/2022/02/ADSBS-Market-Lamb-Classification-Standards.pdf
- [AU-std] *Breed Standard for the Australian Dorper and White Dorper Sheep* : https://www.australiandorper.com.au/wp-content/uploads/2020/10/breedstandard.pdf
- [AU-hist] T. Esson, *A Brief History of the Dorper and White Dorper Sheep* et *An Introduction to the Dorper and White Dorper* : https://www.australiandorper.com.au/wp-content/uploads/2020/10/A2_HistoryDorperDevelopment.pdf et https://www.australiandorper.com.au/wp-content/uploads/2020/10/intro_dorper_whitedorper.pdf
- [Amarula] Amarula Dorper Stud, *Dorper Classification System (Typing)*, explication des types T1 à T5 pour un catalogue de vente (source d'éleveur, secondaire) : https://media.auctionsplus.com.au/Uploads/February/Amarula%20Classification%20Overview.pdf
- [NZ-std] New Zealand Sheepbreeders Association, *Dorper* (standard, couverture, comportement, règles d'exposition) : https://nzsheep.co.nz/dorper/

**FAO et bases de données :**
- [FAO-Wilson] R. T. Wilson 1991, *Small ruminant production and the small ruminant genetic resource in tropical Africa*, FAO Animal Production and Health Paper 88 (sommaire : https://www.fao.org/4/t0376e/t0376e00.htm), fiches « Blackhead Persian » et « Dorper » (« Fat-rumped sheep ») : https://fao.org/docrep/pdf/009/t0376e/t0376e16.pdf
- [DAGRIS] ILRI, *Domestic Animal Genetic Resources Information System*, Dorper, hauteur au garrot, tour de poitrine, couleur, cornes : https://dagris.info/breed/2504/trait_type/2192 (et les autres caractères de la même race)

**Études :**
- [Moreno 2026] Moreno-Avalos S. et al. 2026, « Importance of Social Hierarchy in Morphometry, and Socio-Sexual and Reproductive Behaviors in Dorper Sheep in Northern Mexico », *Animals* 16(6) : 994, tableaux 1 à 3 lus dans le texte intégral (Europe PMC) : https://doi.org/10.3390/ani16060994
- [Madikadike 2022] Madikadike M. K., Mbazima, Gunya, Tyasi 2022, « Growth Traits as Indicators of Body Weights in Dorper Sheep », *American Journal of Animal and Veterinary Sciences* 17(4) : 332-338 : https://thescipub.com/abstract/ajavsp.2022.332.338 (DOI 10.3844/ajavsp.2022.332.338)
- [Nordin 2025] Ahmad Nordin N. S. et al. 2025, « Morphometric Characterization, Scrotal and Udder Measurements of Dorper Sheep Reared in Setiu, Terengganu », *Malaysian Applied Biology* 54(2) : https://doi.org/10.55230/mabjournal.v54i2.2803
- [Utomo 2026] Utomo B. et al. 2026, « Comparative Production Performance of Fullblood, F1, and F2 Dorper Rams at Barokah Farm, Kediri », *Journal of Tropical Animal Bioscience* 1(1) : 25-31 : https://journal.unesa.ac.id/index.php/jtab/article/view/53210
- [Febriana 2026] Febriana A. et al. 2026, « Evaluating morphometric traits and body indices in Dorper sheep in Indonesia », *Journal of Advanced Veterinary Research* 16(1) : 16-19 (âges et sexes mêlés, non retenu pour les rapports) : https://advetresearch.com/index.php/AVR/article/view/2430
- [Kovács 2008] Kovács A., Kukovics S., Jávor A. 2008, « Dorpers, the meat sheep of the future », *Analele Universității din Oradea, Fascicula Ecotoxicologie, Zootehnie și Tehnologii de Industrie Alimentară* 7 (2008) : https://www.protmed.uoradea.ro/facultate/anale/ecotox_zooteh_ind_alim/2008/Kovacs%20A.pdf
- [Brand 2000] Brand T. S. 2000, « Grazing behaviour and diet selection by Dorper sheep », *Small Ruminant Research* 36(2) : 147-158 (résumé, recherche PubMed) : https://doi.org/10.1016/s0921-4488(99)00158-3
- [Shedding 2024] Yuan X. et al. 2024 (université du Ningxia, Chine), « Unlocking the genetic secrets of Dorper sheep: insights into wool shedding and hair follicle development », *Frontiers in Veterinary Science* 11 : 1489379 (texte intégral, recherche PubMed) : https://doi.org/10.3389/fvets.2024.1489379
- [Fourie 2002] Fourie P. J., Neser F. W. C., Olivier J. J., van der Westhuizen C. 2002, « Relationship between production performance, visual appraisal and body measurements of young Dorper rams », *South African Journal of Animal Science* 32(4) : 256-262 : notice seulement (le texte m'a été refusé, erreur 403) : https://agris.fao.org/search/en/records/64723eec2c1d629bc978e920

**Encyclopédies et divers :**
- [WP-EN] https://en.wikipedia.org/wiki/Dorper
- [WP-Awassi] https://en.wikipedia.org/wiki/Awassi (d'après FAO DAD-IS et *Mason's World Dictionary of Livestock Breeds*)
- [Suffolk-US] United Suffolk Sheep Association (États-Unis), *Breed information* (histoire et standard) : https://suffolks.org/breed-information.html
- [Sheep101] S. Schoenian, *Docking and castrating*, Sheep 101 : https://sheep101.info/201/dockcastrate.html

**Pages qui m'ont été refusées ou n'ont pas répondu :** la fiche de l'université d'État de l'Oklahoma (403) ; la thèse de T. Fitaroni (UENF, Brésil) sur les indices zoométriques du Dorper et du White Dorper (connexion coupée) ; l'article de *Animals* sur le site de MDPI (403, lu par Europe PMC).

### 10.2 Photos regardées

Toutes sur Wikimedia Commons, catégorie https://commons.wikimedia.org/wiki/Category:Dorper_Sheep , téléchargées hors du dépôt.

**Profils et sujet de référence :**
- **Dorper ram in the Kalahari - South Africa** (référence) : bélier tondu, profil strict vers la droite, dunes du Kalahari, 27 juillet 2012 (Attiewestraad, CC BY-SA 4.0). Proportions, encolure, plis du cou, limite du noir, scur, couleurs au soleil. https://commons.wikimedia.org/wiki/File:Dorper_ram_in_the_Kalahari_-_South_Africa_.jpg
- **Open Show Res Ch. Dorper Ram (23726476118)** : bélier de concours tondu, profil vers la droite, Keystone International Livestock Exposition 2017 (Harrisburg). Rapports, queue coupée. https://commons.wikimedia.org/wiki/File:Open_Show_Res_Ch._Dorper_Ram_(23726476118).jpg
- **Open Show Champion Dorper Ram (36909052393)** : bélier champion 2017, profil vers la droite ; limite du noir oblique. https://commons.wikimedia.org/wiki/File:Open_Show_Champion_Dorper_Ram_(36909052393).jpg
- **Jr. Show Champion Dorper Ram (45227765541)** : bélier 2018, profil vers la droite ; front et museau très plissés, chanfrein busqué. https://commons.wikimedia.org/wiki/File:Jr._Show_Champion_Dorper_Ram_(45227765541).jpg
- **Jr. Show Ch. Dorper Ram (23727243938)** : bélier 2017, profil vers la droite (petite image). https://commons.wikimedia.org/wiki/File:Jr._Show_Ch._Dorper_Ram_(23727243938).jpg
- **Open Show Champion Dorper Ewe (37530232246)** : brebis championne 2017, profil vers la droite ; le noir s'arrête au-dessus du poitrail, ligne oblique le bas en avant, comme sur le sujet. https://commons.wikimedia.org/wiki/File:Open_Show_Champion_Dorper_Ewe_(37530232246).jpg
- **Grand Champion Dorper Ewe (44234017935)** : brebis tondue, profil vers la droite, 2018 ; limite du noir, taches noires aux paturons, silhouette de brebis. https://commons.wikimedia.org/wiki/File:Grand_Champion_Dorper_Ewe_(44234017935).jpg
- **Ovelha Dorper - Sítio Caramonã** : brebis au pré à Dores do Turvo (Minas Gerais, Brésil), 16 avril 2021 (Rodrigo Padula, CC BY-SA 4.0), profil vers la gauche ; limite du noir inclinée dans l'autre sens, port de tête au pas. https://commons.wikimedia.org/wiki/File:Ovelha_Dorper_-_S%C3%ADtio_Caramon%C3%A3.jpg
- **Point Buchon Trail sheep** : troupeau californien non tondu, 29 février 2008 (Teddy Llovet, CC BY 2.0) ; brebis de profil vers la droite avec laine d'hiver en plaques, cou roussi, agneau couché, animaux broutant. https://commons.wikimedia.org/wiki/File:Point_Buchon_Trail_sheep.jpg

**Têtes de près :**
- **Detalhes da cabeça da Ovelha Dorper - Sítio Caramonã** : gros plan de profil ; iris ambre, pupille horizontale, paupière gris bleuté, oreille tombante, reflets bruns du poil, blanc remontant sur la nuque. https://commons.wikimedia.org/wiki/File:Detalhes_da_cabe%C3%A7a_da_Ovelha_Dorper_-_S%C3%ADtio_Caramon%C3%A3.jpg
- **Dorper ram in trailer** : jeune bélier de trois quarts face, dans une bétaillère ; oreilles horizontales, bavette noire sur le poitrail, onglons noirs, poil d'été. https://commons.wikimedia.org/wiki/File:Dorper_ram_in_trailer.jpg
- **Domba dorper fullblood jantan** : trois béliers indonésiens (affiche de vente) ; plis du front, noir jusqu'au poitrail. https://commons.wikimedia.org/wiki/File:Domba_dorper_fullblood_jantan.jpg

**Brebis, agneaux, troupeaux :**
- **Ovce a jehne dorper** : brebis de face et son agneau couché, ciel couvert ; couleurs neutres, oreilles, tête de l'agneau. https://commons.wikimedia.org/wiki/File:Ovce_a_jehne_dorper.jpg
- **Droperschaf1** : agneau de profil vers la gauche ; queue entière, poil laineux, noir limité à la tête. https://commons.wikimedia.org/wiki/File:Droperschaf1.jpg
- **Borrego dorper** : jeune Dorper de trois quarts ; oreilles larges à l'horizontale, collier noir. https://commons.wikimedia.org/wiki/File:Borrego_dorper.png
- **Dorper sheep (Belagro-2021)** : bélier couché dans un enclos d'exposition ; pose de repos, onglons noirs, scrotum. https://commons.wikimedia.org/wiki/File:Dorper_sheep_(Belagro-2021).jpg
- **Flock with Dorper Sheep in Austria** : un Dorper parmi des moutons laineux, broutant ; silhouette ras-poil au milieu des toisons. https://commons.wikimedia.org/wiki/File:Flock_with_Dorper_Sheep_in_Austria.JPG
- **Dorper Sheep Breed Antigua 9187** : trois animaux de type Dorper, dont deux mal colorés (un brun, un noir à flancs blancs) ; vigilance, erreurs de robe. https://commons.wikimedia.org/wiki/File:Dorper_Sheep_Breed_Antigua_9187.jpg
- **Dorper sheep.jpg** : enclos de White Dorper vus de dos (queues coupées) et un Dorper à tête noire. https://commons.wikimedia.org/wiki/File:Dorper_sheep.jpg

**White Dorper :**
- **Jr. Show Reserve Champion Dorper Ram (45227765471)** : White Dorper de concours, profil vers la droite ; onglons ambrés. https://commons.wikimedia.org/wiki/File:Jr._Show_Reserve_Champion_Dorper_Ram_(45227765471).jpg
- **Dorper white ram** : White Dorper tondu de profil vers la gauche, nez rose, moignon de queue ; Dorper à l'arrière-plan avec une bande blanche sur la nuque. https://commons.wikimedia.org/wiki/File:Dorper_white_ram.jpg
- **White Dorper ewes** : six brebis White Dorper sur sol rouge, de profil ; **restes de laine en plaque sur l'épaule et le garrot** d'une brebis (la laine « predominating on fore quarter »), oreilles horizontales. https://commons.wikimedia.org/wiki/File:White_Dorper_ewes.jpg
- **Winning Australian white dorper ram** : bélier White Dorper champion, profil vers la droite, au licol ; poil d'été court et dense, tête blanche, scrotum très bas. https://commons.wikimedia.org/wiki/File:Winning_Australian_white_dorper_ram.png
- **Carneiro White Dorper - Sítio Caramonã** : jeune White Dorper au nez et aux oreilles roses (défaut de pigmentation). https://commons.wikimedia.org/wiki/File:Carneiro_White_Dorper_-_S%C3%ADtio_Caramon%C3%A3.jpg

**Regardée mais écartée :**
- **Dorper Sheep.jpg** (avec majuscule) : un mouton laineux brun à tête noire, dans un zoo tropical ; ni la robe ni la couverture d'un Dorper. https://commons.wikimedia.org/wiki/File:Dorper_Sheep.jpg
- « Australian mob of White dorper's », « Junior Show Reserve Champion Dorper Ewe » et « Jr. Show Res. Ch. Dorper Ram-W » n'ont pas pu être téléchargées en taille utile (Commons refusait les requêtes, erreur 429) ; je ne les cite pas comme regardées.

### 10.3 Ce sur quoi les sources se contredisent

| Sujet | Désaccord |
|---|---|
| **Crinière** | La consigne de cette fiche évoquait une « crinière de poil » ; or « Manes are a disqualifier » [SA-std][FAO-Wilson] et « a coarse hairy mane » est un défaut [ADSBS-std]. **Choix :** pas de crinière. |
| **Laine : où ?** | « wool predominating on fore quarter » (la laine domine sur l'avant) [SA-std] ; mais la laine reste « on their sides, hindquarters, and neck » [ADSBS-std], et la photo d'avant-mue montre surtout de la laine sur le dos et la croupe [P : Point Buchon], alors qu'une autre montre un reste de laine sur l'épaule [P : White Dorper ewes]. Sous le cou, ventre et culotte : « always hair only » [NZ-std]. |
| **Faut-il muer ?** | Oui, chaque été, c'est un caractère à sélectionner [ADSBS-FAQ] ; « A fully shedding animal is not necessary » [AU-std] ; un tiers seulement des animaux muent proprement selon des éleveurs américains (source non retenue) ; une partie d'un troupeau chinois n'avait pas mué en mai [Shedding 2024]. |
| **Cornes** | « A developed horn base or small horns are the ideal » [SA-std] contre « Most Dorpers are without horns » [ADSBS-FAQ] et des cornes lourdes à « discriminate against » [ADSBS-std]. 14 à 21 % des béliers mexicains en portent [Moreno 2026]. **Choix :** un scur de 3 à 4 cm. |
| **Poitrail** | « A moderate protrusion of the brisket beyond the shoulders [...] are the ideal » [SA-std][ADSBS-std] contre « Protruding brisket undesirable » dans le résumé de la FAO, pourtant tiré des notes de jugement sud-africaines [FAO-Wilson]. **Choix :** un poitrail qui dépasse un peu devant les antérieurs. |
| **Oreilles** | « Long ears are preferable » [NZ-std] contre « Moderate size ear » et « Long, pendulous ears » pénalisées [ADSBS-market] ; « in relation to the head » [SA-std]. Sur les photos : de l'horizontale à presque pendante. |
| **Limite du noir** | « head and neck » [SA-std][AU-std][ADSBS-std] ; « the upper part of the neck » [Kovács 2008] ; jusqu'aux « forequarters » chez l'agneau de boucherie [ADSBS-market]. Sur les photos, la ligne s'incline tantôt vers l'avant, tantôt vers l'arrière, et le noir couvre parfois le haut du poitrail. |
| **Hauteur au garrot** | Aucun standard ne la donne. Béliers adultes 63 à 68 cm [Moreno 2026] ; jeunes béliers de 12 mois 73 cm [Utomo 2026] ; 61 cm à 1-2 ans [Madikadike 2022] ; « 60 - 60 cm » (coquille) [Kovács 2008]. **Choix :** 70 cm [E]. |
| **Poids** | Béliers : 105,5 kg [SA-std] ; 104 à 122 kg [ADSBS-FAQ] ; 90 à 105 kg [Kovács 2008] ; 53 à 61 kg au Mexique [Moreno 2026]. Brebis : 95 kg (dont certaines pleines) [SA-std] ; 82 à 95 [ADSBS-FAQ] ; 70 à 95 [Kovács 2008] ; 37 à 40 kg [Moreno 2026]. |
| **Hauteur du sternum** | 40,9 cm pour 60,7 cm de garrot, soit des pattes à 0,67 H [Madikadike 2022], contre 40 % de pattes [ADSBS-std] et 0,44 H mesuré sur photo [P]. La méthode de [Madikadike 2022] n'est pas décrite assez pour trancher ; je suis l'ADSBS et les photos. |
| **Croupe et garrot** | Croupe plus haute que le garrot chez les jeunes de 1-2 ans (66,9 contre 60,7 cm chez les béliers) [Madikadike 2022] ; dos « long and straight » [SA-std] et dos de niveau sur les adultes photographiés [P]. |
| **Queue** | Coupée en pratique (« re-docking » admis [ADSBS-market] ; moignons sur les photos [P]) ; mais le Dorper « generally do[es] not require tail docking » [Sheep101]. Aucun standard n'en parle. |
| **Origine des parents** | Le Persan à tête noire vient en réalité de Somalie : « In spite of its name it is certain that the sheep has its origins in the Blackheaded Somali » [FAO-Wilson] ; « Somali ewes (called mistakenly "Persian" in South-Africa) » [Kovács 2008] ; « a hardy, fat-tailed desert breed from Arabia » [ADSBS-about] ; la FAO le range parmi les moutons à **croupe** grasse, l'ADSBS parmi ceux à **queue** grasse. |
| **Dates de la race** | Croisements dès les années 1930 [SA-hist][AU-hist] ; « from 1942 onwards » [FAO-Wilson] ; nom « Dorper » adopté en 1947 par Edmeades, première sélection le 21 mai 1946 [SA-hist] ; projet « finalized in 1946 » [ADSBS-about] ; société fondée le 19 juillet 1950 [SA-hist]. |
| **Effectifs** | « over 5 million » en Afrique du Sud [SA-hist] ; « over 10 million head (over 1/3 of the total number of sheep) » [ADSBS-about] ; « exceeds 10 million » [Kovács 2008]. La société elle-même annonce 300 membres dans le texte de sa page et 600 dans la description de la même page [SA-std] ; 600 éleveurs dans son historique [SA-hist]. |
| **Mesures de la tête, des oreilles, de la queue, des articulations** | Seule une étude sur de jeunes animaux donne la tête et l'oreille [Nordin 2025]. Toutes les autres valeurs de 2.3 à 2.6 qui ne portent pas [S] sont des mesures sur photo [P] ou des estimations [E]. |
