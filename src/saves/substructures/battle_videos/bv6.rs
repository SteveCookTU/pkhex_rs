use crate::{poke_crypto, string_converter_6, StringConverterOption, PK6};
use no_std_io::{EndianRead, EndianWrite};

const SIZE: usize = 0x2E60;
const NPC: &str = "NPC";
const PLAYER_COUNT: usize = 4;

#[derive(Copy, Clone, EndianRead, EndianWrite)]
pub struct BV6 {
    pub mode: u8,
    pub style: u8,
    #[no_std_io(pad_before = 4)]
    pub debug_1: [u8; 0x1A],
    #[no_std_io(pad_before = 0x30)]
    pub debug_2: [u8; 0x1A],
    #[no_std_io(pad_before = 0x82)]
    player_1_trash: [u8; 0x1A],
    player_2_trash: [u8; 0x1A],
    player_3_trash: [u8; 0x1A],
    player_4_trash: [u8; 0x1A],
    #[no_std_io(pad_before = 0x4C)]
    pub rng_const_1: u32,
    pub rng_const_2: u32,
    pub rng_seed_1: u64,
    pub rng_seed_2: u64,
    #[no_std_io(pad_before = 4)]
    pub background: u32,
    #[no_std_io(pad_before = 0xE)]
    pub unk_1_ce: u16,
    #[no_std_io(pad_before = 0x20)]
    pub intro_id: u16,
    #[no_std_io(pad_before = 8)]
    pub music_id: u16,
    #[no_std_io(pad_before = 0xC22)]
    player_1_p_1: [u8; poke_crypto::SIZE_6PARTY],
    player_1_p_2: [u8; poke_crypto::SIZE_6PARTY],
    player_1_p_3: [u8; poke_crypto::SIZE_6PARTY],
    player_1_p_4: [u8; poke_crypto::SIZE_6PARTY],
    player_1_p_5: [u8; poke_crypto::SIZE_6PARTY],
    player_1_p_6: [u8; poke_crypto::SIZE_6PARTY],
    #[no_std_io(pad_before = 8)]
    player_2_p_1: [u8; poke_crypto::SIZE_6PARTY],
    player_2_p_2: [u8; poke_crypto::SIZE_6PARTY],
    player_2_p_3: [u8; poke_crypto::SIZE_6PARTY],
    player_2_p_4: [u8; poke_crypto::SIZE_6PARTY],
    player_2_p_5: [u8; poke_crypto::SIZE_6PARTY],
    player_2_p_6: [u8; poke_crypto::SIZE_6PARTY],
    #[no_std_io(pad_before = 8)]
    player_3_p_1: [u8; poke_crypto::SIZE_6PARTY],
    player_3_p_2: [u8; poke_crypto::SIZE_6PARTY],
    player_3_p_3: [u8; poke_crypto::SIZE_6PARTY],
    player_3_p_4: [u8; poke_crypto::SIZE_6PARTY],
    player_3_p_5: [u8; poke_crypto::SIZE_6PARTY],
    player_3_p_6: [u8; poke_crypto::SIZE_6PARTY],
    #[no_std_io(pad_before = 8)]
    player_4_p_1: [u8; poke_crypto::SIZE_6PARTY],
    player_4_p_2: [u8; poke_crypto::SIZE_6PARTY],
    player_4_p_3: [u8; poke_crypto::SIZE_6PARTY],
    player_4_p_4: [u8; poke_crypto::SIZE_6PARTY],
    player_4_p_5: [u8; poke_crypto::SIZE_6PARTY],
    player_4_p_6: [u8; poke_crypto::SIZE_6PARTY],
    #[no_std_io(pad_before = 0x7C0)]
    pub match_year: u16,
    pub match_day: u8,
    pub match_month: u8,
    pub match_hour: u8,
    pub match_seconds: u8,
    pub match_flags: u8,
    pub upload_year: u16,
    pub upload_day: u8,
    pub upload_month: u8,
    pub upload_hour: u8,
    pub upload_minute: u8,
    pub upload_second: u8,
    pub upload_flags: u8,
}

impl Default for BV6 {
    fn default() -> Self {
        Self {
            mode: 0,
            style: 0,
            debug_1: [0; 0x1A],
            debug_2: [0; 0x1A],
            player_1_trash: [0; 0x1A],
            player_2_trash: [0; 0x1A],
            player_3_trash: [0; 0x1A],
            player_4_trash: [0; 0x1A],
            rng_const_1: 0,
            rng_const_2: 0,
            rng_seed_1: 0,
            rng_seed_2: 0,
            background: 0,
            unk_1_ce: 0,
            intro_id: 0,
            music_id: 0,
            player_1_p_1: [0; poke_crypto::SIZE_6PARTY],
            player_1_p_2: [0; poke_crypto::SIZE_6PARTY],
            player_1_p_3: [0; poke_crypto::SIZE_6PARTY],
            player_1_p_4: [0; poke_crypto::SIZE_6PARTY],
            player_1_p_5: [0; poke_crypto::SIZE_6PARTY],
            player_1_p_6: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_1: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_2: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_3: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_4: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_5: [0; poke_crypto::SIZE_6PARTY],
            player_2_p_6: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_1: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_2: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_3: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_4: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_5: [0; poke_crypto::SIZE_6PARTY],
            player_3_p_6: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_1: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_2: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_3: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_4: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_5: [0; poke_crypto::SIZE_6PARTY],
            player_4_p_6: [0; poke_crypto::SIZE_6PARTY],
            match_year: 0,
            match_day: 0,
            match_month: 0,
            match_hour: 0,
            match_seconds: 0,
            match_flags: 0,
            upload_year: 0,
            upload_day: 0,
            upload_month: 0,
            upload_hour: 0,
            upload_minute: 0,
            upload_second: 0,
            upload_flags: 0,
        }
    }
}

impl BV6 {
    pub fn get_player_names(&self) -> Vec<String> {
        let mut trainers = Vec::with_capacity(4);
        trainers.push(string_converter_6::get_string(&self.player_1_trash));
        trainers.push(string_converter_6::get_string(&self.player_2_trash));
        trainers.push(string_converter_6::get_string(&self.player_3_trash));
        trainers.push(string_converter_6::get_string(&self.player_4_trash));
        trainers
    }

    pub fn set_player_names(&mut self, value: [&str; 4]) {
        for (i, name) in value.iter().enumerate() {
            let to_set = if *name == NPC {
                "".chars().collect::<Vec<char>>()
            } else {
                name.chars().collect::<Vec<char>>()
            };

            match i {
                0 => string_converter_6::set_string(
                    &mut self.player_1_trash,
                    to_set,
                    12,
                    StringConverterOption::ClearZero,
                ),
                1 => string_converter_6::set_string(
                    &mut self.player_2_trash,
                    to_set,
                    12,
                    StringConverterOption::ClearZero,
                ),
                2 => string_converter_6::set_string(
                    &mut self.player_3_trash,
                    to_set,
                    12,
                    StringConverterOption::ClearZero,
                ),
                _ => string_converter_6::set_string(
                    &mut self.player_4_trash,
                    to_set,
                    12,
                    StringConverterOption::ClearZero,
                ),
            };
        }
    }

    pub fn get_player_teams(&self) -> [[PK6; 6]; 4] {
        [
            [
                self.player_1_p_1.to_vec().into(),
                self.player_1_p_2.to_vec().into(),
                self.player_1_p_3.to_vec().into(),
                self.player_1_p_4.to_vec().into(),
                self.player_1_p_5.to_vec().into(),
                self.player_1_p_6.to_vec().into(),
            ],
            [
                self.player_2_p_1.to_vec().into(),
                self.player_2_p_2.to_vec().into(),
                self.player_2_p_3.to_vec().into(),
                self.player_2_p_4.to_vec().into(),
                self.player_2_p_5.to_vec().into(),
                self.player_2_p_6.to_vec().into(),
            ],
            [
                self.player_3_p_1.to_vec().into(),
                self.player_3_p_2.to_vec().into(),
                self.player_3_p_3.to_vec().into(),
                self.player_3_p_4.to_vec().into(),
                self.player_3_p_5.to_vec().into(),
                self.player_3_p_6.to_vec().into(),
            ],
            [
                self.player_4_p_1.to_vec().into(),
                self.player_4_p_2.to_vec().into(),
                self.player_4_p_3.to_vec().into(),
                self.player_4_p_4.to_vec().into(),
                self.player_4_p_5.to_vec().into(),
                self.player_4_p_6.to_vec().into(),
            ],
        ]
    }

    pub fn set_player_teams(&mut self, mut value: [[PK6; 6]; 4]) {
        self.player_1_p_1 = (&value[0][0].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_1_p_2 = (&value[0][1].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_1_p_3 = (&value[0][2].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_1_p_4 = (&value[0][3].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_1_p_5 = (&value[0][4].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_1_p_6 = (&value[0][5].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();

        self.player_2_p_1 = (&value[1][0].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_2_p_2 = (&value[1][1].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_2_p_3 = (&value[1][2].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_2_p_4 = (&value[1][3].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_2_p_5 = (&value[1][4].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_2_p_6 = (&value[1][5].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();

        self.player_3_p_1 = (&value[2][0].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_3_p_2 = (&value[2][1].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_3_p_3 = (&value[2][2].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_3_p_4 = (&value[2][3].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_3_p_5 = (&value[2][4].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_3_p_6 = (&value[2][5].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();

        self.player_4_p_1 = (&value[0][0].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_4_p_2 = (&value[0][1].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_4_p_3 = (&value[0][2].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_4_p_4 = (&value[0][3].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_4_p_5 = (&value[0][4].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
        self.player_4_p_6 = (&value[0][5].encrypt())[..poke_crypto::SIZE_6PARTY]
            .try_into()
            .unwrap();
    }

    pub fn get_team(&self, index: usize) -> [PK6; 6] {
        match index {
            0 => [
                self.player_1_p_1.to_vec().into(),
                self.player_1_p_2.to_vec().into(),
                self.player_1_p_3.to_vec().into(),
                self.player_1_p_4.to_vec().into(),
                self.player_1_p_5.to_vec().into(),
                self.player_1_p_6.to_vec().into(),
            ],
            1 => [
                self.player_2_p_1.to_vec().into(),
                self.player_2_p_2.to_vec().into(),
                self.player_2_p_3.to_vec().into(),
                self.player_2_p_4.to_vec().into(),
                self.player_2_p_5.to_vec().into(),
                self.player_2_p_6.to_vec().into(),
            ],
            2 => [
                self.player_3_p_1.to_vec().into(),
                self.player_3_p_2.to_vec().into(),
                self.player_3_p_3.to_vec().into(),
                self.player_3_p_4.to_vec().into(),
                self.player_3_p_5.to_vec().into(),
                self.player_3_p_6.to_vec().into(),
            ],
            3 => [
                self.player_4_p_1.to_vec().into(),
                self.player_4_p_2.to_vec().into(),
                self.player_4_p_3.to_vec().into(),
                self.player_4_p_4.to_vec().into(),
                self.player_4_p_5.to_vec().into(),
                self.player_4_p_6.to_vec().into(),
            ],
            _ => [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        }
    }

    pub fn set_team(&mut self, index: usize, mut value: [PK6; 6]) {
        match index {
            0 => {
                self.player_1_p_1 = (&value[0].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_1_p_2 = (&value[1].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_1_p_3 = (&value[2].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_1_p_4 = (&value[3].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_1_p_5 = (&value[4].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_1_p_6 = (&value[5].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
            }
            1 => {
                self.player_2_p_1 = (&value[0].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_2_p_2 = (&value[1].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_2_p_3 = (&value[2].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_2_p_4 = (&value[3].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_2_p_5 = (&value[4].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_2_p_6 = (&value[5].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
            }
            2 => {
                self.player_3_p_1 = (&value[0].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_3_p_2 = (&value[1].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_3_p_3 = (&value[2].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_3_p_4 = (&value[3].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_3_p_5 = (&value[4].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_3_p_6 = (&value[5].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
            }
            3 => {
                self.player_4_p_1 = (&value[0].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_4_p_2 = (&value[1].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_4_p_3 = (&value[2].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_4_p_4 = (&value[3].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_4_p_5 = (&value[4].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
                self.player_4_p_6 = (&value[5].encrypt())[..poke_crypto::SIZE_6PARTY]
                    .try_into()
                    .unwrap();
            }
            _ => {}
        }
    }
}
