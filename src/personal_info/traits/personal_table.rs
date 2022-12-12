use std::ops::Index;
use crate::personal_info::traits::personal_info::PersonalInfo;

pub trait PersonalTable:
    Index<usize, Output = Self::PersonalInfoInner> + Index<(u16, u8), Output = Self::PersonalInfoInner>
{
    type PersonalInfoInner: PersonalInfo;

    fn max_species_id(&self) -> u16;
    fn get_form_index(&self, species: u16, form: u8) -> usize;
    fn get_form_entry(&self, species: u16, form: u8) -> &Self::PersonalInfoInner;
    fn is_species_in_game(&self, species: u16) -> bool;
    fn is_present_in_game(&self, species: u16, form: u8) -> bool;
}
