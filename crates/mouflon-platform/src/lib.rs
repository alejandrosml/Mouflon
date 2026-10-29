//! Couche système de Mouflon.
//!
//! C'est le **seul** crate du projet autorisé à contenir de l'`unsafe` : appeler
//! macOS (ou un autre système) depuis Rust l'est par nature, car le
//! compilateur ne peut pas vérifier du code C ou Objective-C. Tout est isolé
//! ici, derrière des fonctions sûres, et chaque bloc unsafe est justifié.
//!
//! Repère commun : points logiques, origine en haut à gauche de l'écran
//! principal, y vers le bas (celui de winit et de `CGWindowListCopyWindowInfo`).

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OsRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// Fenêtre d'une autre application.
#[derive(Clone, Debug, PartialEq)]
pub struct OsWindow {
    pub id: u64,
    pub pid: i32,
    pub frame: OsRect,
    /// Niveau d'empilement (macOS : 0 pour une fenêtre ordinaire, davantage
    /// pour les panneaux flottants, le Dock, la barre des menus…).
    pub layer: i32,
    /// Nom du processus propriétaire (pour le journal).
    pub owner: String,
}

/// Niveau du Dock : au-dessus, ce sont des éléments du système (barre des
/// menus, menus, bulles de notification…).
const DOCK_LEVEL: i32 = 20;
/// Niveau des panneaux d'accessibilité, dont le visualiseur de clavier.
const ASSISTIVE_TECH_HIGH_LEVEL: i32 = 1500;

impl OsWindow {
    /// Panneau qui flotte au-dessus des applications (visualiseur de clavier,
    /// vidéo en incrustation…) : visible quelle que soit l'app au premier plan.
    #[must_use]
    pub fn is_floating(&self) -> bool {
        self.layer != 0
    }

    /// Panneau d'accessibilité du système : visualiseur de clavier, clavier
    /// d'accessibilité. Il vit à un niveau élevé (celui des menus), mais c'est
    /// un vrai panneau posé sur le bureau.
    #[must_use]
    pub fn is_accessibility_panel(&self) -> bool {
        let owner = self.owner.to_lowercase();
        [
            "assistivecontrol",
            "keyboardviewer",
            "keyboard viewer",
            "visualiseur de clavier",
        ]
        .iter()
        .any(|name| owner.contains(name))
    }

    /// Peut-on marcher dessus ? Oui pour les fenêtres ordinaires, les panneaux
    /// flottants et utilitaires et les panneaux d'accessibilité ; non pour le
    /// Dock, la barre des menus, les menus et les notifications. (Les calques
    /// transparents qui couvrent tout un écran sont écartés à part, avec la
    /// taille des écrans.)
    #[must_use]
    pub fn is_walkable(&self) -> bool {
        (0..DOCK_LEVEL).contains(&self.layer)
            || self.layer == ASSISTIVE_TECH_HIGH_LEVEL
            || self.is_accessibility_panel()
    }
}

/// Un écran : cadre complet, zone utile (sans barre des menus ni Dock) et échelle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OsScreen {
    pub frame: OsRect,
    pub visible: OsRect,
    pub scale: f64,
}

/// Identifiant du processus courant.
#[must_use]
pub fn current_pid() -> i32 {
    std::process::id() as i32
}

/// Une ligne des crédits de la fenêtre « À propos ».
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AboutLine<'a> {
    Text(&'a str),
    /// Lien cliquable : texte affiché, adresse.
    Link(&'a str, &'a str),
}

/// Contenu de la fenêtre « À propos ».
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct About<'a> {
    pub name: &'a str,
    pub version: &'a str,
    /// Mention de copyright (« © 2026 … »).
    pub copyright: &'a str,
    /// Crédits, de haut en bas.
    pub lines: &'a [AboutLine<'a>],
}

/// Icône de Mouflon dans la barre des menus, où une bulle d'explication
/// peut s'accrocher (voir [`show_note`]).
#[cfg(target_os = "macos")]
pub type StatusItem = objc2_app_kit::NSStatusItem;
/// Pas d'icône où accrocher une bulle sur ce système.
#[cfg(not(target_os = "macos"))]
#[derive(Debug)]
pub enum StatusItem {}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{
    FrameClock, accessibility_trusted, close_note, cursor_position, dock_frame, frontmost_pid,
    open_folder, play_sound, preferred_languages, prepare_pet_window, request_accessibility,
    screens, show_about, show_note, warm_up_sounds, window_max_fps, windows,
};

#[cfg(not(target_os = "macos"))]
mod fallback {
    //! Repli pour les systèmes pas encore pris en charge : aucun rebord de
    //! fenêtre, l'animal marche sur le bas de l'écran.
    use super::{About, AboutLine, OsScreen, OsWindow, StatusItem};

    #[must_use]
    pub fn screens() -> Vec<OsScreen> {
        Vec::new()
    }

    #[must_use]
    pub fn windows() -> Vec<OsWindow> {
        Vec::new()
    }

    #[must_use]
    pub fn frontmost_pid() -> Option<i32> {
        None
    }

    #[must_use]
    pub fn cursor_position() -> Option<(f64, f64)> {
        None
    }

    pub fn prepare_pet_window(_window: &impl raw_window_handle::HasWindowHandle) {}

    /// Pas de fréquence d'écran connue ici (celle de winit sert).
    #[must_use]
    pub fn window_max_fps(_window: &impl raw_window_handle::HasWindowHandle) -> Option<f64> {
        None
    }

    /// Pas d'autorisation d'Accessibilité à demander ici.
    #[must_use]
    pub fn accessibility_trusted() -> bool {
        false
    }

    pub fn request_accessibility() -> bool {
        false
    }

    /// Pas de Dock.
    #[must_use]
    pub fn dock_frame() -> Option<super::OsRect> {
        None
    }

    /// Langues de l'utilisateur, d'après les variables d'environnement POSIX
    /// (`LANGUAGE`, puis `LC_ALL`, `LC_MESSAGES` et `LANG`), en codes BCP 47.
    #[must_use]
    pub fn preferred_languages() -> Vec<String> {
        let mut tags: Vec<String> = std::env::var("LANGUAGE")
            .unwrap_or_default()
            .split(':')
            .filter(|tag| !tag.is_empty())
            .map(|tag| tag.replace('_', "-"))
            .collect();
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            let value = std::env::var(var).unwrap_or_default();
            // « fr_FR.UTF-8 » ou « es_ES@euro » → « fr-FR », « es-ES ».
            let tag = value.split(['.', '@']).next().unwrap_or_default();
            if !tag.is_empty() && tag != "C" && tag != "POSIX" {
                tags.push(tag.replace('_', "-"));
            }
        }
        tags
    }

    /// Pas encore de fenêtre « À propos » ici : on ouvre le premier lien
    /// des crédits (la page du projet) dans le navigateur.
    pub fn show_about(about: &About<'_>) {
        let link = about.lines.iter().find_map(|line| match line {
            AboutLine::Link(_, url) => Some(*url),
            AboutLine::Text(_) => None,
        });
        if let Some(url) = link {
            open_folder(std::path::Path::new(url));
        }
    }

    /// Pas de bulle ici : l'explication va au journal.
    pub fn show_note(_anchor: Option<&StatusItem>, title: &str, text: &str) {
        log::info!("{title} : {text}");
    }

    pub fn close_note() {}

    /// Pas d'horloge d'écran ici : l'app se rythme avec une minuterie.
    #[derive(Debug)]
    pub enum FrameClock {}

    impl FrameClock {
        pub fn new(_tick: impl Fn(f64) + 'static) -> Option<Self> {
            None
        }

        #[must_use]
        pub fn max_fps(&self) -> f32 {
            match *self {}
        }

        #[must_use]
        pub fn is_running(&self) -> bool {
            match *self {}
        }

        pub fn set_running(&self, _running: bool) {
            match *self {}
        }

        pub fn set_rate(&self, _fps: f32) {
            match *self {}
        }
    }

    /// Pas encore de sons sur ce système.
    pub fn play_sound(_data: &[u8], _volume: f32) -> bool {
        false
    }

    pub fn warm_up_sounds(_sample: Option<&[u8]>) {}

    /// Ouvre un dossier dans le gestionnaire de fichiers du système.
    pub fn open_folder(path: &std::path::Path) -> bool {
        let opener = if cfg!(target_os = "windows") {
            "explorer"
        } else {
            "xdg-open"
        };
        std::process::Command::new(opener).arg(path).spawn().is_ok()
    }
}

#[cfg(not(target_os = "macos"))]
pub use fallback::{
    FrameClock, accessibility_trusted, close_note, cursor_position, dock_frame, frontmost_pid,
    open_folder, play_sound, preferred_languages, prepare_pet_window, request_accessibility,
    screens, show_about, show_note, warm_up_sounds, window_max_fps, windows,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn at(layer: i32) -> OsWindow {
        OsWindow {
            id: 1,
            pid: 2,
            frame: OsRect::default(),
            layer,
            owner: String::new(),
        }
    }

    #[test]
    fn the_keyboard_viewer_is_walkable_at_any_level() {
        let viewer = OsWindow {
            owner: "AssistiveControl".into(),
            ..at(101)
        };
        assert!(viewer.is_accessibility_panel() && viewer.is_walkable());
    }

    #[test]
    fn walks_on_apps_and_floating_panels_but_not_on_system_chrome() {
        assert!(at(0).is_walkable() && !at(0).is_floating());
        assert!(
            at(3).is_walkable() && at(3).is_floating(),
            "panneau flottant"
        );
        assert!(at(1500).is_walkable(), "visualiseur de clavier");
        assert!(!at(20).is_walkable(), "Dock");
        assert!(!at(24).is_walkable(), "barre des menus");
        assert!(!at(101).is_walkable(), "menu");
        assert!(!at(-20).is_walkable());
    }
}
