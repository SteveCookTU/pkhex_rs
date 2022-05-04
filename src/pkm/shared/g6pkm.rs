use crate::poke_crypto::{encrypt_array_6, SIZE_6PARTY, SIZE_6STORED};
use crate::tables::locations::LINK_TRADE_6;
use crate::{PersonalInfo, Pkm, SanityChecksum, TrainerInfo};

pub trait G6Pkm<I: PersonalInfo>: Pkm<I> + SanityChecksum {
    fn size_party(&self) -> usize {
        SIZE_6PARTY
    }

    fn size_stored(&self) -> usize {
        SIZE_6STORED
    }

    fn nickname_trash(&self) -> Vec<u8> {
        self.get_data()[0x40..(0x40 + 26)].to_vec()
    }

    fn ht_trash(&self) -> Vec<u8> {
        self.get_data()[0x78..(0x78 + 26)].to_vec()
    }

    fn ot_trash(&self) -> Vec<u8> {
        self.get_data()[0xB0..(0xB0 + 26)].to_vec()
    }

    fn refresh_checksum(&mut self) {
        self.set_checksum(self.calculate_checksum());
    }

    fn checksum_valid(&self) -> bool {
        self.calculate_checksum() == self.get_checksum()
    }

    fn get_valid(&self) -> bool {
        self.get_sanity() == 0 && G6Pkm::checksum_valid(self)
    }

    fn set_valid(&mut self, valid: bool) {
        if valid {
            self.set_sanity(0);
            G6Pkm::refresh_checksum(self);
        }
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0;
        for i in (8..SIZE_6STORED).step_by(2) {
            chk += u16::from_le_bytes(self.get_data()[i..(i + 2)].try_into().unwrap());
        }
        chk
    }

    fn get_current_friendship(&self) -> usize {
        if self.get_current_handler() == 0 {
            self.get_ot_friendship()
        } else {
            self.get_ht_friendship()
        }
    }

    fn set_current_friendship(&mut self, friendship: usize) {
        if self.get_current_handler() == 0 {
            self.set_ot_friendship(friendship);
        } else {
            self.set_ht_friendship(friendship);
        }
    }

    fn get_opposite_friendship(&self) -> usize {
        if self.get_current_handler() == 1 {
            self.get_ot_friendship()
        } else {
            self.get_ht_friendship()
        }
    }

    fn set_opposite_friendship(&mut self, friendship: usize) {
        if self.get_current_handler() == 1 {
            self.set_ot_friendship(friendship);
        } else {
            self.set_ht_friendship(friendship);
        }
    }

    fn psv(&self) -> usize {
        ((self.get_pid() >> 16) ^ (self.get_pid() & 0xFFFF)) >> 4
    }

    fn tsv(&self) -> usize {
        (self.get_tid() ^ self.get_sid()) >> 4
    }

    fn is_untraded(&self) -> bool {
        let data = self.get_data();
        data[0x78] == 0 && data[0x79] == 0 && self.format() == self.generation()
    }

    fn get_characteristic(&self) -> usize {
        let pm6 = self.get_encryption_constant() & 6;
        let max_iv = self.get_max_iv();
        let mut pm6stat = 0;
        for i in 0..6 {
            pm6stat = (pm6 + i) % 6;
            if self.get_iv(pm6stat) == max_iv {
                break;
            }
        }
        (pm6stat + 5) + (max_iv % 5)
    }

    fn encrypt(&mut self) -> Vec<u8> {
        G6Pkm::refresh_checksum(self);
        encrypt_array_6(self.get_data())
    }

    fn fix_relearn(&mut self) {
        loop {
            if self.get_relearn_move_4() != 0 && self.get_relearn_move_3() == 0 {
                self.set_relearn_move_3(self.get_relearn_move_4());
                self.set_relearn_move_4(0);
            }
            if self.get_relearn_move_3() != 0 && self.get_relearn_move_2() == 0 {
                self.set_relearn_move_2(self.get_relearn_move_3());
                self.set_relearn_move_3(0);
                continue;
            }
            if self.get_relearn_move_2() != 0 && self.get_relearn_move_1() == 0 {
                self.set_relearn_move_1(self.get_relearn_move_2());
                self.set_relearn_move_2(0);
                continue;
            }
            break;
        }
    }

    fn trade<T: TrainerInfo + ?Sized>(&mut self, tr: &T, day: usize, month: usize, year: usize) {
        if self.get_is_egg() {
            if tr.get_ot() != self.get_ot_name()
                || tr.get_tid() != self.get_tid()
                || tr.get_sid() != self.get_sid()
                || tr.get_gender() != self.get_gender()
            {
                self.set_link_trade_egg(day, month, year, LINK_TRADE_6);
            }
            return;
        }

        if !self.trade_ot(tr) {
            self.trade_ht(tr);
        }
    }

    fn trade_ot<T: TrainerInfo + ?Sized>(&mut self, tr: &T) -> bool;
    fn trade_ht<T: TrainerInfo + ?Sized>(&mut self, tr: &T);

    fn max_iv(&self) -> usize {
        31
    }

    fn max_ev(&self) -> usize {
        252
    }

    fn ot_length(&self) -> usize {
        12
    }

    fn nick_length(&self) -> usize {
        12
    }
}

pub trait SuperTrain {
    fn get_super_train_bit_flags(&self) -> usize;
    fn set_super_train_bit_flags(&mut self, flags: usize);
    fn get_secret_super_training_unlocked(&self) -> bool;
    fn set_secret_super_training_unlocked(&mut self, unlocked: bool);
    fn get_secret_super_training_complete(&self) -> bool;
    fn set_secret_super_training_complete(&mut self, complete: bool);
    fn get_super_training_medal_count(&self, max_count: usize) -> usize;
}
