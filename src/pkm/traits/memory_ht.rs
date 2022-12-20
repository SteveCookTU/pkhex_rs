use crate::pkm::traits::templates::MemoryHTReadOnly;

pub trait MemoryHT: MemoryHTReadOnly {
    fn set_ht_memory(&mut self, memory: u8);
    fn set_ht_intensity(&mut self, intensity: u8);
    fn set_ht_feeling(&mut self, feeling: u8);
    fn set_ht_text_var(&mut self, var: u16);

    fn clear_memories_ht(&mut self) {
        self.set_ht_memory(0);
        self.set_ht_intensity(0);
        self.set_ht_feeling(0);
        self.set_ht_text_var(0);
    }
}
