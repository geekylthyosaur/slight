use slight::{Id, Input, Mode, Slight};

use clap::{value_parser, Parser, ValueEnum};

/// Utility to control backlight brightness smoothly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Args {
    /// Change brightness of device with given id (use --list to find one)
    #[clap(short, long, requires("input"))]
    id: Option<Id>,

    /// Input string to control backlight brightness of specified or default device
    ///
    /// - `n`: set exact absolute brightness value;
    ///
    /// - `+n`/`-n`: increase/decrease current brightness by absolute value;
    ///
    /// - `n%`: set exact relative brightness value;
    ///
    /// - `+n%`/`-n%`: increase/decrease current brightness by relative value.
    #[clap(allow_hyphen_values(true), value_parser = value_parser!(Input))]
    input: Option<Input>,

    /// List all available devices or the ones with given id
    #[clap(short, long, conflicts_with("input"), num_args = 0.., value_delimiter = ' ')]
    list: Option<Vec<Id>>,

    /// Use exponential range with given exponent (or default = 4.0)
    #[clap(short, long, requires("input"))]
    exponent: Option<Option<f32>>,

    /// Create smooth transition with at most <value> iterations
    #[clap(short, long, requires("input"), value_parser(1..))]
    max_iter: Option<i64>,

    /// Pretend setting brightness
    #[clap(short, long)]
    pretend: bool,

    /// Toggle value of device with only two available values (0/1)
    #[clap(short, long, conflicts_with("input"), requires("id"))]
    toggle: Option<Option<ToggleState>>,

    /// Being verbose about what is going on
    // FIXME: unreachable
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // FIXME
    if args.verbose {
        let env_filter = tracing_subscriber::EnvFilter::from_default_env();
        tracing_subscriber::fmt()
            .compact()
            .without_time()
            .with_env_filter(env_filter)
            .init();
    }

    let mode = if let Some(ids) = args.list {
        Mode::List(ids)
    } else if let Some(toggle) = args.toggle {
        Mode::Toggle(toggle.map(slight::ToggleState::from))
    } else if let Some(input) = args.input {
        Mode::Regular {
            input,
            exponent: args.exponent,
            max_iter: args.max_iter.map(|v| v as usize),
        }
    } else {
        unreachable!()
    };

    let mut slight = Slight::new(args.id);

    slight.set_verbose(args.verbose);
    slight.set_pretend(args.pretend);

    if let Err(e) = slight.run(mode) {
        tracing::error!("{}", e);
        eprintln!("{e}");
    }
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
