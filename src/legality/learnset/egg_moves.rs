use crate::BinLinkerAccessor;

pub trait EggMoves {
    fn get_moves(&self) -> &[usize];
    fn get_has_egg_move(&self, mov: usize) -> bool {
        self.get_moves().iter().any(|&m| m == mov)
    }
}

#[derive(Default, Clone)]
pub struct EggMoves2 {
    moves: Vec<usize>,
}

impl EggMoves2 {
    pub fn new(moves: Vec<usize>) -> Self {
        Self { moves }
    }

    pub fn get_array(data: &[u8], count: usize) -> Vec<Self> {
        let mut entries = Vec::with_capacity(count + 1);
        entries.push(EggMoves2::default());

        let base_offset = u16::from_le_bytes(data[0..2].try_into().unwrap()) as usize - (count * 2);
        for i in 1..entries.capacity() {
            let index = (i - 1) * 2;
            let start = u16::from_le_bytes(data[index..(index + 2)].try_into().unwrap()) as usize
                - base_offset;
            let slice = &data[start..];
            let move_count = slice.iter().position(|&i| i == 0xFF).unwrap();
            if move_count == 0 {
                entries.push(EggMoves2::default());
                continue;
            }

            let mut moves = Vec::with_capacity(move_count);
            for m in slice.iter().take(move_count) {
                moves.push((*m).into());
            }

            entries.push(EggMoves2::new(moves));
        }

        entries
    }
}

impl EggMoves for EggMoves2 {
    fn get_moves(&self) -> &[usize] {
        &self.moves
    }
}

#[derive(Default, Clone)]
pub struct EggMoves6 {
    moves: Vec<usize>,
}

impl EggMoves6 {
    pub fn new(moves: Vec<usize>) -> Self {
        Self { moves }
    }
}

impl From<BinLinkerAccessor> for Vec<EggMoves6> {
    fn from(entries: BinLinkerAccessor) -> Self {
        let mut result = Vec::with_capacity(entries.length());
        result.push(EggMoves6::default());

        for i in 1..result.capacity() {
            let data = &entries[i];
            let count = u16::from_le_bytes(data[..2].try_into().unwrap()) as usize;
            if count == 0 {
                result.push(EggMoves6::default());
                continue;
            }

            let mut moves = Vec::with_capacity(count);
            let slice = &data[2..];
            for j in 0..moves.capacity() {
                let index = j * 2;
                moves
                    .push(u16::from_le_bytes(slice[index..(index + 2)].try_into().unwrap()).into());
            }
            result.push(EggMoves6::new(moves));
        }

        result
    }
}

impl EggMoves for EggMoves6 {
    fn get_moves(&self) -> &[usize] {
        &self.moves
    }
}

#[derive(Default, Clone)]
pub struct EggMoves7 {
    moves: Vec<usize>,
    form_table_index: usize,
}

impl EggMoves7 {
    pub fn new(moves: Vec<usize>, form_table_index: usize) -> Self {
        Self {
            moves,
            form_table_index,
        }
    }

    pub fn get_form_table_index(&self) -> usize {
        self.form_table_index
    }
}

impl From<BinLinkerAccessor> for Vec<EggMoves7> {
    fn from(entries: BinLinkerAccessor) -> Self {
        let mut result = Vec::with_capacity(entries.length());
        result.push(EggMoves7::default());

        for i in 1..result.capacity() {
            let data = &entries[i];
            let count = u16::from_le_bytes(data[2..].try_into().unwrap()) as usize;
            let form_index = u16::from_le_bytes(data[..2].try_into().unwrap()) as usize;
            if count == 0 {
                if form_index != 0 {
                    result.push(EggMoves7::new(vec![], form_index));
                } else {
                    result.push(EggMoves7::default());
                }
                continue;
            }

            let mut moves = Vec::with_capacity(count);
            let slice = &data[4..];
            for j in 0..moves.capacity() {
                let index = j * 2;
                moves
                    .push(u16::from_le_bytes(slice[index..(index + 2)].try_into().unwrap()).into());
            }
            result.push(EggMoves7::new(moves, form_index));
        }

        result
    }
}

impl EggMoves for EggMoves7 {
    fn get_moves(&self) -> &[usize] {
        &self.moves
    }
}
