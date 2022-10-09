use crate::GameVersion;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum GCVersion {
    None = 0,
    FR,
    LG,
    S = 8,
    R,
    E,
    CXD,
}

impl From<u8> for GCVersion {
    fn from(val: u8) -> Self {
        match val {
            1 => GCVersion::FR,
            2 => GCVersion::LG,
            8 => GCVersion::S,
            9 => GCVersion::R,
            10 => GCVersion::E,
            11 => GCVersion::CXD,
            _ => GCVersion::None,
        }
    }
}

impl From<GCVersion> for GameVersion {
    fn from(gc: GCVersion) -> Self {
        match gc {
            GCVersion::FR => GameVersion::FR,
            GCVersion::LG => GameVersion::LG,
            GCVersion::S => GameVersion::S,
            GCVersion::R => GameVersion::R,
            GCVersion::E => GameVersion::E,
            GCVersion::CXD => GameVersion::CXD,
            _ => GameVersion::Unknown,
        }
    }
}

impl From<GameVersion> for GCVersion {
    fn from(gv: GameVersion) -> Self {
        match gv {
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
