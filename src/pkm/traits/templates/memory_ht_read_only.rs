pub trait MemoryHTReadOnly {
    fn ht_memory(&self) -> u8;
    fn ht_intensity(&self) -> u8;
    fn ht_feeling(&self) -> u8;
    fn ht_text_var(&self) -> u16;
}
