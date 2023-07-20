#![warn(clippy::pedantic)]
mod brightness;
mod class;
mod device;
pub mod error;
mod io;
mod range;

use strum::IntoEnumIterator;

use std::path::PathBuf;

use crate::{
    class::Class,
    device::Device,
    error::{Error, Result},
    io::IO,
    range::{Range, RangeBuilder},
};
pub use crate::device::Id;

/// Default value for exponent when using `--exponent` flag without given value
const EXPONENT_DEFAULT: f32 = 4.0;
/// Default value for exponent when `--exponent` flag was not provided
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
/// Default time interval between brightness changes
const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Clone)]
pub enum Mode {
    Regular { input: String },
    Exponential { input: String, exponent: Option<f32> },
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
    device: Device,
    range: Option<Box<dyn RangeBuilder>>,
    flags: Flags,
}

pub struct SlightModeBuilder {
    device: Device,
}

impl SlightModeBuilder {
    pub fn mode(self, mode: Mode) -> SlightRangeBuilder {
        SlightRangeBuilder { mode, device: self.device }
    }
}

pub struct SlightRangeBuilder {
    device: Device,
    mode: Mode,
}

impl SlightRangeBuilder {
    pub fn build(self) -> Result<Slight> {
        let curr = self.device.brightness.as_value();
        let max = self.device.brightness.max();
        // FIXME: clone?
        let range = match self.mode.clone() {
            Mode::List(ids) => {
                if ids.is_empty() {
                    Slight::print_devices()?;
                } else {
                    for id in ids {
                        Slight::print_device(id)?;
                    }
                };
                None
            },
            Mode::Regular { input } => {
                let r = Range::new(curr, max, NO_EXPONENT_DEFAULT);
                Some(r.try_from_input(input.into())?)
            },
            Mode::Exponential { input, exponent } => {
                let exponent = exponent.unwrap_or(EXPONENT_DEFAULT);
                let r = Range::new(curr, max, exponent);
                Some(r.try_from_input(input.into())?)
            },
            Mode::Toggle(_) => {
                // FIXME
                None
            },
        };

        Ok(Slight { mode: self.mode, device: self.device, flags: Flags::default(), range })
    }
}

impl Slight {
    pub fn id(id: impl Into<Option<Id>>) -> Result<SlightModeBuilder> {
        let devices = Self::scan_devices()?
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();

        let device = Self::select_device(&devices, id.into())?.clone();

        Ok(SlightModeBuilder { device })
    }

    pub fn verbose(&mut self, v: bool) {
        self.flags.verbose = v;
    }

    pub fn stdout(&mut self, v: bool) {
        self.flags.stdout = v;
    }

    #[allow(clippy::unnecessary_wraps)]
    fn scan_devices() -> Result<Vec<Result<Device>>> {
        Ok(Class::iter()
            .flat_map(|class| {
                let path = PathBuf::from(class);
                IO::scan(path.as_path()).map(|ids| {
                    ids.map(|id| {
                        let path = path.join(id);
                        Device::new(class, path.as_path())
                    })
                    .collect::<Vec<_>>()
                })
            })
            .flatten()
            .collect())
    }

    /// Print all available devices
    pub fn print_devices() -> Result<()> {
        let devices = Self::scan_devices()?
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();

        if devices.is_empty() {
            return Err(Error::NoDevices);
        }

        for dev in devices {
            println!("{dev}");
        }

        Ok(())
    }

    /// Print device with given `id` if it exists
    pub fn print_device(id: impl Into<Id>) -> Result<()> {
        let devices = Self::scan_devices()?
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();

        if devices.is_empty() {
            return Err(Error::NoDevices);
        }

        let dev = Self::find_device(&devices, &id.into()).ok_or(Error::SpecifiedDeviceNotFound)?;
        println!("{dev}");

        Ok(())
    }

    /// Set brightness of device
    pub fn set_brightness(mut self) -> Result<()> {
        let mut io = if self.flags.stdout {
            IO::stdout()
        } else {
            let path = &self.device.my_path();
            // TODO: show instructions
            // FIXME: false positive
            // IO::check_write_permissions(path)?;
            IO::file(path)?
        };

        if let Mode::Toggle(toggle_state) = self.mode {
            self.device.toggle(&mut io, toggle_state)?;
            return Ok(());
        }

        if self.range.is_some() {
            for v in self.range.as_ref().unwrap().build() { // TODO: error handling
                self.device.brightness.set(v, &mut io)?;
                std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
            }
        }
        Ok(())
    }

    fn select_device(devices: &[Device], id: Option<Id>) -> Result<&Device> {
        if let Some(id) = id {
            Self::find_device(devices, &id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::default_device(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    fn find_device<'a>(devices: &'a [Device], id: &Id) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id == id.as_ref())
    }

    fn default_device(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class == Class::Backlight)
    }
}
