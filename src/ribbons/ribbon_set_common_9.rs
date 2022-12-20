pub trait RibbonSetCommon9 {
    fn ribbon_champion_paldea(&self) -> bool;
    fn set_ribbon_champion_paldea(&mut self, val: bool);
    fn ribbon_once_in_a_lifetime(&self) -> bool;
    fn set_ribbon_once_in_a_lifetime(&mut self, val: bool);

    fn copy_ribbon_set_common_9(&self, dest: &mut impl RibbonSetCommon9) {
        dest.set_ribbon_champion_paldea(self.ribbon_champion_paldea());
        dest.set_ribbon_once_in_a_lifetime(self.ribbon_once_in_a_lifetime())
    }
}
