mod egg_moves;
pub mod learn_set_reader;

pub use egg_moves::*;

use std::collections::HashMap;

#[derive(Default)]
pub struct LearnSet {
    pub(crate) moves: Vec<u16>,
    levels: Vec<u8>,
    learn: Option<HashMap<u16, u8>>,
}

impl LearnSet {
    pub fn new(moves: &[u16], levels: &[u8]) -> Self {
        Self {
            moves: moves.to_vec(),
            levels: levels.to_vec(),
            learn: None,
        }
    }

    pub fn get_move_range(&self, max_level: u8, min_level: Option<u8>) -> (bool, usize, usize) {
        let min_level = min_level.unwrap_or_default();
        if min_level <= 1 && max_level >= 100 {
            return (true, 0, self.moves.len() - 1);
        }
        if min_level > max_level {
            return Default::default();
        }
        let start = self.levels.iter().position(|&z| z >= min_level);
        if let Some(start) = start {
            let end = self.levels.iter().rposition(|&z| z <= min_level);
            if let Some(end) = end {
                (true, start, end)
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }

    pub fn get_encounter_moves(&self, level: u8) -> Vec<u16> {
        let mut moves = vec![0; 4];
        self.set_encounter_moves(level, &mut moves, None);
        moves
    }

    pub fn set_encounter_moves(&self, level: u8, moves: &mut [u16], ctr: Option<usize>) {
        let mut ctr = ctr.unwrap_or_default();
        for (i, &mov) in self.moves.iter().enumerate() {
            if self.levels[i] > level {
                break;
            }

            if moves.contains(&mov) {
                continue;
            }

            moves[ctr] = mov;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_encounter_moves_backwards(&self, level: u8, moves: &mut [u16], ctr: Option<usize>) {
        let mut ctr = ctr.unwrap_or_default();
        let mut index = self.levels.iter().rposition(|&z| z == level);
        loop {
            if index.is_none() {
                return;
            }
            let ind = index.unwrap();
            let mut start = ind;
            while start != 0 && self.levels[start] == self.levels[start - 1] {
                start -= 1;
            }

            for i in start..=ind {
                let mov = self.moves[i];
                if !moves.contains(&mov) {
                    moves[ctr] = mov;
                    ctr += 1;
                }
                if ctr == 4 {
                    return;
                }
            }

            if start == 0 {
                break;
            } else {
                index = Some(start - 1);
            }
        }
    }

    pub fn set_level_up_moves(
        &self,
        start_level: u8,
        end_level: u8,
        moves: &mut [u16],
        ctr: Option<usize>,
    ) {
        let mut ctr = ctr.unwrap_or_default();
        let start_index = self
            .levels
            .iter()
            .position(|&i| i >= start_level)
            .unwrap_or_default();
        let end_index = self
            .levels
            .iter()
            .rposition(|&i| i <= end_level)
            .unwrap_or_default();
        for i in start_index..end_index {
            let mov = self.moves[i];
            if moves.contains(&mov) {
                continue;
            }
            moves[ctr] = mov;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_evolution_moves(&self, moves: &mut [u16], ctr: Option<usize>) {
        let mut ctr = ctr.unwrap_or_default();
        for (i, &mov) in self.moves.iter().enumerate() {
            if self.levels[i] != 0 {
                break;
            }

            if moves.contains(&mov) {
                continue;
            }
            moves[ctr] = mov;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_level_up_moves_ignore(
        &self,
        start_level: u8,
        end_level: u8,
        moves: &mut [u16],
        ignore: &[u16],
        ctr: Option<usize>,
    ) {
        let mut ctr = ctr.unwrap_or_default();
        let start_index = self.levels.iter().position(|&i| i >= start_level);
        let start_index = if let Some(index) = start_index {
            index
        } else {
            return;
        };
        let end_index = self.levels.iter().rposition(|&i| i <= end_level);
        let end_index = end_index.unwrap_or(self.levels.len());
        for i in start_index..end_index {
            let mov = self.moves[i];
            if ignore.contains(&mov) {
                continue;
            }
            if moves.contains(&mov) {
                continue;
            }
            moves[ctr] = mov;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_evolution_moves_ignore(
        &self,
        moves: &mut [u16],
        ignore: &[u16],
        ctr: Option<usize>,
    ) {
        let mut ctr = ctr.unwrap_or_default();
        for (i, &mov) in self.moves.iter().enumerate() {
            if self.levels[i] != 0 {
                break;
            }

            if ignore.contains(&mov) {
                continue;
            }

            if moves.contains(&mov) {
                continue;
            }
            moves[ctr] = mov;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn get_min_move_level(&self, level: u8) -> u8 {
        if self.levels.is_empty() {
            1
        } else {
            let end = self
                .levels
                .iter()
                .rposition(|&i| i <= level)
                .unwrap_or_default() as u8;
            end.saturating_sub(4).max(1)
        }
    }

    pub fn get_move_level(&self, mov: u16) -> Option<u8> {
        self.moves
            .iter()
            .rposition(|&i| i == mov)
            .map(|i| self.levels[i])
    }

    fn get_hash_map(&self) -> HashMap<u16, u8> {
        let mut map = HashMap::with_capacity(self.moves.len());
        for (i, &mov) in self.moves.iter().enumerate().rev() {
            map.insert(mov, self.levels[i]);
        }
        map
    }

    pub fn get_level_learn_move(&mut self, mov: u16) -> Option<u8> {
        if let Some(learn) = self.learn.as_ref() {
            learn.get(&mov).copied()
        } else {
            self.learn = Some(self.get_hash_map());
            self.get_level_learn_move(mov)
        }
    }

    pub fn get_level_learn_move_min(&mut self, mov: u16, min: u8) -> Option<u8> {
        for (i, &mov2) in self.moves.iter().enumerate() {
            if mov2 != mov {
                continue;
            }

            let lv = self.levels[i];
            if lv >= min {
                return Some(lv);
            }
        }
        None
    }

    pub fn get_base_egg_moves(&self, level: u8) -> &[u16] {
        let mut count = 0;
        for &x in self.levels.iter() {
            if x > level {
                break;
            }
            count += 1;
        }
        if count == 0 {
            return &[];
        }

        let mut start = 0;
        if count > 4 {
            start = count - 4;
            count = 4;
        }
        &self.moves[start..(start + count)]
    }
}
