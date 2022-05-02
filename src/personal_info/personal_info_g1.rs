use crate::{get_bits, set_bits, PersonalInfo};

pub const SIZE: usize = 0x1C;

pub struct PersonalInfoG1 {
    pub(super) data: Vec<u8>,
    tmhm: Vec<bool>,
}

impl PersonalInfoG1 {
    pub fn get_dex_id(&self) -> usize {
        self.data[0x0] as usize
    }

    pub fn set_dex_id(&mut self, id: usize) {
        self.data[0x0] = id as u8;
    }

    pub fn get_spc(&self) -> usize {
        self.data[0x5] as usize
    }

    pub fn set_spc(&mut self, spc: usize) {
        self.data[0x5] = spc as u8;
    }

    pub fn get_move_1(&self) -> usize {
        self.data[0xF] as usize
    }

    pub fn set_move_1(&mut self, move_1: usize) {
        self.data[0xF] = move_1 as u8;
    }

    pub fn get_move_2(&self) -> usize {
        self.data[0x10] as usize
    }

    pub fn set_move_2(&mut self, move_2: usize) {
        self.data[0x10] = move_2 as u8;
    }

    pub fn get_move_3(&self) -> usize {
        self.data[0x11] as usize
    }

    pub fn set_move_3(&mut self, move_3: usize) {
        self.data[0x11] = move_3 as u8;
    }

    pub fn get_move_4(&self) -> usize {
        self.data[0x12] as usize
    }

    pub fn set_move_4(&mut self, move_4: usize) {
        self.data[0x12] = move_4 as u8;
    }

    pub fn get_ev_spc(&self) -> usize {
        self.get_spc()
    }

    pub fn set_ev_spc(&mut self, ev_spc: usize) {
        self.set_spc(ev_spc);
    }

    pub fn get_moves(&self) -> [usize; 4] {
        [
            self.get_move_1(),
            self.get_move_2(),
            self.get_move_3(),
            self.get_move_4(),
        ]
    }

    pub fn set_moves(&mut self, moves: &[usize]) {
        if moves.len() == 4 {
            self.set_move_1(moves[0]);
            self.set_move_2(moves[1]);
            self.set_move_3(moves[2]);
            self.set_move_4(moves[3]);
        }
    }
}

impl PersonalInfo for PersonalInfoG1 {
    fn new(data: Vec<u8>) -> Self {
        Self {
            tmhm: get_bits(&data[0x14..0x1C]),
            data,
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn write(&mut self) -> Vec<u8> {
        set_bits(&mut self.data[0x14..], &self.tmhm);
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
        self.get_spc()
    }

    fn set_spa(&mut self, spa: usize) {
        self.set_spc(spa);
    }

    fn get_spd(&self) -> usize {
        self.get_spc()
    }

    fn set_spd(&mut self, spd: usize) {
        self.set_spc(spd);
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
        self.get_ev_spc()
    }

    fn set_ev_spa(&mut self, _: usize) {}

    fn get_ev_spd(&self) -> usize {
        self.get_ev_spc()
    }

    fn set_ev_spd(&mut self, _: usize) {}

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
        0
    }

    fn set_egg_group_1(&mut self, _: usize) {}

    fn get_egg_group_2(&self) -> usize {
        0
    }

    fn set_egg_group_2(&mut self, _: usize) {}

    fn get_catch_rate(&self) -> usize {
        self.data[0x8] as usize
    }

    fn set_catch_rate(&mut self, catch_rate: usize) {
        self.data[0x8] = catch_rate as u8;
    }

    fn get_items(&self) -> Vec<usize> {
        vec![]
    }

    fn set_items(&mut self, _: Vec<usize>) {}

    fn get_gender(&self) -> usize {
        0
    }

    fn set_gender(&mut self, _: usize) {}

    fn get_hatch_cycles(&self) -> usize {
        0
    }

    fn set_hatch_cycles(&mut self, _: usize) {}

    fn get_base_friendship(&self) -> usize {
        0
    }

    fn set_base_friendship(&mut self, _: usize) {}

    fn get_exp_growth(&self) -> usize {
        self.data[0x13] as usize
    }

    fn set_exp_growth(&mut self, exp_growth: usize) {
        self.data[0x13] = exp_growth as u8;
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
        self.data[0x9] as usize
    }

    fn set_base_exp(&mut self, base_exp: usize) {
        self.data[0x9] = base_exp as u8;
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
