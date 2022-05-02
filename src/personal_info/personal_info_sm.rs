use crate::personal_info_xy::PersonalInfoXY;
use crate::{get_bits, set_bits, PersonalInfo};

pub const SIZE: usize = 0x54;

pub struct PersonalInfoSM {
    pub(super) child: PersonalInfoXY,
    tmhm: Vec<bool>,
    type_tutors: Vec<bool>,
    special_tutors: Vec<Vec<bool>>,
}

impl PersonalInfoSM {
    pub fn new_child(data: Vec<u8>) -> Self {
        Self {
            child: PersonalInfoXY::new_child(data),
            tmhm: vec![],
            type_tutors: vec![],
            special_tutors: vec![],
        }
    }

    pub fn get_telekinesis(&self) -> bool {
        self.child.get_telekinesis()
    }

    pub fn set_telekinesis(&mut self, telekinesis: bool) {
        self.child.set_telekinesis(telekinesis);
    }

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

    pub fn get_item_3(&self) -> usize {
        self.child.get_item_3()
    }

    pub fn set_item_3(&mut self, item: usize) {
        self.child.set_item_3(item);
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

    pub fn get_ability_h(&self) -> usize {
        self.child.get_ability_h()
    }

    pub fn set_ability_h(&mut self, ability: usize) {
        self.child.set_ability_h(ability);
    }

    pub fn get_sprite_flip(&self) -> bool {
        self.child.get_sprite_flip()
    }

    pub fn set_sprite_flip(&mut self, sprite_flip: bool) {
        self.child.set_sprite_flip(sprite_flip);
    }

    pub fn get_sprite_form(&self) -> bool {
        self.child.get_sprite_form()
    }

    pub fn set_sprite_form(&mut self, sprite_form: bool) {
        self.child.set_sprite_form(sprite_form)
    }

    pub fn has_hidden_ability(&self) -> bool {
        self.child.has_hidden_ability()
    }

    pub fn get_special_z_item(&self) -> usize {
        u16::from_le_bytes(self.child.get_data()[0x4C..0x4E].try_into().unwrap()) as usize
    }

    pub fn set_special_z_item(&mut self, special_z_item: usize) {
        let special_z_item_bytes = (special_z_item as u16).to_le_bytes();
        self.child
            .child
            .data
            .splice(0x4C..0x4E, special_z_item_bytes);
    }

    pub fn get_special_z_base_move(&self) -> usize {
        u16::from_le_bytes(self.child.get_data()[0x4E..0x50].try_into().unwrap()) as usize
    }

    pub fn set_special_z_base_move(&mut self, special_z_base_move: usize) {
        let special_z_base_move_bytes = (special_z_base_move as u16).to_le_bytes();
        self.child
            .child
            .data
            .splice(0x4E..0x50, special_z_base_move_bytes);
    }

    pub fn get_special_z_move(&self) -> usize {
        u16::from_le_bytes(self.child.get_data()[0x50..0x52].try_into().unwrap()) as usize
    }

    pub fn set_special_z_move(&mut self, special_z_move: usize) {
        let special_z_move_bytes = (special_z_move as u16).to_le_bytes();
        self.child
            .child
            .data
            .splice(0x50..0x52, special_z_move_bytes);
    }

    pub fn get_local_variant(&self) -> bool {
        self.child.get_data()[0x52] == 1
    }

    pub fn set_local_variant(&mut self, local_variant: bool) {
        self.child.child.data[0x52] = if local_variant { 1 } else { 0 };
    }
}

impl PersonalInfo for PersonalInfoSM {
    fn new(data: Vec<u8>) -> Self {
        Self {
            tmhm: get_bits(&data[0x28..0x38]),
            type_tutors: get_bits(&data[0x38..0x3C]),
            special_tutors: vec![get_bits(&data[0x3C..0x46])],
            child: PersonalInfoXY::new_child(data),
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        self.child.get_data()
    }

    fn write(&mut self) -> Vec<u8> {
        set_bits(&mut self.child.child.data[0x28..], &self.tmhm);
        set_bits(&mut self.child.child.data[0x38..], &self.type_tutors);
        set_bits(&mut self.child.child.data[0x3C..], &self.special_tutors[0]);
        self.child.child.data.clone()
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

    fn get_evo_stage(&self) -> usize {
        self.child.get_evo_stage()
    }

    fn set_evo_stage(&mut self, evo_stage: usize) {
        self.child.set_evo_stage(evo_stage)
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
        self.child.get_form_count()
    }

    fn set_form_count(&mut self, form_count: usize) {
        self.child.set_form_count(form_count);
    }

    fn get_form_stats_index(&self) -> usize {
        self.child.get_form_stats_index()
    }

    fn set_form_stats_index(&mut self, form_stats_index: usize) {
        self.child.set_form_stats_index(form_stats_index);
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

    fn get_height(&self) -> usize {
        self.child.get_height()
    }

    fn set_height(&mut self, height: usize) {
        self.child.set_height(height);
    }

    fn get_weight(&self) -> usize {
        self.child.get_weight()
    }

    fn set_weight(&mut self, weight: usize) {
        self.child.set_weight(weight);
    }

    fn set_tmhm(&mut self, tmhm: Vec<bool>) {
        self.tmhm = tmhm;
    }

    fn set_type_tutors(&mut self, type_tutors: Vec<bool>) {
        self.type_tutors = type_tutors
    }

    fn set_special_tutors(&mut self, special_tutors: Vec<Vec<bool>>) {
        self.special_tutors = special_tutors;
    }
}
