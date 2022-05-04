use crate::{GameVersion, LanguageID, PersonalInfo, Pkm};

pub trait GBPkm<T: PersonalInfo>: Pkm<T> {
    fn max_ball_id(&self) -> usize {
        0
    }

    fn min_game_id(&self) -> usize {
        GameVersion::RD as usize
    }

    fn max_game_id(&self) -> usize {
        GameVersion::C as usize
    }

    fn max_iv(&self) -> usize {
        15
    }

    fn max_ev(&self) -> usize {
        u16::MAX as usize
    }

    fn encrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn encrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn decrypted_party_data(&mut self) -> Vec<u8> {
        self.write()
    }

    fn decrypted_box_data(&mut self) -> Vec<u8> {
        self.write()
    }

    fn get_valid(&self) -> bool {
        true
    }

    fn set_valid(&mut self, _valid: bool) {}

    fn refresh_checksum(&mut self) {}

    fn get_non_nickname(&self, language: usize) -> Vec<u8>;

    fn get_is_nicknamed(&self) -> bool {
        self.nickname_trash()
            != self.get_non_nickname(self.guessed_language(LanguageID::English as usize))
    }

    fn set_is_nicknamed(&mut self, nicknamed: bool) {
        if !nicknamed {
            self.set_not_nicknamed(self.guessed_language(LanguageID::English as usize));
        }
    }

    fn is_nicknamed_bank(&self) -> bool {
        todo!()
    }

    fn get_language(&self) -> usize {
        if self.japanese() {
            LanguageID::Japanese as usize
        } else if self.korean() {
            LanguageID::Korean as usize
        } else {
            0
        }
    }

    fn set_not_nicknamed(&mut self, language: usize);

    fn guessed_language(&self, fallback: usize) -> usize {
        let lang = GBPkm::get_language(self);
        if lang > 0 {
            lang
        } else if fallback == LanguageID::French as usize || fallback == LanguageID::German as usize
        {
            fallback
        } else {
            LanguageID::English as usize
        }
    }
}
