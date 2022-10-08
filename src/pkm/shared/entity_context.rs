use crate::GameVersion;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum EntityContext {
    None = 0,
    Gen1,
    Gen2,
    Gen3,
    Gen4,
    Gen5,
    Gen6,
    Gen7,
    Gen8,

    SplitInvalid,
    Gen7b,
    Gen8a,
    Gen8b,

    MaxInvalid,
}

impl From<u8> for EntityContext {
    fn from(v: u8) -> Self {
        match v {
            0 => EntityContext::None,
            1 => EntityContext::Gen1,
            2 => EntityContext::Gen2,
            3 => EntityContext::Gen3,
            4 => EntityContext::Gen4,
            5 => EntityContext::Gen5,
            6 => EntityContext::Gen6,
            7 => EntityContext::Gen7,
            8 => EntityContext::Gen8,
            9 => EntityContext::SplitInvalid,
            10 => EntityContext::Gen7b,
            11 => EntityContext::Gen8a,
            12 => EntityContext::Gen8b,
            _ => EntityContext::MaxInvalid,
        }
    }
}

impl EntityContext {
    pub fn generation(&self) -> u8 {
        if self < &EntityContext::SplitInvalid {
            *self as u8
        } else {
            match self {
                EntityContext::Gen7b => 7,
                EntityContext::Gen8a => 8,
                EntityContext::Gen8b => 8,
                _ => panic!("Invalid value for generation in EntityContext"),
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        *self as u8 != 0
            && *self != EntityContext::SplitInvalid
            && *self < EntityContext::MaxInvalid
    }

    pub fn get_single_game_version(&self) -> GameVersion {
        match self {
            EntityContext::Gen1 => GameVersion::RD,
            EntityContext::Gen2 => GameVersion::C,
            EntityContext::Gen3 => GameVersion::E,
            EntityContext::Gen4 => GameVersion::SS,
            EntityContext::Gen5 => GameVersion::W2,
            EntityContext::Gen6 => GameVersion::AS,
            EntityContext::Gen7 => GameVersion::UM,
            EntityContext::Gen8 => GameVersion::SH,
            EntityContext::Gen7b => GameVersion::GP,
            EntityContext::Gen8a => GameVersion::PLA,
            EntityContext::Gen8b => GameVersion::BD,
            _ => panic!("Invalid value for generation in EntityContext"),
        }
    }
}

impl GameVersion {
    pub fn get_context(&self) -> EntityContext {
        match self {
            GameVersion::GP | GameVersion::GE => EntityContext::Gen7b,
            GameVersion::PLA => EntityContext::Gen8a,
            GameVersion::BD | GameVersion::SP => EntityContext::Gen8b,
            _ => EntityContext::from(self.get_generation()),
        }
    }
}
