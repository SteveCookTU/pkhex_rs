pub trait RibbonSetCommon4 {
    fn ribbon_champion_sinnoh(&self) -> bool;
    fn set_ribbon_champion_sinnoh(&mut self, val: bool);
    fn ribbon_alert(&self) -> bool;
    fn set_ribbon_alert(&mut self, val: bool);
    fn ribbon_shock(&self) -> bool;
    fn set_ribbon_shock(&mut self, val: bool);
    fn ribbon_downcast(&self) -> bool;
    fn set_ribbon_downcast(&mut self, val: bool);
    fn ribbon_careless(&self) -> bool;
    fn set_ribbon_careless(&mut self, val: bool);
    fn ribbon_relax(&self) -> bool;
    fn set_ribbon_relax(&mut self, val: bool);
    fn ribbon_snooze(&self) -> bool;
    fn set_ribbon_snooze(&mut self, val: bool);
    fn ribbon_smile(&self) -> bool;
    fn set_ribbon_smile(&mut self, val: bool);
    fn ribbon_gorgeous(&self) -> bool;
    fn set_ribbon_gorgeous(&mut self, val: bool);
    fn ribbon_royal(&self) -> bool;
    fn set_ribbon_royal(&mut self, val: bool);
    fn ribbon_gorgeous_royal(&self) -> bool;
    fn set_ribbon_gorgeous_royal(&mut self, val: bool);
    fn ribbon_footprint(&self) -> bool;
    fn set_ribbon_footprint(&mut self, val: bool);
    fn ribbon_record(&self) -> bool;
    fn set_ribbon_record(&mut self, val: bool);
    fn ribbon_legend(&self) -> bool;
    fn set_ribbon_legend(&mut self, val: bool);

    fn copy_ribbon_set_common_4(&self, dest: &mut impl RibbonSetCommon4) {
        dest.set_ribbon_champion_sinnoh(self.ribbon_champion_sinnoh());
        dest.set_ribbon_alert(self.ribbon_alert());
        dest.set_ribbon_shock(self.ribbon_shock());
        dest.set_ribbon_downcast(self.ribbon_downcast());
        dest.set_ribbon_careless(self.ribbon_careless());
        dest.set_ribbon_relax(self.ribbon_relax());
        dest.set_ribbon_snooze(self.ribbon_snooze());
        dest.set_ribbon_smile(self.ribbon_smile());
        dest.set_ribbon_gorgeous(self.ribbon_gorgeous());
        dest.set_ribbon_royal(self.ribbon_royal());
        dest.set_ribbon_gorgeous_royal(self.ribbon_gorgeous_royal());
        dest.set_ribbon_footprint(self.ribbon_footprint());
        dest.set_ribbon_record(self.ribbon_record());
        dest.set_ribbon_legend(self.ribbon_legend());
    }
}
