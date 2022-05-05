pub trait RibbonSetAffixed {
    fn get_affixed_ribbon(&self) -> i8;
    fn set_affixed_ribbon(&mut self, i_byte: i8);
}
