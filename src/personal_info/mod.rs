mod info;
mod table;
pub mod traits;

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
