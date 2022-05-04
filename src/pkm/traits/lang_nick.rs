pub trait LangNick {
    fn get_nickname(&self) -> String;
    fn set_nickname(&mut self, nickname: String);
    fn get_is_nicknamed(&self) -> bool;
    fn set_is_nicknamed(&mut self, nicknamed: bool);
    fn get_language(&self) -> usize;
    fn set_language(&mut self, language: usize);
}
