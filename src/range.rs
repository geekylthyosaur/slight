use std::cmp::Ordering;

pub struct Range {
    curr: usize,
    exponent: f32,
    max: usize,
}

impl Range {
    fn from_a_to_b(&self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        let r: Box<dyn Iterator<Item = usize>> = match new.cmp(&self.curr) {
            Ordering::Greater => Box::new(self.curr..=new),
            Ordering::Less => Box::new((new..=self.curr).rev()),
            Ordering::Equal => Box::new(std::iter::empty()),
        };
        r
    }

    fn by_percent(&self, diff_percent: f32, exponent: f32) -> Box<dyn Iterator<Item = usize> + '_> {
        let r: Box<dyn Iterator<Item = usize>> = if diff_percent.is_sign_positive() {
            Box::new(
                Range::exponential(self.max, exponent)
                    .filter(move |&v| v > self.curr)
                    .take((diff_percent) as usize),
            )
        } else {
            Box::new(
                Range::exponential(self.max, exponent)
                    .filter(move |&v| v < self.curr)
                    .rev()
                    .take((diff_percent.copysign(1.0)) as usize),
            )
        };
        r
    }

    fn exponential(max: usize, exponent: f32) -> Box<dyn DoubleEndedIterator<Item = usize>> {
        Box::new((0..=100).map(move |v: usize| {
            ((v as f32).powf(exponent) * 100f32.powf(-exponent) * max as f32) as usize
        }))
    }
}

pub enum Step {
    To(Range),
    By(Range),
}

pub enum Value {
    Absolute(isize, Step),
    Relative(f32, Step),
}

pub trait RangeBuilder {
    fn build(&self) -> Box<dyn Iterator<Item = usize> + '_>;
}

impl Range {
    pub fn new(curr: usize, max: usize, exponent: f32) -> Self {
        Self {
            curr,
            max,
            exponent,
        }
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

impl RangeBuilder for Value {
    fn build(&self) -> Box<dyn Iterator<Item = usize> + '_> {
        match self {
            Value::Absolute(new, Step::To(r)) => r.from_a_to_b(*new as usize),
            Value::Absolute(v, Step::By(r)) => {
                let new = (r.curr as isize).checked_add(*v).unwrap_or(0) as usize;
                r.from_a_to_b(new)
            }
            Value::Relative(percent, Step::To(r)) => {
                let new = (r.max as f32 / 100.0 * percent) as usize;
                r.from_a_to_b(new)
            }
            Value::Relative(percent, Step::By(r)) => r.by_percent(*percent, r.exponent),
        }
    }
}
