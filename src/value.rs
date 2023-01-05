use std::borrow::Cow;

use crate::error::SlightError;

#[derive(Clone, Copy)]
pub enum Input {
    To(Value),
    By(Sign, Value),
}

#[derive(Clone, Copy)]
pub enum Value {
    Absolute(usize),
    Relative(f32),
}

#[derive(Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

impl std::ops::Mul<f32> for Sign {
    type Output = f32;

    fn mul(self, f: f32) -> f32 {
        match self {
            Sign::Plus => f.copysign(1.0),
            Sign::Minus => f.copysign(-1.0),
        }
    }
}

impl TryFrom<Cow<'_, str>> for Input {
    type Error = SlightError;

    fn try_from(s: Cow<str>) -> Result<Self, Self::Error> {
        let mut chars = s.chars().peekable();
        if let Some(c) = chars.next_if(|&c| c == '-' || c == '+') {
            Ok(Self::By(
                Sign::try_from(c)?,
                Value::try_from(chars.collect::<Cow<str>>())?,
            ))
        } else {
            Ok(Self::To(Value::try_from(chars.collect::<Cow<str>>())?))
        }
    }
}

impl TryFrom<Cow<'_, str>> for Value {
    type Error = SlightError;

    fn try_from(s: Cow<'_, str>) -> Result<Self, Self::Error> {
        match s.split('%').count() {
            1 => Ok(Value::Absolute(
                s.parse::<usize>().map_err(|_| SlightError::InvalidInput)?,
            )),
            2 => Ok(Value::Relative(
                s.split('%')
                    .next()
                    .unwrap()
                    .parse::<f32>()
                    .map_err(|_| SlightError::InvalidInput)?,
            )),
            _ => Err(SlightError::InvalidInput),
        }
    }
}

impl TryFrom<char> for Sign {
    type Error = SlightError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            _ => Err(SlightError::InvalidInput),
        }
    }
}
