pub trait HandlerLanguage {
    fn ht_language(&self) -> u8;
    fn set_ht_language(&mut self, lang: u8);
}
