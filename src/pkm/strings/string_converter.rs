pub(crate) fn sanitize_char(chr: char) -> char {
    match chr {
        '\u{E08F}' => '♀',
        '\u{E08E}' => '♂',
        '\u{246E}' => '♀',
        '\u{246D}' => '♂',
        _ => chr,
    }
}

pub(crate) fn un_sanitize_char(chr: char, full_width: bool) -> char {
    if full_width {
        return chr;
    }

    match chr {
        '\u{2640}' => '\u{E08F}',
        '\u{2642}' => '\u{E08E}',
        _ => chr,
    }
}

pub(crate) fn get_is_full_width_string(str: &[char]) -> bool {
    for c in str {
        if [0, 0xE].contains(&((*c as u32) >> 12)) {
            continue;
        }
        if !['\u{2640}', '\u{2642}'].contains(c) {
            continue;
        }
        return true;
    }
    false
}
