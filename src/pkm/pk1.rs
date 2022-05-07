use crate::personal_info_g1::PersonalInfoG1;
use crate::personal_table::Y;
use crate::poke_crypto::{SIZE_1PARTY, SIZE_1STORED};
use crate::string_converter_12::G1_TERMINATOR_CODE;
use crate::{personal_info, species_converter, species_name, string_converter_12, string_converter_2_kor, GBPkm, GBPkml, GameValueLimit, GameVersion, Generation, HyperTrain, LangNick, LanguageID, NatureT, PersonalInfo, Pkm, PokeList, PokeList1, Shiny, ShinyEnum, SpeciesForm, StringConverterOption, TrainerId, PK6, Species, PKMType, ContestStats};
use time::PrimitiveDateTime;
use crate::tables::HELD_ITEMS_GSC;

pub struct PK1 {
    data: Vec<u8>,
    raw_ot: Vec<u8>,
    raw_nickname: Vec<u8>,
}

impl PK1 {
    fn new(data: Vec<u8>, jp: bool) -> Self {
        <PK1 as GBPkml<PersonalInfoG1>>::new(PK1::ensure_party_size(data), jp)
    }

    fn get_stat(&self, base_state: usize, iv: usize, mut effort: usize, level: usize) -> u16 {
        effort = (255u16.min(((effort as f32).sqrt() + 1.0) as u16) >> 2) as usize;
        ((((2 * (base_state + iv)) + effort) * level / 100) + 5) as u16
    }

    fn set_string(
        &self,
        value: &[char],
        dest_buffer: &mut [u8],
        max_length: usize,
        option: StringConverterOption,
    ) -> usize {
        if self.korean() {
            string_converter_2_kor::set_string(dest_buffer, value, max_length, option)
        } else {
            string_converter_12::set_string(dest_buffer, value, max_length, self.japanese(), option)
        }
    }

    fn set_string_keep_terminator_style(&self, value: &[char], exist: &mut [u8]) {
        let zeroed = exist.iter().any(|z| *z == 0);
        let fill = if zeroed { 0 } else { G1_TERMINATOR_CODE };
        for ex in exist.iter_mut() {
            *ex = fill;
        }

        let final_length = exist.len().min(value.len() + 1);
        self.set_string(value, exist, final_length, StringConverterOption::None);
    }

    fn ensure_party_size(mut data: Vec<u8>) -> Vec<u8> {
        if data.len() != SIZE_1PARTY {
            data.resize(SIZE_1PARTY, 0);
        }
        data
    }

    pub fn get_species_id1(&self) -> u8 {
        self.data[0]
    }

    pub fn set_species_id1(&mut self, species_id: u8) {
        self.data[0] = species_id;
    }

    pub fn get_type_a(&self) -> u8 {
        self.data[5]
    }

    pub fn set_type_a(&mut self, type_a: u8) {
        self.data[5] = type_a;
    }

    pub fn get_type_b(&self) -> u8 {
        self.data[6]
    }

    pub fn set_type_b(&mut self, type_b: u8) {
        self.data[6] = type_b;
    }

    pub fn get_catch_rate(&self) -> u8 {
        self.data[7]
    }

    pub fn set_catch_rate(&mut self, rate: u8) {
        self.data[7] = rate;
    }

    pub fn is_catch_rate_held_item(&self, rate: usize) -> bool {
        HELD_ITEMS_GSC.contains(&(rate as u16))
    }

    fn set_species_values(&mut self, value: usize) {
        let updated = species_converter::set_g1_species(value);
        if self.get_species_id1() == updated {
            return;
        }

        self.set_species_id1(updated);

        self.set_type_a(self.get_personal_info().get_type_1() as u8);
        self.set_type_b(self.get_personal_info().get_type_2() as u8);

        let rate = self.get_catch_rate();
        if (rate == 0 || self.is_catch_rate_held_item(rate as usize)) || (value == Species::Pikachu as usize && rate == 0xA3) {
            return;
        }

        todo!()
    }
}

impl GBPkm<PersonalInfoG1> for PK1 {
    fn get_non_nickname(&self, language: usize) -> Vec<u8> {
        let name =
            species_name::get_species_name_generation(self.get_species(), language, self.format());
        let len = self.nickname_trash().len();
        let mut data = vec![0; len];
        self.set_string(
            &name.chars().collect::<Vec<char>>(),
            &mut data,
            len,
            StringConverterOption::Clear50,
        );
        if !self.korean() {
            for i in data.iter_mut() {
                if *i == 0xF2 {
                    *i = 0xE8;
                }
            }
        }
        data
    }

    fn set_not_nicknamed(&mut self, language: usize) {
        let data = self.get_non_nickname(language);
        self.raw_nickname.splice(.., data);
    }

    fn get_ev_spc(&self) -> usize {
        todo!()
    }

    fn set_ev_spc(&mut self, spc: usize) {
        todo!()
    }

    fn get_dv16(&self) -> u16 {
        todo!()
    }

    fn set_dv16(&mut self, dv16: u16) {
        todo!()
    }
}

impl Pkm<PersonalInfoG1> for PK1 {

    fn size_party(&self) -> usize {
        SIZE_1PARTY
    }

    fn size_stored(&self) -> usize {
        SIZE_1STORED
    }

    fn get_type(&self) -> PKMType {
        PKMType::PK1
    }

    fn get_personal_info(&self) -> &PersonalInfoG1 {
        &Y[self.get_species()]
    }

    fn get_data(&self) -> &Vec<u8> {
        todo!()
    }

    fn new(data: Vec<u8>) -> Self {
        todo!()
    }

    fn new_blank() -> Self {
        todo!()
    }

    fn encrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn encrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn decrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn decrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()
    }

    fn set_valid(&mut self, _valid: bool) {}

    fn get_valid(&self) -> bool {
        self.get_species() <= 151 && (self.data[0] == 0 || self.get_species() != 0)
    }

    fn nickname_trash(&self) -> Vec<u8> {
        self.raw_nickname.clone()
    }

    fn ot_trash(&self) -> Vec<u8> {
        self.raw_ot.clone()
    }

    fn encrypt(&mut self) -> Vec<u8> {
        PokeList1::new_from_pk(self.clone()).write()
    }

    fn format(&self) -> usize {
        todo!()
    }

    fn get_held_item(&self) -> usize {
        todo!()
    }

    fn set_held_item(&mut self, held_item: usize) {
        todo!()
    }

    fn get_gender(&self) -> usize {
        let gv = self.get_personal_info().get_gender();
        match gv {
            personal_info::RATIO_MAGIC_GENDERLESS => 2,
            personal_info::RATIO_MAGIC_FEMALE => 1,
            personal_info::RATIO_MAGIC_MALE => 0,
            _ => {
                if self.get_iv_atk() > (gv >> 4) {
                    0
                } else {
                    1
                }
            }
        }
    }

    fn set_gender(&mut self, _gender: usize) {}

    fn get_ability(&self) -> usize {
        usize::MAX
    }

    fn set_ability(&mut self, _ability: usize) {}

    fn get_current_friendship(&self) -> usize {
        todo!()
    }

    fn set_current_friendship(&mut self, current_friendship: usize) {
        todo!()
    }

    fn get_is_egg(&self) -> bool {
        todo!()
    }

    fn set_is_egg(&mut self, is_egg: bool) {
        todo!()
    }

    fn get_exp(&self) -> usize {
        todo!()
    }

    fn set_exp(&mut self, exp: usize) {
        todo!()
    }

    fn get_ot_name(&self) -> String {
        if self.korean() {
            string_converter_2_kor::get_string(&self.raw_nickname)
        } else {
            string_converter_12::get_string(&self.raw_nickname, self.japanese())
        }
    }

    fn set_ot_name(&mut self, ot_name: String) {
        if self.get_ot_name() == ot_name {
            return;
        }
        let mut data = self.raw_ot.clone();
        self.set_string_keep_terminator_style(&ot_name.chars().collect::<Vec<char>>(), &mut data);
        self.raw_ot = data;
    }

    fn get_ot_gender(&self) -> usize {
        todo!()
    }

    fn set_ot_gender(&mut self, ot_gender: usize) {
        todo!()
    }

    fn get_ball(&self) -> usize {
        0
    }

    fn set_ball(&mut self, _ball: usize) {}

    fn get_met_level(&self) -> usize {
        todo!()
    }

    fn set_met_level(&mut self, met_level: usize) {
        todo!()
    }

    fn get_move_1(&self) -> usize {
        todo!()
    }

    fn set_move_1(&mut self, move_1: usize) {
        todo!()
    }

    fn get_move_2(&self) -> usize {
        todo!()
    }

    fn set_move_2(&mut self, move_2: usize) {
        todo!()
    }

    fn get_move_3(&self) -> usize {
        todo!()
    }

    fn set_move_3(&mut self, move3: usize) {
        todo!()
    }

    fn get_move_4(&self) -> usize {
        todo!()
    }

    fn set_move_4(&mut self, move_4: usize) {
        todo!()
    }

    fn get_move_1_pp(&self) -> usize {
        todo!()
    }

    fn set_move_1_pp(&mut self, move_1_pp: usize) {
        todo!()
    }

    fn get_move_2_pp(&self) -> usize {
        todo!()
    }

    fn set_move_2_pp(&mut self, move_2_pp: usize) {
        todo!()
    }

    fn get_move_3_pp(&self) -> usize {
        todo!()
    }

    fn set_move_3_pp(&mut self, move_3_pp: usize) {
        todo!()
    }

    fn get_move_4_pp(&self) -> usize {
        todo!()
    }

    fn set_move_4_pp(&mut self, move_4_pp: usize) {
        todo!()
    }

    fn get_move_1_pp_ups(&self) -> usize {
        todo!()
    }

    fn set_move_1_pp_ups(&mut self, move_1_pp_ups: usize) {
        todo!()
    }

    fn get_move_2_pp_ups(&self) -> usize {
        todo!()
    }

    fn set_move_2_pp_ups(&mut self, move_2_pp_ups: usize) {
        todo!()
    }

    fn get_move_3_pp_ups(&self) -> usize {
        todo!()
    }

    fn set_move_3_pp_ups(&mut self, move_3_pp_ups: usize) {
        todo!()
    }

    fn get_move_4_pp_ups(&self) -> usize {
        todo!()
    }

    fn set_move_4_pp_ups(&mut self, move_4_pp_ups: usize) {
        todo!()
    }

    fn get_ev_hp(&self) -> usize {
        todo!()
    }

    fn set_ev_hp(&mut self, hp: usize) {
        todo!()
    }

    fn get_ev_atk(&self) -> usize {
        todo!()
    }

    fn set_ev_atk(&mut self, atk: usize) {
        todo!()
    }

    fn get_ev_def(&self) -> usize {
        todo!()
    }

    fn set_ev_def(&mut self, def: usize) {
        todo!()
    }

    fn get_ev_spe(&self) -> usize {
        todo!()
    }

    fn set_ev_spe(&mut self, spe: usize) {
        todo!()
    }

    fn get_ev_spa(&self) -> usize {
        self.get_ev_spc()
    }

    fn set_ev_spa(&mut self, spa: usize) {
        self.set_ev_spc(spa);
    }

    fn get_ev_spd(&self) -> usize {
        self.get_ev_spc()
    }

    fn set_ev_spd(&mut self, _spd: usize) {}

    fn get_iv_hp(&self) -> usize {
        ((self.get_iv_atk() & 1) << 3)
            | ((self.get_iv_def() & 1) << 2)
            | ((self.get_iv_spe() & 1) << 1)
            | (self.get_iv_spc() & 1)
    }

    fn set_iv_hp(&mut self, _hp: usize) {}

    fn get_iv_atk(&self) -> usize {
        ((self.get_dv16() >> 12) & 0xF) as usize
    }

    fn set_iv_atk(&mut self, atk: usize) {
        self.set_dv16(
            (self.get_dv16() & !(0xF << 12)) | (if atk > 0xF { 0xF } else { atk as u16 } << 12),
        );
    }

    fn get_iv_def(&self) -> usize {
        ((self.get_dv16() >> 8) & 0xF) as usize
    }

    fn set_iv_def(&mut self, def: usize) {
        self.set_dv16(
            (self.get_dv16() & !(0xF << 8)) | (if def > 0xF { 0xF } else { def as u16 } << 8),
        );
    }

    fn get_iv_spe(&self) -> usize {
        ((self.get_dv16() >> 4) & 0xF) as usize
    }

    fn set_iv_spe(&mut self, spe: usize) {
        self.set_dv16(
            (self.get_dv16() & !(0xF << 4)) | (if spe > 0xF { 0xF } else { spe as u16 } << 4),
        );
    }

    fn get_iv_spa(&self) -> usize {
        self.get_iv_spc()
    }

    fn set_iv_spa(&mut self, spa: usize) {
        self.set_iv_spc(spa)
    }

    fn get_iv_spd(&self) -> usize {
        self.get_iv_spc()
    }

    fn set_iv_spd(&mut self, _spd: usize) {}

    fn get_status_condition(&self) -> usize {
        todo!()
    }

    fn set_status_condition(&mut self, status_condition: usize) {
        todo!()
    }

    fn get_stat_level(&self) -> usize {
        todo!()
    }

    fn set_stat_level(&mut self, level: usize) {
        todo!()
    }

    fn get_stat_hp_max(&self) -> usize {
        todo!()
    }

    fn set_stat_hp_max(&mut self, hp: usize) {
        todo!()
    }

    fn get_stat_hp_current(&self) -> usize {
        todo!()
    }

    fn set_stat_hp_current(&mut self, hp: usize) {
        todo!()
    }

    fn get_stat_atk(&self) -> usize {
        todo!()
    }

    fn set_stat_atk(&mut self, atk: usize) {
        todo!()
    }

    fn get_stat_def(&self) -> usize {
        todo!()
    }

    fn set_stat_def(&mut self, def: usize) {
        todo!()
    }

    fn get_stat_spe(&self) -> usize {
        todo!()
    }

    fn set_stat_spe(&mut self, spe: usize) {
        todo!()
    }

    fn get_stat_spa(&self) -> usize {
        todo!()
    }

    fn set_stat_spa(&mut self, spa: usize) {
        todo!()
    }

    fn get_stat_spd(&self) -> usize {
        todo!()
    }

    fn set_stat_spd(&mut self, spd: usize) {
        todo!()
    }

    fn get_version(&self) -> usize {
        todo!()
    }

    fn set_version(&mut self, version: usize) {
        todo!()
    }

    fn get_pkrs_strain(&self) -> usize {
        todo!()
    }

    fn set_pkrs_strain(&mut self, strain: usize) {
        todo!()
    }

    fn get_pkrs_days(&self) -> usize {
        todo!()
    }

    fn set_pkrs_days(&mut self, days: usize) {
        todo!()
    }

    fn get_encryption_constant(&self) -> usize {
        0
    }

    fn set_encryption_constant(&mut self, _ec: usize) {}

    fn get_pid(&self) -> usize {
        0
    }

    fn set_pid(&mut self, _pid: usize) {}

    fn get_fateful_encounter(&self) -> bool {
        false
    }

    fn set_fateful_encounter(&mut self, _fe: bool) {}

    fn get_tsv(&self) -> usize {
        0x0000
    }

    fn set_tsv(&mut self, _tsv: usize) {}

    fn get_psv(&self) -> usize {
        0xFFFF
    }

    fn set_psv(&mut self, _set_psv: usize) {}

    fn get_characteristic(&self) -> usize {
        usize::MAX
    }

    fn set_characteristic(&mut self, _characteristic: usize) {}

    fn get_mark_value(&self) -> usize {
        0
    }

    fn set_mark_value(&mut self, _value: usize) {}

    fn get_met_location(&self) -> usize {
        todo!()
    }

    fn set_met_location(&mut self, location: usize) {
        todo!()
    }

    fn get_egg_location(&self) -> usize {
        0
    }

    fn set_egg_location(&mut self, _location: usize) {}

    fn get_ot_friendship(&self) -> usize {
        todo!()
    }

    fn set_ot_friendship(&mut self, friendship: usize) {
        todo!()
    }

    fn japanese(&self) -> bool {
        self.raw_ot.len() == PK1::STRING_LENGTH_JAPANESE
    }

    fn korean(&self) -> bool {
        false
    }

    fn get_current_handler(&self) -> usize {
        0
    }

    fn set_current_handler(&mut self, _handler: usize) {}

    fn hp_bit_val_power(&self) -> usize {
        (self.get_iv_atk() & 8)
            | ((self.get_iv_def() & 8) >> 1)
            | ((self.get_iv_spe() & 8) >> 2)
            | ((self.get_iv_spc() & 8) >> 3)
    }

    fn get_hp_type(&self) -> usize {
        ((self.get_iv_atk() & 3) << 2) | (self.get_iv_def() & 3)
    }

    fn set_hp_type(&mut self, hp_type: usize) {
        self.set_iv_def(((self.get_iv_def() >> 2) << 2) | (hp_type & 3));
        self.set_iv_def(((self.get_iv_atk() >> 2) << 2) | ((hp_type >> 2) & 3));
    }

    fn is_gender_valid(&self) -> bool {
        true
    }

    fn refresh_checksum(&mut self) {}

    fn checksum_valid(&self) -> bool {
        todo!()
    }

    fn get_stats_personal_info<I: PersonalInfo>(&self, p: &I) -> [u16; 6] {
        let lv = self.get_stat_level();
        [
            self.get_stat(p.get_hp(), self.get_iv_hp(), self.get_ev_hp(), lv) + (5 + lv as u16),
            self.get_stat(p.get_atk(), self.get_iv_atk(), self.get_ev_atk(), lv),
            self.get_stat(p.get_def(), self.get_iv_def(), self.get_ev_def(), lv),
            self.get_stat(p.get_spe(), self.get_iv_spe(), self.get_ev_spe(), lv),
            self.get_stat(p.get_spa(), self.get_iv_spa(), self.get_ev_spa(), lv),
            self.get_stat(p.get_spd(), self.get_iv_spd(), self.get_ev_spd(), lv),
        ]
    }

    fn hp_power(&self) -> usize {
        (((5 * self.hp_bit_val_power()) + (self.get_iv_spc() & 3)) >> 1) + 31
    }

    fn get_move_pp(&self, move_num: usize, pp_up_count: usize) -> usize {
        let pp = self.get_base_pp(move_num) * (5 + pp_up_count) / 5;
        pp + (pp_up_count * 7.min(pp / 5))
    }

    fn set_shiny(&mut self) {
        self.set_iv_atk(self.get_iv_atk() | 2);
        self.set_iv_def(10);
        self.set_iv_spe(10);
        self.set_iv_spa(10);
    }
}

impl SpeciesForm for PK1 {
    fn get_species(&self) -> usize {
        species_converter::get_g1_species(self.get_species_id1()) as usize
    }

    fn set_species(&mut self, species: usize) {
        todo!()
    }

    fn get_form(&self) -> usize {
        if self.get_species() != 201 {
            return 0;
        }

        let mut form_val = 0;
        form_val |= (self.get_iv_atk() & 0x6) << 5;
        form_val |= (self.get_iv_def() & 0x6) << 3;
        form_val |= (self.get_iv_spe() & 0x6) << 1;
        form_val |= (self.get_iv_spc() & 0x6) >> 1;

        form_val / 10
    }

    fn set_form(&mut self, form: usize) {
        if self.get_species() == 201 {
            while self.get_form() != form {
                self.set_random_ivs(Some(0));
            }
        }
    }
}

impl TrainerId for PK1 {
    fn get_tid(&self) -> usize {
        todo!()
    }

    fn set_tid(&mut self, tid: usize) {
        todo!()
    }

    fn get_sid(&self) -> usize {
        0
    }

    fn set_sid(&mut self, _sid: usize) {}
}

impl Generation for PK1 {
    fn get_generation(&self) -> usize {
        todo!()
    }
}

impl Shiny for PK1 {
    fn get_is_shiny(&self) -> bool {
        self.get_iv_def() == 10
            && self.get_iv_spe() == 10
            && self.get_iv_spc() == 10
            && (self.get_iv_atk() & 2) == 2
    }
}

impl LangNick for PK1 {
    fn get_nickname(&self) -> String {
        if self.korean() {
            string_converter_2_kor::get_string(&self.raw_nickname)
        } else {
            string_converter_12::get_string(&self.raw_nickname, self.japanese())
        }
    }

    fn set_nickname(&mut self, nickname: String) {
        if !self.get_is_nicknamed() && self.get_nickname() == nickname {
            return;
        }
        let mut data = self.raw_nickname.clone();
        self.set_string_keep_terminator_style(&nickname.chars().collect::<Vec<char>>(), &mut data);
        self.raw_nickname = data;
    }

    fn get_is_nicknamed(&self) -> bool {
        self.nickname_trash()
            == self.get_non_nickname(self.guessed_language(LanguageID::English as usize))
    }

    fn set_is_nicknamed(&mut self, nicknamed: bool) {
        if !nicknamed {
            self.set_not_nicknamed(self.guessed_language(LanguageID::English as usize))
        }
    }

    fn get_language(&self) -> usize {
        if self.japanese() {
            LanguageID::Japanese as usize
        } else if self.korean() {
            LanguageID::Korean as usize
        } else if string_converter_12::is_g12_german_byte(&self.ot_trash()) {
            LanguageID::German as usize
        } else if let Some(lang) = species_name::get_species_name_language(
            self.get_species(),
            self.get_nickname(),
            self.format(),
        ) {
            lang
        } else {
            0
        }
    }

    fn set_language(&mut self, language: usize) {
        if self.japanese() || self.korean() || self.get_is_nicknamed() {
            return;
        }
        self.set_not_nicknamed(language);
    }
}

impl GameValueLimit for PK1 {
    fn get_max_move_id(&self) -> usize {
        todo!()
    }

    fn get_max_species_id(&self) -> usize {
        todo!()
    }

    fn get_max_item_id(&self) -> usize {
        todo!()
    }

    fn get_max_ability_id(&self) -> usize {
        todo!()
    }

    fn get_max_ball_id(&self) -> usize {
        usize::MAX
    }

    fn get_max_game_id(&self) -> usize {
        GameVersion::RD as usize
    }

    fn get_min_game_id(&self) -> usize {
        GameVersion::C as usize
    }

    fn get_max_iv(&self) -> usize {
        15
    }

    fn get_max_ev(&self) -> usize {
        u16::MAX as usize
    }

    fn get_ot_length(&self) -> usize {
        if self.japanese() {
            5
        } else {
            7
        }
    }

    fn get_nick_length(&self) -> usize {
        if self.japanese() {
            5
        } else {
            10
        }
    }
}

impl NatureT for PK1 {
    fn get_nature(&self) -> usize {
        todo!()
    }

    fn set_nature(&mut self, nature: usize) {
        todo!()
    }
}

impl Clone for PK1 {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl GBPkml<PersonalInfoG1> for PK1 {
    fn new_blank(jp: bool) -> Self {
        let str_len = if jp {
            PK1::STRING_LENGTH_JAPANESE
        } else {
            PK1::STRING_LENGTH_NOT_JAPANESE
        };
        let mut pkm = <PK1 as Pkm<PersonalInfoG1>>::new_blank();
        pkm.raw_ot = vec![G1_TERMINATOR_CODE; str_len];
        pkm.raw_nickname = vec![G1_TERMINATOR_CODE; str_len];
        pkm
    }

    fn new(data: Vec<u8>, jp: bool) -> Self {
        let str_len = if jp {
            PK1::STRING_LENGTH_JAPANESE
        } else {
            PK1::STRING_LENGTH_NOT_JAPANESE
        };
        let mut pkm = <PK1 as Pkm<PersonalInfoG1>>::new(data);
        pkm.raw_ot = vec![G1_TERMINATOR_CODE; str_len];
        pkm.raw_nickname = vec![G1_TERMINATOR_CODE; str_len];
        pkm
    }
}
