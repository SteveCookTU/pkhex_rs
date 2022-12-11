use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PKError {
    #[snafu(display("Index out of range: {}", index))]
    IndexOutOfRange { index: usize },
}

pub type PKResult<T, E = PKError> = Result<T, E>;
