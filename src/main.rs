mod brightness;
mod class;
mod device;
mod error;
mod io;
mod range;
mod slight;
mod value;

use slight::Slight;

use clap::Parser;

/// Utility to control backlight brightness smoothly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Device Id
    #[clap(short, long)]
    id: Option<String>,

    /// to_value: 10, by_value: +-10, by_percent: +-10.0%
    #[clap()]
    input: String,

    /// Print all available devices and exit
    #[clap(short, long)]
    list: Option<Option<String>>,

    /// Exponent
    #[clap(short, long)]
    exponent: Option<Option<f32>>,

    /// Write to stdout instead of sysfs
    #[clap(short, long)]
    stdout: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(list) = args.list {
        if let Some(_id) = list {
            // TODO: print single device
        } else {
            Slight::print_devices();
        }
        return;
    }

    let mut slight = Slight::try_from(&args).unwrap_or_else(|_| todo!("Error!"));
    slight.set_brightness().unwrap();
}
