use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum MoveType {
    #[num_enum(default)]
    Any = -1,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl MoveType {
    pub fn get_move_type_generation(&self, generation: u8) -> MoveType {
        if generation <= 2 {
            self.get_move_type_from_g12()
        } else {
            *self
        }
    }

    fn get_move_type_from_g12(&self) -> MoveType {
        let mut val = *self as i8;
        if val <= MoveType::Rock as i8 {
            return MoveType::from(val);
        }
        val -= 1;
        if val <= MoveType::Steel as i8 {
            return MoveType::from(val);
        }
        val -= 10;
        MoveType::from(val)
    }
}
