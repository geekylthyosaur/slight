use crate::io::IO;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

const PATH: &'static str = "/sys/class";
const BACKLIGHT: &'static str = "backlight";
const LED: &'static str = "leds";

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
    type Error = todo!();

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        match IO::parent_dir(p) {
            Some(BACKLIGHT) => Ok(Class::Backlight),
            Some(LED) => Ok(Class::Led),
            Some(_) => Err(todo!()),
            None => Err(todo!()),
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
