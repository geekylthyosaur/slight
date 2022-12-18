use std::cmp::Ordering;

pub struct Range {
    curr: usize,
    max: usize,
}

pub enum Step {
    To(Range),
    By(Range),
}

pub enum Value {
    Absolute(isize, Step),
    Relative(f32, Step),
}

pub struct Exponential {
    value: Value,
    exponent: f32,
}

pub trait RangeBuilder {
    fn build(&self) -> Box<dyn Iterator<Item = usize>>;
}

impl Range {
    pub fn new(curr: usize, max: usize) -> Self {
        Self { curr, max }
    }

    pub fn to(self) -> Step {
        Step::To(self)
    }

    pub fn by(self) -> Step {
        Step::By(self)
    }
}

impl Step {
    pub fn absolute(self, v: isize) -> Value {
        Value::Absolute(v, self)
    }

    pub fn relative(self, v: f32) -> Value {
        Value::Relative(v, self)
    }
}

impl Value {
    pub fn exp(self, exponent: f32) -> Exponential {
        Exponential {
            value: self,
            exponent,
        }
    }
}

impl RangeBuilder for Value {
    fn build(&self) -> Box<dyn Iterator<Item = usize>> {
        todo!()
    }
}

impl RangeBuilder for Exponential {
    fn build(&self) -> Box<dyn Iterator<Item = usize>> {
        todo!()
    }
}

/*
impl Range {
    pub fn to_value(&self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        let r: Box<dyn Iterator<Item = usize>>;
        match new.cmp(&self.curr) {
            Ordering::Greater => r = Box::new(self.curr..=new),
            Ordering::Less => r = Box::new((new..=self.curr).rev()),
            Ordering::Equal => r = Box::new(std::iter::empty()),
        }
        r
    }

    pub fn by_value(&self, diff: isize) -> impl Iterator<Item = usize> {
        let new = (self.curr as isize).checked_add(diff).unwrap_or(0) as usize;
        self.to_value(new)
    }

    pub fn by_percent_exp(&self, percent: f32, exp: f32) -> Box<dyn Iterator<Item = usize>> {
        // TODO:
        let (curr, max) = (self.curr, self.max);
        let r: Box<dyn Iterator<Item = usize>> = match percent.is_sign_positive() {
            true => Box::new(
                (0..=max)
                    .filter(move |&v| v > curr)
                    .take((percent * exp) as usize),
            ),
            false => Box::new(
                (0..=max)
                    .filter(move |&v| v < curr)
                    .rev()
                    .take((percent.copysign(1.0) * exp) as usize),
            ),
        };
        r
    }
}
*/
