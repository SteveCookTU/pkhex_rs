use crate::{GameVersion, PersonalInfo, Pkm};

#[derive(PartialEq)]
#[repr(u8)]
pub enum ShinyEnum {
    Random,
    Never,
    Always,
    AlwaysStar,
    AlwaysSquare,
    FixedValue,
}

impl ShinyEnum {
    pub fn is_valid<I: PersonalInfo, T: Pkm<I>>(&self, pkm: &T) -> bool {
        match self {
            ShinyEnum::Never => pkm.is_shiny(),
            ShinyEnum::Always => !pkm.is_shiny(),
            ShinyEnum::AlwaysStar => pkm.shiny_xor() == 0,
            ShinyEnum::AlwaysSquare => pkm.shiny_xor() == 1,
            _ => true,
        }
    }

    pub fn is_shiny(&self) -> bool {
        matches!(
            self,
            ShinyEnum::Always | ShinyEnum::AlwaysStar | ShinyEnum::AlwaysSquare
        )
    }

    pub fn get_type<I: PersonalInfo, T: Pkm<I>>(pkm: &T) -> ShinyEnum {
        let shiny = pkm.is_shiny();
        if !shiny {
            return ShinyEnum::Never;
        }

        if ShinyEnum::is_square_shiny_exist(pkm) {
            return ShinyEnum::AlwaysSquare;
        }

        ShinyEnum::AlwaysStar
    }

    pub fn is_square_shiny_exist<I: PersonalInfo, T: Pkm<I>>(pkm: &T) -> bool {
        if pkm.format() < 8 {
            false
        } else {
            pkm.shiny_xor() == 0
                || pkm.get_fateful_encounter()
                || pkm.get_version() == GameVersion::GO as usize
        }
    }
}
