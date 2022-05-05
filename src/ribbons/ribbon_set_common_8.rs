pub const RIBBON_SET_NAMES_COMMON_8: [&str; 5] = [
    "RibbonChampionGalar",
    "RibbonTowerMaster",
    "RibbonMasterRank",
    "RibbonTwinklingStar",
    "RibbonPioneer",
];

pub trait RibbonSetCommon8 {
    fn get_ribbon_champion_galar(&self) -> bool;
    fn set_ribbon_champion_galar(&mut self, flag: bool);
    fn get_ribbon_tower_master(&self) -> bool;
    fn set_ribbon_tower_master(&mut self, flag: bool);
    fn get_ribbon_master_rank(&self) -> bool;
    fn set_ribbon_master_rank(&mut self, flag: bool);
    fn get_ribbon_twinkling_star(&self) -> bool;
    fn set_ribbon_twinkling_star(&mut self, flag: bool);
    fn get_ribbon_pioneer(&self) -> bool;
    fn set_ribbon_pioneer(&mut self, flag: bool);

    fn ribbon_bits(&self) -> [bool; 5] {
        [
            self.get_ribbon_champion_galar(),
            self.get_ribbon_tower_master(),
            self.get_ribbon_master_rank(),
            self.get_ribbon_twinkling_star(),
            self.get_ribbon_pioneer(),
        ]
    }
}
