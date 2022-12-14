pub trait LocationSet {
    fn get_location_names(&self, bank_id: u16) -> &[String];
    fn get_location_name(&self, location_id: u16) -> &str;
}
