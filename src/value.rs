pub enum Input {
    To(Value),
    By(Sign, Value),
}

pub enum Value {
    Absolute(usize),
    Percent(f32),
}

pub enum Sign {
    Plus,
    Minus,
}

impl TryFrom<String> for Input {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut chars = s.chars().peekable();
        if let Some(c) = chars.next_if(|&c| c == '-' || c == '+') {
            Ok(Self::By(Sign::try_from(c)?, Value::try_from(chars.collect::<String>())?))
        } else {
            Ok(Self::To(Value::try_from(chars.collect::<String>())?))
        }
    }
}

impl TryFrom<String> for Value {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let chars = s.split('%').collect::<Vec<_>>();
        todo!()
    }
}

impl TryFrom<char> for Sign {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            _ => Err(())
        }
    }
}
