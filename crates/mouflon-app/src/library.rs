//! Bibliothèque d'animaux : le mannequin intégré, les animaux livrés dans
//! l'app (`Mouflon.app/Contents/Resources/pets`) et ceux du dossier de
//! l'utilisateur.
//!
//! Dans le dossier `pets/` : un sous-dossier par animal au format Mouflon
//! (`pet.toml` + dessin), un dossier desktopPet (`animations.xml`), ou
//! directement des fichiers `.xml` desktopPet. Le dossier est surveillé : un
//! animal qu'on y dépose apparaît dans le menu sans relancer l'app.

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::SystemTime;

use mouflon_core::{Appearance, LoadError, PetDef, load_pet, parse_pet_toml};
use mouflon_render::PetArt;

/// Mascottes de Mouflon, par ordre de préférence au premier lancement.
const MASCOTS: [&str; 3] = ["poete", "pelote", "lainou"];

#[derive(Clone, Debug)]
pub struct Entry {
    /// L'animal à la taille choisie dans le menu.
    pub def: Arc<PetDef>,
    /// L'animal à la taille prévue par son fichier.
    base: Arc<PetDef>,
    /// Dessin préparé à la première apparition : décoder des centaines
    /// d'images prend un moment, et elles n'occupent la mémoire que si
    /// l'animal sert.
    art: Arc<OnceLock<Option<Arc<PetArt>>>>,
}

impl Entry {
    fn new(def: PetDef, size: f32) -> Self {
        let base = Arc::new(def);
        Self {
            def: Arc::new(base.scaled(size)),
            base,
            art: Arc::new(OnceLock::new()),
        }
    }

    /// Met l'animal à une autre taille ; son dessin sera refait à la demande.
    fn resize(&mut self, size: f32) {
        self.def = Arc::new(self.base.scaled(size));
        self.art = Arc::new(OnceLock::new());
    }

    /// Le dessin de l'animal a-t-il déjà été préparé (ou trouvé illisible) ?
    #[must_use]
    pub fn art_ready(&self) -> bool {
        self.art.get().is_some()
    }

    /// Dessin de l'animal, préparé à la première demande ; `None` s'il est
    /// illisible (l'erreur est notée au journal une fois).
    pub fn art(&self) -> Option<Arc<PetArt>> {
        self.art
            .get_or_init(|| {
                let started = std::time::Instant::now();
                match PetArt::load(&self.def) {
                    Ok(art) => {
                        log::info!(
                            "dessin de {} prêt en {} ms",
                            self.def.meta.name,
                            started.elapsed().as_millis()
                        );
                        Some(Arc::new(art))
                    }
                    Err(e) => {
                        log::error!("{} : dessin invalide : {e}", self.def.meta.name);
                        None
                    }
                }
            })
            .clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Library {
    pub pets: Vec<Entry>,
    /// Dossier des animaux de l'utilisateur.
    dir: Option<PathBuf>,
    /// Animaux livrés dans l'app, dans l'ordre : le premier est l'animal par défaut.
    bundled: Vec<String>,
    /// Animaux passés en argument, rechargés avec le reste.
    extra: Vec<PathBuf>,
    /// Taille des animaux, par rapport à celle prévue par chacun.
    size: f32,
    /// État du dossier au dernier chargement.
    stamp: Stamp,
}

/// Noms et dates de modification du dossier des animaux et de ses sous-dossiers.
pub type Stamp = Vec<(PathBuf, Option<SystemTime>)>;

pub fn stamp(dir: &Path) -> Stamp {
    let mut out = Vec::new();
    let mut visit = |path: &Path| {
        out.push((
            path.to_path_buf(),
            std::fs::metadata(path).and_then(|m| m.modified()).ok(),
        ));
    };
    visit(dir);
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut paths: Vec<PathBuf> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            visit(&path);
            if path.is_dir()
                && let Ok(inner) = std::fs::read_dir(&path)
            {
                let mut files: Vec<PathBuf> =
                    inner.filter_map(Result::ok).map(|e| e.path()).collect();
                files.sort();
                for file in files {
                    visit(&file);
                }
            }
        }
    }
    out
}

/// Animaux livrés avec Mouflon : ceux de l'app (`Mouflon.app/Contents/Resources/pets`)
/// ou, lancé depuis le dépôt (`cargo run`), ceux de `pets/` et du dossier `animaux/`.
fn bundled_dirs() -> Vec<PathBuf> {
    let Some(exe) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.canonicalize().ok())
    else {
        return Vec::new();
    };
    if let Some(contents) = exe.parent().and_then(Path::parent) {
        let dir = contents.join("Resources").join("pets");
        if dir.is_dir() {
            return vec![dir];
        }
    }
    if cfg!(test) {
        return Vec::new();
    }
    // …/target/[cible/]profil/mouflon : le dépôt est quelques dossiers plus haut.
    exe.ancestors()
        .take(6)
        .find(|dir| dir.join("Cargo.toml").is_file() && dir.join("pets").is_dir())
        .map(|repo| {
            ["pets", "animaux"]
                .into_iter()
                .map(|name| repo.join(name))
                .filter(|dir| dir.is_dir())
                .collect()
        })
        .unwrap_or_default()
}

/// Fichier de description d'un animal : `pet.toml`, ou à défaut `animations.xml`.
fn pet_file(path: &Path) -> PathBuf {
    if path.is_dir() {
        let toml = path.join("pet.toml");
        if toml.is_file() {
            toml
        } else {
            path.join("animations.xml")
        }
    } else {
        path.to_path_buf()
    }
}

/// Copie un dossier et tout son contenu.
pub fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn builtin_mannequin() -> Result<PetDef, LoadError> {
    const TOML: &str = include_str!("../../../pets/mannequin/pet.toml");
    const SVG: &[u8] = include_bytes!("../../../pets/mannequin/mannequin.svg");
    parse_pet_toml(TOML, |name: &str| {
        if name == "mannequin.svg" {
            Ok(SVG.to_vec())
        } else {
            Err(LoadError::Format(format!(
                "ressource intégrée inconnue : {name}"
            )))
        }
    })
}

impl Library {
    #[must_use]
    pub fn load(user_dir: Option<&Path>, size: f32) -> Self {
        let mut library = Self {
            dir: user_dir.map(Path::to_path_buf),
            size,
            ..Self::default()
        };
        library.reload();
        library
    }

    /// Met tous les animaux à une autre taille (1 : celle prévue par chacun).
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        for entry in &mut self.pets {
            entry.resize(size);
        }
    }

    /// L'animal de la bibliothèque dont `def` est une version (même
    /// identifiant), à la taille courante.
    #[must_use]
    pub fn current(&self, def: &PetDef) -> Option<&Entry> {
        self.pets.iter().find(|e| e.def.meta.id == def.meta.id)
    }

    /// Recharge tout : l'animal intégré, le dossier de l'utilisateur et les
    /// animaux passés en argument.
    pub fn reload(&mut self) {
        self.pets.clear();
        match builtin_mannequin() {
            Ok(def) => self.add(def, "mannequin intégré"),
            Err(e) => log::error!("animal intégré invalide : {e}"),
        }
        let before: Vec<String> = self.pets.iter().map(|e| e.def.meta.id.clone()).collect();
        for dir in bundled_dirs() {
            self.scan(&dir);
        }
        self.bundled = self
            .pets
            .iter()
            .map(|e| e.def.meta.id.clone())
            .filter(|id| !before.contains(id))
            .collect();
        if let Some(dir) = self.dir.clone() {
            self.scan(&dir);
            self.stamp = stamp(&dir);
        }
        for path in self.extra.clone() {
            self.load_file(&pet_file(&path));
        }
    }

    /// Dossier des animaux de l'utilisateur (surveillé pendant que l'app tourne).
    #[must_use]
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// Le dossier des animaux a-t-il changé depuis le dernier chargement,
    /// d'après ce relevé ?
    #[must_use]
    pub fn changed_since(&self, stamp: &Stamp) -> bool {
        self.dir.is_some() && *stamp != self.stamp
    }

    fn add(&mut self, def: PetDef, origin: &str) {
        let entry = Entry::new(def, self.size);
        // Un dessin vectoriel se lit vite : on le vérifie tout de suite. Les
        // images attendent la première apparition (leurs en-têtes sont déjà vérifiés).
        if matches!(entry.def.appearance, Appearance::Rig { .. }) && entry.art().is_none() {
            log::warn!("{origin} : animal ignoré");
            return;
        }
        log::info!("animal chargé : {} ({origin})", entry.def.meta.name);
        self.pets.retain(|e| e.def.meta.id != entry.def.meta.id);
        self.pets.push(entry);
    }

    fn load_file(&mut self, file: &Path) -> Option<String> {
        match load_pet(file) {
            Ok(def) => {
                let id = def.meta.id.clone();
                self.add(def, &file.display().to_string());
                self.pets.iter().any(|e| e.def.meta.id == id).then_some(id)
            }
            Err(e) => {
                log::error!("{} : {e}", file.display());
                None
            }
        }
    }

    /// Charge un animal désigné à la main (fichier `pet.toml`, fichier XML
    /// desktopPet ou dossier qui en contient un) ; renvoie son identifiant.
    pub fn load_path(&mut self, path: &Path) -> Option<String> {
        self.extra.push(path.to_path_buf());
        self.load_file(&pet_file(path))
    }

    /// Installe un animal (fichier ou dossier déposé sur un animal) dans le
    /// dossier de l'utilisateur, puis recharge ; renvoie son identifiant.
    pub fn install(&mut self, source: &Path) -> Result<String, String> {
        let dir = self
            .dir
            .clone()
            .ok_or("pas de dossier des animaux sur ce système")?;
        let file = pet_file(source);
        // Vérifier avant de copier : on n'installe que ce qui se charge.
        let def = load_pet(&file).map_err(|e| e.to_string())?;
        let name = |p: &Path| {
            p.file_name()
                .map(|n| n.to_os_string())
                .ok_or("nom de fichier invalide")
        };
        let generic = file
            .file_name()
            .is_some_and(|n| n == "animations.xml" || n == "pet.toml");
        let folder = if source.is_dir() {
            Some(source.to_path_buf())
        } else if generic {
            // `animations.xml` ou `pet.toml` seul : on prend son dossier (et ses dessins).
            source.parent().map(Path::to_path_buf)
        } else {
            None
        };
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        match folder {
            Some(folder) => {
                copy_dir(&folder, &dir.join(name(&folder)?)).map_err(|e| e.to_string())?;
            }
            None => {
                std::fs::copy(source, dir.join(name(source)?)).map_err(|e| e.to_string())?;
            }
        }
        self.reload();
        Ok(def.meta.id)
    }

    fn scan(&mut self, dir: &Path) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            let is_pet = path.is_dir()
                || path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("xml"));
            let file = pet_file(&path);
            if !is_pet || !file.is_file() {
                continue;
            }
            match load_pet(&file) {
                Ok(def) => self.add(def, &file.display().to_string()),
                Err(e) => log::warn!("{} : {e}", file.display()),
            }
        }
    }

    /// Indice de l'animal d'identifiant donné, ou du premier.
    #[must_use]
    pub fn index_of(&self, id: &str) -> usize {
        self.pets
            .iter()
            .position(|e| e.def.meta.id == id)
            .unwrap_or(0)
    }

    /// L'animal préféré s'il existe, sinon la mascotte de Mouflon (Poete
    /// dans les builds qui le contiennent, puis Pelote, puis Lainou), sinon
    /// le premier animal livré dans l'app, sinon le mannequin.
    #[must_use]
    pub fn default_index(&self, preferred: Option<&str>) -> usize {
        let find = |id: &str| self.pets.iter().position(|e| e.def.meta.id == id);
        preferred
            .and_then(find)
            .or_else(|| MASCOTS.iter().find_map(|id| find(id)))
            .or_else(|| self.bundled.iter().find_map(|id| find(id)))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_a_pet_folder_and_notices_changes() {
        let root = std::env::temp_dir().join(format!("mouflon-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let pets = root.join("pets");
        let source = root.join("source/robot");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(
            source.join("pet.toml"),
            include_str!("../../../pets/mannequin/pet.toml")
                .replace("id = \"mannequin\"", "id = \"robot\""),
        )
        .unwrap();
        std::fs::write(
            source.join("mannequin.svg"),
            include_bytes!("../../../pets/mannequin/mannequin.svg"),
        )
        .unwrap();

        let mut library = Library::load(Some(&pets), 1.0);
        assert_eq!(library.pets.len(), 1);
        assert_eq!(library.dir(), Some(pets.as_path()));
        assert!(!library.changed_since(&stamp(&pets)));
        let id = library.install(&source.join("pet.toml")).unwrap();
        assert_eq!(id, "robot");
        assert!(pets.join("robot/mannequin.svg").is_file(), "le dessin suit");
        assert_eq!(library.pets.len(), 2);
        assert!(
            !library.changed_since(&stamp(&pets)),
            "un animal installé ne recharge pas tout"
        );

        std::fs::create_dir_all(pets.join("vide")).unwrap();
        assert!(
            library.changed_since(&stamp(&pets)),
            "un nouveau dossier est remarqué"
        );

        // Plus grand : le dessin et le corps suivent, l'original ne change pas.
        let height = |library: &Library| library.pets[0].art().unwrap().height;
        let normal = height(&library);
        let width = library.pets[0].def.physics.width;
        library.set_size(1.5);
        assert!((height(&library) - normal * 1.5).abs() < 0.01);
        assert!((library.pets[0].def.physics.width - width * 1.5).abs() < 0.01);
        library.set_size(1.0);
        assert!((height(&library) - normal).abs() < 0.01);
        let _ = std::fs::remove_dir_all(&root);
    }
}
