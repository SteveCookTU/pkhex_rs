use no_std_io::{EndianWrite, EndianRead, StreamContainer, StreamReader, StreamWriter, Cursor};
use crate::{poke_crypto, string_converter_8};

#[derive(Default, Copy, Clone, EndianRead, EndianWrite)]
pub struct PA8 {
    pub ec: u32,
    pub sanity: u16,
    pub checksum: u16,
    pub species: u16,
    pub held_item: u16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    pub ability: u16,
    ability_flags: u8,
    #[no_std_io(pad_before = 1)]
    pub mark_value: u16,
    #[no_std_io(pad_before = 2)]
    pub pid: u32,
    pub nature: u8,
    pub stat_nature: u8,
    encounter_flags: u8,
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
    pub alpha_move: u16,
    ribbon_2: u32,
    ribbon_3: u32,
    pub sociability: u32,
    #[no_std_io(pad_before = 4)]
    pub height_scalar: u8,
    pub weight_scalar: u8,
    pub height_scalar_copy: u8,
    #[no_std_io(pad_before = 1)]
    pub move_1: u16,
    pub move_2: u16,
    pub move_3: u16,
    pub move_4: u16,
    pub move_1_pp: u8,
    pub move_2_pp: u8,
    pub move_3_pp: u8,
    pub move_4_pp: u8,
    nickname_trash: [u8; 26],
    #[no_std_io(pad_before = 12)]
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
    pub unk_a0: i32,
    pub gv_hp: u8,
    pub gv_atk: u8,
    pub gv_def: u8,
    pub gv_spe: u8,
    pub gv_spa: u8,
    pub gv_spd: u8,
    #[no_std_io(pad_before = 2)]
    pub height_absolute: f32,
    pub weight_absolute: f32,
    #[no_std_io(pad_before = 4)]
    ht_trash: [u8; 26],
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
    #[no_std_io(pad_before = 14)]
    pub fullness: u8,
    pub enjoyment: u8,
    pub version: u8,
    pub battle_version: u8,
    #[no_std_io(pad_before = 2)]
    pub language: u8,
    pub unk_f3: u8,
    pub form_argument: u32,
    pub affixed_ribbon: i8,
    #[no_std_io(pad_before = 23)]
    ot_trash: [u8; 26],
    pub ot_friendship: u8,
    pub ot_intensity: u8,
    pub ot_memory: u8,
    #[no_std_io(pad_before = 1)]
    pub ot_text_var: u16,
    pub ot_feeling: u8,
    pub egg_year: u8,
    pub egg_month: u8,
    pub egg_day: u8,
    pub met_year: u8,
    pub met_month: u8,
    pub met_day: u8,
    pub ball: u8,
    pub egg_location: u16,
    pub met_location: u16,
    #[no_std_io(pad_before = 1)]
    met_flags: u8,
    pub hyper_train_flags: u8,
    pub move_record_flags: [u8; 14],
    pub tracker: u64,
    purchased_record_flags: [u8; 8],
    master_record_flags: [u8; 8],
    #[no_std_io(pad_before = 3)]
    pub stat_level: u8,
    #[no_std_io(pad_before = 1)]
    pub stat_hp_max: u16,
    pub stat_atk: u16,
    pub stat_def: u16,
    pub stat_spe: u16,
    pub stat_spa: u16,
    pub stat_spd: u16,
}

impl PA8 {
    pub fn new() -> Self {
        Self {
            affixed_ribbon: -1,
            ..Default::default()
        }
    }

    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0u16;
        let data: Vec<u8> = (*self).into();
        let mut reader = StreamContainer::new(data);
        reader.set_index(8);
        while reader.get_index() < poke_crypto::SIZE_8ASTORED {
            chk = chk.wrapping_add(reader.default_read_stream_le::<u16>())
        }
        chk
    }

    pub fn get_ot_name(&self) -> String {
        string_converter_8::get_string(&self.ot_trash)
    }

    pub fn get_nickname(&self) -> String {
        string_converter_8::get_string(&self.nickname_trash)
    }
}

impl From<Vec<u8>> for PA8 {
    fn from(mut data: Vec<u8>) -> Self {
        poke_crypto::decrypt_if_encrypted_8a(&mut data);
        data.resize(poke_crypto::SIZE_8APARTY, 0);
        let mut reader = StreamContainer::new(data);
        reader.default_read_stream_le::<PA8>()
    }
}

impl From<PA8> for Vec<u8> {
    fn from(pkm: PA8) -> Self {
        let data = vec![0u8; poke_crypto::SIZE_8APARTY];
        let mut writer = StreamContainer::new(data);
        writer.checked_write_stream_le(&pkm);
        writer.into_raw()
    }
}

#[cfg(test)]
mod tests {
    use crate::pkm::pa8::PA8;

    const PLA_PB8: &[u8] = include_bytes!("../resources/tests/pla.pa8");

    #[test]
    fn should_read() {
        let bytes = PLA_PB8.to_vec();
        let pkm: PA8 = bytes.into();
        assert_eq!(pkm.pid, 0xCAF42611);
        assert_eq!(pkm.exp, 156250);
        assert_eq!(pkm.height_scalar, 127);
        assert_eq!(pkm.move_1_pp_ups, 1);
        assert_eq!(pkm.get_nickname(), "Snover".to_string());
        assert_eq!(pkm.get_ot_name(), "PKHeX".to_string());
    }

    #[test]
    fn should_read_and_write() {
        let bytes = PLA_PB8.to_vec();
        let pkm: PA8 = bytes.clone().into();
        let output: Vec<u8> = pkm.into();
        assert_eq!(bytes, output.to_vec())
    }

    #[test]
    fn should_calc_checksum() {
        let bytes = PLA_PB8.to_vec();
        let pkm: PA8 = bytes.into();
        assert_eq!(pkm.checksum, pkm.calculate_checksum());
    }
}