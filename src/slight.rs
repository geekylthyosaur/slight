use strum::IntoEnumIterator;

use std::cmp::Ordering;
use std::path::PathBuf;

use crate::{
    class::Class,
    device::Device,
    error::{Result, SlightError},
    io::IO,
    Args,
};

const EXPONENT_DEFAULT: f32 = 4.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

pub struct Slight {
    device: Device,
    exponent: f32,
    new_value: Option<usize>,
    percent: Option<f32>,
    stdout: bool,
}

impl Slight {
    pub fn set_brightness(&mut self) -> Result<()> {
        let curr = self.device.brightness.as_value();
        let max = self.device.brightness.max();
        let range = Self::create_range(curr, self.new_value, self.percent, max, self.exponent);
        if self.stdout {
            println!(
                "{}",
                range.map(|v| v.to_string()).collect::<Vec<_>>().join("\n")
            );
            return Ok(());
        }
        Self::set_brightness_range(range, &mut self.device)?;
        Ok(())
    }

    fn scan_devices() -> Vec<Device> {
        let mut devices = Vec::new();
        Class::iter().map(|c| PathBuf::from(&c)).for_each(|class| {
            IO::scan(&class).map_or_else(
                |_| todo!("Log out error"),
                |ids| {
                    ids.iter().for_each(|id| {
                        class
                            .join(id)
                            .as_path()
                            .try_into()
                            .map_or_else(|_| todo!("Log out error"), |device| devices.push(device))
                    });
                },
            );
        });
        devices
    }

    pub fn print_devices() {
        let devices = Self::scan_devices();
        if devices.is_empty() {
            println!("No devices found!");
        } else {
            println!("Found devices:");
            for dev in devices {
                println!("\t{}", dev);
            }
        }
    }

    fn create_range(
        curr: usize,
        new: Option<usize>,
        percent: Option<f32>,
        max: usize,
        exponent: f32,
    ) -> impl Iterator<Item = usize> {
        // TODO
        let range =
            (0..=max).map(move |v| ((v as f32 / max as f32).powf(exponent) * max as f32) as usize);
        if let Some(new) = new {
            let mut range = match curr.cmp(&new) {
                Ordering::Less => range
                    .filter(move |&v| v > curr && v <= new)
                    .collect::<Vec<usize>>(),
                Ordering::Greater => range
                    .filter(move |&v| v < curr && v >= new)
                    .rev()
                    .collect::<Vec<usize>>(),
                Ordering::Equal => vec![],
            };
            range.dedup();
            return range.into_iter();
        } else if let Some(percent) = percent {
            let mut range = match percent.is_sign_positive() {
                true => range
                    .filter(move |&v| v > curr)
                    .take(percent as usize)
                    .collect::<Vec<usize>>(),
                false => range
                    .filter(move |&v| v < curr)
                    .rev()
                    .take(percent.copysign(1.0) as usize)
                    .collect::<Vec<usize>>(),
            };
            range.dedup();
            return range.into_iter();
        } else {
            unreachable!()
        }
    }

    fn set_brightness_range(range: impl Iterator<Item = usize>, device: &mut Device) -> Result<()> {
        let path = device.my_path();
        for v in range {
            device.brightness.set(v, &path)?;
            std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
        }
        Ok(())
    }

    fn select_device<'a>(devices: &'a [Device], id: Option<&'a str>) -> Result<&'a Device> {
        if let Some(id) = id {
            Self::find_device(devices, id).ok_or(
                SlightError::Parse, /*todo!("Error! No specified device found!")*/
            )
        } else {
            Self::default_device(devices).ok_or(
                SlightError::Parse, /*todo!("Error! No suitable default device!")*/
            )
        }
    }

    fn find_device<'a>(devices: &'a [Device], id: &'a str) -> Option<&'a Device> {
        devices.iter().find(|d| d.id == id)
    }

    fn default_device(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|d| d.class == Class::Backlight)
    }
}

impl TryFrom<&Args> for Slight {
    type Error = SlightError;

    fn try_from(a: &Args) -> std::result::Result<Self, Self::Error> {
        let devices = Self::scan_devices();
        // TODO: any reasons to pass a reference?
        let device = Self::select_device(&devices, a.id.as_deref())?;
        let exponent = a.exponent.unwrap_or(EXPONENT_DEFAULT);
        Ok(Self {
            device: device.clone(),
            exponent,
            new_value: a.value,
            percent: a.percent,
            stdout: a.stdout,
        })
    }
}
