use crate::pkm::traits::templates::GigantamaxReadOnly;

pub trait Gigantamax: GigantamaxReadOnly {
    fn set_can_gigantamax(&mut self, value: bool);
}
