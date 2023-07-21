#![warn(clippy::pedantic)]
mod brightness;
mod class;
mod device;
pub mod error;
mod range;

pub use crate::device::Id;
use crate::{
    device::Device,
    error::{Error, Result},
    range::Range,
};

/// Default value for exponent when using `--exponent` flag without given value
const EXPONENT_DEFAULT: f32 = 4.0;
/// Default value for exponent when `--exponent` flag was not provided
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
/// Default time interval between brightness changes
const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Clone)]
pub enum Mode {
    Regular {
        input: String,
    },
    Exponential {
        input: String,
        exponent: Option<f32>,
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
    mode: Mode,
    id: Option<Id>,
    flags: Flags,
}

impl Slight {
    pub fn new(id: impl Into<Option<Id>>, mode: Mode) -> Slight {
        Slight {
            mode,
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

    pub fn run(self) -> Result<()> {
        let devices = Device::all()
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();
        let mut device = Device::select(&devices, self.id)?.clone();
        let curr = device.brightness().current;
        let max = device.brightness().max;

        let range = match self.mode {
            Mode::List(ids) => {
                Self::print_devices(&devices, &ids)?;
                return Ok(());
            }
            Mode::Toggle(toggle_state) => {
                device.toggle(toggle_state)?;
                return Ok(());
            }
            Mode::Regular { input } => {
                let r = Range::new(curr, max, NO_EXPONENT_DEFAULT);
                Some(r.try_from_input(input.into())?)
            }
            Mode::Exponential { input, exponent } => {
                let exponent = exponent.unwrap_or(EXPONENT_DEFAULT);
                let r = Range::new(curr, max, exponent);
                Some(r.try_from_input(input.into())?)
            }
        };

        if let Some(range) = range {
            range.as_ref().build().try_for_each(|v| {
                device.set_brightness(v)?;
                std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
                Ok::<(), Error>(())
            })?;
        }
        Ok(())
    }

    fn print_devices(devices: &[Device], ids: &[Id]) -> Result<()> {
        devices.is_empty().then_some(()).ok_or(Error::NoDevices)?;

        if ids.is_empty() {
            devices.iter().for_each(|d| println!("{d}"));
        } else {
            devices
                .iter()
                .filter(|d| ids.contains(&d.id()))
                .for_each(|d| println!("{d}"));
        }

        Ok(())
    }
}
