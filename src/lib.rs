mod error;
pub mod game;
pub mod legality;
pub mod moves;
pub mod personal_info;
pub mod pkm;
mod util;

use include_dir::{include_dir, Dir};
pub use util::*;

pub use error::*;

pub static RESOURCES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");
