//! Implémentation macOS.

mod clock;
mod sound;
mod ui;

pub use clock::FrameClock;
pub use sound::{play_sound, warm_up_sounds};
pub use ui::{close_note, show_about, show_note};

use std::ptr::NonNull;

use objc2::ClassType;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSEvent, NSPanel, NSRunningApplication, NSScreen, NSView, NSWindow, NSWindowCollectionBehavior,
    NSWindowStyleMask, NSWorkspace,
};
use objc2_application_services::{
    AXError, AXIsProcessTrusted, AXIsProcessTrustedWithOptions, AXUIElement, AXValue, AXValueType,
    kAXTrustedCheckOptionPrompt,
};
use objc2_core_foundation::{
    CFArray, CFBoolean, CFDictionary, CFNumber, CFRetained, CFString, CFType, CGPoint, CGRect,
    CGSize,
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

/// Images par seconde que l'écran d'une fenêtre sait afficher, au plus
/// (120 sur un écran ProMotion). `None` si on ne sait pas.
#[must_use]
pub fn window_max_fps(window: &impl HasWindowHandle) -> Option<f64> {
    let handle = window.window_handle().ok()?;
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return None;
    };
    MainThreadMarker::new()?;
    // `maximumFramesPerSecond` date de macOS 12.
    if !objc2::available!(macos = 12.0) {
        return None;
    }
    // SAFETY: winit garantit que `ns_view` pointe vers une NSView vivante tant
    // que la fenêtre existe ; on est sur le fil principal (vérifié ci-dessus)
    // et on ne garde qu'une référence comptée pendant l'appel.
    let view: Retained<NSView> =
        unsafe { Retained::retain(appkit.ns_view.as_ptr().cast::<NSView>()) }?;
    let fps = view.window()?.screen()?.maximumFramesPerSecond();
    (fps > 0).then_some(fps as f64)
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
    make_non_activating(&ns_window);
    ns_window.setCollectionBehavior(
        NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::IgnoresCycle
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    ns_window.setHidesOnDeactivate(false);
    ns_window.setHasShadow(false);
}

/// Fait de la fenêtre d'un animal un panneau « non activant » : un clic ou
/// un glisser ne fait pas passer Mouflon au premier plan, et l'app où l'on
/// travaille garde la main (et le clavier).
///
/// winit crée des `NSWindow` ; seul un `NSPanel` peut porter le style
/// `NonactivatingPanel`. On change donc la classe de la fenêtre, comme le
/// fait le greffon tauri-nspanel. C'est sans danger si les deux classes ont
/// la même taille d'instance, ce que l'on vérifie : sinon, la fenêtre reste
/// une fenêtre ordinaire (qui active Mouflon quand on attrape l'animal).
fn make_non_activating(window: &NSWindow) {
    let current = window.class();
    let panel = NSPanel::class();
    if current != panel {
        if current.instance_size() != panel.instance_size() {
            log::warn!(
                "fenêtre d'animal : {} ne peut pas devenir un panneau non activant",
                current.name().to_string_lossy()
            );
            return;
        }
        // SAFETY:
        // - `NSPanel` hérite de `NSWindow` sans variable d'instance de plus :
        //   les deux tailles d'instance sont égales (vérifié ci-dessus), et la
        //   classe de winit (`WinitWindow`) n'en ajoute aucune non plus ; elle
        //   ne remplace que `canBecomeKeyWindow` et `canBecomeMainWindow`,
        //   dont on veut justement retrouver le comportement d'origine (une
        //   fenêtre sans bordure ne devient pas la fenêtre active).
        // - Tout le reste du programme ne voit cette fenêtre que comme une
        //   `NSWindow`, dont `NSPanel` garde toutes les méthodes.
        // - On est sur le fil principal (seul endroit où AppKit touche à la
        //   fenêtre), avant le premier affichage de l'animal.
        unsafe {
            let object: &AnyObject = window.as_ref();
            AnyObject::set_class(object, panel);
        }
    }
    window.setStyleMask(window.styleMask() | NSWindowStyleMask::NonactivatingPanel);
    log::info!("fenêtre d'animal : panneau non activant");
}

/// Mouflon a-t-il l'autorisation d'Accessibilité (Réglages Système →
/// Confidentialité et sécurité → Accessibilité) ?
#[must_use]
pub fn accessibility_trusted() -> bool {
    // SAFETY: fonction sans argument, utilisable depuis n'importe quel fil.
    unsafe { AXIsProcessTrusted() }
}

/// Demande l'autorisation d'Accessibilité : macOS affiche sa fenêtre si
/// Mouflon ne l'a pas encore. Renvoie l'état actuel (la réponse de
/// l'utilisateur arrive plus tard).
pub fn request_accessibility() -> bool {
    // SAFETY: `kAXTrustedCheckOptionPrompt` est une CFString immuable exportée
    // par ApplicationServices, valide pendant toute la vie du processus.
    let key: &CFString = unsafe { kAXTrustedCheckOptionPrompt };
    let options = CFDictionary::from_slices(&[key], &[CFBoolean::new(true)]);
    // SAFETY: le dictionnaire a des clés CFString et des valeurs CFBoolean,
    // comme le demande la fonction.
    unsafe { AXIsProcessTrustedWithOptions(Some(options.as_opaque())) }
}

/// Attribut d'un élément d'accessibilité, s'il existe.
fn ax_attribute(element: &AXUIElement, name: &'static str) -> Option<CFRetained<CFType>> {
    let attribute = CFString::from_static_str(name);
    let mut value: *const CFType = std::ptr::null();
    // SAFETY: `value` pointe vers une variable locale ; en cas de succès,
    // la fonction y écrit une référence dont on devient propriétaire
    // (règle « Copy » de Core Foundation).
    let error = unsafe { element.copy_attribute_value(&attribute, NonNull::from(&mut value)) };
    if error != AXError::Success {
        return None;
    }
    let value = NonNull::new(value.cast_mut())?;
    // SAFETY: référence non nulle et possédée (voir ci-dessus).
    Some(unsafe { CFRetained::from_raw(value) })
}

/// Ce qu'une valeur d'accessibilité peut contenir : le type Rust et sa
/// description pour `AXValueGetValue` vont ensemble.
trait AxGeometry: Default {
    const KIND: AXValueType;
}

impl AxGeometry for CGPoint {
    const KIND: AXValueType = AXValueType::CGPoint;
}

impl AxGeometry for CGSize {
    const KIND: AXValueType = AXValueType::CGSize;
}

/// Point ou taille d'un élément d'accessibilité.
fn ax_geometry<T: AxGeometry>(element: &AXUIElement, name: &'static str) -> Option<T> {
    let value = ax_attribute(element, name)?.downcast::<AXValue>().ok()?;
    let mut out = T::default();
    // SAFETY: `out` est une variable locale de type `T`, celui que
    // `T::KIND` décrit ; la fonction renvoie `false` sans rien écrire si la
    // valeur contient autre chose.
    let ok = unsafe { value.value(T::KIND, NonNull::from(&mut out).cast()) };
    ok.then_some(out)
}

/// Cadre des icônes du Dock (repère commun), lu par l'Accessibilité. Rien
/// sans l'autorisation, ou si le Dock ne répond pas.
#[must_use]
pub fn dock_frame() -> Option<OsRect> {
    if !accessibility_trusted() {
        return None;
    }
    let dock = NSRunningApplication::runningApplicationsWithBundleIdentifier(&NSString::from_str(
        "com.apple.dock",
    ))
    .firstObject()?;
    // SAFETY: `processIdentifier` est celui d'une app en cours ; la fonction
    // renvoie l'élément racine de son interface, possédé (+1).
    let app = unsafe { AXUIElement::new_application(dock.processIdentifier()) };
    // Chaque question est un échange avec le Dock : un Dock qui ne répond
    // pas ne doit pas figer les animaux (6 secondes par défaut).
    // SAFETY: `app` est un élément valide ; le délai est un nombre positif.
    unsafe { app.set_messaging_timeout(0.25) };
    let children = ax_attribute(&app, "AXChildren")?
        .downcast::<CFArray>()
        .ok()?;
    // SAFETY: l'attribut « AXChildren » est un tableau d'éléments d'accessibilité.
    let children: CFRetained<CFArray<AXUIElement>> =
        unsafe { CFRetained::cast_unchecked(children) };
    // La liste des icônes : la première dont le rôle est « AXList ».
    let list = children.iter().find(|child| {
        ax_attribute(child, "AXRole")
            .and_then(|role| role.downcast::<CFString>().ok())
            .is_some_and(|role| role.to_string() == "AXList")
    })?;
    let origin: CGPoint = ax_geometry(&list, "AXPosition")?;
    let size: CGSize = ax_geometry(&list, "AXSize")?;
    (size.width > 0.0 && size.height > 0.0).then_some(OsRect {
        x: origin.x,
        y: origin.y,
        w: size.width,
        h: size.height,
    })
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
