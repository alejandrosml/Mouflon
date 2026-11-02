//! Les fenêtres de Mouflon passent devant ses animaux.
//!
//! Les animaux vivent dans des fenêtres flottantes, au-dessus de toutes les
//! apps ; les fenêtres de Mouflon (réglages, galerie, « À propos ») sont des
//! fenêtres ordinaires. Sans plus, un écran plein d'animaux les couvrirait
//! et prendrait tous les clics : on ne pourrait plus toucher aux réglages.
//! Tant que Mouflon est au premier plan, elles passent donc juste devant les
//! animaux ; quand on passe à une autre app, elles reprennent leur niveau
//! ordinaire, juste derrière la fenêtre de devant de cette app (comme toute
//! fenêtre d'une app qu'on quitte).

use std::cell::Cell;

use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, sel};
use objc2_app_kit::{
    NSApplication, NSApplicationDidBecomeActiveNotification,
    NSApplicationDidResignActiveNotification, NSFloatingWindowLevel, NSNormalWindowLevel, NSWindow,
    NSWindowLevel, NSWindowNumberListOptions, NSWindowOrderingMode,
};
use objc2_foundation::NSNotificationCenter;

use super::actions::actions;

/// Le niveau des fenêtres de Mouflon quand elles passent devant les
/// animaux : juste au-dessus du leur (celui des fenêtres flottantes).
const FRONT: NSWindowLevel = NSFloatingWindowLevel + 1;

thread_local! {
    /// Mouflon suit-il déjà son passage au premier plan et son départ ?
    static FOLLOWING: Cell<bool> = const { Cell::new(false) };
}

/// Une fenêtre de Mouflon vient d'être montrée : ses fenêtres passent
/// devant les animaux, et y resteront tant qu'il est au premier plan.
pub(super) fn bring_forward(mtm: MainThreadMarker) {
    follow_activation(mtm);
    set_front(mtm, true);
}

/// Mouflon passe au premier plan (`true`) : ses fenêtres ordinaires passent
/// devant les animaux. Il le quitte (`false`) : elles reprennent leur
/// niveau, derrière la fenêtre de devant des autres apps.
pub(super) fn set_front(mtm: MainThreadMarker, front: bool) {
    let (from, to) = if front {
        (NSNormalWindowLevel, FRONT)
    } else {
        (FRONT, NSNormalWindowLevel)
    };
    let windows = NSApplication::sharedApplication(mtm).windows();
    let mut moved: Vec<_> = windows.iter().filter(|w| w.level() == from).collect();
    if moved.is_empty() {
        return;
    }
    // De la plus en arrière à la plus en avant : chacune passe devant les
    // précédentes, et elles gardent leur ordre entre elles.
    let order = z_order(mtm);
    moved.sort_by_key(|w| {
        let number = w.windowNumber();
        std::cmp::Reverse(
            order
                .iter()
                .position(|&n| n == number)
                .unwrap_or(usize::MAX),
        )
    });
    for window in &moved {
        window.setLevel(to);
    }
    if front {
        return;
    }
    // Changer de niveau a pu les poser devant la fenêtre qu'on vient de
    // choisir dans l'autre app : elles repassent juste derrière elle.
    let Some(top) = super::windows().into_iter().find(|w| w.layer == 0) else {
        return;
    };
    let Ok(top) = isize::try_from(top.id) else {
        return;
    };
    let mut above = top;
    for window in moved.iter().rev().filter(|w| w.isVisible()) {
        window.orderWindow_relativeTo(NSWindowOrderingMode::Below, above);
        above = window.windowNumber();
    }
}

/// Les numéros des fenêtres visibles de Mouflon, de la plus en avant à la
/// plus en arrière.
fn z_order(mtm: MainThreadMarker) -> Vec<isize> {
    NSWindow::windowNumbersWithOptions(NSWindowNumberListOptions(0), mtm)
        .map(|numbers| numbers.iter().map(|n| n.integerValue()).collect())
        .unwrap_or_default()
}

/// Suit, une fois pour toutes, le passage de Mouflon au premier plan et son
/// départ (voir les méthodes `appDidBecomeActive:` et `appDidResignActive:`
/// de la cible commune).
fn follow_activation(mtm: MainThreadMarker) {
    if FOLLOWING.replace(true) {
        return;
    }
    let target = actions(mtm);
    let observer: &AnyObject = target.as_ref();
    let center = NSNotificationCenter::defaultCenter();
    // SAFETY: les deux sélecteurs sont des méthodes de la cible qui prennent
    // une NSNotification ; la cible vit jusqu'à la fin du processus (voir
    // `actions`) ; les noms sont des constantes d'AppKit, valides toute la vie
    // du processus ; pas de filtre sur l'objet.
    unsafe {
        center.addObserver_selector_name_object(
            observer,
            sel!(appDidBecomeActive:),
            Some(NSApplicationDidBecomeActiveNotification),
            None,
        );
        center.addObserver_selector_name_object(
            observer,
            sel!(appDidResignActive:),
            Some(NSApplicationDidResignActiveNotification),
            None,
        );
    }
}
