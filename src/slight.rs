use crate::error::SlightError;
use crate::io::IO;
use crate::Args;
use std::path::Path;

const NUMBER_OF_STEPS: i64 = 4;

pub struct Slight {
    io: IO,
    percent: f32,
}

impl Slight {
    fn new(io: IO, percent: f32) -> Self {
        Self { io, percent }
    }

    pub fn range(&self) -> impl Iterator<Item = i64> {
        let curr = self.io.get_value();
        let new = curr + percent_to_value(self.percent, self.io.max_value());
        let step = match i64::abs(curr - new) / (NUMBER_OF_STEPS - 1) {
            // -1 because rounding
            // gives extra step
            s if s > 0 => s,
            _ => 1,
        } as usize;
        if curr < new {
            (curr..=new).step_by(step).collect::<Vec<i64>>().into_iter()
        } else {
            (new..=curr)
                .rev()
                .step_by(step)
                .collect::<Vec<i64>>()
                .into_iter()
        }
    }

    pub fn set_value(&mut self, v: i64) -> Result<(), SlightError> {
        Ok(self.io.set_value(v)?)
    }
}

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let path = Path::new(&args.path);
        let io = IO::try_new(path)?;
        let percent = args.percent;
        Ok(Slight::new(io, percent))
    }
}

fn percent_to_value(percent: f32, max_value: i64) -> i64 {
    ((max_value as f32 / 100f32) * percent) as i64
}
