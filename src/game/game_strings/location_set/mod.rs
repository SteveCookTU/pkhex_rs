mod location_set_0;
mod location_set_4;
mod location_set_6;

pub use location_set_0::*;
pub use location_set_4::*;
pub use location_set_6::*;

pub trait LocationSet {
    fn get_location_names(&self, bank_id: u16) -> &[String];
    fn get_location_name(&self, location_id: u16) -> &str;
}
