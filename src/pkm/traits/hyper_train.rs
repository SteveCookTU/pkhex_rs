use crate::{PersonalInfo, Pkm};

pub trait HyperTrain {
    fn get_hyper_train_flags(&self) -> u8;
    fn set_hyper_train_flags(&mut self, flags: u8);
    fn get_ht_hp(&self) -> bool;
    fn set_ht_hp(&mut self, trained: bool);
    fn get_ht_atk(&self) -> bool;
    fn set_ht_atk(&mut self, trained: bool);
    fn get_ht_def(&self) -> bool;
    fn set_ht_def(&mut self, trained: bool);
    fn get_ht_spa(&self) -> bool;
    fn set_ht_spa(&mut self, trained: bool);
    fn get_ht_spd(&self) -> bool;
    fn set_ht_spd(&mut self, trained: bool);
    fn get_ht_spe(&self) -> bool;
    fn set_ht_spe(&mut self, trained: bool);

    fn hyper_train_invert(&mut self, index: usize) {
        match index {
            0 => self.set_ht_hp(self.get_ht_hp() ^ true),
            1 => self.set_ht_atk(self.get_ht_atk() ^ true),
            2 => self.set_ht_def(self.get_ht_def() ^ true),
            3 => self.set_ht_spe(self.get_ht_spe() ^ true),
            4 => self.set_ht_spa(self.get_ht_spa() ^ true),
            5 => self.set_ht_spd(self.get_ht_spd() ^ true),
            _ => {}
        }
    }

    fn is_hyper_trained_all(&self) -> bool {
        self.get_hyper_train_flags() == 0x3F
    }

    fn hyper_train_clear(&mut self) {
        self.set_hyper_train_flags(0);
    }

    fn is_hyper_trained(&self) -> bool {
        self.get_hyper_train_flags() != 0
    }

    fn is_hyper_trained_by_index(&self, index: usize) -> bool {
        match index {
            0 => self.get_ht_hp(),
            1 => self.get_ht_atk(),
            2 => self.get_ht_def(),
            3 => self.get_ht_spe(),
            4 => self.get_ht_spa(),
            5 => self.get_ht_spd(),
            _ => false,
        }
    }

    fn set_suggested_hyper_training_data<I: PersonalInfo, T: HyperTrain + Pkm<I> + ?Sized>(
        pkm: &mut T,
        ivs: Option<[usize; 6]>,
    ) {
        if !pkm.is_hyper_training_available() {
            pkm.set_hyper_train_flags(0);
            return;
        }
        let ivs = ivs.unwrap_or(pkm.get_ivs());

        pkm.set_ht_hp(ivs[0] != 31);
        pkm.set_ht_atk(ivs[1] != 31 && ivs[1] > 2);
        pkm.set_ht_def(ivs[2] != 31);
        pkm.set_ht_spa(ivs[4] != 31);
        pkm.set_ht_spd(ivs[5] != 31);
        pkm.set_ht_hp(
            ivs[3] != 31
                && ivs[3] > 2
                && (ivs[3] > 17
                    || pkm.get_ht_hp()
                    || pkm.get_ht_atk()
                    || pkm.get_ht_def()
                    || pkm.get_ht_spa()
                    || pkm.get_ht_spd()),
        );

        //TODO: Reset CP for PB7
    }

    fn is_hyper_training_available(&self) -> bool {
        true
    }
}
