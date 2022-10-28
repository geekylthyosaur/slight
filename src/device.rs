use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

use crate::class::Class;

#[derive(Debug)]
pub struct Device {
    class: Class,
    id: String,
    current_brightness: usize,
    max_brightness: usize,
}

impl Device {
    const CURRENT_BRIGHTNESS_FILENAME: &'static str = "brightness";
    const MAX_BRIGHTNESS_FILENAME: &'static str = "max_brightness";

    pub fn new(path: &Path) -> Self {
        todo!("Read and init device in given path");
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} '{}': {}/{}",
            self.class, self.id, self.current_brightness, self.max_brightness
        )
    }
}
