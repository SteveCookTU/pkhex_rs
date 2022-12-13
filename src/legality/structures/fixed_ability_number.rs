use crate::legality::encounters::generator::AbilityPermission;

pub trait FixedAbilityNumber {
    fn ability(&self) -> AbilityPermission;
}
