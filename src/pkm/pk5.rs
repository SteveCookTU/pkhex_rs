use crate::personal_info_b2w2::PersonalInfoB2W2;
use crate::ribbons::RibbonG5;
use crate::{flag_util, personal_table, poke_crypto, string_converter_5, StringConverterOption};
use no_std_io::{Cursor, EndianRead, EndianWrite, StreamContainer, StreamReader, StreamWriter};

#[derive(Copy, Clone, EndianRead, EndianWrite)]
pub struct PK5 {
    pub pid: u32,
    pub sanity: u16,
    pub checksum: u16,
    pub species: u16,
    pub held_item: u16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    pub ot_friendship: u8,
    pub ability: u8,
    pub mark_value: u8,
    pub language: u8,
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
    ribbon_0: u32,
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
    iv32: u32,
    ribbon_1: u32,
    encounter_flags: u8,
    pub nature: u8,
    ability_flags: u8,
    #[no_std_io(pad_before = 5)]
    nickname_trash: [u8; 22],
    #[no_std_io(pad_before = 1)]
    pub version: u8,
    ribbon_2: u32,
    #[no_std_io(pad_before = 4)]
    ot_trash: [u8; 16],
    pub egg_year: u8,
    pub egg_month: u8,
    pub egg_day: u8,
    pub met_year: u8,
    pub met_month: u8,
    pub met_day: u8,
    pub egg_location: u16,
    pub met_location: u16,
    pkrs: u8,
    pub ball: u8,
    met_flags: u8,
    pub ground_tile: u8,
    #[no_std_io(pad_before = 1)]
    pub poke_star_frame: u8,
    pub status_condition: u32,
    pub stat_level: u8,
    pub junk_byte: u8,
    pub stat_hp_current: u16,
    pub stat_hp_max: u16,
    pub stat_atk: u16,
    pub stat_def: u16,
    pub stat_spe: u16,
    pub stat_spa: u16,
    pub stat_spd: u16,
    pub held_mail: [u8; 0x38],
    pub junk_data: u64,
}

impl Default for PK5 {
    fn default() -> Self {
        Self {
            pid: 0,
            sanity: 0,
            checksum: 0,
            species: 0,
            held_item: 0,
            tid: 0,
            sid: 0,
            exp: 0,
            ot_friendship: 0,
            ability: 0,
            mark_value: 0,
            language: 0,
            ev_hp: 0,
            ev_atk: 0,
            ev_def: 0,
            ev_spe: 0,
            ev_spa: 0,
            ev_spd: 0,
            cnt_cool: 0,
            cnt_beauty: 0,
            cnt_cute: 0,
            cnt_smart: 0,
            cnt_tough: 0,
            cnt_sheen: 0,
            ribbon_0: 0,
            move_1: 0,
            move_2: 0,
            move_3: 0,
            move_4: 0,
            move_1_pp: 0,
            move_2_pp: 0,
            move_3_pp: 0,
            move_4_pp: 0,
            move_1_pp_ups: 0,
            move_2_pp_ups: 0,
            move_3_pp_ups: 0,
            move_4_pp_ups: 0,
            iv32: 0,
            ribbon_1: 0,
            encounter_flags: 0,
            nature: 0,
            ability_flags: 0,
            nickname_trash: [0; 22],
            version: 0,
            ribbon_2: 0,
            ot_trash: [0; 16],
            egg_year: 0,
            egg_month: 0,
            egg_day: 0,
            met_year: 0,
            met_month: 0,
            met_day: 0,
            egg_location: 0,
            met_location: 0,
            pkrs: 0,
            ball: 0,
            met_flags: 0,
            ground_tile: 0,
            poke_star_frame: 0,
            status_condition: 0,
            stat_level: 0,
            junk_byte: 0,
            stat_hp_current: 0,
            stat_hp_max: 0,
            stat_atk: 0,
            stat_def: 0,
            stat_spe: 0,
            stat_spa: 0,
            stat_spd: 0,
            held_mail: [0; 0x38],
            junk_data: 0,
        }
    }
}

impl From<Vec<u8>> for PK5 {
    fn from(mut data: Vec<u8>) -> Self {
        poke_crypto::decrypt_if_encrypted_45(&mut data);
        data.resize(poke_crypto::SIZE_5PARTY, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_le::<PK5>()
    }
}

impl From<PK5> for Vec<u8> {
    fn from(pkm: PK5) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_5PARTY];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_le(&pkm);
        writer.into_raw()
    }
}

impl PK5 {
    pub const MAX_IV: u8 = 31;
    pub const MAX_EV: u8 = 252;

    pub fn new() -> Self {
        Default::default()
    }

    pub fn refresh_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    pub fn checksum_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    pub fn get_valid(&self) -> bool {
        self.sanity == 0 && self.checksum_valid()
    }

    pub fn set_valid(&mut self, value: bool) {
        if !value {
            return;
        }

        self.sanity = 0;
        self.refresh_checksum();
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0u16;
        let data: Vec<u8> = (*self).into();
        let mut reader = StreamContainer::new(data);
        reader.set_index(8);
        while reader.get_index() < poke_crypto::SIZE_5STORED {
            chk = chk.wrapping_add(reader.default_read_stream_le::<u16>())
        }
        chk
    }

    pub fn encrypt(&mut self) -> Vec<u8> {
        let mut pkm = *self;
        pkm.refresh_checksum();
        let bytes: Vec<u8> = pkm.into();
        poke_crypto::encrypt_array_45(&bytes)
    }

    pub fn get_ec(&self) -> u32 {
        self.pid
    }

    pub fn psv(&self) -> u32 {
        ((self.pid >> 16) ^ (self.pid & 0xFFFF)) >> 3
    }

    pub fn tsv(&self) -> u16 {
        (self.tid ^ self.sid) >> 3
    }

    pub fn get_ability_number(&self) -> usize {
        if self.get_hidden_ability() {
            4
        } else {
            1 << ((self.pid as usize >> 16) & 1)
        }
    }

    pub fn get_characteristic(&self) -> u32 {
        let pm6 = self.pid % 6;
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

    pub fn get_personal_info(&self) -> &'static PersonalInfoB2W2 {
        personal_table::B2W2.get_form_entry(self.species as usize, self.get_form() as usize)
    }

    pub fn get_ribbon(&self, ribbon: RibbonG5) -> bool {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => self.ribbon_0,
            1 => self.ribbon_1,
            _ => self.ribbon_2,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let flags = (ribbon_bits >> (section * 4)) as u8;
        flag_util::get_flag_from_u8(flags, index as usize)
    }

    pub fn set_ribbon(&mut self, ribbon: RibbonG5, value: bool) {
        let location = ribbon as u32;
        let ribbon_bits = match location >> 8 {
            0 => &mut self.ribbon_0,
            1 => &mut self.ribbon_1,
            _ => &mut self.ribbon_2,
        };
        let section = (location >> 4) & 0xF;
        let index = location & 0xF;
        let current = *ribbon_bits & !(1 << (section * 4 + index));
        let new_value = current | (if value { 1 } else { 0 } << (section * 4 + index));
        *ribbon_bits = new_value;
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

    pub fn get_hidden_ability(&self) -> bool {
        (self.ability_flags & 1) == 1
    }

    pub fn set_hidden_ability(&mut self, value: bool) {
        self.ability_flags = (self.ability_flags & !1) | if value { 1 } else { 0 };
    }

    pub fn get_n_sparkle(&self) -> bool {
        (self.ability_flags & 2) == 2
    }

    pub fn set_n_sparkle(&mut self, value: bool) {
        self.ability_flags = (self.ability_flags & !2) | if value { 2 } else { 0 };
    }

    pub fn get_nickname(&self) -> String {
        string_converter_5::get_string(&self.nickname_trash)
    }

    pub fn set_nickname(&mut self, value: String) {
        string_converter_5::set_string(
            &mut self.nickname_trash,
            value.chars().collect(),
            10,
            StringConverterOption::None,
        );
    }

    pub fn get_ot_name(&self) -> String {
        string_converter_5::get_string(&self.ot_trash)
    }

    pub fn set_ot_name(&mut self, value: String) {
        string_converter_5::set_string(
            &mut self.ot_trash,
            value.chars().collect(),
            7,
            StringConverterOption::None,
        );
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

    pub fn get_is_poke_star(&self) -> bool {
        self.poke_star_frame > 250
    }

    pub fn set_is_poke_star(&mut self, value: bool) {
        self.poke_star_frame = if value { 255 } else { 0 };
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
}

#[cfg(test)]
mod tests {
    use crate::pkm::pk5::PK5;

    const B2W2_PK5: &[u8] = include_bytes!("../resources/tests/b2w2.pk5");

    #[test]
    fn should_read() {
        let bytes = B2W2_PK5.to_vec();
        let pkm: PK5 = bytes.into();
        assert_eq!(pkm.pid, 0xFAEE65EF);
        assert_eq!(pkm.exp, 100);
        assert_eq!(pkm.ball, 4);
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = B2W2_PK5.to_vec();
        let pkm: PK5 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        assert_eq!(bytes, output.to_vec())
    }

    #[test]
    fn should_calc_checksum() {
        let bytes = B2W2_PK5.to_vec();
        let pkm: PK5 = bytes.clone().into();
        assert_eq!(pkm.checksum, pkm.calculate_checksum());
    }
}
