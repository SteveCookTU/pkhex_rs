use crate::traits::{
    BaseStat, EffortValueYield, GenderDetail, PersonalAbility, PersonalEgg, PersonalEncounter,
    PersonalFormInfo, PersonalMisc, PersonalType,
};

pub trait PersonalInfo:
    BaseStat
    + EffortValueYield
    + GenderDetail
    + PersonalFormInfo
    + PersonalAbility
    + PersonalEgg
    + PersonalEncounter
    + PersonalType
    + PersonalMisc
{
    fn write(&self) -> Vec<u8>;

    fn gender(&self) -> u8;

    fn exp_growth(&self) -> u8;
}
