use crate::LanguageID;

const LANGUAGES_3: [usize; 6] = [
    LanguageID::Japanese as usize,
    LanguageID::English as usize,
    LanguageID::French as usize,
    LanguageID::German as usize,
    LanguageID::Spanish as usize,
    LanguageID::Italian as usize,
];

const LANGUAGES_GB: [usize; 7] = [
    LanguageID::Japanese as usize,
    LanguageID::English as usize,
    LanguageID::French as usize,
    LanguageID::German as usize,
    LanguageID::Spanish as usize,
    LanguageID::Italian as usize,
    LanguageID::Korean as usize,
];

const LANGUAGES_46: [usize; 7] = LANGUAGES_GB;

const LANGUAGES_7: [usize; 9] = [
    LanguageID::Japanese as usize,
    LanguageID::English as usize,
    LanguageID::French as usize,
    LanguageID::German as usize,
    LanguageID::Spanish as usize,
    LanguageID::Italian as usize,
    LanguageID::Korean as usize,
    LanguageID::ChineseS as usize,
    LanguageID::ChineseT as usize,
];

const SAFE_LANGUAGE: LanguageID = LanguageID::English;

pub fn get_available_game_languages(generation: usize) -> &'static [usize] {
    match generation {
        _ if generation < 3 => &LANGUAGES_GB,
        _ if generation < 4 => &LANGUAGES_3,
        _ if generation < 7 => &LANGUAGES_46,
        _ => &LANGUAGES_7,
    }
}
