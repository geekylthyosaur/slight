use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Invalid input!")]
    InvalidInput,
    #[error("Cannot toggle '{}' as it can have more than two values ({}/{})!", .0.id(), .0.brightness().current, .0.brightness().max)]
    CannotToggle(crate::device::Device),
    #[error("No devices found!")]
    NoDevices,
    #[error("No suitable device found!")]
    SuitableDeviceNotFound,
    #[error("The specified device was not found!")]
    SpecifiedDeviceNotFound,
    #[error("Missing write permissions!")]
    Permission,
}
