pub const MAX_SPECIES_ID_8: usize = MAX_SPECIES_ID_8_R2;
pub const MAX_MOVE_ID_8: usize = MAX_MOVE_ID_8_R2;
pub const MAX_ITEM_ID_8: usize = MAX_ITEM_ID_8_R2;
pub const MAX_ABILITY_ID_8: usize = MAX_ABILITY_ID_8_R2;

// Orion (No DLC)
pub const MAX_SPECIES_ID_8_O0: usize = 890; // Eternatus
pub const MAX_MOVE_ID_8_O0: usize = 796; // Steel Beam
pub const MAX_ITEM_ID_8_O0: usize = 1278; // Rotom Catalog, ignore all catalog parts
pub const MAX_ABILITY_ID_8_O0: usize = 258; // Hunger Switch

// Rigel 1 (DLC 1: Isle of Armor)
pub const MAX_SPECIES_ID_8_R1: usize = 893; // Zarude
pub const MAX_MOVE_ID_8_R1: usize = 818; // Surging Strikes
pub const MAX_ITEM_ID_8_R1: usize = 1589; // Mark Charm
pub const MAX_ABILITY_ID_8_R1: usize = 260; // Unseen Fist

// Rigel 2 (DLC 2: Crown Tundra)
pub const MAX_SPECIES_ID_8_R2: usize = 898; // Calyrex
pub const MAX_MOVE_ID_8_R2: usize = 826; // Eerie Spell
pub const MAX_ITEM_ID_8_R2: usize = 1607; // Reins of Unity
pub const MAX_ABILITY_ID_8_R2: usize = 267; // As One (Glastrier)

pub const MAX_BALL_ID_8: usize = 0x1A; // 26 Beast
pub const MAX_GAME_ID_8: usize = 45; // Shield

pub const TM_SWSH: [u16; 100] = [
    328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365,
    366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403,
    404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 618, 619, 620,
    690, 691, 692, 693,  // TM99
    1230, // TM00
];

pub const TR_SWSH: [u16; 100] = [
    1130, 1131, 1132, 1133, 1134, 1135, 1136, 1137, 1138, 1139, 1140, 1141, 1142, 1143, 1144, 1145,
    1146, 1147, 1148, 1149, 1150, 1151, 1152, 1153, 1154, 1155, 1156, 1157, 1158, 1159, 1160, 1161,
    1162, 1163, 1164, 1165, 1166, 1167, 1168, 1169, 1170, 1171, 1172, 1173, 1174, 1175, 1176, 1177,
    1178, 1179, 1180, 1181, 1182, 1183, 1184, 1185, 1186, 1187, 1188, 1189, 1190, 1191, 1192, 1193,
    1194, 1195, 1196, 1197, 1198, 1199, 1200, 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209,
    1210, 1211, 1212, 1213, 1214, 1215, 1216, 1217, 1218, 1219, 1220, 1221, 1222, 1223, 1224, 1225,
    1226, 1227, 1228, 1229,
];

pub const TMHM_SWSH: [usize; 200] = [
    // TM
    005, 025, 006, 007, 008, 009, 019, 042, 063, 416, 345, 076, 669, 083, 086, 091, 103, 113, 115,
    219, 120, 156, 157, 168, 173, 182, 184, 196, 202, 204, 211, 213, 201, 240, 241, 258, 250, 251,
    261, 263, 129, 270, 279, 280, 286, 291, 311, 313, 317, 328, 331, 333, 340, 341, 350, 362, 369,
    371, 372, 374, 384, 385, 683, 409, 419, 421, 422, 423, 424, 427, 433, 472, 478, 440, 474, 490,
    496, 506, 512, 514, 521, 523, 527, 534, 541, 555, 566, 577, 580, 581, 604, 678, 595, 598, 206,
    403, 684, 693, 707, 784, // TR
    014, 034, 053, 056, 057, 058, 059, 067, 085, 087, 089, 094, 097, 116, 118, 126, 127, 133, 141,
    161, 164, 179, 188, 191, 200, 473, 203, 214, 224, 226, 227, 231, 242, 247, 248, 253, 257, 269,
    271, 276, 285, 299, 304, 315, 322, 330, 334, 337, 339, 347, 348, 349, 360, 370, 390, 394, 396,
    398, 399, 402, 404, 405, 406, 408, 411, 412, 413, 414, 417, 428, 430, 437, 438, 441, 442, 444,
    446, 447, 482, 484, 486, 492, 500, 502, 503, 526, 528, 529, 535, 542, 583, 599, 605, 663, 667,
    675, 676, 706, 710, 776,
];

pub const MOVE_PP_SWSH: [u8; 827] = [
    00, 35, 25, 10, 15, 20, 20, 15, 15, 15, 35, 30, 05, 10, 20, 30, 35, 35, 20, 15, 20, 20, 25, 20,
    30, 05, 10, 15, 15, 15, 25, 20, 05, 35, 15, 20, 20, 10, 15, 30, 35, 20, 20, 30, 25, 40, 20, 15,
    20, 20, 20, 30, 25, 15, 30, 25, 05, 15, 10, 05, 20, 20, 20, 05, 35, 20, 20, 20, 20, 20, 15, 25,
    15, 10, 20, 25, 10, 35, 30, 15, 10, 40, 10, 15, 30, 15, 20, 10, 15, 10, 05, 10, 10, 25, 10, 20,
    40, 30, 30, 20, 20, 15, 10, 40, 15, 10, 30, 10, 20, 10, 40, 40, 20, 30, 30, 20, 30, 10, 10, 20,
    05, 10, 30, 20, 20, 20, 05, 15, 15, 20, 10, 15, 35, 20, 15, 10, 10, 30, 15, 40, 20, 10, 10, 05,
    10, 30, 10, 15, 20, 15, 40, 20, 10, 05, 15, 10, 10, 10, 15, 30, 30, 10, 10, 20, 10, 01, 01, 10,
    25, 10, 05, 15, 25, 15, 10, 15, 30, 05, 40, 15, 10, 25, 10, 30, 10, 20, 10, 10, 10, 10, 10, 20,
    05, 40, 05, 05, 15, 05, 10, 05, 10, 10, 10, 10, 20, 20, 40, 15, 10, 20, 20, 25, 05, 15, 10, 05,
    20, 15, 20, 25, 20, 05, 30, 05, 10, 20, 40, 05, 20, 40, 20, 15, 35, 10, 05, 05, 05, 15, 05, 20,
    05, 05, 15, 20, 10, 05, 05, 15, 10, 15, 15, 10, 10, 10, 20, 10, 10, 10, 10, 15, 15, 15, 10, 20,
    20, 10, 20, 20, 20, 20, 20, 10, 10, 10, 20, 20, 05, 15, 10, 10, 15, 10, 20, 05, 05, 10, 10, 20,
    05, 10, 20, 10, 20, 20, 20, 05, 05, 15, 20, 10, 15, 20, 15, 10, 10, 15, 10, 05, 05, 10, 15, 10,
    05, 20, 25, 05, 40, 15, 05, 40, 15, 20, 20, 05, 15, 20, 20, 15, 15, 05, 10, 30, 20, 30, 15, 05,
    40, 15, 05, 20, 05, 15, 25, 25, 15, 20, 15, 20, 15, 20, 10, 20, 20, 05, 05, 10, 05, 40, 10, 10,
    05, 10, 10, 15, 10, 20, 15, 30, 10, 20, 05, 10, 10, 15, 10, 10, 05, 15, 05, 10, 10, 30, 20, 20,
    10, 10, 05, 05, 10, 05, 20, 10, 20, 10, 15, 10, 20, 20, 20, 15, 15, 10, 15, 15, 15, 10, 10, 10,
    20, 10, 30, 05, 10, 15, 10, 10, 05, 20, 30, 10, 30, 15, 15, 15, 15, 30, 10, 20, 15, 10, 10, 20,
    15, 05, 05, 15, 15, 05, 10, 05, 20, 05, 15, 20, 05, 20, 20, 20, 20, 10, 20, 10, 15, 20, 15, 10,
    10, 05, 10, 05, 05, 10, 05, 05, 10, 05, 05, 05, 15, 10, 10, 10, 10, 10, 10, 15, 20, 15, 10, 15,
    10, 15, 10, 20, 10, 10, 10, 20, 20, 20, 20, 20, 15, 15, 15, 15, 15, 15, 20, 15, 10, 15, 15, 15,
    15, 10, 10, 10, 10, 10, 15, 15, 15, 15, 05, 05, 15, 05, 10, 10, 10, 20, 20, 20, 10, 10, 30, 15,
    15, 10, 15, 25, 10, 15, 10, 10, 10, 20, 10, 10, 10, 10, 10, 15, 15, 05, 05, 10, 10, 10, 05, 05,
    10, 05, 05, 15, 10, 05, 05, 05, 10, 10, 10, 10, 20, 25, 10, 20, 30, 25, 20, 20, 15, 20, 15, 20,
    20, 10, 10, 10, 10, 10, 20, 10, 30, 15, 10, 10, 10, 20, 20, 05, 05, 05, 20, 10, 10, 20, 15, 20,
    20, 10, 20, 30, 10, 10, 40, 40, 30, 20, 40, 20, 20, 10, 10, 10, 10, 05, 10, 10, 05, 05, 01, 01,
    01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01,
    01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 40, 15,
    20, 30, 20, 15, 15, 20, 10, 15, 15, 10, 05, 10, 10, 20, 15, 10, 15, 15, 15, 05, 15, 20, 20, 01,
    01, 01, 01, 01, 01, 01, 01, 01, 05, 05, 10, 10, 10, 20, 10, 10, 10, 05, 05, 20, 10, 10, 10, 01,
    05, 15, 05, 01, 01, 01, 01, 01, 01, 10, 15, 15, 20, 20, 20, 20, 15, 15, 10, 10, 05, 20, 05, 10,
    05, 15, 10, 10, 05, 15, 20, 10, 10, 15, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 05, 10, 15, 10, 15, 05, 05, 05, 10, 15, 40, 10, 10, 10, 15, 10, 10,
    10, 10, 05, 05, 05, 10, 05, 20, 10, 10, 05, 20, 20, 10, 10, 05, 05, 05, 40, 10, 20, 10, 10, 10,
    10, 05, 05, 15, 05, 10, 10, 10, 05, 05, 05,
];