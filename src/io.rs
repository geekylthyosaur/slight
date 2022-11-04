use crate::error::Result;

use std::path::Path;

pub struct IO;

impl IO {
    pub fn scan(path: &Path) -> Result<Vec<String>> {
        match path.read_dir() {
            Ok(v) => {
                // TODO: make this cleaner
                Ok(v.filter(|v| v.is_ok())
                    .map(|v| v.unwrap())
                    // TODO: this check is unnececary but returns []
                    // .filter(|v| v.file_type().unwrap().is_dir())
                    .map(|v| v.file_name().into_string())
                    .filter(|v| v.is_ok())
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>())
            }
            Err(_) => todo!(),
        }
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
        if path.is_dir() {
            path.file_name()?.to_str()
        } else {
            None
        }
    }

    pub fn parent_dir(path: &Path) -> Option<&str> {
        path.parent()?.file_name()?.to_str()
    }
}
