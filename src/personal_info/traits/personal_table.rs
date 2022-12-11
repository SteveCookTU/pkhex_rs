use crate::traits::personal_info::PersonalInfo;
use std::ops::Index;

pub trait PersonalTable<T: PersonalInfo>:
    Index<usize, Output = T> + Index<(u16, u8), Output = T>
{
    fn max_species_id(&self) -> u16;
    fn get_form_index(&self, species: u16, form: u8) -> usize;
    fn get_form_entry(&self, species: u16, form: u8) -> T;
    fn is_species_in_game(&self, species: u16) -> bool;
    fn is_present_in_game(&self, species: u16, form: u8) -> bool;
}
