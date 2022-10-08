pub trait HyperTrain {
    fn get_hyper_train_flags(&self) -> u8;
    fn set_hyper_train_flags(&mut self, flags: u8);

    fn get_ht_hp(&self) -> bool;
    fn set_ht_hp(&mut self, hp: bool);

    fn get_ht_atk(&self) -> bool;
    fn set_ht_atk(&mut self, atk: bool);

    fn get_ht_def(&self) -> bool;
    fn set_ht_def(&mut self, def: bool);

    fn get_ht_spa(&self) -> bool;
    fn set_ht_spa(&mut self, spa: bool);

    fn get_ht_spd(&self) -> bool;
    fn set_ht_spd(&mut self, spd: bool);

    fn get_ht_spe(&self) -> bool;
    fn set_ht_spe(&mut self, spe: bool);

    fn hyper_train_invert(&mut self, index: u8) -> bool {
        match index {
            0 => {
                self.set_ht_hp(self.get_ht_hp() ^ true);
                self.get_ht_hp()
            }
            1 => {
                self.set_ht_atk(self.get_ht_atk() ^ true);
                self.get_ht_atk()
            }
            2 => {
                self.set_ht_def(self.get_ht_def() ^ true);
                self.get_ht_def()
            }
            3 => {
                self.set_ht_spe(self.get_ht_spe() ^ true);
                self.get_ht_spe()
            }
            4 => {
                self.set_ht_spa(self.get_ht_spa() ^ true);
                self.get_ht_spa()
            }
            5 => {
                self.set_ht_spd(self.get_ht_spd() ^ true);
                self.get_ht_spd()
            }
            _ => false,
        }
    }

    fn is_hyper_trained_all(&self) -> bool {
        self.get_hyper_train_flags() == 0x3F
    }

    fn hyper_train_clear(&mut self) {
        self.set_hyper_train_flags(0);
    }

    fn is_hyper_trained_any(&self) -> bool {
        self.get_hyper_train_flags() != 0
    }

    fn is_hyper_trained(&self, index: u8) -> bool {
        match index {
            0 => self.get_ht_hp(),
            1 => self.get_ht_atk(),
            2 => self.get_ht_def(),
            3 => self.get_ht_spe(),
            4 => self.get_ht_spa(),
            5 => self.get_ht_spd(),
            _ => panic!("Index out of range for hyper train"),
        }
    }
}
