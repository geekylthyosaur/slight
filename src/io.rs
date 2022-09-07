use std::path::Path;
use std::str::FromStr;

use crate::{error::SlightError, value::Value};

pub type IOError = std::io::Error;

pub struct IO<'a> {
    path: &'a Path,
    value: Value,
}

const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";
const MIN_BRIGHTNESS_FILENAME: &str = "min_brightness";

impl<'a> IO<'a> {
    pub fn try_new(path: &'a Path) -> Result<Self, SlightError> {
        let value = Value::from_str(&read(&path.join(CURRENT_BRIGHTNESS_FILENAME))?)?;

        Ok(Self { path, value })
    }

    pub fn write(&self) -> Result<(), IOError> {
        std::fs::write(self.path, self.value.to_string())
    }

    pub fn read(&self) -> Result<i64, SlightError> {
        Ok(read(self.path)?.trim().parse::<i64>()?)
    }
}

fn read(path: &Path) -> Result<String, IOError> {
    Ok(String::from_utf8_lossy(&std::fs::read(path)?)
        .as_ref()
        .to_owned())
}

fn write(path: &Path, s: &str) -> Result<(), IOError> {
    std::fs::write(path, s)
}
