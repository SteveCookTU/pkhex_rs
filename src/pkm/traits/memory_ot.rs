pub trait MemoryOT {
    fn get_ot_memory(&self) -> u8;
    fn set_ot_memory(&mut self, memory: u8);
    fn get_ot_intensity(&self) -> u8;
    fn set_ot_intensity(&mut self, memory: u8);
    fn get_ot_feeling(&self) -> u8;
    fn set_ot_feeling(&mut self, memory: u8);
    fn get_ot_text_var(&self) -> u16;
    fn set_ot_text_var(&mut self, var: u16);

    fn clear_memories_ot(&mut self) {
        self.set_ot_text_var(0);
        self.set_ot_memory(0);
        self.set_ot_feeling(0);
        self.set_ot_intensity(0);
    }
}
