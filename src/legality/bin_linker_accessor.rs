use std::ops::Index;

pub struct BinLinkerAccessor {
    data: &'static [u8],
}

impl BinLinkerAccessor {
    pub fn new(data: &'static [u8]) -> Self {
        Self { data }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn identifier(&self) -> String {
        self.data
            .iter()
            .take(2)
            .map(|i| char::from(*i))
            .collect::<String>()
    }
}

impl Index<usize> for BinLinkerAccessor {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let offset = 4 + (index * std::mem::size_of::<u32>());
        let end = u32::from_le_bytes((&self.data[(offset + 4)..(offset + 8)]).try_into().unwrap())
            as usize;
        let start =
            u32::from_le_bytes((&self.data[(offset)..(offset + 4)]).try_into().unwrap()) as usize;
        &self.data[start..end]
    }
}
