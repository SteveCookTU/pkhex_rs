mod bin_linker_accessor;
pub mod learnset;
pub mod structures;
pub mod tables;

use crate::tables::{
    MAX_SPECIES_ID_1, MAX_SPECIES_ID_2, MAX_SPECIES_ID_3, MAX_SPECIES_ID_4, MAX_SPECIES_ID_5,
    MAX_SPECIES_ID_6, MAX_SPECIES_ID_7B, MAX_SPECIES_ID_8A,
};
pub use bin_linker_accessor::*;

pub fn get_max_species_origin(generation: usize) -> Option<usize> {
    match generation {
        1 => Some(MAX_SPECIES_ID_1),
        2 => Some(MAX_SPECIES_ID_2),
        3 => Some(MAX_SPECIES_ID_3),
        4 => Some(MAX_SPECIES_ID_4),
        5 => Some(MAX_SPECIES_ID_5),
        6 => Some(MAX_SPECIES_ID_6),
        7 => Some(MAX_SPECIES_ID_7B),
        8 => Some(MAX_SPECIES_ID_8A),
        _ => None,
    }
}

pub fn get_debut_generation(species: usize) -> usize {
    match species {
        _ if species <= MAX_SPECIES_ID_1 => 1,
        _ if species <= MAX_SPECIES_ID_2 => 2,
        _ if species <= MAX_SPECIES_ID_3 => 3,
        _ if species <= MAX_SPECIES_ID_4 => 4,
        _ if species <= MAX_SPECIES_ID_5 => 5,
        _ if species <= MAX_SPECIES_ID_6 => 6,
        _ if species <= MAX_SPECIES_ID_7B => 7,
        _ if species <= MAX_SPECIES_ID_8A => 8,
        _ => 0,
    }
}
