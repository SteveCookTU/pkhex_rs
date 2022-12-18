use crate::flag_util;
use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::PersonalInfo;
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalAbility12, PersonalAbility12H,
    PersonalEgg, PersonalEncounter, PersonalFormInfo, PersonalMisc, PersonalType,
};
use no_std_io::Reader;

pub struct PersonalInfo9SV<'a> {
    data: &'a [u8],
    tmhm: Vec<bool>,
}

impl<'a> PersonalInfo9SV<'a> {
    pub const SIZE: usize = 0x44;
    pub const COUNT_TM: usize = 172;

    pub fn new(data: &'a [u8]) -> PersonalInfo9SV<'a> {
        let mut tmhm = vec![false; Self::COUNT_TM];
        let tm = &data[0x2C..];
        tmhm.iter_mut()
            .enumerate()
            .for_each(|(i, t)| *t = flag_util::get_flag(tm, i >> 3, i));

        Self { data, tmhm }
    }

    fn ev_yield(&self) -> u16 {
        self.data.default_read_le(0xA)
    }

    pub fn is_present_in_game(&self) -> bool {
        self.data.default_read_le::<u8>(0x1C) != 0
    }

    pub fn dex_group(&self) -> u8 {
        self.data.default_read_le(0x1D)
    }

    pub fn dex_index(&self) -> u16 {
        self.data.default_read_le(0x1E)
    }

    pub fn hatch_species(&self) -> u16 {
        self.data.default_read_le(0x24)
    }

    pub fn local_form_index(&self) -> u8 {
        self.data.default_read_le::<u16>(0x26) as u8
    }

    pub fn regional_flags(&self) -> u16 {
        self.data.default_read_le(0x28)
    }

    pub fn is_regional_form(&self) -> bool {
        (self.regional_flags() & 1) == 1
    }

    pub fn regional_form_index(&self) -> u8 {
        self.data.default_read_le::<u16>(0x2A) as u8
    }

    pub fn hatch_form_index_everstone(&self) -> u8 {
        if self.is_regional_form() {
            self.regional_form_index()
        } else {
            self.local_form_index()
        }
    }

    pub fn is_in_dex(&self) -> bool {
        self.dex_index() != 0
    }
}

impl BaseStat for PersonalInfo9SV<'_> {
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

impl EffortValueYield for PersonalInfo9SV<'_> {
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

impl GenderDetail for PersonalInfo9SV<'_> {
    fn gender(&self) -> u8 {
        self.data.default_read_le(0xC)
    }
}

impl PersonalFormInfo for PersonalInfo9SV<'_> {
    fn form_count(&self) -> u8 {
        self.data.default_read_le(0x1A)
    }

    fn form_stats_index(&self) -> Option<usize> {
        Some(self.data.default_read_le::<u16>(0x18) as usize)
    }
}

impl PersonalAbility for PersonalInfo9SV<'_> {
    fn get_index_of_ability(&self, ability_id: u16) -> Option<u8> {
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

impl PersonalAbility12 for PersonalInfo9SV<'_> {
    fn ability_1(&self) -> u16 {
        self.data.default_read_le(0x12)
    }

    fn ability_2(&self) -> u16 {
        self.data.default_read_le(0x14)
    }
}

impl PersonalAbility12H for PersonalInfo9SV<'_> {
    fn ability_h(&self) -> u16 {
        self.data.default_read_le(0x16)
    }
}

impl PersonalEgg for PersonalInfo9SV<'_> {
    fn egg_group_1(&self) -> u8 {
        self.data.default_read_le(0x10)
    }

    fn egg_group_2(&self) -> u8 {
        self.data.default_read_le(0x11)
    }
}

impl PersonalEncounter for PersonalInfo9SV<'_> {
    fn base_exp(&self) -> u16 {
        0
    }

    fn hatch_cycles(&self) -> u8 {
        self.data.default_read_le(0xD)
    }

    fn catch_rate(&self) -> u8 {
        self.data.default_read_le(8)
    }

    fn base_friendship(&self) -> u8 {
        self.data.default_read_le(0xE)
    }

    fn escape_rate(&self) -> u8 {
        0
    }
}

impl PersonalType for PersonalInfo9SV<'_> {
    fn type_1(&self) -> u8 {
        self.data.default_read_le(6)
    }

    fn type_2(&self) -> u8 {
        self.data.default_read_le(7)
    }
}

impl PersonalMisc for PersonalInfo9SV<'_> {
    fn evo_stage(&self) -> u8 {
        self.data.default_read_le(9)
    }

    fn color(&self) -> u8 {
        self.data.default_read_le(0x1B)
    }

    fn height(&self) -> u16 {
        self.data.default_read_le(0x20)
    }

    fn weight(&self) -> u16 {
        self.data.default_read_le(0x22)
    }
}

impl PersonalInfo for PersonalInfo9SV<'_> {
    fn exp_growth(&self) -> u8 {
        self.data.default_read_le(0xF)
    }

    fn tmhm(&self) -> &[bool] {
        &self.tmhm
    }

    fn set_tmhm(&mut self, bits: Vec<bool>) {
        self.tmhm = bits;
    }
}
