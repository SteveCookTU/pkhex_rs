pub fn sanitize_char(cr: char) -> char {
    match cr {
        '\u{E08F}' => '♀',
        '\u{E08E}' => '♂',
        '\u{246E}' => '♀',
        '\u{246D}' => '♀',
        _ => cr,
    }
}

pub fn unsanitize_char(cr: char, full_width: bool) -> char {
    if full_width {
        return cr;
    }

    match cr {
        '\u{2640}' => '\u{E08F}',
        '\u{2642}' => '\u{E08E}',
        _ => cr,
    }
}

pub fn get_is_full_width_string(str: &Vec<char>) -> bool {
    for c in str.iter() {
        let shifted = (*c as u16) >> 12;
        if shifted == 0 || shifted == 0xE {
            continue;
        }
        if *c == '\u{2640}' || *c == '\u{2642}' {
            continue;
        }
        return false;
    }
    true
}
