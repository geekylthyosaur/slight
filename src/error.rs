use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Invalid input!")]
    InvalidInput,
    #[error("Cannot toggle '{id}' as it can have more than two values ({curr}/{max})!")]
    CannotToggle { id: String, curr: usize, max: usize },
    #[error("No devices found!")]
    NoDevices,
    #[error("No suitable device found!")]
    SuitableDeviceNotFound,
    #[error("The specified device was not found!")]
    SpecifiedDeviceNotFound,
}
