# Mouflon — consignes pour les agents (Codex compris)

Les consignes du dépôt sont dans [CLAUDE.md](CLAUDE.md) : elles valent pour
tout agent, pas seulement pour Claude. Lis-le en entier avant de commencer
(le dépôt, les vérifications avant chaque commit, les conventions, ce qui ne
va jamais dans le dépôt).

En plus, pour une tâche Codex :

- Le travail en cours est décrit dans `docs/chantiers/` ; la tâche qu'on te
  confie dit quel chantier faire. Chaque chantier dit quoi lire d'abord.
- Ta pull request vise la branche `fenetres-dock` (la PR n° 2 la portera
  ensuite dans `main`). Une tâche, une PR. D'autres tâches travaillent en
  même temps : ne touche qu'aux fichiers de ton chantier (une race ne touche
  que les siens, voir `docs/chantiers/races-dessins.md`).
- Avant la PR : `cargo fmt --all`, les deux `cargo clippy` de `CLAUDE.md`
  (la cible `aarch64-apple-darwin` se vérifie sans lier :
  `rustup target add aarch64-apple-darwin` au besoin),
  `cargo test --workspace --release`, et les bancs `crowded` et `cout` si
  tu touches au cœur ou au dessin. Dis dans la PR ce que tu as vérifié, et
  ce que tu n'as pas pu vérifier.
- Regarde les images que tu produis (les planches de l'aperçu) avant de dire
  qu'un dessin ou une pose est juste, et compare-les avec des photos de vrais
  moutons. Les photos trouvées sur le web servent de référence seulement :
  jamais copiées dans le dépôt. Aucune donnée personnelle dans une requête
  (User-Agent générique, par exemple `MouflonResearch/1.0`).
- Ne dessine ni ne reproduis jamais un personnage protégé (Poe de
  *Stray Sheep*, le mouton d'eSheep).
- Commentaires, documentation, messages de l'app et description de la PR en
  français ; titres de commits en anglais, corps en liste ; noms des tests
  en anglais, en phrases. Auteur des commits :
  `Alejandro San Martin Lamas <alejandrosml@users.noreply.github.com>`.
