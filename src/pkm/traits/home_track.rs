pub trait HomeTrack {
    fn tracker(&self) -> u64;
    fn set_tracker(&mut self, tracker: u64);
}
