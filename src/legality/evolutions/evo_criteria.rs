use crate::legality::evolutions::EvolutionType;
use crate::pkm::traits::metadata::SpeciesForm;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct EvoCriteria {
    pub species: u16,
    pub form: u8,
    pub level_up_required: u8,
    pub level_max: u8,
    pub level_min: u8,
    pub method: EvolutionType,
}

impl SpeciesForm for EvoCriteria {
    fn species(&self) -> u16 {
        self.species
    }

    fn form(&self) -> u8 {
        self.form
    }
}

impl EvoCriteria {
    pub fn requires_level_up(&self) -> bool {
        self.level_up_required != 0
    }

    pub fn inside_level_range(&self, level: u8) -> bool {
        (self.level_min..=self.level_max).contains(&level)
    }
}

impl Display for EvoCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Species{} [{},{}] via {:?}",
            self.species,
            if self.form != 0 {
                format!("-{}", self.form)
            } else {
                "".to_string()
            },
            self.level_min,
            self.level_max,
            self.method
        )
    }
}
