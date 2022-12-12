mod info;
mod table;
pub mod traits;

pub use info::*;
use lazy_static::lazy_static;
pub use table::*;

const PERSONAL_Y: &[u8] = include_bytes!("../resources/personal/personal_y");
const PERSONAL_RB: &[u8] = include_bytes!("../resources/personal/personal_rb");

lazy_static! {
    pub static ref Y: PersonalInfo1<'static> = PersonalInfo1::new(PERSONAL_Y);
    pub static ref RB: PersonalInfo1<'static> = PersonalInfo1::new(PERSONAL_RB);
}

const PERSONAL_GS: &[u8] = include_bytes!("../resources/personal/personal_gs");
const PERSONAL_C: &[u8] = include_bytes!("../resources/personal/personal_c");

lazy_static! {
    pub static ref GS: PersonalInfo2<'static> = PersonalInfo2::new(PERSONAL_GS);
    pub static ref C: PersonalInfo2<'static> = PersonalInfo2::new(PERSONAL_C);
}
