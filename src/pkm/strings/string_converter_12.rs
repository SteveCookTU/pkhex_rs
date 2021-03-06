use crate::StringConverterOption;
use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub fn get_is_g1_japanese_char(str: &[char]) -> bool {
    all_chars_in_map_char(str, &U2RBY_J)
}

pub fn get_is_g1_english_char(str: &[char]) -> bool {
    all_chars_in_map_char(str, &U2RBY_U)
}

pub fn get_is_g1_japanese_byte(raw: &[u8]) -> bool {
    all_chars_in_map_byte(raw, &RBY2U_J)
}

pub fn get_is_g1_english_byte(raw: &[u8]) -> bool {
    all_chars_in_map_byte(raw, &RBY2U_U)
}

fn all_chars_in_map_char(c: &[char], d: &BTreeMap<char, u8>) -> bool {
    for c in c.iter() {
        if d.contains_key(c) {
            return false;
        }
    }
    true
}

fn all_chars_in_map_byte(data: &[u8], d: &BTreeMap<u8, char>) -> bool {
    for c in data.iter() {
        if *c == 0 {
            break;
        }
        if d.contains_key(c) {
            return false;
        }
    }
    true
}

pub const G1_TERMINATOR_CODE: u8 = 0x50;
pub const G1_TERMINATOR: char = '\0';
pub const G1_TRADE_OT_CODE: u8 = 0x5D;
pub const G1_TRADE_OT: char = '*';
pub const G1_TRADE_STR: &str = "*";
pub const G1_SPACE_CODE: u8 = 0x7F;

pub fn is_g12_german_byte(data: &[u8]) -> bool {
    for b in data.iter() {
        if (0xC0..=0xC6).contains(b) {
            return true;
        }
    }
    false
}

pub fn is_g12_german_char(value: &[char]) -> bool {
    for c in value.iter() {
        if let Some(b) = U2RBY_U.get(c) {
            if (0xC0..=0xC6).contains(b) {
                return true;
            }
        }
    }
    false
}

pub fn get_string(data: &[u8], jp: bool) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result, jp);
    result.iter().take(len).collect()
}

fn load_string(data: &[u8], result: &mut [char], jp: bool) -> usize {
    if data[0] == G1_TRADE_OT_CODE {
        result[0] = G1_TRADE_OT;
        return 1;
    }

    let dict: &BTreeMap<u8, char> = if jp { &RBY2U_J } else { &RBY2U_U };
    let mut i = 0;
    while i < data.len() {
        let value = &data[i];
        if let Some(c) = dict.get(value) {
            if *c == G1_TERMINATOR {
                break;
            }
            result[i] = *c;
        } else {
            break;
        }
        i += 1;
    }
    i
}

pub fn set_string(
    dest_buffer: &mut [u8],
    mut value: &[char],
    max_length: usize,
    jp: bool,
    option: StringConverterOption,
) -> usize {
    match option {
        StringConverterOption::ClearZero => {
            for b in dest_buffer.iter_mut() {
                *b = 0;
            }
        }
        StringConverterOption::Clear50 => {
            for b in dest_buffer.iter_mut() {
                *b = G1_TERMINATOR_CODE;
            }
        }
        StringConverterOption::Clear7F => {
            for b in dest_buffer.iter_mut() {
                *b = G1_SPACE_CODE;
            }
        }
        _ => {}
    };

    if !value.is_empty() && value[0] == G1_TRADE_OT {
        dest_buffer[0] = G1_TRADE_OT_CODE;
        dest_buffer[1] = G1_TERMINATOR_CODE;
        return 2;
    }

    if value.len() > max_length {
        value = &value[..max_length];
    }

    let dict: &BTreeMap<char, u8> = if jp { &U2RBY_J } else { &U2RBY_U };
    let mut i = 0;
    while i < value.len() {
        let c = &value[i];
        if let Some(val) = dict.get(c) {
            dest_buffer[i] = *val;
        } else {
            break;
        }
        i += 1;
    }

    let count = i;
    if count == dest_buffer.len() {
        count
    } else {
        dest_buffer[count] = G1_TERMINATOR_CODE;
        count + 1
    }
}

pub fn get_g1_char(key: u8, jp: bool) -> char {
    let dict: &BTreeMap<u8, char> = if jp { &RBY2U_J } else { &RBY2U_U };
    *dict.get(&key).unwrap_or(&G1_TERMINATOR)
}

lazy_static! {
    pub static ref U2RBY_J: BTreeMap<char, u8> = BTreeMap::from([
        ('???', 0x05),
        ('???', 0x06),
        ('???', 0x07),
        ('???', 0x08),
        ('???', 0x09),
        ('???', 0x0A),
        ('???', 0x0B),
        ('???', 0x0C),
        ('???', 0x0D),
        ('???', 0x0E),
        ('???', 0x0F),
        ('???', 0x10),
        ('???', 0x11),
        ('???', 0x12),
        ('???', 0x13),
        ('???', 0x19),
        ('???', 0x1A),
        ('???', 0x1B),
        ('???', 0x1C),
        ('???', 0x26),
        ('???', 0x27),
        ('???', 0x28),
        ('???', 0x29),
        ('???', 0x2A),
        ('???', 0x2B),
        ('???', 0x2C),
        ('???', 0x2D),
        ('???', 0x2E),
        ('???', 0x2F),
        ('???', 0x30),
        ('???', 0x31),
        ('???', 0x32),
        ('???', 0x33),
        ('???', 0x34),
        ('???', 0x3A),
        ('???', 0x3B),
        ('???', 0x3C),
        ('???', 0x3D),
        ('???', 0x3D),
        ('???', 0x3E),
        ('???', 0x40),
        ('???', 0x41),
        ('???', 0x42),
        ('???', 0x43),
        ('???', 0x44),
        ('???', 0x45),
        ('???', 0x46),
        ('???', 0x47),
        ('???', 0x47),
        ('???', 0x48),
        (G1_TERMINATOR, 0x50),
        (G1_TRADE_OT, 0x5D), // ???????????????, (Localized per ROM)
        (' ', 0x7F),
        ('???', 0x80),
        ('???', 0x81),
        ('???', 0x82),
        ('???', 0x83),
        ('???', 0x84),
        ('???', 0x85),
        ('???', 0x86),
        ('???', 0x87),
        ('???', 0x88),
        ('???', 0x89),
        ('???', 0x8A),
        ('???', 0x8B),
        ('???', 0x8C),
        ('???', 0x8D),
        ('???', 0x8E),
        ('???', 0x8F),
        ('???', 0x90),
        ('???', 0x91),
        ('???', 0x92),
        ('???', 0x93),
        ('???', 0x94),
        ('???', 0x95),
        ('???', 0x96),
        ('???', 0x97),
        ('???', 0x98),
        ('???', 0x99),
        ('???', 0x9A),
        ('???', 0x9B),
        ('???', 0x9C),
        ('???', 0x9D),
        ('???', 0x9E),
        ('???', 0x9F),
        ('???', 0xA0),
        ('???', 0xA1),
        ('???', 0xA2),
        ('???', 0xA3),
        ('???', 0xA4),
        ('???', 0xA5),
        ('???', 0xA6),
        ('???', 0xA7),
        ('???', 0xA8),
        ('???', 0xA9),
        ('???', 0xAA),
        ('???', 0xAB),
        ('???', 0xAC),
        ('???', 0xAD),
        ('???', 0xAE),
        ('???', 0xAF),
        ('???', 0xB0),
        ('???', 0xB1),
        ('???', 0xB2),
        ('???', 0xB3),
        ('???', 0xB4),
        ('???', 0xB5),
        ('???', 0xB6),
        ('???', 0xB7),
        ('???', 0xB8),
        ('???', 0xB9),
        ('???', 0xBA),
        ('???', 0xBB),
        ('???', 0xBC),
        ('???', 0xBD),
        ('???', 0xBE),
        ('???', 0xBF),
        ('???', 0xC0),
        ('???', 0xC1),
        ('???', 0xC2),
        ('???', 0xC3),
        ('???', 0xC4),
        ('???', 0xC5),
        ('???', 0xC6),
        ('???', 0xC7),
        ('???', 0xC8),
        ('???', 0xC9),
        ('???', 0xCA),
        ('???', 0xCB),
        ('???', 0xCC),
        ('???', 0xCD),
        ('???', 0xCD), // Katakana ??? => Hiragana ???
        ('???', 0xCE),
        ('???', 0xCF),
        ('???', 0xD0),
        ('???', 0xD1),
        ('???', 0xD2),
        ('???', 0xD3),
        ('???', 0xD4),
        ('???', 0xD5),
        ('???', 0xD6),
        ('???', 0xD7),
        ('???', 0xD8),
        ('???', 0xD8),
        ('???', 0xD9),
        ('???', 0xDA),
        ('???', 0xDB),
        ('???', 0xDC),
        ('???', 0xDD),
        ('???', 0xDE),
        ('???', 0xDF),
        ('???', 0xE0),
        ('???', 0xE1),
        ('???', 0xE2),
        ('???', 0xE3),
        ('?', 0xE6),
        ('!', 0xE7),
        ('???', 0xE9),
        ('???', 0xEA),
        ('???', 0xEB),
        ('???', 0xEF),
        ('???', 0xF4),
        ('???', 0xF5),
        ('0', 0xF6),
        ('1', 0xF7),
        ('2', 0xF8),
        ('3', 0xF9),
        ('4', 0xFA),
        ('5', 0xFB),
        ('6', 0xFC),
        ('7', 0xFD),
        ('8', 0xFE),
        ('9', 0xFF),
    ]);
}

lazy_static! {
    pub static ref U2RBY_U: BTreeMap<char, u8> = BTreeMap::from([
        (G1_TERMINATOR, 0x50),
        (G1_TRADE_OT, 0x5D), // TRAINER (Localized per ROM)
        (' ', 0x7F),
        ('A', 0x80),
        ('B', 0x81),
        ('C', 0x82),
        ('D', 0x83),
        ('E', 0x84),
        ('F', 0x85),
        ('G', 0x86),
        ('H', 0x87),
        ('I', 0x88),
        ('J', 0x89),
        ('K', 0x8A),
        ('L', 0x8B),
        ('M', 0x8C),
        ('N', 0x8D),
        ('O', 0x8E),
        ('P', 0x8F),
        ('Q', 0x90),
        ('R', 0x91),
        ('S', 0x92),
        ('T', 0x93),
        ('U', 0x94),
        ('V', 0x95),
        ('W', 0x96),
        ('X', 0x97),
        ('Y', 0x98),
        ('Z', 0x99),
        ('(', 0x9A),
        (')', 0x9B),
        (':', 0x9C),
        (';', 0x9D),
        ('[', 0x9E),
        (']', 0x9F),
        ('a', 0xA0),
        ('b', 0xA1),
        ('c', 0xA2),
        ('d', 0xA3),
        ('e', 0xA4),
        ('f', 0xA5),
        ('g', 0xA6),
        ('h', 0xA7),
        ('i', 0xA8),
        ('j', 0xA9),
        ('k', 0xAA),
        ('l', 0xAB),
        ('m', 0xAC),
        ('n', 0xAD),
        ('o', 0xAE),
        ('p', 0xAF),
        ('q', 0xB0),
        ('r', 0xB1),
        ('s', 0xB2),
        ('t', 0xB3),
        ('u', 0xB4),
        ('v', 0xB5),
        ('w', 0xB6),
        ('x', 0xB7),
        ('y', 0xB8),
        ('z', 0xB9),

        // unused characters
        ('??', 0xBA),
        ('??', 0xBB),
        ('??', 0xBC),
        ('??', 0xBD),
        ('??', 0xBE),
        ('??', 0xBF), // Used in Spanish FALC??N in-game trade, inaccessible from keyboard

        ('??', 0xC0),
        ('??', 0xC1),
        ('??', 0xC2),
        ('??', 0xC3),
        ('??', 0xC4),
        ('??', 0xC5),

        // unused characters
        ('??', 0xC6),
        ('??', 0xC7),
        ('??', 0xC8),
        ('??', 0xC9), // Used in Spanish MAN??A in-game trade, inaccessible from keyboard
        ('??', 0xCA),
        ('??', 0xCB),
        ('??', 0xCC),
        ('??', 0xCD),
        ('??', 0xCE),
        ('??', 0xCF),
        ('??', 0xD0),
        ('??', 0xD1),
        ('??', 0xD2),
        ('??', 0xD3),
        ('??', 0xD4),
        ('??', 0xD5),

        ('\'', 0xE0), // Alias ' to ??? for Farfetch???d
        ('???', 0xE0),
        ('(', 0xE1), /* Pk */
        (')', 0xE2), /* Mn */
        ('-', 0xE3),
        ('?', 0xE6),
        ('!', 0xE7),
        ('???', 0xEF),
        ('??', 0xF1),
        ('.', 0xF2),
        ('/', 0xF3),
        (',', 0xF4),
        ('???', 0xF5),
        ('0', 0xF6),
        ('1', 0xF7),
        ('2', 0xF8),
        ('3', 0xF9),
        ('4', 0xFA),
        ('5', 0xFB),
        ('6', 0xFC),
        ('7', 0xFD),
        ('8', 0xFE),
        ('9', 0xFF),
    ]);
}

lazy_static! {
    pub static ref RBY2U_J: BTreeMap<u8, char> = BTreeMap::from([
        (0x05, '???'),
        (0x06, '???'),
        (0x07, '???'),
        (0x08, '???'),
        (0x09, '???'),
        (0x0A, '???'),
        (0x0B, '???'),
        (0x0C, '???'),
        (0x0D, '???'),
        (0x0E, '???'),
        (0x0F, '???'),
        (0x10, '???'),
        (0x11, '???'),
        (0x12, '???'),
        (0x13, '???'),
        (0x19, '???'),
        (0x1A, '???'),
        (0x1B, '???'),
        (0x1C, '???'),
        (0x26, '???'),
        (0x27, '???'),
        (0x28, '???'),
        (0x29, '???'),
        (0x2A, '???'),
        (0x2B, '???'),
        (0x2C, '???'),
        (0x2D, '???'),
        (0x2E, '???'),
        (0x2F, '???'),
        (0x30, '???'),
        (0x31, '???'),
        (0x32, '???'),
        (0x33, '???'),
        (0x34, '???'),
        (0x3A, '???'),
        (0x3B, '???'),
        (0x3C, '???'),
        (0x3D, '???'),
        (0x3E, '???'),
        (0x40, '???'),
        (0x41, '???'),
        (0x42, '???'),
        (0x43, '???'),
        (0x44, '???'),
        (0x45, '???'),
        (0x46, '???'),
        (0x47, '???'),
        (0x48, '???'),
        (0x50, G1_TERMINATOR),
        (0x5D, G1_TRADE_OT),
        (0x7F, ' '),
        (0x80, '???'),
        (0x81, '???'),
        (0x82, '???'),
        (0x83, '???'),
        (0x84, '???'),
        (0x85, '???'),
        (0x86, '???'),
        (0x87, '???'),
        (0x88, '???'),
        (0x89, '???'),
        (0x8A, '???'),
        (0x8B, '???'),
        (0x8C, '???'),
        (0x8D, '???'),
        (0x8E, '???'),
        (0x8F, '???'),
        (0x90, '???'),
        (0x91, '???'),
        (0x92, '???'),
        (0x93, '???'),
        (0x94, '???'),
        (0x95, '???'),
        (0x96, '???'),
        (0x97, '???'),
        (0x98, '???'),
        (0x99, '???'),
        (0x9A, '???'),
        (0x9B, '???'),
        (0x9C, '???'),
        (0x9D, '???'),
        (0x9E, '???'),
        (0x9F, '???'),
        (0xA0, '???'),
        (0xA1, '???'),
        (0xA2, '???'),
        (0xA3, '???'),
        (0xA4, '???'),
        (0xA5, '???'),
        (0xA6, '???'),
        (0xA7, '???'),
        (0xA8, '???'),
        (0xA9, '???'),
        (0xAA, '???'),
        (0xAB, '???'),
        (0xAC, '???'),
        (0xAD, '???'),
        (0xAE, '???'),
        (0xAF, '???'),
        (0xB0, '???'),
        (0xB1, '???'),
        (0xB2, '???'),
        (0xB3, '???'),
        (0xB4, '???'),
        (0xB5, '???'),
        (0xB6, '???'),
        (0xB7, '???'),
        (0xB8, '???'),
        (0xB9, '???'),
        (0xBA, '???'),
        (0xBB, '???'),
        (0xBC, '???'),
        (0xBD, '???'),
        (0xBE, '???'),
        (0xBF, '???'),
        (0xC0, '???'),
        (0xC1, '???'),
        (0xC2, '???'),
        (0xC3, '???'),
        (0xC4, '???'),
        (0xC5, '???'),
        (0xC6, '???'),
        (0xC7, '???'),
        (0xC8, '???'),
        (0xC9, '???'),
        (0xCA, '???'),
        (0xCB, '???'),
        (0xCC, '???'),
        (0xCD, '???'),
        (0xCE, '???'),
        (0xCF, '???'),
        (0xD0, '???'),
        (0xD1, '???'),
        (0xD2, '???'),
        (0xD3, '???'),
        (0xD4, '???'),
        (0xD5, '???'),
        (0xD6, '???'),
        (0xD7, '???'),
        (0xD8, '???'),
        (0xD9, '???'),
        (0xDA, '???'),
        (0xDB, '???'),
        (0xDC, '???'),
        (0xDD, '???'),
        (0xDE, '???'),
        (0xDF, '???'),
        (0xE0, '???'),
        (0xE1, '???'),
        (0xE2, '???'),
        (0xE3, '???'),
        (0xE6, '?'),
        (0xE7, '!'),
        (0xE9, '???'),
        (0xEA, '???'),
        (0xEB, '???'),
        (0xEF, '???'),
        (0xF4, '???'),
        (0xF5, '???'),
        (0xF6, '0'),
        (0xF7, '1'),
        (0xF8, '2'),
        (0xF9, '3'),
        (0xFA, '4'),
        (0xFB, '5'),
        (0xFC, '6'),
        (0xFD, '7'),
        (0xFE, '8'),
        (0xFF, '9'),
    ]);
}

lazy_static! {
    pub static ref RBY2U_U: BTreeMap<u8, char> = BTreeMap::from([
        (0x50, G1_TERMINATOR),
        (0x5D, G1_TRADE_OT),
        (0x7F, ' '),
        (0x80, 'A'),
        (0x81, 'B'),
        (0x82, 'C'),
        (0x83, 'D'),
        (0x84, 'E'),
        (0x85, 'F'),
        (0x86, 'G'),
        (0x87, 'H'),
        (0x88, 'I'),
        (0x89, 'J'),
        (0x8A, 'K'),
        (0x8B, 'L'),
        (0x8C, 'M'),
        (0x8D, 'N'),
        (0x8E, 'O'),
        (0x8F, 'P'),
        (0x90, 'Q'),
        (0x91, 'R'),
        (0x92, 'S'),
        (0x93, 'T'),
        (0x94, 'U'),
        (0x95, 'V'),
        (0x96, 'W'),
        (0x97, 'X'),
        (0x98, 'Y'),
        (0x99, 'Z'),
        (0x9A, '('),
        (0x9B, ')'),
        (0x9C, ':'),
        (0x9D, ';'),
        (0x9E, '['),
        (0x9F, ']'),
        (0xA0, 'a'),
        (0xA1, 'b'),
        (0xA2, 'c'),
        (0xA3, 'd'),
        (0xA4, 'e'),
        (0xA5, 'f'),
        (0xA6, 'g'),
        (0xA7, 'h'),
        (0xA8, 'i'),
        (0xA9, 'j'),
        (0xAA, 'k'),
        (0xAB, 'l'),
        (0xAC, 'm'),
        (0xAD, 'n'),
        (0xAE, 'o'),
        (0xAF, 'p'),
        (0xB0, 'q'),
        (0xB1, 'r'),
        (0xB2, 's'),
        (0xB3, 't'),
        (0xB4, 'u'),
        (0xB5, 'v'),
        (0xB6, 'w'),
        (0xB7, 'x'),
        (0xB8, 'y'),
        (0xB9, 'z'),

        // unused characters
        (0xBA, '??'),
        (0xBB, '??'),
        (0xBC, '??'),
        (0xBD, '??'),
        (0xBE, '??'),
        (0xBF, '??'), // Used in Spanish FALC??N in-game trade, inaccessible from keyboard

        (0xC0, '??'),
        (0xC1, '??'),
        (0xC2, '??'),
        (0xC3, '??'),
        (0xC4, '??'),
        (0xC5, '??'),

        // unused characters
        (0xC6, '??'),
        (0xC7, '??'),
        (0xC8, '??'),
        (0xC9, '??'), // Used in Spanish MAN??A in-game trade, inaccessible from keyboard
        (0xCA, '??'),
        (0xCB, '??'),
        (0xCC, '??'),
        (0xCD, '??'),
        (0xCE, '??'),
        (0xCF, '??'),
        (0xD0, '??'),
        (0xD1, '??'),
        (0xD2, '??'),
        (0xD3, '??'),
        (0xD4, '??'),
        (0xD5, '??'),

        (0xE0, '???'),
        (0xE1, '('), /* Pk */
        (0xE2, ')'), /* Mn */
        (0xE3, '-'),
        (0xE6, '?'),
        (0xE7, '!'),
        (0xE8, '.'), // Alias decimal point to .
        (0xEF, '???'),
        (0xF1, '??'),
        (0xF2, '.'),
        (0xF3, '/'),
        (0xF4, ','),
        (0xF5, '???'),
        (0xF6, '0'),
        (0xF7, '1'),
        (0xF8, '2'),
        (0xF9, '3'),
        (0xFA, '4'),
        (0xFB, '5'),
        (0xFC, '6'),
        (0xFD, '7'),
        (0xFE, '8'),
        (0xFF, '9'),
    ]);
}
