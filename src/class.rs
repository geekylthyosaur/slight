use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Class {
    Backlight,
    Led,
}

impl Class {
    const PATH: &'static Path = Path::new("/sys/class");

    pub fn path(&self) -> PathBuf {
        match self {
            Self::Backlight => Class::PATH.join("backlight"),
            Self::Led => Class::PATH.join("leds"),
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
