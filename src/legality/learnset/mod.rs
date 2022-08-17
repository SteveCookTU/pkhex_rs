mod egg_moves;
mod learnset;
pub mod learnset_reader;

use crate::{tables, BinLinkerAccessor};
pub use egg_moves::*;
use lazy_static::lazy_static;
pub use learnset::*;

lazy_static! {
    // Gen 1
    pub static ref LEVEL_UP_RB: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_rb.pkl"),
        tables::MAX_SPECIES_ID_1
    );
    pub static ref LEVEL_UP_Y: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_y.pkl"),
        tables::MAX_SPECIES_ID_1
    );

    // Gen 2
    pub static ref EGG_MOVES_GS: Vec<EggMoves2> = EggMoves2::get_array(
        include_bytes!("../../resources/byte/eggmove/eggmove_gs.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref LEVEL_UP_GS: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_gs.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref EGG_MOVES_C: Vec<EggMoves2> = EggMoves2::get_array(
        include_bytes!("../../resources/byte/eggmove/eggmove_c.pkl"),
        tables::MAX_SPECIES_ID_2
    );
    pub static ref LEVEL_UP_C: Vec<LearnSet> = learnset_reader::get_array(
        include_bytes!("../../resources/byte/levelup/lvlmove_c.pkl"),
        tables::MAX_SPECIES_ID_2
    );

    // Gen 3
    pub static ref LEVEL_UP_E: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_e.pkl")).into();
    pub static ref LEVEL_UP_RS: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_rs.pkl")).into();
    pub static ref LEVEL_UP_FR: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_fr.pkl")).into();
    pub static ref LEVEL_UP_LG: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_lg.pkl")).into();
    pub static ref EGG_MOVES_RS: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_rs.pkl")).into();

    //Gen 4
    pub static ref LEVEL_UP_DP: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_dp.pkl")).into();
    pub static ref LEVEL_UP_PT: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_pt.pkl")).into();
    pub static ref LEVEL_UP_HGSS: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_hgss.pkl")).into();
    pub static ref EGG_MOVES_DPPT: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_dppt.pkl")).into();
    pub static ref EGG_MOVES_HGSS: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_hgss.pkl")).into();

    //Gen 5
    pub static ref LEVEL_UP_BW: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_bw.pkl")).into();
    pub static ref LEVEL_UP_B2W2: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_b2w2.pkl")).into();
    pub static ref EGG_MOVES_BW: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_bw.pkl")).into();

    // Gen 6
    pub static ref EGG_MOVES_XY: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_xy.pkl")).into();
    pub static ref LEVEL_UP_XY: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_xy.pkl")).into();
    pub static ref EGG_MOVES_AO: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_ao.pkl")).into();
    pub static ref LEVEL_UP_AO: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_ao.pkl")).into();

    // Gen 7
    pub static ref EGG_MOVES_SM: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_sm.pkl")).into();
    pub static ref LEVEL_UP_SM: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_sm.pkl")).into();
    pub static ref EGG_MOVES_USUM: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_uu.pkl")).into();
    pub static ref LEVEL_UP_USUM: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_uu.pkl")).into();
    pub static ref LEVEL_UP_GG: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_gg.pkl")).into();

    // Gen 8
    pub static ref EGG_MOVES_SWSH: Vec<EggMoves7> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_swsh.pkl")).into();
    pub static ref LEVEL_UP_SWSH: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_swsh.pkl")).into();
    pub static ref EGG_MOVES_BDSP: Vec<EggMoves6> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/eggmove/eggmove_bdsp.pkl")).into();
    pub static ref LEVEL_UP_BDSP: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_bdsp.pkl")).into();
    pub static ref LEVEL_UP_LA: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/byte/levelup/lvlmove_la.pkl")).into();
    pub static ref MASTERY_LA: Vec<LearnSet> = BinLinkerAccessor::new(include_bytes!("../../resources/legality/misc/mastery_la.pkl")).into();
}
