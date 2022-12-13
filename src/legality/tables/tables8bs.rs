#![allow(dead_code)]

use crate::array_util;
use crate::game::enums::{Ball, GameVersion};
use crate::legality::tables::get_permit_list_disallowed;
use crate::legality::tables4::MAX_SPECIES_ID_4;
use crate::legality::tables8::{MAX_ABILITY_ID_8_R2, MAX_MOVE_ID_8_R2};

pub(crate) const MAX_SPECIES_ID_8B: u16 = MAX_SPECIES_ID_4;
pub(crate) const MAX_MOVE_ID_8B: u16 = MAX_MOVE_ID_8_R2;
pub(crate) const MAX_ITEM_ID_8B: u16 = 1822;
pub(crate) const MAX_BALL_ID_8B: u16 = Ball::LAOrigin as u16;
pub(crate) const MAX_GAME_ID_8B: u16 = GameVersion::SP as u16;
pub(crate) const MAX_ABILITY_ID_8B: u16 = MAX_ABILITY_ID_8_R2;

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_REGULAR_BS: [u16; 179] = [
    045, 046, 047, 048, 049, 050, 051, 052, 053, 072, 073, 074, 075, 076, 077, 078, 079, 080, 081,
    082, 083, 084, 085, 093, 094, 107, 108, 109, 110, 111, 112, 135, 136, 213, 214, 215, 217, 218,
    219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237,
    238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256,
    257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275,
    276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294,
    295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313,
    314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 537, 565, 566, 567, 568,
    569, 570, 644, 645, 849, 1231, 1232, 1233, 1234, 1235, 1236, 1237, 1238, 1239, 1240, 1241,
    1242, 1243, 1244, 1245, 1246, 1247, 1248, 1249, 1250, 1251, 1606,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_BALL_BS: [u16; 27] = [
    001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 492, 493, 494,
    495, 496, 497, 498, 499, 500, 576, 851,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_BATTLE_BS: [u16; 9] = [055, 056, 057, 058, 059, 060, 061, 062, 063];

pub(crate) const POUCH_ITEMS_BS: [u16; POUCH_REGULAR_BS.len()
    + POUCH_BALL_BS.len()
    + POUCH_BATTLE_BS.len()] =
    array_util::concat_all(&[&POUCH_REGULAR_BS, &POUCH_BALL_BS, &POUCH_BATTLE_BS], 0);

pub(crate) const POUCH_KEY_BS: [u16; 32] = [
    428, 431, 432, 433, 438, 439, 440, 443, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455,
    459, 460, 461, 462, 463, 464, 466, 467, 631, 632, 1267, 1278, 1822,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_MEDICINE_BS: [u16; 29] = [
    017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035,
    036, 037, 038, 039, 040, 041, 042, 043, 044, 054,
];

pub(crate) const POUCH_BERRIES_BS: [u16; 65] = [
    149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
    168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186,
    187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
    206, 207, 208, 209, 210, 211, 212, 686,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_TREASURE_BS: [u16; 31] = [
    086, 087, 088, 089, 090, 091, 092, 099, 100, 101, 102, 103, 104, 105, 106, 795, 796, 1808,
    1809, 1810, 1811, 1812, 1813, 1814, 1815, 1816, 1817, 1818, 1819, 1820, 1821,
];

pub(crate) const POUCH_TMHM_BS: [u16; 100] = [
    328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365,
    366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403,
    404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422,
    423, 424, 425, 426, 427,
];

pub(crate) const HELD_ITEMS_BS: [u16; POUCH_ITEMS_BS.len()
    + POUCH_BERRIES_BS.len()
    + POUCH_TMHM_BS.len()
    + POUCH_MEDICINE_BS.len()
    + POUCH_TREASURE_BS.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_BS,
        &POUCH_BERRIES_BS,
        &POUCH_TMHM_BS,
        &POUCH_MEDICINE_BS,
        &POUCH_TREASURE_BS,
    ],
    0,
);

pub(crate) const RELEASED_HELD_ITEMS_8B: [bool; (MAX_ITEM_ID_8B + 1) as usize] =
    get_permit_list_disallowed(
        &HELD_ITEMS_BS,
        &[
            44,  // Sacred Ash
            537, // Prism Scale
            565, // Health Feather
            566, // Muscle Feather
            567, // Resist Feather
            568, // Genius Feather
            569, // Clever Feather
            570, // Swift Feather
            849, // Ice Stone
            5,   // Safari Ball
            16,  // Cherish Ball
            499, // Sport Ball
            500, // Park Ball
            576, // Dream Ball
            851, // Beast Ball
            // new BDSP items, but they can't be held
            1808, // Mysterious Shard S
            1809, // Mysterious Shard L
            1810, // Digger Drill
            1811, // Kanto Slate
            1812, // Johto Slate
            1813, // Soul Slate
            1814, // Rainbow Slate
            1815, // Squall Slate
            1816, // Oceanic Slate
            1817, // Tectonic Slate
            1818, // Stratospheric Slate
            1819, // Genome Slate
            1820, // Discovery Slate
            1821, // Distortion Slate
        ],
    );
