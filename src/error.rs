use std::path::{Path, PathBuf};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No valid device at {0}!")]
    DeviceBroken(PathBuf),
    #[error("Cannot read {0}: {1}!")]
    ReadNumber(PathBuf, std::io::Error),
    #[error("{0} has invalid data!")]
    ParseNumber(PathBuf),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Invalid input!")]
    InvalidInput,
    #[error("Cannot toggle '{id}' as it can have more than two values ({curr}/{max})!")]
    CannotToggle { id: String, curr: usize, max: usize },
    #[error("No input was provided!")]
    NoInput,
    #[error("No devices found!")]
    NoDevices,
    #[error("No suitable device found!")]
    SuitableDeviceNotFound,
    #[error("The specified device was not found!")]
    SpecifiedDeviceNotFound,
}

impl From<&Path> for Error {
    fn from(p: &Path) -> Self {
        Self::DeviceBroken(p.to_path_buf())
    }
}
