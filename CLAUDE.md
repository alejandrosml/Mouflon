# Mouflon — consignes pour Claude

Mouflon est un gestionnaire de compagnons de bureau pour macOS, écrit en
Rust : des moutons (et d'autres animaux) marchent sur les fenêtres, broutent,
se rencontrent, jouent. Il lit aussi les animaux de desktopPet (eSheep).
L'auteur est Alejandro San Martin Lamas ; il écrit en français et on le
tutoie.

## Le dépôt

| Dossier | Contenu |
|---|---|
| `crates/mouflon-core` | le cœur : troupeau, physique, comportements, monde (`#![forbid(unsafe_code)]`, déterministe, sans dépendance au système) |
| `crates/mouflon-render` | le dessin : rigs SVG, squelettes peints, gestes, allures, cuisson en images, BC7 |
| `crates/mouflon-platform` | macOS (AppKit, Core Animation, Accessibilité) : **le seul crate où `unsafe` est permis**, chaque bloc justifié |
| `crates/mouflon-app` | l'app : fil de l'affichage, menus, réglages, bibliothèque des animaux |
| `pets/` | les animaux du dépôt (Pelote, Lainou, le Mannequin, Aquarelle, et quatre races réelles : `mouflon-corse`, `nez-noir`, `jacob`, `ouessant`) |
| `tools/mouflon-preview` | aperçu hors écran (poses, gestes, jeux, rencontre, vidéo…) |
| `tools/mouflon-peintre` | peint les morceaux d'Aquarelle |
| `tools/races` | les scripts Python qui écrivent les SVG des quatre races : pour retoucher une race, change son script et relance-le (voir son README), ne modifie pas le SVG à la main ; `articulations.py` coupe un dessin à ses articulations (encolure, bas des pattes, lèvre, mèches), `mannequin/articule.py` écrit le Mannequin |
| `tools/essai` | scripts d'essai sur le vrai Mac et `analyse.py` (lecture des journaux) |
| `docs/` | `format.md` (le format `pet.toml`), `rig.md` (dessiner un rig SVG), `squelette.md`, `animation-animale.md` (ce que disent les études sur les moutons, et ce qu'en fait le moteur), `architecture.md` |

## Avant chaque commit

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --target aarch64-apple-darwin -- -D warnings
cargo test --workspace --release
```

La cible `aarch64-apple-darwin` vérifie le code macOS depuis Linux
(`rustup target add aarch64-apple-darwin` au besoin ; on ne fait que
vérifier, sans lier). Pour le coût de la simulation :

```sh
cargo test --release -p mouflon-core --test charge -- --ignored --nocapture --exact crowded
```

Mille animaux en tas : environ 1,4 ms par pas. Une nouveauté ne doit pas
le faire monter sensiblement, ni le coût du dessin d'une image :

```sh
cargo test --release -p mouflon-render --test cout -- --ignored --nocapture
```

Un dessin sans les nouvelles parties doit bouger et se cuire exactement
comme avant : le test `sans_changement` le garde.

## Voir ce qu'on dessine

```sh
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu gestes   # allures et gestes
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu poses
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu jeux     # jeux d'agneaux
cargo run -p mouflon-preview --release -- pets/mannequin/pet.toml apercu allures  # pas, trot, galop phase par phase, coucher, saut
```

Regarde toujours les images produites avant de dire qu'un dessin ou une
pose est juste. `apercu/` n'est pas versionné.

## Conventions

- Commentaires, documentation et messages de l'app en français (le README a
  une partie anglaise et une partie française) ; noms des tests en anglais,
  en phrases (`a_ram_backs_up_facing_forward_and_stops_at_the_end_of_a_ledge`).
- Chaque nouveauté du comportement ou du dessin a ses tests, et se retrouve
  dans la doc qui la concerne (`docs/format.md` pour une clé de `pet.toml`,
  `docs/animation-animale.md` pour la vie des moutons, avec ses sources).
- Pas de nouvelle dépendance sans vraie raison. Rien de bloquant sur le fil
  principal ni sur le fil de l'affichage.
- Les sons et images ajoutés : seulement CC0 ou domaine public, licence
  vérifiée, crédités dans `CREDITS.md`.
- Ne jamais dessiner ni reproduire un personnage protégé (Poe de
  *Stray Sheep*, le mouton d'eSheep, etc.).

## Ce qui ne va jamais dans le dépôt

- `animaux/` : animaux privés ou de tiers, seulement dans les builds de
  l'auteur (ignoré par git).
- `essais/`, `dist/`, `apercu/`, `target/`.
- `pets/aquarelle/morceaux/apercu.png` (écrit par le peintre, à ne pas livrer).

## Commits et CI

- Auteur des commits : `Alejandro San Martin Lamas <alejandrosml@users.noreply.github.com>`
  (`git -c user.name=… -c user.email=… commit`). Titre en anglais, corps en
  liste.
- Branche de travail : `fenetres-dock` (pull request n° 2), sauf indication
  contraire.
- La CI (`.github/workflows/ci.yml`) teste sous Linux, construit l'app Mac
  (artefact `Mouflon-macos-arm64`), puis fait quatre essais sur un Mac
  virtuel (trois minutes, une minute avec les quatre races, GPU, charge) et
  vérifie leurs budgets :
  `python3 tools/essai/analyse.py --budget <dossier>` doit dire « tenu »
  (aucun à-coup dû à Mouflon) et ne relever aucun saut de position. Les
  journaux sont dans l'artefact `essai-journal`.
