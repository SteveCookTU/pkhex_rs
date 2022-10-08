use crate::{entity_gender, personal_info, personal_table, PersonalInfo, Species};
use rand::{Rng, RngCore};

pub fn get_random_pid(
    rnd: &mut impl RngCore,
    species: u16,
    gender: u8,
    origin: u8,
    nature: u8,
    form: u8,
    old_pid: u32,
) -> u32 {
    if origin >= 24 {
        return rnd.gen::<u32>();
    }

    let gt = personal_table::LA[species as usize].get_gender();
    let g34 = origin <= 15;
    let abil_bit_val = if g34 {
        old_pid & 0x1
    } else {
        old_pid & 0x10000
    };

    let g3_unown = origin <= 5 && species == Species::Unown as u16;
    let single_gender = personal_info::is_single_gender(gt);

    loop {
        let pid = rnd.gen::<u32>();

        if g34 && pid % 25 != nature as u32 {
            continue;
        }

        if g3_unown {
            let pid_letter = get_unown_form_3(pid);
            if pid_letter != form {
                continue;
            }
        } else if g34 && abil_bit_val != (pid & 0x1) || (!g34 && abil_bit_val != (pid & 0x10000)) {
            continue;
        }

        if single_gender {
            return pid;
        }

        if gender == entity_gender::get_from_pid_and_ratio(pid, gt) {
            return pid;
        }
    }
}

pub fn get_unown_form_3(pid: u32) -> u8 {
    let value =
        ((pid & 0x3000000) >> 18) | ((pid & 0x30000) >> 12) | ((pid & 0x300) >> 6) | (pid & 0x3);
    (value % 28) as u8
}
