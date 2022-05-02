pub trait RegionOrigin {
    fn get_console_region(&self) -> u8;
    fn set_console_region(&mut self, console_region: u8);
    fn get_country(&self) -> u8;
    fn set_country(&mut self, country: u8);
    fn get_region(&self) -> u8;
    fn set_region(&mut self, region: u8);

    fn set_default_region_origins(&mut self) {
        self.set_console_region(1); // North America
        self.set_region(7); // California
        self.set_country(49); // USA
    }

    fn copy_region_origin<T: RegionOrigin>(&self, dest: &mut T) {
        dest.set_console_region(self.get_console_region());
        dest.set_country(self.get_country());
        dest.set_region(self.get_region());
    }

    fn clear_region_origin(&mut self) {
        self.set_console_region(0);
        self.set_region(0);
        self.set_country(0);
    }
}
