mod brightness;
mod class;
mod device;
mod error;
mod io;
mod range;

use strum::IntoEnumIterator;

use std::{borrow::Cow, path::PathBuf};

use crate::{
    class::Class,
    device::Device,
    error::{Result, SlightError},
    io::IO,
    range::{Range, RangeBuilder},
};

const EXPONENT_DEFAULT: f32 = 4.0;
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Default)]
pub struct Flags {
    pub stdout: bool,
    pub verbose: bool,
}

pub struct Slight {
    device: Device,
    range: Option<Box<dyn RangeBuilder>>,
    io: IO,
}

impl Slight {
    pub fn new(
        id: Option<Cow<str>>,
        exponent: Option<Option<f32>>,
        input: Option<Cow<str>>,
        flags: Flags,
    ) -> Result<Self> {
        let devices = Self::scan_devices()?;
        let device = Self::select_device(&devices, id)?.to_owned();
        let exponent = match exponent {
            None => NO_EXPONENT_DEFAULT,
            Some(None) => EXPONENT_DEFAULT,
            Some(Some(v)) => v,
        };
        let io = if flags.stdout {
            IO::stdout()
        } else {
            IO::file(&device.my_path())?
        };
        let range = if let Some(input) = input {
            let curr = device.brightness.as_value();
            let max = device.brightness.max();
            Some(Range::try_from_input(input, curr, max, exponent)?)
        } else {
            None
        };
        Ok(Self { device, range, io })
    }

    fn scan_devices() -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        Class::iter().for_each(|class| {
            let path: PathBuf = class.into();
            IO::scan(&path).map_or_else(
                //TODO: print only if self.verbose
                |e| eprintln!("Failed to read class '{class}': {e}"),
                |ids| {
                    for id in ids {
                        Device::new(class, path.join(&id).as_path()).map_or_else(
                            //TODO: print only if self.verbose
                            |e| eprintln!("Failed to read device '{id}': {e}"),
                            |device| devices.push(device),
                        )
                    }
                },
            )
        });
        Ok(devices)
    }

    pub fn print_devices() -> Result<()> {
        let devices = Self::scan_devices()?;
        if devices.is_empty() {
            return Err(SlightError::NoDevices);
        } else {
            for dev in devices {
                println!("{dev}");
            }
        }
        Ok(())
    }

    pub fn print_device(id: Cow<str>) -> Result<()> {
        let devices = Self::scan_devices()?;
        if devices.is_empty() {
            return Err(SlightError::NoDevices);
        } else {
            let dev =
                Self::find_device(&devices, id).ok_or(SlightError::SpecifiedDeviceNotFound)?;
            println!("{dev}");
        }
        Ok(())
    }

    pub fn set_brightness(&mut self) -> Result<()> {
        if self.range.is_some() {
            for v in self.range.as_ref().unwrap().build() {
                self.device.brightness.set(v, &mut self.io)?;
                std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
            }
        }
        Ok(())
    }

    fn select_device<'a>(devices: &'a [Device], id: Option<Cow<str>>) -> Result<&'a Device> {
        if let Some(id) = id {
            Self::find_device(devices, id).ok_or(SlightError::SpecifiedDeviceNotFound)
        } else {
            Self::default_device(devices).ok_or(SlightError::SuitableDeviceNotFound)
        }
    }

    fn find_device<'a>(devices: &'a [Device], id: Cow<str>) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id == id.as_ref())
    }

    fn default_device(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class == Class::Backlight)
    }
}
