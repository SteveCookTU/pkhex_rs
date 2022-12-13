use crate::legality::MoveSetData;

pub trait Relearn {
    fn relearn(&self) -> MoveSetData;
}
