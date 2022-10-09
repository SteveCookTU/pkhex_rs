pub trait RibbonSetEvent3 {
    fn get_ribbon_earth(&self) -> bool;
    fn set_ribbon_earth(&mut self, value: bool);

    fn get_ribbon_national(&self) -> bool;
    fn set_ribbon_national(&mut self, value: bool);

    fn get_ribbon_country(&self) -> bool;
    fn set_ribbon_country(&mut self, value: bool);

    fn get_ribbon_champion_battle(&self) -> bool;
    fn set_ribbon_champion_battle(&mut self, value: bool);

    fn get_ribbon_champion_regional(&self) -> bool;
    fn set_ribbon_champion_regional(&mut self, value: bool);

    fn get_ribbon_champion_national(&self) -> bool;
    fn set_ribbon_champion_national(&mut self, value: bool);
}
