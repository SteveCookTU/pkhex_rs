use crate::string_converter_12::G1_TERMINATOR_CODE;
use crate::{GBPkml, PersonalInfo, PokeListType};
use std::ops::{Index, IndexMut};

pub trait PokeList<I: PersonalInfo, T: GBPkml<I>>: Index<usize> + IndexMut<usize> {
    fn pokemon(&self) -> &Vec<T>;

    fn get_count(&self) -> u8;

    fn set_count(&mut self, count: u8);

    fn new(d: Option<Vec<u8>>, c: PokeListType, jp: bool) -> Self;

    fn new_empty(c: PokeListType, jp: bool) -> Self;

    fn new_from_pk(pk: T) -> Self;

    fn get_entry_size(&self) -> usize;

    fn is_format_party(&self) -> bool;

    fn is_capacity_format(&self, capacity: PokeListType) -> bool;

    fn get_data_size(&self, c: PokeListType, jp: bool, entry_size: usize) -> usize;

    fn get_species_box_identifier(&self, pk: T) -> u8;

    fn get_entry_from_info(&self, dat: Vec<u8>, ot_name: &[u8], nick: &[u8], egg: bool) -> T;

    fn write(&mut self) -> Vec<u8>;
}
