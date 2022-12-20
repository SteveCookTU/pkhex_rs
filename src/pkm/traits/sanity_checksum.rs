pub trait SanityChecksum {
    fn sanity(&self) -> u16;
    fn set_sanity(&mut self, sanity: u16);
    fn checksum(&self) -> u16;
    fn set_checksum(&mut self, checksum: u16);
}
