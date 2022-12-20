use crate::game::enums::Nature;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum NatureAmpRequest {
    Neutral,
    Increase,
    Decrease,
}

impl NatureAmpRequest {
    pub fn get_new_nature(&self, stat_index: usize, current_nature: u8) -> Option<u8> {
        if current_nature > Nature::Quirky as u8 {
            return None;
        }
        let (up, dn) = get_nature_modification(current_nature);
        get_new_nature(self, stat_index, up, dn)
    }
}

pub fn get_new_nature(
    t: &NatureAmpRequest,
    stat_index: usize,
    mut up: u8,
    mut dn: u8,
) -> Option<u8> {
    match t {
        NatureAmpRequest::Neutral if usize::from(up) != stat_index => {
            up = stat_index as u8;
        }
        NatureAmpRequest::Increase if usize::from(dn) != stat_index => {
            dn = stat_index as u8;
        }
        NatureAmpRequest::Decrease
            if usize::from(up) != stat_index && usize::from(dn) != stat_index =>
        {
            up = stat_index as u8;
            dn = stat_index as u8;
        }
        _ => return None,
    };

    create_nature_from_amps(up, dn)
}

pub fn create_nature_from_amps(up: u8, dn: u8) -> Option<u8> {
    if up > 5 || dn > 5 {
        None
    } else {
        Some((up * 5) + dn)
    }
}

pub fn get_nature_modification(nature: u8) -> (u8, u8) {
    let up = (nature / 5) + 1;
    let down = (nature % 5) + 1;
    (up, down)
}

pub fn is_nature_or_invalid(nature: u8, up: u8, dn: u8) -> bool {
    up == dn || nature >= 25
}

pub fn is_nature_param_neutral_or_invalid(nature: u8) -> bool {
    let (up, dn) = get_nature_modification(nature);
    is_nature_or_invalid(nature, up, dn)
}

pub fn modify_stats_for_nature(stats: &mut [u16], nature: u8) {
    let (up, dn) = get_nature_modification(nature);
    if is_nature_or_invalid(nature, up, dn) {
        return;
    }
    stats[up as usize] *= 11;
    stats[up as usize] /= 10;
    stats[dn as usize] *= 9;
    stats[dn as usize] /= 10;
}
