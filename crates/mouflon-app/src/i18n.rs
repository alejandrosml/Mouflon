//! Textes de l'interface, dans la langue choisie dans macOS (Réglages Système →
//! Général → Langue et région, langue propre à Mouflon comprise). Mouflon parle
//! anglais, français et espagnol ; pour une autre langue, il parle anglais.
//!
//! Ajouter une langue : copier `locales/en.toml` sous le code ISO 639-1 de la
//! langue (`de.toml`, `it.toml`…), le traduire, l'ajouter à [`LOCALES`] et à
//! `CFBundleLocalizations` dans `packaging/macos/Info.plist.in`.

use std::sync::OnceLock;

use serde::Deserialize;

/// Langues connues : code ISO 639-1 et textes. La première sert de repli.
const LOCALES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en.toml")),
    ("fr", include_str!("../locales/fr.toml")),
    ("es", include_str!("../locales/es.toml")),
];

/// Les textes d'une langue : chaque fichier doit les donner tous, sans autre clé.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub struct Texts {
    pub add_pet: String,
    pub add_menu: String,
    pub remove_all: String,
    pub pause: String,
    pub frontmost_only: String,
    pub collisions: String,
    pub open_pets_folder: String,
    pub about: String,
    pub quit: String,
    pub description: String,
    pub credits: String,
}

/// Code de la première langue préférée que Mouflon parle, l'anglais à défaut.
/// `preferred` : langues de l'utilisateur dans son ordre (« fr-FR », « es-419 »…).
#[must_use]
pub fn language(preferred: &[String]) -> &'static str {
    preferred
        .iter()
        .find_map(|tag| {
            let primary = tag.split(['-', '_']).next()?.to_ascii_lowercase();
            LOCALES
                .iter()
                .find(|(code, _)| *code == primary)
                .map(|(code, _)| *code)
        })
        .unwrap_or(LOCALES[0].0)
}

fn parse(code: &str) -> Texts {
    let (_, source) = LOCALES
        .iter()
        .find(|(c, _)| *c == code)
        .unwrap_or(&LOCALES[0]);
    toml::from_str(source).unwrap_or_else(|e| {
        log::error!("locales/{code}.toml illisible, textes anglais : {e}");
        toml::from_str(LOCALES[0].1).expect("locales/en.toml")
    })
}

/// Textes de l'interface dans la langue de l'utilisateur, choisie au premier appel.
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub fn texts() -> &'static Texts {
    static TEXTS: OnceLock<Texts> = OnceLock::new();
    TEXTS.get_or_init(|| {
        let code = language(&mouflon_platform::preferred_languages());
        log::info!("langue de l'interface : {code}");
        parse(code)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn every_language_gives_every_text() {
        for (code, source) in LOCALES {
            let texts: Texts =
                toml::from_str(source).unwrap_or_else(|e| panic!("locales/{code}.toml : {e}"));
            for text in [
                &texts.add_pet,
                &texts.add_menu,
                &texts.remove_all,
                &texts.pause,
                &texts.frontmost_only,
                &texts.collisions,
                &texts.open_pets_folder,
                &texts.about,
                &texts.quit,
                &texts.description,
                &texts.credits,
            ] {
                assert!(!text.trim().is_empty(), "{code} : texte vide");
            }
            assert!(texts.about.contains("Mouflon") && texts.quit.contains("Mouflon"));
        }
    }

    #[test]
    fn follows_the_first_language_it_speaks() {
        assert_eq!(language(&tags(&["fr-FR", "en-US"])), "fr");
        assert_eq!(language(&tags(&["de-DE", "es-419", "fr-FR"])), "es");
        assert_eq!(language(&tags(&["EN-gb"])), "en");
        assert_eq!(language(&tags(&["fr_CA"])), "fr");
        // Une langue qu'il ne parle pas encore, ou aucune : l'anglais.
        assert_eq!(language(&tags(&["zh-Hans-CN", "ja-JP"])), "en");
        assert_eq!(language(&[]), "en");
    }

    #[test]
    fn speaks_french_and_spanish() {
        assert_eq!(parse("fr").quit, "Quitter Mouflon");
        assert_eq!(parse("es").quit, "Salir de Mouflon");
        assert_eq!(parse("xx").quit, "Quit Mouflon");
    }
}
