//! Les fenêtres des écrans, sur le fil principal : une fenêtre transparente
//! par écran, créée une fois et jamais déplacée, où la scène se pose (voir
//! [`crate::scene`]). Seul le fil principal touche aux fenêtres : il les
//! fait, leur pose la racine des calques de la scène, et décide si elles
//! prennent la souris — ce que l'affichage lui dit à chaque changement
//! (au-dessus d'un animal, oui ; ailleurs, la souris passe aux apps en dessous).

use std::sync::Arc;

use mouflon_core::{Rect, Screen, Vec2};
use mouflon_platform as platform;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition};
use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;
use winit::window::{Theme, Window, WindowId, WindowLevel};

use crate::scene::{Overlay, ScreenInfo, Surface};

/// La fenêtre d'un écran.
pub struct ScreenWindow {
    pub info: ScreenInfo,
    pub window: Arc<Window>,
    surface: Surface,
    /// Prend-elle la souris en ce moment ?
    hittest: bool,
}

/// Les fenêtres de tous les écrans.
pub struct ScreenWindows {
    windows: Vec<ScreenWindow>,
    /// Calques Core Animation quand le système en a (sinon le GPU).
    layers_wanted: bool,
    /// Rien que des calques : l'affichage tourne sur son propre fil, où le
    /// GPU n'irait pas ; un écran sans calques resterait vide.
    layers_only: bool,
    next_key: u64,
}

impl Default for ScreenWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenWindows {
    #[must_use]
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            // `MOUFLON_GPU=1` : le GPU même sur Mac (pour comparer).
            layers_wanted: !std::env::var("MOUFLON_GPU").is_ok_and(|v| !v.is_empty() && v != "0"),
            layers_only: false,
            next_key: 1,
        }
    }

    /// Toutes les fenêtres montrent la scène en calques (et il y en a).
    #[must_use]
    pub fn all_layers(&self) -> bool {
        !self.windows.is_empty()
            && self
                .windows
                .iter()
                .all(|w| matches!(w.surface, Surface::Layers(_)))
    }

    /// Désormais, rien que des calques (voir `layers_only`).
    pub fn set_layers_only(&mut self, on: bool) {
        self.layers_only = on;
    }

    /// Une fenêtre par écran, refaites quand les écrans changent de place ou
    /// de taille. `true` si elles ont changé.
    pub fn sync(&mut self, event_loop: &ActiveEventLoop, screens: &[Screen]) -> bool {
        let same = self.windows.len() == screens.len()
            && self
                .windows
                .iter()
                .all(|w| screens.iter().any(|s| s.frame == w.info.rect));
        if same {
            return false;
        }
        for old in self.windows.drain(..) {
            platform::keep_pet_window(&*old.window);
        }
        for screen in screens {
            match self.open(event_loop, screen.frame) {
                Ok(window) => self.windows.push(window),
                Err(e) => log::error!("fenêtre de l'écran impossible : {e}"),
            }
        }
        true
    }

    fn open(&mut self, event_loop: &ActiveEventLoop, rect: Rect) -> Result<ScreenWindow, String> {
        let attrs = Window::default_attributes()
            .with_title("Mouflon")
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(LogicalSize::new(f64::from(rect.w), f64::from(rect.h)))
            .with_position(LogicalPosition::new(f64::from(rect.x), f64::from(rect.y)))
            .with_active(false);
        #[cfg(target_os = "macos")]
        let attrs = {
            use winit::platform::macos::WindowAttributesExtMacOS;
            attrs.with_has_shadow(false).with_accepts_first_mouse(true)
        };
        let window = Arc::new(event_loop.create_window(attrs).map_err(|e| e.to_string())?);
        platform::prepare_pet_window(&*window);
        // Elle laisse passer la souris, sauf au-dessus d'un animal.
        let _ = window.set_cursor_hittest(false);
        let layers = self
            .layers_wanted
            .then(|| platform::attach_scene(&*window))
            .flatten();
        let surface = match layers {
            Some(root) => Surface::Layers(root),
            None if self.layers_only => {
                platform::keep_pet_window(&*window);
                return Err("calques Core Animation indisponibles".to_owned());
            }
            None => Surface::Gpu(Arc::clone(&window)),
        };
        let key = self.next_key;
        self.next_key += 1;
        Ok(ScreenWindow {
            info: ScreenInfo {
                key,
                rect,
                scale: window.scale_factor(),
                hz: window_hz(&window),
            },
            window,
            surface,
            hittest: false,
        })
    }

    /// Ce que la scène doit savoir des fenêtres.
    #[must_use]
    pub fn overlays(&self) -> Vec<Overlay> {
        self.windows
            .iter()
            .map(|w| Overlay {
                info: w.info,
                surface: w.surface.clone(),
            })
            .collect()
    }

    /// La fenêtre d'écran `id`, s'il en est une.
    #[must_use]
    pub fn get(&self, id: WindowId) -> Option<&ScreenWindow> {
        self.windows.iter().find(|w| w.window.id() == id)
    }

    /// La fenêtre `key` prend la souris (au-dessus d'un animal) ou la
    /// laisse passer.
    pub fn set_hittest(&mut self, key: u64, on: bool) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.info.key == key)
            && w.hittest != on
        {
            let _ = w.window.set_cursor_hittest(on);
            w.hittest = on;
        }
    }

    /// L'échelle de la fenêtre `id` a changé : `true` si c'est celle d'un écran.
    pub fn scale_changed(&mut self, id: WindowId, scale: f64) -> bool {
        match self.windows.iter_mut().find(|w| w.window.id() == id) {
            Some(w) if w.info.scale != scale => {
                w.info.scale = scale;
                true
            }
            _ => false,
        }
    }

    /// La fréquence de chaque écran, relue : `true` si l'une a changé.
    pub fn refresh_hz(&mut self) -> bool {
        let mut changed = false;
        for w in &mut self.windows {
            let hz = window_hz(&w.window);
            if (hz - w.info.hz).abs() > 0.5 {
                w.info.hz = hz;
                changed = true;
            }
        }
        changed
    }

    /// Le thème du système, tel que le voient les fenêtres.
    #[must_use]
    pub fn dark(&self) -> Option<bool> {
        let theme = self.windows.first()?.window.theme()?;
        Some(theme == Theme::Dark)
    }

    /// La position d'un événement de la fenêtre `id`, dans le repère commun.
    #[must_use]
    pub fn to_global(&self, id: WindowId, position: PhysicalPosition<f64>) -> Option<Vec2> {
        let w = self.get(id)?;
        let local = position.to_logical::<f64>(w.info.scale);
        Some(Vec2::new(
            w.info.rect.x + local.x as f32,
            w.info.rect.y + local.y as f32,
        ))
    }

    /// Avant de quitter : les fenêtres ne seront jamais libérées (voir
    /// `platform::keep_pet_window`).
    pub fn shutdown(&self) {
        for w in &self.windows {
            platform::keep_pet_window(&*w.window);
        }
    }
}

/// Fréquence de l'écran d'une fenêtre, en images par seconde : celle que
/// donne macOS, sinon celle de winit, sinon 60.
fn window_hz(window: &Window) -> f64 {
    platform::window_max_fps(window)
        .or_else(|| {
            window
                .current_monitor()
                .as_ref()
                .and_then(MonitorHandle::refresh_rate_millihertz)
                .map(|mhz| f64::from(mhz) / 1000.0)
        })
        .map_or(60.0, |hz| hz.clamp(30.0, 240.0))
}
