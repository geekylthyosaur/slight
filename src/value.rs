use std::str::FromStr;
use std::num::ParseIntError;

pub struct Value {
    max: Option<i64>,
    min: Option<i64>,
    v: i64,
}

impl Value {
    pub fn inc_by(&mut self, d: i64) {
        self.v += d
    }
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value {
            max: None,
            min: None,
            v: s.trim().parse()?,
        })
    }
}
    
