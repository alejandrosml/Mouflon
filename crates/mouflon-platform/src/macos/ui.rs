//! Les petites fenêtres de Mouflon : « À propos » et bulle d'explication.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AllocAnyThread, MainThreadMarker, MainThreadOnly, available};
use objc2_app_kit::{
    NSAboutPanelOptionApplicationName, NSAboutPanelOptionApplicationVersion,
    NSAboutPanelOptionCredits, NSAboutPanelOptionVersion, NSAlert, NSApplication, NSColor, NSFont,
    NSFontAttributeName, NSForegroundColorAttributeName, NSLinkAttributeName,
    NSMutableParagraphStyle, NSParagraphStyleAttributeName, NSPopover, NSPopoverBehavior,
    NSStatusItem, NSTextAlignment, NSTextField, NSView, NSViewController,
};
use objc2_foundation::{
    NSAttributedString, NSAttributedStringKey, NSDictionary, NSMutableAttributedString, NSPoint,
    NSRect, NSRectEdge, NSSize, NSString, NSURL, ns_string,
};

use crate::{About, AboutLine};

/// Largeur du texte d'une bulle, et marge autour, en points.
const NOTE_WIDTH: f64 = 290.0;
const NOTE_MARGIN: f64 = 14.0;
/// Espace entre le titre d'une bulle et son texte.
const NOTE_GAP: f64 = 6.0;

thread_local! {
    /// La bulle ouverte, gardée en vie tant qu'elle est affichée.
    static NOTE: RefCell<Option<Retained<NSPopover>>> = const { RefCell::new(None) };
}

/// Fait passer Mouflon au premier plan : une app de la barre des menus ne
/// l'est jamais d'elle-même, et ses fenêtres resteraient derrière celles de
/// l'app où l'on travaille.
fn activate(app: &NSApplication) {
    if available!(macos = 14.0) {
        app.activate();
    } else {
        #[allow(deprecated)]
        app.activateIgnoringOtherApps(true);
    }
}

/// Texte des crédits : un paragraphe centré par ligne, liens cliquables.
fn credits(lines: &[AboutLine<'_>]) -> Retained<NSAttributedString> {
    let text = NSMutableAttributedString::new();
    let style = NSMutableParagraphStyle::new();
    style.setAlignment(NSTextAlignment::Center);
    style.setParagraphSpacing(5.0);
    let font = NSFont::systemFontOfSize(NSFont::smallSystemFontSize());
    // Couleur du système : lisible en clair comme en sombre.
    let color = NSColor::secondaryLabelColor();
    for (i, line) in lines.iter().enumerate() {
        let (shown, url) = match line {
            AboutLine::Text(shown) => (*shown, None),
            AboutLine::Link(shown, url) => (*shown, NSURL::URLWithString(&NSString::from_str(url))),
        };
        let mut string = shown.to_owned();
        if i + 1 < lines.len() {
            string.push('\n');
        }
        // SAFETY: statiques NSString exportées par AppKit, valides pendant
        // toute la vie du processus.
        let mut keys: Vec<&NSAttributedStringKey> = unsafe {
            vec![
                NSFontAttributeName,
                NSParagraphStyleAttributeName,
                NSForegroundColorAttributeName,
            ]
        };
        let mut values: Vec<&AnyObject> = vec![font.as_ref(), style.as_ref(), color.as_ref()];
        if let Some(url) = &url {
            // SAFETY: comme ci-dessus.
            keys.push(unsafe { NSLinkAttributeName });
            values.push(url.as_ref());
        }
        let attributes = NSDictionary::from_slices(&keys, &values);
        // SAFETY: chaque attribut a une valeur du type qu'AppKit attend :
        // NSFont, NSParagraphStyle, NSColor et NSURL.
        let piece = unsafe {
            NSAttributedString::initWithString_attributes(
                NSAttributedString::alloc(),
                &NSString::from_str(&string),
                Some(&attributes),
            )
        };
        text.appendAttributedString(&piece);
    }
    Retained::into_super(text)
}

/// Ouvre la fenêtre « À propos » de macOS, devant les autres apps : icône,
/// nom, version, crédits (avec liens) et mention de copyright.
pub fn show_about(about: &About<'_>) {
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let name = NSString::from_str(about.name);
    let version = NSString::from_str(about.version);
    // Pas de numéro de build entre parenthèses : il redirait la version.
    let build = NSString::from_str("");
    let copyright = NSString::from_str(about.copyright);
    let credits = credits(about.lines);
    // SAFETY: statiques NSString exportées par AppKit, valides pendant toute
    // la vie du processus.
    let keys: [&NSString; 5] = unsafe {
        [
            NSAboutPanelOptionApplicationName,
            NSAboutPanelOptionApplicationVersion,
            NSAboutPanelOptionVersion,
            ns_string!("Copyright"),
            NSAboutPanelOptionCredits,
        ]
    };
    let values: [&AnyObject; 5] = [
        name.as_ref(),
        version.as_ref(),
        build.as_ref(),
        copyright.as_ref(),
        credits.as_ref(),
    ];
    let options = NSDictionary::from_slices(&keys, &values);
    activate(&app);
    // SAFETY: les clés sont celles que la méthode connaît, avec des valeurs
    // du type attendu (NSString, et NSAttributedString pour les crédits).
    unsafe { app.orderFrontStandardAboutPanelWithOptions(&options) };
}

/// Hauteur d'un texte une fois mis en page sur la largeur d'une bulle.
fn text_height(field: &NSTextField, fallback: f64) -> f64 {
    let bounds = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(NOTE_WIDTH, 10_000.0));
    field
        .cell()
        .map_or(fallback, |cell| cell.cellSizeForBounds(bounds).height)
        .ceil()
}

/// Montre une courte explication dans une bulle sous l'icône de Mouflon
/// (barre des menus), qui se ferme au clic suivant. Sans icône, dans une
/// petite fenêtre.
pub fn show_note(anchor: Option<&NSStatusItem>, title: &str, text: &str) {
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    close_note();
    let Some(button) = anchor.and_then(|item| item.button(mtm)) else {
        let alert = NSAlert::new(mtm);
        alert.setMessageText(&NSString::from_str(title));
        alert.setInformativeText(&NSString::from_str(text));
        activate(&app);
        // Fenêtre modale : winit permet d'en ouvrir une pendant qu'il nous
        // passe un événement (les animaux s'arrêtent le temps de la lire).
        alert.runModal();
        return;
    };

    let heading = NSTextField::labelWithString(&NSString::from_str(title), mtm);
    heading.setFont(Some(
        &NSFont::boldSystemFontOfSize(NSFont::systemFontSize()),
    ));
    let body = NSTextField::wrappingLabelWithString(&NSString::from_str(text), mtm);
    body.setSelectable(false);
    body.setPreferredMaxLayoutWidth(NOTE_WIDTH);
    let heading_height = text_height(&heading, 17.0);
    let body_height = text_height(&body, 64.0);
    let size = NSSize::new(
        NOTE_WIDTH + 2.0 * NOTE_MARGIN,
        heading_height + NOTE_GAP + body_height + 2.0 * NOTE_MARGIN,
    );
    // Repère d'AppKit : l'origine est en bas à gauche.
    body.setFrame(NSRect::new(
        NSPoint::new(NOTE_MARGIN, NOTE_MARGIN),
        NSSize::new(NOTE_WIDTH, body_height),
    ));
    heading.setFrame(NSRect::new(
        NSPoint::new(NOTE_MARGIN, NOTE_MARGIN + body_height + NOTE_GAP),
        NSSize::new(NOTE_WIDTH, heading_height),
    ));
    let content = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), size),
    );
    content.addSubview(&heading);
    content.addSubview(&body);
    let controller = NSViewController::new(mtm);
    controller.setView(&content);

    let popover = NSPopover::new(mtm);
    popover.setBehavior(NSPopoverBehavior::Transient);
    popover.setContentViewController(Some(&controller));
    popover.setContentSize(size);
    activate(&app);
    popover.showRelativeToRect_ofView_preferredEdge(button.bounds(), &button, NSRectEdge::MinY);
    NOTE.with(|note| *note.borrow_mut() = Some(popover));
}

/// Ferme la bulle d'explication si elle est encore ouverte.
pub fn close_note() {
    if MainThreadMarker::new().is_none() {
        return;
    }
    if let Some(popover) = NOTE.with(|note| note.borrow_mut().take())
        && popover.isShown()
    {
        popover.close();
    }
}
