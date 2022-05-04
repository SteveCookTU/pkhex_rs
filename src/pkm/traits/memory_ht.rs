use crate::{memory_context_6, memory_context_8};

pub trait MemoryHT {
    fn get_ht_memory(&self) -> u8;
    fn set_ht_memory(&mut self, memory: u8);
    fn get_ht_intensity(&self) -> u8;
    fn set_ht_intensity(&mut self, intensity: u8);
    fn get_ht_feeling(&self) -> u8;
    fn set_ht_feeling(&mut self, feeling: u8);
    fn get_ht_text_var(&self) -> u16;
    fn set_ht_text_var(&mut self, var: u16);

    fn set_trade_memory_ht_6(&mut self, bank: bool) {
        self.set_ht_memory(4);
        self.set_ht_text_var(if bank { 0 } else { 9 });
        self.set_ht_intensity(1);
        self.set_ht_memory(memory_context_6::get_random_feeling_6(
            4,
            if bank { 10 } else { 20 },
        ));
    }

    fn set_trade_memory_ht_8(&mut self) {
        self.set_ht_memory(4);
        self.set_ht_text_var(9);
        self.set_ht_intensity(1);
        self.set_ht_memory(memory_context_8::get_random_feeling_8(4, 20));
    }

    fn clear_memories_ht(&mut self) {
        self.set_ht_memory(0);
        self.set_ht_text_var(0);
        self.set_ht_intensity(0);
        self.set_ht_feeling(0);
    }
}
