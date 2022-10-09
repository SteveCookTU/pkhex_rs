use crate::conversion::species_converter;
use crate::g3pkm::{swap_bits, G3PKM};
use crate::personal_info_g3::PersonalInfoG3;
use crate::ribbons::{RibbonSetCommon3, RibbonSetEvent3, RibbonSetOnly3, RibbonSetUnique3};
use crate::traits::trainer_id::TrainerId;
use crate::traits::{
    ContestStats, ContestStatsMutable, GameValueLimit, LangNick, Nature, Shiny, SpeciesForm,
};
use crate::{
    entity_gender, entity_pid, flag_util, species_name, string_converter_3, tables, LanguageID,
    Species, StringConverterOption,
};
use crate::{personal_table, poke_crypto, EntityContext, PKM};
use no_std_io::{EndianRead, EndianWrite, StreamContainer, StreamReader, StreamWriter};
use rand::{thread_rng, Rng};

#[derive(Default, Copy, Clone, EndianRead, EndianWrite)]
pub struct PK3 {
    pid: u32,
    tid: u16,
    sid: u16,
    nickname_trash: [u8; 10],
    language: u8,
    egg_flags: u8,
    ot_trash: [u8; 7],
    mark_value: u8,
    checksum: u16,
    sanity: u16,
    species_id_3: u16,
    held_item: u16,
    exp: u32,
    pp_ups: u8,
    ot_friendship: u8,
    #[no_std_io(pad_before = 2)]
    move_1: u16,
    move_2: u16,
    move_3: u16,
    move_4: u16,
    move_1_pp: u8,
    move_2_pp: u8,
    move_3_pp: u8,
    move_4_pp: u8,
    ev_hp: u8,
    ev_atk: u8,
    ev_def: u8,
    ev_spe: u8,
    ev_spa: u8,
    ev_spd: u8,
    cnt_cool: u8,
    cnt_beauty: u8,
    cnt_cute: u8,
    cnt_smart: u8,
    cnt_tough: u8,
    cnt_sheen: u8,
    pkrs: u8,
    met_location: u8,
    origins: u16,
    iv32: u32,
    rib0: u32,
    status_condition: u32,
    stat_level: u8,
    held_mail_id: u8,
    stat_hp_current: u16,
    stat_hp_max: u16,
    stat_atk: u16,
    stat_def: u16,
    stat_spe: u16,
    stat_spa: u16,
    stat_spd: u16,
}

const EGG_NAME_JAPANESE: &str = "タマゴ";

impl PK3 {
    pub fn get_flag_is_bad_egg(&self) -> bool {
        flag_util::get_flag_from_u8(self.egg_flags, 0)
    }

    pub fn set_flag_is_bad_egg(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.egg_flags, 0, value)
    }

    pub fn get_flag_has_species(&self) -> bool {
        flag_util::get_flag_from_u8(self.egg_flags, 1)
    }

    pub fn set_flag_has_species(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.egg_flags, 1, value)
    }

    pub fn get_flag_is_egg(&self) -> bool {
        flag_util::get_flag_from_u8(self.egg_flags, 2)
    }

    pub fn set_flag_is_egg(&mut self, value: bool) {
        flag_util::set_flag_in_u8(&mut self.egg_flags, 2, value)
    }

    fn calculate_checksum(&self) -> u16 {
        let data: Vec<u8> = (*self).into();
        poke_crypto::get_chk3(&data)
    }
}

impl PKM<PersonalInfoG3> for PK3 {
    fn size_party(&self) -> usize {
        poke_crypto::SIZE_3PARTY
    }

    fn size_stored(&self) -> usize {
        poke_crypto::SIZE_3STORED
    }

    fn extension(&self) -> String {
        "pk3".to_string()
    }

    fn personal_info(&self) -> &'static PersonalInfoG3 {
        &personal_table::RS[self.species() as usize]
    }

    fn get_valid(&self) -> bool {
        true
    }

    fn set_valid(&mut self, _valid: bool) {}

    fn nickname_trash(&self) -> &[u8] {
        &self.nickname_trash
    }

    fn ot_trash(&self) -> &[u8] {
        &self.ot_trash
    }

    fn encrypt(&mut self) -> Vec<u8> {
        self.refresh_checksum();
        let data: Vec<u8> = (*self).into();
        poke_crypto::encrypt_array_3(&data)
    }

    fn context(&self) -> EntityContext {
        EntityContext::Gen3
    }

    fn set_species(&mut self, species: u16) {
        let s3 = species_converter::get_g3_species(species);
        self.species_id_3 = s3;
        self.set_flag_has_species(s3 != 0);
    }

    fn set_nickname(&mut self, nickname: &str) {
        let jp = self.japanese();
        let is_egg = self.get_is_egg();
        string_converter_3::set_string(
            &mut self.nickname_trash,
            &(if is_egg { EGG_NAME_JAPANESE } else { nickname })
                .chars()
                .collect::<Vec<char>>(),
            10,
            jp,
            StringConverterOption::None,
        );
    }

    fn get_held_item(&self) -> u16 {
        self.held_item
    }

    fn set_held_item(&mut self, item: u16) {
        self.held_item = item
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
        self.ot_friendship
    }

    fn set_current_friendship(&mut self, friendship: u8) {
        self.ot_friendship = friendship;
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
        ((self.iv32 >> 30) & 1) == 1
    }

    fn set_is_egg(&mut self, is_egg: bool) {
        self.iv32 = (self.iv32 & !0x40000000) | if is_egg { 0x40000000 } else { 0 };
        self.set_flag_is_egg(is_egg);
        if is_egg {
            self.set_nickname(EGG_NAME_JAPANESE);
            self.language = LanguageID::Japanese as u8;
        }
    }

    fn set_is_nicknamed(&mut self, _is_nicknamed: bool) {}

    fn get_exp(&self) -> u32 {
        self.exp
    }

    fn set_exp(&mut self, exp: u32) {
        self.exp = exp;
    }

    fn get_ot_name(&self) -> String {
        string_converter_3::get_string(&self.ot_trash, self.japanese())
    }

    fn set_ot_name(&mut self, name: String) {
        let jp = self.japanese();
        string_converter_3::set_string(
            &mut self.ot_trash,
            &(name.chars().collect::<Vec<char>>()),
            7,
            jp,
            StringConverterOption::None,
        );
    }

    fn get_ot_gender(&self) -> u8 {
        ((self.origins >> 15) & 1) as u8
    }

    fn set_ot_gender(&mut self, gender: u8) {
        self.origins = (self.origins & !(1 << 15)) | ((gender as u16 & 1) << 15);
    }

    fn get_ball(&self) -> u8 {
        ((self.origins >> 11) & 0xF) as u8
    }

    fn set_ball(&mut self, ball: u8) {
        self.origins = (self.origins & !0x7800) | ((ball as u16 & 0xF) << 11);
    }

    fn get_met_level(&self) -> u8 {
        (self.origins & 0x7F) as u8
    }

    fn set_met_level(&mut self, met_level: u8) {
        self.origins = (self.origins & !0x7F) | met_level as u16;
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
        self.pp_ups & 3
    }

    fn set_move_1_pp_ups(&mut self, pp_ups: u8) {
        self.pp_ups = (self.pp_ups & !3) | pp_ups;
    }

    fn get_move_2_pp_ups(&self) -> u8 {
        (self.pp_ups >> 2) & 3
    }

    fn set_move_2_pp_ups(&mut self, pp_ups: u8) {
        self.pp_ups = (self.pp_ups & !(3 << 2)) | (pp_ups << 2);
    }

    fn get_move_3_pp_ups(&self) -> u8 {
        (self.pp_ups >> 4) & 3
    }

    fn set_move_3_pp_ups(&mut self, pp_ups: u8) {
        self.pp_ups = (self.pp_ups & !(3 << 4)) | (pp_ups << 4);
    }

    fn get_move_4_pp_ups(&self) -> u8 {
        (self.pp_ups >> 6) & 3
    }

    fn set_move_4_pp_ups(&mut self, pp_ups: u8) {
        self.pp_ups = (self.pp_ups & !(3 << 6)) | (pp_ups << 6);
    }

    fn get_ev_hp(&self) -> u8 {
        self.ev_hp
    }

    fn set_ev_hp(&mut self, ev_hp: u8) {
        self.ev_hp = ev_hp;
    }

    fn get_ev_atk(&self) -> u8 {
        self.ev_atk
    }

    fn set_ev_atk(&mut self, ev_atk: u8) {
        self.ev_atk = ev_atk;
    }

    fn get_ev_def(&self) -> u8 {
        self.ev_def
    }

    fn set_ev_def(&mut self, ev_def: u8) {
        self.ev_def = ev_def;
    }

    fn get_ev_spe(&self) -> u8 {
        self.ev_spe
    }

    fn set_ev_spe(&mut self, ev_spe: u8) {
        self.ev_spe = ev_spe;
    }

    fn get_ev_spa(&self) -> u8 {
        self.ev_spa
    }

    fn set_ev_spa(&mut self, ev_spa: u8) {
        self.ev_spa = ev_spa;
    }

    fn get_ev_spd(&self) -> u8 {
        self.ev_spd
    }

    fn set_ev_spd(&mut self, ev_spd: u8) {
        self.ev_spd = ev_spd;
    }

    fn get_iv_hp(&self) -> u8 {
        (self.iv32 & 0x1F) as u8
    }

    fn set_iv_hp(&mut self, iv_hp: u8) {
        self.iv32 = (self.iv32 & !0x1F) | if iv_hp > 31 { 31 } else { iv_hp as u32 };
    }

    fn get_iv_atk(&self) -> u8 {
        ((self.iv32 >> 5) & 0x1F) as u8
    }

    fn set_iv_atk(&mut self, iv_atk: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 5)) | (if iv_atk > 31 { 31 } else { iv_atk as u32 } << 5);
    }

    fn get_iv_def(&self) -> u8 {
        ((self.iv32 >> 10) & 0x1F) as u8
    }

    fn set_iv_def(&mut self, iv_def: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 10)) | (if iv_def > 31 { 31 } else { iv_def as u32 } << 10);
    }

    fn get_iv_spe(&self) -> u8 {
        ((self.iv32 >> 15) & 0x1F) as u8
    }

    fn set_iv_spe(&mut self, iv_spe: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 15)) | (if iv_spe > 31 { 31 } else { iv_spe as u32 } << 15);
    }

    fn get_iv_spa(&self) -> u8 {
        ((self.iv32 >> 20) & 0x1F) as u8
    }

    fn set_iv_spa(&mut self, iv_spa: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 20)) | (if iv_spa > 31 { 31 } else { iv_spa as u32 } << 20);
    }

    fn get_iv_spd(&self) -> u8 {
        ((self.iv32 >> 25) & 0x1F) as u8
    }

    fn set_iv_spd(&mut self, iv_spd: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 25)) | (if iv_spd > 31 { 31 } else { iv_spd as u32 } << 25);
    }

    fn get_status_condition(&self) -> u32 {
        self.status_condition
    }

    fn set_status_condition(&mut self, condition: u32) {
        self.status_condition = condition;
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
        ((self.origins >> 7) & 0xF) as u8
    }

    fn set_version(&mut self, version: u8) {
        self.origins = (self.origins & !0x780) | ((version as u16 & 0xF) << 7);
    }

    fn get_pkrs_strain(&self) -> u8 {
        self.pkrs >> 4
    }

    fn set_pkrs_strain(&mut self, strain: u8) {
        self.pkrs = (self.pkrs & 0xF) | (strain << 4);
    }

    fn get_pkrs_days(&self) -> u8 {
        self.pkrs & 0xF
    }

    fn set_pkrs_days(&mut self, days: u8) {
        self.pkrs = (self.pkrs & !0xF) | days;
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
        self.language = language;
    }

    fn get_fateful_encounter(&self) -> bool {
        (self.rib0 >> 31) == 1
    }

    fn set_fateful_encounter(&mut self, fateful: bool) {
        self.rib0 = (self.rib0 & !(1 << 31)) | (u32::from(fateful) << 31)
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
        self.met_location as u16
    }

    fn set_met_location(&mut self, location: u16) {
        self.met_location = location as u8;
    }

    fn get_egg_location(&self) -> u16 {
        0
    }

    fn set_egg_location(&mut self, _location: u16) {}

    fn get_ot_friendship(&self) -> u8 {
        self.ot_friendship
    }

    fn set_ot_friendship(&mut self, friendship: u8) {
        self.ot_friendship = friendship;
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
            panic!("Mark index out of range for PK3")
        } else {
            (self.mark_value >> index) & 1
        }
    }

    fn set_marking(&mut self, index: u8, value: u8) {
        if index > self.marking_count() {
            panic!("Mark index out of range for PK3")
        } else {
            self.mark_value = (self.mark_value & !(1 << index)) | ((value & 1) << index);
        }
    }

    fn refresh_checksum(&mut self) {
        self.set_flag_is_bad_egg(false);
        let data: Vec<u8> = (*self).into();
        self.checksum = poke_crypto::get_chk3(&data);
    }

    fn checksum_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}

impl SpeciesForm for PK3 {
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

impl TrainerId for PK3 {
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

impl Shiny for PK3 {
    fn tsv(&self) -> u16 {
        (self.tid ^ self.sid) >> 3
    }

    fn psv(&self) -> u16 {
        (((self.pid >> 16) ^ (self.pid & 0xFFFF)) >> 3) as u16
    }
}

impl LangNick for PK3 {
    fn nickname(&self) -> String {
        string_converter_3::get_string(&self.nickname_trash, self.japanese())
    }

    fn is_nicknamed(&self) -> bool {
        species_name::is_nicknamed(self.species(), &self.nickname(), self.language, 3)
    }

    fn language(&self) -> u8 {
        self.language
    }
}

impl GameValueLimit for PK3 {
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

impl Nature for PK3 {
    fn get_nature(&self) -> u8 {
        (self.pid % 25) as u8
    }

    fn set_nature(&mut self, _nature: u8) {}
}

impl From<Vec<u8>> for PK3 {
    fn from(mut data: Vec<u8>) -> Self {
        poke_crypto::decrypt_if_encrypted_3(&mut data);
        data.resize(poke_crypto::SIZE_3PARTY, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_le::<PK3>()
    }
}

impl From<PK3> for Vec<u8> {
    fn from(pkm: PK3) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_3PARTY];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_le(&pkm);
        writer.into_raw()
    }
}

impl RibbonSetEvent3 for PK3 {
    fn get_ribbon_earth(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 25)
    }

    fn set_ribbon_earth(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 25, value);
    }

    fn get_ribbon_national(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 24)
    }

    fn set_ribbon_national(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 24, value);
    }

    fn get_ribbon_country(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 23)
    }

    fn set_ribbon_country(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 23, value);
    }

    fn get_ribbon_champion_battle(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 20)
    }

    fn set_ribbon_champion_battle(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 20, value);
    }

    fn get_ribbon_champion_regional(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 21)
    }

    fn set_ribbon_champion_regional(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 21, value);
    }

    fn get_ribbon_champion_national(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 22)
    }

    fn set_ribbon_champion_national(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 22, value);
    }
}

impl RibbonSetCommon3 for PK3 {
    fn get_ribbon_champion_g3(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 15)
    }

    fn set_ribbon_champion_g3(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 15, value);
    }

    fn get_ribbon_artist(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 18)
    }

    fn set_ribbon_artist(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 18, value);
    }

    fn get_ribbon_effort(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 19)
    }

    fn set_ribbon_effort(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 19, value);
    }
}

impl RibbonSetUnique3 for PK3 {
    fn get_ribbon_winning(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 16)
    }

    fn set_ribbon_winning(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 16, value);
    }

    fn get_ribbon_victory(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 17)
    }

    fn set_ribbon_victory(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 17, value);
    }
}

impl RibbonSetOnly3 for PK3 {
    fn get_ribbon_count_g3_cool(&self) -> u8 {
        (self.rib0 & 7) as u8
    }

    fn set_ribbon_count_g3_cool(&mut self, count: u8) {
        self.rib0 = (self.rib0 & !7) | (count as u32 & 7);
    }

    fn get_ribbon_count_g3_beauty(&self) -> u8 {
        ((self.rib0 >> 3) & 7) as u8
    }

    fn set_ribbon_count_g3_beauty(&mut self, count: u8) {
        self.rib0 = (self.rib0 & !(7 << 3)) | ((count as u32 & 7) << 3);
    }

    fn get_ribbon_count_g3_cute(&self) -> u8 {
        ((self.rib0 >> 6) & 7) as u8
    }

    fn set_ribbon_count_g3_cute(&mut self, count: u8) {
        self.rib0 = (self.rib0 & !(7 << 6)) | ((count as u32 & 7) << 6);
    }

    fn get_ribbon_count_g3_smart(&self) -> u8 {
        ((self.rib0 >> 9) & 7) as u8
    }

    fn set_ribbon_count_g3_smart(&mut self, count: u8) {
        self.rib0 = (self.rib0 & !(7 << 9)) | ((count as u32 & 7) << 9);
    }

    fn get_ribbon_count_g3_tough(&self) -> u8 {
        ((self.rib0 >> 12) & 7) as u8
    }

    fn set_ribbon_count_g3_tough(&mut self, count: u8) {
        self.rib0 = (self.rib0 & !(7 << 12)) | ((count as u32 & 7) << 12);
    }

    fn get_ribbon_world(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 26)
    }

    fn set_ribbon_world(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 26, value);
    }

    fn get_unused1(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 27)
    }

    fn set_unused1(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 27, value);
    }

    fn get_unused2(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 28)
    }

    fn set_unused2(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 28, value);
    }

    fn get_unused3(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 29)
    }

    fn set_unused3(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 29, value);
    }

    fn get_unused4(&self) -> bool {
        flag_util::get_flag_in_u32(self.rib0, 30)
    }

    fn set_unused4(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.rib0, 30, value);
    }
}

impl ContestStats for PK3 {
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

impl ContestStatsMutable for PK3 {
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

impl G3PKM<PersonalInfoG3> for PK3 {
    fn get_species_id_3(&self) -> u16 {
        self.species_id_3
    }

    fn set_species_id_3(&mut self, species: u16) {
        self.species_id_3 = species;
    }

    fn get_ability_bit(&self) -> bool {
        flag_util::get_flag_in_u32(self.iv32, 31)
    }

    fn set_ability_bit(&mut self, value: bool) {
        flag_util::set_flag_in_u32(&mut self.iv32, 31, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::pkm::pk3::PK3;
    use crate::PKM;

    const E_PK3: &[u8] = include_bytes!("../resources/tests/e.pk3");

    #[test]
    fn should_read() {
        let bytes = E_PK3.to_vec();
        let pkm: PK3 = bytes.into();
        assert_eq!(pkm.pid, 0xC673685D);
        assert_eq!(pkm.exp, 19683);
        assert_eq!(pkm.get_iv_spe(), 22);
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = E_PK3.to_vec();
        let pkm: PK3 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        assert_eq!(bytes, output.to_vec())
    }

    #[test]
    fn should_calc_checksum() {
        let bytes = E_PK3.to_vec();
        let pkm: PK3 = bytes.into();
        assert_eq!(pkm.checksum, pkm.calculate_checksum());
    }
}
