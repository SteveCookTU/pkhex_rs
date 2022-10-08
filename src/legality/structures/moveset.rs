#[derive(Copy, Clone)]
pub struct Moveset {
    pub move1: u16,
    pub move2: u16,
    pub move3: u16,
    pub move4: u16,
}

impl Moveset {
    pub fn contains(&self, mov: &u16) -> bool {
        &self.move1 == mov || &self.move2 == mov || &self.move3 == mov || &self.move4 == mov
    }

    pub fn any_above(&self, max: u16) -> bool {
        self.move1 > max || self.move2 > max || self.move3 > max || self.move4 > max
    }

    pub fn contains_any(&self, moves: &[u16]) -> bool {
        moves.iter().any(|m| self.contains(m))
    }

    pub fn contains_all(&self, moves: &[u16]) -> bool {
        moves.iter().all(|m| self.contains(m))
    }

    pub fn get_moveset_lines(&self, names: &[&str], split: &str) -> String {
        let mut lines = String::with_capacity(128);
        lines += names[self.move1 as usize];

        if self.move2 != 0 {
            lines += split;
            lines += names[self.move2 as usize];
            if self.move3 != 0 {
                lines += split;
                lines += names[self.move3 as usize];
                if self.move4 != 0 {
                    lines += split;
                    lines += names[self.move4 as usize];
                }
            }
        }

        lines
    }

    pub fn bit_overlap(&self, slice: &[u16]) -> u32 {
        let mut flags = 0;
        for (i, mov) in slice.iter().enumerate() {
            if self.contains(mov) {
                flags |= 1 << i;
            }
        }
        flags
    }
}

pub fn bit_overlap(moves: &[u16], slice: &[u16]) -> u32 {
    let mut flags = 0;
    for (i, mov) in slice.iter().enumerate() {
        if moves.contains(mov) {
            flags |= 1 << i;
        }
    }
    flags
}

impl From<Moveset> for [u16; 4] {
    fn from(set: Moveset) -> Self {
        [set.move1, set.move2, set.move3, set.move4]
    }
}
