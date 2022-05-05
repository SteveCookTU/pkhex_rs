pub trait HandlerLanguage {
    fn get_ht_language(&self) -> u8;
    fn set_ht_language(&mut self, language: u8);
}
