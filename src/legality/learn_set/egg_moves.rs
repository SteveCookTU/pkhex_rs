use crate::legality::BinLinkerAccessor;
use no_std_io::Reader;

pub trait EggMoves {
    fn moves(&self) -> &[u16];
    fn get_has_egg_move(&self, mov: u16) -> bool {
        self.moves().iter().any(|&m| m == mov)
    }
}

#[derive(Default)]
pub struct EggMoves2 {
    moves: Vec<u16>,
}

impl EggMoves for EggMoves2 {
    fn moves(&self) -> &[u16] {
        &self.moves
    }
}

impl EggMoves2 {
    fn new(moves: Vec<u16>) -> Self {
        Self { moves }
    }

    pub fn get_array(data: &[u8], count: usize) -> Vec<EggMoves2> {
        let mut entries = Vec::with_capacity(count + 1);
        entries.push(EggMoves2::default());

        let base_offset = data.default_read_le::<u16>(0) as usize - (count * 2);
        for i in 0..count {
            let start = data.default_read_le::<u16>((i - 1) * 2) as usize - base_offset;
            let slice = &data[start..];
            let move_count = slice.iter().position(|&i| i == 0xFF).unwrap_or_default();
            if move_count == 0 {
                entries.push(EggMoves2::default());
                continue;
            }
            let mut moves = Vec::with_capacity(move_count);
            for m in slice.iter().take(move_count) {
                moves.push(*m as u16);
            }
            entries.push(EggMoves2::new(moves));
        }

        entries
    }
}

#[derive(Default)]
pub struct EggMoves6 {
    moves: Vec<u16>,
}

impl EggMoves for EggMoves6 {
    fn moves(&self) -> &[u16] {
        &self.moves
    }
}

impl EggMoves6 {
    fn new(moves: Vec<u16>) -> Self {
        Self { moves }
    }

    pub fn get_array(entries: BinLinkerAccessor) -> Vec<EggMoves6> {
        let mut result = Vec::with_capacity(entries.length() as usize);

        result.push(EggMoves6::default());
        for i in 1..(entries.length() as usize) {
            let data = &entries[i];
            let count = data.default_read_le::<u16>(0) as usize;
            if count == 0 {
                result.push(EggMoves6::default());
                continue;
            }

            let mut moves = Vec::with_capacity(count);
            let slice = &data[2..];
            for j in 0..count {
                moves.push(slice.default_read_le::<u16>(j * 2));
            }
            result.push(EggMoves6::new(moves));
        }

        result
    }
}

#[derive(Default)]
pub struct EggMoves7 {
    moves: Vec<u16>,
    pub form_table_index: usize,
}

impl EggMoves for EggMoves7 {
    fn moves(&self) -> &[u16] {
        &self.moves
    }
}

impl EggMoves7 {
    fn new(moves: Vec<u16>, form_table_index: usize) -> Self {
        Self {
            moves,
            form_table_index,
        }
    }

    pub fn get_array(entries: BinLinkerAccessor) -> Vec<EggMoves7> {
        let mut result = Vec::with_capacity(entries.length() as usize);
        result.push(EggMoves7::default());

        for i in 1..(entries.length() as usize) {
            let data = &entries[i];
            let count = data.default_read_le::<u16>(2) as usize;
            let form_index = data.default_read_le::<u16>(0) as usize;
            if count == 0 {
                if form_index != 0 {
                    result.push(EggMoves7 {
                        form_table_index: form_index,
                        ..Default::default()
                    });
                } else {
                    result.push(EggMoves7::default());
                }
                continue;
            }

            let mut moves = Vec::with_capacity(count);
            let slice = &data[4..];
            for j in 0..count {
                moves.push(slice.default_read_le::<u16>(j * 2));
            }
            result.push(EggMoves7::new(moves, form_index));
        }

        result
    }
}

pub mod egg_moves_9 {
    use crate::legality::BinLinkerAccessor;
    use no_std_io::Reader;

    pub fn get_array(entries: BinLinkerAccessor) -> Vec<Vec<u16>> {
        let mut result = vec![vec![]; entries.length() as usize];
        for i in 1..result.len() {
            let data = &entries[i];
            if data.is_empty() {
                continue;
            }
            let mut moves = vec![0; data.len() >> 1];
            for j in 0..data.len() {
                moves[j >> i] = data.default_read_le::<u16>(j);
            }
            result[i] = moves;
        }
        result
    }
}
