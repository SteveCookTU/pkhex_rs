use crate::{get_bits, set_bits, PersonalInfo};

pub const SIZE: usize = 0x20;

pub struct PersonalInfoG2 {
    pub(super) data: Vec<u8>,
    tmhm: Vec<bool>,
}

impl PersonalInfoG2 {
    pub fn get_dex_id(&self) -> usize {
        self.data[0x0] as usize
    }

    pub fn set_dex_id(&mut self, id: usize) {
        self.data[0x0] = id as u8;
    }

    pub fn get_item_1(&self) -> usize {
        self.data[0xB] as usize
    }

    pub fn set_item_1(&mut self, item: usize) {
        self.data[0xB] = item as u8;
    }

    pub fn get_item_2(&self) -> usize {
        self.data[0xC] as usize
    }

    pub fn set_item_2(&mut self, item: usize) {
        self.data[0xC] = item as u8;
    }
}

impl PersonalInfo for PersonalInfoG2 {
    fn new(data: Vec<u8>) -> Self {
        Self {
            tmhm: get_bits(&data[0x18..0x20]),
            data,
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn write(&mut self) -> Vec<u8> {
        set_bits(&mut self.data[0x18..], &self.tmhm);
        self.data.clone()
    }

    fn get_hp(&self) -> usize {
        self.data[0x1] as usize
    }

    fn set_hp(&mut self, hp: usize) {
        self.data[0x1] = hp as u8;
    }

    fn get_atk(&self) -> usize {
        self.data[0x2] as usize
    }

    fn set_atk(&mut self, atk: usize) {
        self.data[0x2] = atk as u8;
    }

    fn get_def(&self) -> usize {
        self.data[0x3] as usize
    }

    fn set_def(&mut self, def: usize) {
        self.data[0x3] = def as u8;
    }

    fn get_spe(&self) -> usize {
        self.data[0x4] as usize
    }

    fn set_spe(&mut self, spe: usize) {
        self.data[0x4] = spe as u8;
    }

    fn get_spa(&self) -> usize {
        self.data[0x5] as usize
    }

    fn set_spa(&mut self, spa: usize) {
        self.data[0x5] = spa as u8;
    }

    fn get_spd(&self) -> usize {
        self.data[0x6] as usize
    }

    fn set_spd(&mut self, spd: usize) {
        self.data[0x6] = spd as u8;
    }

    fn get_ev_hp(&self) -> usize {
        self.get_hp()
    }

    fn set_ev_hp(&mut self, _: usize) {}

    fn get_ev_atk(&self) -> usize {
        self.get_atk()
    }

    fn set_ev_atk(&mut self, _: usize) {}

    fn get_ev_def(&self) -> usize {
        self.get_def()
    }

    fn set_ev_def(&mut self, _: usize) {}

    fn get_ev_spe(&self) -> usize {
        self.get_spe()
    }

    fn set_ev_spe(&mut self, _: usize) {}

    fn get_ev_spa(&self) -> usize {
        self.get_spa()
    }

    fn set_ev_spa(&mut self, _: usize) {}

    fn get_ev_spd(&self) -> usize {
        self.get_spd()
    }

    fn set_ev_spd(&mut self, _: usize) {}

    fn get_type_1(&self) -> usize {
        self.data[0x7] as usize
    }

    fn set_type_1(&mut self, type_1: usize) {
        self.data[0x7] = type_1 as u8;
    }

    fn get_type_2(&self) -> usize {
        self.data[0x8] as usize
    }

    fn set_type_2(&mut self, type_2: usize) {
        self.data[0x8] = type_2 as u8;
    }

    fn get_egg_group_1(&self) -> usize {
        (self.data[0x17] & 0xF) as usize
    }

    fn set_egg_group_1(&mut self, egg_group_1: usize) {
        self.data[0x17] = (self.data[0x17] & 0xF0) | egg_group_1 as u8
    }

    fn get_egg_group_2(&self) -> usize {
        (self.data[0x17] >> 4) as usize
    }

    fn set_egg_group_2(&mut self, egg_group_2: usize) {
        self.data[0x17] = (self.data[0x17] & 0x0F) | (egg_group_2 << 4) as u8
    }

    fn get_catch_rate(&self) -> usize {
        self.data[0x9] as usize
    }

    fn set_catch_rate(&mut self, catch_rate: usize) {
        self.data[0x9] = catch_rate as u8;
    }

    fn get_items(&self) -> Vec<usize> {
        vec![self.get_item_1(), self.get_item_2()]
    }

    fn set_items(&mut self, items: Vec<usize>) {
        if items.len() == 2 {
            self.set_item_1(items[0]);
            self.set_item_2(items[1]);
        }
    }

    fn get_gender(&self) -> usize {
        self.data[0xD] as usize
    }

    fn set_gender(&mut self, gender: usize) {
        self.data[0xD] = gender as u8;
    }

    fn get_hatch_cycles(&self) -> usize {
        self.data[0xF] as usize
    }

    fn set_hatch_cycles(&mut self, hatch_cycles: usize) {
        self.data[0xF] = hatch_cycles as u8;
    }

    fn get_base_friendship(&self) -> usize {
        70
    }

    fn set_base_friendship(&mut self, _: usize) {}

    fn get_exp_growth(&self) -> usize {
        self.data[0x16] as usize
    }

    fn set_exp_growth(&mut self, exp_growth: usize) {
        self.data[0x16] = exp_growth as u8;
    }

    fn get_abilities(&self) -> Vec<usize> {
        vec![]
    }

    fn set_abilities(&mut self, _: Vec<usize>) {}

    fn get_ability_index(&self, _: usize) -> Option<usize> {
        None
    }

    fn get_escape_rate(&self) -> usize {
        0
    }

    fn set_escape_rate(&mut self, _: usize) {}

    fn get_base_exp(&self) -> usize {
        self.data[0xA] as usize
    }

    fn set_base_exp(&mut self, base_exp: usize) {
        self.data[0xA] = base_exp as u8;
    }

    fn get_color(&self) -> usize {
        0
    }

    fn set_color(&mut self, _: usize) {}

    fn get_tmhm(&self) -> Vec<bool> {
        self.tmhm.clone()
    }

    fn set_tmhm(&mut self, tmhm: Vec<bool>) {
        self.tmhm = tmhm
    }

    fn set_type_tutors(&mut self, _: Vec<bool>) {}

    fn set_special_tutors(&mut self, _: Vec<Vec<bool>>) {}
}
