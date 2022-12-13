use crate::legality;
use crate::personal_info::traits::{PersonalFormInfo, PersonalTable};
use crate::personal_info::PersonalInfo8BDSP;
use std::ops::Index;

pub struct PersonalTable8BDSP<'a> {
    table: Vec<PersonalInfo8BDSP<'a>>,
}

impl<'a> PersonalTable8BDSP<'a> {
    const SIZE: usize = PersonalInfo8BDSP::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable8BDSP<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo8BDSP::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }
}

impl<'a> Index<usize> for PersonalTable8BDSP<'a> {
    type Output = PersonalInfo8BDSP<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable8BDSP<'a> {
    type Output = PersonalInfo8BDSP<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable8BDSP<'a> {
    type PersonalInfoInner = PersonalInfo8BDSP<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables8bs::MAX_SPECIES_ID_8B
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
    }

    fn is_present_in_game(&self, species: u16, form: u8) -> bool {
        if !self.is_species_in_game(species) {
            false
        } else {
            form == 0 || self.table[species as usize].has_form(form)
        }
    }
}
