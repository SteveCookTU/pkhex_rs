pub trait Shiny {
    fn tsv(&self) -> u16;
    fn psv(&self) -> u16;

    fn is_shiny(&self) -> bool {
        self.tsv() == self.psv()
    }
}
