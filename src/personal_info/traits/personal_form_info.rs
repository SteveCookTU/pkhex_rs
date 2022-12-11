pub trait PersonalFormInfo {
    fn form_count(&self) -> u8;
    fn form_stats_index(&self) -> usize;
    fn has_forms(&self) -> bool;
    fn form_index(&self, species: u16, form: u8) -> usize;
    fn has_form(&self, form: u8) -> bool;
    fn is_form_within_range(&self, form: u8) -> bool;
}
