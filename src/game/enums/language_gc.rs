#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LanguageGC {
    Hacked = 0,
    Japanese,
    English,
    German,
    French,
    Italian,
    Spanish,
    Unused6,
}

impl From<u8> for LanguageGC {
    fn from(val: u8) -> Self {
        match val {
            1 => LanguageGC::Japanese,
            2 => LanguageGC::English,
            3 => LanguageGC::German,
            4 => LanguageGC::French,
            5 => LanguageGC::Italian,
            6 => LanguageGC::Spanish,
            7 => LanguageGC::Unused6,
            _ => LanguageGC::Hacked,
        }
    }
}
