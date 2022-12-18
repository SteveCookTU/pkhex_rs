pub trait Shiny {
    fn tsv(&self) -> u16;
    fn set_tsv(&mut self, tsv: u16);
    fn psv(&self) -> u16;
    fn set_psv(&mut self, psv: u16);
    fn is_shiny(&self) -> bool {
        self.tsv() == self.psv()
    }
}
