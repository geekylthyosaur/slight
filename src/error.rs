use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::{Path, PathBuf},
};

type IOError = std::io::Error;

pub type Result<T> = std::result::Result<T, SlightError>;

#[derive(Debug)]
pub enum SlightError {
    DeviceBroken(PathBuf),
    ReadNumber(PathBuf, IOError),
    ParseNumber(PathBuf),
    IO(IOError),
    NoInput,
    NoDevices,
    NoSuitableDeviceFound,
    NoSpecifiedDeviceFound,
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

impl Display for SlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DeviceBroken(p) => write!(f, "No valid device at {}", p.display()),
            Self::ReadNumber(p, e) => write!(f, "Cannot read {}: {}", p.display(), e),
            Self::ParseNumber(p) => write!(f, "{} has invalid data", p.display()),
            Self::IO(e) => write!(f, "{}", e),
            Self::NoInput => write!(f, "No input was provided!"),
            Self::NoDevices => write!(f, "No devices found!"),
            Self::NoSuitableDeviceFound => write!(f, "No suitable device found!"),
            Self::NoSpecifiedDeviceFound => write!(f, "No specified device found!"),
        }
    }
}
