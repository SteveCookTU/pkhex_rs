use crate::personal_info;
use crate::personal_info::traits::gender_detail;
use crate::personal_info::traits::gender_detail::GenderDetail;

pub fn get_from_string(s: &str) -> u8 {
    if s.len() != 1 {
        return 2;
    }
    get_from_char(s.chars().next().unwrap())
}

pub fn get_from_char(c: char) -> u8 {
    match c {
        'M' | '♂' => 0,
        'F' | '♀' => 1,
        _ => 2,
    }
}

pub fn get_from_pid(species: u16, pid: u32) -> u8 {
    let gt = personal_info::SV[species as usize].gender();
    get_from_pid_and_ratio(pid, gt)
}

pub fn get_from_pid_and_ratio(pid: u32, gr: u8) -> u8 {
    match gr {
        gender_detail::RATIO_MAGIC_GENDERLESS => 2,
        gender_detail::RATIO_MAGIC_FEMALE => 1,
        gender_detail::RATIO_MAGIC_MALE => 0,
        _ => u8::from(((pid & 0xFF) as u8) < gr),
    }
}
