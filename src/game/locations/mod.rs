pub mod locations_4;
pub mod locations_5;
pub mod locations_6;
pub mod locations_7;
pub mod locations_7b;
pub mod locations_8;
pub mod locations_8a;
pub mod locations_8b;
pub mod locations_9;

use crate::game::enums::GameVersion;

pub const LINK_TRADE_4: u16 = 2002;
pub const LINK_TRADE_5: u16 = 30003;
pub const LINK_TRADE_6: u16 = 30002;

pub const DAYCARE_4: u16 = 2000;
pub const DAYCARE_5: u16 = 60002;
pub const DAYCARE_8B: u16 = 60010;
pub const PICNIC_9: u16 = 30023;
pub const TERA_CAVERN_9: u16 = 30024;

pub const LINK_TRADE_2_NPC: u16 = 126;
pub const LINK_TRADE_3_NPC: u16 = 254;
pub const LINK_TRADE_4_NPC: u16 = 2001;
pub const LINK_TRADE_5_NPC: u16 = 30002;
pub const LINK_TRADE_6_NPC: u16 = 30001;

pub const BREEDER_5: u16 = 60003;
pub const BREEDER_6: u16 = 60004;

pub const POKE_WALKER_4: u16 = 233;
pub const RANGER_4: u16 = 3001;
pub const FARAWAY_4: u16 = 3002;

pub const HATCH_LOCATION_C: u8 = 16;
pub const HATCH_LOCATION_RSE: u8 = 32;
pub const HATCH_LOCATION_FRLG: u8 = 146;
pub const HATCH_LOCATION_DPPT: u16 = 4;
pub const HATCH_LOCATION_HGSS: u16 = 182;
pub const HATCH_LOCATION_5: u16 = 64;
pub const HATCH_LOCATION_6XY: u16 = 38;
pub const HATCH_LOCATION_6AO: u16 = 318;
pub const HATCH_LOCATION_7: u16 = 78;
pub const HATCH_LOCATION_8: u16 = 40;
pub const HATCH_LOCATION_8B: u16 = 446;
pub const HATCH_LOCATION_9: u16 = 6;

pub const TRANSFER_1: u16 = 30013;
pub const TRANSFER_2: u16 = 30017;
pub const TRANSFER_3: u16 = 0x37;
pub const TRANSFER_4: u16 = 30001;
pub const TRANSFER_4_CELEBI_UNUSED: u16 = 30010;
pub const TRANSFER_4_CELEBI_USED: u16 = 30011;
pub const TRANSFER_4_CROWN_UNUSED: u16 = 30012;
pub const TRANSFER_4_CROWN_USED: u16 = 30013;

pub const LINK_GIFT_6: u16 = 30011;

pub const PELAGO_7: u16 = 30016;

pub const GO_7: u16 = 50;
pub const GO_8: u16 = 30012;

pub const HOME_8: u16 = 30018;

pub const HOME_SHSP: u16 = 59998;
pub const HOME_SWBD: u16 = 59999;
pub const HOME_SWLA: u16 = 60000;
pub const HOME_SWSH_BDSP_EGG: u16 = 65534;
pub const DEFAULT_8B_NONE: u16 = 65535;

pub fn get_version_swsh(ver: u8) -> u8 {
    let version: GameVersion = ver.into();
    match version {
        GameVersion::PLA => GameVersion::SW as u8,
        GameVersion::BD => GameVersion::SW as u8,
        GameVersion::SP => GameVersion::SH as u8,
        _ => ver,
    }
}

pub fn get_met_swsh(loc: u16, ver: u8) -> u16 {
    let version: GameVersion = ver.into();
    match version {
        GameVersion::PLA => HOME_SWLA,
        GameVersion::BD => HOME_SWBD,
        GameVersion::SP => HOME_SHSP,
        _ => loc,
    }
}

pub fn is_valid_met_bdsp(loc: u16, ver: u8) -> bool {
    match loc {
        HOME_SHSP if ver == GameVersion::SH as u8 => true,
        HOME_SWBD if ver == GameVersion::SW as u8 => true,
        _ => false,
    }
}

pub fn traded_egg_location_npc(generation: u8) -> u16 {
    match generation {
        1 => LINK_TRADE_2_NPC,
        2 => LINK_TRADE_2_NPC,
        3 => LINK_TRADE_3_NPC,
        4 => LINK_TRADE_4_NPC,
        5 => LINK_TRADE_5_NPC,
        _ => LINK_TRADE_6_NPC,
    }
}

pub fn traded_egg_location(generation: u8, ver: GameVersion) -> u16 {
    match generation {
        4 => LINK_TRADE_4,
        5 => LINK_TRADE_5,
        8 if GameVersion::BDSP.contains(&ver) => LINK_TRADE_6_NPC,
        _ => LINK_TRADE_6,
    }
}

pub const fn is_pt_hgss_location(location: u16) -> bool {
    location > 111 && location < 2000
}

pub const fn is_pt_hgss_location_egg(location: u16) -> bool {
    location > 2010 && location < 3000
}

pub const fn is_event_location_3(location: u16) -> bool {
    location == 255
}

pub const fn is_event_location_4(location: u16) -> bool {
    location >= 3000 && location <= 3076
}

pub const fn is_event_location_5(location: u16) -> bool {
    location > 40000 && location < 50000
}

const SAFARI_LOCATION_RSE: u16 = 57;
const SAFARI_LOCATION_FRLG: u16 = 136;
const SAGARI_LOCATION_HGSS: u16 = 202;
const MARSH_LOCATION_DPPT: u16 = 52;

pub const fn is_safari_zone_location_3(loc: u16) -> bool {
    loc == SAFARI_LOCATION_RSE || loc == SAFARI_LOCATION_FRLG
}

pub const fn is_safari_zone_location_4(loc: u16) -> bool {
    loc == MARSH_LOCATION_DPPT || loc == SAGARI_LOCATION_HGSS
}

pub const fn is_safari_zone_location_8b(loc: u16) -> bool {
    loc >= 219 && loc <= 224
}

pub fn is_egg_location_bred_4(loc: u16, ver: GameVersion) -> bool {
    if loc == DAYCARE_4 || loc == LINK_TRADE_4 {
        true
    } else {
        loc == FARAWAY_4
            && (ver == GameVersion::Pt || ver == GameVersion::HG || ver == GameVersion::SS)
    }
}

pub fn is_egg_location_bred_5(loc: u16) -> bool {
    loc == DAYCARE_5 || loc == LINK_TRADE_5
}

pub fn is_egg_location_bred_6(loc: u16) -> bool {
    loc == DAYCARE_5 || loc == LINK_TRADE_6
}

pub fn is_egg_location_bred_8b(loc: u16) -> bool {
    loc == DAYCARE_8B || loc == LINK_TRADE_6_NPC
}

pub fn is_egg_location_bred_9(loc: u16) -> bool {
    loc == PICNIC_9 || loc == LINK_TRADE_6
}

pub fn get_daycare_location(generation: u8, version: GameVersion) -> u16 {
    match generation {
        1 | 2 | 3 => 0,
        4 => DAYCARE_4,
        5 => DAYCARE_5,
        8 if [GameVersion::BD, GameVersion::SP].contains(&version) => DAYCARE_8B,
        9 => PICNIC_9,
        _ => DAYCARE_5,
    }
}

pub const fn is_met_location_3_rs(loc: u16) -> bool {
    loc <= 87
}

pub const fn is_met_location_3_e(loc: u16) -> bool {
    loc <= 87 || (loc >= 197 && loc <= 212)
}

pub const fn is_met_location_3_frlg(loc: u16) -> bool {
    loc > 87 && loc < 197
}

pub const fn is_met_location_4_dp(loc: u16) -> bool {
    loc <= 111
}

pub const fn is_met_location_4_pt(loc: u16) -> bool {
    loc <= 125
}

pub const fn is_met_location_4_hgss(loc: u16) -> bool {
    loc > 125 && loc < 234
}

pub const fn is_met_location_5_bw(loc: u16) -> bool {
    loc <= 116
}

pub const fn is_met_location_6_xy(loc: u16) -> bool {
    loc <= 168
}

pub const fn is_met_location_6_ao(loc: u16) -> bool {
    loc > 168 && loc <= 354
}

pub const fn is_met_location_7_sm(loc: u16) -> bool {
    loc < 200
}

pub const fn is_met_location_7_usum(loc: u16) -> bool {
    loc < 234
}

pub const fn is_met_location_7_gg(loc: u16) -> bool {
    loc <= 54
}

pub const fn is_met_location_8_swsh(loc: u16) -> bool {
    loc <= 246
}

pub const fn is_met_location_8_bdsp(loc: u16) -> bool {
    loc <= 657
}

pub const fn is_met_location_8_la(loc: u16) -> bool {
    loc <= 155
}

pub const fn is_met_location_9_sv(loc: u16) -> bool {
    loc <= 131
}
