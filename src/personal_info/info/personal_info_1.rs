use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::{get_bits, PersonalInfo};
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalEgg, PersonalEncounter, PersonalFormInfo,
    PersonalMisc, PersonalType,
};
use no_std_io::Reader;

pub struct PersonalInfo1<'a> {
    data: &'a [u8],
    tmhm: Vec<bool>,
}

impl<'a> PersonalInfo1<'a> {
    pub const SIZE: usize = 0x1C;

    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            tmhm: get_bits(&data[0x14..0x1C]),
        }
    }

    pub fn spc(&self) -> u8 {
        self.data.default_read_le(5)
    }

    pub fn move1(&self) -> u8 {
        self.data.default_read_le(0xF)
    }

    pub fn move2(&self) -> u8 {
        self.data.default_read_le(0x10)
    }

    pub fn move3(&self) -> u8 {
        self.data.default_read_le(0x11)
    }

    pub fn move4(&self) -> u8 {
        self.data.default_read_le(0x12)
    }
}

impl BaseStat for PersonalInfo1<'_> {
    fn hp(&self) -> u8 {
        self.data.default_read_le(1)
    }

    fn atk(&self) -> u8 {
        self.data.default_read_le(2)
    }

    fn def(&self) -> u8 {
        self.data.default_read_le(3)
    }

    fn spa(&self) -> u8 {
        self.spc()
    }

    fn spd(&self) -> u8 {
        self.spc()
    }

    fn spe(&self) -> u8 {
        self.data.default_read_le(4)
    }
}

impl EffortValueYield for PersonalInfo1<'_> {
    fn ev_hp(&self) -> u8 {
        self.hp()
    }

    fn ev_atk(&self) -> u8 {
        self.atk()
    }

    fn ev_def(&self) -> u8 {
        self.def()
    }

    fn ev_spe(&self) -> u8 {
        self.spe()
    }

    fn ev_spa(&self) -> u8 {
        self.spa()
    }

    fn ev_spd(&self) -> u8 {
        self.spd()
    }
}

impl GenderDetail for PersonalInfo1<'_> {
    fn gender(&self) -> u8 {
        self.data.default_read_le(0)
    }
}

impl PersonalFormInfo for PersonalInfo1<'_> {
    fn form_count(&self) -> u8 {
        1
    }

    fn form_stats_index(&self) -> Option<usize> {
        None
    }
}

impl PersonalAbility for PersonalInfo1<'_> {
    fn get_index_of_ability(&self, _ability_id: u16) -> Option<usize> {
        None
    }

    fn get_ability_at_index(&self, _ability_index: usize) -> Option<u16> {
        None
    }

    fn get_ability_count(&self) -> u8 {
        0
    }
}

impl PersonalEgg for PersonalInfo1<'_> {
    fn egg_group_1(&self) -> u8 {
        0
    }

    fn egg_group_2(&self) -> u8 {
        0
    }
}

impl PersonalEncounter for PersonalInfo1<'_> {
    fn base_exp(&self) -> u8 {
        self.data.default_read_le(9)
    }

    fn hatch_cycles(&self) -> u8 {
        0
    }

    fn catch_rate(&self) -> u8 {
        self.data.default_read_le(8)
    }

    fn base_friendship(&self) -> u8 {
        0
    }

    fn escape_rate(&self) -> u8 {
        0
    }
}

impl PersonalType for PersonalInfo1<'_> {
    fn type_1(&self) -> u8 {
        self.data.default_read_le(6)
    }

    fn type_2(&self) -> u8 {
        self.data.default_read_le(7)
    }
}

impl PersonalMisc for PersonalInfo1<'_> {
    fn evo_stage(&self) -> u8 {
        0
    }

    fn color(&self) -> u8 {
        0
    }

    fn height(&self) -> u8 {
        0
    }

    fn weight(&self) -> u8 {
        0
    }
}

impl PersonalInfo for PersonalInfo1<'_> {
    fn exp_growth(&self) -> u8 {
        self.data.default_read_le(0x13)
    }

    fn tmhm(&self) -> &[bool] {
        &self.tmhm
    }

    fn add_tmhm(&mut self, data: &[u8]) {
        self.tmhm = get_bits(data);
    }
}
