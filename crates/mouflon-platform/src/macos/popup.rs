//! Menu surgissant : le menu d'un animal, au clic droit.

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly, available, sel};
use objc2_app_kit::{NSControlStateValueOn, NSEvent, NSMenu, NSMenuItem};
use objc2_foundation::{NSString, ns_string};

use super::actions::actions;
use crate::PopupItem;

/// Ouvre un menu sous le curseur et attend qu'il se ferme (les animaux
/// s'arrêtent le temps de choisir). L'article choisi arrive ensuite à l'app
/// comme [`crate::UiEvent::Menu`], avec son identifiant.
pub fn pop_up_menu(items: &[PopupItem]) {
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let menu = build(items, mtm);
    // Sans vue, la position est dans le repère de l'écran, celui du curseur.
    menu.popUpMenuPositioningItem_atLocation_inView(None, NSEvent::mouseLocation(), None);
}

fn build(items: &[PopupItem], mtm: MainThreadMarker) -> Retained<NSMenu> {
    let menu = NSMenu::new(mtm);
    // Chaque article dit lui-même s'il est disponible.
    menu.setAutoenablesItems(false);
    let target = actions(mtm);
    for item in items {
        match item {
            PopupItem::Header(title) => {
                let title = NSString::from_str(title);
                let header = if available!(macos = 14.0) {
                    NSMenuItem::sectionHeaderWithTitle(&title, mtm)
                } else {
                    let header = NSMenuItem::new(mtm);
                    header.setTitle(&title);
                    header.setEnabled(false);
                    header
                };
                menu.addItem(&header);
            }
            PopupItem::Item {
                id,
                label,
                checked,
                enabled,
            } => {
                // SAFETY: `choose:` est une action d'`Actions`, la cible
                // donnée ci-dessous, qui vit jusqu'à la fin du processus.
                let entry = unsafe {
                    NSMenuItem::initWithTitle_action_keyEquivalent(
                        NSMenuItem::alloc(mtm),
                        &NSString::from_str(label),
                        Some(sel!(choose:)),
                        ns_string!(""),
                    )
                };
                // SAFETY: la cible est un `Actions`, qui répond à `choose:`.
                unsafe { entry.setTarget(Some(&target)) };
                entry.setTag(*id as isize);
                entry.setEnabled(*enabled);
                if *checked {
                    entry.setState(NSControlStateValueOn);
                }
                menu.addItem(&entry);
            }
            PopupItem::Submenu { label, items } => {
                let entry = NSMenuItem::new(mtm);
                entry.setTitle(&NSString::from_str(label));
                entry.setSubmenu(Some(&build(items, mtm)));
                menu.addItem(&entry);
            }
            PopupItem::Separator => menu.addItem(&NSMenuItem::separatorItem(mtm)),
        }
    }
    menu
}
