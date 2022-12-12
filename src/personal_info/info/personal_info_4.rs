use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::{get_bits, PersonalInfo};
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalAbility12, PersonalEgg, PersonalEncounter,
    PersonalFormInfo, PersonalMisc, PersonalType,
};
use no_std_io::Reader;

pub struct PersonalInfo4<'a> {
    data: &'a [u8],
    tmhm: Vec<bool>,
    type_tutors: Vec<bool>,
}

impl<'a> PersonalInfo4<'a> {
    pub const SIZE: usize = 0x2C;

    pub fn new(data: &'a [u8]) -> PersonalInfo4<'a> {
        Self {
            data,
            tmhm: get_bits(&data[0x1C..0x29]),
            type_tutors: vec![],
        }
    }

    fn ev_yield(&self) -> u16 {
        self.data.default_read_le(0xA)
    }

    pub fn item_1(&self) -> u16 {
        self.data.default_read_le(0xC)
    }

    pub fn item_2(&self) -> u16 {
        self.data.default_read_le(0xE)
    }

    pub fn no_flip(&self) -> bool {
        (self.data.default_read_le::<u8>(0x14) >> 7) == 1
    }

    pub fn get_ability(&self, second: bool) -> u16 {
        if second && !self.get_is_ability_12_same() {
            self.ability_2()
        } else {
            self.ability_1()
        }
    }
}

impl BaseStat for PersonalInfo4<'_> {
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

impl EffortValueYield for PersonalInfo4<'_> {
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

impl GenderDetail for PersonalInfo4<'_> {
    fn gender(&self) -> u8 {
        self.data.default_read_le(0x10)
    }
}

impl PersonalFormInfo for PersonalInfo4<'_> {
    fn form_count(&self) -> u8 {
        self.data.default_read_le(0x29)
    }

    fn form_stats_index(&self) -> Option<usize> {
        Some(self.data.default_read_le::<u16>(0x2A) as usize)
    }
}

impl PersonalAbility for PersonalInfo4<'_> {
    fn get_index_of_ability(&self, ability_id: u16) -> Option<usize> {
        if ability_id == self.ability_1() {
            Some(0)
        } else if ability_id == self.ability_2() {
            Some(1)
        } else {
            None
        }
    }

    fn get_ability_at_index(&self, ability_index: usize) -> Option<u16> {
        match ability_index {
            0 => Some(self.ability_1()),
            1 => Some(self.ability_2()),
            _ => None,
        }
    }

    fn get_ability_count(&self) -> u8 {
        2
    }
}

impl PersonalAbility12 for PersonalInfo4<'_> {
    fn ability_1(&self) -> u16 {
        self.data.default_read_le::<u8>(0x16) as u16
    }

    fn ability_2(&self) -> u16 {
        self.data.default_read_le::<u8>(0x17) as u16
    }
}

impl PersonalEgg for PersonalInfo4<'_> {
    fn egg_group_1(&self) -> u8 {
        self.data.default_read_le(0x14)
    }

    fn egg_group_2(&self) -> u8 {
        self.data.default_read_le(0x15)
    }
}

impl PersonalEncounter for PersonalInfo4<'_> {
    fn base_exp(&self) -> u16 {
        self.data.default_read_le::<u8>(9) as u16
    }

    fn hatch_cycles(&self) -> u8 {
        self.data.default_read_le(0x11)
    }

    fn catch_rate(&self) -> u8 {
        self.data.default_read_le(8)
    }

    fn base_friendship(&self) -> u8 {
        self.data.default_read_le(0x12)
    }

    fn escape_rate(&self) -> u8 {
        self.data.default_read_le(0x18)
    }
}

impl PersonalType for PersonalInfo4<'_> {
    fn type_1(&self) -> u8 {
        self.data.default_read_le(6)
    }

    fn type_2(&self) -> u8 {
        self.data.default_read_le(7)
    }
}

impl PersonalMisc for PersonalInfo4<'_> {
    fn evo_stage(&self) -> u8 {
        0
    }

    fn color(&self) -> u8 {
        self.data.default_read_le::<u8>(0x19) & 0x7F
    }

    fn height(&self) -> u16 {
        0
    }

    fn weight(&self) -> u16 {
        0
    }
}

impl PersonalInfo for PersonalInfo4<'_> {
    fn exp_growth(&self) -> u8 {
        self.data.default_read_le(0x13)
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
}
