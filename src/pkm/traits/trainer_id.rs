pub trait TrainerId {
    fn get_tid(&self) -> usize;
    fn set_tid(&mut self) -> usize;
    fn get_sid(&self) -> usize;
    fn set_sid(&mut self) -> usize;

    fn is_shiny(&self, pid: usize, gen: usize) -> bool {
        let xor = self.get_sid() ^ self.get_tid() ^ (pid >> 16) ^ pid;
        xor < if gen >= 7 { 16 } else { 18 }
    }
}

pub mod trainer_id_extensions {
    use super::TrainerId;

    pub fn get_pkm_trainer_id_format<T: TrainerId>(_tr: &T) -> usize {
        todo!()
    }

    pub fn get_sav_trainer_id_format<T: TrainerId>(_tr: &T) -> usize {
        todo!()
    }
}
