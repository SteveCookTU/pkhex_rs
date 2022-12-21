mod info;
mod table;
pub mod traits;

use crate::{legality, resource_util};
pub use info::*;
use lazy_static::lazy_static;
pub use table::*;

use crate::legality::BinLinkerAccessor;

lazy_static! {
    pub static ref Y: PersonalTable1<'static> =
        PersonalTable1::new(resource_util::get_bin_resource("personal_y"));
    pub static ref RB: PersonalTable1<'static> =
        PersonalTable1::new(resource_util::get_bin_resource("personal_rb"));
}

lazy_static! {
    pub static ref GS: PersonalTable2<'static> =
        PersonalTable2::new(resource_util::get_bin_resource("personal_gs"));
    pub static ref C: PersonalTable2<'static> =
        PersonalTable2::new(resource_util::get_bin_resource("personal_c"));
}

lazy_static! {
    pub static ref RS: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(resource_util::get_bin_resource("personal_rs"));
        let machine = BinLinkerAccessor::new(resource_util::get_bin_resource("hmtm_g3.pkl"));
        let tutors = BinLinkerAccessor::new(resource_util::get_bin_resource("tutors_g3.pkl"));
        table.load_tables(machine, tutors);
        table
    };
    pub static ref E: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(resource_util::get_bin_resource("personal_e"));
        let machine = BinLinkerAccessor::new(resource_util::get_bin_resource("hmtm_g3.pkl"));
        let tutors = BinLinkerAccessor::new(resource_util::get_bin_resource("tutors_g3.pkl"));
        table.load_tables(machine, tutors);
        table
    };
    pub static ref FR: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(resource_util::get_bin_resource("personal_fr"));
        let machine = BinLinkerAccessor::new(resource_util::get_bin_resource("hmtm_g3.pkl"));
        let tutors = BinLinkerAccessor::new(resource_util::get_bin_resource("tutors_g3.pkl"));
        table.load_tables(machine, tutors);
        table
    };
    pub static ref LG: PersonalTable3<'static> = {
        let mut table = PersonalTable3::new(resource_util::get_bin_resource("personal_lg"));
        let machine = BinLinkerAccessor::new(resource_util::get_bin_resource("hmtm_g3.pkl"));
        let tutors = BinLinkerAccessor::new(resource_util::get_bin_resource("tutors_g3.pkl"));
        table.load_tables(machine, tutors);
        table
    };
}

lazy_static! {
    pub static ref DP: PersonalTable4<'static> =
        PersonalTable4::new(resource_util::get_bin_resource("personal_dp"));
    pub static ref PT: PersonalTable4<'static> =
        PersonalTable4::new(resource_util::get_bin_resource("personal_pt"));
    pub static ref HGSS: PersonalTable4<'static> = {
        let mut tables = PersonalTable4::new(resource_util::get_bin_resource("personal_hgss"));
        let tutors = BinLinkerAccessor::new(resource_util::get_bin_resource("tutors_g4.pkl"));
        tables.load_tables(tutors);
        tables
    };
}

lazy_static! {
    pub static ref BW: PersonalTable5BW<'static> =
        PersonalTable5BW::new(resource_util::get_bin_resource("personal_bw"));
    pub static ref B2W2: PersonalTable5B2BW<'static> =
        PersonalTable5B2BW::new(resource_util::get_bin_resource("personal_b2w2"));
}

lazy_static! {
    pub static ref XY: PersonalTable6XY<'static> =
        PersonalTable6XY::new(resource_util::get_bin_resource("personal_xy"));
    pub static ref AO: PersonalTable6AO<'static> =
        PersonalTable6AO::new(resource_util::get_bin_resource("personal_ao"));
}

lazy_static! {
    pub static ref SM: PersonalTable7<'static> = PersonalTable7::new(
        resource_util::get_bin_resource("personal_sm"),
        legality::tables7::MAX_SPECIES_ID_7
    );
    pub static ref USUM: PersonalTable7<'static> = PersonalTable7::new(
        resource_util::get_bin_resource("personal_uu"),
        legality::tables7::MAX_SPECIES_ID_7_USUM
    );
    pub static ref GG: PersonalTable7GG<'static> =
        PersonalTable7GG::new(resource_util::get_bin_resource("personal_gg"));
}

lazy_static! {
    pub static ref SWSH: PersonalTable8SWSH<'static> =
        PersonalTable8SWSH::new(resource_util::get_bin_resource("personal_swsh"));
    pub static ref BDSP: PersonalTable8BDSP<'static> =
        PersonalTable8BDSP::new(resource_util::get_bin_resource("personal_bdsp"));
    pub static ref LA: PersonalTable8LA<'static> =
        PersonalTable8LA::new(resource_util::get_bin_resource("personal_la"));
}

lazy_static! {
    pub static ref SV: PersonalTable9SV<'static> =
        PersonalTable9SV::new(resource_util::get_bin_resource("personal_sv"));
}
