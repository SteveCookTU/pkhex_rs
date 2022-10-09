use crate::personal_info_bdsp::PersonalInfoBDSP;
use crate::ribbons::{MarkG8, RibbonG8};
use crate::tables::locations;
use crate::{flag_util, personal_table, poke_crypto, string_converter_8, StringConverterOption};
use no_std_io::{Cursor, EndianRead, EndianWrite, StreamContainer, StreamReader, StreamWriter};

#[derive(Default, Copy, Clone, EndianRead, EndianWrite)]
pub struct PB8 {
    pub ec: u32,
    pub sanity: u16,
    pub checksum: u16,
    pub species: u16,
    pub held_item: u16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    pub ability: u16,
    ability_bits: u8,
    #[no_std_io(pad_before = 1)]
    pub mark_value: u16,
    #[no_std_io(pad_before = 2)]
    pub pid: u32,
    pub nature: u8,
    pub stat_nature: u8,
    pub encounter_flags: u8,
    #[no_std_io(pad_before = 1)]
    pub form: u16,
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
    pkrs: u8,
    #[no_std_io(pad_before = 1)]
    ribbon_0: u32,
    ribbon_1: u32,
    ribbon_count_memory_contest: u8,
    ribbon_count_memory_battle: u8,
    #[no_std_io(pad_before = 2)]
    ribbon_2: u32,
    ribbon_3: u32,
    pub sociability: u32,
    #[no_std_io(pad_before = 4)]
    pub height_scalar: u8,
    pub weight_scalar: u8,
    dpr_illegal_flag: u8,
    #[no_std_io(pad_before = 5)]
    pub nickname_trash: [u8; 26],
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
    pub stat_hp_current: u16,
    iv32: u32,
    pub dynamax_level: u8,
    #[no_std_io(pad_before = 3)]
    pub status_condition: i32,
    pub palma: i32,
    #[no_std_io(pad_before = 12)]
    pub ht_trash: [u8; 26],
    pub ht_gender: u8,
    pub ht_language: u8,
    pub current_handler: u8,
    #[no_std_io(pad_before = 1)]
    pub ht_trainer_id: u16,
    pub ht_friendship: u8,
    pub ht_intensity: u8,
    pub ht_memory: u8,
    pub ht_feeling: u8,
    pub ht_text_var: u16,
    poke_job_flags: [u8; 14],
    pub fullness: u8,
    pub enjoyment: u8,
    pub version: u8,
    pub battle_version: u8,
    #[no_std_io(pad_before = 2)]
    pub language: u8,
    #[no_std_io(pad_before = 1)]
    pub form_argument: u32,
    pub affixed_ribbon: i8,
    #[no_std_io(pad_before = 15)]
    pub ot_trash: [u8; 26],
    pub ot_friendship: u8,
    pub ot_memory: u8,
    #[no_std_io(pad_before = 1)]
    pub ot_text_var: u16,
    pub ot_feeling: u16,
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
    pub hyper_train_flags: u8,
    move_record_flags: [u8; 14],
    pub tracker: u64,
    #[no_std_io(pad_before = 11)]
    pub stat_level: u8,
    #[no_std_io(pad_before = 1)]
    pub stat_hp_max: u16,
    pub stat_atk: u16,
    pub stat_def: u16,
    pub stat_spe: u16,
    pub stat_spa: u16,
    pub stat_spd: u16,
}

impl From<Vec<u8>> for PB8 {
    fn from(mut data: Vec<u8>) -> Self {
        poke_crypto::decrypt_if_encrypted_8(&mut data);
        data.resize(poke_crypto::SIZE_8PARTY, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_le::<PB8>()
    }
}

impl From<PB8> for Vec<u8> {
    fn from(pkm: PB8) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_8PARTY];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_le(&pkm);
        writer.into_raw()
    }
}

impl PB8 {
    const SIZE_PARTY: usize = poke_crypto::SIZE_8PARTY;
    const SIZE_STORED: usize = poke_crypto::SIZE_8STORED;
    const MAX_IV: u8 = 31;
    const MAX_EV: u8 = 252;
    const OT_LENGTH: usize = 12;
    const NICK_LENGTH: usize = 12;

    pub fn new() -> Self {
        Self {
            affixed_ribbon: -1,
            egg_location: locations::DEFAULT_8B_NONE as u16,
            met_location: locations::DEFAULT_8B_NONE as u16,
            ..Default::default()
        }
    }

    pub fn get_personal_info(&self) -> &'static PersonalInfoBDSP {
        personal_table::BDSP.get_form_entry(self.species as usize, self.form as usize)
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0u16;
        let data: Vec<u8> = (*self).into();
        let mut reader = StreamContainer::new(data);
        reader.set_index(8);
        while reader.get_index() < poke_crypto::SIZE_8STORED {
            chk = chk.wrapping_add(reader.default_read_stream_le::<u16>())
        }
        chk
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

    pub fn checksum_valid(&self) -> bool {
        self.calculate_checksum() == self.checksum
    }

    pub fn refresh_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    pub fn get_valid(&self) -> bool {
        self.sanity == 0 && self.checksum_valid()
    }

    pub fn set_is_valid(&mut self, value: bool) {
        if !value {
            return;
        }

        self.sanity = 0;
        self.refresh_checksum();
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
        poke_crypto::encrypt_array_8(&bytes)
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

    pub fn get_ability_num(&self) -> u8 {
        self.ability_bits & 7
    }

    pub fn set_ability_num(&mut self, value: u8) {
        self.ability_bits = (self.ability_bits & !7) | (value & 7);
    }

    pub fn get_favorite(&self) -> bool {
        (self.ability_bits & 8) != 0
    }

    pub fn set_favorite(&mut self, value: bool) {
        self.ability_bits = (self.ability_bits & !8) | (u8::from(value) << 3);
    }

    pub fn get_can_gigantamax(&self) -> bool {
        (self.ability_bits & 16) != 0
    }

    pub fn set_can_gigantamax(&mut self, value: bool) {
        self.ability_bits = (self.ability_bits & !16) | if value { 16 } else { 0 };
    }

    pub fn get_fateful_encounter(&self) -> bool {
        (self.encounter_flags & 1) == 1
    }

    pub fn set_fateful_encounter(&mut self, value: bool) {
        self.encounter_flags = (self.encounter_flags & !1) | u8::from(value);
    }

    pub fn get_flag_2(&self) -> bool {
        (self.encounter_flags & 2) == 2
    }

    pub fn set_flag_2(&mut self, value: bool) {
        self.encounter_flags = (self.encounter_flags & !2) | if value { 2 } else { 0 };
    }

    pub fn get_gender(&self) -> u8 {
        (self.encounter_flags >> 2) & 3
    }

    pub fn set_gender(&mut self, value: u8) {
        self.encounter_flags = (self.encounter_flags & 0xF3) | (value << 2);
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

    pub fn get_ribbon(&self, ribbon: RibbonG8) -> bool {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => self.ribbon_0,
            1 => self.ribbon_1,
            2 => self.ribbon_2,
            _ => self.ribbon_3,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let flags = (ribbon_bits >> (section * 8)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_ribbon(&mut self, ribbon: RibbonG8, value: bool) {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => &mut self.ribbon_0,
            1 => &mut self.ribbon_1,
            2 => &mut self.ribbon_2,
            _ => &mut self.ribbon_3,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (u32::from(value) << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn get_has_contest_memory_ribbon(&self) -> bool {
        let ribbon_bits = self.ribbon_1;
        let section = 0;
        let index = 5;
        let flags = (ribbon_bits >> (section * 8)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_has_contest_memory_ribbon(&mut self, value: bool) {
        let ribbon_bits = &mut self.ribbon_1;
        let section = 0;
        let index = 5;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (u32::from(value) << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn get_has_battle_memory_ribbon(&self) -> bool {
        let ribbon_bits = self.ribbon_1;
        let section = 0;
        let index = 6;
        let flags = (ribbon_bits >> (section * 8)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_has_battle_memory_ribbon(&mut self, value: bool) {
        let ribbon_bits = &mut self.ribbon_1;
        let section = 0;
        let index = 6;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (u32::from(value) << (section * 4 + index));
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

    pub fn get_mark(&self, mark: MarkG8) -> bool {
        let location = mark as u32;
        let ribbon_bits = match location >> 8 {
            0 => self.ribbon_0,
            1 => self.ribbon_1,
            2 => self.ribbon_2,
            _ => self.ribbon_3,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let flags = (ribbon_bits >> (section * 8)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_mark(&mut self, mark: MarkG8, value: bool) {
        let location = mark as u32;
        let ribbon_bits = match location >> 8 {
            0 => &mut self.ribbon_0,
            1 => &mut self.ribbon_1,
            2 => &mut self.ribbon_2,
            _ => &mut self.ribbon_3,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (u32::from(value) << (section * 4 + index));
        *ribbon_bits = new_value;
    }

    pub fn has_mark(&self) -> bool {
        if ((self.ribbon_1 >> 8) & 0xFFE0) != 0 {
            return true;
        }

        if self.ribbon_3 != 0 {
            return true;
        }

        self.ribbon_3 & 3 != 0
    }

    pub fn get_nickname(&self) -> String {
        string_converter_8::get_string(&self.nickname_trash)
    }

    pub fn set_nickname(&mut self, value: String) {
        string_converter_8::set_string(
            &mut self.nickname_trash,
            value.chars().collect(),
            12,
            StringConverterOption::None,
        );
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
        string_converter_8::get_string(&self.ht_trash)
    }

    pub fn set_ht_name(&mut self, value: String) {
        string_converter_8::set_string(
            &mut self.ht_trash,
            value.chars().collect(),
            12,
            StringConverterOption::None,
        );
    }

    pub fn get_poke_job_flag(&self, index: usize) -> bool {
        if index > 112 {
            return false;
        }
        let ofs = index >> 3;
        flag_util::get_flag(&self.poke_job_flags, ofs, index & 7)
    }

    pub fn set_poke_job_flag(&mut self, index: usize, value: bool) {
        if index > 112 {
            return;
        }
        let ofs = index >> 3;
        flag_util::set_flag(&mut self.poke_job_flags, ofs, index & 7, value);
    }

    pub fn get_poke_job_flag_any(&self) -> bool {
        self.poke_job_flags.iter().any(|i| *i != 0)
    }

    pub fn clear_poke_job_flags(&mut self) {
        self.poke_job_flags = [0; 14];
    }

    pub fn get_form_argument_remain(&self) -> u8 {
        self.form_argument as u8
    }

    pub fn set_form_argument_remain(&mut self, value: u8) {
        self.form_argument = (self.form_argument & !0xFF) | value as u32;
    }

    pub fn get_form_argument_elapsed(&self) -> u8 {
        (self.form_argument >> 8) as u8
    }

    pub fn set_form_argument_elapsed(&mut self, value: u8) {
        self.form_argument = (self.form_argument & !0xFF00) | ((value as u32) << 8);
    }

    pub fn get_form_argument_maximum(&self) -> u8 {
        (self.form_argument >> 16) as u8
    }

    pub fn set_form_argument_maximum(&mut self, value: u8) {
        self.form_argument = (self.form_argument & !0xFF0000) | ((value as u32) << 16);
    }

    pub fn get_ot_name(&self) -> String {
        string_converter_8::get_string(&self.ot_trash)
    }

    pub fn set_ot_name(&mut self, value: String) {
        string_converter_8::set_string(
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

    pub fn get_ht_hp(&self) -> bool {
        (self.hyper_train_flags & 1) == 1
    }

    pub fn set_ht_hp(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !1) | u8::from(value);
    }

    pub fn get_ht_atk(&self) -> bool {
        ((self.hyper_train_flags >> 1) & 1) == 1
    }

    pub fn set_ht_atk(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !(1 << 1)) | (u8::from(value) << 1);
    }

    pub fn get_ht_def(&self) -> bool {
        ((self.hyper_train_flags >> 2) & 1) == 1
    }

    pub fn set_ht_def(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !(1 << 2)) | (u8::from(value) << 2);
    }

    pub fn get_ht_spa(&self) -> bool {
        ((self.hyper_train_flags >> 3) & 1) == 1
    }

    pub fn set_ht_spa(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !(1 << 3)) | (u8::from(value) << 3);
    }

    pub fn get_ht_spd(&self) -> bool {
        ((self.hyper_train_flags >> 4) & 1) == 1
    }

    pub fn set_ht_spd(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !(1 << 4)) | (u8::from(value) << 4);
    }

    pub fn get_ht_spe(&self) -> bool {
        ((self.hyper_train_flags >> 5) & 1) == 1
    }

    pub fn set_ht_spe(&mut self, value: bool) {
        self.hyper_train_flags = (self.hyper_train_flags & !(1 << 5)) | (u8::from(value) << 5);
    }

    pub fn get_move_record_flag(&self, index: usize) -> bool {
        if index > 112 {
            return false;
        }
        let ofs = index >> 3;
        flag_util::get_flag(&self.move_record_flags, ofs, index & 7)
    }

    pub fn set_move_record_flag(&mut self, index: usize, value: bool) {
        if index > 112 {
            return;
        }
        let ofs = index >> 3;
        flag_util::set_flag(&mut self.move_record_flags, ofs, index & 7, value);
    }

    pub fn get_move_record_flag_any(&self) -> bool {
        self.move_record_flags.iter().any(|i| *i != 0)
    }

    pub fn clear_move_record_flags(&mut self) {
        self.move_record_flags = [0; 14];
    }
}

#[cfg(test)]
mod tests {
    use crate::pkm::pb8::PB8;

    const BDSP_PB8: &[u8] = include_bytes!("../resources/tests/bdsp.pb8");

    #[test]
    fn should_read() {
        let bytes = BDSP_PB8.to_vec();
        let pkm: PB8 = bytes.into();
        assert_eq!(pkm.pid, 0xCA68C597);
        assert_eq!(pkm.exp, 125971);
        assert_eq!(pkm.height_scalar, 77);
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = BDSP_PB8.to_vec();
        let pkm: PB8 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        assert_eq!(bytes, output.to_vec())
    }

    #[test]
    fn should_calc_checksum() {
        let bytes = BDSP_PB8.to_vec();
        let pkm: PB8 = bytes.clone().into();
        assert_eq!(pkm.checksum, pkm.calculate_checksum());
    }
}
