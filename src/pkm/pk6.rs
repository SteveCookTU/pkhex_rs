use crate::personal_info_oras::PersonalInfoORAS;
use crate::personal_table::AO;
use crate::poke_crypto::{decrypt_if_encrypted_67, SIZE_6PARTY};
use crate::tables::{
    MAX_ABILITY_ID_6_AO, MAX_BALL_ID_6, MAX_GAME_ID_6, MAX_ITEM_ID_6_AO, MAX_MOVE_ID_6_AO,
    MAX_SPECIES_ID_6,
};
use crate::{
    string_converter_6, Affection, ContestStats, ContestStatsMutable, FormArgument, G6Pkm,
    GameValueLimit, Generation, GeoTrack, GroundTile, GroundTileType, LangNick, MemoryHT, MemoryOT,
    NatureT, PersonalInfo, Pkm, RegionOrigin, RibbonSetCommon3, RibbonSetCommon4, RibbonSetCommon6,
    RibbonSetEvent3, RibbonSetEvent4, SanityChecksum, Shiny, SpeciesForm, StringConverterOption,
    SuperTrain, TrainerId, TrainerInfo, TrainerMemories,
};

const UNUSED: [u16; 23] = [
    0x36, 0x37, // Unused Ribbons
    0x58, 0x59, 0x73, 0x90, 0x91, 0x9E, 0x9F, 0xA0, 0xA1, 0xA7, 0xAA, 0xAB, 0xAC, 0xAD, 0xC8, 0xC9,
    0xD7, 0xE4, 0xE5, 0xE6, 0xE7,
];

#[derive(Clone)]
pub struct PK6 {
    data: Vec<u8>,
}

impl PK6 {
    fn decrypt_party(mut data: Vec<u8>) -> Vec<u8> {
        decrypt_if_encrypted_67(&mut data);
        data.resize(SIZE_6PARTY, 0);
        data
    }

    pub fn get_training_bag_hits(&self) -> u8 {
        self.data[0x16]
    }

    pub fn set_training_bag_hits(&mut self, hits: u8) {
        self.data[0x16] = hits;
    }

    pub fn get_training_bag(&self) -> u8 {
        self.data[0x17]
    }

    pub fn set_training_bag(&mut self, bag: u8) {
        self.data[0x17] = bag;
    }

    fn get_pkrs(&self) -> u8 {
        self.data[0x2B]
    }

    fn set_pkrs(&mut self, pkrs: u8) {
        self.data[0x2B] = pkrs;
    }

    fn get_st1(&self) -> u8 {
        self.data[0x2C]
    }

    fn set_st1(&mut self, st1: u8) {
        self.data[0x2C] = st1;
    }

    pub fn get_unused_0(&self) -> bool {
        (self.get_st1() & 1) == 1
    }

    pub fn set_unused_0(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !1) | if flag { 1 } else { 0 });
    }

    pub fn get_unused_1(&self) -> bool {
        (self.get_st1() & (1 << 1)) == (1 << 1)
    }

    pub fn set_unused_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    pub fn get_super_train_1_spa(&self) -> bool {
        (self.get_st1() & (1 << 2)) == (1 << 2)
    }

    pub fn set_super_train_1_spa(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    pub fn get_super_train_1_hp(&self) -> bool {
        (self.get_st1() & (1 << 3)) == (1 << 3)
    }

    pub fn set_super_train_1_hp(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    pub fn get_super_train_1_atk(&self) -> bool {
        (self.get_st1() & (1 << 4)) == (1 << 4)
    }

    pub fn set_super_train_1_atk(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    pub fn get_super_train_1_spd(&self) -> bool {
        (self.get_st1() & (1 << 5)) == (1 << 5)
    }

    pub fn set_super_train_1_spd(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_super_train_1_spe(&self) -> bool {
        (self.get_st1() & (1 << 6)) == (1 << 6)
    }

    pub fn set_super_train_1_spe(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    pub fn get_super_train_1_def(&self) -> bool {
        (self.get_st1() & (1 << 7)) == (1 << 7)
    }

    pub fn set_super_train_1_def(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_st2(&self) -> u8 {
        self.data[0x2D]
    }

    fn set_st2(&mut self, st2: u8) {
        self.data[0x2D] = st2;
    }

    pub fn get_super_train_2_spa(&self) -> bool {
        (self.get_st1() & 1) == 1
    }

    pub fn set_super_train_2_spa(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !1) | if flag { 1 } else { 0 });
    }

    pub fn get_super_train_2_hp(&self) -> bool {
        (self.get_st1() & (1 << 1)) == (1 << 1)
    }

    pub fn set_super_train_2_hp(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    pub fn get_super_train_2_atk(&self) -> bool {
        (self.get_st1() & (1 << 2)) == (1 << 2)
    }

    pub fn set_super_train_2_atk(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    pub fn get_super_train_2_spd(&self) -> bool {
        (self.get_st1() & (1 << 3)) == (1 << 3)
    }

    pub fn set_super_train_2_spd(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    pub fn get_super_train_2_spe(&self) -> bool {
        (self.get_st1() & (1 << 4)) == (1 << 4)
    }

    pub fn set_super_train_2_spe(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    pub fn get_super_train_2_def(&self) -> bool {
        (self.get_st1() & (1 << 5)) == (1 << 5)
    }

    pub fn set_super_train_2_def(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_super_train_3_spa(&self) -> bool {
        (self.get_st1() & (1 << 6)) == (1 << 6)
    }

    pub fn set_super_train_3_spa(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    pub fn get_super_train_3_hp(&self) -> bool {
        (self.get_st1() & (1 << 7)) == (1 << 7)
    }

    pub fn set_super_train_3_hp(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_st3(&self) -> u8 {
        self.data[0x2E]
    }

    fn set_st3(&mut self, st3: u8) {
        self.data[0x2E] = st3;
    }

    pub fn get_super_train_3_atk(&self) -> bool {
        (self.get_st1() & 1) == 1
    }

    pub fn set_super_train_3_atk(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !1) | if flag { 1 } else { 0 });
    }

    pub fn get_super_train_3_spd(&self) -> bool {
        (self.get_st1() & (1 << 1)) == (1 << 1)
    }

    pub fn set_super_train_3_spd(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    pub fn get_super_train_3_spe(&self) -> bool {
        (self.get_st1() & (1 << 2)) == (1 << 2)
    }

    pub fn set_super_train_3_spe(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    pub fn get_super_train_3_def(&self) -> bool {
        (self.get_st1() & (1 << 3)) == (1 << 3)
    }

    pub fn set_super_train_3_def(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    pub fn get_super_train_4_1(&self) -> bool {
        (self.get_st1() & (1 << 4)) == (1 << 4)
    }

    pub fn set_super_train_4_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    pub fn get_super_train_5_1(&self) -> bool {
        (self.get_st1() & (1 << 5)) == (1 << 5)
    }

    pub fn set_super_train_5_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_super_train_5_2(&self) -> bool {
        (self.get_st1() & (1 << 6)) == (1 << 6)
    }

    pub fn set_super_train_5_2(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    pub fn get_super_train_5_3(&self) -> bool {
        (self.get_st1() & (1 << 7)) == (1 << 7)
    }

    pub fn set_super_train_5_3(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_st4(&self) -> u8 {
        self.data[0x2F]
    }

    fn set_st4(&mut self, st3: u8) {
        self.data[0x2F] = st3;
    }

    pub fn get_super_train_5_4(&self) -> bool {
        (self.get_st1() & 1) == 1
    }

    pub fn set_super_train_5_4(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !1) | if flag { 1 } else { 0 });
    }

    pub fn get_super_train_6_1(&self) -> bool {
        (self.get_st1() & (1 << 1)) == (1 << 1)
    }

    pub fn set_super_train_6_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    pub fn get_super_train_6_2(&self) -> bool {
        (self.get_st1() & (1 << 2)) == (1 << 2)
    }

    pub fn set_super_train_6_2(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    pub fn get_super_train_6_3(&self) -> bool {
        (self.get_st1() & (1 << 3)) == (1 << 3)
    }

    pub fn set_super_train_6_3(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    pub fn get_super_train_7_1(&self) -> bool {
        (self.get_st1() & (1 << 4)) == (1 << 4)
    }

    pub fn set_super_train_7_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    pub fn get_super_train_7_2(&self) -> bool {
        (self.get_st1() & (1 << 5)) == (1 << 5)
    }

    pub fn set_super_train_7_2(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_super_train_7_3(&self) -> bool {
        (self.get_st1() & (1 << 6)) == (1 << 6)
    }

    pub fn set_super_train_7_3(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    pub fn get_super_train_8_1(&self) -> bool {
        (self.get_st1() & (1 << 7)) == (1 << 7)
    }

    pub fn set_super_train_8_1(&mut self, flag: bool) {
        self.set_st1((self.get_st1() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    pub fn get_super_training_bit_flags(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x2C..0x30]).try_into().unwrap())
    }

    pub fn set_super_training_bit_flags(&mut self, flags: u32) {
        let bytes = flags.to_le_bytes();
        self.data.splice(0x2C..0x30, bytes);
    }

    fn get_rib_0(&self) -> u8 {
        self.data[0x30]
    }

    fn set_rib_0(&mut self, flags: u8) {
        self.data[0x30] = flags;
    }

    fn get_rib_1(&self) -> u8 {
        self.data[0x31]
    }

    fn set_rib_1(&mut self, flags: u8) {
        self.data[0x31] = flags;
    }

    fn get_rib_2(&self) -> u8 {
        self.data[0x32]
    }

    fn set_rib_2(&mut self, flags: u8) {
        self.data[0x32] = flags;
    }

    fn get_rib_3(&self) -> u8 {
        self.data[0x33]
    }

    fn set_rib_3(&mut self, flags: u8) {
        self.data[0x33] = flags;
    }

    fn get_rib_4(&self) -> u8 {
        self.data[0x34]
    }

    fn set_rib_4(&mut self, flags: u8) {
        self.data[0x34] = flags;
    }

    fn get_rib_5(&self) -> u8 {
        self.data[0x35]
    }

    fn set_rib_5(&mut self, flags: u8) {
        self.data[0x35] = flags;
    }

    pub fn get_rib_5_6(&self) -> bool {
        (self.get_rib_5() & (1 << 6)) == (1 << 6)
    }

    pub fn set_rib_5_6(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    pub fn get_rib_5_7(&self) -> bool {
        (self.get_rib_5() & (1 << 7)) == (1 << 7)
    }

    pub fn set_rib_5_7(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_dist_byte(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x3A..0x3C]).try_into().unwrap())
    }

    fn set_dist_byte(&mut self, byte: u16) {
        let bytes = byte.to_le_bytes();
        self.data.splice(0x3A..0x3C, bytes);
    }

    pub fn get_dist_super_train_1(&self) -> bool {
        (self.get_dist_byte() & 1) == 1
    }

    pub fn set_dist_super_train_1(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !1) | if flag { 1 } else { 0 });
    }

    pub fn get_dist_super_train_2(&self) -> bool {
        (self.get_dist_byte() & (1 << 1)) == (1 << 1)
    }

    pub fn set_dist_super_train_2(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    pub fn get_dist_super_train_3(&self) -> bool {
        (self.get_dist_byte() & (1 << 2)) == (1 << 2)
    }

    pub fn set_dist_super_train_3(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    pub fn get_dist_super_train_4(&self) -> bool {
        (self.get_dist_byte() & (1 << 3)) == (1 << 3)
    }

    pub fn set_dist_super_train_4(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    pub fn get_dist_super_train_5(&self) -> bool {
        (self.get_dist_byte() & (1 << 4)) == (1 << 4)
    }

    pub fn set_dist_super_train_5(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    pub fn get_dist_super_train_6(&self) -> bool {
        (self.get_dist_byte() & (1 << 5)) == (1 << 5)
    }

    pub fn set_dist_super_train_6(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_dist_7(&self) -> bool {
        (self.get_dist_byte() & (1 << 6)) == (1 << 6)
    }

    pub fn set_dist_7(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    pub fn get_dist_8(&self) -> bool {
        (self.get_dist_byte() & (1 << 7)) == (1 << 7)
    }

    pub fn set_dist_8(&mut self, flag: bool) {
        self.set_dist_byte((self.get_dist_byte() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_iv32(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x74..0x78]).try_into().unwrap())
    }

    fn set_iv32(&mut self, iv32: u32) {
        let bytes = (iv32 as u32).to_le_bytes();
        self.data.splice(0x74..0x78, bytes);
    }

    pub fn is_untraded_event_6(&self) -> bool {
        self.get_geo_1_country() == 0
            && self.get_geo_1_region() == 0
            && self.get_met_location() / 10000 == 4
            && self.gen6()
    }

    pub fn fix_memories(&mut self) {
        if self.get_is_egg() {
            self.set_geo_1_country(0);
            self.set_geo_1_region(0);
            self.set_geo_2_country(0);
            self.set_geo_2_region(0);
            self.set_geo_3_country(0);
            self.set_geo_3_region(0);
            self.set_geo_4_country(0);
            self.set_geo_4_region(0);
            self.set_geo_5_country(0);
            self.set_geo_5_region(0);
            self.set_ht_friendship(0);
            self.set_ht_text_var(0);
            self.set_ht_memory(0);
            self.set_ht_intensity(0);
            self.set_ht_feeling(0);
            self.set_ot_text_var(0);
            self.set_ot_memory(0);
            self.set_ot_intensity(0);
            self.set_ot_feeling(0);
            self.set_ot_affection(0);
            self.set_ht_affection(0);

            let clear = vec![0u8; 26];
            self.data.splice(0x78..(0x78 + 26), clear);
            return;
        }

        if G6Pkm::is_untraded(self) {
            self.set_ht_friendship(0);
            self.set_ht_text_var(0);
            self.set_ht_memory(0);
            self.set_ht_intensity(0);
            self.set_ht_feeling(0);
            self.set_ht_affection(0);
        }

        if !self.gen6() {
            self.set_ot_text_var(0);
            self.set_ot_memory(0);
            self.set_ot_intensity(0);
            self.set_ot_feeling(0);
        }

        self.sanitize_geo_location_data();
    }

    pub fn get_has_contest_memory_ribbon(&self) -> bool {
        (self.get_rib_4() & (1 << 5)) == (1 << 5)
    }

    pub fn set_has_contest_memory_ribbon(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    pub fn get_has_battle_memory_ribbon(&self) -> bool {
        (self.get_rib_4() & (1 << 6)) == (1 << 6)
    }

    pub fn set_has_battle_memory_ribbon(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }
}

impl RibbonSetCommon3 for PK6 {
    fn get_ribbon_champion_g3(&self) -> bool {
        (self.get_rib_0() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_champion_g3(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_artist(&self) -> bool {
        (self.get_rib_2() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_artist(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_effort(&self) -> bool {
        (self.get_rib_0() & (1 << 7)) == (1 << 7)
    }

    fn set_ribbon_effort(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }
}

impl RibbonSetEvent3 for PK6 {
    fn get_ribbon_earth(&self) -> bool {
        (self.get_rib_3() & 1) == 1
    }

    fn set_ribbon_earth(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_national(&self) -> bool {
        (self.get_rib_2() & (1 << 7)) == (1 << 7)
    }

    fn set_ribbon_national(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_ribbon_country(&self) -> bool {
        (self.get_rib_2() & (1 << 6)) == (1 << 6)
    }

    fn set_ribbon_country(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    fn get_ribbon_champion_battle(&self) -> bool {
        (self.get_rib_4() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_champion_battle(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_champion_regional(&self) -> bool {
        (self.get_rib_4() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_champion_regional(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_champion_national(&self) -> bool {
        (self.get_rib_4() & (1 << 3)) == (1 << 3)
    }

    fn set_ribbon_champion_national(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }
}

impl RibbonSetEvent4 for PK6 {
    fn get_ribbon_classic(&self) -> bool {
        (self.get_rib_3() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_classic(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_wishing(&self) -> bool {
        (self.get_rib_4() & 1) == 1
    }

    fn set_ribbon_wishing(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_premier(&self) -> bool {
        (self.get_rib_3() & (1 << 3)) == (1 << 3)
    }

    fn set_ribbon_premier(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    fn get_ribbon_event(&self) -> bool {
        (self.get_rib_3() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_event(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_birthday(&self) -> bool {
        (self.get_rib_3() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_birthday(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    fn get_ribbon_special(&self) -> bool {
        (self.get_rib_3() & (1 << 6)) == (1 << 6)
    }

    fn set_ribbon_special(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    fn get_ribbon_world(&self) -> bool {
        (self.get_rib_3() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_world(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_champion_world(&self) -> bool {
        (self.get_rib_4() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_champion_world(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_souvenir(&self) -> bool {
        (self.get_rib_3() & (1 << 7)) == (1 << 7)
    }

    fn set_ribbon_souvenir(&mut self, flag: bool) {
        self.set_rib_3((self.get_rib_3() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }
}

impl RibbonSetCommon4 for PK6 {
    fn get_ribbon_champion_sinnoh(&self) -> bool {
        (self.get_rib_0() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_champion_sinnoh(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_alert(&self) -> bool {
        (self.get_rib_1() & 1) == 1
    }

    fn set_ribbon_alert(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_shock(&self) -> bool {
        (self.get_rib_1() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_shock(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_downcast(&self) -> bool {
        (self.get_rib_1() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_downcast(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_careless(&self) -> bool {
        (self.get_rib_1() & (1 << 3)) == (1 << 3)
    }

    fn set_ribbon_careless(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    fn get_ribbon_relax(&self) -> bool {
        (self.get_rib_1() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_relax(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_snooze(&self) -> bool {
        (self.get_rib_1() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_snooze(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    fn get_ribbon_smile(&self) -> bool {
        (self.get_rib_1() & (1 << 6)) == (1 << 6)
    }

    fn set_ribbon_smile(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    fn get_ribbon_gorgeous(&self) -> bool {
        (self.get_rib_1() & (1 << 7)) == (1 << 7)
    }

    fn set_ribbon_gorgeous(&mut self, flag: bool) {
        self.set_rib_1((self.get_rib_1() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_ribbon_royal(&self) -> bool {
        (self.get_rib_2() & 1) == 1
    }

    fn set_ribbon_royal(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_gorgeous_royal(&self) -> bool {
        (self.get_rib_2() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_gorgeous_royal(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_footprint(&self) -> bool {
        (self.get_rib_2() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_footprint(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    fn get_ribbon_record(&self) -> bool {
        (self.get_rib_2() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_record(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_legend(&self) -> bool {
        (self.get_rib_2() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_legend(&mut self, flag: bool) {
        self.set_rib_2((self.get_rib_2() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }
}

impl RibbonSetCommon6 for PK6 {
    fn get_ribbon_champion_kalos(&self) -> bool {
        (self.get_rib_0() & 1) == 1
    }

    fn set_ribbon_champion_kalos(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_champion_g6_hoenn(&self) -> bool {
        (self.get_rib_4() & (1 << 7)) == (1 << 7)
    }

    fn set_ribbon_champion_g6_hoenn(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 7)) | if flag { 1 << 7 } else { 0 });
    }

    fn get_ribbon_best_friends(&self) -> bool {
        (self.get_rib_0() & (1 << 3)) == (1 << 3)
    }

    fn set_ribbon_best_friends(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    fn get_ribbon_training(&self) -> bool {
        (self.get_rib_0() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_training(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_battler_skillful(&self) -> bool {
        (self.get_rib_0() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_battler_skillful(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    fn get_ribbon_battler_expert(&self) -> bool {
        (self.get_rib_0() & (1 << 6)) == (1 << 6)
    }

    fn set_ribbon_battler_expert(&mut self, flag: bool) {
        self.set_rib_0((self.get_rib_0() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
    }

    fn get_ribbon_contest_star(&self) -> bool {
        (self.get_rib_5() & 1) == 1
    }

    fn set_ribbon_contest_star(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !1) | if flag { 1 } else { 0 });
    }

    fn get_ribbon_master_coolness(&self) -> bool {
        (self.get_rib_5() & (1 << 1)) == (1 << 1)
    }

    fn set_ribbon_master_coolness(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 1)) | if flag { 1 << 1 } else { 0 });
    }

    fn get_ribbon_master_beauty(&self) -> bool {
        (self.get_rib_5() & (1 << 2)) == (1 << 2)
    }

    fn set_ribbon_master_beauty(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 2)) | if flag { 1 << 2 } else { 0 });
    }

    fn get_ribbon_master_cuteness(&self) -> bool {
        (self.get_rib_5() & (1 << 3)) == (1 << 3)
    }

    fn set_ribbon_master_cuteness(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 3)) | if flag { 1 << 3 } else { 0 });
    }

    fn get_ribbon_master_cleverness(&self) -> bool {
        (self.get_rib_5() & (1 << 4)) == (1 << 4)
    }

    fn set_ribbon_master_cleverness(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 4)) | if flag { 1 << 4 } else { 0 });
    }

    fn get_ribbon_master_toughness(&self) -> bool {
        (self.get_rib_5() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_master_toughness(&mut self, flag: bool) {
        self.set_rib_5((self.get_rib_5() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    fn get_ribbon_count_memory_contest(&self) -> usize {
        self.data[0x38] as usize
    }

    fn set_ribbon_count_memory_contest(&mut self, count: usize) {
        self.data[0x38] = count as u8;
        self.set_has_contest_memory_ribbon(count != 0);
    }

    fn get_ribbon_count_memory_battle(&self) -> usize {
        self.data[0x39] as usize
    }

    fn set_ribbon_count_memory_battle(&mut self, count: usize) {
        self.data[0x39] = count as u8;
        self.set_has_battle_memory_ribbon(count != 0);
    }
}

impl ContestStats for PK6 {
    fn get_cnt_cool(&self) -> u8 {
        self.data[0x24]
    }

    fn get_cnt_beauty(&self) -> u8 {
        self.data[0x25]
    }

    fn get_cnt_cute(&self) -> u8 {
        self.data[0x26]
    }

    fn get_cnt_smart(&self) -> u8 {
        self.data[0x27]
    }

    fn get_cnt_tough(&self) -> u8 {
        self.data[0x28]
    }

    fn get_cnt_sheen(&self) -> u8 {
        self.data[0x29]
    }
}

impl ContestStatsMutable for PK6 {
    fn set_cnt_cool(&mut self, cnt: u8) {
        self.data[0x24] = cnt;
    }

    fn set_cnt_beauty(&mut self, cnt: u8) {
        self.data[0x25] = cnt;
    }

    fn set_cnt_cute(&mut self, cnt: u8) {
        self.data[0x26] = cnt;
    }

    fn set_cnt_smart(&mut self, cnt: u8) {
        self.data[0x27] = cnt;
    }

    fn set_cnt_tough(&mut self, cnt: u8) {
        self.data[0x28] = cnt;
    }

    fn set_cnt_sheen(&mut self, cnt: u8) {
        self.data[0x29] = cnt;
    }
}

impl RegionOrigin for PK6 {
    fn get_console_region(&self) -> u8 {
        self.data[0xE2]
    }

    fn set_console_region(&mut self, console_region: u8) {
        self.data[0xE2] = console_region;
    }

    fn get_country(&self) -> u8 {
        self.data[0xE0]
    }

    fn set_country(&mut self, country: u8) {
        self.data[0xE0] = country;
    }

    fn get_region(&self) -> u8 {
        self.data[0xE1]
    }

    fn set_region(&mut self, region: u8) {
        self.data[0xE1] = region;
    }
}

impl GeoTrack for PK6 {
    fn get_geo_1_region(&self) -> u8 {
        self.data[0x94]
    }

    fn set_geo_1_region(&mut self, region: u8) {
        self.data[0x94] = region;
    }

    fn get_geo_2_region(&self) -> u8 {
        self.data[0x96]
    }

    fn set_geo_2_region(&mut self, region: u8) {
        self.data[0x96] = region;
    }

    fn get_geo_3_region(&self) -> u8 {
        self.data[0x98]
    }

    fn set_geo_3_region(&mut self, region: u8) {
        self.data[0x98] = region;
    }

    fn get_geo_4_region(&self) -> u8 {
        self.data[0x9A]
    }

    fn set_geo_4_region(&mut self, region: u8) {
        self.data[0x9A] = region;
    }

    fn get_geo_5_region(&self) -> u8 {
        self.data[0x9C]
    }

    fn set_geo_5_region(&mut self, region: u8) {
        self.data[0x9C] = region;
    }

    fn get_geo_1_country(&self) -> u8 {
        self.data[0x95]
    }

    fn set_geo_1_country(&mut self, country: u8) {
        self.data[0x95] = country;
    }

    fn get_geo_2_country(&self) -> u8 {
        self.data[0x97]
    }

    fn set_geo_2_country(&mut self, country: u8) {
        self.data[0x97] = country;
    }

    fn get_geo_3_country(&self) -> u8 {
        self.data[0x99]
    }

    fn set_geo_3_country(&mut self, country: u8) {
        self.data[0x99] = country;
    }

    fn get_geo_4_country(&self) -> u8 {
        self.data[0x9B]
    }

    fn set_geo_4_country(&mut self, country: u8) {
        self.data[0x9B] = country;
    }

    fn get_geo_5_country(&self) -> u8 {
        self.data[0x9D]
    }

    fn set_geo_5_country(&mut self, country: u8) {
        self.data[0x9D] = country;
    }
}

impl SuperTrain for PK6 {
    fn get_super_train_bit_flags(&self) -> usize {
        u32::from_le_bytes((&self.data[0x2C..0x30]).try_into().unwrap()) as usize
    }

    fn set_super_train_bit_flags(&mut self, flags: usize) {
        let bytes = (flags as u32).to_le_bytes();
        self.data.splice(0x2C..0x30, bytes);
    }

    fn get_secret_super_training_unlocked(&self) -> bool {
        (self.data[0x72] & 1) == 1
    }

    fn set_secret_super_training_unlocked(&mut self, unlocked: bool) {
        self.data[0x72] = (self.data[0x72] & !1) | if unlocked { 1 } else { 0 }
    }

    fn get_secret_super_training_complete(&self) -> bool {
        (self.data[0x72] & 2) == 2
    }

    fn set_secret_super_training_complete(&mut self, complete: bool) {
        self.data[0x72] = (self.data[0x72] & !2) | if complete { 2 } else { 0 }
    }

    fn get_super_training_medal_count(&self, max_count: usize) -> usize {
        let mut value = self.get_super_training_bit_flags() >> 2;

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

impl FormArgument for PK6 {
    fn get_form_argument(&self) -> usize {
        u32::from_le_bytes((&self.data[0x3C..0x40]).try_into().unwrap()) as usize
    }

    fn set_form_argument(&mut self, arg: usize) {
        let bytes = (arg as u32).to_le_bytes();
        self.data.splice(0x3C..0x40, bytes);
    }

    fn get_form_argument_remain(&self) -> u8 {
        self.data[0xED]
    }

    fn set_form_argument_remain(&mut self, remain: u8) {
        self.data[0xED] = remain;
    }

    fn get_form_argument_elapsed(&self) -> u8 {
        self.data[0xEE]
    }

    fn set_form_argument_elapsed(&mut self, elapsed: u8) {
        self.data[0xEE] = elapsed;
    }

    fn get_form_argument_maximum(&self) -> u8 {
        self.get_form_argument() as u8
    }

    fn set_form_argument_maximum(&mut self, maximum: u8) {
        self.set_form_argument(maximum as usize);
    }
}

impl MemoryOT for PK6 {
    fn get_ot_memory(&self) -> u8 {
        self.data[0xCD]
    }

    fn set_ot_memory(&mut self, memory: u8) {
        self.data[0xCD] = memory
    }

    fn get_ot_intensity(&self) -> u8 {
        self.data[0xCC]
    }

    fn set_ot_intensity(&mut self, intensity: u8) {
        self.data[0xCC] = intensity;
    }

    fn get_ot_feeling(&self) -> u8 {
        self.data[0xD0]
    }

    fn set_ot_feeling(&mut self, feeling: u8) {
        self.data[0xD0] = feeling;
    }

    fn get_ot_text_var(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xCE..0xD0]).try_into().unwrap())
    }

    fn set_ot_text_var(&mut self, var: u16) {
        let bytes = var.to_le_bytes();
        self.data.splice(0xCE..0xD0, bytes);
    }
}

impl MemoryHT for PK6 {
    fn get_ht_memory(&self) -> u8 {
        self.data[0xA5]
    }

    fn set_ht_memory(&mut self, memory: u8) {
        self.data[0xA5] = memory;
    }

    fn get_ht_intensity(&self) -> u8 {
        self.data[0xA4]
    }

    fn set_ht_intensity(&mut self, intensity: u8) {
        self.data[0xA4] = intensity;
    }

    fn get_ht_feeling(&self) -> u8 {
        self.data[0xA6]
    }

    fn set_ht_feeling(&mut self, feeling: u8) {
        self.data[0xA6] = feeling;
    }

    fn get_ht_text_var(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xA8..0xAA]).try_into().unwrap())
    }

    fn set_ht_text_var(&mut self, var: u16) {
        let bytes = var.to_le_bytes();
        self.data.splice(0xA8..0xAA, bytes);
    }
}

impl TrainerMemories for PK6 {}

impl Affection for PK6 {
    fn get_ot_affection(&self) -> u8 {
        self.data[0xCB]
    }

    fn set_ot_affection(&mut self, affection: u8) {
        self.data[0xCB] = affection;
    }

    fn get_ht_affection(&self) -> u8 {
        self.data[0xA3]
    }

    fn set_ht_affection(&mut self, affection: u8) {
        self.data[0xA3] = affection;
    }
}

impl GroundTile for PK6 {
    fn get_ground_tile(&self) -> GroundTileType {
        self.data[0xDE].into()
    }

    fn set_ground_tile(&mut self, tile: GroundTileType) {
        self.data[0xDE] = tile as u8;
    }
}

impl SpeciesForm for PK6 {
    fn get_species(&self) -> usize {
        u16::from_le_bytes((&self.data[8..0xA]).try_into().unwrap()) as usize
    }

    fn set_species(&mut self, species: usize) {
        let bytes = (species as u16).to_le_bytes();
        self.data.splice(8..0xA, bytes);
    }

    fn get_form(&self) -> usize {
        (self.data[0x1D] >> 3) as usize
    }

    fn set_form(&mut self, form: usize) {
        self.data[0x1D] = (self.data[0x1D] & 0x7) | ((form as u8) << 3);
    }
}

impl TrainerId for PK6 {
    fn get_tid(&self) -> usize {
        u16::from_le_bytes((&self.data[0xC..0xE]).try_into().unwrap()) as usize
    }

    fn set_tid(&mut self, tid: usize) {
        let bytes = (tid as u16).to_le_bytes();
        self.data.splice(0xC..0xE, bytes);
    }

    fn get_sid(&self) -> usize {
        u16::from_le_bytes((&self.data[0xE..0x10]).try_into().unwrap()) as usize
    }

    fn set_sid(&mut self, sid: usize) {
        let bytes = (sid as u16).to_le_bytes();
        self.data.splice(0xE..0x10, bytes);
    }
}

impl Generation for PK6 {
    fn get_generation(&self) -> usize {
        6
    }
}

impl Shiny for PK6 {
    fn get_is_shiny(&self) -> bool {
        Pkm::is_shiny(self)
    }
}

impl LangNick for PK6 {
    fn get_nickname(&self) -> String {
        string_converter_6::get_string(G6Pkm::nickname_trash(self))
    }

    fn set_nickname(&mut self, nickname: String) {
        let mut trash = G6Pkm::nickname_trash(self);
        string_converter_6::set_string(
            &mut trash,
            nickname.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0x40..(0x40 + 26), trash);
    }

    fn get_is_nicknamed(&self) -> bool {
        ((self.get_iv32() >> 31) & 1) == 1
    }

    fn set_is_nicknamed(&mut self, nicknamed: bool) {
        self.set_iv32((self.get_iv32() & !0x7FFFFFFF) | if nicknamed { 0x80000000 } else { 0 });
    }

    fn get_language(&self) -> usize {
        self.data[0xE3] as usize
    }

    fn set_language(&mut self, language: usize) {
        self.data[0xE3] = language as u8;
    }
}

impl GameValueLimit for PK6 {
    fn get_max_move_id(&self) -> usize {
        MAX_MOVE_ID_6_AO
    }

    fn get_max_species_id(&self) -> usize {
        MAX_SPECIES_ID_6
    }

    fn get_max_item_id(&self) -> usize {
        MAX_ITEM_ID_6_AO
    }

    fn get_max_ability_id(&self) -> usize {
        MAX_ABILITY_ID_6_AO
    }

    fn get_max_ball_id(&self) -> usize {
        MAX_BALL_ID_6
    }

    fn get_max_game_id(&self) -> usize {
        MAX_GAME_ID_6
    }

    fn get_min_game_id(&self) -> usize {
        0
    }

    fn get_max_iv(&self) -> usize {
        G6Pkm::max_iv(self)
    }

    fn get_max_ev(&self) -> usize {
        G6Pkm::max_ev(self)
    }

    fn get_ot_length(&self) -> usize {
        G6Pkm::ot_length(self)
    }

    fn get_nick_length(&self) -> usize {
        G6Pkm::nick_length(self)
    }
}

impl NatureT for PK6 {
    fn get_nature(&self) -> usize {
        self.data[0x1C] as usize
    }

    fn set_nature(&mut self, nature: usize) {
        self.data[0x1C] = nature as u8;
    }
}

impl Pkm<PersonalInfoORAS> for PK6 {
    fn size_party(&self) -> usize {
        G6Pkm::size_party(self)
    }

    fn size_stored(&self) -> usize {
        G6Pkm::size_stored(self)
    }

    fn get_type_name(&self) -> String {
        "PK6".to_string()
    }

    fn get_personal_info(&self) -> &PersonalInfoORAS {
        AO.get_form_entry(self.get_species(), self.get_form())
    }

    fn extra_bytes(&self) -> Vec<u16> {
        UNUSED.to_vec()
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn new(data: Vec<u8>) -> Self {
        Self {
            data: PK6::decrypt_party(data),
        }
    }

    fn new_blank() -> Self {
        Self {
            data: vec![0; SIZE_6PARTY],
        }
    }

    fn set_valid(&mut self, valid: bool) {
        G6Pkm::set_valid(self, valid);
    }

    fn get_valid(&self) -> bool {
        G6Pkm::get_valid(self)
    }

    fn nickname_trash(&self) -> Vec<u8> {
        G6Pkm::nickname_trash(self)
    }

    fn ot_trash(&self) -> Vec<u8> {
        G6Pkm::ot_trash(self)
    }

    fn encrypt(&mut self) -> Vec<u8> {
        G6Pkm::encrypt(self)
    }

    fn format(&self) -> usize {
        6
    }

    fn get_held_item(&self) -> usize {
        u16::from_le_bytes((&self.data[0xA..0xC]).try_into().unwrap()) as usize
    }

    fn set_held_item(&mut self, held_item: usize) {
        let bytes = (held_item as u16).to_le_bytes();
        self.data.splice(0xA..0xC, bytes);
    }

    fn get_gender(&self) -> usize {
        ((self.data[0x1D] >> 1) & 0x3) as usize
    }

    fn set_gender(&mut self, gender: usize) {
        self.data[0x1D] = (self.data[0x1D] & !6) | ((gender as u8) << 1);
    }

    fn get_ability(&self) -> usize {
        self.data[0x14] as usize
    }

    fn set_ability(&mut self, ability: usize) {
        self.data[0x14] = ability as u8;
    }

    fn get_current_friendship(&self) -> usize {
        G6Pkm::get_current_friendship(self)
    }

    fn set_current_friendship(&mut self, current_friendship: usize) {
        G6Pkm::set_current_friendship(self, current_friendship)
    }

    fn get_is_egg(&self) -> bool {
        ((self.get_iv32() >> 30) & 1) == 1
    }

    fn set_is_egg(&mut self, is_egg: bool) {
        self.set_iv32((self.get_iv32() & !0x40000000) | if is_egg { 0x40000000 } else { 0 });
    }

    fn get_exp(&self) -> usize {
        u32::from_le_bytes((&self.data[0x10..0x14]).try_into().unwrap()) as usize
    }

    fn set_exp(&mut self, exp: usize) {
        let bytes = (exp as u32).to_le_bytes();
        self.data.splice(0x10..0x14, bytes);
    }

    fn get_ot_name(&self) -> String {
        string_converter_6::get_string(G6Pkm::ot_trash(self))
    }

    fn set_ot_name(&mut self, ot_name: String) {
        let mut trash = G6Pkm::nickname_trash(self);
        string_converter_6::set_string(
            &mut trash,
            ot_name.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0xB0..(0xB0 + 26), trash);
    }

    fn get_ot_gender(&self) -> usize {
        (self.data[0xDD] >> 7) as usize
    }

    fn set_ot_gender(&mut self, ot_gender: usize) {
        self.data[0xDD] = (self.data[0xDD] & !0x80) | ((ot_gender as u8) << 7);
    }

    fn get_ball(&self) -> usize {
        self.data[0xDC] as usize
    }

    fn set_ball(&mut self, ball: usize) {
        self.data[0xDC] = ball as u8;
    }

    fn get_met_level(&self) -> usize {
        (self.data[0xDD] & !0x80) as usize
    }

    fn set_met_level(&mut self, met_level: usize) {
        self.data[0xDD] = (self.data[0xDD] & 0x80) | met_level as u8;
    }

    fn get_move_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0x5A..0x5C]).try_into().unwrap()) as usize
    }

    fn set_move_1(&mut self, move_1: usize) {
        let bytes = (move_1 as u16).to_le_bytes();
        self.data.splice(0x5A..0x5C, bytes);
    }

    fn get_move_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x5C..0x5E]).try_into().unwrap()) as usize
    }

    fn set_move_2(&mut self, move_2: usize) {
        let bytes = (move_2 as u16).to_le_bytes();
        self.data.splice(0x5C..0x5E, bytes);
    }

    fn get_move_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x5E..0x60]).try_into().unwrap()) as usize
    }

    fn set_move_3(&mut self, move_3: usize) {
        let bytes = (move_3 as u16).to_le_bytes();
        self.data.splice(0x5E..0x60, bytes);
    }

    fn get_move_4(&self) -> usize {
        u16::from_le_bytes((&self.data[0x60..0x62]).try_into().unwrap()) as usize
    }

    fn set_move_4(&mut self, move_4: usize) {
        let bytes = (move_4 as u16).to_le_bytes();
        self.data.splice(0x60..0x62, bytes);
    }

    fn get_move_1_pp(&self) -> usize {
        self.data[0x62] as usize
    }

    fn set_move_1_pp(&mut self, move_1_pp: usize) {
        self.data[0x62] = move_1_pp as u8;
    }

    fn get_move_2_pp(&self) -> usize {
        self.data[0x63] as usize
    }

    fn set_move_2_pp(&mut self, move_2_pp: usize) {
        self.data[0x63] = move_2_pp as u8;
    }

    fn get_move_3_pp(&self) -> usize {
        self.data[0x64] as usize
    }

    fn set_move_3_pp(&mut self, move_3_pp: usize) {
        self.data[0x64] = move_3_pp as u8;
    }

    fn get_move_4_pp(&self) -> usize {
        self.data[0x65] as usize
    }

    fn set_move_4_pp(&mut self, move_4_pp: usize) {
        self.data[0x65] = move_4_pp as u8;
    }

    fn get_move_1_pp_ups(&self) -> usize {
        self.data[0x66] as usize
    }

    fn set_move_1_pp_ups(&mut self, move_1_pp_ups: usize) {
        self.data[0x66] = move_1_pp_ups as u8;
    }

    fn get_move_2_pp_ups(&self) -> usize {
        self.data[0x67] as usize
    }

    fn set_move_2_pp_ups(&mut self, move_2_pp_ups: usize) {
        self.data[0x67] = move_2_pp_ups as u8;
    }

    fn get_move_3_pp_ups(&self) -> usize {
        self.data[0x68] as usize
    }

    fn set_move_3_pp_ups(&mut self, move_3_pp_ups: usize) {
        self.data[0x68] = move_3_pp_ups as u8;
    }

    fn get_move_4_pp_ups(&self) -> usize {
        self.data[0x69] as usize
    }

    fn set_move_4_pp_ups(&mut self, move_4_pp_ups: usize) {
        self.data[0x69] = move_4_pp_ups as u8;
    }

    fn get_ev_hp(&self) -> usize {
        self.data[0x1E] as usize
    }

    fn set_ev_hp(&mut self, hp: usize) {
        self.data[0x1E] = hp as u8;
    }

    fn get_ev_atk(&self) -> usize {
        self.data[0x1F] as usize
    }

    fn set_ev_atk(&mut self, atk: usize) {
        self.data[0x1F] = atk as u8;
    }

    fn get_ev_def(&self) -> usize {
        self.data[0x20] as usize
    }

    fn set_ev_def(&mut self, def: usize) {
        self.data[0x20] = def as u8;
    }

    fn get_ev_spe(&self) -> usize {
        self.data[0x21] as usize
    }

    fn set_ev_spe(&mut self, spe: usize) {
        self.data[0x21] = spe as u8;
    }

    fn get_ev_spa(&self) -> usize {
        self.data[0x22] as usize
    }

    fn set_ev_spa(&mut self, spa: usize) {
        self.data[0x22] = spa as u8;
    }

    fn get_ev_spd(&self) -> usize {
        self.data[0x23] as usize
    }

    fn set_ev_spd(&mut self, spd: usize) {
        self.data[0x23] = spd as u8;
    }

    fn get_iv_hp(&self) -> usize {
        (self.get_iv32() & 0x1F) as usize
    }

    fn set_iv_hp(&mut self, hp: usize) {
        self.set_iv32((self.get_iv32() & !0x1F) | if hp > 31 { 31 } else { hp as u32 });
    }

    fn get_iv_atk(&self) -> usize {
        ((self.get_iv32() >> 5) & 0x1F) as usize
    }

    fn set_iv_atk(&mut self, atk: usize) {
        self.set_iv32(
            (self.get_iv32() & !(0x1F << 5)) | (if atk > 31 { 31 } else { atk as u32 } << 5),
        );
    }

    fn get_iv_def(&self) -> usize {
        ((self.get_iv32() >> 10) & 0x1F) as usize
    }

    fn set_iv_def(&mut self, def: usize) {
        self.set_iv32(
            (self.get_iv32() & !(0x1F << 10)) | (if def > 31 { 31 } else { def as u32 } << 10),
        );
    }

    fn get_iv_spe(&self) -> usize {
        ((self.get_iv32() >> 15) & 0x1F) as usize
    }

    fn set_iv_spe(&mut self, spe: usize) {
        self.set_iv32(
            (self.get_iv32() & !(0x1F << 15)) | (if spe > 31 { 31 } else { spe as u32 } << 15),
        );
    }

    fn get_iv_spa(&self) -> usize {
        ((self.get_iv32() >> 20) & 0x1F) as usize
    }

    fn set_iv_spa(&mut self, spa: usize) {
        self.set_iv32(
            (self.get_iv32() & !(0x1F << 20)) | (if spa > 31 { 31 } else { spa as u32 } << 20),
        );
    }

    fn get_iv_spd(&self) -> usize {
        ((self.get_iv32() >> 25) & 0x1F) as usize
    }

    fn set_iv_spd(&mut self, spd: usize) {
        self.set_iv32(
            (self.get_iv32() & !(0x1F << 25)) | (if spd > 31 { 31 } else { spd as u32 } << 25),
        );
    }

    fn get_status_condition(&self) -> usize {
        u32::from_le_bytes((&self.data[0xE8..0xEC]).try_into().unwrap()) as usize
    }

    fn set_status_condition(&mut self, status_condition: usize) {
        let bytes = (status_condition as u32).to_le_bytes();
        self.data.splice(0xE8..0xEC, bytes);
    }

    fn get_stat_level(&self) -> usize {
        self.data[0xEC] as usize
    }

    fn set_stat_level(&mut self, level: usize) {
        self.data[0xEC] = level as u8;
    }

    fn get_stat_hp_max(&self) -> usize {
        u16::from_le_bytes((&self.data[0xF2..0xF4]).try_into().unwrap()) as usize
    }

    fn set_stat_hp_max(&mut self, hp: usize) {
        let bytes = (hp as u16).to_le_bytes();
        self.data.splice(0xF2..0xF4, bytes);
    }

    fn get_stat_hp_current(&self) -> usize {
        u16::from_le_bytes((&self.data[0xF0..0xF2]).try_into().unwrap()) as usize
    }

    fn set_stat_hp_current(&mut self, hp: usize) {
        let bytes = (hp as u16).to_le_bytes();
        self.data.splice(0xF0..0xF2, bytes);
    }

    fn get_stat_atk(&self) -> usize {
        u16::from_le_bytes((&self.data[0xF4..0xF6]).try_into().unwrap()) as usize
    }

    fn set_stat_atk(&mut self, atk: usize) {
        let bytes = (atk as u16).to_le_bytes();
        self.data.splice(0xF4..0xF6, bytes);
    }

    fn get_stat_def(&self) -> usize {
        u16::from_le_bytes((&self.data[0xF6..0xF8]).try_into().unwrap()) as usize
    }

    fn set_stat_def(&mut self, def: usize) {
        let bytes = (def as u16).to_le_bytes();
        self.data.splice(0xF6..0xF8, bytes);
    }

    fn get_stat_spe(&self) -> usize {
        u16::from_le_bytes((&self.data[0xF8..0xFA]).try_into().unwrap()) as usize
    }

    fn set_stat_spe(&mut self, spe: usize) {
        let bytes = (spe as u16).to_le_bytes();
        self.data.splice(0xF8..0xFA, bytes);
    }

    fn get_stat_spa(&self) -> usize {
        u16::from_le_bytes((&self.data[0xFA..0xFC]).try_into().unwrap()) as usize
    }

    fn set_stat_spa(&mut self, spa: usize) {
        let bytes = (spa as u16).to_le_bytes();
        self.data.splice(0xFA..0xFC, bytes);
    }

    fn get_stat_spd(&self) -> usize {
        u16::from_le_bytes((&self.data[0xFC..0xFE]).try_into().unwrap()) as usize
    }

    fn set_stat_spd(&mut self, spd: usize) {
        let bytes = (spd as u16).to_le_bytes();
        self.data.splice(0xFC..0xFE, bytes);
    }

    fn get_version(&self) -> usize {
        self.data[0xDF] as usize
    }

    fn set_version(&mut self, version: usize) {
        self.data[0xDF] = version as u8;
    }

    fn get_pkrs_strain(&self) -> usize {
        (self.get_pkrs() >> 4) as usize
    }

    fn set_pkrs_strain(&mut self, strain: usize) {
        self.set_pkrs((self.get_pkrs() & 0xF) | ((strain as u8) << 4));
    }

    fn get_pkrs_days(&self) -> usize {
        (self.get_pkrs() & 0xF) as usize
    }

    fn set_pkrs_days(&mut self, days: usize) {
        self.set_pkrs((self.get_pkrs() & !0xF) | days as u8)
    }

    fn get_encryption_constant(&self) -> usize {
        u32::from_le_bytes((&self.data[0..4]).try_into().unwrap()) as usize
    }

    fn set_encryption_constant(&mut self, ec: usize) {
        let ec_bytes = (ec as u32).to_le_bytes();
        self.data.splice(0..4, ec_bytes);
    }

    fn get_pid(&self) -> usize {
        u16::from_le_bytes((&self.data[0x18..0x1C]).try_into().unwrap()) as usize
    }

    fn set_pid(&mut self, pid: usize) {
        let bytes = (pid as u32).to_le_bytes();
        self.data.splice(0x18..0x1C, bytes);
    }

    fn get_fateful_encounter(&self) -> bool {
        (self.data[0x1D] & 1) == 1
    }

    fn set_fateful_encounter(&mut self, fe: bool) {
        self.data[0x1D] = (self.data[0x1D] & !1) | if fe { 1 } else { 0 };
    }

    fn get_tsv(&self) -> usize {
        G6Pkm::tsv(self)
    }

    fn set_tsv(&mut self, _tsv: usize) {}

    fn get_psv(&self) -> usize {
        G6Pkm::psv(self)
    }

    fn set_psv(&mut self, _set_psv: usize) {}

    fn get_characteristic(&self) -> usize {
        G6Pkm::get_characteristic(self)
    }

    fn set_characteristic(&mut self, _characteristic: usize) {}

    fn get_mark_value(&self) -> usize {
        self.data[0x2A] as usize
    }

    fn set_mark_value(&mut self, value: usize) {
        self.data[0x2A] = value as u8
    }

    fn get_met_location(&self) -> usize {
        u16::from_le_bytes((&self.data[0xDA..0xDC]).try_into().unwrap()) as usize
    }

    fn set_met_location(&mut self, location: usize) {
        let bytes = (location as u16).to_le_bytes();
        self.data.splice(0xDA..0xDC, bytes);
    }

    fn get_egg_location(&self) -> usize {
        u16::from_le_bytes((&self.data[0xD8..0xDA]).try_into().unwrap()) as usize
    }

    fn set_egg_location(&mut self, location: usize) {
        let bytes = (location as u16).to_le_bytes();
        self.data.splice(0xD8..0xDA, bytes);
    }

    fn get_ot_friendship(&self) -> usize {
        self.data[0xCA] as usize
    }

    fn set_ot_friendship(&mut self, friendship: usize) {
        self.data[0xCA] = friendship as u8;
    }

    fn get_met_year(&self) -> usize {
        self.data[0xD4] as usize
    }

    fn set_met_year(&mut self, year: usize) {
        self.data[0xD4] = year as u8;
    }

    fn get_met_month(&self) -> usize {
        self.data[0xD5] as usize
    }

    fn set_met_month(&mut self, month: usize) {
        self.data[0xD5] = month as u8;
    }

    fn get_met_day(&self) -> usize {
        self.data[0xD6] as usize
    }

    fn set_met_day(&mut self, day: usize) {
        self.data[0xD6] = day as u8;
    }

    fn get_ht_name(&self) -> String {
        string_converter_6::get_string(G6Pkm::ht_trash(self))
    }

    fn set_ht_name(&mut self, name: String) {
        let mut trash = G6Pkm::nickname_trash(self);
        string_converter_6::set_string(
            &mut trash,
            name.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0x78..(0x78 + 26), trash);
    }

    fn get_ht_gender(&self) -> usize {
        self.data[0x92] as usize
    }

    fn set_ht_gender(&mut self, gender: usize) {
        self.data[0x92] = gender as u8
    }

    fn get_ht_friendship(&self) -> usize {
        self.data[0xA2] as usize
    }

    fn set_ht_friendship(&mut self, friendship: usize) {
        self.data[0xA2] = friendship as u8;
    }

    fn get_enjoyment(&self) -> u8 {
        self.data[0xAF]
    }

    fn set_enjoyment(&mut self, enjoyment: u8) {
        self.data[0xAF] = enjoyment;
    }

    fn get_fullness(&self) -> u8 {
        self.data[0xAE]
    }

    fn set_fullness(&mut self, fullness: u8) {
        self.data[0xAE] = fullness;
    }

    fn get_ability_number(&self) -> usize {
        self.data[0x15] as usize
    }

    fn set_ability_number(&mut self, ability_number: usize) {
        self.data[0x15] = ability_number as u8;
    }

    fn get_egg_year(&self) -> usize {
        self.data[0xD1] as usize
    }

    fn set_egg_year(&mut self, year: usize) {
        self.data[0xD1] = year as u8;
    }

    fn get_egg_month(&self) -> usize {
        self.data[0xD2] as usize
    }

    fn set_egg_month(&mut self, month: usize) {
        self.data[0xD2] = month as u8;
    }

    fn get_egg_day(&self) -> usize {
        self.data[0xD3] as usize
    }

    fn set_egg_day(&mut self, day: usize) {
        self.data[0xD3] = day as u8;
    }

    fn get_relearn_move_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0x6A..0x6C]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_1(&mut self, move_1: usize) {
        let bytes = (move_1 as u16).to_le_bytes();
        self.data.splice(0x6A..0x6C, bytes);
    }

    fn get_relearn_move_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x6C..0x6E]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_2(&mut self, move_2: usize) {
        let bytes = (move_2 as u16).to_le_bytes();
        self.data.splice(0x6C..0x6E, bytes);
    }

    fn get_relearn_move_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x6E..0x70]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_3(&mut self, move_3: usize) {
        let bytes = (move_3 as u16).to_le_bytes();
        self.data.splice(0x6E..0x70, bytes);
    }

    fn get_relearn_move_4(&self) -> usize {
        u16::from_le_bytes((&self.data[0x70..0x72]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_4(&mut self, move_4: usize) {
        let bytes = (move_4 as u16).to_le_bytes();
        self.data.splice(0x70..0x72, bytes);
    }

    fn get_current_handler(&self) -> usize {
        self.data[0x93] as usize
    }

    fn set_current_handler(&mut self, handler: usize) {
        self.data[0x93] = handler as u8;
    }

    fn refresh_checksum(&mut self) {
        G6Pkm::refresh_checksum(self);
    }

    fn checksum_valid(&self) -> bool {
        G6Pkm::checksum_valid(self)
    }
}

impl SanityChecksum for PK6 {
    fn get_sanity(&self) -> u16 {
        u16::from_le_bytes((&self.data[4..6]).try_into().unwrap()) as u16
    }

    fn set_sanity(&mut self, sanity: u16) {
        let sanity_bytes = sanity.to_le_bytes();
        self.data.splice(4..6, sanity_bytes);
    }

    fn get_checksum(&self) -> u16 {
        u16::from_le_bytes((&self.data[6..8]).try_into().unwrap()) as u16
    }

    fn set_checksum(&mut self, checksum: u16) {
        let bytes = checksum.to_le_bytes();
        self.data.splice(6..8, bytes);
    }
}

impl G6Pkm<PersonalInfoORAS> for PK6 {
    fn trade_ot<T: TrainerInfo + ?Sized>(&mut self, tr: &T) -> bool {
        if !(tr.get_ot() == self.get_ot_name()
            && tr.get_tid() == self.get_tid()
            && tr.get_sid() == self.get_sid()
            && tr.get_gender() == self.get_gender())
        {
            return false;
        }

        self.set_current_handler(0);
        // TODO: RegionOrigin trade geo location

        true
    }

    fn trade_ht<T: TrainerInfo + ?Sized>(&mut self, tr: &T) {
        if tr.get_ot() != self.get_ht_name()
            || tr.get_gender() != self.get_ht_gender()
            || (self.get_geo_1_region() == 0
                && self.get_geo_1_country() == 0
                && !self.is_untraded_event_6())
        {
            //TODO: RegionOrigin trade geo location
        }

        if tr.get_ot() != self.get_ht_name() {
            self.set_ht_friendship(self.get_personal_info().get_base_friendship());
            self.set_ht_affection(0);
            self.set_ht_name(tr.get_ot());
        }

        self.set_current_handler(1);
        self.set_ht_gender(tr.get_gender());

        if self.get_ht_memory() == 0 {
            self.set_trade_memory_ht_6(false);
        }
    }
}
