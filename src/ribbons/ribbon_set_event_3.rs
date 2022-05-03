pub const RIBBON_SET_NAMES_EVENT_3: [&str; 6] = [
    "RibbonEarth",
    "RibbonNational",
    "RibbonCountry",
    "RibbonChampionBattle",
    "RibbonChampionRegional",
    "RibbonChampionNational",
];

pub trait RibbonSetEvent3 {
    fn get_ribbon_earth(&self) -> bool;
    fn set_ribbon_earth(&mut self, flag: bool);
    fn get_ribbon_national(&self) -> bool;
    fn set_ribbon_national(&mut self, flag: bool);
    fn get_ribbon_country(&self) -> bool;
    fn set_ribbon_country(&mut self, flag: bool);
    fn get_ribbon_champion_battle(&self) -> bool;
    fn set_ribbon_champion_battle(&mut self, flag: bool);
    fn get_ribbon_champion_regional(&self) -> bool;
    fn set_ribbon_champion_regional(&mut self, flag: bool);
    fn get_ribbon_champion_national(&self) -> bool;
    fn set_ribbon_champion_national(&mut self, flag: bool);

    fn ribbon_bits(&self) -> [bool; 6] {
        [
            self.get_ribbon_earth(),
            self.get_ribbon_national(),
            self.get_ribbon_country(),
            self.get_ribbon_champion_battle(),
            self.get_ribbon_champion_regional(),
            self.get_ribbon_champion_national(),
        ]
    }
}
