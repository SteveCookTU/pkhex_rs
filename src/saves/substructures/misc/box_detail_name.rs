pub trait BoxDetailName {
    fn get_box_name(&self, index: u8) -> String;
    fn set_box_name(&self, index: u8, value: &str);
}
