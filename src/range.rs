use std::cmp::Ordering;

pub struct Range {
    curr: usize,
    exponent: f32,
    max: usize,
}

impl Range {
    fn curr_to_new(&self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        let r: Box<dyn Iterator<Item = usize>> = match new.cmp(&self.curr) {
            Ordering::Greater => Box::new(self.curr..=new),
            Ordering::Less => Box::new((new..=self.curr).rev()),
            Ordering::Equal => Box::new(std::iter::empty()),
        };
        r
    }

    fn by_percent(&self, diff: f32) -> Box<dyn Iterator<Item = usize> + '_> {
        let r: Box<dyn Iterator<Item = usize>> = if diff.is_sign_positive() {
            Box::new(
                self.exponential()
                    .filter(move |&v| v > self.curr)
                    .take((diff) as usize),
            )
        } else {
            Box::new(
                self.exponential()
                    .filter(move |&v| v < self.curr)
                    .rev()
                    .take((diff.copysign(1.0)) as usize),
            )
        };
        r
    }

    fn exponential(&self) -> Box<dyn DoubleEndedIterator<Item = usize> + '_> {
        Box::new((0..=100).map(move |v: usize| {
            ((v as f32).powf(self.exponent) * 100f32.powf(-self.exponent) * self.max as f32)
                as usize
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
        // TODO dedup ends
        // 2 2 1 1 1 1 0 0 0 0 0 0 0 0 -> 2 2 1 1 1 1 0
        match self {
            Value::Absolute(new, Step::To(r)) => r.curr_to_new(*new as usize),
            Value::Absolute(v, Step::By(r)) => {
                let new = (r.curr as isize).checked_add(*v).unwrap_or(0) as usize;
                r.curr_to_new(new)
            }
            Value::Relative(percent, Step::To(r)) => {
                let new = (r.max as f32 / 100.0 * percent) as usize;
                r.curr_to_new(new)
            }
            Value::Relative(percent, Step::By(r)) => r.by_percent(*percent),
        }
    }
}
