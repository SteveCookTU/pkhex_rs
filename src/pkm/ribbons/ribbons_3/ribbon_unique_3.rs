pub trait RibbonSetUnique3 {
    fn get_ribbon_winning(&self) -> bool;
    fn set_ribbon_winning(&mut self, value: bool);

    fn get_ribbon_victory(&self) -> bool;
    fn set_ribbon_victory(&mut self, value: bool);
}
