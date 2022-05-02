use crate::tables::AWAKENING_MAX;
use rand::Rng;

pub trait Awakened {
    fn get_av_hp(&self) -> u8;

    fn set_av_hp(&mut self, av: u8);

    fn get_av_atk(&self) -> u8;

    fn set_av_atk(&mut self, av: u8);

    fn get_av_def(&self) -> u8;

    fn set_av_def(&mut self, av: u8);

    fn get_av_spe(&self) -> u8;

    fn set_av_spe(&mut self, av: u8);

    fn get_av_spa(&self) -> u8;

    fn set_av_spa(&mut self, av: u8);

    fn get_av_spd(&self) -> u8;

    fn set_av_spd(&mut self, av: u8);

    fn awakening_sum(&self) -> u16 {
        u16::from(self.get_av_hp())
            + u16::from(self.get_av_atk())
            + u16::from(self.get_av_def())
            + u16::from(self.get_av_spa())
            + u16::from(self.get_av_spd())
            + u16::from(self.get_av_spe())
    }

    fn awakening_clear(&mut self) {
        self.awakening_set_all_to(0);
    }

    fn awakening_max(&mut self) {
        self.awakening_set_all_to(AWAKENING_MAX);
    }

    fn awakening_set_all_to(&mut self, value: u8) {
        self.set_av_hp(value);
        self.set_av_atk(value);
        self.set_av_def(value);
        self.set_av_spa(value);
        self.set_av_spd(value);
        self.set_av_spe(value);
    }

    fn awakening_set_random(&mut self, min: u8, max: u8) {
        let mut rnd = rand::thread_rng();
        let rand_clamp = max.saturating_add(1);
        for i in 0..6 {
            self.set_av(i, rnd.gen_range(min..rand_clamp));
        }
    }

    fn awakening_all_valid(&self) -> bool {
        !(self.get_av_hp() > AWAKENING_MAX
            || self.get_av_atk() > AWAKENING_MAX
            || self.get_av_def() > AWAKENING_MAX
            || self.get_av_spa() > AWAKENING_MAX
            || self.get_av_spd() > AWAKENING_MAX
            || self.get_av_spe() > AWAKENING_MAX)
    }

    fn set_av(&mut self, index: usize, value: u8) {
        match index {
            0 => self.set_av_hp(value),
            1 => self.set_av_atk(value),
            2 => self.set_av_def(value),
            3 => self.set_av_spe(value),
            4 => self.set_av_spa(value),
            5 => self.set_av_spd(value),
            _ => {}
        }
    }

    fn get_av(&self, index: usize) -> Option<u8> {
        match index {
            0 => Some(self.get_av_hp()),
            1 => Some(self.get_av_atk()),
            2 => Some(self.get_av_def()),
            3 => Some(self.get_av_spe()),
            4 => Some(self.get_av_spa()),
            5 => Some(self.get_av_spd()),
            _ => None,
        }
    }

    fn get_avs(&self, value: &mut [u8]) {
        if value.len() == 6 {
            value[0] = self.get_av_hp();
            value[1] = self.get_av_atk();
            value[2] = self.get_av_def();
            value[3] = self.get_av_spe();
            value[4] = self.get_av_spa();
            value[5] = self.get_av_spd();
        }
    }

    fn set_suggested_awakened_values(&self) {
        todo!()
    }

    fn is_awakening_below<T: Awakened>(&self, initial: &T) -> bool {
        !self.is_awakening_above_or_equal(initial)
    }

    fn is_awakening_above_or_equal<T: Awakened>(&self, initial: &T) -> bool {
        !(self.get_av_hp() < initial.get_av_hp()
            || self.get_av_atk() < initial.get_av_atk()
            || self.get_av_def() < initial.get_av_def()
            || self.get_av_spe() < initial.get_av_spe()
            || self.get_av_spa() < initial.get_av_spa()
            || self.get_av_spd() < initial.get_av_spd())
    }
}
