mod error;
mod io;
mod slight;
mod value;

use error::SlightError;
use slight::Slight;

use clap::Parser;

use std::path::Path;

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    #[clap(short, long)]
    path: String,

    /// Percent
    #[clap(value_parser(-100..=100), allow_hyphen_values = true)]
    percent: i64, // value_parser only accepts i64 & u64
}

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let path = Path::new(&args.path);
        let v = Value::new(
            read_as_num(&path.join(CURRENT_BRIGHTNESS_FILENAME))?,
            read_as_num(&path.join(MAX_BRIGHTNESS_FILENAME))?,
            0,
        );
        Ok(Self { value: v })
    }
}

fn main() {
    let args = Args::parse();

    let slight: Slight = match args.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
}

#[cfg(test)]
mod tests {
    use super::Slight;

    #[test]
    fn percent_to_value() {}
}
