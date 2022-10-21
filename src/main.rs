mod error;
mod io;
mod slight;
mod percent;
mod value;

use std::time::Duration;

use slight::Slight;

use clap::Parser;

const TIME_BETWEEN_STEPS: Duration = Duration::from_millis(64);

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to file
    #[clap(long)]
    path: String,

    /// Use exponential brightness
    #[clap(short)]
    exp: bool,

    /// Write to stdout instead of file
    #[clap(short, long)]
    pretend: bool,

    /// Percent
    #[clap(allow_hyphen_values = true)]
    percent: f32,
}

fn main() {
    let args = Args::parse();

    let mut slight: Slight = match args.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    for i in slight.range() {
        if slight.args.pretend {
            println!("{}", i);
        } else {
            slight.set_value(i).unwrap();
            std::thread::sleep(TIME_BETWEEN_STEPS);
        }
    }
}
