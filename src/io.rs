use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::{error::SlightError, value::Value};

pub type IOError = std::io::Error;

pub struct IO {
    path: PathBuf,
    pub value: Value,
}

pub const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
pub const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";
pub const MIN_BRIGHTNESS_FILENAME: &str = "min_brightness";

impl IO {
    pub fn try_new(path: &Path) -> Result<Self, SlightError> {
        let mut value = Value::from_str(&read(&path.join(CURRENT_BRIGHTNESS_FILENAME))?)?;
        let max_value = match read(&path.join(MAX_BRIGHTNESS_FILENAME)) {
            Ok(s) => Some(s.trim().parse::<i64>()?),
            _ => None,
        };
        if let Some(max) = max_value {
            value.max = max;
        }

        Ok(Self { path: path.to_path_buf(), value })
    }

    pub fn set_value(&mut self, value: i64) {
        self.value.ch(value)
    }

    pub fn get_value(&self) -> i64 {
        self.value.current
    }

    pub fn write(&self, filename: &str) -> Result<(), IOError> {
        std::fs::write(&self.path.join(filename), self.value.to_string())
    }

    pub fn read(&self) -> Result<i64, SlightError> {
        Ok(read(&self.path)?.trim().parse::<i64>()?)
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
