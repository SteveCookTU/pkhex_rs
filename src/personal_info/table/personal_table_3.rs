use crate::game::enums::Species;
use crate::legality;
use crate::legality::BinLinkerAccessor;
use crate::personal_info::traits::personal_info::PersonalInfo;
use crate::personal_info::traits::PersonalTable;
use crate::personal_info::PersonalInfo3;
use std::ops::Index;

pub struct PersonalTable3<'a> {
    table: Vec<PersonalInfo3<'a>>,
}

impl<'a> PersonalTable3<'a> {
    const SIZE: usize = PersonalInfo3::SIZE;

    pub fn new(data: &'a [u8]) -> PersonalTable3<'a> {
        let count = data.len() / Self::SIZE;
        let mut table = Vec::with_capacity(count);
        let mut ofs = 0;
        for _ in 0..count {
            let slice = &data[ofs..(ofs + Self::SIZE)];
            table.push(PersonalInfo3::new(slice));
            ofs += Self::SIZE;
        }
        Self { table }
    }

    pub(crate) fn load_tables(&mut self, machine: BinLinkerAccessor, tutors: BinLinkerAccessor) {
        let table = &mut self.table;
        for i in 1..=legality::tables3::MAX_SPECIES_ID_3 {
            let entry = &mut table[i as usize];
            entry.add_tmhm(&machine[i as usize]);
            entry.add_type_tutors(&tutors[i as usize]);
        }
    }
}

impl<'a> Index<usize> for PersonalTable3<'a> {
    type Output = PersonalInfo3<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[if index < self.table.len() { index } else { 0 }]
    }
}

impl<'a> Index<(u16, u8)> for PersonalTable3<'a> {
    type Output = PersonalInfo3<'a>;

    fn index(&self, index: (u16, u8)) -> &Self::Output {
        &self.table[self.get_form_index(index.0, index.1)]
    }
}

impl<'a> PersonalTable for PersonalTable3<'a> {
    type PersonalInfoInner = PersonalInfo3<'a>;

    fn max_species_id(&self) -> u16 {
        legality::tables3::MAX_SPECIES_ID_3
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
        } else {
            form == 0
                || match species {
                    _ if species == Species::Unown as u16 => form < 28,
                    _ if species == Species::Castform as u16 => form < 4,
                    _ if species == Species::Deoxys as u16 => form < 4,
                    _ => false,
                }
        }
    }
}
