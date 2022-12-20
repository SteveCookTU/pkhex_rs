pub trait RibbonSetMemory6 {
    fn ribbon_count_memory_contest(&self) -> u8;
    fn set_ribbon_count_memory_contest(&mut self, count: u8);
    fn ribbon_count_memory_battle(&self) -> u8;
    fn set_ribbon_count_memory_battle(&mut self, count: u8);
    fn has_contest_memory_ribbon(&self) -> bool;
    fn set_has_contest_memory_ribbon(&mut self, has: bool);
    fn has_battle_memory_ribbon(&self) -> bool;
    fn set_has_battle_memory_ribbon(&mut self, has: bool);

    fn copy_ribbon_set_event_3(&self, dest: &mut impl RibbonSetMemory6) {
        dest.set_has_contest_memory_ribbon(self.has_contest_memory_ribbon());
        dest.set_has_battle_memory_ribbon(self.has_battle_memory_ribbon());
        dest.set_ribbon_count_memory_contest(self.ribbon_count_memory_contest());
        dest.set_ribbon_count_memory_battle(self.ribbon_count_memory_battle());
    }
}
