use crate::game::enums::LanguageGC;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum LanguageID {
    Hacked,
    Japanese,
    English,
    French,
    Italian,
    German,
    Unused6,
    Spanish,
    Korean,
    ChineseS,
    ChineseT,
}

impl From<LanguageGC> for LanguageID {
    fn from(gc: LanguageGC) -> Self {
        match gc {
            LanguageGC::Hacked => LanguageID::Hacked,
            LanguageGC::Japanese => LanguageID::Japanese,
            LanguageGC::English => LanguageID::English,
            LanguageGC::German => LanguageID::German,
            LanguageGC::French => LanguageID::French,
            LanguageGC::Italian => LanguageID::Italian,
            LanguageGC::Spanish => LanguageID::Spanish,
            _ => LanguageID::English,
        }
    }
}
