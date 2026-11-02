//! Mouflon : un gestionnaire de compagnons de bureau, léger et beau.
//!
//! © 2026 Alejandro San Martin Lamas — MIT OR Apache-2.0.
#![forbid(unsafe_code)]
// Le binaire n'a que des tests unitaires : le reste paraît inutilisé dans ce contexte.
#![cfg_attr(test, allow(dead_code))]

mod app;
mod display;
mod gpu;
mod helper;
mod i18n;
mod icon;
mod keys;
mod library;
mod menu;
mod quick;
mod scene;
mod screens;
mod settings;
mod settings_window;
mod stats;
mod survey;
mod watch;

use winit::event_loop::EventLoop;

/// L'allocateur du système, qui compte les allocations pendant les essais
/// (une lecture atomique par allocation le reste du temps).
#[global_allocator]
static ALLOCATOR: mouflon_platform::CountingAlloc = mouflon_platform::CountingAlloc;

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("warn,mouflon=info,mouflon_core=info,mouflon_render=info"),
    )
    .format_timestamp_millis()
    .init();

    // Le processus auxiliaire du relevé (voir `helper`) : rien d'autre.
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == survey::HELPER_ARG)
    {
        helper::run();
        return;
    }

    let paths = settings::Paths::new();
    // Premier lancement : pas encore de réglages enregistrés.
    let first_launch = paths.as_ref().is_some_and(|p| !p.settings.exists());
    let mut settings = paths
        .as_ref()
        .map(|p| settings::Settings::load(&p.settings))
        .unwrap_or_default();
    if first_launch {
        // La taille qui va à l'écran, gardée ensuite comme un choix.
        settings.size = settings::default_size(&mouflon_platform::screens());
        if let Some(paths) = &paths {
            settings.save(&paths.settings);
        }
        log::info!("premier lancement : taille × {}", settings.size);
    }
    // Les images prêtes se projettent en mémoire depuis le disque.
    let images = paths.as_ref().map(|p| {
        mouflon_render::cache::Store::new(&p.images).opened_with(mouflon_platform::map_file)
    });
    let mut library = library::Library::load(
        paths.as_ref().map(|p| p.pets.as_path()),
        images,
        settings.size_factor(),
    );
    // `mouflon chemin/vers/animal…` : charge ces animaux en plus et lance le
    // premier (pratique pour essayer un animal sans l'installer).
    let mut first = None;
    for arg in std::env::args_os().skip(1) {
        let id = library.load_path(std::path::Path::new(&arg));
        first = first.or(id);
    }
    if let Some(id) = first {
        settings.default_pet = Some(id);
    }
    if library.pets.is_empty() {
        log::error!("aucun animal disponible");
        return;
    }

    let mut builder = EventLoop::<app::UserEvent>::with_user_event();
    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};
        // Pas d'icône dans le Dock ni dans ⌘-Tab : tout passe par la barre des menus.
        builder
            .with_activation_policy(ActivationPolicy::Accessory)
            .with_default_menu(false);
    }
    let event_loop = match builder.build() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            log::error!("impossible de démarrer : {e}");
            return;
        }
    };
    let proxy = event_loop.create_proxy();
    let mut app = app::App::new(settings, paths, library, proxy);
    if let Err(e) = event_loop.run_app(&mut app) {
        log::error!("{e}");
    }
    app.shutdown();
    // On sort sans détruire les fenêtres des écrans une à une : ce sont des
    // panneaux dont la classe a changé après leur création, et AppKit, en
    // les libérant, échoue à défaire certaines de ses observations internes
    // (NSRangeException, l'app « quitte de manière imprévue »). Elles sont
    // toutes gardées (`keep_pet_window`) ; le système reprend tout à la
    // sortie du processus.
    std::process::exit(0);
}
