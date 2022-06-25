use crate::personal_info_b2w2::PersonalInfoB2W2;
use crate::personal_info_bdsp::PersonalInfoBDSP;
use crate::personal_info_bw::PersonalInfoBW;
use crate::personal_info_g1::PersonalInfoG1;
use crate::personal_info_g2::PersonalInfoG2;
use crate::personal_info_g3::PersonalInfoG3;
use crate::personal_info_g4::PersonalInfoG4;
use crate::personal_info_gg::PersonalInfoGG;
use crate::personal_info_la::PersonalInfoLA;
use crate::personal_info_oras::PersonalInfoORAS;
use crate::personal_info_sm::PersonalInfoSM;
use crate::personal_info_swsh::PersonalInfoSWSH;
use crate::personal_info_xy::PersonalInfoXY;
use crate::tables::{
    MAX_SPECIES_ID_1, MAX_SPECIES_ID_3, MAX_SPECIES_ID_7_USUM, MAX_SPECIES_ID_8_R2,
};
use crate::{
    form_index, get_bits, is_valid_type_combination, personal_info_b2w2, personal_info_bdsp,
    personal_info_bw, personal_info_g1, personal_info_g2, personal_info_g3, personal_info_g4,
    personal_info_la, personal_info_oras, personal_info_sm, personal_info_swsh, personal_info_xy,
    BinLinkerAccessor, GameVersion, PersonalInfo,
};
use lazy_static::lazy_static;
use std::ops::{Index, IndexMut};

const PERSONAL_LA: &[u8] = include_bytes!("../resources/byte/personal/personal_la");
const PERSONAL_BDSP: &[u8] = include_bytes!("../resources/byte/personal/personal_bdsp");
const PERSONAL_SWSH: &[u8] = include_bytes!("../resources/byte/personal/personal_swsh");
const PERSONAL_GG: &[u8] = include_bytes!("../resources/byte/personal/personal_gg");
const PERSONAL_USUM: &[u8] = include_bytes!("../resources/byte/personal/personal_uu");
const PERSONAL_SM: &[u8] = include_bytes!("../resources/byte/personal/personal_sm");
const PERSONAL_AO: &[u8] = include_bytes!("../resources/byte/personal/personal_ao");
const PERSONAL_XY: &[u8] = include_bytes!("../resources/byte/personal/personal_xy");
const PERSONAL_B2W2: &[u8] = include_bytes!("../resources/byte/personal/personal_b2w2");
const PERSONAL_BW: &[u8] = include_bytes!("../resources/byte/personal/personal_bw");
const PERSONAL_HGSS: &[u8] = include_bytes!("../resources/byte/personal/personal_hgss");
const PERSONAL_PT: &[u8] = include_bytes!("../resources/byte/personal/personal_pt");
const PERSONAL_DP: &[u8] = include_bytes!("../resources/byte/personal/personal_dp");
const PERSONAL_LG: &[u8] = include_bytes!("../resources/byte/personal/personal_lg");
const PERSONAL_FR: &[u8] = include_bytes!("../resources/byte/personal/personal_fr");
const PERSONAL_E: &[u8] = include_bytes!("../resources/byte/personal/personal_e");
const PERSONAL_RS: &[u8] = include_bytes!("../resources/byte/personal/personal_rs");
const PERSONAL_C_GS: &[u8] = include_bytes!("../resources/byte/personal/personal_c");
const PERSONAL_RB: &[u8] = include_bytes!("../resources/byte/personal/personal_rb");
const PERSONAL_Y: &[u8] = include_bytes!("../resources/byte/personal/personal_y");

const HMTM_G3: &[u8] = include_bytes!("../resources/byte/personal/hmtm_g3.pkl");
const TUTORS_G3: &[u8] = include_bytes!("../resources/byte/personal/tutors_g3.pkl");
const TUTORS_G4: &[u8] = include_bytes!("../resources/byte/personal/tutors_g4.pkl");

lazy_static! {
    pub static ref LA: PersonalTable<PersonalInfoLA> = {
        let mut table = PersonalTable::new(PERSONAL_LA.to_vec(), GameVersion::PLA);
        copy_dexit_genders_la(&mut table);
        table
    };
    pub static ref BDSP: PersonalTable<PersonalInfoBDSP> =
        PersonalTable::new(PERSONAL_BDSP.to_vec(), GameVersion::BDSP);
    pub static ref SWSH: PersonalTable<PersonalInfoSWSH> = {
        let mut table = PersonalTable::new(PERSONAL_SWSH.to_vec(), GameVersion::SWSH);
        copy_dexit_genders_swsh(&mut table);
        table
    };
    pub static ref GG: PersonalTable<PersonalInfoGG> =
        PersonalTable::new(PERSONAL_GG.to_vec(), GameVersion::GG);
    pub static ref USUM: PersonalTable<PersonalInfoSM> =
        PersonalTable::new(PERSONAL_USUM.to_vec(), GameVersion::USUM);
    pub static ref SM: PersonalTable<PersonalInfoSM> =
        PersonalTable::new(PERSONAL_SM.to_vec(), GameVersion::SM);
    pub static ref AO: PersonalTable<PersonalInfoORAS> =
        PersonalTable::new(PERSONAL_AO.to_vec(), GameVersion::ORAS);
    pub static ref XY: PersonalTable<PersonalInfoXY> =
        PersonalTable::new(PERSONAL_XY.to_vec(), GameVersion::XY);
    pub static ref B2W2: PersonalTable<PersonalInfoB2W2> =
        PersonalTable::new(PERSONAL_B2W2.to_vec(), GameVersion::B2W2);
    pub static ref BW: PersonalTable<PersonalInfoBW> =
        PersonalTable::new(PERSONAL_BW.to_vec(), GameVersion::BW);
    pub static ref HGSS: PersonalTable<PersonalInfoG4> =
        PersonalTable::new(PERSONAL_HGSS.to_vec(), GameVersion::HGSS);
    pub static ref PT: PersonalTable<PersonalInfoG4> =
        PersonalTable::new(PERSONAL_PT.to_vec(), GameVersion::Pt);
    pub static ref DP: PersonalTable<PersonalInfoG4> =
        PersonalTable::new(PERSONAL_DP.to_vec(), GameVersion::DP);
    pub static ref LG: PersonalTable<PersonalInfoG3> = {
        let mut table = PersonalTable::new(PERSONAL_LG.to_vec(), GameVersion::LG);
        populate_gen_3_tutors(&mut table);
        table
    };
    pub static ref FR: PersonalTable<PersonalInfoG3> = {
        let mut table = PersonalTable::new(PERSONAL_FR.to_vec(), GameVersion::FR);
        populate_gen_3_tutors(&mut table);
        table
    };
    pub static ref E: PersonalTable<PersonalInfoG3> = {
        let mut table = PersonalTable::new(PERSONAL_E.to_vec(), GameVersion::E);
        populate_gen_3_tutors(&mut table);
        table
    };
    pub static ref RS: PersonalTable<PersonalInfoG3> = {
        let mut table = PersonalTable::new(PERSONAL_RS.to_vec(), GameVersion::RS);
        populate_gen_3_tutors(&mut table);
        table
    };
    pub static ref GS: PersonalTable<PersonalInfoG2> =
        PersonalTable::new(PERSONAL_C_GS.to_vec(), GameVersion::GS);
    pub static ref C: PersonalTable<PersonalInfoG2> =
        PersonalTable::new(PERSONAL_C_GS.to_vec(), GameVersion::C);
    pub static ref RB: PersonalTable<PersonalInfoG1> = {
        let mut table = PersonalTable::new(PERSONAL_RB.to_vec(), GameVersion::RB);
        fix_personal_table_g1(&mut table);
        table
    };
    pub static ref Y: PersonalTable<PersonalInfoG1> = {
        let mut table = PersonalTable::new(PERSONAL_Y.to_vec(), GameVersion::YW);
        fix_personal_table_g1(&mut table);
        table
    };
}

pub struct PersonalTable<T: PersonalInfo> {
    table: Vec<T>,
    max_species_id: usize,
    game: GameVersion,
}

impl<T: PersonalInfo> Index<usize> for PersonalTable<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl<T: PersonalInfo> IndexMut<usize> for PersonalTable<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}

impl<T: PersonalInfo> PersonalTable<T> {
    pub fn new(data: Vec<u8>, version: GameVersion) -> Self {
        let size = get_entry_size(version);
        let count = data.len() / size;
        let mut table = Vec::with_capacity(count);
        for i in 0..count {
            table.push(T::new((&data[(size * i)..((size * i) + size)]).to_vec()))
        }
        Self {
            table,
            max_species_id: version.get_max_species_id(),
            game: version,
        }
    }

    pub fn get_form_index(&self, species: usize, form: usize) -> usize {
        if species <= self.max_species_id {
            form_index(&self[species], species, form)
        } else {
            0
        }
    }

    pub fn get_form_entry(&self, species: usize, form: usize) -> &T {
        &self[self.get_form_index(species, form)]
    }

    pub fn get_form_entry_mut(&mut self, species: usize, form: usize) -> &mut T {
        let index = self.get_form_index(species, form);
        &mut self[index]
    }

    pub fn table_length(&self) -> usize {
        self.table.len()
    }

    pub fn get_form_list(&self, species: Vec<String>, max_species: usize) -> Vec<Vec<String>> {
        let mut form_list = Vec::with_capacity(max_species + 1);
        for i in 0..(max_species + 1) {
            let form_count = self[i].get_form_count();
            form_list[i] = Vec::with_capacity(form_count);
            if form_count == 0 {
                continue;
            }

            form_list[i][0] = species[i].clone();

            for j in 1..form_count {
                form_list[i][j] = format!("{} {}", species[i], j);
            }
        }
        form_list
    }

    pub fn get_personal_entry_list(
        &self,
        forms: Vec<Vec<String>>,
        species: Vec<String>,
        max_species: usize,
        base_form: &mut Vec<usize>,
        form_val: &mut Vec<usize>,
    ) -> Vec<String> {
        let mut result = Vec::with_capacity(self.table.len());
        *base_form = Vec::with_capacity(self.table.len());
        *form_val = Vec::with_capacity(self.table.len());

        for i in 0..max_species {
            result[i] = species[i].clone();
            if forms[i].is_empty() {
                continue;
            }
            let base_ptr = self[i].get_form_stats_index();
            if base_ptr == 0 {
                continue;
            }

            for j in 1..forms[i].len() {
                let ptr = base_ptr + j - 1;
                base_form[ptr] = i;
                form_val[ptr] = j;
                result[ptr] = forms[i][j].clone();
            }
        }

        result
    }

    pub fn is_species_in_game(&self, species: usize) -> bool {
        if species >= self.max_species_id {
            return false;
        }
        let form0 = &self[species];
        if form0.get_hp() != 0 {
            return true;
        }
        let fc = form0.get_form_count();
        for i in 1..fc {
            if self.get_form_entry(species, i).get_hp() != 0 {
                return true;
            }
        }
        false
    }

    pub fn is_valid_type_combination(&self, type_1: usize, type_2: usize) -> bool {
        self.table
            .iter()
            .any(|info| is_valid_type_combination(info, type_1, type_2))
    }
}

fn fix_personal_table_g1(table: &mut PersonalTable<PersonalInfoG1>) {
    for i in (0..=MAX_SPECIES_ID_1).rev() {
        table.table[i].set_gender(GS[i].get_gender())
    }
}

fn populate_gen_3_tutors(table: &mut PersonalTable<PersonalInfoG3>) {
    let machine = BinLinkerAccessor::new(HMTM_G3);
    let tutors = BinLinkerAccessor::new(TUTORS_G3);
    for i in (0..=MAX_SPECIES_ID_3).rev() {
        let entry = &mut table[i];
        entry.set_tmhm(get_bits(&machine[i]));
        entry.set_type_tutors(get_bits(&tutors[i]));
    }
}

fn populate_gen_4_tutors(table: &mut PersonalTable<PersonalInfoG4>) {
    let tutors = BinLinkerAccessor::new(TUTORS_G4);
    for i in 0..tutors.length() {
        table[i].set_type_tutors(get_bits(&tutors[i]));
    }
}

fn copy_dexit_genders_swsh(table: &mut PersonalTable<PersonalInfoSWSH>) {
    let usum = &USUM.table;
    for i in 1..=MAX_SPECIES_ID_7_USUM {
        let ss = &mut table[i];
        if ss.get_hp() == 0 {
            ss.set_gender(usum[i].get_gender());
        }
    }
}

fn copy_dexit_genders_la(table: &mut PersonalTable<PersonalInfoLA>) {
    for i in 1..=MAX_SPECIES_ID_8_R2 {
        let fc = table[i].get_form_count();
        for f in 0..fc {
            let l = table.get_form_entry_mut(i, f);
            if l.get_hp() != 0 {
                continue;
            }
            let s = SWSH.get_form_entry(i, f);
            l.set_ability_1(s.get_ability_1());
            l.set_ability_2(s.get_ability_2());
            l.set_ability_h(s.get_ability_h());
            l.set_gender(s.get_gender());
            l.set_exp_growth(s.get_exp_growth());
        }
    }
}

fn get_entry_size(version: GameVersion) -> usize {
    match version {
        GameVersion::RB | GameVersion::YW => personal_info_g1::SIZE,
        GameVersion::GS | GameVersion::C => personal_info_g2::SIZE,
        GameVersion::RS | GameVersion::E | GameVersion::FR | GameVersion::LG => {
            personal_info_g3::SIZE
        }
        GameVersion::DP | GameVersion::Pt | GameVersion::HGSS => personal_info_g4::SIZE,
        GameVersion::BW => personal_info_bw::SIZE,
        GameVersion::B2W2 => personal_info_b2w2::SIZE,
        GameVersion::XY => personal_info_xy::SIZE,
        GameVersion::ORAS => personal_info_oras::SIZE,
        GameVersion::SM | GameVersion::USUM | GameVersion::GG => personal_info_sm::SIZE,
        GameVersion::SWSH => personal_info_swsh::SIZE,
        GameVersion::BDSP => personal_info_bdsp::SIZE,
        GameVersion::PLA => personal_info_la::SIZE,
        _ => 0,
    }
}
