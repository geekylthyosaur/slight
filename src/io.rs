use crate::{
    brightness::CURRENT_BRIGHTNESS_FILENAME,
    error::{Result, SlightError},
};

use std::path::Path;

pub struct IO {
    out: Box<dyn std::io::Write>,
}

impl IO {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Self {
            out: Box::new(std::fs::File::create(
                path.join(CURRENT_BRIGHTNESS_FILENAME),
            )?) as Box<dyn std::io::Write>,
        })
    }

    pub fn stdout() -> Self {
        Self {
            out: Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
        }
    }
}

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

    pub fn write_number(&mut self, value: usize) -> Result<()> {
        Ok(self.out.write_all(format!("{}\n", value).as_bytes())?)
    }

    pub fn dir(path: &Path) -> Option<&str> {
        path.is_dir().then_some(path.file_name()?.to_str()?)
    }
}
