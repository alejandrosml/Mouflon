//! Implémentation macOS.

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSEvent, NSScreen, NSView, NSWindowCollectionBehavior, NSWorkspace};
use objc2_core_foundation::{
    CFArray, CFBoolean, CFDictionary, CFNumber, CFRetained, CFString, CFType, CGRect,
};
use objc2_core_graphics::{
    CGRectMakeWithDictionaryRepresentation, CGWindowListCopyWindowInfo, CGWindowListOption,
    kCGNullWindowID, kCGWindowAlpha, kCGWindowBounds, kCGWindowIsOnscreen, kCGWindowLayer,
    kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID,
};
use objc2_foundation::{NSLocale, NSString, NSURL};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use crate::{OsRect, OsScreen, OsWindow, current_pid};

/// Taille minimale d'une fenêtre pour qu'on la considère (élimine les
/// fenêtres techniques invisibles que certaines apps laissent à l'écran).
const MIN_SIZE: f64 = 48.0;

fn cocoa_to_top_left(rect: CGRect, primary_height: f64) -> OsRect {
    OsRect {
        x: rect.origin.x,
        y: primary_height - (rect.origin.y + rect.size.height),
        w: rect.size.width,
        h: rect.size.height,
    }
}

fn primary_height(mtm: MainThreadMarker) -> Option<f64> {
    NSScreen::screens(mtm)
        .iter()
        .next()
        .map(|s| s.frame().size.height)
}

/// Écrans connectés. Doit être appelé depuis le fil principal (sinon : liste vide).
#[must_use]
pub fn screens() -> Vec<OsScreen> {
    let Some(mtm) = MainThreadMarker::new() else {
        log::warn!("screens() appelé hors du fil principal");
        return Vec::new();
    };
    let Some(primary) = primary_height(mtm) else {
        return Vec::new();
    };
    NSScreen::screens(mtm)
        .iter()
        .map(|screen| OsScreen {
            frame: cocoa_to_top_left(screen.frame(), primary),
            visible: cocoa_to_top_left(screen.visibleFrame(), primary),
            scale: screen.backingScaleFactor(),
        })
        .collect()
}

fn number(dict: &CFDictionary<CFString, CFType>, key: &CFString) -> Option<f64> {
    dict.get(key)?.downcast::<CFNumber>().ok()?.as_f64()
}

fn string(dict: &CFDictionary<CFString, CFType>, key: &CFString) -> Option<String> {
    Some(dict.get(key)?.downcast::<CFString>().ok()?.to_string())
}

/// Fenêtres visibles des autres applications, de la plus en avant à la plus
/// en arrière, tous niveaux confondus (voir [`OsWindow::is_walkable`]).
#[must_use]
pub fn windows() -> Vec<OsWindow> {
    let options =
        CGWindowListOption::OptionOnScreenOnly | CGWindowListOption::ExcludeDesktopElements;
    let Some(list) = CGWindowListCopyWindowInfo(options, kCGNullWindowID) else {
        return Vec::new();
    };
    // SAFETY: la documentation de CGWindowListCopyWindowInfo garantit un
    // tableau de dictionnaires dont les clés sont des CFString.
    let list: CFRetained<CFArray<CFDictionary<CFString, CFType>>> =
        unsafe { CFRetained::cast_unchecked(list) };
    // SAFETY: ces constantes sont des CFString immuables exportées par
    // CoreGraphics, valides pendant toute la vie du processus.
    let (k_bounds, k_layer, k_number, k_pid, k_owner, k_alpha, k_onscreen) = unsafe {
        (
            kCGWindowBounds,
            kCGWindowLayer,
            kCGWindowNumber,
            kCGWindowOwnerPID,
            kCGWindowOwnerName,
            kCGWindowAlpha,
            kCGWindowIsOnscreen,
        )
    };
    let me = current_pid();
    let mut out = Vec::new();
    for info in list.iter() {
        let Some(layer) = number(&info, k_layer) else {
            continue;
        };
        let pid = number(&info, k_pid).unwrap_or(-1.0) as i32;
        if pid == me {
            continue;
        }
        if number(&info, k_alpha).is_some_and(|alpha| alpha < 0.05) {
            continue;
        }
        let onscreen = info
            .get(k_onscreen)
            .and_then(|v| v.downcast::<CFBoolean>().ok())
            .is_none_or(|b| b.as_bool());
        if !onscreen {
            continue;
        }
        let Some(bounds) = info
            .get(k_bounds)
            .and_then(|v| v.downcast::<CFDictionary>().ok())
        else {
            continue;
        };
        let mut rect = CGRect::default();
        // SAFETY: `bounds` est le dictionnaire de rectangle fourni par
        // CoreGraphics et `rect` un pointeur valide vers une variable locale.
        if !unsafe { CGRectMakeWithDictionaryRepresentation(Some(&bounds), &mut rect) } {
            continue;
        }
        if rect.size.width < MIN_SIZE || rect.size.height < MIN_SIZE {
            continue;
        }
        let id = number(&info, k_number).unwrap_or(0.0) as u64;
        out.push(OsWindow {
            id,
            pid,
            frame: OsRect {
                x: rect.origin.x,
                y: rect.origin.y,
                w: rect.size.width,
                h: rect.size.height,
            },
            layer: layer as i32,
            owner: string(&info, k_owner).unwrap_or_default(),
        });
    }
    out
}

/// Processus de l'application au premier plan.
#[must_use]
pub fn frontmost_pid() -> Option<i32> {
    NSWorkspace::sharedWorkspace()
        .frontmostApplication()
        .map(|app| app.processIdentifier())
}

/// Ouvre un dossier dans le Finder. Passe par NSWorkspace plutôt que par la
/// commande `open` : c'est ce que permet le bac à sable de l'App Store.
pub fn open_folder(path: &std::path::Path) -> bool {
    let url = NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()));
    NSWorkspace::sharedWorkspace().openURL(&url)
}

/// Position du curseur dans le repère commun.
#[must_use]
pub fn cursor_position() -> Option<(f64, f64)> {
    let mtm = MainThreadMarker::new()?;
    let primary = primary_height(mtm)?;
    let p = NSEvent::mouseLocation();
    Some((p.x, primary - p.y))
}

/// Règle une fenêtre d'animal : visible sur tous les Spaces et par-dessus les
/// apps en plein écran, ignorée par ⌘-Tab et Mission Control, jamais masquée
/// quand Mouflon passe en arrière-plan.
pub fn prepare_pet_window(window: &impl HasWindowHandle) {
    let Ok(handle) = window.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return;
    };
    if MainThreadMarker::new().is_none() {
        return;
    }
    // SAFETY: winit garantit que `ns_view` pointe vers une NSView vivante tant
    // que la fenêtre existe ; on est sur le fil principal (vérifié ci-dessus)
    // et on ne garde qu'une référence comptée pendant l'appel.
    let view: Retained<NSView> =
        match unsafe { Retained::retain(appkit.ns_view.as_ptr().cast::<NSView>()) } {
            Some(view) => view,
            None => return,
        };
    let Some(ns_window) = view.window() else {
        return;
    };
    ns_window.setCollectionBehavior(
        NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::IgnoresCycle
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    ns_window.setHidesOnDeactivate(false);
    ns_window.setHasShadow(false);
}

/// Langues de l'utilisateur, dans son ordre de préférence (codes BCP 47 :
/// « fr-FR », « es-419 »…), langue choisie pour Mouflon seul comprise.
#[must_use]
pub fn preferred_languages() -> Vec<String> {
    NSLocale::preferredLanguages()
        .iter()
        .map(|tag| tag.to_string())
        .collect()
}
