use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};

const PATH: &str = "/sys/class";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Backlight,
    Led,
}

impl Class {
    fn name(self) -> &'static str {
        match self {
            Self::Backlight => "backlight",
            Self::Led => "leds",
        }
    }
}

impl From<Class> for PathBuf {
    fn from(c: Class) -> Self {
        Path::new(PATH).join(c.name())
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
