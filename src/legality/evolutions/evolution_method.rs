use std::fmt::{Display, Formatter};
use crate::{EvolutionType, PersonalInfo, Pkm};
use crate::game_strings::SPECIES_EN;

pub struct EvolutionMethod {
    method: usize,
    species: usize,
    argument: usize,
    level: u8,
    form: Option<usize>,
    requires_level_up: bool
}

impl EvolutionMethod {
    pub fn new(method: usize, species: usize, argument: usize, level: u8, form: Option<usize>) -> Self {
        Self {
            method,
            species,
            argument,
            level,
            form,
            requires_level_up: false
        }
    }

    pub fn get_destination_form(&self, form: usize) -> usize {
        if self.method == EvolutionType::LevelUpFormFemale1 as usize {
            1
        } else {
            self.form.unwrap_or(form)
        }
    }
}

impl Display for EvolutionMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} [{}] @ {}{}", SPECIES_EN[self.species], self.form.unwrap_or(0), self.argument, self.level, if self.requires_level_up { "X" } else { "" })
    }
}
