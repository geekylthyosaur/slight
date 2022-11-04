use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::{Path, PathBuf},
};

type IOError = std::io::Error;
type ParseError = std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, SlightError>;

#[derive(Debug)]
pub enum SlightError {
    DeviceBroken(PathBuf),
    IO(IOError),
    Parse, // TODO: Say where error occured
}

impl From<&Path> for SlightError {
    fn from(p: &Path) -> Self {
        Self::DeviceBroken(p.to_path_buf())
    }
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
            Self::DeviceBroken(p) => write!(f, "No valid device at {}", p.display()),
            Self::IO(e) => write!(f, "{}", e),
            Self::Parse => write!(f, "Given file has invalid data"),
        }
    }
}
