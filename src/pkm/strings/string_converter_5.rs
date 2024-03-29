use crate::string_converter::{sanitize_char, unsanitize_char};
use crate::StringConverterOption;

const TERMINATOR_FFFF: u16 = 0xFFFF;

pub fn get_string(data: &[u8]) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result);
    result.iter().take(len).collect()
}

pub fn load_string(data: &[u8], result: &mut [char]) -> usize {
    let mut i = 0;
    while i < data.len() {
        let value = u16::from_le_bytes((&data[i..(i + 2)]).try_into().unwrap());
        if value == TERMINATOR_FFFF {
            break;
        }
        result[i / 2] = sanitize_char(char::from_u32(value as u32).unwrap());
        i += 2;
    }
    i / 2
}

pub fn set_string(
    dest_buffer: &mut [u8],
    mut value: Vec<char>,
    max_length: usize,
    option: StringConverterOption,
) -> usize {
    if value.len() > max_length {
        value = value[..max_length].to_vec();
    }

    match option {
        StringConverterOption::ClearZero => {
            for b in dest_buffer.iter_mut() {
                *b = 0;
            }
        }
        StringConverterOption::ClearFF => {
            for b in dest_buffer.iter_mut() {
                *b = 0xFF;
            }
        }
        _ => {}
    };

    for (i, mut c) in value.clone().into_iter().enumerate() {
        c = unsanitize_char(c, false);
        let bytes = (c as u16).to_le_bytes();
        dest_buffer[i * 2] = bytes[0];
        dest_buffer[i * 2 + 1] = bytes[1];
    }

    let count = value.len() * 2;
    if count == dest_buffer.len() {
        return count;
    }
    let null_bytes = TERMINATOR_FFFF.to_le_bytes();
    dest_buffer[count] = null_bytes[0];
    dest_buffer[count + 1] = null_bytes[1];
    count + 2
}
