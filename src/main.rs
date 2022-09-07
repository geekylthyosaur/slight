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

fn main() {
    let args = Args::parse();

    let slight: Slight = match args.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
}
