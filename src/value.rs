pub type ParseError = std::num::ParseIntError;

pub struct Value {
    current: i64,
    max: Option<i64>,
    min: Option<i64>,
}

impl Value {
    pub fn new(current: i64, max: Option<i64>, min: Option<i64>) -> Self {
        Self { current, max, min }
    }

    pub fn set_max(&mut self, max: i64) {
        self.max = Some(max);
    }

    pub fn set_min(&mut self, min: i64) {
        self.min = Some(min);
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
        Ok(Self {
            current: s.trim().parse::<i64>()?,
            max: None,
            min: None,
        })
    }
}
