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

/// Rend au système la mémoire libérée que l'allocateur garde encore pour
/// la réutiliser (sur Mac, des pages restées « sales » qui comptent dans
/// l'empreinte de l'app pendant des minutes). Renvoie les octets rendus.
/// À appeler hors du fil de l'affichage : le parcours prend quelques
/// millisecondes.
pub fn release_free_memory() -> usize {
    #[cfg(target_os = "macos")]
    {
        unsafe extern "C" {
            /// `<malloc/malloc.h>` (macOS 10.7) : `zone` nul = toutes les
            /// zones, `goal` nul = tout ce qui peut l'être.
            fn malloc_zone_pressure_relief(zone: *mut std::ffi::c_void, goal: usize) -> usize;
        }
        // SAFETY: fonction de la bibliothèque C du système, sûre depuis
        // n'importe quel fil ; un pointeur nul désigne toutes les zones.
        unsafe { malloc_zone_pressure_relief(std::ptr::null_mut(), 0) }
    }
    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}

/// Fait passer le fil courant après tout le reste : un travail de fond (la
/// compression des images) qui ne doit jamais retenir l'affichage. Sur Mac,
/// la classe de service « utilitaire » : le système le place de préférence
/// sur les cœurs économes, et le ralentit quand la batterie est basse.
pub fn lower_thread_priority() {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: fonction de libpthread sans pointeur, qui ne change que la
        // classe de service du fil qui l'appelle.
        let status =
            unsafe { libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_UTILITY, 0) };
        if status != 0 {
            log::debug!("classe de service du fil inchangée ({status})");
        }
    }
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

/// Ce que l'utilisateur fait dans les fenêtres et les menus propres à
/// Mouflon (réglages, menu d'un animal). L'app les reçoit par
/// [`set_ui_handler`] ; les identifiants sont ceux qu'elle a donnés.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiEvent {
    /// Article choisi dans un menu surgissant ([`pop_up_menu`]).
    Menu(u32),
    /// Le menu surgissant s'est refermé (après l'article choisi, s'il y en a un).
    MenuClosed,
    /// Case des réglages cochée (`true`) ou décochée.
    Check(u32, bool),
    /// Cran choisi sur un curseur des réglages.
    Step(u32, usize),
    /// Bouton des réglages.
    Button(u32),
    /// La fenêtre des réglages passe au premier plan : ce qu'elle montre de
    /// l'état du système (autorisations…) est à relire, il a pu changer.
    FormShown,
    /// ⌘Q pendant que la fenêtre des réglages est au premier plan.
    Quit,
}

/// Un article de menu surgissant (le menu d'un animal, au clic droit).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopupItem {
    /// Titre de section, qu'on ne choisit pas (le nom de l'animal).
    Header(String),
    /// Article à choisir, coché ou non.
    Item {
        id: u32,
        label: String,
        checked: bool,
        enabled: bool,
    },
    /// Sous-menu.
    Submenu {
        label: String,
        items: Vec<PopupItem>,
    },
    Separator,
}

/// La fenêtre des réglages : des sections de lignes, de haut en bas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub title: String,
    pub sections: Vec<FormSection>,
}

/// Une section : un titre, puis ses lignes dans un cadre.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormSection {
    pub title: String,
    pub rows: Vec<FormRow>,
}

/// Une ligne des réglages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormRow {
    /// Case à cocher, avec une explication dessous et, au besoin, un
    /// complément qui n'apparaît que quand l'app le demande.
    Check {
        id: u32,
        label: String,
        help: Option<String>,
        extra: Option<FormExtra>,
    },
    /// Curseur à crans : un libellé au-dessus, le nom de chaque cran
    /// dessous, puis une explication s'il y en a une.
    Steps {
        id: u32,
        label: String,
        steps: Vec<String>,
        help: Option<String>,
    },
    /// Un texte, et un bouton à sa droite.
    Text {
        text: String,
        button: Option<FormButton>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormButton {
    pub id: u32,
    pub label: String,
}

/// Complément d'une case : une remarque et un bouton, montrés ou cachés
/// ensemble ([`FormValue::Shown`], avec l'identifiant du bouton).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormExtra {
    pub note: String,
    pub button: FormButton,
}

/// Ce que la fenêtre des réglages montre : l'app les donne à chaque
/// changement, d'où qu'il vienne.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormValue {
    Check(u32, bool),
    Step(u32, usize),
    /// Montre ou cache un complément (identifiant de son bouton).
    Shown(u32, bool),
}

/// Mouflon s'ouvre-t-il à l'ouverture de session ? C'est le système qui le
/// sait : il le garde, et l'utilisateur peut le changer de son côté
/// (Réglages Système → Général → Ouverture).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginItem {
    /// Pas réglable d'ici : macOS 12 ou plus ancien, ou Mouflon lancé hors
    /// de son app (`Mouflon.app`).
    Unavailable,
    Off,
    On,
    /// Demandé, mais en attente de l'accord de l'utilisateur dans Réglages
    /// Système.
    NeedsApproval,
}

impl LoginItem {
    /// La case « Ouvrir à l'ouverture de session » est-elle cochée ?
    #[must_use]
    pub fn is_requested(self) -> bool {
        matches!(self, Self::On | Self::NeedsApproval)
    }
}

mod alloc;
pub mod diag;
#[cfg(target_os = "macos")]
mod macos;
mod mapped;
mod usage;

pub use alloc::{AllocCounts, CountingAlloc, alloc_counts, set_alloc_counting};
pub use mapped::{SharedBytes, map_file};
pub use usage::{ProcessUsage, process_usage};

/// Un battement de l'horloge de l'écran, en secondes de l'horloge des
/// médias ([`diag::media_time`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Beat {
    /// Heure à laquelle l'image préparée maintenant sera affichée.
    pub target: f64,
    /// Heure du rafraîchissement qui vient d'avoir lieu (la synchronisation
    /// verticale) : `target - vsync` est la durée d'une image de l'écran.
    pub vsync: f64,
    /// Heure à laquelle l'horloge nous a appelés.
    pub called: f64,
}

#[cfg(target_os = "macos")]
pub use macos::{
    FrameClock, accessibility_trusted, control_key_down, cursor_position, dock_frame,
    frontmost_pid, keep_pet_window, login_item, open_accessibility_settings, open_folder,
    open_login_items_settings, play_sound, pop_up_menu, preferred_languages, prepare_pet_window,
    request_accessibility, screens, set_login_item, set_ui_handler, show_about, show_form,
    snapshot_form, update_form, user_activity, warm_up_sounds, window_max_fps, windows,
};

/// Ce que fait l'utilisateur, tous programmes confondus : quand, jamais quoi.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserActivity {
    /// Secondes depuis la dernière touche frappée (laquelle, on ne le sait pas).
    pub since_key: f64,
    /// Secondes depuis son dernier geste : clavier, souris, trackpad.
    pub idle: f64,
}

#[cfg(not(target_os = "macos"))]
mod fallback {
    //! Repli pour les systèmes pas encore pris en charge : aucun rebord de
    //! fenêtre, l'animal marche sur le bas de l'écran.
    use super::{
        About, AboutLine, Form, FormValue, LoginItem, OsScreen, OsWindow, PopupItem, UiEvent,
    };

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

    #[must_use]
    pub fn user_activity() -> Option<super::UserActivity> {
        None
    }

    pub fn prepare_pet_window(_window: &impl raw_window_handle::HasWindowHandle) {}

    /// Les fenêtres d'animaux restent ici des fenêtres ordinaires : rien à garder.
    pub fn keep_pet_window(_window: &impl raw_window_handle::HasWindowHandle) {}

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

    /// Pas encore de fenêtres ni de menus surgissants à Mouflon ici : rien
    /// ne viendra par là.
    pub fn set_ui_handler(_handler: impl Fn(UiEvent) + 'static) {}

    pub fn pop_up_menu(_items: &[PopupItem]) {}

    /// Pas de fenêtre des réglages ici : `false`, à l'app de proposer
    /// autre chose (le fichier des réglages).
    pub fn show_form(_form: &Form, _values: &[FormValue]) -> bool {
        false
    }

    pub fn update_form(_values: &[FormValue]) {}

    pub fn snapshot_form(_path: &std::path::Path, _dark: bool) -> bool {
        false
    }

    /// Le clic droit suffit ici.
    #[must_use]
    pub fn control_key_down() -> bool {
        false
    }

    pub fn open_accessibility_settings() -> bool {
        false
    }

    #[must_use]
    pub fn login_item() -> LoginItem {
        LoginItem::Unavailable
    }

    pub fn set_login_item(_on: bool) -> LoginItem {
        LoginItem::Unavailable
    }

    pub fn open_login_items_settings() {}

    /// Pas d'horloge d'écran ici : l'app se rythme avec une minuterie.
    #[derive(Debug)]
    pub enum FrameClock {}

    impl FrameClock {
        pub fn new(_tick: impl Fn(super::Beat) + 'static) -> Option<Self> {
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

        #[must_use]
        pub fn frame_duration(&self) -> f64 {
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
    pub fn play_sound(_data: &std::sync::Arc<[u8]>, _volume: f32) {}

    pub fn warm_up_sounds(_sample: Option<&std::sync::Arc<[u8]>>) {}

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
    FrameClock, accessibility_trusted, control_key_down, cursor_position, dock_frame,
    frontmost_pid, keep_pet_window, login_item, open_accessibility_settings, open_folder,
    open_login_items_settings, play_sound, pop_up_menu, preferred_languages, prepare_pet_window,
    request_accessibility, screens, set_login_item, set_ui_handler, show_about, show_form,
    snapshot_form, update_form, user_activity, warm_up_sounds, window_max_fps, windows,
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
