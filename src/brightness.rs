use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

use crate::{
    error::{Result, SlightError},
    io::IO,
};

const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";

#[derive(Debug, Clone)]
pub struct Brightness {
    current: usize,
    max: usize,
}

impl Brightness {
    pub fn set(&mut self, new: usize, path: &Path) -> Result<()> {
        if new != self.current && new <= self.max {
            return IO::write_number(&path.join(CURRENT_BRIGHTNESS_FILENAME), new)
                .map(|_| self.current = new);
        } // TODO: else
        Ok(())
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn as_value(&self) -> usize {
        self.current
    }

    pub fn as_percent(&self) -> usize {
        ((self.current as f32 / self.max as f32) * 100.0) as usize
    }
}

impl TryFrom<&Path> for Brightness {
    type Error = SlightError;

    fn try_from(p: &Path) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            current: IO::read_number(&p.join(CURRENT_BRIGHTNESS_FILENAME))?,
            max: IO::read_number(&p.join(MAX_BRIGHTNESS_FILENAME))?,
        })
    }
}

impl Display for Brightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}/{}", self.current, self.max)
    }
}
