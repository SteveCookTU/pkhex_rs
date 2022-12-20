pub trait RibbonSetAffixed {
    fn affixed_ribbon(&self) -> Option<u8>;
    fn set_affixed_ribbon(&mut self, ribbon: Option<u8>);
}
