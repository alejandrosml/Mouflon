# Credits · Crédits

[English](#english) · [Français](#français)

## English

**Mouflon** © 2026 Alejandro San Martin Lamas. Free software, available under
the MIT License or the Apache License 2.0, at your option (`LICENSE-MIT`,
`LICENSE-APACHE`).

### Heritage

Mouflon is a tribute to the “screen mates” that have lived on desktops since 1995:

- **Stray Sheep**: character created by Tatsutoshi Nomura, series broadcast on
  Fuji Television (1994).
- **Stray Sheep: The Screen Mate**: the original program, published by Village
  Center, Inc. under license from Fuji Television (1995).
- **eSheep / desktopPet**: Adriano Petrucci's 64-bit remake, under the MIT
  License (<https://github.com/Adrianotiger/desktopPet>, notice below). Mouflon
  reads its pet format; the rules of its animation engine (steps, repeats,
  borders, gravity, scenes) come from its source code, rewritten in Rust.
  Pelote's behaviors are adapted from its `animations.xml` file.
- **Baaaa** (pda): a macOS port whose notes on the pitfalls of
  `CGWindowListCopyWindowInfo` guided window detection (no code taken).

The rights in the characters belong to their respective owners.

### Characters in this repository

- The **Mannequin** (`pets/mannequin`) is an original character created for
  Mouflon, under the same licenses as the code.
- **Lainou** (`pets/lainou`) is an original character by Alejandro San Martin
  Lamas, drawn for Mouflon, under the same licenses as the code.
- **Pelote** (`pets/pelote`) is an original character by Alejandro San Martin
  Lamas: its drawings are under the same licenses as the code, and its
  behaviors are adapted from desktopPet (MIT License,
  `pets/pelote/NOTICE_desktopPet_MIT.txt`). The app icon is Pelote's head.
- **Poete**, the mascot of Mouflon, is not part of this repository: it lives in
  its own repository, with its own notice.

### Libraries

Vello, wgpu, kurbo and peniko (Linebender), winit, tray-icon and muda (Tauri),
usvg and resvg, objc2 and its bindings, serde, toml, roxmltree, directories and
their dependencies: permissive licenses, mostly MIT and Apache-2.0 (also Zlib,
BSD, ISC, Unicode-3.0; option-ext is under MPL-2.0, used unmodified). Their
license texts come with the app (`THIRD-PARTY-LICENSES.txt`, gathered by
`tools/mouflon-licenses`).

## Français

**Mouflon** © 2026 Alejandro San Martin Lamas. Logiciel libre, sous licence MIT
ou licence Apache 2.0, au choix (`LICENSE-MIT`, `LICENSE-APACHE`).

### Héritage

Mouflon est un hommage aux « screen mates » qui peuplent les bureaux depuis 1995 :

- **Stray Sheep** : personnage créé par Tatsutoshi Nomura, série diffusée sur
  Fuji Television (1994).
- **Stray Sheep: The Screen Mate** : le programme d'origine, édité par Village
  Center, Inc. sous licence de Fuji Television (1995).
- **eSheep / desktopPet** : remake 64 bits d'Adriano Petrucci, sous licence MIT
  (<https://github.com/Adrianotiger/desktopPet>, notice ci-dessous). Mouflon lit
  son format d'animaux ; les règles de son moteur d'animation (pas,
  répétitions, bords, gravité, scènes) viennent de son code source, réécrites
  en Rust. Les comportements de Pelote sont adaptés de son fichier
  `animations.xml`.
- **Baaaa** (pda) : portage macOS dont la description des pièges de
  `CGWindowListCopyWindowInfo` a guidé la détection des fenêtres (aucun code repris).

Les droits sur les personnages appartiennent à leurs auteurs respectifs.

### Personnages de ce dépôt

- Le **Mannequin** (`pets/mannequin`) est un personnage original créé pour
  Mouflon, sous les mêmes licences que le code.
- **Lainou** (`pets/lainou`) est un personnage original d'Alejandro San Martin
  Lamas, dessiné pour Mouflon, sous les mêmes licences que le code.
- **Pelote** (`pets/pelote`) est un personnage original d'Alejandro San Martin
  Lamas : ses dessins sont sous les mêmes licences que le code, et ses
  comportements sont adaptés de desktopPet (licence MIT,
  `pets/pelote/NOTICE_desktopPet_MIT.txt`). L'icône de l'app est la tête de Pelote.
- **Poete**, la mascotte de Mouflon, ne fait pas partie de ce dépôt : il vit
  dans son propre dépôt, avec sa propre notice.

### Bibliothèques

Vello, wgpu, kurbo et peniko (Linebender), winit, tray-icon et muda (Tauri),
usvg et resvg, objc2 et ses bindings, serde, toml, roxmltree, directories et
leurs dépendances : licences permissives, surtout MIT et Apache-2.0 (aussi
Zlib, BSD, ISC, Unicode-3.0 ; option-ext est sous MPL-2.0, utilisée telle quelle).
Leurs textes de licence accompagnent l'app (`THIRD-PARTY-LICENSES.txt`, réunis
par `tools/mouflon-licenses`).

## desktopPet — MIT License

```text
MIT License

Copyright (c) [2016] [Adriano Petrucci]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
