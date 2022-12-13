#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Shiny {
    Random,
    Never,
    Always,
    AlwaysStar,
    AlwaysSquare,
    FixedValue,
}
