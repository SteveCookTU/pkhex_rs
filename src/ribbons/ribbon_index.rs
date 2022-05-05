pub trait RibbonIndex {
    fn get_ribbon(&self, index: usize) -> Option<bool>;

    fn set_ribbon(&mut self, index: usize, flag: bool);

    fn get_ribbon_byte(&self, index: usize) -> Option<usize>;
}
