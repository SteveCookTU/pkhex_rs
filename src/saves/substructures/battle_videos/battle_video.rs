use crate::{PersonalInfo, Pkm, PokeGroup};

pub trait BattleVideo<I: PersonalInfo, T: Pkm<I>>: PokeGroup<I, T> {
    fn get_battle_pkms(&self) -> Vec<T>;
    fn get_generation(&self) -> usize;

    fn get_contents(&self) -> Vec<T> {
        self.get_battle_pkms()
    }
}
