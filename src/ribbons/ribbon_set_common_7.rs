pub trait RibbonSetCommon7 {
    fn ribbon_champion_alola(&self) -> bool;
    fn set_ribbon_champion_alola(&mut self, val: bool);
    fn ribbon_battle_royale(&self) -> bool;
    fn set_ribbon_battle_royale(&mut self, val: bool);
    fn ribbon_battle_tree_great(&self) -> bool;
    fn set_ribbon_battle_tree_great(&mut self, val: bool);
    fn ribbon_battle_tree_master(&self) -> bool;
    fn set_ribbon_battle_tree_master(&mut self, val: bool);

    fn copy_ribbon_set_common_6(&self, dest: &mut impl RibbonSetCommon7) {
        dest.set_ribbon_champion_alola(self.ribbon_champion_alola());
        dest.set_ribbon_battle_royale(self.ribbon_battle_royale());
        dest.set_ribbon_battle_tree_great(self.ribbon_battle_tree_great());
        dest.set_ribbon_battle_tree_master(self.ribbon_battle_tree_master());
    }
}
