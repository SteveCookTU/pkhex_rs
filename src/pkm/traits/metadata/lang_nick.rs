pub trait LangNick {
    fn nickname(&self) -> String;
    fn is_nicknamed(&self) -> bool;
    fn language(&self) -> u8;
}
