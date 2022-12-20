use crate::game::enums::MoveType;

pub trait TeraTypeReadOnly {
    fn tera_type(&self) -> MoveType;
}
