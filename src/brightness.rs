use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

use crate::{
    error::{Result, SlightError},
    io::IO,
};

const PERCENT_MAX: f32 = 100.0;
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

    pub fn as_percent(&self, exponent: f32) -> usize {
        value_to_percent(self.current, self.max, exponent)
    }
}

pub fn value_to_percent(value: usize, max: usize, exponent: f32) -> usize {
    (f32::powf(value as f32 / max as f32, 1.0 / exponent) * PERCENT_MAX) as usize
}

pub fn percent_to_value(percent: usize, max: usize, exponent: f32) -> usize {
    (f32::powf(percent as f32 / PERCENT_MAX, exponent) * max as f32) as usize
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
