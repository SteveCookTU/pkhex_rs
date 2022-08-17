use crate::learnset::LearnSet;
use crate::BinLinkerAccessor;

pub fn get_array(input: &[u8], max_species: usize) -> Vec<LearnSet> {
    let mut offset = 0;
    let mut result = Vec::with_capacity(max_species + 1);

    for _ in 0..result.capacity() {
        result.push(read_learnset_8(input, &mut offset))
    }

    result
}

impl From<BinLinkerAccessor> for Vec<LearnSet> {
    fn from(entries: BinLinkerAccessor) -> Self {
        let mut result = Vec::with_capacity(entries.length());
        for i in 0..result.capacity() {
            result.push(read_learnset_16(&entries[i]));
        }

        result
    }
}

fn read_learnset_8(data: &[u8], offset: &mut usize) -> LearnSet {
    let mut end = *offset;

    if data[end] == 0 {
        *offset += 1;
        return LearnSet::default();
    }
    while {
        end += 2;
        data[end] != 0
    } {}

    let count = (end - *offset) / 2;
    let mut moves = Vec::with_capacity(count);
    let mut levels = Vec::with_capacity(count);
    for _ in 0..count {
        levels.push(data[*offset].try_into().unwrap());
        *offset += 1;
        moves.push(data[*offset].try_into().unwrap());
        *offset += 1;
    }

    *offset += 1;

    LearnSet::new(moves, levels)
}

fn read_learnset_16(data: &[u8]) -> LearnSet {
    if data.is_empty() {
        return LearnSet::default();
    }

    let count = (data.len() / 4) - 1;
    let mut moves = Vec::with_capacity(count);
    let mut levels = Vec::with_capacity(count);

    for i in 0..count {
        let index = i * 4;
        let m = &data[index..(index + 4)];
        levels.push(u16::from_le_bytes(m[2..].try_into().unwrap()).into());
        moves.push(u16::from_le_bytes(m[..2].try_into().unwrap()).into());
    }

    LearnSet::new(moves, levels)
}
