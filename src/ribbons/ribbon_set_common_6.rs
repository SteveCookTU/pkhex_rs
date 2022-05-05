pub const RIBBON_SET_NAMES_COMMON_6_BOOL: [&str; 11] = [
    "RibbonChampionKalos",
    "RibbonChampionG6Hoenn",
    "RibbonTraining",
    "RibbonBattlerSkillful",
    "RibbonBattlerExpert",
    "RibbonContestStar",
    "RibbonMasterCoolness",
    "RibbonMasterBeauty",
    "RibbonMasterCuteness",
    "RibbonMasterCleverness",
    "RibbonMasterToughness",
];

pub const RIBBON_SET_NAMES_COMMON_6_CONTEST: [&str; 5] = [
    "RibbonMasterCoolness",
    "RibbonMasterBeauty",
    "RibbonMasterCuteness",
    "RibbonMasterCleverness",
    "RibbonMasterToughness",
];

pub trait RibbonSetCommon6 {
    fn get_ribbon_champion_kalos(&self) -> bool;
    fn set_ribbon_champion_kalos(&mut self, flag: bool);
    fn get_ribbon_champion_g6_hoenn(&self) -> bool;
    fn set_ribbon_champion_g6_hoenn(&mut self, flag: bool);
    fn get_ribbon_best_friends(&self) -> bool;
    fn set_ribbon_best_friends(&mut self, flag: bool);
    fn get_ribbon_training(&self) -> bool;
    fn set_ribbon_training(&mut self, flag: bool);
    fn get_ribbon_battler_skillful(&self) -> bool;
    fn set_ribbon_battler_skillful(&mut self, flag: bool);
    fn get_ribbon_battler_expert(&self) -> bool;
    fn set_ribbon_battler_expert(&mut self, flag: bool);
    fn get_ribbon_contest_star(&self) -> bool;
    fn set_ribbon_contest_star(&mut self, flag: bool);
    fn get_ribbon_master_coolness(&self) -> bool;
    fn set_ribbon_master_coolness(&mut self, flag: bool);
    fn get_ribbon_master_beauty(&self) -> bool;
    fn set_ribbon_master_beauty(&mut self, flag: bool);
    fn get_ribbon_master_cuteness(&self) -> bool;
    fn set_ribbon_master_cuteness(&mut self, flag: bool);
    fn get_ribbon_master_cleverness(&self) -> bool;
    fn set_ribbon_master_cleverness(&mut self, flag: bool);
    fn get_ribbon_master_toughness(&self) -> bool;
    fn set_ribbon_master_toughness(&mut self, flag: bool);
    fn get_ribbon_count_memory_contest(&self) -> usize;
    fn set_ribbon_count_memory_contest(&mut self, value: usize);
    fn get_ribbon_count_memory_battle(&self) -> usize;
    fn set_ribbon_count_memory_battle(&mut self, value: usize);

    fn ribbon_bits(&self) -> [bool; 11] {
        [
            self.get_ribbon_champion_kalos(),
            self.get_ribbon_champion_g6_hoenn(),
            self.get_ribbon_training(),
            self.get_ribbon_battler_skillful(),
            self.get_ribbon_battler_expert(),
            self.get_ribbon_contest_star(),
            self.get_ribbon_master_coolness(),
            self.get_ribbon_master_beauty(),
            self.get_ribbon_master_cuteness(),
            self.get_ribbon_master_cleverness(),
            self.get_ribbon_master_toughness(),
        ]
    }
}
