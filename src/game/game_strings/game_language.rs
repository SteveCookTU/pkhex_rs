use crate::resource_util;
use std::sync::Arc;

pub const DEFAULT_LANGUAGE: &str = "en";
pub const DEFAULT_LANGUAGE_INDEX: usize = 2;

pub const fn language_2_char(lang: usize) -> &'static str {
    if lang > LANGUAGE_CODES.len() {
        DEFAULT_LANGUAGE
    } else {
        LANGUAGE_CODES[lang]
    }
}

pub const LANGUAGE_COUNT: usize = LANGUAGE_CODES.len();

pub fn get_language_index(lang: &str) -> usize {
    let l = LANGUAGE_CODES.iter().position(|&s| s == lang);
    l.unwrap_or(DEFAULT_LANGUAGE_INDEX)
}

const LANGUAGE_CODES: [&str; 9] = ["ja", "en", "fr", "it", "de", "es", "ko", "zh", "zh2"];

const P_TRANS_P: [&str; 9] = [
    "ポケシフター",
    "Poké Transfer",
    "Poké Fret",
    "Pokétrasporto",
    "Poképorter",
    "Pokétransfer",
    "포케시프터",
    "宝可传送",
    "寶可傳送",
];

pub fn get_transporter_name(mut index: usize) -> &'static str {
    if index >= P_TRANS_P.len() {
        index = 2;
    }
    P_TRANS_P[index]
}

pub fn get_transporter_name_from_lang(lang: &str) -> &'static str {
    get_transporter_name(get_language_index(lang))
}

pub fn get_strings(
    ident: impl AsRef<str>,
    lang: impl AsRef<str>,
    data_type: Option<&str>,
) -> Arc<Vec<&'static str>> {
    let mut data = resource_util::get_string_list_from_param(ident.as_ref(), lang, data_type);
    if data.is_empty() {
        data = resource_util::get_string_list_from_param(ident, DEFAULT_LANGUAGE, data_type);
    }
    data
}
