pub trait EffortValueYield {
    fn ev_hp(&self) -> u8;
    fn ev_atk(&self) -> u8;
    fn ev_def(&self) -> u8;
    fn ev_spe(&self) -> u8;
    fn ev_spa(&self) -> u8;
    fn ev_spd(&self) -> u8;
}
