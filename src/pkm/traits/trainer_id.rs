use crate::{PersonalInfo, SaveFile, PKM};

pub trait TrainerId {
    fn get_tid(&self) -> u16;
    fn set_tid(&mut self, tid: u16);

    fn get_sid(&self) -> u16;
    fn set_sid(&mut self, sid: u16);

    fn pid_is_shiny(&self, pid: u32, gen: u8) -> bool {
        let xor = self.get_sid() as u32 ^ self.get_tid() as u32 ^ (pid >> 16) ^ (pid & 0xFFFF);
        xor < if gen >= 7 { 16 } else { 8 }
    }
}

pub fn get_pkm_trainer_id_format<Personal: PersonalInfo + 'static, T: PKM<Personal>>(p: &T) -> u8 {
    let format = p.generation();
    if (format < 3 && p.format() >= 7) || format == 0 {
        4
    } else {
        format
    }
}

pub fn get_sav_trainer_id_format<T: SaveFile>(s: &T) -> u8 {
    s.generation()
}
