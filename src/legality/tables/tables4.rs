#![allow(dead_code)]

use crate::array_util;
use crate::legality::tables::get_permit_list_disallowed;

pub(crate) const MAX_SPECIES_ID_4: u16 = 493;
pub(crate) const MAX_MOVE_ID_4: u16 = 467;
pub(crate) const MAX_ITEM_ID_4_DP: u16 = 464;
pub(crate) const MAX_ITEM_ID_PT: u16 = 467;
pub(crate) const MAX_ITEM_ID_4_HGSS: u16 = 536;
pub(crate) const MAX_ABILITY_ID_4: u16 = 123;
pub(crate) const MAX_BALL_ID_4: u16 = 0x18;
pub(crate) const MAX_GAME_ID_4: u16 = 15;

pub(crate) const POUCH_ITEMS_DP: [u16; 161] = [
    68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
    92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    135, 136, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
    230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248,
    249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267,
    268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286,
    287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305,
    306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324,
    325, 326, 327,
];

pub(crate) const POUCH_KEY_DP: [u16; 37] = [
    428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444, 445, 446,
    447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464,
];

pub(crate) const POUCH_TM_DP: [u16; 92] = [
    328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365,
    366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403,
    404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419,
];

pub(crate) const POUCH_HM_DP: [u16; 8] = [420, 421, 422, 423, 424, 425, 426, 427];

pub(crate) const POUCH_TMHM_DP: [u16; 100] =
    array_util::concat_all(&[&POUCH_TM_DP, &POUCH_HM_DP], 0);

pub(crate) const POUCH_MAIL_DP: [u16; 12] =
    [137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148];

pub(crate) const POUCH_MEDICINE_DP: [u16; 38] = [
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
];

pub(crate) const POUCH_BERRIES_DP: [u16; 64] = [
    149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
    168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186,
    187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
    206, 207, 208, 209, 210, 211, 212,
];

pub(crate) const POUCH_BALL_DP: [u16; 15] = [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

pub(crate) const POUCH_BATTLE_DP: [u16; 13] = [55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67];

pub(crate) const HELD_ITEMS_DP: [u16; POUCH_ITEMS_DP.len()
    + POUCH_MAIL_DP.len()
    + POUCH_MEDICINE_DP.len()
    + POUCH_BERRIES_DP.len()
    + POUCH_BALL_DP.len()
    + POUCH_TM_DP.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_DP,
        &POUCH_MAIL_DP,
        &POUCH_MEDICINE_DP,
        &POUCH_BERRIES_DP,
        &POUCH_BALL_DP,
        &POUCH_TM_DP,
    ],
    0,
);

pub(crate) const POUCH_ITEMS_PT: [u16; 162] = [
    68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
    92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 135, 136, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228,
    229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247,
    248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266,
    267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285,
    286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304,
    305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323,
    324, 325, 326, 327,
];

pub(crate) const POUCH_KEY_PT: [u16; 40] = [
    428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444, 445, 446,
    447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465,
    466, 467,
];

pub(crate) const POUCH_TM_PT: [u16; 92] = POUCH_TM_DP;
pub(crate) const POUCH_HM_PT: [u16; 8] = POUCH_HM_DP;
pub(crate) const POUCH_TMHM_PT: [u16; 100] = POUCH_TMHM_DP;
pub(crate) const POUCH_MAIL_PT: [u16; 12] = POUCH_MAIL_DP;
pub(crate) const POUCH_MEDICINE_PT: [u16; 38] = POUCH_MEDICINE_DP;
pub(crate) const POUCH_BERRIES_PT: [u16; 64] = POUCH_BERRIES_DP;
pub(crate) const POUCH_BALL_PT: [u16; 15] = POUCH_BALL_DP;
pub(crate) const POUCH_BATTLE_PT: [u16; 13] = POUCH_BATTLE_DP;

pub(crate) const HELD_ITEMS_PT: [u16; POUCH_ITEMS_PT.len()
    + POUCH_MAIL_PT.len()
    + POUCH_MEDICINE_PT.len()
    + POUCH_BERRIES_PT.len()
    + POUCH_BALL_PT.len()
    + POUCH_TM_PT.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_PT,
        &POUCH_MAIL_PT,
        &POUCH_MEDICINE_PT,
        &POUCH_BERRIES_PT,
        &POUCH_BALL_PT,
        &POUCH_TM_PT,
    ],
    0,
);

pub(crate) const POUCH_ITEMS_HGSS: [u16; 162] = POUCH_ITEMS_PT;

pub(crate) const POUCH_KEY_HGSS: [u16; 38] = [
    434, 435, 437, 444, 445, 446, 447, 450, 456, 464, 465, 466, 468, 469, 470, 471, 472, 473, 474,
    475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 501, 502, 503, 504, 532, 533, 534, 535, 536,
];

pub(crate) const POUCH_TM_HGSS: [u16; 92] = POUCH_TM_DP;
pub(crate) const POUCH_HM_HGSS: [u16; 8] = POUCH_HM_DP;
pub(crate) const POUCH_TMHM_HGSS: [u16; 100] = POUCH_TMHM_DP;
pub(crate) const POUCH_MAIL_HGSS: [u16; 12] = POUCH_MAIL_DP;
pub(crate) const POUCH_MEDICINE_HGSS: [u16; 38] = POUCH_MEDICINE_DP;
pub(crate) const POUCH_BERRIES_HGSS: [u16; 64] = POUCH_BERRIES_DP;
pub(crate) const POUCH_BALL_HGSS: [u16; 24] = [
    1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 492, 493, 494, 495, 496, 497, 498, 499, 500,
];
pub(crate) const POUCH_BATTLE_HGSS: [u16; 13] = POUCH_BATTLE_DP;

pub(crate) const HELD_ITEMS_HGSS: [u16; POUCH_ITEMS_HGSS.len()
    + POUCH_MAIL_HGSS.len()
    + POUCH_MEDICINE_HGSS.len()
    + POUCH_BERRIES_HGSS.len()
    + POUCH_BALL_HGSS.len()
    + POUCH_TM_HGSS.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_HGSS,
        &POUCH_MAIL_HGSS,
        &POUCH_MEDICINE_HGSS,
        &POUCH_BERRIES_HGSS,
        &POUCH_BALL_HGSS,
        &POUCH_TM_HGSS,
    ],
    0,
);

pub(crate) const RELEASED_HELD_ITEMS_4: [bool; (MAX_ITEM_ID_4_HGSS + 1) as usize] =
    get_permit_list_disallowed(&HELD_ITEMS_HGSS, &[5, 16, 147, 499, 500]);

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const VALID_MET_DP: [u16; 107] = [
    001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019,
    020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038,
    039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057,
    058, 059, 060, 061, 062, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, 075, 076, 077,
    078, 080, 081, 082, 083, 084, 087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097, 098, 099,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
];

pub(crate) const VALID_MET_PT: [u16; VALID_MET_DP.len() + 17] = array_util::concat_all(
    &[
        &VALID_MET_DP,
        &[
            63, 79, 85, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
        ],
    ],
    0,
);

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const VALID_MET_HGSS: [u16; 113] = [
    080, 112, 113, 114, 115, 116, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
    139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 172, 173, 174, 175, 176, 177,
    178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196,
    197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215,
    216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 234,
];

pub(crate) const VALID_MET_4: [u16; VALID_MET_PT.len() + VALID_MET_HGSS.len()] =
    array_util::concat_all(&[&VALID_MET_PT, &VALID_MET_HGSS], 0);

pub(crate) const GIFT_EGG_LOCATION_4: [u16; 5] = [2009, 2010, 2011, 2013, 2014];
