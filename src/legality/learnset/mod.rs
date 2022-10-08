mod egg_moves;
pub mod learnset_reader;

use crate::{tables, BinLinkerAccessor};
pub use egg_moves::*;
use lazy_static::lazy_static;

lazy_static! {
    // Gen 1
    pub static ref LEVEL_UP_RB: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_rb.pkl"),
        tables::MAX_SPECIES_ID_1
    );
    pub static ref LEVEL_UP_Y: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_y.pkl"),
        tables::MAX_SPECIES_ID_1
    );

    // Gen 2
    pub static ref EGG_MOVES_GS: Vec<EggMoves2> = EggMoves2::get_array(
        include_bytes!("../../resources/byte/eggmove/eggmove_gs.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref LEVEL_UP_GS: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_gs.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref EGG_MOVES_C: Vec<EggMoves2> = EggMoves2::get_array(
        include_bytes!("../../resources/byte/eggmove/eggmove_c.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref LEVEL_UP_C: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_c.pkl"),
        tables::MAX_SPECIES_ID_2
    );

    // Gen 3
    pub static ref LEVEL_UP_E: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_e.pkl")).into();
    pub static ref LEVEL_UP_RS: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_rs.pkl")).into();
    pub static ref LEVEL_UP_FR: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_fr.pkl")).into();
    pub static ref LEVEL_UP_LG: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_lg.pkl")).into();
    pub static ref EGG_MOVES_RS: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_rs.pkl")).into();

    //Gen 4
    pub static ref LEVEL_UP_DP: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_dp.pkl")).into();
    pub static ref LEVEL_UP_PT: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_pt.pkl")).into();
    pub static ref LEVEL_UP_HGSS: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_hgss.pkl")).into();
    pub static ref EGG_MOVES_DPPT: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_dppt.pkl")).into();
    pub static ref EGG_MOVES_HGSS: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_hgss.pkl")).into();

    //Gen 5
    pub static ref LEVEL_UP_BW: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_bw.pkl")).into();
    pub static ref LEVEL_UP_B2W2: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_b2w2.pkl")).into();
    pub static ref EGG_MOVES_BW: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_bw.pkl")).into();

    // Gen 6
    pub static ref EGG_MOVES_XY: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_xy.pkl")).into();
    pub static ref LEVEL_UP_XY: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_xy.pkl")).into();
    pub static ref EGG_MOVES_AO: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_ao.pkl")).into();
    pub static ref LEVEL_UP_AO: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_ao.pkl")).into();

    // Gen 7
    pub static ref EGG_MOVES_SM: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_sm.pkl")).into();
    pub static ref LEVEL_UP_SM: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_sm.pkl")).into();
    pub static ref EGG_MOVES_USUM: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_uu.pkl")).into();
    pub static ref LEVEL_UP_USUM: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_uu.pkl")).into();
    pub static ref LEVEL_UP_GG: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_gg.pkl")).into();

    // Gen 8
    pub static ref EGG_MOVES_SWSH: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_swsh.pkl")).into();
    pub static ref LEVEL_UP_SWSH: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_swsh.pkl")).into();
    pub static ref EGG_MOVES_BDSP: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_bdsp.pkl")).into();
    pub static ref LEVEL_UP_BDSP: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_bdsp.pkl")).into();
    pub static ref LEVEL_UP_LA: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_la.pkl")).into();
    pub static ref MASTERY_LA: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/legality/misc/mastery_la.pkl")).into();
}

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
                .unwrap_or(self.levels.len());
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
            1_usize
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
