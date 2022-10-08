use crate::tables::locations::{LINK_TRADE_4, LINK_TRADE_5, LINK_TRADE_6, LINK_TRADE_6_NPC};
use crate::GameVersion;

pub const GO7: u16 = 50;
pub const GO8: u16 = 30012;

pub fn traded_egg_location(generation: u8, ver: GameVersion) -> u16 {
    match generation {
        4 => LINK_TRADE_4,
        5 => LINK_TRADE_5,
        8 if GameVersion::BDSP.contains(&ver) => LINK_TRADE_6_NPC,
        _ => LINK_TRADE_6,
    }
}
