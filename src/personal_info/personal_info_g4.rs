use crate::personal_info_g3::PersonalInfoG3;
use crate::{get_bits, set_bits, PersonalInfo};

pub const SIZE: usize = 0x2C;

pub struct PersonalInfoG4 {
    tmhm: Vec<bool>,
    child: PersonalInfoG3,
}

impl PersonalInfoG4 {
    pub fn get_item_1(&self) -> usize {
        self.child.get_item_1()
    }

    pub fn set_item_1(&mut self, item: usize) {
        self.child.set_item_1(item);
    }

    pub fn get_item_2(&self) -> usize {
        self.child.get_item_2()
    }

    pub fn set_item_2(&mut self, item: usize) {
        self.child.set_item_2(item);
    }

    pub fn get_ability_1(&self) -> usize {
        self.child.get_ability_1()
    }

    pub fn set_ability_1(&mut self, ability: usize) {
        self.child.set_ability_1(ability);
    }

    pub fn get_ability_2(&self) -> usize {
        self.child.get_ability_2()
    }

    pub fn set_ability_2(&mut self, ability: usize) {
        self.child.set_ability_2(ability);
    }

    pub fn get_no_flip(&self) -> bool {
        self.child.get_no_flip()
    }

    pub fn set_no_flip(&mut self, no_flip: bool) {
        self.child.set_no_flip(no_flip);
    }

    pub fn get_ability(&self, second: bool) -> usize {
        self.child.get_ability(second)
    }

    pub fn has_second_ability(&self) -> bool {
        self.child.has_second_ability()
    }
}

impl PersonalInfo for PersonalInfoG4 {
    fn new(data: Vec<u8>) -> Self {
        Self {
            tmhm: get_bits(&data[0x1C..0x29]),
            child: PersonalInfoG3::new(data),
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        self.child.get_data()
    }

    fn write(&mut self) -> Vec<u8> {
        set_bits(&mut self.child.data[0x1C..], &self.tmhm);
        self.child.data.clone()
    }

    fn get_hp(&self) -> usize {
        self.child.get_hp()
    }

    fn set_hp(&mut self, hp: usize) {
        self.child.set_hp(hp);
    }

    fn get_atk(&self) -> usize {
        self.child.get_atk()
    }

    fn set_atk(&mut self, atk: usize) {
        self.child.set_atk(atk);
    }

    fn get_def(&self) -> usize {
        self.child.get_def()
    }

    fn set_def(&mut self, def: usize) {
        self.child.set_def(def);
    }

    fn get_spe(&self) -> usize {
        self.child.get_spe()
    }

    fn set_spe(&mut self, spe: usize) {
        self.child.set_spe(spe);
    }

    fn get_spa(&self) -> usize {
        self.child.get_spa()
    }

    fn set_spa(&mut self, spa: usize) {
        self.child.set_spa(spa);
    }

    fn get_spd(&self) -> usize {
        self.child.get_spd()
    }

    fn set_spd(&mut self, spd: usize) {
        self.child.set_spd(spd);
    }

    fn get_ev_hp(&self) -> usize {
        self.child.get_ev_hp()
    }

    fn set_ev_hp(&mut self, hp: usize) {
        self.child.set_ev_hp(hp);
    }

    fn get_ev_atk(&self) -> usize {
        self.child.get_ev_atk()
    }

    fn set_ev_atk(&mut self, atk: usize) {
        self.child.set_ev_atk(atk);
    }

    fn get_ev_def(&self) -> usize {
        self.child.get_ev_def()
    }

    fn set_ev_def(&mut self, def: usize) {
        self.child.set_ev_def(def);
    }

    fn get_ev_spe(&self) -> usize {
        self.child.get_ev_spe()
    }

    fn set_ev_spe(&mut self, spe: usize) {
        self.child.set_ev_spe(spe);
    }

    fn get_ev_spa(&self) -> usize {
        self.child.get_ev_spa()
    }

    fn set_ev_spa(&mut self, spa: usize) {
        self.child.set_ev_spa(spa);
    }

    fn get_ev_spd(&self) -> usize {
        self.child.get_ev_spd()
    }

    fn set_ev_spd(&mut self, spd: usize) {
        self.child.set_ev_spd(spd)
    }

    fn get_type_1(&self) -> usize {
        self.child.get_type_1()
    }

    fn set_type_1(&mut self, type_1: usize) {
        self.child.set_type_1(type_1);
    }

    fn get_type_2(&self) -> usize {
        self.child.get_type_2()
    }

    fn set_type_2(&mut self, type_2: usize) {
        self.child.set_type_2(type_2);
    }

    fn get_egg_group_1(&self) -> usize {
        self.child.get_egg_group_1()
    }

    fn set_egg_group_1(&mut self, egg_group_1: usize) {
        self.child.set_egg_group_1(egg_group_1);
    }

    fn get_egg_group_2(&self) -> usize {
        self.child.get_egg_group_2()
    }

    fn set_egg_group_2(&mut self, egg_group_2: usize) {
        self.child.set_egg_group_2(egg_group_2);
    }

    fn get_catch_rate(&self) -> usize {
        self.child.get_catch_rate()
    }

    fn set_catch_rate(&mut self, catch_rate: usize) {
        self.child.set_catch_rate(catch_rate);
    }

    fn get_items(&self) -> Vec<usize> {
        self.child.get_items()
    }

    fn set_items(&mut self, items: Vec<usize>) {
        self.child.set_items(items)
    }

    fn get_gender(&self) -> usize {
        self.child.get_gender()
    }

    fn set_gender(&mut self, gender: usize) {
        self.child.set_gender(gender);
    }

    fn get_hatch_cycles(&self) -> usize {
        self.child.get_hatch_cycles()
    }

    fn set_hatch_cycles(&mut self, hatch_cycles: usize) {
        self.child.set_hatch_cycles(hatch_cycles);
    }

    fn get_base_friendship(&self) -> usize {
        self.child.get_base_friendship()
    }

    fn set_base_friendship(&mut self, base_friendship: usize) {
        self.child.set_base_friendship(base_friendship);
    }

    fn get_exp_growth(&self) -> usize {
        self.child.get_exp_growth()
    }

    fn set_exp_growth(&mut self, exp_growth: usize) {
        self.child.set_exp_growth(exp_growth);
    }

    fn get_abilities(&self) -> Vec<usize> {
        self.child.get_abilities()
    }

    fn set_abilities(&mut self, abilities: Vec<usize>) {
        self.child.set_abilities(abilities);
    }

    fn get_ability_index(&self, ability_id: usize) -> Option<usize> {
        self.child.get_ability_index(ability_id)
    }

    fn get_escape_rate(&self) -> usize {
        self.child.get_escape_rate()
    }

    fn set_escape_rate(&mut self, escape_rate: usize) {
        self.child.set_escape_rate(escape_rate);
    }

    fn get_form_count(&self) -> usize {
        self.child.data[0x29] as usize
    }

    fn get_form_stats_index(&self) -> usize {
        u16::from_le_bytes((&self.child.data[0x2A..0x2C]).try_into().unwrap()) as usize
    }

    fn get_base_exp(&self) -> usize {
        self.child.get_base_exp()
    }

    fn set_base_exp(&mut self, base_exp: usize) {
        self.child.set_base_exp(base_exp);
    }

    fn get_color(&self) -> usize {
        self.child.get_color()
    }

    fn set_color(&mut self, color: usize) {
        self.child.set_color(color);
    }

    fn set_tmhm(&mut self, tmhm: Vec<bool>) {
        self.tmhm = tmhm;
    }

    fn set_type_tutors(&mut self, _: Vec<bool>) {}

    fn set_special_tutors(&mut self, _: Vec<Vec<bool>>) {}
}
