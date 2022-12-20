pub trait RibbonSetCommon8 {
    fn ribbon_champion_galar(&self) -> bool;
    fn set_ribbon_champion_galar(&mut self, val: bool);
    fn ribbon_tower_master(&self) -> bool;
    fn set_ribbon_tower_master(&mut self, val: bool);
    fn ribbon_master_rank(&self) -> bool;
    fn set_ribbon_master_rank(&mut self, val: bool);
    fn ribbon_twinkling_star(&self) -> bool;
    fn set_ribbon_twinkling_star(&mut self, val: bool);
    fn ribbon_hisui(&self) -> bool;
    fn set_ribbon_hisui(&mut self, val: bool);

    fn copy_ribbon_set_common_6(&self, dest: &mut impl RibbonSetCommon8) {
        dest.set_ribbon_champion_galar(self.ribbon_champion_galar());
        dest.set_ribbon_tower_master(self.ribbon_tower_master());
        dest.set_ribbon_master_rank(self.ribbon_master_rank());
        dest.set_ribbon_twinkling_star(self.ribbon_twinkling_star());
        dest.set_ribbon_hisui(self.ribbon_hisui());
    }
}
