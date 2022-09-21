pub type ParseError = std::num::ParseIntError;

pub struct Value {
    current: i64,
    max: i64,
    min: i64,
}

impl Value {
    pub fn new(current: i64, max: Option<i64>, min: Option<i64>) -> Self {
        if let Some(max) = max {
            return Self { current, max, ..Default::default() }
        }
        Self { current, ..Default::default() }
    }

    pub fn get(&self) -> i64 {
        self.current
    }

    pub fn set(&mut self, new: i64) {
        if (self.min..self.max).contains(&new) {
            self.current = new
        }
    }
}

impl std::string::ToString for Value {
    fn to_string(&self) -> String {
        self.current.to_string()
    }
}

impl Default for Value {
    fn default() -> Self {
        Self {
            current: 0,
            max: 255,
            min: 0,
        }
    }
}
