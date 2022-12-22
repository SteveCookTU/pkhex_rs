use num_enum::{FromPrimitive, IntoPrimitive};
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum EncounterTime {
    #[num_enum(default)]
    Any,
    Morning = 1 << 1,
    Day = 1 << 2,
    Night = 1 << 3,
}

impl EncounterTime {
    pub fn contains(&self, t2: u8) -> bool {
        *self == EncounterTime::Any || ((*self as usize) & (1 << t2)) != 0
    }

    pub fn random_valid_time(&self) -> u8 {
        let mut rnd = thread_rng();
        let mut val = rnd.gen_range(1..4);
        if *self == EncounterTime::Any {
            return val;
        }
        while !self.contains(val) {
            val = rnd.gen_range(1..4);
        }
        val
    }
}
