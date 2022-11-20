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
pub struct Args {
    /// Device Id
    #[clap(short, long)]
    id: Option<String>,

    /// New brightness value
    #[clap(short, long)]
    new: usize,
}

fn main() {
    let args = Args::parse();

    let mut slight = Slight::new();
    slight.read_devices();
    slight.print_devices();
    slight.create_range(args.id).unwrap();
}
