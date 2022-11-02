#[cfg(feature = "gen5")]
pub mod personal_info_b2w2;
#[cfg(feature = "gen8")]
pub mod personal_info_bdsp;
pub mod personal_info_bw;
#[cfg(feature = "gen1")]
pub mod personal_info_g1;
#[cfg(feature = "gen2")]
pub mod personal_info_g2;
#[cfg(feature = "gen3")]
pub mod personal_info_g3;
#[cfg(feature = "gen4")]
pub mod personal_info_g4;
#[cfg(feature = "gen7")]
pub mod personal_info_gg;
pub mod personal_info_la;
#[cfg(feature = "gen6")]
pub mod personal_info_oras;
pub mod personal_info_sm;
pub mod personal_info_swsh;
mod personal_info_trait;
pub mod personal_info_xy;
pub mod personal_table;

pub use personal_info_trait::*;
