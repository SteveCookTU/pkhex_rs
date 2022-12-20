pub trait EggMoves {
    fn moves(&self) -> &[u16];
    fn get_has_egg_move(&self, mov: u16) -> bool {
        self.moves().iter().any(|&m| m == mov)
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
