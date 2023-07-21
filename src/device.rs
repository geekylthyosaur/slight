use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

use crate::{
    brightness::Brightness,
    class::Class,
    error::{Error, Result},
    io::IO,
    ToggleState,
};

#[derive(Debug, Clone)]
pub struct Device(udev::Device);

impl Device {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Device(udev::Device::from_syspath(path)?))
    }

    pub fn my_path(&self) -> PathBuf {
        self.0.syspath().into()
    }

    fn is_toggleable(&self) -> bool {
        self.brightness().max() == 1
    }

    pub fn toggle(self, io: &mut IO, state: Option<ToggleState>) -> Result<()> {
        if self.is_toggleable() {
            let new = if let Some(ToggleState::On) = state {
                self.brightness().max()
            } else if let Some(ToggleState::Off) = state {
                0
            } else {
                self.brightness().as_value() ^ 1
            };
            self.brightness().set(new, io)
        } else {
            Err(Error::CannotToggle {
                id: self.id().to_string(),
                curr: self.brightness().as_value(),
                max: self.brightness().max(),
            })
        }
    }

    pub fn brightness(&self) -> Brightness {
        let current = self
            .0
            .attribute_value("brightness")
            .and_then(OsStr::to_str)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();
        let max = self
            .0
            .attribute_value("max_brightness")
            .and_then(OsStr::to_str)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();
        Brightness::new(current, max)
    }

    pub fn class(&self) -> Class {
        match self.0.subsystem().and_then(OsStr::to_str) {
            Some("backlight") => Class::Backlight,
            Some("leds") => Class::Led,
            c => unreachable!("{:?}", c),
        }
    }

    pub fn id(&self) -> Id {
        self.0.sysname().to_string_lossy().to_string().into()
    }

    pub fn select(devices: &[Device], id: Option<Id>) -> Result<&Device> {
        if let Some(id) = id {
            Self::find(devices, &id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::find_default(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    pub fn find<'a>(devices: &'a [Device], id: &Id) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id() == id.as_ref())
    }

    fn find_default(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class() == Class::Backlight)
    }

    pub fn all() -> Vec<Result<Self>> {
        [Class::Backlight, Class::Led]
            .iter()
            .flat_map(|&class| {
                let path = PathBuf::from(class);
                path.read_dir()
                    .map(|v| {
                        v.filter_map(std::result::Result::ok)
                            .map(|v| v.file_name().into_string())
                            .filter_map(std::result::Result::ok)
                    })
                    .map(|ids| {
                        ids.map(|id| {
                            let path = path.join(id);
                            Self::new(&path)
                        })
                        .collect::<Vec<_>>()
                    })
            })
            .flatten()
            .collect()
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} '{}': {}", self.class(), self.id(), self.brightness())
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
