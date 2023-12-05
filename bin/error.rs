use core::result;
use mulib::error::Error as MuLibError;
use repl_rs::Error as ReplError;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Music Lib crate error {0}")]
    MuLib(MuLibError),
    #[error("Repl crate error {0}")]
    Repl(ReplError),
    #[error("StrumParse {0}")]
    Strum(#[from] strum::ParseError),
}

impl From<MuLibError> for Error {
    fn from(e: MuLibError) -> Self {
        Error::MuLib(e)
    }
}

impl From<ReplError> for Error {
    fn from(e: ReplError) -> Self {
        Error::Repl(e)
    }
}
