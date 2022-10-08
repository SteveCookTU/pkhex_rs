use crate::tables;

pub trait Awakened {
    fn get_av_hp(&self) -> u8;
    fn get_av_atk(&self) -> u8;
    fn get_av_def(&self) -> u8;
    fn get_av_spe(&self) -> u8;
    fn get_av_spa(&self) -> u8;
    fn get_av_spd(&self) -> u8;

    fn set_av_hp(&mut self, val: u8);
    fn set_av_atk(&mut self, val: u8);
    fn set_av_def(&mut self, val: u8);
    fn set_av_spe(&mut self, val: u8);
    fn set_av_spa(&mut self, val: u8);
    fn set_av_spd(&mut self, val: u8);

    fn awakening_sum(&self) -> u32 {
        u32::from(self.get_av_hp())
            + u32::from(self.get_av_atk())
            + u32::from(self.get_av_def())
            + u32::from(self.get_av_spa())
            + u32::from(self.get_av_spd())
            + u32::from(self.get_av_spe())
    }

    fn awakening_set_all_to(&mut self, value: u8) {
        self.set_av_hp(value);
        self.set_av_atk(value);
        self.set_av_def(value);
        self.set_av_spa(value);
        self.set_av_spd(value);
        self.set_av_spe(value);
    }

    fn awakening_clear(&mut self) {
        self.awakening_set_all_to(0);
    }

    fn awakening_max(&mut self) {
        self.awakening_set_all_to(tables::AWAKENING_MAX);
    }

    fn awakening_get_visual(&self, value: &mut [u8; 6]) {
        value[0] = self.get_av_hp();
        value[1] = self.get_av_atk();
        value[2] = self.get_av_def();
        value[3] = self.get_av_spa();
        value[4] = self.get_av_spd();
        value[5] = self.get_av_spe();
    }

    fn awakening_set_visual(&mut self, value: &[u8; 6]) {
        self.set_av_hp(value[0]);
        self.set_av_atk(value[1]);
        self.set_av_def(value[2]);
        self.set_av_spa(value[3]);
        self.set_av_spd(value[4]);
        self.set_av_spe(value[5]);
    }

    fn awakening_all_valid(&self) -> bool {
        ![
            self.get_av_hp(),
            self.get_av_atk(),
            self.get_av_def(),
            self.get_av_spa(),
            self.get_av_spd(),
            self.get_av_spe(),
        ]
        .iter()
        .any(|&v| v > tables::AWAKENING_MAX)
    }

    fn set_av(&mut self, index: usize, value: u8) {
        match index {
            0 => {
                self.set_av_hp(value);
            }
            1 => {
                self.set_av_atk(value);
            }
            2 => {
                self.set_av_def(value);
            }
            3 => {
                self.set_av_spe(value);
            }
            4 => {
                self.set_av_spa(value);
            }
            5 => {
                self.set_av_spd(value);
            }
            _ => panic!("AV Index out of range"),
        }
    }

    fn get_av(&self, index: usize) -> u8 {
        match index {
            0 => self.get_av_hp(),
            1 => self.get_av_atk(),
            2 => self.get_av_def(),
            3 => self.get_av_spe(),
            4 => self.get_av_spa(),
            5 => self.get_av_spd(),
            _ => panic!("AV Index out of range"),
        }
    }

    fn get_avs(&self, value: &mut [u8; 6]) {
        value[0] = self.get_av_hp();
        value[1] = self.get_av_atk();
        value[2] = self.get_av_def();
        value[3] = self.get_av_spe();
        value[4] = self.get_av_spa();
        value[5] = self.get_av_spd();
    }

    fn is_awakening_below(&self, initial: &dyn Awakened) -> bool {
        !self.is_awakening_above_or_equal(initial)
    }

    fn is_awakening_above_or_equal(&self, initial: &dyn Awakened) -> bool {
        self.get_av_hp() >= initial.get_av_hp()
            && self.get_av_atk() >= initial.get_av_atk()
            && self.get_av_def() >= initial.get_av_def()
            && self.get_av_spe() >= initial.get_av_spe()
            && self.get_av_spa() >= initial.get_av_spa()
            && self.get_av_spd() >= initial.get_av_spd()
    }
}

// TODO: PB7 awakening
