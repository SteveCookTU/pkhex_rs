use crate::{personal_info, personal_table, PersonalInfo};

pub fn get_from_string(s: &str) -> u8 {
    if s.len() != 1 {
        2
    } else {
        get_from_char(s.chars().next().unwrap_or_default())
    }
}

pub fn get_from_char(c: char) -> u8 {
    match c {
        '♂' | 'M' => 0,
        '♀' | 'F' => 1,
        _ => 2,
    }
}

pub fn get_from_pid(species: u16, pid: u32) -> u8 {
    let gt = personal_table::LA[species as usize].get_gender();
    get_from_pid_and_ratio(pid, gt)
}

pub fn get_from_pid_and_ratio(pid: u32, gr: usize) -> u8 {
    match gr {
        _ if gr == personal_info::RATIO_MAGIC_GENDERLESS => 2,
        _ if gr == personal_info::RATIO_MAGIC_FEMALE => 1,
        _ if gr == personal_info::RATIO_MAGIC_MALE => 0,
        _ => u8::from(((pid & 0xFF) as usize) < gr),
    }
}
