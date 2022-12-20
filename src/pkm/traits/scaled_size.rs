use crate::pkm::traits::templates::ScaledSizeReadOnly;

pub trait ScaledSize: ScaledSizeReadOnly {
    fn set_weight_scalar(&mut self, scalar: u8);
    fn set_height_scalar(&mut self, scalar: u8);
}

pub trait ScaledSize3 {
    fn scale(&self) -> u8;
    fn set_scale(&mut self, scale: u8);
}

pub trait ScaledSizeAbsolute {
    fn height_absolute(&self) -> u8;
    fn set_height_absolute(&mut self, height: u8);
    fn weight_absolute(&self) -> u8;
    fn set_weight_absolute(&mut self, weight: u8);
}

pub trait ScaledSizeValue: ScaledSize + ScaledSizeAbsolute {
    fn reset_height(&mut self);
    fn reset_weight(&mut self);
    fn calc_height_absolute(&self) -> f32;
    fn calc_weight_absolute(&self) -> f32;
}

pub trait CombatPower {
    fn stat_cp(&self) -> u32;
    fn set_stat_cp(&mut self, cp: u32);
    fn reset_cp(&mut self);
}
