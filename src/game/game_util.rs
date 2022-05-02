use crate::{tables, GameVersion};

impl GameVersion {
    pub fn get_max_species_id(&self) -> usize {
        if GameVersion::Gen1.contains(self) {
            tables::MAX_SPECIES_ID_1
        } else if GameVersion::Gen2.contains(self) {
            tables::MAX_SPECIES_ID_2
        } else if GameVersion::Gen3.contains(self) {
            tables::MAX_SPECIES_ID_3
        } else if GameVersion::Gen4.contains(self) {
            tables::MAX_SPECIES_ID_4
        } else if GameVersion::Gen5.contains(self) {
            tables::MAX_SPECIES_ID_5
        } else if GameVersion::Gen6.contains(self) {
            tables::MAX_SPECIES_ID_6
        } else if GameVersion::Gen7.contains(self) {
            if GameVersion::SM.contains(self) {
                tables::MAX_SPECIES_ID_7
            } else {
                tables::MAX_SPECIES_ID_7_USUM
            }
        } else if *self == GameVersion::PLA {
            tables::MAX_SPECIES_ID_8A
        } else if GameVersion::BDSP.contains(self) {
            tables::MAX_SPECIES_ID_8B
        } else if GameVersion::Gen8.contains(self) {
            tables::MAX_SPECIES_ID_8
        } else {
            0
        }
    }

    pub fn contains_raw(&self, other: usize) -> bool {
        self.contains(&other.into())
    }

    pub fn contains(&self, other: &Self) -> bool {
        if self == other || *self == GameVersion::Any {
            return true;
        }
        match self {
            GameVersion::RB => {
                *other == GameVersion::RD || *other == GameVersion::BU || *other == GameVersion::GN
            }
            GameVersion::RBY | GameVersion::Stadium => {
                GameVersion::RB.contains(other) || *other == GameVersion::YW
            }
            GameVersion::Gen1 => GameVersion::RBY.contains(other) || *other == GameVersion::Stadium,

            GameVersion::GS => *other == GameVersion::GD || *other == GameVersion::SI,
            GameVersion::GSC | GameVersion::Stadium2 => {
                GameVersion::GS.contains(other) || *other == GameVersion::C
            }
            GameVersion::Gen2 => {
                GameVersion::GSC.contains(other) || *other == GameVersion::Stadium2
            }

            GameVersion::RS => *other == GameVersion::R || *other == GameVersion::S,
            GameVersion::RSE => GameVersion::RS.contains(other) || *other == GameVersion::E,
            GameVersion::FRLG => *other == GameVersion::FR || *other == GameVersion::LG,
            GameVersion::COLO | GameVersion::XD => *other == GameVersion::CXD,
            GameVersion::CXD => *other == GameVersion::COLO || *other == GameVersion::XD,
            GameVersion::RSBOX => {
                GameVersion::RS.contains(other)
                    || *other == GameVersion::E
                    || GameVersion::FRLG.contains(other)
            }
            GameVersion::Gen3 => {
                GameVersion::RSE.contains(other)
                    || GameVersion::FRLG.contains(other)
                    || GameVersion::CXD.contains(other)
                    || *other == GameVersion::RSBOX
            }

            GameVersion::DP => *other == GameVersion::D || *other == GameVersion::P,
            GameVersion::HGSS => *other == GameVersion::HG || *other == GameVersion::SS,
            GameVersion::DPPt => GameVersion::DP.contains(other) || *other == GameVersion::Pt,
            GameVersion::BATREV => {
                GameVersion::DP.contains(other)
                    || *other == GameVersion::Pt
                    || GameVersion::HGSS.contains(other)
            }
            GameVersion::Gen4 => {
                GameVersion::DPPt.contains(other)
                    || GameVersion::HGSS.contains(other)
                    || *other == GameVersion::BATREV
            }

            GameVersion::BW => *other == GameVersion::B || *other == GameVersion::W,
            GameVersion::B2W2 => *other == GameVersion::B2 || *other == GameVersion::W2,
            GameVersion::Gen5 => {
                GameVersion::BW.contains(other) || GameVersion::B2W2.contains(other)
            }

            GameVersion::XY => *other == GameVersion::X || *other == GameVersion::Y,
            GameVersion::ORAS => *other == GameVersion::OR || *other == GameVersion::AS,
            GameVersion::Gen6 => {
                GameVersion::XY.contains(other) || GameVersion::ORAS.contains(other)
            }

            GameVersion::SM => *other == GameVersion::SN || *other == GameVersion::MN,
            GameVersion::USUM => *other == GameVersion::US || *other == GameVersion::UM,
            GameVersion::GG => *other == GameVersion::GP || *other == GameVersion::GE,
            GameVersion::Gen7 => {
                GameVersion::SM.contains(other) || GameVersion::USUM.contains(other)
            }
            GameVersion::Gen7b => GameVersion::GG.contains(other) || *other == GameVersion::GO,

            GameVersion::SWSH => *other == GameVersion::SW || *other == GameVersion::SH,
            GameVersion::BDSP => *other == GameVersion::BD || *other == GameVersion::SP,
            GameVersion::Gen8 => {
                GameVersion::SWSH.contains(other)
                    || GameVersion::BDSP.contains(other)
                    || *other == GameVersion::PLA
            }
            _ => false,
        }
    }
}
