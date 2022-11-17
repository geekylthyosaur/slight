use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

use crate::{brightness::Brightness, class::Class, error::SlightError, io::IO};

#[derive(Debug)]
pub struct Device {
    pub class: Class,
    pub id: Id,
    pub brightness: Brightness,
}

impl Device {
    pub fn my_path(&self) -> PathBuf {
        PathBuf::from(&self.class).join(&self.id.0)
    }
}

impl TryFrom<&Path> for Device {
    type Error = SlightError;

    fn try_from(p: &Path) -> std::result::Result<Self, Self::Error> {
        Ok(Device {
            class: p.try_into()?,
            id: p.try_into()?,
            brightness: p.try_into()?,
        })
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} '{}': {}", self.class, self.id, self.brightness)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Id(String);

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl TryFrom<&Path> for Id {
    type Error = SlightError;

    // TODO: std::result
    fn try_from(p: &Path) -> std::result::Result<Self, Self::Error> {
        match IO::dir(p) {
            Some(s) => Ok(Id(s.to_owned())),
            None => Err(p.into()),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
