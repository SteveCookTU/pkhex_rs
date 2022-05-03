#[derive(PartialEq, Copy, Clone, PartialOrd)]
#[repr(i8)]
pub enum MoveType {
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

impl From<i8> for MoveType {
    fn from(val: i8) -> Self {
        match val {
            0 => MoveType::Normal,
            1 => MoveType::Fighting,
            2 => MoveType::Flying,
            3 => MoveType::Poison,
            4 => MoveType::Ground,
            5 => MoveType::Rock,
            6 => MoveType::Bug,
            7 => MoveType::Ghost,
            8 => MoveType::Steel,
            9 => MoveType::Fire,
            10 => MoveType::Water,
            11 => MoveType::Grass,
            12 => MoveType::Electric,
            13 => MoveType::Psychic,
            14 => MoveType::Ice,
            15 => MoveType::Dragon,
            16 => MoveType::Dark,
            17 => MoveType::Fairy,
            _ => MoveType::Any,
        }
    }
}

impl MoveType {
    pub fn get_move_type_generation(&self, generation: usize) -> MoveType {
        if generation <= 2 {
            self.get_move_type_from_g12()
        } else {
            *self
        }
    }

    fn get_move_type_from_g12(&self) -> MoveType {
        let mut val = *self as i8;
        if val <= MoveType::Rock as i8 {
            return *self;
        }
        val -= 1;
        if val <= MoveType::Steel as i8 {
            return *self;
        }
        val -= 10;
        MoveType::from(val)
    }
}
