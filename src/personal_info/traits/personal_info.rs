use crate::traits::gender_detail::GenderDetail;
use crate::traits::{
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
    fn write(&self) -> Vec<u8>;

    fn exp_growth(&self) -> u8;

    fn tmhm(&self) -> Vec<bool> {
        vec![]
    }

    fn set_tmhm(&mut self, _bits: Vec<bool>) {}

    fn type_tutors(&self) -> Vec<bool> {
        vec![]
    }

    fn set_type_tutors(&mut self, _bits: Vec<bool>) {}

    fn special_tutors(&self) -> Vec<Vec<bool>> {
        vec![]
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

// pub(crate) fn set_bits(bits: &[bool], data: &mut [u8]) {
//     for i in (0..bits.len()).rev() {
//         data[i >> 3] |= if bits[i] { 1 << (i & 7) } else { 0 };
//     }
// }
