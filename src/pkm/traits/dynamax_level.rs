pub trait DynamaxLevel {
    fn get_dynamax_level(&self) -> u8;
    fn set_dynamax_level(&mut self, dynamax_level: u8);
}

fn pkm_can_have_dynamax_level() -> bool {
    todo!()
}

fn species_can_have_dynamax_level(_species: usize) -> bool {
    todo!()
}
