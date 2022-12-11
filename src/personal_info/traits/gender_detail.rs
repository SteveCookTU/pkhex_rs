use rand::{thread_rng, Rng};

pub const RATIO_MAGIC_GENDERLESS: u8 = 255;
pub const RATIO_MAGIC_FEMALE: u8 = 254;
pub const RATIO_MAGIC_MALE: u8 = 0;

pub fn is_single_gender(gt: u8) -> bool {
    gt.wrapping_sub(1) >= 253
}

pub trait GenderDetail {
    fn gender(&self) -> u8;
    fn is_dual_gender(&self) -> bool {
        self.gender().wrapping_sub(1) < 253
    }
    fn genderless(&self) -> bool {
        self.gender() == RATIO_MAGIC_GENDERLESS
    }
    fn only_female(&self) -> bool {
        self.gender() == RATIO_MAGIC_FEMALE
    }
    fn only_male(&self) -> bool {
        self.gender() == RATIO_MAGIC_MALE
    }

    fn random_gender(&self) -> u8 {
        if self.genderless() {
            2
        } else if self.only_female() {
            1
        } else if self.only_male() {
            0
        } else {
            thread_rng().gen_range(0..2)
        }
    }

    fn fixed_gender(&self) -> Option<u8> {
        if self.genderless() {
            Some(2)
        } else if self.only_female() {
            Some(1)
        } else if self.only_male() {
            Some(0)
        } else {
            None
        }
    }
}
