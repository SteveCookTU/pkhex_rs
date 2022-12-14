pub trait GameValueLimit {
    fn max_species_id(&self) -> u16;
    fn max_move_id(&self) -> u16;
    fn max_item_id(&self) -> u16;
    fn max_ability_id(&self) -> u16;
    fn max_ball_id(&self) -> u16;
    fn max_game_id(&self) -> u8;
    fn min_game_id(&self) -> u8;
    fn max_iv(&self) -> u8;
    fn max_ev(&self) -> u8;
    fn max_string_length_ot(&self) -> usize;
    fn max_string_length_nickname(&self) -> usize;
}
