pub trait RibbonSetCommon6 {
    fn ribbon_champion_kalos(&self) -> bool;
    fn set_ribbon_champion_kalos(&mut self, val: bool);
    fn ribbon_champion_g6_hoenn(&self) -> bool;
    fn set_ribbon_champion_g6_hoenn(&mut self, val: bool);
    fn ribbon_best_friends(&self) -> bool;
    fn set_ribbon_best_friends(&mut self, val: bool);
    fn ribbon_training(&self) -> bool;
    fn set_ribbon_training(&mut self, val: bool);
    fn ribbon_battler_skillful(&self) -> bool;
    fn set_ribbon_battler_skillful(&mut self, val: bool);
    fn ribbon_battler_expert(&self) -> bool;
    fn set_ribbon_battler_expert(&mut self, val: bool);

    fn ribbon_contest_star(&self) -> bool;
    fn set_ribbon_contest_star(&mut self, val: bool);
    fn ribbon_master_coolness(&self) -> bool;
    fn set_ribbon_master_coolness(&mut self, val: bool);
    fn ribbon_master_beauty(&self) -> bool;
    fn set_ribbon_master_beauty(&mut self, val: bool);
    fn ribbon_master_cuteness(&self) -> bool;
    fn set_ribbon_master_cuteness(&mut self, val: bool);
    fn ribbon_master_cleverness(&self) -> bool;
    fn set_ribbon_master_cleverness(&mut self, val: bool);
    fn ribbon_master_toughness(&self) -> bool;
    fn set_ribbon_master_toughness(&mut self, val: bool);

    fn has_all_contest_ribbons(&self) -> bool {
        self.ribbon_master_coolness()
            && self.ribbon_master_beauty()
            && self.ribbon_master_cuteness()
            && self.ribbon_master_cleverness()
            && self.ribbon_master_toughness()
    }

    fn copy_ribbon_set_common_6(&self, dest: &mut impl RibbonSetCommon6) {
        dest.set_ribbon_champion_kalos(self.ribbon_champion_kalos());
        dest.set_ribbon_champion_g6_hoenn(self.ribbon_champion_g6_hoenn());
        dest.set_ribbon_best_friends(self.ribbon_best_friends());
        dest.set_ribbon_training(self.ribbon_training());
        dest.set_ribbon_battler_skillful(self.ribbon_battler_skillful());
        dest.set_ribbon_battler_expert(self.ribbon_battler_expert());
        dest.set_ribbon_contest_star(self.ribbon_contest_star());
        dest.set_ribbon_master_coolness(self.ribbon_master_coolness());
        dest.set_ribbon_master_beauty(self.ribbon_master_beauty());
        dest.set_ribbon_master_cuteness(self.ribbon_master_cuteness());
        dest.set_ribbon_master_cleverness(self.ribbon_master_cleverness());
        dest.set_ribbon_master_toughness(self.ribbon_master_toughness());
    }
}
