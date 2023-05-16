#![warn(clippy::pedantic)]
mod brightness;
mod class;
mod device;
mod error;
mod io;
mod range;

use device::Toggle;
use strum::IntoEnumIterator;

use std::{borrow::Cow, path::PathBuf};

use crate::{
    class::Class,
    device::Device,
    error::{Error, Result},
    io::IO,
    range::{Range, RangeBuilder},
};

/// Default value for exponent when using `--exponent` flag without given value
const EXPONENT_DEFAULT: f32 = 4.0;
/// Default value for exponent when `--exponent` flag was not provided
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
/// Default time interval between brightness changes
const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Default)]
pub struct Flags {
    /// Write to `stdout` instead of `sysfs`
    pub stdout: bool,
    /// Toggle value of device with only two values (0/1)
    pub toggle: bool,
    /// Being verbose about what is going on
    pub verbose: bool,
}

pub struct Slight {
    device: Device,
    range: Option<Box<dyn RangeBuilder>>,
    flags: Flags,
}

impl Slight {
    pub fn new(
        id: impl Into<Option<String>>,
        exponent: Option<Option<f32>>,
        input: impl Into<Option<String>>,
        flags: Flags,
    ) -> Result<Self> {
        let id = id.into();
        let input = input.into();
        let devices = Self::scan_devices()?
            .into_iter()
            .map(|d| d.unwrap()) // TODO: error hangling
            .collect::<Vec<_>>();
        let device = Self::select_device(&devices, id.map(Cow::from))?.clone();
        let exponent = match exponent {
            None => NO_EXPONENT_DEFAULT,
            Some(None) => EXPONENT_DEFAULT,
            Some(Some(v)) => v,
        };
        let range = if let Some(input) = input {
            let curr = device.brightness.as_value();
            let max = device.brightness.max();
            let r = Range::new(curr, max, exponent);
            Some(r.try_from_input(input.into())?)
        } else {
            None
        };
        Ok(Self {
            device,
            range,
            flags,
        })
    }

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
            .map(|d| d.unwrap()) // TODO: error hangling
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
    pub fn print_device(id: Cow<str>) -> Result<()> {
        let devices = Self::scan_devices()?
            .into_iter()
            .map(|d| d.unwrap()) // TODO: error hangling
            .collect::<Vec<_>>();

        if devices.is_empty() {
            return Err(Error::NoDevices);
        }

        let dev = Self::find_device(&devices, id).ok_or(Error::SpecifiedDeviceNotFound)?;
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
            IO::check_write_permissions(path)?;
            IO::file(path)?
        };

        if self.flags.toggle {
            self.device.toggle(&mut io)?;
            return Ok(());
        }

        if self.range.is_some() {
            for v in self.range.as_ref().unwrap().build() { // TODO: error hangling
                self.device.brightness.set(v, &mut io)?;
                std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
            }
        }
        Ok(())
    }

    fn select_device<'a>(devices: &'a [Device], id: Option<Cow<str>>) -> Result<&'a Device> {
        if let Some(id) = id {
            Self::find_device(devices, id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::default_device(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    fn find_device<'a>(devices: &'a [Device], id: Cow<str>) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id == id.as_ref())
    }

    fn default_device(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class == Class::Backlight)
    }
}
