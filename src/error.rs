use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::io::IOError;
use crate::value::ParseError;

pub type Result<T> = std::result::Result<T, SlightError>;

#[derive(Debug)]
pub enum SlightError {
    IO(IOError),
    Parse,
}

impl From<IOError> for SlightError {
    fn from(e: IOError) -> Self {
        Self::IO(e)
    }
}

impl From<ParseError> for SlightError {
    fn from(_: ParseError) -> Self {
        Self::Parse
    }
}

impl Display for SlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IO(e) => write!(f, "{}", e),
            Self::Parse => write!(f, "Given file has invalid data"),
        }
    }
}
