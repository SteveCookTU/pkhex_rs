use crate::game::enums::LanguageID;

impl LanguageID {
    pub fn get_language_2_char_name(&self) -> &'static str {
        match self {
            LanguageID::Japanese => "ja",
            LanguageID::French => "fr",
            LanguageID::Italian => "it",
            LanguageID::German => "de",
            LanguageID::Spanish => "es",
            LanguageID::Korean => "ko",
            LanguageID::ChineseS | LanguageID::ChineseT => "zh",
            _ => "en",
        }
    }
}
