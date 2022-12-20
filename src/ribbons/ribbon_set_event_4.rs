pub trait RibbonSetEvent4 {
    fn ribbon_classic(&self) -> bool;
    fn set_ribbon_classic(&mut self, val: bool);
    fn ribbon_wishing(&self) -> bool;
    fn set_ribbon_wishing(&mut self, val: bool);
    fn ribbon_premier(&self) -> bool;
    fn set_ribbon_premier(&mut self, val: bool);
    fn ribbon_event(&self) -> bool;
    fn set_ribbon_event(&mut self, val: bool);
    fn ribbon_birthday(&self) -> bool;
    fn set_ribbon_birthday(&mut self, val: bool);
    fn ribbon_special(&self) -> bool;
    fn set_ribbon_special(&mut self, val: bool);
    fn ribbon_world(&self) -> bool;
    fn set_ribbon_world(&mut self, val: bool);
    fn ribbon_champion_world(&self) -> bool;
    fn set_ribbon_champion_world(&mut self, val: bool);
    fn ribbon_souvenir(&self) -> bool;
    fn set_ribbon_souvenir(&mut self, val: bool);

    fn copy_ribbon_set_event_4(&self, dest: &mut impl RibbonSetEvent4) {
        dest.set_ribbon_classic(self.ribbon_classic());
        dest.set_ribbon_wishing(self.ribbon_wishing());
        dest.set_ribbon_premier(self.ribbon_premier());
        dest.set_ribbon_event(self.ribbon_event());
        dest.set_ribbon_birthday(self.ribbon_birthday());
        dest.set_ribbon_special(self.ribbon_special());
        dest.set_ribbon_world(self.ribbon_world());
        dest.set_ribbon_champion_world(self.ribbon_champion_world());
        dest.set_ribbon_souvenir(self.ribbon_souvenir());
    }
}
