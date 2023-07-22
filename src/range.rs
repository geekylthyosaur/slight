use crate::error::{Error, Result};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::Neg;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub struct Range {
    curr: usize,
    exponent: f32,
    max: usize,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub enum Step {
    To(Range),
    By(Range),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Value {
    Absolute(f32, Step),
    Relative(f32, Step),
}

impl Range {
    pub fn new(curr: usize, max: usize, exponent: f32) -> Self {
        Self {
            curr,
            exponent,
            max,
        }
    }

    pub fn try_from_input(self, input: &str) -> Result<Box<dyn Iterator<Item = usize>>> {
        let range = self.parse_input(input)?;

        Ok(range.iter())
    }

    fn parse_input(&self, input: &str) -> Result<Value> {
        let mut chars = input.chars().peekable();

        let (range, sign) = chars
            .next_if_eq(&'-')
            .map(|_| (self.by(), -1.0))
            .unwrap_or_else(|| {
                chars
                    .next_if_eq(&'+')
                    .map_or((self.to(), 1.0), |_| (self.by(), 1.0))
            });

        let input = chars
            .clone()
            .take_while(|&c| c != '%')
            .collect::<Cow<str>>()
            .parse::<f32>()
            .map(|v| v.copysign(sign))
            .map_err(|_| Error::InvalidInput)?;

        Ok(chars
            .last()
            .is_some_and(|c| c == '%')
            .then(|| range.relative(input))
            .unwrap_or(range.absolute(input)))
    }

    fn curr_to_new(self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        match new.cmp(&self.curr) {
            Ordering::Greater => Box::new((self.curr..=new).skip(1)),
            Ordering::Less => Box::new((new..=self.curr).rev().skip(1)),
            Ordering::Equal => Box::new(std::iter::empty()),
        }
    }

    fn by_percent(self, diff: f32) -> Box<dyn Iterator<Item = usize>> {
        if diff.is_sign_positive() {
            Box::new(
                self.exponential()
                    .filter(move |&v| v > self.curr)
                    .take(diff as usize),
            )
        } else {
            Box::new(
                self.exponential()
                    .filter(move |&v| v < self.curr)
                    .rev()
                    .take(diff.abs() as usize),
            )
        }
    }

    fn exponential(self) -> Box<dyn DoubleEndedIterator<Item = usize>> {
        Box::new((0..=100).map(move |v: usize| {
            ((v as f32).powf(self.exponent) * 100f32.powf(self.exponent.neg()) * self.max as f32)
                as usize
        }))
    }
}

impl Value {
    pub fn iter(self) -> Box<dyn Iterator<Item = usize>> {
        // TODO dedup ends
        // 2 2 1 1 1 1 0 0 0 0 0 0 0 0 -> 2 2 1 1 1 1 0
        match self {
            Value::Absolute(new, Step::To(r)) => {
                let new = usize::min(new as usize, r.max);
                r.curr_to_new(new)
            }
            Value::Absolute(v, Step::By(r)) => {
                let new = r.curr.saturating_add_signed(v as isize);
                let new = usize::min(new, r.max);
                r.curr_to_new(new)
            }
            Value::Relative(percent, Step::To(r)) => {
                let new = r.max as f32 / 100.0 * percent;
                r.curr_to_new(new as usize)
            }
            Value::Relative(percent, Step::By(r)) => r.by_percent(percent),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{Range, Result, Step, Value};

    #[test]
    fn parse_correct_input_ok() -> Result<()> {
        let r = Range::new(32, 64, 1.0);

        assert_eq!(r.parse_input("10")?, Value::Absolute(10.0, Step::To(r)));
        assert_eq!(r.parse_input("-10")?, Value::Absolute(-10.0, Step::By(r)));
        assert_eq!(r.parse_input("+10")?, Value::Absolute(10.0, Step::By(r)));
        assert_eq!(r.parse_input("10%")?, Value::Relative(10.0, Step::To(r)));
        assert_eq!(r.parse_input("-10%")?, Value::Relative(-10.0, Step::By(r)));
        assert_eq!(r.parse_input("+10%")?, Value::Relative(10.0, Step::By(r)));

        Ok(())
    }

    #[test]
    fn parse_incorrect_input_err() -> Result<()> {
        let r = Range::new(32, 64, 1.0);

        assert!(r.parse_input("-%").is_err());
        assert!(r.parse_input("-").is_err());
        assert!(r.parse_input("%").is_err());
        assert!(r.parse_input("+1a%").is_err());

        // FIXME
        assert_eq!(r.parse_input("+-10")?, Value::Absolute(10.0, Step::By(r)));
        assert_eq!(r.parse_input("-+10")?, Value::Absolute(-10.0, Step::By(r)));
        assert_eq!(r.parse_input("10%%")?, Value::Relative(10.0, Step::To(r)));

        Ok(())
    }

    #[test]
    fn no_overflow() -> Result<()> {
        let r = Range::new(32, 64, 1.0);

        assert_eq!(r.try_from_input("100")?.last(), Some(64));
        assert_eq!(r.try_from_input("-100")?.last(), Some(0));
        assert_eq!(r.try_from_input("+100")?.last(), Some(64));
        assert_eq!(r.try_from_input("-100%")?.last(), Some(0));
        assert_eq!(r.try_from_input("+100%")?.last(), Some(64));

        Ok(())
    }
}
