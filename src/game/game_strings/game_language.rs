pub const DEFAULT_LANGUAGE: &str = "en";
pub const LANGUAGE_CODES: [&str; 9] = ["ja", "en", "fr", "it", "de", "es", "ko", "zh", "zh2"];

pub const fn default_language_index() -> usize {
    1
}

pub const fn language_to_char(lang: usize) -> &'static str{
    if lang > LANGUAGE_CODES.len() {
        DEFAULT_LANGUAGE
    } else {
        LANGUAGE_CODES[lang]
    }
}

pub const fn language_count() -> usize {
    LANGUAGE_CODES.len()
}

pub fn get_language_index(lang: &str) -> usize {
    if let Some(l) = LANGUAGE_CODES.iter().position(|&s| s == lang) {
        l
    } else {
        default_language_index()
    }
}

const P_TRANSP: [&str; 9] = ["ポケシフター", "Poké Transfer", "Poké Fret", "Pokétrasporto", "Poképorter", "Pokétransfer", "포케시프터", "宝可传送", "寶可傳送"];

pub const fn get_transporter_name(mut index: usize) -> &'static str {
    if index >= P_TRANSP.len() {
        index = 2;
    }
    P_TRANSP[index]
}

pub fn get_transporter_name_from_lang(lang: &str) -> &'static str {
    get_transporter_name(get_language_index(lang))
}

pub enum ProgramLanguage {
    Japanese,
    English,
    French,
    Italian,
    German,
    Spanish,
    Korean,
    Chinese
}