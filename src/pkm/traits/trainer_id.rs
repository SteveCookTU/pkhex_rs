pub trait TrainerID {
    fn tid(&self) -> u16;
    fn sid(&self) -> u16;
    fn set_tid(&mut self, tid: u16);
    fn set_sid(&mut self, sid: u16);
}

pub trait TrainerIDPKM: TrainerID {}

pub trait TrainerIDSav: TrainerID {
    fn is_shiny(&self, pid: u32, gen: Option<u8>) -> bool {
        let gen = gen.unwrap_or(7);
        let xor = u32::from(self.sid()) ^ u32::from(self.tid()) ^ (pid >> 16) ^ (pid & 0xFFFF);
        xor < if gen >= 7 { 16 } else { 8 }
    }
}
