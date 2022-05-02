pub trait Favorite {
    fn get_favorite(&self) -> bool;
    fn set_favorite(&mut self, is_favorite: bool);
}
