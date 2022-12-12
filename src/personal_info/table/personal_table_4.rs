use crate::game::enums::Species;
use crate::legality;
use crate::legality::BinLinkerAccessor;
use crate::personal_info::traits::personal_info::PersonalInfo;
use crate::personal_info::traits::{PersonalFormInfo, PersonalTable};
use crate::personal_info::PersonalInfo4;
use std::ops::Index;

pub struct PersonalTable4<'a> {
    table: Vec<PersonalInfo4<'a>>,
}

impl<'a> PersonalTable4<'a> {
    const SIZE: usize = PersonalInfo4::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable4<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo4::new(slice));
            ofs += Self::SIZE;
        }

        Self { table }
    }

    pub fn load_tables(&mut self, tutors: BinLinkerAccessor) {
        let table = &mut self.table;
        for i in 1..=legality::tables4::MAX_SPECIES_ID_4 {
            table[i as usize].add_type_tutors(&tutors[i as usize]);
        }
    }
}

impl<'a> Index<usize> for PersonalTable4<'a> {
    type Output = PersonalInfo4<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable4<'a> {
    type Output = PersonalInfo4<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable4<'a> {
    type PersonalInfoInner = PersonalInfo4<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables4::MAX_SPECIES_ID_4
    }

    fn get_form_index(&self, species: u16, form: u8) -> usize {
        if species <= self.max_species_id() {
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
        } else if form == 0 || self.table[species as usize].has_form(form) {
            true
        } else {
            match species {
                _ if species == Species::Pichu as u16 => form == 1,
                _ if species == Species::Unown as u16 => form < 28,
                _ if species == Species::Castform as u16 => form < 4,
                _ if species == Species::Burmy as u16 => form < 3,
                _ if species == Species::Mothim as u16 => form < 3,
                _ if species == Species::Cherrim as u16 => form == 1,
                _ if species == Species::Shellos as u16 || species == Species::Gastrodon as u16 => {
                    form == 1
                }
                _ if species == Species::Arceus as u16 => form < 18 && form != 9,
                _ => false,
            }
        }
    }
}
