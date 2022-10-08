use crate::{GameVersion, PersonalInfo, PKM};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Shiny {
    Random,
    Never,
    Always,
    AlwaysStar,
    AlwaysSquare,
    FixedValue,
}

impl Shiny {
    pub fn is_valid<Personal: PersonalInfo + 'static, T: PKM<Personal>>(&self, pk: &T) -> bool {
        match self {
            Shiny::Always => pk.is_shiny(),
            Shiny::Never => !pk.is_shiny(),
            Shiny::AlwaysSquare => pk.shiny_xor() == 0,
            Shiny::AlwaysStar => pk.shiny_xor() == 1,
            _ => true,
        }
    }

    pub fn is_shiny(&self) -> bool {
        matches!(
            self,
            Shiny::Always | Shiny::AlwaysStar | Shiny::AlwaysSquare
        )
    }
}

pub fn get_type<Personal: PersonalInfo + 'static, T: PKM<Personal>>(pk: &T) -> Shiny {
    let shiny = pk.is_shiny();
    if !shiny {
        Shiny::Never
    } else if is_square_shiny_exist(pk) {
        Shiny::AlwaysSquare
    } else {
        Shiny::AlwaysStar
    }
}

pub fn is_square_shiny_exist<Personal: PersonalInfo + 'static, T: PKM<Personal>>(pk: &T) -> bool {
    if pk.format() < 8 {
        false
    } else {
        pk.shiny_xor() == 0
            || pk.get_fateful_encounter()
            || pk.get_version() == GameVersion::GO as u8
    }
}
