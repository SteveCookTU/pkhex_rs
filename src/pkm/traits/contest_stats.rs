const CONTEST_MAX: u8 = 255;

pub trait ContestStats {
    fn get_cnt_cool(&self) -> u8;
    fn get_cnt_beauty(&self) -> u8;
    fn get_cnt_cute(&self) -> u8;
    fn get_cnt_smart(&self) -> u8;
    fn get_cnt_tough(&self) -> u8;
    fn get_cnt_sheen(&self) -> u8;

    fn has_contest_stats(&self) -> bool {
        self.get_cnt_cool() != 0
            || self.get_cnt_beauty() != 0
            || self.get_cnt_cute() != 0
            || self.get_cnt_smart() != 0
            || self.get_cnt_tough() != 0
            || self.get_cnt_sheen() != 0
    }

    fn is_contest_below<T: ContestStats>(&self, initial: &T) -> bool {
        !self.is_contest_above_or_equal(initial)
    }

    fn is_contest_above_or_equal<T: ContestStats>(&self, initial: &T) -> bool {
        !(self.get_cnt_cool() < initial.get_cnt_cool()
            || self.get_cnt_beauty() < initial.get_cnt_beauty()
            || self.get_cnt_cute() < initial.get_cnt_cute()
            || self.get_cnt_smart() < initial.get_cnt_smart()
            || self.get_cnt_tough() < initial.get_cnt_tough()
            || self.get_cnt_sheen() < initial.get_cnt_sheen())
    }

    fn is_contest_equal<T: ContestStats>(&self, initial: &T) -> bool {
        !(self.get_cnt_cool() != initial.get_cnt_cool()
            || self.get_cnt_beauty() != initial.get_cnt_beauty()
            || self.get_cnt_cute() != initial.get_cnt_cute()
            || self.get_cnt_smart() != initial.get_cnt_smart()
            || self.get_cnt_tough() != initial.get_cnt_tough()
            || self.get_cnt_sheen() != initial.get_cnt_sheen())
    }

    fn copy_contest_stats_to<T: ContestStatsMutable>(&self, dest: &mut T) {
        dest.set_cnt_cool(self.get_cnt_cool());
        dest.set_cnt_beauty(self.get_cnt_beauty());
        dest.set_cnt_cute(self.get_cnt_cute());
        dest.set_cnt_smart(self.get_cnt_smart());
        dest.set_cnt_tough(self.get_cnt_tough());
        dest.set_cnt_sheen(self.get_cnt_sheen());
    }

    fn is_any_contest_stat_max(&self) -> bool {
        self.get_cnt_cool() == CONTEST_MAX
            || self.get_cnt_beauty() == CONTEST_MAX
            || self.get_cnt_cute() == CONTEST_MAX
            || self.get_cnt_smart() == CONTEST_MAX
            || self.get_cnt_tough() == CONTEST_MAX
    }
}

pub trait ContestStatsMutable {
    fn set_cnt_cool(&mut self, cnt: u8);
    fn set_cnt_beauty(&mut self, cnt: u8);
    fn set_cnt_cute(&mut self, cnt: u8);
    fn set_cnt_smart(&mut self, cnt: u8);
    fn set_cnt_tough(&mut self, cnt: u8);
    fn set_cnt_sheen(&mut self, cnt: u8);

    fn set_all_contest_stats_to(&mut self, value: u8, sheen: u8) {
        self.set_cnt_cool(value);
        self.set_cnt_beauty(value);
        self.set_cnt_cute(value);
        self.set_cnt_smart(value);
        self.set_cnt_tough(value);
        self.set_cnt_sheen(sheen);
    }
}
