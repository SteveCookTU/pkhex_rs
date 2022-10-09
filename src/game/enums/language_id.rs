#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LanguageID {
    Hacked = 0,
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

impl From<u8> for LanguageID {
    fn from(val: u8) -> Self {
        match val {
            1 => LanguageID::Japanese,
            2 => LanguageID::English,
            3 => LanguageID::French,
            4 => LanguageID::Italian,
            5 => LanguageID::German,
            6 => LanguageID::Unused6,
            7 => LanguageID::Spanish,
            8 => LanguageID::Korean,
            9 => LanguageID::ChineseS,
            10 => LanguageID::ChineseT,
            _ => LanguageID::Hacked,
        }
    }
}
