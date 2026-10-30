//! Les menus de Mouflon : celui de son icône dans la barre des menus (macOS)
//! ou la zone de notification (Windows), et celui d'un animal (clic droit).
//! Ce sont de vrais menus natifs : ils prennent d'eux-mêmes le style du
//! système.
//!
//! Les menus et la fenêtre des réglages mènent aux mêmes [`Action`], et
//! montrent tous le même état, celui de l'app : un réglage changé d'un côté
//! se voit aussitôt de l'autre.

use mouflon_platform::PopupItem;

use crate::i18n::Texts;

/// Ce que l'utilisateur demande, d'un menu ou de la fenêtre des réglages.
/// Un réglage y vient avec sa nouvelle valeur, jamais « l'inverse de
/// l'actuelle » : deux endroits qui le changent ne peuvent pas se contredire.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub enum Action {
    /// Ajoute l'animal préféré (le dernier choisi).
    AddDefault,
    /// Ajoute cet animal de la bibliothèque (un de plus), qui devient le préféré.
    Add(usize),
    /// Dit au revoir à un animal de cette espèce (le dernier arrivé).
    RemoveOne(usize),
    RemoveAll,
    /// Cet animal-là (son numéro) dit au revoir.
    Goodbye(u32),
    /// Tous les animaux jouent ensemble (`sync` de desktopPet).
    Together,
    SetPaused(bool),
    /// Les animaux font entendre leurs sons, ou se taisent.
    SetSounds(bool),
    /// Taille des animaux : rang dans [`crate::settings::SIZES`].
    SetSize(usize),
    SetCollisions(bool),
    SetRealisticPhysics(bool),
    SetFrontmostOnly(bool),
    /// Marcher sous le Dock (il faut l'autorisation d'Accessibilité).
    SetWalkUnderDock(bool),
    /// Ouvrir Mouflon à l'ouverture de session.
    SetOpenAtLogin(bool),
    OpenSettings,
    /// La fenêtre des réglages revient au premier plan : ce qu'elle montre
    /// de l'état du système (autorisations) est relu.
    RefreshSettings,
    OpenPetsFolder,
    OpenAccessibilitySettings,
    OpenLoginItemsSettings,
    About,
    Quit,
}

/// Une espèce dans le menu : son nom, suivi du nombre d'animaux à l'écran
/// quand il y en a plusieurs (un seul se voit bien sans le dire).
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
fn counted(name: &str, count: usize) -> String {
    if count <= 1 {
        name.to_owned()
    } else {
        format!("{name} ({count})")
    }
}

/// Un article du menu d'un animal. Son identifiant dit ce qu'il fait, et
/// pour un réglage, la valeur qu'il donne (celle qu'on voyait en cliquant).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PetChoice {
    /// Cet animal dit au revoir.
    Goodbye,
    /// Un animal de plus de la même espèce.
    AddSame,
    Size(usize),
    Sounds(bool),
    Pause(bool),
    Settings,
}

impl PetChoice {
    const SIZES: u32 = 100;

    fn id(self) -> u32 {
        match self {
            Self::Goodbye => 1,
            Self::AddSame => 2,
            Self::Settings => 3,
            Self::Sounds(on) => 10 + u32::from(on),
            Self::Pause(on) => 20 + u32::from(on),
            Self::Size(i) => Self::SIZES + i as u32,
        }
    }

    #[must_use]
    pub fn from_id(id: u32) -> Option<Self> {
        Some(match id {
            1 => Self::Goodbye,
            2 => Self::AddSame,
            3 => Self::Settings,
            10 | 11 => Self::Sounds(id == 11),
            20 | 21 => Self::Pause(id == 21),
            id if (Self::SIZES..Self::SIZES + 100).contains(&id) => {
                Self::Size((id - Self::SIZES) as usize)
            }
            _ => return None,
        })
    }

    /// Ce que fait ce choix pour l'animal `pet`, d'une espèce de la
    /// bibliothèque (`species` : son rang, s'il y est encore).
    #[must_use]
    pub fn action(self, pet: u32, species: Option<usize>) -> Option<Action> {
        Some(match self {
            Self::Goodbye => Action::Goodbye(pet),
            Self::AddSame => Action::Add(species?),
            Self::Size(i) => Action::SetSize(i),
            Self::Sounds(on) => Action::SetSounds(on),
            Self::Pause(on) => Action::SetPaused(on),
            Self::Settings => Action::OpenSettings,
        })
    }
}

/// Ce que le menu d'un animal montre de l'état de l'app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PetMenuState {
    pub paused: bool,
    pub sounds: bool,
    /// Rang de la taille dans [`crate::settings::SIZES`].
    pub size: usize,
}

/// Le menu d'un animal (clic droit) : son nom, ce qu'on peut faire de lui,
/// puis les réglages qu'on change le plus, comme dans la barre des menus.
#[must_use]
pub fn pet_menu(name: &str, state: PetMenuState, t: &Texts) -> Vec<PopupItem> {
    let item = |choice: PetChoice, label: &str, checked: bool| PopupItem::Item {
        id: choice.id(),
        label: label.to_owned(),
        checked,
        enabled: true,
    };
    let sizes = [&t.size_small, &t.size_normal, &t.size_large, &t.size_huge]
        .into_iter()
        .enumerate()
        .map(|(i, label)| item(PetChoice::Size(i), label, i == state.size))
        .collect();
    let pause = if state.paused { &t.resume } else { &t.pause };
    vec![
        PopupItem::Header(name.to_owned()),
        item(
            PetChoice::Goodbye,
            &t.goodbye_named.replace("{name}", name),
            false,
        ),
        item(
            PetChoice::AddSame,
            &t.add_named.replace("{name}", name),
            false,
        ),
        PopupItem::Separator,
        PopupItem::Submenu {
            label: t.size.clone(),
            items: sizes,
        },
        item(PetChoice::Sounds(!state.sounds), &t.sounds, state.sounds),
        item(PetChoice::Pause(!state.paused), pause, false),
        PopupItem::Separator,
        item(PetChoice::Settings, &t.settings, false),
    ]
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod native {
    use std::cell::Cell;

    use tray_icon::menu::accelerator::{Accelerator, CMD_OR_CTRL, Code};
    use tray_icon::menu::{
        CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    };
    use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

    use super::{Action, counted};
    use crate::library::Library;

    /// Raccourci affiché dans le menu (⌘, ou ⌘Q sur Mac).
    fn shortcut(key: Code) -> Option<Accelerator> {
        cfg!(target_os = "macos").then(|| Accelerator::new(CMD_OR_CTRL, key))
    }

    /// Le menu de l'icône de Mouflon, en neuf articles :
    ///
    /// - « Ajouter Lainou » (le préféré), « Ajouter un autre animal ▸ »
    ///   (chaque espèce, avec le nombre d'animaux déjà à l'écran s'il y en a
    ///   plusieurs : un clic en ajoute un de plus), « Dire au revoir ▸ » (à
    ///   tous, ou à un animal d'une espèce), « Faire jouer ensemble » ;
    /// - « Mettre en pause » (ou « Reprendre »), « Sons » ;
    /// - « Réglages… » (⌘,), qui ouvre la fenêtre des réglages ;
    /// - « À propos », « Quitter » (⌘Q).
    pub struct TrayMenu {
        _tray: TrayIcon,
        add_favorite: MenuItem,
        add: Vec<(MenuItem, usize)>,
        goodbye_all: MenuItem,
        goodbye: Vec<(MenuItem, usize)>,
        names: Vec<String>,
        together: MenuItem,
        pause: MenuItem,
        /// Ce que montre l'article de pause : en pause, il propose de reprendre.
        paused: Cell<bool>,
        sounds: CheckMenuItem,
        settings: MenuId,
        about: MenuId,
        quit: MenuId,
    }

    impl std::fmt::Debug for TrayMenu {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TrayMenu").finish_non_exhaustive()
        }
    }

    impl TrayMenu {
        pub fn new(library: &Library, sounds_on: bool) -> Result<Self, Box<dyn std::error::Error>> {
            let t = crate::i18n::texts();
            let menu = Menu::new();
            let names: Vec<String> = library
                .pets
                .iter()
                .map(|entry| entry.def.meta.name.clone())
                .collect();

            // Les animaux.
            let add_favorite = MenuItem::new(&t.add_named, true, None);
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
            let together = MenuItem::new(&t.together, false, None);
            menu.append(&together)?;
            menu.append(&PredefinedMenuItem::separator())?;

            // Ce qu'on change souvent.
            let pause = MenuItem::new(&t.pause, true, None);
            menu.append(&pause)?;
            let sounds = CheckMenuItem::new(&t.sounds, true, sounds_on, None);
            menu.append(&sounds)?;
            menu.append(&PredefinedMenuItem::separator())?;

            // Tout le reste est dans la fenêtre des réglages.
            let settings = MenuItem::new(&t.settings, true, shortcut(Code::Comma));
            menu.append(&settings)?;
            menu.append(&PredefinedMenuItem::separator())?;

            let about = MenuItem::new(&t.about, true, None);
            menu.append(&about)?;
            let quit = MenuItem::new(&t.quit, true, shortcut(Code::KeyQ));
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
                together,
                pause,
                paused: Cell::new(false),
                sounds,
                settings: settings.id().clone(),
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
            let anyone = counts.iter().any(|c| *c > 0);
            self.goodbye_all.set_enabled(anyone);
            self.together.set_enabled(anyone);
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
                Some(Action::AddDefault)
            } else if id == self.goodbye_all.id() {
                Some(Action::RemoveAll)
            } else if id == self.together.id() {
                Some(Action::Together)
            } else if id == self.pause.id() {
                Some(Action::SetPaused(!self.paused.get()))
            } else if id == self.sounds.id() {
                // La case vient de changer d'elle-même : c'est ce que l'on voit.
                Some(Action::SetSounds(self.sounds.is_checked()))
            } else if id == &self.settings {
                Some(Action::OpenSettings)
            } else if id == &self.about {
                Some(Action::About)
            } else if id == &self.quit {
                Some(Action::Quit)
            } else {
                None
            }
        }

        pub fn set_paused(&self, paused: bool) {
            if self.paused.replace(paused) != paused {
                let t = crate::i18n::texts();
                self.pause
                    .set_text(if paused { &t.resume } else { &t.pause });
            }
        }

        pub fn set_sounds(&self, value: bool) {
            self.sounds.set_checked(value);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use native::TrayMenu;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use tray_icon::menu::MenuEvent;

#[cfg(test)]
mod tests {
    use super::*;

    fn texts() -> Texts {
        toml::from_str(include_str!("../locales/fr.toml")).unwrap()
    }

    #[test]
    fn a_species_shows_how_many_are_on_screen_from_two() {
        assert_eq!(counted("Lainou", 0), "Lainou");
        assert_eq!(counted("Lainou", 1), "Lainou");
        assert_eq!(counted("Lainou", 2), "Lainou (2)");
        assert_eq!(counted("Lainou", 12), "Lainou (12)");
    }

    #[test]
    fn every_choice_of_a_pet_menu_keeps_its_meaning_through_its_id() {
        let choices = [
            PetChoice::Goodbye,
            PetChoice::AddSame,
            PetChoice::Settings,
            PetChoice::Sounds(false),
            PetChoice::Sounds(true),
            PetChoice::Pause(false),
            PetChoice::Pause(true),
            PetChoice::Size(0),
            PetChoice::Size(3),
        ];
        for choice in choices {
            assert_eq!(PetChoice::from_id(choice.id()), Some(choice));
        }
        assert_eq!(PetChoice::from_id(0), None);
        assert_eq!(PetChoice::from_id(99), None);
    }

    /// Les cases du menu d'un animal montrent l'état de l'app, et chaque
    /// réglage y donne la valeur contraire de celle qu'on voit.
    #[test]
    fn a_pet_menu_shows_the_state_and_offers_its_opposite() {
        let t = texts();
        for paused in [false, true] {
            for sounds in [false, true] {
                for size in 0..4 {
                    let state = PetMenuState {
                        paused,
                        sounds,
                        size,
                    };
                    let items = pet_menu("Pelote", state, &t);
                    assert_eq!(items[0], PopupItem::Header("Pelote".into()));
                    let mut seen = Vec::new();
                    let mut collect = |item: &PopupItem| {
                        if let PopupItem::Item {
                            id, label, checked, ..
                        } = item
                        {
                            seen.push((PetChoice::from_id(*id).unwrap(), label.clone(), *checked));
                        }
                    };
                    for item in &items {
                        match item {
                            PopupItem::Submenu { items, .. } => items.iter().for_each(&mut collect),
                            other => collect(other),
                        }
                    }
                    let find = |wanted: fn(&PetChoice) -> bool| {
                        seen.iter().find(|(c, ..)| wanted(c)).unwrap().clone()
                    };
                    let (choice, _, checked) = find(|c| matches!(c, PetChoice::Sounds(_)));
                    assert_eq!((choice, checked), (PetChoice::Sounds(!sounds), sounds));
                    let (choice, label, _) = find(|c| matches!(c, PetChoice::Pause(_)));
                    assert_eq!(choice, PetChoice::Pause(!paused));
                    assert_eq!(
                        label,
                        if paused {
                            "Reprendre"
                        } else {
                            "Mettre en pause"
                        }
                    );
                    let checked: Vec<usize> = seen
                        .iter()
                        .filter_map(|(c, _, on)| match c {
                            PetChoice::Size(i) if *on => Some(*i),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(checked, vec![size], "une seule taille cochée");
                    assert!(seen.iter().any(|(_, l, _)| l == "Dire au revoir à Pelote"));
                }
            }
        }
    }

    #[test]
    fn a_pet_menu_acts_on_that_pet_and_its_species() {
        assert_eq!(
            PetChoice::Goodbye.action(7, Some(2)),
            Some(Action::Goodbye(7))
        );
        assert_eq!(PetChoice::AddSame.action(7, Some(2)), Some(Action::Add(2)));
        // L'espèce n'est plus dans la bibliothèque : rien à ajouter.
        assert_eq!(PetChoice::AddSame.action(7, None), None);
        assert_eq!(PetChoice::Size(3).action(7, None), Some(Action::SetSize(3)));
        assert_eq!(
            PetChoice::Settings.action(7, None),
            Some(Action::OpenSettings)
        );
    }
}
