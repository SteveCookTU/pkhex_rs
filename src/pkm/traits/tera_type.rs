use crate::game::enums::MoveType;
use crate::pkm::traits::templates::TeraTypeReadOnly;

pub const OVERRIDE_NONE: u8 = 19;
pub const MAX_TYPE: u8 = 17;
const FALLBACK: MoveType = MoveType::Normal;

pub fn get_tera_type_from_param(original: u8, over: u8) -> MoveType {
    if over <= MAX_TYPE {
        return MoveType::from(over as i8);
    }
    if over != OVERRIDE_NONE {
        return FALLBACK;
    }
    if original <= MAX_TYPE {
        return MoveType::from(original as i8);
    }
    FALLBACK
}

pub trait TeraType: TeraTypeReadOnly {
    fn tera_type_original(&self) -> MoveType;
    fn set_tera_type_original(&mut self, t: MoveType);
    fn tera_type_override(&self) -> MoveType;
    fn set_tera_type_override(&mut self, t: MoveType);

    fn get_tera_type(&self) -> MoveType {
        get_tera_type_from_param(
            self.tera_type_original() as u8,
            self.tera_type_override() as u8,
        )
    }

    fn set_tera_type_from_type(&mut self, mut mt: MoveType) {
        if mt as u8 > MAX_TYPE {
            mt = FALLBACK;
        }
        let original = self.tera_type_original();
        if original == mt {
            self.set_tera_type_override(MoveType::from(OVERRIDE_NONE as i8));
        } else {
            self.set_tera_type_override(mt);
        }
    }

    fn set_tera_type(&mut self, t: u8) {
        self.set_tera_type_from_type(MoveType::from(t as i8))
    }
}
