use std::path::Path;

use crate::{error::{SlightError, Result}, io::IO};

const PERCENT_MAX: usize = 100;
const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";

pub struct Brightness {
    current: usize,
    max: usize,
}

impl Brightness {
    pub fn set(&mut self, new: usize, path: &Path) -> Result<()> {
        // TODO: Any ways to avoid passing path here?
        if new <= self.max {
            return IO::write_number(&path.join(CURRENT_BRIGHTNESS_FILENAME), new)
                .and_then(|()| Ok(self.current = new));
        } // TODO: else
        Ok(())
    }

    pub fn as_value(&self) -> usize {
        self.current
    }

    pub fn as_percent(&self) -> usize {
        self.current * PERCENT_MAX / self.max
    }
}

impl TryFrom<&Path> for Brightness {
    type Error = SlightError;

    fn try_from(p: &Path) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            current: IO::read_number(&p.join(CURRENT_BRIGHTNESS_FILENAME))?,
            max: IO::read_number(&p.join(MAX_BRIGHTNESS_FILENAME))?,
        })
    }
}
