pub trait LangNick {
    fn get_nickname(&self) -> String;
    fn get_is_nicknamed(&self) -> bool;
    fn get_language(&self) -> usize;
}
