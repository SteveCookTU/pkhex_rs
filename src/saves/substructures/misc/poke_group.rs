use crate::{PersonalInfo, Pkm};

pub trait PokeGroup<I: PersonalInfo, T: Pkm<I>> {
    fn get_contents(&self) -> Vec<T>;
}
