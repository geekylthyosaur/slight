use std::{
    num::ParseIntError,
    io::Error as IOError,
    fmt::{Display, Formatter, Result as FmtResult},
};

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

impl From<ParseIntError> for SlightError {
    fn from(_: ParseIntError) -> Self {
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
