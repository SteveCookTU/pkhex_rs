use crate::{species_name, LanguageID, PersonalInfo, Pkm};

pub trait GBPkm<T: PersonalInfo>: Pkm<T> {
    fn get_non_nickname(&self, language: usize) -> Vec<u8>;

    fn set_not_nicknamed(&mut self, language: usize);

    fn guessed_language(&self, fallback: usize) -> usize {
        let lang = self.get_language();
        if lang > 0 {
            lang
        } else if fallback == LanguageID::French as usize || fallback == LanguageID::German as usize
        {
            fallback
        } else {
            LanguageID::English as usize
        }
    }

    fn is_nicknamed_bank(&self) -> bool {
        let sp_name = species_name::get_species_name_generation(
            self.get_species(),
            self.guessed_language(LanguageID::English as usize),
            self.format(),
        );
        self.get_nickname() != sp_name
    }

    fn transfer_language(&self, dest_language: usize) -> usize {
        let expect =
            species_name::get_species_name_generation(self.get_species(), dest_language, 2);
        if self.get_nickname() == expect {
            dest_language
        } else {
            self.guessed_language(dest_language)
        }
    }

    fn get_ev_spc(&self) -> usize;

    fn set_ev_spc(&mut self, spc: usize);

    fn get_dv16(&self) -> u16;

    fn set_dv16(&mut self, dv16: u16);

    fn get_iv_spc(&self) -> usize {
        (self.get_dv16() & 0xF) as usize
    }

    fn set_iv_spc(&mut self, spc: usize) {
        self.set_dv16((self.get_dv16() & !0xF) | if spc > 0xF { 0xF } else { spc as u16 });
    }

    fn max_evs(&mut self) {
        self.set_ev_hp(self.get_max_ev());
        self.set_ev_atk(self.get_max_ev());
        self.set_ev_def(self.get_max_ev());
        self.set_ev_spe(self.get_max_ev());
        self.set_ev_spc(self.get_max_ev());
    }
}
