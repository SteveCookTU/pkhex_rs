use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::{get_bits, PersonalInfo};
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalAbility12, PersonalAbility12H,
    PersonalEgg, PersonalEncounter, PersonalFormInfo, PersonalMisc, PersonalType,
};
use no_std_io::Reader;

pub struct PersonalInfo6AO<'a> {
    data: &'a [u8],
    tmhm: Vec<bool>,
    type_tutors: Vec<bool>,
    special_tutors: Vec<Vec<bool>>,
}

impl<'a> PersonalInfo6AO<'a> {
    pub const SIZE: usize = 0x50;

    pub fn new(data: &'a [u8]) -> PersonalInfo6AO<'a> {
        Self {
            data,
            tmhm: get_bits(&data[0x28..0x38]),
            type_tutors: get_bits(&data[0x38..0x3C]),
            special_tutors: vec![
                get_bits(&data[0x40..0x44]),
                get_bits(&data[0x44..0x48]),
                get_bits(&data[0x48..0x4C]),
                get_bits(&data[0x4C..0x50]),
            ],
        }
    }

    fn ev_yield(&self) -> u16 {
        self.data.default_read_le(0xA)
    }

    pub fn telekinesis(&self) -> bool {
        ((self.ev_yield() >> 12) & 1) == 1
    }

    pub fn item_1(&self) -> u16 {
        self.data.default_read_le(0xC)
    }

    pub fn item_2(&self) -> u16 {
        self.data.default_read_le(0xE)
    }

    pub fn item_3(&self) -> u16 {
        self.data.default_read_le(0x10)
    }

    pub fn form_sprite(&self) -> u16 {
        self.data.default_read_le(0x1E)
    }

    pub fn sprite_flip(&self) -> bool {
        ((self.data.default_read_le::<u8>(0x21) >> 6) & 1) == 1
    }

    pub fn sprite_form(&self) -> bool {
        ((self.data.default_read_le::<u8>(0x21) >> 7) & 1) == 1
    }
}

impl BaseStat for PersonalInfo6AO<'_> {
    fn hp(&self) -> u8 {
        self.data.default_read_le(0)
    }

    fn atk(&self) -> u8 {
        self.data.default_read_le(1)
    }

    fn def(&self) -> u8 {
        self.data.default_read_le(2)
    }

    fn spa(&self) -> u8 {
        self.data.default_read_le(4)
    }

    fn spd(&self) -> u8 {
        self.data.default_read_le(5)
    }

    fn spe(&self) -> u8 {
        self.data.default_read_le(3)
    }
}

impl EffortValueYield for PersonalInfo6AO<'_> {
    fn ev_hp(&self) -> u8 {
        (self.ev_yield() & 0x3) as u8
    }

    fn ev_atk(&self) -> u8 {
        ((self.ev_yield() >> 2) & 0x3) as u8
    }

    fn ev_def(&self) -> u8 {
        ((self.ev_yield() >> 4) & 0x3) as u8
    }

    fn ev_spe(&self) -> u8 {
        ((self.ev_yield() >> 6) & 0x3) as u8
    }

    fn ev_spa(&self) -> u8 {
        ((self.ev_yield() >> 8) & 0x3) as u8
    }

    fn ev_spd(&self) -> u8 {
        ((self.ev_yield() >> 10) & 0x3) as u8
    }
}

impl GenderDetail for PersonalInfo6AO<'_> {
    fn gender(&self) -> u8 {
        self.data.default_read_le(0x12)
    }
}

impl PersonalFormInfo for PersonalInfo6AO<'_> {
    fn form_count(&self) -> u8 {
        self.data.default_read_le(0x20)
    }

    fn form_stats_index(&self) -> Option<usize> {
        Some(self.data.default_read_le::<u16>(0x1C) as usize)
    }
}

impl PersonalAbility for PersonalInfo6AO<'_> {
    fn get_index_of_ability(&self, ability_id: u16) -> Option<usize> {
        if ability_id == self.ability_1() {
            Some(0)
        } else if ability_id == self.ability_2() {
            Some(1)
        } else if ability_id == self.ability_h() {
            Some(2)
        } else {
            None
        }
    }

    fn get_ability_at_index(&self, ability_index: usize) -> Option<u16> {
        match ability_index {
            0 => Some(self.ability_1()),
            1 => Some(self.ability_2()),
            2 => Some(self.ability_h()),
            _ => None,
        }
    }

    fn get_ability_count(&self) -> u8 {
        3
    }
}

impl PersonalAbility12 for PersonalInfo6AO<'_> {
    fn ability_1(&self) -> u16 {
        self.data.default_read_le::<u8>(0x18) as u16
    }

    fn ability_2(&self) -> u16 {
        self.data.default_read_le::<u8>(0x19) as u16
    }
}

impl PersonalAbility12H for PersonalInfo6AO<'_> {
    fn ability_h(&self) -> u16 {
        self.data.default_read_le::<u8>(0x1A) as u16
    }
}

impl PersonalEgg for PersonalInfo6AO<'_> {
    fn egg_group_1(&self) -> u8 {
        self.data.default_read_le(0x16)
    }

    fn egg_group_2(&self) -> u8 {
        self.data.default_read_le(0x17)
    }
}

impl PersonalEncounter for PersonalInfo6AO<'_> {
    fn base_exp(&self) -> u16 {
        self.data.default_read_le(0x22)
    }

    fn hatch_cycles(&self) -> u8 {
        self.data.default_read_le(0x13)
    }

    fn catch_rate(&self) -> u8 {
        self.data.default_read_le(8)
    }

    fn base_friendship(&self) -> u8 {
        self.data.default_read_le(0x14)
    }

    fn escape_rate(&self) -> u8 {
        self.data.default_read_le(0x1B)
    }
}

impl PersonalType for PersonalInfo6AO<'_> {
    fn type_1(&self) -> u8 {
        self.data.default_read_le(6)
    }

    fn type_2(&self) -> u8 {
        self.data.default_read_le(7)
    }
}

impl PersonalMisc for PersonalInfo6AO<'_> {
    fn evo_stage(&self) -> u8 {
        self.data.default_read_le(9)
    }

    fn color(&self) -> u8 {
        self.data.default_read_le::<u8>(0x21) & 0x3F
    }

    fn height(&self) -> u16 {
        self.data.default_read_le(0x24)
    }

    fn weight(&self) -> u16 {
        self.data.default_read_le(0x26)
    }
}

impl PersonalInfo for PersonalInfo6AO<'_> {
    fn exp_growth(&self) -> u8 {
        self.data.default_read_le(0x15)
    }

    fn tmhm(&self) -> &[bool] {
        &self.tmhm
    }

    fn set_tmhm(&mut self, bits: Vec<bool>) {
        self.tmhm = bits;
    }

    fn type_tutors(&self) -> &[bool] {
        &self.type_tutors
    }

    fn set_type_tutors(&mut self, bits: Vec<bool>) {
        self.type_tutors = bits;
    }

    fn special_tutors(&self) -> &[Vec<bool>] {
        &self.special_tutors
    }
}
