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

    fn get_pkm_trainer_id_format<Personal: PersonalInfo + 'static>(&self) -> u8
    where
        Self: PKM<Personal>,
    {
        let format = self.generation();
        if (format < 3 && self.format() >= 7) || format == 0 {
            4
        } else {
            format
        }
    }

    fn get_sav_trainer_id_format<Personal: PersonalInfo + 'static>(&self) -> u8
    where
        Self: SaveFile<Personal>,
    {
        self.generation()
    }
}
