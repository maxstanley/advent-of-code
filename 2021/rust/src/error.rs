use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Failed to Parse Value.")]
    ParseError,

    #[error("IO Error")]
    IOError,
}

impl From<ParseIntError> for Error {
    fn from(_err: ParseIntError) -> Error {
        Error::ParseError
    }
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::IOError
    }
}
