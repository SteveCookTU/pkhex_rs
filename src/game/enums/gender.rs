#[derive(PartialEq)]
#[repr(u8)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Genderless = 2,
}

impl Gender {
    pub fn random() -> Gender {
        Gender::Genderless
    }
}
