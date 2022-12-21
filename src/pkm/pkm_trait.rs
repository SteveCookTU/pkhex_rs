use crate::editing::{hidden_power, nature_amp};
use crate::game::enums::{GameVersion, LanguageID, Species};
use crate::game::locations;
use crate::legality::{IndividualValueSet, MoveSetData};
use crate::moves::move_info;
use crate::personal_info::traits::gender_detail::GenderDetail;
use crate::personal_info::traits::personal_info::PersonalInfo;
use crate::personal_info::traits::{gender_detail, BaseStat, PersonalAbility, PersonalEgg};
use crate::pkm::shared::EntityContext;
use crate::pkm::traits::metadata::{GameValueLimit, Generation, LangNick, Shiny, SpeciesForm};
use crate::pkm::traits::{HyperTrain, Nature, TrainerIDPKM};
use crate::pkm::util::{entity_gender, entity_pid, experience};
use crate::{legality, rand_util, PKError, PKResult};
use chrono::{Datelike, NaiveDate};
use rand::{thread_rng, Rng};

fn write<T: PersonalInfo>(pkm: &mut dyn Pkm<PersonalInfoOutput = T>) -> Vec<u8> {
    pkm.refresh_checksum();
    pkm.data().to_vec()
}

fn set_id_7<T: PersonalInfo>(pkm: &mut dyn Pkm<PersonalInfoOutput = T>, sid: u32, tid: u32) {
    let oid = (sid * 1000000) + (tid % 1000000);
    pkm.set_tid(oid as u16);
    pkm.set_tid((oid >> 16) as u16);
}

fn get_base_pp<T: PersonalInfo>(pkm: &dyn Pkm<PersonalInfoOutput = T>, mov: u16) -> u8 {
    move_info::get_pp(pkm.context(), mov).unwrap_or_default()
}

fn hp_bit_val_power<T: PersonalInfo>(pkm: &dyn Pkm<PersonalInfoOutput = T>) -> u16 {
    (u16::from(pkm.iv_hp() & 2) >> 1)
        | u16::from(pkm.iv_atk() & 2)
        | (u16::from(pkm.iv_def() & 2) << 1)
        | (u16::from(pkm.iv_spe() & 2) << 2)
        | (u16::from(pkm.iv_spa() & 2) << 3)
        | (u16::from(pkm.iv_spd() & 2) << 4)
}

fn hp_bit_val_type<T: PersonalInfo>(pkm: &dyn Pkm<PersonalInfoOutput = T>) -> u8 {
    (pkm.iv_hp() & 1)
        | ((pkm.iv_atk() & 1) << 1)
        | ((pkm.iv_def() & 1) << 2)
        | ((pkm.iv_spe() & 1) << 3)
        | ((pkm.iv_spa() & 1) << 4)
        | ((pkm.iv_spd() & 1) << 5)
}

fn get_traded_egg_location<T: PersonalInfo>(pkm: &dyn Pkm<PersonalInfoOutput = T>) -> u16 {
    locations::traded_egg_location(pkm.generation(), pkm.version().into())
}

fn reorder_moves<T: PersonalInfo>(pkm: &mut dyn Pkm<PersonalInfoOutput = T>) {
    if pkm.move_1() == 0 && pkm.move_2() != 0 {
        pkm.set_move_1(pkm.move_2());
        pkm.set_move_1_pp(pkm.move_2_pp());
        pkm.set_move_1_pp_ups(pkm.move_2_pp_ups());
        pkm.set_move_2(0);
    }
    if pkm.move_2() == 0 && pkm.move_3() != 0 {
        pkm.set_move_2(pkm.move_3());
        pkm.set_move_2_pp(pkm.move_3_pp());
        pkm.set_move_2_pp_ups(pkm.move_3_pp_ups());
        pkm.set_move_3(0);
    }
    if pkm.move_3() == 0 && pkm.move_4() != 0 {
        pkm.set_move_3(pkm.move_4());
        pkm.set_move_3_pp(pkm.move_4_pp());
        pkm.set_move_3_pp_ups(pkm.move_4_pp_ups());
        pkm.set_move_4(0);
    }
}

fn is_display_7<T: PersonalInfo>(pkm: &dyn Pkm<PersonalInfoOutput = T>) -> bool {
    match pkm.generation() {
        i if i >= 7 => true,
        i if i == 0 && pkm.context() == EntityContext::Gen9 => true,
        _ => false,
    }
}

pub trait Pkm:
    SpeciesForm + TrainerIDPKM + Generation + Shiny + LangNick + GameValueLimit + Nature
{
    type PersonalInfoOutput: PersonalInfo;

    fn size_party(&self) -> usize;
    fn size_stored(&self) -> usize;
    fn extension(&self) -> &str;
    fn personal_info(&self) -> &Self::PersonalInfoOutput;
    fn extra_bytes(&self) -> Vec<u16> {
        vec![]
    }

    fn data(&self) -> &[u8];

    fn encrypted_party_data(&mut self) -> Vec<u8> {
        self.encrypt()[..self.size_party()].to_vec()
    }
    fn encrypted_box_data(&mut self) -> Vec<u8> {
        self.encrypt()[..self.size_stored()].to_vec()
    }
    fn decrypted_party_data(&mut self) -> Vec<u8>
    where
        Self: Sized,
    {
        write(self)[..self.size_party()].to_vec()
    }
    fn decrypted_box_data(&mut self) -> Vec<u8>
    where
        Self: Sized,
    {
        write(self)[..self.size_stored()].to_vec()
    }

    fn valid(&self) -> bool;
    fn set_valid(&mut self, valid: bool);

    fn nickname_trash(&self) -> &[u8];
    fn nickname_trash_mut(&mut self) -> &mut [u8];
    fn ot_trash(&self) -> &[u8];
    fn ot_trash_mut(&mut self) -> &mut [u8];
    fn ht_trash(&self) -> &[u8] {
        &[]
    }
    fn ht_trash_mut(&mut self) -> &mut [u8] {
        &mut []
    }

    fn encrypt(&mut self) -> Vec<u8>;
    fn context(&self) -> EntityContext;
    fn format(&self) -> Option<u8> {
        self.context().generation().ok()
    }

    fn set_species(&mut self, species: u16);
    fn set_nickname(&mut self, nickname: &str);
    fn held_item(&self) -> u16;
    fn set_held_item(&mut self, item: u16);
    fn gender(&self) -> u8;
    fn set_gender(&mut self, gender: u8);
    fn stat_nature(&self) -> u8 {
        self.nature()
    }
    fn set_stat_nature(&mut self, nature: u8) {
        self.set_nature(nature);
    }
    fn ability(&self) -> u16;
    fn set_ability(&mut self, ability: u16);
    fn current_friendship(&self) -> u8;
    fn set_current_friendship(&mut self, friendship: u8);
    fn set_form(&mut self, form: u8);
    fn is_egg(&self) -> bool;
    fn set_is_egg(&mut self, egg: bool);
    fn set_is_nicknamed(&mut self, nicknamed: bool);
    fn exp(&self) -> u32;
    fn set_exp(&mut self, exp: u32);
    fn ot_name(&self) -> String;
    fn set_ot_name(&mut self, name: &str);
    fn ot_gender(&self) -> u8;
    fn set_ot_gender(&mut self, gender: u8);
    fn ball(&self) -> u8;
    fn set_ball(&mut self, ball: u8);
    fn met_level(&self) -> u8;
    fn set_met_level(&mut self, level: u8);

    fn move_1(&self) -> u16;
    fn set_move_1(&mut self, mov: u16);
    fn move_2(&self) -> u16;
    fn set_move_2(&mut self, mov: u16);
    fn move_3(&self) -> u16;
    fn set_move_3(&mut self, mov: u16);
    fn move_4(&self) -> u16;
    fn set_move_4(&mut self, mov: u16);
    fn move_1_pp(&self) -> u8;
    fn set_move_1_pp(&mut self, pp: u8);
    fn move_2_pp(&self) -> u8;
    fn set_move_2_pp(&mut self, pp: u8);
    fn move_3_pp(&self) -> u8;
    fn set_move_3_pp(&mut self, pp: u8);
    fn move_4_pp(&self) -> u8;
    fn set_move_4_pp(&mut self, pp: u8);
    fn move_1_pp_ups(&self) -> u8;
    fn set_move_1_pp_ups(&mut self, ups: u8);
    fn move_2_pp_ups(&self) -> u8;
    fn set_move_2_pp_ups(&mut self, ups: u8);
    fn move_3_pp_ups(&self) -> u8;
    fn set_move_3_pp_ups(&mut self, ups: u8);
    fn move_4_pp_ups(&self) -> u8;
    fn set_move_4_pp_ups(&mut self, ups: u8);
    fn ev_hp(&self) -> u8;
    fn set_ev_hp(&mut self, ev: u8);
    fn ev_atk(&self) -> u8;
    fn set_ev_atk(&mut self, ev: u8);
    fn ev_def(&self) -> u8;
    fn set_ev_def(&mut self, ev: u8);
    fn ev_spe(&self) -> u8;
    fn set_ev_spe(&mut self, ev: u8);
    fn ev_spa(&self) -> u8;
    fn set_ev_spa(&mut self, ev: u8);
    fn ev_spd(&self) -> u8;
    fn set_ev_spd(&mut self, ev: u8);
    fn iv_hp(&self) -> u8;
    fn set_iv_hp(&mut self, iv: u8);
    fn iv_atk(&self) -> u8;
    fn set_iv_atk(&mut self, iv: u8);
    fn iv_def(&self) -> u8;
    fn set_iv_def(&mut self, iv: u8);
    fn iv_spe(&self) -> u8;
    fn set_iv_spe(&mut self, iv: u8);
    fn iv_spa(&self) -> u8;
    fn set_iv_spa(&mut self, iv: u8);
    fn iv_spd(&self) -> u8;
    fn set_iv_spd(&mut self, iv: u8);
    fn status_condition(&self) -> u32;
    fn set_status_condition(&mut self, status: u32);
    fn stat_level(&self) -> u8;
    fn set_stat_level(&mut self, level: u8);
    fn stat_hp_max(&self) -> u16;
    fn set_stat_hp_max(&mut self, max: u16);
    fn stat_hp_current(&self) -> u16;
    fn set_stat_hp_current(&mut self, current: u16);
    fn stat_atk(&self) -> u16;
    fn set_stat_atk(&mut self, stat: u16);
    fn stat_def(&self) -> u16;
    fn set_stat_def(&mut self, stat: u16);
    fn stat_spe(&self) -> u16;
    fn set_stat_spe(&mut self, stat: u16);
    fn stat_spa(&self) -> u16;
    fn set_stat_spa(&mut self, stat: u16);
    fn stat_spd(&self) -> u16;
    fn set_stat_spd(&mut self, stat: u16);

    fn version(&self) -> u8;
    fn set_version(&mut self, version: u8);
    fn pkrs_strain(&self) -> u8;
    fn set_pkrs_strain(&mut self, strain: u8);
    fn pkrs_days(&self) -> u8;
    fn set_pkrs_days(&mut self, days: u8);

    fn encryption_constant(&self) -> u32;
    fn set_encryption_constant(&mut self, ec: u32);
    fn pid(&self) -> u32;
    fn set_pid(&mut self, pid: u32);

    fn set_language(&mut self, lang: u8);
    fn fateful_encounter(&self) -> bool;
    fn set_fateful_encounter(&mut self, fateful: bool);
    fn characteristic(&self) -> u8;
    fn mark_value(&self) -> u16;
    fn set_mark_value(&mut self, value: u16);
    fn met_location(&self) -> u16;
    fn set_met_location(&mut self, location: u16);
    fn egg_location(&self) -> u16;
    fn set_egg_location(&mut self, location: u16);
    fn ot_friendship(&self) -> u8;
    fn set_ot_friendship(&mut self, friendship: u8);
    fn japanese(&self) -> bool {
        self.language() == LanguageID::Japanese as u8
    }
    fn korean(&self) -> bool {
        self.language() == LanguageID::Korean as u8
    }

    fn met_year(&self) -> u16 {
        0
    }
    fn set_met_year(&mut self, _year: u16) {}
    fn met_month(&self) -> u8 {
        0
    }
    fn set_met_month(&mut self, _month: u8) {}
    fn met_day(&self) -> u8 {
        0
    }
    fn set_met_day(&mut self, _day: u8) {}
    fn ht_name(&self) -> String {
        String::new()
    }
    fn set_ht_name(&mut self, _name: &str) {}
    fn ht_gender(&self) -> u8 {
        0
    }
    fn set_ht_gender(&mut self, _gender: u8) {}
    fn ht_friendship(&self) -> u8 {
        0
    }
    fn set_ht_friendship(&mut self, _friendship: u8) {}
    fn enjoyment(&self) -> u8 {
        0
    }
    fn set_enjoyment(&mut self, _enjoyment: u8) {}
    fn fullness(&self) -> u8 {
        0
    }
    fn set_fullness(&mut self, _fullness: u8) {}
    fn ability_number(&self) -> u8 {
        0
    }
    fn set_ability_number(&mut self, _number: u8) {}
    fn met_date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            (2000 + self.met_year()) as i32,
            self.met_month() as u32,
            self.met_day() as u32,
        )
    }
    fn set_met_date(&mut self, date: Option<NaiveDate>) {
        if let Some(date) = date {
            self.set_met_year((date.year() as u16).saturating_sub(2000));
            self.set_met_month(date.month() as u8);
            self.set_met_day(date.day() as u8);
        } else {
            self.set_met_year(0);
            self.set_met_month(0);
            self.set_met_day(0);
        }
    }

    fn egg_year(&self) -> u16 {
        0
    }
    fn set_egg_year(&mut self, _year: u16) {}
    fn egg_month(&self) -> u8 {
        0
    }
    fn set_egg_month(&mut self, _month: u8) {}
    fn egg_day(&self) -> u8 {
        0
    }
    fn set_egg_day(&mut self, _day: u8) {}

    fn egg_date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            (2000 + self.egg_year()) as i32,
            self.egg_month() as u32,
            self.egg_day() as u32,
        )
    }
    fn set_egg_date(&mut self, date: Option<NaiveDate>) {
        if let Some(date) = date {
            self.set_egg_year((date.year() as u16).saturating_sub(2000));
            self.set_egg_month(date.month() as u8);
            self.set_egg_day(date.day() as u8);
        } else {
            self.set_egg_year(0);
            self.set_egg_month(0);
            self.set_egg_day(0);
        }
    }

    fn relearn_move_1(&self) -> u16 {
        0
    }
    fn set_relearn_move_1(&mut self, _mov: u16) {}
    fn relearn_move_2(&self) -> u16 {
        0
    }
    fn set_relearn_move_2(&mut self, _mov: u16) {}
    fn relearn_move_3(&self) -> u16 {
        0
    }
    fn set_relearn_move_3(&mut self, _mov: u16) {}
    fn relearn_move_4(&self) -> u16 {
        0
    }
    fn set_relearn_move_4(&mut self, _mov: u16) {}

    fn current_handler(&self) -> u8;
    fn set_current_handler(&mut self, handler: u8);

    fn sprite_item(&self) -> u16 {
        self.held_item()
    }
    fn trainer_id_7(&self) -> u32 {
        (u32::from(self.tid()) | (u32::from(self.sid()) << 16)) % 1000000
    }

    fn set_trainer_id_7(&mut self, tid: u32)
    where
        Self: Sized,
    {
        let sid = self.trainer_sid_7();
        set_id_7(self, sid, tid);
    }

    fn trainer_sid_7(&self) -> u32 {
        (u32::from(self.tid()) | (u32::from(self.sid()) << 16)) / 1000000
    }

    fn set_trainer_sid_7(&mut self, sid: u32)
    where
        Self: Sized,
    {
        let tid = self.trainer_sid_7();
        set_id_7(self, sid, tid);
    }

    fn shiny_xor(&self) -> u16 {
        let pid = self.pid();
        let upper = ((pid >> 16) as u16) ^ self.sid();
        ((pid & 0xFFFF) as u16) ^ self.tid() ^ upper
    }

    fn display_tid(&self) -> u32
    where
        Self: Sized,
    {
        if is_display_7(self) {
            self.trainer_id_7()
        } else {
            self.tid() as u32
        }
    }

    fn set_display_tid(&mut self, tid: u32)
    where
        Self: Sized,
    {
        if is_display_7(self) {
            self.set_trainer_id_7(tid)
        } else {
            self.set_tid(tid as u16)
        }
    }

    fn display_sid(&self) -> u32
    where
        Self: Sized,
    {
        if is_display_7(self) {
            self.trainer_sid_7()
        } else {
            self.sid() as u32
        }
    }

    fn set_display_sid(&mut self, sid: u32)
    where
        Self: Sized,
    {
        if is_display_7(self) {
            self.set_trainer_sid_7(sid)
        } else {
            self.set_sid(sid as u16)
        }
    }

    fn e(&self) -> bool {
        self.version() == GameVersion::E as u8
    }
    fn frlg(&self) -> bool {
        [GameVersion::FR as u8, GameVersion::LG as u8].contains(&self.version())
    }
    fn pt(&self) -> bool {
        self.version() == GameVersion::Pt as u8
    }
    fn hgss(&self) -> bool {
        [GameVersion::HG as u8, GameVersion::SS as u8].contains(&self.version())
    }
    fn bw(&self) -> bool {
        [GameVersion::B as u8, GameVersion::W as u8].contains(&self.version())
    }
    fn b2w2(&self) -> bool {
        [GameVersion::B2 as u8, GameVersion::W2 as u8].contains(&self.version())
    }
    fn xy(&self) -> bool {
        [GameVersion::X as u8, GameVersion::Y as u8].contains(&self.version())
    }
    fn ao(&self) -> bool {
        [GameVersion::AS as u8, GameVersion::OR as u8].contains(&self.version())
    }
    fn sm(&self) -> bool {
        [GameVersion::SN as u8, GameVersion::MN as u8].contains(&self.version())
    }
    fn usum(&self) -> bool {
        [GameVersion::US as u8, GameVersion::UM as u8].contains(&self.version())
    }
    fn go(&self) -> bool {
        self.version() == GameVersion::GO as u8
    }
    fn vc1(&self) -> bool {
        ((GameVersion::RD as u8)..=(GameVersion::YW as u8)).contains(&self.version())
    }
    fn vc2(&self) -> bool {
        ((GameVersion::GD as u8)..=(GameVersion::C as u8)).contains(&self.version())
    }
    fn lgpe(&self) -> bool {
        [GameVersion::GP as u8, GameVersion::GE as u8].contains(&self.version())
    }
    fn swsh(&self) -> bool {
        [GameVersion::SW as u8, GameVersion::SH as u8].contains(&self.version())
    }
    fn bdsp(&self) -> bool {
        [GameVersion::BD as u8, GameVersion::SP as u8].contains(&self.version())
    }
    fn la(&self) -> bool {
        self.version() == GameVersion::PLA as u8
    }
    fn sv(&self) -> bool {
        [GameVersion::SL as u8, GameVersion::VL as u8].contains(&self.version())
    }

    fn go_lgpe(&self) -> bool {
        self.go() && self.met_location() == locations::GO_7
    }
    fn go_home(&self) -> bool {
        self.go() && self.met_location() == locations::GO_8
    }
    fn vc(&self) -> bool {
        self.vc1() || self.vc2()
    }
    fn gg(&self) -> bool {
        self.lgpe() || self.go_lgpe()
    }
    fn gen_9(&self) -> bool {
        self.sv()
    }
    fn gen_8(&self) -> bool {
        (44..=49).contains(&self.version()) || self.go_home()
    }
    fn gen_7(&self) -> bool {
        (30..=33).contains(&self.version()) || self.gg()
    }
    fn gen_6(&self) -> bool {
        (24..=29).contains(&self.version())
    }
    fn gen_5(&self) -> bool {
        (20..=23).contains(&self.version())
    }
    fn gen_4(&self) -> bool {
        (7..=12).contains(&self.version()) && self.version() != 9
    }
    fn gen_3(&self) -> bool {
        (1..=5).contains(&self.version()) || self.version() == 15
    }
    fn gen_2(&self) -> bool {
        self.version() == GameVersion::GSC as u8
    }
    fn gen_1(&self) -> bool {
        self.version() == GameVersion::RBY as u8
    }
    fn gen_u(&self) -> bool {
        self.generation() == 0
    }

    fn pkrs_infected(&self) -> bool {
        self.pkrs_strain() != 0
    }

    fn set_pkrs_infected(&mut self, infected: bool) {
        let strain = self.pkrs_strain();
        self.set_pkrs_days(if infected { strain.max(1) } else { 0 });
    }

    fn pkrs_cursed(&self) -> bool {
        self.pkrs_days() == 0 && self.pkrs_strain() > 0
    }

    fn set_pkrs_cursed(&mut self, cursed: bool) {
        self.set_pkrs_days(u8::from(cursed));
        self.set_pkrs_infected(true);
    }

    fn current_level(&self) -> u8 {
        experience::get_level(self.exp(), self.personal_info().exp_growth())
    }
    fn set_current_level(&mut self, level: u8) {
        let growth = self.personal_info().exp_growth();
        self.set_exp(experience::get_exp(level, growth));
        self.set_stat_level(level);
    }

    fn iv_total(&self) -> u16 {
        u16::from(self.iv_hp())
            + u16::from(self.iv_atk())
            + u16::from(self.iv_def())
            + u16::from(self.iv_spa())
            + u16::from(self.iv_spd())
            + u16::from(self.iv_spe())
    }

    fn ev_total(&self) -> u16 {
        u16::from(self.ev_hp())
            + u16::from(self.ev_atk())
            + u16::from(self.ev_def())
            + u16::from(self.ev_spa())
            + u16::from(self.ev_spd())
            + u16::from(self.ev_spe())
    }

    fn maximum_iv(&self) -> u8 {
        self.iv_hp().max(
            self.iv_atk().max(
                self.iv_def()
                    .max(self.iv_spa().max(self.iv_spd().max(self.iv_spe()))),
            ),
        )
    }

    fn flawless_iv_count(&self) -> u8 {
        [
            self.iv_hp(),
            self.iv_atk(),
            self.iv_def(),
            self.iv_spa(),
            self.iv_spd(),
            self.iv_spe(),
        ]
        .into_iter()
        .filter(|&i| i == self.max_iv())
        .count() as u8
    }

    // TODO: FilenameWithoutExtension

    fn ivs(&self) -> [u8; 6] {
        [
            self.iv_hp(),
            self.iv_atk(),
            self.iv_def(),
            self.iv_spe(),
            self.iv_spa(),
            self.iv_spd(),
        ]
    }
    fn set_ivs(&mut self, ivs: &[u8]) {
        if ivs.len() != 6 {
            return;
        }
        self.set_iv_hp(ivs[0]);
        self.set_iv_atk(ivs[1]);
        self.set_iv_def(ivs[2]);
        self.set_iv_spe(ivs[3]);
        self.set_iv_spa(ivs[4]);
        self.set_iv_spd(ivs[5]);
    }

    fn evs(&self) -> [u8; 6] {
        [
            self.ev_hp(),
            self.ev_atk(),
            self.ev_def(),
            self.ev_spe(),
            self.ev_spa(),
            self.ev_spd(),
        ]
    }
    fn set_evs(&mut self, evs: &[u8]) {
        if evs.len() != 6 {
            return;
        }
        self.set_ev_hp(evs[0]);
        self.set_ev_atk(evs[1]);
        self.set_ev_def(evs[2]);
        self.set_ev_spe(evs[3]);
        self.set_ev_spa(evs[4]);
        self.set_ev_spd(evs[5]);
    }

    fn stats(&self) -> [u16; 6] {
        [
            self.stat_hp_current(),
            self.stat_atk(),
            self.stat_def(),
            self.stat_spe(),
            self.stat_spa(),
            self.stat_spd(),
        ]
    }
    fn set_stats(&mut self, stats: &[u16]) {
        if stats.len() != 6 {
            return;
        }
        self.set_stat_hp_current(stats[0]);
        self.set_stat_atk(stats[1]);
        self.set_stat_def(stats[2]);
        self.set_stat_spe(stats[3]);
        self.set_stat_spa(stats[4]);
        self.set_stat_spd(stats[5]);
    }

    fn moves(&self) -> [u16; 4] {
        [self.move_1(), self.move_2(), self.move_3(), self.move_4()]
    }
    fn push_move(&mut self, mov: u16)
    where
        Self: Sized,
    {
        if mov == 0 || mov >= self.max_move_id() || self.has_move(mov) {
            return;
        }
        let mut ct = self.move_count();
        if ct == 4 {
            ct = 0;
        }
        self.set_move(ct, mov).unwrap();
        self.heal_pp_index(ct).unwrap();
    }
    fn move_count(&self) -> u8 {
        u8::from(self.move_1() != 0)
            + u8::from(self.move_2() != 0)
            + u8::from(self.move_3() != 0)
            + u8::from(self.move_4() != 0)
    }
    fn set_moves(&mut self, moves: &[u16]) {
        self.set_move_1(*moves.first().unwrap_or(&0));
        self.set_move_2(*moves.get(1).unwrap_or(&0));
        self.set_move_3(*moves.get(2).unwrap_or(&0));
        self.set_move_4(*moves.get(3).unwrap_or(&0));
    }
    fn set_moves_from_set(&mut self, set: MoveSetData) {
        self.set_move_1(set.move_1);
        self.set_move_2(set.move_2);
        self.set_move_3(set.move_3);
        self.set_move_4(set.move_4);
    }

    fn relearn_moves(&self) -> [u16; 4] {
        [
            self.relearn_move_1(),
            self.relearn_move_2(),
            self.relearn_move_3(),
            self.relearn_move_4(),
        ]
    }
    fn set_relearn_moves(&mut self, moves: &[u16]) {
        self.set_relearn_move_1(*moves.first().unwrap_or(&0));
        self.set_relearn_move_2(*moves.get(1).unwrap_or(&0));
        self.set_relearn_move_3(*moves.get(2).unwrap_or(&0));
        self.set_relearn_move_4(*moves.get(3).unwrap_or(&0));
    }
    fn set_relearn_moves_from_set(&mut self, set: MoveSetData) {
        self.set_relearn_move_1(set.move_1);
        self.set_relearn_move_2(set.move_2);
        self.set_relearn_move_3(set.move_3);
        self.set_relearn_move_4(set.move_4);
    }

    fn pid_ability(&self) -> Option<u8> {
        if self.generation() > 5 || self.format().unwrap_or_default() > 5 {
            return None;
        }
        if self.version() == GameVersion::CXD as u8 {
            return self.personal_info().get_index_of_ability(self.ability());
        }
        Some(
            (if self.gen_5() {
                self.pid() >> 16
            } else {
                self.pid()
            } & 1) as u8,
        )
    }

    fn marking_count(&self) -> u8;
    fn get_marking(&self, index: usize) -> u8;
    fn set_marking(&mut self, index: usize, value: u8);

    fn hp_power(&self) -> u16
    where
        Self: Sized,
    {
        if self.format().unwrap_or_default() < 6 {
            ((40 * hp_bit_val_power(self)) / 63) + 30
        } else {
            60
        }
    }

    fn hp_type(&self) -> u8
    where
        Self: Sized,
    {
        15 * hp_bit_val_type(self) / 63
    }

    fn set_hp_type(&mut self, t: u8) {
        let arr = &hidden_power::DEFAULT_LOW_BITS;
        let bits = if t as usize > arr.len() {
            0
        } else {
            arr[t as usize]
        };
        let iv_hp = (self.iv_hp() & !1) + (bits & 1);
        let iv_atk = (self.iv_atk() & !1) + ((bits >> 1) & 1);
        let iv_def = (self.iv_def() & !1) + ((bits >> 2) & 1);
        let iv_spe = (self.iv_spe() & !1) + ((bits >> 3) & 1);
        let iv_spa = (self.iv_spa() & !1) + ((bits >> 4) & 1);
        let iv_spd = (self.iv_spd() & !1) + ((bits >> 5) & 1);
        self.set_iv_hp(iv_hp);
        self.set_iv_atk(iv_atk);
        self.set_iv_def(iv_def);
        self.set_iv_spe(iv_spe);
        self.set_iv_spe(iv_spa);
        self.set_iv_spd(iv_spd);
    }

    fn was_egg(&self) -> bool {
        self.is_egg() || self.egg_day() != 0
    }
    fn was_traded_egg(&self) -> bool
    where
        Self: Sized,
    {
        self.egg_location() == get_traded_egg_location(self)
    }
    fn is_traded_egg(&self) -> bool
    where
        Self: Sized,
    {
        self.met_location() == get_traded_egg_location(self)
    }

    fn is_untraded(&self) -> bool {
        false
    }
    fn is_native(&self) -> bool {
        self.generation() == self.format().unwrap_or_default()
    }
    fn is_origin_valid(&self) -> bool {
        self.species() <= self.max_species_id()
    }

    fn has_original_met_location(&self) -> bool {
        !(self.format().unwrap_or_default() < 3
            || self.vc()
            || (self.generation() <= 4 && self.format().unwrap_or_default() != self.generation()))
    }

    fn is_gender_valid(&self) -> bool {
        let gender = self.gender();
        let gv = self.personal_info().gender();
        if gv == gender_detail::RATIO_MAGIC_GENDERLESS {
            return gender == 2;
        }
        if gv == gender_detail::RATIO_MAGIC_FEMALE {
            return gender == 1;
        }
        if gv == gender_detail::RATIO_MAGIC_MALE {
            return gender == 0;
        }

        let gen = self.generation();
        if !(3..=5).contains(&gen) {
            return gender == (gender & 1);
        }
        gender == entity_gender::get_from_pid_and_ratio(self.pid(), gv)
    }

    fn refresh_checksum(&mut self);

    fn checksum_valid(&self) -> bool;

    fn fix_moves(&mut self)
    where
        Self: Sized,
    {
        reorder_moves(self);
        if self.move_1() == 0 {
            self.set_move_1_pp(0);
            self.set_move_1_pp_ups(0);
        }
        if self.move_2() == 0 {
            self.set_move_2_pp(0);
            self.set_move_2_pp_ups(0);
        }
        if self.move_3() == 0 {
            self.set_move_3_pp(0);
            self.set_move_3_pp_ups(0);
        }
        if self.move_4() == 0 {
            self.set_move_4_pp(0);
            self.set_move_4_pp_ups(0);
        }
    }

    fn refresh_ability(&mut self, n: u8) {
        self.set_ability_number(1 << n);
        let pi = self.personal_info();
        if let Some(ability) = pi.get_ability_at_index(n as usize) {
            self.set_ability(ability);
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

    fn get_stats(&self, p: &dyn BaseStat) -> [u16; 6]
    where
        Self: Sized,
    {
        let mut stats = [0; 6];
        self.load_stats(p, &mut stats);
        stats
    }

    fn load_stats(&self, p: &dyn BaseStat, stats: &mut [u16])
    where
        Self: Sized,
    {
        let level = self.current_level();
        load_stats(stats, p, self, level);
        nature_amp::modify_stats_for_nature(stats, self.stat_nature());
    }

    fn party_stats_present(&self) -> bool {
        self.stat_hp_max() != 0
    }

    fn reset_party_stats(&mut self)
    where
        Self: Sized,
    {
        let mut stats = [0; 6];
        self.load_stats(self.personal_info(), &mut stats);
        self.set_stats(&stats);
        self.set_stat_level(self.current_level());
        self.set_status_condition(0);
    }

    fn heal(&mut self)
    where
        Self: Sized,
    {
        self.reset_party_stats();
        self.heal_pp();
    }

    fn heal_pp(&mut self)
    where
        Self: Sized,
    {
        let mut pp = self.get_move_pp(self.move_1(), self.move_1_pp_ups());
        self.set_move_1_pp(pp);
        pp = self.get_move_pp(self.move_2(), self.move_2_pp_ups());
        self.set_move_2_pp(pp);
        pp = self.get_move_pp(self.move_3(), self.move_3_pp_ups());
        self.set_move_3_pp(pp);
        pp = self.get_move_pp(self.move_4(), self.move_4_pp_ups());
        self.set_move_4_pp(pp);
    }

    fn heal_pp_index(&mut self, index: u8) -> PKResult<()>
    where
        Self: Sized,
    {
        match index {
            0 => {
                let pp = self.get_move_pp(self.move_1(), self.move_1_pp_ups());
                self.set_move_1_pp(pp)
            }
            1 => {
                let pp = self.get_move_pp(self.move_2(), self.move_2_pp_ups());
                self.set_move_2_pp(pp)
            }
            2 => {
                let pp = self.get_move_pp(self.move_3(), self.move_3_pp_ups());
                self.set_move_3_pp(pp)
            }
            3 => {
                let pp = self.get_move_pp(self.move_4(), self.move_4_pp_ups());
                self.set_move_4_pp(pp)
            }
            _ => {
                return Err(PKError::IndexOutOfRange {
                    index: index as usize,
                })
            }
        }
        Ok(())
    }

    fn force_party_data(&mut self) -> bool
    where
        Self: Sized,
    {
        if self.party_stats_present() {
            return false;
        }
        self.reset_party_stats();
        true
    }

    fn can_hold_item(&self, valid: &[u16]) -> bool {
        valid.contains(&self.held_item())
    }

    fn set_link_trade_egg(&mut self, day: u8, month: u8, year: u16, location: u16) {
        self.set_met_day(day);
        self.set_met_month(month);
        self.set_met_year(year);
        self.set_met_location(location);
    }

    fn get_move_pp(&self, mov: u16, pp_up_count: u8) -> u8
    where
        Self: Sized,
    {
        get_base_pp(self, mov) * (5 + pp_up_count) / 5
    }

    fn set_shiny(&mut self) {
        let mut rand = thread_rng();
        while {
            let pid = self.pid();
            self.set_pid(entity_pid::get_random_pid(
                &mut rand,
                self.species(),
                self.gender(),
                self.version(),
                self.nature(),
                self.form(),
                pid,
            ));
            !self.is_shiny()
        } {}
        if self.format().unwrap_or_default() >= 6 && (self.gen_3() || self.gen_4() || self.gen_5())
        {
            self.set_encryption_constant(self.pid());
        }
    }

    fn set_shiny_sid(&mut self, shiny: Option<legality::Shiny>)
    where
        Self: Sized,
    {
        let shiny = shiny.unwrap_or(legality::Shiny::Random);

        if self.is_shiny() && shiny.is_valid(self) {
            return;
        }

        let xor = self.tid() ^ ((self.pid() >> 16) as u16) ^ ((self.pid() & 0xFFFF) as u16);
        let bits = match shiny {
            legality::Shiny::AlwaysSquare => 0,
            legality::Shiny::AlwaysStar => 1,
            _ => thread_rng().gen_range(0u16..8),
        };

        self.set_sid(xor ^ bits)
    }

    fn set_pid_gender(&mut self, gender: u8) {
        let mut rand = thread_rng();
        while {
            let pid = self.pid();
            self.set_pid(entity_pid::get_random_pid(
                &mut rand,
                self.species(),
                gender,
                self.version(),
                self.nature(),
                self.form(),
                pid,
            ));
            self.is_shiny()
        } {}
        if self.format().unwrap_or_default() >= 6 && (self.gen_3() || self.gen_4() || self.gen_5())
        {
            self.set_encryption_constant(self.pid());
        }
    }

    fn set_pid_nature(&mut self, nature: u8) {
        let mut rand = thread_rng();
        while {
            let pid = self.pid();
            self.set_pid(entity_pid::get_random_pid(
                &mut rand,
                self.species(),
                self.gender(),
                self.version(),
                nature,
                self.form(),
                pid,
            ));
            self.is_shiny()
        } {}
        if self.format().unwrap_or_default() >= 6 && (self.gen_3() || self.gen_4() || self.gen_5())
        {
            self.set_encryption_constant(self.pid());
        }
    }

    fn set_pid_unown_3(&mut self, form: u8) {
        let mut rand = thread_rng();
        while {
            self.set_pid(rand.gen());
            entity_pid::get_unown_form_3(self.pid()) != form
        } {}
        if self.format().unwrap_or_default() >= 6 && (self.gen_3() || self.gen_4() || self.gen_5())
        {
            self.set_encryption_constant(self.pid());
        }
    }

    fn set_random_ivs(&mut self, ivs: &mut [u8; 6], min_flawless: Option<u8>) {
        if self.version() == GameVersion::GO as u8 && min_flawless.unwrap_or_default() != 6 {
            self.set_random_ivs_go(ivs, None, None);
            return;
        }

        let mut rnd = thread_rng();
        for iv in ivs.iter_mut() {
            *iv = rnd.gen_range(0..=self.max_iv());
        }

        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        if count != 0 {
            for iv in ivs.iter_mut().take(count as usize) {
                *iv = self.max_iv();
            }
            rand_util::shuffle(ivs, 0, 6, &mut rnd);
        }

        self.set_ivs(ivs);
    }

    fn set_random_ivs_go(&mut self, ivs: &mut [u8; 6], min_iv: Option<u8>, max_iv: Option<u8>) {
        let min_iv = min_iv.unwrap_or_default();
        let max_iv = max_iv.unwrap_or(15);
        let mut rnd = thread_rng();
        ivs[0] = (rnd.gen_range(min_iv..=max_iv) << 1) | 1;
        ivs[1] = (rnd.gen_range(min_iv..=max_iv) << 1) | 1;
        ivs[4] = ivs[0];
        ivs[2] = (rnd.gen_range(min_iv..=max_iv) << 1) | 1;
        ivs[5] = ivs[2];
        ivs[3] = (rnd.gen_range(0..=self.max_iv()) << 1) | 1;
        self.set_ivs(ivs);
    }

    fn set_random_ivs_template(&mut self, template: IndividualValueSet, min_flawless: Option<u8>) {
        let count = min_flawless.unwrap_or_else(|| self.get_flawless_iv_count());
        let mut ivs = [0u8; 6];
        let mut rnd = thread_rng();
        while {
            for (i, iv) in ivs.iter_mut().enumerate() {
                *iv = template[i].unwrap_or_else(|| rnd.gen_range(0..=self.max_iv()));
            }
            (ivs.iter().copied().filter(|&i| i == self.max_iv()).count() as u8) < count
        } {}
        self.set_ivs(&ivs);
    }

    fn get_flawless_iv_count(&self) -> u8 {
        if self.generation() >= 6 {
            let species = self.species();
            if legality::LEGENDS.contains(&species) || legality::SUB_LEGENDS.contains(&species) {
                return 3;
            }
        }
        if self.xy() {
            if self.personal_info().egg_group_1() == 15 {
                return 3;
            }
            if self.met_location() == 148 && self.met_level() == 30 {
                return 2;
            }
        }
        if self.vc() {
            return if [Species::Mew as u16, Species::Celebi as u16].contains(&self.species()) {
                5
            } else {
                3
            };
        }
        // PLA Impl should handle alpha
        0
    }

    fn has_move(&self, mov: u16) -> bool {
        [self.move_1(), self.move_2(), self.move_3(), self.move_4()].contains(&mov)
    }

    fn get_move_index(&self, mov: u16) -> Option<u8> {
        [self.move_1(), self.move_2(), self.move_3(), self.move_4()]
            .into_iter()
            .position(|i| i == mov)
            .map(|i| i as u8)
    }

    fn get_move(&self, index: u8) -> PKResult<u16> {
        match index {
            0 => Ok(self.move_1()),
            1 => Ok(self.move_2()),
            2 => Ok(self.move_3()),
            3 => Ok(self.move_4()),
            _ => Err(PKError::IndexOutOfRange {
                index: index as usize,
            }),
        }
    }

    fn set_move(&mut self, index: u8, mov: u16) -> PKResult<()> {
        match index {
            0 => self.set_move_1(mov),
            1 => self.set_move_2(mov),
            2 => self.set_move_3(mov),
            3 => self.set_move_4(mov),
            _ => {
                return Err(PKError::IndexOutOfRange {
                    index: index as usize,
                })
            }
        }
        Ok(())
    }

    fn get_relearn_move(&self, index: u8) -> PKResult<u16> {
        match index {
            0 => Ok(self.relearn_move_1()),
            1 => Ok(self.relearn_move_2()),
            2 => Ok(self.relearn_move_3()),
            3 => Ok(self.relearn_move_4()),
            _ => Err(PKError::IndexOutOfRange {
                index: index as usize,
            }),
        }
    }

    fn set_relearn_move(&mut self, index: u8, mov: u16) -> PKResult<()> {
        match index {
            0 => self.set_relearn_move_1(mov),
            1 => self.set_relearn_move_2(mov),
            2 => self.set_relearn_move_3(mov),
            3 => self.set_relearn_move_4(mov),
            _ => {
                return Err(PKError::IndexOutOfRange {
                    index: index as usize,
                })
            }
        }
        Ok(())
    }

    fn has_relearn_move(&self, mov: u16) -> bool {
        [
            self.relearn_move_1(),
            self.relearn_move_2(),
            self.relearn_move_3(),
            self.relearn_move_4(),
        ]
        .contains(&mov)
    }

    fn get_relearn_moves(&self) -> [u16; 4] {
        [
            self.relearn_move_1(),
            self.relearn_move_2(),
            self.relearn_move_3(),
            self.relearn_move_4(),
        ]
    }

    fn clear_invalid_moves(&mut self)
    where
        Self: Sized,
    {
        let mut invalid = 0;
        let mut moves = self.moves();
        for mov in moves.iter_mut() {
            if *mov <= self.max_move_id() {
                continue;
            }
            invalid += 1;
            *mov = 0;
        }

        if invalid == 0 {
            return;
        }
        if invalid == 4 {
            moves[0] = 1;
            let pp = self.get_move_pp(1, self.move_1_pp_ups());
            self.set_move_1_pp(pp);
        }

        self.set_moves(&moves);
        self.fix_moves();
    }

    fn get_ev(&self, index: usize) -> PKResult<u8> {
        match index {
            0 => Ok(self.ev_hp()),
            1 => Ok(self.ev_atk()),
            2 => Ok(self.ev_def()),
            3 => Ok(self.ev_spe()),
            4 => Ok(self.ev_spa()),
            5 => Ok(self.ev_spd()),
            _ => Err(PKError::IndexOutOfRange { index }),
        }
    }

    fn get_iv(&self, index: usize) -> PKResult<u8> {
        match index {
            0 => Ok(self.iv_hp()),
            1 => Ok(self.iv_atk()),
            2 => Ok(self.iv_def()),
            3 => Ok(self.iv_spe()),
            4 => Ok(self.iv_spa()),
            5 => Ok(self.iv_spd()),
            _ => Err(PKError::IndexOutOfRange { index }),
        }
    }

    fn set_maximum_pp_current_moves(&mut self, moves: &[u16])
    where
        Self: Sized,
    {
        self.set_move_1_pp(if moves.is_empty() {
            0
        } else {
            self.get_move_pp(moves[0], self.move_1_pp_ups())
        });
        self.set_move_2_pp(if moves.len() <= 1 {
            0
        } else {
            self.get_move_pp(moves[1], self.move_2_pp_ups())
        });
        self.set_move_3_pp(if moves.len() <= 2 {
            0
        } else {
            self.get_move_pp(moves[2], self.move_3_pp_ups())
        });
        self.set_move_4_pp(if moves.len() <= 3 {
            0
        } else {
            self.get_move_pp(moves[3], self.move_4_pp_ups())
        });
    }

    fn set_maximum_pp_current(&mut self)
    where
        Self: Sized,
    {
        let moves = self.moves();
        self.set_maximum_pp_current_moves(&moves);
    }
}

pub(crate) fn load_stats_ht(
    stats: &mut [u16],
    p: &(impl BaseStat + ?Sized),
    t: &(impl HyperTrain + Pkm),
    level: u8,
) {
    stats[0] = u16::from(if p.hp() == 1 {
        1
    } else {
        ((if t.ht_hp() { 31 } else { t.iv_hp() } + (2 * p.hp()) + (t.ev_hp() / 4) + 100) * level
            / 100)
            + 10
    });
    stats[1] = u16::from(
        ((if t.ht_atk() { 31 } else { t.iv_atk() } + (2 * p.atk()) + (t.ev_atk() / 4)) * level
            / 100)
            + 5,
    );
    stats[2] = u16::from(
        ((if t.ht_def() { 31 } else { t.iv_def() } + (2 * p.def()) + (t.ev_def() / 4)) * level
            / 100)
            + 5,
    );
    stats[3] = u16::from(
        ((if t.ht_spa() { 31 } else { t.iv_spa() } + (2 * p.spa()) + (t.ev_spa() / 4)) * level
            / 100)
            + 5,
    );
    stats[4] = u16::from(
        ((if t.ht_spd() { 31 } else { t.iv_spd() } + (2 * p.spd()) + (t.ev_spd() / 4)) * level
            / 100)
            + 5,
    );
    stats[5] = u16::from(
        ((if t.ht_spe() { 31 } else { t.iv_spe() } + (2 * p.spe()) + (t.ev_spe() / 4)) * level
            / 100)
            + 5,
    );
}

pub(crate) fn load_stats(stats: &mut [u16], p: &(impl BaseStat + ?Sized), t: &impl Pkm, level: u8) {
    stats[0] = u16::from(if p.hp() == 1 {
        1
    } else {
        ((t.iv_hp() + (2 * p.hp()) + (t.ev_hp() / 4) + 100) * level / 100) + 10
    });
    stats[1] = u16::from(((t.iv_atk() + (2 * p.atk()) + (t.ev_atk() / 4)) * level / 100) + 5);
    stats[2] = u16::from(((t.iv_def() + (2 * p.def()) + (t.ev_def() / 4)) * level / 100) + 5);
    stats[3] = u16::from(((t.iv_spa() + (2 * p.spa()) + (t.ev_spa() / 4)) * level / 100) + 5);
    stats[4] = u16::from(((t.iv_spd() + (2 * p.spd()) + (t.ev_spd() / 4)) * level / 100) + 5);
    stats[5] = u16::from(((t.iv_spe() + (2 * p.spe()) + (t.ev_spe() / 4)) * level / 100) + 5);
}
