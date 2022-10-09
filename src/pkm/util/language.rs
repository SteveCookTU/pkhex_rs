use crate::{LanguageGC, LanguageID};

const LANGUAGES: [u8; 9] = [
    LanguageID::Japanese as u8,
    LanguageID::English as u8,
    LanguageID::French as u8,
    LanguageID::German as u8,
    LanguageID::Spanish as u8,
    LanguageID::Italian as u8,
    LanguageID::Korean as u8,
    LanguageID::ChineseS as u8,
    LanguageID::ChineseT as u8,
];

const LANGUAGES_GB: [u8; 7] = [
    LanguageID::Japanese as u8,
    LanguageID::English as u8,
    LanguageID::French as u8,
    LanguageID::German as u8,
    LanguageID::Spanish as u8,
    LanguageID::Italian as u8,
    LanguageID::Korean as u8,
];

const LANGUAGES_3: [u8; 6] = [
    LanguageID::Japanese as u8,
    LanguageID::English as u8,
    LanguageID::French as u8,
    LanguageID::German as u8,
    LanguageID::Spanish as u8,
    LanguageID::Italian as u8,
];

const SAFE_LANGUAGE: LanguageID = LanguageID::English;

pub fn get_main_lang_id_from_gc(value: u8) -> u8 {
    match LanguageGC::from(value) {
        LanguageGC::German => LanguageID::German as u8,
        LanguageGC::French => LanguageID::French as u8,
        LanguageGC::Italian => LanguageID::Italian as u8,
        LanguageGC::Spanish => LanguageID::Spanish as u8,
        LanguageGC::Unused6 => LanguageID::Unused6 as u8,
        _ => value,
    }
}

pub fn get_gc_lang_id_from_main(value: u8) -> u8 {
    match LanguageID::from(value) {
        LanguageID::German => LanguageGC::German as u8,
        LanguageID::French => LanguageGC::French as u8,
        LanguageID::Italian => LanguageGC::Italian as u8,
        LanguageID::Spanish => LanguageGC::Spanish as u8,
        LanguageID::Unused6 => LanguageGC::Unused6 as u8,
        _ => value,
    }
}
