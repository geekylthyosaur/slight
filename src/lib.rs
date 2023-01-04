mod brightness;
mod class;
mod device;
mod error;
mod io;
mod range;
mod value;

use strum::IntoEnumIterator;

use std::{borrow::Cow, path::PathBuf};

use crate::{
    class::Class,
    device::Device,
    error::{Result, SlightError},
    io::IO,
    range::{Range, RangeBuilder},
    value::{Input, Value},
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
    exponent: f32,
    input: Input,
    io: IO,
}

impl Slight {
    pub fn new(
        id: Option<String>,
        exponent: Option<Option<f32>>,
        input: Option<String>,
        flags: Flags,
    ) -> Result<Self> {
        let devices = Self::scan_devices()?;
        // TODO: any reasons to pass a reference?
        let device = Self::select_device(&devices, id.as_deref())?;
        let exponent = match exponent {
            None => NO_EXPONENT_DEFAULT,
            Some(None) => EXPONENT_DEFAULT,
            Some(Some(v)) => v,
        };
        let io = if flags.stdout {
            IO::stdout()
        } else {
            IO::new(&device.my_path())?
        };
        Ok(Self {
            device: device.clone(),
            exponent,
            // TODO unwrap
            input: Input::try_from(input.as_ref().ok_or(SlightError::NoInput)?.as_str()).unwrap(),
            io,
        })
    }

    pub fn set_brightness(&mut self) -> Result<()> {
        let curr = self.device.brightness.as_value();
        let max = self.device.brightness.max();
        let range = Self::create_range(curr, &self.input, max, self.exponent);
        self.set_brightness_range(range)?;
        Ok(())
    }

    fn scan_devices() -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        Class::iter().for_each(|class| {
            let path: PathBuf = class.into();
            IO::scan(&path).map_or_else(
                //TODO: print only if self.verbose
                |e| eprintln!("Failed to read class '{}': {}", class, e),
                |ids| {
                    for id in ids {
                        Device::new(class, path.join(&id).as_path()).map_or_else(
                            //TODO: print only if self.verbose
                            |e| eprintln!("Failed to read device '{}': {}", id, e),
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
                println!("{}", dev);
            }
        }
        Ok(())
    }

    pub fn print_device(id: Cow<str>) -> Result<()> {
        let devices = Self::scan_devices()?;
        if devices.is_empty() {
            return Err(SlightError::NoDevices);
        } else {
            let dev = Self::find_device(&devices, id.as_ref())
                .ok_or(SlightError::NoSpecifiedDeviceFound)?;
            println!("{}", dev);
        }
        Ok(())
    }

    fn create_range(
        curr: usize,
        input: &Input,
        max: usize,
        exponent: f32,
    ) -> Box<dyn RangeBuilder> {
        let r = Range::new(curr, max, exponent);
        Box::new(match &input {
            Input::To(Value::Relative(p)) => r.to().relative(*p),
            Input::To(Value::Absolute(v)) => r.to().absolute(*v as isize),
            Input::By(s, Value::Absolute(v)) => r.by().absolute((s * *v as f32) as isize),
            Input::By(s, Value::Relative(p)) => r.by().relative(s * *p),
        })
    }

    fn set_brightness_range(&mut self, range: Box<dyn RangeBuilder>) -> Result<()> {
        for v in range.build() {
            self.device.brightness.set(v, &mut self.io)?;
            std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
        }
        Ok(())
    }

    fn select_device<'a>(devices: &'a [Device], id: Option<&'a str>) -> Result<&'a Device> {
        if let Some(id) = id {
            Self::find_device(devices, id).ok_or(SlightError::NoSpecifiedDeviceFound)
        } else {
            Self::default_device(devices).ok_or(SlightError::NoSuitableDeviceFound)
        }
    }

    fn find_device<'a>(devices: &'a [Device], id: &'a str) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id == id)
    }

    fn default_device(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class == Class::Backlight)
    }
}
