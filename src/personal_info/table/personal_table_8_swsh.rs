use crate::legality;
use crate::personal_info::traits::{PersonalFormInfo, PersonalTable};
use crate::personal_info::PersonalInfo8SWSH;
use std::ops::Index;

pub struct PersonalTable8SWSH<'a> {
    table: Vec<PersonalInfo8SWSH<'a>>,
}

impl<'a> PersonalTable8SWSH<'a> {
    const SIZE: usize = PersonalInfo8SWSH::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable8SWSH<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo8SWSH::new(slice));
            ofs += Self::SIZE;
        }

        Self { table }
    }
}

impl<'a> Index<usize> for PersonalTable8SWSH<'a> {
    type Output = PersonalInfo8SWSH<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable8SWSH<'a> {
    type Output = PersonalInfo8SWSH<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable8SWSH<'a> {
    type PersonalInfoInner = PersonalInfo8SWSH<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables8::MAX_SPECIES_ID_8_R2
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
        if species > self.max_species_id() {
            return false;
        }

        let form_0 = &self.table[species as usize];
        if form_0.is_present_in_game() {
            return true;
        }

        let fc = form_0.form_count();
        for i in 1..fc {
            let entry = self.get_form_entry(species, i);
            if entry.is_present_in_game() {
                return true;
            }
        }

        false
    }

    fn is_present_in_game(&self, species: u16, form: u8) -> bool {
        if species > self.max_species_id() {
            return false;
        }

        let form_0 = &self.table[species as usize];
        if form == 0 {
            return form_0.is_present_in_game();
        }
        if !form_0.has_form(form) {
            return false;
        }

        let entry = self.get_form_entry(species, form);
        entry.is_present_in_game()
    }
}
