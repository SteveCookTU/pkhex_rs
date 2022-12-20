pub trait MemoryOTReadOnly {
    fn ot_memory(&self) -> u8;
    fn ot_intensity(&self) -> u8;
    fn ot_feeling(&self) -> u8;
    fn ot_text_var(&self) -> u16;
}
