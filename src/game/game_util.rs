use crate::game::enums::GameVersion;
use crate::legality;
use crate::pkm::traits::metadata::GameValueLimit;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GAME_VERSIONS: Vec<GameVersion> = get_valid_game_versions();
}

fn get_valid_game_versions() -> Vec<GameVersion> {
    let mut filtered = enum_iterator::all::<GameVersion>()
        .filter(GameVersion::is_valid_saved_version)
        .collect::<Vec<_>>();
    filtered.reverse();
    filtered
}

impl GameVersion {
    pub fn is_valid_saved_version(&self) -> bool {
        *self > GameVersion::Any && *self <= HIGHEST_GAME_ID
    }

    pub fn contains_raw(&self, g2: u8) -> bool {
        self.contains(&GameVersion::from(g2))
    }

    pub fn contains(&self, g2: &GameVersion) -> bool {
        if self == g2 || self == &GameVersion::Any {
            return true;
        }

        match self {
            GameVersion::RB => [GameVersion::RB, GameVersion::BU, GameVersion::GN].contains(g2),
            GameVersion::RBY | GameVersion::Stadium => {
                GameVersion::RB.contains(g2) || g2 == &GameVersion::Stadium
            }
            GameVersion::Gen1 => GameVersion::RBY.contains(g2) || g2 == &GameVersion::Stadium,

            GameVersion::GS => [GameVersion::GD, GameVersion::SI].contains(g2),
            GameVersion::GSC | GameVersion::Stadium2 => {
                GameVersion::GS.contains(g2) || g2 == &GameVersion::Stadium2
            }
            GameVersion::Gen2 => GameVersion::GSC.contains(g2) || g2 == &GameVersion::Stadium,

            GameVersion::RS => [GameVersion::R, GameVersion::S].contains(g2),
            GameVersion::RSE => GameVersion::RS.contains(g2) || g2 == &GameVersion::E,
            GameVersion::FRLG => [GameVersion::FR, GameVersion::LG].contains(g2),
            GameVersion::COLO | GameVersion::XD => g2 == &GameVersion::CXD,
            GameVersion::CXD => [GameVersion::COLO, GameVersion::XD].contains(g2),
            GameVersion::RSBOX => {
                GameVersion::RS.contains(g2)
                    || g2 == &GameVersion::E
                    || GameVersion::FRLG.contains(g2)
            }
            GameVersion::Gen3 => {
                GameVersion::RSE.contains(g2)
                    || GameVersion::FRLG.contains(g2)
                    || GameVersion::CXD.contains(g2)
                    || g2 == &GameVersion::RSBOX
            }

            GameVersion::DP => [GameVersion::D, GameVersion::P].contains(g2),
            GameVersion::HGSS => [GameVersion::HG, GameVersion::SS].contains(g2),
            GameVersion::DPPt => GameVersion::DP.contains(g2) || g2 == &GameVersion::Pt,
            GameVersion::BATREV => {
                GameVersion::DP.contains(g2)
                    || g2 == &GameVersion::Pt
                    || GameVersion::HGSS.contains(g2)
            }
            GameVersion::Gen4 => {
                GameVersion::DPPt.contains(g2)
                    || GameVersion::HGSS.contains(g2)
                    || g2 == &GameVersion::BATREV
            }

            GameVersion::BW => [GameVersion::B, GameVersion::W].contains(g2),
            GameVersion::B2W2 => [GameVersion::B2, GameVersion::W2].contains(g2),
            GameVersion::Gen5 => GameVersion::BW.contains(g2) || GameVersion::B2W2.contains(g2),

            GameVersion::XY => [GameVersion::X, GameVersion::Y].contains(g2),
            GameVersion::ORAS => [GameVersion::OR, GameVersion::AS].contains(g2),
            GameVersion::Gen6 => GameVersion::XY.contains(g2) || GameVersion::ORAS.contains(g2),

            GameVersion::SM => [GameVersion::SN, GameVersion::MN].contains(g2),
            GameVersion::USUM => [GameVersion::US, GameVersion::UM].contains(g2),
            GameVersion::GG => [GameVersion::GP, GameVersion::GE].contains(g2),
            GameVersion::Gen7 => GameVersion::SM.contains(g2) || GameVersion::USUM.contains(g2),
            GameVersion::Gen7b => GameVersion::GG.contains(g2) || g2 == &GameVersion::GO,

            GameVersion::SWSH => [GameVersion::SW, GameVersion::SH].contains(g2),
            GameVersion::BDSP => [GameVersion::BD, GameVersion::SP].contains(g2),
            GameVersion::Gen8 => {
                GameVersion::SWSH.contains(g2)
                    || GameVersion::BDSP.contains(g2)
                    || g2 == &GameVersion::PLA
            }

            GameVersion::SV => [GameVersion::SL, GameVersion::VL].contains(g2),
            GameVersion::Gen9 => GameVersion::SV.contains(g2),
            _ => false,
        }
    }

    pub fn get_generation(&self) -> Option<u8> {
        if GameVersion::Gen1.contains(self) {
            Some(1)
        } else if GameVersion::Gen2.contains(self) {
            Some(2)
        } else if GameVersion::Gen3.contains(self) {
            Some(3)
        } else if GameVersion::Gen4.contains(self) {
            Some(4)
        } else if GameVersion::Gen5.contains(self) {
            Some(5)
        } else if GameVersion::Gen6.contains(self) {
            Some(6)
        } else if GameVersion::Gen7.contains(self) || GameVersion::Gen7b.contains(self) {
            Some(7)
        } else if GameVersion::Gen8.contains(self) {
            Some(8)
        } else if GameVersion::Gen9.contains(self) {
            Some(9)
        } else {
            None
        }
    }

    pub fn get_max_species_id(&self) -> u16 {
        if GameVersion::Gen1.contains(self) {
            legality::tables1::MAX_SPECIES_ID_1
        } else if GameVersion::Gen2.contains(self) {
            legality::tables2::MAX_SPECIES_ID_2
        } else if GameVersion::Gen3.contains(self) {
            legality::tables3::MAX_SPECIES_ID_3
        } else if GameVersion::Gen4.contains(self) {
            legality::tables4::MAX_SPECIES_ID_4
        } else if GameVersion::Gen5.contains(self) {
            legality::tables5::MAX_SPECIES_ID_5
        } else if GameVersion::Gen6.contains(self) {
            legality::tables6::MAX_SPECIES_ID_6
        } else if GameVersion::Gen7b.contains(self) {
            legality::tables7b::MAX_SPECIES_ID_7B
        } else if GameVersion::Gen7.contains(self) {
            if GameVersion::SM.contains(self) {
                legality::tables7::MAX_SPECIES_ID_7
            } else {
                legality::tables7::MAX_SPECIES_ID_7_USUM
            }
        } else if self == &GameVersion::PLA {
            legality::tables8a::MAX_SPECIES_ID_8A
        } else if GameVersion::BDSP.contains(self) {
            legality::tables8bs::MAX_SPECIES_ID_8B
        } else if GameVersion::Gen8.contains(self) {
            legality::tables8::MAX_SPECIES_ID_8
        } else if GameVersion::Gen9.contains(self) {
            legality::tables9::MAX_SPECIES_ID_9
        } else {
            0
        }
    }
}

pub(crate) const HIGHEST_GAME_ID: GameVersion = GameVersion::VL;

pub fn get_met_location_version_group(version: GameVersion) -> GameVersion {
    match version {
        GameVersion::CXD => GameVersion::CXD,
        GameVersion::GO => GameVersion::GO,
        GameVersion::RD
        | GameVersion::BU
        | GameVersion::YW
        | GameVersion::GN
        | GameVersion::GD
        | GameVersion::SI
        | GameVersion::C => GameVersion::USUM,
        GameVersion::GS | GameVersion::GSC => GameVersion::GSC,
        GameVersion::R | GameVersion::S => GameVersion::RS,
        GameVersion::E => GameVersion::E,
        GameVersion::FR | GameVersion::LG => GameVersion::FR,
        GameVersion::D | GameVersion::P => GameVersion::DP,
        GameVersion::Pt => GameVersion::Pt,
        GameVersion::HG | GameVersion::SS => GameVersion::HGSS,
        GameVersion::B | GameVersion::W => GameVersion::BW,
        GameVersion::B2 | GameVersion::W2 => GameVersion::B2W2,
        GameVersion::X | GameVersion::Y => GameVersion::XY,
        GameVersion::OR | GameVersion::AS => GameVersion::ORAS,
        GameVersion::SN | GameVersion::MN => GameVersion::SM,
        GameVersion::US | GameVersion::UM => GameVersion::USUM,
        GameVersion::GP | GameVersion::GE => GameVersion::GG,
        GameVersion::SW | GameVersion::SH => GameVersion::SWSH,
        GameVersion::BD | GameVersion::SP => GameVersion::BDSP,
        GameVersion::PLA => GameVersion::PLA,
        GameVersion::SL | GameVersion::VL => GameVersion::SV,
        _ => GameVersion::Invalid,
    }
}

pub fn get_version(generation: u8) -> GameVersion {
    match generation {
        1 => GameVersion::RBY,
        2 => GameVersion::C,
        3 => GameVersion::E,
        4 => GameVersion::SS,
        5 => GameVersion::W2,
        6 => GameVersion::AS,
        7 => GameVersion::UM,
        8 => GameVersion::SH,
        9 => GameVersion::VL,
        _ => GameVersion::Invalid,
    }
}

pub fn get_versions_in_generation(generation: u8, pk_version: u8) -> Vec<GameVersion> {
    if GameVersion::Gen7b.contains_raw(pk_version) {
        vec![GameVersion::GO, GameVersion::GP, GameVersion::GE]
    } else {
        enum_iterator::all::<GameVersion>()
            .filter(|gv| {
                if let Some(gen) = gv.get_generation() {
                    gen == generation
                } else {
                    false
                }
            })
            .collect()
    }
}

pub fn get_versions_within_range(
    obj: impl GameValueLimit,
    generation: Option<u8>,
) -> Vec<GameVersion> {
    let max = obj.max_game_id();
    if max == legality::tables7b::MAX_GAME_ID_7B {
        return vec![GameVersion::GO, GameVersion::GP, GameVersion::GE];
    }

    let mut versions = enum_iterator::all::<GameVersion>()
        .filter(|version| {
            &GameVersion::from(obj.min_game_id()) <= version && version <= &GameVersion::from(max)
        })
        .collect::<Vec<_>>();

    if let Some(generation) = generation {
        if max == legality::tables7::MAX_GAME_ID_7 && generation == 7 {
            versions.retain(|v| v != &GameVersion::GO);
        }
        versions
            .into_iter()
            .filter(|v| v.get_generation().unwrap_or_default() <= generation)
            .collect()
    } else {
        versions
    }
}
