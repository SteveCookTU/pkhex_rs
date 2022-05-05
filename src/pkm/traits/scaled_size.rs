pub trait ScaledSize {
    fn get_weight_scalar(&self) -> u8;
    fn set_weight_scalar(&mut self, weight: u8);
    fn get_height_scalar(&self) -> u8;
    fn set_height_scalar(&mut self, height: u8);
}

pub trait ScaledSizeAbsolute {
    fn get_height_absolute(&self) -> f32;
    fn set_height_absolute(&mut self, height: f32);
    fn get_weight_absolute(&self) -> f32;
    fn set_weight_absolute(&mut self, weight: f32);
}

pub trait ScaledSizeValue: ScaledSize + ScaledSizeAbsolute {
    fn reset_height(&mut self);
    fn reset_weight(&mut self);

    fn calc_height_absolute(&self) -> f32;
    fn calc_weight_absolute(&self) -> f32;
}

pub trait CombatPower {
    fn get_stat_cp(&self) -> usize;
    fn set_stat_cp(&mut self, cp: usize);
    fn reset_cp(&mut self);
}
