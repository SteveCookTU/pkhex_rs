pub trait PersonalEgg {
    fn egg_group_1(&self) -> u8;
    fn egg_group_2(&self) -> u8;

    fn is_egg_group(&self, group: u8) -> bool {
        self.egg_group_1() == group || self.egg_group_2() == group
    }
}
