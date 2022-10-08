pub trait SanityChecksum {
    fn get_sanity(&self) -> u16;
    fn set_sanity(&mut self, sanity: u16);

    fn get_checksum(&self) -> u16;
    fn set_checksum(&mut self, checksum: u16);
}
