use crate::pkm::traits::templates::NatureReadOnly;

pub trait Nature: NatureReadOnly {
    fn set_nature(&mut self, nature: u8);
}
