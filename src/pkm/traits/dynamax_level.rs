use crate::pkm::traits::templates::DynamaxLevelReadOnly;

pub trait DynamaxLevel: DynamaxLevelReadOnly {
    fn set_dynamax_level(&mut self, level: u8);
}
