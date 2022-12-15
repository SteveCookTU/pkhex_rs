use crate::game::enums::GameVersion;
use crate::{PKError, PKResult};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum EntityContext {
    #[num_enum(default)]
    None = 0,
    Gen1 = 1,
    Gen2 = 2,
    Gen3 = 3,
    Gen4 = 4,
    Gen5 = 5,
    Gen6 = 6,
    Gen7 = 7,
    Gen8 = 8,
    Gen9 = 9,

    SplitInvalid,
    Gen7b,
    Gen8a,
    Gen8b,

    MaxInvalid,
}

impl EntityContext {
    pub fn generation(&self) -> PKResult<u8> {
        let val = *self as u8;
        if val < EntityContext::SplitInvalid as u8 {
            Ok(val)
        } else {
            match self {
                EntityContext::Gen7b => Ok(7),
                EntityContext::Gen8a => Ok(8),
                EntityContext::Gen8b => Ok(8),
                _ => Err(PKError::IndexOutOfRange {
                    index: val as usize,
                }),
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        ![EntityContext::None, EntityContext::SplitInvalid].contains(self)
            && self < &EntityContext::MaxInvalid
    }

    pub fn get_single_game_version(&self) -> PKResult<GameVersion> {
        match self {
            EntityContext::Gen1 => Ok(GameVersion::RD),
            EntityContext::Gen2 => Ok(GameVersion::C),
            EntityContext::Gen3 => Ok(GameVersion::E),
            EntityContext::Gen4 => Ok(GameVersion::SS),
            EntityContext::Gen5 => Ok(GameVersion::W2),
            EntityContext::Gen6 => Ok(GameVersion::AS),
            EntityContext::Gen7 => Ok(GameVersion::UM),
            EntityContext::Gen8 => Ok(GameVersion::SH),
            EntityContext::Gen9 => Ok(GameVersion::VL),
            EntityContext::Gen7b => Ok(GameVersion::GP),
            EntityContext::Gen8a => Ok(GameVersion::PLA),
            EntityContext::Gen8b => Ok(GameVersion::BD),
            _ => Err(PKError::IndexOutOfRange {
                index: *self as usize,
            }),
        }
    }
}

impl GameVersion {
    pub fn get_context(&self) -> EntityContext {
        match self {
            GameVersion::GP | GameVersion::GE => EntityContext::Gen7b,
            GameVersion::PLA => EntityContext::Gen8a,
            GameVersion::BD | GameVersion::SP => EntityContext::Gen8b,
            _ => EntityContext::from(*self as u8),
        }
    }
}
