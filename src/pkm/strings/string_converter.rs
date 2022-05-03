pub fn sanitize_char(cr: char) -> char {
    match cr {
        '\u{E08F}' => '♀',
        '\u{E08E}' => '♂',
        '\u{246E}' => '♀',
        '\u{246D}' => '♀',
        _ => cr
    }
}