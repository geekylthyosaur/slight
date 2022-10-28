pub type ParseError = std::num::ParseIntError;

pub struct Value {
    current: usize,
    max: i32,
}

impl Value {
    pub fn new(current: i32, max: Option<i32>) -> Self {
        if let Some(max) = max {
            return Self {
                current,
                max,
                ..Default::default()
            };
        }
        Self {
            current,
            ..Default::default()
        }
    }

    pub fn set(&mut self, new: i32) -> Option<i32> {
        if (self.min..self.max).contains(&new) {
            self.current = new;
            return Some(new);
        }
        None
    }

    pub fn get(&self) -> i32 {
        self.current
    }

    pub fn max(&self) -> i32 {
        self.max
    }
}

/*
impl std::io::Write for Value {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    }

    fn flush(&mut self) -> std::io::Result<()> {
    }
}

impl std::io::Read for Value {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    }
}
*/

impl From<Percent> for Value {
    fn from(p: Percent) -> Self {
        (p.powf(4.) * 255. * f32::powf(255., -4.) as f32) as i32
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
