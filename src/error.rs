use crate::{brightness::Brightness, device::Id};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Cannot toggle '{}' as it can have more than two values ({}/{})!", .id, .brightness.current, .brightness.max)]
    CannotToggle { id: Id, brightness: Brightness },
    #[error(transparent)]
    Dbus(#[from] dbus::Error),
    #[error("No devices found!")]
    NoDevices,
    #[error("No suitable device found!")]
    SuitableDeviceNotFound,
    #[error("The specified device was not found!")]
    SpecifiedDeviceNotFound,
    #[error("Missing write permissions!")]
    Permission,
}
