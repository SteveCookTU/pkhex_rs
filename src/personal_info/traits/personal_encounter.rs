pub trait PersonalEncounter {
    fn base_exp(&self) -> u16;
    fn hatch_cycles(&self) -> u8;
    fn catch_rate(&self) -> u8;
    fn base_friendship(&self) -> u8;
    fn escape_rate(&self) -> u8;
}
