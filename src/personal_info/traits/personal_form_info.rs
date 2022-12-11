pub trait PersonalFormInfo {
    fn form_count(&self) -> u8;
    fn form_stats_index(&self) -> Option<usize>;
    fn has_forms(&self) -> bool {
        self.form_count() > 1
    }
    fn form_index(&self, species: u16, form: u8) -> usize {
        if !self.has_form(form) {
            species as usize
        } else {
            self.form_stats_index().unwrap_or_default() + form as usize - 1
        }
    }
    fn has_form(&self, form: u8) -> bool {
        !(form == 0
            || self.form_stats_index().unwrap_or_default() == 0
            || form >= self.form_count())
    }
    fn is_form_within_range(&self, form: u8) -> bool {
        form == 0 || form < self.form_count()
    }
}
