use crate::game::enums::Species;
use crate::personal_info;
use crate::personal_info::traits::gender_detail;
use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::pkm::util::entity_gender;
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn get_random_pid(
    rnd: &mut ThreadRng,
    species: u16,
    gender: u8,
    origin: u8,
    nature: u8,
    form: u8,
    old_pid: u32,
) -> u32 {
    if origin >= 24 {
        return rnd.gen();
    }

    let gt = personal_info::SV[species as usize].gender();
    let g34 = origin <= 15;
    let abil_bit_val = if g34 {
        old_pid & 0x1
    } else {
        old_pid & 0x10000
    };
    let g3_unown = origin <= 5 && species == Species::Unown as u16;
    let single_gender = gender_detail::is_single_gender(gt);
    loop {
        let pid: u32 = rnd.gen();
        if g34 && ((pid % 25) as u8) != nature {
            continue;
        }

        if g3_unown {
            let pid_letter = get_unown_form_3(pid);
            if pid_letter != form {
                continue;
            }
        } else if g34 {
            if abil_bit_val != (pid & 1) {
                continue;
            }
        } else if abil_bit_val != (pid & 0x10000) {
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
