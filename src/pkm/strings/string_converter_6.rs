use crate::string_converter::sanitize_char;

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
        result[i / 2] = sanitize_char(char::from_u32(value as u32).unwrap());
        i += 2;
    }
    i / 2
}