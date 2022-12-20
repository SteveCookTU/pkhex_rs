pub trait RibbonSetEvent3 {
    fn ribbon_earth(&self) -> bool;
    fn set_ribbon_earth(&mut self, val: bool);
    fn ribbon_national(&self) -> bool;
    fn set_ribbon_national(&mut self, val: bool);
    fn ribbon_country(&self) -> bool;
    fn set_ribbon_country(&mut self, val: bool);
    fn ribbon_champion_battle(&self) -> bool;
    fn set_ribbon_champion_battle(&mut self, val: bool);
    fn ribbon_champion_regional(&self) -> bool;
    fn set_ribbon_champion_regional(&mut self, val: bool);
    fn ribbon_champion_national(&self) -> bool;
    fn set_ribbon_champion_national(&mut self, val: bool);

    fn copy_ribbon_set_event_3(&self, dest: &mut impl RibbonSetEvent3) {
        dest.set_ribbon_earth(self.ribbon_earth());
        dest.set_ribbon_national(self.ribbon_national());
        dest.set_ribbon_country(self.ribbon_country());
        dest.set_ribbon_champion_battle(self.ribbon_champion_battle());
        dest.set_ribbon_champion_regional(self.ribbon_champion_regional());
        dest.set_ribbon_champion_national(self.ribbon_champion_national());
    }
}
