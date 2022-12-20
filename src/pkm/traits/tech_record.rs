pub trait TechRecord {
    fn tech_record_permit_flags(&self) -> Vec<bool>;
    fn tech_record_permit_indexes(&self) -> Vec<u16>;
    fn record_count_total(&self) -> u16;
    fn record_count_used(&self) -> u16;
    fn get_move_record_flag(&self, index: u16) -> bool;
    fn set_move_record_flag(&mut self, index: u16, value: Option<bool>);
    fn get_move_record_flag_any(&self) -> bool;
}
