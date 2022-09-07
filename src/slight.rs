use crate::error::SlightError;
use crate::io::read_num;
use crate::value::Value;
use crate::Args;
use std::path::Path;

const CURRENT_BRIGHTNESS_FILENAME: &str = "brightness";
const MAX_BRIGHTNESS_FILENAME: &str = "max_brightness";

pub struct Slight {
    value: Value,
}

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let path = Path::new(&args.path);
        let v = Value::new(
            read_num(&path.join(CURRENT_BRIGHTNESS_FILENAME))?,
            read_num(&path.join(MAX_BRIGHTNESS_FILENAME))?,
            0,
        );

        Ok(Self { value: v })
    }
}

fn percent_to_value(percent: i64, max_value: i64) -> i64 {
    (max_value / 100) * percent
}
