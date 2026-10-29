//! Icône et menu de la barre des menus (macOS) ou de la zone de notification (Windows).
//! Ce sont de vrais menus natifs : ils prennent d'eux-mêmes le style du système.

/// Ce que l'utilisateur demande depuis le menu.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub enum Action {
    /// Ajoute l'animal préféré (le dernier choisi).
    AddDefault,
    /// Ajoute cet animal de la bibliothèque (un de plus), qui devient le préféré.
    Add(usize),
    /// Dit au revoir à un animal de cette espèce (le dernier arrivé).
    RemoveOne(usize),
    RemoveAll,
    /// Tous les animaux jouent ensemble (`sync` de desktopPet).
    Together,
    TogglePause,
    /// Les animaux font entendre leurs sons, ou se taisent.
    ToggleSounds,
    ToggleFrontmostOnly,
    ToggleCollisions,
    /// Marcher sous le Dock (il faut l'autorisation d'Accessibilité).
    ToggleWalkUnderDock,
    /// Explique pourquoi « Marcher sous le Dock » demande l'Accessibilité.
    DockInfo,
    /// Taille des animaux : rang dans [`crate::settings::SIZES`].
    SetSize(usize),
    OpenPetsFolder,
    About,
    Quit,
}

/// Une espèce dans le menu : son nom, suivi du nombre d'animaux à l'écran.
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
fn counted(name: &str, count: usize) -> String {
    if count == 0 {
        name.to_owned()
    } else {
        format!("{name} ({count})")
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod native {
    use tray_icon::menu::{
        CheckMenuItem, IconMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    };
    use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

    use super::{Action, counted};
    use crate::library::Library;

    /// Le menu de l'icône de Mouflon :
    ///
    /// - « Ajouter Lainou » (le préféré), « Ajouter un animal ▸ » (chaque espèce,
    ///   avec le nombre d'animaux déjà à l'écran : un clic en ajoute un de plus),
    ///   « Dire au revoir ▸ » (à tous, ou à un animal d'une espèce),
    ///   « Tous ensemble » ;
    /// - « Pause », « Sons », « Collisions entre animaux » ;
    /// - « Réglages ▸ » : taille, app au premier plan, Dock, dossier des animaux ;
    /// - « À propos », « Quitter ».
    pub struct TrayMenu {
        _tray: TrayIcon,
        add_favorite: MenuItem,
        add: Vec<(MenuItem, usize)>,
        goodbye_all: MenuItem,
        goodbye: Vec<(MenuItem, usize)>,
        names: Vec<String>,
        together: MenuId,
        pause: CheckMenuItem,
        sounds: CheckMenuItem,
        frontmost: CheckMenuItem,
        collisions: CheckMenuItem,
        walk_under_dock: CheckMenuItem,
        dock_info: MenuId,
        sizes: Vec<CheckMenuItem>,
        open_folder: MenuId,
        about: MenuId,
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
            sounds_on: bool,
            frontmost_only: bool,
            collisions_on: bool,
            walk_under_dock_on: bool,
            size: usize,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let t = crate::i18n::texts();
            let menu = Menu::new();
            let names: Vec<String> = library
                .pets
                .iter()
                .map(|entry| entry.def.meta.name.clone())
                .collect();

            // Les animaux.
            let add_favorite = MenuItem::new(&t.add_pet, true, None);
            menu.append(&add_favorite)?;
            let add_menu = Submenu::new(&t.add_pet, true);
            let mut add = Vec::new();
            for (i, name) in names.iter().enumerate() {
                let item = MenuItem::new(name, true, None);
                add_menu.append(&item)?;
                add.push((item, i));
            }
            menu.append(&add_menu)?;
            let goodbye_menu = Submenu::new(&t.goodbye, true);
            let goodbye_all = MenuItem::new(&t.goodbye_all, false, None);
            goodbye_menu.append(&goodbye_all)?;
            goodbye_menu.append(&PredefinedMenuItem::separator())?;
            let mut goodbye = Vec::new();
            for (i, name) in names.iter().enumerate() {
                let item = MenuItem::new(name, false, None);
                goodbye_menu.append(&item)?;
                goodbye.push((item, i));
            }
            menu.append(&goodbye_menu)?;
            let together = MenuItem::new(&t.together, true, None);
            menu.append(&together)?;
            menu.append(&PredefinedMenuItem::separator())?;

            // Ce qu'on change souvent.
            let pause = CheckMenuItem::new(&t.pause, true, false, None);
            menu.append(&pause)?;
            let sounds = CheckMenuItem::new(&t.sounds, true, sounds_on, None);
            menu.append(&sounds)?;
            let collisions = CheckMenuItem::new(&t.collisions, true, collisions_on, None);
            menu.append(&collisions)?;
            menu.append(&PredefinedMenuItem::separator())?;

            // Réglages.
            let settings = Submenu::new(&t.settings, true);
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
            settings.append(&size_menu)?;
            let frontmost = CheckMenuItem::new(&t.frontmost_only, true, frontmost_only, None);
            settings.append(&frontmost)?;
            let walk_under_dock =
                CheckMenuItem::new(&t.walk_under_dock, true, walk_under_dock_on, None);
            settings.append(&walk_under_dock)?;
            // Juste dessous, « ⓘ » : pourquoi il faut l'autorisation d'Accessibilité.
            let dock_info = info_item(&t.dock_info);
            settings.append(&dock_info)?;
            settings.append(&PredefinedMenuItem::separator())?;
            let open_folder = MenuItem::new(&t.open_pets_folder, true, None);
            settings.append(&open_folder)?;
            menu.append(&settings)?;
            menu.append(&PredefinedMenuItem::separator())?;

            let about = MenuItem::new(&t.about, true, None);
            menu.append(&about)?;
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
                add_favorite,
                add,
                goodbye_all,
                goodbye,
                names,
                together: together.id().clone(),
                pause,
                sounds,
                frontmost,
                collisions,
                walk_under_dock,
                dock_info: dock_info.id().clone(),
                sizes,
                open_folder: open_folder.id().clone(),
                about: about.id().clone(),
                quit: quit.id().clone(),
            })
        }

        /// Le préféré (rang dans la bibliothèque) et le nombre d'animaux de
        /// chaque espèce à l'écran : les articles d'ajout et d'au revoir suivent.
        pub fn set_flock(&self, favorite: usize, counts: &[usize]) {
            let t = crate::i18n::texts();
            if let Some(name) = self.names.get(favorite) {
                self.add_favorite
                    .set_text(t.add_named.replace("{name}", name));
            }
            let count = |i: usize| counts.get(i).copied().unwrap_or(0);
            for (item, i) in &self.add {
                item.set_text(counted(&self.names[*i], count(*i)));
            }
            for (item, i) in &self.goodbye {
                item.set_text(counted(&self.names[*i], count(*i)));
                item.set_enabled(count(*i) > 0);
            }
            self.goodbye_all.set_enabled(counts.iter().any(|c| *c > 0));
        }

        pub fn action(&self, event: &MenuEvent) -> Option<Action> {
            let id = &event.id;
            if let Some((_, i)) = self.add.iter().find(|(item, _)| item.id() == id) {
                return Some(Action::Add(*i));
            }
            if let Some((_, i)) = self.goodbye.iter().find(|(item, _)| item.id() == id) {
                return Some(Action::RemoveOne(*i));
            }
            if id == self.add_favorite.id() {
                return Some(Action::AddDefault);
            }
            if let Some(i) = self.sizes.iter().position(|item| item.id() == id) {
                return Some(Action::SetSize(i));
            }
            if id == self.goodbye_all.id() {
                Some(Action::RemoveAll)
            } else if id == &self.together {
                Some(Action::Together)
            } else if id == self.pause.id() {
                Some(Action::TogglePause)
            } else if id == self.sounds.id() {
                Some(Action::ToggleSounds)
            } else if id == self.frontmost.id() {
                Some(Action::ToggleFrontmostOnly)
            } else if id == self.collisions.id() {
                Some(Action::ToggleCollisions)
            } else if id == self.walk_under_dock.id() {
                Some(Action::ToggleWalkUnderDock)
            } else if id == &self.dock_info {
                Some(Action::DockInfo)
            } else if id == &self.open_folder {
                Some(Action::OpenPetsFolder)
            } else if id == &self.about {
                Some(Action::About)
            } else if id == &self.quit {
                Some(Action::Quit)
            } else {
                None
            }
        }

        pub fn set_paused(&self, paused: bool) {
            self.pause.set_checked(paused);
        }

        pub fn set_sounds(&self, value: bool) {
            self.sounds.set_checked(value);
        }

        pub fn set_frontmost_only(&self, value: bool) {
            self.frontmost.set_checked(value);
        }

        pub fn set_collisions(&self, value: bool) {
            self.collisions.set_checked(value);
        }

        pub fn set_walk_under_dock(&self, value: bool) {
            self.walk_under_dock.set_checked(value);
        }

        /// Montre une courte explication dans une bulle sous l'icône de Mouflon.
        pub fn show_note(&self, title: &str, text: &str) {
            #[cfg(target_os = "macos")]
            let anchor = self._tray.ns_status_item();
            #[cfg(not(target_os = "macos"))]
            let anchor: Option<&mouflon_platform::StatusItem> = None;
            mouflon_platform::show_note(anchor.as_deref(), title, text);
        }

        /// Coche la taille choisie, et elle seule.
        pub fn set_size(&self, size: usize) {
            for (i, item) in self.sizes.iter().enumerate() {
                item.set_checked(i == size);
            }
        }
    }

    /// Article d'information, précédé d'un « ⓘ » que macOS teinte comme le
    /// texte du menu (image « modèle »).
    fn info_item(text: &str) -> IconMenuItem {
        let item = IconMenuItem::new(text, true, None, None);
        let size = 36;
        if let Ok(icon) = tray_icon::menu::Icon::from_rgba(crate::icon::info_icon(size), size, size)
        {
            #[cfg(target_os = "macos")]
            item.set_icon_templated(Some(icon));
            #[cfg(not(target_os = "macos"))]
            item.set_icon(Some(icon));
        }
        item
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use native::TrayMenu;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use tray_icon::menu::MenuEvent;

#[cfg(test)]
mod tests {
    use super::counted;

    #[test]
    fn a_species_shows_how_many_are_on_screen() {
        assert_eq!(counted("Lainou", 0), "Lainou");
        assert_eq!(counted("Lainou", 2), "Lainou (2)");
    }
}
