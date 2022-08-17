#[derive(Default, Clone)]
pub struct LearnSet {
    moves: Vec<usize>,
    levels: Vec<usize>,
}

impl LearnSet {
    pub fn new(moves: Vec<usize>, levels: Vec<usize>) -> Self {
        Self { moves, levels }
    }

    pub fn get_moves(&self, max_level: usize, min_level: usize) -> Vec<usize> {
        if min_level <= 1 && max_level >= 100 {
            return self.moves.clone();
        }
        if min_level <= max_level {
            let start = self.levels.iter().position(|&l| l >= min_level);
            if let Some(start) = start {
                let end = self.levels.iter().rposition(|&l| l <= max_level);
                if let Some(end) = end {
                    return if end - start + 1 == self.moves.len() {
                        self.moves.clone()
                    } else {
                        self.moves[start..=end].to_vec()
                    };
                }
            }
        }
        vec![]
    }

    pub fn get_move_range(&self, max_level: usize, min_level: usize) -> (bool, usize, usize) {
        if min_level <= 1 && max_level >= 100 {
            return (true, 0, self.moves.len() - 1);
        }
        if min_level > max_level {
            return Default::default();
        }
        if let Some(start) = self.levels.iter().position(|&l| l >= min_level) {
            if let Some(end) = self.levels.iter().rposition(|&l| l <= max_level) {
                return (true, start, end);
            }
        }

        Default::default()
    }

    pub fn add_moves(
        &self,
        mut moves: Vec<usize>,
        max_level: usize,
        min_level: usize,
    ) -> Vec<usize> {
        if min_level <= 1 && max_level >= 100 {
            moves.extend(&self.moves);
            return moves;
        }
        if min_level > max_level {
            return moves;
        }
        if let Some(start) = self.levels.iter().position(|&l| l >= min_level) {
            if let Some(end) = self.levels.iter().rposition(|&l| l <= max_level) {
                moves.extend(&self.moves[start..=end])
            }
        }

        moves
    }

    pub fn get_move_list(&self, max_level: usize, min_level: usize) -> Vec<usize> {
        let list = Vec::new();
        self.add_moves(list, max_level, min_level)
    }

    pub fn set_encounter_moves(&self, level: usize, moves: &mut [usize], mut ctr: usize) {
        for (&m, &l) in self.moves.iter().zip(self.levels.iter()) {
            if l > level {
                break;
            }

            if moves.contains(&m) {
                continue;
            }

            moves[ctr] = m;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_encounter_moves_backwards(&self, level: usize, moves: &mut [usize], mut ctr: usize) {
        if let Some(mut index) = self.levels.iter().rposition(|&l| l <= level) {
            loop {
                let mut start = index;
                while start != 0 && self.levels[start] == self.levels[start - 1] {
                    start -= 1;
                }

                for &mov in &self.moves[start..=index] {
                    if !moves.contains(&mov) {
                        moves[ctr] = mov;
                        ctr += 1;
                    }
                    if ctr == 4 {
                        return;
                    }
                }
                if let Some(new) = index.checked_sub(start - 1) {
                    index = new;
                } else {
                    return;
                }
            }
        }
    }

    pub fn set_level_up_moves(
        &self,
        start_level: usize,
        end_level: usize,
        moves: &mut [usize],
        mut ctr: usize,
    ) {
        if let Some(start) = self.levels.iter().position(|&l| l >= start_level) {
            if let Some(end) = self.levels.iter().position(|&l| l > end_level) {
                for &mov in &self.moves[start..end] {
                    if moves.contains(&mov) {
                        continue;
                    }

                    moves[ctr] = mov;
                    ctr += 1;
                    ctr &= 3;
                }
            }
        }
    }

    pub fn set_evolution_moves(&self, moves: &mut [usize], mut ctr: usize) {
        for (&m, &l) in self.moves.iter().zip(self.levels.iter()) {
            if l != 0 {
                break;
            }
            if moves.contains(&m) {
                continue;
            }
            moves[ctr] = m;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn set_level_up_moves_ignore(
        &self,
        start_level: usize,
        end_level: usize,
        moves: &mut [usize],
        ignore: &[usize],
        mut ctr: usize,
    ) {
        if let Some(start) = self.levels.iter().position(|&l| l >= start_level) {
            let end = self
                .levels
                .iter()
                .position(|&l| l > end_level)
                .unwrap_or_else(|| self.levels.len());
            for &mov in &self.moves[start..end] {
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
    }

    pub fn set_evolution_moves_ignore(
        &self,
        moves: &mut [usize],
        ignore: &[usize],
        mut ctr: usize,
    ) {
        for (&m, &l) in self.moves.iter().zip(self.levels.iter()) {
            if l != 0 {
                break;
            }

            if ignore.contains(&m) {
                continue;
            }

            if moves.contains(&m) {
                continue;
            }
            moves[ctr] = m;
            ctr += 1;
            ctr &= 3;
        }
    }

    pub fn get_unique_moves_learned(
        &self,
        seed: &[usize],
        max_level: usize,
        min_level: usize,
    ) -> Vec<usize> {
        let mut list = seed.to_vec();
        if let Some(start) = self.levels.iter().position(|&l| l >= min_level) {
            if let Some(end) = self.levels.iter().rposition(|&l| l <= max_level) {
                for &mov in &self.moves[start..=end] {
                    if !list.contains(&mov) {
                        list.push(mov);
                    }
                }
            }
        }
        list
    }

    pub fn get_min_move_level(&self, level: usize) -> usize {
        if self.levels.is_empty() {
            return 1;
        }

        if let Some(end) = self.levels.iter().rposition(|&l| l <= level) {
            end.saturating_sub(4).max(1) as usize
        } else {
            1 as usize
        }
    }

    pub fn get_move_level(&self, mov: usize) -> Option<usize> {
        if let Some((_, &l)) = self
            .moves
            .iter()
            .zip(self.levels.iter())
            .find(|&(&m, _)| m == mov)
        {
            Some(l)
        } else {
            None
        }
    }

    pub fn get_level_learn_move(&self, mov: usize, min: usize) -> Option<usize> {
        for (&m, &l) in self.moves.iter().zip(self.levels.iter()) {
            if mov != m {
                continue;
            }
            if l >= min {
                return Some(l);
            }
        }
        None
    }

    pub fn get_base_egg_moves(&self, level: usize) -> &[usize] {
        let mut count = 0;
        for &x in &self.levels {
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
