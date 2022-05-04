pub trait Affection {
    fn get_ot_affection(&self) -> u8;

    fn set_ot_affection(&mut self, affection: u8);

    fn get_ht_affection(&self) -> u8;

    fn set_ht_affection(&mut self, affection: u8);
}
