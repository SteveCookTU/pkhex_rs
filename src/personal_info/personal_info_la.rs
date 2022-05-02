use crate::tables::MOVE_SHOP_8_LA;
use crate::{flag_util, PersonalInfo};

pub const SIZE: usize = 0xB0;

pub struct PersonalInfoLA {
    data: Vec<u8>,
    special_tutors: Vec<Vec<bool>>,
}

impl PersonalInfoLA {
    fn get_ev_yield(&self) -> usize {
        u16::from_le_bytes((&self.data[0xA..0xC]).try_into().unwrap()) as usize
    }

    fn set_ev_yield(&mut self, ev_yield: usize) {
        let ev_yield_bytes = (ev_yield as u16).to_le_bytes();
        self.data.splice(0xA..0xC, ev_yield_bytes);
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
        u16::from_le_bytes((&self.data[0x18..0x1A]).try_into().unwrap()) as usize
    }

    pub fn set_ability_1(&mut self, ability: usize) {
        let ability_bytes = (ability as u16).to_le_bytes();
        self.data.splice(0x18..0x1A, ability_bytes);
    }

    pub fn get_ability_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x1A..0x1C]).try_into().unwrap()) as usize
    }

    pub fn set_ability_2(&mut self, ability: usize) {
        let ability_bytes = (ability as u16).to_le_bytes();
        self.data.splice(0x1A..0x1C, ability_bytes);
    }

    pub fn get_ability_h(&self) -> usize {
        u16::from_le_bytes((&self.data[0x1C..0x1E]).try_into().unwrap()) as usize
    }

    pub fn set_ability_h(&mut self, ability: usize) {
        let ability_bytes = (ability as u16).to_le_bytes();
        self.data.splice(0x1C..0x1E, ability_bytes);
    }

    pub fn get_is_present_in_game(&self) -> bool {
        ((self.data[0x21] >> 6) & 1) == 1
    }

    pub fn set_is_present_in_game(&mut self, present_in_game: bool) {
        self.data[0x21] = (self.data[0x21] & !(0x40)) | if present_in_game { 0x40 } else { 0 };
    }

    pub fn get_sprite_form(&self) -> bool {
        ((self.data[0x21] >> 7) & 1) == 1
    }

    pub fn set_sprite_form(&mut self, sprite_form: bool) {
        self.data[0x21] = (self.data[0x21] & !(0x80)) | if sprite_form { 0x80 } else { 0 };
    }

    pub fn get_hatch_species(&self) -> usize {
        u16::from_le_bytes((&self.data[0x56..0x58]).try_into().unwrap()) as usize
    }

    pub fn set_hatch_species(&mut self, hatch_species: usize) {
        let hatch_species_bytes = (hatch_species as u16).to_le_bytes();
        self.data.splice(0x56..0x58, hatch_species_bytes);
    }

    pub fn get_hatch_form_index(&self) -> usize {
        u16::from_le_bytes((&self.data[0x58..0x5A]).try_into().unwrap()) as usize
    }

    pub fn set_hatch_form_index(&mut self, hatch_form_index: usize) {
        let hatch_form_index_bytes = (hatch_form_index as u16).to_le_bytes();
        self.data.splice(0x58..0x5A, hatch_form_index_bytes);
    }

    pub fn get_regional_flags(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x5A..0x5C]).try_into().unwrap())
    }

    pub fn set_regional_flags(&mut self, regional_flags: u16) {
        let regional_flags_bytes = regional_flags.to_le_bytes();
        self.data.splice(0x5A..0x5C, regional_flags_bytes);
    }

    pub fn get_is_regional_form(&self) -> bool {
        (self.get_regional_flags() & 1) == 1
    }

    pub fn set_is_regional_form(&mut self, is_regional_form: bool) {
        self.set_regional_flags(
            (self.get_regional_flags() & 0xFFFE) | if is_regional_form { 1 } else { 0 },
        );
    }

    pub fn get_species(&self) -> usize {
        u16::from_le_bytes((&self.data[0x5C..0x5E]).try_into().unwrap()) as usize
    }

    pub fn set_species(&mut self, species: usize) {
        let species_bytes = (species as u16).to_le_bytes();
        self.data.splice(0x5C..0x5E, species_bytes);
    }

    pub fn get_form(&self) -> usize {
        u16::from_le_bytes((&self.data[0x5E..0x60]).try_into().unwrap()) as usize
    }

    pub fn set_form(&mut self, form: usize) {
        let form_bytes = (form as u16).to_le_bytes();
        self.data.splice(0x5E..0x60, form_bytes);
    }

    pub fn get_dex_index_hisui(&self) -> usize {
        u16::from_le_bytes((&self.data[0x60..0x62]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_hisui(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x60..0x62, dex_index_bytes);
    }

    pub fn get_dex_index_local_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0x62..0x64]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_local_1(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x62..0x64, dex_index_bytes);
    }

    pub fn get_dex_index_local_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x64..0x66]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_local_2(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x64..0x66, dex_index_bytes);
    }

    pub fn get_dex_index_local_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x66..0x68]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_local_3(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x66..0x68, dex_index_bytes);
    }

    pub fn get_dex_index_local_4(&self) -> usize {
        u16::from_le_bytes((&self.data[0x68..0x6A]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_local_4(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x68..0x6A, dex_index_bytes);
    }

    pub fn get_dex_index_local_5(&self) -> usize {
        u16::from_le_bytes((&self.data[0x6A..0x6C]).try_into().unwrap()) as usize
    }

    pub fn set_dex_index_local_5(&mut self, dex_index: usize) {
        let dex_index_bytes = (dex_index as u16).to_le_bytes();
        self.data.splice(0x6A..0x6C, dex_index_bytes);
    }

    pub fn get_move_shop_count(&self) -> usize {
        let arr = &self.special_tutors[0];
        let mut count = 0;
        for index in arr {
            if *index {
                count += 1;
            }
        }
        count
    }

    pub fn get_move_shop_index(&self, mut rand_index_from_count: usize) -> Option<usize> {
        let arr = &self.special_tutors[0];
        for (i, index) in arr.iter().enumerate() {
            if !index {
                continue;
            }
            if rand_index_from_count == 0 {
                return Some(i);
            }
            rand_index_from_count -= 1;
        }
        None
    }
}

impl PersonalInfo for PersonalInfoLA {
    fn new(data: Vec<u8>) -> Self {
        let mut move_shop = vec![false; MOVE_SHOP_8_LA.len()];
        for (i, entry) in move_shop.iter_mut().enumerate() {
            *entry = flag_util::get_flag(&data, 0xA8 + (i >> 3), i);
        }
        Self {
            data,
            special_tutors: vec![move_shop],
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn write(&mut self) -> Vec<u8> {
        for i in 0..self.special_tutors[0].len() {
            flag_util::set_flag(
                &mut self.data,
                0xA8 + (i >> 3),
                i,
                self.special_tutors[0][i],
            );
        }
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
        0
    }

    fn set_escape_rate(&mut self, _escape_rate: usize) {}

    fn get_form_count(&self) -> usize {
        self.data[0x20] as usize
    }

    fn set_form_count(&mut self, form_count: usize) {
        self.data[0x20] = form_count as u8;
    }

    fn get_form_stats_index(&self) -> usize {
        u16::from_le_bytes((&self.data[0x1E..0x20]).try_into().unwrap()) as usize
    }

    fn set_form_stats_index(&mut self, form_stats_index: usize) {
        let form_stats_index_bytes = (form_stats_index as u16).to_le_bytes();
        self.data.splice(0x1E..0x20, form_stats_index_bytes);
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

    fn set_tmhm(&mut self, _tmhm: Vec<bool>) {}

    fn set_type_tutors(&mut self, _type_tutors: Vec<bool>) {}

    fn set_special_tutors(&mut self, special_tutors: Vec<Vec<bool>>) {
        self.special_tutors = special_tutors
    }
}
