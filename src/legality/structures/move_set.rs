pub trait MoveSet {
    fn moves(&self) -> MoveSetData;
}

#[derive(Default, Copy, Clone)]
pub struct MoveSetData {
    pub move_1: u16,
    pub move_2: u16,
    pub move_3: u16,
    pub move_4: u16,
}

impl MoveSetData {
    pub fn has_moves(&self) -> bool {
        self.move_1 != 0
    }

    pub fn contains(&self, mov: u16) -> bool {
        self.move_1 == mov || self.move_2 == mov || self.move_3 == mov || self.move_4 == mov
    }

    pub fn any_above(&self, mov: u16) -> bool {
        self.move_1 > mov || self.move_2 > mov || self.move_3 > mov || self.move_4 > mov
    }

    pub fn to_array(self) -> [u16; 4] {
        [self.move_1, self.move_2, self.move_3, self.move_4]
    }

    pub fn copy_to(&self, moves: &mut [u16]) {
        moves[3] = self.move_4;
        moves[2] = self.move_3;
        moves[1] = self.move_2;
        moves[0] = self.move_1;
    }

    pub fn contains_any(&self, moves: &[u16]) -> bool {
        moves.iter().any(|&i| self.contains(i))
    }

    pub fn contains_all(&self, moves: &[u16]) -> bool {
        moves.iter().all(|&i| self.contains(i))
    }

    pub fn get_move_set_line(&self, names: &[impl AsRef<str>], split: &str) -> String {
        let mut result = String::new();

        result += names[self.move_1 as usize].as_ref();
        if self.move_2 != 0 {
            result += split;
            result += names[self.move_2 as usize].as_ref();
            if self.move_3 != 0 {
                result += split;
                result += names[self.move_3 as usize].as_ref();
                if self.move_4 != 0 {
                    result += split;
                    result += names[self.move_4 as usize].as_ref();
                }
            }
        }

        result
    }

    pub fn bit_overlap(&self, data: &[u16]) -> usize {
        let mut flags = 0;
        for (i, &mov) in data.iter().enumerate() {
            if self.contains(mov) {
                flags |= 1 << i;
            }
        }
        flags
    }

    pub fn bit_overlap_from_list(moves: &[u16], data: &[u16]) -> usize {
        let mut flags = 0;
        for (i, mov) in data.iter().enumerate() {
            if moves.contains(mov) {
                flags |= 1 << i;
            }
        }
        flags
    }
}
