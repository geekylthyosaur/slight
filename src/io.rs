use crate::error::Result;

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
        Ok(String::from_utf8_lossy(&std::fs::read(path)?)
            .trim()
            .parse::<usize>()?)
    }

    pub fn write_number(path: &Path, value: usize) -> Result<()> {
        Ok(std::fs::write(path, value.to_string())?)
    }

    pub fn dir(path: &Path) -> Option<&str> {
        path.is_dir().then_some(path.file_name()?.to_str()?)
    }

    pub fn parent_dir(path: &Path) -> Option<&str> {
        path.parent()?.file_name()?.to_str()
    }
}
