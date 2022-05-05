pub trait HomeTrack {
    fn get_tracker(&self) -> u64;
    fn set_tracker(&mut self, tracker: u64);
}
