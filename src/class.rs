use strum::EnumIter;

use crate::{error::SlightError, io::IO};

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

const PATH: &str = "/sys/class";
const BACKLIGHT: &str = "backlight";
const LED: &str = "leds";

#[derive(Debug, EnumIter)]
pub enum Class {
    Backlight,
    Led,
}

impl From<&Class> for PathBuf {
    fn from(c: &Class) -> Self {
        let path = Path::new(PATH);
        match c {
            Class::Backlight => path.join(BACKLIGHT),
            Class::Led => path.join(LED),
        }
    }
}

impl TryFrom<&Path> for Class {
    type Error = SlightError;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        match IO::parent_dir(p) {
            Some(BACKLIGHT) => Ok(Class::Backlight),
            Some(LED) => Ok(Class::Led),
            Some(_) | None => Err(p.into()),
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
