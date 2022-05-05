pub trait TechRecord8 {
    fn tech_record_permit_flags(&self) -> Vec<bool>;
    fn tech_record_permit_indexes(&self) -> Vec<usize>;

    fn get_move_record_flag(&self, index: usize) -> Option<bool>;
    fn set_move_record_flag(&mut self, index: usize, state: bool);
    fn get_move_record_flag_any(&self) -> bool;
}
