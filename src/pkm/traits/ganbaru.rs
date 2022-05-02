pub trait Ganbaru {
    fn get_gv_hp(&self) -> u8;
    fn set_gv_hp(&mut self, gv: u8);
    fn get_gv_atk(&self) -> u8;
    fn set_gv_atk(&mut self, gv: u8);
    fn get_gv_def(&self) -> u8;
    fn set_gv_def(&mut self, gv: u8);
    fn get_gv_spe(&self) -> u8;
    fn set_gv_spe(&mut self, gv: u8);
    fn get_gv_spa(&self) -> u8;
    fn set_gv_spa(&mut self, gv: u8);
    fn get_gv_spd(&self) -> u8;
    fn set_gv_spd(&mut self, gv: u8);

    fn get_max(&self, _index: usize) -> u8 {
        todo!()
    }

    fn set_suggested_ganbaru_values(&mut self, _pkm: ()) {
        todo!()
    }

    fn is_ganbaru_values_max(&self, _pkm: ()) {
        todo!()
    }

    fn clear_ganbaru_values(&mut self) {
        self.set_gv_hp(0);
        self.set_gv_atk(0);
        self.set_gv_def(0);
        self.set_gv_spe(0);
        self.set_gv_spa(0);
        self.set_gv_spd(0);
    }

    fn get_gv(&self, index: usize) -> Option<u8> {
        match index {
            0 => Some(self.get_gv_hp()),
            1 => Some(self.get_gv_atk()),
            2 => Some(self.get_gv_def()),
            3 => Some(self.get_gv_spe()),
            4 => Some(self.get_gv_spa()),
            5 => Some(self.get_gv_spd()),
            _ => None,
        }
    }

    fn set_gv(&mut self, index: usize, value: u8) {
        match index {
            0 => self.set_gv_hp(value),
            1 => self.set_gv_atk(value),
            2 => self.set_gv_def(value),
            3 => self.set_gv_spe(value),
            4 => self.set_gv_spa(value),
            5 => self.set_gv_spd(value),
            _ => {}
        }
    }
}

pub mod ganbaru_extensions {

    pub const TRUE_MAX: u8 = 10;

    const GANBARU_MULTIPLIER: [u8; 11] = [0, 2, 3, 4, 7, 8, 9, 14, 15, 16, 25];

    pub fn get_pkm_max_ganbaru(_index: usize) -> u8 {
        todo!()
    }

    fn get_max_ganbaru(iv: usize) -> u8 {
        let bias = get_bias(iv);
        TRUE_MAX - bias
    }

    pub fn get_bias(iv: usize) -> u8 {
        match iv {
            _ if iv >= 31 => 3,
            _ if iv >= 26 => 2,
            _ if iv >= 20 => 1,
            _ => 0,
        }
    }

    pub fn get_ganbaru_multiplier(gv: u8, iv: usize) -> u8 {
        GANBARU_MULTIPLIER[u8::min(gv + get_bias(iv), TRUE_MAX) as usize]
    }
}
