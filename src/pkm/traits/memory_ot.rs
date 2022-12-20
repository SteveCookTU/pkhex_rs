use crate::pkm::traits::templates::MemoryOTReadOnly;

pub trait MemoryOT: MemoryOTReadOnly {
    fn set_ot_memory(&mut self, memory: u8);
    fn set_ot_intensity(&mut self, intensity: u8);
    fn set_ot_feeling(&mut self, feeling: u8);
    fn set_ot_text_var(&mut self, var: u16);

    fn clear_memories_ot(&mut self) {
        self.set_ot_memory(0);
        self.set_ot_intensity(0);
        self.set_ot_feeling(0);
        self.set_ot_text_var(0);
    }
}
