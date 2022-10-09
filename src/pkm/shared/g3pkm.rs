use crate::personal_info_g3::PersonalInfoG3;
use crate::ribbons::{RibbonSetCommon3, RibbonSetEvent3, RibbonSetOnly3, RibbonSetUnique3};
use crate::traits::{ContestStats, ContestStatsMutable};
use crate::{GCVersion, GameVersion, PersonalInfo, PKM};

pub trait G3PKM<Personal: PersonalInfo + 'static>:
    PKM<Personal>
    + RibbonSetEvent3
    + RibbonSetCommon3
    + RibbonSetUnique3
    + RibbonSetOnly3
    + ContestStats
    + ContestStatsMutable
{
    fn get_species_id_3(&self) -> u16;
    fn set_species_id_3(&mut self, species: u16);

    fn get_ability_bit(&self) -> bool;
    fn set_ability_bit(&mut self, value: bool);
}

pub(crate) fn swap_bits(value: u8, p1: u32, p2: u32) -> u8 {
    let bit1 = (value >> p1) & 1;
    let bit2 = (value >> p2) & 1;
    let mut x = bit1 ^ bit2;
    x = (x << p1) | (x << p2);
    value ^ x
}

pub(crate) fn get_gba_version_id(gc: u8) -> u8 {
    GameVersion::from(GCVersion::from(gc)) as u8
}

pub(crate) fn get_gc_version_id(gba: u8) -> u8 {
    GCVersion::from(GameVersion::from(gba as usize)) as u8
}

pub(crate) fn convert_to<T: G3PKM<PersonalInfoG3>, K: G3PKM<PersonalInfoG3> + Default>(
    origin: T,
) -> K {
    let mut new = K::default();
    new.set_species_id_3(origin.get_species_id_3());
    new.set_language(origin.language());
    new.set_pid(origin.get_pid());
    new.set_tid(origin.get_tid());
    new.set_sid(origin.get_sid());
    new.set_exp(origin.get_exp());
    new.set_held_item(origin.get_held_item());
    new.set_ability_bit(origin.get_ability_bit());
    new.set_is_egg(origin.get_is_egg());
    new.set_fateful_encounter(origin.get_fateful_encounter());

    new.set_met_location(origin.get_met_location());
    new.set_met_level(origin.get_met_level());
    new.set_version(origin.get_version());
    new.set_ball(origin.get_ball());

    new.set_nickname(&origin.nickname());
    new.set_ot_name(&origin.get_ot_name());
    new.set_ot_gender(origin.get_ot_gender());
    new.set_ot_friendship(origin.get_ot_friendship());

    new.set_move_1_pp_ups(origin.get_move_1_pp_ups());
    new.set_move_2_pp_ups(origin.get_move_2_pp_ups());
    new.set_move_3_pp_ups(origin.get_move_3_pp_ups());
    new.set_move_4_pp_ups(origin.get_move_4_pp_ups());
    new.set_move_1(origin.get_move_1());
    new.set_move_2(origin.get_move_2());
    new.set_move_3(origin.get_move_3());
    new.set_move_4(origin.get_move_4());
    new.set_move_1_pp(origin.get_move_1_pp());
    new.set_move_2_pp(origin.get_move_2_pp());
    new.set_move_3_pp(origin.get_move_3_pp());
    new.set_move_4_pp(origin.get_move_4_pp());

    new.set_iv_hp(origin.get_iv_hp());
    new.set_iv_atk(origin.get_iv_atk());
    new.set_iv_def(origin.get_iv_def());
    new.set_iv_spa(origin.get_iv_spa());
    new.set_iv_spd(origin.get_iv_spd());
    new.set_iv_spe(origin.get_iv_spe());
    new.set_ev_hp(origin.get_ev_hp());
    new.set_ev_atk(origin.get_ev_atk());
    new.set_ev_def(origin.get_ev_def());
    new.set_ev_spa(origin.get_ev_spa());
    new.set_ev_spd(origin.get_ev_spd());
    new.set_ev_spe(origin.get_ev_spe());
    new.set_cnt_cool(origin.get_cnt_cool());
    new.set_cnt_beauty(origin.get_cnt_beauty());
    new.set_cnt_cute(origin.get_cnt_cute());
    new.set_cnt_smart(origin.get_cnt_smart());
    new.set_cnt_tough(origin.get_cnt_tough());
    new.set_cnt_sheen(origin.get_cnt_sheen());

    new.set_pkrs_days(origin.get_pkrs_days());
    new.set_pkrs_strain(origin.get_pkrs_strain());

    new.set_ribbon_count_g3_cool(origin.get_ribbon_count_g3_cool());
    new.set_ribbon_count_g3_beauty(origin.get_ribbon_count_g3_beauty());
    new.set_ribbon_count_g3_cute(origin.get_ribbon_count_g3_cute());
    new.set_ribbon_count_g3_smart(origin.get_ribbon_count_g3_smart());
    new.set_ribbon_count_g3_tough(origin.get_ribbon_count_g3_tough());
    new.set_ribbon_champion_g3(origin.get_ribbon_champion_g3());
    new.set_ribbon_winning(origin.get_ribbon_winning());
    new.set_ribbon_victory(origin.get_ribbon_victory());
    new.set_ribbon_artist(origin.get_ribbon_artist());
    new.set_ribbon_effort(origin.get_ribbon_effort());
    new.set_ribbon_champion_battle(origin.get_ribbon_champion_battle());
    new.set_ribbon_champion_regional(origin.get_ribbon_champion_regional());
    new.set_ribbon_champion_national(origin.get_ribbon_champion_national());
    new.set_ribbon_country(origin.get_ribbon_country());
    new.set_ribbon_national(origin.get_ribbon_national());
    new.set_ribbon_earth(origin.get_ribbon_earth());
    new.set_ribbon_world(origin.get_ribbon_world());
    new.set_unused1(origin.get_unused1());
    new.set_unused2(origin.get_unused2());
    new.set_unused3(origin.get_unused3());
    new.set_unused4(origin.get_unused4());

    new
}
