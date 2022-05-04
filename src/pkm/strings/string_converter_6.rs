use crate::string_converter::{get_is_full_width_string, sanitize_char, unsanitize_char};
use crate::StringConverterOption;

const TERMINATOR_NULL: u16 = 0;

pub fn get_string(data: Vec<u8>) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result);
    result.iter().take(len).collect()
}

fn load_string(data: Vec<u8>, result: &mut Vec<char>) -> usize {
    let mut i = 0;
    while i < data.len() {
        let value = u16::from_le_bytes((&data[i..(i + 2)]).try_into().unwrap());
        if value == TERMINATOR_NULL {
            break;
        }
		println!("{}", char::from_u32(value as u32).unwrap());
        result[i / 2] = sanitize_char(char::from_u32(value as u32).unwrap());
        i += 2;
    }
    i / 2
}

pub fn set_string(
    dest_buffer: &mut Vec<u8>,
    mut value: Vec<char>,
    max_length: usize,
    option: StringConverterOption,
) -> usize {
    if value.len() > max_length {
        value = value[..max_length].to_vec();
    }

    if option == StringConverterOption::ClearZero {
        *dest_buffer = dest_buffer.iter_mut().map(|_| 0).collect::<Vec<u8>>();
    }

    let is_full_width = get_is_full_width_string(&value);

    for (i, mut c) in value.clone().into_iter().enumerate() {
        if !is_full_width {
            c = unsanitize_char(c, false);
        }

        dest_buffer.splice((i * 2)..(i * 2 + 2), (c as u16).to_le_bytes());
    }

    let count = value.len();
    if count == dest_buffer.len() {
        return count;
    }
    dest_buffer.splice(count..(count + 2), TERMINATOR_NULL.to_le_bytes());
    count + 2
}
