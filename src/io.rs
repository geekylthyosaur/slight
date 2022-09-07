use std::path::Path;

use crate::{
    error::SlightError,
    value::Value,
};

pub type IOError = std::io::Error;

pub struct IO {
    path: Path,
    value: Value,
}

impl IO {
    pub fn new(path: Path, value: Value) -> Self {
        Self { path, value }
    }

    pub fn read(&self) -> Result<String, IOError> {
        Ok(String::from_utf8_lossy(&std::fs::read(self.path)?)
            .as_ref()
            .to_owned())
    }

    pub fn read_num(&self) -> Result<i64, SlightError> {
        Ok(self.read()?.trim().parse::<i64>()?)
    }

    pub fn write(&self) -> Result<(), IOError> {
        std::fs::write(self.path, self.value.to_string())
    }
}

