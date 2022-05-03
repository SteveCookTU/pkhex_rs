use crate::personal_info_oras::PersonalInfoORAS;
use crate::personal_table::AO;
use crate::{Affection, ContestStats, ContestStatsMutable, FormArgument, G6Pkm, GameValueLimit, Generation, GeoTrack, GroundTile, GroundTileType, HyperTrain, LangNick, MemoryHT, MemoryOT, NatureT, PersonalInfo, Pkm, RegionOrigin, RibbonSetCommon3, RibbonSetCommon4, RibbonSetCommon6, RibbonSetEvent3, RibbonSetEvent4, SanityChecksum, Shiny, ShinyEnum, SpeciesForm, string_converter_6, SuperTrain, TrainerId, TrainerInfo, TrainerMemories};
use crate::poke_crypto::{decrypt_if_encrypted_67, SIZE_6PARTY};

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

    pub fn get_ribbon_count_memory_contest(&self) -> u8 {
        self.data[0x38]
    }

    pub fn set_ribbon_count_memory_contest(&mut self, count: u8) {
        self.data[0x38] = count;
        self.set_ribbon_memory_contest(count != 0);
    }

    pub fn get_ribbon_count_memory_battle(&self) -> u8 {
        self.data[0x39]
    }

    pub fn set_ribbon_count_memory_battle(&mut self, count: u8) {
        self.data[0x39] = count;
        self.set_ribbon_memory_battle(count != 0);
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
        self.set_dist_byte((self.get_dist_byte() & !1) | if flag { 1} else { 0 });
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

    fn get_ribbon_memory_contest(&self) -> bool {
        (self.get_rib_4() & (1 << 5)) == (1 << 5)
    }

    fn set_ribbon_memory_contest(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 5)) | if flag { 1 << 5 } else { 0 });
    }

    fn get_ribbon_memory_battle(&self) -> bool {
        (self.get_rib_4() & (1 << 6)) == (1 << 6)
    }

    fn set_ribbon_memory_battle(&mut self, flag: bool) {
        self.set_rib_4((self.get_rib_4() & !(1 << 6)) | if flag { 1 << 6 } else { 0 });
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
        todo!()
    }

    fn set_console_region(&mut self, console_region: u8) {
        todo!()
    }

    fn get_country(&self) -> u8 {
        todo!()
    }

    fn set_country(&mut self, country: u8) {
        todo!()
    }

    fn get_region(&self) -> u8 {
        todo!()
    }

    fn set_region(&mut self, region: u8) {
        todo!()
    }
}

impl GeoTrack for PK6 {
    fn get_geo_1_region(&self) -> u8 {
        todo!()
    }

    fn set_geo_1_region(&mut self, region: u8) {
        todo!()
    }

    fn get_geo_2_region(&self) -> u8 {
        todo!()
    }

    fn set_geo_2_region(&mut self, region: u8) {
        todo!()
    }

    fn get_geo_3_region(&self) -> u8 {
        todo!()
    }

    fn set_geo_3_region(&mut self, region: u8) {
        todo!()
    }

    fn get_geo_4_region(&self) -> u8 {
        todo!()
    }

    fn set_geo_4_region(&mut self, region: u8) {
        todo!()
    }

    fn get_geo_5_region(&self) -> u8 {
        todo!()
    }

    fn set_geo_5_region(&mut self, region: u8) {
        todo!()
    }

    fn get_geo_1_country(&self) -> u8 {
        todo!()
    }

    fn set_geo_1_country(&mut self, country: u8) {
        todo!()
    }

    fn get_geo_2_country(&self) -> u8 {
        todo!()
    }

    fn set_geo_2_country(&mut self, country: u8) {
        todo!()
    }

    fn get_geo_3_country(&self) -> u8 {
        todo!()
    }

    fn set_geo_3_country(&mut self, country: u8) {
        todo!()
    }

    fn get_geo_4_country(&self) -> u8 {
        todo!()
    }

    fn set_geo_4_country(&mut self, country: u8) {
        todo!()
    }

    fn get_geo_5_country(&self) -> u8 {
        todo!()
    }

    fn set_geo_5_country(&mut self, country: u8) {
        todo!()
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
        todo!()
    }

    fn set_secret_super_training_unlocked(&mut self, unlocked: bool) {
        todo!()
    }

    fn get_secret_super_training_complete(&self) -> bool {
        todo!()
    }

    fn set_secret_super_training_complete(&mut self, complete: bool) {
        todo!()
    }

    fn get_super_training_medal_count(&self) -> usize {
        todo!()
    }

    fn set_super_training_medal_count(&mut self, count: usize) {
        todo!()
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
        todo!()
    }

    fn set_form_argument_remain(&mut self, remain: u8) {
        todo!()
    }

    fn get_form_argument_elapsed(&self) -> u8 {
        todo!()
    }

    fn set_form_argument_elapsed(&mut self, elapsed: u8) {
        todo!()
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
        todo!()
    }

    fn set_ot_memory(&mut self, memory: u8) {
        todo!()
    }

    fn get_ot_intensity(&self) -> u8 {
        todo!()
    }

    fn set_ot_intensity(&mut self, memory: u8) {
        todo!()
    }

    fn get_ot_feeling(&self) -> u8 {
        todo!()
    }

    fn set_ot_feeling(&mut self, memory: u8) {
        todo!()
    }

    fn get_ot_text_var(&self) -> u16 {
        todo!()
    }

    fn set_ot_text_var(&mut self, var: u16) {
        todo!()
    }
}

impl MemoryHT for PK6 {
    fn get_ht_memory(&self) -> u8 {
        todo!()
    }

    fn set_ht_memory(&mut self, memory: u8) {
        todo!()
    }

    fn get_ht_intensity(&self) -> u8 {
        todo!()
    }

    fn set_ht_intensity(&mut self, memory: u8) {
        todo!()
    }

    fn get_ht_feeling(&self) -> u8 {
        todo!()
    }

    fn set_ht_feeling(&mut self, memory: u8) {
        todo!()
    }

    fn get_ht_text_var(&self) -> u16 {
        todo!()
    }

    fn set_ht_text_var(&mut self, var: u16) {
        todo!()
    }
}

impl TrainerMemories for PK6 {}

impl Affection for PK6 {
    fn get_ot_affection(&self) -> u8 {
        todo!()
    }

    fn set_ot_affect(&mut self, affection: u8) {
        todo!()
    }

    fn get_ht_affection(&self) -> u8 {
        todo!()
    }

    fn set_ht_affection(&self, affection: u8) {
        todo!()
    }
}

impl GroundTile for PK6 {
    fn get_ground_tile(&self) -> GroundTileType {
        todo!()
    }

    fn set_ground_tile(&mut self, tile: GroundTileType) {
        todo!()
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
        todo!()
    }
}

impl Shiny for PK6 {
    fn get_is_shiny(&self) -> bool {
        todo!()
    }
}

impl LangNick for PK6 {
    fn get_nickname(&self) -> String {
        string_converter_6::get_string(G6Pkm::nickname_trash(self))
    }

    fn set_nickname(&mut self, nickname: String) {
        todo!()
    }

    fn get_is_nicknamed(&self) -> bool {
        todo!()
    }

    fn get_language(&self) -> usize {
        todo!()
    }

    fn set_language(&mut self, language: usize) {
        todo!()
    }
}

impl GameValueLimit for PK6 {
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
        todo!()
    }

    fn get_max_game_id(&self) -> usize {
        todo!()
    }

    fn get_min_game_id(&self) -> usize {
        todo!()
    }

    fn get_max_iv(&self) -> usize {
        todo!()
    }

    fn get_max_ev(&self) -> usize {
        todo!()
    }

    fn get_ot_length(&self) -> usize {
        todo!()
    }

    fn get_nick_length(&self) -> usize {
        todo!()
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
        todo!()
    }

    fn size_stored(&self) -> usize {
        todo!()
    }

    fn get_type_name(&self) -> String {
        todo!()
    }

    fn get_personal_info(&self) -> &PersonalInfoORAS {
        AO.get_form_entry(self.get_species(), self.get_form())
    }

    fn extra_bytes(&self) -> Vec<u16> {
        UNUSED.to_vec()
    }

    fn get_data(&self) -> &Vec<u8> {
        todo!()
    }

    fn new(data: Vec<u8>) -> Self {
        todo!()
    }

    fn new_blank() -> Self {
        Self {
            data: vec![0; SIZE_6PARTY]
        }
    }

    fn set_valid(&mut self, valid: bool) {
        todo!()
    }

    fn get_valid(&self) -> bool {
        todo!()
    }

    fn nickname_trash(&self) -> Vec<u8> {
        todo!()
    }

    fn ot_trash(&self) -> Vec<u8> {
        todo!()
    }

    fn encrypt(&self) -> Vec<u8> {
        todo!()
    }

    fn format(&self) -> usize {
        6
    }

    fn get_nickname(&self) -> String {
        todo!()
    }

    fn set_nickname(&mut self, nickname: String) {
        todo!()
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

    fn get_is_nicknamed(&self) -> bool {
        todo!()
    }

    fn set_is_nicknamed(&mut self, is_nicknamed: bool) {
        todo!()
    }

    fn get_exp(&self) -> usize {
        u32::from_le_bytes((&self.data[0x10..0x14]).try_into().unwrap()) as usize
    }

    fn set_exp(&mut self, exp: usize) {
        let bytes = (exp as u32).to_le_bytes();
        self.data.splice(0x10..0x14, bytes);
    }

    fn get_ot_name(&self) -> String {
        todo!()
    }

    fn set_ot_name(&mut self, ot_name: String) {
        todo!()
    }

    fn get_ot_gender(&self) -> usize {
        todo!()
    }

    fn set_ot_gender(&mut self, ot_gender: usize) {
        todo!()
    }

    fn get_ball(&self) -> usize {
        todo!()
    }

    fn set_ball(&mut self, ball: usize) {
        todo!()
    }

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
        todo!()
    }

    fn set_iv_hp(&mut self, hp: usize) {
        todo!()
    }

    fn get_iv_atk(&self) -> usize {
        todo!()
    }

    fn set_iv_atk(&mut self, atk: usize) {
        todo!()
    }

    fn get_iv_def(&self) -> usize {
        todo!()
    }

    fn set_iv_def(&mut self, def: usize) {
        todo!()
    }

    fn get_iv_spe(&self) -> usize {
        todo!()
    }

    fn set_iv_spe(&mut self, spe: usize) {
        todo!()
    }

    fn get_iv_spa(&self) -> usize {
        todo!()
    }

    fn set_iv_spa(&mut self, spa: usize) {
        todo!()
    }

    fn get_iv_spd(&self) -> usize {
        todo!()
    }

    fn set_iv_spd(&mut self, spd: usize) {
        todo!()
    }

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
        todo!()
    }

    fn set_tsv(&mut self, tsv: usize) {
        todo!()
    }

    fn get_psv(&self) -> usize {
        todo!()
    }

    fn set_psv(&mut self, set_psv: usize) {
        todo!()
    }

    fn get_characteristic(&self) -> usize {
        todo!()
    }

    fn set_characteristic(&mut self, characteristic: usize) {
        todo!()
    }

    fn get_mark_value(&self) -> usize {
        self.data[0x2A] as usize
    }

    fn set_mark_value(&mut self, value: usize) {
        self.data[0x2A] = value as u8
    }

    fn get_met_location(&self) -> usize {
        todo!()
    }

    fn set_met_location(&mut self, location: usize) {
        todo!()
    }

    fn get_egg_location(&self) -> usize {
        todo!()
    }

    fn set_egg_location(&mut self, location: usize) {
        todo!()
    }

    fn get_ot_friendship(&self) -> usize {
        todo!()
    }

    fn set_ot_friendship(&mut self, friendship: usize) {
        todo!()
    }

    fn get_ability_number(&self) -> usize {
        self.data[0x15] as usize
    }

    fn set_ability_number(&mut self, ability_number: usize) {
        self.data[0x15] = ability_number as u8;
    }

    fn get_current_handler(&self) -> usize {
        todo!()
    }

    fn set_current_handler(&mut self, handler: usize) {
        todo!()
    }

    fn max_move_id(&self) -> usize {
        todo!()
    }

    fn max_species_id(&self) -> usize {
        todo!()
    }

    fn max_item_id(&self) -> usize {
        todo!()
    }

    fn max_ability_id(&self) -> usize {
        todo!()
    }

    fn max_ball_id(&self) -> usize {
        todo!()
    }

    fn max_game_id(&self) -> usize {
        todo!()
    }

    fn max_iv(&self) -> usize {
        todo!()
    }

    fn max_ev(&self) -> usize {
        todo!()
    }

    fn ot_length(&self) -> usize {
        todo!()
    }

    fn nick_length(&self) -> usize {
        todo!()
    }

    fn refresh_checksum(&mut self) {
        todo!()
    }

    fn checksum_valid(&self) -> bool {
        todo!()
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
        todo!()
    }

    fn trade_ht<T: TrainerInfo + ?Sized>(&mut self, tr: &T) {
        todo!()
    }
}
