use crate::game::enums::LanguageID;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum LanguageGC {
    Hacked,
    Japanese,
    English,
    German,
    French,
    Italian,
    Spanish,
    Unused6,
}

impl From<LanguageID> for LanguageGC {
    fn from(id: LanguageID) -> Self {
        match id {
            LanguageID::Hacked => LanguageGC::Hacked,
            LanguageID::Japanese => LanguageGC::Japanese,
            LanguageID::English => LanguageGC::English,
            LanguageID::German => LanguageGC::German,
            LanguageID::French => LanguageGC::French,
            LanguageID::Italian => LanguageGC::Italian,
            LanguageID::Spanish => LanguageGC::Spanish,
            _ => LanguageGC::English,
        }
    }
}
