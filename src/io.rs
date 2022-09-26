use std::path::{Path, PathBuf};

use crate::{error::SlightError, value::Value};

pub type IOError = std::io::Error;

pub const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
pub const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";
pub const MIN_BRIGHTNESS_FILENAME: &str = "min_brightness";

pub struct IO {
    path: PathBuf,
    value: Value,
}

impl IO {
    pub fn try_new(path: &Path) -> Result<Self, SlightError> {
        let current = read(&path.join(CURRENT_BRIGHTNESS_FILENAME))?;
        let max_value = read(&path.join(MAX_BRIGHTNESS_FILENAME)).ok();
        let min_value = read(&path.join(MIN_BRIGHTNESS_FILENAME)).ok();

        let value = Value::new(current, max_value, min_value);

        Ok(Self {
            path: path.to_path_buf(),
            value,
        })
    }

    pub fn set_value(&mut self, value: i64) -> Result<(), SlightError> {
        if let Some(new) = self.value.set(value) {
            write(&self.path.join(CURRENT_BRIGHTNESS_FILENAME), new)?
        }
        Ok(())
    }

    pub fn get_value(&self) -> i64 {
        self.value.get()
    }

    pub fn max_value(&self) -> i64 {
        self.value.max()
    }
}

fn write(path: &Path, value: i64) -> Result<(), IOError> {
    std::fs::write(path, value.to_string())
}

fn read(path: &Path) -> Result<i64, SlightError> {
    Ok(String::from_utf8_lossy(&std::fs::read(path)?)
        .as_ref()
        .to_owned()
        .trim()
        .parse::<i64>()?)
}
