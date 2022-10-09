pub trait RibbonSetCommon3 {
    fn get_ribbon_champion_g3(&self) -> bool;
    fn set_ribbon_champion_g3(&mut self, value: bool);

    fn get_ribbon_artist(&self) -> bool;
    fn set_ribbon_artist(&mut self, value: bool);

    fn get_ribbon_effort(&self) -> bool;
    fn set_ribbon_effort(&mut self, value: bool);
}
