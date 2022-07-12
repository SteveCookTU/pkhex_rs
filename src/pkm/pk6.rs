use crate::personal_info_oras::PersonalInfoORAS;
use crate::ribbons::RibbonG6;
use crate::{flag_util, personal_table, poke_crypto, string_converter_6, StringConverterOption, tables};
use no_std_io::{Cursor, EndianRead, EndianWrite, StreamContainer, StreamReader, StreamWriter};

#[derive(Default, Copy, Clone, EndianRead, EndianWrite)]
pub struct PK6 {
    pub ec: u32,
    pub sanity: u16,
    pub checksum: u16,
    pub species: u16,
    pub held_item: u16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    pub ability: u8,
    pub ability_num: u8,
    pub training_bag_hits: u8,
    pub training_bag: u8,
    pub pid: u32,
    pub nature: u8,
    encounter_flags: u8,
    pub ev_hp: u8,
    pub ev_atk: u8,
    pub ev_def: u8,
    pub ev_spe: u8,
    pub ev_spa: u8,
    pub ev_spd: u8,
    pub cnt_cool: u8,
    pub cnt_beauty: u8,
    pub cnt_cute: u8,
    pub cnt_smart: u8,
    pub cnt_tough: u8,
    pub cnt_sheen: u8,
    pub mark_value: u8,
    pkrs: u8,
    pub super_train_bit_flags: u32,
    ribbon_0: u32,
    ribbon_1: u32,
    ribbon_count_memory_contest: u8,
    ribbon_count_memory_battle: u8,
    dist_byte: u16,
    pub form_argument: u32,
    nickname_trash: [u8; 26],
    pub move_1: u16,
    pub move_2: u16,
    pub move_3: u16,
    pub move_4: u16,
    pub move_1_pp: u8,
    pub move_2_pp: u8,
    pub move_3_pp: u8,
    pub move_4_pp: u8,
    pub move_1_pp_ups: u8,
    pub move_2_pp_ups: u8,
    pub move_3_pp_ups: u8,
    pub move_4_pp_ups: u8,
    pub relearn_move_1: u16,
    pub relearn_move_2: u16,
    pub relearn_move_3: u16,
    pub relearn_move_4: u16,
    super_train_progress_bits: u8,
    #[no_std_io(pad_before = 1)]
    iv32: u32,
    ht_trash: [u8; 26],
    ht_gender: u8,
    current_handler: u8,
    pub geo_1_region: u8,
    pub geo_1_country: u8,
    pub geo_2_region: u8,
    pub geo_2_country: u8,
    pub geo_3_region: u8,
    pub geo_3_country: u8,
    pub geo_4_region: u8,
    pub geo_4_country: u8,
    pub geo_5_region: u8,
    pub geo_5_country: u8,
    #[no_std_io(pad_before = 4)]
    pub ht_friendship: u8,
    pub ht_affection: u8,
    pub ht_intensity: u8,
    pub ht_memory: u8,
    pub ht_feeling: u8,
    #[no_std_io(pad_before = 1)]
    pub ht_text_var: u16,
    #[no_std_io(pad_before = 4)]
    pub fullness: u8,
    pub enjoyment: u8,
    ot_trash: [u8; 26],
    pub ot_friendship: u8,
    pub ot_affection: u8,
    pub ot_intensity: u8,
    pub ot_memory: u8,
    pub ot_text_var: u16,
    pub ot_feeling: u8,
    pub egg_year: u8,
    pub egg_month: u8,
    pub egg_day: u8,
    pub met_year: u8,
    pub met_month: u8,
    pub met_day: u8,
    #[no_std_io(pad_before = 1)]
    pub egg_location: u16,
    pub met_location: u16,
    pub ball: u8,
    met_flags: u8,
    pub ground_tile: u8,
    pub version: u8,
    pub country: u8,
    pub region: u8,
    pub console_region: u8,
    pub language: u8,
    #[no_std_io(pad_before = 4)]
    pub status_condition: u32,
    pub stat_level: u8,
    pub form_argument_remain: u8,
    pub form_argument_elapsed: u8,
    #[no_std_io(pad_before = 1)]
    pub stat_hp_current: u16,
    pub stat_hp_max: u16,
    pub stat_atk: u16,
    pub stat_def: u16,
    pub stat_spe: u16,
    pub stat_spa: u16,
    pub stat_spd: u16,
}

impl PK6 {
    pub const SIZE_PARTY: usize = poke_crypto::SIZE_6PARTY;
    pub const SIZE_STORED: usize = poke_crypto::SIZE_6STORED;

    pub const MAX_IV: u8 = 31;
    pub const MAX_EV: u8 = 252;
    pub const OT_LENGTH: usize = 12;
    pub const NICK_LENGTH: usize = 12;

    pub const MAX_MOVE_ID: usize = tables::MAX_MOVE_ID_6_AO;
    pub const MAX_SPECIES_ID: usize = tables::MAX_SPECIES_ID_6;
    pub const MAX_ABILITY_ID: usize = tables::MAX_ABILITY_ID_6_AO;
    pub const MAX_ITEM_ID: usize = tables::MAX_ITEM_ID_6_AO;
    pub const MAX_BALL_ID: usize = tables::MAX_BALL_ID_6;
    pub const MAX_GAME_ID: usize = tables::MAX_GAME_ID_6;
    pub const MARKING_COUNT: usize = 6;

    pub fn get_personal_info(&self) -> &'static PersonalInfoORAS {
        personal_table::AO.get_form_entry(self.species as usize, self.get_form() as usize)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn refresh_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    pub fn checksum_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    pub fn get_is_valid(&self) -> bool {
        self.sanity == 0 && self.checksum_valid()
    }

    pub fn set_is_valid(&mut self, value: bool) {
        if !value {
            return;
        }
        self.sanity = 0;
        self.refresh_checksum();
    }

    pub fn get_current_friendship(&self) -> u8 {
        if self.current_handler == 0 {
            self.ot_friendship
        } else {
            self.ht_friendship
        }
    }

    pub fn set_current_friendship(&mut self, value: u8) {
        if self.current_handler == 0 {
            self.ot_friendship = value;
        } else {
            self.ht_friendship = value;
        }
    }

    pub fn psv(&self) -> u32 {
        ((self.pid >> 16) ^ (self.pid & 0xFFFF)) >> 4
    }

    pub fn tsv(&self) -> u16 {
        (self.tid ^ self.sid) >> 4
    }

    pub fn is_untraded(&self) -> bool {
        self.ht_trash[0] == 0 && self.ht_trash[1] == 0
    }

    pub fn get_characteristic(&self) -> u32 {
        let pm6 = self.ec % 6;
        let max_iv = Self::MAX_IV;
        let mut pm6stat = 0;
        for i in 0..6 {
            pm6stat = (pm6 + i) % 6;
            let iv = match pm6stat {
                0 => self.get_iv_hp(),
                1 => self.get_iv_atk(),
                2 => self.get_iv_def(),
                3 => self.get_iv_spe(),
                4 => self.get_iv_spa(),
                5 => self.get_iv_spd(),
                _ => 0,
            };
            if iv == max_iv {
                break;
            }
        }
        (pm6stat * 5) + (max_iv as u32 % 5)
    }

    pub fn encrypt(&mut self) -> Vec<u8> {
        let mut pkm = *self;
        pkm.refresh_checksum();
        let bytes: Vec<u8> = pkm.into();
        poke_crypto::encrypt_array_6(&bytes)
    }

    pub fn fix_relearn(&mut self) {
        loop {
            if self.relearn_move_4 != 0 && self.relearn_move_3 == 0 {
                self.relearn_move_3 = self.relearn_move_4;
                self.relearn_move_4 = 0;
            }
            if self.relearn_move_3 != 0 && self.relearn_move_2 == 0 {
                self.relearn_move_2 = self.relearn_move_3;
                self.relearn_move_3 = 0;
                continue;
            }
            if self.relearn_move_2 != 0 && self.relearn_move_1 == 0 {
                self.relearn_move_1 = self.relearn_move_2;
                self.relearn_move_2 = 0;
                continue;
            }
            break;
        }
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0u16;
        let data: Vec<u8> = (*self).into();
        let mut reader = StreamContainer::new(data);
        reader.set_index(8);
        while reader.get_index() < poke_crypto::SIZE_6STORED {
            chk = chk.wrapping_add(reader.default_read_stream_le::<u16>())
        }
        chk
    }

    pub fn get_fateful_encounter(&self) -> bool {
        (self.encounter_flags & 1) == 1
    }

    pub fn set_fateful_encounter(&mut self, value: bool) {
        self.encounter_flags = (self.encounter_flags & !1) | if value { 1 } else { 0 };
    }

    pub fn get_gender(&self) -> u8 {
        (self.encounter_flags >> 1) & 3
    }

    pub fn set_gender(&mut self, value: u8) {
        self.encounter_flags = (self.encounter_flags & !0x6) | (value << 1);
    }

    pub fn get_form(&self) -> u8 {
        self.encounter_flags >> 3
    }

    pub fn set_form(&mut self, value: u8) {
        self.encounter_flags = (self.encounter_flags & 0x7) | (value << 3);
    }

    pub fn get_pkrs_days(&self) -> u8 {
        self.pkrs & 0xF
    }

    pub fn set_pkrs_days(&mut self, value: u8) {
        self.pkrs = (self.pkrs & !0xF) | value;
    }

    pub fn get_pkrs_strain(&self) -> u8 {
        self.pkrs >> 4
    }

    pub fn set_pkrs_strain(&mut self, value: u8) {
        self.pkrs = (self.pkrs & 0xF) | (value << 4);
    }

    pub fn get_ribbon(&self, ribbon: RibbonG6) -> bool {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => self.ribbon_0,
            _ => self.ribbon_1,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let flags = (ribbon_bits >> (section * 4)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_ribbon(&mut self, ribbon: RibbonG6, value: bool) {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => &mut self.ribbon_0,
            _ => &mut self.ribbon_1,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (if value { 1 } else { 0 } << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn get_has_contest_memory_ribbon(&self) -> bool {
        let ribbon_bits = self.ribbon_1;
        let section = 0;
        let index = 5;
        let flags = (ribbon_bits >> (section * 4)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_has_contest_memory_ribbon(&mut self, value: bool) {
        let ribbon_bits = &mut self.ribbon_1;
        let section = 0;
        let index = 5;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (if value { 1 } else { 0 } << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn get_has_battle_memory_ribbon(&self) -> bool {
        let ribbon_bits = self.ribbon_1;
        let section = 0;
        let index = 6;
        let flags = (ribbon_bits >> (section * 4)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_has_battle_memory_ribbon(&mut self, value: bool) {
        let ribbon_bits = &mut self.ribbon_1;
        let section = 0;
        let index = 6;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (if value { 1 } else { 0 } << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn get_ribbon_count_memory_contest(&self) -> u8 {
        self.ribbon_count_memory_contest
    }

    pub fn set_ribbon_count_memory_contest(&mut self, value: u8) {
        self.ribbon_count_memory_contest = value;
        self.set_has_contest_memory_ribbon(value != 0);
    }

    pub fn get_ribbon_count_memory_battle(&self) -> u8 {
        self.ribbon_count_memory_battle
    }

    pub fn set_ribbon_count_memory_battle(&mut self, value: u8) {
        self.ribbon_count_memory_battle = value;
        self.set_has_battle_memory_ribbon(value != 0);
    }

    pub fn get_form_argument_maximum(&self) -> u8 {
        self.form_argument as u8
    }

    pub fn set_form_argument_maximum(&mut self, value: u8) {
        self.form_argument = value as u32 & 0xFF
    }

    pub fn get_nickname(&self) -> String {
        string_converter_6::get_string(&self.nickname_trash)
    }

    pub fn set_nickname(&mut self, value: String) {
        string_converter_6::set_string(
            &mut self.nickname_trash,
            value.chars().collect(),
            12,
            StringConverterOption::None,
        );
    }

    pub fn get_secret_super_training_unlocked(&self) -> bool {
        (self.super_train_progress_bits & 1) == 1
    }

    pub fn set_secret_super_training_unlocked(&mut self, value: bool) {
        self.super_train_progress_bits =
            (self.super_train_progress_bits & !1) | if value { 1 } else { 0 };
    }

    pub fn get_secret_super_training_complete(&self) -> bool {
        (self.super_train_progress_bits & 2) == 2
    }

    pub fn set_secret_super_training_complete(&mut self, value: bool) {
        self.super_train_progress_bits =
            (self.super_train_progress_bits & !2) | if value { 2 } else { 0 };
    }

    pub fn get_iv_hp(&self) -> u8 {
        (self.iv32 & 0x1F) as u8
    }

    pub fn set_iv_hp(&mut self, value: u8) {
        self.iv32 = (self.iv32 & !0x1F) | if value > 31 { 31 } else { value as u32 };
    }

    pub fn get_iv_atk(&self) -> u8 {
        ((self.iv32 >> 5) & 0x1F) as u8
    }

    pub fn set_iv_atk(&mut self, value: u8) {
        self.iv32 = (self.iv32 & !(0x1F << 5)) | (if value > 31 { 31 } else { value as u32 } << 5);
    }

    pub fn get_iv_def(&self) -> u8 {
        ((self.iv32 >> 10) & 0x1F) as u8
    }

    pub fn set_iv_def(&mut self, value: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 10)) | (if value > 31 { 31 } else { value as u32 } << 10);
    }

    pub fn get_iv_spe(&self) -> u8 {
        ((self.iv32 >> 15) & 0x1F) as u8
    }

    pub fn set_iv_spe(&mut self, value: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 15)) | (if value > 31 { 31 } else { value as u32 } << 15);
    }

    pub fn get_iv_spa(&self) -> u8 {
        ((self.iv32 >> 20) & 0x1F) as u8
    }

    pub fn set_iv_spa(&mut self, value: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 20)) | (if value > 31 { 31 } else { value as u32 } << 20);
    }

    pub fn get_iv_spd(&self) -> u8 {
        ((self.iv32 >> 25) & 0x1F) as u8
    }

    pub fn set_iv_spd(&mut self, value: u8) {
        self.iv32 =
            (self.iv32 & !(0x1F << 25)) | (if value > 31 { 31 } else { value as u32 } << 25);
    }

    pub fn get_is_egg(&self) -> bool {
        ((self.iv32 >> 30) & 1) == 1
    }

    pub fn set_is_egg(&mut self, value: bool) {
        self.iv32 = (self.iv32 & !0x40000000) | if value { 0x40000000 } else { 0 };
    }

    pub fn get_is_nicknamed(&self) -> bool {
        ((self.iv32 >> 31) & 1) == 1
    }

    pub fn set_is_nicknamed(&mut self, value: bool) {
        self.iv32 = (self.iv32 & !0x7FFFFFFF) | if value { 0x80000000 } else { 0 };
    }

    pub fn get_ht_name(&self) -> String {
        string_converter_6::get_string(&self.ht_trash)
    }

    pub fn set_ht_name(&mut self, value: String) {
        string_converter_6::set_string(
            &mut self.ht_trash,
            value.chars().collect(),
            12,
            StringConverterOption::None,
        );
    }

    pub fn get_ot_name(&self) -> String {
        string_converter_6::get_string(&self.ot_trash)
    }

    pub fn set_ot_name(&mut self, value: String) {
        string_converter_6::set_string(
            &mut self.ot_trash,
            value.chars().collect(),
            12,
            StringConverterOption::None,
        );
    }

    pub fn get_met_level(&self) -> u8 {
        self.met_flags & !0x80
    }

    pub fn set_met_level(&mut self, value: u8) {
        self.met_flags = (self.met_flags & 0x80) | value;
    }

    pub fn get_ot_gender(&self) -> u8 {
        self.met_flags >> 7
    }

    pub fn set_ot_gender(&mut self, value: u8) {
        self.met_flags = (self.met_flags & !0x80) | (value << 7);
    }

    pub fn super_training_medal_count(&self, max_count: usize) -> usize {
        let mut value = self.super_train_bit_flags >> 2;
        let mut train_count = 0;
        for _ in 0..max_count {
            if (value & 1) != 0 {
                train_count += 1;
            }
            value >>= 1;
        }
        train_count
    }
}

impl From<Vec<u8>> for PK6 {
    fn from(mut data: Vec<u8>) -> Self {
        poke_crypto::decrypt_if_encrypted_67(&mut data);
        data.resize(poke_crypto::SIZE_6PARTY, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_le::<PK6>()
    }
}

impl From<PK6> for Vec<u8> {
    fn from(pkm: PK6) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_6PARTY];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_le(&pkm);
        writer.into_raw()
    }
}

#[cfg(test)]
mod tests {
    use crate::pkm::pk6::PK6;

    const X_PK6: &[u8] = include_bytes!("../resources/tests/x.pk6");

    #[test]
    fn should_read() {
        let bytes = X_PK6.to_vec();
        let pkm: PK6 = bytes.into();
        assert_eq!(pkm.pid, 0xD64558F2);
        assert_eq!(pkm.exp, 23760);
        assert_eq!(pkm.ball, 4);
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = X_PK6.to_vec();
        let pkm: PK6 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        assert_eq!(bytes, output.to_vec())
    }

    #[test]
    fn should_calc_checksum() {
        let bytes = X_PK6.to_vec();
        let pkm: PK6 = bytes.clone().into();
        assert_eq!(pkm.checksum, pkm.calculate_checksum());
    }
}
