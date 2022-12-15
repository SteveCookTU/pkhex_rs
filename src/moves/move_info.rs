use crate::game::enums::Move;
use crate::moves::{
    move_info_1, move_info_2, move_info_3, move_info_4, move_info_5, move_info_6, move_info_7,
    move_info_7b, move_info_8, move_info_8a, move_info_8b, move_info_9,
};
use crate::pkm::shared::EntityContext;
use crate::{legality, PKError, PKResult};
use std::collections::HashSet;

pub fn get_pp(context: EntityContext, mov: u16) -> PKResult<u8> {
    let table = get_pp_table(context)?;
    if mov as usize >= table.len() {
        Ok(0)
    } else {
        Ok(table[mov as usize])
    }
}

pub fn get_pp_table(context: EntityContext) -> PKResult<&'static [u8]> {
    match context {
        EntityContext::Gen1 => Ok(&move_info_1::MOVE_PP_RBY),
        EntityContext::Gen2 => Ok(&move_info_2::MOVE_PP_GSC),
        EntityContext::Gen3 => Ok(&move_info_3::MOVE_PP_RS),
        EntityContext::Gen4 => Ok(&move_info_4::MOVE_PP_DP),
        EntityContext::Gen5 => Ok(&move_info_5::MOVE_PP_BW),
        EntityContext::Gen6 => Ok(&move_info_6::MOVE_PP),
        EntityContext::Gen7 => Ok(&move_info_7::MOVE_PP_SM),
        EntityContext::Gen8 => Ok(&move_info_8::MOVE_PP_SWSH),
        EntityContext::Gen9 => Ok(&move_info_9::MOVE_PP_SV),
        EntityContext::Gen7b => Ok(&move_info_7b::MOVE_PP_GG),
        EntityContext::Gen8a => Ok(&move_info_8a::MOVE_PP_LA),
        EntityContext::Gen8b => Ok(&move_info_8::MOVE_PP_SWSH),
        _ => Err(PKError::IndexOutOfRange {
            index: context as usize,
        }),
    }
}

pub fn get_dummied_moves_hash_set(context: EntityContext) -> HashSet<u16> {
    match context {
        EntityContext::Gen8 => HashSet::from(move_info_8::DUMMIED_MOVES_SWSH),
        EntityContext::Gen8a => HashSet::from(move_info_8a::DUMMIED_MOVES_LA),
        EntityContext::Gen8b => HashSet::from(move_info_8b::DUMMIED_MOVES_BDSP),
        EntityContext::Gen9 => HashSet::from(move_info_9::DUMMIED_MOVES_SV),
        _ => HashSet::new(),
    }
}

pub fn is_move_z(mov: u16) -> bool {
    match mov {
        _ if ((Move::BreakneckBlitzP as u16)..=(Move::Catastropika as u16)).contains(&mov) => true,
        _ if ((Move::SinisterArrowRaid as u16)..=(Move::GenesisSupernova as u16))
            .contains(&mov) =>
        {
            true
        }
        _ if mov == Move::TenMVoltThunderbolt as u16 => true,
        _ if ((Move::LightThatBurnstheSky as u16)..=(Move::ClangorousSoulblaze as u16))
            .contains(&mov) =>
        {
            true
        }
        _ => false,
    }
}

pub fn is_move_dynamax(mov: u16) -> bool {
    ((Move::MaxFlare as u16)..=(Move::MaxSteelspike as u16)).contains(&mov)
}

pub fn is_move_knowable(mov: u16) -> bool {
    !is_move_z(mov) && !is_move_dynamax(mov)
}

pub fn is_move_sketch(mov: u16) -> bool {
    match mov {
        _ if mov == Move::Struggle as u16 => false,
        _ if mov == Move::Chatter as u16 => false,

        _ if mov == Move::LightofRuin as u16 => false,
        _ => is_move_knowable(mov),
    }
}

pub fn is_valid_sketch(mov: u16, context: EntityContext) -> bool {
    if !is_move_sketch(mov) {
        return false;
    }
    if context == EntityContext::Gen6
        && [Move::ThousandArrows as u16, Move::ThousandWaves as u16].contains(&mov)
    {
        return false;
    }
    if context == EntityContext::Gen8b {
        if move_info_8b::DUMMIED_MOVES_BDSP.contains(&mov) {
            return false;
        }
        if mov > legality::tables8::MAX_MOVE_ID_8 {
            return false;
        }
    }

    mov <= get_max_move_id(context).unwrap_or_default()
}

fn get_max_move_id(context: EntityContext) -> Option<u16> {
    match context {
        EntityContext::Gen1 => Some(legality::tables1::MAX_MOVE_ID_1),
        EntityContext::Gen2 => Some(legality::tables2::MAX_MOVE_ID_2),
        EntityContext::Gen3 => Some(legality::tables3::MAX_MOVE_ID_3),
        EntityContext::Gen4 => Some(legality::tables4::MAX_MOVE_ID_4),
        EntityContext::Gen5 => Some(legality::tables5::MAX_MOVE_ID_5),
        EntityContext::Gen6 => Some(legality::tables6::MAX_MOVE_ID_6_AO),
        EntityContext::Gen7 => Some(legality::tables7::MAX_MOVE_ID_7_USUM),
        EntityContext::Gen7b => Some(legality::tables7b::MAX_MOVE_ID_7B),
        EntityContext::Gen8 => Some(legality::tables8a::MAX_MOVE_ID_8A),
        EntityContext::Gen8a => Some(legality::tables8a::MAX_MOVE_ID_8A),
        EntityContext::Gen8b => Some(legality::tables8bs::MAX_MOVE_ID_8B),
        EntityContext::Gen9 => Some(legality::tables9::MAX_MOVE_ID_9),
        _ => None,
    }
}
