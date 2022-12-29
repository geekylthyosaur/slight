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
    #[clap(allow_hyphen_values(true))]
    input: Option<String>,

    /// Print all available devices and exit
    #[clap(short, long, conflicts_with("input"))]
    list: Option<Option<String>>,

    /// Exponent
    #[clap(short, long)]
    exponent: Option<Option<f32>>,

    /// Write to stdout instead of sysfs
    #[clap(short, long)]
    stdout: bool,

    /// Being verbose about what is going on
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(list) = args.list {
        if let Some(_id) = list {
            todo!("print single device")
        } else if let Err(e) = Slight::print_devices() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        return;
    }

    let mut slight = Slight::new(
        args.id,
        args.exponent,
        args.input,
        args.stdout,
        args.verbose,
    )
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    slight.set_brightness().unwrap();
}
