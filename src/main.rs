use slight::{Id, Mode, Slight};

use clap::{Parser, ValueEnum};

/// Utility to control backlight brightness smoothly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Args {
    /// Change brightness of device with given id (use --list to find one)
    // FIXME: require input
    #[clap(short, long)]
    id: Option<Id>,

    /// Input string to control backlight brightness
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
    #[clap(short, long, conflicts_with("input"), num_args = 0.., value_delimiter = ' ')]
    list: Option<Vec<Id>>,

    /// Use exponential range with given exponent (or default = 4.0)
    #[clap(short, long)]
    exponent: Option<Option<f32>>,

    /// Write to stdout instead of sysfs
    #[clap(short, long, requires("input"))]
    stdout: bool,

    /// Toggle value of device with only two available values (0/1)
    #[clap(short, long, requires("id"))]
    toggle: Option<Option<ToggleState>>,

    /// Being verbose about what is going on
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> slight::error::Result<()> {
    let args = Args::parse();

    let mode = if let Some(ids) = args.list {
        Mode::List(ids)
    } else if let Some(toggle) = args.toggle {
        Mode::Toggle(toggle.map(slight::ToggleState::from))
    } else if let Some(input) = args.input {
        if let Some(exponent) = args.exponent {
            Mode::Exponential { input, exponent }
        } else {
            Mode::Regular { input }
        }
    } else {
        unreachable!()
    };

    let mut slight = Slight::id(args.id)?
        .mode(mode)
        .build()?;

    slight.verbose(args.verbose);
    slight.stdout(args.stdout);
    slight.set_brightness()?;

    Ok(())
}

#[derive(ValueEnum, Clone, Copy)]
enum ToggleState {
    On,
    Off,
}

impl From<ToggleState> for slight::ToggleState {
    fn from(value: ToggleState) -> Self {
        match value {
            ToggleState::On => slight::ToggleState::On,
            ToggleState::Off => slight::ToggleState::Off,
        }
    }
}
