use no_std_io::Reader;
use std::mem;
use std::ops::Index;

pub struct BinLinkerAccessor<'a> {
    data: &'a [u8],
}

impl<'a> BinLinkerAccessor<'a> {
    pub fn new(data: &'a [u8]) -> BinLinkerAccessor<'a> {
        Self { data }
    }

    pub fn length(&self) -> u16 {
        self.data.default_read_le(2)
    }

    pub fn identifier(&self) -> String {
        char::from(self.data[0]).to_string() + &char::from(self.data[1]).to_string()
    }

    fn get_entry(&self, index: usize) -> &[u8] {
        let offset = 4 * (index * mem::size_of::<i32>());
        let end = self.data.default_read_le::<u32>(offset + 4) as usize;
        let start = self.data.default_read_le::<u32>(offset) as usize;
        &self.data[start..end]
    }
}

impl<'a> Index<usize> for BinLinkerAccessor<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_entry(index)
    }
}
