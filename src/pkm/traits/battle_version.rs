pub trait BattleVersion {
    fn get_battle_version(&self) -> u8;

    fn set_battle_version(&mut self, battle_version: u8);

    fn adapt_to_battle_version(&mut self) {
        todo!()
    }

    fn get_min_generation(&self) -> Option<u8> {
        todo!()
    }
}
