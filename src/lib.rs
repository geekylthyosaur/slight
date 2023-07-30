#![warn(clippy::pedantic)]
mod brightness;
mod class;
mod device;
mod error;
mod range;

use crate::{device::Device, range::Range};
pub use crate::{
    device::Id,
    error::{Error, Result},
    range::Input,
};

/// Default value for exponent when using `--exponent` flag without given value
const EXPONENT_DEFAULT: f32 = 4.0;
/// Default value for exponent when `--exponent` flag was not provided
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
/// Default time interval between brightness changes
pub const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Clone)]
pub enum Mode {
    Regular {
        input: Input,
        exponent: Option<Option<f32>>,
    },
    List(Vec<Id>),
    Toggle(Option<ToggleState>),
}

#[derive(Clone, Copy)]
pub enum ToggleState {
    On,
    Off,
}

#[derive(Default)]
struct Flags {
    /// Write to `stdout` instead of `sysfs`
    pub stdout: bool,
    /// Being verbose about what is going on
    pub verbose: bool,
}

pub struct Slight {
    id: Option<Id>,
    flags: Flags,
}

impl Slight {
    pub fn new(id: impl Into<Option<Id>>) -> Slight {
        Slight {
            id: id.into(),
            flags: Flags::default(),
        }
    }

    pub fn verbose(&mut self, v: bool) {
        self.flags.verbose = v;
    }

    pub fn stdout(&mut self, v: bool) {
        self.flags.stdout = v;
    }

    pub fn run(&self, mode: Mode) -> Result<()> {
        let devices = Device::all()
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();
        let mut device = Device::select(&devices, &self.id)?.clone();
        let curr = device.brightness().current;
        let max = device.brightness().max;

        match mode {
            Mode::List(ids) => Self::print_devices(&devices, &ids),
            Mode::Toggle(toggle_state) => device.toggle(toggle_state),
            Mode::Regular { input, exponent } => {
                let exponent = match exponent {
                    None => NO_EXPONENT_DEFAULT,
                    Some(None) => EXPONENT_DEFAULT,
                    Some(Some(v)) => v,
                };
                let r = Range::new(curr, max, exponent);
                let r = input.iter_with(r);
                device.set_range(r)
            }
        }
    }

    fn print_devices(devices: &[Device], ids: &[Id]) -> Result<()> {
        if devices.is_empty() {
            Err(Error::NoDevices)?;
        }

        ids.is_empty()
            .then(|| devices.iter().for_each(|d| println!("{d}")))
            .unwrap_or_else(|| {
                devices
                    .iter()
                    .filter(|d| ids.contains(&d.id()))
                    .for_each(|d| println!("{d}"));
            });

        Ok(())
    }
}
