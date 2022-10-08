pub trait Affection {
    fn get_ot_affection(&self) -> u8;
    fn set_ot_affection(&mut self, affection: u8);

    fn get_ht_affection(&self) -> u8 {
        0
    }
    fn set_ht_affection(&mut self, _affection: u8) {}
}

#[macro_export]
macro_rules! impl_affection {
    ($t:ty) => {
        impl Affection for $t {
            fn get_ot_affection(&self) -> u8 {
                self.ot_affection
            }

            fn get_ht_affection(&self) -> u8 {
                self.ht_affection
            }

            fn set_ot_affection(&mut self, affection: u8) {
                self.ot_affection = affection;
            }

            fn set_ht_affection(&mut self, affection: u8) {
                self.ht_affection = affection;
            }
        }
    };
}
