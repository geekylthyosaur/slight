use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

use crate::{class::Class, io::IO};

const CURRENT_BRIGHTNESS_FILENAME: &'static str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &'static str = "max_brightness";

#[derive(Debug)]
pub struct Device {
    class: Class,
    id: Id,
    current_brightness: usize,
    max_brightness: usize,
}

impl Device {
    pub fn try_new(path: &Path) -> Result<Self, ()> {
        Ok(Device {
            class: path.try_into()?,
            id: path.try_into()?,
            current_brightness: IO::read_number(&path.join(CURRENT_BRIGHTNESS_FILENAME))?,
            max_brightness: IO::read_number(&path.join(MAX_BRIGHTNESS_FILENAME))?,
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
    type Error = todo!();

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        match IO::dir(p) {
            Some(s) => Ok(Id(s.to_owned())),
            None => Err(todo!()),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
