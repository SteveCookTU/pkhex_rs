use crate::StringConverterOption;

const TERMINATOR_BYTE: u8 = 0xFF;
const TERMINATOR: char = TERMINATOR_BYTE as char;

pub fn get_string(data: &[u8], jp: bool) -> String {
    let mut result = vec![char::default(); data.len()];
    let mut i = 0;
    while i < data.len() {
        let value = data[i];
        let c = get_g3_char(value, jp);
        if c == TERMINATOR {
            break;
        }
        result[i] = c;
        i += 1;
    }

    result[..i].iter().collect::<String>()
}

pub fn set_string(
    buffer: &mut [u8],
    mut value: &[char],
    max_length: usize,
    jp: bool,
    option: StringConverterOption,
) -> usize {
    if value.len() > max_length {
        value = &value[..max_length];
    }

    match option {
        StringConverterOption::ClearFF => {
            buffer.iter_mut().for_each(|b| *b = 0xFF);
        }
        StringConverterOption::ClearZero => {
            buffer.iter_mut().for_each(|b| *b = 0);
        }
        _ => {}
    }

    let mut i = 0;
    while i < value.len() {
        let chr = value[i];
        let b = set_g3_char(chr, jp);
        if b == TERMINATOR_BYTE {
            break;
        }
        buffer[i] = b;
        i += 1;
    }

    let mut count = i;
    if count < buffer.len() {
        buffer[count] = TERMINATOR_BYTE;
        count += 1;
    }

    count
}

fn get_g3_char(chr: u8, jp: bool) -> char {
    let table = if jp { &G3_JP } else { &G3_EN };
    table[chr as usize]
}

fn set_g3_char(chr: char, jp: bool) -> u8 {
    if chr == '\'' {
        0xB4
    } else {
        let table = if jp { &G3_JP } else { &G3_EN };
        let index = table.iter().enumerate().find(|(_, &c)| c == chr);
        if let Some((index, _)) = index {
            index as u8
        } else {
            TERMINATOR_BYTE
        }
    }
}

const G3_EN: [char; 256] = [
    ' ', 'À', 'Á', 'Â', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'こ', 'Î', 'Ï', 'Ò', 'Ó', 'Ô', // 0
    'Œ', 'Ù', 'Ú', 'Û', 'Ñ', 'ß', 'à', 'á', 'ね', 'Ç', 'È', 'é', 'ê', 'ë', 'ì', 'í', // 1
    'î', 'ï', 'ò', 'ó', 'ô', 'œ', 'ù', 'ú', 'û', 'ñ', 'º', 'ª', '⒅', '&', '+', 'あ', // 2
    'ぃ', 'ぅ', 'ぇ', 'ぉ', 'ゃ', '=', 'ょ', 'が', 'ぎ', 'ぐ', 'げ', 'ご', 'ざ', 'じ', 'ず',
    'ぜ', // 3
    'ぞ', 'だ', 'ぢ', 'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ', 'ぱ', 'ぴ', 'ぷ', 'ぺ',
    'ぽ', // 4
    'っ', '¿', '¡', '⒆', '⒇', 'オ', 'カ', 'キ', 'ク', 'ケ', 'Í', 'コ', 'サ', 'ス', 'セ',
    'ソ', // 5
    'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'â', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'í', // 6
    'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
    'ァ', // 7
    'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ガ', 'ギ', 'グ', 'ゲ', 'ゴ', 'ザ', 'ジ', 'ズ',
    'ゼ', // 8
    'ゾ', 'ダ', 'ヂ', 'ヅ', 'デ', 'ド', 'バ', 'ビ', 'ブ', 'ベ', 'ボ', 'パ', 'ピ', 'プ', 'ペ',
    'ポ', // 9
    'ッ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '-', '・', // A
    '⑬', '“', '”', '‘', '’', '♂', '♀', '$', ',', '⑧', '/', 'A', 'B', 'C', 'D', 'E', // B
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', // C
    'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', // D
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', // E
    ':', 'Ä', 'Ö', 'Ü', 'ä', 'ö', 'ü', // F
    // Make the total length 256 so that any byte access is always within the array
    TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR,
    TERMINATOR,
];

const G3_JP: [char; 256] = [
    '　', 'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ',
    'そ', // 0
    'た', 'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ま', // 1
    'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん',
    'ぁ', // 2
    'ぃ', 'ぅ', 'ぇ', 'ぉ', 'ゃ', 'ゅ', 'ょ', 'が', 'ぎ', 'ぐ', 'げ', 'ご', 'ざ', 'じ', 'ず',
    'ぜ', // 3
    'ぞ', 'だ', 'ぢ', 'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ', 'ぱ', 'ぴ', 'ぷ', 'ぺ',
    'ぽ', // 4
    'っ', 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ',
    'ソ', // 5
    'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'マ', // 6
    'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
    'ァ', // 7
    'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ガ', 'ギ', 'グ', 'ゲ', 'ゴ', 'ザ', 'ジ', 'ズ',
    'ゼ', // 8
    'ゾ', 'ダ', 'ヂ', 'ヅ', 'デ', 'ド', 'バ', 'ビ', 'ブ', 'ベ', 'ボ', 'パ', 'ピ', 'プ', 'ペ',
    'ポ', // 9
    'ッ', '０', '１', '２', '３', '４', '５', '６', '７', '８', '９', '！', '？', '。', 'ー',
    '・', // A
    '⋯', '『', '』', '「', '」', '♂', '♀', '$', '.', '⑧', '/', 'Ａ', 'Ｂ', 'Ｃ', 'Ｄ',
    'Ｅ', // B
    'Ｆ', 'Ｇ', 'Ｈ', 'Ｉ', 'Ｊ', 'Ｋ', 'Ｌ', 'Ｍ', 'Ｎ', 'Ｏ', 'Ｐ', 'Ｑ', 'Ｒ', 'Ｓ', 'Ｔ',
    'Ｕ', // C
    'Ｖ', 'Ｗ', 'Ｘ', 'Ｙ', 'Ｚ', 'ａ', 'ｂ', 'ｃ', 'ｄ', 'ｅ', 'ｆ', 'ｇ', 'ｈ', 'ｉ', 'ｊ',
    'ｋ', // D
    'ｌ', 'ｍ', 'ｎ', 'ｏ', 'ｐ', 'ｑ', 'ｒ', 'ｓ', 'ｔ', 'ｕ', 'ｖ', 'ｗ', 'ｘ', 'ｙ', 'ｚ',
    '0', // E
    ':', 'Ä', 'Ö', 'Ü', 'ä', 'ö', 'ü', // F
    // Make the total length 256 so that any byte access is always within the array
    TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR, TERMINATOR,
    TERMINATOR,
];
