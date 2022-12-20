#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StringConverterOption {
    None,
    ClearZero,
    Clear50,
    Clear7F,
    ClearFF,
}
