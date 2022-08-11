mod error;
mod io;
mod percent;
mod value;

use error::SlightError;
use io::{read, write};
use value::Value;

use clap::Parser;

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    #[clap(short, long)]
    path: String,

    /// Percent
    #[clap(value_parser(-100..=100), allow_hyphen_values = true)]
    percent: i64, // value_parser only accepts i64 / u64
}

struct Slight {
    curr_value: Value,
    path: String,
    percent: i64,
}

impl Slight {
    fn percent_to_value(percent: i64) -> i64 {
        ((u8::MAX as f32 / 100f32) * percent as f32) as i64
    }

    fn write_range(&self, r: std::slice::Iter<u8>) -> Result<(), std::io::Error> {
        for &v in r {
            write(&self.path, v)?;
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
        }
        Ok(())
    }
}

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let curr_value = read(&args.path)?.trim().parse::<Value>()?;

        Ok(Self {
            curr_value,
            path: args.path,
            percent: args.percent,
        })
    }
}

fn main() {
    let args = Args::parse();

    let slight: Slight = match args.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    /*

    let v = if slight.curr_value < slight.new_value {
        (slight.curr_value..=slight.new_value).collect::<Vec<u8>>()
    } else {
        ((slight.new_value..=slight.curr_value).rev()).collect::<Vec<u8>>()
    };

    if let Err(e) = slight.write_range(v.iter()) {
        panic!("{}", e);
    }
    */
}

#[cfg(test)]
mod tests {
    use super::Slight;

    #[test]
    fn percent_to_value() {
        assert_eq!(Slight::percent_to_value(0i64), 0i64);
        assert_eq!(Slight::percent_to_value(100i64), 255i64);
        assert_eq!(Slight::percent_to_value(-100i64), -255i64);
        assert_eq!(Slight::percent_to_value(13i64), 33i64);
        assert_eq!(Slight::percent_to_value(-62i64), -158i64);
    }
}
