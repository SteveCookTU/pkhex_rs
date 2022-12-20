pub trait RibbonIndex {
    fn get_ribbon(&self, index: usize) -> bool;
    fn set_ribbon(&mut self, index: usize, val: bool);
    fn get_ribbon_byte(&self, index: usize) -> usize;
}
