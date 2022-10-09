use crate::conversion::species_converter;
use crate::g3pkm::{convert_to, get_gba_version_id, get_gc_version_id, swap_bits, G3PKM};
use crate::personal_info_g3::PersonalInfoG3;
use crate::ribbons::{RibbonSetCommon3, RibbonSetEvent3, RibbonSetOnly3, RibbonSetUnique3};
use crate::traits::trainer_id::TrainerId;
use crate::traits::{
    ContestStats, ContestStatsMutable, GameValueLimit, LangNick, Nature, ShadowPKM, Shiny,
    SpeciesForm,
};
use crate::{
    entity_gender, entity_pid, flag_util, language, personal_table, poke_crypto, species_name,
    string_converter_3gc, tables, EntityContext, Species, StringConverterOption, PK3, PKM,
};
use no_std_io::{EndianRead, EndianWrite, StreamContainer, StreamReader, StreamWriter};
use rand::{thread_rng, Rng};

#[derive(Default, Copy, Clone, EndianRead, EndianWrite)]
pub struct CK3 {
    species_id_3: u16,
    #[no_std_io(pad_before = 2)]
    pid: u32,
    version: u8,
    current_region: u8,
    original_region: u8,
    language: u8,
    met_location: u16,
    met_level: u8,
    ball: u8,
    #[no_std_io(pad_before = 3)]
    ot_gender: u8,
    sid: u16,
    tid: u16,
    ot_trash: [u8; 22],
    nickname_trash: [u8; 22],
    nickname_copy_trash: [u8; 22],
    #[no_std_io(pad_before = 2)]
    exp: u32,
    stat_level: u8,
    #[no_std_io(pad_before = 0x17)]
    move_1: u16,
    move_1_pp: u8,
    move_1_pp_ups: u8,
    move_2: u16,
    move_2_pp: u8,
    move_2_pp_ups: u8,
    move_3: u16,
    move_3_pp: u8,
    move_3_pp_ups: u8,
    move_4: u16,
    move_4_pp: u8,
    move_4_pp_ups: u8,
    held_item: u16,
    stat_hp_current: u16,
    stat_hp_max: u16,
    stat_atk: u16,
    stat_def: u16,
    stat_spa: u16,
    stat_spd: u16,
    stat_spe: u16,
    ev_hp: u16,
    ev_atk: u16,
    ev_def: u16,
    ev_spa: u16,
    ev_spd: u16,
    ev_spe: u16,
    iv_hp: u16,
    iv_atk: u16,
    iv_def: u16,
    iv_spa: u16,
    iv_spd: u16,
    iv_spe: u16,
    ot_friendship: u16,
    cnt_cool: u8,
    cnt_beauty: u8,
    cnt_cute: u8,
    cnt_smart: u8,
    cnt_tough: u8,
    ribbon_count_g3_cool: u8,
    ribbon_count_g3_beauty: u8,
    ribbon_count_g3_cute: u8,
    ribbon_count_g3_smart: u8,
    ribbon_count_g3_tough: u8,
    cnt_sheen: u8,
    ribbon_champion_g3: u8,
    ribbon_winning: u8,
    ribbon_victory: u8,
    ribbon_artist: u8,
    ribbon_effort: u8,
    ribbon_champion_battle: u8,
    ribbon_champion_regional: u8,
    ribbon_champion_national: u8,
    ribbon_country: u8,
    ribbon_national: u8,
    ribbon_earth: u8,
    ribbon_world: u8,
    ribbon_unused: u8,
    pkrs_strain: u8,
    is_egg: u8,
    ability_bit: u8,
    valid: u8,
    #[no_std_io(pad_before = 1)]
    mark_value: u8,
    pkrs_days: u8,
    #[no_std_io(pad_before = 6)]
    party_slot: u8,
    shadow_id: u16,
    #[no_std_io(pad_before = 2)]
    purification: i32,
}

impl CK3 {
    const PURIFIED: i32 = -100;
}

impl From<CK3> for PK3 {
    fn from(ck3: CK3) -> Self {
        let mut pk3: PK3 = convert_to(ck3);
        pk3.set_flag_has_species(pk3.get_species_id_3() != 0);
        pk3.refresh_checksum();
        pk3
    }
}

impl PKM<PersonalInfoG3> for CK3 {
    fn size_party(&self) -> usize {
        poke_crypto::SIZE_3CSTORED
    }

    fn size_stored(&self) -> usize {
        poke_crypto::SIZE_3CSTORED
    }

    fn extension(&self) -> String {
        "ck3".to_string()
    }

    fn personal_info(&self) -> &'static PersonalInfoG3 {
        &personal_table::RS[self.species() as usize]
    }

    fn get_valid(&self) -> bool {
        self.valid == 0
    }

    fn set_valid(&mut self, valid: bool) {
        self.valid = u8::from(!valid);
    }

    fn nickname_trash(&self) -> &[u8] {
        &self.nickname_trash
    }

    fn ot_trash(&self) -> &[u8] {
        &self.ot_trash
    }

    fn encrypt(&mut self) -> Vec<u8> {
        (*self).into()
    }

    fn context(&self) -> EntityContext {
        EntityContext::Gen3
    }

    fn set_species(&mut self, species: u16) {
        self.species_id_3 = species_converter::get_g3_species(species);
    }

    fn set_nickname(&mut self, nickname: &str) {
        string_converter_3gc::set_string(
            &mut self.nickname_trash,
            nickname.chars().collect::<Vec<char>>(),
            10,
            StringConverterOption::None,
        );
        self.nickname_copy_trash = self.nickname_trash;
    }

    fn get_held_item(&self) -> u16 {
        self.held_item
    }

    fn set_held_item(&mut self, item: u16) {
        self.held_item = item;
    }

    fn get_gender(&self) -> u8 {
        entity_gender::get_from_pid(self.species(), self.pid)
    }

    fn set_gender(&mut self, _gender: u8) {}

    fn get_ability(&self) -> u16 {
        self.personal_info().get_ability(self.get_ability_bit()) as u16
    }

    fn set_ability(&mut self, _ability: u16) {}

    fn get_current_friendship(&self) -> u8 {
        self.ot_friendship as u8
    }

    fn set_current_friendship(&mut self, friendship: u8) {
        self.ot_friendship = friendship as u16;
    }

    fn set_form(&mut self, form: u8) {
        if self.species() == Species::Unown as u16 {
            let mut rnd = thread_rng();
            while entity_pid::get_unown_form_3(self.pid) != form {
                self.pid = rnd.gen::<u32>();
            }
        }
    }

    fn get_is_egg(&self) -> bool {
        self.is_egg == 1
    }

    fn set_is_egg(&mut self, is_egg: bool) {
        self.is_egg = u8::from(is_egg);
    }

    fn set_is_nicknamed(&mut self, _is_nicknamed: bool) {}

    fn get_exp(&self) -> u32 {
        self.exp
    }

    fn set_exp(&mut self, exp: u32) {
        self.exp = exp;
    }

    fn get_ot_name(&self) -> String {
        string_converter_3gc::get_string(&self.ot_trash)
    }

    fn set_ot_name(&mut self, name: &str) {
        string_converter_3gc::set_string(
            &mut self.ot_trash,
            name.chars().collect::<Vec<char>>(),
            10,
            StringConverterOption::None,
        );
    }

    fn get_ot_gender(&self) -> u8 {
        self.ot_gender
    }

    fn set_ot_gender(&mut self, gender: u8) {
        self.ot_gender = gender;
    }

    fn get_ball(&self) -> u8 {
        self.ball
    }

    fn set_ball(&mut self, ball: u8) {
        self.ball = ball;
    }

    fn get_met_level(&self) -> u8 {
        self.met_level
    }

    fn set_met_level(&mut self, met_level: u8) {
        self.met_level = met_level;
    }

    fn get_move_1(&self) -> u16 {
        self.move_1
    }

    fn set_move_1(&mut self, move_1: u16) {
        self.move_1 = move_1;
    }

    fn get_move_2(&self) -> u16 {
        self.move_2
    }

    fn set_move_2(&mut self, move_2: u16) {
        self.move_2 = move_2;
    }

    fn get_move_3(&self) -> u16 {
        self.move_3
    }

    fn set_move_3(&mut self, move_3: u16) {
        self.move_3 = move_3;
    }

    fn get_move_4(&self) -> u16 {
        self.move_4
    }

    fn set_move_4(&mut self, move_4: u16) {
        self.move_4 = move_4;
    }

    fn get_move_1_pp(&self) -> u8 {
        self.move_1_pp
    }

    fn set_move_1_pp(&mut self, pp: u8) {
        self.move_1_pp = pp;
    }

    fn get_move_2_pp(&self) -> u8 {
        self.move_2_pp
    }

    fn set_move_2_pp(&mut self, pp: u8) {
        self.move_2_pp = pp;
    }

    fn get_move_3_pp(&self) -> u8 {
        self.move_3_pp
    }

    fn set_move_3_pp(&mut self, pp: u8) {
        self.move_3_pp = pp;
    }

    fn get_move_4_pp(&self) -> u8 {
        self.move_4_pp
    }

    fn set_move_4_pp(&mut self, pp: u8) {
        self.move_4_pp = pp;
    }

    fn get_move_1_pp_ups(&self) -> u8 {
        self.move_1_pp_ups
    }

    fn set_move_1_pp_ups(&mut self, pp_ups: u8) {
        self.move_1_pp_ups = pp_ups;
    }

    fn get_move_2_pp_ups(&self) -> u8 {
        self.move_2_pp_ups
    }

    fn set_move_2_pp_ups(&mut self, pp_ups: u8) {
        self.move_2_pp_ups = pp_ups;
    }

    fn get_move_3_pp_ups(&self) -> u8 {
        self.move_3_pp_ups
    }

    fn set_move_3_pp_ups(&mut self, pp_ups: u8) {
        self.move_3_pp_ups = pp_ups;
    }

    fn get_move_4_pp_ups(&self) -> u8 {
        self.move_4_pp_ups
    }

    fn set_move_4_pp_ups(&mut self, pp_ups: u8) {
        self.move_4_pp_ups = pp_ups;
    }

    fn get_ev_hp(&self) -> u8 {
        self.ev_hp as u8
    }

    fn set_ev_hp(&mut self, ev_hp: u8) {
        self.ev_hp = ev_hp as u16;
    }

    fn get_ev_atk(&self) -> u8 {
        self.ev_atk as u8
    }

    fn set_ev_atk(&mut self, ev_atk: u8) {
        self.ev_atk = ev_atk as u16;
    }

    fn get_ev_def(&self) -> u8 {
        self.ev_def as u8
    }

    fn set_ev_def(&mut self, ev_def: u8) {
        self.ev_def = ev_def as u16;
    }

    fn get_ev_spe(&self) -> u8 {
        self.ev_spe as u8
    }

    fn set_ev_spe(&mut self, ev_spe: u8) {
        self.ev_spe = ev_spe as u16;
    }

    fn get_ev_spa(&self) -> u8 {
        self.ev_spa as u8
    }

    fn set_ev_spa(&mut self, ev_spa: u8) {
        self.ev_spa = ev_spa as u16;
    }

    fn get_ev_spd(&self) -> u8 {
        self.ev_spd as u8
    }

    fn set_ev_spd(&mut self, ev_spd: u8) {
        self.ev_spd = ev_spd as u16;
    }

    fn get_iv_hp(&self) -> u8 {
        (self.iv_hp as u8).min(31)
    }

    fn set_iv_hp(&mut self, iv_hp: u8) {
        self.iv_hp = (iv_hp & 0x1F) as u16;
    }

    fn get_iv_atk(&self) -> u8 {
        (self.iv_atk as u8).min(31)
    }

    fn set_iv_atk(&mut self, iv_atk: u8) {
        self.iv_atk = (iv_atk & 0x1F) as u16;
    }

    fn get_iv_def(&self) -> u8 {
        (self.iv_def as u8).min(31)
    }

    fn set_iv_def(&mut self, iv_def: u8) {
        self.iv_def = (iv_def & 0x1F) as u16;
    }

    fn get_iv_spe(&self) -> u8 {
        (self.iv_spe as u8).min(31)
    }

    fn set_iv_spe(&mut self, iv_spe: u8) {
        self.iv_spe = (iv_spe & 0x1F) as u16;
    }

    fn get_iv_spa(&self) -> u8 {
        (self.iv_spa as u8).min(31)
    }

    fn set_iv_spa(&mut self, iv_spa: u8) {
        self.iv_spa = (iv_spa & 0x1F) as u16;
    }

    fn get_iv_spd(&self) -> u8 {
        (self.iv_spd as u8).min(31)
    }

    fn set_iv_spd(&mut self, iv_spd: u8) {
        self.iv_spd = (iv_spd & 0x1F) as u16;
    }

    fn get_status_condition(&self) -> u32 {
        unimplemented!()
    }

    fn set_status_condition(&mut self, _condition: u32) {
        unimplemented!()
    }

    fn get_stat_level(&self) -> u8 {
        self.stat_level
    }

    fn set_stat_level(&mut self, stat_level: u8) {
        self.stat_level = stat_level;
    }

    fn get_stat_hp_max(&self) -> u16 {
        self.stat_hp_max
    }

    fn set_stat_hp_max(&mut self, stat_hp_max: u16) {
        self.stat_hp_max = stat_hp_max;
    }

    fn get_stat_hp_current(&self) -> u16 {
        self.stat_hp_current
    }

    fn set_stat_hp_current(&mut self, stat_hp_current: u16) {
        self.stat_hp_current = stat_hp_current;
    }

    fn get_stat_atk(&self) -> u16 {
        self.stat_atk
    }

    fn set_stat_atk(&mut self, stat_atk: u16) {
        self.stat_atk = stat_atk;
    }

    fn get_stat_def(&self) -> u16 {
        self.stat_def
    }

    fn set_stat_def(&mut self, stat_def: u16) {
        self.stat_def = stat_def;
    }

    fn get_stat_spe(&self) -> u16 {
        self.stat_spe
    }

    fn set_stat_spe(&mut self, stat_spe: u16) {
        self.stat_spe = stat_spe;
    }

    fn get_stat_spa(&self) -> u16 {
        self.stat_spa
    }

    fn set_stat_spa(&mut self, stat_spa: u16) {
        self.stat_spa = stat_spa;
    }

    fn get_stat_spd(&self) -> u16 {
        self.stat_spd
    }

    fn set_stat_spd(&mut self, stat_spd: u16) {
        self.stat_spd = stat_spd;
    }

    fn get_version(&self) -> u8 {
        get_gba_version_id(self.version)
    }

    fn set_version(&mut self, version: u8) {
        self.version = get_gc_version_id(version);
    }

    fn get_pkrs_strain(&self) -> u8 {
        self.pkrs_strain & 0xF
    }

    fn set_pkrs_strain(&mut self, strain: u8) {
        self.pkrs_strain = strain & 0xF;
    }

    fn get_pkrs_days(&self) -> u8 {
        (self.pkrs_days as i8).max(0) as u8
    }

    fn set_pkrs_days(&mut self, days: u8) {
        self.pkrs_days = if days == 0 { 0xFF } else { days & 0xF };
    }

    fn get_encryption_constant(&self) -> u32 {
        self.pid
    }

    fn set_encryption_constant(&mut self, _constant: u32) {}

    fn get_pid(&self) -> u32 {
        self.pid
    }

    fn set_pid(&mut self, pid: u32) {
        self.pid = pid;
    }

    fn set_language(&mut self, language: u8) {
        self.language = language::get_gc_lang_id_from_gc(language);
    }

    fn get_fateful_encounter(&self) -> bool {
        flag_util::get_flag_from_u8(self.ribbon_unused, 4)
    }

    fn set_fateful_encounter(&mut self, fateful: bool) {
        flag_util::set_flag_in_u8(&mut self.ribbon_unused, 4, fateful);
    }

    fn characteristic(&self) -> Option<u8> {
        None
    }

    fn get_mark_value(&self) -> u8 {
        swap_bits(self.mark_value, 1, 2)
    }

    fn set_mark_value(&mut self, value: u8) {
        self.mark_value = swap_bits(value, 1, 2);
    }

    fn get_met_location(&self) -> u16 {
        self.met_location
    }

    fn set_met_location(&mut self, location: u16) {
        self.met_location = location;
    }

    fn get_egg_location(&self) -> u16 {
        0
    }

    fn set_egg_location(&mut self, _location: u16) {}

    fn get_ot_friendship(&self) -> u8 {
        self.ot_friendship as u8
    }

    fn set_ot_friendship(&mut self, friendship: u8) {
        self.ot_friendship = friendship as u16;
    }

    fn get_current_handler(&self) -> u8 {
        0
    }

    fn set_current_handler(&mut self, _handler: u8) {}

    fn marking_count(&self) -> u8 {
        4
    }

    fn get_marking(&self, index: u8) -> u8 {
        if index > self.marking_count() {
            panic!("Mark index out of range for CK3")
        } else {
            (self.mark_value >> index) & 1
        }
    }

    fn set_marking(&mut self, index: u8, value: u8) {
        if index > self.marking_count() {
            panic!("Mark index out of range for CK3")
        } else {
            self.mark_value = (self.mark_value & !(1 << index)) | ((value & 1) << index);
        }
    }

    fn refresh_checksum(&mut self) {}

    fn checksum_valid(&self) -> bool {
        self.get_valid()
    }
}

impl SpeciesForm for CK3 {
    fn species(&self) -> u16 {
        species_converter::get_g3_species_raw(self.species_id_3)
    }

    fn form(&self) -> u8 {
        if self.species() == Species::Unown as u16 {
            entity_pid::get_unown_form_3(self.pid)
        } else {
            0
        }
    }
}

impl TrainerId for CK3 {
    fn get_tid(&self) -> u16 {
        self.tid
    }

    fn set_tid(&mut self, tid: u16) {
        self.tid = tid;
    }

    fn get_sid(&self) -> u16 {
        self.sid
    }

    fn set_sid(&mut self, sid: u16) {
        self.sid = sid;
    }
}

impl Shiny for CK3 {
    fn tsv(&self) -> u16 {
        (self.tid ^ self.sid) >> 3
    }

    fn psv(&self) -> u16 {
        (((self.pid >> 16) ^ (self.pid & 0xFFFF)) >> 3) as u16
    }
}

impl LangNick for CK3 {
    fn nickname(&self) -> String {
        string_converter_3gc::get_string(&self.nickname_trash)
    }

    fn is_nicknamed(&self) -> bool {
        species_name::is_nicknamed(self.species(), &self.nickname(), self.language(), 3)
    }

    fn language(&self) -> u8 {
        language::get_main_lang_id_from_gc(self.language)
    }
}

impl GameValueLimit for CK3 {
    fn max_move_id(&self) -> u16 {
        tables::MAX_MOVE_ID_3
    }

    fn max_species_id(&self) -> u16 {
        tables::MAX_SPECIES_ID_3
    }

    fn max_item_id(&self) -> u16 {
        tables::MAX_ITEM_ID_3
    }

    fn max_ability_id(&self) -> u16 {
        tables::MAX_ABILITY_ID_3
    }

    fn max_ball_id(&self) -> u8 {
        tables::MAX_BALL_ID_3
    }

    fn max_game_id(&self) -> u8 {
        tables::MAX_GAME_ID_3
    }

    fn max_iv(&self) -> u8 {
        31
    }

    fn max_ev(&self) -> u8 {
        255
    }

    fn ot_length(&self) -> u8 {
        7
    }

    fn nick_length(&self) -> u8 {
        10
    }
}

impl Nature for CK3 {
    fn get_nature(&self) -> u8 {
        (self.pid % 25) as u8
    }

    fn set_nature(&mut self, _nature: u8) {}
}

impl From<CK3> for Vec<u8> {
    fn from(ck3: CK3) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_3CSTORED];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_be(&ck3);
        writer.into_raw()
    }
}

impl From<Vec<u8>> for CK3 {
    fn from(mut data: Vec<u8>) -> Self {
        data.resize(poke_crypto::SIZE_3CSTORED, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_be::<CK3>()
    }
}

impl RibbonSetEvent3 for CK3 {
    fn get_ribbon_earth(&self) -> bool {
        self.ribbon_earth == 1
    }

    fn set_ribbon_earth(&mut self, value: bool) {
        self.ribbon_earth = u8::from(value);
    }

    fn get_ribbon_national(&self) -> bool {
        self.ribbon_national == 1
    }

    fn set_ribbon_national(&mut self, value: bool) {
        self.ribbon_national = u8::from(value);
    }

    fn get_ribbon_country(&self) -> bool {
        self.ribbon_country == 1
    }

    fn set_ribbon_country(&mut self, value: bool) {
        self.ribbon_country = u8::from(value);
    }

    fn get_ribbon_champion_battle(&self) -> bool {
        self.ribbon_champion_battle == 1
    }

    fn set_ribbon_champion_battle(&mut self, value: bool) {
        self.ribbon_champion_battle = u8::from(value);
    }

    fn get_ribbon_champion_regional(&self) -> bool {
        self.ribbon_champion_regional == 1
    }

    fn set_ribbon_champion_regional(&mut self, value: bool) {
        self.ribbon_champion_regional = u8::from(value);
    }

    fn get_ribbon_champion_national(&self) -> bool {
        self.ribbon_champion_national == 1
    }

    fn set_ribbon_champion_national(&mut self, value: bool) {
        self.ribbon_champion_national = u8::from(value);
    }
}

impl RibbonSetCommon3 for CK3 {
    fn get_ribbon_champion_g3(&self) -> bool {
        self.ribbon_champion_g3 == 1
    }

    fn set_ribbon_champion_g3(&mut self, value: bool) {
        self.ribbon_champion_g3 = u8::from(value);
    }

    fn get_ribbon_artist(&self) -> bool {
        self.ribbon_artist == 1
    }

    fn set_ribbon_artist(&mut self, value: bool) {
        self.ribbon_artist = u8::from(value);
    }

    fn get_ribbon_effort(&self) -> bool {
        self.ribbon_effort == 1
    }

    fn set_ribbon_effort(&mut self, value: bool) {
        self.ribbon_effort = u8::from(value);
    }
}

impl RibbonSetUnique3 for CK3 {
    fn get_ribbon_winning(&self) -> bool {
        self.ribbon_winning == 1
    }

    fn set_ribbon_winning(&mut self, value: bool) {
        self.ribbon_winning = u8::from(value);
    }

    fn get_ribbon_victory(&self) -> bool {
        self.ribbon_victory == 1
    }

    fn set_ribbon_victory(&mut self, value: bool) {
        self.ribbon_victory = u8::from(value);
    }
}

impl RibbonSetOnly3 for CK3 {
    fn get_ribbon_count_g3_cool(&self) -> u8 {
        self.ribbon_count_g3_cool
    }

    fn set_ribbon_count_g3_cool(&mut self, count: u8) {
        self.ribbon_count_g3_cool = count;
    }

    fn get_ribbon_count_g3_beauty(&self) -> u8 {
        self.ribbon_count_g3_beauty
    }

    fn set_ribbon_count_g3_beauty(&mut self, count: u8) {
        self.ribbon_count_g3_beauty = count;
    }

    fn get_ribbon_count_g3_cute(&self) -> u8 {
        self.ribbon_count_g3_cute
    }

    fn set_ribbon_count_g3_cute(&mut self, count: u8) {
        self.ribbon_count_g3_cute = count;
    }

    fn get_ribbon_count_g3_smart(&self) -> u8 {
        self.ribbon_count_g3_smart
    }

    fn set_ribbon_count_g3_smart(&mut self, count: u8) {
        self.ribbon_count_g3_smart = count;
    }

    fn get_ribbon_count_g3_tough(&self) -> u8 {
        self.ribbon_count_g3_tough
    }

    fn set_ribbon_count_g3_tough(&mut self, count: u8) {
        self.ribbon_count_g3_tough = count;
    }

    fn get_ribbon_world(&self) -> bool {
        self.ribbon_world == 1
    }

    fn set_ribbon_world(&mut self, value: bool) {
        self.ribbon_world = u8::from(value);
    }

    fn get_unused1(&self) -> bool {
        flag_util::get_flag_from_u8(self.ribbon_unused, 0)
    }

    fn set_unused1(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.ribbon_unused, 0, value);
    }

    fn get_unused2(&self) -> bool {
        flag_util::get_flag_from_u8(self.ribbon_unused, 1)
    }

    fn set_unused2(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.ribbon_unused, 1, value);
    }

    fn get_unused3(&self) -> bool {
        flag_util::get_flag_from_u8(self.ribbon_unused, 2)
    }

    fn set_unused3(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.ribbon_unused, 2, value);
    }

    fn get_unused4(&self) -> bool {
        flag_util::get_flag_from_u8(self.ribbon_unused, 3)
    }

    fn set_unused4(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.ribbon_unused, 3, value);
    }
}

impl ContestStats for CK3 {
    fn get_cnt_cool(&self) -> u8 {
        self.cnt_cool
    }

    fn get_cnt_beauty(&self) -> u8 {
        self.cnt_beauty
    }

    fn get_cnt_cute(&self) -> u8 {
        self.cnt_cute
    }

    fn get_cnt_smart(&self) -> u8 {
        self.cnt_smart
    }

    fn get_cnt_tough(&self) -> u8 {
        self.cnt_tough
    }

    fn get_cnt_sheen(&self) -> u8 {
        self.cnt_sheen
    }
}

impl ContestStatsMutable for CK3 {
    fn set_cnt_cool(&mut self, count: u8) {
        self.cnt_cool = count;
    }

    fn set_cnt_beauty(&mut self, count: u8) {
        self.cnt_beauty = count;
    }

    fn set_cnt_cute(&mut self, count: u8) {
        self.cnt_cute = count;
    }

    fn set_cnt_smart(&mut self, count: u8) {
        self.cnt_smart = count;
    }

    fn set_cnt_tough(&mut self, count: u8) {
        self.cnt_tough = count;
    }

    fn set_cnt_sheen(&mut self, count: u8) {
        self.cnt_sheen = count;
    }
}

impl G3PKM<PersonalInfoG3> for CK3 {
    fn get_species_id_3(&self) -> u16 {
        self.species_id_3
    }

    fn set_species_id_3(&mut self, species: u16) {
        self.species_id_3 = species;
    }

    fn get_ability_bit(&self) -> bool {
        self.ability_bit == 1
    }

    fn set_ability_bit(&mut self, value: bool) {
        self.ability_bit = u8::from(value);
    }
}

impl ShadowPKM for CK3 {
    fn get_shadow_id(&self) -> u16 {
        self.shadow_id
    }

    fn set_shadow_id(&mut self, id: u16) {
        self.shadow_id = id;
    }

    fn get_purification(&self) -> i32 {
        self.purification
    }

    fn set_purification(&mut self, purification: i32) {
        self.purification = purification;
    }

    fn is_shadow(&self) -> bool {
        self.shadow_id != 0 && self.purification != CK3::PURIFIED
    }
}

#[cfg(test)]
mod tests {
    use crate::pkm::ck3::CK3;
    use crate::{Move, PKM};

    const COLO_CK3: &[u8] = include_bytes!("../resources/tests/colo.ck3");

    #[test]
    fn should_read() {
        let bytes = COLO_CK3.to_vec();
        let pkm: CK3 = bytes.into();
        assert_eq!(pkm.pid, 0x317A28D1);
        assert_eq!(pkm.exp, 15625);
        assert_eq!(pkm.move_1, Move::Confusion as u16);
        assert_eq!(pkm.ot_friendship, 70);
        assert_eq!(pkm.get_iv_spe(), 14);
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = COLO_CK3.to_vec();
        let pkm: CK3 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        println!("{:0>2X?}", bytes);
        println!("{:0>2X?}", output);
        assert_eq!(bytes, output.to_vec())
    }
}
