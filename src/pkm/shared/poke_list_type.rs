#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PokeListType {
    Single = 1,
    Party = 6,
    Stored = 20,
    StoredJP = 30,
}

impl From<u8> for PokeListType {
    fn from(val: u8) -> Self {
        match val {
            1 => PokeListType::Single,
            6 => PokeListType::Party,
            20 => PokeListType::Stored,
            _ => PokeListType::StoredJP,
        }
    }
}
