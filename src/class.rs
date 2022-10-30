use crate::{error::SlightError, io::IO};

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

const PATH: &str = "/sys/class";
const BACKLIGHT: &str = "backlight";
const LED: &str = "leds";

#[derive(Debug)]
pub enum Class {
    Backlight,
    Led,
}

impl Class {
    pub fn path(&self) -> PathBuf {
        let path = Path::new(PATH);
        match self {
            Self::Backlight => path.join(BACKLIGHT),
            Self::Led => path.join(LED),
        }
    }
}

impl TryFrom<&Path> for Class {
    type Error = SlightError;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        match IO::parent_dir(p) {
            Some(BACKLIGHT) => Ok(Class::Backlight),
            Some(LED) => Ok(Class::Led),
            Some(_) | None => Err(SlightError::DeviceBroken(p.to_path_buf())),
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
