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
    #[clap(short, long, conflicts_with("percent"))]
    value: Option<usize>,

    /// New brightness percent delta
    #[clap(short, long)]
    percent: Option<f32>,

    /// Print all available devices and exit
    #[clap(short, long)]
    list: bool,

    /// Exponent
    #[clap(short, long)]
    exponent: Option<f32>,
}

fn main() {
    let args = Args::parse();

    if args.list {
        Slight::print_devices();
        return;
    }

    let mut slight = Slight::try_from(&args).unwrap_or_else(|_| todo!("Error!"));
    // TODO: pass value or percent
    slight.set_brightness(args.value.unwrap()).unwrap();
}
