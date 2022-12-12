#![allow(dead_code)]

use crate::array_util;
use crate::legality::tables::get_permit_list_disallowed;

pub(crate) const MAX_SPECIES_INDEX_3: usize = 412;
pub(crate) const MAX_SPECIES_ID_3: u16 = 386;
pub(crate) const MAX_MOVE_ID_3: u16 = 354;
pub(crate) const MAX_ITEM_ID_3: u16 = 374;
pub(crate) const MAX_ITEM_ID_3_COLO: u16 = 547;
pub(crate) const MAX_ITEM_ID_3_XD: u16 = 593;
pub(crate) const MAX_ABILITY_ID_3: u16 = 77;
pub(crate) const MAX_BALL_ID_3: u16 = 0xC;
pub(crate) const MAX_GAME_ID_3: u16 = 15;

pub(crate) const POUCH_ITEMS_RS: [u16; 139] = [
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
    37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    73, 74, 75, 76, 77, 78, 79, 80, 81, 83, 84, 85, 86, 93, 94, 95, 96, 97, 98, 103, 104, 106, 107,
    108, 109, 110, 111, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 179, 180, 181,
    182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
    220, 221, 222, 223, 224, 225, 254, 255, 256, 257, 258,
];

pub(crate) const POUCH_KEY_RS: [u16; 29] = [
    259, 260, 261, 262, 263, 264, 265, 266, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278,
    279, 280, 281, 282, 283, 284, 285, 286, 287, 288,
];

pub(crate) const POUCH_TM_RS: [u16; 50] = [
    289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307,
    308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326,
    327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338,
];

pub(crate) const POUCH_HM_RS: [u16; 8] = [339, 340, 341, 342, 343, 344, 345, 346];

pub(crate) const POUCH_BERRIES_RS: [u16; 43] = [
    133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
    171, 172, 173, 174, 175,
];

pub(crate) const POUCH_BALL_RS: [u16; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

pub(crate) const POUCH_KEY_FRLG: [u16; POUCH_KEY_RS.len() + 26] = array_util::concat_two(
    &POUCH_KEY_RS,
    &[
        349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366,
        367, 368, 369, 370, 371, 372, 373, 374,
    ],
    0,
);

pub(crate) const POUCH_KEY_E: [u16; POUCH_KEY_FRLG.len() + 2] =
    array_util::concat_two(&POUCH_KEY_FRLG, &[375, 376], 0);

pub(crate) const POUCH_TMHM_RS: [u16; POUCH_TM_RS.len() + POUCH_HM_RS.len()] =
    array_util::concat_two(&POUCH_TM_RS, &POUCH_HM_RS, 0);

pub(crate) const HELD_ITEMS_RS: [u16; POUCH_ITEMS_RS.len()
    + POUCH_BALL_RS.len()
    + POUCH_BERRIES_RS.len()
    + POUCH_TM_RS.len()] = array_util::concat_four(
    &POUCH_ITEMS_RS,
    &POUCH_BALL_RS,
    &POUCH_BERRIES_RS,
    &POUCH_TM_RS,
    0,
);

pub(crate) const POUCH_COLOGNE_COLO: [u16; 3] = [543, 544, 545];
pub(crate) const POUCH_ITEMS_COLO: [u16; POUCH_ITEMS_RS.len() + 1] =
    array_util::concat_two(&POUCH_ITEMS_RS, &[537], 0);
pub(crate) const HELD_ITEMS_COLO: [u16; POUCH_ITEMS_COLO.len()
    + POUCH_BALL_RS.len()
    + POUCH_BERRIES_RS.len()
    + POUCH_TM_RS.len()] = array_util::concat_four(
    &POUCH_ITEMS_COLO,
    &POUCH_BALL_RS,
    &POUCH_BERRIES_RS,
    &POUCH_TM_RS,
    0,
);

pub(crate) const POUCH_KEY_COLO: [u16; 44] = [
    500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518,
    519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 538,
    539, 540, 541, 542, 546, 547,
];

pub(crate) const POUCH_COLOGNE_XD: [u16; 3] = [513, 514, 515];
pub(crate) const POUCH_ITEMS_XD: [u16; POUCH_ITEMS_RS.len() + 1] =
    array_util::concat_two(&POUCH_ITEMS_RS, &[511], 0);
pub(crate) const HELD_ITEMS_XD: [u16; POUCH_ITEMS_XD.len()
    + POUCH_BALL_RS.len()
    + POUCH_BERRIES_RS.len()
    + POUCH_TM_RS.len()] = array_util::concat_four(
    &POUCH_ITEMS_XD,
    &POUCH_BALL_RS,
    &POUCH_BERRIES_RS,
    &POUCH_TM_RS,
    0,
);

pub(crate) const POUCH_KEY_XD: [u16; 27] = [
    500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 512, 516, 517, 518, 519, 523, 524, 525,
    526, 527, 528, 529, 530, 531, 532, 533,
];

pub(crate) const POUCH_DISC_XD: [u16; 60] = [
    534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552,
    553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571,
    572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590,
    591, 592, 593,
];

pub(crate) const RELEASED_HELD_ITEMS_3: [bool; (MAX_ITEM_ID_3 + 1) as usize] =
    get_permit_list_disallowed(&HELD_ITEMS_RS, &[5]);

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const VALID_MET_RS: [u8; 83] = [
    000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018,
    019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037,
    038, 039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056,
    057, 058, 059, 060, 061, 062, 063, 065, 066, 067, 068, 069, 070, 072, 073, 074, 076, 078, 079,
    080, 081, 082, 083, 085, 086, 087,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const VALID_MET_FRLG: [u8; 103] = [
    087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097, 098, 099, 100, 101, 102, 103, 104, 105,
    106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
    125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 159, 160, 161, 162, 163, 164, 165, 166,
    167, 168, 169, 170, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188,
    189, 190, 191, 192, 193, 194, 195, 196,
];

pub(crate) const VALID_MET_E: [u8; VALID_MET_RS.len() + 17] = array_util::concat_two(
    &VALID_MET_RS,
    &[
        196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212,
    ],
    0,
);
