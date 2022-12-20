use crate::legality::evolutions::EvolutionHistory;
use crate::{PKError, PKResult};

pub trait HyperTrain {
    fn hyper_train_flags(&self) -> u8;
    fn set_hyper_train_flags(&mut self, flags: u8);
    fn ht_hp(&self) -> bool;
    fn set_ht_hp(&mut self, val: bool);
    fn ht_atk(&self) -> bool;
    fn set_ht_atk(&mut self, val: bool);
    fn ht_def(&self) -> bool;
    fn set_ht_def(&mut self, val: bool);
    fn ht_spa(&self) -> bool;
    fn set_ht_spa(&mut self, val: bool);
    fn ht_spd(&self) -> bool;
    fn set_ht_spd(&mut self, val: bool);
    fn ht_spe(&self) -> bool;
    fn set_ht_spe(&mut self, val: bool);

    fn hyper_train_invert(&mut self, index: usize) -> bool {
        match index {
            0 => {
                self.set_ht_hp(self.ht_hp() ^ true);
                self.ht_hp()
            }
            1 => {
                self.set_ht_atk(self.ht_atk() ^ true);
                self.ht_atk()
            }
            2 => {
                self.set_ht_def(self.ht_def() ^ true);
                self.ht_def()
            }
            3 => {
                self.set_ht_spe(self.ht_spe() ^ true);
                self.ht_spe()
            }
            4 => {
                self.set_ht_spa(self.ht_spa() ^ true);
                self.ht_spa()
            }
            5 => {
                self.set_ht_spd(self.ht_spd() ^ true);
                self.ht_spd()
            }
            _ => false,
        }
    }

    fn is_hyper_trained_all(&self) -> bool {
        self.hyper_train_flags() == 0x3F
    }

    fn hyper_train_clear(&mut self) {
        self.set_hyper_train_flags(0);
    }

    fn is_hyper_trained(&self) -> bool {
        self.hyper_train_flags() != 0
    }

    fn is_index_hyper_trained(&self, index: usize) -> PKResult<bool> {
        match index {
            0 => Ok(self.ht_hp()),
            1 => Ok(self.ht_atk()),
            2 => Ok(self.ht_def()),
            3 => Ok(self.ht_spe()),
            4 => Ok(self.ht_spa()),
            5 => Ok(self.ht_spd()),
            _ => Err(PKError::IndexOutOfRange { index }),
        }
    }

    fn get_hyper_train_min_level(&self, h: &EvolutionHistory) -> u8 {
        if h.has_visited_gen_9() {
            50
        } else {
            100
        }
    }
}
