//! Réglages persistants et emplacements des fichiers.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_count: 1,
            default_pet: None,
            frontmost_only: true,
            collisions: false,
        }
    }
}

impl Settings {
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
