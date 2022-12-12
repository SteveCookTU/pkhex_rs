use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::{get_bits, PersonalInfo};
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalEgg, PersonalEncounter, PersonalFormInfo,
    PersonalMisc, PersonalType,
};
use no_std_io::Reader;

pub struct PersonalInfo2<'a> {
    data: &'a [u8],
    tmhm: Vec<bool>,
}

impl<'a> PersonalInfo2<'a> {
    pub const SIZE: usize = 0x20;

    pub fn new(data: &'a [u8]) -> PersonalInfo2<'a> {
        Self {
            data,
            tmhm: get_bits(&data[0x18..0x20]),
        }
    }

    pub fn dex_id(&self) -> u8 {
        self.data.default_read_le(0)
    }

    pub fn item_1(&self) -> u8 {
        self.data.default_read_le(0xB)
    }

    pub fn item_2(&self) -> u8 {
        self.data.default_read_le(0xC)
    }

    pub fn items(&self) -> [u8; 2] {
        [self.item_1(), self.item_2()]
    }
}

impl BaseStat for PersonalInfo2<'_> {
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
        self.data.default_read_le(5)
    }

    fn spd(&self) -> u8 {
        self.data.default_read_le(6)
    }

    fn spe(&self) -> u8 {
        self.data.default_read_le(4)
    }
}

impl EffortValueYield for PersonalInfo2<'_> {
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

impl GenderDetail for PersonalInfo2<'_> {
    fn gender(&self) -> u8 {
        self.data.default_read_le(0xD)
    }
}

impl PersonalFormInfo for PersonalInfo2<'_> {
    fn form_count(&self) -> u8 {
        1
    }

    fn form_stats_index(&self) -> Option<usize> {
        None
    }
}

impl PersonalAbility for PersonalInfo2<'_> {
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

impl PersonalEgg for PersonalInfo2<'_> {
    fn egg_group_1(&self) -> u8 {
        self.data.default_read_le::<u8>(0x17) & 0xF
    }

    fn egg_group_2(&self) -> u8 {
        self.data.default_read_le::<u8>(0x17) >> 4
    }
}

impl PersonalEncounter for PersonalInfo2<'_> {
    fn base_exp(&self) -> u16 {
        self.data.default_read_le::<u8>(0xA) as u16
    }

    fn hatch_cycles(&self) -> u8 {
        self.data.default_read_le(0xF)
    }

    fn catch_rate(&self) -> u8 {
        self.data.default_read_le(0x9)
    }

    fn base_friendship(&self) -> u8 {
        0
    }

    fn escape_rate(&self) -> u8 {
        0
    }
}

impl PersonalType for PersonalInfo2<'_> {
    fn type_1(&self) -> u8 {
        self.data.default_read_le(0x7)
    }

    fn type_2(&self) -> u8 {
        self.data.default_read_le(0x8)
    }
}

impl PersonalMisc for PersonalInfo2<'_> {
    fn evo_stage(&self) -> u8 {
        0
    }

    fn color(&self) -> u8 {
        0
    }

    fn height(&self) -> u16 {
        0
    }

    fn weight(&self) -> u16 {
        0
    }
}

impl PersonalInfo for PersonalInfo2<'_> {
    fn exp_growth(&self) -> u8 {
        self.data.default_read_le(0x16)
    }

    fn tmhm(&self) -> &[bool] {
        &self.tmhm
    }

    fn set_tmhm(&mut self, bits: Vec<bool>) {
        self.tmhm = bits
    }
}
