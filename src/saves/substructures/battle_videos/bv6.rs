use crate::personal_info_oras::PersonalInfoORAS;
use crate::{string_converter_6, BattleVideo, Pkm, PokeGroup, StringConverterOption, PK6};
use time::{Date, Month, PrimitiveDateTime, Time};
use crate::poke_crypto::SIZE_6PARTY;

const SIZE: usize = 0x2E60;
const NPC: &str = "NPC";
const PLAYER_COUNT: usize = 4;

pub struct BV6 {
    data: Vec<u8>,
}

impl BV6 {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn get_mode(&self) -> u8 {
        self.data[0x0]
    }

    pub fn set_mode(&mut self, mode: u8) {
        self.data[0x0] = mode;
    }

    pub fn get_style(&self) -> u8 {
        self.data[0x1]
    }

    pub fn set_style(&mut self, style: u8) {
        self.data[0x1] = style;
    }

    pub fn get_debug_1(&self) -> String {
        string_converter_6::get_string(self.data[0x6..0x20].to_vec())
    }

    pub fn set_debug_1(&mut self, debug_1: String) {
        let mut trash = self.data[0x6..0x20].to_vec();
        string_converter_6::set_string(
            &mut trash,
            debug_1.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::ClearZero,
        );
        self.data.splice(0x6..0x20, trash);
    }

    pub fn get_debug_2(&self) -> String {
        string_converter_6::get_string(self.data[0x50..0x6A].to_vec())
    }

    pub fn set_debug_2(&mut self, debug_2: String) {
        let mut trash = self.data[0x50..0x6A].to_vec();
        string_converter_6::set_string(
            &mut trash,
            debug_2.chars().collect::<Vec<char>>(),
            12,
            StringConverterOption::ClearZero,
        );
        self.data.splice(0x50..0x6A, trash);
    }

    pub fn get_rng_const_1(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1A0..0x1A4]).try_into().unwrap())
    }

    pub fn set_rng_const_1(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.data.splice(0x1A0..0x1A4, bytes);
    }

    pub fn get_rng_const_2(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1A4..0x1A8]).try_into().unwrap())
    }

    pub fn set_rng_const_2(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.data.splice(0x1A4..0x1A8, bytes);
    }

    pub fn get_rng_seed_1(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x1A8..0x1B0]).try_into().unwrap())
    }

    pub fn set_rng_seed_1(&mut self, val: u64) {
        let bytes = val.to_le_bytes();
        self.data.splice(0x1A8..0x1B0, bytes);
    }

    pub fn get_rng_seed_2(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x1B0..0x1B8]).try_into().unwrap())
    }

    pub fn set_rng_seed_2(&mut self, val: u64) {
        let bytes = val.to_le_bytes();
        self.data.splice(0x1B0..0x1B8, bytes);
    }

    pub fn get_background(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1BC..0x1C0]).try_into().unwrap())
    }

    pub fn set_background(&mut self, background: u32) {
        let bytes = background.to_le_bytes();
        self.data.splice(0x1BC..0x1C0, bytes);
    }

    pub fn get_unk_1_ce(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1CE..0x1D0]).try_into().unwrap())
    }

    pub fn set_unk_1_ce(&mut self, ce: u16) {
        let bytes = ce.to_le_bytes();
        self.data.splice(0x1CE..0x1D0, bytes);
    }

    pub fn get_intro_id(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1E4..0x1E6]).try_into().unwrap())
    }

    pub fn set_intro_id(&mut self, id: u16) {
        let bytes = id.to_le_bytes();
        self.data.splice(0x1E4..0x1E6, bytes);
    }

    pub fn get_music_id(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1F0..0x1F2]).try_into().unwrap())
    }

    pub fn set_music_id(&mut self, id: u16) {
        let bytes = id.to_le_bytes();
        self.data.splice(0x1F0..0x1F2, bytes);
    }

    pub fn get_player_names(&self) -> Vec<String> {
        let mut trainers = Vec::with_capacity(PLAYER_COUNT);
        for i in 0..PLAYER_COUNT {
            let trainer_name = string_converter_6::get_string(
                self.data[0xEC + (0x1A * i)..(0xEC + (0x1A * i) + 0x1A)].to_vec(),
            );
            if trainer_name.trim().is_empty() {
                trainers.push(NPC.to_string())
            } else {
                trainers.push(trainer_name);
            }
        }
        trainers
    }

    pub fn set_player_names(&mut self, names: Vec<String>) {
        if names.len() != PLAYER_COUNT {
            return;
        }

        for (i, name) in names.iter().enumerate() {
            let mut trash = self.data[0xEC + (0x1A * i)..(0xEC + (0x1A * i) + 0x1A)].to_vec();
            let tr = if *name == NPC.to_string() {
                "".to_string()
            } else {
                name.to_string()
            };
            string_converter_6::set_string(
                &mut trash,
                tr.chars().collect::<Vec<char>>(),
                12,
                StringConverterOption::ClearZero,
            );
            self.data
                .splice(0xEC + (0x1A * i)..(0xEC + (0x1A * i) + 0x1A), trash);
        }
    }

    pub fn get_player_teams(&self) -> Vec<Vec<PK6>> {
        let mut teams = Vec::with_capacity(PLAYER_COUNT);
        for i in 0..PLAYER_COUNT {
            teams.push(self.get_team(i));
        }
        teams
    }

    pub fn set_player_teams(&mut self, teams: Vec<Vec<PK6>>) {
        for t in 0..PLAYER_COUNT {
            self.set_team(teams[t].clone(), t);
        }
    }

    pub fn get_team(&self, t: usize) -> Vec<PK6> {
        let mut team = Vec::with_capacity(6);
        let start: usize = 0xE18;
        for p in 0..6 {
            let mut offset = start + (0x104 * ((t * 6) + p));
            offset += 8 * (((t * 6) + p) / 6);
            team.push(PK6::new(self.data[offset..(offset + 0x104)].to_vec()));
        }
        team
    }

    pub fn set_team(&mut self, mut team: Vec<PK6>, t: usize) {
        let start = 0xE18;
        for p in 0..6 {
            let mut offset = start + (SIZE_6PARTY * ((t * 6) + p));
            offset += 8 * (((t * 6) + p) / 6);
            self.data.splice(offset..(offset + SIZE_6PARTY), team[p].encrypted_party_data());
        }
    }

    pub fn get_match_year(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x2E50..0x2E52]).try_into().unwrap())
    }

    pub fn set_match_year(&mut self, year: u16) {
        let bytes = year.to_le_bytes();
        self.data.splice(0x2E50..0x2E52, bytes);
    }

    pub fn get_match_day(&self) -> u8 {
        self.data[0x2E52]
    }

    pub fn set_match_day(&mut self, day: u8) {
        self.data[0x2E52] = day;
    }

    pub fn get_match_month(&self) -> u8 {
        self.data[0x2E53]
    }

    pub fn set_match_month(&mut self, month: u8) {
        self.data[0x2E53] = month;
    }

    pub fn get_match_hour(&self) -> u8 {
        self.data[0x2E54]
    }

    pub fn set_match_hour(&mut self, hour: u8) {
        self.data[0x2E54] = hour;
    }

    pub fn get_match_min(&self) -> u8 {
        self.data[0x2E55]
    }

    pub fn set_match_min(&mut self, min: u8) {
        self.data[0x2E55] = min;
    }

    pub fn get_match_second(&self) -> u8 {
        self.data[0x2E56]
    }

    pub fn set_match_second(&mut self, second: u8) {
        self.data[0x2E56] = second;
    }

    pub fn get_match_flags(&self) -> u8 {
        self.data[0x2E57]
    }

    pub fn set_match_flags(&mut self, flags: u8) {
        self.data[0x2E57] = flags;
    }

    pub fn get_upload_year(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x2E58..0x2E5A]).try_into().unwrap())
    }

    pub fn set_upload_year(&mut self, year: u16) {
        let bytes = year.to_le_bytes();
        self.data.splice(0x2E58..0x2E5A, bytes);
    }

    pub fn get_upload_day(&self) -> u8 {
        self.data[0x2E5A]
    }

    pub fn set_upload_day(&mut self, day: u8) {
        self.data[0x2E5A] = day;
    }

    pub fn get_upload_month(&self) -> u8 {
        self.data[0x2E5B]
    }

    pub fn set_upload_month(&mut self, month: u8) {
        self.data[0x2E5B] = month;
    }

    pub fn get_upload_hour(&self) -> u8 {
        self.data[0x2E5C]
    }

    pub fn set_upload_hour(&mut self, hour: u8) {
        self.data[0x2E5C] = hour;
    }

    pub fn get_upload_min(&self) -> u8 {
        self.data[0x2E5D]
    }

    pub fn set_upload_min(&mut self, min: u8) {
        self.data[0x2E5D] = min;
    }

    pub fn get_upload_second(&self) -> u8 {
        self.data[0x2E5E]
    }

    pub fn set_upload_second(&mut self, second: u8) {
        self.data[0x2E5E] = second;
    }

    pub fn get_upload_flags(&self) -> u8 {
        self.data[0x2E5F]
    }

    pub fn set_upload_flags(&mut self, flags: u8) {
        self.data[0x2E5F] = flags;
    }

    pub fn get_match_stamp(&self) -> Option<PrimitiveDateTime> {
        if let Ok(month) = Month::try_from(self.get_match_month()) {
            if let Ok(date) =
                Date::from_calendar_date(self.get_match_year() as i32, month, self.get_match_day())
            {
                if let Ok(time) = Time::from_hms(
                    self.get_match_hour(),
                    self.get_match_min(),
                    self.get_match_second(),
                ) {
                    return Some(PrimitiveDateTime::new(date, time));
                }
            }
        }

        None
    }

    pub fn set_match_stamp(&mut self, datetime: Option<PrimitiveDateTime>) {
        if let Some(datetime) = datetime {
            self.set_match_year(datetime.year() as u16);
            self.set_match_day(datetime.day());
            self.set_match_month(datetime.month() as u8);
            self.set_match_hour(datetime.hour());
            self.set_match_min(datetime.minute());
            self.set_match_second(datetime.second());
        } else {
            self.set_match_year(0);
            self.set_match_day(0);
            self.set_match_month(0);
            self.set_match_hour(0);
            self.set_match_min(0);
            self.set_match_second(0);
            self.set_match_flags(0);
        }
    }

    pub fn get_upload_stamp(&self) -> Option<PrimitiveDateTime> {
        if let Ok(month) = Month::try_from(self.get_upload_month()) {
            if let Ok(date) = Date::from_calendar_date(
                self.get_upload_year() as i32,
                month,
                self.get_upload_day(),
            ) {
                if let Ok(time) = Time::from_hms(
                    self.get_upload_hour(),
                    self.get_upload_min(),
                    self.get_upload_second(),
                ) {
                    return Some(PrimitiveDateTime::new(date, time));
                }
            }
        }

        None
    }

    pub fn set_upload_stamp(&mut self, datetime: Option<PrimitiveDateTime>) {
        if let Some(datetime) = datetime {
            self.set_upload_year(datetime.year() as u16);
            self.set_upload_day(datetime.day());
            self.set_upload_month(datetime.month() as u8);
            self.set_upload_hour(datetime.hour());
            self.set_upload_min(datetime.minute());
            self.set_upload_second(datetime.second());
        } else {
            self.set_upload_year(0);
            self.set_upload_day(0);
            self.set_upload_month(0);
            self.set_upload_hour(0);
            self.set_upload_min(0);
            self.set_upload_second(0);
            self.set_upload_flags(0);
        }
    }
}

impl PokeGroup<PersonalInfoORAS, PK6> for BV6 {
    fn get_contents(&self) -> Vec<PK6> {
        BattleVideo::get_contents(self)
    }
}

impl BattleVideo<PersonalInfoORAS, PK6> for BV6 {
    fn get_battle_pkms(&self) -> Vec<PK6> {
        self.get_player_teams()
            .into_iter()
            .flatten()
            .collect::<Vec<PK6>>()
    }

    fn get_generation(&self) -> usize {
        6
    }
}

enum TurnAction {
    None = 0,
    Fight = 1,
    Unk2 = 2,
    Switch = 3,
    Run = 4,
    Unk5 = 5,
    Rotate = 6,
    Unk7 = 7,
    MegaEvolve = 8,
}

enum TurnTarget {
    U0 = 0,
    U1 = 1,
    U2 = 2,
    U3 = 3,
    U4 = 4,
    U5 = 5,
    U6 = 6,
    U7 = 7,
    U8 = 8,
    U9 = 9,
    OppositeEnemy,
    U11 = 11,
    U12 = 12,
    U13 = 13,
    AllExceptUser = 14,
    Everyone = 15,
}

enum TurnRotate {
    None,
    Right,
    Left,
    Unk3,
}

pub enum BVType {
    Link = 0,
    Maison = 1,
    SuperMaison = 2,
    BattleSpotFree = 3,
    BattleSpotRating = 4,
    BattleSpotSpecial = 5,
    UNUSED = 6,
    JP1 = 7,
    JP2 = 8,
    BROKEN = 9,
}

pub enum BVStyle {
    Single = 0,
    Double = 1,
    Triple = 2,
    Rotation = 3,
    Multi = 4,
}

pub fn is_valid(data: Vec<u8>) -> bool {
    if data.len() != SIZE {
        false
    } else {
        u64::from_le_bytes((&data[0xE18..0xE20]).try_into().unwrap()) != 0
            && u16::from_le_bytes((&data[0xE12..0xE14]).try_into().unwrap()) != 0
    }
}
