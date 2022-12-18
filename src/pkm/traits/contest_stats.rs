use crate::pkm::traits::templates::ContestStatsReadOnly;

pub trait ContestStats: ContestStatsReadOnly {
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
