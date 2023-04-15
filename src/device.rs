use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

use crate::{
    brightness::Brightness,
    class::Class,
    error::{Result, SlightError},
    io::IO,
};

pub trait Toggle {
    fn is_toggleable(&self) -> bool;
    fn toggle(self, io: &mut IO) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Device {
    pub class: Class,
    pub id: Id,
    pub brightness: Brightness,
}

impl Toggle for Device {
    fn is_toggleable(&self) -> bool {
        self.brightness.max() == 1
    }

    fn toggle(mut self, io: &mut IO) -> Result<()> {
        if self.is_toggleable() {
            let new = self.brightness.as_value() ^ 1;
            self.brightness.set(new, io)
        } else {
            Err(SlightError::CannotToggle(self))
        }
    }
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
    type Error = SlightError;

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
