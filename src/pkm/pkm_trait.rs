use crate::evolution_legality::get_future_gen_evolutions;
use crate::hidden_power::DEFAULT_LOW_BITS;
use crate::pkx::{
    get_gender_from_pid_and_ratio, get_pkm_extensions, get_random_pid, get_unown_form,
    modify_stats_for_nature, GENERATION,
};
use crate::tables::locations::{is_none_location, traded_egg_location, GO7, GO8};
use crate::tables::{LEGENDS, SUB_LEGENDS};
use crate::{date_util, experience, get_debut_generation, get_max_species_origin, get_pp_table, rand_util, GameValueLimit, GameVersion, Generation, HyperTrain, LangNick, LanguageID, NatureT, PersonalInfo, Shiny, ShinyEnum, Species, SpeciesForm, TrainerId, RATIO_MAGIC_FEMALE, RATIO_MAGIC_GENDERLESS, RATIO_MAGIC_MALE, PKMType, ContestStats};
use rand::{Rng, RngCore};
use time::{Date, Month, PrimitiveDateTime, Time};

pub trait Pkm<T: PersonalInfo>:
    SpeciesForm + TrainerId + Generation + Shiny + LangNick + GameValueLimit + NatureT + Clone
{
    fn get_contest_stats(&self) -> Option<Box<&dyn ContestStats>> {
        None
    }

    fn extensions(&self) -> Vec<String> {
        get_pkm_extensions(GENERATION)
    }

    fn size_party(&self) -> usize;
    fn size_stored(&self) -> usize;

    fn get_type(&self) -> PKMType;

    fn extension(&self) -> String {
        self.get_type().to_string().to_lowercase()
    }

    fn get_personal_info(&self) -> &T;

    fn extra_bytes(&self) -> Vec<u16> {
        vec![]
    }

    fn get_data(&self) -> &Vec<u8>;

    fn new(data: Vec<u8>) -> Self;

    fn new_blank() -> Self;

    fn encrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()[..self.size_party()].to_vec()
    }

    fn encrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()[..self.size_stored()].to_vec()
    }

    fn decrypted_party_data(&mut self) -> Vec<u8> {
        self.write()[..self.size_party()].to_vec()
    }

    fn decrypted_box_data(&mut self) -> Vec<u8> {
        self.write()[..self.size_stored()].to_vec()
    }

    fn set_valid(&mut self, valid: bool);

    fn get_valid(&self) -> bool;

    fn nickname_trash(&self) -> Vec<u8>;
    fn ot_trash(&self) -> Vec<u8>;
    fn ht_trash(&self) -> Vec<u8> {
        vec![]
    }

    fn encrypt(&mut self) -> Vec<u8>;
    fn format(&self) -> usize;

    fn write(&mut self) -> Vec<u8> {
        self.refresh_checksum(); // TODO
        self.get_data().clone()
    }

    fn get_held_item(&self) -> usize;
    fn set_held_item(&mut self, held_item: usize);

    fn get_gender(&self) -> usize;
    fn set_gender(&mut self, gender: usize);

    fn get_stat_nature(&self) -> usize {
        self.get_nature()
    }

    fn set_stat_nature(&mut self, stat_nature: usize) {
        self.set_nature(stat_nature);
    }

    fn get_ability(&self) -> usize;
    fn set_ability(&mut self, ability: usize);

    fn get_current_friendship(&self) -> usize;
    fn set_current_friendship(&mut self, current_friendship: usize);

    fn get_is_egg(&self) -> bool;
    fn set_is_egg(&mut self, is_egg: bool);

    fn get_exp(&self) -> usize;
    fn set_exp(&mut self, exp: usize);

    fn get_ot_name(&self) -> String;
    fn set_ot_name(&mut self, ot_name: String);

    fn get_ot_gender(&self) -> usize;
    fn set_ot_gender(&mut self, ot_gender: usize);

    fn get_ball(&self) -> usize;
    fn set_ball(&mut self, ball: usize);

    fn get_met_level(&self) -> usize;
    fn set_met_level(&mut self, met_level: usize);

    fn get_move_1(&self) -> usize;
    fn set_move_1(&mut self, move_1: usize);

    fn get_move_2(&self) -> usize;
    fn set_move_2(&mut self, move_2: usize);

    fn get_move_3(&self) -> usize;
    fn set_move_3(&mut self, move3: usize);

    fn get_move_4(&self) -> usize;
    fn set_move_4(&mut self, move_4: usize);

    fn get_move_1_pp(&self) -> usize;
    fn set_move_1_pp(&mut self, move_1_pp: usize);

    fn get_move_2_pp(&self) -> usize;
    fn set_move_2_pp(&mut self, move_2_pp: usize);

    fn get_move_3_pp(&self) -> usize;
    fn set_move_3_pp(&mut self, move_3_pp: usize);

    fn get_move_4_pp(&self) -> usize;
    fn set_move_4_pp(&mut self, move_4_pp: usize);

    fn get_move_1_pp_ups(&self) -> usize;
    fn set_move_1_pp_ups(&mut self, move_1_pp_ups: usize);

    fn get_move_2_pp_ups(&self) -> usize;
    fn set_move_2_pp_ups(&mut self, move_2_pp_ups: usize);

    fn get_move_3_pp_ups(&self) -> usize;
    fn set_move_3_pp_ups(&mut self, move_3_pp_ups: usize);

    fn get_move_4_pp_ups(&self) -> usize;
    fn set_move_4_pp_ups(&mut self, move_4_pp_ups: usize);

    fn get_ev_hp(&self) -> usize;
    fn set_ev_hp(&mut self, hp: usize);

    fn get_ev_atk(&self) -> usize;
    fn set_ev_atk(&mut self, atk: usize);

    fn get_ev_def(&self) -> usize;
    fn set_ev_def(&mut self, def: usize);

    fn get_ev_spe(&self) -> usize;
    fn set_ev_spe(&mut self, spe: usize);

    fn get_ev_spa(&self) -> usize;
    fn set_ev_spa(&mut self, spa: usize);

    fn get_ev_spd(&self) -> usize;
    fn set_ev_spd(&mut self, spd: usize);

    fn get_iv_hp(&self) -> usize;
    fn set_iv_hp(&mut self, hp: usize);

    fn get_iv_atk(&self) -> usize;
    fn set_iv_atk(&mut self, atk: usize);

    fn get_iv_def(&self) -> usize;
    fn set_iv_def(&mut self, def: usize);

    fn get_iv_spe(&self) -> usize;
    fn set_iv_spe(&mut self, spe: usize);

    fn get_iv_spa(&self) -> usize;
    fn set_iv_spa(&mut self, spa: usize);

    fn get_iv_spd(&self) -> usize;
    fn set_iv_spd(&mut self, spd: usize);

    fn get_status_condition(&self) -> usize;
    fn set_status_condition(&mut self, status_condition: usize);

    fn get_stat_level(&self) -> usize;
    fn set_stat_level(&mut self, level: usize);

    fn get_stat_hp_max(&self) -> usize;
    fn set_stat_hp_max(&mut self, hp: usize);

    fn get_stat_hp_current(&self) -> usize;
    fn set_stat_hp_current(&mut self, hp: usize);

    fn get_stat_atk(&self) -> usize;
    fn set_stat_atk(&mut self, atk: usize);

    fn get_stat_def(&self) -> usize;
    fn set_stat_def(&mut self, def: usize);

    fn get_stat_spe(&self) -> usize;
    fn set_stat_spe(&mut self, spe: usize);

    fn get_stat_spa(&self) -> usize;
    fn set_stat_spa(&mut self, spa: usize);

    fn get_stat_spd(&self) -> usize;
    fn set_stat_spd(&mut self, spd: usize);

    fn get_version(&self) -> usize;
    fn set_version(&mut self, version: usize);

    fn get_pkrs_strain(&self) -> usize;
    fn set_pkrs_strain(&mut self, strain: usize);

    fn get_pkrs_days(&self) -> usize;
    fn set_pkrs_days(&mut self, days: usize);

    fn get_encryption_constant(&self) -> usize;
    fn set_encryption_constant(&mut self, ec: usize);

    fn get_pid(&self) -> usize;
    fn set_pid(&mut self, pid: usize);

    fn get_fateful_encounter(&self) -> bool;
    fn set_fateful_encounter(&mut self, fe: bool);

    fn get_tsv(&self) -> usize;
    fn set_tsv(&mut self, tsv: usize);

    fn get_psv(&self) -> usize;
    fn set_psv(&mut self, set_psv: usize);

    fn get_characteristic(&self) -> usize;
    fn set_characteristic(&mut self, characteristic: usize);

    fn get_mark_value(&self) -> usize;
    fn set_mark_value(&mut self, value: usize);

    fn get_met_location(&self) -> usize;
    fn set_met_location(&mut self, location: usize);

    fn get_egg_location(&self) -> usize;
    fn set_egg_location(&mut self, location: usize);

    fn get_ot_friendship(&self) -> usize;
    fn set_ot_friendship(&mut self, friendship: usize);

    fn japanese(&self) -> bool {
        self.get_language() == LanguageID::Japanese as usize
    }

    fn korean(&self) -> bool {
        self.get_language() == LanguageID::Korean as usize
    }

    fn get_met_year(&self) -> usize {
        0
    }
    fn set_met_year(&mut self, _year: usize) {}

    fn get_met_month(&self) -> usize {
        0
    }
    fn set_met_month(&mut self, _month: usize) {}

    fn get_met_day(&self) -> usize {
        0
    }
    fn set_met_day(&mut self, _day: usize) {}

    fn get_ht_name(&self) -> String {
        "".to_string()
    }
    fn set_ht_name(&mut self, _name: String) {}

    fn get_ht_gender(&self) -> usize {
        0
    }
    fn set_ht_gender(&mut self, _gender: usize) {}

    fn get_ht_friendship(&self) -> usize {
        0
    }
    fn set_ht_friendship(&mut self, _friendship: usize) {}

    fn get_enjoyment(&self) -> u8 {
        0
    }
    fn set_enjoyment(&mut self, _enjoyment: u8) {}

    fn get_fullness(&self) -> u8 {
        0
    }
    fn set_fullness(&mut self, _fullness: u8) {}

    fn get_ability_number(&self) -> usize {
        0
    }
    fn set_ability_number(&mut self, _ability_number: usize) {}

    fn get_met_date(&self) -> Option<PrimitiveDateTime> {
        if !date_util::is_date_valid(
            2000 + self.get_met_year(),
            self.get_met_month(),
            self.get_met_day(),
        ) {
            None
        } else {
            Some(PrimitiveDateTime::new(
                Date::from_calendar_date(
                    2000 + self.get_met_year() as i32,
                    Month::try_from(self.get_met_month() as u8).unwrap(),
                    self.get_met_day() as u8,
                )
                .unwrap(),
                Time::MIDNIGHT,
            ))
        }
    }

    fn set_met_date(&mut self, date_time: Option<PrimitiveDateTime>) {
        if let Some(date_util) = date_time {
            self.set_met_year(date_util.year() as usize);
            self.set_met_month(date_util.month() as usize);
            self.set_met_day(date_util.day() as usize);
        } else {
            self.set_met_year(0);
            self.set_met_month(0);
            self.set_met_day(0);
        }
    }

    fn get_egg_year(&self) -> usize {
        0
    }
    fn set_egg_year(&mut self, _year: usize) {}

    fn get_egg_month(&self) -> usize {
        0
    }
    fn set_egg_month(&mut self, _month: usize) {}

    fn get_egg_day(&self) -> usize {
        0
    }
    fn set_egg_day(&mut self, _day: usize) {}

    fn get_egg_met_date(&self) -> Option<PrimitiveDateTime> {
        if !date_util::is_date_valid(
            2000 + self.get_egg_year(),
            self.get_egg_month(),
            self.get_egg_day(),
        ) {
            None
        } else {
            Some(PrimitiveDateTime::new(
                Date::from_calendar_date(
                    2000 + self.get_egg_year() as i32,
                    Month::try_from(self.get_egg_month() as u8).unwrap(),
                    self.get_egg_day() as u8,
                )
                .unwrap(),
                Time::MIDNIGHT,
            ))
        }
    }

    fn set_egg_met_date(&mut self, date_time: Option<PrimitiveDateTime>) {
        if let Some(date_util) = date_time {
            self.set_egg_year(date_util.year() as usize);
            self.set_egg_month(date_util.month() as usize);
            self.set_egg_day(date_util.day() as usize);
        } else {
            self.set_egg_year(0);
            self.set_egg_month(0);
            self.set_egg_day(0);
        }
    }

    fn get_relearn_move_1(&self) -> usize {
        0
    }
    fn set_relearn_move_1(&mut self, _move_1: usize) {}

    fn get_relearn_move_2(&self) -> usize {
        0
    }
    fn set_relearn_move_2(&mut self, _move_2: usize) {}

    fn get_relearn_move_3(&self) -> usize {
        0
    }
    fn set_relearn_move_3(&mut self, _move_3: usize) {}

    fn get_relearn_move_4(&self) -> usize {
        0
    }
    fn set_relearn_move_4(&mut self, _move_4: usize) {}

    fn get_current_handler(&self) -> usize;
    fn set_current_handler(&mut self, handler: usize);

    fn get_spec_form(&self) -> usize {
        self.get_species() + (self.get_form() << 11)
    }

    fn set_spec_form(&mut self, spec_form: usize) {
        self.set_species(spec_form & 0x7FF);
        self.set_form(spec_form >> 11);
    }

    fn sprite_item(&self) -> usize {
        self.get_held_item()
    }

    fn is_shiny(&self) -> bool {
        self.get_tsv() == self.get_psv()
    }

    fn get_trainer_id_7(&self) -> usize {
        (self.get_tid() | (self.get_sid() << 16)) % 1000000
    }

    fn set_trainer_id_7(&mut self, id_7: usize) {
        let old = (self.get_trainer_sid_7() * 1000000) + (id_7 % 1000000);
        self.set_tid(old & 0xFFFF);
        self.set_sid((old >> 16) & 0xFFFF);
    }

    fn get_trainer_sid_7(&self) -> usize {
        (self.get_tid() | (self.get_sid() << 16)) / 1000000
    }

    fn set_trainer_sid_7(&mut self, sid_7: usize) {
        let old = (sid_7 * 1000000) + (self.get_trainer_sid_7() % 1000000);
        self.set_tid(old & 0xFFFF);
        self.set_sid((old >> 16) & 0xFFFF);
    }

    fn shiny_xor(&self) -> usize {
        let pid = self.get_pid();
        let upper = (pid >> 16) ^ (self.get_sid() & 0xFFFF);
        (pid & 0xFFFF) ^ (self.get_tid() & 0xFFFF) ^ upper
    }

    fn get_display_tid(&self) -> usize {
        if self.get_generation() >= 7 {
            self.get_trainer_id_7()
        } else {
            self.get_tid()
        }
    }

    fn set_display_tid(&mut self, tid: usize) {
        if self.get_generation() >= 7 {
            self.set_trainer_id_7(tid);
        } else {
            self.set_tid(tid);
        }
    }

    fn get_display_sid(&self) -> usize {
        if self.get_generation() >= 7 {
            self.get_trainer_sid_7()
        } else {
            self.get_sid()
        }
    }

    fn set_display_sid(&mut self, sid: usize) {
        if self.get_generation() >= 7 {
            self.set_trainer_sid_7(sid);
        } else {
            self.set_sid(sid);
        }
    }

    fn e(&self) -> bool {
        self.get_version() == GameVersion::E as usize
    }

    fn frlg(&self) -> bool {
        self.get_version() == GameVersion::FR as usize
            || self.get_version() == GameVersion::LG as usize
    }

    fn pt(&self) -> bool {
        self.get_version() == GameVersion::Pt as usize
    }

    fn hgss(&self) -> bool {
        self.get_version() == GameVersion::HG as usize
            || self.get_version() == GameVersion::SS as usize
    }

    fn bw(&self) -> bool {
        self.get_version() == GameVersion::B as usize
            || self.get_version() == GameVersion::W as usize
    }

    fn b2w2(&self) -> bool {
        self.get_version() == GameVersion::B2 as usize
            || self.get_version() == GameVersion::W2 as usize
    }

    fn xy(&self) -> bool {
        self.get_version() == GameVersion::X as usize
            || self.get_version() == GameVersion::Y as usize
    }

    fn ao(&self) -> bool {
        self.get_version() == GameVersion::AS as usize
            || self.get_version() == GameVersion::OR as usize
    }

    fn sm(&self) -> bool {
        self.get_version() == GameVersion::SN as usize
            || self.get_version() == GameVersion::MN as usize
    }

    fn usum(&self) -> bool {
        self.get_version() == GameVersion::US as usize
            || self.get_version() == GameVersion::UM as usize
    }

    fn go(&self) -> bool {
        self.get_version() == GameVersion::GO as usize
    }

    fn vc1(&self) -> bool {
        self.get_version() >= GameVersion::RD as usize
            && self.get_version() <= GameVersion::YW as usize
    }

    fn vc2(&self) -> bool {
        self.get_version() >= GameVersion::GD as usize
            && self.get_version() <= GameVersion::C as usize
    }

    fn lgpe(&self) -> bool {
        self.get_version() == GameVersion::GP as usize
            || self.get_version() == GameVersion::GE as usize
    }

    fn swsh(&self) -> bool {
        self.get_version() == GameVersion::SW as usize
            || self.get_version() == GameVersion::SH as usize
    }

    fn bdsp(&self) -> bool {
        self.get_version() == GameVersion::BD as usize
            || self.get_version() == GameVersion::SP as usize
    }

    fn la(&self) -> bool {
        self.get_version() == GameVersion::PLA as usize
    }

    fn go_lgpe(&self) -> bool {
        self.go() && self.get_met_location() == GO7
    }

    fn go_home(&self) -> bool {
        self.go() && self.get_met_location() == GO8
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
        self.get_version() == GameVersion::GSC as usize
    }

    fn gen1(&self) -> bool {
        self.get_version() == GameVersion::RBY as usize
    }

    fn gen_u(&self) -> bool {
        self.generation() == 0
    }

    fn generation(&self) -> usize {
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

    fn debut_generation(&self) -> usize {
        get_debut_generation(self.get_species())
    }

    fn get_pkrs_infected(&self) -> bool {
        self.get_pkrs_strain() != 0
    }

    fn set_pkrs_infected(&mut self, infected: bool) {
        self.set_pkrs_strain(if infected {
            self.get_pkrs_strain().max(1)
        } else {
            0
        });
    }

    fn get_pkrs_cured(&self) -> bool {
        self.get_pkrs_days() == 0 && self.get_pkrs_strain() > 0
    }

    fn set_pkrs_cured(&mut self, cured: bool) {
        self.set_pkrs_days(if cured { 0 } else { 1 });
        self.set_pkrs_infected(true);
    }

    fn get_current_level(&self) -> usize {
        experience::get_level(
            self.get_exp() as u32,
            self.get_personal_info().get_exp_growth(),
        )
    }

    fn set_current_level(&mut self, level: usize) {
        self.set_stat_level(level);
        self.set_exp(
            experience::get_exp(level, self.get_personal_info().get_exp_growth()) as usize,
        );
    }

    fn get_mark_circle(&self) -> usize {
        self.get_markings()[0]
    }

    fn set_mark_circle(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[0] = mark;
        self.set_markings(&marks)
    }

    fn get_mark_triangle(&self) -> usize {
        self.get_markings()[1]
    }

    fn set_mark_triangle(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[1] = mark;
        self.set_markings(&marks)
    }

    fn get_mark_square(&self) -> usize {
        self.get_markings()[2]
    }

    fn set_mark_square(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[2] = mark;
        self.set_markings(&marks)
    }

    fn get_mark_heart(&self) -> usize {
        self.get_markings()[3]
    }

    fn set_mark_heart(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[3] = mark;
        self.set_markings(&marks)
    }

    fn get_mark_star(&self) -> usize {
        self.get_markings()[4]
    }

    fn set_mark_star(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[4] = mark;
        self.set_markings(&marks)
    }

    fn get_mark_diamond(&self) -> usize {
        self.get_markings()[5]
    }

    fn set_mark_diamond(&mut self, mark: usize) {
        let mut marks = self.get_markings();
        marks[5] = mark;
        self.set_markings(&marks)
    }

    fn iv_total(&self) -> usize {
        self.get_iv_hp()
            + self.get_iv_atk()
            + self.get_iv_def()
            + self.get_iv_spe()
            + self.get_iv_spa()
            + self.get_iv_spd()
    }

    fn ev_total(&self) -> usize {
        self.get_ev_hp()
            + self.get_ev_atk()
            + self.get_ev_def()
            + self.get_ev_spe()
            + self.get_ev_spa()
            + self.get_ev_spd()
    }

    fn maximum_iv(&self) -> usize {
        self.get_iv_hp()
            .max(self.get_iv_atk())
            .max(self.get_iv_def())
            .max(self.get_iv_spe())
            .max(self.get_iv_spa())
            .max(self.get_iv_spd())
    }

    fn flawless_iv_count(&self) -> usize {
        let max = self.get_max_iv();
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
        "file".to_string()
    }

    fn get_ivs(&self) -> [usize; 6] {
        [
            self.get_iv_hp(),
            self.get_iv_atk(),
            self.get_iv_def(),
            self.get_iv_spe(),
            self.get_iv_spa(),
            self.get_iv_spd(),
        ]
    }

    fn set_ivs(&mut self, ivs: Vec<usize>) {
        if ivs.len() == 6 {
            self.set_iv_hp(ivs[0]);
            self.set_iv_atk(ivs[1]);
            self.set_iv_def(ivs[2]);
            self.set_iv_spe(ivs[3]);
            self.set_iv_spa(ivs[4]);
            self.set_iv_spd(ivs[5]);
        }
    }

    fn get_evs(&self) -> [usize; 6] {
        [
            self.get_ev_hp(),
            self.get_ev_atk(),
            self.get_ev_def(),
            self.get_ev_spe(),
            self.get_ev_spa(),
            self.get_ev_spd(),
        ]
    }

    fn set_evs(&mut self, evs: Vec<usize>) {
        if evs.len() == 6 {
            self.set_ev_hp(evs[0]);
            self.set_ev_atk(evs[1]);
            self.set_ev_def(evs[2]);
            self.set_ev_spe(evs[3]);
            self.set_ev_spa(evs[4]);
            self.set_ev_spd(evs[5]);
        }
    }

    fn get_stats(&self) -> [usize; 6] {
        [
            self.get_stat_hp_current(),
            self.get_stat_atk(),
            self.get_stat_def(),
            self.get_stat_spe(),
            self.get_stat_spa(),
            self.get_stat_spd(),
        ]
    }

    fn set_stats(&mut self, stats: Vec<usize>) {
        if stats.len() == 6 {
            self.set_stat_hp_current(stats[0]);
            self.set_stat_hp_max(stats[0]);
            self.set_stat_atk(stats[1]);
            self.set_stat_def(stats[2]);
            self.set_stat_spe(stats[3]);
            self.set_stat_spa(stats[4]);
            self.set_stat_spd(stats[5]);
        }
    }

    fn get_moves(&self) -> [usize; 4] {
        [
            self.get_move_1(),
            self.get_move_2(),
            self.get_move_3(),
            self.get_move_4(),
        ]
    }

    fn push_move(&mut self, move_num: usize) {
        if move_num != 0 && move_num < self.get_max_move_id() && !self.has_move(move_num) {
            let mut ct = self.move_count();
            if ct == 4 {
                ct = 0;
            }
            self.set_move(ct, move_num);
            self.heal_pp_index(ct);
        }
    }

    fn move_count(&self) -> usize {
        (self.get_move_1() != 0) as usize
            + (self.get_move_2() != 0) as usize
            + (self.get_move_3() != 0) as usize
            + (self.get_move_4() != 0) as usize
    }

    fn set_moves(&mut self, moves: &[usize]) {
        self.set_move_1(if !moves.is_empty() { moves[0] } else { 0 });
        self.set_move_2(if moves.len() > 1 { moves[1] } else { 0 });
        self.set_move_3(if moves.len() > 2 { moves[2] } else { 0 });
        self.set_move_4(if moves.len() > 3 { moves[3] } else { 0 });
    }

    fn get_relearn_moves(&self) -> [usize; 4] {
        [
            self.get_relearn_move_1(),
            self.get_relearn_move_2(),
            self.get_relearn_move_3(),
            self.get_relearn_move_4(),
        ]
    }

    fn set_relearn_moves(&mut self, moves: Vec<usize>) {
        self.set_relearn_move_1(if !moves.is_empty() { moves[0] } else { 0 });
        self.set_relearn_move_2(if moves.len() > 1 { moves[1] } else { 0 });
        self.set_relearn_move_3(if moves.len() > 2 { moves[2] } else { 0 });
        self.set_relearn_move_4(if moves.len() > 3 { moves[3] } else { 0 });
    }

    fn pid_ability(&self) -> Option<usize> {
        if self.generation() > 5 || self.format() > 5 {
            None
        } else if self.get_version() == GameVersion::CXD as usize {
            self.get_personal_info()
                .get_ability_index(self.get_ability())
        } else {
            Some(
                if self.gen5() {
                    self.get_pid() >> 16
                } else {
                    self.get_pid()
                } & 1,
            )
        }
    }

    fn get_markings(&self) -> Vec<usize> {
        let mut mark = Vec::with_capacity(8);
        for i in 0..8 {
            mark.push((self.get_mark_value() >> i) & 1);
        }
        mark
    }

    fn hp_bit_val_power(&self) -> usize {
        ((self.get_iv_hp() & 2) >> 1)
            | (self.get_iv_atk() & 2)
            | ((self.get_iv_def() & 2) << 1)
            | ((self.get_iv_spe() & 2) << 2)
            | ((self.get_iv_spa() & 2) << 3)
            | ((self.get_iv_spd() & 2) << 4)
    }

    fn get_hp_type(&self) -> usize {
        15 * self.hp_bit_val_type() / 63
    }

    fn set_hp_type(&mut self, hp_type: usize) {
        let arr = DEFAULT_LOW_BITS;
        let bits = if hp_type >= arr.len() {
            0
        } else {
            arr[hp_type] as usize
        };

        self.set_iv_hp((self.get_iv_hp() & !1) + (bits & 1));
        self.set_iv_atk((self.get_iv_atk() & !1) + ((bits >> 1) & 1));
        self.set_iv_def((self.get_iv_def() & !1) + ((bits >> 2) & 1));
        self.set_iv_spe((self.get_iv_spe() & !1) + ((bits >> 3) & 1));
        self.set_iv_spa((self.get_iv_spa() & !1) + ((bits >> 4) & 1));
        self.set_iv_spd((self.get_iv_spd() & !1) + ((bits >> 5) & 1));
    }

    fn was_egg(&self) -> bool {
        self.get_is_egg() || !is_none_location(self.get_version().into(), self.get_egg_location())
    }

    fn was_traded_egg(&self) -> bool {
        self.get_egg_location() == self.get_traded_egg_location()
    }

    fn is_traded_egg(&self) -> bool {
        self.get_met_location() == self.get_traded_egg_location()
    }

    fn get_traded_egg_location(&self) -> usize {
        traded_egg_location(self.generation(), self.get_version().into())
    }

    fn is_untraded(&self) -> bool {
        false
    }

    fn is_native(&self) -> bool {
        self.generation() == self.format()
    }

    fn is_origin_valid(&self) -> bool {
        self.get_species() <= self.get_max_species_id()
    }

    fn inhabited_generation(&self, generation: usize, mut species: usize) -> bool {
        if (species as isize) < 0 {
            species = self.get_species();
        }

        let format = self.format();
        if format == self.generation() {
            true
        } else if !self.is_origin_valid()
            || (species > get_max_species_origin(generation).unwrap_or_default()
                && get_future_gen_evolutions(generation).contains(&species))
        {
            false
        } else if (format == 2 && generation == 1 && !self.korean())
            || (format == 1 && generation == 2)
        {
            //TODO: Check for gen 1 tradeback rule
            true
        } else if format < generation {
            false
        } else {
            let gen = self.generation();
            match gen {
                1 => format == 1 || self.vc(),
                2 => format == 2 || self.vc(),
                3 => self.gen3(),
                4 => (3..=4).contains(&gen),
                5 => (3..=5).contains(&gen),
                6 => (3..=6).contains(&gen),
                7 => (3..=7).contains(&gen) || self.vc(),
                8 => (3..=8).contains(&gen) || self.vc(),
                _ => false,
            }
        }
    }

    fn has_origin_met_location(&self) -> bool {
        !(self.format() < 3
            || self.vc()
            || (self.generation() <= 4 && self.format() != self.generation()))
    }

    fn is_gender_valid(&self) -> bool {
        let gender = self.get_gender();
        let gv = self.get_personal_info().get_gender();
        if gv == RATIO_MAGIC_GENDERLESS {
            gender == 2
        } else if gv == RATIO_MAGIC_FEMALE {
            gender == 1
        } else if gv == RATIO_MAGIC_MALE {
            gender == 0
        } else {
            let gen = self.generation();
            if !((3..6).contains(&gen)) {
                gender == (gender & 1)
            } else {
                gender == get_gender_from_pid_and_ratio(self.get_pid(), gv)
            }
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
            self.set_move_3(self.get_move_4());
            self.set_move_3_pp(self.get_move_4_pp());
            self.set_move_3_pp_ups(self.get_move_4_pp_ups());
            self.set_move_4(0);
        }
    }

    fn refresh_ability(&mut self, n: usize) {
        self.set_ability_number(1 << n);
        let abilities = self.get_personal_info().get_abilities();
        if n < abilities.len() {
            self.set_ability(abilities[n]);
        }
    }

    fn potential_rating(&self) -> usize {
        let total = self.iv_total();
        match total {
            _ if total <= 90 => 0,
            _ if total <= 120 => 1,
            _ if total <= 150 => 2,
            _ => 3,
        }
    }

    fn get_stats_personal_info<I: PersonalInfo>(&self, p: &I) -> [u16; 6] {
        let level = self.get_current_level();
        let mut stats: [u16; 6] = [
            if p.get_hp() == 1 {
                1
            } else {
                ((self.get_iv_hp() + (2 * p.get_hp()) + (self.get_ev_hp() / 4) + 100) * level / 100)
                    + 10
            } as u16,
            (((self.get_iv_atk() + (2 * p.get_atk()) + (self.get_ev_atk() / 4)) * level / 100) + 5)
                as u16,
            (((self.get_iv_def() + (2 * p.get_def()) + (self.get_ev_def() / 4)) * level / 100) + 5)
                as u16,
            (((self.get_iv_spa() + (2 * p.get_spa()) + (self.get_ev_spa() / 4)) * level / 100) + 5)
                as u16,
            (((self.get_iv_spd() + (2 * p.get_spd()) + (self.get_ev_spd() / 4)) * level / 100) + 5)
                as u16,
            (((self.get_iv_spe() + (2 * p.get_spe()) + (self.get_ev_spe() / 4)) * level / 100) + 5)
                as u16,
        ];
        modify_stats_for_nature(&mut stats, self.get_stat_nature());
        stats
    }

    fn get_stats_hyper_train<I: PersonalInfo, P: HyperTrain>(&self, p: &I, t: &P) -> [u16; 6] {
        let level = self.get_current_level();
        let mut stats: [u16; 6] = [
            if p.get_hp() == 1 {
                1
            } else {
                ((if t.get_ht_hp() { 31 } else { self.get_iv_hp() }
                    + (2 * p.get_hp())
                    + (self.get_ev_hp() / 4)
                    + 100)
                    * level
                    / 100)
                    + 10
            } as u16,
            (((if t.get_ht_atk() {
                31
            } else {
                self.get_iv_atk()
            } + (2 * p.get_atk())
                + (self.get_ev_atk() / 4))
                * level
                / 100)
                + 5) as u16,
            (((if t.get_ht_def() {
                31
            } else {
                self.get_iv_def()
            } + (2 * p.get_def())
                + (self.get_ev_def() / 4))
                * level
                / 100)
                + 5) as u16,
            (((if t.get_ht_spa() {
                31
            } else {
                self.get_iv_spa()
            } + (2 * p.get_spa())
                + (self.get_ev_spa() / 4))
                * level
                / 100)
                + 5) as u16,
            (((if t.get_ht_spd() {
                31
            } else {
                self.get_iv_spd()
            } + (2 * p.get_spd())
                + (self.get_ev_spd() / 4))
                * level
                / 100)
                + 5) as u16,
            (((if t.get_ht_spe() {
                31
            } else {
                self.get_iv_spe()
            } + (2 * p.get_spe())
                + (self.get_ev_spe() / 4))
                * level
                / 100)
                + 5) as u16,
        ];
        modify_stats_for_nature(&mut stats, self.get_stat_nature());
        stats
    }

    fn party_stats_present(&self) -> bool {
        self.get_stat_hp_max() != 0
    }

    fn reset_party_stats_personal_info(&mut self) {
        let stats = self.get_stats_personal_info(self.get_personal_info());
        self.set_stats(stats.map(|i| i as usize).to_vec());
        self.set_stat_level(self.get_current_level());
        self.set_status_condition(0);
    }

    fn reset_party_stats_hyper_train<P: HyperTrain + Pkm<T>>(&mut self, p: &P) {
        let stats = self.get_stats_hyper_train(self.get_personal_info(), p);
        self.set_stats(stats.map(|i| i as usize).to_vec());
        self.set_stat_level(self.get_current_level());
        self.set_status_condition(0);
    }

    fn heal(&mut self) {
        self.reset_party_stats_personal_info();
        self.heal_pp();
    }

    fn heal_pp(&mut self) {
        self.set_move_1_pp(self.get_move_pp(self.get_move_1(), self.get_move_1_pp_ups()));
        self.set_move_2_pp(self.get_move_pp(self.get_move_2(), self.get_move_2_pp_ups()));
        self.set_move_3_pp(self.get_move_pp(self.get_move_3(), self.get_move_3_pp_ups()));
        self.set_move_4_pp(self.get_move_pp(self.get_move_4(), self.get_move_4_pp_ups()));
    }

    fn force_party_data(&mut self) -> bool {
        if self.party_stats_present() {
            false
        } else {
            self.reset_party_stats_personal_info();
            false
        }
    }

    fn can_hold_item(&self, valid: &[u16]) -> bool {
        valid.contains(&(self.get_held_item() as u16))
    }

    fn set_link_trade_egg(&mut self, day: usize, month: usize, year: usize, location: usize) {
        self.set_met_day(day);
        self.set_met_month(month);
        self.set_met_year(year - 2000);
        self.set_met_location(location);
    }

    fn hp_power(&self) -> usize {
        if self.format() < 6 {
            ((40 * self.hp_bit_val_power()) / 63) + 30
        } else {
            60
        }
    }

    fn hp_bit_val_type(&self) -> usize {
        (self.get_iv_hp() & 1)
            | ((self.get_iv_atk() & 1) << 1)
            | ((self.get_iv_def() & 1) << 2)
            | ((self.get_iv_spe() & 1) << 3)
            | ((self.get_iv_spa() & 1) << 4)
            | ((self.get_iv_spd() & 1) << 5)
    }

    fn set_markings(&mut self, markings: &[usize]) {
        if markings.len() <= 8 {
            let mut b: usize = 0;
            for (i, marking) in markings.iter().enumerate() {
                b |= (marking.min(&1) << i) & 0xFF
            }
            self.set_mark_value(b);
        }
    }

    fn heal_pp_index(&mut self, index: usize) {
        match index {
            0 => self.set_move_1_pp(self.get_move_pp(self.get_move_1(), self.get_move_1_pp_ups())),
            1 => self.set_move_2_pp(self.get_move_pp(self.get_move_2(), self.get_move_2_pp_ups())),
            2 => self.set_move_3_pp(self.get_move_pp(self.get_move_3(), self.get_move_3_pp_ups())),
            3 => self.set_move_4_pp(self.get_move_pp(self.get_move_4(), self.get_move_4_pp_ups())),
            _ => {}
        }
    }

    fn get_move_pp(&self, move_num: usize, pp_up_count: usize) -> usize {
        self.get_base_pp(move_num) * (5 + pp_up_count) / 5
    }

    fn get_base_pp(&self, mut move_num: usize) -> usize {
        let table = get_pp_table(self, self.format());
        if move_num >= table.len() {
            move_num = 0;
        }
        table[move_num] as usize
    }

    fn set_shiny(&mut self) {
        let mut rand = rand::thread_rng();
        self.set_pid(get_random_pid(
            &mut rand,
            self.get_species(),
            self.get_gender(),
            self.get_version(),
            self.get_nature(),
            self.get_form(),
            self.get_pid(),
        ));
        while !self.is_shiny() {
            self.set_pid(get_random_pid(
                &mut rand,
                self.get_species(),
                self.get_gender(),
                self.get_version(),
                self.get_nature(),
                self.get_form(),
                self.get_pid(),
            ));
        }
        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_shiny_sid(&mut self, shiny: ShinyEnum) {
        if self.is_shiny() && shiny.is_valid(self) {
            return;
        }

        let xor = self.get_tid() ^ (self.get_pid() >> 16) ^ (self.get_pid() & 0xFFFF);
        let bits = match shiny {
            ShinyEnum::AlwaysSquare => 0,
            ShinyEnum::AlwaysStar => 1,
            _ => rand::thread_rng().gen_range(0..8),
        };

        self.set_sid(xor ^ bits);
    }

    fn set_pid_gender(&mut self, gender: usize) {
        let mut rand = rand::thread_rng();
        self.set_pid(get_random_pid(
            &mut rand,
            self.get_species(),
            gender,
            self.get_version(),
            self.get_nature(),
            self.get_form(),
            self.get_pid(),
        ));
        while self.is_shiny() {
            self.set_pid(get_random_pid(
                &mut rand,
                self.get_species(),
                gender,
                self.get_version(),
                self.get_nature(),
                self.get_form(),
                self.get_pid(),
            ));
        }
        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_pid_nature(&mut self, nature: usize) {
        let mut rand = rand::thread_rng();
        self.set_pid(get_random_pid(
            &mut rand,
            self.get_species(),
            self.get_gender(),
            self.get_version(),
            nature,
            self.get_form(),
            self.get_pid(),
        ));
        while self.is_shiny() {
            self.set_pid(get_random_pid(
                &mut rand,
                self.get_species(),
                self.get_gender(),
                self.get_version(),
                nature,
                self.get_form(),
                self.get_pid(),
            ));
        }
        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_pid_unown_3(&mut self, form: usize) {
        let mut rand = rand::thread_rng();
        self.set_pid(rand.next_u32() as usize);
        while get_unown_form(self.get_pid()) != form {
            self.set_pid(rand.next_u32() as usize);
        }
        if self.format() >= 6 && (self.gen3() || self.gen4() || self.gen5()) {
            self.set_encryption_constant(self.get_pid());
        }
    }

    fn set_random_ivs(&mut self, flawless: Option<usize>) -> [usize; 6] {
        if self.get_version() == GameVersion::GO as usize && flawless.unwrap_or_default() == 6 {
            return self.set_random_ivs_go(0, 5);
        }
        let mut ivs = [0, 0, 0, 0, 0, 0];
        let mut rand = rand::thread_rng();
        for iv in ivs.iter_mut() {
            *iv = rand.gen_range(0..(self.get_max_iv() + 1));
        }

        let count = flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        if count != 0 {
            for i in ivs.iter_mut().take(count) {
                *i = self.get_max_iv();
            }
            let len = ivs.len();
            rand_util::shuffle(&mut ivs, 0, len, &mut rand);
        }
        ivs
    }

    fn set_random_ivs_go(&mut self, min_iv: usize, max_iv: usize) -> [usize; 6] {
        let mut ivs = [0, 0, 0, 0, 0, 0];
        let mut rand = rand::thread_rng();
        ivs[0] = ((rand.gen_range(min_iv..(max_iv + 1)) << 1) | 1) as usize;
        ivs[1] = ((rand.gen_range(min_iv..(max_iv + 1)) << 1) | 1) as usize;
        ivs[4] = ivs[1];
        ivs[2] = ((rand.gen_range(min_iv..(max_iv + 1)) << 1) | 1) as usize;
        ivs[5] = ivs[2];
        ivs[3] = rand.gen_range(0..(max_iv + 1)) as usize;
        self.set_ivs(ivs.to_vec());

        ivs
    }

    fn set_random_ivs_from_template(
        &mut self,
        template: &[Option<usize>],
        flawless: Option<usize>,
    ) -> [usize; 6] {
        let count = flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        let mut ivs = [0, 0, 0, 0, 0, 0];
        let mut rand = rand::thread_rng();
        for i in 0..6 {
            ivs[i] = *template[i]
                .as_ref()
                .unwrap_or(&rand.gen_range(0..(self.get_max_iv() + 1)));
        }
        while ivs.iter().filter(|&&z| z == self.get_max_iv()).count() < count {
            for i in 0..6 {
                ivs[i] = *template[i]
                    .as_ref()
                    .unwrap_or(&rand.gen_range(0..(self.get_max_iv() + 1)));
            }
        }
        self.set_ivs(ivs.to_vec());
        ivs
    }

    fn get_flawless_iv_count(&self) -> usize {
        if self.generation() >= 6
            && (LEGENDS.contains(&self.get_species()) || SUB_LEGENDS.contains(&self.get_species()))
        {
            3
        } else if self.xy() {
            if self.get_personal_info().get_egg_group_1() == 15 {
                3
            } else if self.get_met_location() == 148 && self.get_met_level() == 30 {
                2
            } else {
                0
            }
        } else if self.vc() {
            if self.get_species() == Species::Mew as usize
                || self.get_species() == Species::Celebi as usize
            {
                5
            } else {
                3
            }
        } else {
            0
        }
    }

    fn has_move(&self, move_num: usize) -> bool {
        self.get_move_1() == move_num
            || self.get_move_2() == move_num
            || self.get_move_3() == move_num
            || self.get_move_4() == move_num
    }

    fn get_move(&self, index: usize) -> usize {
        match index {
            0 => self.get_move_1(),
            1 => self.get_move_2(),
            2 => self.get_move_3(),
            3 => self.get_move_4(),
            _ => 0,
        }
    }

    fn set_move(&mut self, index: usize, value: usize) {
        match index {
            0 => self.set_move_1(value),
            1 => self.set_move_2(value),
            2 => self.set_move_3(value),
            3 => self.set_move_4(value),
            _ => {}
        }
    }

    fn clear_invalid_moves(&mut self) {
        let mut invalid = 0;
        let mut moves = self.get_moves();
        for move_num in moves.iter_mut() {
            if *move_num <= self.get_max_move_id() {
                continue;
            }

            invalid += 1;
            *move_num = 0;
        }
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

    fn get_ev(&self, index: usize) -> usize {
        match index {
            0 => self.get_ev_hp(),
            1 => self.get_ev_atk(),
            2 => self.get_ev_def(),
            3 => self.get_ev_spe(),
            4 => self.get_ev_spa(),
            5 => self.get_ev_spd(),
            _ => 0,
        }
    }

    fn get_iv(&self, index: usize) -> usize {
        match index {
            0 => self.get_iv_hp(),
            1 => self.get_iv_atk(),
            2 => self.get_iv_def(),
            3 => self.get_iv_spe(),
            4 => self.get_iv_spa(),
            5 => self.get_iv_spd(),
            _ => 0,
        }
    }
}
