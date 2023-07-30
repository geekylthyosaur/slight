use crate::{device::Device, error::Result};

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

const BASE_PATH: &str = "/sys/class";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Backlight,
    Led,
}

impl Class {
    pub fn scan(self) -> Result<Vec<Result<Device>>> {
        let path = Path::new(BASE_PATH).join(self.filename());
        Ok(path
            .read_dir()
            .map(|v| {
                v.filter_map(std::result::Result::ok)
                    .map(|v| v.file_name().into_string())
                    .filter_map(std::result::Result::ok)
            })
            .map(|ids| {
                ids.map(|id| {
                    let path = path.join(id);
                    Device::new(&path)
                })
                .collect::<Vec<_>>()
            })?)
    }

    pub fn filename(self) -> &'static str {
        match self {
            Self::Backlight => "backlight",
            Self::Led => "leds",
        }
    }

    pub fn from_filename(s: &str) -> Option<Self> {
        match s {
            "backlight" => Some(Self::Backlight),
            "leds" => Some(Self::Led),
            _ => None,
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Backlight => write!(f, "Backlight"),
            Self::Led => write!(f, "Led"),
        }
    }
}
