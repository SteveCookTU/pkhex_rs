use crate::game::enums::GameVersion;
use crate::legality::evolutions::EvolutionHistory;
use crate::pkm::Pkm;

pub trait BattleVersion {
    fn battle_version(&self) -> u8;
    fn set_battle_version(&mut self, version: u8);

    fn is_battle_version_valid(&self, h: &EvolutionHistory) -> bool
    where
        Self: Pkm,
    {
        match self.battle_version() {
            0 => true,
            i if [GameVersion::SW as u8, GameVersion::SH as u8].contains(&i) => {
                !(self.swsh() || self.bdsp() || self.la()) && h.has_visited_swsh()
            }
            _ => false,
        }
    }

    fn get_min_generation(&self) -> Option<u8> {
        let ver = self.battle_version();
        if ver == 0 {
            return Some(1);
        }
        let game = GameVersion::from(ver);
        if !game.is_valid_saved_version() {
            return None;
        }
        game.get_generation().filter(|&gen| gen >= 8)
    }
}
