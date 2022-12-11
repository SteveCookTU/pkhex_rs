pub trait PersonalType {
    fn type_1(&self) -> u8;
    fn type_2(&self) -> u8;

    fn is_type(&self, type_1: u8) -> bool {
        self.type_1() == type_1 || self.type_2() == type_1
    }

    fn is_both_types(&self, type_1: u8, type_2: u8) -> bool {
        (self.type_1() == type_1 && self.type_2() == type_2)
            || (self.type_1() == type_2 && self.type_2() == type_1)
    }

    fn is_valid_type_combination(&self, type_1: u8, type_2: u8) -> bool {
        self.type_1() == type_1 && self.type_2() == type_2
    }
}
