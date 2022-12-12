use crate::game::enums::Species;
use crate::legality;
use crate::personal_info::traits::PersonalTable;
use crate::personal_info::PersonalInfo2;
use std::ops::Index;

pub struct PersonalTable2<'a> {
    table: Vec<PersonalInfo2<'a>>,
}

impl<'a> PersonalTable2<'a> {
    const SIZE: usize = PersonalInfo2::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable2<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo2::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }
}

impl<'a> Index<usize> for PersonalTable2<'a> {
    type Output = PersonalInfo2<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable2<'a> {
    type Output = PersonalInfo2<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable2<'a> {
    type PersonalInfoInner = PersonalInfo2<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables2::MAX_SPECIES_ID_2
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
        if !self.is_species_in_game(species) {
            false
        } else if form == 0 {
            true
        } else {
            species == Species::Unown as u16 && form < 26
        }
    }
}
