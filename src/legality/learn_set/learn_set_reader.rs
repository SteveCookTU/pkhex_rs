use crate::legality::{BinLinkerAccessor, LearnSet};
use no_std_io::Reader;

pub fn get_array(input: &[u8], max_species: u16) -> Vec<LearnSet> {
    let mut offset = 0;
    let mut result = Vec::with_capacity(max_species as usize + 1);
    for _ in 0..(max_species + 1) {
        result.push(read_learn_set_8(input, &mut offset));
    }
    result
}

pub fn get_array_linker(entries: BinLinkerAccessor) -> Vec<LearnSet> {
    let mut result = Vec::with_capacity(entries.length() as usize);
    for i in 0..(entries.length() as usize) {
        result.push(read_learn_set_16(&entries[i]));
    }
    result
}

fn read_learn_set_8(data: &[u8], offset: &mut usize) -> LearnSet {
    let mut end = *offset;
    if data[end] == 0 {
        *offset += 1;
        return Default::default();
    }
    while {
        end += 2;
        data[end] != 0
    } {}

    let count = (end - *offset) / 2;
    let mut moves = vec![0u16; count];
    let mut levels = vec![0u8; count];
    for (mov, lv) in moves.iter_mut().zip(levels.iter_mut()) {
        *lv = data[*offset];
        *offset += 1;
        *mov = data[*offset] as u16;
        *offset += 1;
    }
    *offset += 1;
    LearnSet::new(&moves, &levels)
}

fn read_learn_set_16(data: &[u8]) -> LearnSet {
    if data.len() <= 4 {
        return Default::default();
    }

    let count = (data.len() / 4) - 1;
    let mut moves = vec![0u16; count];
    let mut levels = vec![0u8; count];
    for (i, (mov, lv)) in moves.iter_mut().zip(levels.iter_mut()).enumerate() {
        let mov_data = &data[(i * 4)..((i * 4) + 4)];
        *lv = mov_data[2];
        *mov = mov_data.default_read_le::<u16>(0);
    }
    LearnSet::new(&moves, &levels)
}
