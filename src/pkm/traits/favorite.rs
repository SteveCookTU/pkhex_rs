pub trait Favorite {
    fn is_favorite(&self) -> bool;
    fn set_is_favorite(&mut self, val: bool);
}
