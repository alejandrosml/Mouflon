//! Icône et menu de la barre des menus (macOS) ou de la zone de notification (Windows).
//! Ce sont de vrais menus natifs : ils prennent d'eux-mêmes le style du système.

/// Ce que l'utilisateur demande depuis le menu.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub enum Action {
    /// Ajoute l'animal préféré (le dernier choisi).
    AddDefault,
    /// Ajoute cet animal de la bibliothèque, qui devient le préféré.
    Add(usize),
    RemoveAll,
    /// Tous les animaux jouent ensemble (`sync` de desktopPet).
    Together,
    TogglePause,
    ToggleFrontmostOnly,
    ToggleCollisions,
    /// Marcher sur le Dock tel qu'il est (autorisation d'Accessibilité).
    ToggleWalkOnDock,
    /// Taille des animaux : rang dans [`crate::settings::SIZES`].
    SetSize(usize),
    OpenPetsFolder,
    Quit,
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod native {
    use tray_icon::menu::{
        AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem,
        Submenu,
    };
    use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

    use super::Action;
    use crate::library::Library;

    pub struct TrayMenu {
        _tray: TrayIcon,
        add: Vec<(MenuId, usize)>,
        add_default: MenuId,
        remove_all: MenuId,
        together: MenuId,
        pause: CheckMenuItem,
        frontmost: CheckMenuItem,
        collisions: CheckMenuItem,
        walk_on_dock: CheckMenuItem,
        sizes: Vec<CheckMenuItem>,
        open_folder: MenuId,
        quit: MenuId,
    }

    impl std::fmt::Debug for TrayMenu {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TrayMenu").finish_non_exhaustive()
        }
    }

    impl TrayMenu {
        pub fn new(
            library: &Library,
            frontmost_only: bool,
            collisions_on: bool,
            walk_on_dock_on: bool,
            size: usize,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let t = crate::i18n::texts();
            let menu = Menu::new();
            let mut add = Vec::new();

            let first = MenuItem::new(&t.add_pet, true, None);
            menu.append(&first)?;
            if library.pets.len() > 1 {
                let submenu = Submenu::new(&t.add_menu, true);
                for (i, entry) in library.pets.iter().enumerate() {
                    let item = MenuItem::new(&entry.def.meta.name, true, None);
                    add.push((item.id().clone(), i));
                    submenu.append(&item)?;
                }
                menu.append(&submenu)?;
            }
            let together = MenuItem::new(&t.together, true, None);
            menu.append(&together)?;
            let remove_all = MenuItem::new(&t.remove_all, true, None);
            menu.append(&remove_all)?;
            menu.append(&PredefinedMenuItem::separator())?;

            let pause = CheckMenuItem::new(&t.pause, true, false, None);
            menu.append(&pause)?;
            let frontmost = CheckMenuItem::new(&t.frontmost_only, true, frontmost_only, None);
            menu.append(&frontmost)?;
            let collisions = CheckMenuItem::new(&t.collisions, true, collisions_on, None);
            menu.append(&collisions)?;
            let walk_on_dock = CheckMenuItem::new(&t.walk_on_dock, true, walk_on_dock_on, None);
            menu.append(&walk_on_dock)?;
            let size_menu = Submenu::new(&t.size, true);
            let mut sizes = Vec::new();
            for (i, label) in [&t.size_small, &t.size_normal, &t.size_large, &t.size_huge]
                .into_iter()
                .enumerate()
            {
                let item = CheckMenuItem::new(label, true, i == size, None);
                size_menu.append(&item)?;
                sizes.push(item);
            }
            menu.append(&size_menu)?;
            let open_folder = MenuItem::new(&t.open_pets_folder, true, None);
            menu.append(&open_folder)?;
            menu.append(&PredefinedMenuItem::separator())?;

            let about = AboutMetadata {
                name: Some("Mouflon".into()),
                version: Some(env!("CARGO_PKG_VERSION").into()),
                authors: Some(vec!["Alejandro San Martin Lamas".into()]),
                copyright: Some("© 2026 Alejandro San Martin Lamas".into()),
                license: Some("MIT OR Apache-2.0".into()),
                comments: Some(t.description.clone()),
                credits: Some(t.credits.clone()),
                ..AboutMetadata::default()
            };
            menu.append(&PredefinedMenuItem::about(Some(&t.about), Some(about)))?;
            let quit = MenuItem::new(&t.quit, true, None);
            menu.append(&quit)?;

            let size = 36;
            let icon = Icon::from_rgba(crate::icon::tray_icon(size), size, size)?;
            let builder = TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_tooltip("Mouflon");
            // Image « modèle » : macOS la teinte selon le thème de la barre des menus.
            #[cfg(target_os = "macos")]
            let builder = builder.with_icon_templated(icon);
            #[cfg(not(target_os = "macos"))]
            let builder = builder.with_icon(icon);
            let tray = builder.build()?;

            Ok(Self {
                _tray: tray,
                add,
                add_default: first.id().clone(),
                remove_all: remove_all.id().clone(),
                together: together.id().clone(),
                pause,
                frontmost,
                collisions,
                walk_on_dock,
                sizes,
                open_folder: open_folder.id().clone(),
                quit: quit.id().clone(),
            })
        }

        pub fn action(&self, event: &MenuEvent) -> Option<Action> {
            let id = &event.id;
            if let Some((_, i)) = self.add.iter().find(|(item, _)| item == id) {
                return Some(Action::Add(*i));
            }
            if id == &self.add_default {
                return Some(Action::AddDefault);
            }
            if let Some(i) = self.sizes.iter().position(|item| item.id() == id) {
                return Some(Action::SetSize(i));
            }
            if id == &self.remove_all {
                Some(Action::RemoveAll)
            } else if id == &self.together {
                Some(Action::Together)
            } else if id == self.pause.id() {
                Some(Action::TogglePause)
            } else if id == self.frontmost.id() {
                Some(Action::ToggleFrontmostOnly)
            } else if id == self.collisions.id() {
                Some(Action::ToggleCollisions)
            } else if id == self.walk_on_dock.id() {
                Some(Action::ToggleWalkOnDock)
            } else if id == &self.open_folder {
                Some(Action::OpenPetsFolder)
            } else if id == &self.quit {
                Some(Action::Quit)
            } else {
                None
            }
        }

        pub fn set_paused(&self, paused: bool) {
            self.pause.set_checked(paused);
        }

        pub fn set_frontmost_only(&self, value: bool) {
            self.frontmost.set_checked(value);
        }

        pub fn set_collisions(&self, value: bool) {
            self.collisions.set_checked(value);
        }

        pub fn set_walk_on_dock(&self, value: bool) {
            self.walk_on_dock.set_checked(value);
        }

        /// Coche la taille choisie, et elle seule.
        pub fn set_size(&self, size: usize) {
            for (i, item) in self.sizes.iter().enumerate() {
                item.set_checked(i == size);
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use native::TrayMenu;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use tray_icon::menu::MenuEvent;
