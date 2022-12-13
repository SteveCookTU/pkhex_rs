use crate::game::enums::GameVersion;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum GCVersion {
    None,
    FR,
    LG,
    S = 8,
    R,
    E,
    CXD,
}

impl GameVersion {
    pub fn get_cxd_version_id(&self) -> GCVersion {
        match self {
            GameVersion::S => GCVersion::S,
            GameVersion::R => GCVersion::R,
            GameVersion::E => GCVersion::E,
            GameVersion::FR => GCVersion::FR,
            GameVersion::LG => GCVersion::LG,
            GameVersion::CXD => GCVersion::CXD,
            _ => GCVersion::None,
        }
    }
}

impl GCVersion {
    pub fn get_g3_version_id(&self) -> GameVersion {
        match self {
            GCVersion::S => GameVersion::S,
            GCVersion::R => GameVersion::R,
            GCVersion::E => GameVersion::E,
            GCVersion::FR => GameVersion::FR,
            GCVersion::LG => GameVersion::LG,
            GCVersion::CXD => GameVersion::CXD,
            _ => GameVersion::Unknown,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum GCRegion {
    NoRegion,
    NtscJ,
    NtscU,
    PAL,
}
