use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy)]
pub struct Brightness {
    pub current: usize,
    pub max: usize,
}

impl Brightness {
    pub fn new(current: usize, max: usize) -> Self {
        Self { current, max }
    }
}

impl Display for Brightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}/{}", self.current, self.max)
    }
}
