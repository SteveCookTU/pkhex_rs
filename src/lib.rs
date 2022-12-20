pub mod editing;
mod error;
pub mod game;
pub mod legality;
pub mod moves;
pub mod personal_info;
pub mod pkm;
pub mod ribbons;
mod util;

pub use error::*;
use include_dir::{include_dir, Dir};
pub use util::*;

pub static RESOURCES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");
