use crate::error::{Error, Result};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::Neg;

pub struct Range {
    curr: usize,
    exponent: f32,
    max: usize,
}

impl Range {
    pub fn new(curr: usize, max: usize, exponent: f32) -> Self {
        Self {
            curr,
            exponent,
            max,
        }
    }

    pub fn try_from_input(self, s: Cow<str>) -> Result<Box<dyn RangeBuilder>> {
        let mut chars = s.chars().peekable();

        let (r, sign) = if chars.next_if_eq(&'-').is_some() {
            (self.by(), -1.0)
        } else if chars.next_if_eq(&'+').is_some() {
            (self.by(), 1.0)
        } else {
            (self.to(), 1.0)
        };

        let s = chars
            .clone()
            .take_while(|&c| c != '%')
            .collect::<Cow<str>>();
        let v = sign * s.parse::<f32>().map_err(|_| Error::InvalidInput)?;
        Ok(Box::new(if let Some('%') = chars.last() {
            r.relative(v)
        } else {
            r.absolute(v)
        }))
    }

    fn curr_to_new(&self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        match new.cmp(&self.curr) {
            Ordering::Greater => Box::new((self.curr..=new).skip(1)),
            Ordering::Less => Box::new((new..=self.curr).rev().skip(1)),
            Ordering::Equal => Box::new(std::iter::empty()),
        }
    }

    fn by_percent(&self, diff: f32) -> Box<dyn Iterator<Item = usize> + '_> {
        if diff.is_sign_positive() {
            Box::new(
                self.exponential()
                    .filter(|&v| v > self.curr)
                    .take(diff as usize),
            )
        } else {
            Box::new(
                self.exponential()
                    .filter(|&v| v < self.curr)
                    .rev()
                    .take((diff.abs()) as usize),
            )
        }
    }

    fn exponential(&self) -> Box<dyn DoubleEndedIterator<Item = usize> + '_> {
        Box::new((0..=100).map(|v: usize| {
            ((v as f32).powf(self.exponent) * 100f32.powf(self.exponent.neg()) * self.max as f32)
                as usize
        }))
    }
}

pub enum Step {
    To(Range),
    By(Range),
}

pub enum Value {
    Absolute(f32, Step),
    Relative(f32, Step),
}

pub trait RangeBuilder {
    fn build(&self) -> Box<dyn Iterator<Item = usize> + '_>;
}

impl Range {
    pub fn to(self) -> Step {
        Step::To(self)
    }

    pub fn by(self) -> Step {
        Step::By(self)
    }
}

impl Step {
    pub fn absolute(self, v: f32) -> Value {
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
            Value::Absolute(new, Step::To(r)) => {
                let new = usize::min(*new as usize, r.max);
                r.curr_to_new(new)
            },
            Value::Absolute(v, Step::By(r)) => {
                let new = r.curr.saturating_add_signed(*v as isize);
                let new = usize::min(new, r.max);
                r.curr_to_new(new)
            }
            Value::Relative(percent, Step::To(r)) => {
                let new = r.max as f32 / 100.0 * percent;
                r.curr_to_new(new as usize)
            }
            Value::Relative(percent, Step::By(r)) => r.by_percent(*percent),
        }
    }
}
