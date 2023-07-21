use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Backlight,
    Led,
}

impl Class {
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
