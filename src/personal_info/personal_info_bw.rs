use crate::{get_bits, set_bits, PersonalInfo};

pub const SIZE: usize = 0x3C;

pub struct PersonalInfoBW {
    pub(super) data: Vec<u8>,
    tmhm: Vec<bool>,
    type_tutors: Vec<bool>,
}

impl PersonalInfoBW {
    pub fn new_child(data: Vec<u8>) -> Self {
        Self {
            data,
            tmhm: vec![],
            type_tutors: vec![],
        }
    }

    fn get_ev_yield(&self) -> usize {
        u16::from_le_bytes((&self.data[0xA..0xC]).try_into().unwrap()) as usize
    }

    fn set_ev_yield(&mut self, ev_yield: usize) {
        let ev_yield_bytes = (ev_yield as u16).to_le_bytes();
        self.data.splice(0xA..0xC, ev_yield_bytes);
    }

    pub fn get_telekinesis(&self) -> bool {
        (self.get_ev_yield() >> 12 & 0x1) == 1
    }

    pub fn set_telekinesis(&mut self, telekinesis: bool) {
        self.set_ev_yield((self.get_ev_yield() & !(0x1 << 12)) | (usize::from(telekinesis) << 12));
    }

    pub fn get_item_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0xC..0xE]).try_into().unwrap()) as usize
    }

    pub fn set_item_1(&mut self, item: usize) {
        let item_bytes = (item as u16).to_le_bytes();
        self.data.splice(0xC..0xE, item_bytes);
    }

    pub fn get_item_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0xE..0x10]).try_into().unwrap()) as usize
    }

    pub fn set_item_2(&mut self, item: usize) {
        let item_bytes = (item as u16).to_le_bytes();
        self.data.splice(0xE..0x10, item_bytes);
    }

    pub fn get_item_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x10..0x12]).try_into().unwrap()) as usize
    }

    pub fn set_item_3(&mut self, item: usize) {
        let item_bytes = (item as u16).to_le_bytes();
        self.data.splice(0x10..0x12, item_bytes);
    }

    pub fn get_ability_1(&self) -> usize {
        self.data[0x18] as usize
    }

    pub fn set_ability_1(&mut self, ability: usize) {
        self.data[0x18] = ability as u8;
    }

    pub fn get_ability_2(&self) -> usize {
        self.data[0x19] as usize
    }

    pub fn set_ability_2(&mut self, ability: usize) {
        self.data[0x19] = ability as u8;
    }

    pub fn get_ability_h(&self) -> usize {
        self.data[0x1A] as usize
    }

    pub fn set_ability_h(&mut self, ability: usize) {
        self.data[0x1A] = ability as u8;
    }

    pub fn get_sprite_flip(&self) -> bool {
        ((self.data[0x21] >> 6) & 1) == 1
    }

    pub fn set_sprite_flip(&mut self, sprite_flip: bool) {
        self.data[0x21] = (self.data[0x21] & !(0x40)) | if sprite_flip { 0x40 } else { 0 };
    }

    pub fn get_sprite_form(&self) -> bool {
        ((self.data[0x21] >> 7) & 1) == 1
    }

    pub fn set_sprite_form(&mut self, sprite_form: bool) {
        self.data[0x21] = (self.data[0x21] & !(0x80)) | if sprite_form { 0x80 } else { 0 };
    }

    pub fn has_hidden_ability(&self) -> bool {
        self.get_ability_h() != self.get_ability_1()
    }
}

impl PersonalInfo for PersonalInfoBW {
    fn new(data: Vec<u8>) -> Self {
        Self {
            tmhm: get_bits(&data[0x28..0x38]),
            type_tutors: get_bits(&data[0x38..0x3C]),
            data,
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn write(&mut self) -> Vec<u8> {
        set_bits(&mut self.data[0x28..], &self.tmhm);
        set_bits(&mut self.data[0x38..], &self.type_tutors);
        self.data.clone()
    }

    fn get_hp(&self) -> usize {
        self.data[0x0] as usize
    }

    fn set_hp(&mut self, hp: usize) {
        self.data[0x0] = hp as u8;
    }

    fn get_atk(&self) -> usize {
        self.data[0x1] as usize
    }

    fn set_atk(&mut self, atk: usize) {
        self.data[0x1] = atk as u8;
    }

    fn get_def(&self) -> usize {
        self.data[0x2] as usize
    }

    fn set_def(&mut self, def: usize) {
        self.data[0x2] = def as u8;
    }

    fn get_spe(&self) -> usize {
        self.data[0x3] as usize
    }

    fn set_spe(&mut self, spe: usize) {
        self.data[0x3] = spe as u8;
    }

    fn get_spa(&self) -> usize {
        self.data[0x4] as usize
    }

    fn set_spa(&mut self, spa: usize) {
        self.data[0x4] = spa as u8;
    }

    fn get_spd(&self) -> usize {
        self.data[0x5] as usize
    }

    fn set_spd(&mut self, spd: usize) {
        self.data[0x5] = spd as u8;
    }

    fn get_ev_hp(&self) -> usize {
        self.get_ev_yield() & 0x3
    }

    fn set_ev_hp(&mut self, hp: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3)) | (hp & 0x3))
    }

    fn get_ev_atk(&self) -> usize {
        self.get_ev_yield() >> 2 & 0x3
    }

    fn set_ev_atk(&mut self, atk: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3 << 2)) | ((atk & 0x3) << 2))
    }

    fn get_ev_def(&self) -> usize {
        self.get_ev_yield() >> 4 & 0x3
    }

    fn set_ev_def(&mut self, def: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3 << 4)) | ((def & 0x3) << 4))
    }

    fn get_ev_spe(&self) -> usize {
        self.get_ev_yield() >> 6 & 0x3
    }

    fn set_ev_spe(&mut self, spe: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3 << 6)) | ((spe & 0x3) << 6))
    }

    fn get_ev_spa(&self) -> usize {
        self.get_ev_yield() >> 8 & 0x3
    }

    fn set_ev_spa(&mut self, spa: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3 << 8)) | ((spa & 0x3) << 8))
    }

    fn get_ev_spd(&self) -> usize {
        self.get_ev_yield() >> 10 & 0x3
    }

    fn set_ev_spd(&mut self, spd: usize) {
        self.set_ev_yield((self.get_ev_yield() & !(0x3 << 10)) | ((spd & 0x3) << 10))
    }

    fn get_type_1(&self) -> usize {
        self.data[0x6] as usize
    }

    fn set_type_1(&mut self, type_1: usize) {
        self.data[0x6] = type_1 as u8;
    }

    fn get_type_2(&self) -> usize {
        self.data[0x7] as usize
    }

    fn set_type_2(&mut self, type_2: usize) {
        self.data[0x7] = type_2 as u8;
    }

    fn get_egg_group_1(&self) -> usize {
        self.data[0x16] as usize
    }

    fn set_egg_group_1(&mut self, egg_group_1: usize) {
        self.data[0x16] = egg_group_1 as u8
    }

    fn get_egg_group_2(&self) -> usize {
        self.data[0x17] as usize
    }

    fn set_egg_group_2(&mut self, egg_group_2: usize) {
        self.data[0x17] = egg_group_2 as u8
    }

    fn get_catch_rate(&self) -> usize {
        self.data[0x8] as usize
    }

    fn set_catch_rate(&mut self, catch_rate: usize) {
        self.data[0x8] = catch_rate as u8;
    }

    fn get_evo_stage(&self) -> usize {
        self.data[0x9] as usize
    }

    fn set_evo_stage(&mut self, evo_stage: usize) {
        self.data[0x9] = evo_stage as u8
    }

    fn get_items(&self) -> Vec<usize> {
        vec![self.get_item_1(), self.get_item_2(), self.get_item_3()]
    }

    fn set_items(&mut self, items: Vec<usize>) {
        if items.len() == 3 {
            self.set_item_1(items[0]);
            self.set_item_2(items[1]);
            self.set_item_3(items[2]);
        }
    }

    fn get_gender(&self) -> usize {
        self.data[0x12] as usize
    }

    fn set_gender(&mut self, gender: usize) {
        self.data[0x12] = gender as u8;
    }

    fn get_hatch_cycles(&self) -> usize {
        self.data[0x13] as usize
    }

    fn set_hatch_cycles(&mut self, hatch_cycles: usize) {
        self.data[0x13] = hatch_cycles as u8;
    }

    fn get_base_friendship(&self) -> usize {
        self.data[0x14] as usize
    }

    fn set_base_friendship(&mut self, base_friendship: usize) {
        self.data[0x14] = base_friendship as u8;
    }

    fn get_exp_growth(&self) -> usize {
        self.data[0x15] as usize
    }

    fn set_exp_growth(&mut self, exp_growth: usize) {
        self.data[0x15] = exp_growth as u8;
    }

    fn get_abilities(&self) -> Vec<usize> {
        vec![
            self.get_ability_1(),
            self.get_ability_2(),
            self.get_ability_h(),
        ]
    }

    fn set_abilities(&mut self, abilities: Vec<usize>) {
        if abilities.len() == 3 {
            self.set_ability_1(abilities[0]);
            self.set_ability_2(abilities[1]);
            self.set_ability_h(abilities[2]);
        }
    }

    fn get_ability_index(&self, ability_id: usize) -> Option<usize> {
        if ability_id == self.get_ability_1() {
            Some(0)
        } else if ability_id == self.get_ability_2() {
            Some(1)
        } else if ability_id == self.get_ability_h() {
            Some(2)
        } else {
            None
        }
    }

    fn get_escape_rate(&self) -> usize {
        self.data[0x1B] as usize
    }

    fn set_escape_rate(&mut self, escape_rate: usize) {
        self.data[0x1B] = escape_rate as u8;
    }

    fn get_form_count(&self) -> usize {
        self.data[0x20] as usize
    }

    fn set_form_count(&mut self, form_count: usize) {
        self.data[0x20] = form_count as u8;
    }

    fn get_form_stats_index(&self) -> usize {
        u16::from_le_bytes((&self.data[0x1C..0x1E]).try_into().unwrap()) as usize
    }

    fn set_form_stats_index(&mut self, form_stats_index: usize) {
        let form_stats_index_bytes = (form_stats_index as u16).to_le_bytes();
        self.data.splice(0x1C..0x1E, form_stats_index_bytes);
    }

    fn get_form_sprite(&self) -> usize {
        u16::from_le_bytes((&self.data[0x1E..0x20]).try_into().unwrap()) as usize
    }

    fn set_form_sprite(&mut self, form_sprite: usize) {
        let form_sprite_bytes = (form_sprite as u16).to_le_bytes();
        self.data.splice(0x1E..0x20, form_sprite_bytes);
    }

    fn get_base_exp(&self) -> usize {
        u16::from_le_bytes((&self.data[0x22..0x24]).try_into().unwrap()) as usize
    }

    fn set_base_exp(&mut self, base_exp: usize) {
        let base_exp_bytes = (base_exp as u16).to_le_bytes();
        self.data.splice(0x22..0x24, base_exp_bytes);
    }

    fn get_color(&self) -> usize {
        (self.data[0x21] & 0x3F) as usize
    }

    fn set_color(&mut self, color: usize) {
        self.data[0x21] = (self.data[0x21] & 0xC0) | (color & 0x3F) as u8;
    }

    fn get_height(&self) -> usize {
        u16::from_le_bytes((&self.data[0x24..0x26]).try_into().unwrap()) as usize
    }

    fn set_height(&mut self, height: usize) {
        let height_bytes = (height as u16).to_le_bytes();
        self.data.splice(0x24..0x26, height_bytes);
    }

    fn get_weight(&self) -> usize {
        u16::from_le_bytes((&self.data[0x26..0x28]).try_into().unwrap()) as usize
    }

    fn set_weight(&mut self, weight: usize) {
        let weight_bytes = (weight as u16).to_le_bytes();
        self.data.splice(0x26..0x28, weight_bytes);
    }

    fn set_tmhm(&mut self, tmhm: Vec<bool>) {
        self.tmhm = tmhm;
    }

    fn set_type_tutors(&mut self, type_tutors: Vec<bool>) {
        self.type_tutors = type_tutors;
    }

    fn set_special_tutors(&mut self, _special_tutors: Vec<Vec<bool>>) {}
}
