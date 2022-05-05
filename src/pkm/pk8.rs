use crate::personal_info_swsh::PersonalInfoSWSH;
use crate::personal_table::SWSH;
use crate::poke_crypto::{
    decrypt_if_encrypted_8, encrypt_array_8, SIZE_8APARTY, SIZE_8PARTY, SIZE_8STORED,
};
use crate::tables::locations::LINK_TRADE_6;
use crate::tables::{
    MAX_ABILITY_ID_8, MAX_BALL_ID_8, MAX_GAME_ID_8, MAX_ITEM_ID_8, MAX_MOVE_ID_8, MAX_SPECIES_ID_8,
    TMHM_SWSH,
};
use crate::{
    flag_util, personal_info_swsh, string_converter_8, BattleVersion, ContestStats,
    ContestStatsMutable, DynamaxLevel, Favorite, FormArgument, GameValueLimit, Generation,
    Gigantamax, HandlerLanguage, HomeTrack, HyperTrain, LangNick, MemoryHT, MemoryOT, NatureT,
    PersonalInfo, Pkm, RibbonIndex, RibbonSetAffixed, RibbonSetCommon3, RibbonSetCommon4,
    RibbonSetCommon6, RibbonSetCommon7, RibbonSetCommon8, RibbonSetEvent3, RibbonSetEvent4,
    RibbonSetMark8, SanityChecksum, ScaledSize, Shiny, Sociability, SpeciesForm,
    StringConverterOption, TechRecord8, TrainerId, TrainerInfo, TrainerMemories, G8PKM,
};

const UNUSED: [u16; 77] = [
    0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F, 0x4C, 0x4D, 0x4E, 0x4F, 0x52, 0x53, 0x54, 0x55, 0x56,
    0x57, 0x91, 0x92, 0x93, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7,
    0xC5, 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xE0,
    0xE1, // Old Console Region / Region
    0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
    0x115, 0x11F, // Alignment
    0x13D, 0x13E, 0x13F, 0x140, 0x141, 0x142, 0x143, 0x144, 0x145, 0x146, 0x147,
];

#[derive(Clone)]
pub struct PK8 {
    data: Vec<u8>,
}

impl PK8 {
    fn decrypt_party(mut data: Vec<u8>) -> Vec<u8> {
        decrypt_if_encrypted_8(&mut data);
        data.resize(SIZE_8APARTY, 0);
        data
    }

    pub fn get_flag_2(&self) -> bool {
        (self.data[0x22] & 2) == 2
    }

    pub fn set_flag_2(&mut self, flag: bool) {
        self.data[0x22] = (self.data[0x22] & !2) | if flag { 2 } else { 0 };
    }

    fn get_pkrs(&self) -> u8 {
        self.data[0x32]
    }

    fn set_pkrs(&mut self, pkrs: u8) {
        self.data[0x32] = pkrs;
    }

    pub fn get_has_contest_memory_ribbon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 5)
    }

    pub fn set_has_contest_memory_ribbon(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 5, flag);
    }

    pub fn get_has_battle_memory_ribbon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 6)
    }

    pub fn set_has_battle_memory_ribbon(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 6, flag);
    }

    pub fn get_rib_44_4(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 4)
    }

    pub fn set_rib_44_4(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 4, flag);
    }

    pub fn get_rib_44_5(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 5)
    }

    pub fn set_rib_44_5(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 5, flag);
    }

    pub fn get_rib_44_6(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 6)
    }

    pub fn set_rib_44_6(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 6, flag);
    }

    pub fn get_rib_44_7(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 7)
    }

    pub fn set_rib_44_7(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 7, flag);
    }

    pub fn get_rib_45_0(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 0)
    }

    pub fn set_rib_45_0(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 0, flag);
    }

    pub fn get_rib_45_1(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 1)
    }

    pub fn set_rib_45_1(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 1, flag);
    }

    pub fn get_rib_45_2(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 2)
    }

    pub fn set_rib_45_2(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 2, flag);
    }

    pub fn get_rib_45_3(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 3)
    }

    pub fn set_rib_45_3(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 3, flag);
    }

    pub fn get_rib_45_4(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 4)
    }

    pub fn set_rib_45_4(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 4, flag);
    }

    pub fn get_rib_45_5(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 5)
    }

    pub fn set_rib_45_5(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 5, flag);
    }

    pub fn get_rib_45_6(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 6)
    }

    pub fn set_rib_45_6(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 6, flag);
    }

    pub fn get_rib_45_7(&self) -> bool {
        flag_util::get_flag(&self.data, 0x45, 7)
    }

    pub fn set_rib_45_7(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x45, 7, flag);
    }

    pub fn get_rib_46_0(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 0)
    }

    pub fn set_rib_46_0(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 0, flag);
    }

    pub fn get_rib_46_1(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 1)
    }

    pub fn set_rib_46_1(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 1, flag);
    }

    pub fn get_rib_46_2(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 2)
    }

    pub fn set_rib_46_2(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 2, flag);
    }

    pub fn get_rib_46_3(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 3)
    }

    pub fn set_rib_46_3(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 3, flag);
    }

    pub fn get_rib_46_4(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 4)
    }

    pub fn set_rib_46_4(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 4, flag);
    }

    pub fn get_rib_46_5(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 5)
    }

    pub fn set_rib_46_5(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 5, flag);
    }

    pub fn get_rib_46_6(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 6)
    }

    pub fn set_rib_46_6(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 6, flag);
    }

    pub fn get_rib_46_7(&self) -> bool {
        flag_util::get_flag(&self.data, 0x46, 7)
    }

    pub fn set_rib_46_7(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x46, 7, flag);
    }

    pub fn get_rib_47_0(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 0)
    }

    pub fn set_rib_47_0(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 0, flag);
    }

    pub fn get_rib_47_1(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 1)
    }

    pub fn set_rib_47_1(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 1, flag);
    }

    pub fn get_rib_47_2(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 2)
    }

    pub fn set_rib_47_2(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 2, flag);
    }

    pub fn get_rib_47_3(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 3)
    }

    pub fn set_rib_47_3(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 3, flag);
    }

    pub fn get_rib_47_4(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 4)
    }

    pub fn set_rib_47_4(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 4, flag);
    }

    pub fn get_rib_47_5(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 5)
    }

    pub fn set_rib_47_5(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 5, flag);
    }

    pub fn get_rib_47_6(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 6)
    }

    pub fn set_rib_47_6(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 6, flag);
    }

    pub fn get_rib_47_7(&self) -> bool {
        flag_util::get_flag(&self.data, 0x47, 7)
    }

    pub fn set_rib_47_7(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x47, 7, flag);
    }

    fn get_iv32(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x8C..0x90]).try_into().unwrap())
    }

    fn set_iv32(&mut self, iv_32: u32) {
        let bytes = iv_32.to_le_bytes();
        self.data.splice(0x8C..090, bytes);
    }

    fn get_ht_trainer_id(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xC6..0xC8]).try_into().unwrap())
    }

    fn set_ht_trainer_id(&mut self, trainer_id: u16) {
        let bytes = (trainer_id as u16).to_le_bytes();
        self.data.splice(0xC6..0xC8, bytes);
    }

    pub fn trade<T: TrainerInfo>(&mut self, tr: &T, day: usize, month: usize, year: usize) {
        if self.get_is_egg() {
            if tr.get_ot() != self.get_ot_name()
                || tr.get_tid() != self.get_tid()
                || tr.get_sid() != self.get_sid()
                || tr.get_gender() != self.get_ot_gender()
            {
                self.set_link_trade_egg(day, month, year, LINK_TRADE_6);
            }
            return;
        }
        if !self.trade_ot(tr) {}
    }

    fn trade_ot<T: TrainerInfo>(&mut self, tr: &T) -> bool {
        if !(tr.get_ot() == self.get_ot_name()
            && tr.get_tid() == self.get_tid()
            && tr.get_sid() == self.get_sid()
            && tr.get_gender() == self.get_ot_gender())
        {
            false
        } else {
            self.set_current_handler(0);
            true
        }
    }

    fn trade_ht<T: TrainerInfo>(&mut self, tr: &T) {
        if self.get_ht_name() != tr.get_ot() {
            self.set_ht_friendship(50);
            self.set_ht_name(tr.get_ot());
        }
        self.set_current_handler(1);
        self.set_ht_gender(tr.get_gender());
        self.set_ht_language(tr.get_language() as u8);
        self.set_trade_memory_ht_8();
    }

    pub fn get_dynamax_type(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x156..0x158]).try_into().unwrap())
    }

    pub fn set_dynamax_type(&mut self, dynamax_type: u16) {
        let bytes = dynamax_type.to_le_bytes();
        self.data.splice(0x156..0x158, bytes);
    }

    pub fn fix_memories(&mut self) {
        if self.get_is_egg() {
            self.set_ht_friendship(0);
            self.set_ht_text_var(0);
            self.set_ht_memory(0);
            self.set_ht_intensity(0);
            self.set_ht_feeling(0);
            self.set_ht_language(0);
            self.set_ot_text_var(0);
            self.set_ot_memory(0);
            self.set_ot_intensity(0);
            self.set_ot_feeling(0);

            let clear = vec![0u8; 26];
            self.data.splice(0xA8..(0xA8 + 26), clear);
            return;
        }

        if self.is_untraded() {
            self.set_ht_friendship(0);
            self.set_ht_text_var(0);
            self.set_ht_memory(0);
            self.set_ht_intensity(0);
            self.set_ht_feeling(0);
            self.set_ht_language(0);
        }

        let gen = self.generation();
        if gen < 6 {
            self.set_ot_text_var(0);
            self.set_ot_memory(0);
            self.set_ot_intensity(0);
            self.set_ot_feeling(0)
        }
        if gen != 8 {
            self.set_trade_memory_ht_8();
        }
    }
}

impl Pkm<PersonalInfoSWSH> for PK8 {
    fn size_party(&self) -> usize {
        SIZE_8PARTY
    }

    fn size_stored(&self) -> usize {
        SIZE_8STORED
    }

    fn get_type_name(&self) -> String {
        "PK8".to_string()
    }

    fn get_personal_info(&self) -> &PersonalInfoSWSH {
        SWSH.get_form_entry(self.get_species(), self.get_form())
    }

    fn extra_bytes(&self) -> Vec<u16> {
        UNUSED.to_vec()
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn new(data: Vec<u8>) -> Self {
        Self {
            data: PK8::decrypt_party(data),
        }
    }

    fn new_blank() -> Self {
        let mut pk8 = Self {
            data: vec![0; SIZE_8APARTY],
        };
        pk8.set_affixed_ribbon(-1);
        pk8
    }

    fn set_valid(&mut self, valid: bool) {
        if valid {
            self.set_sanity(0);
            self.refresh_checksum();
        }
    }

    fn get_valid(&self) -> bool {
        self.get_sanity() == 0 && self.checksum_valid()
    }

    fn nickname_trash(&self) -> Vec<u8> {
        self.data[0x58..(0x58 + 26)].to_vec()
    }

    fn ot_trash(&self) -> Vec<u8> {
        self.data[0xF8..(0xF8 + 26)].to_vec()
    }

    fn ht_trash(&self) -> Vec<u8> {
        self.data[0xA8..(0xA8 + 26)].to_vec()
    }

    fn encrypt(&mut self) -> Vec<u8> {
        self.refresh_checksum();
        encrypt_array_8(&self.data)
    }

    fn format(&self) -> usize {
        8
    }

    fn get_held_item(&self) -> usize {
        u16::from_le_bytes((&self.data[0xA..0xC]).try_into().unwrap()) as usize
    }

    fn set_held_item(&mut self, held_item: usize) {
        let bytes = (held_item as u16).to_le_bytes();
        self.data.splice(0xA..0xC, bytes);
    }

    fn get_gender(&self) -> usize {
        ((self.data[0x22] >> 2) & 0x3) as usize
    }

    fn set_gender(&mut self, gender: usize) {
        self.data[0x22] = (self.data[0x22] & 0xF3) | ((gender as u8) << 2);
    }

    fn get_stat_nature(&self) -> usize {
        self.data[0x21] as usize
    }

    fn set_stat_nature(&mut self, stat_nature: usize) {
        self.data[0x21] = stat_nature as u8;
    }

    fn get_ability(&self) -> usize {
        u16::from_le_bytes((&self.data[0x14..0x16]).try_into().unwrap()) as usize
    }

    fn set_ability(&mut self, ability: usize) {
        let bytes = (ability as u16).to_le_bytes();
        self.data.splice(0x14..0x16, bytes);
    }

    fn get_current_friendship(&self) -> usize {
        if self.get_current_handler() == 0 {
            self.get_ot_friendship()
        } else {
            self.get_ht_friendship()
        }
    }

    fn set_current_friendship(&mut self, current_friendship: usize) {
        if self.get_current_handler() == 0 {
            self.set_ot_friendship(current_friendship);
        } else {
            self.set_ht_friendship(current_friendship);
        }
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
        let bytes = (exp as u16).to_le_bytes();
        self.data.splice(0x10..0x14, bytes);
    }

    fn get_ot_name(&self) -> String {
        string_converter_8::get_string(self.ot_trash())
    }

    fn set_ot_name(&mut self, ot_name: String) {
        let mut trash = self.ot_trash();
        string_converter_8::set_string(
            &mut trash,
            ot_name.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0xF8..(0xF8 + 26), trash);
    }

    fn get_ot_gender(&self) -> usize {
        (self.data[0x125] >> 7) as usize
    }

    fn set_ot_gender(&mut self, ot_gender: usize) {
        self.data[0x125] = (self.data[0x125] & !0x80) | ((ot_gender as u8) << 7);
    }

    fn get_ball(&self) -> usize {
        self.data[0x124] as usize
    }

    fn set_ball(&mut self, ball: usize) {
        self.data[0x124] = ball as u8;
    }

    fn get_met_level(&self) -> usize {
        (self.data[0x125] & !0x80) as usize
    }

    fn set_met_level(&mut self, met_level: usize) {
        self.data[0x125] = (self.data[0x125] & 0x80) | met_level as u8;
    }

    fn get_move_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0x72..0x74]).try_into().unwrap()) as usize
    }

    fn set_move_1(&mut self, move_1: usize) {
        let bytes = (move_1 as u16).to_le_bytes();
        self.data.splice(0x72..0x74, bytes);
    }

    fn get_move_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x74..0x76]).try_into().unwrap()) as usize
    }

    fn set_move_2(&mut self, move_2: usize) {
        let bytes = (move_2 as u16).to_le_bytes();
        self.data.splice(0x74..0x76, bytes);
    }

    fn get_move_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x76..0x78]).try_into().unwrap()) as usize
    }

    fn set_move_3(&mut self, move_3: usize) {
        let bytes = (move_3 as u16).to_le_bytes();
        self.data.splice(0x76..0x78, bytes);
    }

    fn get_move_4(&self) -> usize {
        u16::from_le_bytes((&self.data[0x78..0x7A]).try_into().unwrap()) as usize
    }

    fn set_move_4(&mut self, move_4: usize) {
        let bytes = (move_4 as u16).to_le_bytes();
        self.data.splice(0x78..0x7A, bytes);
    }

    fn get_move_1_pp(&self) -> usize {
        self.data[0x7A] as usize
    }

    fn set_move_1_pp(&mut self, move_1_pp: usize) {
        self.data[0x7A] = move_1_pp as u8;
    }

    fn get_move_2_pp(&self) -> usize {
        self.data[0x7B] as usize
    }

    fn set_move_2_pp(&mut self, move_2_pp: usize) {
        self.data[0x7B] = move_2_pp as u8;
    }

    fn get_move_3_pp(&self) -> usize {
        self.data[0x7C] as usize
    }

    fn set_move_3_pp(&mut self, move_3_pp: usize) {
        self.data[0x7C] = move_3_pp as u8;
    }

    fn get_move_4_pp(&self) -> usize {
        self.data[0x7D] as usize
    }

    fn set_move_4_pp(&mut self, move_4_pp: usize) {
        self.data[0x7D] = move_4_pp as u8;
    }

    fn get_move_1_pp_ups(&self) -> usize {
        self.data[0x7E] as usize
    }

    fn set_move_1_pp_ups(&mut self, move_1_pp_ups: usize) {
        self.data[0x7E] = move_1_pp_ups as u8;
    }

    fn get_move_2_pp_ups(&self) -> usize {
        self.data[0x7F] as usize
    }

    fn set_move_2_pp_ups(&mut self, move_2_pp_ups: usize) {
        self.data[0x7F] = move_2_pp_ups as u8;
    }

    fn get_move_3_pp_ups(&self) -> usize {
        self.data[0x80] as usize
    }

    fn set_move_3_pp_ups(&mut self, move_3_pp_ups: usize) {
        self.data[0x80] = move_3_pp_ups as u8;
    }

    fn get_move_4_pp_ups(&self) -> usize {
        self.data[0x81] as usize
    }

    fn set_move_4_pp_ups(&mut self, move_4_pp_ups: usize) {
        self.data[0x81] = move_4_pp_ups as u8;
    }

    fn get_ev_hp(&self) -> usize {
        self.data[0x26] as usize
    }

    fn set_ev_hp(&mut self, hp: usize) {
        self.data[0x26] = hp as u8;
    }

    fn get_ev_atk(&self) -> usize {
        self.data[0x27] as usize
    }

    fn set_ev_atk(&mut self, atk: usize) {
        self.data[0x27] = atk as u8;
    }

    fn get_ev_def(&self) -> usize {
        self.data[0x28] as usize
    }

    fn set_ev_def(&mut self, def: usize) {
        self.data[0x28] = def as u8;
    }

    fn get_ev_spe(&self) -> usize {
        self.data[0x29] as usize
    }

    fn set_ev_spe(&mut self, spe: usize) {
        self.data[0x29] = spe as u8;
    }

    fn get_ev_spa(&self) -> usize {
        self.data[0x2A] as usize
    }

    fn set_ev_spa(&mut self, spa: usize) {
        self.data[0x2A] = spa as u8;
    }

    fn get_ev_spd(&self) -> usize {
        self.data[0x2B] as usize
    }

    fn set_ev_spd(&mut self, spd: usize) {
        self.data[0x2B] = spd as u8;
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
        u32::from_le_bytes((&self.data[0x94..0x98]).try_into().unwrap()) as usize
    }

    fn set_status_condition(&mut self, status_condition: usize) {
        let bytes = (status_condition as u32).to_le_bytes();
        self.data.splice(0x94..0x98, bytes);
    }

    fn get_stat_level(&self) -> usize {
        self.data[0x148] as usize
    }

    fn set_stat_level(&mut self, level: usize) {
        self.data[0x148] = level as u8;
    }

    fn get_stat_hp_max(&self) -> usize {
        u16::from_le_bytes((&self.data[0x14A..0x14C]).try_into().unwrap()) as usize
    }

    fn set_stat_hp_max(&mut self, hp: usize) {
        let bytes = (hp as u16).to_le_bytes();
        self.data.splice(0x14A..0x14C, bytes);
    }

    fn get_stat_hp_current(&self) -> usize {
        u16::from_le_bytes((&self.data[0x8A..0x8C]).try_into().unwrap()) as usize
    }

    fn set_stat_hp_current(&mut self, hp: usize) {
        let bytes = (hp as u16).to_le_bytes();
        self.data.splice(0x8A..0x8C, bytes);
    }

    fn get_stat_atk(&self) -> usize {
        u16::from_le_bytes((&self.data[0x14C..0x14E]).try_into().unwrap()) as usize
    }

    fn set_stat_atk(&mut self, atk: usize) {
        let bytes = (atk as u16).to_le_bytes();
        self.data.splice(0x14C..0x14E, bytes);
    }

    fn get_stat_def(&self) -> usize {
        u16::from_le_bytes((&self.data[0x14E..0x150]).try_into().unwrap()) as usize
    }

    fn set_stat_def(&mut self, def: usize) {
        let bytes = (def as u16).to_le_bytes();
        self.data.splice(0x14E..0x150, bytes);
    }

    fn get_stat_spe(&self) -> usize {
        u16::from_le_bytes((&self.data[0x150..0x152]).try_into().unwrap()) as usize
    }

    fn set_stat_spe(&mut self, spe: usize) {
        let bytes = (spe as u16).to_le_bytes();
        self.data.splice(0x150..0x152, bytes);
    }

    fn get_stat_spa(&self) -> usize {
        u16::from_le_bytes((&self.data[0x152..0x154]).try_into().unwrap()) as usize
    }

    fn set_stat_spa(&mut self, spa: usize) {
        let bytes = (spa as u16).to_le_bytes();
        self.data.splice(0x152..0x154, bytes);
    }

    fn get_stat_spd(&self) -> usize {
        u16::from_le_bytes((&self.data[0x154..0x156]).try_into().unwrap()) as usize
    }

    fn set_stat_spd(&mut self, spd: usize) {
        let bytes = (spd as u16).to_le_bytes();
        self.data.splice(0x154..0x156, bytes);
    }

    fn get_version(&self) -> usize {
        self.data[0xDE] as usize
    }

    fn set_version(&mut self, version: usize) {
        self.data[0xDE] = version as u8;
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
        self.set_pkrs((self.get_pkrs() & !0xF) | days as u8);
    }

    fn get_encryption_constant(&self) -> usize {
        u32::from_le_bytes((&self.data[0x0..0x4]).try_into().unwrap()) as usize
    }

    fn set_encryption_constant(&mut self, ec: usize) {
        let bytes = (ec as u32).to_le_bytes();
        self.data.splice(0x0..0x4, bytes);
    }

    fn get_pid(&self) -> usize {
        u32::from_le_bytes((&self.data[0x1C..0x20]).try_into().unwrap()) as usize
    }

    fn set_pid(&mut self, pid: usize) {
        let bytes = (pid as u32).to_le_bytes();
        self.data.splice(0x1C..0x20, bytes);
    }

    fn get_fateful_encounter(&self) -> bool {
        (self.data[0x22] & 1) == 1
    }

    fn set_fateful_encounter(&mut self, fe: bool) {
        self.data[0x22] = (self.data[0x22] & !1) | if fe { 1 } else { 0 };
    }

    fn get_tsv(&self) -> usize {
        (self.get_tid() ^ self.get_sid()) >> 4
    }

    fn set_tsv(&mut self, _tsv: usize) {}

    fn get_psv(&self) -> usize {
        ((self.get_pid() >> 16) ^ (self.get_pid() & 0xFFFF)) >> 4
    }

    fn set_psv(&mut self, _set_psv: usize) {}

    fn get_characteristic(&self) -> usize {
        let pm6 = self.get_encryption_constant() % 6;
        let max_iv = self.get_max_iv();
        let mut pm6_stat = 0;
        for i in 0..6 {
            pm6_stat = (pm6 + i) % 6;
            if self.get_iv(pm6_stat) == max_iv {
                break;
            }
        }
        (pm6_stat * 5) + (max_iv % 5)
    }

    fn set_characteristic(&mut self, _characteristic: usize) {}

    fn get_mark_value(&self) -> usize {
        u16::from_le_bytes((&self.data[0x18..0x1A]).try_into().unwrap()) as usize
    }

    fn set_mark_value(&mut self, value: usize) {
        let bytes = (value as u16).to_le_bytes();
        self.data.splice(0x18..0x1A, bytes);
    }

    fn get_met_location(&self) -> usize {
        u16::from_le_bytes((&self.data[0x122..0x124]).try_into().unwrap()) as usize
    }

    fn set_met_location(&mut self, location: usize) {
        let bytes = (location as u16).to_le_bytes();
        self.data.splice(0x122..0x124, bytes);
    }

    fn get_egg_location(&self) -> usize {
        u16::from_le_bytes((&self.data[0x120..0x122]).try_into().unwrap()) as usize
    }

    fn set_egg_location(&mut self, location: usize) {
        let bytes = (location as u16).to_le_bytes();
        self.data.splice(0x120..0x122, bytes);
    }

    fn get_ot_friendship(&self) -> usize {
        self.data[0x112] as usize
    }

    fn set_ot_friendship(&mut self, friendship: usize) {
        self.data[0x112] = friendship as u8;
    }

    fn get_met_year(&self) -> usize {
        self.data[0x11C] as usize
    }

    fn set_met_year(&mut self, year: usize) {
        self.data[0x11C] = year as u8;
    }

    fn get_met_month(&self) -> usize {
        self.data[0x11D] as usize
    }

    fn set_met_month(&mut self, month: usize) {
        self.data[0x11D] = month as u8;
    }

    fn get_met_day(&self) -> usize {
        self.data[0x11E] as usize
    }

    fn set_met_day(&mut self, day: usize) {
        self.data[0x11E] = day as u8;
    }

    fn get_ht_name(&self) -> String {
        string_converter_8::get_string(self.ht_trash())
    }

    fn set_ht_name(&mut self, name: String) {
        let mut trash = self.ht_trash();
        string_converter_8::set_string(
            &mut trash,
            name.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0xA8..(0xA8 + 26), trash);
    }

    fn get_ht_gender(&self) -> usize {
        self.data[0xC2] as usize
    }

    fn set_ht_gender(&mut self, gender: usize) {
        self.data[0xC2] = gender as u8;
    }

    fn get_ht_friendship(&self) -> usize {
        self.data[0xC8] as usize
    }

    fn set_ht_friendship(&mut self, friendship: usize) {
        self.data[0xC8] = friendship as u8;
    }

    fn get_enjoyment(&self) -> u8 {
        self.data[0xDE]
    }

    fn set_enjoyment(&mut self, enjoyment: u8) {
        self.data[0xDE] = enjoyment;
    }

    fn get_fullness(&self) -> u8 {
        self.data[0xDC]
    }

    fn set_fullness(&mut self, fullness: u8) {
        self.data[0xDC] = fullness;
    }

    fn get_ability_number(&self) -> usize {
        (self.data[0x16] & 7) as usize
    }

    fn set_ability_number(&mut self, ability_number: usize) {
        self.data[0x16] = (self.data[0x16] & !7) | ((ability_number as u8) & 7);
    }

    fn get_egg_year(&self) -> usize {
        self.data[0x119] as usize
    }

    fn set_egg_year(&mut self, year: usize) {
        self.data[0x119] = year as u8;
    }

    fn get_egg_month(&self) -> usize {
        self.data[0x11A] as usize
    }

    fn set_egg_month(&mut self, month: usize) {
        self.data[0x11A] = month as u8;
    }

    fn get_egg_day(&self) -> usize {
        self.data[0x11B] as usize
    }

    fn set_egg_day(&mut self, day: usize) {
        self.data[0x11B] = day as u8;
    }

    fn get_relearn_move_1(&self) -> usize {
        u16::from_le_bytes((&self.data[0x82..0x84]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_1(&mut self, move_1: usize) {
        let bytes = (move_1 as u16).to_le_bytes();
        self.data.splice(0x82..0x84, bytes);
    }

    fn get_relearn_move_2(&self) -> usize {
        u16::from_le_bytes((&self.data[0x84..0x86]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_2(&mut self, move_2: usize) {
        let bytes = (move_2 as u16).to_le_bytes();
        self.data.splice(0x84..0x86, bytes);
    }

    fn get_relearn_move_3(&self) -> usize {
        u16::from_le_bytes((&self.data[0x86..0x88]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_3(&mut self, move_3: usize) {
        let bytes = (move_3 as u16).to_le_bytes();
        self.data.splice(0x86..0x88, bytes);
    }

    fn get_relearn_move_4(&self) -> usize {
        u16::from_le_bytes((&self.data[0x88..0x8A]).try_into().unwrap()) as usize
    }

    fn set_relearn_move_4(&mut self, move_4: usize) {
        let bytes = (move_4 as u16).to_le_bytes();
        self.data.splice(0x88..0x8A, bytes);
    }

    fn get_current_handler(&self) -> usize {
        self.data[0xC4] as usize
    }

    fn set_current_handler(&mut self, handler: usize) {
        self.data[0xC4] = handler as u8;
    }

    fn get_markings(&self) -> Vec<usize> {
        let mut marks = vec![0; 8];
        let val = self.get_mark_value();
        for (i, mark) in marks.iter_mut().enumerate() {
            *mark = ((val >> (i * 2)) & 3) % 3;
        }
        marks
    }

    fn is_untraded(&self) -> bool {
        self.data[0xA8] == 0 && self.data[0xA9] == 0 && self.format() == self.generation()
    }

    fn refresh_checksum(&mut self) {
        self.set_checksum(self.calculate_checksum());
    }

    fn checksum_valid(&self) -> bool {
        self.calculate_checksum() == self.get_checksum()
    }

    fn set_markings(&mut self, markings: &[usize]) {
        if markings.len() > 8 {
            return;
        }

        let mut v = 0;
        for (i, marking) in markings.iter().enumerate() {
            v |= (marking % 3) << (i * 2);
        }
        self.set_mark_value(v);
    }
}

impl SpeciesForm for PK8 {
    fn get_species(&self) -> usize {
        u16::from_le_bytes((&self.data[0x8..0xA]).try_into().unwrap()) as usize
    }

    fn set_species(&mut self, species: usize) {
        let bytes = (species as u16).to_le_bytes();
        self.data.splice(0x8..0xA, bytes);
    }

    fn get_form(&self) -> usize {
        u16::from_le_bytes((&self.data[0x24..0x26]).try_into().unwrap()) as usize
    }

    fn set_form(&mut self, form: usize) {
        let bytes = (form as u16).to_le_bytes();
        self.data.splice(0x24..0x26, bytes);
    }
}

impl TrainerId for PK8 {
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

impl Generation for PK8 {
    fn get_generation(&self) -> usize {
        todo!()
    }
}

impl Shiny for PK8 {
    fn get_is_shiny(&self) -> bool {
        todo!()
    }
}

impl LangNick for PK8 {
    fn get_nickname(&self) -> String {
        string_converter_8::get_string(self.nickname_trash())
    }

    fn set_nickname(&mut self, nickname: String) {
        let mut trash = self.nickname_trash();
        string_converter_8::set_string(
            &mut trash,
            nickname.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::None,
        );
        self.data.splice(0x58..(0x58 + 26), trash);
    }

    fn get_is_nicknamed(&self) -> bool {
        ((self.get_iv32() >> 31) & 1) == 1
    }

    fn set_is_nicknamed(&mut self, nicknamed: bool) {
        self.set_iv32((self.get_iv32() & !0x7FFFFFFF) | if nicknamed { 0x80000000 } else { 0 });
    }

    fn get_language(&self) -> usize {
        self.data[0xE2] as usize
    }

    fn set_language(&mut self, language: usize) {
        self.data[0xE2] = language as u8;
    }
}

impl GameValueLimit for PK8 {
    fn get_max_move_id(&self) -> usize {
        MAX_MOVE_ID_8
    }

    fn get_max_species_id(&self) -> usize {
        MAX_SPECIES_ID_8
    }

    fn get_max_item_id(&self) -> usize {
        MAX_ITEM_ID_8
    }

    fn get_max_ability_id(&self) -> usize {
        MAX_ABILITY_ID_8
    }

    fn get_max_ball_id(&self) -> usize {
        MAX_BALL_ID_8
    }

    fn get_max_game_id(&self) -> usize {
        MAX_GAME_ID_8
    }

    fn get_min_game_id(&self) -> usize {
        0
    }

    fn get_max_iv(&self) -> usize {
        31
    }

    fn get_max_ev(&self) -> usize {
        252
    }

    fn get_ot_length(&self) -> usize {
        12
    }

    fn get_nick_length(&self) -> usize {
        12
    }
}

impl NatureT for PK8 {
    fn get_nature(&self) -> usize {
        self.data[0x20] as usize
    }

    fn set_nature(&mut self, nature: usize) {
        self.data[0x20] = nature as u8;
    }
}

impl SanityChecksum for PK8 {
    fn get_sanity(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x4..0x6]).try_into().unwrap())
    }

    fn set_sanity(&mut self, sanity: u16) {
        let bytes = (sanity as u16).to_le_bytes();
        self.data.splice(0x4..0x6, bytes);
    }

    fn get_checksum(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x6..0x8]).try_into().unwrap())
    }

    fn set_checksum(&mut self, checksum: u16) {
        let bytes = (checksum as u16).to_le_bytes();
        self.data.splice(0x6..0x8, bytes);
    }
}

impl RibbonSetEvent3 for PK8 {
    fn get_ribbon_earth(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 0)
    }

    fn set_ribbon_earth(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 0, flag);
    }

    fn get_ribbon_national(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 7)
    }

    fn set_ribbon_national(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 7, flag);
    }

    fn get_ribbon_country(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 6)
    }

    fn set_ribbon_country(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 6, flag);
    }

    fn get_ribbon_champion_battle(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 1)
    }

    fn set_ribbon_champion_battle(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 1, flag);
    }

    fn get_ribbon_champion_regional(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 2)
    }

    fn set_ribbon_champion_regional(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 2, flag);
    }

    fn get_ribbon_champion_national(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 3)
    }

    fn set_ribbon_champion_national(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 3, flag);
    }
}

impl RibbonSetEvent4 for PK8 {
    fn get_ribbon_classic(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 2)
    }

    fn set_ribbon_classic(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 2, flag);
    }

    fn get_ribbon_wishing(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 0)
    }

    fn set_ribbon_wishing(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 0, flag);
    }

    fn get_ribbon_premier(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 3)
    }

    fn set_ribbon_premier(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 3, flag);
    }

    fn get_ribbon_event(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 4)
    }

    fn set_ribbon_event(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 4, flag);
    }

    fn get_ribbon_birthday(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 5)
    }

    fn set_ribbon_birthday(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 5, flag);
    }

    fn get_ribbon_special(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 6)
    }

    fn set_ribbon_special(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 6, flag);
    }

    fn get_ribbon_world(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 1)
    }

    fn set_ribbon_world(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 1, flag);
    }

    fn get_ribbon_champion_world(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 4)
    }

    fn set_ribbon_champion_world(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 4, flag);
    }

    fn get_ribbon_souvenir(&self) -> bool {
        flag_util::get_flag(&self.data, 0x37, 7)
    }

    fn set_ribbon_souvenir(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x37, 7, flag);
    }
}

impl RibbonSetCommon3 for PK8 {
    fn get_ribbon_champion_g3(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 1)
    }

    fn set_ribbon_champion_g3(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 1, flag);
    }

    fn get_ribbon_artist(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 2)
    }

    fn set_ribbon_artist(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 2, flag);
    }

    fn get_ribbon_effort(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 7)
    }

    fn set_ribbon_effort(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 7, flag);
    }
}

impl RibbonSetCommon4 for PK8 {
    fn get_ribbon_champion_sinnoh(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 2)
    }

    fn set_ribbon_champion_sinnoh(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 2, flag);
    }

    fn get_ribbon_alert(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 0)
    }

    fn set_ribbon_alert(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 0, flag);
    }

    fn get_ribbon_shock(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 1)
    }

    fn set_ribbon_shock(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 1, flag);
    }

    fn get_ribbon_downcast(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 2)
    }

    fn set_ribbon_downcast(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 2, flag);
    }

    fn get_ribbon_careless(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 3)
    }

    fn set_ribbon_careless(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 3, flag);
    }

    fn get_ribbon_relax(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 4)
    }

    fn set_ribbon_relax(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 4, flag);
    }

    fn get_ribbon_snooze(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 5)
    }

    fn set_ribbon_snooze(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 5, flag);
    }

    fn get_ribbon_smile(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 6)
    }

    fn set_ribbon_smile(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 6, flag);
    }

    fn get_ribbon_gorgeous(&self) -> bool {
        flag_util::get_flag(&self.data, 0x35, 7)
    }

    fn set_ribbon_gorgeous(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x35, 7, flag);
    }

    fn get_ribbon_royal(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 0)
    }

    fn set_ribbon_royal(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 0, flag);
    }

    fn get_ribbon_gorgeous_royal(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 1)
    }

    fn set_ribbon_gorgeous_royal(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 1, flag);
    }

    fn get_ribbon_footprint(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 3)
    }

    fn set_ribbon_footprint(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 3, flag);
    }

    fn get_ribbon_record(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 4)
    }

    fn set_ribbon_record(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 4, flag);
    }

    fn get_ribbon_legend(&self) -> bool {
        flag_util::get_flag(&self.data, 0x36, 5)
    }

    fn set_ribbon_legend(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x36, 5, flag);
    }
}

impl RibbonSetCommon6 for PK8 {
    fn get_ribbon_champion_kalos(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 0)
    }

    fn set_ribbon_champion_kalos(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 0, flag);
    }

    fn get_ribbon_champion_g6_hoenn(&self) -> bool {
        flag_util::get_flag(&self.data, 0x38, 7)
    }

    fn set_ribbon_champion_g6_hoenn(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x38, 7, flag);
    }

    fn get_ribbon_best_friends(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 3)
    }

    fn set_ribbon_best_friends(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 3, flag);
    }

    fn get_ribbon_training(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 4)
    }

    fn set_ribbon_training(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 4, flag);
    }

    fn get_ribbon_battler_skillful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 5)
    }

    fn set_ribbon_battler_skillful(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 5, flag);
    }

    fn get_ribbon_battler_expert(&self) -> bool {
        flag_util::get_flag(&self.data, 0x34, 6)
    }

    fn set_ribbon_battler_expert(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x34, 6, flag);
    }

    fn get_ribbon_contest_star(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 0)
    }

    fn set_ribbon_contest_star(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 0, flag);
    }

    fn get_ribbon_master_coolness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 1)
    }

    fn set_ribbon_master_coolness(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 1, flag);
    }

    fn get_ribbon_master_beauty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 2)
    }

    fn set_ribbon_master_beauty(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 2, flag);
    }

    fn get_ribbon_master_cuteness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 3)
    }

    fn set_ribbon_master_cuteness(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 3, flag);
    }

    fn get_ribbon_master_cleverness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 4)
    }

    fn set_ribbon_master_cleverness(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 4, flag);
    }

    fn get_ribbon_master_toughness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 5)
    }

    fn set_ribbon_master_toughness(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 5, flag);
    }

    fn get_ribbon_count_memory_contest(&self) -> usize {
        self.data[0x3C] as usize
    }

    fn set_ribbon_count_memory_contest(&mut self, value: usize) {
        self.data[0x3C] = value as u8;
        self.set_has_contest_memory_ribbon(value != 0);
    }

    fn get_ribbon_count_memory_battle(&self) -> usize {
        self.data[0x3D] as usize
    }

    fn set_ribbon_count_memory_battle(&mut self, value: usize) {
        self.data[0x3D] = value as u8;
        self.set_has_battle_memory_ribbon(value != 0);
    }
}

impl RibbonSetCommon8 for PK8 {
    fn get_ribbon_champion_galar(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 2)
    }

    fn set_ribbon_champion_galar(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 2, flag);
    }

    fn get_ribbon_tower_master(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 3)
    }

    fn set_ribbon_tower_master(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 3, flag);
    }

    fn get_ribbon_master_rank(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 4)
    }

    fn set_ribbon_master_rank(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 4, flag);
    }

    fn get_ribbon_twinkling_star(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 3)
    }

    fn set_ribbon_twinkling_star(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 3, flag);
    }

    fn get_ribbon_pioneer(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 2)
    }

    fn set_ribbon_pioneer(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 2, flag);
    }
}

impl RibbonSetCommon7 for PK8 {
    fn get_ribbon_champion_alola(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 6)
    }

    fn set_ribbon_champion_alola(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 6, flag);
    }

    fn get_ribbon_battle_royale(&self) -> bool {
        flag_util::get_flag(&self.data, 0x39, 7)
    }

    fn set_ribbon_battle_royale(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x39, 7, flag);
    }

    fn get_ribbon_battle_tree_great(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 0)
    }

    fn set_ribbon_battle_tree_great(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 0, flag);
    }

    fn get_ribbon_battle_tree_master(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 1)
    }

    fn set_ribbon_battle_tree_master(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 1, flag);
    }
}

impl RibbonSetMark8 for PK8 {
    fn get_ribbon_mark_lunchtime(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 5)
    }

    fn set_ribbon_mark_lunchtime(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 5, flag);
    }

    fn get_ribbon_mark_sleepytime(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 6)
    }

    fn set_ribbon_mark_sleepytime(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 6, flag);
    }

    fn get_ribbon_mark_dusk(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3A, 7)
    }

    fn set_ribbon_mark_dusk(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3A, 7, flag);
    }

    fn get_ribbon_mark_dawn(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 0)
    }

    fn set_ribbon_mark_dawn(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 0, flag);
    }

    fn get_ribbon_mark_cloudy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 1)
    }

    fn set_ribbon_mark_cloudy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 1, flag);
    }

    fn get_ribbon_mark_rainy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 2)
    }

    fn set_ribbon_mark_rainy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 2, flag);
    }

    fn get_ribbon_mark_stormy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 3)
    }

    fn set_ribbon_mark_stormy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 3, flag);
    }

    fn get_ribbon_mark_snowy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 4)
    }

    fn set_ribbon_mark_snowy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 4, flag);
    }

    fn get_ribbon_mark_blizzard(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 5)
    }

    fn set_ribbon_mark_blizzard(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 5, flag);
    }

    fn get_ribbon_mark_dry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 6)
    }

    fn set_ribbon_mark_dry(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 6, flag);
    }

    fn get_ribbon_mark_sandstorm(&self) -> bool {
        flag_util::get_flag(&self.data, 0x3B, 7)
    }

    fn set_ribbon_mark_sandstorm(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x3B, 7, flag);
    }

    fn get_ribbon_mark_misty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 0)
    }

    fn set_ribbon_mark_misty(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 0, flag);
    }

    fn get_ribbon_mark_destiny(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 1)
    }

    fn set_ribbon_mark_destiny(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 1, flag);
    }

    fn get_ribbon_mark_fishing(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 2)
    }

    fn set_ribbon_mark_fishing(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 2, flag);
    }

    fn get_ribbon_mark_curry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 3)
    }

    fn set_ribbon_mark_curry(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 3, flag);
    }

    fn get_ribbon_mark_uncommon(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 4)
    }

    fn set_ribbon_mark_uncommon(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 4, flag);
    }

    fn get_ribbon_mark_rare(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 5)
    }

    fn set_ribbon_mark_rare(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 5, flag);
    }

    fn get_ribbon_mark_rowdy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 6)
    }

    fn set_ribbon_mark_rowdy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 6, flag);
    }

    fn get_ribbon_mark_absent_minded(&self) -> bool {
        flag_util::get_flag(&self.data, 0x40, 7)
    }

    fn set_ribbon_mark_absent_minded(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x40, 7, flag);
    }

    fn get_ribbon_mark_jittery(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 0)
    }

    fn set_ribbon_mark_jittery(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 0, flag);
    }

    fn get_ribbon_mark_excited(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 1)
    }

    fn set_ribbon_mark_excited(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 1, flag);
    }

    fn get_ribbon_mark_charismatic(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 2)
    }

    fn set_ribbon_mark_charismatic(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 2, flag);
    }

    fn get_ribbon_mark_calmness(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 3)
    }

    fn set_ribbon_mark_calmness(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 3, flag);
    }

    fn get_ribbon_mark_intense(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 4)
    }

    fn set_ribbon_mark_intense(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 4, flag);
    }

    fn get_ribbon_mark_zoned_out(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 5)
    }

    fn set_ribbon_mark_zoned_out(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 5, flag);
    }

    fn get_ribbon_mark_joyful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 6)
    }

    fn set_ribbon_mark_joyful(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 6, flag);
    }

    fn get_ribbon_mark_angry(&self) -> bool {
        flag_util::get_flag(&self.data, 0x41, 7)
    }

    fn set_ribbon_mark_angry(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x41, 7, flag);
    }

    fn get_ribbon_mark_smiley(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 0)
    }

    fn set_ribbon_mark_smiley(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 0, flag);
    }

    fn get_ribbon_mark_teary(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 1)
    }

    fn set_ribbon_mark_teary(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 1, flag);
    }

    fn get_ribbon_mark_upbeat(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 2)
    }

    fn set_ribbon_mark_upbeat(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 2, flag);
    }

    fn get_ribbon_mark_peeved(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 3)
    }

    fn set_ribbon_mark_peeved(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 3, flag);
    }

    fn get_ribbon_mark_intellectual(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 4)
    }

    fn set_ribbon_mark_intellectual(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 4, flag);
    }

    fn get_ribbon_mark_ferocious(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 5)
    }

    fn set_ribbon_mark_ferocious(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 5, flag);
    }

    fn get_ribbon_mark_crafty(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 6)
    }

    fn set_ribbon_mark_crafty(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 6, flag);
    }

    fn get_ribbon_mark_scowling(&self) -> bool {
        flag_util::get_flag(&self.data, 0x42, 7)
    }

    fn set_ribbon_mark_scowling(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x42, 7, flag);
    }

    fn get_ribbon_mark_kindly(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 0)
    }

    fn set_ribbon_mark_kindly(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 0, flag);
    }

    fn get_ribbon_mark_flustered(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 1)
    }

    fn set_ribbon_mark_flustered(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 1, flag);
    }

    fn get_ribbon_mark_pumped_up(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 2)
    }

    fn set_ribbon_mark_pumped_up(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 2, flag);
    }

    fn get_ribbon_mark_zero_energy(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 3)
    }

    fn set_ribbon_mark_zero_energy(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 3, flag);
    }

    fn get_ribbon_mark_prideful(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 4)
    }

    fn set_ribbon_mark_prideful(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 4, flag);
    }

    fn get_ribbon_mark_unsure(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 5)
    }

    fn set_ribbon_mark_unsure(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 5, flag);
    }

    fn get_ribbon_mark_humble(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 6)
    }

    fn set_ribbon_mark_humble(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 6, flag);
    }

    fn get_ribbon_mark_thorny(&self) -> bool {
        flag_util::get_flag(&self.data, 0x43, 7)
    }

    fn set_ribbon_mark_thorny(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x43, 7, flag);
    }

    fn get_ribbon_mark_vigor(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 0)
    }

    fn set_ribbon_mark_vigor(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 0, flag);
    }

    fn get_ribbon_mark_slump(&self) -> bool {
        flag_util::get_flag(&self.data, 0x44, 1)
    }

    fn set_ribbon_mark_slump(&mut self, flag: bool) {
        flag_util::set_flag(&mut self.data, 0x44, 1, flag);
    }

    fn has_mark(&self) -> bool {
        let d = &self.data;
        if (u16::from_le_bytes(d[0x3A..0x3C].try_into().unwrap()) & 0xFFE0) != 0
            || u32::from_le_bytes(d[0x40..0x44].try_into().unwrap()) != 0
        {
            true
        } else {
            (d[0x44] & 3) != 0
        }
    }
}

impl RibbonSetAffixed for PK8 {
    fn get_affixed_ribbon(&self) -> i8 {
        self.data[0xE8] as i8
    }

    fn set_affixed_ribbon(&mut self, i_byte: i8) {
        self.data[0xE8] = i_byte as u8;
    }
}

impl TechRecord8 for PK8 {
    fn tech_record_permit_flags(&self) -> Vec<bool> {
        self.get_personal_info().get_tmhm()[..personal_info_swsh::TM_COUNT].to_vec()
    }

    fn tech_record_permit_indexes(&self) -> Vec<usize> {
        TMHM_SWSH[..personal_info_swsh::TM_COUNT].to_vec()
    }

    fn get_move_record_flag(&self, index: usize) -> Option<bool> {
        if index > 112 {
            None
        } else {
            let ofs = index >> 3;
            Some(flag_util::get_flag(&self.data, 0x127 + ofs, index & 7))
        }
    }

    fn set_move_record_flag(&mut self, index: usize, state: bool) {
        if index > 112 {
            return;
        }
        let ofs = index >> 3;
        flag_util::set_flag(&mut self.data, 0x127 + ofs, index & 7, state);
    }

    fn get_move_record_flag_any(&self) -> bool {
        self.data
            .iter()
            .skip(0x126)
            .take(14)
            .position(|z| *z != 0)
            .is_some()
    }
}

impl Sociability for PK8 {
    fn get_sociability(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x48..0x4C]).try_into().unwrap())
    }

    fn set_sociability(&mut self, sociability: u32) {
        let bytes = sociability.to_le_bytes();
        self.data.splice(0x48..0x4C, bytes);
    }
}

impl ContestStats for PK8 {
    fn get_cnt_cool(&self) -> u8 {
        self.data[0x2C]
    }

    fn get_cnt_beauty(&self) -> u8 {
        self.data[0x2D]
    }

    fn get_cnt_cute(&self) -> u8 {
        self.data[0x2E]
    }

    fn get_cnt_smart(&self) -> u8 {
        self.data[0x2F]
    }

    fn get_cnt_tough(&self) -> u8 {
        self.data[0x30]
    }

    fn get_cnt_sheen(&self) -> u8 {
        self.data[0x31]
    }
}

impl ContestStatsMutable for PK8 {
    fn set_cnt_cool(&mut self, cnt: u8) {
        self.data[0x2C] = cnt;
    }

    fn set_cnt_beauty(&mut self, cnt: u8) {
        self.data[0x2D] = cnt;
    }

    fn set_cnt_cute(&mut self, cnt: u8) {
        self.data[0x2E] = cnt;
    }

    fn set_cnt_smart(&mut self, cnt: u8) {
        self.data[0x2F] = cnt;
    }

    fn set_cnt_tough(&mut self, cnt: u8) {
        self.data[0x30] = cnt;
    }

    fn set_cnt_sheen(&mut self, cnt: u8) {
        self.data[0x31] = cnt;
    }
}

impl HyperTrain for PK8 {
    fn get_hyper_train_flags(&self) -> u8 {
        self.data[0x126]
    }

    fn set_hyper_train_flags(&mut self, flags: u8) {
        self.data[0x126] = flags;
    }

    fn get_ht_hp(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 0)
    }

    fn set_ht_hp(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 0, trained);
    }

    fn get_ht_atk(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 1)
    }

    fn set_ht_atk(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 1, trained);
    }

    fn get_ht_def(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 2)
    }

    fn set_ht_def(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 2, trained);
    }

    fn get_ht_spa(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 3)
    }

    fn set_ht_spa(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 3, trained);
    }

    fn get_ht_spd(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 4)
    }

    fn set_ht_spd(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 4, trained);
    }

    fn get_ht_spe(&self) -> bool {
        flag_util::get_flag(&self.data, 0x126, 5)
    }

    fn set_ht_spe(&mut self, trained: bool) {
        flag_util::set_flag(&mut self.data, 0x126, 5, trained);
    }
}

impl ScaledSize for PK8 {
    fn get_weight_scalar(&self) -> u8 {
        self.data[0x51]
    }

    fn set_weight_scalar(&mut self, weight: u8) {
        self.data[0x51] = weight;
    }

    fn get_height_scalar(&self) -> u8 {
        self.data[0x50]
    }

    fn set_height_scalar(&mut self, height: u8) {
        self.data[0x50] = height;
    }
}

impl Gigantamax for PK8 {
    fn get_can_gigantamax(&self) -> bool {
        (self.data[0x16] & 16) != 0
    }

    fn set_can_gigantamax(&mut self, can_gigantamax: bool) {
        self.data[0x16] = (self.data[0x16] & !16) | if can_gigantamax { 16 } else { 0 }
    }
}

impl Favorite for PK8 {
    fn get_favorite(&self) -> bool {
        (self.data[0x16] & 8) != 0
    }

    fn set_favorite(&mut self, is_favorite: bool) {
        self.data[0x16] = (self.data[0x16] & !8) | (if is_favorite { 1 } else { 0 } << 3);
    }
}

impl DynamaxLevel for PK8 {
    fn get_dynamax_level(&self) -> u8 {
        self.data[0x90]
    }

    fn set_dynamax_level(&mut self, dynamax_level: u8) {
        self.data[0x90] = dynamax_level;
    }
}

impl RibbonIndex for PK8 {
    fn get_ribbon(&self, index: usize) -> Option<bool> {
        if let Some(byte) = self.get_ribbon_byte(index) {
            Some(flag_util::get_flag(&self.data, byte, index & 7))
        } else {
            None
        }
    }

    fn set_ribbon(&mut self, index: usize, flag: bool) {
        if let Some(byte) = self.get_ribbon_byte(index) {
            flag_util::set_flag(&mut self.data, byte, index & 7, flag);
        }
    }

    fn get_ribbon_byte(&self, mut index: usize) -> Option<usize> {
        if index >= 128 {
            None
        } else if index < 64 {
            Some(0x34 + (index >> 3))
        } else {
            index -= 64;
            Some(0x40 + (index >> 3))
        }
    }
}

impl HandlerLanguage for PK8 {
    fn get_ht_language(&self) -> u8 {
        self.data[0xC3]
    }

    fn set_ht_language(&mut self, language: u8) {
        self.data[0xC3] = language
    }
}

impl FormArgument for PK8 {
    fn get_form_argument(&self) -> usize {
        u32::from_le_bytes((&self.data[0xE4..0xE8]).try_into().unwrap()) as usize
    }

    fn set_form_argument(&mut self, arg: usize) {
        let bytes = (arg as u32).to_le_bytes();
        self.data.splice(0xE4..0xE8, bytes);
    }

    fn get_form_argument_remain(&self) -> u8 {
        self.get_form_argument() as u8
    }

    fn set_form_argument_remain(&mut self, remain: u8) {
        self.set_form_argument((self.get_form_argument() & !0xFF) | remain as usize);
    }

    fn get_form_argument_elapsed(&self) -> u8 {
        (self.get_form_argument() >> 8) as u8
    }

    fn set_form_argument_elapsed(&mut self, elapsed: u8) {
        self.set_form_argument((self.get_form_argument() & !0xFF00) | ((elapsed as usize) << 8));
    }

    fn get_form_argument_maximum(&self) -> u8 {
        (self.get_form_argument() >> 16) as u8
    }

    fn set_form_argument_maximum(&mut self, maximum: u8) {
        self.set_form_argument((self.get_form_argument() & !0xFF00) | ((maximum as usize) << 16));
    }
}

impl HomeTrack for PK8 {
    fn get_tracker(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x135..0x13D]).try_into().unwrap())
    }

    fn set_tracker(&mut self, tracker: u64) {
        let bytes = tracker.to_le_bytes();
        self.data.splice(0x135..0x13D, bytes);
    }
}

impl BattleVersion for PK8 {
    fn get_battle_version(&self) -> u8 {
        self.data[0xDF]
    }

    fn set_battle_version(&mut self, battle_version: u8) {
        self.data[0xDF] = battle_version;
    }
}

impl TrainerMemories for PK8 {}

impl MemoryOT for PK8 {
    fn get_ot_memory(&self) -> u8 {
        self.data[0x114]
    }

    fn set_ot_memory(&mut self, memory: u8) {
        self.data[0x114] = memory;
    }

    fn get_ot_intensity(&self) -> u8 {
        self.data[0x113]
    }

    fn set_ot_intensity(&mut self, intensity: u8) {
        self.data[0x113] = intensity;
    }

    fn get_ot_feeling(&self) -> u8 {
        self.data[0x118]
    }

    fn set_ot_feeling(&mut self, feeling: u8) {
        self.data[0x118] = feeling;
    }

    fn get_ot_text_var(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x116..0x118]).try_into().unwrap())
    }

    fn set_ot_text_var(&mut self, var: u16) {
        let bytes = var.to_le_bytes();
        self.data.splice(0x116..0x118, bytes);
    }
}

impl MemoryHT for PK8 {
    fn get_ht_memory(&self) -> u8 {
        self.data[0xCA]
    }

    fn set_ht_memory(&mut self, memory: u8) {
        self.data[0xCA] = memory;
    }

    fn get_ht_intensity(&self) -> u8 {
        self.data[0xC9]
    }

    fn set_ht_intensity(&mut self, intensity: u8) {
        self.data[0xC9] = intensity;
    }

    fn get_ht_feeling(&self) -> u8 {
        self.data[0xCB]
    }

    fn set_ht_feeling(&mut self, feeling: u8) {
        self.data[0xCB] = feeling;
    }

    fn get_ht_text_var(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xCC..0xCE]).try_into().unwrap())
    }

    fn set_ht_text_var(&mut self, var: u16) {
        let bytes = var.to_le_bytes();
        self.data.splice(0xCC..0xCE, bytes);
    }
}

impl G8PKM<PersonalInfoSWSH> for PK8 {
    fn get_palma(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x98..0x9C]).try_into().unwrap())
    }

    fn set_palma(&mut self, palma: u32) {
        let bytes = palma.to_le_bytes();
        self.data.splice(0x98..0x9C, bytes);
    }

    fn get_unk_e3(&self) -> u8 {
        self.data[0xE3]
    }

    fn set_unk_e3(&mut self, e3: u8) {
        self.data[0xE3] = e3;
    }
}
