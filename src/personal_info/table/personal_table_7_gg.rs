use crate::game::enums::Species;
use crate::legality;
use crate::personal_info::traits::{PersonalFormInfo, PersonalTable};
use crate::personal_info::PersonalInfo7GG;
use std::ops::Index;

pub struct PersonalTable7GG<'a> {
    table: Vec<PersonalInfo7GG<'a>>,
}

impl<'a> PersonalTable7GG<'a> {
    const SIZE: usize = PersonalInfo7GG::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable7GG<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo7GG::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }
}

impl<'a> Index<usize> for PersonalTable7GG<'a> {
    type Output = PersonalInfo7GG<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable7GG<'a> {
    type Output = PersonalInfo7GG<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable7GG<'a> {
    type PersonalInfoInner = PersonalInfo7GG<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables7b::MAX_SPECIES_ID_7B
    }

    fn get_form_index(&self, species: u16, form: u8) -> usize {
        if self.is_species_in_game(species) {
            self.table[species as usize].form_index(species, form)
        } else {
            0
        }
    }

    fn get_form_entry(&self, species: u16, form: u8) -> &Self::PersonalInfoInner {
        &self.table[self.get_form_index(species, form)]
    }

    fn is_species_in_game(&self, species: u16) -> bool {
        species <= self.max_species_id()
            || species == Species::Meltan as u16
            || species == Species::Melmetal as u16
    }

    fn is_present_in_game(&self, species: u16, form: u8) -> bool {
        if !self.is_species_in_game(species) {
            false
        } else if form == 0 {
            true
        } else if species == Species::Pikachu as u16 {
            form == 8
        } else {
            self.table[species as usize].has_form(form)
        }
    }
}
