//! Réglages persistants et emplacements des fichiers.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Tailles proposées dans le menu (petite, normale, grande, très grande) :
/// facteur appliqué à la taille prévue par chaque animal.
pub const SIZES: [f32; 4] = [0.75, 1.0, 1.5, 2.0];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Nombre d'animaux au lancement.
    pub start_count: u32,
    /// Identifiant de l'animal préféré (`[pet] id` de son fichier) ; sans
    /// choix, le premier animal intégré à l'app, sinon le mannequin.
    pub default_pet: Option<String>,
    /// Ne marcher que sur les fenêtres de l'app au premier plan : évite les
    /// « fenêtres fantômes » que macOS signale sur les autres Spaces.
    pub frontmost_only: bool,
    /// Mode collisions : deux animaux qui se rentrent dedans partent à la renverse.
    pub collisions: bool,
    /// Taille des animaux, par rapport à celle prévue par chacun.
    pub size: f32,
    /// Le Dock tel qu'il est : les animaux marchent dessus et en bas de
    /// l'écran à côté de lui (il faut l'autorisation d'Accessibilité).
    pub walk_on_dock: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_count: 1,
            default_pet: None,
            frontmost_only: true,
            collisions: false,
            size: 1.0,
            walk_on_dock: false,
        }
    }
}

impl Settings {
    /// Rang de la taille dans [`SIZES`] (la plus proche, pour une valeur écrite à la main).
    #[must_use]
    pub fn size_index(&self) -> usize {
        SIZES
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| (*a - self.size).abs().total_cmp(&(*b - self.size).abs()))
            .map_or(1, |(i, _)| i)
    }

    /// Taille à appliquer : une valeur de [`SIZES`].
    #[must_use]
    pub fn size_factor(&self) -> f32 {
        SIZES[self.size_index()]
    }

    #[must_use]
    pub fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text).unwrap_or_else(|e| {
                log::warn!(
                    "réglages illisibles ({}), valeurs par défaut : {e}",
                    path.display()
                );
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub fn save(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        match toml::to_string_pretty(self) {
            Ok(text) => {
                if let Err(e) = std::fs::write(path, text) {
                    log::warn!("impossible d'enregistrer les réglages : {e}");
                }
            }
            Err(e) => log::warn!("réglages non sérialisables : {e}"),
        }
    }
}

/// Dossiers de l'application (sur macOS : `~/Library/Application Support/…`).
#[derive(Clone, Debug)]
pub struct Paths {
    pub settings: PathBuf,
    pub pets: PathBuf,
}

impl Paths {
    #[must_use]
    pub fn new() -> Option<Self> {
        let paths = Self::of(&directories::ProjectDirs::from(
            "com",
            "alejandrosanmartinlamas",
            "Mouflon",
        )?);
        // Mouflon s'appelait Bergerie : on reprend ses animaux et ses réglages.
        if let Some(old) =
            directories::ProjectDirs::from("com", "alejandrosanmartinlamas", "Bergerie")
        {
            paths.migrate_from(&Self::of(&old));
        }
        Some(paths)
    }

    fn of(dirs: &directories::ProjectDirs) -> Self {
        Self {
            settings: dirs.config_dir().join("settings.toml"),
            pets: dirs.data_dir().join("pets"),
        }
    }

    /// Reprend les animaux installés et les réglages d'un ancien emplacement,
    /// une seule fois (rien n'est fait si le nouvel emplacement existe déjà).
    /// L'animal préféré n'est pas repris : la mascotte de Mouflon accueille
    /// au premier lancement.
    fn migrate_from(&self, old: &Self) {
        if old.pets.is_dir() && !self.pets.exists() {
            match crate::library::copy_dir(&old.pets, &self.pets) {
                Ok(()) => log::info!("animaux repris de {}", old.pets.display()),
                Err(e) => log::warn!("impossible de reprendre {} : {e}", old.pets.display()),
            }
        }
        if old.settings.is_file() && !self.settings.exists() {
            let settings = Settings {
                default_pet: None,
                ..Settings::load(&old.settings)
            };
            settings.save(&self.settings);
            log::info!("réglages repris de {}", old.settings.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_snaps_to_a_menu_step() {
        let with = |size: f32| Settings {
            size,
            ..Settings::default()
        };
        assert_eq!(Settings::default().size_index(), 1);
        assert_eq!(with(2.0).size_factor(), 2.0);
        // Une valeur écrite à la main prend la taille du menu la plus proche.
        assert_eq!(with(1.4).size_factor(), 1.5);
        assert_eq!(with(0.1).size_index(), 0);
        // Les anciens réglages, sans taille, gardent la taille normale.
        let old: Settings = toml::from_str("collisions = true").unwrap();
        assert_eq!(old.size_factor(), 1.0);
    }

    #[test]
    fn takes_over_the_old_pets_and_settings_once() {
        let root = std::env::temp_dir().join(format!("mouflon-migration-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let old = Paths {
            settings: root.join("ancien/settings.toml"),
            pets: root.join("ancien/pets"),
        };
        let new = Paths {
            settings: root.join("nouveau/settings.toml"),
            pets: root.join("nouveau/pets"),
        };
        std::fs::create_dir_all(old.pets.join("esheep")).unwrap();
        std::fs::write(old.pets.join("esheep/animations.xml"), "<animations/>").unwrap();
        Settings {
            start_count: 3,
            default_pet: Some("esheep".into()),
            frontmost_only: false,
            collisions: true,
            size: 1.0,
            walk_on_dock: false,
        }
        .save(&old.settings);

        new.migrate_from(&old);
        assert!(new.pets.join("esheep/animations.xml").is_file());
        let settings = Settings::load(&new.settings);
        assert_eq!(settings.start_count, 3);
        assert!(settings.collisions && !settings.frontmost_only);
        assert_eq!(
            settings.default_pet, None,
            "la mascotte de Mouflon accueille"
        );

        // Une seule fois : un réglage changé depuis n'est pas écrasé.
        Settings::default().save(&new.settings);
        new.migrate_from(&old);
        assert_eq!(Settings::load(&new.settings), Settings::default());
        let _ = std::fs::remove_dir_all(&root);
    }
}
