#![allow(dead_code)]

use crate::array_util;
use crate::game::enums::Species;
use crate::legality::tables::get_permit_list_disallowed;

pub(crate) const MAX_SPECIES_ID_7: u16 = 802;
pub(crate) const MAX_MOVE_ID_7: u16 = 719;
pub(crate) const MAX_ITEM_ID_7: u16 = 920;
pub(crate) const MAX_ABILITY_ID_7: u16 = 232;
pub(crate) const MAX_BALL_ID_7: u16 = 0x1A;
pub(crate) const MAX_GAME_ID_7: u16 = 41;

pub(crate) const MAX_SPECIES_ID_7_USUM: u16 = 807;
pub(crate) const MAX_MOVE_ID_7_USUM: u16 = 728;
pub(crate) const MAX_ITEM_ID_7_USUM: u16 = 959;
pub(crate) const MAX_ABILITY_ID_7_USUM: u16 = 233;

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_REGULAR_SM: [u16; 298] = [
    068, 069, 070, 071, 072, 073, 074, 075, 076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086,
    087, 088, 089, 090, 091, 092, 093, 094, 099, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
    110, 111, 112, 116, 117, 118, 119, 135, 136, 137, 213, 214, 215, 217, 218, 219, 220, 221, 222,
    223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241,
    242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260,
    261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279,
    280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298,
    299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317,
    318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 499, 534, 535, 537, 538, 539, 540, 541, 542,
    543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561,
    562, 563, 564, 571, 572, 573, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 639, 640,
    644, 646, 647, 648, 649, 650, 656, 657, 658, 659, 660, 661, 662, 663, 664, 665, 666, 667, 668,
    669, 670, 671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 681, 682, 683, 684, 685, 699, 704,
    710, 711, 715, 752, 753, 754, 755, 756, 757, 758, 759, 760, 761, 762, 763, 764, 767, 768, 769,
    770, 795, 796, 844, 849, 853, 854, 855, 856, 879, 880, 881, 882, 883, 884, 904, 905, 906, 907,
    908, 909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920,
];

pub(crate) const POUCH_BALL_SM: [u16; 25] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 492, 493, 494, 495, 496, 497, 498, 576,
    851,
];

pub(crate) const POUCH_BATTLE_SM: [u16; 12] = [55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 577, 846];

pub(crate) const POUCH_ITEMS_SM: [u16; POUCH_REGULAR_SM.len()
    + POUCH_BALL_SM.len()
    + POUCH_BATTLE_SM.len()] =
    array_util::concat_all(&[&POUCH_REGULAR_SM, &POUCH_BALL_SM, &POUCH_BATTLE_SM], 0);

pub(crate) const POUCH_KEY_SM: [u16; 22] = [
    216, 465, 466, 628, 629, 631, 632, 638, 705, 706, 765, 773, 797, 841, 842, 843, 845, 847, 850,
    857, 858, 860,
];

pub(crate) const POUCH_KEY_USUM: [u16; POUCH_KEY_SM.len() + 16] = array_util::concat_all(
    &[
        &POUCH_KEY_SM,
        &[
            933, 934, 935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945, 946, 947, 948, 440,
        ],
    ],
    0,
);

pub(crate) const POUCH_ROTO_USUM: [u16; 11] =
    [949, 950, 951, 952, 953, 954, 955, 956, 957, 958, 959];

pub(crate) const POUCH_TMHM_SM: [u16; 100] = [
    328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365,
    366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403,
    404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 618, 619, 620,
    690, 691, 692, 693, 694,
];

pub(crate) const POUCH_MEDICINE_SM: [u16; 54] = [
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 65, 66, 67, 134, 504, 565, 566, 567,
    568, 569, 570, 591, 645, 708, 709, 852,
];

pub(crate) const POUCH_BERRIES_SM: [u16; 67] = [
    149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
    168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186,
    187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
    206, 207, 208, 209, 210, 211, 212, 686, 687, 688,
];

pub(crate) const POUCH_Z_CRYSTAL_SM: [u16; 29] = [
    807, 808, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820, 821, 822, 823, 824, 825,
    826, 827, 828, 829, 830, 831, 832, 833, 834, 835,
];

pub(crate) const POUCH_Z_CRYSTAL_HELD_SM: [u16; 29] = [
    776, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 791, 792, 793, 794,
    798, 799, 800, 801, 802, 803, 804, 805, 806, 836,
];

pub(crate) const POUCH_Z_CRYSTAL_USUM: [u16; POUCH_Z_CRYSTAL_SM.len() + 6] =
    array_util::concat_all(&[&POUCH_Z_CRYSTAL_SM, &[927, 928, 929, 930, 931, 932]], 0);

pub(crate) const POUCH_Z_CRYSTAL_HELD_USUM: [u16; POUCH_Z_CRYSTAL_HELD_SM.len() + 6] =
    array_util::concat_all(
        &[&POUCH_Z_CRYSTAL_HELD_SM, &[921, 922, 923, 924, 925, 926]],
        0,
    );

pub(crate) const Z_CRYSTAL_PAIRS: [(u16, u16); POUCH_Z_CRYSTAL_USUM.len()] =
    get_pair_arr(&POUCH_Z_CRYSTAL_USUM, &POUCH_Z_CRYSTAL_HELD_USUM);

const fn get_pair_arr<const SIZE: usize>(key: &[u16], held: &[u16]) -> [(u16, u16); SIZE] {
    let mut result = [(0, 0); SIZE];
    let mut index = 0;
    while index < SIZE {
        result[index] = (key[index], held[index]);
        index += 1;
    }
    result
}

pub(crate) const HELD_ITEMS_SM: [u16; POUCH_ITEMS_SM.len()
    + POUCH_BERRIES_SM.len()
    + POUCH_MEDICINE_SM.len()
    + POUCH_Z_CRYSTAL_HELD_SM.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_SM,
        &POUCH_BERRIES_SM,
        &POUCH_MEDICINE_SM,
        &POUCH_Z_CRYSTAL_HELD_SM,
    ],
    0,
);

pub(crate) const HELD_ITEMS_USUM: [u16; POUCH_ITEMS_SM.len()
    + POUCH_BERRIES_SM.len()
    + POUCH_MEDICINE_SM.len()
    + POUCH_Z_CRYSTAL_HELD_USUM.len()
    + POUCH_ROTO_USUM.len()] = array_util::concat_all(
    &[
        &POUCH_ITEMS_SM,
        &POUCH_BERRIES_SM,
        &POUCH_MEDICINE_SM,
        &POUCH_Z_CRYSTAL_HELD_USUM,
        &POUCH_ROTO_USUM,
    ],
    0,
);

pub(crate) const ALOLAN_ORIGIN_FORMS: [u16; 15] = [
    Species::Rattata as u16,
    Species::Raticate as u16,
    Species::Sandshrew as u16,
    Species::Sandslash as u16,
    Species::Vulpix as u16,
    Species::Ninetales as u16,
    Species::Diglett as u16,
    Species::Dugtrio as u16,
    Species::Meowth as u16,
    Species::Persian as u16,
    Species::Geodude as u16,
    Species::Graveler as u16,
    Species::Golem as u16,
    Species::Grimer as u16,
    Species::Muk as u16,
];

pub(crate) const ALOLAN_VARIANT_EVOLUTIONS_12: [u16; 3] = [
    Species::Raichu as u16,
    Species::Exeggutor as u16,
    Species::Marowak as u16,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const PAST_GEN_ALOLAN_NATIVES: [u16; 314] = [
    010, 011, 012, 019, 020, 021, 022, 025, 026, 027, 028, 035, 036, 037, 038, 039, 040, 041, 042,
    046, 047, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, 060, 061, 062, 063, 064, 065, 066,
    067, 068, 072, 073, 074, 075, 076, 079, 080, 081, 082, 088, 089, 090, 091, 092, 093, 094, 096,
    097, 102, 103, 104, 105, 113, 115, 118, 119, 120, 121, 123, 125, 126, 127, 128, 129, 130, 131,
    132, 133, 134, 135, 136, 137, 142, 143, 147, 148, 149, 165, 166, 167, 168, 169, 170, 171, 172,
    173, 174, 185, 186, 196, 197, 198, 199, 200, 209, 210, 212, 215, 222, 225, 227, 233, 235, 239,
    240, 241, 242, 278, 279, 283, 284, 296, 297, 299, 302, 318, 319, 320, 321, 324, 327, 328, 329,
    330, 339, 340, 349, 350, 351, 359, 361, 362, 369, 370, 371, 372, 373, 374, 375, 376, 408, 409,
    410, 411, 422, 423, 425, 426, 429, 430, 438, 440, 443, 444, 445, 446, 447, 448, 456, 457, 461,
    462, 466, 467, 470, 471, 474, 476, 478, 506, 507, 508, 524, 525, 526, 546, 547, 548, 549, 551,
    552, 553, 564, 565, 566, 567, 568, 569, 582, 583, 584, 587, 594, 627, 628, 629, 630, 661, 662,
    663, 674, 675, 700, 703, 704, 705, 706, 707, 708, 709, 718, // Regular
    023, 086, 108, 122, 138, 140, 163, 177, 179, 190, 204, 206, 214, 223, 228, 238, 246, 303, 309,
    341, 343, 345, 347, 352, 353, 357, 366, 427, 439, 458, 550, 559, 570, 572, 592, 605, 619, 621,
    622, 624, 636, 667, 669, 676, 686, 690, 692, 696, 698, 701, 702, 714, // Wormhole
    333, 334, // Altaria
    193, 469, // Yanmega
    561, // Sigilyph
    580, 581, // Swanna
    276, 277, // Swellow
    451, 452, // Drapion
    531, // Audino
    694, 695, // Heliolisk
    273, 274, 275, // Nuzleaf
    325, 326, // Gumpig
    459, 460, // Abomasnow
    307, 308, // Medicham
    449, 450, // Hippowdon
    557, 558, // Crustle
    218, 219, // Magcargo
    688, 689, // Barbaracle
    270, 271, 272, // Lombre
    618, // Stunfisk
    418, 419, // Floatzel
    194, 195, // Quagsire
    100, 101, // Voltorb & Electrode
];

pub(crate) const TOTEM_ALOLAN: [u16; 3] = [
    Species::Raticate as u16,
    Species::Marowak as u16,
    Species::Mimikyu as u16,
];

pub(crate) const TOTEM_NO_TRANSFER: [u16; 4] = [
    Species::Marowak as u16,
    Species::Araquanid as u16,
    Species::Togedemaru as u16,
    Species::Ribombee as u16,
];

pub(crate) const TOTEM_USUM: [u16; 11] = [
    Species::Raticate as u16,
    Species::Gumshoos as u16,
    Species::Salazzle as u16,
    Species::Lurantis as u16,
    Species::Vikavolt as u16,
    Species::Mimikyu as u16,
    Species::Kommoo as u16,
    Species::Marowak as u16,
    Species::Araquanid as u16,
    Species::Togedemaru as u16,
    Species::Ribombee as u16,
];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const VALID_MET_SM: [u16; 90] = [
    006, 008, 010, 012, 014, 016, 018, 020, 022, 024, 026, 028, 030, 032, 034, 036, 038, 040, 042,
    044, 046, 048, 050, 052, 054, 056, 058, 060, 062, 064, 068, 070, 072, 074, 076, 078, 082, 084,
    086, 088, 090, 092, 094, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126,
    128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164,
    166, 168, 170, 172, 174, 176, 178, 180, 182, 184, 186, 188, 190, 192,
];

pub(crate) const VALID_MET_USUM: [u16; VALID_MET_SM.len() + 18] = array_util::concat_all(
    &[
        &VALID_MET_SM,
        &[
            198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218, 220, 222, 224, 226, 228, 230,
            232,
        ],
    ],
    0,
);

pub(crate) const RELEASED_HELD_ITEMS_7: [bool; (MAX_ITEM_ID_7_USUM + 1) as usize] =
    get_permit_list_disallowed(
        &HELD_ITEMS_USUM,
        &[
            5,   // Safari Ball
            16,  // Cherish Ball
            64,  // Fluffy Tail
            65,  // Blue Flute
            66,  // Yellow Flute
            67,  // Red Flute
            68,  // Black Flute
            69,  // White Flute
            70,  // Shoal Salt
            71,  // Shoal Shell
            103, // Old Amber
            111, // Odd Keystone
            164, // Razz Berry
            166, // Nanab Berry
            167, // Wepear Berry
            175, // Cornn Berry
            176, // Magost Berry
            177, // Rabuta Berry
            178, // Nomel Berry
            179, // Spelon Berry
            180, // Pamtre Berry
            181, // Watmel Berry
            182, // Durin Berry
            183, // Belue Berry
            //208, // Enigma Berry
            //209, // Micle Berry
            //210, // Custap Berry
            //211, // Jaboca Berry
            //212, // Rowap Berry
            215, // Macho Brace
            260, // Red Scarf
            261, // Blue Scarf
            262, // Pink Scarf
            263, // Green Scarf
            264, // Yellow Scarf
            499, // Sport Ball
            548, // Fire Gem
            549, // Water Gem
            550, // Electric Gem
            551, // Grass Gem
            552, // Ice Gem
            553, // Fighting Gem
            554, // Poison Gem
            555, // Ground Gem
            556, // Flying Gem
            557, // Psychic Gem
            558, // Bug Gem
            559, // Rock Gem
            560, // Ghost Gem
            561, // Dragon Gem
            562, // Dark Gem
            563, // Steel Gem
            576, // Dream Ball
            584, // Relic Copper
            585, // Relic Silver
            587, // Relic Vase
            588, // Relic Band
            589, // Relic Statue
            590, // Relic Crown
            699, // Discount Coupon
            715, // Fairy Gem
        ],
    );
