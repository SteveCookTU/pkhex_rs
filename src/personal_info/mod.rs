mod info;
mod table;
pub mod traits;

use crate::legality;
pub use info::*;
use lazy_static::lazy_static;
pub use table::*;

use crate::legality::BinLinkerAccessor;

const PERSONAL_Y: &[u8] = include_bytes!("../resources/personal/personal_y");
const PERSONAL_RB: &[u8] = include_bytes!("../resources/personal/personal_rb");

lazy_static! {
    pub static ref Y: PersonalTable1<'static> = PersonalTable1::new(PERSONAL_Y);
    pub static ref RB: PersonalTable1<'static> = PersonalTable1::new(PERSONAL_RB);
}

const PERSONAL_GS: &[u8] = include_bytes!("../resources/personal/personal_gs");
const PERSONAL_C: &[u8] = include_bytes!("../resources/personal/personal_c");

lazy_static! {
    pub static ref GS: PersonalTable2<'static> = PersonalTable2::new(PERSONAL_GS);
    pub static ref C: PersonalTable2<'static> = PersonalTable2::new(PERSONAL_C);
}

const PERSONAL_RS: &[u8] = include_bytes!("../resources/personal/personal_rs");
const PERSONAL_E: &[u8] = include_bytes!("../resources/personal/personal_e");
const PERSONAL_FR: &[u8] = include_bytes!("../resources/personal/personal_fr");
const PERSONAL_LG: &[u8] = include_bytes!("../resources/personal/personal_lg");
const HMTM_G3: &[u8] = include_bytes!("../resources/personal/hmtm_g3.pkl");
const TUTORS_G3: &[u8] = include_bytes!("../resources/personal/tutors_g3.pkl");

lazy_static! {
    pub static ref RS: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(PERSONAL_RS);
        let machine = BinLinkerAccessor::new(HMTM_G3);
        let tutors = BinLinkerAccessor::new(TUTORS_G3);
        table.load_tables(machine, tutors);
        table
    };
    pub static ref E: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(PERSONAL_E);
        let machine = BinLinkerAccessor::new(HMTM_G3);
        let tutors = BinLinkerAccessor::new(TUTORS_G3);
        table.load_tables(machine, tutors);
        table
    };
    pub static ref FR: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(PERSONAL_FR);
        let machine = BinLinkerAccessor::new(HMTM_G3);
        let tutors = BinLinkerAccessor::new(TUTORS_G3);
        table.load_tables(machine, tutors);
        table
    };
    pub static ref LG: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(PERSONAL_LG);
        let machine = BinLinkerAccessor::new(HMTM_G3);
        let tutors = BinLinkerAccessor::new(TUTORS_G3);
        table.load_tables(machine, tutors);
        table
    };
}

const PERSONAL_DP: &[u8] = include_bytes!("../resources/personal/personal_dp");
const PERSONAL_PT: &[u8] = include_bytes!("../resources/personal/personal_pt");
const PERSONAL_HGSS: &[u8] = include_bytes!("../resources/personal/personal_hgss");
const TUTORS_G4: &[u8] = include_bytes!("../resources/personal/tutors_g4.pkl");

lazy_static! {
    pub static ref DP: PersonalTable4<'static> = PersonalTable4::new(PERSONAL_DP);
    pub static ref PT: PersonalTable4<'static> = PersonalTable4::new(PERSONAL_PT);
    pub static ref HGSS: PersonalTable4<'static> = {
        let mut tables = PersonalTable4::new(PERSONAL_HGSS);
        let tutors = BinLinkerAccessor::new(TUTORS_G4);
        tables.load_tables(tutors);
        tables
    };
}

const PERSONAL_BW: &[u8] = include_bytes!("../resources/personal/personal_bw");
const PERSONAL_B2W2: &[u8] = include_bytes!("../resources/personal/personal_b2w2");

lazy_static! {
    pub static ref BW: PersonalTable5BW<'static> = PersonalTable5BW::new(PERSONAL_BW);
    pub static ref B2W2: PersonalTable5B2BW<'static> = PersonalTable5B2BW::new(PERSONAL_B2W2);
}

const PERSONAL_XY: &[u8] = include_bytes!("../resources/personal/personal_xy");
const PERSONAL_AO: &[u8] = include_bytes!("../resources/personal/personal_ao");

lazy_static! {
    pub static ref XY: PersonalTable6XY<'static> = PersonalTable6XY::new(PERSONAL_XY);
    pub static ref AO: PersonalTable6AO<'static> = PersonalTable6AO::new(PERSONAL_AO);
}

const PERSONAL_SM: &[u8] = include_bytes!("../resources/personal/personal_sm");
const PERSONAL_USUM: &[u8] = include_bytes!("../resources/personal/personal_uu");
const PERSONAL_GG: &[u8] = include_bytes!("../resources/personal/personal_gg");

lazy_static! {
    pub static ref SM: PersonalTable7<'static> =
        PersonalTable7::new(PERSONAL_SM, legality::tables7::MAX_SPECIES_ID_7);
    pub static ref USUM: PersonalTable7<'static> =
        PersonalTable7::new(PERSONAL_USUM, legality::tables7::MAX_SPECIES_ID_7_USUM);
    pub static ref GG: PersonalTable7GG<'static> = PersonalTable7GG::new(PERSONAL_GG);
}

const PERSONAL_SWSH: &[u8] = include_bytes!("../resources/personal/personal_swsh");
const PERSONAL_BDSP: &[u8] = include_bytes!("../resources/personal/personal_bdsp");
const PERSONAL_LA: &[u8] = include_bytes!("../resources/personal/personal_la");

lazy_static! {
    pub static ref SWSH: PersonalTable8SWSH<'static> = PersonalTable8SWSH::new(PERSONAL_SWSH);
    pub static ref BDSP: PersonalTable8BDSP<'static> = PersonalTable8BDSP::new(PERSONAL_BDSP);
    pub static ref LA: PersonalTable8LA<'static> = PersonalTable8LA::new(PERSONAL_LA);
}

const PERSONAL_SV: &[u8] = include_bytes!("../resources/personal/personal_sv");

lazy_static! {
    pub static ref SV: PersonalTable9SV<'static> = PersonalTable9SV::new(PERSONAL_SV);
}
