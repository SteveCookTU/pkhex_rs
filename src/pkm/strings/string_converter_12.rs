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
        ('ガ', 0x05),
        ('ギ', 0x06),
        ('グ', 0x07),
        ('ゲ', 0x08),
        ('ゴ', 0x09),
        ('ザ', 0x0A),
        ('ジ', 0x0B),
        ('ズ', 0x0C),
        ('ゼ', 0x0D),
        ('ゾ', 0x0E),
        ('ダ', 0x0F),
        ('ヂ', 0x10),
        ('ヅ', 0x11),
        ('デ', 0x12),
        ('ド', 0x13),
        ('バ', 0x19),
        ('ビ', 0x1A),
        ('ブ', 0x1B),
        ('ボ', 0x1C),
        ('が', 0x26),
        ('ぎ', 0x27),
        ('ぐ', 0x28),
        ('げ', 0x29),
        ('ご', 0x2A),
        ('ざ', 0x2B),
        ('じ', 0x2C),
        ('ず', 0x2D),
        ('ぜ', 0x2E),
        ('ぞ', 0x2F),
        ('だ', 0x30),
        ('ぢ', 0x31),
        ('づ', 0x32),
        ('で', 0x33),
        ('ど', 0x34),
        ('ば', 0x3A),
        ('び', 0x3B),
        ('ぶ', 0x3C),
        ('ベ', 0x3D),
        ('べ', 0x3D),
        ('ぼ', 0x3E),
        ('パ', 0x40),
        ('ピ', 0x41),
        ('プ', 0x42),
        ('ポ', 0x43),
        ('ぱ', 0x44),
        ('ぴ', 0x45),
        ('ぷ', 0x46),
        ('ぺ', 0x47),
        ('ペ', 0x47),
        ('ぽ', 0x48),
        (G1_TERMINATOR, 0x50),
        (G1_TRADE_OT, 0x5D), // トレーナー, (Localized per ROM)
        (' ', 0x7F),
        ('ア', 0x80),
        ('イ', 0x81),
        ('ウ', 0x82),
        ('エ', 0x83),
        ('オ', 0x84),
        ('カ', 0x85),
        ('キ', 0x86),
        ('ク', 0x87),
        ('ケ', 0x88),
        ('コ', 0x89),
        ('サ', 0x8A),
        ('シ', 0x8B),
        ('ス', 0x8C),
        ('セ', 0x8D),
        ('ソ', 0x8E),
        ('タ', 0x8F),
        ('チ', 0x90),
        ('ツ', 0x91),
        ('テ', 0x92),
        ('ト', 0x93),
        ('ナ', 0x94),
        ('ニ', 0x95),
        ('ヌ', 0x96),
        ('ネ', 0x97),
        ('ノ', 0x98),
        ('ハ', 0x99),
        ('ヒ', 0x9A),
        ('フ', 0x9B),
        ('ホ', 0x9C),
        ('マ', 0x9D),
        ('ミ', 0x9E),
        ('ム', 0x9F),
        ('メ', 0xA0),
        ('モ', 0xA1),
        ('ヤ', 0xA2),
        ('ユ', 0xA3),
        ('ヨ', 0xA4),
        ('ラ', 0xA5),
        ('ル', 0xA6),
        ('レ', 0xA7),
        ('ロ', 0xA8),
        ('ワ', 0xA9),
        ('ヲ', 0xAA),
        ('ン', 0xAB),
        ('ッ', 0xAC),
        ('ャ', 0xAD),
        ('ュ', 0xAE),
        ('ョ', 0xAF),
        ('ィ', 0xB0),
        ('あ', 0xB1),
        ('い', 0xB2),
        ('う', 0xB3),
        ('え', 0xB4),
        ('お', 0xB5),
        ('か', 0xB6),
        ('き', 0xB7),
        ('く', 0xB8),
        ('け', 0xB9),
        ('こ', 0xBA),
        ('さ', 0xBB),
        ('し', 0xBC),
        ('す', 0xBD),
        ('せ', 0xBE),
        ('そ', 0xBF),
        ('た', 0xC0),
        ('ち', 0xC1),
        ('つ', 0xC2),
        ('て', 0xC3),
        ('と', 0xC4),
        ('な', 0xC5),
        ('に', 0xC6),
        ('ぬ', 0xC7),
        ('ね', 0xC8),
        ('の', 0xC9),
        ('は', 0xCA),
        ('ひ', 0xCB),
        ('ふ', 0xCC),
        ('へ', 0xCD),
        ('ヘ', 0xCD), // Katakana ヘ => Hiragana へ
        ('ほ', 0xCE),
        ('ま', 0xCF),
        ('み', 0xD0),
        ('む', 0xD1),
        ('め', 0xD2),
        ('も', 0xD3),
        ('や', 0xD4),
        ('ゆ', 0xD5),
        ('よ', 0xD6),
        ('ら', 0xD7),
        ('リ', 0xD8),
        ('り', 0xD8),
        ('る', 0xD9),
        ('れ', 0xDA),
        ('ろ', 0xDB),
        ('わ', 0xDC),
        ('を', 0xDD),
        ('ん', 0xDE),
        ('っ', 0xDF),
        ('ゃ', 0xE0),
        ('ゅ', 0xE1),
        ('ょ', 0xE2),
        ('ー', 0xE3),
        ('?', 0xE6),
        ('!', 0xE7),
        ('ァ', 0xE9),
        ('ゥ', 0xEA),
        ('ェ', 0xEB),
        ('♂', 0xEF),
        ('ォ', 0xF4),
        ('♀', 0xF5),
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
        ('à', 0xBA),
        ('è', 0xBB),
        ('é', 0xBC),
        ('ù', 0xBD),
        ('À', 0xBE),
        ('Á', 0xBF), // Used in Spanish FALCÁN in-game trade, inaccessible from keyboard

        ('Ä', 0xC0),
        ('Ö', 0xC1),
        ('Ü', 0xC2),
        ('ä', 0xC3),
        ('ö', 0xC4),
        ('ü', 0xC5),

        // unused characters
        ('È', 0xC6),
        ('É', 0xC7),
        ('Ì', 0xC8),
        ('Í', 0xC9), // Used in Spanish MANÍA in-game trade, inaccessible from keyboard
        ('Ñ', 0xCA),
        ('Ò', 0xCB),
        ('Ó', 0xCC),
        ('Ù', 0xCD),
        ('Ú', 0xCE),
        ('á', 0xCF),
        ('ì', 0xD0),
        ('í', 0xD1),
        ('ñ', 0xD2),
        ('ò', 0xD3),
        ('ó', 0xD4),
        ('ú', 0xD5),

        ('\'', 0xE0), // Alias ' to ’ for Farfetch’d
        ('’', 0xE0),
        ('(', 0xE1), /* Pk */
        (')', 0xE2), /* Mn */
        ('-', 0xE3),
        ('?', 0xE6),
        ('!', 0xE7),
        ('♂', 0xEF),
        ('×', 0xF1),
        ('.', 0xF2),
        ('/', 0xF3),
        (',', 0xF4),
        ('♀', 0xF5),
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
        (0x05, 'ガ'),
        (0x06, 'ギ'),
        (0x07, 'グ'),
        (0x08, 'ゲ'),
        (0x09, 'ゴ'),
        (0x0A, 'ザ'),
        (0x0B, 'ジ'),
        (0x0C, 'ズ'),
        (0x0D, 'ゼ'),
        (0x0E, 'ゾ'),
        (0x0F, 'ダ'),
        (0x10, 'ヂ'),
        (0x11, 'ヅ'),
        (0x12, 'デ'),
        (0x13, 'ド'),
        (0x19, 'バ'),
        (0x1A, 'ビ'),
        (0x1B, 'ブ'),
        (0x1C, 'ボ'),
        (0x26, 'が'),
        (0x27, 'ぎ'),
        (0x28, 'ぐ'),
        (0x29, 'げ'),
        (0x2A, 'ご'),
        (0x2B, 'ざ'),
        (0x2C, 'じ'),
        (0x2D, 'ず'),
        (0x2E, 'ぜ'),
        (0x2F, 'ぞ'),
        (0x30, 'だ'),
        (0x31, 'ぢ'),
        (0x32, 'づ'),
        (0x33, 'で'),
        (0x34, 'ど'),
        (0x3A, 'ば'),
        (0x3B, 'び'),
        (0x3C, 'ぶ'),
        (0x3D, 'ベ'),
        (0x3E, 'ぼ'),
        (0x40, 'パ'),
        (0x41, 'ピ'),
        (0x42, 'プ'),
        (0x43, 'ポ'),
        (0x44, 'ぱ'),
        (0x45, 'ぴ'),
        (0x46, 'ぷ'),
        (0x47, 'ペ'),
        (0x48, 'ぽ'),
        (0x50, G1_TERMINATOR),
        (0x5D, G1_TRADE_OT),
        (0x7F, ' '),
        (0x80, 'ア'),
        (0x81, 'イ'),
        (0x82, 'ウ'),
        (0x83, 'エ'),
        (0x84, 'オ'),
        (0x85, 'カ'),
        (0x86, 'キ'),
        (0x87, 'ク'),
        (0x88, 'ケ'),
        (0x89, 'コ'),
        (0x8A, 'サ'),
        (0x8B, 'シ'),
        (0x8C, 'ス'),
        (0x8D, 'セ'),
        (0x8E, 'ソ'),
        (0x8F, 'タ'),
        (0x90, 'チ'),
        (0x91, 'ツ'),
        (0x92, 'テ'),
        (0x93, 'ト'),
        (0x94, 'ナ'),
        (0x95, 'ニ'),
        (0x96, 'ヌ'),
        (0x97, 'ネ'),
        (0x98, 'ノ'),
        (0x99, 'ハ'),
        (0x9A, 'ヒ'),
        (0x9B, 'フ'),
        (0x9C, 'ホ'),
        (0x9D, 'マ'),
        (0x9E, 'ミ'),
        (0x9F, 'ム'),
        (0xA0, 'メ'),
        (0xA1, 'モ'),
        (0xA2, 'ヤ'),
        (0xA3, 'ユ'),
        (0xA4, 'ヨ'),
        (0xA5, 'ラ'),
        (0xA6, 'ル'),
        (0xA7, 'レ'),
        (0xA8, 'ロ'),
        (0xA9, 'ワ'),
        (0xAA, 'ヲ'),
        (0xAB, 'ン'),
        (0xAC, 'ッ'),
        (0xAD, 'ャ'),
        (0xAE, 'ュ'),
        (0xAF, 'ョ'),
        (0xB0, 'ィ'),
        (0xB1, 'あ'),
        (0xB2, 'い'),
        (0xB3, 'う'),
        (0xB4, 'え'),
        (0xB5, 'お'),
        (0xB6, 'か'),
        (0xB7, 'き'),
        (0xB8, 'く'),
        (0xB9, 'け'),
        (0xBA, 'こ'),
        (0xBB, 'さ'),
        (0xBC, 'し'),
        (0xBD, 'す'),
        (0xBE, 'せ'),
        (0xBF, 'そ'),
        (0xC0, 'た'),
        (0xC1, 'ち'),
        (0xC2, 'つ'),
        (0xC3, 'て'),
        (0xC4, 'と'),
        (0xC5, 'な'),
        (0xC6, 'に'),
        (0xC7, 'ぬ'),
        (0xC8, 'ね'),
        (0xC9, 'の'),
        (0xCA, 'は'),
        (0xCB, 'ひ'),
        (0xCC, 'ふ'),
        (0xCD, 'へ'),
        (0xCE, 'ほ'),
        (0xCF, 'ま'),
        (0xD0, 'み'),
        (0xD1, 'む'),
        (0xD2, 'め'),
        (0xD3, 'も'),
        (0xD4, 'や'),
        (0xD5, 'ゆ'),
        (0xD6, 'よ'),
        (0xD7, 'ら'),
        (0xD8, 'リ'),
        (0xD9, 'る'),
        (0xDA, 'れ'),
        (0xDB, 'ろ'),
        (0xDC, 'わ'),
        (0xDD, 'を'),
        (0xDE, 'ん'),
        (0xDF, 'っ'),
        (0xE0, 'ゃ'),
        (0xE1, 'ゅ'),
        (0xE2, 'ょ'),
        (0xE3, 'ー'),
        (0xE6, '?'),
        (0xE7, '!'),
        (0xE9, 'ァ'),
        (0xEA, 'ゥ'),
        (0xEB, 'ェ'),
        (0xEF, '♂'),
        (0xF4, 'ォ'),
        (0xF5, '♀'),
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
        (0xBA, 'à'),
        (0xBB, 'è'),
        (0xBC, 'é'),
        (0xBD, 'ù'),
        (0xBE, 'À'),
        (0xBF, 'Á'), // Used in Spanish FALCÁN in-game trade, inaccessible from keyboard

        (0xC0, 'Ä'),
        (0xC1, 'Ö'),
        (0xC2, 'Ü'),
        (0xC3, 'ä'),
        (0xC4, 'ö'),
        (0xC5, 'ü'),

        // unused characters
        (0xC6, 'È'),
        (0xC7, 'É'),
        (0xC8, 'Ì'),
        (0xC9, 'Í'), // Used in Spanish MANÍA in-game trade, inaccessible from keyboard
        (0xCA, 'Ñ'),
        (0xCB, 'Ò'),
        (0xCC, 'Ó'),
        (0xCD, 'Ù'),
        (0xCE, 'Ú'),
        (0xCF, 'á'),
        (0xD0, 'ì'),
        (0xD1, 'í'),
        (0xD2, 'ñ'),
        (0xD3, 'ò'),
        (0xD4, 'ó'),
        (0xD5, 'ú'),

        (0xE0, '’'),
        (0xE1, '('), /* Pk */
        (0xE2, ')'), /* Mn */
        (0xE3, '-'),
        (0xE6, '?'),
        (0xE7, '!'),
        (0xE8, '.'), // Alias decimal point to .
        (0xEF, '♂'),
        (0xF1, '×'),
        (0xF2, '.'),
        (0xF3, '/'),
        (0xF4, ','),
        (0xF5, '♀'),
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
