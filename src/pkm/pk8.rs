use crate::editing::nature_amp;
use crate::legality::learn_source;
use crate::personal_info::traits::personal_info::PersonalInfo;
use crate::personal_info::traits::{BaseStat, PersonalTable};
use crate::personal_info::PersonalInfo8SWSH;
use crate::pkm::shared::{EntityContext, G8Pkm};
use crate::pkm::strings::string_converter_8;
use crate::pkm::traits::form_argument::FormArgument;
use crate::pkm::traits::metadata::{GameValueLimit, Generation, LangNick, Shiny, SpeciesForm};
use crate::pkm::traits::templates::{
    ContestStatsReadOnly, DynamaxLevelReadOnly, GigantamaxReadOnly, MemoryHTReadOnly,
    MemoryOTReadOnly, NatureReadOnly, ScaledSizeReadOnly,
};
use crate::pkm::traits::{
    BattleVersion, ContestStats, DynamaxLevel, Favorite, Gigantamax, HandlerLanguage, HomeTrack,
    HyperTrain, MemoryHT, MemoryOT, MoveReset, Nature, SanityChecksum, ScaledSize, Sociability,
    TechRecord, TrainerID, TrainerIDPKM, TrainerMemories,
};
use crate::pkm::util::poke_crypto;
use crate::pkm::{pkm_trait, Pkm};
use crate::ribbons::{
    RibbonHasMark, RibbonIndex, RibbonSetAffixed, RibbonSetCommon3, RibbonSetCommon4,
    RibbonSetCommon6, RibbonSetCommon7, RibbonSetCommon8, RibbonSetEvent3, RibbonSetEvent4,
    RibbonSetMark8, RibbonSetMemory6,
};
use crate::{flag_util, legality, personal_info};
use no_std_io::{Reader, Writer};

#[derive(Clone, Default)]
pub struct PK8 {
    data: Vec<u8>,
}

impl PK8 {
    pub fn new(data: &[u8]) -> Self {
        let data = data.to_vec();
        let mut pk8 = Self {
            data: PK8::decrypt_party(data),
        };
        pk8.set_affixed_ribbon(None);
        pk8
    }

    fn decrypt_party(mut data: Vec<u8>) -> Vec<u8> {
        poke_crypto::decrypt_if_encrypted_8(&mut data);
        data.resize(poke_crypto::SIZE_8PARTY, 0);
        data
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0;
        for i in (8..poke_crypto::SIZE_9STORED).step_by(2) {
            chk += self.data.default_read_le::<u16>(i);
        }
        chk
    }

    fn iv32(&self) -> u32 {
        self.data.default_read_le(0x8C)
    }

    fn set_iv32(&mut self, iv32: u32) {
        self.data.checked_write_le(0x8C, &iv32);
    }

    fn pkrs(&self) -> u8 {
        self.data.default_read_le(0x32)
    }

    fn set_pkrs(&mut self, pkrs: u8) {
        self.data.checked_write_le(0x32, &pkrs);
    }
}

impl Pkm for PK8 {
    type PersonalInfoOutput = PersonalInfo8SWSH<'static>;

    fn size_party(&self) -> usize {
        poke_crypto::SIZE_8PARTY
    }

    fn size_stored(&self) -> usize {
        poke_crypto::SIZE_8STORED
    }

    fn extension(&self) -> &str {
        "pk8"
    }

    fn personal_info(&self) -> &Self::PersonalInfoOutput {
        personal_info::SWSH.get_form_entry(self.species(), self.form())
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn valid(&self) -> bool {
        self.sanity() == 0 && self.checksum_valid()
    }

    fn set_valid(&mut self, valid: bool) {
        if !valid {
            return;
        }
        self.set_sanity(0);
        self.refresh_checksum();
    }

    fn nickname_trash(&self) -> &[u8] {
        &self.data[0x58..0x7E]
    }

    fn nickname_trash_mut(&mut self) -> &mut [u8] {
        &mut self.data[0x58..0x72]
    }

    fn ot_trash(&self) -> &[u8] {
        &self.data[0xF8..0x112]
    }

    fn ot_trash_mut(&mut self) -> &mut [u8] {
        &mut self.data[0xF8..0x112]
    }

    fn ht_trash(&self) -> &[u8] {
        &self.data[0xA8..0xC2]
    }

    fn ht_trash_mut(&mut self) -> &mut [u8] {
        &mut self.data[0xA8..0xC2]
    }

    fn encrypt(&mut self) -> Vec<u8> {
        self.refresh_checksum();
        poke_crypto::encrypt_array_8(&self.data)
    }

    fn context(&self) -> EntityContext {
        EntityContext::Gen8
    }

    fn set_species(&mut self, species: u16) {
        self.data.checked_write_le(0x8, &species);
    }

    fn set_nickname(&mut self, nickname: &str) {
        let chars = nickname.chars().collect::<Vec<_>>();
        string_converter_8::set_string(self.nickname_trash_mut(), &chars, 12, None);
    }

    fn held_item(&self) -> u16 {
        self.data.default_read_le(0xA)
    }

    fn set_held_item(&mut self, item: u16) {
        self.data.checked_write_le(0xA, &item);
    }

    fn gender(&self) -> u8 {
        (self.data.default_read_le::<u8>(0x22) >> 2) & 0x3
    }

    fn set_gender(&mut self, gender: u8) {
        let gender = (self.data.default_read_le::<u8>(0x22) & 0xF3) | (gender << 2);
        self.data.checked_write_le(0x22, &gender);
    }

    fn ability(&self) -> u16 {
        self.data.default_read_le(0x14)
    }

    fn set_ability(&mut self, ability: u16) {
        self.data.checked_write_le(0x14, &ability);
    }

    fn current_friendship(&self) -> u8 {
        if self.current_handler() == 0 {
            self.ot_friendship()
        } else {
            self.ht_friendship()
        }
    }

    fn set_current_friendship(&mut self, friendship: u8) {
        if self.current_handler() == 0 {
            self.set_ot_friendship(friendship);
        } else {
            self.set_ht_friendship(friendship);
        }
    }

    fn set_form(&mut self, form: u8) {
        self.data.checked_write_le(0x24, &form);
    }

    fn is_egg(&self) -> bool {
        ((self.iv32() >> 30) & 1) == 1
    }

    fn set_is_egg(&mut self, egg: bool) {
        self.set_iv32((self.iv32() & !0x40000000) | if egg { 0x40000000 } else { 0 })
    }

    fn set_is_nicknamed(&mut self, nicknamed: bool) {
        self.set_iv32((self.iv32() & 0x7FFFFFFF) | if nicknamed { 0x80000000 } else { 0 })
    }

    fn exp(&self) -> u32 {
        self.data.default_read_le(0x10)
    }

    fn set_exp(&mut self, exp: u32) {
        self.data.checked_write_le(0x10, &exp);
    }

    fn ot_name(&self) -> String {
        string_converter_8::get_string(self.ot_trash())
    }

    fn set_ot_name(&mut self, name: &str) {
        let chars = name.chars().collect::<Vec<_>>();
        string_converter_8::set_string(self.ot_trash_mut(), &chars, 12, None);
    }

    fn ot_gender(&self) -> u8 {
        self.data.default_read_le::<u8>(0x125) >> 7
    }

    fn set_ot_gender(&mut self, gender: u8) {
        let gender = (self.data.default_read_le::<u8>(0x125) & 0x80) | gender;
        self.data.checked_write_le(0x125, &gender);
    }

    fn ball(&self) -> u8 {
        self.data.default_read_le(0x124)
    }

    fn set_ball(&mut self, ball: u8) {
        self.data.checked_write_le(0x124, &ball);
    }

    fn met_level(&self) -> u8 {
        self.data.default_read_le::<u8>(0x125) & !0x80
    }

    fn set_met_level(&mut self, level: u8) {
        let level = (self.data.default_read_le::<u8>(0x125) & 0x80) | level;
        self.data.checked_write_le(0x125, &level);
    }

    fn move_1(&self) -> u16 {
        self.data.default_read_le(0x72)
    }

    fn set_move_1(&mut self, mov: u16) {
        self.data.checked_write_le(0x72, &mov);
    }

    fn move_2(&self) -> u16 {
        self.data.default_read_le(0x74)
    }

    fn set_move_2(&mut self, mov: u16) {
        self.data.checked_write_le(0x74, &mov);
    }

    fn move_3(&self) -> u16 {
        self.data.default_read_le(0x76)
    }

    fn set_move_3(&mut self, mov: u16) {
        self.data.checked_write_le(0x76, &mov);
    }

    fn move_4(&self) -> u16 {
        self.data.default_read_le(0x78)
    }

    fn set_move_4(&mut self, mov: u16) {
        self.data.checked_write_le(0x78, &mov);
    }

    fn move_1_pp(&self) -> u8 {
        self.data.default_read_le(0x7A)
    }

    fn set_move_1_pp(&mut self, pp: u8) {
        self.data.checked_write_le(0x7A, &pp);
    }

    fn move_2_pp(&self) -> u8 {
        self.data.default_read_le(0x7B)
    }

    fn set_move_2_pp(&mut self, pp: u8) {
        self.data.checked_write_le(0x7B, &pp);
    }

    fn move_3_pp(&self) -> u8 {
        self.data.default_read_le(0x7C)
    }

    fn set_move_3_pp(&mut self, pp: u8) {
        self.data.checked_write_le(0x7C, &pp);
    }

    fn move_4_pp(&self) -> u8 {
        self.data.default_read_le(0x7D)
    }

    fn set_move_4_pp(&mut self, pp: u8) {
        self.data.checked_write_le(0x7D, &pp);
    }

    fn move_1_pp_ups(&self) -> u8 {
        self.data.default_read_le(0x7E)
    }

    fn set_move_1_pp_ups(&mut self, ups: u8) {
        self.data.checked_write_le(0x7E, &ups);
    }

    fn move_2_pp_ups(&self) -> u8 {
        self.data.default_read_le(0x7F)
    }

    fn set_move_2_pp_ups(&mut self, ups: u8) {
        self.data.checked_write_le(0x7F, &ups);
    }

    fn move_3_pp_ups(&self) -> u8 {
        self.data.default_read_le(0x80)
    }

    fn set_move_3_pp_ups(&mut self, ups: u8) {
        self.data.checked_write_le(0x80, &ups);
    }

    fn move_4_pp_ups(&self) -> u8 {
        self.data.default_read_le(0x81)
    }

    fn set_move_4_pp_ups(&mut self, ups: u8) {
        self.data.checked_write_le(0x81, &ups);
    }

    fn ev_hp(&self) -> u8 {
        self.data.default_read_le(0x26)
    }

    fn set_ev_hp(&mut self, ev: u8) {
        self.data.checked_write_le(0x26, &ev);
    }

    fn ev_atk(&self) -> u8 {
        self.data.default_read_le(0x27)
    }

    fn set_ev_atk(&mut self, ev: u8) {
        self.data.checked_write_le(0x27, &ev);
    }

    fn ev_def(&self) -> u8 {
        self.data.default_read_le(0x28)
    }

    fn set_ev_def(&mut self, ev: u8) {
        self.data.checked_write_le(0x28, &ev);
    }

    fn ev_spe(&self) -> u8 {
        self.data.default_read_le(0x29)
    }

    fn set_ev_spe(&mut self, ev: u8) {
        self.data.checked_write_le(0x29, &ev);
    }

    fn ev_spa(&self) -> u8 {
        self.data.default_read_le(0x2A)
    }

    fn set_ev_spa(&mut self, ev: u8) {
        self.data.checked_write_le(0x2A, &ev);
    }

    fn ev_spd(&self) -> u8 {
        self.data.default_read_le(0x2B)
    }

    fn set_ev_spd(&mut self, ev: u8) {
        self.data.checked_write_le(0x2B, &ev);
    }

    fn iv_hp(&self) -> u8 {
        (self.iv32() as u8) & 0x1F
    }

    fn set_iv_hp(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !0x1F) | u32::from(iv.min(31)));
    }

    fn iv_atk(&self) -> u8 {
        ((self.iv32() >> 5) as u8) & 0x1F
    }

    fn set_iv_atk(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !(0x1F << 5)) | (u32::from(iv.min(31)) << 5));
    }

    fn iv_def(&self) -> u8 {
        ((self.iv32() >> 10) as u8) & 0x1F
    }

    fn set_iv_def(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !(0x1F << 10)) | (u32::from(iv.min(31)) << 10));
    }

    fn iv_spe(&self) -> u8 {
        ((self.iv32() >> 15) as u8) & 0x1F
    }

    fn set_iv_spe(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !(0x1F << 15)) | (u32::from(iv.min(31)) << 15));
    }

    fn iv_spa(&self) -> u8 {
        ((self.iv32() >> 20) as u8) & 0x1F
    }

    fn set_iv_spa(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !(0x1F << 20)) | (u32::from(iv.min(31)) << 20));
    }

    fn iv_spd(&self) -> u8 {
        ((self.iv32() >> 25) as u8) & 0x1F
    }

    fn set_iv_spd(&mut self, iv: u8) {
        self.set_iv32((self.iv32() & !(0x1F << 25)) | (u32::from(iv.min(31)) << 25));
    }

    fn status_condition(&self) -> u32 {
        self.data.default_read_le(0x94)
    }

    fn set_status_condition(&mut self, status: u32) {
        self.data.checked_write_le(0x94, &status);
    }

    fn stat_level(&self) -> u8 {
        self.data.default_read_le(0x148)
    }

    fn set_stat_level(&mut self, level: u8) {
        self.data.checked_write_le(0x148, &level);
    }

    fn stat_hp_max(&self) -> u16 {
        self.data.default_read_le(0x14A)
    }

    fn set_stat_hp_max(&mut self, max: u16) {
        self.data.checked_write_le(0x14A, &max);
    }

    fn stat_hp_current(&self) -> u16 {
        self.data.default_read_le(0x8A)
    }

    fn set_stat_hp_current(&mut self, current: u16) {
        self.data.checked_write_le(0x8A, &current);
    }

    fn stat_atk(&self) -> u16 {
        self.data.default_read_le(0x14C)
    }

    fn set_stat_atk(&mut self, stat: u16) {
        self.data.checked_write_le(0x14C, &stat);
    }

    fn stat_def(&self) -> u16 {
        self.data.default_read_le(0x14E)
    }

    fn set_stat_def(&mut self, stat: u16) {
        self.data.checked_write_le(0x14E, &stat);
    }

    fn stat_spe(&self) -> u16 {
        self.data.default_read_le(0x150)
    }

    fn set_stat_spe(&mut self, stat: u16) {
        self.data.checked_write_le(0x150, &stat);
    }

    fn stat_spa(&self) -> u16 {
        self.data.default_read_le(0x152)
    }

    fn set_stat_spa(&mut self, stat: u16) {
        self.data.checked_write_le(0x152, &stat);
    }

    fn stat_spd(&self) -> u16 {
        self.data.default_read_le(0x154)
    }

    fn set_stat_spd(&mut self, stat: u16) {
        self.data.checked_write_le(0x154, &stat);
    }

    fn version(&self) -> u8 {
        self.data.default_read_le(0xDE)
    }

    fn set_version(&mut self, version: u8) {
        self.data.checked_write_le(0xDE, &version);
    }

    fn pkrs_strain(&self) -> u8 {
        self.pkrs() >> 4
    }

    fn set_pkrs_strain(&mut self, strain: u8) {
        self.set_pkrs((self.pkrs() & 0xF) | (strain << 4));
    }

    fn pkrs_days(&self) -> u8 {
        self.pkrs() & 0xF
    }

    fn set_pkrs_days(&mut self, days: u8) {
        self.set_pkrs((self.pkrs() & !0xF) | days);
    }

    fn encryption_constant(&self) -> u32 {
        self.data.default_read_le(0)
    }

    fn set_encryption_constant(&mut self, ec: u32) {
        self.data.checked_write_le(0, &ec);
    }

    fn pid(&self) -> u32 {
        self.data.default_read_le(0x1C)
    }

    fn set_pid(&mut self, pid: u32) {
        self.data.checked_write_le(0x1C, &pid);
    }

    fn set_language(&mut self, lang: u8) {
        self.data.checked_write_le(0xE2, &lang);
    }

    fn fateful_encounter(&self) -> bool {
        (self.data.default_read_le::<u8>(0x22) & 1) == 1
    }

    fn set_fateful_encounter(&mut self, fateful: bool) {
        let byte = (self.data.default_read_le::<u8>(0x22) & !1) | u8::from(fateful);
        self.data.checked_write_le(0x22, &byte);
    }

    fn characteristic(&self) -> u8 {
        let pm6 = self.encryption_constant() % 6;
        let max_iv = self.maximum_iv();
        let mut pm6stat = 0;
        for i in 0..6 {
            pm6stat = (pm6 + i) % 6;
            if self.get_iv(pm6stat as usize).unwrap_or_default() == max_iv {
                break;
            }
        }
        ((pm6stat * 5) + (max_iv as u32 % 5)) as u8
    }

    fn mark_value(&self) -> u16 {
        self.data.default_read_le(0x18)
    }

    fn set_mark_value(&mut self, value: u16) {
        self.data.checked_write_le(0x18, &value);
    }

    fn met_location(&self) -> u16 {
        self.data.default_read_le(0x122)
    }

    fn set_met_location(&mut self, location: u16) {
        self.data.checked_write_le(0x122, &location);
    }

    fn egg_location(&self) -> u16 {
        self.data.default_read_le(0x120)
    }

    fn set_egg_location(&mut self, location: u16) {
        self.data.checked_write_le(0x120, &location);
    }

    fn ot_friendship(&self) -> u8 {
        self.data.default_read_le(0x112)
    }

    fn set_ot_friendship(&mut self, friendship: u8) {
        self.data.checked_write_le(0x112, &friendship);
    }

    fn met_year(&self) -> u16 {
        self.data.default_read_le::<u8>(0x11C) as u16
    }

    fn set_met_year(&mut self, year: u16) {
        self.data.checked_write_le(0x11C, &(year as u8));
    }

    fn met_month(&self) -> u8 {
        self.data.default_read_le(0x11D)
    }

    fn set_met_month(&mut self, month: u8) {
        self.data.checked_write_le(0x11D, &month);
    }

    fn met_day(&self) -> u8 {
        self.data.default_read_le(0x11E)
    }

    fn set_met_day(&mut self, day: u8) {
        self.data.checked_write_le(0x11E, &day);
    }

    fn ht_name(&self) -> String {
        string_converter_8::get_string(self.ht_trash())
    }

    fn set_ht_name(&mut self, name: &str) {
        let chars = name.chars().collect::<Vec<_>>();
        string_converter_8::set_string(self.ht_trash_mut(), &chars, 12, None);
    }

    fn ht_gender(&self) -> u8 {
        self.data.default_read_le(0xC2)
    }

    fn set_ht_gender(&mut self, gender: u8) {
        self.data.checked_write_le(0xC2, &gender);
    }

    fn ht_friendship(&self) -> u8 {
        self.data.default_read_le(0xC8)
    }

    fn set_ht_friendship(&mut self, friendship: u8) {
        self.data.checked_write_le(0xC8, &friendship);
    }

    fn enjoyment(&self) -> u8 {
        self.data.default_read_le(0xDD)
    }

    fn set_enjoyment(&mut self, enjoyment: u8) {
        self.data.checked_write_le(0xDD, &enjoyment);
    }

    fn fullness(&self) -> u8 {
        self.data.default_read_le(0xDC)
    }

    fn set_fullness(&mut self, fullness: u8) {
        self.data.checked_write_le(0xDC, &fullness);
    }

    fn ability_number(&self) -> u8 {
        self.data.default_read_le::<u8>(0x16) & 7
    }

    fn set_ability_number(&mut self, number: u8) {
        let byte = (self.data.default_read_le::<u8>(0x16) & !7) | (number & 7);
        self.data.checked_write_le(0x16, &byte);
    }

    fn egg_year(&self) -> u16 {
        self.data.default_read_le::<u8>(0x119) as u16
    }

    fn set_egg_year(&mut self, year: u16) {
        self.data.checked_write_le(0x119, &(year as u8));
    }

    fn egg_month(&self) -> u8 {
        self.data.default_read_le(0x11A)
    }

    fn set_egg_month(&mut self, month: u8) {
        self.data.checked_write_le(0x11A, &month);
    }

    fn egg_day(&self) -> u8 {
        self.data.default_read_le(0x11B)
    }

    fn set_egg_day(&mut self, day: u8) {
        self.data.checked_write_le(0x11B, &day);
    }

    fn current_handler(&self) -> u8 {
        self.data.default_read_le(0xC4)
    }

    fn set_current_handler(&mut self, handler: u8) {
        self.data.checked_write_le(0xC4, &handler);
    }

    fn marking_count(&self) -> u8 {
        6
    }

    fn get_marking(&self, index: usize) -> u8 {
        if index >= self.marking_count() as usize {
            return 0;
        }
        ((self.mark_value() >> (index * 2)) & 3) as u8
    }

    fn set_marking(&mut self, index: usize, value: u8) {
        if index >= self.marking_count() as usize {
            return;
        }
        let shift = index * 2;
        self.set_mark_value(
            (self.mark_value() & !(0b11 << shift)) | ((u16::from(value) & 3) << shift),
        );
    }

    fn is_untraded(&self) -> bool {
        self.data.default_read_le::<u8>(0xA8) == 0
            && self.data.default_read_le::<u8>(0xA9) == 0
            && self.format().unwrap_or_default() == self.generation()
    }

    fn is_native(&self) -> bool {
        self.swsh()
    }

    fn refresh_checksum(&mut self) {
        self.set_checksum(self.calculate_checksum());
    }

    fn checksum_valid(&self) -> bool {
        self.calculate_checksum() == self.checksum()
    }

    fn load_stats(&self, p: &dyn BaseStat, stats: &mut [u16]) {
        let level = self.current_level();
        pkm_trait::load_stats_ht(stats, p, self, level);
        nature_amp::modify_stats_for_nature(stats, self.stat_nature());
    }
}

impl SpeciesForm for PK8 {
    fn species(&self) -> u16 {
        self.data.default_read_le(0x8)
    }

    fn form(&self) -> u8 {
        self.data.default_read_le(0x24)
    }
}

impl TrainerIDPKM for PK8 {}

impl TrainerID for PK8 {
    fn tid(&self) -> u16 {
        self.data.default_read_le(0xC)
    }

    fn sid(&self) -> u16 {
        self.data.default_read_le(0xE)
    }

    fn set_tid(&mut self, tid: u16) {
        self.data.checked_write_le(0xC, &tid);
    }

    fn set_sid(&mut self, sid: u16) {
        self.data.checked_write_le(0xE, &sid);
    }
}

impl Generation for PK8 {
    fn generation(&self) -> u8 {
        if self.gen_8() {
            8
        } else if self.gen_7() {
            7
        } else if self.gen_6() {
            6
        } else if self.gen_5() {
            5
        } else if self.gen_4() {
            4
        } else if self.gen_3() {
            3
        } else if self.gen_2() || self.gen_1() {
            self.format().unwrap_or_default()
        } else if self.vc1() {
            1
        } else if self.vc2() {
            2
        } else {
            0
        }
    }
}

impl Shiny for PK8 {
    fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    fn psv(&self) -> u16 {
        (((self.pid() >> 16) ^ (self.pid() & 0xFFFF)) >> 4) as u16
    }
}

impl LangNick for PK8 {
    fn nickname(&self) -> String {
        string_converter_8::get_string(self.nickname_trash())
    }

    fn is_nicknamed(&self) -> bool {
        ((self.iv32() >> 31) & 1) == 1
    }

    fn language(&self) -> u8 {
        self.data.default_read_le(0xE2)
    }
}

impl GameValueLimit for PK8 {
    fn max_species_id(&self) -> u16 {
        legality::tables8::MAX_SPECIES_ID_8
    }

    fn max_move_id(&self) -> u16 {
        legality::tables8::MAX_MOVE_ID_8
    }

    fn max_item_id(&self) -> u16 {
        legality::tables8::MAX_ITEM_ID_8
    }

    fn max_ability_id(&self) -> u16 {
        legality::tables8::MAX_ABILITY_ID_8
    }

    fn max_ball_id(&self) -> u8 {
        legality::tables8::MAX_BALL_ID_8
    }

    fn max_game_id(&self) -> u8 {
        legality::tables8::MAX_GAME_ID_8
    }

    fn max_iv(&self) -> u8 {
        31
    }

    fn max_ev(&self) -> u8 {
        252
    }

    fn max_string_length_ot(&self) -> usize {
        12
    }

    fn max_string_length_nickname(&self) -> usize {
        12
    }
}

impl Nature for PK8 {
    fn set_nature(&mut self, nature: u8) {
        self.data.checked_write_le(0x20, &nature);
    }
}

impl NatureReadOnly for PK8 {
    fn nature(&self) -> u8 {
        self.data.default_read_le(0x20)
    }
}

impl SanityChecksum for PK8 {
    fn sanity(&self) -> u16 {
        self.data.default_read_le(0x4)
    }

    fn set_sanity(&mut self, sanity: u16) {
        self.data.checked_write_le(0x4, &sanity);
    }

    fn checksum(&self) -> u16 {
        self.data.default_read_le(0x6)
    }

    fn set_checksum(&mut self, checksum: u16) {
        self.data.checked_write_le(0x6, &checksum);
    }
}

impl MoveReset for PK8 {
    fn reset_moves(&mut self) {
        let learn_sets = &legality::legality_core::LEVEL_UP_SWSH;
        let table = &personal_info::SWSH;

        let index = table.get_form_index(self.species(), self.form());
        let learn = &learn_sets[index];
        let mut moves = [0; 4];
        learn.set_encounter_moves(self.current_level(), &mut moves, None);
        self.set_moves(&moves);
        self.set_maximum_pp_current();
    }
}

impl RibbonSetEvent3 for PK8 {
    fn ribbon_earth(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 0)
    }

    fn set_ribbon_earth(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 0, val)
    }

    fn ribbon_national(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 7)
    }

    fn set_ribbon_national(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 7, val)
    }

    fn ribbon_country(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 6)
    }

    fn set_ribbon_country(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 6, val)
    }

    fn ribbon_champion_battle(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 1)
    }

    fn set_ribbon_champion_battle(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 1, val)
    }

    fn ribbon_champion_regional(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 2)
    }

    fn set_ribbon_champion_regional(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 2, val)
    }

    fn ribbon_champion_national(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 3)
    }

    fn set_ribbon_champion_national(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 3, val)
    }
}

impl RibbonSetEvent4 for PK8 {
    fn ribbon_classic(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 2)
    }

    fn set_ribbon_classic(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 2, val)
    }

    fn ribbon_wishing(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 0)
    }

    fn set_ribbon_wishing(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 0, val)
    }

    fn ribbon_premier(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 3)
    }

    fn set_ribbon_premier(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 3, val)
    }

    fn ribbon_event(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 4)
    }

    fn set_ribbon_event(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 4, val)
    }

    fn ribbon_birthday(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 5)
    }

    fn set_ribbon_birthday(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 5, val)
    }

    fn ribbon_special(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 6)
    }

    fn set_ribbon_special(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 6, val)
    }

    fn ribbon_world(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 1)
    }

    fn set_ribbon_world(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 1, val)
    }

    fn ribbon_champion_world(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 4)
    }

    fn set_ribbon_champion_world(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 4, val)
    }

    fn ribbon_souvenir(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 7)
    }

    fn set_ribbon_souvenir(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 7, val)
    }
}

impl RibbonSetCommon3 for PK8 {
    fn ribbon_champion_g3(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 1)
    }

    fn set_ribbon_champion_g3(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 1, val)
    }

    fn ribbon_artist(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 2)
    }

    fn set_ribbon_artist(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 2, val)
    }

    fn ribbon_effort(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 7)
    }

    fn set_ribbon_effort(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 7, val)
    }
}

impl RibbonSetCommon4 for PK8 {
    fn ribbon_champion_sinnoh(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 2)
    }

    fn set_ribbon_champion_sinnoh(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 2, val)
    }

    fn ribbon_alert(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 0)
    }

    fn set_ribbon_alert(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 0, val)
    }

    fn ribbon_shock(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 1)
    }

    fn set_ribbon_shock(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 1, val)
    }

    fn ribbon_downcast(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 2)
    }

    fn set_ribbon_downcast(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 2, val)
    }

    fn ribbon_careless(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 3)
    }

    fn set_ribbon_careless(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 3, val)
    }

    fn ribbon_relax(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 4)
    }

    fn set_ribbon_relax(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 4, val)
    }

    fn ribbon_snooze(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 5)
    }

    fn set_ribbon_snooze(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 5, val)
    }

    fn ribbon_smile(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 6)
    }

    fn set_ribbon_smile(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 6, val)
    }

    fn ribbon_gorgeous(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 7)
    }

    fn set_ribbon_gorgeous(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 7, val)
    }

    fn ribbon_royal(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 0)
    }

    fn set_ribbon_royal(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 0, val)
    }

    fn ribbon_gorgeous_royal(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 1)
    }

    fn set_ribbon_gorgeous_royal(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 1, val)
    }

    fn ribbon_footprint(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 3)
    }

    fn set_ribbon_footprint(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 3, val)
    }

    fn ribbon_record(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 4)
    }

    fn set_ribbon_record(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 4, val)
    }

    fn ribbon_legend(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 5)
    }

    fn set_ribbon_legend(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 5, val)
    }
}

impl RibbonSetCommon6 for PK8 {
    fn ribbon_champion_kalos(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 0)
    }

    fn set_ribbon_champion_kalos(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 0, val)
    }

    fn ribbon_champion_g6_hoenn(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 7)
    }

    fn set_ribbon_champion_g6_hoenn(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 7, val)
    }

    fn ribbon_best_friends(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 3)
    }

    fn set_ribbon_best_friends(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 3, val)
    }

    fn ribbon_training(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 4)
    }

    fn set_ribbon_training(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 4, val)
    }

    fn ribbon_battler_skillful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 5)
    }

    fn set_ribbon_battler_skillful(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 5, val)
    }

    fn ribbon_battler_expert(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 6)
    }

    fn set_ribbon_battler_expert(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 6, val)
    }

    fn ribbon_contest_star(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 0)
    }

    fn set_ribbon_contest_star(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 0, val)
    }

    fn ribbon_master_coolness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 1)
    }

    fn set_ribbon_master_coolness(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 1, val)
    }

    fn ribbon_master_beauty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 2)
    }

    fn set_ribbon_master_beauty(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 2, val)
    }

    fn ribbon_master_cuteness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 3)
    }

    fn set_ribbon_master_cuteness(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 3, val)
    }

    fn ribbon_master_cleverness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 4)
    }

    fn set_ribbon_master_cleverness(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 4, val)
    }

    fn ribbon_master_toughness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 5)
    }

    fn set_ribbon_master_toughness(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 5, val)
    }
}

impl RibbonSetMemory6 for PK8 {
    fn ribbon_count_memory_contest(&self) -> u8 {
        self.data.default_read_le(0x3C)
    }

    fn set_ribbon_count_memory_contest(&mut self, count: u8) {
        self.data.checked_write_le(0x3C, &count);
        self.set_has_contest_memory_ribbon(count != 0);
    }

    fn ribbon_count_memory_battle(&self) -> u8 {
        self.data.default_read_le(0x3D)
    }

    fn set_ribbon_count_memory_battle(&mut self, count: u8) {
        self.data.checked_write_le(0x3D, &count);
        self.set_has_battle_memory_ribbon(count != 0);
    }

    fn has_contest_memory_ribbon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 5)
    }

    fn set_has_contest_memory_ribbon(&mut self, has: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 5, has)
    }

    fn has_battle_memory_ribbon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 6)
    }

    fn set_has_battle_memory_ribbon(&mut self, has: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 6, has)
    }
}

impl RibbonSetCommon7 for PK8 {
    fn ribbon_champion_alola(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 6)
    }

    fn set_ribbon_champion_alola(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 6, val)
    }

    fn ribbon_battle_royale(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 7)
    }

    fn set_ribbon_battle_royale(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 7, val)
    }

    fn ribbon_battle_tree_great(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 0)
    }

    fn set_ribbon_battle_tree_great(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 0, val)
    }

    fn ribbon_battle_tree_master(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 1)
    }

    fn set_ribbon_battle_tree_master(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 1, val)
    }
}

impl RibbonSetCommon8 for PK8 {
    fn ribbon_champion_galar(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 2)
    }

    fn set_ribbon_champion_galar(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 2, val)
    }

    fn ribbon_tower_master(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 3)
    }

    fn set_ribbon_tower_master(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 3, val)
    }

    fn ribbon_master_rank(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 4)
    }

    fn set_ribbon_master_rank(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 4, val)
    }

    fn ribbon_twinkling_star(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 3)
    }

    fn set_ribbon_twinkling_star(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 3, val)
    }

    fn ribbon_hisui(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 2)
    }

    fn set_ribbon_hisui(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 2, val)
    }
}

impl RibbonSetMark8 for PK8 {
    fn ribbon_mark_lunchtime(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 5)
    }

    fn set_ribbon_mark_lunchtime(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 5, val)
    }

    fn ribbon_mark_sleepy_time(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 5)
    }

    fn set_ribbon_mark_sleepy_time(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 6, val)
    }

    fn ribbon_mark_dusk(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 7)
    }

    fn set_ribbon_mark_dusk(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 7, val)
    }

    fn ribbon_mark_dawn(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 0)
    }

    fn set_ribbon_mark_dawn(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 0, val)
    }

    fn ribbon_mark_cloudy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 1)
    }

    fn set_ribbon_mark_cloudy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 1, val)
    }

    fn ribbon_mark_rainy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 2)
    }

    fn set_ribbon_mark_rainy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 2, val)
    }

    fn ribbon_mark_stormy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 3)
    }

    fn set_ribbon_mark_stormy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 3, val)
    }

    fn ribbon_mark_snowy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 4)
    }

    fn set_ribbon_mark_snowy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 4, val)
    }

    fn ribbon_mark_blizzard(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 5)
    }

    fn set_ribbon_mark_blizzard(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 5, val)
    }

    fn ribbon_mark_dry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 6)
    }

    fn set_ribbon_mark_dry(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 6, val)
    }

    fn ribbon_mark_sandstorm(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 7)
    }

    fn set_ribbon_mark_sandstorm(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 7, val)
    }

    fn ribbon_mark_misty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 0)
    }

    fn set_ribbon_mark_misty(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 0, val)
    }

    fn ribbon_mark_destiny(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 1)
    }

    fn set_ribbon_mark_destiny(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 1, val)
    }

    fn ribbon_mark_fishing(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 2)
    }

    fn set_ribbon_mark_fishing(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 2, val)
    }

    fn ribbon_mark_curry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 3)
    }

    fn set_ribbon_mark_curry(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 3, val)
    }

    fn ribbon_mark_uncommon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 4)
    }

    fn set_ribbon_mark_uncommon(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 4, val)
    }

    fn ribbon_mark_rare(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 5)
    }

    fn set_ribbon_mark_rare(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 5, val)
    }

    fn ribbon_mark_rowdy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 6)
    }

    fn set_ribbon_mark_rowdy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 6, val)
    }

    fn ribbon_mark_absent_minded(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 7)
    }

    fn set_ribbon_mark_absent_minded(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 7, val)
    }

    fn ribbon_mark_jittery(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 0)
    }

    fn set_ribbon_mark_jittery(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 0, val)
    }

    fn ribbon_mark_excited(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 1)
    }

    fn set_ribbon_mark_excited(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 1, val)
    }

    fn ribbon_mark_charismatic(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 2)
    }

    fn set_ribbon_mark_charismatic(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 2, val)
    }

    fn ribbon_mark_calmness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 3)
    }

    fn set_ribbon_mark_calmness(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 3, val)
    }

    fn ribbon_mark_intense(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 4)
    }

    fn set_ribbon_mark_intense(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 4, val)
    }

    fn ribbon_mark_zoned_out(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 5)
    }

    fn set_ribbon_mark_zoned_out(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 5, val)
    }

    fn ribbon_mark_joyful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 6)
    }

    fn set_ribbon_mark_joyful(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 6, val)
    }

    fn ribbon_mark_angry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 7)
    }

    fn set_ribbon_mark_angry(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 7, val)
    }

    fn ribbon_mark_smiley(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 0)
    }

    fn set_ribbon_mark_smiley(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 0, val)
    }

    fn ribbon_mark_teary(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 1)
    }

    fn set_ribbon_mark_teary(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 1, val)
    }

    fn ribbon_mark_upbeat(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 2)
    }

    fn set_ribbon_mark_upbeat(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 2, val)
    }

    fn ribbon_mark_peeved(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 3)
    }

    fn set_ribbon_mark_peeved(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 3, val)
    }

    fn ribbon_mark_intellectual(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 4)
    }

    fn set_ribbon_mark_intellectual(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 4, val)
    }

    fn ribbon_mark_ferocious(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 5)
    }

    fn set_ribbon_mark_ferocious(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 5, val)
    }

    fn ribbon_mark_crafty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 6)
    }

    fn set_ribbon_mark_crafty(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 6, val)
    }

    fn ribbon_mark_scowling(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 7)
    }

    fn set_ribbon_mark_scowling(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 7, val)
    }

    fn ribbon_mark_kindly(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 0)
    }

    fn set_ribbon_mark_kindly(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 0, val)
    }

    fn ribbon_mark_flustered(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 1)
    }

    fn set_ribbon_mark_flustered(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 1, val)
    }

    fn ribbon_mark_pumped_up(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 2)
    }

    fn set_ribbon_mark_pumped_up(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 2, val)
    }

    fn ribbon_mark_zero_energy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 3)
    }

    fn set_ribbon_mark_zero_energy(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 3, val)
    }

    fn ribbon_mark_prideful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 4)
    }

    fn set_ribbon_mark_prideful(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 4, val)
    }

    fn ribbon_mark_unsure(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 5)
    }

    fn set_ribbon_mark_unsure(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 5, val)
    }

    fn ribbon_mark_humble(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 6)
    }

    fn set_ribbon_mark_humble(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 6, val)
    }

    fn ribbon_mark_thorny(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 7)
    }

    fn set_ribbon_mark_thorny(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 7, val)
    }

    fn ribbon_mark_vigor(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 0)
    }

    fn set_ribbon_mark_vigor(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 0, val)
    }

    fn ribbon_mark_slump(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 1)
    }

    fn set_ribbon_mark_slump(&mut self, val: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 1, val)
    }
}

impl RibbonHasMark for PK8 {
    fn has_mark(&self) -> bool {
        if self.data.default_read_le::<u16>(0x3A) & 0xFFE0 != 0 {
            return true;
        }
        if self.data.default_read_le::<u32>(0x40) != 0 {
            return true;
        }
        (self.data[0x44] & 3) != 0
    }
}

impl RibbonSetAffixed for PK8 {
    fn affixed_ribbon(&self) -> Option<u8> {
        let data = self.data.default_read_le::<i8>(0xE8);
        if data < 0 {
            None
        } else {
            Some(data as u8)
        }
    }

    fn set_affixed_ribbon(&mut self, ribbon: Option<u8>) {
        let ribbon = ribbon.map(|i| i as i8).unwrap_or(-1);
        self.data.checked_write_le(0xE8, &ribbon);
    }
}

impl TechRecord for PK8 {
    fn tech_record_permit_flags(&self) -> Vec<bool> {
        self.personal_info().tmhm()[..PersonalInfo8SWSH::COUNT_TM].to_vec()
    }

    fn tech_record_permit_indexes(&self) -> Vec<u16> {
        learn_source::TR_SWSH.to_vec()
    }

    fn record_count_total(&self) -> u16 {
        112
    }

    fn record_count_used(&self) -> u16 {
        PersonalInfo8SWSH::COUNT_TR as u16
    }

    fn get_move_record_flag(&self, index: u16) -> bool {
        if index > 112 {
            return false;
        }
        let ofs = (index >> 3) as usize;
        flag_util::get_flag(&self.data, 0x127 + ofs, (index & 7) as usize)
    }

    fn set_move_record_flag(&mut self, index: u16, value: Option<bool>) {
        let value = value.unwrap_or(true);
        if index > 112 {
            return;
        }
        let ofs = (index >> 3) as usize;
        flag_util::set_flag(&mut self.data, 112 + ofs, (index & 7) as usize, value)
    }

    fn get_move_record_flag_any(&self) -> bool {
        self.data.iter().skip(0x127).take(14).any(|&i| i != 0)
    }
}

impl Sociability for PK8 {
    fn sociability(&self) -> u32 {
        self.data.default_read_le(0x48)
    }

    fn set_sociability(&mut self, soc: u32) {
        self.data.checked_write_le(0x48, &soc);
    }
}

impl ContestStats for PK8 {
    fn set_cnt_cool(&mut self, cnt: u8) {
        self.data.checked_write_le(0x2C, &cnt);
    }

    fn set_cnt_beauty(&mut self, cnt: u8) {
        self.data.checked_write_le(0x2D, &cnt);
    }

    fn set_cnt_cute(&mut self, cnt: u8) {
        self.data.checked_write_le(0x2E, &cnt);
    }

    fn set_cnt_smart(&mut self, cnt: u8) {
        self.data.checked_write_le(0x2F, &cnt);
    }

    fn set_cnt_tough(&mut self, cnt: u8) {
        self.data.checked_write_le(0x30, &cnt);
    }

    fn set_cnt_sheen(&mut self, cnt: u8) {
        self.data.checked_write_le(0x31, &cnt);
    }
}

impl ContestStatsReadOnly for PK8 {
    fn cnt_cool(&self) -> u8 {
        self.data.default_read_le(0x2C)
    }

    fn cnt_beauty(&self) -> u8 {
        self.data.default_read_le(0x2D)
    }

    fn cnt_cute(&self) -> u8 {
        self.data.default_read_le(0x2E)
    }

    fn cnt_smart(&self) -> u8 {
        self.data.default_read_le(0x2F)
    }

    fn cnt_tough(&self) -> u8 {
        self.data.default_read_le(0x30)
    }

    fn cnt_sheen(&self) -> u8 {
        self.data.default_read_le(0x31)
    }
}

impl HyperTrain for PK8 {
    fn hyper_train_flags(&self) -> u8 {
        self.data.default_read_le(0x126)
    }

    fn set_hyper_train_flags(&mut self, flags: u8) {
        self.data.checked_write_le(0x126, &flags);
    }

    fn ht_hp(&self) -> bool {
        (self.hyper_train_flags() & 1) == 1
    }

    fn set_ht_hp(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !1) | u8::from(val);
        self.set_hyper_train_flags(flags);
    }

    fn ht_atk(&self) -> bool {
        ((self.hyper_train_flags() >> 1) & 1) == 1
    }

    fn set_ht_atk(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !(1 << 1)) | (u8::from(val) << 1);
        self.set_hyper_train_flags(flags);
    }

    fn ht_def(&self) -> bool {
        ((self.hyper_train_flags() >> 2) & 1) == 1
    }

    fn set_ht_def(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !(1 << 2)) | (u8::from(val) << 2);
        self.set_hyper_train_flags(flags);
    }

    fn ht_spa(&self) -> bool {
        ((self.hyper_train_flags() >> 3) & 1) == 1
    }

    fn set_ht_spa(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !(1 << 3)) | (u8::from(val) << 3);
        self.set_hyper_train_flags(flags);
    }

    fn ht_spd(&self) -> bool {
        ((self.hyper_train_flags() >> 4) & 1) == 1
    }

    fn set_ht_spd(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !(1 << 4)) | (u8::from(val) << 4);
        self.set_hyper_train_flags(flags);
    }

    fn ht_spe(&self) -> bool {
        ((self.hyper_train_flags() >> 5) & 1) == 1
    }

    fn set_ht_spe(&mut self, val: bool) {
        let flags = (self.hyper_train_flags() & !(1 << 5)) | (u8::from(val) << 5);
        self.set_hyper_train_flags(flags);
    }
}

impl ScaledSize for PK8 {
    fn set_weight_scalar(&mut self, scalar: u8) {
        self.data.checked_write_le(0x51, &scalar);
    }

    fn set_height_scalar(&mut self, scalar: u8) {
        self.data.checked_write_le(0x50, &scalar);
    }
}

impl ScaledSizeReadOnly for PK8 {
    fn weight_scalar(&self) -> u8 {
        self.data.default_read_le(0x51)
    }

    fn height_scalar(&self) -> u8 {
        self.data.default_read_le(0x50)
    }
}

impl Gigantamax for PK8 {
    fn set_can_gigantamax(&mut self, value: bool) {
        let flags = (self.data.default_read_le::<u8>(0x16) & !16) | (u8::from(value) << 4);
        self.data.checked_write_le(0x16, &flags);
    }
}

impl GigantamaxReadOnly for PK8 {
    fn can_gigantamax(&self) -> bool {
        (self.data.default_read_le::<u8>(0x16) & 16) != 0
    }
}

impl Favorite for PK8 {
    fn is_favorite(&self) -> bool {
        (self.data.default_read_le::<u8>(0x16) & 8) != 0
    }

    fn set_is_favorite(&mut self, val: bool) {
        let flags = (self.data.default_read_le::<u8>(0x16) & !8) | (u8::from(val) << 3);
        self.data.checked_write_le(0x16, &flags);
    }
}

impl DynamaxLevel for PK8 {
    fn set_dynamax_level(&mut self, level: u8) {
        self.data.checked_write_le(0x90, &level);
    }
}

impl DynamaxLevelReadOnly for PK8 {
    fn dynamax_level(&self) -> u8 {
        self.data.default_read_le(0x90)
    }
}

impl RibbonIndex for PK8 {
    fn get_ribbon(&self, index: usize) -> bool {
        flag_util::get_flag(&self.data, self.get_ribbon_byte(index), index & 7)
    }

    fn set_ribbon(&mut self, index: usize, val: bool) {
        let byte = self.get_ribbon_byte(index);
        flag_util::set_flag(&mut self.data, byte, index & 7, val);
    }

    fn get_ribbon_byte(&self, mut index: usize) -> usize {
        if index >= 128 {
            return 0;
        }
        if index < 64 {
            return 0x34 + (index >> 3);
        }
        index -= 64;
        0x40 + (index >> 3)
    }
}

impl HandlerLanguage for PK8 {
    fn ht_language(&self) -> u8 {
        self.data.default_read_le(0xC3)
    }

    fn set_ht_language(&mut self, lang: u8) {
        self.data.checked_write_le(0xC3, &lang);
    }
}

impl FormArgument for PK8 {
    fn form_argument(&self) -> u32 {
        self.data.default_read_le(0xE4)
    }

    fn set_form_argument(&mut self, arg: u32) {
        self.data.checked_write_le(0xE4, &arg);
    }

    fn form_argument_remain(&self) -> u8 {
        self.form_argument() as u8
    }

    fn set_form_argument_remain(&mut self, remain: u8) {
        self.set_form_argument((self.form_argument() & !0xFF) | remain as u32);
    }

    fn form_argument_elapsed(&self) -> u8 {
        (self.form_argument() >> 8) as u8
    }

    fn set_form_argument_elapsed(&mut self, elapsed: u8) {
        self.set_form_argument((self.form_argument() & !0xFF00) | ((elapsed as u32) << 8));
    }

    fn form_argument_maximum(&self) -> u8 {
        (self.form_argument() >> 16) as u8
    }

    fn set_form_argument_maximum(&mut self, maximum: u8) {
        self.set_form_argument((self.form_argument() & !0xFF0000) | ((maximum as u32) << 16));
    }
}

impl HomeTrack for PK8 {
    fn tracker(&self) -> u64 {
        self.data.default_read_le(0x135)
    }

    fn set_tracker(&mut self, tracker: u64) {
        self.data.checked_write_le(0x135, &tracker);
    }
}

impl BattleVersion for PK8 {
    fn battle_version(&self) -> u8 {
        self.data.default_read_le(0xDF)
    }

    fn set_battle_version(&mut self, version: u8) {
        self.data.checked_write_le(0xDF, &version);
    }
}

impl TrainerMemories for PK8 {}

impl MemoryOT for PK8 {
    fn set_ot_memory(&mut self, memory: u8) {
        self.data.checked_write_le(0x114, &memory);
    }

    fn set_ot_intensity(&mut self, intensity: u8) {
        self.data.checked_write_le(0x113, &intensity);
    }

    fn set_ot_feeling(&mut self, feeling: u8) {
        self.data.checked_write_le(0x118, &feeling);
    }

    fn set_ot_text_var(&mut self, var: u16) {
        self.data.checked_write_le(0x116, &var);
    }
}

impl MemoryOTReadOnly for PK8 {
    fn ot_memory(&self) -> u8 {
        self.data.default_read_le(0x114)
    }

    fn ot_intensity(&self) -> u8 {
        self.data.default_read_le(0x113)
    }

    fn ot_feeling(&self) -> u8 {
        self.data.default_read_le(0x118)
    }

    fn ot_text_var(&self) -> u16 {
        self.data.default_read_le(0x116)
    }
}

impl MemoryHT for PK8 {
    fn set_ht_memory(&mut self, memory: u8) {
        self.data.checked_write_le(0xCA, &memory);
    }

    fn set_ht_intensity(&mut self, intensity: u8) {
        self.data.checked_write_le(0xC9, &intensity);
    }

    fn set_ht_feeling(&mut self, feeling: u8) {
        self.data.checked_write_le(0xCB, &feeling);
    }

    fn set_ht_text_var(&mut self, var: u16) {
        self.data.checked_write_le(0xCC, &var);
    }
}

impl MemoryHTReadOnly for PK8 {
    fn ht_memory(&self) -> u8 {
        self.data.default_read_le(0xCA)
    }

    fn ht_intensity(&self) -> u8 {
        self.data.default_read_le(0xC9)
    }

    fn ht_feeling(&self) -> u8 {
        self.data.default_read_le(0xCB)
    }

    fn ht_text_var(&self) -> u16 {
        self.data.default_read_le(0xCC)
    }
}

impl G8Pkm for PK8 {}
