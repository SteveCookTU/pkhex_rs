pub trait GameValueLimit {
    fn get_max_move_id(&self) -> usize;
    fn get_max_species_id(&self) -> usize;
    fn get_max_item_id(&self) -> usize;
    fn get_max_ability_id(&self) -> usize;
    fn get_max_ball_id(&self) -> usize;
    fn get_max_game_id(&self) -> usize;
    fn get_min_game_id(&self) -> usize;
    fn get_max_iv(&self) -> usize;
    fn get_max_ev(&self) -> usize;
    fn get_ot_length(&self) -> usize;
    fn get_nick_length(&self) -> usize;
}
