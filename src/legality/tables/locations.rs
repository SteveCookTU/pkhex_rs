use crate::GameVersion;

pub const LINK_TRADE_4: usize = 2002;
pub const LINK_TRADE_5: usize = 30003;
pub const LINK_TRADE_6: usize = 30002;

pub const DAYCARE_4: usize = 2000;
pub const DAYCARE_5: usize = 60002;
pub const DAYCARE_8B: usize = 60010;

pub const LINK_TRADE_2_NPC: usize = 126;
pub const LINK_TRADE_3_NPC: usize = 254;
pub const LINK_TRADE_4_NPC: usize = 2001;
pub const LINK_TRADE_5_NPC: usize = 30002;
pub const LINK_TRADE_6_NPC: usize = 30001;

pub const BREEDER_5: usize = 60003;
pub const BREEDER_6: usize = 60004;

pub const POKE_WALKER_4: usize = 233;
pub const RANGER_4: usize = 3001;
pub const FARAWAY_4: usize = 3002;

pub const GO7: usize = 50;
pub const GO8: usize = 30012;

pub const DEFAULT_8B_NONE: isize = -1;

pub fn traded_egg_location(generation: usize, ver: GameVersion) -> usize {
    match generation {
        4 => LINK_TRADE_4,
        5 => LINK_TRADE_5,
        8 if GameVersion::BDSP.contains(&ver) => LINK_TRADE_6_NPC,
        _ => LINK_TRADE_6,
    }
}

pub fn is_none_location(ver: GameVersion, location: usize) -> bool {
    get_none_location(ver) as u16 == location as u16
}

pub fn get_none_location(ver: GameVersion) -> usize {
    match ver {
        GameVersion::BD | GameVersion::SP => DEFAULT_8B_NONE as usize,
        _ => 0,
    }
}
