use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum PKMType {
    PK1,
    PK2,
    PK3,
    PK4,
    PK5,
    PK6,
    PK7,
    PK8,
    PB7,
    PB8,
    PA8,
    SK2,
    XK3,
    CK3,
    BK4
}

impl Display for PKMType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PKMType::PK1 => write!(f, "PK1"),
            PKMType::PK2 => write!(f, "PK2"),
            PKMType::PK3 => write!(f, "PK3"),
            PKMType::PK4 => write!(f, "PK4"),
            PKMType::PK5 => write!(f, "PK5"),
            PKMType::PK6 => write!(f, "PK6"),
            PKMType::PK7 => write!(f, "PK7"),
            PKMType::PK8 => write!(f, "PK8"),
            PKMType::PB7 => write!(f, "PB7"),
            PKMType::PB8 => write!(f, "PB8"),
            PKMType::PA8 => write!(f, "PA8"),
            PKMType::SK2 => write!(f, "SK2"),
            PKMType::XK3 => write!(f, "XK3"),
            PKMType::CK3 => write!(f, "CK3"),
            PKMType::BK4 => write!(f, "BK4"),
        }
    }
}