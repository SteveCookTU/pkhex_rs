pub mod enums;
mod pkm_trait;
pub mod shared;
pub mod traits;
pub mod util;

pub use pkm_trait::*;

use crate::pkm::util::entity_file_extension;

pub fn extensions() -> Vec<String> {
    entity_file_extension::get_extensions(None)
}

pub enum PKM {
    PK1,
}
