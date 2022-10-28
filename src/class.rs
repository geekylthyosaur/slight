use std::path::PathBuf;

const PATH: PathBuf = PathBuf::from("/sys/class");

pub enum Class {
    Backlight,
    Led,
}

impl Class {
    pub fn path(&self) -> PathBuf {
        match self {
            Self::Backlight => {
                PATH.push("backlight");
                PATH
            }
            Self::Led => {
                PATH.push("leds");
                PATH
            }
        }
    }
}
