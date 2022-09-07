use crate::error::SlightError;
use crate::io::IO;
use crate::Args;
use std::path::Path;

pub struct Slight;

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let path = Path::new(&args.path);
        let io = IO::try_new(path);
        Ok(Self {})
    }
}

fn percent_to_value(percent: i64, max_value: i64) -> i64 {
    (max_value / 100) * percent
}
