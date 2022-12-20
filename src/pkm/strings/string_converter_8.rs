use crate::pkm::strings::{string_converter, StringConverterOption};
use no_std_io::{Reader, Writer};

const TERMINATOR_NULL: u16 = 0;

pub fn get_string(data: &[u8]) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result);
    result.into_iter().take(len).collect::<String>()
}

fn load_string(data: &[u8], result: &mut [char]) -> usize {
    let mut i = 0;
    while i < data.len() {
        let val = data.default_read_le::<u16>(i);
        if val == TERMINATOR_NULL {
            break;
        }
        result[i / 2] = string_converter::sanitize_char(char::from_u32(val as u32).unwrap());
        i += 2;
    }
    i / 2
}

pub fn set_string(
    mut dest_buffer: &mut [u8],
    mut value: &[char],
    max_length: usize,
    option: Option<StringConverterOption>,
) -> usize {
    let option = option.unwrap_or(StringConverterOption::ClearZero);
    if value.len() > max_length {
        value = &value[..max_length];
    }

    if option == StringConverterOption::ClearZero {
        dest_buffer.fill(0);
    }

    let is_full_width = string_converter::get_is_full_width_string(value);
    for (i, mut c) in value.iter().copied().enumerate() {
        if !is_full_width {
            c = string_converter::un_sanitize_char(c, false);
        }
        dest_buffer.checked_write_le(i * 2, &(c as u16));
    }
    let count = value.len() * 2;
    if count == dest_buffer.len() {
        count
    } else {
        dest_buffer.checked_write_le(count, &TERMINATOR_NULL);
        count + 2
    }
}
