pub const RIBBON_SET_NAMES_EVENT_4: [&str; 9] = [
    "RibbonClassic",
    "RibbonWishing",
    "RibbonPremier",
    "RibbonEvent",
    "RibbonBirthday",
    "RibbonSpecial",
    "RibbonWorld",
    "RibbonChampionWorld",
    "RibbonSouvenir",
];

pub trait RibbonSetEvent4 {
    fn get_ribbon_classic(&self) -> bool;
    fn set_ribbon_classic(&mut self, flag: bool);
    fn get_ribbon_wishing(&self) -> bool;
    fn set_ribbon_wishing(&mut self, flag: bool);
    fn get_ribbon_premier(&self) -> bool;
    fn set_ribbon_premier(&mut self, flag: bool);
    fn get_ribbon_event(&self) -> bool;
    fn set_ribbon_event(&mut self, flag: bool);
    fn get_ribbon_birthday(&self) -> bool;
    fn set_ribbon_birthday(&mut self, flag: bool);
    fn get_ribbon_special(&self) -> bool;
    fn set_ribbon_special(&mut self, flag: bool);
    fn get_ribbon_world(&self) -> bool;
    fn set_ribbon_world(&mut self, flag: bool);
    fn get_ribbon_champion_world(&self) -> bool;
    fn set_ribbon_champion_world(&mut self, flag: bool);
    fn get_ribbon_souvenir(&self) -> bool;
    fn set_ribbon_souvenir(&mut self, flag: bool);

    fn ribbon_bits(&self) -> [bool; 9] {
        [
            self.get_ribbon_classic(),
            self.get_ribbon_wishing(),
            self.get_ribbon_premier(),
            self.get_ribbon_event(),
            self.get_ribbon_birthday(),
            self.get_ribbon_special(),
            self.get_ribbon_world(),
            self.get_ribbon_champion_world(),
            self.get_ribbon_souvenir(),
        ]
    }
}
