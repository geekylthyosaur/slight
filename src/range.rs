use crate::error::Error;
use std::cmp::Ordering;
use std::ops::Neg;
use std::str::FromStr;

pub trait RangeIterator: ExactSizeIterator + DoubleEndedIterator {}

impl<I: DoubleEndedIterator + ExactSizeIterator> RangeIterator for I {}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub struct Range {
    curr: usize,
    exponent: f32,
    max: usize,
    max_iter: usize,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub enum Step {
    To,
    By,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone)]
pub enum Input {
    Absolute(f32, Step),
    Relative(f32, Step),
}

impl Range {
    pub fn new(curr: usize, max: usize, exponent: f32, max_iter: usize) -> Self {
        Self {
            curr,
            exponent,
            max,
            max_iter,
        }
    }

    fn curr_to_new(self, new: usize) -> Box<dyn RangeIterator<Item = usize>> {
        match new.cmp(&self.curr) {
            Ordering::Greater => Box::new((self.curr..new + 1).skip(1)),
            Ordering::Less => Box::new((new..self.curr + 1).rev().skip(1)),
            Ordering::Equal => Box::new(std::iter::empty()),
        }
    }

    fn by_percent(self, diff: f32) -> Box<dyn RangeIterator<Item = usize>> {
        if diff.is_sign_positive() {
            Box::new(
                self.exponential()
                    // FIXME: Filer does not implement ExactSizeIterator
                    //.filter(move |&v| v > self.curr)
                    .take(diff as usize),
            )
        } else {
            Box::new(
                self.exponential()
                    // FIXME: Filer does not implement ExactSizeIterator
                    //.filter(move |&v| v < self.curr)
                    .rev()
                    .take(diff.abs() as usize),
            )
        }
    }

    fn exponential(self) -> Box<dyn RangeIterator<Item = usize>> {
        Box::new((0..100 + 1).map(move |v: usize| {
            ((v as f32).powf(self.exponent) * 100f32.powf(self.exponent.neg()) * self.max as f32)
                as usize
        }))
    }
}

impl Input {
    #[must_use]
    pub fn iter_with(self, r: Range) -> Box<dyn RangeIterator<Item = usize>> {
        // TODO dedup ends
        // 2 2 1 1 1 1 0 0 0 0 0 0 0 0 -> 2 2 1 1 1 1 0
        match self {
            Input::Absolute(new, Step::To) => {
                let new = usize::min(new as usize, r.max);
                r.curr_to_new(new)
            }
            Input::Absolute(v, Step::By) => {
                let new = r.curr.saturating_add_signed(v as isize);
                let new = usize::min(new, r.max);
                r.curr_to_new(new)
            }
            Input::Relative(percent, Step::To) => {
                let new = r.max as f32 / 100.0 * percent;
                r.curr_to_new(new as usize)
            }
            Input::Relative(percent, Step::By) => r.by_percent(percent),
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let step = if input.starts_with(|c| matches!(c, '-' | '+')) {
            Step::By
        } else {
            Step::To
        };

        Ok(if let Some(input) = input.strip_suffix('%') {
            step.relative(input.parse::<f32>()?)
        } else {
            step.absolute(input.parse::<f32>()?)
        })
    }
}

impl Step {
    pub fn absolute(self, v: f32) -> Input {
        Input::Absolute(v, self)
    }

    pub fn relative(self, v: f32) -> Input {
        Input::Relative(v, self)
    }
}

pub struct LimitLen<I: IntoIterator> {
    iter: Box<dyn Iterator<Item = I::Item>>,
}

impl<I> Iterator for LimitLen<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait LimitLenExt: DoubleEndedIterator + ExactSizeIterator {
    fn limit(mut self, len: usize) -> LimitLen<Self>
    where
        Self: Sized + 'static,
        <Self as Iterator>::Item: 'static,
    {
        let source_len = self.len();
        let target_len = len - 1; // Saving space for last item
        let step = usize::max(1, ((source_len + target_len) / target_len) - 1);
        let iter: Box<dyn Iterator<Item = Self::Item>> = if let Some(last) = self.next_back() {
            Box::new(
                self.step_by(step)
                    .take(target_len)
                    .chain(std::iter::once(last)),
            )
        } else {
            Box::new(std::iter::empty())
        };
        LimitLen { iter }
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> LimitLenExt for I {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn parse_correct_input_ok() -> Result<()> {
        assert_eq!("10".parse::<Input>()?, Input::Absolute(10.0, Step::To));
        assert_eq!("-10".parse::<Input>()?, Input::Absolute(-10.0, Step::By));
        assert_eq!("+10".parse::<Input>()?, Input::Absolute(10.0, Step::By));
        assert_eq!("10%".parse::<Input>()?, Input::Relative(10.0, Step::To));
        assert_eq!("-10%".parse::<Input>()?, Input::Relative(-10.0, Step::By));
        assert_eq!("+10%".parse::<Input>()?, Input::Relative(10.0, Step::By));

        Ok(())
    }

    #[test]
    fn parse_incorrect_input_err() {
        assert!("-%".parse::<Input>().is_err());
        assert!("-".parse::<Input>().is_err());
        assert!("%".parse::<Input>().is_err());
        assert!("+1a%".parse::<Input>().is_err());
        assert!("+-10".parse::<Input>().is_err());
        assert!("-+10".parse::<Input>().is_err());
        assert!("10%%".parse::<Input>().is_err());
    }

    #[test]
    fn no_overflow() -> Result<()> {
        let r = Range::new(32, 64, 1.0, 10);

        assert_eq!("100".parse::<Input>()?.iter_with(r).last(), Some(64));
        assert_eq!("-100".parse::<Input>()?.iter_with(r).last(), Some(0));
        assert_eq!("+100".parse::<Input>()?.iter_with(r).last(), Some(64));
        assert_eq!("-100%".parse::<Input>()?.iter_with(r).last(), Some(0));
        assert_eq!("+100%".parse::<Input>()?.iter_with(r).last(), Some(64));

        Ok(())
    }

    #[test]
    fn limit() {
        const I: std::ops::Range<usize> = 0..100;

        assert_eq!(I.limit(80).len(), 80);
    }
}
