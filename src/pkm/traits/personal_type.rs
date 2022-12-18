pub trait PersonalType {
    fn type1(&self) -> u8;
    fn type2(&self) -> u8;
    fn set_type1(&mut self, t: u8);
    fn set_type2(&mut self, t: u8);

    fn is_type(&self, t: u8) -> bool {
        self.type1() == t || self.type2() == t
    }

    fn is_both_types(&self, t1: u8, t2: u8) -> bool {
        (self.type1() == t1 && self.type2() == 2) || (self.type2() == t1 && self.type1() == t2)
    }

    fn is_valid_type_combination(&self, t1: u8, t2: u8) -> bool {
        self.type1() == t1 && self.type2() == t2
    }
}
