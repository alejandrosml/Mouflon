//! La fenêtre des réglages : ce qu'elle contient, dans la langue de
//! l'utilisateur, ce qu'elle montre de l'état de l'app, et ce que fait
//! chacun de ses contrôles (une [`Action`], comme les menus).

use mouflon_platform::{
    Form, FormButton, FormExtra, FormRow, FormSection, FormValue, LoginItem, UiEvent,
};

use crate::i18n::Texts;
use crate::menu::Action;

// Identifiants des contrôles.
const SIZE: u32 = 1;
const SOUNDS: u32 = 2;
const COLLISIONS: u32 = 3;
const FRONTMOST: u32 = 4;
const DOCK: u32 = 5;
const LOGIN: u32 = 6;
/// « Ouvrir Réglages Système » sous « Marcher sous le Dock ».
const ACCESSIBILITY: u32 = 7;
/// « Ouvrir Réglages Système » sous « Ouvrir Mouflon à l'ouverture de session ».
const LOGIN_APPROVAL: u32 = 8;
const FOLDER: u32 = 9;
const PHYSICS: u32 = 10;

/// Ce que la fenêtre montre : les réglages de l'app, et ce qu'en dit le
/// système (autorisation d'Accessibilité, ouverture de session).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shown {
    /// Rang de la taille dans [`crate::settings::SIZES`].
    pub size: usize,
    pub sounds: bool,
    pub collisions: bool,
    pub realistic_physics: bool,
    pub frontmost_only: bool,
    pub walk_under_dock: bool,
    /// Mouflon a l'autorisation d'Accessibilité.
    pub trusted: bool,
    pub login: LoginItem,
}

fn check(id: u32, label: &str, help: Option<&str>, extra: Option<FormExtra>) -> FormRow {
    FormRow::Check {
        id,
        label: label.to_owned(),
        help: help.map(str::to_owned),
        extra,
    }
}

fn extra(note: &str, id: u32, button: &str) -> FormExtra {
    FormExtra {
        note: note.to_owned(),
        button: FormButton {
            id,
            label: button.to_owned(),
        },
    }
}

/// La fenêtre : les animaux, où ils se promènent, le démarrage (si le
/// système permet de le régler d'ici) et l'installation de nouveaux animaux.
#[must_use]
pub fn form(t: &Texts, login: LoginItem) -> Form {
    let mut sections = vec![
        FormSection {
            title: t.pets_section.clone(),
            rows: vec![
                FormRow::Steps {
                    id: SIZE,
                    label: t.size.clone(),
                    steps: [&t.size_small, &t.size_normal, &t.size_large, &t.size_huge]
                        .into_iter()
                        .cloned()
                        .collect(),
                    help: Some(t.size_help.clone()),
                },
                check(SOUNDS, &t.pet_sounds, None, None),
            ],
        },
        FormSection {
            title: t.walks_section.clone(),
            rows: vec![
                check(
                    PHYSICS,
                    &t.realistic_physics,
                    Some(&t.realistic_physics_help),
                    None,
                ),
                check(COLLISIONS, &t.collisions, Some(&t.collisions_help), None),
                check(
                    FRONTMOST,
                    &t.frontmost_only,
                    Some(&t.frontmost_only_help),
                    None,
                ),
                check(
                    DOCK,
                    &t.walk_under_dock,
                    Some(&t.walk_under_dock_help),
                    Some(extra(
                        &t.accessibility_waiting,
                        ACCESSIBILITY,
                        &t.open_system_settings,
                    )),
                ),
            ],
        },
    ];
    if login != LoginItem::Unavailable {
        sections.push(FormSection {
            title: t.startup_section.clone(),
            rows: vec![check(
                LOGIN,
                &t.open_at_login,
                None,
                Some(extra(
                    &t.login_waiting,
                    LOGIN_APPROVAL,
                    &t.open_system_settings,
                )),
            )],
        });
    }
    sections.push(FormSection {
        title: t.my_pets_section.clone(),
        rows: vec![FormRow::Text {
            text: t.my_pets_help.clone(),
            button: Some(FormButton {
                id: FOLDER,
                label: t.open_pets_folder.clone(),
            }),
        }],
    });
    Form {
        title: t.settings_title.clone(),
        sections,
    }
}

/// Ce que montre chaque contrôle. Une remarque n'apparaît que quand il
/// manque quelque chose : l'autorisation d'Accessibilité pour marcher sous
/// le Dock, l'accord de l'utilisateur pour l'ouverture de session.
#[must_use]
pub fn values(shown: &Shown) -> Vec<FormValue> {
    vec![
        FormValue::Step(SIZE, shown.size),
        FormValue::Check(SOUNDS, shown.sounds),
        FormValue::Check(PHYSICS, shown.realistic_physics),
        FormValue::Check(COLLISIONS, shown.collisions),
        FormValue::Check(FRONTMOST, shown.frontmost_only),
        FormValue::Check(DOCK, shown.walk_under_dock),
        FormValue::Shown(ACCESSIBILITY, shown.walk_under_dock && !shown.trusted),
        FormValue::Check(LOGIN, shown.login.is_requested()),
        FormValue::Shown(LOGIN_APPROVAL, shown.login == LoginItem::NeedsApproval),
    ]
}

/// Ce que fait un contrôle de la fenêtre des réglages (les articles des
/// menus surgissants ne passent pas par ici).
#[must_use]
pub fn action(event: UiEvent) -> Option<Action> {
    Some(match event {
        UiEvent::Step(SIZE, size) => Action::SetSize(size),
        UiEvent::Check(SOUNDS, on) => Action::SetSounds(on),
        UiEvent::Check(COLLISIONS, on) => Action::SetCollisions(on),
        UiEvent::Check(PHYSICS, on) => Action::SetRealisticPhysics(on),
        UiEvent::Check(FRONTMOST, on) => Action::SetFrontmostOnly(on),
        UiEvent::Check(DOCK, on) => Action::SetWalkUnderDock(on),
        UiEvent::Check(LOGIN, on) => Action::SetOpenAtLogin(on),
        UiEvent::Button(ACCESSIBILITY) => Action::OpenAccessibilitySettings,
        UiEvent::Button(LOGIN_APPROVAL) => Action::OpenLoginItemsSettings,
        UiEvent::Button(FOLDER) => Action::OpenPetsFolder,
        UiEvent::FormShown => Action::RefreshSettings,
        UiEvent::Quit => Action::Quit,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts() -> Texts {
        toml::from_str(include_str!("../locales/fr.toml")).unwrap()
    }

    fn shown() -> Shown {
        Shown {
            size: 1,
            sounds: true,
            collisions: false,
            realistic_physics: true,
            frontmost_only: true,
            walk_under_dock: false,
            trusted: false,
            login: LoginItem::Off,
        }
    }

    /// Identifiants des contrôles de la fenêtre : cases et curseurs, puis
    /// boutons des compléments, puis autres boutons.
    fn controls(form: &Form) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let (mut values, mut extras, mut buttons) = (Vec::new(), Vec::new(), Vec::new());
        for row in form.sections.iter().flat_map(|s| &s.rows) {
            match row {
                FormRow::Check { id, extra, .. } => {
                    values.push(*id);
                    extras.extend(extra.as_ref().map(|e| e.button.id));
                }
                FormRow::Steps { id, .. } => values.push(*id),
                FormRow::Text { button, .. } => buttons.extend(button.as_ref().map(|b| b.id)),
            }
        }
        (values, extras, buttons)
    }

    #[test]
    fn every_control_shows_a_value_and_leads_to_an_action() {
        let form = form(&texts(), LoginItem::Off);
        let (values_ids, extras, buttons) = controls(&form);
        let shown = values(&shown());
        for id in &values_ids {
            assert!(
                shown.iter().any(
                    |v| matches!(v, FormValue::Check(i, _) | FormValue::Step(i, _) if i == id)
                ),
                "le contrôle {id} ne montre rien"
            );
            assert!(
                action(UiEvent::Check(*id, true)).is_some()
                    || action(UiEvent::Step(*id, 2)).is_some(),
                "le contrôle {id} ne fait rien"
            );
        }
        for id in &extras {
            assert!(shown.contains(&FormValue::Shown(*id, false)));
        }
        for id in extras.iter().chain(&buttons) {
            assert!(action(UiEvent::Button(*id)).is_some(), "bouton {id}");
        }
        // Chaque valeur va à un contrôle qui existe.
        for value in &shown {
            let (FormValue::Check(id, _) | FormValue::Step(id, _) | FormValue::Shown(id, _)) =
                value;
            assert!(values_ids.contains(id) || extras.contains(id), "{value:?}");
        }
    }

    #[test]
    fn a_control_gives_the_value_it_shows() {
        assert_eq!(
            action(UiEvent::Check(SOUNDS, false)),
            Some(Action::SetSounds(false))
        );
        assert_eq!(
            action(UiEvent::Check(DOCK, true)),
            Some(Action::SetWalkUnderDock(true))
        );
        assert_eq!(action(UiEvent::Step(SIZE, 3)), Some(Action::SetSize(3)));
        assert_eq!(
            action(UiEvent::Check(PHYSICS, false)),
            Some(Action::SetRealisticPhysics(false))
        );
        assert_eq!(action(UiEvent::FormShown), Some(Action::RefreshSettings));
        assert_eq!(action(UiEvent::Menu(1)), None);
    }

    #[test]
    fn notes_appear_only_when_something_is_missing() {
        let shown_extra = |s: &Shown, id: u32| values(s).contains(&FormValue::Shown(id, true));
        let mut s = shown();
        assert!(
            !shown_extra(&s, ACCESSIBILITY),
            "option coupée : rien à dire"
        );
        s.walk_under_dock = true;
        assert!(shown_extra(&s, ACCESSIBILITY), "autorisation manquante");
        s.trusted = true;
        assert!(!shown_extra(&s, ACCESSIBILITY));
        assert!(!shown_extra(&s, LOGIN_APPROVAL));
        s.login = LoginItem::NeedsApproval;
        assert!(shown_extra(&s, LOGIN_APPROVAL));
        assert!(values(&s).contains(&FormValue::Check(LOGIN, true)));
    }

    #[test]
    fn the_startup_section_needs_a_system_that_can_set_it() {
        let t = texts();
        let has_login = |login| controls(&form(&t, login)).0.contains(&LOGIN);
        assert!(has_login(LoginItem::On));
        assert!(!has_login(LoginItem::Unavailable));
    }
}
