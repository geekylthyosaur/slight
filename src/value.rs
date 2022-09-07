use std::string::ToString;

pub struct Value {
    current: i64,
    max: i64,
    min: i64,
}

impl Value {
    pub fn new(current: i64, max: i64, min: i64) -> Self {
        Self { current, max, min }
    }

    pub fn ch(&mut self, new: i64) {
        self.current = new;
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.current.to_string()
    }
}
