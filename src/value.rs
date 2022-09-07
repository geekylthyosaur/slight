pub type ParseError = std::num::ParseIntError;

pub struct Value {
    current: i64,
    max: i64,
    min: i64,
}

impl Value {
    pub fn new(current: i64, max: i64, min: i64) -> Self {
        Self { current, max, min }
    }

    pub fn iter_to(&mut self, thr: i64) -> impl Iterator<Item=i64> {
        self.current..thr
    }
}
