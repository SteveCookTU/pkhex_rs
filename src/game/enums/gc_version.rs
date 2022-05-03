use crate::GameVersion;

#[derive(PartialEq)]
#[repr(u8)]
pub enum GCVersion {
    None = 0,
    FR = 1,
    LG = 2,
    S = 8,
    R = 9,
    E = 10,
    Cxd = 11,
}

impl From<GameVersion> for GCVersion {
    fn from(gv: GameVersion) -> Self {
        match gv {
            GameVersion::S => GCVersion::S,
            GameVersion::R => GCVersion::R,
            GameVersion::E => GCVersion::E,
            GameVersion::FR => GCVersion::FR,
            GameVersion::LG => GCVersion::LG,
            GameVersion::CXD => GCVersion::Cxd,
            _ => GCVersion::None,
        }
    }
}

impl From<GCVersion> for GameVersion {
    fn from(gcv: GCVersion) -> Self {
        match gcv {
            GCVersion::S => GameVersion::S,
            GCVersion::R => GameVersion::R,
            GCVersion::E => GameVersion::E,
            GCVersion::FR => GameVersion::FR,
            GCVersion::LG => GameVersion::LG,
            GCVersion::Cxd => GameVersion::CXD,
            _ => GameVersion::Unknown,
        }
    }
}
