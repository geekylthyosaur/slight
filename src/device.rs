use crate::class::Class;

use std::path::Path;

pub struct Device {
    class: Class,
    id: String,
    current_brightness: usize,
    max_brightness: usize,
}

impl Device {
    pub fn new(path: &Path) -> Self {
        todo!();
    }
}
