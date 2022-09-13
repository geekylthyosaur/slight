pub type ParseError = std::num::ParseIntError;

pub struct Value {
    pub current: i64,
    pub max: i64,
    pub min: i64,
}

impl Value {
    pub fn new(current: i64) -> Self {
        Self { current, ..Default::default() }
    }

    pub fn ch(&mut self, new: i64) {
        self.current = new;
    }
}

impl std::string::ToString for Value {
    fn to_string(&self) -> String {
        self.current.to_string()
    }
}

impl std::str::FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.trim().parse::<i64>()?,
        ))
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
