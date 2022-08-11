use std::num::ParseIntError;
use std::str::FromStr;

pub struct Value(i64);
pub type ParseError = ParseIntError;

impl Value {
    pub fn inc_by(&mut self, d: i64) {
        self.0 += d
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value(s.trim().parse()?))
    }
}
