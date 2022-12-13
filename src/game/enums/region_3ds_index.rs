#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Region3DSIndex {
    None,
    Japan,
    NorthAmerica,
    Europe,
    China,
    Korea,
    Taiwan,
}
