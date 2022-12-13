#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i8)]
pub enum AbilityPermission {
    Any12H = -1,
    Any12,
    OnlyFirst,
    OnlySecond,
    OnlyHidden = 4,
}
