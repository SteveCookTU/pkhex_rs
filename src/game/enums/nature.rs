use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Nature {
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,

    #[num_enum(default)]
    Random = 25,
}

impl Nature {
    pub fn is_fixed(&self) -> bool {
        *self >= Nature::Hardy && *self < Nature::Random
    }

    pub fn is_neutral(&self) -> bool {
        self.is_fixed() && (*self as u8) % 6 == 0
    }
}
