use crate::personal_table::LA;
use crate::{is_single_gender, PersonalInfo, Species};
use rand::rngs::ThreadRng;
use rand::RngCore;

pub const GENERATION: usize = 8;

const EXTENSION_PB7: &str = "pb7";
const EXTENSION_PB8: &str = "pb8";
const EXTENSION_PA8: &str = "pa8";

pub fn get_gender_from_pid_and_ratio(pid: usize, gr: usize) -> usize {
    match gr {
        crate::RATIO_MAGIC_GENDERLESS => 2,
        crate::RATIO_MAGIC_FEMALE => 1,
        crate::RATIO_MAGIC_MALE => 0,
        _ => {
            if (pid & 0xFF) < gr {
                1
            } else {
                0
            }
        }
    }
}

pub fn get_nature_modification(nature: usize, incr: &mut usize, decr: &mut usize) -> bool {
    *incr = (nature / 5) + 1;
    *decr = (nature % 5) + 1;
    incr == decr || nature >= 25
}

pub fn modify_stats_for_nature(stats: &mut [u16], nature: usize) {
    let mut incr = 0;
    let mut decr = 0;
    if get_nature_modification(nature, &mut incr, &mut decr) {
        return;
    }
    stats[incr] *= 11;
    stats[incr] /= 10;
    stats[decr] *= 9;
    stats[decr] /= 10;
}

pub fn get_random_pid(
    rand: &mut ThreadRng,
    species: usize,
    gender: usize,
    origin: usize,
    nature: usize,
    form: usize,
    old_pid: usize,
) -> usize {
    if origin >= 24 {
        return rand.next_u32() as usize;
    }

    let gt = LA[species].get_gender();
    let g34 = origin <= 15;
    let abil_bit_val = if g34 {
        old_pid & 0x1
    } else {
        old_pid & 0x10000
    };

    let g3_unown = origin <= 5 && species == Species::Unown as usize;
    let single_gender = is_single_gender(gt);
    loop {
        let pid = rand.next_u32() as usize;
        if g34 && pid % 25 != nature {
            continue;
        }

        if g3_unown {
            let pid_letter = get_unown_form(pid);
            if pid_letter != form {
                continue;
            }
        } else if g34 {
            if abil_bit_val != (pid & 0x1) {
                continue;
            }
        } else if abil_bit_val != (pid & 0x10000) {
            continue;
        }

        if single_gender {
            return pid;
        }

        if gender == get_gender_from_pid_and_ratio(pid, gt) {
            return pid;
        }
    }
}

pub fn get_unown_form(pid: usize) -> usize {
    let value = (pid & 0x3000000) >> 18 | (pid & 0x30000) >> 12 | (pid & 0x300) >> 6 | (pid & 0x3);
    value % 28
}

pub fn get_pkm_extensions(max_generation: usize) -> Vec<String> {
    let mut result = vec![];
    let min = if max_generation <= 2 || max_generation >= 7 {
        1
    } else {
        0
    };
    for i in min..=max_generation {
        result.push(format!("pk{}", i));
    }

    if max_generation >= 3 {
        result.push("ck3".to_string());
        result.push("xk3".to_string());
    }
    if max_generation >= 4 {
        result.push("bk4".to_string());
    }
    if max_generation >= 7 {
        result.push(EXTENSION_PB7.to_string());
    }
    if max_generation >= 8 {
        result.push(EXTENSION_PB8.to_string());
        result.push(EXTENSION_PA8.to_string());
    }
    result
}
