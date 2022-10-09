pub trait RibbonSetOnly3 {
    fn get_ribbon_count_g3_cool(&self) -> u8;
    fn set_ribbon_count_g3_cool(&mut self, count: u8);

    fn get_ribbon_count_g3_beauty(&self) -> u8;
    fn set_ribbon_count_g3_beauty(&mut self, count: u8);

    fn get_ribbon_count_g3_cute(&self) -> u8;
    fn set_ribbon_count_g3_cute(&mut self, count: u8);

    fn get_ribbon_count_g3_smart(&self) -> u8;
    fn set_ribbon_count_g3_smart(&mut self, count: u8);

    fn get_ribbon_count_g3_tough(&self) -> u8;
    fn set_ribbon_count_g3_tough(&mut self, count: u8);

    fn get_ribbon_world(&self) -> bool;
    fn set_ribbon_world(&mut self, value: bool);

    fn get_unused1(&self) -> bool;
    fn set_unused1(&mut self, value: bool);

    fn get_unused2(&self) -> bool;
    fn set_unused2(&mut self, value: bool);

    fn get_unused3(&self) -> bool;
    fn set_unused3(&mut self, value: bool);

    fn get_unused4(&self) -> bool;
    fn set_unused4(&mut self, value: bool);
}
