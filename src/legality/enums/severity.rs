use crate::legality::formatting::legality_check_strings::{L_S_FISHY, L_S_INVALID, L_S_VALID};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i8)]
pub enum Severity {
    #[num_enum(default)]
    Invalid = -1,
    Fishy,
    Valid,
}

impl Severity {
    pub fn description(&self) -> String {
        match self {
            Severity::Invalid => L_S_INVALID.lock().unwrap().clone(),
            Severity::Fishy => L_S_FISHY.lock().unwrap().clone(),
            Severity::Valid => L_S_VALID.lock().unwrap().clone(),
        }
    }
}
