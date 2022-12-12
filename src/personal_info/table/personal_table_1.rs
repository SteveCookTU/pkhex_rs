use crate::personal_info::traits::{PersonalTable, PersonalType};
use crate::{legality, personal_info::PersonalInfo1};
use std::ops::Index;

pub struct PersonalTable1<'a> {
    table: Vec<PersonalInfo1<'a>>,
}

impl<'a> PersonalTable1<'a> {
    const SIZE: usize = PersonalInfo1::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable1<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(Self::SIZE + ofs)];
            table.push(PersonalInfo1::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }

    pub fn is_valid_type_combination(&self, type_1: u8, type_2: u8) -> bool {
        for i in 1..self.max_species_id() {
            let pi = &self.table[i as usize];
            if pi.is_valid_type_combination(type_1, type_2) {
                return true;
            }
        }
        false
    }
}

impl<'a> Index<usize> for PersonalTable1<'a> {
    type Output = PersonalInfo1<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable1<'a> {
    type Output = PersonalInfo1<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable1<'a> {
    type PersonalInfoInner = PersonalInfo1<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables1::MAX_SPECIES_ID_1
    }

    fn get_form_index(&self, species: u16, _form: u8) -> usize {
        if self.is_species_in_game(species) {
            species as usize
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
        form == 0 && self.is_species_in_game(species)
    }
}
