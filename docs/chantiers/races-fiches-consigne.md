# Chantier : la consigne des sous-agents pour les fiches de races

*Copiée telle quelle de la session du 3 novembre 2026 qui a écrit les
fiches du lot A (`docs/chantiers/races-fiches.md`). Pour reprendre, donne
cette consigne à chaque sous-agent, avec une note propre à sa race. Le
chemin du dossier de travail (`/tmp/claude-0/…/scratchpad/`) était celui de
cette session : remplace-le par celui de la nouvelle.*

## Les dix fiches qui restent

- **Lot A** : `hebridean`, `manx-loaghtan`, `soay`, `shetland`,
  `wensleydale`.
- **Lot B** : `barbados-blackbelly`, `navajo-churro`, `mouflon-canadien`,
  `argali`, `mouflon-de-dall`.

---

# Consigne commune : une fiche de race pour Mouflon

Tu écris UNE fiche de race de mouton pour le projet Mouflon (compagnons de
bureau pour macOS ; des moutons dessinés de profil marchent sur les fenêtres).
Dépôt : /home/user/Mouflon.

## À lire d'abord (obligatoire)
1. /home/user/Mouflon/CLAUDE.md
2. /home/user/Mouflon/docs/chantiers/races-fiches.md (le plan en 10 points
   qu'une fiche doit suivre : c'est TA consigne principale)
3. Au moins deux fiches modèles en entier : docs/races/ouessant.md et
   docs/races/nez-noir.md (parcours aussi jacob.md et mouflon-corse.md).
   Elles sont en anglais ; la tienne sera EN FRANÇAIS, au moins aussi
   détaillée (≈ 450 à 600 lignes), avec le même soin : chiffres sourcés,
   tableaux, balises de source [S1]…, distinction entre [STD] (standard),
   [SRC] (autre source), [PHOTO] (vu sur une photo), [EST] (ton estimation).

## Ce que doit contenir la fiche (plan obligatoire, titres en français)
En tête, une ligne citée comme les modèles :
> Fiche de la race « <Nom> », préparée le 3 novembre 2026 pour dessiner
> `pets/<id>` dans Mouflon. Le dessin viendra ensuite
> (`docs/chantiers/races-dessins.md`).
Puis un « 0. En bref » (tableau : individu, vue, toison, signature, palette
principale), puis les sections 1 à 10 du plan de races-fiches.md :
1 individu et pourquoi ; 2 tailles et proportions CHIFFRÉES (garrot,
longueur, tour de poitrine, hauteur sous le ventre, tête, oreilles, queue,
genou/carpe et jarret ; un tableau en fractions de la hauteur au garrot
H = 100) ; un paragraphe ENCOLURE (longueur, port, angle au repos et en
broutant, en degrés) et un paragraphe ARTICULATIONS DES PATTES (hauteur du
carpe et du jarret, angles du jarret au repos, du paturon, longueurs
relatives avant-bras/canon, bras/cuisse) — le moteur va plier le cou et les
pattes en deux segments, ces chiffres serviront ; 3 toison ou poil ; 4
couleurs zone par zone avec valeurs hexadécimales (base, lumière, ombre)
TIRÉES DE PHOTOS, marques exactes (place, bords) ; 5 tête (profil, museau,
naseaux, yeux : iris, pupille horizontale, oreilles : taille, port, toupet) ;
6 cornes (ou absence, et ce qu'il faut ne pas dessiner) : nombre, section,
enroulement en profil (angles, tours, où va la pointe), annelures, couleur,
bélier/brebis ; 7 pattes et onglons ; 8 tempérament et comportement utiles
à l'animation (grégarité, vigilance, fuite, jeux, voix, rythme de la
journée, ce qui la distingue) ; 9 les dix détails qui la font reconnaître +
les erreurs à éviter (ce qui en ferait une AUTRE race — compare aux races
voisines nommément) ; 10 sources (liens), photos regardées (liens, une
quinzaine, avec ce que chacune t'a appris) et ce sur quoi les sources se
contredisent.

## Recherche
- Cherche sur Internet (WebSearch, WebFetch) : standard officiel de la race
  (association, organisme de sélection, IDELE, races-de-france, FAO DAD-IS,
  Rare Breeds Survival Trust, Livestock Conservancy, Oklahoma State
  University « Breeds of Livestock », Wikipédia FR/EN/DE en appui), études
  morphométriques (Google Scholar, PubMed, ResearchGate) pour les mesures.
  Ne jamais inventer un chiffre : si aucune source ne le donne, mesure-le
  sur des photos et balise-le [EST].
- Regarde de VRAIES PHOTOS : télécharge-les (curl, User-Agent
  « MouflonResearch/1.0 », aucune donnée personnelle dans les requêtes)
  dans /tmp/claude-0/-home-user-Mouflon/01280bc2-fbcb-5cf6-a3fc-da242d3b0f65/scratchpad/photos/<id>/
  (JAMAIS dans le dépôt), puis OUVRE-LES avec l'outil Read pour les voir.
  Prends des valeurs hexadécimales en échantillonnant des zones avec
  `python3 -I` et PIL (moyenne d'un petit carré, en lumière et en ombre).
  Mesure les proportions sur les photos de profil (pixels → fractions de H).
  Attention : l'API de Wikimedia Commons renvoie souvent 429 (trop de
  requêtes) depuis ce réseau ; espace tes requêtes (quelques secondes),
  réessaie, et passe aussi par les pages Wikipédia (les URL d'images
  upload.wikimedia.org s'y trouvent dans le HTML), les sites des
  associations de race, des instituts, des éleveurs.
- N'utilise ni n'écris aucune information personnelle.

## Règles
- Écris SEULEMENT le fichier /home/user/Mouflon/docs/races/<id>.md. Ne
  touche à rien d'autre dans le dépôt. Ne fais AUCUNE opération git.
- Français correct et clair, phrases courtes ; noms de race d'origine entre
  parenthèses au besoin.
- Ne décris ni ne reproduis aucun personnage protégé.
- À la fin, réponds en quelques lignes : l'individu choisi, le nombre de
  lignes de la fiche, le nombre de photos regardées, et ce qui reste
  incertain.
