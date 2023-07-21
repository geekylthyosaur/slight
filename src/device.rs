use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

use crate::{brightness::Brightness, class::Class, error::{Error, Result}, io::IO, ToggleState};

#[derive(Debug, Clone)]
pub struct Device {
    pub class: Class,
    pub id: Id,
    pub brightness: Brightness,
}

impl Device {
    pub fn new(class: Class, path: &Path) -> Result<Self> {
        Ok(Self {
            class,
            id: path.try_into()?,
            brightness: path.try_into()?,
        })
    }

    pub fn my_path(&self) -> PathBuf {
        PathBuf::from(self.class).join(&self.id.0)
    }

    fn is_toggleable(&self) -> bool {
        self.brightness.max() == 1
    }

    pub fn toggle(mut self, io: &mut IO, state: Option<ToggleState>) -> Result<()> {
        if self.is_toggleable() {
            let new = if let Some(ToggleState::On) = state {
                self.brightness.max()
            } else if let Some(ToggleState::Off) = state {
                0
            } else {
                self.brightness.as_value() ^ 1
            };
            self.brightness.set(new, io)
        } else {
            Err(Error::CannotToggle {
                id: self.id.to_string(),
                curr: self.brightness.as_value(),
                max: self.brightness.max(),
            })
        }
    }

    pub fn select(devices: &[Device], id: Option<Id>) -> Result<&Device> {
        if let Some(id) = id {
            Self::find(devices, &id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::find_default(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    pub fn find<'a>(devices: &'a [Device], id: &Id) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id == id.as_ref())
    }

    fn find_default(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class == Class::Backlight)
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} '{}': {}", self.class, self.id, self.brightness)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id(String);

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl TryFrom<&Path> for Id {
    type Error = Error;

    fn try_from(p: &Path) -> Result<Self> {
        match IO::dir(p) {
            Some(s) => Ok(Id(s.to_owned())),
            None => Err(p.into()),
        }
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
