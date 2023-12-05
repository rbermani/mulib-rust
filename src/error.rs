use core::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Unsupported Feature")]
    Unsupported,
    #[error("IO Kind {0}")]
    IoKind(String),
    #[error("Unnamed Error")]
    Unit,
    #[error("Data Out of Bounds")]
    OutofBounds,
    #[error("Alter value needed")]
    AlterValue,
    #[error("Inversion not supported for Chord Type")]
    InvalidInversion,
    #[error("Missing Reader")]
    MissingReader,
    #[error("Parsing Error")]
    Parse,
    #[error("Encoding Error")]
    Encoding,
    #[error("Item Already Exists")]
    ItemExists,
    #[error("Not Initialized")]
    NotInitialized,
    #[error("StrumParse {0}")]
    Strum(#[from] strum::ParseError),
}
