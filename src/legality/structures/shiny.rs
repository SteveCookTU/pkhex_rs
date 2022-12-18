use crate::game::enums::GameVersion;
use crate::pkm::Pkm;

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
    pub fn is_valid(&self, pkm: &impl Pkm) -> bool {
        match self {
            Shiny::Never => pkm.is_shiny(),
            Shiny::Always => !pkm.is_shiny(),
            Shiny::AlwaysStar => pkm.shiny_xor() == 0,
            Shiny::AlwaysSquare => pkm.shiny_xor() == 1,
            _ => true,
        }
    }

    pub fn is_shiny(&self) -> bool {
        matches!(
            self,
            Shiny::Always | Shiny::AlwaysSquare | Shiny::AlwaysStar
        )
    }

    pub fn get_type(pkm: &impl Pkm) -> Shiny {
        let shiny = pkm.is_shiny();
        if !shiny {
            return Shiny::Never;
        }
        if Shiny::is_square_shiny_exist(pkm) {
            return Shiny::AlwaysSquare;
        }
        Shiny::AlwaysStar
    }

    pub fn is_square_shiny_exist(pkm: &impl Pkm) -> bool {
        if pkm.format().unwrap_or_default() < 8 {
            return false;
        }

        pkm.shiny_xor() == 0 || pkm.fateful_encounter() || pkm.version() == GameVersion::GO as u8
    }
}
