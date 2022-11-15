mod brightness;
mod class;
mod device;
mod error;
mod io;
mod slight;

use slight::Slight;

use clap::Parser;

/// Utility to control backlight brightness smoothly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {}

fn main() {
    let _args = Args::parse();

    let mut slight = Slight::new();
    slight.read_devices();
    slight.print_devices();
}
