use crate::error::{Result, SlightError};

use std::path::Path;

pub struct IO;

impl IO {
    pub fn scan(path: &Path) -> Result<Vec<String>> {
        Ok(path.read_dir().map(|v| {
            v.filter_map(|v| v.ok())
                .map(|v| v.file_name().into_string())
                .filter_map(|v| v.ok())
                .collect()
        })?)
    }

    pub fn read_number(path: &Path) -> Result<usize> {
        String::from_utf8_lossy(
            &std::fs::read(path).map_err(|e| SlightError::ReadNumber(path.to_path_buf(), e))?,
        )
        .trim()
        .parse::<usize>()
        .map_err(|_| SlightError::ParseNumber(path.to_path_buf()))
    }

    pub fn write_number(path: &Path, value: usize) -> Result<()> {
        Ok(std::fs::write(path, value.to_string())?)
    }

    pub fn dir(path: &Path) -> Option<&str> {
        path.is_dir().then_some(path.file_name()?.to_str()?)
    }
}
