use rand::{thread_rng, Rng};

pub trait GenderDetail {
    fn is_dual_gender(&self) -> bool;
    fn genderless(&self) -> bool;
    fn only_female(&self) -> bool;
    fn only_male(&self) -> bool;

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
