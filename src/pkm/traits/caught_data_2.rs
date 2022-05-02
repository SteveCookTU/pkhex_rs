pub trait CaughtData2 {
    fn get_caught_data(&self) -> u16;
    fn set_caught_data(&mut self, data: u16);
    fn get_met_time_of_day(&self) -> usize;
    fn set_met_time_of_day(&mut self, met_time_of_day: usize);
    fn get_met_level(&self) -> usize;
    fn set_met_level(&mut self, met_level: usize);
    fn get_ot_gender(&self) -> usize;
    fn set_ot_gender(&mut self, ot_gender: usize);
    fn get_met_location(&self) -> usize;
    fn set_met_location(&mut self, met_location: usize);
}
