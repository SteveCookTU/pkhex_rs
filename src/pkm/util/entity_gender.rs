use crate::personal_info;

pub fn get_from_pid_and_ratio(pid: u32, gr: usize) -> u8 {
    match gr {
        _ if gr == personal_info::RATIO_MAGIC_GENDERLESS => 2,
        _ if gr == personal_info::RATIO_MAGIC_FEMALE => 1,
        _ if gr == personal_info::RATIO_MAGIC_MALE => 0,
        _ => u8::from(((pid & 0xFF) as usize) < gr),
    }
}
