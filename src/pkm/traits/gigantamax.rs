pub trait Gigantamax {
    fn get_can_gigantamax(&self) -> bool;
    fn set_can_gigantamax(&mut self, can_gigantamax: bool);
}

pub mod gigantamax_extensions {
    pub fn can_toggle_gigantamax(
        _current_species: usize,
        _current_form: usize,
        _origin_species: usize,
        _origin_form: usize,
    ) -> bool {
        todo!()
    }
}
