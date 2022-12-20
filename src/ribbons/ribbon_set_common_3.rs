pub trait RibbonSetCommon3 {
    fn ribbon_champion_g3(&self) -> bool;
    fn set_ribbon_champion_g3(&mut self, val: bool);
    fn ribbon_artist(&self) -> bool;
    fn set_ribbon_artist(&mut self, val: bool);
    fn ribbon_effort(&self) -> bool;
    fn set_ribbon_effort(&mut self, val: bool);

    fn copy_ribbon_set_common_3(&self, dest: &mut impl RibbonSetCommon3) {
        dest.set_ribbon_champion_g3(self.ribbon_champion_g3());
        dest.set_ribbon_artist(self.ribbon_artist());
        dest.set_ribbon_effort(self.ribbon_effort());
    }
}
