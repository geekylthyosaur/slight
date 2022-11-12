use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

use crate::{
    class::Class,
    error::{Result, SlightError},
    io::IO,
};

const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";

#[derive(Debug)]
pub struct Device {
    class: Class,
    id: Id,
    current_brightness: usize,
    max_brightness: usize,
}

impl Device {
    pub fn set_brightness(&mut self, new: usize) -> Result<()> {
        if new <= self.max_brightness {
            return match IO::write_number(&self.my_path().join(CURRENT_BRIGHTNESS_FILENAME), new) {
                Ok(v) => {
                    self.current_brightness = new;
                    Ok(v)
                }
                e => e,
            };
        }
        Ok(())
    }

    fn my_path(&self) -> PathBuf {
        PathBuf::from(&self.class).join(&self.id.0)
    }
}

impl TryFrom<&Path> for Device {
    type Error = SlightError;

    fn try_from(p: &Path) -> std::result::Result<Self, Self::Error> {
        Ok(Device {
            class: p.try_into()?,
            id: p.try_into()?,
            current_brightness: IO::read_number(&p.join(CURRENT_BRIGHTNESS_FILENAME))?,
            max_brightness: IO::read_number(&p.join(MAX_BRIGHTNESS_FILENAME))?,
        })
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} '{}': {}/{}",
            self.class, self.id, self.current_brightness, self.max_brightness
        )
    }
}

#[derive(Debug)]
pub struct Id(String);

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
