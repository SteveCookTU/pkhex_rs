pub const RIBBON_SET_NAMES_COMMON_4: [&str; 3] =
    ["RibbonGorgeous", "RibbonRoyal", "RibbonGorgeousRoyal"];
pub const RIBBON_SET_NAMES_COMMON_4_ONLY: [&str; 3] =
    ["RibbonRecord", "RibbonChampionSinnoh", "RibbonLegend"];
pub const RIBBON_SET_NAMES_COMMON_4_DAILY: [&str; 7] = [
    "RibbonAlert",
    "RibbonShock",
    "RibbonDowncast",
    "RibbonCareless",
    "RibbonRelax",
    "RibbonSnooze",
    "RibbonSmile",
];

pub trait RibbonSetCommon4 {
    fn get_ribbon_champion_sinnoh(&self) -> bool;
    fn set_ribbon_champion_sinnoh(&mut self, flag: bool);
    fn get_ribbon_alert(&self) -> bool;
    fn set_ribbon_alert(&mut self, flag: bool);
    fn get_ribbon_shock(&self) -> bool;
    fn set_ribbon_shock(&mut self, flag: bool);
    fn get_ribbon_downcast(&self) -> bool;
    fn set_ribbon_downcast(&mut self, flag: bool);
    fn get_ribbon_careless(&self) -> bool;
    fn set_ribbon_careless(&mut self, flag: bool);
    fn get_ribbon_relax(&self) -> bool;
    fn set_ribbon_relax(&mut self, flag: bool);
    fn get_ribbon_snooze(&self) -> bool;
    fn set_ribbon_snooze(&mut self, flag: bool);
    fn get_ribbon_smile(&self) -> bool;
    fn set_ribbon_smile(&mut self, flag: bool);
    fn get_ribbon_gorgeous(&self) -> bool;
    fn set_ribbon_gorgeous(&mut self, flag: bool);
    fn get_ribbon_royal(&self) -> bool;
    fn set_ribbon_royal(&mut self, flag: bool);
    fn get_ribbon_gorgeous_royal(&self) -> bool;
    fn set_ribbon_gorgeous_royal(&mut self, flag: bool);
    fn get_ribbon_footprint(&self) -> bool;
    fn set_ribbon_footprint(&mut self, flag: bool);
    fn get_ribbon_record(&self) -> bool;
    fn set_ribbon_record(&mut self, flag: bool);
    fn get_ribbon_legend(&self) -> bool;
    fn set_ribbon_legend(&mut self, flag: bool);

    fn ribbon_bits_cosmetic(&self) -> [bool; 3] {
        [
            self.get_ribbon_gorgeous(),
            self.get_ribbon_royal(),
            self.get_ribbon_gorgeous_royal(),
        ]
    }

    fn ribbon_bits_only(&self) -> [bool; 3] {
        [
            self.get_ribbon_record(),
            self.get_ribbon_champion_sinnoh(),
            self.get_ribbon_legend(),
        ]
    }

    fn ribbon_bits_daily(&self) -> [bool; 7] {
        [
            self.get_ribbon_alert(),
            self.get_ribbon_shock(),
            self.get_ribbon_downcast(),
            self.get_ribbon_careless(),
            self.get_ribbon_relax(),
            self.get_ribbon_snooze(),
            self.get_ribbon_smile(),
        ]
    }
}
