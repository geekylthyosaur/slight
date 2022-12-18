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
        match self.value {
            Value::Absolute(new, Step::To(Range { curr, .. })) => {
                let new = new as usize;
                let r: Box<dyn Iterator<Item = usize>> = match new.cmp(&curr) {
                    Ordering::Greater => Box::new(curr..=new),
                    Ordering::Less => Box::new((new..=curr).rev()),
                    Ordering::Equal => Box::new(std::iter::empty()),
                };
                r
            }
            Value::Absolute(v, Step::By(Range { curr, .. })) => {
                let new = (curr as isize).checked_add(v).unwrap_or(0) as usize;
                let r: Box<dyn Iterator<Item = usize>> = match new.cmp(&curr) {
                    Ordering::Greater => Box::new(curr..=new),
                    Ordering::Less => Box::new((new..=curr).rev()),
                    Ordering::Equal => Box::new(std::iter::empty()),
                };
                r
            }
            Value::Relative(_v, Step::To(Range { curr, max })) => {
                todo!()
            }
            Value::Relative(v, Step::By(Range { curr, max })) => {
                let r: Box<dyn Iterator<Item = usize>> = if v.is_sign_positive() {
                    Box::new(
                        (0..=max)
                            .filter(move |&v| v > curr)
                            .take((v * self.exponent) as usize),
                    )
                } else {
                    Box::new(
                        (0..=max)
                            .filter(move |&v| v < curr)
                            .rev()
                            .take((v.copysign(1.0) * self.exponent) as usize),
                    )
                };
                r
            }
        }
    }
}
