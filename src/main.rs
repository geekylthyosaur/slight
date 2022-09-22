mod error;
mod io;
mod slight;
mod value;

use slight::Slight;

use clap::Parser;

const TIME_BETWEEN_STEPS: std::time::Duration = std::time::Duration::from_millis(64);

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    #[clap(short, long)]
    path: String,

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
        slight.set_value(i).unwrap();
        std::thread::sleep(TIME_BETWEEN_STEPS);
    }
}
