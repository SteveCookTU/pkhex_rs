use crate::{GBPkm, PersonalInfo};

pub trait GBPkml<I: PersonalInfo>: GBPkm<I> {
    const STRING_LENGTH_JAPANESE: usize = 6;
    const STRING_LENGTH_NOT_JAPANESE: usize = 11;

    fn new_blank(jp: bool) -> Self;

    fn new(data: Vec<u8>, jp: bool) -> Self;
}
