use crate::string_converter::{get_is_full_width_string, sanitize_char, unsanitize_char};
use crate::{string_converter_7zh, StringConverterOption};

const TERMINATOR_NULL: u16 = 0;

pub fn get_string(data: &[u8]) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result);
    result.iter().take(len).collect()
}

fn load_string(data: &[u8], result: &mut [char]) -> usize {
    let mut i = 0;
    while i < data.len() {
        let value = u16::from_le_bytes((&data[i..(i + 2)]).try_into().unwrap());
        if value == TERMINATOR_NULL {
            break;
        }
        result[i / 2] = sanitize_char(char::from_u32(value as u32).unwrap());
        i += 2;
    }
    string_converter_7zh::remap_chinese_glyphs_bin_to_string(&mut result[..(i / 2)]);
    i / 2
}

pub fn set_string(
    dest_buffer: &mut [u8],
    mut value: Vec<char>,
    max_length: usize,
    language: usize,
    option: StringConverterOption,
    chinese: bool,
) -> usize {
    if value.len() > max_length {
        value = value[..max_length].to_vec();
    }

    if option == StringConverterOption::ClearZero {
        for b in dest_buffer.iter_mut() {
            *b = 0;
        }
    }

    let trad = string_converter_7zh::is_traditional(&value, language);
    let is_full_width = get_is_full_width_string(&value);

    for (i, mut c) in value.clone().into_iter().enumerate() {
        if !is_full_width {
            c = unsanitize_char(c, false);
        }

        if chinese {
            c = string_converter_7zh::convert_string_to_bin_g7_zh(c, trad);
        }

        let bytes = (c as u16).to_le_bytes();
        dest_buffer[i * 2] = bytes[0];
        dest_buffer[i * 2 + 1] = bytes[1];
    }

    let count = value.len() * 2;
    if count == dest_buffer.len() {
        return count;
    }
    let null_bytes = TERMINATOR_NULL.to_le_bytes();
    dest_buffer[count] = null_bytes[0];
    dest_buffer[count + 1] = null_bytes[1];
    count + 2
}
