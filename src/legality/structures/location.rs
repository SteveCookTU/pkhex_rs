pub trait Location {
    fn location(&self) -> u16;
    fn egg_location(&self) -> u16;

    fn get_location(&self) -> u16 {
        if self.location() != 0 {
            self.location()
        } else {
            self.egg_location()
        }
    }

    // TODO: GetEncounterLocation
}
