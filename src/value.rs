pub enum Input {
    To(Value),
    By(Sign, Value),
}

pub enum Value {
    Absolute(usize),
    Relative(f32),
}

pub enum Sign {
    Plus,
    Minus,
}

impl std::ops::Mul<f32> for &Sign {
    type Output = f32;

    fn mul(self, f: f32) -> f32 {
        match self {
            Sign::Plus => f.copysign(1.0),
            Sign::Minus => f.copysign(-1.0),
        }
    }
}

impl TryFrom<&str> for Input {
    type Error = ();
    // TODO
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut chars = s.chars().peekable();
        if let Some(c) = chars.next_if(|&c| c == '-' || c == '+') {
            Ok(Self::By(
                Sign::try_from(c)?,
                Value::try_from(chars.collect::<String>())?,
            ))
        } else {
            Ok(Self::To(Value::try_from(chars.collect::<String>())?))
        }
    }
}

impl TryFrom<String> for Value {
    type Error = ();
    // TODO
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let chars = s.split('%').collect::<Vec<_>>();
        if chars.len() == 1 {
            Ok(Value::Absolute(chars[0].parse::<usize>().unwrap()))
        } else if chars.len() == 2 {
            Ok(Value::Relative(chars[0].parse::<f32>().unwrap()))
        } else {
            Err(())
        }
    }
}

impl TryFrom<char> for Sign {
    type Error = ();
    // TODO
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            _ => Err(()),
        }
    }
}
