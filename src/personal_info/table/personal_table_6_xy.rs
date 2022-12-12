use crate::game::enums::Species;
use crate::legality;
use crate::personal_info::traits::{PersonalFormInfo, PersonalTable};
use crate::personal_info::PersonalInfo6XY;
use std::ops::Index;

pub struct PersonalTable6XY<'a> {
    table: Vec<PersonalInfo6XY<'a>>,
}

impl<'a> PersonalTable6XY<'a> {
    const SIZE: usize = PersonalInfo6XY::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable6XY<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo6XY::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }
}

impl<'a> Index<usize> for PersonalTable6XY<'a> {
    type Output = PersonalInfo6XY<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable6XY<'a> {
    type Output = PersonalInfo6XY<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable6XY<'a> {
    type PersonalInfoInner = PersonalInfo6XY<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables6::MAX_SPECIES_ID_6
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
        } else if form == 0 || self.table[species as usize].has_form(form) {
            true
        } else {
            match species {
                _ if species == Species::Unown as u16 => form < 28,
                _ if species == Species::Burmy as u16 => form < 3,
                _ if species == Species::Mothim as u16 => form < 3,
                _ if species == Species::Shellos as u16 || species == Species::Gastrodon as u16 => {
                    form == 1
                }
                _ if species == Species::Arceus as u16 => form < 17,
                _ if species == Species::Deerling as u16 || species == Species::Sawsbuck as u16 => {
                    form < 4
                }
                _ if species == Species::Genesect as u16 => form <= 4,
                _ if species == Species::Scatterbug as u16 || species == Species::Spewpa as u16 => {
                    form < 18
                }
                _ if species == Species::Vivillon as u16 => form <= 19,
                _ if species == Species::Flabébé as u16 || species == Species::Florges as u16 => {
                    form < 5
                }
                _ if species == Species::Floette as u16 => form <= 5,
                _ if species == Species::Xerneas as u16 => form == 1,
                _ => false,
            }
        }
    }
}
