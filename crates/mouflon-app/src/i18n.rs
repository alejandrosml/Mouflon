//! Textes de l'interface, dans la langue choisie dans macOS (Réglages Système →
//! Général → Langue et région, langue propre à Mouflon comprise). Mouflon parle
//! anglais, français, espagnol, allemand, italien, néerlandais, portugais (du
//! Brésil et du Portugal), russe, japonais, coréen et chinois (simplifié et
//! traditionnel) ; pour une autre langue, il parle anglais.
//!
//! Ajouter une langue : copier `locales/en.toml` sous le code de la langue
//! (ISO 639-1 : `pl.toml`, `sv.toml`…), le traduire, l'ajouter à [`LOCALES`] et
//! à `CFBundleLocalizations` dans `packaging/macos/Info.plist.in`.

use std::sync::OnceLock;

use serde::Deserialize;

/// Langues connues : code (comme les dossiers `.lproj` de macOS) et textes.
/// La première sert de repli.
const LOCALES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en.toml")),
    ("fr", include_str!("../locales/fr.toml")),
    ("es", include_str!("../locales/es.toml")),
    ("de", include_str!("../locales/de.toml")),
    ("it", include_str!("../locales/it.toml")),
    ("nl", include_str!("../locales/nl.toml")),
    ("pt", include_str!("../locales/pt.toml")),
    ("pt-PT", include_str!("../locales/pt-PT.toml")),
    ("ru", include_str!("../locales/ru.toml")),
    ("ja", include_str!("../locales/ja.toml")),
    ("ko", include_str!("../locales/ko.toml")),
    ("zh-Hans", include_str!("../locales/zh-Hans.toml")),
    ("zh-Hant", include_str!("../locales/zh-Hant.toml")),
];

/// Pays où l'on écrit le portugais du Portugal (l'autre variante, celle du
/// Brésil, sert aussi quand la région n'est pas dite).
const EUROPEAN_PORTUGUESE: &[&str] = &["pt", "ao", "mz", "cv", "gw", "st", "tl", "mo"];
/// Régions où l'on écrit le chinois traditionnel, quand l'écriture n'est pas dite.
const TRADITIONAL_CHINESE: &[&str] = &["tw", "hk", "mo"];

/// Code de [`LOCALES`] qui convient à une étiquette BCP 47 (« fr-FR »,
/// « zh-Hant-TW », « pt_BR »…), s'il y en a un.
fn locale_of(tag: &str) -> Option<&'static str> {
    let tag = tag.replace('_', "-").to_ascii_lowercase();
    let mut parts = tag.split('-');
    let primary = parts.next()?;
    let rest: Vec<&str> = parts.collect();
    // Écriture : quatre lettres (« hant ») ; région : deux lettres ou trois chiffres.
    let script = rest.iter().find(|p| p.len() == 4);
    let region = rest
        .iter()
        .find(|p| p.len() == 2 || (p.len() == 3 && p.chars().all(|c| c.is_ascii_digit())));
    let code = match primary {
        "zh" => match script {
            Some(&"hant") => "zh-Hant",
            Some(_) => "zh-Hans",
            None if region.is_some_and(|r| TRADITIONAL_CHINESE.contains(r)) => "zh-Hant",
            None => "zh-Hans",
        },
        "pt" if region.is_some_and(|r| EUROPEAN_PORTUGUESE.contains(r)) => "pt-PT",
        other => other,
    };
    LOCALES
        .iter()
        .map(|(c, _)| *c)
        .find(|c| c.eq_ignore_ascii_case(code))
}

/// Les textes d'une langue : chaque fichier doit les donner tous, sans autre clé.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
pub struct Texts {
    pub add_pet: String,
    pub add_menu: String,
    pub remove_all: String,
    pub together: String,
    pub pause: String,
    pub frontmost_only: String,
    pub collisions: String,
    pub size: String,
    pub size_small: String,
    pub size_normal: String,
    pub size_large: String,
    pub size_huge: String,
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
        .find_map(|tag| locale_of(tag))
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
                &texts.together,
                &texts.pause,
                &texts.frontmost_only,
                &texts.collisions,
                &texts.size,
                &texts.size_small,
                &texts.size_normal,
                &texts.size_large,
                &texts.size_huge,
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
        assert_eq!(language(&tags(&["sv-SE", "es-419", "fr-FR"])), "es");
        assert_eq!(language(&tags(&["EN-gb"])), "en");
        assert_eq!(language(&tags(&["fr_CA"])), "fr");
        assert_eq!(language(&tags(&["de-CH"])), "de");
        assert_eq!(language(&tags(&["ja-JP"])), "ja");
        // Une langue qu'il ne parle pas encore, ou aucune : l'anglais.
        assert_eq!(language(&tags(&["sv-SE", "fi-FI"])), "en");
        assert_eq!(language(&[]), "en");
    }

    #[test]
    fn tells_the_two_portugueses_apart() {
        assert_eq!(language(&tags(&["pt-BR"])), "pt");
        assert_eq!(language(&tags(&["pt"])), "pt");
        assert_eq!(language(&tags(&["pt-PT"])), "pt-PT");
        assert_eq!(language(&tags(&["pt_AO"])), "pt-PT");
    }

    #[test]
    fn tells_simplified_and_traditional_chinese_apart() {
        assert_eq!(language(&tags(&["zh-Hans-CN"])), "zh-Hans");
        assert_eq!(language(&tags(&["zh-Hant-TW"])), "zh-Hant");
        assert_eq!(language(&tags(&["zh-Hant-HK"])), "zh-Hant");
        // Écriture traditionnelle à Hong Kong si elle n'est pas dite, simplifiée sinon.
        assert_eq!(language(&tags(&["zh-HK"])), "zh-Hant");
        assert_eq!(language(&tags(&["zh-TW"])), "zh-Hant");
        assert_eq!(language(&tags(&["zh-CN"])), "zh-Hans");
        assert_eq!(language(&tags(&["zh"])), "zh-Hans");
        assert_eq!(language(&tags(&["zh-Hans-HK"])), "zh-Hans");
    }

    #[test]
    fn speaks_every_language_it_lists() {
        assert_eq!(parse("fr").quit, "Quitter Mouflon");
        assert_eq!(parse("es").quit, "Salir de Mouflon");
        assert_eq!(parse("de").quit, "Mouflon beenden");
        assert_eq!(parse("ja").quit, "Mouflonを終了");
        assert_eq!(parse("zh-Hant").quit, "結束 Mouflon");
        assert_eq!(parse("xx").quit, "Quit Mouflon");
        // Chaque code se retrouve tel quel depuis une étiquette du système.
        for (code, _) in LOCALES {
            assert_eq!(language(&tags(&[code])), *code);
        }
    }

    #[test]
    fn the_app_bundle_lists_every_language() {
        let plist = include_str!("../../../packaging/macos/Info.plist.in");
        for (code, _) in LOCALES {
            assert!(
                plist.contains(&format!("<string>{code}</string>")),
                "{code} manque dans CFBundleLocalizations"
            );
        }
    }
}
