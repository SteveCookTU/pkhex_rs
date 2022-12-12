use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::{
    BaseStat, EffortValueYield, PersonalAbility, PersonalEgg, PersonalEncounter, PersonalFormInfo,
    PersonalMisc, PersonalType,
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
    fn exp_growth(&self) -> u8;

    fn tmhm(&self) -> &[bool] {
        &[]
    }

    fn set_tmhm(&mut self, _bits: Vec<bool>) {}

    fn type_tutors(&self) -> &[bool] {
        &[]
    }

    fn set_type_tutors(&mut self, _bits: Vec<bool>) {}

    fn special_tutors(&self) -> &[Vec<bool>] {
        &[]
    }

    fn add_tmhm(&mut self, data: &[u8]) {
        self.set_tmhm(get_bits(data));
    }

    fn add_type_tutors(&mut self, data: &[u8]) {
        self.set_type_tutors(get_bits(data));
    }
}

pub(crate) fn get_bits(data: &[u8]) -> Vec<bool> {
    let mut result = vec![false; data.len() << 3];
    for i in (0..result.len()).rev() {
        result[i] = ((data[i >> 3] >> (i & 7)) & 1) == 1;
    }
    result
}
