#![allow(dead_code)]

use crate::array_util;
use crate::game::enums::{Ball, GameVersion};
use crate::legality::tables7;

pub(crate) const MAX_SPECIES_ID_7B: u16 = 809;
pub(crate) const MAX_MOVE_ID_7B: u16 = 742;
pub(crate) const MAX_ITEM_ID_7B: u16 = 1057;
pub(crate) const MAX_BALL_ID_7B: u16 = Ball::Beast as u16;
pub(crate) const MAX_GAME_ID_7B: u16 = GameVersion::GE as u16;
pub(crate) const MAX_ABILITY_ID_7B: u16 = tables7::MAX_ABILITY_ID_7_USUM;
pub(crate) const HELD_ITEMS_GG: [u16; 0] = [];
pub const AWAKENING_MAX: u8 = 200;

pub(crate) const POUCH_CANDY_GG_REGULAR: [u16; 19] = [
    50, // Rare Candy
    960, 961, 962, 963, 964, 965, // S
    966, 967, 968, 969, 970, 971, // L
    972, 973, 974, 975, 976, 977, // XL
];

pub(crate) const POUCH_CANDY_GG_SPECIES: [u16; 80] = [
    978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996,
    997, 998, 999, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012,
    1013, 1014, 1015, 1016, 1017, 1018, 1019, 1020, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028,
    1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044,
    1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057,
];

pub(crate) const POUCH_CANDY_GG: [u16; POUCH_CANDY_GG_REGULAR.len()
    + POUCH_CANDY_GG_SPECIES.len()] =
    array_util::concat_all(&[&POUCH_CANDY_GG_REGULAR, &POUCH_CANDY_GG_SPECIES], 0);

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_MEDICINE_GG: [u16; 22] = [
    017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 038, 039, 040,
    041, 709, 903,
];

pub(crate) const POUCH_TM_GG: [u16; 60] = [
    328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365,
    366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 387,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_POWER_UP_GG: [u16; 8] = [051, 053, 081, 082, 083, 084, 085, 849];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_CATCHING_GG: [u16; 14] = [
    001, 002, 003, 004, 012, 164, 166, 168, 861, 862, 863, 864, 865, 866,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_BATTLE_GG: [u16; 24] = [
    055, 056, 057, 058, 059, 060, 061, 062, 656, 659, 660, 661, 662, 663, 671, 672, 675, 676, 678,
    679, 760, 762, 770, 773,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_REGULAR_GG: [u16; 53] = [
    076, 077, 078, 079, 086, 087, 088, 089, 090, 091, 092, 093, 101, 102, 103, 113, 115, 121, 122,
    123, 124, 125, 126, 127, 128, 442, 571, 632, 651, 795, 796, 872, 873, 874, 875, 876, 877, 878,
    885, 886, 887, 888, 889, 890, 891, 892, 893, 894, 895, 896, 900, 901, 902,
];

pub(crate) const POUCH_REGULAR_GG_KEY: [u16; 32] = [
    113, // Tea
    115, // Autograph
    121, // Pokémon Box
    122, // Medicine Pocket
    123, // TM Case
    124, // Candy Jar
    125, // Power-Up Pocket
    126, // Clothing Trunk
    127, // Catching Pocket
    128, // Battle Pocket
    442, // Town Map
    632, // Shiny Charm
    651, // Poké Flute
    872, // Secret Key
    873, // S.S. Ticket
    874, // Silph Scope
    875, // Parcel
    876, // Card Key
    877, // Gold Teeth
    878, // Lift Key
    885, // Stretchy Spring
    886, // Chalky Stone
    887, // Marble
    888, // Lone Earring
    889, // Beach Glass
    890, // Gold Leaf
    891, // Silver Leaf
    892, // Polished Mud Ball
    893, // Tropical Shell
    894, // Leaf Letter (P)
    895, // Leaf Letter (E)
    896, // Small Bouquet
];

pub fn is_allowed_move_gg(move_id: u16) -> bool {
    ALLOWED_MOVES_GG.binary_search(&move_id).is_ok()
}

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const ALLOWED_MOVES_GG: [u16; 227] = [
    000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018,
    019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037,
    038, 039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056,
    057, 058, 059, 060, 061, 062, 063, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, 075,
    076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086, 087, 088, 089, 090, 091, 092, 093, 094,
    095, 096, 097, 098, 099, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
    114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
    133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 182, 188, 200, 224, 227, 231,
    242, 243, 247, 252, 257, 261, 263, 269, 270, 276, 280, 281, 339, 347, 355, 364, 369, 389, 394,
    398, 399, 403, 404, 405, 406, 417, 420, 430, 438, 446, 453, 483, 492, 499, 503, 504, 525, 529,
    583, 585, 603, 605, 606, 607, 729, 730, 731, 733, 734, 735, 736, 737, 738, 739, 740, 742,
];
