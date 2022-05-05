pub const RIBBON_SET_NAMES_COMMON_7: [&str; 4] = [
    "RibbonChampionAlola",
    "RibbonBattleRoyale",
    "RibbonBattleTreeGreat",
    "RibbonBattleTreeMaster",
];

pub trait RibbonSetCommon7 {
    fn get_ribbon_champion_alola(&self) -> bool;
    fn set_ribbon_champion_alola(&mut self, flag: bool);
    fn get_ribbon_battle_royale(&self) -> bool;
    fn set_ribbon_battle_royale(&mut self, flag: bool);
    fn get_ribbon_battle_tree_great(&self) -> bool;
    fn set_ribbon_battle_tree_great(&mut self, flag: bool);
    fn get_ribbon_battle_tree_master(&self) -> bool;
    fn set_ribbon_battle_tree_master(&mut self, flag: bool);

    fn ribbon_bits(&self) -> [bool; 4] {
        [
            self.get_ribbon_champion_alola(),
            self.get_ribbon_battle_royale(),
            self.get_ribbon_battle_tree_great(),
            self.get_ribbon_battle_tree_master(),
        ]
    }
}
