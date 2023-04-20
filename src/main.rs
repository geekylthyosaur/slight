use slight::{Flags, Slight};

use clap::Parser;

/// Utility to control backlight brightness smoothly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Change brightness of device with given id (use --list to find one)
    #[clap(short, long)]
    id: Option<String>,

    /// Input string to control backlight brightness:
    ///
    /// - set exact absolute brightness value: `n`;
    ///
    /// - increase/decrease current brightness by absolute value: `+n`/`-n`;
    ///
    /// - set exact relative brightness value: `n%`;
    ///
    /// - increase/decrease current brightness by relative value: `+n%`/`-n%`.
    #[clap(allow_hyphen_values(true))]
    input: Option<String>,

    /// List all available devices or the one with given id
    #[clap(short, long, conflicts_with("input"))]
    list: Option<Option<String>>,

    /// Use exponential range with given exponent (or default = 4.0)
    #[clap(short, long)]
    exponent: Option<Option<f32>>,

    /// Write to stdout instead of sysfs
    #[clap(short, long)]
    stdout: bool,

    /// Toggle value of device with only two values (0/1)
    #[clap(short, long)]
    toggle: bool,

    /// Being verbose about what is going on
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(list) = args.list {
        if let Some(id) = list {
            Slight::print_device(id.into())
        } else {
            Slight::print_devices()
        }
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
        return;
    }

    let slight = Slight::new(
        args.id,
        args.exponent,
        args.input,
        Flags {
            stdout: args.stdout,
            toggle: args.toggle,
            ..Flags::default()
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    slight.set_brightness().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
}
