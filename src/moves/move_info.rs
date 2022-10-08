use crate::moves::{
    move_info_1, move_info_2, move_info_3, move_info_4, move_info_5, move_info_6, move_info_7,
    move_info_7b, move_info_8, move_info_8a,
};
use crate::EntityContext;

pub fn get_pp(context: EntityContext, mov: u16) -> u8 {
    let table = get_pp_table(context);
    if mov as usize >= table.len() {
        0
    } else {
        table[mov as usize]
    }
}

pub fn get_pp_table(context: EntityContext) -> &'static [u8] {
    match context {
        EntityContext::Gen1 => &move_info_1::MOVE_PP_RBY,
        EntityContext::Gen2 => &move_info_2::MOVE_PP_GSC,
        EntityContext::Gen3 => &move_info_3::MOVE_PP_RS,
        EntityContext::Gen4 => &move_info_4::MOVE_PP_DB,
        EntityContext::Gen5 => &move_info_5::MOVE_PP_BW,
        EntityContext::Gen6 => &move_info_6::MOVE_PP_6,
        EntityContext::Gen7 => &move_info_7::MOVE_PP_SM,
        EntityContext::Gen8 => &move_info_8::MOVE_PP_SWSH,

        EntityContext::Gen7b => &move_info_7b::MOVE_PP_GG,
        EntityContext::Gen8a => &move_info_8a::MOVE_PP_LA,
        EntityContext::Gen8b => &move_info_8::MOVE_PP_SWSH,
        _ => panic!("Invalid EntityContext for PP Table"),
    }
}
