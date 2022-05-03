pub trait SpeciesForm {
    fn get_species(&self) -> usize;
    fn set_species(&mut self, species: usize);
    fn get_form(&self) -> usize;
    fn set_form(&mut self, form: usize);
}
