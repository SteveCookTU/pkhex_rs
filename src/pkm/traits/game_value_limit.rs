pub trait GameValueLimit {
    fn max_move_id(&self) -> u16;
    fn max_species_id(&self) -> u16;
    fn max_item_id(&self) -> u16;
    fn max_ability_id(&self) -> u16;
    fn max_ball_id(&self) -> u8;
    fn max_game_id(&self) -> u8;
    fn min_game_id(&self) -> u8 {
        0
    }
    fn max_iv(&self) -> u8;
    fn max_ev(&self) -> u8;
    fn ot_length(&self) -> u8;
    fn nick_length(&self) -> u8;
}
