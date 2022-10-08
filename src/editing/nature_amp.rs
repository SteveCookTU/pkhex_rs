pub fn get_nature_modification(nature: u8) -> (u8, u8) {
    let up = (nature / 5) + 1;
    let dn = (nature % 5) + 1;
    (up, dn)
}

pub fn is_neutral_or_invalid(nature: u8, up: u8, dn: u8) -> bool {
    up == dn || nature >= 25
}

pub fn is_neutral_or_invalid_nature(nature: u8) -> bool {
    let (up, dn) = get_nature_modification(nature);
    is_neutral_or_invalid(nature, up, dn)
}

pub fn modify_stats_for_nature(stats: &mut [u16; 6], nature: u8) {
    let (up, dn) = get_nature_modification(nature);
    if !is_neutral_or_invalid(nature, up, dn) {
        stats[up as usize] *= 11;
        stats[up as usize] /= 10;
        stats[dn as usize] *= 9;
        stats[dn as usize] /= 10;
    }
}

pub enum NatureAmpRequest {
    Neutral,
    Increase,
    Decrease,
}
