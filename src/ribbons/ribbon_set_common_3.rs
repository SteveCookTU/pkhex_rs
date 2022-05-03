pub const RIBBON_SET_NAMES_COMMON_3: [&str; 3] =
    ["RibbonChampionG3", "RibbonArtist", "RibbonEffort"];

pub trait RibbonSetCommon3 {
    fn get_ribbon_champion_g3(&self) -> bool;
    fn set_ribbon_champion_g3(&mut self, flag: bool);
    fn get_ribbon_artist(&self) -> bool;
    fn set_ribbon_artist(&mut self, flag: bool);
    fn get_ribbon_effort(&self) -> bool;
    fn set_ribbon_effort(&mut self, flag: bool);

    fn ribbon_bits(&self) -> [bool; 3] {
        [
            self.get_ribbon_champion_g3(),
            self.get_ribbon_artist(),
            self.get_ribbon_effort(),
        ]
    }
}
