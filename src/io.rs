use std::path::Path;

use crate::error::SlightError;

pub type IOError = std::io::Error;

pub fn read(path: &Path) -> Result<String, IOError> {
    Ok(String::from_utf8_lossy(&std::fs::read(path)?)
        .as_ref()
        .to_owned())
}

pub fn read_num(path: &Path) -> Result<i64, SlightError> {
    Ok(read(path)?.trim().parse::<i64>()?)
}

pub fn write(path: &String, value: u8) -> Result<(), IOError> {
    std::fs::write(path, value.to_string())
}
