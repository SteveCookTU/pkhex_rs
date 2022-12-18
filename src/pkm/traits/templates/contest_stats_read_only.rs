use crate::pkm::traits::ContestStats;

const CONTEST_MAX: u8 = 255;

pub trait ContestStatsReadOnly {
    fn cnt_cool(&self) -> u8;
    fn cnt_beauty(&self) -> u8;
    fn cnt_cute(&self) -> u8;
    fn cnt_smart(&self) -> u8;
    fn cnt_tough(&self) -> u8;
    fn cnt_sheen(&self) -> u8;

    fn has_contest_stats(&self) -> bool {
        self.cnt_cool() != 0
            || self.cnt_beauty() != 0
            || self.cnt_cute() != 0
            || self.cnt_smart() != 0
            || self.cnt_tough() != 0
            || self.cnt_sheen() != 0
    }

    fn is_contest_below(&self, initial: &dyn ContestStatsReadOnly) -> bool {
        !self.is_contest_above_or_equal(initial)
    }

    fn is_contest_above_or_equal(&self, initial: &dyn ContestStatsReadOnly) -> bool {
        !(self.cnt_cool() < initial.cnt_cool()
            || self.cnt_beauty() < initial.cnt_beauty()
            || self.cnt_cute() < initial.cnt_cute()
            || self.cnt_smart() < initial.cnt_smart()
            || self.cnt_tough() < initial.cnt_tough()
            || self.cnt_sheen() < initial.cnt_sheen())
    }

    fn is_contest_equal(&self, initial: &dyn ContestStatsReadOnly) -> bool {
        !(self.cnt_cool() != initial.cnt_cool()
            || self.cnt_beauty() != initial.cnt_beauty()
            || self.cnt_cute() != initial.cnt_cute()
            || self.cnt_smart() != initial.cnt_smart()
            || self.cnt_tough() != initial.cnt_tough()
            || self.cnt_sheen() != initial.cnt_sheen())
    }

    fn copy_contests_stats_to(&self, dest: &mut dyn ContestStats) {
        dest.set_cnt_cool(self.cnt_cool());
        dest.set_cnt_beauty(self.cnt_beauty());
        dest.set_cnt_cute(self.cnt_cute());
        dest.set_cnt_smart(self.cnt_smart());
        dest.set_cnt_tough(self.cnt_tough());
        dest.set_cnt_sheen(self.cnt_sheen());
    }

    fn is_any_contest_stat_max(&self) -> bool {
        self.cnt_cool() == CONTEST_MAX
            || self.cnt_beauty() == CONTEST_MAX
            || self.cnt_cute() == CONTEST_MAX
            || self.cnt_smart() == CONTEST_MAX
            || self.cnt_tough() == CONTEST_MAX
            || self.cnt_sheen() == CONTEST_MAX
    }
}
