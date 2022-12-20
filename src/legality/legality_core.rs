use crate::legality::{egg_moves_9, learn_set_reader, BinLinkerAccessor, LearnSet};
use crate::resource_util;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref EGG_MOVES_SV: Vec<Vec<u16>> = egg_moves_9::get_array(
        BinLinkerAccessor::new(resource_util::get_bin_resource("eggmove_sv.pkl"))
    );
    pub(crate) static ref REMINDER_SV: Vec<Vec<u16>> = egg_moves_9::get_array(
        BinLinkerAccessor::new(resource_util::get_bin_resource("reminder_sv.pkl"))
    );
    pub(crate) static ref LEVEL_UP_SV: Vec<LearnSet> = learn_set_reader::get_array_linker(
        BinLinkerAccessor::new(resource_util::get_bin_resource("lvlmove_sv.pkl"))
    );
}
