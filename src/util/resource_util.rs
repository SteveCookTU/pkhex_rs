

pub fn load_string_list(list: &str) -> Vec<&str> {
    list.split('\n').map(|s| {
        if s.len() == 0 {
            s
        } else if s.as_bytes()[s.len() - 1] == ('\r' as u8) {
            &s[..(s.len() - 1)]
        } else {
            s
        }
    }).collect()
}