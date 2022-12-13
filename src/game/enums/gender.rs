#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Gender {
    Male,
    Female,
    RandomGenderless,
}
