#[derive(PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
pub enum StringConverterOption {
    None,
    ClearZero,
    Clear50,
    Clear7F,
    ClearFF,
}
