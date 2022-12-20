use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum AlchremieDecoration {
    #[num_enum(default)]
    Strawberry = 0,
    Berry = 1,
    Love = 2,
    Star = 3,
    Clover = 4,
    Flower = 5,
    Ribbon = 6,
}
