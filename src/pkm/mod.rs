mod ck3;
mod pa8;
mod pb8;
mod pk3;
mod pk5;
mod pk6;
mod pk7;
mod pk8;
pub mod ribbons;
mod shared;
mod strings;
pub mod traits;
mod util;

pub use ck3::*;
pub use pa8::*;
pub use pb8::*;
pub use pk3::*;
pub use pk5::*;
pub use pk6::*;
pub use pk7::*;
pub use pk8::*;
pub use shared::*;
pub use strings::*;
pub use util::*;

use crate::editing::{hidden_power, nature_amp};
use crate::structures::{shiny, IndividualValueSet, Moveset};
use crate::traits::hyper_train::HyperTrain;
use crate::traits::trainer_id::TrainerId;
use crate::traits::{Alpha, GameValueLimit, LangNick, Nature, Shiny, SpeciesForm};
use crate::{
    date_util, experience, locations, moves, personal_info, rand_util, tables, GameVersion,
    LanguageID, PersonalInfo, Species,
};
use rand::{thread_rng, Rng};
use time::{Date, Month, PrimitiveDateTime, Time};

pub trait PKM<Personal: PersonalInfo + 'static>:
    SpeciesForm + TrainerId + Shiny + LangNick + GameValueLimit + Nature + Into<Vec<u8>> + Clone + Copy
{
    fn extensions() -> Vec<String> {
        entity_file_extension::get_extensions(pkx::GENERATION)
    }
    fn size_party(&self) -> usize;
    fn size_stored(&self) -> usize;
    fn extension(&self) -> String;
    fn personal_info(&self) -> &'static Personal;
    fn extra_bytes(&self) -> Vec<u16> {
        vec![]
    }

    fn encrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()[0..self.size_party()].to_vec()
    }
    fn encrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()[0..self.size_stored()].to_vec()
    }
    fn decrypted_party_data(&mut self) -> Vec<u8> {
        self.write()[0..self.size_party()].to_vec()
    }
    fn decrypted_box_data(&mut self) -> Vec<u8> {
        self.write()[0..self.size_stored()].to_vec()
    }

    fn get_valid(&self) -> bool;

    fn set_valid(&mut self, valid: bool);

    fn nickname_trash(&self) -> &[u8];
    fn ot_trash(&self) -> &[u8];
    fn ht_trash(&self) -> &[u8] {
        &[]
    }

    fn encrypt(&mut self) -> Vec<u8>;
    fn context(&self) -> EntityContext;
    fn format(&self) -> u8 {
        self.context().generation()
    }

    fn write(&mut self) -> Vec<u8> {
        self.refresh_checksum();
        (*self).into()
    }

    fn set_species(&mut self, species: u16);

    fn set_nickname(&mut self, nickname: &str);

    fn get_held_item(&self) -> u16;
    fn set_held_item(&mut self, item: u16);

    fn get_gender(&self) -> u8;
    fn set_gender(&mut self, gender: u8);

    fn get_stat_nature(&self) -> u8 {
        self.get_nature()
    }
    fn set_stat_nature(&mut self, nature: u8) {
        self.set_nature(nature);
    }

    fn get_ability(&self) -> u16;
    fn set_ability(&mut self, ability: u16);

    fn get_current_friendship(&self) -> u8;
    fn set_current_friendship(&mut self, friendship: u8);

    fn set_form(&mut self, form: u8);

    fn get_is_egg(&self) -> bool;
    fn set_is_egg(&mut self, is_egg: bool);

    fn set_is_nicknamed(&mut self, is_nicknamed: bool);

    fn get_exp(&self) -> u32;
    fn set_exp(&mut self, exp: u32);

    fn get_ot_name(&self) -> String;
    fn set_ot_name(&mut self, name: &str);

    fn get_ot_gender(&self) -> u8;
    fn set_ot_gender(&mut self, gender: u8);

    fn get_ball(&self) -> u8;
    fn set_ball(&mut self, ball: u8);

    fn get_met_level(&self) -> u8;
    fn set_met_level(&mut self, met_level: u8);

    fn get_move_1(&self) -> u16;
    fn set_move_1(&mut self, move_1: u16);
    fn get_move_2(&self) -> u16;
    fn set_move_2(&mut self, move_2: u16);
    fn get_move_3(&self) -> u16;
    fn set_move_3(&mut self, move_3: u16);
    fn get_move_4(&self) -> u16;
    fn set_move_4(&mut self, move_4: u16);

    fn get_move_1_pp(&self) -> u8;
    fn set_move_1_pp(&mut self, pp: u8);
    fn get_move_2_pp(&self) -> u8;
    fn set_move_2_pp(&mut self, pp: u8);
    fn get_move_3_pp(&self) -> u8;
    fn set_move_3_pp(&mut self, pp: u8);
    fn get_move_4_pp(&self) -> u8;
    fn set_move_4_pp(&mut self, pp: u8);

    fn get_move_1_pp_ups(&self) -> u8;
    fn set_move_1_pp_ups(&mut self, pp_ups: u8);
    fn get_move_2_pp_ups(&self) -> u8;
    fn set_move_2_pp_ups(&mut self, pp_ups: u8);
    fn get_move_3_pp_ups(&self) -> u8;
    fn set_move_3_pp_ups(&mut self, pp_ups: u8);
    fn get_move_4_pp_ups(&self) -> u8;
    fn set_move_4_pp_ups(&mut self, pp_ups: u8);

    fn get_ev_hp(&self) -> u8;
    fn set_ev_hp(&mut self, ev_hp: u8);
    fn get_ev_atk(&self) -> u8;
    fn set_ev_atk(&mut self, ev_atk: u8);
    fn get_ev_def(&self) -> u8;
    fn set_ev_def(&mut self, ev_def: u8);
    fn get_ev_spe(&self) -> u8;
    fn set_ev_spe(&mut self, ev_spe: u8);
    fn get_ev_spa(&self) -> u8;
    fn set_ev_spa(&mut self, ev_spa: u8);
    fn get_ev_spd(&self) -> u8;
    fn set_ev_spd(&mut self, ev_spd: u8);

    fn get_iv_hp(&self) -> u8;
    fn set_iv_hp(&mut self, iv_hp: u8);
    fn get_iv_atk(&self) -> u8;
    fn set_iv_atk(&mut self, iv_atk: u8);
    fn get_iv_def(&self) -> u8;
    fn set_iv_def(&mut self, iv_def: u8);
    fn get_iv_spe(&self) -> u8;
    fn set_iv_spe(&mut self, iv_spe: u8);
    fn get_iv_spa(&self) -> u8;
    fn set_iv_spa(&mut self, iv_spa: u8);
    fn get_iv_spd(&self) -> u8;
    fn set_iv_spd(&mut self, iv_spd: u8);

    fn get_status_condition(&self) -> u32;
    fn set_status_condition(&mut self, condition: u32);

    fn get_stat_level(&self) -> u8;
    fn set_stat_level(&mut self, stat_level: u8);
    fn get_stat_hp_max(&self) -> u16;
    fn set_stat_hp_max(&mut self, stat_hp_max: u16);
    fn get_stat_hp_current(&self) -> u16;
    fn set_stat_hp_current(&mut self, stat_hp_current: u16);
    fn get_stat_atk(&self) -> u16;
    fn set_stat_atk(&mut self, stat_atk: u16);
    fn get_stat_def(&self) -> u16;
    fn set_stat_def(&mut self, stat_def: u16);
    fn get_stat_spe(&self) -> u16;
    fn set_stat_spe(&mut self, stat_spe: u16);
    fn get_stat_spa(&self) -> u16;
    fn set_stat_spa(&mut self, stat_spa: u16);
    fn get_stat_spd(&self) -> u16;
    fn set_stat_spd(&mut self, stat_spd: u16);

    fn get_version(&self) -> u8;
    fn set_version(&mut self, version: u8);

    fn get_pkrs_strain(&self) -> u8;
    fn set_pkrs_strain(&mut self, strain: u8);

    fn get_pkrs_days(&self) -> u8;
    fn set_pkrs_days(&mut self, days: u8);

    fn get_encryption_constant(&self) -> u32;
    fn set_encryption_constant(&mut self, constant: u32);

    fn get_pid(&self) -> u32;
    fn set_pid(&mut self, pid: u32);

    fn set_language(&mut self, language: u8);

    fn get_fateful_encounter(&self) -> bool;
    fn set_fateful_encounter(&mut self, fateful: bool);

    fn characteristic(&self) -> Option<u8>;

    fn get_mark_value(&self) -> u8;
    fn set_mark_value(&mut self, value: u8);

    fn get_met_location(&self) -> u16;
    fn set_met_location(&mut self, location: u16);

    fn get_egg_location(&self) -> u16;
    fn set_egg_location(&mut self, location: u16);

    fn get_ot_friendship(&self) -> u8;
    fn set_ot_friendship(&mut self, friendship: u8);

    fn japanese(&self) -> bool {
        self.language() == LanguageID::Japanese as u8
    }

    fn korean(&self) -> bool {
        self.language() == LanguageID::Korean as u8
    }

    fn get_met_year(&self) -> u8 {
        0
    }
    fn set_met_year(&mut self, _year: u8) {}
    fn get_met_month(&self) -> u8 {
        0
    }
    fn set_met_month(&mut self, _month: u8) {}
    fn get_met_day(&self) -> u8 {
        0
    }
    fn set_met_day(&mut self, _day: u8) {}
    fn get_ht_name(&self) -> String {
        String::new()
    }
    fn set_ht_name(&mut self, _name: String) {}
    fn get_ht_gender(&self) -> u8 {
        0
    }
    fn set_ht_gender(&mut self, _gender: u8) {}
    fn get_ht_friendship(&self) -> u8 {
        0
    }
    fn set_ht_friendship(&mut self, _friendship: u8) {}
    fn get_enjoyment(&self) -> u8 {
        0
    }
    fn set_enjoyment(&mut self, _enjoyment: u8) {}
    fn get_fullness(&self) -> u8 {
        0
    }
    fn set_fullness(&mut self, _fullness: u8) {}
    fn get_ability_number(&self) -> u8 {
        0
    }
    fn set_ability_number(&mut self, _number: u8) {}

    fn get_met_date(&self) -> Option<PrimitiveDateTime> {
        if !date_util::is_date_valid(
            2000 + self.get_met_year() as i32,
            self.get_met_month(),
            self.get_met_day(),
        ) {
            None
        } else {
            Some(PrimitiveDateTime::new(
                Date::from_calendar_date(
                    2000 + self.get_met_year() as i32,
                    Month::try_from(self.get_met_month()).unwrap(),
                    self.get_met_day(),
                )
                .unwrap(),
                Time::MIDNIGHT,
            ))
        }
    }

    fn set_met_date(&mut self, date_time: Option<PrimitiveDateTime>) {
        if let Some(date_time) = date_time {
            self.set_met_year((date_time.year() - 2000) as u8);
            self.set_met_month(date_time.month().into());
            self.set_met_day(date_time.day());
        } else {
            self.set_met_year(0);
            self.set_met_month(0);
            self.set_met_day(0);
        }
    }

    fn get_egg_year(&self) -> u8 {
        0
    }
    fn set_egg_year(&mut self, _year: u8) {}
    fn get_egg_month(&self) -> u8 {
        0
    }
    fn set_egg_month(&mut self, _month: u8) {}
    fn get_egg_day(&self) -> u8 {
        0
    }
    fn set_egg_day(&mut self, _day: u8) {}

    fn get_egg_date(&self) -> Option<PrimitiveDateTime> {
        if !date_util::is_date_valid(
            2000 + self.get_egg_year() as i32,
            self.get_egg_month(),
            self.get_egg_day(),
        ) {
            None
        } else {
            Some(PrimitiveDateTime::new(
                Date::from_calendar_date(
                    2000 + self.get_egg_year() as i32,
                    Month::try_from(self.get_egg_month()).unwrap(),
                    self.get_egg_day(),
                )
                .unwrap(),
                Time::MIDNIGHT,
            ))
        }
    }

    fn set_egg_date(&mut self, date_time: Option<PrimitiveDateTime>) {
        if let Some(date_time) = date_time {
            self.set_egg_year((date_time.year() - 2000) as u8);
            self.set_egg_month(date_time.month().into());
            self.set_egg_day(date_time.day());
        } else {
            self.set_egg_year(0);
            self.set_egg_month(0);
            self.set_egg_day(0);
        }
    }

    fn get_relearn_move_1(&self) -> u16 {
        0
    }
    fn set_relearn_move_1(&mut self, _move_1: u16) {}
    fn get_relearn_move_2(&self) -> u16 {
        0
    }
    fn set_relearn_move_2(&mut self, _move_2: u16) {}
    fn get_relearn_move_3(&self) -> u16 {
        0
    }
    fn set_relearn_move_3(&mut self, _move_3: u16) {}
    fn get_relearn_move_4(&self) -> u16 {
        0
    }
    fn set_relearn_move_4(&mut self, _move_4: u16) {}

    fn get_current_handler(&self) -> u8;
    fn set_current_handler(&mut self, handler: u8);

    fn get_sprite_item(&self) -> u16 {
        self.get_held_item()
    }
    fn set_sprite_item(&mut self, item: u16) {
        self.set_held_item(item);
    }

    fn get_trainer_id_7(&self) -> u32 {
        (self.get_tid() as u32 | ((self.get_sid() as u32) << 16)) % 1000000
    }

    fn set_trainer_id_7(&mut self, id: u32) {
        self.set_id_7(self.get_trainer_sid_7(), id);
    }

    fn get_trainer_sid_7(&self) -> u32 {
        (self.get_tid() as u32 | ((self.get_sid() as u32) << 16)) / 1000000
    }

    fn set_trainer_sid_7(&mut self, sid: u32) {
        self.set_id_7(sid, self.get_trainer_id_7());
    }

    fn shiny_xor(&self) -> u32 {
        let pid = self.get_pid();
        let upper = (pid >> 16) ^ self.get_sid() as u32;
        (pid & 0xFFFF) ^ self.get_tid() as u32 ^ upper
    }

    fn get_display_tid(&self) -> u32 {
        if self.generation() >= 7 {
            self.get_trainer_id_7()
        } else {
            self.get_tid() as u32
        }
    }

    fn set_display_tid(&mut self, tid: u32) {
        if self.generation() >= 7 {
            self.set_trainer_id_7(tid);
        } else {
            self.set_tid(tid as u16);
        }
    }

    fn get_display_sid(&self) -> u32 {
        if self.generation() >= 7 {
            self.get_trainer_sid_7()
        } else {
            self.get_sid() as u32
        }
    }

    fn set_display_sid(&mut self, sid: u32) {
        if self.generation() >= 7 {
            self.set_trainer_sid_7(sid);
        } else {
            self.set_sid(sid as u16);
        }
    }

    fn set_id_7(&mut self, sid7: u32, tid7: u32) {
        let oid = sid7.wrapping_mul(1000000).wrapping_add(tid7 % 1000000);
        self.set_tid(oid as u16);
        self.set_sid((oid >> 16) as u16);
    }

    fn e(&self) -> bool {
        self.get_version() == GameVersion::E as u8
    }

    fn frlg(&self) -> bool {
        self.get_version() == GameVersion::FR as u8 || self.get_version() == GameVersion::LG as u8
    }

    fn pt(&self) -> bool {
        self.get_version() == GameVersion::Pt as u8
    }

    fn hgss(&self) -> bool {
        self.get_version() == GameVersion::HG as u8 || self.get_version() == GameVersion::SS as u8
    }

    fn bw(&self) -> bool {
        self.get_version() == GameVersion::B as u8 || self.get_version() == GameVersion::W as u8
    }

    fn b2w2(&self) -> bool {
        self.get_version() == GameVersion::B2 as u8 || self.get_version() == GameVersion::W2 as u8
    }

    fn xy(&self) -> bool {
        self.get_version() == GameVersion::X as u8 || self.get_version() == GameVersion::Y as u8
    }

    fn ao(&self) -> bool {
        self.get_version() == GameVersion::AS as u8 || self.get_version() == GameVersion::OR as u8
    }

    fn sm(&self) -> bool {
        self.get_version() == GameVersion::SN as u8 || self.get_version() == GameVersion::MN as u8
    }

    fn usum(&self) -> bool {
        self.get_version() == GameVersion::US as u8 || self.get_version() == GameVersion::UM as u8
    }

    fn go(&self) -> bool {
        self.get_version() == GameVersion::GO as u8
    }

    fn vc1(&self) -> bool {
        self.get_version() >= GameVersion::RD as u8 && self.get_version() <= GameVersion::YW as u8
    }

    fn vc2(&self) -> bool {
        self.get_version() >= GameVersion::GD as u8 && self.get_version() <= GameVersion::C as u8
    }

    fn lgpe(&self) -> bool {
        self.get_version() == GameVersion::GP as u8 || self.get_version() == GameVersion::GE as u8
    }

    fn swsh(&self) -> bool {
        self.get_version() == GameVersion::SW as u8 || self.get_version() == GameVersion::SH as u8
    }

    fn bdsp(&self) -> bool {
        self.get_version() == GameVersion::BD as u8 || self.get_version() == GameVersion::SP as u8
    }

    fn la(&self) -> bool {
        self.get_version() == GameVersion::PLA as u8
    }

    fn go_lgpe(&self) -> bool {
        self.go() && self.get_met_location() == locations::GO7
    }

    fn go_home(&self) -> bool {
        self.go() && self.get_met_location() == locations::GO8
    }

    fn vc(&self) -> bool {
        self.vc1() || self.vc2()
    }

    fn gg(&self) -> bool {
        self.lgpe() || self.go_lgpe()
    }

    fn gen8(&self) -> bool {
        self.get_version() >= 44 && self.get_version() <= 49 || self.go_home()
    }

    fn gen7(&self) -> bool {
        self.get_version() >= 30 && self.get_version() <= 33 || self.gg()
    }

    fn gen6(&self) -> bool {
        self.get_version() >= 24 && self.get_version() <= 29
    }

    fn gen5(&self) -> bool {
        self.get_version() >= 20 && self.get_version() <= 23
    }

    fn gen4(&self) -> bool {
        self.get_version() >= 7 && self.get_version() <= 12 && self.get_version() != 9
    }

    fn gen3(&self) -> bool {
        self.get_version() >= 1 && self.get_version() <= 5 || self.get_version() == 15
    }

    fn gen2(&self) -> bool {
        self.get_version() == GameVersion::GSC as u8
    }

    fn gen1(&self) -> bool {
        self.get_version() == GameVersion::RBY as u8
    }

    fn genu(&self) -> bool {
        self.generation() == 0
    }

    fn generation(&self) -> u8 {
        if self.gen8() {
            8
        } else if self.gen7() {
            7
        } else if self.gen6() {
            6
        } else if self.gen5() {
            5
        } else if self.gen4() {
            4
        } else if self.gen3() {
            3
        } else if self.gen2() || self.gen1() {
            self.format()
        } else if self.vc1() {
            1
        } else if self.vc2() {
            2
        } else {
            0
        }
    }

    fn get_pkrs_infected(&self) -> bool {
        self.get_pkrs_strain() != 0
    }

    fn set_pkrs_infected(&mut self, infected: bool) {
        let strain = self.get_pkrs_strain();
        self.set_pkrs_strain(if infected { strain.max(1) } else { 0 });
    }

    fn get_pkrs_cursed(&self) -> bool {
        self.get_pkrs_days() == 0 && self.get_pkrs_strain() > 0
    }

    fn set_pkrs_cursed(&mut self, cursed: bool) {
        self.set_pkrs_days(u8::from(!cursed));
        self.set_pkrs_infected(true);
    }

    fn get_current_level(&self) -> usize {
        experience::get_level(self.get_exp(), self.personal_info().get_exp_growth())
    }

    fn set_current_level(&mut self, level: usize) {
        self.set_stat_level(level as u8);
        self.set_exp(experience::get_exp(
            level,
            self.personal_info().get_exp_growth(),
        ))
    }

    fn iv_total(&self) -> u16 {
        u16::from(self.get_iv_hp())
            + u16::from(self.get_iv_atk())
            + u16::from(self.get_iv_def())
            + u16::from(self.get_iv_spe())
            + u16::from(self.get_iv_spa())
            + u16::from(self.get_iv_spd())
    }

    fn ev_total(&self) -> u16 {
        u16::from(self.get_ev_hp())
            + u16::from(self.get_ev_atk())
            + u16::from(self.get_ev_def())
            + u16::from(self.get_ev_spe())
            + u16::from(self.get_ev_spa())
            + u16::from(self.get_ev_spd())
    }

    fn maximum_iv(&self) -> u8 {
        self.get_iv_hp()
            .max(self.get_iv_atk())
            .max(self.get_iv_def())
            .max(self.get_iv_spe())
            .max(self.get_iv_spa())
            .max(self.get_iv_spd())
    }

    fn flawless_iv_count(&self) -> u8 {
        let max = self.max_iv();
        let mut ctr = 0;
        if self.get_iv_hp() == max {
            ctr += 1;
        }
        if self.get_iv_atk() == max {
            ctr += 1;
        }
        if self.get_iv_def() == max {
            ctr += 1;
        }
        if self.get_iv_spe() == max {
            ctr += 1;
        }
        if self.get_iv_spa() == max {
            ctr += 1;
        }
        if self.get_iv_spd() == max {
            ctr += 1;
        }
        ctr
    }

    fn file_name(&self) -> String {
        format!(
            "{}.{}",
            self.file_name_without_extension(),
            self.extension()
        )
    }

    fn file_name_without_extension(&self) -> String {
        entity_file_namer::get_name_regular_without_checksum(self)
    }

    fn get_ivs(&self) -> [u8; 6] {
        [
            self.get_iv_hp(),
            self.get_iv_atk(),
            self.get_iv_def(),
            self.get_iv_spe(),
            self.get_iv_spa(),
            self.get_iv_spd(),
        ]
    }

    fn set_ivs(&mut self, ivs: &[u8; 6]) {
        self.set_iv_hp(ivs[0]);
        self.set_iv_atk(ivs[1]);
        self.set_iv_def(ivs[2]);
        self.set_iv_spe(ivs[3]);
        self.set_iv_spa(ivs[4]);
        self.set_iv_spd(ivs[5]);
    }

    fn get_evs(&self) -> [u8; 6] {
        [
            self.get_ev_hp(),
            self.get_ev_atk(),
            self.get_ev_def(),
            self.get_ev_spe(),
            self.get_ev_spa(),
            self.get_ev_spd(),
        ]
    }

    fn set_evs(&mut self, evs: &[u8; 6]) {
        self.set_ev_hp(evs[0]);
        self.set_ev_atk(evs[1]);
        self.set_ev_def(evs[2]);
        self.set_ev_spe(evs[3]);
        self.set_ev_spa(evs[4]);
        self.set_ev_spd(evs[5]);
    }

    fn get_stats(&self) -> [u16; 6] {
        [
            self.get_stat_hp_current(),
            self.get_stat_atk(),
            self.get_stat_def(),
            self.get_stat_spe(),
            self.get_stat_spa(),
            self.get_stat_spd(),
        ]
    }

    fn set_stats(&mut self, stats: [u16; 6]) {
        self.set_stat_hp_max(stats[0]);
        self.set_stat_hp_current(stats[0]);
        self.set_stat_atk(stats[1]);
        self.set_stat_def(stats[2]);
        self.set_stat_spe(stats[3]);
        self.set_stat_spa(stats[4]);
        self.set_stat_spd(stats[5]);
    }

    fn get_moves(&self) -> [u16; 4] {
        [
            self.get_move_1(),
            self.get_move_2(),
            self.get_move_3(),
            self.get_move_4(),
        ]
    }

    fn set_moves(&mut self, moves: &[u16; 4]) {
        self.set_move_1(moves[0]);
        self.set_move_2(moves[1]);
        self.set_move_3(moves[2]);
        self.set_move_4(moves[3]);
    }

    fn push_move(&mut self, mov: u16) {
        if mov == 0 || mov >= self.max_move_id() || self.has_move(mov) {
            return;
        }
        let mut ct = self.move_count();
        if ct == 4 {
            ct = 0;
        }
        self.set_move(ct, mov);
        self.heal_pp_index(ct);
    }

    fn move_count(&self) -> u8 {
        u8::from(self.get_move_1() != 0)
            + u8::from(self.get_move_2() != 0)
            + u8::from(self.get_move_3() != 0)
            + u8::from(self.get_move_4() != 0)
    }

    fn set_moves_from_set(&mut self, set: Moveset) {
        self.set_move_1(set.move1);
        self.set_move_2(set.move2);
        self.set_move_3(set.move3);
        self.set_move_4(set.move4);
    }

    fn get_relearn_moves(&self) -> [u16; 4] {
        [
            self.get_relearn_move_1(),
            self.get_relearn_move_2(),
            self.get_relearn_move_3(),
            self.get_relearn_move_4(),
        ]
    }

    fn set_relearn_moves(&mut self, moves: &[u16; 4]) {
        self.set_relearn_move_1(moves[0]);
        self.set_relearn_move_2(moves[1]);
        self.set_relearn_move_3(moves[2]);
        self.set_relearn_move_4(moves[3]);
    }

    fn pid_ability(&self) -> Option<u16> {
        if self.generation() > 5 || self.format() > 5 {
            None
        } else if self.get_version() == GameVersion::CXD as u8 {
            self.personal_info()
                .get_ability_index(self.get_ability() as usize)
                .map(|a| a as u16)
        } else {
            Some(
                (if self.gen5() {
                    self.get_pid() >> 16
                } else {
                    self.get_pid()
                } & 1) as u16,
            )
        }
    }

    fn marking_count(&self) -> u8;
    fn get_marking(&self, index: u8) -> u8;
    fn set_marking(&mut self, index: u8, value: u8);

    fn hp_bit_val_power(&self) -> u16 {
        ((u16::from(self.get_iv_hp()) & 2) >> 1)
            | (u16::from(self.get_iv_atk()) & 2)
            | ((u16::from(self.get_iv_def()) & 2) << 1)
            | ((u16::from(self.get_iv_spe()) & 2) << 2)
            | ((u16::from(self.get_iv_spa()) & 2) << 3)
            | ((u16::from(self.get_iv_spd()) & 2) << 4)
    }

    fn hp_power(&self) -> u16 {
        if self.format() < 6 {
            ((40 * self.hp_bit_val_power()) / 63) + 30
        } else {
            60
        }
    }

    fn hp_bit_val_type(&self) -> u16 {
        (u16::from(self.get_iv_hp()) & 2)
            | ((u16::from(self.get_iv_atk()) & 2) << 1)
            | ((u16::from(self.get_iv_def()) & 2) << 2)
            | ((u16::from(self.get_iv_spe()) & 2) << 3)
            | ((u16::from(self.get_iv_spa()) & 2) << 4)
            | ((u16::from(self.get_iv_spd()) & 2) << 5)
    }

    fn get_hp_type(&self) -> u16 {
        15 * self.hp_bit_val_type() / 63
    }

    fn set_hp_type(&mut self, val: u16) {
        let arr = &hidden_power::DEFAULT_LOW_BITS;
        let bits = if usize::from(val) >= arr.len() {
            0
        } else {
            arr[usize::from(val)]
        };
        self.set_iv_hp((self.get_iv_hp() & !1) + (bits & 1));
        self.set_iv_atk((self.get_iv_atk() & !1) + ((bits >> 1) & 1));
        self.set_iv_def((self.get_iv_def() & !1) + ((bits >> 2) & 1));
        self.set_iv_spe((self.get_iv_spe() & !1) + ((bits >> 3) & 1));
        self.set_iv_spa((self.get_iv_spa() & !1) + ((bits >> 4) & 1));
        self.set_iv_spd((self.get_iv_spd() & !1) + ((bits >> 5) & 1));
    }

    fn was_egg(&self) -> bool {
        self.get_is_egg() || self.get_egg_day() != 0
    }

    fn was_traded_egg(&self) -> bool {
        self.get_egg_location() == self.get_traded_egg_location()
    }

    fn get_traded_egg_location(&self) -> u16 {
        locations::traded_egg_location(
            self.generation(),
            GameVersion::from(self.get_version() as usize),
        )
    }

    fn is_untraded(&self) -> bool {
        false
    }

    fn is_native(&self) -> bool {
        self.generation() == self.format()
    }

    fn is_origin_valid(&self) -> bool {
        self.species() <= self.max_species_id()
    }

    fn has_original_met_location(&self) -> bool {
        !(self.format() < 3
            || self.vc()
            || (self.generation() <= 4 && self.format() != self.generation()))
    }

    fn is_gender_valid(&self) -> bool {
        let gender = self.get_gender();
        let gv = self.personal_info().get_gender();
        if gv == personal_info::RATIO_MAGIC_GENDERLESS {
            gender == 2
        } else if gv == personal_info::RATIO_MAGIC_FEMALE {
            gender == 1
        } else if gv == personal_info::RATIO_MAGIC_MALE {
            gender == 0
        } else if self.generation() != 3 && self.generation() != 4 && self.generation() != 5 {
            gender == (gender & 1)
        } else {
            gender == entity_gender::get_from_pid_and_ratio(self.get_pid(), gv)
        }
    }

    fn refresh_checksum(&mut self);

    fn checksum_valid(&self) -> bool;

    fn fix_moves(&mut self) {
        self.reorder_moves();

        if self.get_move_1() == 0 {
            self.set_move_1_pp(0);
            self.set_move_1_pp_ups(0);
        }
        if self.get_move_2() == 0 {
            self.set_move_2_pp(0);
            self.set_move_2_pp_ups(0);
        }
        if self.get_move_3() == 0 {
            self.set_move_3_pp(0);
            self.set_move_3_pp_ups(0);
        }
        if self.get_move_4() == 0 {
            self.set_move_4_pp(0);
            self.set_move_4_pp_ups(0);
        }
    }

    fn reorder_moves(&mut self) {
        if self.get_move_1() == 0 && self.get_move_2() != 0 {
            self.set_move_1(self.get_move_2());
            self.set_move_1_pp(self.get_move_2_pp());
            self.set_move_1_pp_ups(self.get_move_2_pp_ups());
            self.set_move_2(0);
        }
        if self.get_move_2() == 0 && self.get_move_3() != 0 {
            self.set_move_2(self.get_move_3());
            self.set_move_2_pp(self.get_move_3_pp());
            self.set_move_2_pp_ups(self.get_move_3_pp_ups());
            self.set_move_3(0);
        }
        if self.get_move_3() == 0 && self.get_move_4() != 0 {
            self.set_move_3(self.get_move_3());
            self.set_move_3_pp(self.get_move_3_pp());
            self.set_move_3_pp_ups(self.get_move_3_pp_ups());
            self.set_move_4(0);
        }
    }

    fn refresh_ability(&mut self, n: u8) {
        self.set_ability_number(1 << n);
        let pi = self.personal_info();
        let abilities = pi.get_abilities();
        if n < abilities.len() as u8 {
            self.set_ability(abilities[n as usize] as u16)
        }
    }

    fn potential_rating(&self) -> u8 {
        match self.iv_total() {
            i if i <= 90 => 0,
            i if i <= 120 => 1,
            i if i <= 150 => 2,
            _ => 3,
        }
    }

    fn get_battle_stats(&self, p: &impl PersonalInfo) -> [u16; 6] {
        let mut stats = [0; 6];

        stats[0] = if p.get_hp() == 1 {
            1
        } else {
            ((u16::from(self.get_iv_hp())
                + (2 * p.get_hp() as u16)
                + (u16::from(self.get_ev_hp()) / 4)
                + 100)
                * self.get_current_level() as u16
                / 100)
                + 10
        };
        stats[1] = ((u16::from(self.get_iv_atk())
            + (2 * p.get_atk() as u16)
            + (u16::from(self.get_ev_atk()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[2] = ((u16::from(self.get_iv_def())
            + (2 * p.get_def() as u16)
            + (u16::from(self.get_ev_def()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[4] = ((u16::from(self.get_iv_spa())
            + (2 * p.get_spa() as u16)
            + (u16::from(self.get_ev_spa()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[5] = ((u16::from(self.get_iv_spd())
            + (2 * p.get_spd() as u16)
            + (u16::from(self.get_ev_spd()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[3] = ((u16::from(self.get_iv_spe())
            + (2 * p.get_spe() as u16)
            + (u16::from(self.get_ev_spe()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;

        nature_amp::modify_stats_for_nature(&mut stats, self.get_nature());

        stats
    }

    fn get_battle_stats_ht(&self, p: &impl PersonalInfo) -> [u16; 6]
    where
        Self: HyperTrain,
    {
        let mut stats = [0; 6];

        stats[0] = if p.get_hp() == 1 {
            1
        } else {
            ((u16::from(if self.get_ht_hp() {
                31
            } else {
                self.get_iv_hp()
            }) + (2 * p.get_hp() as u16)
                + (u16::from(self.get_ev_hp()) / 4)
                + 100)
                * self.get_current_level() as u16
                / 100)
                + 10
        };
        stats[1] = ((u16::from(if self.get_ht_atk() {
            31
        } else {
            self.get_iv_atk()
        }) + (2 * p.get_atk() as u16)
            + (u16::from(self.get_ev_atk()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[2] = ((u16::from(if self.get_ht_def() {
            31
        } else {
            self.get_iv_def()
        }) + (2 * p.get_def() as u16)
            + (u16::from(self.get_ev_def()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[4] = ((u16::from(if self.get_ht_spa() {
            31
        } else {
            self.get_iv_spa()
        }) + (2 * p.get_spa() as u16)
            + (u16::from(self.get_ev_spa()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[5] = ((u16::from(if self.get_ht_spd() {
            31
        } else {
            self.get_iv_spd()
        }) + (2 * p.get_spd() as u16)
            + (u16::from(self.get_ev_spd()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;
        stats[3] = ((u16::from(if self.get_ht_spe() {
            31
        } else {
            self.get_iv_spe()
        }) + (2 * p.get_spe() as u16)
            + (u16::from(self.get_ev_spe()) / 4)
            + 100)
            * self.get_current_level() as u16
            / 100)
            + 5;

        nature_amp::modify_stats_for_nature(&mut stats, self.get_nature());

        stats
    }

    fn party_stats_present(&self) -> bool {
        self.get_stat_hp_max() != 0
    }

    fn reset_party_stats(&mut self) {
        let stats = self.get_battle_stats(self.personal_info());
        self.set_stats(stats);
        self.set_stat_level(self.get_current_level() as u8);
        self.set_status_condition(0);
    }

    fn reset_party_stats_ht(&mut self)
    where
        Self: HyperTrain,
    {
        let stats = self.get_battle_stats_ht(self.personal_info());
        self.set_stats(stats);
        self.set_stat_level(self.get_current_level() as u8);
        self.set_status_condition(0);
    }

    fn heal(&mut self) {
        self.reset_party_stats();
        self.heal_pp();
    }

    fn heal_ht(&mut self)
    where
        Self: HyperTrain,
    {
        self.reset_party_stats_ht();
        self.heal_pp();
    }

    fn heal_pp(&mut self) {
        self.set_move_1_pp(self.get_move_pp(self.get_move_1(), self.get_move_1_pp_ups()));
        self.set_move_2_pp(self.get_move_pp(self.get_move_2(), self.get_move_2_pp_ups()));
        self.set_move_3_pp(self.get_move_pp(self.get_move_3(), self.get_move_3_pp_ups()));
        self.set_move_4_pp(self.get_move_pp(self.get_move_4(), self.get_move_4_pp_ups()));
    }

    fn heal_pp_index(&mut self, index: u8) -> u8 {
        match index {
            0 => {
                self.set_move_1_pp(self.get_move_pp(self.get_move_1(), self.get_move_1_pp_ups()));
                self.get_move_1_pp()
            }
            1 => {
                self.set_move_2_pp(self.get_move_pp(self.get_move_2(), self.get_move_2_pp_ups()));
                self.get_move_2_pp()
            }
            2 => {
                self.set_move_3_pp(self.get_move_pp(self.get_move_3(), self.get_move_3_pp_ups()));
                self.get_move_3_pp()
            }
            3 => {
                self.set_move_4_pp(self.get_move_pp(self.get_move_4(), self.get_move_4_pp_ups()));
                self.get_move_4_pp()
            }
            _ => panic!("Move index outside of range"),
        }
    }

    fn force_party_data(&mut self) -> bool {
        if self.party_stats_present() {
            false
        } else {
            self.reset_party_stats();
            true
        }
    }

    fn force_party_data_ht(&mut self) -> bool
    where
        Self: HyperTrain,
    {
        if self.party_stats_present() {
            false
        } else {
            self.reset_party_stats_ht();
            true
        }
    }

    fn can_hold_item(&self, valid: &[u16]) -> bool {
        valid.contains(&self.get_held_item())
    }

    fn set_link_trade_egg(&mut self, day: u8, month: u8, year: i32, location: u16) {
        self.set_met_day(day);
        self.set_met_month(month);
        self.set_met_year((year - 2000) as u8);
        self.set_met_location(location);
    }

    fn get_move_pp(&self, mov: u16, pp_up_count: u8) -> u8 {
        self.get_base_pp(mov) * (5 + pp_up_count) / 5
    }

    fn get_base_pp(&self, mov: u16) -> u8 {
        moves::get_pp(self.context(), mov)
    }

    fn set_shiny(&mut self) {
        let mut rnd = thread_rng();
        while {
            self.set_pid(entity_pid::get_random_pid(
                &mut rnd,
                self.species(),
                self.get_gender(),
                self.get_version(),
                self.get_nature(),
                self.form(),
                self.get_pid(),
            ));
            !self.is_shiny()
        } {}
        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_shiny_sid(&mut self, shiny: shiny::Shiny) {
        if self.is_shiny() && shiny.is_valid(self) {
            return;
        }

        let xor = self.get_tid() as u32 ^ (self.get_pid() >> 16) ^ (self.get_pid() & 0xFFFF);
        let bits = match shiny {
            shiny::Shiny::AlwaysSquare => 0,
            shiny::Shiny::AlwaysStar => 1,
            _ => thread_rng().gen_range(0u32..8u32),
        };

        self.set_sid((xor ^ bits) as u16);
    }

    fn set_pid_gender(&mut self, gender: u8) {
        let mut rnd = thread_rng();
        while {
            self.set_pid(entity_pid::get_random_pid(
                &mut rnd,
                self.species(),
                gender,
                self.get_version(),
                self.get_nature(),
                self.form(),
                self.get_pid(),
            ));
            self.is_shiny()
        } {}

        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_pid_nature(&mut self, nature: u8) {
        let mut rnd = thread_rng();
        while {
            self.set_pid(entity_pid::get_random_pid(
                &mut rnd,
                self.species(),
                self.get_gender(),
                self.get_version(),
                nature,
                self.form(),
                self.get_pid(),
            ));
            self.is_shiny()
        } {}

        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_pid_unown_3(&mut self, form: u8) {
        let mut rnd = thread_rng();
        while {
            self.set_pid(rnd.gen::<u32>());
            entity_pid::get_unown_form_3(self.get_pid()) != form
        } {}

        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_random_ivs(&mut self, min_flawless: Option<u8>) {
        if self.get_version() == GameVersion::GO as u8 {
            if let Some(min_flawless) = min_flawless {
                if min_flawless != 6 {
                    self.set_random_ivs_go(0, 15);
                    return;
                }
            }
        }

        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        ivs.iter_mut()
            .for_each(|iv| *iv = rnd.gen_range(0..=self.max_iv()));
        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        if count != 0 {
            ivs.iter_mut().take(3).for_each(|iv| *iv = self.max_iv());
            let len = ivs.len();
            rand_util::shuffle(&mut ivs, 0, len, &mut rnd);
        }
        self.set_ivs(&ivs);
    }

    fn set_random_ivs_alpha(&mut self, min_flawless: Option<u8>)
    where
        Self: Alpha,
    {
        if self.get_version() == GameVersion::GO as u8 {
            if let Some(min_flawless) = min_flawless {
                if min_flawless != 6 {
                    self.set_random_ivs_go(0, 15);
                    return;
                }
            }
        }

        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        ivs.iter_mut()
            .for_each(|iv| *iv = rnd.gen_range(0..=self.max_iv()));
        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count_alpha());
        if count != 0 {
            ivs.iter_mut().take(3).for_each(|iv| *iv = self.max_iv());
            let len = ivs.len();
            rand_util::shuffle(&mut ivs, 0, len, &mut rnd);
        }
        self.set_ivs(&ivs);
    }

    fn set_random_ivs_go(&mut self, min_iv: u8, max_iv: u8) {
        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        ivs.iter_mut()
            .for_each(|iv| *iv = (rnd.gen_range(min_iv..=max_iv) << 1) | 1);
        self.set_ivs(&ivs);
    }

    fn set_random_ivs_template(&mut self, template: IndividualValueSet, min_flawless: Option<u8>) {
        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        while {
            ivs.iter_mut().enumerate().for_each(|(i, iv)| {
                if let Some(val) = template[i] {
                    *iv = val;
                } else {
                    *iv = rnd.gen_range(0..=self.max_iv());
                }
            });
            ivs.iter().filter(|&&iv| iv == self.max_iv()).count() < count as usize
        } {}

        self.set_ivs(&ivs);
    }

    fn set_random_ivs_template_alpha(
        &mut self,
        template: IndividualValueSet,
        min_flawless: Option<u8>,
    ) where
        Self: Alpha,
    {
        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count_alpha());
        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        while {
            ivs.iter_mut().enumerate().for_each(|(i, iv)| {
                if let Some(val) = template[i] {
                    *iv = val;
                } else {
                    *iv = rnd.gen_range(0..=self.max_iv());
                }
            });
            ivs.iter().filter(|&&iv| iv == self.max_iv()).count() < count as usize
        } {}

        self.set_ivs(&ivs);
    }

    fn get_flawless_iv_count_alpha(&self) -> u8
    where
        Self: Alpha,
    {
        match self.get_flawless_iv_count() {
            i if i > 0 => i,
            _ => {
                if self.get_is_alpha() {
                    3
                } else {
                    0
                }
            }
        }
    }

    fn get_flawless_iv_count(&self) -> u8 {
        if self.generation() >= 6
            && (tables::LEGENDS.contains(&(self.species() as usize))
                || tables::SUB_LEGENDS.contains(&(self.species() as usize)))
        {
            return 3;
        }

        if self.xy() {
            if self.personal_info().get_egg_group_1() == 15 {
                return 3;
            }
            if self.get_met_location() == 148 && self.get_met_level() == 30 {
                return 2;
            }
        }

        if self.vc() {
            return if self.species() == Species::Mew as u16
                || self.species() == Species::Celebi as u16
            {
                5
            } else {
                3
            };
        }
        0
    }

    fn has_move(&self, mov: u16) -> bool {
        self.get_move_1() == mov
            || self.get_move_2() == mov
            || self.get_move_3() == mov
            || self.get_move_4() == mov
    }

    fn get_move_index(&self, mov: u16) -> Option<u8> {
        if self.get_move_1() == mov {
            Some(0)
        } else if self.get_move_2() == mov {
            Some(1)
        } else if self.get_move_3() == mov {
            Some(2)
        } else if self.get_move_4() == mov {
            Some(3)
        } else {
            None
        }
    }

    fn get_move(&self, index: u8) -> u16 {
        match index {
            0 => self.get_move_1(),
            1 => self.get_move_2(),
            2 => self.get_move_3(),
            3 => self.get_move_4(),
            _ => panic!("Move index out of range for PKM"),
        }
    }

    fn set_move(&mut self, index: u8, mov: u16) {
        match index {
            0 => self.set_move_1(mov),
            1 => self.set_move_2(mov),
            2 => self.set_move_3(mov),
            3 => self.set_move_4(mov),
            _ => panic!("Move index outside of range"),
        }
    }

    fn get_relearn_move(&self, index: u8) -> u16 {
        match index {
            0 => self.get_relearn_move_1(),
            1 => self.get_relearn_move_2(),
            2 => self.get_relearn_move_3(),
            3 => self.get_relearn_move_4(),
            _ => panic!("Relearn move index out of range for PKM"),
        }
    }

    fn set_relearn_move(&mut self, index: u8, mov: u16) {
        match index {
            0 => self.set_relearn_move_1(mov),
            1 => self.set_relearn_move_2(mov),
            2 => self.set_relearn_move_3(mov),
            3 => self.set_relearn_move_4(mov),
            _ => panic!("Relearn move index outside of range"),
        }
    }

    fn has_relearn_move(&self, mov: u16) -> bool {
        self.get_relearn_move_1() == mov
            || self.get_relearn_move_2() == mov
            || self.get_relearn_move_3() == mov
            || self.get_relearn_move_4() == mov
    }

    fn clear_invalid_moves(&mut self) {
        let mut invalid = 0;
        let mut moves = self.get_moves();
        moves.iter_mut().for_each(|mov| {
            if *mov > self.max_move_id() {
                invalid += 1;
                *mov = 0;
            }
        });

        if invalid == 0 {
            return;
        }

        if invalid == 4 {
            moves[0] = 1;
            self.set_move_1_pp(self.get_move_pp(1, self.get_move_1_pp_ups()));
        }
        self.set_moves(&moves);
        self.fix_moves();
    }

    fn get_ev(&self, index: u8) -> u8 {
        match index {
            0 => self.get_ev_hp(),
            1 => self.get_ev_atk(),
            2 => self.get_ev_def(),
            3 => self.get_ev_spe(),
            4 => self.get_ev_spa(),
            5 => self.get_ev_spd(),
            _ => panic!("EV index out of range for PKM"),
        }
    }

    fn get_iv(&self, index: u8) -> u8 {
        match index {
            0 => self.get_iv_hp(),
            1 => self.get_iv_atk(),
            2 => self.get_iv_def(),
            3 => self.get_iv_spe(),
            4 => self.get_iv_spa(),
            5 => self.get_iv_spd(),
            _ => panic!("IV index out of range for PKM"),
        }
    }
}
