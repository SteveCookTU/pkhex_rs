pub trait EffortValueYield {
    fn ev_hp(&self) -> i32;
    fn ev_atk(&self) -> i32;
    fn ev_def(&self) -> i32;
    fn ev_spe(&self) -> i32;
    fn ev_spa(&self) -> i32;
    fn ev_spd(&self) -> i32;
}
