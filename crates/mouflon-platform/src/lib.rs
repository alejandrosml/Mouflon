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

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{
    accessibility_trusted, cursor_position, dock_frame, frontmost_pid, open_folder,
    preferred_languages, prepare_pet_window, request_accessibility, screens, windows,
};

#[cfg(not(target_os = "macos"))]
mod fallback {
    //! Repli pour les systèmes pas encore pris en charge : aucun rebord de
    //! fenêtre, l'animal marche sur le bas de l'écran.
    use super::{OsScreen, OsWindow};

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
    accessibility_trusted, cursor_position, dock_frame, frontmost_pid, open_folder,
    preferred_languages, prepare_pet_window, request_accessibility, screens, windows,
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
