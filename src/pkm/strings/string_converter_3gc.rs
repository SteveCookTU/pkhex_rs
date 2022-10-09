use crate::StringConverterOption;

const TERMINATOR_BIG_ENDIAN: u16 = 0;

pub fn get_string(data: &[u8]) -> String {
    let mut result = vec![char::default(); data.len()];
    let len = load_string(data, &mut result);
    result.iter().take(len).collect()
}

fn load_string(data: &[u8], result: &mut [char]) -> usize {
    let mut i = 0;
    while i < data.len() {
        let value = u16::from_be_bytes((&data[i..(i + 2)]).try_into().unwrap());
        if value == TERMINATOR_BIG_ENDIAN {
            break;
        }
        result[i / 2] = char::from_u32(value as u32).unwrap();
        i += 2;
    }
    i / 2
}

pub fn set_string(
    dest: &mut [u8],
    mut value: Vec<char>,
    max_length: usize,
    option: StringConverterOption,
) -> usize {
    if value.len() > max_length {
        value = value[..max_length].to_vec();
    }

    if option == StringConverterOption::ClearZero {
        dest.iter_mut().for_each(|b| *b = 0);
    }

    for (i, c) in value.clone().into_iter().enumerate() {
        let bytes = (c as u16).to_be_bytes();
        dest[i * 2] = bytes[0];
        dest[i * 2 + 1] = bytes[1];
    }

    if dest.len() == value.len() * 2 {
        value.len() * 2
    } else {
        let bytes = TERMINATOR_BIG_ENDIAN.to_be_bytes();
        dest[value.len() * 2] = bytes[0];
        dest[value.len() * 2 + 1] = bytes[1];
        (value.len() * 2) + 2
    }
}
